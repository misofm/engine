# Bound route gain and matrix values

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).
Code anchors verified on `main` at `6fb211594`.

## Mission

Give every route the bounded domain the owner agreed on 2026-10-03: `gain_db` in `[-144, 24]` dB,
each `channel_matrix` coefficient in `[-1, 1]`, and a subnormal folded coefficient flushed to
`+0.0`. One domain on every path: the session, the compiler's lowering, and every live producer.

## Owner direction

Owner question Q2 of decision 13 (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, "Owner
questions"): "This makes sense to add bounded min/max for these values right?" -- agreed on
2026-10-03 with the bounds above. This is the former deferred item O11 (DESIGN section 10,
`docs/handoffs/submix-sends-2026-10-02/DESIGN.md`), minus its parameter-metadata half (see
non-goals). It is outside *Submix strips and live aux sends* (#1196), whose closure is fixed.

## Today

- **Session.** `validate_routes` (`crates/session/src/validate.rs:688-726`) checks a route's
  `gain_db` (`:704`) and its four matrix coefficients (`:716-724`) with `validate_finite` only. So
  a route gain is accepted wherever its linear gain is a normal finite `f32` (about -758 to +770
  dB, plus anything low enough to round to exactly 0), and a coefficient may be any finite value.
  The bounded precedent is the pan's `validate_finite_range(.., -1.0, 1.0, ..)` (`:464-465`), which
  refuses `numeric.out_of_schema_range` (`validate_finite_range`, `:1065-1084`).
- **Compiler and live producers.** `graph_compiler::route_values` (`crates/graph-compiler/src/ids.rs:309`)
  backs both `route_coefficients` (`:341`) and the lowering (`crates/graph-compiler/src/compile.rs`,
  the route loop): it refuses (`RouteValueError::Domain`) a value `route_transform` refuses or an
  open fold that overflows. host-core's send producer (`RouteControlProducer::record`,
  `crates/host-core/src/route_controls.rs:86`) and host-web's admission (`COMMAND_REASON_DOMAIN`,
  `hosts/host-web/src/lib.rs:1033`, refused at `:4970-4973`; doc at `:902-906`) refuse a live value through it.
