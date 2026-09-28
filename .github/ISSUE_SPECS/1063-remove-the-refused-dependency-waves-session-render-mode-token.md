# Remove the refused `dependency_waves` session render-mode token

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 8): remove it. The dependency-wave multicore scheduler was removed from the engine (AGENTS.md: reintroducing multicore render needs a new issue that re-earns it). The session grammar still knows the token and refuses it with a specific error (`crates/session/src/model.rs:97`; `docs/SESSION_SCHEMA_V1.md:57-59`; tests `crates/session/tests/render_mode_tiers.rs:59,72`). Source: the test-value audit, ruling R7, and its verification.

## Smallest closable slice

Remove the token from the session model, parser, canonical writer and protocol encoding, so a session naming it is refused as an unknown value; update `docs/SESSION_SCHEMA_V1.md`; delete or adapt the tests that exercised the known-but-refused behaviour; check the SDK (`sdk/`) and generated surfaces for the spelling.

## Objective gates

- A session naming `dependency_waves` is refused (unknown value) at parse, with a test; `single_thread` sessions and every fixture are unchanged.
- Protocol encoding and wire vectors stay consistent (no silent renumbering of other tokens; state what changed).
- All four targets build; console digests unchanged; clippy, fmt, the session and step-vocabulary policies and the SDK tests pass.

## Attempt 1 evidence

Terra, attempt 1, branch `codex/1063-remove-dependency-waves-token` from `codex/batch-slim-1`
(`c867ec2a`). Implementation commit `6ed5a14d`.

### What changed

`session::RenderMode` has one token, `single_thread`. Removed:

| item | what it covered |
|---|---|
| `RenderMode::DependencyWaves` (`"dependency_waves"`, wire code `2`) | the declaration of the removed native dependency-wave scheduler |
| `DiagnosticCode::RenderModeUnsupportedAtLaunch` (`render_mode.unsupported_at_launch`) | the typed refusal of that token; nothing else emitted it |
| the `validate_session` branch that emitted it | the refusal at parse, typed compile and canonical write; the type can no longer hold another mode |
| the protocol encode and decode arms for code `2` | the BTLV spelling of the token |
| test `dependency_waves_rejects_with_one_stable_diagnostic_at_every_entry_point` | the known-but-refused claim at three entry points, which this issue retires |
| test `dependency_waves_is_still_a_known_token_and_not_an_unknown_enum` | the known-token-versus-`InvalidEnum` contrast, which this issue inverts |

Added:

- `crates/session/tests/render_mode_tiers.rs::retired_dependency_waves_is_an_unknown_enum_value`:
  `dependency_waves` and `wave_farm` each refuse at parse with exactly one `schema.invalid_enum`
  at `$.render_profile.mode`, message `expected one of: single_thread`.
- `crates/protocol/src/session_wire/tests.rs::render_profile_mode_decodes_only_single_thread`:
  code `1` decodes to `SingleThread`; `0`, `2`, `3` and `255` refuse with `InvalidTlv`.

Docs: `docs/SESSION_SCHEMA_V1.md` (the render-mode paragraph), `docs/session-v1.schema.json`
(`"mode": { "const": "single_thread" }`, the style of `sample_format`),
`docs/CONTROL_PROTOCOL_REGISTRY.md` (code `2` retired, never reallocated), and the checked-in
`author-session` skill's token list.

Code diff against `c867ec2a`: 64 lines added, 73 removed, in 10 files (Rust 54 added, 61 removed;
26 of the Rust additions are the new protocol test).

### Gates

- **Refusal at parse:** the new session test above. `single_thread` sessions are unchanged: no
  fixture was edited, `single_thread_parses_compiles_and_canonicalizes` and every fixture test
  pass.
- **Wire:** `single_thread` keeps code `1` (`token_tables.rs` still pins `wire() == index + 1`),
  so every golden and wire vector is byte-identical; the only change is that code `2` now refuses.
  No other token moved. `check-protocol-wasm-parity.sh`: ok (scalar + simd128).
- **SDK and generated surfaces:** no SDK source or generated file spells the token (the SDK emits
  only `single_thread`). `check-sdk-generated.sh`, `check-sdk-types.sh`, `sdk-package.sh check`
  and `check-sdk-headless.sh` (284/284 evals) pass against this commit's module.
- **Targets:** `cargo check --workspace --all-targets --all-features` (native x86-64); wasm
  `simd128` `cargo check -p host-web -p protocol -p session -p target-smoke`; `cargo check --target
  aarch64-apple-ios` and `--target aarch64-linux-android` of `capi session protocol host-core
  engine source`: all pass. `check-cross-targets.sh`: PASS (x86-64-v3, wasm scalar and simd128,
  armv7 refused). Its first run failed only on this attempt's own `sdk/dist` staging output left by
  `sdk-package.sh check`; removed, it passed.
- **Tests:** `cargo test --workspace` (dev): 2,417 passed, 0 failed, 40 ignored across all non-doc
  targets. The workspace-wide doctest step fails for 8 packages (`capi`, `console-workload`,
  `graph-compiler`, `host-core`, `host-web`, `native-pcm-runner`, `parameter-metadata`,
  `session-validator`) with `E0463 can't find crate` on an `--extern` rlib: a feature-unification
  artifact of doctests under `--workspace`, not source. Each passes with `cargo test --doc -p <pkg>`,
  and CI runs `--all-targets`, which has no doctest step. Release: `cargo test --release -p session
  -p protocol -p console-workload`: 251 passed, 0 failed. The console-workload digests are unchanged
  (its tests pass in dev and release).
- **Test inventory:** `cargo test --workspace -- --list` before and after: 2,466 and 2,466. The
  diff is exactly the two removed tests and the two added ones; no test guarding a live claim was
  lost.
- **Lint and policy:** `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt
  --check` clean. `check-session-policy.sh`, `check-step-vocabulary.py`,
  `check-workspace-policy.sh`, `check-capi-abi.sh`, `run-wasm-gates.sh` (native, wasm scalar,
  simd128, V8 spill) and `check-web-audioworklet.sh` (on this commit's module) pass, as do the
  static `check-*.sh` scripts and every argument-free `check-*.py`.

### AudioWorklet artifact

Not re-pinned. The pin (`476e58ad…`) was already stale at the branch base: `c867ec2a` builds
`f7bd75ca…` (3,486,194 bytes). This commit builds `065528b3…` (3,485,448 bytes, 746 smaller): the
module no longer carries the `dependency_waves` token, the `render_mode.unsupported_at_launch`
code and its message, or the validation branch. The batch boundary re-pins.
