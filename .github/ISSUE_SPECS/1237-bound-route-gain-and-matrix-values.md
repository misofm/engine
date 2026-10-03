# Bound route gain and matrix values

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

Every anchor below is verified on the batch K3 follow-up tree (branch `codex/batch-submix-k3`).

- **Session.** `validate_routes` (`crates/session/src/validate.rs:548-585`) checks a route's
  `gain_db` (`:564`) and its four matrix coefficients (`:575-584`) with `validate_finite` only. So
  a route gain is accepted wherever its linear gain is a normal finite `f32` (about -758 to +770
  dB, plus anything low enough to round to exactly 0), and a coefficient may be any finite value.
  The bounded precedent is the pan's `validate_finite_range(.., -1.0, 1.0, ..)` (`:446-447`), which
  refuses `numeric.out_of_schema_range` (`validate_finite_range`, `:925-944`).
- **Compiler and live producers.** `graph_compiler::route_values` (`crates/graph-compiler/src/ids.rs:309`)
  backs both `route_coefficients` (`:341`) and the lowering (`crates/graph-compiler/src/compile.rs`,
  the route loop): it refuses (`RouteValueError::Domain`) a value `route_transform` refuses or an
  open fold that overflows. host-core's send producer (`RouteControlProducer::record`,
  `crates/host-core/src/route_controls.rs:86`) and host-web's admission (`COMMAND_REASON_DOMAIN`,
  `hosts/host-web/src/lib.rs:982`; doc at `:878-883`) refuse a live value through it.
- **The fold.** `graph::gated_route_coefficients` (`crates/graph/src/lib.rs:784`) is the one
  derivation of a route's four constants, for the bound op, the fold lane and every live record.
  It accepts a subnormal product: `route_coefficients(-120.0, [1.2e-38, 0, 0, 1], ..)` returns a
  first coefficient near `1.3e-44` (#1215 verdict INFO). Under the new bounds a subnormal product
  is still reachable (a coefficient below about `1.9e-31` at -144 dB).
- **SDK.** The builder checks a route's `gainDb` and matrix with `f32` only
  (`sdk/src/core/session.ts:800-808`, normalised at `:1544-1545`); `enginectl` passes a route
  request's values to the builder (`sdk/src/cli/session-request.ts`). Live `RouteEdits`
  (`sdk/src/core/live-controls.ts:738-772`) checks finiteness and leaves the domain to the engine's
  `domain` refusal (`docs/handoffs/submix-strips-and-sends/APP-LIVE.md:66-70`).
- **Migration.** All 312 routes in checked-in JSON documents (`git ls-files '*.json'`) are inside
  the new domains, so no document changes. The writer corpus's `ll = 1.25` is a **track** matrix
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
