#!/usr/bin/env python3
"""
Sync version from Cargo.toml workspace to all package manifests.

This script reads the version from Cargo.toml [workspace.package] and updates:
- pyproject.toml (root and packages/python/)
- package.json files (root and packages/typescript/)
- composer.json (root and packages/php/)
- packages/elixir/mix.exs
- packages/ruby/*.gemspec (via VERSION constant)
- packages/java/pom.xml
- packages/csharp/*.csproj
- packages/go/go.mod (note: Go versioning is via git tags; the module path is updated if present)

Missing files are skipped with a warning. Use --check for CI validation.
"""

import argparse
import json
import re
import sys
from collections.abc import Callable
from pathlib import Path
from typing import NamedTuple

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

YELLOW = "\033[33m"
GREEN = "\033[32m"
RED = "\033[31m"
CYAN = "\033[36m"
RESET = "\033[0m"
BOLD = "\033[1m"


def _color(text: str, code: str) -> str:
    """Wrap text in ANSI color if stdout is a TTY."""
    if sys.stdout.isatty():
        return f"{code}{text}{RESET}"
    return text


def info(msg: str) -> None:
    print(f"  {_color('•', CYAN)} {msg}")


def ok(msg: str) -> None:
    print(f"  {_color('✓', GREEN)} {msg}")


def warn(msg: str) -> None:
    print(f"  {_color('!', YELLOW)} {msg}", file=sys.stderr)


def error(msg: str) -> None:
    print(f"  {_color('✗', RED)} {msg}", file=sys.stderr)


# ---------------------------------------------------------------------------
# Version extraction
# ---------------------------------------------------------------------------


def get_repo_root() -> Path:
    """Return the repository root (directory containing Cargo.toml)."""
    script_dir = Path(__file__).resolve().parent
    return script_dir.parent


def get_workspace_version(repo_root: Path) -> str:
    """Extract version from Cargo.toml [workspace.package]."""
    cargo_toml = repo_root / "Cargo.toml"
    if not cargo_toml.exists():
        raise FileNotFoundError(f"Cargo.toml not found at {cargo_toml}")

    content = cargo_toml.read_text(encoding="utf-8")
    match = re.search(
        r"^\[workspace\.package\]\s*\nversion\s*=\s*\"([^\"]+)\"",
        content,
        re.MULTILINE,
    )
    if not match:
        raise ValueError(
            "Could not find version in Cargo.toml [workspace.package].\n"
            'Expected a block like:\n  [workspace.package]\n  version = "x.y.z"'
        )
    return match.group(1)


# ---------------------------------------------------------------------------
# Updater functions — each returns (changed, old_version, new_version)
# ---------------------------------------------------------------------------


def update_pyproject_toml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update the top-level version field in a pyproject.toml."""
    content = file_path.read_text(encoding="utf-8")
    original = content

    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    old_version = match.group(1) if match else "NOT FOUND"

    content = re.sub(
        r'^(version\s*=\s*)"[^"]+"',
        rf'\1"{version}"',
        content,
        count=1,
        flags=re.MULTILINE,
    )

    if content != original:
        file_path.write_text(content, encoding="utf-8")
        return True, old_version, version

    return False, old_version, version


def update_package_json(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update the version field (and any liter-lm/* dep versions) in package.json."""
    data = json.loads(file_path.read_text(encoding="utf-8"))
    old_version = data.get("version", "N/A")
    changed = False

    if data.get("version") is not None and data["version"] != version:
        data["version"] = version
        changed = True

    # Update any internal liter-lm package cross-references.
    def _maybe_update_deps(section: str) -> None:
        nonlocal changed
        deps = data.get(section)
        if not isinstance(deps, dict):
            return
        for dep_name, dep_ver in list(deps.items()):
            if not dep_name.startswith(("liter-lm", "@liter-lm/")):
                continue
            if isinstance(dep_ver, str) and dep_ver.startswith(("workspace:", "file:", "link:", "portal:")):
                continue
            if dep_ver != version:
                deps[dep_name] = version
                changed = True

    for section in ("dependencies", "devDependencies", "peerDependencies", "optionalDependencies"):
        _maybe_update_deps(section)

    if changed:
        file_path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

    return changed, old_version, version


