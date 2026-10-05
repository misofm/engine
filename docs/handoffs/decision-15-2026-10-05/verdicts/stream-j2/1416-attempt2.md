PASS

# #1416 *Let only host-core build the live route records that hosts push*: verdict, attempt 2

Commit `b42e6b8c1` on `codex/d15-stream-j2`. I reviewed `git diff 34274bf34^ b42e6b8c1 -- crates/host-core
.github/ISSUE_SPECS/1416-*` against the spec at `b42e6b8c1` (D1 to D4, with the twin rule), AGENTS.md,
decision 15 and the owner principle. Attempt 2's own diff (`d23cbf56f..b42e6b8c1`) touches only
`crates/host-core/src/route_controls.rs` (doc comment) and the spec. I exported the commit to
`/tmp/claude-1002/v1416b/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1416b/target`. I did
not write to the worktree. A script applied each mutation to the export only. After each run it
restored the file and compared it byte for byte.

The product change holds: today no public path outside host-core builds a
`host_core::RouteControlRecord`. Each fence fails for exactly one reason. Each fence is red under the
defect class it names. The shared twin is aligned. The attempt passes. One MINOR and three NITs remain
for the follow-ups commit.

## Attempt 1 findings

- **MAJOR-1: fixed.** Fence 1 is now the twin plus `let _ = host_core::RouteControlRecord::new;`.
  Fence 1 is red for each non-generic public `new` I tried: `-> Self`, `(inner) -> Self`,
  `-> Option<Self>`, `-> Result<Self, _>`, zero arguments, and a `&'a` argument. It is also red under
  the spec's re-export. For generic `new`, see NIT-1.
- **MINOR-1: fixed.** Fence 3 (`inner.into()`) is red under `impl From<graph::RouteControlRecord>`. It
  is also red under a manual `impl Into<RouteControlRecord> for graph::RouteControlRecord` and under the
  re-export. For `TryFrom`, see MINOR-1 below.
- **MINOR-2: fixed.** The spec's Attempt record (`spec:172-184`) corrects the gate 3 sentence in
  place. I rebuilt the module from `d23cbf56f` (`--module-only`) and from `b42e6b8c1`
  (`--named-twin`). Both give `ffc62262ec890efd154d6eaa65d615df8d308f340bc2398fcb3b4988b2421cbf`
  (2896155 B). So attempt 2 moves no shipped byte, as the record says. The Hazards line (`spec:84`)
  is a batch follow-up, as root directed. It is not counted here.
- **NIT-1: mostly fixed.** The doc now names the doors the fences check, and it states a limit. That
  limit and one claim are still not exact (NIT-1, NIT-2 and MINOR-1 below).

## MINOR

1. **`crates/host-core/src/route_controls.rs:68-79` and `:95-96`: no fence sees `TryFrom`, but a fence
   could.** I added
   `impl TryFrom<graph::RouteControlRecord> for RouteControlRecord { type Error = RouteControlError; .. Ok(Self(inner)) }`.
   It gives a host `inner.try_into()`, which builds the newtype from graph's unchecked record. All
   three fences and the twin stay green (step exit 0). The doc at `:95-96` says "No fence can see a
   public constructor under another name". That is false for this door.
   - **Fix (measured on the export).** Replace fence 3's expression `inner.into()` with
     `inner.try_into().ok().unwrap()`. `TryInto` is in the 2024 prelude. Through the standard blanket
     impls, `try_into` exists if there is a `From`, an `Into` or a `TryFrom`. `.ok()` puts no `Debug`
     bound on a custom error type.
   - **At baseline:** one error, `E0277: the trait bound 'host_core::RouteControlRecord:
     TryFrom<graph::RouteControlRecord>' is not satisfied`.
   - **Red under:** the re-export, `From`, `Into` and `TryFrom`.
   - **No change elsewhere:** fence 1, fence 2 and the twin keep their results under all of these.
     The twin is unchanged, and it still differs from the fence only in `<expr>`.

   This is the same class as attempt 1's MINOR-1, and it costs one expression. Under the owner
   principle, put it in the follow-ups commit. Update the doc comment at `:68-69` to match.

## NIT

