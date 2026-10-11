FAIL

# #1416 *Let only host-core build the live route records that hosts push*: verdict, attempt 1

Commit `34274bf34` on `codex/d15-stream-j2`, reviewed as `git diff 34274bf34^ 34274bf34` against the
committed spec (D1 to D4, with root's twin rule from #1422 Amendment 1), AGENTS.md, decision 15 and
the owner principle. I exported the commit to `/tmp/claude-1002/v1416/tree` and built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1416/target`. I did not write to the worktree. Mutations were
applied by a script to the export only, and the file was restored and compared byte for byte after
each run.

The product change is correct. D1, D2 and D3 hold, and no safe public path builds the newtype today.
The attempt fails on its test: fence 1 cannot see the defect class that the spec says it defends.

## MAJOR

1. **`crates/host-core/src/route_controls.rs:41-50` (fence 1): the fence is tied to one signature of
   `new`, so it stays green for the wrong reason when D1 breaks.** The forbidden construct is
   `host_core::RouteControlRecord::new([1.0, 0.0, 0.0, 1.0], false, 0).unwrap()`. It turns red only
   for a `new` that takes exactly `([f32; 4], bool, u32)` and returns a type that has `.unwrap()` to
   the record. I added two other public constructors to the newtype. Each one breaks D1 ("no public
   constructor") and the whole doctest step stays green:
   - `pub fn new(target: [f32; 4], mute: bool, length: u32) -> Self`: fence 1 green, fence 2 green,
     twin green, exit 0. With fence 1 made plain, rustc reports `E0599: no method named 'unwrap'
     found for struct host_core::RouteControlRecord`. This is the same code that the comment at
     `:38` names, for a different reason.
   - `pub const fn new(inner: graph::RouteControlRecord) -> Self`: all three green, exit 0. Fence 1
     plain reports `E0061` (3 arguments supplied, 1 expected) and `E0599` (`unwrap`). This is the most
     likely shape of the regression: a later slice (#1225 C ABI sends, #1347 cells, both on this
     producer) that wants to convert graph's record.

   No other test sees this, because the bypass is an API (no runtime test can see it). So D4 is not
   met: the fence does not show that "`host_core::RouteControlRecord::new(..)` does not exist", only
   that one signature does not exist. The spec's test value ("red if host-core again exposes ... a
   public constructor on the newtype") and the attempt record's (`spec:182`, "Fence 1 is red if
   host-core ... gives the newtype a public constructor") are false for these shapes. This is the
   class that #1422's verdict found (MINOR 2 there). It is MAJOR here because there no guarantee was
   left unprotected (other fences still went red). Here no fence goes red and the guarantee breaks.

   **Fix (measured on the export).** Make the forbidden construct independent of the signature:
   the twin's text plus one statement, `let _ = host_core::RouteControlRecord::new;`. The fence is
   then identical to the existing twin except for that one statement, so the twin rule holds with no
   new twin. Results:
   - Baseline: the fence fails with one error, `E0599: no associated function or constant named
     'new'`.
   - Red under the spec's re-export mutation, `new(..) -> Self` and `new(inner) -> Self`.
   - The spec's own `Option` shape is caught too, because any non-generic `new` compiles as a path
     value.

## MINOR

1. **`route_controls.rs:52-61` and `:34-36`: no fence for `From<graph::RouteControlRecord>`.**
   `impl From<graph::RouteControlRecord> for RouteControlRecord` gives `inner.into()` and
   `RouteControlRecord::from(inner)`: a public way to build the newtype from graph's unchecked
   record. Both fences and the twin stay green (exit 0). D4 names only the tuple constructor. But the
   type's doc ("no public constructor, so every record a host can push has passed
   `route_coefficients`") and the spec's test value ("a way to build a route record without
   `record`'s domain check") cover it. The coordinator named it. `inner` is already in the shared
   signature and is not used there, so the fence costs nothing: replace `<expr>` with `inner.into()`.
   I measured this:
   - Baseline: one error, `E0277: the trait bound 'host_core::RouteControlRecord:
     From<graph::RouteControlRecord>' is not satisfied`.
   - Red under the `From` impl and under the spec's re-export mutation (reflexive `From`).
   - The twin is unchanged.

   Under the owner principle, put it in attempt 2 together with MAJOR 1.
2. **Spec Attempt record `:174-176`: the gate 3 evidence is not accurate.** It says "the module
   digest agrees with `expected.json`, so no rendered bit moved". `hosts/host-web/tests/browser-v1/expected.json`
   holds rendered PCM digests (`oracle`, `pcm`, `directOracle`), not the module's digest. The
   shipped module did change. I built the parent `34274bf34^` from the same path twice. It gives
   `d13e812c70b3f6b5d0080e9e95dd45da4746fe6edd5b31817d2f2982caf601e9` (2896157 B) both times, so
   the build is deterministic. This commit gives `46c8b035...` (2896155 B). In the named twins:
   - One body changes size: host-web's control-plane `ReadyOwnership::push` (1407 to 1383 B, block
     layout).
   - 51 bodies keep their size and differ only in call indices. Example: `SpectrumCaptureObserver::observe`,
     `call 520` to `call 517`. The cause is that `RouteControlProducer`'s impl got a new
     disambiguator (`Ms_`) after the new `impl RouteControlRecord`, and the function order moved.

   The rendered PCM digests do not move (`check-browser-expected-resources.py --artifacts` exit 0),
   so the spec's gate 3 holds. But the spec's hazard ("A newtype should not change generated code")
   is wrong, and the PR's `artifact-identity` summary will show ARTIFACT CHANGED. Correct the
   sentence and record the digest pair for the batch record.

