# Route the builtins fader domain checks through checked_fader_gain

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).

Refactor follow-up of *Classify a committed session delta as a live track fader, mute and pan
update or a rebuild* (#1255, slice of umbrella #1053). Its verdict (Sol, attempt 1, NIT-1;
`docs/handoffs/live-updates-1053/1255-attempt1.md`) found that two preparation-side checks still
spell the `[-144, 24]` dB fader domain themselves instead of calling the shared authority.
Recorded in `docs/handoffs/live-updates-1053/README.md`, "Follow-up candidates". A pure refactor:
no rendered bit and no diagnostic changes.

## Problem (verified on `main` at `d2fe0555a`)

- **The authority.** `checked_fader_gain` (`crates/builtins/src/lib.rs:4292-4297`, `pub`) refuses a
  `db` that is not finite or is outside `-144.0..=24.0` with `BuiltinParameterError::GainDomain`,
  then returns `db_gain(db)` (`:5357-5364`). Three callers share it: `fader_lanes`
  (`:4318-4328`, used by `FaderMuteRampBuiltins::new` at `:4178`), the render setters
  `BuiltinFaderBank::set_fader_db` (`:3879`) and `FaderMuteRampBuiltins::set_fader_db`
  (`:4197`), and host-core's live-delta classifier (`crates/host-core/src/live_delta.rs:518`).
- **Copy 1: `prepare_sections`** (`crates/builtins/src/lib.rs:3274-3326`), which every
  `BuiltinChain::new` runs (`:3201`) and which session preparation reaches through
  `crates/builtins-compiler/src/lib.rs:3258`, `:3488`, `:3552` and `:3559`.
  - Its per-lane loop (`:3282-3295`) checks, for left then right: `trim_db` in domain, then
    `fader_db` in domain (`:3286`, spelled `!(-144.0..=24.0).contains(&lane.fader_db)`), then the
    two filter cutoffs, then the filter order. The first failure returns.
  - Its `fader` closure (`:3304-3309`) then builds each `FaderLane` with `db_gain(params.fader_db)?`
    and `faders` (`:3314`) pairs them. That is exactly what `fader_lanes` computes.
- **Copy 2: `gain_path`** (`crates/builtins-compiler/src/lib.rs:5031-5044`), which names the field
  a `GainDomain` refusal points at (`parameter_diagnostic`, `:5004-5029`). For left then right it
  returns the `trim_db` path, then the `fader.{lane}_db` path (`:5040`, spelled
  `!fader.is_finite() || !(-144.0..=24.0).contains(&fader)`). builtins-compiler already depends on
  builtins (`crates/builtins-compiler/Cargo.toml:17`).
- **The doc says so.** `checked_fader_gain`'s doc (`crates/builtins/src/lib.rs:4282-4287`) names
  the two copies as still spelled separately.