1. **`route_controls.rs:39`, spec `:205` and `:259`: "a public associated `new` of any signature"
   claims too much.** A generic `new` leaves fence 1 green. Fence 1 made plain then fails with
   "type annotations needed", not with E0599. I measured three shapes:
   - `new<T: Into<graph::RouteControlRecord>>(inner: T)`: E0283.
   - `new(inner: impl Into<..>)`: E0283.
   - `new<const N: usize>(..)`: E0284.

   The spec's own `:201` is correct: "any non-generic `new`". Narrow `:39` and the test-value
   sentence to "non-generic". There is an optional stronger form, but it is not the `compile_fail`
   form that D4 names. A plain doctest such as
   `trait Probe { fn new() -> u8 { 0 } } impl Probe for host_core::RouteControlRecord {} let _: u8 = host_core::RouteControlRecord::new();`
   compiles only when no public inherent `new` shadows the trait. I measured it:
   - Red for every public `new`, generic ones included (`E0061` or `E0308`).
   - Red under the re-export.
   - Green for a `pub(crate) fn new`, which is correct.
2. **`route_controls.rs:95-96`: the stated limit leaves out the doors that change a record.** Each of
   these leaves all fences green (step exit 0) and breaks the guarantee at `:34-36`:
   - `impl DerefMut<Target = graph::RouteControlRecord>`: `*record = inner`.
   - `impl AsMut<graph::RouteControlRecord>`.
   - A `pub fn set_inner(&mut self, inner)`.
   - A manual `impl Default` built from graph's `new`.

   No derive can add `Default`, because graph's record has none. Say that a public constructor,
   conversion or mutator under another name is held by privacy and review. Do not say only
   "constructor".
3. **`route_controls.rs:81-82`: the twin's sentence does not describe fence 1.** "Identical except
   that the producer builds the record" is true for fences 2 and 3. Fence 1 also gets its record from
   the producer: it differs from the twin only by the extra `let _ = ..::new;` statement.

## Points the coordinator asked about

- **Any remaining public way to build the newtype from outside host-core? None today.**
  - **Constructors.** The only public source of a value is `RouteControlProducer::record`
    (`route_controls.rs:172-195`). The field is private (`:98`). `RouteControlProducer::new` is
    `pub(crate)` (`:152`).
  - **Trait impls.** The newtype has only `Clone, Copy, Debug, PartialEq` and three `const`
    accessors. No other crate implements a trait for it (searched across `crates hosts tools`).
  - **Doors that do not exist today:** another associated fn, `Default`, `Deref`/`DerefMut`, `AsMut`,
    `TryFrom` and `FromIterator`. The crate's lints refuse `unsafe fn new`: the lib does not build with
    it.
  - **Clone of a value.** No host-core function returns a record except `record`. host-web holds
    records only in its `Route(RouteControlRecord)` variant (`hosts/host-web/src/lib.rs:1905`), and
    those come from `record`.
  - **Path to graph's types.** host-core re-exports only `graph::RouteControlResources` (`:30`). No
    public field or signature in host-core names a `graph::` type. host-core has no path to
    `GraphRouteControlProducer`.
  - **Dependencies.** host-web has no `graph` dependency. capi has `graph` only as a dev-dependency
    (`crates/capi/Cargo.toml:25-27`).
  - **Observation, not a finding.** host-web depends on builtins-compiler.
    `attach_route_live_controls` (`crates/builtins-compiler/src/lib.rs:2974`) returns
    `graph::GraphRouteControlProducer`s, but host-web cannot name graph's record, and host-core hands
    out no unattached artifact. That is the graph level, which D3 keeps public.
- **Each fence fails for exactly one reason at the commit.** I made each fence plain:
  - F1: one error, `E0599 no associated function or constant named 'new'`.
  - F2: one error, `E0423 cannot initialize a tuple struct which contains private fields`.
  - F3: one error, `E0277 ... From<graph::RouteControlRecord> is not satisfied`.

  The codes appear only as comments. There is no `RUSTC_BOOTSTRAP`.
- **Twin alignment.** Each fence is identical to the twin except for the one forbidden construct:
  - F1 adds one statement.
  - F2 and F3 differ in `<expr>` only.

  I renamed six shared items in all four snippets: `RouteControlProducer`, `graph::RouteControlRecord`,
  `RouteControlError`, `push`, the annotated type `host_core::RouteControlRecord` and `.record`. Each
  rename turns only the twin red (E0425 or E0599), and all fences stay green. Deleting each fence's
  forbidden construct turns only that fence red.
- **Realtime and the acked batch.** No code changed in attempt 2. `push` still decides `Full` from
  `free()` before `try_push`, so an ack cannot come before a drop. `check-realtime-policy.sh` exits 0.

## Mutation matrix

Run with `cargo test --locked -p host-core --features control-provider,test-support --doc route_controls`.
"Uncaught" means the whole step exits 0.

