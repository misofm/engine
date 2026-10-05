# Spell the route gain and matrix domain once, in the session model

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the verdicts of *Bound route gain and matrix values* (#1237), finding J1-2 (attempt 1, carried as
open in attempt 2: `docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1237-attempt2.md`, and
the stream-J `README.md` there, "Open for root/S0"). No behaviour changes.

## Problem (verified on `main` at `0a1176b3b`)

The route domain decided by #1237 (gain in `[-144, 24]` dB, each `channel_matrix` coefficient in
`[-1, 1]`, both inclusive) is written twice, as two private sets of the same three constants:

- `crates/session/src/validate.rs:688-695` (`ROUTE_GAIN_DB_MINIMUM`, `ROUTE_GAIN_DB_MAXIMUM`,
  `ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`), read by `validate_routes` at `:713-718` and `:738-743`.
- `crates/graph-compiler/src/ids.rs:309-315`, the same three names and values, read by
  `route_values` at `:325-338`, which backs both the compiler's lowering and the live path
  (`route_coefficients`, `:359`; host-core's `RouteControlProducer::record` and host-web's send
  admission call it).

Each doc comment points at the other copy and at the test that holds them together:
`crates/graph-compiler/tests/route_coefficients.rs:346`
(`a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it`). That test catches a drift
after it happens. Nothing stops it from happening. A future domain change must find and edit both
copies, and a reviewer must know they exist.

`graph-compiler` already depends on `session` (`crates/graph-compiler/Cargo.toml:16`), and the
session model already publishes a domain constant that other crates read:
`CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM` (`crates/session/src/model.rs:477-481`, read by
`crates/session/src/validate.rs:445` and `crates/builtins-compiler/tests/track_delay_domain.rs`).

## Decisions

- **D1. One definition.** Add three public constants to `crates/session/src/model.rs`, next to
  `Route` (`:780-800`), with doc comments that name #1237 and owner question Q2 of decision 13:
  `ROUTE_GAIN_DB_MINIMUM: f32 = -144.0`, `ROUTE_GAIN_DB_MAXIMUM: f32 = 24.0` and
  `ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM: f32 = 1.0`. They are re-exported by the existing
  `pub use model::*` (`crates/session/src/lib.rs:28`).
- **D2. Both readers use it.** Delete the private constants in `validate.rs` and `ids.rs`.
  `validate_routes` and `route_values` read the session constants. Each doc comment that says
  "the session validator refuses the same domain" or "the graph compiler's `route_values` refuses
  the same domain" now says that both read the session constants.
- **D3. The test keeps its literals.** `route_coefficients.rs` keeps `[-144.0, 24.0]` and
  `[-1.0, 1.0]` as literals. They are the test's oracle for the decided values, independent of the
  constants. The test stays as it is, because it also checks inclusivity, subnormal values and
  non-finite values on both paths.

## Authorized paths

- `crates/session/src/model.rs` (the three constants only)
- `crates/session/src/validate.rs` (delete the private constants; `validate_routes` reads the
  shared ones; its doc comment)
- `crates/graph-compiler/src/ids.rs` (delete the private constants; `route_values` reads the shared
  ones; the doc comments of `route_transform`, `route_values`, `RouteValueError` and
  `route_coefficients` where they name the other copy)
- This spec

## Non-goals

- The fader, trim and VCA domains (`[-144, 24]` dB), which are spelled in `crates/builtins`,
  `crates/builtins-compiler`, `crates/session/src/validate.rs:584`, `crates/session/src/vca.rs:15`
  and `hosts/host-web/src/lib.rs`. They are a different domain with other owners. *Route the
  builtins fader domain checks through checked_fader_gain* (#1303) covers part of it.
- The SDK builder's copy (`ROUTE_GAIN_DB_DOMAIN`, `sdk/src/core/session.ts:325`). The SDK is
  TypeScript and cannot read a Rust constant. `sdk/test/builder-evals.mjs` holds it to the engine
  module at boot.
- Any change to the domain itself.

## Hazards

- `crates/graph-compiler/src/*` is a hot file (`STREAMS.md`). This edit is three constants and
  their doc comments; whichever slice lands second rebases.
- A name clash: `session::*` is glob-imported in some crates. Check that no crate already defines
  a constant with one of the three names in a scope that also glob-imports `session`. If one does,
  qualify the use; do not rename the session constants.

## Objective gates

1. **No drift possible (mutation, PR evidence).** Set `session::ROUTE_GAIN_DB_MAXIMUM` to `24.5`:
   `a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it` turns red (the session and
   the live path move together, so the refused count and the literal boundaries fail). Set
   `session::ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM` to `0.5`: the same test turns red. Revert both.
2. **No behaviour moved.** These pass unchanged:
   - `cargo test --locked -p session`
   - `cargo test --locked -p graph-compiler`
   - `cargo test --locked -p host-core --features control-provider,test-support`
   - `cargo test --locked -p host-web --features test-support`
   - `cargo test --locked -p graph-compiler --test graph_fixture -- --check`
   - `bash scripts/check-graph-determinism.sh`
3. **Workspace.** `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   `bash scripts/check-session-policy.sh`; `bash scripts/check-workspace-policy.sh`.

*Test value.* No new test. The existing gate-2 test of #1237 keeps its value: it is red if the two
paths disagree on inclusivity, subnormal or non-finite values, and, with its literals, if the
decided domain moves.

## Evidence

- Gate 1's two mutation runs (the failing assertion of each) and the revert.
- Each gate command and its exit status at the PR head.

## Dependencies

- None. *Bound route gain and matrix values* (#1237) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