- **The fold.** `graph::gated_route_coefficients` (`crates/graph/src/lib.rs:785`) is the one
  derivation of a route's four constants, for the bound op, the fold lane and every live record.
  It accepts a subnormal product: `route_coefficients(-120.0, [1.2e-38, 0, 0, 1], ..)` returns a
  first coefficient near `1.3e-44` (#1215 verdict INFO). Under the new bounds a subnormal product
  is still reachable (a coefficient below about `1.9e-31` at -144 dB).
- **SDK.** The builder checks a route's `gainDb` and matrix with `f32` only
  (`sdk/src/core/session.ts:862-867`, normalised at `:1635-1636`); `enginectl` passes a route
  request's values to the builder (`sdk/src/cli/session-request.ts`). Live `RouteEdits`
  (`sdk/src/core/live-controls.ts:759-793`) checks finiteness and leaves the domain to the engine's
  `domain` refusal (`docs/handoffs/submix-strips-and-sends/APP-LIVE.md:66-69`).
- **Migration.** All 514 routes in checked-in JSON documents (`git ls-files '*.json'`; objects with
  `gain_db` and `channel_matrix` in a `routes` array, counted at `6fb211594`) are inside the new
  domains, so no document changes. The writer corpus's `ll = 1.25` is a **track** matrix
  and is not a route.

## Decisions

- **D1. Session.** `validate_routes` uses `validate_finite_range` for `gain_db` with `[-144, 24]`
  at `$.routes[<i>].gain_db`, and for each coefficient with `[-1, 1]` at
  `$.routes[<i>].channel_matrix.<ll|lr|rl|rr>`. A non-finite value keeps `numeric.non_finite`.
- **D2. One domain everywhere.** `route_values` refuses the same domains with
  `RouteValueError::Domain`, so a session value, a compiled value and a live record value share one
  check. A live record outside the domain is refused with reason `domain`, as today.
- **D3. Flush.** `gated_route_coefficients` returns `+0.0` for any folded product that is
  subnormal (of either sign). It stays the one derivation, so a bound op, a fold lane and a live
  record flush identically. (It is a `const fn`; if `f32::is_subnormal` is not usable there at the
  pinned toolchain, drop `const` and record it -- #1215 verdict NIT 3 notes `const` is part of its
  interface.)
- **D4. SDK.** The builder refuses an out-of-domain `gainDb` or matrix coefficient with
  `MisoUsageError` code `numeric.out_of_schema_range` at the route's path, mirroring D1;
  `enginectl` inherits it. Live `RouteEdits` keeps leaving the domain to the engine.
- **D5. Docs.** `docs/SESSION_SCHEMA_V1.md`'s route paragraph states the domains and the flush;
  APP-LIVE's "Live sends" names the domain; the `author-session` skill's route bullet names it.

## Authorized paths

- `crates/session/src/validate.rs` (`validate_routes`), and session tests.
- `crates/graph-compiler/src/ids.rs` (`route_values`, docs), `crates/graph-compiler/tests/route_coefficients.rs`.
- `crates/graph/src/lib.rs` (`gated_route_coefficients` and its doc) and graph tests.
- Tests that draw route values outside the new domain (for example `route_coefficients.rs`'s
  +700 dB overflow cases and host-web's gate-2 `3.0e38` matrix case): rescoped to the new domain,
  each with a reason.
- `sdk/src/core/session.ts` (route validation), SDK evals.
- `docs/SESSION_SCHEMA_V1.md`, `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`, the
  `author-session` skill, and this spec's record sections.

## Non-goals

- Publishing route rows in the parameter metadata (O11's second half): file it when an SDK
  consumer needs route metadata.
- Changing track fader, trim or matrix domains, or the indexed ramp (a ramp between two in-domain
  targets may still pass through a subnormal; that is #1219's documented, inaudible case).
- A C ABI live path (#1225 inherits D2 through `route_coefficients`).

## Hazards

- A session test or fixture generator that draws route gains or coefficients from a wider range
  must be narrowed, not deleted; record each.
- D3 can move bits only for a route whose folded product is subnormal. Confirm that no checked-in
  render digest, graph fixture or console-workload count moves; if one does, stop and record it.
- The order of refusals: a non-finite value still refuses `numeric.non_finite`, never
  `numeric.out_of_schema_range`.

## Objective gates

1. Boundary values on the session path: `24.0`, `-144.0`, `1.0` and `-1.0` accepted; `24.5`,
   `-144.5`, `1.5` and `-1.5` refused with `numeric.out_of_schema_range` at the exact path; NaN and
   infinity keep `numeric.non_finite`.
2. The same boundary values through `route_coefficients` (the live path): accepted or refused
   `Domain` exactly as gate 1. Red if the two paths' domains differ.
3. A browser live send edit to `gainDb(24.5)` and to a `matrix` coefficient `1.5` is refused with
   reason `domain` and moves nothing; `24.0` and `1.0` are admitted.
4. D3: `gated_route_coefficients` of a route whose product is subnormal returns `+0.0` in that
   position (both signs), and a session with such a route renders bit-identical to one whose
   coefficient is `0.0`.
5. SDK: the builder refuses the gate-1 out-of-domain values at the route's path and accepts the
   boundaries; `check-sdk-types.sh`, `check-sdk-headless.sh <A>` and `sdk-package.sh check <A>`
   pass.
6. Nothing checked in moves: `graph_fixture -- --check`, `check-graph-determinism.sh`,
   `check-builtins-fixtures.sh`, `check-console-fixtures.sh` and
   `check-browser-expected-resources.py --artifacts <A>` pass unchanged.
7. Workspace gates: `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets
   --all-features -- -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`, test-debug-a and test-debug-b, and the policy check/test pairs.
7a. Worklet and cross-target gates (K3 verdict MINOR-5: `graph` and `graph-compiler` compile into
   the shipped worklet and the iOS product crates): `bash scripts/build-web-audioworklet.sh
   --named-twin <B> <A>`, then `bash scripts/check-web-audioworklet.sh <A>
   <B>/miso-engine-v1-audio-worklet.simd128.named.wasm` and `bash scripts/test-web-audioworklet.sh`
   exit 0 (gate 6 runs `check-browser-expected-resources.py --artifacts <A>` on the same `<A>`);
   and `bash scripts/check-cross-targets.sh` exits 0, with no `memset_pattern16` ceiling raised.
8. Each new test has a one-sentence test-value answer and a recorded mutation run (for example:
   `validate_routes` back to `validate_finite` for `gain_db` turns gate 1 red; the flush removed
   turns gate 4 red).

## Evidence

- Each gate's command and exit status from the PR head; the mutation table; the list of rescoped
  tests with reasons.

## Dependencies

- None blocking. Schedule after batch K3 of #1196 (it touches `route_coefficients`, which K3's live
  sends use).

## Attempt record

### Attempt 1 (implementer, 2026-10-05) -- implementation committed, gates incomplete (host disk)

Commit `d1a9daf30` on `codex/d15-stream-j`.

- **D1.** `validate_routes` checks `gain_db` with `validate_finite_range(.., -144, 24, ..)` and each
  coefficient with `[-1, 1]`; non-finite keeps `numeric.non_finite` (the helper's own order).
- **D2.** `route_values` refuses the same inclusive domain (`RangeInclusive::contains`, false for
  NaN) before `route_transform`. Inside the domain no fold can overflow (largest product about
  `15.85`), so the former open-fold finiteness check was unreachable and is removed; doc updated.
  The bounds are literal constants in `session/src/validate.rs` and `graph-compiler/src/ids.rs`
  (a shared constant would need `session/src/lib.rs`, outside the authorized paths); gate 2's test
  is the structural guard that they agree.
- **D3.** `gated_route_coefficients` folds through a `const fn fold` that returns `+0.0` for a
  subnormal product of either sign. `f32::is_subnormal` is `const` at 1.97.1, so the function
  stays `const fn`.
- **D4.** SDK `route()` and normalisation use `routeNumber` (f32 then inclusive bound, code
  `numeric.out_of_schema_range`) for `gainDb` and each matrix key; the now-unused `matrixRecord`
  was removed (`routeMatrixRecord` replaces it). `enginectl` inherits it through the builder.
- **D5.** `SESSION_SCHEMA_V1.md` route paragraph, APP-LIVE "Live sends", author-session skill route
  bullet.

Rescoped tests (each with an in-code reason):

- `route_coefficients.rs` `Draw::coefficient`: `uniform(-4, 4)` -> `uniform(-1, 1)`.
- `route_coefficients.rs` +700 dB overflow block: deleted; replaced by the gate-2 test (overflow is
  unreachable in domain).
- `host-web` `a_refused_send_batch_pushes_nothing_and_keeps_the_mirror`: both `3.0e38` matrix
  cases -> `1.5` (outside `[-1, 1]`), doc bullet updated.

New tests, test value and mutation runs (each mutation applied, red observed, reverted, green):

| Test | Defect it catches | Mutation | Result |
| --- | --- | --- | --- |
| `session/tests/route_domain.rs::route_gain_and_matrix_values_are_bounded_at_their_own_path` (gate 1) | session gain or a coefficient left finite-only, wrong/exclusive bound, wrong path, or range pre-empting `non_finite` (typed and text paths) | M1 `gain_db` back to `validate_finite`; M1b coefficient bound widened to `f32::MAX` | red both (`route_domain.rs:83`) |
| `graph-compiler/tests/route_coefficients.rs::a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it` (gate 2) | live path and session domains differ | M1, M1b (session wider); M2 `route_values` bound removed | red all three (`:389`) |
| `hosts/host-web/src/tests.rs::a_live_send_edit_outside_the_route_domain_is_refused` (gate 3) | browser send admission bounds differ from the session's | M2 | red (`tests.rs:11069`) |
| `route_coefficients.rs::a_subnormal_folded_coefficient_is_positive_zero` (gate 4, coefficients) | subnormal product reaches the bound/pushed constant, or flushes to `-0.0` | M3 flush removed; M3b flush to `-0.0` | red both (`:442`) |
| `host-web tests.rs::a_subnormal_route_renders_as_a_zero_coefficient` (gate 4, render) | subnormal constant renders a nonzero subnormal sample | M3 (`6.25e-43 vs 0e0`); M3b | red both |
| `sdk/test/builder-evals.mjs` "a route's gain and matrix are bounded ..." (gate 5) | builder checks f32 only, exclusive bound, or wrong code/path | M5a `route()` `gainDb` back to `f32`; M5b `route()` matrix back to `f32`; M5c exclusive bound (`<=`/`>=`); M5d refusal without `CODE.outOfRange` | red all four, green on revert (run in the gate-completion pass below) |

Gates run at `d1a9daf30`:

- `cargo fmt --all -- --check`: exit 0.
- `cargo test -p session --test route_domain`, `-p graph-compiler --test route_coefficients`,
  `-p host-web --lib -- send` (15) and `-- subnormal`: all pass.
- `bash scripts/check-sdk-types.sh`: exit 0.

Not run -- **blocked by host disk**: free space on `/` fell from 9.3 GB to 3.8 GB during the
attempt, mostly from other worktrees' concurrent builds (`wt-d15-g-1328`). `test-debug-a` was
stopped mid-compile when free space crossed the 4 GB floor. Outstanding: gate 5's SDK legs
(`check-sdk-headless.sh <A>`, `sdk-package.sh check <A>`, the new builder eval), gate 6
(`graph_fixture -- --check`, `check-graph-determinism.sh`, `check-builtins-fixtures.sh`,
`check-console-fixtures.sh`, `check-browser-expected-resources.py`), gate 7 (clippy, doc,
test-debug-a/b, policy pairs) and gate 7a (worklet build/check/test, cross-targets). Hazard check
(no checked-in digest moves under D3) therefore remains unproven by gate 6; by construction D3 only
moves bits for a route whose folded product is subnormal, and no checked-in route has a coefficient
below `1.9e-31`.

#### Gate completion (same attempt, 2026-10-05) -- all remaining gates run

Not a new attempt: the outstanding gates above were run at `16a5b920b` (attempt-1 implementation
plus #1302 attempt 2's policy scripts), with `<A>`/`<B>` rebuilt from that head by
`bash scripts/build-web-audioworklet.sh --named-twin <B> <A>` (exit 0; shipped module
`1a334cd5...c101d`, 2894346 B). One #1237 defect found and fixed:

- **clippy** (gate 7) refused `crates/session/tests/route_domain.rs` with
  `clippy::type_complexity` on the gate-1 field table under `-D warnings`. Fixed in `784b5d61c` with
  a `type Setter` alias (table, test and assertions unchanged); clippy then exit 0 and
  `route_domain` passes. Gates 5, 6, 7a's worklet legs, fmt and rustdoc ran at `16a5b920b`;
  clippy (rerun), test-debug-a/b, the policy pairs and `check-cross-targets.sh` ran at
  `784b5d61c`. The fix touches only that test file, so no earlier result can move.

| Gate | Command | Exit |
| --- | --- | --- |
| 5 | `check-sdk-headless.sh <A>` (all evals, incl. the new route-domain eval) | 0 |
| 5 | `sdk-package.sh check <A>` | 0 |
| 5 | new builder eval alone, mutations M5a-M5d (table above) | red x4, green on revert |
| 6 | `cargo run -p graph-compiler --bin graph_fixture -- --check` | 0 |
| 6 | `check-graph-determinism.sh` (100/100) | 0 |
| 6 | `check-builtins-fixtures.sh . target/release/audit` (50 files) | 0 |
| 6 | `check-console-fixtures.sh target/release/session_validator` | 0 |
| 6 | `check-browser-expected-resources.py --artifacts <A>` | 0 |
| 7 | `cargo fmt --all -- --check` | 0 |
| 7 | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 101 at `16a5b920b` (above), 0 at `784b5d61c` |
| 7 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | 0 |
| 7 | test-debug-a (qualification.yml's exact command, plus `builtins-compiler --no-run`) | 0 (1432 passed, 0 failed) |
| 7 | test-debug-b (exact command) and `conformance_fixtures -- --check` | 0 (813 passed, 0 failed); 0 |
| 7 | policy check/test pairs: workspace, session, bench, host-core, protocol-control, realtime, lane, rack, builtins, graph, effect-runtime | all 0 |
| 7a | `check-web-audioworklet.sh <A> <B>/...named.wasm` | 0 |
| 7a | `test-web-audioworklet.sh` | 0 |
| 7a | `check-cross-targets.sh` | 0 (the ten `ios-asm-memset-pattern16` rows are #1018's expected failures; no #1237 commit touches any script or ceiling) |

Hazard (D3 moves no checked-in bits): proven by gate 6 -- graph fixtures, console fixtures,
builtins fixtures and the browser expected-resources digests and rows all pass unchanged.


### Attempt 2 (implementer, 2026-10-05) -- J1-1 fixed, J1-4 folded

Attempt 1's verdict was FAIL on MAJOR J1-1: the session (D1) and the SDK builder (D4) accept a
subnormal `channel_matrix` coefficient, but `route_transform` refused it, so the lowering refused
the route at boot as `graph.gain.non_finite` at `$.routes[id=<r>].gain_db` (wrong code, wrong
field) and a live send of the same value was `domain`.

- **Decision followed.** The spec does not decide subnormal inputs separately: D1 bounds a
  coefficient to `[-1, 1]` and D3 flushes every subnormal *product* to `+0.0`. So a subnormal
  coefficient is in the domain on every path. `route_transform`
  (`crates/graph-compiler/src/ids.rs`) now checks coefficients with `is_finite` only; its doc and
  `RouteValueError::Domain`'s doc say so. The gain's finite/subnormal checks stay as defence
  (unreachable in `[-144, 24]` dB, per the verdict's exhaustive sweep). D2's one domain now holds:
  `[-144, 24] x [-1, 1]^4` on the session, the lowering, the live path and the SDK builder.
- **SDK builder.** No change needed: `routeNumber` already accepts a subnormal (`Math.fround`,
  finite, in bounds). The eval now proves the builder and the engine agree on it end to end.
- **J1-4 (NIT).** `route_coefficients.rs`'s module doc no longer says the file "refuses a fold
  that overflows".
- **J1-5 (NIT), recorded.** The lowering's refusal arm (`crates/graph-compiler/src/compile.rs`,
  `let Ok(transform) = route_values(..) else { .. "graph.gain.non_finite" .. }`) is now
  unreachable from a validated session: every value the session accepts is in `route_values`'
  domain. It is kept as defence (a lowering must not panic on an unchecked model) and is untested
  by construction. Its gate independence (#1216 D2) still holds structurally: `route_values` takes
  no gate, and gate 2's 8-gate loop holds it on the live path.
- **Open for the S0/root coordinator, not implemented here (outside the authorized paths):**
  - **J1-2 (MINOR).** The route domain is still two literal copies (`session/src/validate.rs`,
    `graph-compiler/src/ids.rs`); one exported constant would live in `crates/session/src/model.rs`
    (exported by `pub use model::*`), which this spec does not authorize. Gate 2 remains the drift
    guard.
  - **J1-3 (MINOR).** `host_core::RouteControlRecord` (re-export of
    `graph::RouteControlRecord::new`, public) lets an embedder push coefficients that never went
    through `route_values` or D3. No shipped host does; needs a follow-up or folding into #1225.

Tests extended (no new test functions):

| Test | Change | Defect it now also catches | Mutation | Result |
| --- | --- | --- | --- | --- |
| `route_coefficients.rs::a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it` (gate 2) | adds `±1e-45`, `±f32::MIN_POSITIVE / 2`, `±f32::MIN_POSITIVE` to the gain and to every coefficient | the live path or the lowering refuses a subnormal value the session accepts | M6: restore `route_transform`'s `!v.is_subnormal()` coefficient check | red (`:396`, `channel_matrix.ll = 1e-45`: live `Err(Domain)`, session `None`); green on revert |
| `route_coefficients.rs::a_subnormal_folded_coefficient_is_positive_zero` (gate 4) | adds a 0 dB case with coefficients `1e-45`, `-1e-45`, `-f32::MIN_POSITIVE / 2` | a subnormal coefficient is refused at boot instead of compiling and binding `+0.0` | M6 | red (`:449`, the compile panic); green on revert |
| `sdk/test/builder-evals.mjs` "a route's gain and matrix are bounded ..." (gate 5) | accepted set adds `gainDb ±1e-45` and each coefficient `±1e-45`, `±2^-127`; with the asset, each must also `validate()` ok | the builder accepts a subnormal value the engine then refuses at boot (the J1-1 symptom), or the builder refuses it | the eval run against the attempt-1 module (`/tmp/claude-1002/v1237/A`, built from `e9dc1a4b2` by the verifier) | red (`validate` ok `false`); green against this attempt's module; builder-only (`MISO_ENGINE_SDK_SKIP_ASSET=1`) green |

Gates run at the attempt-2 tree (`<A>`/`<B>` rebuilt by `build-web-audioworklet.sh --named-twin`,
exit 0, shipped module `d6b7b8df...51d4b0`, 2894073 B):

| Gate | Command | Exit |
| --- | --- | --- |
| focused | `cargo test -p session -p graph-compiler -p graph -p host-web -p host-core` (744 passed) | 0 |
| 5 | `check-sdk-headless.sh <A>` (361/361), `sdk-package.sh check <A>`, `check-sdk-types.sh` | 0, 0, 0 |
| 6 | `graph_fixture -- --check`; `check-graph-determinism.sh` (100/100); `check-browser-expected-resources.py --artifacts <A>` | 0, 0, 0 |
| 7 | `cargo fmt --all -- --check`; clippy `-D warnings` (workspace, all targets/features); `RUSTDOCFLAGS='-D warnings' cargo doc` | 0, 0, 0 |
| 7 | test-debug-a (exact command, plus `builtins-compiler --no-run`): 1432 passed, 0 failed | 0 |
| 7 | policy check/test pairs: bench, builtins, effect-runtime, graph, host-core, lane, protocol-control, rack, realtime, session, workspace (22/22) | all 0 |
| 7a | `check-web-audioworklet.sh <A> <B>/...named.wasm` | 0 |

Not rerun in attempt 2 (attempt-1 results stand; this attempt changes only `route_transform`'s
coefficient check, which no checked-in route reaches with a subnormal value, and test files):
test-debug-b, `check-builtins-fixtures.sh`, `check-console-fixtures.sh`, `test-web-audioworklet.sh`,
`check-cross-targets.sh`.
