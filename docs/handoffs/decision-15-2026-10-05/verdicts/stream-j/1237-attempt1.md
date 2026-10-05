VERDICT: FAIL

# #1237 attempt 1: verdict (stream J, commit e9dc1a4b2, base 5304737c1)

Reviewed diff: `git diff 5304737c1 e9dc1a4b2 -- . ':!scripts/check-realtime-policy.sh' ':!scripts/test-realtime-policy.sh' ':!.github/ISSUE_SPECS/1302-*'`
(commits d1a9daf30, a8332e93b, 784b5d61c, e9dc1a4b2). Everything was built and run in the export
`/tmp/claude-1002/v1237/src` (a git archive of e9dc1a4b2) with `CARGO_TARGET_DIR=/tmp/claude-1002/v1237/target`.
I did not touch the worktree.

All twelve changed paths are inside the spec's authorized paths. D1, D3, D4 and D5 are implemented as decided.
The boundaries are exactly right: inclusive `[-144, 24]` dB and `[-1, 1]`, refused with
`numeric.out_of_schema_range` at `$.routes[<i>].gain_db` and `$.routes[<i>].channel_matrix.<k>`, and
`numeric.non_finite` keeps precedence. The overflow proof holds; an exhaustive sweep is below. D3 adds no
render-path cost. Every gate passes, and every new test turns red under its mutation.

The FAIL rests on one MAJOR finding: the issue's own invariant, "one domain on every path" (D2: "a session value, a
compiled value and a live record value share one check"), does not hold for a subnormal matrix coefficient.

## Findings

### MAJOR

**J1-1. A subnormal `channel_matrix` coefficient is in the session's domain but not in the compiler's or the live
path's. The boot refusal then names the wrong code and the wrong field.**

- `route_values` (`crates/graph-compiler/src/ids.rs:322-336`) first checks the new range. It then calls
  `route_transform`, which still refuses any subnormal coefficient: `.all(|v| v.is_finite() && !v.is_subnormal())`,
  `ids.rs:296`.
- D1's `validate_finite_range(.., -1, 1, ..)` (`crates/session/src/validate.rs:737-744`) accepts such a coefficient,
  and so does the SDK builder (`routeNumber`, `sdk/src/core/session.ts:333-339`).
- The lowering therefore refuses it at `crates/graph-compiler/src/compile.rs:360-365` as `graph.gain.non_finite` at
  `$.routes[id=<r>].gain_db`. That is the wrong code, because the value is finite, and the wrong field, because the
  gain is fine.
- A live send edit to the same value is `DOMAIN`.

Reproduced in the export:

- Rust probe for `ll` in {1e-40, -1e-40, f32::MIN_POSITIVE/2}: `session_refusal` gives `None`,
  `route_coefficients` gives `Err(Domain)`, and `compile` gives `Some([("graph.gain.non_finite", "$.routes[id=eq0-main].gain_db")])`.
- SDK end to end against the built module: `route({ matrix: { ll: 1e-40, lr: 0, rl: 0, rr: 1 } })` is accepted.
  `validate()` then returns `{ok:false, phase:"boot", diagnostics:[{code:"graph.gain.non_finite", path:"$.routes[id=r].gain_db"}]}`.

This contradicts several statements:

- The mission and D2 in the spec.
- The gate-2 test's own claim, "in domain on the live path ... exactly when the session accepts it"
  (`crates/graph-compiler/tests/route_coefficients.rs:332`). The claim holds only on the test's curated value set,
  which contains no subnormal.
- `docs/SESSION_SCHEMA_V1.md:317`: "The compiler's lowering and every live send record check the same domain".
- The author-session skill's route bullet, which tells an agent that any coefficient in `[-1, 1]` is accepted.

