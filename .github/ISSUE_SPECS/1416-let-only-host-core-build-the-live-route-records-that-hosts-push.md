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
  newtype cannot be built from graph's type by a tuple constructor. Each `compile_fail` doctest
  has a passing twin, by the rule of *Run doctests in CI* (#1422, D2 and Amendment 1): a plain
  doctest identical except for exactly the one forbidden construct, reaching the same items by the
  intended path (host-core's own builder), so a rename, a typo or an unrelated error in the shared
  code turns the twin red. Stable rustdoc does not check `compile_fail,E....` codes (*The rustdoc
  book*, "Unstable features", "Error numbers for compile-fail doctests"), so the fence stays
  `compile_fail` and a comment beside it names the code rustc reports today (`E0599` for the
  missing associated function; `E0423` or `E0603` for the private tuple constructor, whichever
  rustc reports) as documentation only. No `RUSTC_BOOTSTRAP`.

## Authorized paths

- `crates/host-core/src/route_controls.rs` (the newtype, `record`, `push`, the module doc's "One
  authority" paragraph)
- This spec

## Non-goals

- The strip lanes. `TrackControlProducer` (`crates/builtins-compiler/src/lib.rs:254`) lets an
  embedder push an unchecked strip record: the same class of gap, covered by *Make live strip
  records valid by construction* (#1423).
- Effect records. The C ABI send edits (#1225). Route cells (*Hold route-lane values in
  latest-target cells*, #1347).

## Hazards

- **#1347 and #1225 edit the same producer.** #1347 changes the graph producer's `try_push` to an
  infallible `write` and keeps validation in `RouteControlRecord::new`. Either order works; the
  slice that lands second rebases and keeps D1: the host-core producer accepts only the host-core
  newtype.
- **CI does not run doctests on `main` yet.** The required workflow's `cargo test` steps use
  `--all-targets`, which excludes doctests (`.github/workflows/qualification.yml:609-620`).
  *Run doctests in CI* (#1422) adds the doctest steps; this issue lands after it, so gate 1 runs in
  `test-debug-a`. Do not change CI in this issue.
- **Shipped module bytes.** A newtype should not change generated code, but `push`'s signature
  changes. Gate 3 confirms that no render digest moves.

## Objective gates

1. **The bypass does not compile (new doctests).**
   `cargo test --locked -p host-core --features control-provider,test-support --doc` passes, with
   D4's `compile_fail` doctests. Mutation (PR evidence): restore
   `pub use graph::RouteControlRecord` in place of the newtype; the missing-constructor doctest
   turns red. Revert. Per pair (PR evidence): rename an item in the shared part of both snippets,
   and the twin turns red; delete the forbidden construct from the `compile_fail` snippet, and it
   turns red. Revert each.
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
- Each gate command and its exit status at the PR head.

## Dependencies

- *Run doctests in CI* (#1422): D4's guarantee counts only once CI runs doctests.
  *Bound route gain and matrix values* (#1237) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt record

### Attempt 1 (implementer, stream J batch 2)

**Change.** `crates/host-core/src/route_controls.rs` only. `RouteControlRecord` is now
`pub struct RouteControlRecord(graph::RouteControlRecord)` with a private field, the derives
`Clone, Copy, Debug, PartialEq` and the `const` accessors `target`, `mute`, `length` (D1).
`RouteControlProducer::record` builds it (`graph::RouteControlRecord::new(..).map(RouteControlRecord)`);
`push` takes it and pushes `record.0`. `RouteControlResources` stays a re-export. The module doc's
"One authority" section says the producer is the only builder. `lib.rs` and host-web are unchanged
and compile (D2). graph is unchanged (D3).

**Doctests (D4).** On the newtype: two `compile_fail` fences and one plain twin. All three share
`fn send(producer: &mut host_core::RouteControlProducer, inner: graph::RouteControlRecord) ->
Result<(), host_core::RouteControlError>` with `let record: host_core::RouteControlRecord = <expr>;
producer.push(record)`; only `<expr>` differs: `host_core::RouteControlRecord::new([1.0, 0.0, 0.0,
1.0], false, 0).unwrap()` (fence 1), `host_core::RouteControlRecord(inner)` (fence 2),
`producer.record(0.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2], 0).unwrap()` (twin). Codes are
comments only; no `RUSTC_BOOTSTRAP`.

**Each fence fails only for its stated reason.** With both fences turned into plain doctests, rustc
reports exactly one error each: fence 1 `error[E0599]: no associated function or constant named
`new` found for struct `host_core::RouteControlRecord``; fence 2 `error[E0423]: cannot initialize a
tuple struct which contains private fields`. Reverted.

**Mutation runs** (`cargo test --locked -p host-core --features control-provider,test-support --doc
route_controls`; each reverted, then green 3/3):

| Mutation | Fence 1 (`new`) | Fence 2 (tuple) | Twin |
|---|---|---|---|
| Spec's: `pub use graph::RouteControlRecord` in place of the newtype (record/push restored) | **red** | green | green |
| Public `new(..) -> Option<Self>` on the newtype | **red** | green | green |
| Field made `pub` | green | **red** | green |
| Pair rename: `RouteControlProducer` -> `RouteControlProduce` in all three snippets | green | green | **red** (E0425) |
| Pair 1 delete: fence 1's `<expr>` replaced by the twin's | **red** | green | green |
| Pair 2 delete: fence 2's `<expr>` replaced by the twin's | green | **red** | green |

**Gates at the attempt head** (all exit 0):

1. `cargo test --locked -p host-core --features control-provider,test-support --doc`: 8 passed.
2. host-core (`control-provider,test-support`): 283 passed; host-web (`test-support`): 189 passed;
   graph-compiler `live_routes`: 11 passed; `route_coefficients`: 5 passed.
3. `build-web-audioworklet.sh --named-twin`: shipped module
   `46c8b035206b350470b41d7f20a4cd29341215e556511b8fb1f21bd71a610e79` (2896155 B);
   `check-web-audioworklet.sh` and `check-browser-expected-resources.py --artifacts` pass (the
   module digest agrees with `expected.json`, so no rendered bit moved);
   `test-web-audioworklet.sh` passes.
4. `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   `check-host-core-policy.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh`.

*Test value.* Fence 1 is red if host-core again exposes graph's record or gives the newtype a
public constructor; fence 2 is red if the newtype's field becomes public; no runtime test can see
either, because the bypass is an API.