- **Why it matters.** The copies agree today. If the fader domain ever changes in one place, a
  value could pass the live path and fail preparation (or the reverse), so a live commit would
  leave a committed model that its own rebuild refuses (#1053 D9), or a refusal would point at the
  wrong field.

### Why the routing is exact

`checked_fader_gain(db)` errs exactly when the spelled check fails. Inside the domain, `db_gain`
cannot err: it errs only on a non-normal coefficient, and `[-144, 24]` dB maps to about
`[6.3e-8, 15.85]`, all normal `f32`. Inside the domain it returns `db_gain(db)`, the value the
`fader` closure computes today. So every `FaderLane` and every diagnostic is unchanged.

### Other spellings of the same range (not in scope, see Non-goals)

- `crates/builtins/src/lib.rs:539-540`: `fader_db`'s declared descriptor domain
  (`BUILTIN_PARAMETER_DESCRIPTORS`), metadata rather than a check.
- `crates/builtins/src/lib.rs:3284` and `:4311` (`checked_trim_gain`), and
  `crates/builtins-compiler/src/lib.rs:5037`: `trim_db`, a separate parameter with the same range.
- `crates/session/src/validate.rs:584` and `crates/session/src/vca.rs:15-17`: the session's own
  VCA fader validation and clamp. `session` does not depend on `builtins`.
- `hosts/host-web/src/lib.rs:4284`, `:4323` (trim) and `:4403` (VCA): browser command admission,
  which spells the range by that file's stated convention (comment at `:4320-4322`).

## Decisions

- **D1. `prepare_sections`.** In the per-lane loop, replace the two `fader_db` clauses of the
  condition at `:3283-3287` with a call `checked_fader_gain(lane.fader_db)?;` placed after the
  `trim_db` check and before the filter checks, so the order of checks, and so which error a
  multiply-invalid lane reports, is unchanged. Replace the `fader` closure and `faders`
  (`:3304-3309`, `:3314`) with `let faders = [fader_lanes(parameters)?];`.
- **D2. `gain_path`.** Replace the `fader` condition at `:5040` with
  `builtins::checked_fader_gain(fader).is_err()`. The `trim_db` condition stays as it is.
- **D3. The doc.** Reword `checked_fader_gain`'s doc (`:4282-4287`) to list its callers, including
  `prepare_sections` and builtins-compiler's `gain_path`, and to drop the sentence about the two
  separate spellings.
- **D4. No new test.** Two existing tests already refuse an out-of-domain fader through both
  copies, and the live side is covered by host-core's
  `the_classifier_domain_is_the_render_setters` (`crates/host-core/tests/live_delta.rs:649`):
  - `deterministic_builtin_compiler_mutations_cover_domains_and_preparation`
    (`crates/builtins-compiler/src/lib.rs:12108`), class 5 (`:12234-12239`): `fader.right_db =
    24.001` refuses `builtin.gain.domain` at `$.tracks[id=vocal].fader.right_db`;
  - `a_vca_member_fader_outside_its_domain_refuses_at_its_own_path` (`:12956`): `fader.left_db =
    24.5` refuses at `$.tracks[id=vocal].fader.left_db`.

  After this change, a defect in `checked_fader_gain`'s domain turns these red as well (gate 2),
  which is the point of having one authority. A new test would catch no defect they miss.

## Authorized paths

- `crates/builtins/src/lib.rs`: `prepare_sections` (D1) and `checked_fader_gain`'s doc (D3) only.
- `crates/builtins-compiler/src/lib.rs`: `gain_path` (D2) only.
- This spec.

## Non-goals

- The `trim_db` checks. Routing `prepare_sections`' trim check through `checked_trim_gain` is the
  same shape, but `checked_trim_gain` is private and `gain_path` could not call it; a follow-up may
  do both together.
- A shared constant for the descriptor's `minimum`/`maximum`.
- `session` and `host-web`. `session` is the schema's own authority below `builtins`. host-web's
  command admission keeps its file's convention; it would also pay a `pow` per admitted command to
  call `checked_fader_gain` only for its domain.
- Any change to the domain, the error variant, the diagnostic code or path, or the check order.

## Hazards

- **Check order.** Moving the fader check out of the lane loop (for example, relying on
  `fader_lanes` alone after the loop) changes which error a session with two bad fields reports:
  a bad right `fader_db` would beat a bad left `hpf_hz`. Keep the in-loop call (D1).
- **Bits.** The change is in preparation only. No render path, kernel or ramp changes, so the
  rendered output and the fixtures must not move; if one moves, the refactor is not what D1 and D2
  say.

## Objective gates

Run every command from the repository root.

1. **Nothing moved.**
   - `cargo test --locked --all-targets -p builtins --features builtins/test-support`
   - `cargo test --locked -p builtins-compiler --features test-support`
   - `cargo test --locked -p host-core --all-targets --features control-provider,test-support`
   - `cargo build --locked --release -p audit -p capi`, then `./target/release/audit capi` on this
     branch and on `d2fe0555a`: the `pcm_digest` field is equal, and allocations, frees, locks and
     syscalls are 0.
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit` passes (the builtins PCM
     fixtures are unchanged).
   - `cargo test --locked --release -p audit -p bench -p console-workload`
2. **One authority (PR evidence, not committed).** In a scratch copy, change
   `checked_fader_gain`'s upper bound from `24.0` to `25.0`. On `d2fe0555a` the two tests of D4
   stay green (the copies still refuse); on this branch both go red. Record both runs.
3. **The browser module.** Build it on this branch and on `d2fe0555a` and report both SHA-256
   digests (it may change: preparation code moved):
   `rm -rf target/ci/qualification-artifacts target/ci/qualification-named-twin && mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && sha256sum target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm`;
   then `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
   `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   (every expected render digest agrees) and `bash scripts/test-web-audioworklet.sh`.
4. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in builtins realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`; 4-lane (NEON) is CI-only.

*Test value.* No new or rewritten test (D4). Gate 2 shows once, as PR evidence, that the existing
preparation tests now guard the shared domain.

## Evidence

- `audit capi`'s `pcm_digest` before and after.
- Gate 2's two runs.
- The module digests before and after.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277). Both edit `crates/builtins`; this
  slice lands after it. #1255 is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