## NIT

1. **`route_controls.rs:38`: the prose claims more than the fences can show.** "A host cannot build
   one with an associated function" is true today, but no `compile_fail` fence can defend it for
   every name. A public constructor with any other name (I added
   `pub const fn from_graph(inner) -> Self`) leaves all fences green. The same is true for a refactor
   to a named public field (`pub struct RouteControlRecord { pub inner: .. }`), where fence 2 fails
   for a different reason. D1 forbids that shape. Privacy and review hold these doors, not the
   doctests. Say so in the doc or the Attempt record, and narrow the sentence to what the fences
   check: `new` with any signature, the tuple constructor, and `From`/`Into` after MINOR 1.

## Points the coordinator asked about

- **Each fence fails only for its stated reason at the commit.** Yes. Fence 1 made plain: one error,
  `E0599 no associated function or constant named 'new'`. Fence 2 made plain: one error,
  `E0423 cannot initialize a tuple struct which contains private fields`. They do not fail only for
  that reason when the guarantee breaks (MAJOR 1, MINOR 1, NIT 1).
- **Bypass matrix** (fence 1 / fence 2 / twin; "uncaught" = whole step exit 0):

  | Mutation | F1 | F2 | Twin |
  |---|---|---|---|
  | `pub use graph::RouteControlRecord` (spec's) | red | green | green |
  | `pub fn new(..) -> Option<Self>` | red | green | green |
  | `pub fn new(..) -> Result<Self, RouteControlError>` | red | green | green |
  | `pub fn new(..) -> Self` | **uncaught** | | |
  | `pub const fn new(inner: graph::RouteControlRecord) -> Self` | **uncaught** | | |
  | `impl From<graph::RouteControlRecord>` | **uncaught** | | |
  | `pub const fn from_graph(inner) -> Self` | **uncaught** (no fence can catch it) | | |
  | tuple field `pub` | green | red | green |
  | named field `pub inner` | **uncaught** (D1 forbids the shape) | | |

  Proposed fences (MAJOR 1, MINOR 1), added beside the existing ones on the export: `::new` as a path
  value is red under re-export, `new -> Self` and `new(inner)`. `inner.into()` is red under re-export
  and `From`. Each fails for one reason at baseline. Both pass with the existing twin.
- **Twin.** It meets the rule. It differs from each fence only in the forbidden expression. The four
  rename runs in the shared part (`RouteControlProducer`, `graph::RouteControlRecord`,
  `RouteControlError`, `push`, each in all three snippets) turn only the twin red. The two delete runs
  (each fence's forbidden expression replaced by the twin's) turn only that fence red.
- **D2/D3.** The diff touches only `route_controls.rs` and the spec. `lib.rs:160-162`, host-web and
  graph are unchanged and compile. host-web's suite and clippy over the whole workspace pass. No
  `Deref`, `Default`, serde, `From` or `AsRef` path exists today. graph's record has no `Default`, so
  a derive cannot be added by mistake. The only safe way to get a newtype is
  `RouteControlProducer::record`. host-core exposes no `GraphRouteControlProducer`
  (`HostLiveControlHandles::route_controls` is `Vec<RouteControlProducer>`, a private-field
  `repr(transparent)` wrapper). A `transmute` needs `unsafe`, and Rust privacy does not cover that.
- **Realtime and acked batch.** `push` copies the 24-byte `Copy` record out (`record.0`) and calls the
  same `try_push`. There is no allocation, lock or syscall. `check-realtime-policy.sh` exits 0.
  `Full` is still decided from `free()` before the push, so an ack cannot come before a drop. No
  behaviour moved: rendered PCM digests are unchanged (MINOR 2 has the module bytes).

## Test value

- **Fence 1 (`route_controls.rs:41`).** Red if host-core again re-exports graph's record, or gives
  the newtype a public `new(target, mute, length)` that returns `Option<Self>` or
  `Result<Self, E: Debug>` (re-export, `Option` and `Result` runs: red). No other test sees an API
  addition. But it does not defend "a public constructor on the newtype" in general (MAJOR 1).
- **Fence 2 (`:55`).** Red if the newtype's tuple field becomes `pub` (run: red, fences 1 and the twin
  green). No other test sees it.