def update_composer_json(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update the version field in a composer.json."""
    data = json.loads(file_path.read_text(encoding="utf-8"))
    old_version = data.get("version", "N/A")

    if data.get("version") == version:
        return False, old_version, version

    data["version"] = version
    file_path.write_text(json.dumps(data, indent="\t") + "\n", encoding="utf-8")
    return True, old_version, version


def update_mix_exs(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update @version attribute in an Elixir mix.exs."""
    content = file_path.read_text(encoding="utf-8")

    match = re.search(r'@version\s+"([^"]+)"', content)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(
        r'(@version\s+)"[^"]+"',
        rf'\1"{version}"',
        content,
    )

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        return True, old_version, version

    return False, old_version, version


def update_gemspec(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update spec.version or VERSION in a .gemspec file."""
    content = file_path.read_text(encoding="utf-8")

    # Match either: spec.version = "x.y.z" or VERSION = "x.y.z"
    match = re.search(
        r'(?:spec\.version|VERSION)\s*=\s*["\']([^"\']+)["\']',
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"
    ruby_version = _normalize_rubygems_version(version)

    if old_version == ruby_version:
        return False, old_version, ruby_version

    new_content = re.sub(
        r'((?:spec\.version|VERSION)\s*=\s*["\'])([^"\']+)(["\'])',
        rf"\g<1>{ruby_version}\g<3>",
        content,
    )

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        return True, old_version, ruby_version

    return False, old_version, ruby_version


def update_pom_xml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update the top-level <version> tag in pom.xml (the project version)."""
    content = file_path.read_text(encoding="utf-8")

    # Match the project-level <version> (first occurrence after <project …>)
    pattern = r"(<project[^>]*>.*?<version>)([^<]+)(</version>)"
    match = re.search(pattern, content, re.DOTALL)
    old_version = match.group(2).strip() if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(
        pattern,
        rf"\g<1>{version}\g<3>",
        content,
        count=1,
        flags=re.DOTALL,
    )

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        return True, old_version, version

    return False, old_version, version


def update_csproj(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update <Version> in a .csproj file."""
    content = file_path.read_text(encoding="utf-8")

    pattern = r"(<Version>)([^<]+)(</Version>)"
    match = re.search(pattern, content)
    old_version = match.group(2).strip() if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(
        pattern,
        rf"\g<1>{version}\g<3>",
        content,
    )

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        return True, old_version, version

    return False, old_version, version


def update_go_mod(file_path: Path, version: str) -> tuple[bool, str, str]:
    """
    Update liter-lm module version references in a go.mod require block.

    Note: Go module versioning is typically handled via git tags (v0.1.0).
    This function updates any require lines that reference the liter-lm module.
    If no such require line exists (standalone package), a warning is emitted.
    """
    content = file_path.read_text(encoding="utf-8")

    pattern = (
        r"(github\.com/kreuzberg-dev/liter-lm(?:/[^\s]+)?\s+)"
        r"v([0-9]+\.[0-9]+\.[0-9]+(?:-[^\s]+)?)"
    )
    match = re.search(pattern, content)
    old_version = match.group(2) if match else "NOT FOUND"

    if old_version == "NOT FOUND":
        # No cross-module version reference; nothing to update.
        return False, "NOT FOUND (no require line — versioned via git tags)", version

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(
        pattern,
        rf"\g<1>v{version}",
        content,
    )

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        return True, old_version, version

    return False, old_version, version


# ---------------------------------------------------------------------------
# Normalisation helpers
# ---------------------------------------------------------------------------


def _normalize_rubygems_version(version: str) -> str:
    """Convert semver pre-release (1.0.0-rc.1) to RubyGems form (1.0.0.pre.rc.1)."""
    if "-" not in version:
        return version
    base, pre = version.split("-", 1)
    return f"{base}.pre.{pre.replace('-', '.')}"


# ---------------------------------------------------------------------------
# Result accumulation
# ---------------------------------------------------------------------------


class FileResult(NamedTuple):
    rel_path: str
    old_version: str
    new_version: str
    changed: bool
    skipped: bool
    skip_reason: str | None = None


def _process_file(
    repo_root: Path,
    file_path: Path,
    updater: Callable[[Path, str], tuple[bool, str, str]],
    version: str,
    dry_run: bool,
) -> FileResult:
    """
    Run an updater against a file, optionally rolling back changes in dry-run mode.

    Returns a FileResult describing the outcome.
    """
    rel = str(file_path.relative_to(repo_root))

    if not file_path.exists():
        return FileResult(
            rel_path=rel,
            old_version="",
            new_version=version,
            changed=False,
            skipped=True,
            skip_reason="file not found",
        )

    if dry_run:
        # Read current state, run updater, then restore original content.
        original_content = file_path.read_text(encoding="utf-8")
        changed, old_ver, new_ver = updater(file_path, version)
        if changed:
            file_path.write_text(original_content, encoding="utf-8")
        return FileResult(rel, old_ver, new_ver, changed, False)

    changed, old_ver, new_ver = updater(file_path, version)
    return FileResult(rel, old_ver, new_ver, changed, False)


# ---------------------------------------------------------------------------
# Main orchestration
# ---------------------------------------------------------------------------


def build_targets(
    repo_root: Path,
) -> list[tuple[Path, Callable[[Path, str], tuple[bool, str, str]]]]:
    """
    Return the ordered list of (file_path, updater) pairs for liter-lm.

    Files that do not exist are still listed; _process_file handles the skip.
    """
    return [
        # Root manifests
        (repo_root / "pyproject.toml", update_pyproject_toml),
        (repo_root / "composer.json", update_composer_json),
        # Python binding
        (repo_root / "packages" / "python" / "pyproject.toml", update_pyproject_toml),
        # TypeScript binding — root package.json and workspace package
        (repo_root / "package.json", update_package_json),
        (repo_root / "packages" / "typescript" / "package.json", update_package_json),
        # PHP binding
        (repo_root / "packages" / "php" / "composer.json", update_composer_json),
        # Go binding (see note in update_go_mod)
        (repo_root / "packages" / "go" / "go.mod", update_go_mod),
        # Java binding
        (repo_root / "packages" / "java" / "pom.xml", update_pom_xml),
        # Elixir binding
        (repo_root / "packages" / "elixir" / "mix.exs", update_mix_exs),
    ]


def collect_gemspecs(repo_root: Path) -> list[tuple[Path, object]]:
    """Find all *.gemspec files under packages/ruby/."""
    ruby_dir = repo_root / "packages" / "ruby"
    if not ruby_dir.exists():
        return []
    return [(p, update_gemspec) for p in ruby_dir.rglob("*.gemspec")]


def collect_csproj(repo_root: Path) -> list[tuple[Path, object]]:
    """Find all *.csproj files under packages/csharp/."""
    csharp_dir = repo_root / "packages" / "csharp"
    if not csharp_dir.exists():
        return []
    return [(p, update_csproj) for p in csharp_dir.rglob("*.csproj")]


def _print_summary(
    results: list[FileResult],
    version: str,
    dry_run: bool,
    check_mode: bool,
) -> None:
    """Print the summary of version sync results."""
    updated = [r for r in results if r.changed]
    unchanged = [r for r in results if not r.changed and not r.skipped]
    skipped = [r for r in results if r.skipped]

    if updated:
        print(_color("Changed:", BOLD))
        for r in updated:
            prefix = "[would update]" if (dry_run or check_mode) else "updated"
            ok(f"{r.rel_path}  {_color(r.old_version, YELLOW)} → {_color(r.new_version, GREEN)}  ({prefix})")

    if skipped:
        print(_color("\nSkipped (not yet created):", BOLD))
        for r in skipped:
            warn(f"{r.rel_path}  ({r.skip_reason})")

    if unchanged:
        print(_color("\nAlready at target version:", BOLD))
        for r in unchanged:
            info(f"{r.rel_path}  {_color(r.new_version, GREEN)}")

    print()

    if check_mode and updated:
        error(
            f"{len(updated)} manifest(s) are out of sync with Cargo.toml version {version}.\n"
            "    Run  python scripts/sync_versions.py  to fix."
        )

    if updated and not dry_run:
        print(_color(f"Synced {len(updated)} file(s) to version {version}.", GREEN))
    elif not updated:
        print(_color(f"All manifests already at version {version}.", GREEN))


def run(
    repo_root: Path,
    version: str,
    dry_run: bool = False,
    check_mode: bool = False,
) -> int:
    """
    Synchronise versions across all manifests.

    In check mode: exit 1 if any file would change (for CI).
    In dry-run mode: report what would change without writing.
    Returns an exit code (0 = success, 1 = failure / would-change).
    """
    label = "dry-run" if dry_run else ("check" if check_mode else "sync")
    print(f"\n{_color(BOLD + f'liter-lm version {label}', BOLD)} — {_color(version, CYAN)} (from Cargo.toml)\n")

    targets = build_targets(repo_root)
    targets += collect_gemspecs(repo_root)
    targets += collect_csproj(repo_root)

    results: list[FileResult] = []
    for file_path, updater in targets:
        result = _process_file(repo_root, file_path, updater, version, dry_run or check_mode)
        results.append(result)

    # Print summary and determine exit code
    _print_summary(results, version, dry_run, check_mode)

    # Return exit code (1 if check mode and files would change, else 0)
    updated = [r for r in results if r.changed]
    return 1 if (check_mode and updated) else 0


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Synchronise version across all liter-lm package manifests.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Sync all manifests to the version in Cargo.toml
  python scripts/sync_versions.py

  # Preview changes without writing
  python scripts/sync_versions.py --dry-run

  # CI check — exit 1 if versions are out of sync
  python scripts/sync_versions.py --check
        """,
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would change without writing to disk.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Exit with code 1 if any manifest is out of sync (for CI).",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    repo_root = get_repo_root()

    try:
        version = get_workspace_version(repo_root)
    except (FileNotFoundError, ValueError) as exc:
        error(str(exc))
        return 1

    return run(
        repo_root=repo_root,
        version=version,
        dry_run=args.dry_run,
        check_mode=args.check,
    )


if __name__ == "__main__":
    sys.exit(main())