The refusal predates this issue (since #1215). It is still this issue's slice, and D3 removes its only purpose: every
subnormal product is now flushed to `+0.0`, and a subnormal coefficient folded at a high gain gives a normal product,
which needs no refusal.

**Fix**, inside the authorized `ids.rs`:
1. Stop refusing a subnormal coefficient in `route_transform`/`route_values`, so that the check is exactly
   `[-144, 24] x [-1, 1]^4`. The gain's finite and subnormal checks are unreachable in domain; the sweep below
   proves it.
2. Update `RouteValueError::Domain`'s doc (`ids.rs:341-342`).
3. Add ±`1e-45`, ±`f32::MIN_POSITIVE / 2` and ±`f32::MIN_POSITIVE` to gate 2's coefficient values. They are red on
   today's code.
4. Assert that such a route compiles and binds `+0.0` at 0 dB.

The alternative is for the S0 coordinator to decide that the session refuses a subnormal coefficient, which adds a
D1 clause and a code. Either way the result must be one domain.

### MINOR

**J1-2. The route domain is written as literals in two crates.** The literals are at `crates/session/src/validate.rs:688-695`
and `crates/graph-compiler/src/ids.rs:305-312`.
- **Is it safe today? Yes.** The gate-2 test is an effective drift guard: mutations M1, M1x, M1b, M2, M2x and M2g are
  all red, and because the test pins the absolute bounds, a coordinated change of both literals is red too.
- **Why it still falls short.** Under the owner principle, one exported constant is the correct shape, and it is
  cheap. The implementer's stated reason, "would need `session/src/lib.rs`", is slightly off: a `pub const` in
  `crates/session/src/model.rs` is exported by the existing `pub use model::*` (`lib.rs:28`) with no `lib.rs` edit.
  That follows the precedent of `CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM` (`model.rs:481`). `ids.rs` would then import
  it.
- **Who resolves it.** `model.rs` is also outside the authorized paths, so this is a spec gap for S0 rather than an
  implementer fault. Recommendation: S0 amends the spec to authorize `crates/session/src/model.rs` (route-domain
  consts only) for attempt 2, which J1-1 requires anyway, and removes the `ids.rs` copies.
- **The SDK copy.** The SDK's TypeScript copy is an unavoidable cross-language duplicate. Its eval's engine leg holds
  it.

**J1-3. A public host-core API bypasses `route_values` and D3. This predates the issue and lies outside its paths.**
- `host_core::RouteControlRecord` re-exports `graph::RouteControlRecord` (`crates/host-core/src/route_controls.rs:25`,
  `lib.rs:161`). Its constructor `new(target, mute, length)` (`crates/graph/src/lib.rs:948`) is public and checks
  only the length and the muted-zero rule.
- `RouteControlProducer::push` (`route_controls.rs:106`) accepts any such record. An embedder can therefore push
  coefficients that `route_values` never saw: out of domain, non-finite, or subnormal and unflushed.
- No shipped host does this. host-web builds every record through `record`, which calls `route_coefficients`, and
  the C ABI has no live route path yet (#1225). So no product path bypasses the check today. The hole remains in the
  type.

Fix: S0 files a follow-up, or folds it into #1225, so that the checked record is the only type `push` accepts. For
example, `push` could take a host-core newtype that only `record` can build, or `graph::RouteControlRecord::new`
could become crate-private behind a constructor that takes a `RouteTransform` and a gate.

### NIT

**J1-4.** The module doc at `crates/graph-compiler/tests/route_coefficients.rs:6` still says the file "refuses a fold
that overflows". That block was removed.

**J1-5.** The deleted +700 dB block was the only test that reached the lowering's refusal arm
(`compile.rs:360-365`) through `compile()`. After J1-1's fix, no validated session can reach that arm. Record in the
spec that the arm is kept as defense, which leaves it untested by construction. Its gate independence (#1216 D2)
still holds structurally, because `route_values` takes no gate, and gate 2's 8-gate loop holds it on the live path.

## Focus answers

1. **Bounds, codes and paths.** They are exactly as D1, D2 and D4 decide, on the session, `route_values` (both the
   lowering and the live path) and the SDK builder. The entry points are covered:
   - Session JSON and the typed model: `compile_session` and `parse_session_json` share `validate_routes`.
   - Protocol session edits (`SetRouteGainDb`, `SetRouteChannelMatrix`): `crates/protocol/src/model.rs:944` runs
     `compile_session` on the candidate.
   - Live sends through host-web: `RouteControlProducer::record` calls `route_coefficients`.
   - host-core sends: `record` and `set`.
   - SDK `route()`, and `enginectl` through the builder.

   The C ABI has no live route path (non-goal, #1225). The exceptions are J1-1 (subnormal) and J1-3 (the raw-record
   API).
2. **Overflow.** It is proven impossible in domain. An exhaustive sweep of all 2,228,224,001 `f32` gains in
   `[-144, 24]` through `math::db_to_gain_f32` gives a gain in `[6.3095705e-8, 15.848933]`, every value normal and
   finite. Since `|c| <= 1` and rounding is monotone, `|round(g*c)| <= g <= 15.85`. `route_coefficients` at the
   extremes (±1, `f32::MIN_POSITIVE`, ±1e-35) under all 8 gates returns finite, non-subnormal values `<= 16`.
   Removing the open-fold check is therefore sound. No shipped live path bypasses `route_values` (see J1-3 for the
   API hole).
3. **D3.**
   - `const fn fold` is still `const`; `f32::is_subnormal` is const at 1.97.1 and it compiles.
   - Both signs flush to `+0.0`.
   - A product of `-0.0` is unchanged, as it was before.
   - Render-path cost is zero: `gated_route_coefficients` runs only in plan construction (`runtime.rs:4651`
     `node_kind`, `runtime.rs:7030` `plain_route_gains`) and in `route_coefficients` on the control thread. The
     render op multiplies by the bound constant.
   - No render code changed, and `check-realtime-policy` passes.
   - Hazard: no checked-in route has a subnormal product. All 514 routes in 142 JSON files are in domain, and the
     smallest nonzero `|gain*coef|` is 0.126 (`console-sixty-four-track-sends.json`, `ch00-fx-b.ll`). Gate 6 is green.
4. **Duplicated literals.** See J1-2. They are acceptable as safe, but S0 should amend the spec (authorize
   `model.rs`) and land one constant in attempt 2.
5. **Rescoped tests.** Every reason is sound:
   - `Draw::coefficient` changed from `uniform(-4, 4)` to `uniform(-1, 1)`. This is required, because values above 1
     are now refused. No in-domain coverage is lost, and the gain draw `(-120, 24)` was already in domain.
   - The +700 dB block was deleted. Its claims were:
     - overflow is refused under every gate (now unreachable, as proven);
     - 700 dB with unity coefficients is accepted (now out of domain);
     - a muted route is refused exactly as an open one is.

     Gate 2 covers the last claim on the live path, and the lowering is gate-free. One small loss remains: J1-5.
   - host-web: `3.0e38` became `1.5`. The test's purpose is unchanged: a value that is finite on the wire and refused
     by the domain. M2 turns it red.
6. **Docs and skill.** These are accurate except for the subnormal statement in J1-1:
   - `SESSION_SCHEMA_V1.md:313-320`: the 1.9e-31 figure checks (`1.1755e-38 / 6.3096e-8 = 1.86e-31`).
   - The APP-LIVE "Live sends" bullet.
   - The author-session route bullet.

## Gates run (export, all exit 0 unless stated)

- Focused tests:
  - `cargo test -p session -p graph-compiler -p graph`: all ok.
  - `cargo test -p host-web`: 183 lib tests and the rest, ok.
  - `cargo test -p host-core`: ok.
- test-debug-a (the exact `qualification.yml` command, with `--features ...`, preceded by `builtins-compiler --no-run`):
  0, 1432 passed, 0 failed.
- Conformance: `cargo test --all-targets -p conformance` passes (24), and `conformance_fixtures -- --check` exits 0.
  These are the only test-debug-b packages that touch routes; I did not run the rest of test-debug-b, whose DSP
  crates have no #1237 change.
- `cargo fmt --all -- --check`: 0.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: 0.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: 0.
- Policy check/test pairs: workspace, session, bench, host-core, protocol-control, realtime, lane, rack, builtins,
  graph and effect-runtime all exit 0 (22/22).
- Gate 6:
  - `graph_fixture -- --check`: 0.
  - `check-graph-determinism.sh`: 100/100.
  - `check-builtins-fixtures.sh . <release audit>`: 50 files.
  - `check-console-fixtures.sh <release session_validator>`: 0.
  - `check-browser-expected-resources.py --artifacts <A>`: 0, with its 32 red self-test mutations.
- Gate 7a:
  - `build-web-audioworklet.sh --named-twin <B> <A>`: 0. The shipped module is `1a334cd5...c101d` (2894346 B),
    identical to the implementer's.
  - `check-web-audioworklet.sh <A> <B>/...named.wasm`: 0.
  - `test-web-audioworklet.sh`: 0.
  - `check-cross-targets.sh`: 0. Its only failures are the ten expected `ios-asm-memset-pattern16` rows (#1018), and
    no ceiling file changed.
- Gate 5:
  - `check-sdk-headless.sh <A>`: 0 (361/361, including the new eval).
  - `sdk-package.sh check <A>`: 0, with `node_modules` copied read-only from the worktree into the export.
  - `check-sdk-types.sh`: 0.

## Mutation runs (reproduced; each red on mutation, green on revert, export verified pristine afterwards)

| Mutation | g1 session | g2 live==session | g4 coeff | g3 host-web | g4 render | batch (rescoped) |
| --- | --- | --- | --- | --- | --- | --- |
| M1 session `gain_db` -> `validate_finite` | RED | RED | green | green | green | green |
| M1x session max 24 -> 23.999998 (exclusive) | RED | RED | green | green | green | green |
| M1b session coefficient bound -> f32::MAX | RED | RED | green | green | green | green |
| M1c every coefficient reported at `.ll` | RED | RED | green | green | green | green |
| M2 `route_values` bound removed | green | RED | green | RED | green | RED |
| M2x `route_values` coefficient max -> 0.99999994 | green | RED | RED | RED | green | RED |
| M2g `route_values` gain min -> -150 | green | RED | green | RED | green | green |
| M3 flush removed | green | green | RED | green | RED (6.25e-43 vs 0) | green |
| M3b flush to -0.0 | green | green | RED | green | RED (-0 vs 0) | green |
| M3c flush positive subnormals only | green | green | RED | green | RED (4e-43 vs 0) | green |

SDK eval (`MISO_ENGINE_SDK_SKIP_ASSET=1`, the builder half):

| Mutation | Result |
| --- | --- |
| M5a: `route()` `gainDb` back to `f32` | RED |
| M5b: `route()` matrix back to `f32` | RED |
| M5c: exclusive bound | RED |
| M5d: no `CODE.outOfRange` | RED |
| M5e: `normalize`'s gain check only | green |

M5e is green as expected: `route()` refuses first, and the deep `freeze` makes the `normalize` check an unreachable
backstop. This is the same pattern as before, so it is not a finding.

## Test value (one sentence each)

- **`session/tests/route_domain.rs::route_gain_and_matrix_values_are_bounded_at_their_own_path`.** Red if the session
  checks a route gain or any coefficient finite-only, with a wrong or exclusive bound, at another path, or lets the
  range pre-empt `numeric.non_finite`, on either the typed or the text path. No earlier test bounded route values.
- **`route_coefficients.rs::a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it`.** Red if the live
  path's domain differs from the session's at the boundaries ±1 ulp, at the half-steps or for non-finite values,
  under any of the 8 gates. Today it misses the subnormal case (J1-1).
- **`route_coefficients.rs::a_subnormal_folded_coefficient_is_positive_zero`.** Red if
  `gated_route_coefficients` returns a subnormal product, flushes to `-0.0` or flushes only one sign, in the plan's
  bound constants or on the live path.
- **`host-web tests.rs::a_live_send_edit_outside_the_route_domain_is_refused`.** Red if browser send admission refuses
  an out-of-domain gain or coefficient with anything but `DOMAIN`, moves the mirror or queue on refusal, or refuses the
  bounds. M2g shows it catches a gain-domain defect that the rescoped batch test misses.
- **`host-web tests.rs::a_subnormal_route_renders_as_a_zero_coefficient`.** Red if a subnormal constant reaches the
  rendered route, through any runtime path that does not use the one gated derivation.
- **`sdk/test/builder-evals.mjs` "a route's gain and matrix are bounded ...".** Red if the builder checks
  `gainDb` or a coefficient as `f32` only, uses an exclusive bound, or refuses without
  `numeric.out_of_schema_range` at the route's path.

## Required for attempt 2

- J1-1: one domain for a subnormal coefficient; extend gate 2 accordingly.
- J1-2: the S0 spec amendment authorizing `model.rs` consts, then a single exported constant.
- J1-4: fix the stale doc.
- J1-3 and J1-5: S0 records or files them.
