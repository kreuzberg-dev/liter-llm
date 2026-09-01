#!/usr/bin/env bash
set -euo pipefail

#   TAG=v1.4.0-rc.31 VERSION=1.4.0-rc.31 \

tag="${TAG:?TAG is required (e.g. v1.4.0-rc.31)}"
version="${VERSION:?VERSION is required (e.g. 1.4.0-rc.31)}"
tap_dir="${TAP_DIR:?TAP_DIR is required (path to homebrew-tap checkout)}"
dry_run="${DRY_RUN:-false}"

formula="${tap_dir}/Formula/liter-llm.rb"

[[ -f "$formula" ]] || {
  echo "Missing $formula" >&2
  exit 1
}

tarball_url="https://github.com/xberg-io/liter-llm/archive/${tag}.tar.gz"

echo "Updating Homebrew formula for liter-llm ${version} (tag ${tag})"

if [[ "$dry_run" == "true" ]]; then
  echo "[dry-run] target formula: $formula"
  echo "[dry-run] would set url to: $tarball_url"
  echo "[dry-run] would compute sha256 of source tarball and rewrite the formula"
  echo "[dry-run] would leave bottle DSL untouched (handled by homebrew-merge-bottles)"
  exit 0
fi

echo "Fetching source tarball SHA256 for ${tag}..."
sha256=$(curl -fsSL "$tarball_url" | shasum -a 256 | awk '{print $1}')
echo "  url:    $tarball_url"
echo "  sha256: $sha256"

python3 - "$formula" "$tarball_url" "$sha256" <<'PY'
import re
import sys

formula_path, new_url, new_sha = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(formula_path).read()


def sub_once(pattern, repl, subject, what):
    """Substitute exactly one match, failing loudly when there is none.

    `re.sub` returns the subject unchanged when nothing matched, so a failed
    substitution is indistinguishable from a successful one that happened to be a
    no-op. This writes to an external tap repo that nothing downstream re-validates,
    so an unmatched anchor would silently publish a formula still carrying the
    previous release's url, sha256, or build deps. Count first, fail on zero. ~keep
    """
    result, count = re.subn(pattern, repl, subject, count=1, flags=re.MULTILINE)
    if count == 0:
        sys.exit(f"{formula_path}: no match for {what}; the formula's shape has drifted")
    return result


# Split off the bottle block so the regex only touches the formula header.
bottle_start = text.find("bottle do")
if bottle_start == -1:
    head, tail = text, ""
else:
    head, tail = text[:bottle_start], text[bottle_start:]

head = sub_once(r"""^(\s*url\s+)["'][^"']*["']""", rf'\1"{new_url}"', head, "the source url")
head = sub_once(r"""^(\s*sha256\s+)["'][^"']*["']""", rf'\1"{new_sha}"', head, "the source sha256")

# ~keep The build-dep injections below operate on the WHOLE formula, not on `head`. They
# anchor on `depends_on "rust" => :build`, which sits *after* the bottle block in this
# formula, so anchoring them inside the header could never match: both injections had been
# silent no-ops for the tap's entire history (`git log -S protobuf -- Formula/liter-llm.rb`
# in xberg-io/homebrew-tap returns nothing) and the missing post-substitution assertion is
# why nobody found out. The header split exists only to keep the url/sha256 regexes off the
# bottle block's own sha256 lines; it was never meant to scope these.
text = head + tail

# liter-llm-cli pulls liter-llm-proxy -> etcd-client v0.15 (with the
# `etcd-watch` feature) whose build.rs shells out to `protoc` via prost-build.
# `etcd-watch` is off by default since v1.6.4, so this dep is only a no-op
# safety net on default brew builds — but it does no harm to keep it. Idempotent.
if "depends_on 'protobuf' => :build" not in text and 'depends_on "protobuf" => :build' not in text:
    text = sub_once(
        r"""(^\s*depends_on\s+['"]rust['"]\s+=>\s+:build[^\n]*\n)""",
        r"\1  depends_on 'protobuf' => :build\n",
        text,
        "the `depends_on 'rust'` anchor the protobuf build dep is injected after",
    )

# opendal-core (used by liter-llm-proxy's opendal-cache feature) unconditionally
# pulls reqwest with `hyper-tls` -> `native-tls` -> `openssl-sys`. brew's
# arm64_linux/x86_64_linux source-build sandbox lacks system OpenSSL, so the
# `openssl-sys` build script fails with "Could not find directory of OpenSSL
# installation". Without an upstream fix in opendal-core to honour `default-
# features = false` on its reqwest dep, brew needs `openssl@3` as a build dep.
# Idempotent injection.
if "depends_on 'openssl@3' => :build" not in text and 'depends_on "openssl@3" => :build' not in text:
    # The anchor is the protobuf line the block above injects, so this substitution
    # cascades off that one: if the protobuf anchor were ever to stop matching, this
    # one would stop matching too. Both now fail loudly rather than in silence. ~keep
    text = sub_once(
        r"""(^\s*depends_on\s+['"]protobuf['"]\s+=>\s+:build[^\n]*\n)""",
        r"\1  depends_on 'openssl@3' => :build\n",
        text,
        "the `depends_on 'protobuf'` anchor the openssl@3 build dep is injected after",
    )

with open(formula_path, "w") as f:
    f.write(text)
PY

echo "Updated $formula"
