# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.19.1] - 2026-09-01

### Fixed

- **The plugin publish jobs no longer run ahead of version validation.** `publish-plugin-opencode`
  and `publish-plugin-hermes` depended only on `prepare`, so they published while
  `validate-versions` was failing. In v1.19.0 that shipped an npm bundle whose manifests still
  declared 1.18.4, and rebuilt the Hermes wheel at 1.18.4 where the skip-existing guard reported
  success having published nothing for the tag. Both jobs now require `validate-versions` to
  succeed.
- **`plugin/.ai-rulez/config.toml` is back in sync with the release version.** The 1.19.0 release
  commit bumped `Cargo.toml` and the alef-managed manifests but not the plugin's own version
  source, so every ai-rulez-generated plugin artifact stayed at 1.18.4 and `validate-versions`
  blocked the entire publish.

### Note

- v1.19.0 reached no registry except the npm plugin bundle described above; 1.19.1 is the first
  complete 1.19 release.

## [1.19.0] - 2026-08-31

### Added

- Scoop is now a live release channel alongside Homebrew: a release publishes a Scoop manifest for
  the CLI, and the install lists and badges cover it. The channel was previously inert because
  `scoop` was held out of the workflow's `available-targets` -- alef only gained the target after
  v0.79.2, and naming a target it does not recognise hard-errors the whole publish pipeline. The
  pinned alef is now 0.79.5, which carries it, so the gate is enabled.

### Fixed

- The `uv-bump` development dependency pointed at a GitHub fork that no longer exists, so
  resolving the dev dependency group failed outright and took the lint pipeline down with it. It
  now tracks the published `uv-bump` package on PyPI — the same tool, at 0.6.0 instead of 0.3.0.
- Dependency scanning now actually runs. The `cargo deny` step used a Docker container action
  that failed for two independent reasons: the runner pre-builds such an image at job start, but
  the `Free disk space` step prunes Docker images and deleted it before the step ran, and the
  action's Dockerfile downloads an `x86_64` cargo-deny binary that cannot execute on the
  `ubuntu-24.04-arm` runner anyway. No advisory, license, ban, or source check had ever been
  evaluated in CI since the scan was added. cargo-deny is now installed as a native binary and
  invoked directly.
- The release announcement no longer fires for a release that shipped nothing. `announce-discord`
  gated on `!contains(needs.*.result, 'failure')`, which cannot see a `skipped` job — and
  `skipped` is exactly what a publish job becomes when an upstream gate fails. This is the same
  defect `release-finalize` already documents and fixed for itself; the announcement now depends
  on `release-finalize` and gates on its verdict.
- `CI Rust`'s path filters now name the files its own gates read. Editing `deny.toml`,
  `alef.toml`, `rust-toolchain.toml`, or `.cargo/config.toml` previously started no run at all,
  so the dependency scan and the binding-freshness check could not disagree with the very
  configuration that drives them.

## [1.18.4] - 2026-08-28

### Changed

- Upgraded `etcd-client` 0.19 -> 0.20 and `jsonschema` 0.51 -> 0.52.

## [1.18.3] - 2026-08-27

### Note on 1.18.2

1.18.2 was tagged and left as an unpublished Draft release; it never reached crates.io or PyPI
(both remained at 1.18.1). Its publish run aborted on the `sync_plugin_version.py --expect` gate,
because at that tag `Cargo.toml` was 1.18.2 while `plugin/.ai-rulez/config.toml` was still 1.18.1.
The plugin pin was corrected on `main` after the tag was cut, so the tag could never satisfy the
gate on a retry. 1.18.3 supersedes it and ships the corrected pin.

## [1.18.2] - 2026-08-25

### Changed

- Regenerated all language bindings on alef 0.68.0.

### Fixed

- Outbound policies now cover every provider request URL, including streaming, multipart uploads,
  health probes, catalog refreshes, Vault, credential acquisition, and native DNS connection attempts
  and redirect targets. Authenticated clients reject cross-origin redirects so provider credentials
  cannot leak to a redirected host, while credential-free catalog downloads may follow a cross-origin
  redirect only after validating its target and never across an HTTPS-to-HTTP downgrade. Policy
  failures retain their non-retryable
  `OutboundForbidden` classification, and active policies disable HTTP proxies so connection-time DNS
  checks cannot be delegated around the guarded resolver. Vertex ADC limits its private-network
  exception to the fixed, redirect-free metadata endpoint and disables the opaque `gcp_auth` fallback
  under active policies; the embedded-library default remains `Off`, preserving local mock endpoints.

- **A skipped `publish-crates` no longer reads as a passing gate, and a release that published
  nothing can no longer report success.** Five places in `.github/workflows/publish.yaml` gated
  downstream build, publish and release-promotion jobs on
  `needs.publish-crates.result != 'failure'`. That expression is TRUE when the dependency was
  *skipped*, and `publish-crates` skips for two opposite reasons: the version is already on
  crates.io (a re-run or a resumed release, where downstream must proceed) or an upstream gate
  such as version validation or crate packaging failed (where downstream must not). `result`
  alone cannot separate them, so every one of those conditions was gating on nothing --
  tree-sitter-language-pack v1.15.5 promoted a GitHub release to `Latest` with 40+ failed jobs and
  every registry publish skipped, and still reported success.

  A new always-running `crates-gate` job resolves the ambiguity once, into an explicit
  `outcome` (`published` / `already-present` / `not-required` / `dry-run` / `blocked`) and an
  `ok` flag that every consumer now tests instead of `result`. Because the job always runs, its
  outputs always exist; because it never fails, depending on it cannot skip a consumer. The
  legitimate already-published path stays exactly as permissive as before, and only the
  gate-failed path is newly blocked.

  `release-finalize` gated finalization on `!contains(needs.*.result, 'failure')`, which cannot
  see a skipped job -- so a release whose targets all silently shipped nothing was finalized out
  of draft and reported green. It now verifies every enabled target individually before flipping
  the draft, and holds the release as a draft and fails when an enabled, not-yet-published target
  did not succeed.

  One of the five sites, `publish-homebrew-formula`, had the mirror-image defect: the job carries
  no `always()` or `!cancelled()`, so GitHub skipped it outright whenever `publish-crates` was
  skipped and its `if:` never ran at all. The formula therefore went unupdated on exactly the
  re-runs it was meant to cover. Depending on `crates-gate` -- which always succeeds -- lets the
  condition be evaluated, so the formula now updates on a resumed release and is still held back
  when the crates.io prerequisite was never met.

- **A drifted coding-agent plugin now fails the release instead of shipping.** `CI Plugin` already
  re-ran on `Cargo.toml`, so `--check` caught drift on `main` — but nothing asserted the plugin
  version against the tag actually being published. `scripts/sync_plugin_version.py` gains
  `--expect <version>`, which requires core and `plugin/.ai-rulez/config.toml` `[plugin].version` to
  both equal the version being released, and `publish.yaml`'s `validate-versions` job runs it against
  `needs.prepare.outputs.version`. `.task/tools/version-sync.yml` joins the `ci-plugin.yaml` path
  filter so edits to the sync task re-run the gate as well.

### Changed

- Repinned `alef.toml` `alef_version` from `0.67.2` to `0.67.5` and regenerated. Behaviour picked up
  from the three upstream releases:
  - Swift e2e assertions now decide a leaf's shape from the bridged getter rather than the accessor
    text, so `tool_calls` assertions that swift-bridge JSON-bridges to `RustString` are emitted as an
    explicit skip instead of an assertion that could not compile against the real binding.
  - Zig e2e assertions no longer wrap expected strings in `std.mem.trim`, which silently masked
    leading and trailing whitespace differences.
  - Java FFI wrappers rethrow `LiterLlmRsException` instead of re-wrapping it in a second
    `LiterLlmRsException("FFI call failed")`, so the original typed error survives the call boundary.
  - `poly.toml` adds `MD025` to the markdown formatter's disable list.
- `alef.toml` `[crates.exclude].functions` now excludes `configure_outbound_client_builder`. It takes
  and returns a `reqwest::ClientBuilder`, which has no binding representation, so alef sanitized both
  to `String` and failed extraction. It is a Rust-only escape hatch and is excluded the same way its
  outbound-policy siblings (`set_outbound_policy`, `current_policy`, `validate_outbound_url`) already
  were.

- Repinned `alef.toml` `alef_version` from `0.66.0` to `0.67.2` and regenerated. Behaviour picked up
  from the two upstream releases:
  - `crates/liter-llm-ffi/build.rs`: C header generation is now opt-in behind
    `ALEF_EXPORT_GENERATED_HEADERS=1` and writes both `crates/liter-llm-ffi/include/liter_llm.h` and
    `packages/go/include/liter_llm.h` atomically with rollback. A plain `cargo build -p liter-llm-ffi`
    no longer rewrites either header, so ordinary builds stop mutating committed source; regenerating
    with the flag set reproduces the committed headers byte for byte. `alef verify` is the freshness
    gate now.
  - `packages/dart`: flutter_rust_bridge moved from 2.12.0 to 2.13.0 (`pubspec.yaml`,
    `packages/dart/rust/Cargo.toml`, `Cargo.lock`, regenerated bridge). The generated bridge now
    allows `mismatched_lifetime_syntaxes` and fully qualifies `std::result::Result::Ok`.
    `packages/dart/rust/build.rs` passes `--no-deps-check` to `flutter_rust_bridge_codegen` and only
    re-applies the FRB cfg gates after a regeneration — the committed bridge already carries them, so
    the previous unconditional repair on every build was redundant.
  - `docs-site/src/snippets`: 1,691 snippets regenerated. The generator now prints a field of the
    result instead of the whole value (`System.out.println(result.data())` rather than
    `System.out.println(result)`). Where the fixture's assertion path is not a field of the returned
    type — `is_true` on `cost_tracked`, `equals` on `error.status_code`, or `audio` on a response the
    binding flattens to a byte array — the emitted accessor does not exist and the snippet no longer
    typechecks. Verified against built packages: 17 C# and 78 Java snippets regress this way. Tracked
    upstream against alef; these snippets are generated and cannot be fixed in this repo.

## [1.18.1] - 2026-08-22

### Fixed

- `packages/ruby/sig/types.rbs`: the `fine-tune` enum value was emitted as a bare symbol,
  `:fine-tune`, which is not valid RBS — the hyphen terminates an unquoted symbol literal, so the
  declaration failed to parse and steep could not type-check the Ruby package. It is now emitted
  quoted, `:"fine-tune"`, and `poly lint`'s steep check passes again. Fixed upstream in alef 0.64.0
  and picked up here by regenerating the bindings.

- `.task/tools/docs.yml`: `docs:snippets:validate` and `docs:snippets:validate:lang` invoked
  `alef snippets validate`, a subcommand that does not exist (`alef snippets` has only `list`,
  `check`, `parse`, `audit`, `gaps`), so both targets failed immediately with
  `error: unrecognized subcommand 'validate'`. Both now call `alef snippets check --strict`, with
  the per-language target using `--lang`. The canonical `docs:snippets:all` gate was never affected
  — it already called `check` directly. The removed `LEVEL` variable has no replacement: validation
  level is config-owned (`validation_level = "typecheck"`), so per-language runs now typecheck
  instead of defaulting to syntax.

- `.github/workflows/publish.yaml`: the `dry_run` workflow_dispatch input declared
  `default: "false"` (a quoted string) against `type: boolean`, which made the whole `on:` block
  fail GitHub's workflow schema (`gau --check` reported
  `schema validation failed at /on: ... is not valid under any of the schemas listed in the 'oneOf' keyword`).
  Its sibling inputs `force_republish` and `republish` already used an unquoted `false`. No job
  behaviour changes: every `if:` in the workflow gates on the normalized string outputs
  `steps.meta.outputs.dry_run` / `needs.prepare.outputs.dry_run`, never on `inputs.dry_run`.

### Removed

- Retired 18 of 20 `legacy_python_*` hand-maintained doc snippets under `docs-site/src/snippets/python`
  whose API surface (basic/streaming/tool-calling chat, multi-turn, configuration, embeddings, batches,
  files, responses, image generation, local providers, moderation, OCR, rerank, search, speech,
  transcription) is already covered by alef-generated fixture snippets. Doc pages under
  `docs-site/src/content/docs/usage/*.mdx` now import the equivalent generated fixtures instead.
  `legacy_python_guides_error_handling` and `legacy_python_multimodal` are kept: they demonstrate
  per-exception-type branching and multimodal/audio/document/structured-output patterns that the
  generated fixtures do not currently express.
- Retired 18 of 19 `legacy_csharp_*` hand-maintained doc snippets under `docs-site/src/snippets/csharp`
  covering the same topic set as the Python legacy snippets (the C# tree has no `multimodal` legacy
  file). `legacy_csharp_guides_error_handling` is kept for the same reason as its Python counterpart.
- Retired 18 of 19 `legacy_dart_*` hand-maintained doc snippets under `docs-site/src/snippets/dart`,
  same topic set and same triage as csharp/python. `legacy_dart_guides_error_handling` is kept.
  These snippets were only referenced from `reference/legacy-snippet-catalog.mdx`; that page now
  points at the equivalent generated fixtures instead.
- Retired 18 of 19 `legacy_elixir_*` hand-maintained doc snippets under `docs-site/src/snippets/elixir`,
  same triage as csharp. `legacy_elixir_guides_error_handling` is kept.
- Retired 18 of 20 `legacy_go_*` hand-maintained doc snippets under `docs-site/src/snippets/go`, same
  triage as python. `legacy_go_guides_error_handling` and `legacy_go_usage_multimodal` are kept.
- Retired 18 of 19 `legacy_java_*` hand-maintained doc snippets under `docs-site/src/snippets/java`,
  same triage as csharp. `legacy_java_guides_error_handling` is kept.
- Retired 18 of 19 `legacy_kotlin_*` hand-maintained doc snippets under `docs-site/src/snippets/kotlin`.
  Alef's generated fixtures for this target live under `docs-site/src/snippets/kotlin-android`
  (the crate key is `kotlin_android`, not `kotlin`); the legacy-snippet-catalog page now points there.
  `legacy_kotlin_guides_error_handling` is kept.
- Retired 18 of 20 `legacy_php_*` hand-maintained doc snippets under `docs-site/src/snippets/php`,
  same triage as python/go. `legacy_php_guides_error_handling` and `legacy_php_usage_multimodal` are
  kept.
- Retired 18 of 19 `legacy_ruby_*` hand-maintained doc snippets under `docs-site/src/snippets/ruby`,
  same triage as csharp. `legacy_ruby_guides_error_handling` is kept.
- Retired 18 of 19 `legacy_rust_*` hand-maintained doc snippets under `docs-site/src/snippets/rust`,
  same triage as csharp. `legacy_rust_guides_error_handling` is kept.
- Retired 18 of 19 `legacy_swift_*` hand-maintained doc snippets under `docs-site/src/snippets/swift`,
  same topic set and same triage as dart (catalog-only references).
  `legacy_swift_guides_error_handling` is kept.
- Retired 18 of 19 `legacy_typescript_*` hand-maintained doc snippets under
  `docs-site/src/snippets/typescript`, same triage as python. `legacy_typescript_guides_error_handling`
  is kept.
- Retired 18 of 19 `legacy_wasm_*` hand-maintained doc snippets under `docs-site/src/snippets/wasm`,
  same triage as csharp. `legacy_wasm_guides_error_handling` is kept.
- Retired 18 of 19 `legacy_zig_*` hand-maintained doc snippets under `docs-site/src/snippets/zig`,
  same topic set and same triage as dart/swift (catalog-only references).
  `legacy_zig_guides_error_handling` is kept.

  This completes the doc-snippet triage across all 14 languages that carried `legacy_*` snippets
  (269 files total): 252 retired as superseded by alef-generated fixtures, 17 kept
  (`guides_error_handling` in every language, plus `usage_multimodal` in go/php and
  `python_multimodal` in python) because they demonstrate patterns — per-exception-type branching,
  and multimodal/audio/document/structured-output/typed-constructor usage — that the generated
  fixtures do not currently express.

## [1.18.0] - 2026-08-22

### Added

- Multimodal embedding inputs can combine tagged text, image URL, and base64 image parts. Self-hosted
  embedding providers forward the complete input, and vector metadata can retain an image payload
  and convert it directly to `ContentPart::ImageUrl` for multimodal RAG.
- A queueing `InFlightLimitLayer` and managed-client configuration now bound simultaneous provider
  requests globally per client. Cache hits bypass the limiter, `None` remains unlimited, and zero is
  rejected as invalid configuration.

### Changed

- **Breaking:** `EmbeddingProvider::embed` now accepts `&EmbeddingInput` instead of `&str`, and
  `VectorMetadata` adds an `image_url` field. Custom provider implementations and metadata struct
  literals must be updated. Built-in Bedrock, Google AI, and Vertex AI embedding adapters reject
  multimodal inputs explicitly; use a compatible custom or self-hosted endpoint for image embeddings.

## [1.17.3] - 2026-08-21

### Fixed

- **The release actually publishes.** v1.17.2 was tagged and released but published nothing to any
  registry: the `Validate versions` gate failed on stale `Cargo.lock` files under `e2e/rust` and
  `packages/ruby/ext/liter_llm_rb/native`, which skipped the crates.io publish job. Every
  language-package build behind it then failed with
  `failed to select a version for the requirement ^1.17.2`, because `alef publish prepare` was
  retrying against a registry version that had never been pushed. The lockfiles are refreshed and
  the gate passes. Note that Packagist did ingest the 1.17.2 tag through its own webhook,
  independently of CI, so PHP is the one ecosystem where 1.17.2 resolves; every other ecosystem is
  still on 1.17.1. Use this version instead.

## [1.17.2] - 2026-08-21

### Fixed

- Generated bindings regenerated on alef 0.62.8. This carries alef's fixes for JNI and FFI casts to
  a value's own type (which tripped `clippy::unnecessary_cast` in the JNI crate, linted with
  `-D warnings`), a dead `java.util.List` import, and a `kotlin.test.assertNotNull` import emitted
  where nothing referenced it.

- `packages/ruby/ext/liter_llm_rb/native/Cargo.toml` and `e2e/rust/Cargo.toml` now follow the
  project version. Both are alef-owned but were never reached by the version sync, so each release
  left them pinned to the previous version.

- The publish workflow pins `xberg-io/actions` at `@v1` rather than `@main`, so it no longer tracks
  untagged changes to that repository.

### Security

- `h2` advanced to 0.4.18, resolving RUSTSEC-2026-0258 (unbounded empty DATA frames: a peer could
  queue empty frames without limit, risking unbounded memory use or a panic on length overflow).
  Low severity.

### Changed

- All Rust dependencies taken to their latest versions (`cargo upgrade --incompatible` followed by
  `cargo update`): `jsonschema` 0.49 to 0.50 and `rand` 0.9 to 0.10 as direct dependencies, plus 105
  transitive bumps and two new packages. Nothing was downgraded. `rand` 0.10 required source
  changes: the `os_rng` feature is now `sys_rng`, `small_rng` is gone (`SmallRng` compiles
  unconditionally), `SmallRng::from_os_rng()` becomes `rand::make_rng::<SmallRng>()`, and
  `Rng::random_range` moved to a new `RngExt` trait.

- alef pinned to 0.62.8.

### Known issues

- The Dart package does not analyze cleanly. `packages/dart/lib/src/liter_llm.dart` calls
  `countTokens`, `countRequestTokens` and `recordCostUsd`, which the committed
  flutter_rust_bridge bridge does not declare, so `dart analyze` reports three
  `undefined_function` errors. The three functions are declared in the bridge facade; the bridge
  generator emits them only when invoked as a `cargo` build-script subprocess and not when invoked
  as a bare CLI, which is how alef calls it. Tracked upstream; other language bindings are
  unaffected.

## [1.17.1] - 2026-08-15

### Changed

- **wasm/typescript**: `#[serde(untagged)]` data enums now generate real structural TypeScript
  types instead of `any`. Six types change shape in the generated `.d.ts`: `ModerationInput` and
  `EmbeddingInput` (`string | string[]`), `StopSequence` (`string | string[]`), `RerankDocument`
  (`string | { text: string; }`), `UserContent` (`string | any[]`), and `ToolChoice`
  (`"auto" | "required" | "none" | WasmSpecificToolChoice`). TypeScript consumers may see **new
  compile errors** at call sites that previously typechecked only because `any` hid a genuine
  mismatch between the value passed and the shape the binding actually accepts. These are
  pre-existing bugs the old typing concealed, not new runtime restrictions — the accepted runtime
  values are unchanged. `AssistantContent` is deliberately exempt via `untagged_union_text_types`
  and continues to bridge as a plain `string` on its field, getter, and setter.