| Mutation | F1 `new` | F2 tuple | F3 `into` | Twin |
|---|---|---|---|---|
| Spec's: `pub use graph::RouteControlRecord` | **red** | green | **red** | green |
| `pub fn new(target, mute, length) -> Self` | **red** | green | green | green |
| `pub const fn new(inner) -> Self` | **red** | green | green | green |
| `pub fn new(..) -> Option<Self>` | **red** | green | green | green |
| `pub fn new(..) -> Result<Self, RouteControlError>` | **red** | green | green | green |
| `pub fn new() -> Self` | **red** | green | green | green |
| `pub fn new<'a>(inner: &'a graph::RouteControlRecord) -> Self` | **red** | green | green | green |
| `pub fn new<T: Into<graph::RouteControlRecord>>(inner: T)` | uncaught (NIT-1) | | | |
| `pub fn new(inner: impl Into<..>)` | uncaught (NIT-1) | | | |
| `pub const fn new<const N: usize>(inner)` | uncaught (NIT-1) | | | |
| `impl From<graph::RouteControlRecord>` | green | green | **red** | green |
| `impl Into<RouteControlRecord> for graph::RouteControlRecord` | green | green | **red** | green |
| `impl TryFrom<graph::RouteControlRecord>` | uncaught (MINOR-1; red with the proposed fence) | | | |
| tuple field `pub` | green | **red** | green | green |
| `DerefMut` / `AsMut` / `set_inner(&mut self)` / manual `Default` | uncaught (NIT-2) | | | |
| `pub const fn from_graph(inner)` | uncaught (stated limit) | | | |
| `pub(crate) fn new` (allowed by D1) | green | green | green | green (correct) |
| `pub unsafe fn new` | lib does not build (lint) | | | |
| Six shared renames | green | green | green | **red** |
| Delete F1 / F2 / F3 construct | only that fence **red** | | | |

The implementer's 13 rows reproduce exactly.

## Test value

- **Fence 1 (`route_controls.rs:43`).** Red if host-core re-exports graph's record or gives the
  newtype a public non-generic associated `new` of any arity or return type. Measured: re-export,
  `-> Self`, `(inner)`, `Option`, `Result`, zero-arg and `&'a` all turn it red. No runtime test can see
  an API addition.
- **Fence 2 (`:58`).** Red if the newtype's tuple field becomes `pub` (measured). No other test sees
  it.
- **Fence 3 (`:71`).** Red if host-core adds `From<graph::RouteControlRecord>` or a manual `Into` for
  graph's record, or re-exports graph's record (measured). No other test sees it.
- **Twin (`:84`).** Red if a shared item that the snippets name is renamed or moved and the snippets
  are not updated (six renames measured). Without it, every fence would pass for the wrong reason.

## Gates run

Export of `b42e6b8c1`, x86-64, rustc 1.97.1.

- **Gate 1.** `cargo test --locked -p host-core --features control-provider,test-support --doc`: exit
  0, 9 passed (3 plain, 6 `compile_fail`). The CI `test-debug-a` "Workspace doctests" step carries
  `host-core/test-support`, and `route_controls` is not feature-gated (`lib.rs:109`), so these fences
  run in CI.
- **Gate 2.** All exit 0:
  - host-core (`control-provider,test-support`): 284 passed.
  - host-web (`test-support`): 189 passed.
  - graph-compiler `live_routes`: 11 passed.
  - graph-compiler `route_coefficients`: 5 passed.
- **Gate 3.** All exit 0:
  - `build-web-audioworklet.sh --named-twin`: shipped `ffc62262...` (2896155 B), named twin
    `033567e9...`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts`.
  - `test-web-audioworklet.sh` with a scratch `TMPDIR`, 0 entries left.
  - Parent `d23cbf56f` `--module-only`: the same `ffc62262...`.
- **Gate 4.** All exit 0:
  - `cargo fmt --all -- --check`.
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
  - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
  - `check-host-core-policy.sh`, `check-realtime-policy.sh` (90 regions in 25 files) and
    `check-workspace-policy.sh`.
- **Other checks.** No new version-suffixed names. Only authorized paths changed. Not run:
  cross-targets (no engine code changed), the C ABI audit (capi does not use the type), AArch64 (CI
  only).

Evidence kept: `/tmp/claude-1002/v1416b/ev/` (mutation and gate logs; `ev/probe/` holds the optional
trait-probe runs) and the scripts `/tmp/claude-1002/v1416b/{mut,alt,mut2,gates,g3}.*`.