- **Twin (`:68`).** Red if an item that the shared code names (the producer, graph's record, the error
  or `push`) is renamed or moved and the snippets are not updated (four rename runs: twin red, both
  fences green). Without it, such a rename would make both fences pass for the wrong reason.

## Gates run (export of `34274bf34`, x86-64, toolchain 1.97.1)

- **Gate 1.** `cargo test --locked -p host-core --features control-provider,test-support --doc`:
  exit 0, 8 passed (3 plain, 5 `compile_fail`). The CI `test-debug-a` doctest step carries
  `host-core/test-support`. `route_controls` is not feature-gated (`lib.rs:109`), so the fences run
  in CI. Mutation runs: see the matrix. The implementer's six rows reproduce exactly.
- **Gate 2.** All exit 0:
  - host-core (`control-provider,test-support`): 283 passed.
  - host-web (`test-support`): 189 passed.
  - graph-compiler `live_routes`: 11 passed.
  - graph-compiler `route_coefficients`: 5 passed.
- **Gate 3.** All exit 0:
  - `build-web-audioworklet.sh --named-twin`: shipped `46c8b035...` (2896155 B), named twin
    `57855cd5...`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts`: PCM digests agree.
  - `test-web-audioworklet.sh`.
  - Parent module comparison: MINOR 2.
- **Gate 4.** All exit 0:
  - `cargo fmt --all -- --check`
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
  - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
  - `check-host-core-policy.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh`
- **Other checks.** No `RUSTC_BOOTSTRAP`. Only authorized paths changed. No version suffix on new
  names. Not run: cross-targets (no engine or target-specific code changed), C ABI audit (capi does
  not use the type), AArch64 (CI only).

Evidence kept: `/tmp/claude-1002/v1416/ev/` (mutation logs) and the mutation scripts
`/tmp/claude-1002/v1416/{mut,alt,fdiff}.py`.