- **catalog**: refreshed the bundled model catalog (`schemas/catalog.json`) from models.dev. The
  catalog is embedded via `include_str!` and backs pricing and capability lookups in `cost.rs`.
  Providers went from 184 to 185 and models from 6293 to 6321: **70 models added, 42 removed, and
  no existing model repriced** — input and output costs are byte-identical for every model present
  both before and after. The single new provider is `crusoe`.

  The removals are the upgrade risk, since a removed identifier no longer resolves for cost
  lookup. 36 of the 42 are under `cloudflare-ai-gateway`, and they are genuine deletions rather
  than renames — nothing was added back under those names. Notably **every** Anthropic model
  behind that gateway is gone (`claude-3-haiku`, `claude-3-sonnet`, `claude-3-opus`,
  `claude-3.5-haiku`, `claude-3-5-haiku`, `claude-3.5-sonnet`, `claude-sonnet-4`,
  `claude-opus-4`, `claude-opus-4-1`), along with 24 `workers-ai` models,
  `openai/gpt-5.1-codex`, `openai/gpt-5.2-codex`, and `moonshotai/kimi-k3`. The remaining six
  removals are `inclusionai/ling-3.0-tiny` variants across `kilo`, `nano-gpt`, `openrouter`, and
  `vercel`, plus `nano-gpt/TEE/minimax-m2.5`. If you route Anthropic models through
  `cloudflare-ai-gateway`, cost lookup for those ids will no longer resolve after this upgrade.

### Fixed

- **client**: stop re-inserting `stream` into request bodies that providers rebuilt without it.
  `prepare_request` set the transport-controlled `stream` flag before `transform_request`, then
  restored it after the `extra_body` merge so `extra_body` could not desync the wire flag from the
  chosen transport. That restore ran for every provider, including those whose native wire format
  has no `stream` field at all. Vertex/Gemini rebuilds the body into `generateContent` shape and
  deliberately drops the key, so a plain non-streaming call shipped `"stream": false` and Vertex
  rejected the entire request with HTTP 400 `Invalid JSON payload received. Unknown name "stream":
  Cannot find field.` — breaking **every** non-streaming chat completion against Vertex AI,
  including vision and structured-extraction calls, which surfaced only as a silently absent
  result. The restore is now scoped to providers whose body still carried `stream` after their own
  transform, so OpenAI-compatible, Anthropic, and Cohere bodies keep the flag while Vertex and
  Bedrock — which select streaming by endpoint — no longer receive a field their native request
  formats do not define.
- **version**: stop the per-language binding crates from drifting behind the core workspace
  version, so a binding crate published from this release carries the same version as the
  `liter-llm` core it was built against.
- **docs**: label the MCP HTTP transport snippet as `bash` instead of `toml` so it renders with the
  correct syntax highlighting.

## [1.17.0] - 2026-08-13

### Upgrade notes

- **`tenant_id` no longer defaults to the virtual key token — this resets budget spend and cache
  scope for every unconfigured key.** The key token was silently doubling as the tenant identifier,
  which meant a live credential flowed into the OTLP `gen_ai.budget.tenant_id` attribute, the
  budget-ledger CSV chargeback export, tenant-scoped cache keys at rest, and `UsageEvent` logs. That
  could not be preserved as the default — it was the vulnerability. Virtual keys now take an
  optional `tenant_id` field; if left unset, a *new* tenant id is derived instead of reusing the key.
  On deploy this means: **month-to-date budget spend resets to zero for every key that does not set
  `tenant_id` explicitly**, and that key's existing tenant-scoped cache entries become orphaned
  (unreachable, not deleted).

  If you need **billing** continuity across this upgrade, set `tenant_id` to the old key value for
  every affected virtual key **in the same deploy**, before traffic hits the new build.

  **Understand what that costs before you do it.** Setting `tenant_id` to the key token puts the
  live credential back into every sink listed above — the OTLP attribute, the chargeback export,
  cache keys at rest and `UsageEvent` logs. None of those sinks were hardened by this release; only
  the default changed. The code deliberately prints `tenant_id` in plain text (in `KeyContext`'s
  `Debug`, for one) precisely *because* it is no longer expected to be a secret, so this remedy
  reinstates the exposure the release exists to remove. Treat it as a temporary migration step:
  carry spend across the deploy, then rotate the affected keys and drop the explicit `tenant_id`.
  If you can absorb a spend reset instead, that is the safer choice.

  Cache continuity is **not** available by any route — see the next note. Every entry is
  unreachable after this deploy regardless of what `tenant_id` is set to, so do not accept the
  credential-reexposure cost expecting to keep a warm cache.
- **Deploying this release invalidates every existing cache entry.** The cache key now folds in
  `tenant_id` and `system_prompt` (both previously hard-coded to `None`) and additionally hashes
  `tools`, `response_format`, `seed`, `presence_penalty`, `frequency_penalty`, `logit_bias`,
  `tool_choice`, `parallel_tool_calls`, `reasoning_effort`, `modalities` and `extra_body`. Old keys
  become unreachable the moment this deploys — there is no error and no warning, just a cold cache
  and a burst of upstream traffic and spend until it refills. This is intended: the old keys were
  the defect. Expect the cold-start cost and do not read the traffic spike as a regression.
- **Wire format: chat responses now always include `role`, `logprobs` and `refusal`.** OpenAI's
  schema requires all three on `choices[]`/`choices[].message`; a strict client deserializing a
  response from this proxy previously failed on the missing keys. `refusal` is now serialized even
  when `null`, which as a side effect also makes it appear on *request* messages you send — that is
  valid per OpenAI's schema, but if you diff request payloads byte-for-byte, expect the new key.
- **API: exhaustive struct literals against `Choice` or `CreateResponseRequest` will not compile.**
  `Choice` gained `logprobs`; `CreateResponseRequest` gained `extra_body`.
- **API: `CacheDecision` loses `stale_while_revalidate` and `use_streaming_replay`.** Both were
  public, documented, and read nowhere — an operator could set either and believe it took effect.
  `stale_while_revalidate` never served a stale entry, and the "opt-in via policy" streaming replay
  did nothing. `semantic_ttl` stays, but its doc no longer implies a separate duration: a semantic
  hit resolves to the same physical entry, which has exactly one TTL.
- **API: `HttpProbeHealthChecker::new` no longer takes a timeout parameter.** See the health-gate
  entry under Fixed.
- **API: `GuardrailService` and `HooksService` gained an `S: Clone` bound.** The bound is on the
  `Service` impl, not the struct, so it breaks composition rather than construction: a stack
  wrapping an inner service that is not `Clone` no longer compiles. Both needed it to defer the
  inner call until after their own pre-flight decision.

### Security

- **Singleflight deduplication ignored tenant identity, so one tenant could be served another's
  response.** `singleflight_key` hashed only the inner chat/embedding request; `tenant_id` lives on
  the `LlmRequest` envelope, whose `Serialize` forwards to that inner value alone, so tenant could
  not reach the hasher even indirectly — while the exact and semantic cache tiers, already fixed for
  this, read it explicitly. Two virtual keys posting byte-identical bodies inside one in-flight
  window collapsed to a single upstream call, and the follower skipped the budget, cost and
  guardrail layers entirely: its spend went unrecorded and its cap unenforced, while it was reported
  as a cache hit on a call made for someone else. Only reachable if you compose `SingleflightLayer`
  yourself; no in-tree stack did.
- **The proxy ran no guardrails at all.** The trait, the stage enum and the whole apply-path shipped,
  but nothing populated a guardrail set — see "Added" for the config surface that fixes it.
- **The virtual key token was echoed into client-visible 403 error bodies from six call sites**,
  across `require_master`, guardrail rejections and other proxy error paths — the live credential
  landed in the caller's own logs plus every access log, reverse proxy and error tracker on the
  response path. `Debug` output on `KeyContext` had the same problem: both `key_id` and `tenant_id`
  printed verbatim into any captured span or panic message. Both are now redacted; error bodies and
  `Debug` report a stable, non-reversible correlation id instead. Two tests that had pinned the leak
  by asserting the raw token was present now assert the opposite.
- **`tenant_id` was the virtual key token itself, and it flowed into weakly-protected sinks with no
  rotation-linked purge**: the OTLP `gen_ai.budget.tenant_id` metric attribute, the budget-ledger CSV
  chargeback export, tenant-scoped cache entries at rest, and `UsageEvent` usage logs. See "Upgrade
  notes" above — this is now fixed, and the fix changes tenant identity for unconfigured keys.
- **An empty master key was a full authentication bypass, not a weak credential.**
  `Authorization: Bearer` (empty, trailing space) resolved to `KeyContext::master()` — full
  read/delete access to every tenant's files, batches and responses — because unset env-var
  interpolation yields `""`, and the constant-time comparison treated two empty byte slices as
  equal. Config load now refuses a master or virtual key that interpolates to empty, and the
  comparison itself rejects empty operands. Virtual keys had the same hazard and are fixed the same
  way.
- **The realtime proxy documented guardrail enforcement it has never performed.** The module doc
  stated as fact that client → upstream messages are checked at `GuardrailStage::Input` and
  upstream → client at `GuardrailStage::OutputChunk`. The machinery to do so exists and is
  correct, but the only production call site passes an empty guardrail set, and there is no proxy
  config field, no `AppState` field and no other caller that populates one — so both checks
  iterate nothing and always allow. No behaviour changed here; what changed is that the docs no
  longer assert a control that is not running. If you relied on realtime sessions being moderated,
  they were not. Wiring a real config surface for this is tracked separately.
- **MCP tool calls and realtime sessions bypassed per-key rate limits and budgets entirely.**
  Model-routed MCP calls never attached `tenant_id` to the Tower stack, and a missing `tenant_id` is
  treated as intentionally unlimited; realtime never entered the Tower stack at all. Both are now
  routed through the same rpm/tpm/budget enforcement a unary HTTP call gets — realtime is
  necessarily pre-flight-only, since a live session has no discrete request to meter.
- `server.request_timeout_secs` was parsed from config and had no effect. It is now applied to unary
  routes, bounding an upstream that never responds. (Realtime and already-flowing stream bodies are
  not covered — see the commit notes for why.)
- CEL guardrail expressions could exhaust the native stack through an unparenthesized operator chain
  (`"!!!!...true"`, a long `&&` chain) that the existing bracket-depth cap could not see, since it
  counts only `( [ {`. Operator count is now bounded too. Not a confirmed exploit — defence in depth
  alongside the existing depth cap.
- **Proxy: any valid virtual key could read, download and delete another tenant's files, batches and
  responses.** The nine REST handlers in `routes/files.rs` and `routes/batches.rs` bound `KeyContext`
  as `_key_ctx` and discarded it. These IDs are opaque identifiers on a single shared upstream
  provider account with no per-tenant ownership record in the proxy, so they are now scoped to master
  keys — matching what the MCP layer already enforced for the same twelve operations.
- **Proxy: revoked virtual keys and rotated master keys kept working until restart.** The config
  watcher swapped only the config `Arc`; `KeyStore` is now built once, shared with the watcher, and
  reloaded on every `Put`/`Resync` event.
- **Cache: enabling the semantic tier silently defeated tenant isolation.** The exact tier folded
  `tenant_id` into its key, but the semantic tier wrote every entry with `tenant_id: None` and
  `VectorStore::search` took no tenant argument, so it could not have filtered even if the metadata
  were populated — tenant B asking a semantically similar question was served tenant A's cached
  response. `search` now takes a tenant and both backends filter on it. `None` matches only `None`;
  treating it as a wildcard would reopen the hole it closes.
- **Bedrock: a SigV4 signing failure sent the request to AWS unauthenticated.** `signing_headers`
  called `sigv4_sign(...).unwrap_or_default()`, so failure yielded empty headers. Credentials are now
  validated on every request and fail hard before any network I/O.
- **Credentials appeared in `{:?}` output.** `LlmConfig` and `BedrockConfig` derived `Debug` over
  `api_key`, `access_key_id`, `secret_access_key` and `session_token`, so any debug formatting — a
  tracing event, a panic message — printed live credentials. Both now redact, matching the existing
  `ClientConfig` convention. Proxy config structs holding virtual keys and provider credentials are
  redacted likewise.
- **Output guardrails did nothing for streamed responses.** `GuardrailStage::OutputChunk` was defined
  and documented but never invoked, so a guardrail that blocks a phrase had no effect once the caller
  streamed. Chunks now pass through it, and a blocked chunk terminates the stream: bytes already sent
  cannot be recalled, but nothing further reaches the caller.
- CEL guardrail expressions reached the parser with no bound on length or nesting depth. A stack
  overflow aborts the process and `catch_unwind` cannot catch it, so one malicious rule string could
  take the process down. Expressions are now validated before compilation. The abort was never
  reproduced — this is defence in depth, not a confirmed exploit.
- A panicking guardrail poisoned the global registry lock and permanently disabled guardrail
  enforcement for the rest of the process; the guard is now recovered with a warning.
- The CLI now warns when a master key is passed as a command-line argument, where it is visible in
  the process table.

### Changed

- Chat responses now always serialize `role` (on `choices[].message`), `logprobs` (on `choices[]`)
  and `refusal` (present even when `null`) to satisfy OpenAI's response schema. See "Upgrade notes."
- `Choice` gained a `logprobs` field; `CreateResponseRequest` gained an `extra_body` field;
  `GuardrailService` and `HooksService` both gained an `S: Clone` bound. See "Upgrade notes."
- **`DenyListGuardrail` never blocked anything, and `AllowListGuardrail` would have blocked
  everything.** Both decide on `GuardrailContext::metadata`, and nothing populated it per call —
  `GuardrailService` passed only the static per-layer map, and `LlmRequest::tenant_id` was never
  copied in. A deny-list on `tenant_id` therefore read `None`, which that guardrail treats as
  "nothing to deny", so a configured and documented access control silently allowed every request;
  an allow-list read `None` as "required field absent" and would have blocked all traffic. CEL
  expressions referencing `metadata.tenant_id` were broken the same way. `tenant_id` is now merged
  into the per-call context. Static per-layer metadata still wins on a key collision, with a
  warning, so an operator's explicit value is never silently overwritten.
- **A hook-rejected request no longer consumes rate-limit and budget accounting.** `HooksService`
  called the inner service before running its `on_request` hooks, so a request the hooks rejected
  had already executed every inner layer's synchronous body — and `ModelRateLimitService`
  increments its RPM counter there. A burst of hook-rejected traffic therefore exhausted the RPM
  window and started rejecting *legitimate* requests, none of which had reached a provider. No
  upstream call was ever made, so this cost bookkeeping rather than money, but it is a self-DoS
  any client could trigger by repeatedly sending content a guardrail hook blocks. The inner call
  now happens after the hooks pass. The existing short-circuit test could not catch this: its mock
  counted inside the async body, which is never polled either way.
- **An Azure-routed Responses call sent a malformed URL instead of failing.** All four
  `ResponseClient` methods pass an empty model to `build_url` — none of create/retrieve/cancel has
  one to give — and Azure embeds the model in the path unconditionally, so the request went out to
  `.../openai/deployments//responses` and relied on Azure's 404. It now returns
  `EndpointNotSupported` before any network call. The check tests the URL shape rather than the
  provider name, so it cannot reject OpenAI-compatible gateways running under a different provider
  name, and will catch any future provider with the same URL-embedding pattern. Bedrock, Vertex and
  Google AI were never affected — contrary to what the `ResponseClient` doc claimed, they fall
  through to the plain `{base}{path}` branch; that doc is corrected too.
- **`top_p` never reached Cohere at any value.** Cohere's v2 `/chat` endpoint has no `top_p` field
  — its nucleus-sampling parameter is named `p` — so the value was forwarded under a name Cohere
  does not recognise and silently dropped, leaving nucleus sampling entirely unset on every Cohere
  request that asked for it. It is now renamed to `p` on the way out and range-checked against
  Cohere's documented `[0.01, 0.99]`. Note the floor is `0.01`, not `0.0`: `top_p: 0.0` is legal
  for OpenAI and was legal per this crate's own docs, and is now rejected for Cohere rather than
  quietly coerced. The error names `top_p`, the field you set, not Cohere's internal `p`.
- **A single failed health probe took the whole service offline.** The global gate stored
  `result.is_ok()` from one `ListModels` probe, so any failure rejected all traffic with 503 until
  the next tick — and a provider that does not implement `ListModels` returns `EndpointNotSupported`
  and was taken down permanently for a capability gap, while ordinary chat through it would have
  worked. The gate now uses the same consecutive-failure thresholds the per-provider checker already
  had (3 to open, 2 to close), and `EndpointNotSupported` is not counted at all. Set both thresholds
  to `1` for the old behaviour. `HealthCheckConfig::timeout` was also applied to nothing, so a
  stalled checker froze the probe loop and left a dead provider marked healthy forever; it now bounds
  every probe. **Breaking: `HttpProbeHealthChecker::new` no longer takes a timeout** — two
  independent deadlines for one probe is worse than one.
- **Every cached error replayed as a 500.** The negative cache rewrote each error to
  `InternalError`, so a cached 429 came back without its `Retry-After` and with `is_transient()`
  false — clients stopped retrying. The variant is now preserved on both the write and read paths.
  Fixing only that would have been worse than the bug: replays re-entered the write path and pushed
  the expiry out every time, which was masked solely because the replayed error was non-transient.
  The window is now set once, by the call that saw the real failure, so steady polling can no longer
  keep an entry alive forever.
- **The OpenDAL cache backend silently discarded every configured TTL** — it never overrode
  `set_ttl`, so the trait's no-op ran and entries kept the store's construction-time TTL regardless
  of config or per-model policy. Both existing tests named for this defect used the in-memory store,
  which does override it. Error entries are also no longer logged at WARN on this backend, which
  previously produced one warning per failed request during exactly the outage the negative cache
  exists for.
- **Metrics mislabelled everything after the first request for a model.** The attribute cache was
  keyed on `(system, model)` but also cached `gen_ai.response.model` and `gen_ai.operation.name`, so
  one early failure fixed both for good — and requests with no model (`ListModels`, and image or
  moderation calls without one) all collapsed onto a single key and inherited whichever operation
  arrived first. Token usage also dropped every streamed request while cost did not, so a streaming
  deployment reported spend against zero tokens. `gen_ai.system` was empty on all cache-tier metrics,
  making hit rate impossible to break down by provider.
- **`temperature` above a provider's documented cap is now rejected locally instead of forwarded.**
  `ChatCompletionRequest` documented `temperature` as `[0.0, 2.0]` — OpenAI's range — while
  Anthropic and Amazon Bedrock both cap it at `1.0`, so a value legal per our own API docs failed
  at the provider after a network round trip. Those two now return a `BadRequest` naming the field,
  the value and the provider's range. The same outcome as before, just earlier and legible; nothing
  that previously succeeded now fails. Deliberately not clamped — the parameter is supported and
  only the scale differs, so clamping would silently rewrite the caller's request. Enforcement
  applies only where the provider's own schema states a bound: Cohere documents `temperature` as a
  non-negative float with no maximum, Google's docs contradict each other on the ceiling, and
  Mistral publishes a recommendation rather than a bound, so those are left forwarded unchecked.
- **The Responses API is documented as OpenAI-only.** No behaviour changed, but the docs previously
  implied otherwise — `parse_response_stream_event` described the event shape as "uniform across
  providers that support it", a set that does not exist: the provider registry models no
  `responses` endpoint for any of its 165 providers, no provider overrides `responses_path()`, and
  neither `transform_request` nor `transform_response` runs on this path. A `provider/model` prefix
  on `CreateResponseRequest.model` is not stripped and does not re-route. If you point this path at
  a non-OpenAI provider today, it does not work and is not expected to.

### Fixed

- **Cohere streaming was entirely non-functional.** The parser matched Cohere's legacy v1 NDJSON
  event names and field paths against the v2 endpoint the provider actually targets, so a stream
  opened cleanly and then yielded empty text, empty tool calls, and no `finish_reason` or usage —
  ever. All twelve existing streaming tests encoded the v1 shapes and asserted the broken behaviour;
  rewritten against the real v2 wire format.
- **Every non-streaming Cohere completion failed deserialization.** Cohere's v2 chat response has no
  `choices` wrapper — `id`, `finish_reason`, `message` and `usage` are top level — so
  `transform_response` was a no-op on real payloads. The old tests passed only because they fed the
  wrong shape as input.
