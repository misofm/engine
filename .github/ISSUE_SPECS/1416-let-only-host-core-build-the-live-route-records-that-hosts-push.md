# Let only host-core build the live route records that hosts push

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the verdicts of *Bound route gain and matrix values* (#1237), finding J1-3 (attempt 1, carried as
open in attempt 2: `docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1237-attempt2.md`, and
the stream-J `README.md` there, "Open for root/S0"). The gap predates #1237.

## Problem (verified on `main` at `0a1176b3b`)

- **The record.** `graph::RouteControlRecord` (`crates/graph/src/lib.rs:937-975`) is a send's live
  target: four coefficients, a mute flag and a ramp length. Its public constructor `new`
  (`:948-959`) checks only the ramp length and the mute rule (a muted target is four `+0.0`). It
  does not check the route domain, and it cannot: `graph` does not depend on `graph-compiler`,
  where the domain is checked (`route_values`, `crates/graph-compiler/src/ids.rs:325`).
- **The intended path.** `host_core::RouteControlProducer::record`
  (`crates/host-core/src/route_controls.rs:81-102`) builds a record from a gain, a matrix, a mute
  and the source-lane mutes. It checks the domain through `route_coefficients` (#1237 D2) before it
  calls `RouteControlRecord::new`. `push` (`:106-113`) then queues any record it is given.
- **The bypass.** host-core re-exports graph's type unchanged
  (`pub use graph::{RouteControlRecord, ..}`, `route_controls.rs:25`;
  `crates/host-core/src/lib.rs:160-162`).
  So any host-core embedder can write
  `producer.push(RouteControlRecord::new([f32::NAN; 4], false, 0).unwrap())`, or push a target of
  `1.0e6`, or a subnormal target that `gated_route_coefficients` would have flushed to `+0.0`
  (#1237 D3). The render thread applies it as is. No test can catch this, because it is an API
  that allows it.
- **Who embeds host-core.** `hosts/host-web` and `crates/capi` depend on `host-core` and not on
  `graph` (their `Cargo.toml`s). host-web names the type only to carry a record from `record` to
  `push` (`hosts/host-web/src/lib.rs:37`, `:1821-1827`, `:1905`). capi has no route lane yet;
  *Deliver value-only send edits to the running C ABI plan* (#1225) will add one through
  `RouteControlProducer::record`.

## Decisions

- **D1. host-core owns the record type its producer accepts.** In
  `crates/host-core/src/route_controls.rs`, replace the re-export of graph's record with a host-core
  newtype of the same name, `pub struct RouteControlRecord(graph::RouteControlRecord);`, with a
  private field and no public constructor. `RouteControlProducer::record` is the only code that
  builds one. `push` takes the newtype and pushes its inner record. Keep the derives graph's type
  has (`Clone, Copy, Debug, PartialEq`) and read-only accessors (`target`, `mute`, `length`) that
  forward to the inner record. `RouteControlResources` stays a re-export.
- **D2. Keep the name.** The newtype keeps the name `RouteControlRecord`, so the re-export in
  `crates/host-core/src/lib.rs` and every host-web use compile unchanged.
- **D3. graph keeps its constructor.** `graph::RouteControlRecord::new` stays public: host-core
  must call it across a crate boundary, and graph's own tests use it
  (`crates/graph-compiler/tests/live_routes.rs`). No embedder depends on `graph`.
- **D4. The guarantee is a compile-time one, and the test says so.** A `compile_fail` doctest on
  the newtype shows that `host_core::RouteControlRecord::new(..)` does not exist and that the
  newtype cannot be built from graph's type by a tuple constructor. Pin the error code
  (`compile_fail,E0599` for the missing associated function; `compile_fail,E0423` or `E0603` for the
  private tuple constructor, whichever rustc reports), so the doctest cannot pass for another
  reason. A second, plain doctest shows the intended path compiles, as
  `crates/host-core/src/prepare.rs:514-523` does for `PreparedHost`.

## Authorized paths

- `crates/host-core/src/route_controls.rs` (the newtype, `record`, `push`, the module doc's "One
  authority" paragraph)
- This spec

## Non-goals

- The strip producers. `TrackControlProducer` (`crates/builtins-compiler/src/lib.rs:254`,
  re-exported by `crates/host-core/src/lib.rs:236-237`) exposes its raw
  `Producer<TrackControlRecord>` fields, which an embedder can push an unchecked matrix into.
  That is the same class of gap on another lane, with other owners (stream B's latest-target
  cells, #1312 and #1346, replace those queues). Record it in the Evidence for root.
- Effect records. The C ABI send edits (#1225). Route cells (*Hold route-lane values in
  latest-target cells*, #1347).

## Hazards

- **#1347 and #1225 edit the same producer.** #1347 changes the graph producer's `try_push` to an
  infallible `write` and keeps validation in `RouteControlRecord::new`. Either order works; the
  slice that lands second rebases and keeps D1: the host-core producer accepts only the host-core
  newtype.
- **CI does not run doctests.** The required workflow's `cargo test` steps use `--all-targets`,
  which excludes doctests (`.github/workflows/qualification.yml:609-620`). The existing
  `compile_fail` doctests have the same gap. Run gate 1 locally and record it; do not change CI in
  this issue.
- **Shipped module bytes.** A newtype should not change generated code, but `push`'s signature
  changes. Gate 3 confirms that no render digest moves.

## Objective gates

1. **The bypass does not compile (new doctests).**
   `cargo test --locked -p host-core --features control-provider,test-support --doc` passes, with
   D4's `compile_fail` doctests. Mutation (PR evidence): restore
   `pub use graph::RouteControlRecord` in place of the newtype; the missing-constructor doctest
   turns red. Revert.
2. **Every existing path still works.**
   - `cargo test --locked -p host-core --features control-provider,test-support`
   - `cargo test --locked -p host-web --features test-support`
   - `cargo test --locked -p graph-compiler --test live_routes`
   - `cargo test --locked -p graph-compiler --test route_coefficients`
3. **No rendered bit moved.** `bash scripts/build-web-audioworklet.sh --named-twin <N> <A>`, then
   `bash scripts/check-web-audioworklet.sh <A> <N>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   and `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` exit 0.
4. **Workspace.** `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   `bash scripts/check-host-core-policy.sh`; `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-workspace-policy.sh`.

*Test value.* The `compile_fail` doctest is red if host-core again exposes a way to build a route
record without `RouteControlProducer::record`'s domain check (a re-export of graph's type, or a
public constructor on the newtype). No other test can see that, because the bypass is an API, not
a behaviour.

## Evidence

- Gate 1's mutation run (the doctest's failure) and the revert.
- The strip-producer gap from the Non-goals, with its file and line, for root.
- Each gate command and its exit status at the PR head.

## Dependencies

- None. *Bound route gain and matrix values* (#1237) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
