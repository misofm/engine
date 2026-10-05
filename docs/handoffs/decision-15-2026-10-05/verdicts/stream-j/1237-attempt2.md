VERDICT: PASS

# #1237 attempt 2: verdict (stream J, commit 842139f4e, base e9dc1a4b2)

Reviewed diff: `git diff e9dc1a4b2 842139f4e -- . ':!scripts/check-realtime-policy.sh' ':!scripts/test-realtime-policy.sh' ':!.github/ISSUE_SPECS/1302-*'`
(commit 842139f4e; the #1302 commit 4f15b5c7e only touches the excluded paths). I also re-read the whole #1237
change, `5304737c1..842139f4e` minus the same paths. Everything was built and run in a fresh export,
`/tmp/claude-1002/v1237/a2/src` (a `git archive` of 842139f4e), with `CARGO_TARGET_DIR=/tmp/claude-1002/v1237/target`.
I did not touch the worktree. The export was diffed back to pristine after every mutation and probe.

Attempt 2 changes four paths, all authorized: `crates/graph-compiler/src/ids.rs` (`route_transform` checks a
coefficient with `is_finite` only; docs), `crates/graph-compiler/tests/route_coefficients.rs` (gates 2 and 4
extended, J1-4 doc), `sdk/test/builder-evals.mjs` (gate 5 extended) and the spec's attempt record.

**J1-1 is fixed.** The route domain is now exactly `[-144, 24]` dB x `[-1, 1]`^4 on every path, with subnormal
values of both signs inside it:

| Path | Evidence |
| --- | --- |
| Session (`validate_routes`, typed and text) | unchanged since attempt 1; gate 2's `session_refusal` is `None` for ±1e-45, ±2^-127, ±`f32::MIN_POSITIVE` on every field |
| Lowering (`compile.rs` -> `route_values`) | gate 2 now compiles every accepted value; gate 4 compiles a 0 dB route with ±1e-45 and -2^-127 coefficients |
| Live path, host-core (`RouteControlProducer::record` -> `route_coefficients`) | gate 2, under all 8 gates |
| Live path, host-web (`COMMAND_ROUTE_MATRIX` -> `record`) | my probe below: a ±1e-45 / ±2^-127 matrix edit is admitted (`RESULT_OK`), the mirror holds the exact bits |
| C ABI | no live route path (#1225, non-goal); its boot path is the same lowering |
| Protocol (`SetRouteChannelMatrix`/`SetRouteGainDb`) | `compile_session` on the candidate (`crates/protocol/src/model.rs:944`), then the same lowering |
| SDK builder | `routeNumber` already accepted subnormals; the extended eval now proves the engine boots them |

**D3 holds where a subnormal coefficient can enter.** I checked this by probe at boot and on a live edit
(temporary host-web tests, removed afterwards):

- **Boot.** Routes at 0 dB `[1e-45, -1e-45, 0.5, -2^-127]`, at +24 dB `[1e-40, -1e-40, 0.5, 1e-45]` and at -144 dB
  `[1e-45, 2^-127, 0.5, f32::MIN_POSITIVE]` each render bit-identical over 4 blocks to the same route with those
  coefficients `0.0`, with no subnormal output sample and 512 nonzero samples.
- **Live edit.** A step matrix edit of send `send-a` to `[1e-45, -1e-45, 2^-127, -2^-127]`, and of `send-b` to the
  sign-swapped set, renders bit-identical over 4 blocks to the same edit with `[0; 4]`. Against the unedited host it
  differs at 1024 samples, so the edit took effect.
- **Under M6** (attempt 1's behaviour restored), both probes are red: the boot fails to prepare, and the live edit
  is refused (`RESULT_INVALID_ARGUMENT`, `DOMAIN`).

NIT J1-4 is fixed (`route_coefficients.rs:6`). J1-5 is recorded in the attempt-2 entry. J1-2 and J1-3 are recorded
as open for S0/root, which is correct: both need paths outside this spec, and neither is a FAIL reason.

## Findings

No BLOCKER. No MAJOR.

### MINOR

None new. Carried from attempt 1 and recorded in the spec as open for S0/root:

- **J1-2.** The route domain is two literal copies, in `session/src/validate.rs` and `graph-compiler/src/ids.rs`.
  Gate 2 still guards the drift, and every bound mutation in attempt 1's table was red.
- **J1-3.** `host_core::RouteControlRecord::new` is public and bypasses `route_values` and D3. S0 must file it or
  fold it into #1225.

### NIT

**N2-1.** `crates/graph-compiler/tests/route_coefficients.rs:337` is a 174-column doc line. The sentence "An accepted
value must also compile ... This replaces #1215's +700 dB overflow case: inside the domain no fold can" was not
reflowed. rustfmt does not wrap comments, so `fmt --check` passes. Fix: reflow the paragraph to 100 columns.

**N2-2.** No committed host-web test holds the browser admission to the subnormal half of the domain. Under M6,
`a_live_send_edit_outside_the_route_domain_is_refused` and `a_subnormal_route_renders_as_a_zero_coefficient` both
stay green. Today the agreement holds only because host-web adds no value check of its own: `into_route_edit` passes
`values` through, and gate 2 holds the shared `route_coefficients`. The spec's gate 3 does not require more. Fix
(optional): add `1.0e-45`, `-1.0e-45`, `f32::MIN_POSITIVE / 2.0` and `-f32::MIN_POSITIVE / 2.0` to the `admitted`
coefficients at `hosts/host-web/src/tests.rs:11063`. That catches a browser-side subnormal refusal, such as a
DAZ-style check added to `into_route_edit`, which no test catches today.

### INFO (not a defect)

**N2-3.** A negative coefficient whose product underflows past the subnormal range binds `-0.0`, not `+0.0`. For
example, `-1e-45` at -144 dB gives `-8.8e-53`, which rounds to `-0.0`. This is exactly D3 as written: only a
*subnormal* product is flushed. Attempt 1's verdict already noted "a product of -0.0 is unchanged". My probe shows
such a route renders bit-identical to the same route with an explicit `-0.0` coefficient, which the session accepts
and whose signed-zero behaviour `SESSION_SCHEMA_V1.md` documents. So "a subnormal coefficient renders as a `+0.0`
contribution" holds exactly when its product is subnormal. When the product rounds to a zero, the coefficient
renders as that signed zero. A positive one renders `+0.0` either way. This is inaudible, and the docs (schema,
skill: "a folded `gain * coefficient` that is subnormal is applied as `+0.0`") are accurate.

## Full-change re-read (anything attempt 1 missed)

I found nothing new beyond N2-1 to N2-3:

- **`route_transform`'s gain checks** are still dead code in domain. `route_values` is its only caller, and attempt
  1's exhaustive sweep proves it. They are kept as documented defence.
- **`RouteTransform` consumers.** Outside tests, only `canonical.rs` (hashed as `to_bits`, exact for subnormals) and
  the runtime read it. The runtime reads it only through `gated_route_coefficients`, at `runtime.rs:4651` and
  `runtime.rs:7030`.
- **Route values reaching render.** No render code changed. No other crate (audit, conformance, builtins-compiler)
  reads `channel_matrix` values on a render path.
- **Subnormal spelling in canonical JSON** is held by `crates/session/src/canonical.rs`'s subnormal cases and the
  SDK's canonical-writer corpus.
- **Stale statements.** Grep finds no remaining doc or skill text that says a subnormal coefficient is refused.
  `docs/handoffs/submix-sends-2026-10-02/DESIGN.md:391` and `VERIFY-1.md:718` record #1196-era evidence
  (`graph.gain.non_finite` at -800/+1000 dB). They are historical handoff records, not current-behaviour docs.

## Gates run (export; exit 0 unless stated)

**Focused tests and test-debug-a**

- test-debug-a is the exact `qualification.yml` command with its `--features`, preceded by
  `cargo test --locked -p builtins-compiler --no-run`, which exits 0. It exits 0: 1432 passed, 0 failed,
  10 ignored, across 122 test binaries. It includes every focused package: session, graph, graph-compiler, host-web
  and host-core.

**Gate 7**

- `cargo fmt --all -- --check`: 0.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: 0.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: 0.
- Policy check/test pairs: bench, builtins, effect-runtime, graph, host-core, lane, protocol-control, rack,
  realtime, session and workspace. All 22 exit 0.
- test-debug-b: not rerun. No test-debug-b package depends on `graph` or `graph-compiler` (`cargo tree -e normal,dev`
  per package). The only #1237 crate in their trees is `session`, which attempt 2 does not change. Attempt 1's
  results stand: 813 passed, and `conformance_fixtures -- --check` exits 0.

**Gate 5**

- `check-sdk-headless.sh <A>`: 0 (361/361, including the extended eval).
- `sdk-package.sh check <A>`: 0, with `node_modules` copied from the worktree into the export.
- `check-sdk-types.sh`: 0.

**Gate 6**

- `graph_fixture -- --check`: 0.
- `check-graph-determinism.sh`: 0 (100/100).
- `cargo build --locked --release -p audit -p session-validator`: 0.
- `check-builtins-fixtures.sh . <release audit>`: 0 (50 files).
- `check-console-fixtures.sh <release session_validator>`: 0.
- `check-browser-expected-resources.py --artifacts <A>`: 0, with its 32 red self-test mutations.

**Gate 7a**

- `build-web-audioworklet.sh --named-twin <B> <A>`: 0. The shipped module is
  `d6b7b8dfad59e96e005d349efbdd11682f7707b965f7e98bf954e039d251d4b0` (2894073 B), identical to the implementer's.
- `check-web-audioworklet.sh <A> <B>/...named.wasm`: 0.
- `test-web-audioworklet.sh`: 0.
- `check-cross-targets.sh`: 0. Its only failures are the ten expected `ios-asm-memset-pattern16` rows (#1018). No
  ceiling or script outside the two excluded #1302 scripts changed in `5304737c1..842139f4e`.

## Mutation runs (reproduced; each red on mutation, green on revert, export verified pristine)

| Mutation | gate 2 (live == session, compiles) | gate 4 (coefficients) | SDK eval |
| --- | --- | --- | --- |
| M6: restore `route_transform`'s `!v.is_subnormal()` (attempt 1's behaviour) | RED `:396` (`channel_matrix.ll = 1e-45`: live `Err(Domain)`, session `None`) | RED `:449` (0 dB compile: `graph.gain.non_finite` at `gain_db`) | n/a (Rust) |
| M6n: refuse only a negative subnormal coefficient | RED `:396` (`ll = -1e-45`) | RED `:449` | n/a |
| M7: the lowering alone refuses a subnormal coefficient (`compile.rs`), `route_values` unchanged | RED `:408` (the new compile assertion) | RED `:449` | n/a |
| M3d: flush only when the coefficient is normal (`fold`) | green | RED `:460` (0 dB case; the -144 dB case passes) | n/a |
| Engine module of attempt 1 (`/tmp/claude-1002/v1237/A`, `1a334cd5...c101d`) | n/a | n/a | RED: `{"matrix":{"ll":1e-45,...}}` `validate().ok` false |
| Engine module of attempt 2 (`<A>` above, `d6b7b8df...51d4b0`) | n/a | n/a | green |
| M5f: the builder's `routeNumber` refuses a nonzero subnormal | n/a | n/a | RED (builder-only, `MISO_ENGINE_SDK_SKIP_ASSET=1`): `route("r").gainDb: subnormal` |

## Test value (one sentence each, extended tests)

- **`route_coefficients.rs::a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it`** (extended).
  It turns red if the live path or the compiler's lowering refuses a subnormal or smallest-normal gain or
  coefficient, of either sign, that the session accepts. That is the J1-1 defect, which the attempt-1 version
  (no subnormal value) and every other test missed. M6, M6n and M7 are all red, and M7 shows that the new compile
  assertion is load-bearing on its own.
- **`route_coefficients.rs::a_subnormal_folded_coefficient_is_positive_zero`** (extended with the 0 dB case).
  It turns red if a subnormal coefficient is refused at boot (M6, M7) instead of compiling. It also turns red if
  the flush misses a product that is a subnormal coefficient itself (M3d). The -144 dB case alone does not catch
  M3d.
- **`sdk/test/builder-evals.mjs` "a route's gain and matrix are bounded ..."** (extended). It turns red if the
  builder accepts a subnormal value that the engine module then refuses at boot (attempt 1's module), or if the
  builder refuses a subnormal (M5f). No other eval sends a subnormal route value through `validate()`.