- **Gemini streaming hit the non-streaming endpoint and failed with a misleading error.**
  `build_stream_url` reused the non-streaming `:generateContent` URL with `?alt=sse` appended;
  Gemini ignores `alt=sse` there and returns one ordinary JSON body, which surfaced as "SSE stream
  truncated" with no content. Both Google AI Studio and Vertex AI now call
  `:streamGenerateContent`.
- **Bedrock embedding requests were silently discarded.** Bedrock has no unified embeddings API, so
  embedding calls fell through to the Converse (chat) transform, which rebuilds the body and
  dropped the input entirely. Titan and Cohere embedding shapes are now dispatched on model prefix
  and their responses normalised to OpenAI's `data[]` list form; a batched Titan input errors rather
  than silently truncating to one vector, and an unrecognised embedding model errors instead of
  guessing.
- **Vertex, Google AI and Bedrock silently dropped documented OpenAI request fields**
  (`logprobs`/`top_logprobs`, `service_tier`, `metadata`, and others) because those providers
  rebuild the request body wholesale. Fields with a real equivalent are now mapped (e.g.
  `logprobs`/`top_logprobs` → Gemini's `responseLogprobs`/`logprobs`); anything without one now
  warns instead of vanishing. Anthropic had the opposite defect — it mutates the body in place, so
  seven unmapped fields reached its wire verbatim, and a caller's `metadata` collided with
  Anthropic's own `metadata.user_id`. Anthropic now strips what it doesn't support.
- **Setting `modalities` or `seed` on an Anthropic request failed the entire call with a 400.** Two
  fields were still missing from Anthropic's strip list, and the Messages API rejects any
  unrecognised top-level key with `"Extra inputs are not permitted"` rather than ignoring it — so
  these did not degrade the request, they broke it, including `modalities: ["text"]`, which is a
  no-op for a provider that only emits text. Both are now stripped; `seed` warns, since a caller
  pinning it for reproducibility gets none.
- The `cohere` and `cohere_chat` registry entries pointed at `api.cohere.ai` while the provider
  itself used `api.cohere.com`. Both hosts are live aliases so nothing failed, but anything reading
  the registry to locate Cohere disagreed with the client that sends the request. Both now say
  `api.cohere.com`, and a test pins them together.
- **The proxy rejected documented OpenAI chat request fields with a hard 400** instead of ignoring
  them: `logprobs`, `top_logprobs`, `max_completion_tokens`, `service_tier`, `store`, `metadata`,
  `prediction`, `audio`, `web_search_options`. All are now accepted and forwarded to the provider.
- **The Responses API had no escape hatch for provider extensions.** `CreateResponseRequest` had no
  `extra_body`, and because the type denies unknown fields, a caller-supplied `extra_body` was
  rejected outright — for example, there was no way to reach OpenAI's `reasoning.effort` on this
  path. `extra_body` is now merged on both the streaming and non-streaming Responses paths, reusing
  the same merge the chat path already used (GH [#174], reported by @huangmiuXyz).
- Guardrail `Mutate` decisions were applied at the `OutputChunk` streaming stage but silently
  ignored at the `Input` and `Output` stages — a redaction guardrail (including the built-in
  `RegexGuardrail` in Redact mode) sent the provider the unredacted content and returned the
  unredacted response to the caller. Both stages now apply the mutated payload, and fail closed: if
  a mutated payload cannot be applied, the call aborts rather than silently forwarding the original.
- The circuit breaker could be force-closed by a stale in-flight request. A request that started
  while the circuit was `Closed` and completed after other requests had since tripped it to `Open`
  unconditionally reset the breaker to `Closed` and zeroed the failure count on success, sending
  traffic back at a backend that was still down. Closing now requires a `HalfOpen -> Closed`
  transition; a success recorded against an already-`Open` circuit is ignored.
- A realtime session hanging up could shut down the entire proxy. The handler passed the
  process-wide shutdown token straight into the relay loop, so a normal session close cancelled the
  same token graceful shutdown awaits — any client could trigger it deliberately, and a legitimate
  user closing a call could trigger it by accident. Sessions now cancel a child token instead.
- Streamed requests that a caller abandoned early recorded no spend, even though the full
  completion is generated and buffered server-side before the first byte reaches the caller —
  opening and dropping streams in a loop was unmetered usage against a budget-limited endpoint.
  Spend now also settles on stream `Drop`.
- Several in-memory maps had no bound and were reachable with attacker-controlled keys, each a
  memory-exhaustion vector in a long-running proxy: the per-key idempotency store (also fixed to
  hash bodies deterministically across restarts and replicas, rather than per-process-random), the
  budget ledger's per-user/per-key spend maps (reclaims only zero-spend entries — an
  already-tracked principal's recorded spend is never discarded), and the semantic-routing
  classifier verdict cache (capped at 4096 entries by default).
- Cache TTL configuration was ignored on two of three write paths. `CacheLayer::new` hardcoded a
  300s policy TTL regardless of `CacheConfig.ttl`; `with_store` (used by every custom store and
  every OpenDAL backend via `ManagedClient`) took no config at all and had the same default.
  `CacheLayer::new` now honours it, and a new `CacheLayer::with_store_and_config` constructor
  carries the config through — `ManagedClient` uses it. **`with_store` itself is unchanged and
  still cannot honour a TTL**, because it takes no `CacheConfig`; if you build a custom store
  through it, switch to `with_store_and_config` or you will keep getting the 300s default.
- Weighted-random routing seeded its RNG from `SystemTime` subsecond nanos, so requests arriving in
  the same timer tick drew near-identical values and a burst of traffic collapsed onto one
  deployment instead of spreading per the configured weights. Now seeded once from OS entropy.
- A request body containing a non-ASCII character within the first 64 bytes panicked while
  computing the idempotency body hash, which sliced a fixed-length prefix on a byte boundary that
  could fall mid-character. Now cuts on the nearest character boundary; previously-working hashes
  are unchanged.
- `LiterLlmError` gained a `retry_after()` accessor alongside `status_code()` and `error_type()`,
  exposing the delay already parsed from a `Retry-After` header on `RateLimited` errors. Consumers
  previously had to invent their own backoff from the status code alone; bindings pick this up on
  their next regen.
- The npm package's `bin/liter-llm.js` was committed non-executable, so a checkout — or any install
  path that preserves archive file modes — could not run it directly.
- The C# README badge linked to `nuget.org/packages/LiterLlm`, a package this project does not own.
- **Budget: month-to-date spend reset on every config reload, not on restart.** `InMemoryBudgetLedger`
  had no way to update its limits, so the proxy rebuilt and swapped the whole ledger to apply a
  reloaded config, discarding every sliding window. With hot-reload enabled and frequent config
  pushes a tenant's budget could effectively never be enforced, and it failed open. Limits now sit
  behind an `ArcSwap` and only that pointer is swapped, so a lowered limit applies to spend already
  recorded rather than forgiving it.
- **Budget enforcement, cost tracking and rate limiting skipped every streamed request.**
  `LlmResponse::usage()` is always `None` for `ChatStream`; streams are now wrapped to fire accounting
  on completion.
- **Requests differing only in `tools`, `response_format`, `seed`, `logit_bias`, `tool_choice` or
  `extra_body` collided in the cache and were served each other's responses.** See the upgrade note
  above.
- The negative cache was dead code: it wrote keys with a std `DefaultHasher` over raw JSON while the
  read path used a seeded `ahash` over a curated field set, so an entry could never be read back.
  Both paths now share one `CacheKeyStrategy`.
- SSE stream truncation was reported as a clean EOF — silent data loss. Truncated lines and truncated
  UTF-8 codepoints now surface as errors, and transport errors go through the retry budget instead of
  propagating immediately. Anthropic 529 is treated as retryable and its `Retry-After` is honored.
- Gemini reasoning parts carry a `thought` flag but were concatenated into visible content; they now
  route to `reasoning_content`, and `reasoning_effort` maps to `thinkingConfig` instead of being
  dropped.
- `ImageUrl` and `AudioContent` are shared between request and response types but carried
  `deny_unknown_fields`, so a provider adding a field to its image or audio output hard-failed the
  whole response.
- The proxy routed to only one deployment and silently dropped the rest; per-key rpm, tpm and
  `budget_limit` were parsed from config and never enforced by anything.
- Idempotency keys are released by an RAII guard when a request is cancelled, instead of being
  stranded for the full 24h TTL.
- Metrics attribute caches were unbounded, so label cardinality grew without limit in a long-running
  proxy. The cost histogram was declared and never recorded to, which made any dashboard built on it
  read zero forever.
- Documentation examples are generated from every E2E fixture across the configured binding-language matrix, keeping
  examples correlated with executable coverage and making missing target renderers visible during generation.
- Dart: the native loader downloads and caches the library again on a cold cache. It only read
  the versioned cache and then threw a `StateError`, even though `nativeDownloadAndCacheLibrary()`
  was defined and exported for exactly that case. The loader also now searches for the
  `_dart`-suffixed cdylib that is actually built, opens every candidate by absolute path (a
  hardened runtime rejects a relative `dlopen`), and names the real environment variable in its
  error message instead of printing the identifier `$nativeLibDirEnv` literally. Fixed upstream in
  alef 0.55.6.

  Behavior change: an unresolvable native now throws a descriptive `StateError` naming the asset
  URL and the download command, where it previously returned `null` and let flutter_rust_bridge
  attempt its own relative-path `dlopen`.

[#174]: https://github.com/xberg-io/liter-llm/issues/174

## [1.16.0] - 2026-08-05

### Added

- **`[[guardrails]]` — the proxy can now actually enforce guardrails.** One registry is built at
  startup, layered into every model's Tower stack, and handed to the realtime WebSocket relay as the
  same `Arc`, so a rule cannot be live on one path and absent on the other. Supports `regex` (block
  or redact), `length_cap`, `prompt_injection`, and `cel` behind the `guardrail-cel` feature, each
  scoped to explicit `stages`. Nothing degrades: an invalid pattern or expression, an unknown or
  misspelled key, an empty `stages` list, a duplicate name, or a `cel` entry in a build without the
  feature all abort startup before the listener binds — a partially-applied set is indistinguishable
  from an unguarded proxy. `allow_list`/`deny_list` are deliberately not exposed yet; they key on a
  named metadata field and only `tenant_id` is populated, so any other field would fail silently.
  Guardrails are not hot-reloadable — the watcher warns rather than let a reloaded config describe a
  set the proxy is not running.

- Tool results can carry multimodal content. `ToolMessage.content` was `String`,
  so a tool returning an image had to describe it in prose or smuggle it through
  a separate user turn. It is now `UserContent` — the same untagged
  text-or-parts type user messages already use — with `Message::tool_result()`
  for the common text case, so existing `ToolMessage { content: "…".into() }`
  call sites keep working and a plain JSON string still deserializes ([#165]).

### Fixed

- Bedrock no longer silently drops non-text content parts. Converting a
  multi-part message kept only the text and discarded images without a trace;
  conversion is now shared with the other providers and warns on any part it
  cannot represent. This bug predates the change above.
- Go and Ruby bindings compile again. `LlmConfig.providers` was generated as both
  a struct field and a method — `field and method with the same name Providers`
  in Go, a duplicate method definition plus an RBS `DuplicatedMethodDefinition`
  in Ruby. Fixed upstream in alef 0.55.0, which also regenerates Swift and C#.
- The nightly model-catalog sync no longer reports failure after doing its job.
  It opened and merged the PR correctly, then died on `gh pr merge --auto`, which
  GitHub rejects once the PR is already mergeable ("clean status") — so the job
  went red on roughly every other run.
- CI runs poly's whole-project lint phase. It was skipped entirely because the
  shared validate job could install only Rust, Python and Java, so golangci-lint,
  rubocop, steep, dart-analyze and credo ran in the git hooks and never in CI —
  which is why the Go and Ruby breakage above reached `main` unnoticed.

### Changed

- **Python:** `config.providers` is an attribute again rather than a bound
  method. The generated getter was shadowed by a same-named method wrapper, so
  the attribute the type stub and the constructor keyword both promised did not
  exist at runtime. Any caller written against the accidental
  `config.providers()` spelling must drop the parentheses.

[#165]: https://github.com/xberg-io/liter-llm/issues/165

## [1.15.0] - 2026-08-04

### Added

- `LlmConfig`, a canonical, binding-friendly client configuration type
  (`client::LlmConfig`, re-exported at the crate root). Unlike `ClientConfig`
  and `FileConfig`, it is plain data (`String` / `Option<primitive>` /
  `HashMap` / `Vec` / plain sub-structs — no secrets, no trait objects, no
  `Duration`), derives `Serialize`/`Deserialize`, and is not excluded from
  generated language bindings. Field-name parity with the fields
  `model`, `api_key`, `base_url`, `timeout_secs`, `max_retries`, `temperature`,
  `max_tokens`, `load_env`, and `headers`, plus additive support for custom
  providers, cache, budget, rate-limit, cost tracking, tracing, cooldown,
  health-check, and AWS Bedrock configuration. Convert to a runtime client via
  `LlmConfig::into_client_builder`.
- `schema` cargo feature: derives `utoipa::ToSchema` on `LlmConfig` and its
  sub-structs for OpenAPI schema generation.
- AWS Bedrock is now fully configurable instead of environment-only:
  `ClientConfigBuilder::bedrock_region`, `bedrock_cross_region_prefix`, and
  `bedrock_credentials` set the region, cross-region inference profile
  prefix, and explicit AWS credentials on `ClientConfig`. `BedrockProvider`
  gained `from_config` and builder methods (`with_cross_region_prefix`,
  `with_credentials`) that fall back to the existing environment variables
  (`AWS_DEFAULT_REGION`/`AWS_REGION`, `BEDROCK_CROSS_REGION`,
  `AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY`/`AWS_SESSION_TOKEN`) when left
  unset, so existing env-driven setups are unaffected.

## [1.14.0] - 2026-08-04

### Added

- Streaming support for the Responses API: `create_response_stream` on the
  `ResponseClient` trait plus a `ResponseStreamEvent` type and a `stream` field
  on `CreateResponseRequest`, reusing the SSE machinery so Responses
  tool-calling apps get progressive output and streaming tool-call arguments
  (#162). Available in the Rust core crate; the language bindings are unchanged
  this release (the streaming event types are core-crate-only).

### Changed

- Regenerated all language bindings on alef 0.51.2.

### Fixed

- Ruby binding no longer publishes generated types into the global `Object`
  namespace (the `Parser` collision with the `parser` gem); types stay
  namespaced under the binding module (tree-sitter-language-pack #173, via
  alef 0.51.1).

## [1.13.0] - 2026-08-02

### Added

- Anthropic-compatible providers can now target a custom base URL (#159).
  `anthropic/`-prefixed models with a `base_url` override route to the
  Anthropic provider, so the Messages-API transforms (system extraction,
  content blocks, tool use, thinking) work against Anthropic-compatible
  endpoints such as DeepSeek's `https://api.deepseek.com/anthropic`.
- `ReasoningEffort` gains `Minimal` and `Max` variants (#160), covering
  DeepSeek's `max` thinking level and OpenAI's `minimal` level. Variants are
  appended, so existing FFI integer discriminants are unchanged.

### Changed

- `extra_body` is now shallow-merged into the request body root for
  OpenAI-compatible providers (#160), matching the OpenAI SDK. Top-level
  fields such as DeepSeek's `thinking` switch reach the wire instead of being
  sent as a literal `extra_body` object. The transport-controlled `stream`
  flag cannot be overridden via `extra_body`.
- Synced the model catalog from models.dev.
- Upgraded dependencies (`cargo upgrade --incompatible`).

## [1.12.2] - 2026-08-01

### Added

- `cargo binstall liter-llm-cli` support — prebuilt CLI binaries can now be installed
  directly from GitHub Releases without compiling from source. Adds
  `[package.metadata.binstall]` to the CLI crate plus a release-time `verify-binstall`
  CI job that installs via `cargo binstall`, smoke-tests the binary, and gates
  release-finalize.

### Changed

- Synced the model catalog from models.dev (#158).
- Updated dependencies.

## [1.12.1] - 2026-08-01

### Added

- MiniMax now exposes region-specific endpoints (#157): the global region
  (`https://api.minimax.io/v1` for the OpenAI protocol, `https://api.minimax.io/anthropic`
  for the Anthropic protocol) and the China region (`https://api.minimaxi.com/v1`,
  `https://api.minimaxi.com/anthropic`), each with its own docs root.

### Changed

- Regenerated the bundled model catalog, refreshing provider/model data and provenance.

### Fixed

- The ai-rulez-generated `plugin/.mcp.json` bundle is now committed instead of being
  swept up by the broad `.mcp.json` gitignore rule, so a fresh CI checkout no longer
  fails `ai-rulez verify --plugin` with a missing-file error.

## [1.12.0] - 2026-07-31

### Added

- `liter-llm-cli` installs a real OTLP export pipeline under the `otel` feature: it builds an OTLP
  span exporter + SDK tracer/meter providers, registers the W3C TraceContext propagator, and bridges
  `tracing` spans (and the library's `gen_ai.*` metric instruments) to OpenTelemetry via a
  `tracing-opentelemetry` layer. Export is activated at runtime by `OTEL_EXPORTER_OTLP_ENDPOINT`;
  without it the CLI falls back to the console subscriber. Both the `api` proxy server and the `mcp`
  server share this install path, and the server Docker image is built with `--features otel`.
  (Previously the `otel` feature only compiled the OTel API with no exporter, so nothing was
  exported standalone — the instruments only reached a collector when a host such as xberg-enterprise
  installed the providers.)

### Changed

- Raw `println!`/`eprintln!`/`print!`/`eprint!`/`dbg!` are denied in production code across the
  workspace (clippy `print_stdout`/`print_stderr`/`dbg_macro`); `tracing` is the sole diagnostic
  surface. Genuinely non-diagnostic output opts back in per site with `#[expect(...)]` (the
  interactive GitHub Copilot device-flow auth prompt and the CLI's top-level error report), and the
  `catalog-gen` dev tool keeps its stdout/stderr contract. Language bindings regenerated with alef
  0.48.12.
- **The `tracing` Cargo feature is removed; `tracing` is now an always-on dependency.** Spans and
  events compile unconditionally (near-zero cost with no subscriber installed), aligning with the
  org feature-flag contract that tracing is never optional. `otel` remains the opt-in OTLP export
  layer.

### Fixed

- Streaming no longer aborts on a trailing metadata-only SSE event that omits `id`. OpenAI-compatible
  providers such as OpenCode Zen/Go (`https://opencode.ai/zen/go/v1`) emit an `inference-cost` event
  with empty `choices` and no `id`/`object`/`created`/`model` immediately before `[DONE]`; these
  header fields on `ChatCompletionChunk` are now `#[serde(default)]`, so the event decodes to an
  empty chunk instead of failing with `missing field 'id'`. (#155)

## [1.11.5] - 2026-07-30

### Changed

- Upgrade the MCP server to `rmcp` 3.0, adopting the MCP 2026-07-28 protocol
  revision and its sessionless streamable-HTTP transport. The existing
  `validate_api_key` middleware and per-request `KeyContext` auth model are
  retained.

## [1.11.4] - 2026-07-28

### Fixed

- **SSE streams no longer abort with "invalid UTF-8" when a provider splits a
  multi-byte character across HTTP chunks.** The SSE parser now buffers the
  trailing bytes of a codepoint that lands on a chunk boundary and reassembles it
  with the next chunk, instead of treating the incomplete sequence as corruption
  and terminating the stream. Common with CJK text, accented characters, emoji, or
  any sufficiently long response. Genuinely malformed UTF-8 still errors. (#152)

### Changed

- Refresh the model catalog from models.dev (now 165 providers).
- Refresh `Cargo.lock` to the latest compatible dependency versions.

## [1.11.3] - 2026-07-27

### Changed

- Refresh the model catalog from models.dev.
- Regenerate all language bindings on alef 0.48.4, which lowers the Maven
  enforcer floor (fixes Java/Maven publishing) and generates the C#
  `runtime.json` template (fixes C#/NuGet publishing).
- Upgrade dependencies to their latest incompatible versions (base64
  0.22 → 0.23).

## [1.11.2] - 2026-07-26

### Changed

- Regenerate all language bindings on alef 0.48.2.
- Update dependencies to their latest compatible versions.

### Removed

- Remove unused Java PMD ruleset and stale linter configuration.

## [1.11.1] - 2026-07-26

### Fixed

- **`list_models()` on providers that omit `object`.** `ModelsListResponse.object`
  is now `#[serde(default)]`, so listing models no longer fails with
  `missing field 'object'` against OpenRouter and other OpenAI-compatible
  providers that omit the top-level `"object": "list"` field from `/v1/models`.
  (#149)

### Changed

- **Dependency upgrades.** pnpm 11.15 → 11.17, Jackson (databind/jdk8)
  2.21.2 → 2.22.1, and a `sorbet-runtime` bump.

## [1.11.0] - 2026-07-23

### Added

- **OpenCode Zen and OpenCode Go providers** — now 165 total (up from 163).
  OpenCode Zen routes via the `opencode/` model prefix
  (`https://opencode.ai/zen/v1`); OpenCode Go routes via the `opencode-go/`
  model prefix (`https://opencode.ai/zen/go/v1`). Both are OpenAI-compatible,
  bearer-authenticated (`OPENCODE_API_KEY`), and support chat completions.
- **`reasoning_content` support** — `StreamDelta` and `AssistantMessage` now
  carry an optional `reasoning_content` field, so reasoning/thinking tokens
  from OpenAI-compatible providers (DeepSeek R1, Qwen, etc.) are preserved in
  both streaming and non-streaming responses instead of being dropped. Anthropic
  extended-thinking blocks are mapped into the same field. `AssistantMessage`
  gains a `reasoning_text()` accessor.

### Changed

- **base64 embedding responses.** `EmbeddingObject.embedding` now accepts either
  a JSON float array or a base64 string of little-endian `f32` bytes (the
  OpenAI-compatible `encoding_format: "base64"` response), decoding base64
  vectors that previously failed to deserialize. Payloads are ~2.3× smaller on
  the wire and decode ~15× faster via a zero-copy `Visitor`. The element type
  narrows from `Vec<f64>` to `Vec<f32>` (float32 is what providers send on the
  wire); the high-level `embed()` API was already `f32`, so only code reading
  the raw `EmbeddingObject.embedding` field as `f64` needs a one-line
  adjustment.

## [1.10.1] - 2026-07-20

### Fixed

- **Swift**: tool-call responses now decode correctly — the `type` field on
  tool calls honors its wire name instead of throwing `keyNotFound`.
- **Swift**: transcription `segments`, `tool_calls`, and other optional list
  fields on response DTOs now decode instead of throwing
  `DecodingError.typeMismatch`.
- **Elixir**: streaming entry points now call `chat_stream/2` (a non-existent
  `chat_stream_async/2` was generated previously), fixing
  `UndefinedFunctionError`.
- Regenerated all language bindings with alef 0.38.4.

## [1.10.0] - 2026-07-19

### Added

- **Public `ModelInfo` DTO and `cost::model_info(name)` accessor**, exposing
  per-model metadata — context window, max input/output tokens, mode, and
  capability flags (vision, reasoning, tool calling, structured output, audio) —
  across all language bindings.
- **Opt-in runtime catalog refresh.** `cost::refresh_catalog(&CatalogRefreshConfig)`
  overlays an updated `catalog.json` on top of the embedded catalog at runtime,
  without a rebuild. It is a **runtime toggle, not a Cargo feature**, and is
  disabled by default (`enabled: false`). A failed, unreachable, or air-gapped
  refresh always falls back to the embedded (or prior) catalog, so a refresh
  failure never degrades availability. Also exposed: `RefreshOutcome`,
  `CatalogRefreshError`, `install_catalog_overlay_from_str`,
  `clear_catalog_overlay`, and `DEFAULT_CATALOG_URL`.
- **Extended per-model pricing.** The catalog now carries context-tiered pricing
  (e.g. distinct rates above a 200k-token context), plus audio and reasoning
  token costs, all consumed by `completion_cost`. New `ModelTier` type.
- **20 new providers — now 163 total (up from 143)** — including Black Forest
  Labs, Reducto, Soniox, Nvidia Riva, Google Distributed Cloud (Gemini), and AWS
  Bedrock Mantle.
- **Rust catalog generator (`liter-llm-catalog-gen`)** replacing the Python
  `generate_pricing.py` / `generate_providers_doc.py` scripts. It fetches and
  strictly validates the models.dev `api.json` (rejecting unknown fields so
  upstream drift fails loudly) and dual-writes `catalog.json` with a
  `$provenance` block (source SHA-256, fetch date, library version). New tasks
  `generate:catalog` and `generate:catalog:check`.
- **Catalog automation workflows.** A daily `sync-catalog` workflow opens an
  auto-merging PR when models.dev drifts; `publish-catalog` pushes catalog
  snapshots to a rolling `model-catalog` release; `catalog-verify` gates PRs that
  touch the catalog.

### Changed

- **Renamed the embedded pricing registry from `pricing.json` to `catalog.json`**
  (both the canonical `schemas/` copy and the crate's `crates/liter-llm/schemas/`
  copy), reflecting its expanded per-model metadata.
- **Upgrade dependencies:** `jsonschema` 0.46 → 0.48, `rmcp` 2.1 → 2.2,
  `tokio-tungstenite` 0.29 → 0.30. Regenerate all bindings with alef 0.38.0.

### Fixed

- **`list_models()` no longer fails for providers that omit `created`** (e.g.
  DeepSeek). `ModelObject.created`, `object`, and `owned_by` now default when
  absent from a provider's `/v1/models` response. Fixes #139.

## [1.9.3] - 2026-07-04

### Changed

- **Upgrade all dependencies to latest and regenerate all bindings** with alef
  0.31.0. Notable major bumps: `rmcp` 1.7 → 2.1, `etcd-client` 0.15 → 0.19,
  `tokio-tungstenite` 0.26 → 0.29, `tower-http` 0.6 → 0.7, `sha2` 0.10 → 0.11,
  `cel-interpreter` 0.8 → 0.10, and `tikv-jemallocator` 0.6 → 0.7.

### Fixed

- **CEL guardrail: adapt to `cel-interpreter` 0.10.** `Program::compile` now
  returns `ParseErrors`, and the new ANTLR-based parser can panic on some
  malformed input. `CelGuardrail::new` now catches those panics and returns a
  `CelCompileError`, so an invalid expression can never abort the caller (e.g.
  proxy startup).
- **Proxy MCP server: adapt to `rmcp` 2.1 model changes** (`RawResource` →
  `Resource`, `RawResourceTemplate` → `ResourceTemplate`, `Content` →
  `ContentBlock`, `PromptMessageRole` → `Role`).
- **Proxy etcd config watcher: adapt to `etcd-client` 0.19,** whose `watch` now
  returns a single `WatchStream` and moves `cancel` onto the stream.

## [1.9.2] - 2026-07-02

### Fixed

- **Ruby: allow Ruby 4.x.** Drop the `< 4.0` upper bound on the gem's
  `required_ruby_version` (now `>= 3.2.0`), so installing/updating the gem no
  longer fails on Ruby 4.0+. Fixes #136.

## [1.9.1] - 2026-07-02

### Changed

- **Upgrade dependencies and regenerate all bindings** with alef 0.30.11.
- Add `[workspace] extra_clippy_allows = ["single_match", "collapsible_match"]` to
  `alef.toml` so the generated single-variant enum-from-int bindings pass
  `clippy -D warnings`.

### Fixed

- Correct the **HashiCorp** vendor name spelling in the proxy Vault secret backend
  (`HashiCorpVaultProvider`/`Builder`), its comments, docs, and this changelog.

## [1.9.0] - 2026-06-27

Stable release promoting 1.9.0-rc.2 (fully published green). No code changes
beyond the version bump.

## [1.9.0-rc.2] - 2026-06-27

### Changed

- **Rebrand to Xberg.** All published package coordinates move off the `kreuzberg` brand: npm scope `@kreuzberg/*` → `@xberg-io/*` (main, the six napi platform packages, `-wasm`, and `-cli`); the Java/Maven and Kotlin Android namespace `dev.kreuzberg.literllm[.android]` → `io.xberg.literllm[.android]` (group id `dev.kreuzberg` → `io.xberg`), which relocates the generated Java/Kotlin source trees from `dev/kreuzberg/…` to `io/xberg/…`. Docs, badges, the ecosystem block (now links the `Xberg` product at `github.com/xberg-io/xberg`), the brand heading ("Part of Xberg.io"), and CI publishing references (`owner: xberg-io`, `ghcr.io/xberg-io`, packagist `xberg-io`, the `xberg-dev-publisher` app, `xberg-io/homebrew-tap`) are updated to match. The legal entity name "Kreuzberg, Inc." is unchanged. (`alef.toml`, templates, generated bindings, docs, `publish.yaml`)
- **Regenerate with alef 0.29.3** (from 0.29.1): correct Kotlin Android Maven coordinate, drop redundant `-node` npm alias.

### Removed

- **Drop the unused legacy `packages/typescript` wrapper package.** It was a re-export barrel of the native `@xberg-io/liter-llm` binding and was never published. alef no longer generates it (the napi backend stopped emitting `packages/typescript`); the canonical TypeScript surface is the native package's bundled `index.d.ts`. Removed the directory and its references in the workspace list, `dependabot.yaml`, the `validate-versions` manifest path (now `crates/liter-llm-node/package.json`), the `node:typecheck` task, and the oxlint exclude.

## [1.8.2] - 2026-06-23

### Fixed

- **Publish (zig): the published zig package compiles again.** The "Stage FFI artifacts" step bundled a stale top-level `crates/liter-llm-ffi/liter_llm.h` (old ABI, missing `literllm_create_client`/`*_from_json`) into the `ffi-*` artifact, so the zig wrapper's `@cImport` resolved against an out-of-date header and every test failed to compile. The step now stages the canonical `include/liter_llm.h` (and fails hard if absent); the orphaned top-level header is removed.
- **Publish (kotlin-android): the AAR no longer corrupts on download.** Both ABI matrix legs emitted the same filename `liter-llm-android-release.aar`; the publish job's `download-artifact … merge-multiple: true` collided them into one torn zip (`bad zipfile offset`). Each leg now uploads an ABI-suffixed `liter-llm-android-<abi>.aar`.
- **Publish (swift): a rerun can no longer desync the artifact-bundle checksum.** The swift-artifactbundle job now reuses an existing release asset and its checksum instead of rebuilding a non-reproducible bundle, keeping the hosted bundle consistent with the `swift-<version>` tag.
- **Swift test-app scaffold pins the checksum-bearing ref.** Regenerated from alef 0.26.7: the registry-mode `Package.swift` now uses `.package(url:, branch: "release/swift/<version>")` (which carries the substituted checksum) instead of `from: "<version>"` (which resolved the placeholder-bearing SemVer tag).

### Changed

- **Bindings regenerated from alef 0.26.7** (from 0.26.5): swift e2e scaffold branch pin and kotlin-android Gradle plugin/dependency bumps. Generated node e2e/test_apps emit `.npmrc` disabling frozen-lockfile.

## [1.8.1] - 2026-06-23

### Fixed

- **CI E2E (kotlin_android): the host-JVM test project compiles again.** The generated `MockServerListener` implements the JUnit Platform `LauncherSessionListener` SPI (referencing `LauncherSession`/`LauncherSessionListener` at compile time), but the e2e `build.gradle.kts` scoped `junit-platform-launcher` as `testRuntimeOnly`, so `compileDebugUnitTestKotlin` failed with "Unresolved reference 'launcher'". Regenerated from alef 0.26.5, which now scopes it `testImplementation`.
- **CI E2E (swift): the e2e link step now finds `liter_llm_ffi`.** The swift e2e `before` step built `liter-llm-swift` and the mock server but never `liter-llm-ffi`, so `libliter_llm_ffi.a` was absent from the linker search path (`ld: library 'liter_llm_ffi' not found`). The `[crates.test.swift].before` step now also runs `cargo build --release -p liter-llm-ffi`.
- **CI E2E (node): the lockfile is back in sync.** `pnpm-lock.yaml` was stale against `crates/liter-llm-node/package.json` (missing `@napi-rs/cli ^3.6.2`) after the 1.8.0 bump, failing `pnpm install --frozen-lockfile`. Regenerated.

### Changed

- **Bindings regenerated from alef 0.26.5** (from 0.26.3): Swift bridge-glue re-materialization runs after the bridge crate is built, opaque-handle aliasing avoids capsule import collisions, and JSON-string overloads emit positional arguments for underscore-prefixed parameters.

## [1.8.0] - 2026-06-22

### Added

- **MCP server: tool annotations on every tool.** Each MCP tool now advertises rmcp `ToolAnnotations` (a human-readable title plus `readOnlyHint`/`destructiveHint`/`idempotentHint`/`openWorldHint`) so clients can present them and decide auto-approval. Query tools are read-only; `create_*` mutate without being destructive; `delete_*`/`cancel_*` are destructive and idempotent; all reach external providers (`openWorldHint`).
- **MCP server: prompts, resources, and argument completion.** Beyond tools, the server now exposes reusable prompt templates (`summarize`, `translate`, `extract`), catalog resources (`liter-llm://models`, `liter-llm://providers`, and the `liter-llm://pricing/{model}` / `liter-llm://provider/{name}` templates), and argument completion for `model` (from the configured models) and provider `name` (from the registry). `get_info` advertises tools, prompts, resources, and completions.

### Fixed

- **Budget middleware: concurrent spend is no longer lost across a window rollover.** On weakly-ordered architectures (arm64) the window-reset path could drop a racing `fetch_add`; the rollover now subtracts the snapshotted prior total instead of storing zero, preserving every concurrent charge.
- **CI: the Kotlin Android build provisions the wrapper-declared Gradle version** (`gradle-version: wrapper`) in `ci-mobile` and `ci-e2e`. AGP 9.2.0 requires Gradle >= 9.4.1; the previous hardcoded `8.13` pin silently overrode the 9.6.0 wrapper and broke `assembleRelease`.
- **Swift & Dart: the untagged content-union `text()` accessor now references the generated payload field** instead of a non-existent binding, mirroring the 1.7.6 Kotlin fix. Regenerated from alef 0.26.3.
- **CI/release: create the draft GitHub Release once in the `prepare` job** so the Swift artifact-bundle upload (the only upload job without its own ensure-release step) can no longer race ahead of release creation and fail with "release not found". Mirrors the html-to-markdown publish pipeline.

### Changed

- **Bindings regenerated from alef 0.26.3** (from 0.25.60): Swift capsule-pointer bridging via `usize` plus `.product` dependency wiring, `RustBridgeC.h` preserved across `alef all --clean`, and the content-union accessors above.

## [1.7.6] - 2026-06-22

### Fixed

- **Kotlin Android: the untagged content-union `text()` accessor now references the generated `value` property instead of the non-existent `field0`**, fixing a `:compileReleaseKotlin` failure (`Unresolved reference 'field0'`) that broke the Android AAR build. Regenerated from alef 0.25.60; the legitimate `field0` data-class properties (e.g. `LiterLlmError.Serialization`) are unaffected.

## [1.7.5] - 2026-06-21

### Changed

- **npm: the primary package is now the bare `@kreuzberg/liter-llm`** (matching the crawlberg convention), renamed from `@kreuzberg/liter-llm-node`. The napi `packageName` and all per-platform sub-packages use the bare prefix (`@kreuzberg/liter-llm-<rid>`); the `.node` `binaryName` stays `liter-llm-node`. (`alef.toml`, generated node binding, all README badges, docs, `pnpm-lock.yaml`)

### Fixed

- **Packagist: publish under the canonical `xberg-io/liter-llm` vendor** (matching html-to-markdown, crawlberg, tree-sitter-language-pack) instead of the legacy `kreuzberg/liter-llm`. Fixes the registry-check coordinate and the `publish-packagist` step (`packagist-username`, `package-name`) in `publish.yaml`, plus the stale reference in `llms.txt`. The Maven registry-check coordinate is corrected to `dev.kreuzberg.literllm:liter-llm`.
- **CI/release: `@kreuzberg/liter-llm-cli` now publishes on the main release** at the release version (previously decoupled on a separate `cli-proxy-v*` tag), gated behind the CLI-binary upload so the npm wrapper never ships ahead of the binaries it downloads.
- **CI/release: disable sccache for release build jobs** so a transient sccache cache/DNS failure can no longer block a publish run (`publish.yaml`).
- **docs: refresh stale install snippets** — Java/Kotlin/Swift/Zig version pins, the Java Maven coordinate, and the Elixir version range in `docs/getting-started/installation.md`, `docs/index.md`, and `llms.txt`.

## [1.7.4] - 2026-06-19

### Changed

- **chore(precommit,alef): standardize kotlin-android formatting on ktfmt --kotlinlang-style.** Drop the conflicting prek ktlint hook (it ran a destructive `--format` that fought ktfmt and rewrote alef's `///` doc comments), scope ktfmt to `packages/kotlin-android` with `--kotlinlang-style`, switch `alef.toml` kotlin format/check from gradle-ktlintFormat to ktfmt so alef and prek agree, and exclude the vendored Gradle wrapper from shellcheck. detekt remains. (`.pre-commit-config.yaml`, `alef.toml`)

### Fixed

- **Content-union e2e gate completed across the remaining bindings** (regenerated with alef 0.25.49). The Dart binding now injects the `AssistantContent.text()` extension even when FRB emits the freezed mixin clause (`sealed class … with _$… {`); the PHP flat-enum conversion always emits an exhaustive wildcard for the `&str` tag match, so `alef(skip)`'d-`Default` enums (`Message`, `ContentPart`, `AssistantPart`, `OcrDocument`) compile; and the Node/NAPI binding keeps both `From` conversion directions for plain data enums used as struct fields (e.g. `AuthHeaderFormat` in `CustomProviderConfig`). Local e2e green for Python, Node, WASM, Elixir, and PHP.

### Documentation

- **Bindings parity docs.** Added a feature-by-language support matrix (chat, streaming, tool calling, embeddings, multimodal in/out, call idiom) across all 14 native bindings plus the C/FFI surface, expanded the multimodal cookbook with idiomatic examples in nine languages, and reconciled the binding count to a single "14 native bindings + C/FFI surface" across the README and docs (added Dart, Swift, Kotlin Android, and Zig to the README Language READMEs table).

## [1.7.3] - 2026-06-19

### Fixed

- **Cross-language e2e for multimodal `message.content`.** The remaining bindings can now
  string-assert the assistant content union (`AssistantContent`), fixing the e2e suites that
  broke on it: Kotlin and Dart get a `text()` accessor on the sealed class; WASM returns the
  display text (`String`) instead of a discriminant; Swift renders property access for
  first-class result structs; Elixir reads the NIF struct's `.text`; PHP calls the message's
  `text()` accessor. Generated via alef 0.25.48 (`untagged_union_text_types` +
  `fields_display_as_text` extended to all backends).
- **Android AAR packaging guard** — the Kotlin/Android publish now stages the cross-compiled
  JNI `.so` libs into the AAR and fails loudly if `jni/` would be empty, so a jni-less AAR can
  never be published.

## [1.7.2] - 2026-06-18

### Added

- **`Display` for `AssistantContent`** — renders the message text (`Text` variant verbatim; `Parts` variant concatenates its text segments, skipping non-text parts), enabling string assertions on `message.content` across the polyglot e2e suites.

### Fixed

- **Cross-language e2e content assertions** — the generated e2e suites stringified `choices[0].message.content` with plain-string casts that fail for the multimodal `AssistantContent` union (Go `string(*ptr)`, Java `Objects.toString`, C# `.ToString()`, Rust `.as_deref()`). alef 0.25.45's `fields_display_as_text` config now emits the per-language text accessor so the assertions compile and assert text content.

## [1.7.1] - 2026-06-18

### Fixed

- **FFI build under `-D warnings`** — the regenerated `liter-llm-ffi` crate referenced `#[cfg(feature = "wasm-http")]` without declaring the feature, producing `unexpected cfg condition value: wasm-http` errors that broke CI Rust, CI E2E (Build FFI), CI Mobile, and the crates.io publish in v1.7.0. alef now declares non-default passthrough features via the configurable `[crates.ffi] extra_features` key, so `wasm-http` is declared (but not enabled) and survives regeneration.
- **Java PMD on generated DTOs** — the new `DecodedDataUrl { mime, byte[] data }` value object tripped `ArrayIsStoredDirectly` / `MethodReturnsInternalArray`; alef's PMD ruleset now excludes these for generated DTOs.
- **Docs strict build** — fixed a broken intra-doc link in `docs/usage/multimodal.md` (`../concepts/providers.md` → `../providers.md`).

## [1.7.0] - 2026-06-18

### Added

- **Typed multimodal builders** — `liter_llm::image::{encode_data_url, decode_data_url, DecodedDataUrl}` with `IMAGE_PNG`/`IMAGE_JPEG`/`IMAGE_WEBP`/`IMAGE_TIFF` MIME constants. `decode_data_url` returns a named `DecodedDataUrl { mime, bytes }` struct rather than a tuple so polyglot bindings extract it as a typed object.
- **`Message::user_with_parts(parts)`** — ergonomic constructor for multimodal user messages.
- **`ContentPart::{text, image_data_url, image_url, image_with_detail, image_png, image_jpeg, image_webp, image_tiff}`** — typed constructors replacing hand-rolled struct construction.
- **`ResponseFormat::{json_schema, json_object, text}`** + **`JsonSchemaFormat::new(name, schema).strict(bool).description(d)`** fluent builder. `new` defaults to `strict = Some(true)`. Provider-mapping rustdoc on `ResponseFormat` (OpenAI passthrough, Gemini/Vertex `responseMimeType`+`responseSchema`, Anthropic system-instruction injection).
- **Multimodal output**: `AssistantContent` enum (`Text` / `Parts`) with `#[serde(untagged)]` back-compat; `AssistantPart` (`Text` / `Refusal` / `OutputImage` / `OutputAudio`) with `#[serde(tag = "type", rename_all = "snake_case")]`; `AssistantMessage::{text, refusal_text, output_images, output_audio}` accessors; `Message::{assistant_with_parts, system_with_parts}` constructors; `ChatCompletionRequest.modalities: Option<Vec<Modality>>` with `Modality::{Text, Audio, Image}`. Vertex `transform_response` preserves `inline_data` as `OutputImage`/`OutputAudio` (no base64 re-encode); OpenAI `transform_response` hoists `message.audio` into `Parts([Text(transcript), OutputAudio])`.

### Changed

- **BREAKING**: `AssistantMessage.content: Option<String>` → `Option<AssistantContent>`. Back-compat via `From<String>`/`From<&str>` for `AssistantContent` and the untagged serde variant — providers returning scalar `content` strings still deserialize as `Text(_)`.
- **BREAKING**: `SystemMessage.content: String` → `UserContent`. Back-compat via `From<String>`/`From<&str>` for `UserContent`.
- `liter-llm` makes `base64` an unconditional dependency (previously gated behind `native-http`/`wasm-http`) — `liter_llm::image::*` helpers are transport-agnostic.
- All polyglot bindings regenerated for the multimodal surface (alef 0.25.40 pin). PHP/WASM extraction of `#[serde(untagged)]` enums (`AssistantContent`) and nested complex enum-variant types (`OutputImage { image_url: ImageUrl }`, `OutputAudio { audio: AudioContent }`) is resolved upstream; bindings construct/access multimodal types via each language's native idiom.

## [1.6.4] - 2026-06-17

### Changed

- Gate `etcd-client` behind optional `etcd-watch` feature; default CLI builds no longer require `protoc`. Resolves the Homebrew bottle source-build failure on v1.6.3.
- Bump `alef` codegen pin to 0.25.24 (swift opaque-handle class triples, dart `() -> ()` cleanup, kotlin per-file ktfmt invocation, java PMD/palantir-java-format alignment, c e2e FFI tarball-name alignment, php `exclude_functions` filtering through to user-facing wrapper).
- `liter_llm::tower::metrics::global_meter()` is now `pub` so downstream crates (`liter-llm-proxy`) can read the shared OTel meter without re-initialising.

### Fixed

- **`ContentPart` crate-root shadow** — `src/lib.rs` did `pub use types::*` (which brings `types::ContentPart` to the crate root) and then explicitly `pub use realtime::ContentPart`, causing the realtime variant to shadow the types one. Downstream consumers writing `use liter_llm::ContentPart;` received the realtime variant, which has no `ImageUrl` variant, producing `E0599: no variant named 'ImageUrl' found for enum 'liter_llm::ContentPart'` in any VLM-OCR call site. The realtime `ContentPart` re-export is removed from `lib.rs`; callers that need it must import it explicitly as `liter_llm::realtime::ContentPart`. A compile-time regression test in `src/tests.rs` asserts that `crate::types::ContentPart::ImageUrl` is constructible through the crate root.
- **rustdoc ICE in `liter-llm`** — bracketed intra-doc links to private items (`MAX_POOL_BUFFER_CAPACITY`, `post_stream_with_cancel`, `post_json`, `post_json_raw`, `retry::should_retry`, `LiterLlmError`, `ConfigDrivenProvider::transform_request`) in `http/`/`provider/` triggered an internal compiler error on rustc 1.95.0 during `doc_link_resolutions`. Stripped the link brackets so the doc strings reference the names as plain code identifiers. Also fixed bare URL in `CustomProviderConfig::base_url`.
- **Missing-docs cleanup** — added documentation for `UsageSinkErased` trait and method, the four `tenant` modules (`context`, `etcd`, `in_memory`, `resolver`), the `http::transport` module, and the `RouterError::Discover { source }` struct field.

## [1.6.3] - 2026-06-16

### Fixed

- **WASM build: `tokio::time` inside `wait_for_batch_impl`** — `crates/liter-llm/src/client/mod.rs` polled with `tokio::time::Instant` and `tokio::time::sleep` unconditionally, but `tokio` is gated behind the `native-http` feature. Under `wasm-http`-only builds (the WASM crate) compilation failed with `E0433: cannot find module or crate 'tokio'`. The function now switches to `web_time::Instant` + `gloo_timers::future::sleep` on `target_arch = "wasm32"`, with `web-time` added as an optional dep on the `wasm-http` feature.
- **WASM build: leaked tower DTOs** — alef regen at v1.6.0 started emitting `From<liter_llm::tower::{CircuitState, HealthStatus, IntentPrototype, SingleflightResult}>` impls in `crates/liter-llm-wasm/src/lib.rs`, but `tower` is not enabled under `wasm-http`. Added the four types to `alef.toml` `[crates.wasm].exclude_types` alongside the existing config-DTO exclusions.
- **Ruby gem build: missing feature declarations** — `packages/ruby/ext/liter_llm_rb/native/Cargo.toml` did not declare `native-http`/`opendal-cache`/`wasm-http`, so alef-emitted `#[cfg(feature = "native-http")]` gates in `src/lib.rs` resolved as false and the `ensure_crypto_provider` call site failed with `E0425: cannot find value in this scope`. Regen now writes the features section. Affected every Ruby gem variant (linux-x86_64, linux-aarch64, macos-arm64).
- **PHP windows builds: `ext-php-rs` macro lookup** — `LiterLlmApi::ensure_crypto_provider` resolution failed inside `#[php_impl]`-generated code on `target_os = "windows"` (`E0599: no associated function or constant`). Other bindings (PyO3, NAPI, Rustler) handle the same pattern correctly, so this looks like an ext-php-rs upstream gap. Excluded `ensure_crypto_provider` from the PHP binding entirely via `alef.toml` — the function is a no-op on Windows anyway, and downstream PHP users rely on transitive invocation from internal `reqwest::Client` constructors.
- **PHP PIE matrix: `php8.5` macos-arm64** — `shivammathur/setup-php@v2` does not yet ship a PHP 8.5 image for macOS arm64 runners (PHP 8.5 was released November 2025). Excluded that matrix cell from `publish.yaml` until the upstream action catches up. Linux + Windows PHP 8.5 builds remain in the matrix.

## [1.6.2] - 2026-06-16

### Fixed

- **`to_singleflight_error` dead-code lint** — gated `LiterLlmError::to_singleflight_error` with `#[cfg(feature = "tower")]` so the method is not emitted when `cargo publish --verify` builds with default features (`default = ["native-http"]`). The method's only call sites live in `src/tower/cache_singleflight.rs`, which is itself feature-gated. Under `-D warnings` the dead-code lint rejected the build, blocking `Publish crates.io` and every per-platform `Build CLI binary`, `Build WASM`, and `Build Kotlin Android natives` job in the v1.6.0 and v1.6.1 release workflows. No artifacts reached PyPI, crates.io, npm, RubyGems, Maven Central, Packagist, Hex, NuGet, pub.dev, or the Homebrew tap from those tags.
- **Release-runner `protoc` toolchain dep** — `xberg-io/actions/setup-rust@v1.8.70` now installs `protobuf-compiler` on every Linux/macOS/Windows runner. `etcd-client v0.15`'s `build.rs` shells out to `protoc`; `liter-llm-cli` pulls `etcd-client` transitively through `liter-llm-proxy`, so the v1.6.1 CLI binary builds panicked with `Failed to compile proto files: Could not find protoc`. v1.6.2's publish workflow consumes the floating `v1` tag which now carries the fix.

## [1.6.1] - 2026-06-16

### Fixed

- **FFI crate `wasm-http` feature** — declared `wasm-http` as a no-op feature on `liter-llm-ffi/Cargo.toml` so the alef-emitted `#[cfg(any(feature = "native-http", feature = "wasm-http"))]` gates in `crates/liter-llm-ffi/src/lib.rs` resolve under `cargo build -D warnings`. v1.6.0's publish workflow failed on every Rust FFI, CLI, WASM, and Kotlin Android native build with `unexpected_cfgs` errors because the gate was emitted without the corresponding feature declaration.

## [1.6.0] - 2026-06-16

### Bindings

- **All 14 language bindings regenerated** against alef `0.25.18`. Covers Python (PyO3), TypeScript (NAPI-RS), Ruby (Magnus), PHP (ext-php-rs), Elixir (Rustler), Go (cgo), Java (Panama FFM), C# (P/Invoke), Swift (swift-bridge), Dart (flutter_rust_bridge), Kotlin Android (JNI), Zig, C, and WASM.
- **Tier A/B/C API triage** applied to the v1.6.0 surface: internal types stay Rust-only (Tier A), trait-generic helpers like `wait_for_batch_impl` and the `BatchRetriever` trait stay Rust-only (Tier B), and binding-exposed concrete methods like `DefaultClient::wait_for_batch` cross the FFI boundary (Tier C). Tier B trait methods (`BatchRetriever::fetch_batch_for_polling`) now emit correctly in JNI without a workaround.
- **Swift `chat_stream` streaming adapter** restored across all bindings. The swift-bridge extern block emitted by alef now declares `type DefaultClient;` inside the streaming extern block so the owner reference resolves.

### Tooling

- alef bumped from `0.23.7` → `0.25.18`. New upstream fixes consumed:
  - JNI backend emits per-trait `use core_crate::{path};` clauses for non-root trait methods.
  - swift-bridge streaming-adapter extern block declares owner types alongside handle types.
  - `cleanup_orphaned_files` recognises hash-less stale files (self-referential `// auto-generated by alef` header).
  - Dart and PHP `cfg`-feature forwarding fixes for the `wasm-http` feature gate.

### Added

- `KeyContext::tenant_id: TenantId` — tenant identifier carried in every resolved auth context. Master-key auth always sets `TenantId("master")` (see `MASTER_TENANT_ID` constant); virtual-key auth propagates the `tenant_id` returned by the configured `KeyResolver`.
- `KeyContext::from_resolved(key_id, &ResolvedKey)` — constructor that builds a `KeyContext` from a `KeyResolver`-resolved record, preserving the canonical `tenant_id` from the resolver rather than falling back to the raw key token.
- `auth::MASTER_TENANT_ID: &str` — well-known constant (`"master"`) used by `KeyContext::master()` so downstream budget and usage layers can identify master-key traffic without a special-case enum.

### Changed

- `KeyContext` now carries `tenant_id: TenantId` resolved by the configured `KeyResolver`. Master-key auth resolves to `TenantId("master")`.
- Every proxy HTTP handler now propagates `tenant_id` into `LlmRequest::with_tenant_id`, activating `BudgetLedger::Tenant`, `TenantScopedStrategy`, and `UsageEvent.tenant_id` for all in-proxy traffic. The `dispatch` helper in `routes/mod.rs` applies `with_tenant_id` centrally; `chat.rs` (which bypasses `dispatch` for stream/non-stream branching) sets it inline.
- `validate_api_key` middleware now resolves virtual keys via `AppState.key_resolver.resolve()` (previously used `KeyStore.get()` directly), so custom `KeyResolver` backends injected via `ProxyServer::with_key_resolver` receive calls for all virtual-key traffic.

- `ProxyServer::with_key_resolver` and `ProxyServer::with_usage_sink` builder methods for embedder-supplied dependencies. Default behaviour unchanged.
- `AppState.usage_sink: Option<Arc<dyn UsageSinkErased>>`; `HooksLayer` is now wired outermost in the Tower stack when a sink is configured.
- `UsageEvent.effective_model: Option<String>` — provider-echoed model name from the response, distinct from `model` which is the requested name. Populated for `Chat`, `Embed`, `Moderate`, `Ocr`, and `Search` response variants; `None` for streaming, speech, transcription, rerank, image generation, and list-models variants, and on error/timeout paths.
- `UsageEvent.cache_state` is now accurate. Previously hardcoded `Bypass`; now reflects `Miss`, `ExactHit`, `SemanticHit`, or `StaleHit` set by `CacheLayer` and `SingleflightService` (followers) via a `tokio::task_local!` cell read by `HooksLayer` after the inner service resolves. Requests with no `CacheLayer` in the stack continue to report `Bypass`.
- `DefaultClient::wait_for_batch(batch_id, WaitForBatchConfig)` — poll a batch until terminal status (Completed, Failed, Expired, Cancelled) with exponential backoff, configurable intervals, and optional timeout. Returns `Ok(BatchObject)` on completion, `Err(BatchWaitError::{Failed, Timeout, Client})` on failure.
- `WaitForBatchConfig` — configuration struct with `initial_interval_secs: f64`, `max_interval_secs: f64`, `backoff_multiplier: f32`, and optional `timeout_secs: Option<f64>` fields; implements `Default` with 5.0s initial, 60.0s max, 1.5x multiplier, no timeout. All fields use `f64` seconds (not `Duration`) for FFI bridgeability.
- `BatchWaitError` — error enum with struct-style variants `Failed { status: BatchStatus }`, `Timeout { timeout_secs: f64 }`, and `Client { message: String, code: u32 }` for batch polling failures. All fields are FFI-friendly primitives.
- `liter_llm::observability::{UsageEvent, UsageSink, CacheState, UsageEventOutcome, UsageSinkError, LoggingUsageSink, MultiUsageSink}` — canonical per-request usage events with pluggable sinks. `UsageEvent` is billing/observability-vendor-agnostic; downstream consumers translate it into metrics, ledgers, or OTel events.
- `HooksLayer::with_usage_sink` — wires a `UsageSink` into the Tower stack; emits one event per request completion (success or error). Sink errors are best-effort: logged and not propagated to callers.
- `liter_llm::tower::IdempotencyLayer` — `Idempotency-Key` dedup with pluggable `IdempotencyStore` (default `InMemoryIdempotencyStore` via `DashMap`, 24h TTL). Mismatched body for same key returns `LiterLlmError::IdempotencyConflict`; in-flight key returns `LiterLlmError::IdempotencyInFlight` (error-out, not sleep-poll).
- `LlmRequest::idempotency_key: Option<String>` field and `LlmRequest::with_idempotency_key` builder for opt-in idempotency.
- `LiterLlmError::IdempotencyConflict { key: String }` and `LiterLlmError::IdempotencyInFlight { key: String }` error variants (HTTP 409 equivalent).
- `liter_llm::tower::FallbackChainLayer` — walk an ordered `Vec<S>` of services, advancing on transient errors via pluggable `RetryPolicy`. `DefaultRetryPolicy` treats 5xx/timeouts/429 as transient; auth and validation errors as terminal. Exports `RetryClass`, `RetryPolicy`, `DefaultRetryPolicy`, `FallbackChainLayer`, and `FallbackChainService`.
- `liter_llm::tenant::{TenantId, TenantContext, KeyResolver, ResolvedKey, KeyResolverError, InMemoryKeyResolver}` — generic multi-tenant primitives.
- `liter_llm::tenant::EtcdKeyResolver` — distributed `KeyResolver` backed by an etcd cluster. Behind feature `etcd-key-resolver`. Reads JSON-serialised `ResolvedKey` at `{prefix}/{sha256(api_key)}`. Configure via `EtcdKeyResolverConfig` (endpoints, prefix, timeouts, optional auth).
- `LlmRequest::with_tenant_id` / `LlmRequest::tenant_id` for tenant propagation through the Tower stack.
- `LlmRequestKind` — the discriminant enum extracted from `LlmRequest` to carry the variant payload; re-exported from `liter_llm::tower`.

### Changed

- **Migration — `RealtimeEvent::RateLimitsUpdated`**: field `reset_at: SystemTime` renamed to `reset_at_unix_ms: i64` (Unix milliseconds). Pattern-match sites must update the field name; the value can be reconstructed with `SystemTime::UNIX_EPOCH + Duration::from_millis(reset_at_unix_ms as u64)` if needed.
- **Migration — `WaitForBatchConfig`**: fields renamed for FFI bridgeability — `initial_interval: Duration` → `initial_interval_secs: f64`, `max_interval: Duration` → `max_interval_secs: f64`, `timeout: Option<Duration>` → `timeout_secs: Option<f64>`. Update all construction sites to use `f64` seconds.
- **Migration — `BatchWaitError`**: variants changed to struct-style for FFI bridgeability — `Failed(BatchStatus)` → `Failed { status: BatchStatus }`, `Timeout(Duration)` → `Timeout { timeout_secs: f64 }`, `Client(LiterLlmError)` → `Client { message: String, code: u32 }`. Update all pattern-match and construction sites.
- `LlmRequest` is now a struct (`kind: LlmRequestKind`, `tenant_id: Option<TenantId>`) rather than a plain enum. All existing constructor call sites (`LlmRequest::Chat(r)`, `LlmRequest::Embed(r)`, etc.) continue to compile unchanged via `#[allow(non_snake_case)]` associated functions. Pattern-match sites that directly match on `LlmRequest::Variant` must be updated to match on `req.kind`.
- `liter-llm-proxy` `AppState` gains `key_resolver: Arc<dyn KeyResolver>` alongside the existing `key_store: Arc<KeyStore>`. `KeyStore` implements `KeyResolver`; behaviour is unchanged.
- Cache (`CacheService`, `SingleflightService`) reads `tenant_id` from the request via `LlmRequest::tenant_id()` so cached responses are scoped to the correct tenant automatically.
- `KeyResolver::resolve` now takes an owned `String` and returns a `'static` future so the future can be spawned across `tower::Service::call` boundaries.
- `ProviderCredential.api_key` is now `secrecy::SecretString` (was `String`); zeroed on drop, `Debug` impl redacts to `[REDACTED]`.
- `liter-llm-proxy` activates `liter_llm::provider::set_outbound_policy` at startup based on `SecurityConfig.outbound_policy`. Defaults to `DenyPrivate` (blocks RFC1918, loopback, link-local, multicast, unspecified); `Allowlist` parse errors are logged and skipped with a startup summary line.
- `liter-llm-proxy` virtual-key lookup uses `subtle::ConstantTimeEq` over a full iteration of the key store, preventing timing-side-channel inference of key existence.
- `tower::HooksLayer::with_usage_sink` now dispatches sink emit via `tokio::spawn`; slow sinks no longer add latency to caller-observed responses.

### Fixed

- **`tower::circuit`** — `maybe_half_open` is wired into the request path; circuits no longer get permanently stuck `Open` after tripping. `probe_in_flight` is now a `ProbeGuard<P>` RAII enum so the probe slot is reclaimed even if the future panics or is cancelled. `should_allow` gates HalfOpen to exactly one probe via atomic CAS.
- **`tower::hedge`** — `HedgeService::call` uses `mem::replace` correctly so the polled-ready inner becomes the primary attempt and hedge attempts clone from a fresh standby; eliminates Tower-readiness contract violation and `ConcurrencyLimit` permit double-consumption.
- **`tower::cache::CacheService::call`** — applies the `mem::replace(&mut self.inner, self.inner.clone())` Tower swap so permit-bearing inner services (e.g. `ConcurrencyLimit`, `Buffer`) are not double-consumed.
- **`tower::cache_singleflight`** — `tx.send(result)` now precedes `map.remove(&key)` so late-arriving followers don't become duplicate leaders. Added `LeaderDropGuard` to clean up the in-flight map when the leader is cancelled; leader errors propagate to followers with variant preserved (no longer mapped to `InternalError`).
- **`tower::cache_key::ExactHashStrategy`** — uses `ahash::RandomState::generate_with(fixed_seeds)` via `OnceLock` so cache keys are deterministic across process restarts and distributed nodes (was `std::hash::DefaultHasher`, which is randomized per process).
- **`tower::idempotency::compute_body_hash`** — same deterministic-ahash fix as the cache key strategy; required for distributed `IdempotencyStore` backends to function.
- **`tower::idempotency::IdempotencyService::call`** — store key is now `format!("{tenant_id}:{idempotency_key}")` so two tenants with the same idempotency key value do not cross-pollinate responses.
- **`tower::fallback_chain::FallbackChainService::call`** — drives `svc.ready().await?` before each `svc.call(...)`, restoring the Tower readiness contract for chain elements (previously silently bypassed `ConcurrencyLimit` permits). `FallbackChainLayer` implements `Layer<()>` with a `prepend(head)` helper for `ServiceBuilder` composition.
- **`tower::budget`** — window rollover is atomic via CAS; concurrent writers across the rollover boundary cannot torn-read a $0 window mid-zero-then-add.
- **`liter-llm-proxy::routes::realtime`** — the WebSocket upgrade now resolves the upstream credential via the per-model `ProviderCredential` registry instead of `general.master_key`; restricted virtual keys can no longer trigger Realtime sessions that bill the master key. The handler also calls `key_ctx.can_access_model(&model)` before `ws.on_upgrade`, so the model allowlist is enforced before any upstream connection is opened.
- **`liter-llm-proxy::secrets`** (`aws.rs`, `vault.rs`) — cache values are stored as `secrecy::SecretString` so secrets are zeroed on TTL eviction; previously plain `String` left key material on the heap.
- **`liter-llm-proxy::secrets::vault`** — `HashiCorpVaultProviderBuilder::build` validates the address against the outbound policy (`validate_outbound_url_sync`) before constructing the `VaultClient`; misconfigured addresses pointing at internal endpoints (link-local, loopback, RFC1918) are rejected with `SecretError::Forbidden`.
- **`guardrail::cel`** — CEL evaluation errors are no longer returned to the caller verbatim (could leak expression internals); callers see a fixed `"policy evaluation error"` reason while the full error is logged server-side via `tracing::error!`.
- **`http::transport::TransportConfig`** — re-exported at the crate root so the rustdoc example compiles; previously the type was reachable only via the `pub(crate)` `http` module.
- **Trait re-exports** — `CacheKeyStrategy`, `Guardrail`, `GuardrailContext`, `GuardrailDecision`, `GuardrailStage`, `VectorStore`, `VectorMatch`, `EmbeddingProvider`, `NoOpEmbeddingProvider`, `TenantId`, `TenantContext`, `KeyResolver`, `ResolvedKey`, `KeyResolverError`, `InMemoryKeyResolver`, `IdempotencyStoreError` are now reachable from `liter_llm::*` or `liter_llm::tower::*` without spelling out the full module path.

### Security

- All findings from three rounds of critical security audit are resolved: per-model credential isolation on the Realtime route, model-allowlist enforcement before WebSocket upgrade, constant-time virtual-key lookup, `SecretString` for cached secrets and provider credentials (zeroed on drop), SSRF outbound-policy guard activated at proxy startup, Vault address validation, CEL error redaction, fail-closed CEL guardrails, and JSON-tree redaction for regex guardrails.

## [1.6.0-rc.0] - 2026-06-15

### Added

- **`tower::circuit` module** — `CircuitPolicy` trait with `ExponentialBackoffCircuit` default impl, `CircuitState` enum (Closed→Open→HalfOpen), `CircuitLayer` and `CircuitService` for fault isolation. State transitions on configurable consecutive-failure threshold; half-open probes reset after configurable interval. (`crates/liter-llm/src/tower/circuit.rs`)
- **`tower::hedge` module** — `HedgePolicy` trait with `FixedDelayHedge` default impl, `HedgeLayer` and `HedgeService` for concurrent retry with jitter. Races `max_attempts` copies staggered by fixed delay; cancels losers via `tokio::task::JoinSet::abort_all()`. Fast path when `max_attempts == 1` skips `JoinSet` entirely. (`crates/liter-llm/src/tower/hedge.rs`)
- **`tower::metrics` module** — `MetricsLayer` and `MetricsService` with OTel-native GenAI semantic-convention meters (gated behind `otel` feature with no-op fallback when disabled). Emits: `gen_ai.client.operation.duration` histogram (request latency, success/failure/circuit-open labels), `gen_ai.cache.{hit,miss,stale}` counters, `gen_ai.circuit.trip` counter, `gen_ai.retry.attempt` counter, plus `gen_ai.client.token.usage` histogram and `gen_ai.request.cost_usd` histogram. Instruments cached in `OnceLock<Arc<Instruments>>` to eliminate per-request meter lookups. (`crates/liter-llm/src/tower/metrics.rs`)
- **`tower::router` module** — `Weight(u32)` saturating wrapper with NaN/Inf-safe `from_f64`, `UpstreamDiscover` trait alias, `StaticDiscover` stream-based discovery, `DynamicRouter<D>` wrapping tower's `Discover` trait with per-upstream `ConcurrencyLimit` (default 256). `HealthCheckConfig` struct with interval/timeout/unhealthy_threshold/healthy_threshold, `HealthChecker` trait, `HttpProbeHealthChecker` default impl, and `PerProviderHealthCheck` service for per-provider health status tracking. (`crates/liter-llm/src/tower/health.rs`, `crates/liter-llm/src/tower/router.rs`)
- **`http::transport::TransportConfig`** — exposed public module with configurable knobs: `pool_max_idle_per_host` (default 32), `pool_idle_timeout` (default 90 s), `tcp_keepalive` (default 60 s), `http2_prior_knowledge` (default false), `dns_cache_ttl` (default 30 s, best-effort — reqwest 0.13 lacks DNS TTL setter), `enable_http3` (default false, gated behind `http3` feature flag). Builder pattern with sensible defaults. Wired into `ClientConfig` via new `transport` field; `DefaultClient::new` applies all settings to reqwest `ClientBuilder` except dns_cache_ttl. (`crates/liter-llm/src/http/transport.rs`)
- **`client::ClientConfig::transport`** field of type `TransportConfig` with default impl for backward compatibility.
- **`liter-llm-cli` runtime flags** — `--tokio-worker-threads N` and `--tokio-max-blocking-threads N` for runtime tuning, applied to both `api` and `mcp` subcommands via explicit `tokio::runtime::Builder::new_multi_thread()` (replaces `#[tokio::main]` macro). Defaults: physical CPU count (workers), 512 (blocking threads).
- **`liter-llm-proxy::shutdown` module** — `ShutdownCoordinator`, `Drainable` trait, `ShutdownPhase` enum (Idle→Draining→Drained/Aborted), `DrainResult` enum, `ShutdownHandle` for signal handling and graceful shutdown. Signal pre-registration eliminates the miss window between first and second SIGTERM/SIGINT handlers. `spawn_signal_handler` orchestrates two-signal escalation (first → Draining, second within 5 s or 30 s hard deadline → Aborted); concurrent drain via `FuturesUnordered` so slow `Drainable`s don't block faster ones. (`crates/liter-llm-proxy/src/shutdown.rs`)
- **`liter-llm-proxy::routes::health` module enhancements** — `/healthz` (liveness: 200 always, never blocks) and `/readyz` (readiness: 200 if all probes pass, 503 otherwise). `ReadinessProbe` trait for composable health checks; built-in probes: `ServicePoolProbe` (at least one upstream configured), `TokioQueueDepthProbe` (injection queue depth < 1000). Probes run sequentially; pub `run_probes()` allows custom implementations. (`crates/liter-llm-proxy/src/routes/health.rs`)
- **`util::bounds` module** — memory-budget guard constants (`SSE_BUFFER_MAX_BYTES = 1 MiB`, `EVENT_STREAM_BUFFER_MAX_BYTES = 16 MiB`, `RESPONSE_BODY_MAX_BYTES = 32 MiB`) and `check_bound()` helper for stream overflow detection (returns `Err(LiterLlmError::Streaming)` with `tracing::warn!` if exceeded). (`crates/liter-llm/src/util/bounds.rs`)
- **Workspace `[lints.clippy]` policy** — deny `correctness/suspicious/perf`; warn `style`. Document allow overrides: `unused-unit` (generated FFI), `needless-pass-by-value` (FFI ABI), `module-name-repetitions` (library ergonomics), `missing-errors-doc`, `missing-panics-doc`. (`Cargo.toml`)
- **Feature partition** — `liter-llm`: new `lite` (native-http only, no tower/opendal/tokenizer), `http3`, `otel`, and per-auth-method gates with explicit defaults and doc comments. `liter-llm-proxy`: `otel`, `opendal-cache`, `proxy` (named surface); default = `proxy`. `liter-llm-cli`: `mimalloc`, `jemalloc` (mutually exclusive allocator selection; defaults to system allocator). Workspace dependency pinning added for allocators. (`Cargo.toml`, `crates/*/Cargo.toml`)
- **Global allocator selection** — `crates/liter-llm-cli/src/allocator.rs` gates `#[global_allocator]` behind `mimalloc`/`jemalloc` features; `compile_error` if both enabled simultaneously.
- **`tower::cache_key` module** — `CacheKeyStrategy` trait with three impls: `ExactHashStrategy` (SHA256 hash of full request), `SystemPromptAwareStrategy` (omits system-prompt field from hash), `TenantScopedStrategy` (includes tenant ID). All three hash deterministically via `serde_json` to stable JSON. (`crates/liter-llm/src/tower/cache_key.rs`)
- **`crates/liter-llm/src/{embedding,vectorstore}` modules** — `EmbeddingProvider` trait with two impls: `SelfHostedEmbeddingProvider` (calls local LLM endpoint for embeddings via `embed()` method), `NoOpEmbeddingProvider` (returns zero vectors for unit tests). `VectorStore` trait with `InMemoryVectorStore` (DashMap-backed with brute-force cosine similarity), `OpenDalVectorStore` (persists embeddings to OpenDAL backends, gated behind `opendal-cache` feature). Both impls support `store(key, embedding)`, `retrieve_similar(query, threshold, top_k)`, `delete(key)`. (`crates/liter-llm/src/tower/vectorstore/{mod,memory,opendal}.rs`, `crates/liter-llm/src/tower/embedding.rs`)
- **`tower::cache.rs` trait extensions** — `CacheStore` gains `set_ttl(key, ttl)`, `iter_keys()`, `metadata(key) -> CacheMetadata` (expiry, creation_time, hit_count). Default no-op bodies preserve backward compat for existing impls. `CachedResponse` new variant: `Error { error: Arc<LiterLlmError>, expires_at: Instant }` for transient-error caching with custom `Serialize` impl that rejects persistence to external backends (in-memory only). (`crates/liter-llm/src/tower/cache.rs`)
- **`tower::cache_policy` module** — `CachePolicy` trait with `StandardCachePolicy` impl. Controls: `bypass_cache()` (per-request bypass), `ttl()` (seconds), `semantic_similarity_threshold` (0.85), `stale_while_revalidate` (5 minutes). `CacheService::call()` implements three-tier lookup: exact-hash match → semantic similarity via `EmbeddingProvider` → streaming-replay from stored chunks. `warm(requests)` async warming hook for batch pre-population. (`crates/liter-llm/src/tower/cache_policy.rs`)
- **`tower::cache_singleflight` module** — `SingleflightCoordinator` trait with `InMemorySingleflight` impl backed by DashMap. Coordinates concurrent identical requests: first caller blocks all followers; response broadcast via `tokio::sync::broadcast`. Eliminates thundering-herd when cache miss aligns with identical in-flight requests. (`crates/liter-llm/src/tower/cache_singleflight.rs`)
- **`tower::cache_negative` module** — `NegativeCachePolicy` trait with `FixedWindowNegativeCache` impl (caches transient errors only: retryable status 429/5xx, defaults to 60-second window). `CachedResponse::Error` variant with custom Serialize that prevents persistence to non-memory backends. (`crates/liter-llm/src/tower/cache_negative.rs`)
- **`tower::budget` module** — `BudgetLedger` trait with `CostRecordContext`, `CostCheckContext`, `BudgetVerdict`, `BudgetDimension` enum (Global/Model/Tenant/User/ApiKey), `BudgetSnapshot` struct. `InMemoryBudgetLedger` impl backed by DashMap per dimension; `export_csv()` for chargeback/reconciliation. Every `record_cost(context, usd_amount)` call checks all applicable dimensions atomically. (`crates/liter-llm/src/tower/budget.rs`)
- **`tower::rate_limit` module** — `CostRateLimitConfig { max_usd_per_minute, max_usd_per_hour, max_usd_per_day }` and `CostRateLimitLayer`/`CostRateLimitService` for hard spend ceilings. Integrates with `BudgetLedger` dimension checks; returns `Err(LiterLlmError::BudgetExceeded)` when cost would exceed any ceiling. `should_hedge()` helper returns true/false based on cost and latency signals for intelligent hedging. (`crates/liter-llm/src/tower/rate_limit.rs`)
- **`tower::metrics` OTel additions** — new meters: `gen_ai.budget.spend_usd` (histogram, labeled by dimension), `gen_ai.budget.rejection` (counter, labeled by dimension + reason). Emitted by `BudgetLedger` impls and `CostRateLimitService`. (`crates/liter-llm/src/tower/metrics.rs`)
- **`guardrail` module** — `Guardrail` trait (`name`, `supported_stages`, `check(context) -> GuardrailDecision`). `GuardrailStage` enum (Input/Output/OutputChunk). `GuardrailDecision` enum (Allow/Block/Mutate). `GuardrailContext` struct (request, response, reason). Built-in guardrails: `RegexGuardrail`, `AllowListGuardrail`, `DenyListGuardrail`, `LengthCapGuardrail`, `PromptInjectionHeuristic` (10-pattern keyword check, documented as heuristic not classifier). `GuardrailRegistry` global via `OnceLock<RwLock<…>>` matching `provider::custom` pattern. `GuardrailLayer`/`GuardrailService` Tower wrapper — runs Input on request, Output on full response, OutputChunk per streaming chunk, short-circuits on Block. CEL policy DSL gated behind `guardrail-cel` feature via `cel-interpreter` crate; eval errors fail-open with `tracing::warn!`. (`crates/liter-llm/src/guardrail/{mod,builtin,registry,cel,tests}.rs`, `crates/liter-llm/src/tower/guardrail.rs`)
- **`tower::route_classify` module** — `RouteClassifier` trait (`classify(context) -> ClassifyResult`, `confidence_threshold`). Built-in classifiers: `KeywordClassifier` (regex-pair → model), `EmbeddingSimilarityClassifier` (reuses `EmbeddingProvider` from 2.A), `LlmClassifier` (delegates to an LLM), `CascadeClassifier` (priority-ordered composition). `ClassifierVerdictCache` caches verdicts via `CacheStore`. `RoutingStrategy::Semantic(Arc<dyn RouteClassifier>)` variant in `tower/router.rs`; falls back to round-robin when classifier defers. OTel meters: `gen_ai.route.classify.duration` histogram + `gen_ai.route.classify.tier{keyword,embedding,llm}.hit` counters. (`crates/liter-llm/src/tower/route_classify.rs`)
- **Type-state builder pattern** — `ClientBuilder<HasApiKey, HasProvider>` with marker types `NoApiKey`/`WithApiKey`/`NoProvider`/`WithProvider`. `build()` only callable on `ClientBuilder<WithApiKey, WithProvider>` (compile-time error otherwise). Enforces API key and provider selection before use. (`crates/liter-llm/src/client/builder.rs`)
- **`ProviderCapabilities` struct** — `vision`, `reasoning`, `structured_output`, `function_calling`, `audio_in`, `audio_out`, `video_in` bools. Exposed via `pub fn capabilities(provider_name: &str) -> &'static ProviderCapabilities`. (`crates/liter-llm/src/provider/mod.rs`)
- **142 provider schema entries updated** — `crates/liter-llm/schemas/providers.json` now carries explicit `capabilities` object and `streaming_format` field ("sse" everywhere except Bedrock = "aws_event_stream") for every provider. Enables capability-aware client construction and streaming-format detection. (`crates/liter-llm/schemas/providers.json`)

### Phase 3 — Realtime streaming, secret backends, credential rotation, and config hot-reload

- **`streaming` module** — unified ingress/egress streaming with three composable layers: `IngressStream<S, P>` (typed SSE decoder), `StreamPipeline<S>` (ordered per-chunk middleware via `ChunkMiddleware` trait), `EgressStream<S>` (typed OpenAI SSE encoder). When ingress format == egress format and no middleware is registered, `EgressStream` enters passthrough mode for zero-copy forwarding without deserialise/re-serialise cycle. `StreamFormat` (SSE vs. AWS EventStream) promoted to `pub` for explicit wire-format selection. Per-thread `BytesMut` pool in `EGRESS_BYTES_POOL` threadlocal reuses frame buffers under load. `CancellationToken` threaded through every layer; each `poll_next` checks it first for clean abort on client disconnect. (`crates/liter-llm/src/streaming.rs`)

- **`liter-llm-proxy::secrets` module** — `SecretManager` trait (object-safe via `Pin<Box<dyn Future>>`) with `get(name) -> SecretValue` (field: zeroed `SecretString` + `SecretMetadata`), `set(name, value, tags)`, `delete(name)`. URI-scheme routing: `env://NAME` (always available), `aws://PATH` (requires `secrets-aws` feature), `vault://PATH` (requires `secrets-vault` feature). Built-in impls: `EnvVarSecretManager` (environment variables), `AwsSecretsManagerProvider` (AWS Secrets Manager with key rotation warnings), `HashiCorpVaultProvider` (Vault KV-v2 with expiry tracking). `SecretManagerRegistry` routes by scheme and holds one singleton per backend. OTel gauge `gen_ai.secret.expires_in_seconds` (gated behind `otel` feature) emitted when secret expires within 24 h. (`crates/liter-llm-proxy/src/secrets/{mod,env,aws,vault}.rs`, `crates/liter-llm-proxy/src/secrets/registry.rs`)

- **`liter-llm-proxy::config::ConfigProvider` trait** — `load() -> ProxyConfig` (single snapshot) and `watch() -> mpsc::Receiver<ConfigEvent>` (live updates). Impls: `StaticFileConfigProvider` (TOML file, no hot-reload), `FileWatchConfigProvider` (OS file watch via `notify` crate), `EtcdConfigProvider` (distributed etcd key prefix watch with `Put`/`Delete`/`Resync` semantics). `ProxyConfig` interpolation now supports `${SECRET_URI}` syntax so `base_url = "${env://ANTHROPIC_BASE_URL}"` fetches at startup; secret rotation does not auto-reload URLs. (`crates/liter-llm-proxy/src/config/{provider,watcher}.rs`)

- **`liter-llm-proxy::provider::CredentialPool` trait** — rotates per-provider API keys on 429/5xx rate-limit signals. Methods: `current(provider) -> CredentialHandle` (round-robin active credential), `mark_exhausted(provider, handle, cooldown)` (park for cool-down, advance to next), `snapshot(provider) -> PoolSnapshot` (observability: total/active/exhausted counts + next recovery time). `InMemoryCredentialPool` impl backed by `DashMap` with per-credential cooldown state. `ProviderCredential` struct (model `ProviderCredential` in `VirtualKeyConfig` with `id`, `api_key: String`, `model_allowlist`) seeds pool entries from TOML. Decouples proxy credential cycling from `SecretManager` — supports static inline keys and external secret backends interchangeably. (`crates/liter-llm-proxy/src/provider/{credential_pool,credential_pool_memory}.rs`)

- **`liter-llm::realtime` module** — unified envelope + event types for vendor-neutral realtime streaming. `RealtimeEvent` enum (24 variants: SessionCreated, ConversationItemCreated, ResponseCreated, ResponseTextDelta, ResponseAudioDelta, ResponseFunctionCallArgumentsDelta, InputAudioBufferAppend, RateLimitsUpdated, Error, Raw, …). `ContentPart` enum (Text, Audio, ImageRef) used in conversation items. `ResponseStatus` enum (Completed, Cancelled, Failed, Incomplete). `RealtimeEnvelope` wraps event + optional `event_id`. `RealtimeTranslator` trait for pluggable per-provider translation (maps wire format ↔ unified schema, object-safe, thread-safe). Built-in impl: `openai::OpenAiRealtimeTranslator` (1-to-1 mapping because OpenAI's schema is already the reference shape). `crates/liter-llm/src/realtime/{mod,openai}.rs`)

- **`AppState` refactor** — `config` field changed from `Arc<ProxyConfig>` to `Arc<ArcSwap<ProxyConfig>>` for atomic hot-reload without blocking in-flight requests. New `secret_registry: Arc<SecretManagerRegistry>` field for resolving secret URIs in model configs. Callers must call `state.config.load()` to obtain a consistent snapshot per request. (`crates/liter-llm-proxy/src/state.rs`)

### Migration notes

- `AppState` now requires `secret_registry: Arc<SecretManagerRegistry>` and `config: Arc<ArcSwap<ProxyConfig>>` fields. Applications using `ProxyServer::builder` are unaffected; manual state construction must update both fields.
- New optional feature flags: `secrets-aws`, `secrets-vault`, `secrets-env` (env backend always enabled, others optional). `mimalloc`, `jemalloc` for allocator selection. `http3` for HTTP/3 support. `tokenizer` for `count_tokens` availability.
- `VirtualKeyConfig` gains new `provider_credentials: Vec<ProviderCredential>` field (defaults to empty). Inline credentials in TOML via repeated `[[keys.provider_credentials]]` blocks; proxy auto-rotates among them on 429/5xx.
- Workspace clippy is now `-D warnings`; downstream consumers compiling with strict lints should review suppressions — the main crate is now warnings-clean.
- **`LlmRequest` pattern-matching change** — `LlmRequest` was previously an enum; it is now a struct with a `kind: LlmRequestKind` field. `match req { LlmRequest::Chat(r) => ... }` no longer compiles. Migrate to `match req.kind() { LlmRequestKind::Chat(r) => ... }` (preferred, using the new `kind()` accessor) or `match req.kind { LlmRequestKind::Chat(r) => ... }` (direct field access). The PascalCase constructor aliases (`LlmRequest::Chat(r)` etc.) remain callable for constructing requests and continue to compile unchanged; they are marked `#[doc(hidden)]` and will be removed in a future minor release.
- **`KeyResolver::resolve` signature change** — the method now takes `api_key: String` (owned) instead of `api_key: &str`, and returns `Pin<Box<dyn Future<Output = ...> + Send + 'static>>` instead of `... + 'a`. Custom `KeyResolver` implementations must update their signature. Call sites that previously passed a `&str` literal must add `.to_owned()`: `resolver.resolve("sk-...".to_owned())`.

### Changed

- **Bindings regenerated against alef v0.25.9**; refreshes all 16 language surfaces, e2e suites, and README templates. New `[crates.e2e.fields_c_types]` entry `chat_completion_response.usage = "Usage"` and per-call C# e2e override `class = "LiterLlmConverter"` to satisfy alef v0.25.9's stricter intermediate-accessor checks.
- **`tower/router.rs`**: `WeightedRandom` now uses the new `Weight(u32)` saturating type (handles f64 NaN/Inf cleanly). `DynamicRouter` replaces ad-hoc hardcoded routing with tower::discover integration.
- **`tower/health.rs`**: health-check configuration is now per-provider (`HealthCheckConfig { interval, timeout, unhealthy_threshold, healthy_threshold }`) instead of a single global setting.
- **`http/streaming.rs`**: SSE pipeline now propagates a `tokio_util::sync::CancellationToken` end-to-end via `post_stream_with_cancel()` so client disconnect aborts the upstream stream cleanly. Threadlocal `BytesMut` pool wired in for SSE frame buffers (currently used by tests; production callers will be added in Phase 2).
- **`cli/main.rs`**: explicit `tokio::runtime::Builder::new_multi_thread()` replaces `#[tokio::main]`. Worker/blocking-thread counts now configurable.
- **Clippy policy enforcement** — workspace-wide `cargo clippy --workspace -- -D warnings` is now clean without per-crate suppressions. Narrower allow lists (correctness pass, style warnings only) reduce oversight surface.
- **`tower/cache.rs` trait extensions** — `CacheStore` method signatures extended with `set_ttl(key, ttl)`, `iter_keys()`, `metadata(key)` with default no-op bodies; backward compatible with existing impls. `CachedResponse` struct gained `Error { error: Arc<LiterLlmError>, expires_at: Instant }` variant with custom serialization.
- **`tower/router.rs` `RoutingStrategy` enum** — gained `Semantic(Arc<dyn RouteClassifier>)` variant for classifier-driven routing. Removed `#[derive(Debug)]` and now has manual `Debug` impl (dyn Trait is not Debug). Round-robin fallback when classifier defers.

### Fixed

- **Test diagnostic clarity** — all 417 test `.unwrap()` calls replaced with `.expect("descriptive message")` naming the asserted invariant, improving failure diagnostics when assertions fire. Production code paths remain unwrap-clean. (`crates/liter-llm-*/src/**`)
- **`tower/circuit.rs`**: `record_failure()` no longer spawns a tokio task to flip state (uses synchronous CAS loop) — eliminates duplicate-spawn race under burst failure and removes the runtime dependency that made `record_failure` panic outside async contexts.
- **`tower/hedge.rs`**: `HedgeService::call` now honours the Tower `ServiceExt::ready()` readiness contract via `std::mem::replace`, so wrapping a `ConcurrencyLimit`-protected upstream no longer silently bypasses the semaphore. Hedge fast-path (max_attempts == 1) skips `JoinSet` entirely.
- **`tower/metrics.rs`**: instrument lookups cached in `OnceLock<Arc<Instruments>>` instead of constructed per-request. Removes ~8k redundant meter lookups/sec at 1k req/s production load.
- **`http/streaming.rs`**: dead `BytesMut` scratch field removed from `SseParser` — was acquired from threadlocal pool but never read/written, pinning ~4 MiB across 1k concurrent streams. Pool helpers gated under `#[cfg(test)]` since production has no remaining callers.
- **`liter-llm-proxy/shutdown.rs`**: pre-registered SIGTERM/SIGINT handles eliminate the miss window between first signal returning and second-signal listener registering. Concurrent drain via `FuturesUnordered` ensures slow `Drainable`s don't block faster ones before 30 s hard deadline.
- **`liter-llm-proxy/routes/health.rs`**: `/readyz` now uses stable `tokio::runtime::RuntimeMetrics::num_alive_tasks()` (original `injection_queue_depth` only exists behind `tokio_unstable` cfg).
- **1081 alef-generated file conflict markers** — `git stash`-introduced merge conflict markers (<<<<<<, ======, >>>>>>) systematically scrubbed from bindings, e2e suites, test_apps, and generated docs. The C# `LiterLlmConverter.cs` FFI null-check pattern required manual resolution; workspace builds clean. (commit `892500ec6`)
- **Cache singleflight flake elimination** — test race where the leader completed before followers attached to the broadcast channel eliminated via atomic `Arc<Broadcast>` initialization before channel send. Fast mock services under parallel load now stable. (commit `4e3a3e51e`)

### Tooling

- **Workspace clippy lint policy enforcement** via `[workspace.lints.clippy]` blocks; per-crate suppressions consolidated at source.
- **Feature flag audit** — split composite features (e.g., `native-http` still depends on `http2`, now gated on both); avoid silent breakage from feature interaction.
- **Allocator build variants** — `BUILD_PROFILE=release task build` with `--features jemalloc` for performance-sensitive deployments; system allocator is default for lighter containers.
- **New optional dep `cel-interpreter`** (~110 KB compressed) behind `guardrail-cel` feature flag for CEL policy DSL evaluation in guardrails module.
- **`regex` workspace dependency exposed** — already present in transitive tree; now explicit for `guardrail::builtin::RegexGuardrail` and `KeywordClassifier`.

## [1.5.1] - 2026-06-13

### Changed

- **publish workflow**: migrate every push, release-asset upload, and homebrew-tap commit to the `kreuzberg-dev-publisher[bot]` GitHub App via `actions/create-github-app-token@v2`, replacing `secrets.GITHUB_TOKEN` and `secrets.HOMEBREW_TOKEN` with scoped app installation tokens.
- **Bindings regenerated against the latest alef**, refreshing all 16 language surfaces and e2e suites.

### Fixed

- **Dart binding**: named parameters and null-safety annotations, plus per-language README sync and updated method/type counts (#133).
- **PyO3 0.29 method rename**: `pyo3::Bound::downcast_into` callsites in `crates/liter-llm-py/src/lib.rs` migrated to the new `cast_into` name so the Python binding builds against pyo3 0.29.
- **PMD ruleset**: exclude `UnnecessaryWarningSuppression` from `category/java/bestpractices.xml`. Alef emits a blanket `@SuppressWarnings("PMD")` on every generated DTO record; PMD flags some as unnecessary depending on which rules fire on the surrounding record, breaking the Java hook on every regeneration.

## [1.5.0] - 2026-06-07

### Security

External security audit identified six exploitable gaps in the v1.4.1 codebase. All six are fixed here with regression tests; releasing as a minor version because three of them change defaults.

- **(F1, CRITICAL) Master-key constant-time comparison** — `KeyStore::is_master_key` previously compared the bearer token to the configured master key via `==`, exposing a per-request timing sidechannel. Now stores the master key in `secrecy::SecretString` and compares via `subtle::ConstantTimeEq::ct_eq` on the raw bytes. (`crates/liter-llm-proxy/src/auth/key_store.rs`, new `subtle = "2.6"` dep in `crates/liter-llm-proxy/Cargo.toml`.)
- **(F2, HIGH, BREAKING) SSRF guard on outbound provider URLs** — `CustomProviderConfig::base_url` accepted arbitrary URLs and the `reqwest::Client` had no DNS-resolution policy, so a malicious custom-provider registration could point at `127.0.0.1` / `169.254.169.254` / RFC1918 networks. New `liter_llm::provider::OutboundPolicy { Off, DenyPrivate, Allowlist(_) }` chokepoint validates URLs at registration time and a `GuardedResolver` re-applies the policy per-request via `reqwest`'s `dns_resolver` hook, including redirect-hop validation. Library default is `Off` (back-compat preserves embedded/FFI behaviour); proxy default is `DenyPrivate`. New `LiterLlmError::OutboundForbidden` variant maps to HTTP 502. New TOML key `[security] outbound_policy = "deny_private" | "off" | { allowlist = ["…"] }`. (`crates/liter-llm/src/provider/outbound_policy.rs`, `crates/liter-llm/src/provider/custom.rs`, `crates/liter-llm/src/client/mod.rs`, `crates/liter-llm-proxy/src/config/server.rs`, `crates/liter-llm-cli/src/commands/serve.rs`.)
- **(F3, HIGH, BREAKING) MCP per-tool model-access gate + HTTP transport auth** — every `#[tool]` handler in `crates/liter-llm-proxy/src/mcp/mod.rs` (chat, embed, list_models, generate_image, speech, transcribe, moderate, rerank, search, ocr, create_response, plus all file and batch management tools) now resolves a `KeyContext` from the rmcp `RequestContext.extensions` and pre-flight-checks `can_access_model(&params.model)` or `is_master` before routing through `ServicePool` / `FileStore`. The HTTP/SSE MCP transport mounted in `crates/liter-llm-cli/src/commands/mcp.rs` is wrapped with the same `validate_api_key` middleware as the OpenAI endpoint, so virtual-key restrictions apply uniformly. Stdio transport requires an explicit `mcp.stdio_key_id` / `mcp.stdio_trust_local = true` opt-in or refuses to start.
- **(F4, MED-HIGH) Error message sanitization** — SSE error events and `ProxyError::from(LiterLlmError)` previously embedded raw provider error strings via `Display` with no truncation or control-character handling. New `crates/liter-llm-proxy/src/error.rs::sanitize_message` (UTF-8-safe 200-char truncation, control-character strip except `\t`/`\n`) is applied at the single `From<LiterLlmError>` chokepoint; SSE payloads now build via `serde_json` rather than string interpolation, and `ProxyError::to_sse_payload` is the canonical serializer.
- **(F5, MED-HIGH) Mutex poisoning recovery** — `SyncService::clone_service` (`crates/liter-llm/src/client/managed.rs`) previously panicked when the inner `std::sync::Mutex` was poisoned. The lock guard only protects the clone step over a `BoxCloneService`, which is `Clone` and stateless across the lock, so recovery is safe: poisoned guards are now reclaimed via `PoisonError::into_inner` and the next request proceeds normally.
- **(F7, MED-HIGH, BREAKING) CORS default is empty + wildcard origin loses Authorization header** — the proxy's `default_cors()` is now `vec![]` instead of `vec!["*"]`; with no `cors_origins`, the router skips the `CorsLayer` entirely. When `cors_origins` is set to `"*"`, the wildcard branch restricts `allow_headers` to a fixed list (`CONTENT_TYPE`, `ACCEPT`, …) and explicitly does **not** include `Authorization` — wildcard origins must not see credentialed headers per CORS-fetch spec. `liter-llm-cli serve` also logs a `tracing::warn!` when `cors_origins.contains("*") && host == "0.0.0.0"`.

### Changed

- **Bindings regenerated against alef v0.23.28** (was v0.23.16). All 16 language surfaces — Python, Node, Ruby, PHP, Go, Java, Kotlin Android, C#, Elixir, WASM, C/FFI, Zig, Dart, Swift, R, Homebrew — and the e2e suites refresh end-to-end. The new alef ships my upstream java/magnus/go template patches (PMD braces, jinja whitespace, `MethodHandle.invoke` `throws Throwable` wrap, `data_enum` close-brace, magnus top-level module doc) plus the parallel agent's brew/zig/php/dart/snippets/kotlin/swift fixes.
- **Tighter Rust clippy allow surface** in the core and proxy crates: removed three unused `#[allow]` annotations, the unused `get_json` helper in `crates/liter-llm/src/http/request.rs`, and a now-dead `serde::de::DeserializeOwned` import. `cargo clippy --workspace … -- -D warnings` is clean without the deleted suppressions.

### Tooling

- **`xberg-io/pre-commit-hooks` bumped to v2.1.10** — picks up the consumer-side `alef-sync-versions --no-regen` fix (full regen no longer fires on every commit), the palantir-java-format multi-platform sha256 manifest acceptance, the ktfmt checksum entry, and the `godoc-lint` / `golangci-lint` go.work-aware module discovery (no longer scans stale `test_apps/swift_e2e/.build/checkouts/.../e2e/go/`).
- **Project-local PMD ruleset** at `packages/java/pmd-ruleset.xml` wired into the `pmd` hook to suppress alef-generated FFI patterns that PMD's quickstart ruleset misflags (`AvoidCatchingGenericException`, `PreserveStackTrace`, `CloseResource`, `UnusedLocalVariable`, `UnnecessaryFullyQualifiedName`, `VariableCanBeInlined`, `ReturnEmptyCollectionRatherThanNull`).
- **`deny.toml`** ignores `RUSTSEC-2023-0071` (Marvin Attack timing sidechannel in `rsa@0.9.x`, transitive via `opendal -> reqsign-core`). No safe upstream version yet; the underlying RSA private-key signing path is not exercised on our network-observable code paths.
- **`alef-docs-fresh` hook and the CI `Verify alef-generated code is up-to-date` step soft-disabled** pending an alef v0.23.28 `inputs-hash` regression fix — `alef verify` currently flags files as stale immediately after a fresh `alef all` run (the hash recomputed during verify disagrees with the hash written at emit time).
- **`markdownlint-rumdl-strict`** exclude expanded to cover the root `README.md` (alef-generated badge row uses inline HTML), `CONTRIBUTING.md`, `templates/readme/`, and `.github/PULL_REQUEST_TEMPLATE.md`.

### Migration notes

The three behaviour-changing defaults above (`cors_origins = []`, `outbound_policy = "deny_private"`, MCP per-tool model gate) are all reversible via explicit config. Operators who relied on the old defaults should add to their proxy config:

```toml
cors_origins = ["*"]                # opt back into the v1.4.x wildcard CORS default
[security]
outbound_policy = "off"             # opt back into the v1.4.x unguarded outbound HTTP
```

Virtual-key holders who previously hit MCP tools without a model-access policy need their `[[virtual_keys]]` entries updated to include the model names they expect to call — or be granted `is_master = true`.

## [1.4.1] - 2026-06-05

### Fixed

- **Docker build**: removed stale `COPY tools/ tools/` from `docker/Dockerfile` — the `tools/` directory was deleted in v1.3.0 and the unfixed copy was failing every Docker image build since.
- **`publish-crates` job timeout**: bumped `.github/workflows/publish.yaml` `publish-crates` `timeout-minutes` from 30 to 60. The 30-minute ceiling was cancelling mid-publish on busy `crates.io` index-propagation days, which (combined with the Python stdout buffering issue below) made cancelled runs look like silent failures with no per-crate log output.
- **Upstream `xberg-io/actions` to v1.8.29**: `publish-crates/scripts/publish.py` now line-buffers stdout/stderr (`sys.stdout.reconfigure(line_buffering=True)`), so per-crate "Publishing X (n/total)..." progress survives job cancellation. Before this fix, GitHub Actions' block-buffered Python stdout swallowed all in-flight progress when the job hit `timeout-minutes`, hiding which crate was actually mid-publish.

### Notes

- v1.4.0 was a no-op release because `task version:bump` was not run before tagging — the tree still carried `1.4.0-rc.61` in `Cargo.toml`, so every publish job either re-shipped `rc.61` artifacts (already on the registry) or failed verification looking for `1.4.0`. v1.4.1 is the first real `1.4.x` release.
- alef pin advanced to `0.23.16` (was `0.23.12`) — no functional codegen changes vs. `0.23.12`; bump tracks the latest released `0.23.x`.

## [1.4.0] - 2026-06-05

### Added

- `feat(provider/vertex): auto-install VertexAdcCredentialProvider in DefaultClient::new` — when the resolved provider is `vertex_ai` and the caller supplied neither an explicit `api_key` nor a `credential_provider`, the client now auto-constructs `VertexAdcCredentialProvider::new()` and installs it on the config. This is the canonical auth path for GKE Workload Identity / Cloud Run / Compute Engine deployments — short-lived OAuth2 tokens are fetched from the metadata server (with a `gcp_auth` ADC discovery fallback for local development) and cached with a 5-minute pre-expiry refresh buffer. Pre-obtained tokens supplied via `api_key` and explicit `credential_provider`s continue to take precedence. The ADC module is now reachable through the `native-http` feature (gated behind `native-http` instead of `vertex-adc`, with `vertex-adc` retained as a back-compat alias).
- `feat(provider/azure): per-model `base_url` overrides for Azure deployments` — `[[models]]` entries that pin a `base_url` for an `azure/...` `provider_model` now route through `AzureProvider::with_base_url(...)`, producing the required `{base_url}/openai/deployments/{model}{path}?api-version=…` shape instead of the generic OpenAI-compatible URL. Unblocks multi-resource Azure setups (different deployments per region/subscription). Closes #83.
- `feat(wasm-backend): emit chat_stream returning JS async iterator` — the WASM binding now exposes `WasmDefaultClient.chat_stream(req)` alongside the existing `chat`, `embed`, etc. The streaming adapter buffers the underlying `BoxStream<ChatCompletionChunk>` into an array and returns it as a `JsValue`, mirroring the NAPI binding's streaming semantics.
- CLI binary tarballs (Linux x86_64/aarch64, macOS aarch64, Windows x86_64) attached to GitHub Releases for direct download — closes #64.
- `schemas/pricing.json` regenerated from [models.dev](https://models.dev) and now covers 4,219 models (up from 35); `scripts/generate_pricing.py` wired into `task generate:pricing`, `task update`, and `task upgrade`. Closes #48.
- `Usage::prompt_tokens_details` (`{ cached_tokens, audio_tokens }`) deserialised from the OpenAI-compatible response body, plus `cost::completion_cost_with_cache` and matching `cache_read_input_token_cost` / `cache_creation_input_token_cost` fields on `ModelPricing`. `ChatCompletionResponse::estimated_cost` and the `CostTrackingLayer` now bill cached prompt tokens at the provider's discounted cache-read rate. `schemas/pricing.json` carries cache-read/cache-creation costs for the 1,500+ models on models.dev that publish them. Closes #65.
- `ci-mobile`: new `.github/workflows/ci-mobile.yaml` running `android-check` (ubuntu, `arm64-v8a` + `x86_64` via `cargo ndk`), `ios-check` (macos, `aarch64-apple-ios` + `aarch64-apple-ios-sim`), and `xcframework-build` (macos, SPM-ready XCFramework + SHA256 checksum). Uses shared composite actions from `xberg-io/actions@v1`.
- **Alef migration to v0.23.11**: the entire polyglot surface (16 language bindings — Python, Node, Ruby, PHP, Go, Java, C#, Kotlin Android, Elixir, WASM, C/FFI, Zig, Dart, Swift, Homebrew + Rust core) is regenerated end-to-end via [alef](https://github.com/xberg-io/alef). Streaming (`chat_stream`) is available across every applicable language, including Go (cgo channel bridge), Dart (FRB v2 `StreamSink<T>`), and WASM. Skipped-assertion total across e2e suites: 354 → 0.

### Changed

- **API rename**: `ResponseClient::retrieve_response` / `cancel_response` now take a parameter named `response_id` (was `id`). Positional callers are unaffected; named-arg callers must update. Consistent with `file_id` / `batch_id` on the file and batch clients, and unblocks the alef-generated Python binding from shadowing the `id` builtin.
- **GitHub Release CLI assets** ship a single sorted `SHA256SUMS-<version>.txt` instead of one `.sha256` per archive — closes #67.
- **WebAssembly build verified `mio`-free.** `liter-llm` exposes two mutually exclusive HTTP-stack features — `native-http` (reqwest + tokio + memchr + base64) and `wasm-http` (reqwest + memchr + base64 + gloo-timers, *no* tokio). `liter-llm-wasm` enables only `wasm-http`; reqwest is pinned with `default-features = false, features = ["json", "stream", "rustls", "multipart", "form"]`. `cargo build --target wasm32-unknown-unknown -p liter-llm-wasm` pulls neither `mio` nor `tokio` — reqwest auto-routes to the browser/Node `fetch` API on `wasm32` targets.
- **Ruby publish** vendors core crates exclusively via the shared `xberg-io/actions/rewrite-native-deps@v1` action (alef `publish prepare`, `vendor_mode = "core-only"`). The bespoke `scripts/ci/ruby/vendor-liter-llm-core.py`, the local `ruby:vendor` Task, and the `ruby:build` dependency on it are removed.
- **Repo hygiene**: `.gitattributes` marks all alef-generated output directories (`packages/**`, `crates/*-{py,php,ffi,node,wasm}/**`, `e2e/**`) as `linguist-generated=true` so generated files collapse in GitHub PR diffs.

### Fixed

- **TLS ABI floor**: reqwest crypto provider switched from `aws-lc-rs` to `ring` (`rustls-no-provider` feature + explicit `rustls` dep with `ring` backend). Eliminates `__isoc23_strtol` and related glibc 2.38+ symbols emitted by `aws-lc-sys` 0.40.0, restoring the GLIBC_2.28 ABI floor required by downstream users (e.g. Node.js aarch64 bindings).
- **HTTP retry jitter on `wasm32-unknown-unknown`**: the jitter calculation called `std::time::SystemTime::now()` which panics with `RuntimeError: unreachable` on bare wasm32 (std time is not implemented). On `wasm32` the jitter step is skipped; native targets keep the existing `[0.5x, 1.0x]` jitter. Unblocks WASM e2e tests that exercise 429/5xx retry paths.
- **WASM and JNI bindings** no longer fail to compile against the `tokenizer`-gated `count_tokens` / `count_request_tokens` functions. Both now `exclude_functions` in `alef.toml`; apps that need token counting on those targets should call a server-side endpoint.
- **C/FFI header** emits the opaque `typedef struct LITERLLMLiterLlmError LITERLLMLiterLlmError;` referenced by the `literllm_liter_llm_error_{status_code,is_transient,error_type}` accessors.
- **Java** `ResponseObject` / `ResponseTool` DTOs round-trip the full OpenAI Responses payload. `ResponseOutputItem.content` is a `List<…>` (was a misaligned `LinkedHashMap`); `ResponseTool` accepts `description` via the `@JsonAnyGetter` / `@JsonAnySetter` flatten path. Fixes `MismatchedInputException` and `UnrecognizedPropertyException` thrown by `createResponse` / `retrieveResponse` / `cancelResponse`.
- **Node (NAPI) streaming** HTTP-init errors (400 content-policy, 401 unauthorized on `chatStream`) now reject through the iterator. Binding remains lazy (parity with Python's `async for _ in stream: pass`).
- **Python `api.py` wrapper** emits the correct shape for non-streaming methods (22 `DefaultClient` ops). Previously every method was wrapped as a streaming `AsyncIterator`; only `chat_stream` is genuinely streaming now. Also fixes `String` → `str` and `bytes::Bytes` → `bytes` mappings.

## [1.3.0] - 2026-04-23

### Changed

- **Alef migration**: All language bindings are now auto-generated by [alef](https://github.com/xberg-io/alef) instead of hand-written
- `BoxFuture`/`BoxStream` type aliases no longer wrap `Result<T>` — all method signatures now explicitly return `Result<T>`
- `provider` module is now public (was `pub(crate)`)
- `ChatCompletionRequest.stream` field is now public (was `pub(crate)`)
- Switched spell checker from codespell to [typos](https://github.com/crate-ci/typos)
- CI no longer runs code generation — only `alef verify --exit-code` for freshness checks
- Updated alef to v0.5.9

### Added

- `alef.toml` configuration for 10 language targets, 23 API method call configs, mock server support
- `bindings.rs` adapter module with `create_client` and `create_client_from_json` binding-friendly constructors
- `Default` derives on all public types for binding compatibility
- `Clone` derive on `DefaultClient`
- E2E test fixtures converted to alef format (167+ fixtures across 23 categories)
- E2E tests regenerated for 13 languages with mock HTTP server support
- Test apps generated with `alef e2e generate --registry`
- API reference documentation auto-generated with `alef docs` for all 10 languages
- Package READMEs generated with `alef readme` using restored Jinja templates
- `alef-verify` and `alef-sync-versions` pre-commit hooks
- `alef verify --exit-code` step in CI validation workflow
- `.lychee.toml` link checker configuration
- `_typos.toml` spell checker configuration
- Auto-load API keys from environment variables
- FFI callback streaming support
- `chat_stream` method across all bindings

### Removed

- `liter-llm-bindings-core` crate — replaced by alef codegen
- `tools/e2e-generator` crate — replaced by `alef e2e generate`
- `scripts/sync_versions.py` — replaced by `alef sync-versions`
- `scripts/generate_readme.py` — replaced by `alef readme`
- `scripts/readme_config.yaml` and `scripts/readme_templates/` — replaced by `templates/readme/`
- `tests/test_apps/` — replaced by `test_apps/` (alef registry mode)
- Hand-written binding source in `crates/liter-llm-{py,node,ffi,wasm,php}/src/`
- Hand-written package source in `packages/{go,java,csharp,ruby,elixir}/`

## [1.2.2] - 2026-04-18

### Added

- GitHub Copilot OAuth Device Flow credential provider (`copilot-auth` feature) — use your Copilot subscription as an LLM backend via `github_copilot/` model prefix ([#12](https://github.com/xberg-io/liter-llm/issues/12))
- GitHub Copilot provider with OpenAI-compatible routing, required Copilot headers, per-request UUID, and `X-Initiator` header
- E2E test fixtures for GitHub Copilot provider (chat + auth error)

### Fixed

- Provider registry audit: corrected base URLs for 20 providers (aiml, assemblyai, clarifai, dashscope, deepseek, elevenlabs, firecrawl, friendliai, gradient_ai, gmi, helicone, lambda_ai, minimax, moonshot, morph, nlp_cloud, ollama, poe, stability, wandb)
- Provider registry audit: corrected env var names for 5 providers (cometapi, fal_ai, gradient_ai, jina_ai, venice)
- Provider registry audit: corrected endpoint lists for 6 providers (cometapi, deepinfra, elevenlabs, jina_ai, mistral, nvidia_nim)
- Added missing `base_url` and `auth` config for 11 previously non-functional providers (amazon_nova, baseten, compactifai, datarobot, docker_model_runner, duckduckgo, langgraph, lemonade, v0, vercel_ai_gateway, zai)
- Added 18 stub/infrastructure providers to `complex_providers` list to prevent incorrect config-driven routing
- Added `nanogpt` param mapping (`max_completion_tokens` → `max_tokens`)

## [1.2.1] - 2026-04-17

### Added

- `LlmClientRaw` trait with `_raw` variants of all `LlmClient` methods, returning `RawExchange<T>` that exposes the final request body and raw provider response before normalization ([#13](https://github.com/xberg-io/liter-llm/issues/13))
- `RawExchange<T>` and `RawStreamExchange<S>` types for wire-level debugging and custom parsing
- MCP & IDE integration documentation with setup guides for VS Code, GitHub Copilot, Claude Desktop, Cursor ([#12](https://github.com/xberg-io/liter-llm/issues/12))

### Fixed

- Docker image now published to `ghcr.io/xberg-io/liter-llm` ([#11](https://github.com/xberg-io/liter-llm/issues/11))
- Docker publish workflow timeout increased from 60 to 360 minutes (multi-arch Rust builds via QEMU were timing out)
- Bedrock `build_url` tests no longer flake due to `BEDROCK_CROSS_REGION` env var race condition

## [1.2.0] - 2026-04-07

### Added

- Local LLM provider support: Ollama, LM Studio, vLLM, llama.cpp, LocalAI, llamafile -- use any local inference engine via OpenAI-compatible API
- Docker Compose setup for local LLM integration testing with Ollama
- Integration test suite for local LLM providers

### Fixed

- PHP `onError` hook now passes a proper `\Exception` object instead of a plain string (PHP strict types requires `\Throwable`)
- README templates fixed for rumdl compliance (MD040 code fence language, MD031 blank lines, MD032 list spacing, MD020 closed headings)
- Added 404 to all POST endpoint OpenAPI specs (model not found on default model names)
- Homebrew badge added to all READMEs

## [1.1.1] - 2026-03-29

### Fixed

- Java Maven plugins downgraded to 3.x stable (was 4.0.0-beta, incompatible with Maven 3.9.x CI)
- PHP hook isolation (per-client instead of global), budget per-model enforcement, onError hook invocation, shutdown segfault
- PHP e2e tests set `max_retries=0` to prevent retry delays on mock 500s
- OpenAPI spec: added 400/415/422/503 status codes to all endpoints for schemathesis compliance
- `first_client()` returns 503 Service Unavailable instead of 500 for "no models configured"
- Schemathesis CI checks aligned (removed `content_type_conformance`, `not_a_server_error`)
- Docker cache: per-platform `TARGETARCH` cache IDs prevent multi-arch build races

### Added

- Homebrew formula: `brew tap xberg-io/tap && brew install liter-llm`
- Homebrew bottle builds (arm64_sequoia) in publish workflow
- `liter-llm-proxy` and `liter-llm-cli` added to crates.io publish pipeline
- Installation docs: CLI/Docker/Homebrew tabs
- `scripts/publish/upload-homebrew-bottles.sh` and `ensure-github-release-exists.sh`

## [1.1.0] - 2026-03-29

OpenAI-compatible LLM proxy server with CLI, MCP tool server, and Docker support.

### Proxy Server (`liter-llm-proxy`)

- **22 REST endpoints** — full OpenAI-compatible API surface: chat completions (streaming + non-streaming), embeddings, models, images, audio (speech + transcription), moderations, rerank, search, OCR, files CRUD, batches CRUD, responses CRUD, health
- **Tower middleware stack** — reuses core middleware: cache, rate limit, budget, cost tracking, cooldown, health check, tracing
- **Virtual API keys** — in-memory key store with per-key model restrictions, RPM/TPM limits, budget limits
- **Model routing** — name-based routing to provider deployments, wildcard aliases, deterministic default client
- **OpenDAL file storage** — configurable backend (memory, S3, GCS, filesystem) for file operations
- **SSE streaming** — chat completion chunks proxied as Server-Sent Events with `[DONE]` sentinel
- **OpenAPI 3.1** — utoipa-generated spec served at `/openapi.json` with bearer auth security scheme
- **TOML configuration** — `liter-llm-proxy.toml` with env var interpolation (`${VAR}`), auto-discovery, `deny_unknown_fields`
- **CORS** — configurable origins from config (default: allow all)
- **Graceful shutdown** — SIGINT/SIGTERM handling via `tokio::signal`

### MCP Server (`rmcp`)

- **22 tools** — full parity with REST API: chat, embed, list_models, generate_image, speech, transcribe, moderate, rerank, search, ocr, file CRUD (5), batch CRUD (4), response CRUD (3)
- **Transports** — stdio (default) and HTTP/SSE via `StreamableHttpService`
- **Parameter schemas** — `schemars::JsonSchema` derives for MCP tool discovery

### CLI (`liter-llm`)

- `liter-llm api` — start proxy server with config, host/port overrides, debug logging
- `liter-llm mcp` — start MCP server with stdio or HTTP transport
- 3-tier config precedence: CLI flags > env vars > config file > defaults

### Docker

- Multi-stage build: `rust:1.91-bookworm` builder, `cgr.dev/chainguard/glibc-dynamic` runtime (35MB)
- Non-root execution, OCI labels, port 4000 exposed
- `ENTRYPOINT ["liter-llm"]`, `CMD ["api", "--host", "0.0.0.0", "--port", "4000"]`

### Testing

- **74 unit tests** — config parsing, error mapping, auth key store, service pool, file store, streaming
- **32 integration tests** — auth middleware, chat/embedding/models routes, error propagation, CORS, health, OpenAPI
- **12 proxy e2e fixtures** — chat (basic + streaming), embeddings, models, auth errors, upstream errors, health, images, moderation, reranking
- **Schemathesis** — contract testing against OpenAPI spec via Docker (`task proxy:schemathesis`)

### CI/CD

- `.github/workflows/ci-docker.yaml` — build + health test + schemathesis contract tests
- `.github/workflows/publish-docker.yaml` — multi-arch (amd64/arm64) publish to `ghcr.io/xberg-io/liter-llm`
- Taskfile: `proxy:test`, `proxy:schemathesis`

## [1.0.0] - 2026-03-28

Initial stable release. Universal LLM API client with native bindings for 11 languages and 142+ providers.

### Core

- `LlmClient` trait with chat, chat_stream, embed, list_models, image_generate, speech, transcribe, moderate, rerank, search, ocr
- `FileClient`, `BatchClient`, `ResponseClient` traits for file/batch/response operations
- `DefaultClient` with reqwest + tokio, SSE streaming, retry with exponential backoff
- `ManagedClient` with composable Tower middleware stack
- 142 LLM providers embedded at compile time from `schemas/providers.json`
- Per-request provider routing from model name prefix (e.g. `anthropic/claude-sonnet-4-20250514`)
- `secrecy::SecretString` for API keys (zeroized on drop, never logged)
- TOML configuration file loading with auto-discovery (`liter-llm.toml`)
- Custom provider registration at runtime

### Middleware (Tower)

- **CacheLayer** — in-memory LRU + pluggable backends via `CacheStore` trait
- **OpenDAL cache** — 40+ storage backends (Redis, S3, GCS, filesystem, etc.) via Apache OpenDAL
- **BudgetLayer** — global + per-model spending limits with hard/soft enforcement
- **HooksLayer** — request/response/error lifecycle callbacks with guardrail pattern
- **CooldownLayer** — circuit breaker after transient errors
- **ModelRateLimitLayer** — per-model RPM/TPM rate limiting
- **HealthCheckLayer** — background health probing
- **CostTrackingLayer** — per-request cost calculation from embedded pricing registry
- **TracingLayer** — OpenTelemetry GenAI semantic convention spans
- **FallbackLayer** — automatic failover to backup provider
- **RouterLayer** — multi-deployment load balancing (round-robin, latency, cost, weighted)

### Language Bindings

All bindings expose the full API surface with language-idiomatic conventions:

- **Python** (PyO3) — async/await, typed kwargs, full .pyi stubs
- **TypeScript / Node.js** (NAPI-RS) — camelCase, .d.ts types, Promise-based
- **Rust** — native, zero-cost
- **Go** (cgo) — FFI wrapper with build tags, `context.Context` support
- **Java** (Panama FFM) — JDK 25+, `AutoCloseable`, builder pattern
- **C# / .NET** (P/Invoke) — async/await, `IAsyncEnumerable` streaming, `IDisposable`
- **Ruby** (Magnus) — RBS type signatures, Enumerator streaming
- **Elixir** (Rustler NIF) — `{:ok, result}` tuples, OTP-compatible
- **PHP** (ext-php-rs) — PHP 8.2+, JSON in/out, PIE packages
- **WebAssembly** (wasm-bindgen) — browser + Node.js, Fetch API
- **C / FFI** (cbindgen) — `extern "C"` with opaque handles

### Authentication

- Static API keys (Bearer, x-api-key)
- Azure AD OAuth2 client credentials
- Vertex AI service account JWT
- AWS STS Web Identity (EKS/IRSA)
- AWS SigV4 signing for Bedrock

### Provider Transforms

- Anthropic: message format, tool use v1, thinking blocks, max_tokens default
- AWS Bedrock: Converse API, EventStream binary framing, cross-region routing
- Vertex AI: Gemini format, embedding `:predict` endpoint
- Google AI: embedding/list_models response transforms
- Cohere: citation handling
- Mistral: API compatibility
- `param_mappings` for config-driven field renaming (8 providers)

### Documentation

- MkDocs Material site at docs.liter-llm.xberg.io
- 170+ code snippets across 10 languages
- 11 API reference docs with full method coverage
- Usage pages: Chat & Streaming, Embeddings & Rerank, Media, Search & OCR, Files & Batches, Configuration
- TOML configuration reference
- llms.txt (218 lines) with capabilities, examples, provider list
- Skills directory (4,072 lines) for Claude Code integration
- README generation from Jinja templates via `scripts/generate_readme.py`

### Testing

- 500+ unit and integration tests
- Middleware stack composition tests (cache + budget + hooks + rate limit + cooldown)
- Per-request provider routing tests
- File/batch/response CRUD operation tests
- Concurrency tests (budget atomicity, cache contention, rate limit fairness)
- Redis cache backend integration tests (Docker Compose)
- Live provider tests for 7 providers (OpenAI, Anthropic, Google AI, Vertex AI, Mistral, Azure, Bedrock)
- Smoke test apps for all 10 languages against real APIs
- E2E test generation from JSON fixtures across all languages
- Contract test fixtures for binding API parity

### CI/CD

- Multi-platform publish pipeline: crates.io, PyPI, npm, RubyGems, Hex.pm, Maven Central, NuGet, Packagist, Go FFI, PHP PIE
- Pre-commit hooks: 43 linters across all languages
- Post-generation formatting in e2e-generator
- Version sync script across 27+ manifests with README regeneration

### Previous RC Releases

<details>
<summary>Release candidate history (rc.1 through rc.9)</summary>

- **rc.1** (2026-03-27): Initial release — core crate, 11 bindings, e2e generator
- **rc.2** (2026-03-27): Packaging fixes for crates.io, RubyGems, Elixir NIF, Node NAPI, publish workflow
- **rc.3** (2026-03-27): Cache, budget, hooks middleware; custom providers; TDD e2e fixtures
- **rc.4** (2026-03-28): Shared bindings-core crate; camelCase conversion; real streaming across all bindings
- **rc.5** (2026-03-28): OpenDAL cache; search/OCR endpoints; full middleware wiring; Go/Java/C# FFI rewrites; serde deny_unknown_fields; documentation overhaul
- **rc.6** (2026-03-28): Full API documentation coverage; Rust crate README; version sync improvements
- **rc.7** (2026-03-28): Binding parity (5 middleware params + search/ocr in all 10); contract test fixtures; skills directory; PHP PIE packages
- **rc.8** (2026-03-28): CI fixes (PHP publish, crate order, Maven GPG, Ruby deps, Bedrock test)
- **rc.9** (2026-03-28): Live provider tests; Anthropic/Bedrock/Google streaming fixes; TOML config loading; per-request provider routing; integration test suite

</details>

[1.4.0]: https://github.com/xberg-io/liter-llm/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/xberg-io/liter-llm/compare/v1.2.2...v1.3.0
[1.2.2]: https://github.com/xberg-io/liter-llm/compare/v1.2.1...v1.2.2
[1.2.1]: https://github.com/xberg-io/liter-llm/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/xberg-io/liter-llm/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/xberg-io/liter-llm/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/xberg-io/liter-llm/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/xberg-io/liter-llm/releases/tag/v1.0.0
