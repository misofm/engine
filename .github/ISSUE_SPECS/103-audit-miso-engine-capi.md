# 103 Audit: miso-engine-capi

## Outcome and readiness

Close audit findings F2 and F3 as one indivisible wave-0 C ABI soundness checkpoint: plan queries
must be legal concurrently with the exclusive render owner, and every caller-owned byte/plane/
output region must be rejected before Rust constructs a slice when its length, extent, or alignment
is invalid.

**TERMINAL PRE-IMPLEMENTATION STOP.** The original Issue-103 plan predates accepted Issues 113–121.
Sol XHigh rebriefed this slice on 2026-08-23 against `main` at `97e1a03`, preserving the accepted
two-phase replacement, active-epoch resource-report transition, lifecycle, event and ownership
contracts. Two Miri preflight/scaffold failures exhausted the synchronized pre-fix qualification
budget before production changes began. F2/F3 did not start and have no implementation PASS.

Issue 103 remains open after this checkpoint for F1 and the later wave-4 CAPI/web facade work.

### Briefing/preflight correction

The first Sol High turn stopped before implementation after the originally named pre-fix Miri
command ran zero tests (`0 passed; 18 filtered out`): E1 had not yet been scaffolded. It exercised
no code and is invalid evidence, not an implementation attempt. Sol XHigh approved one bounded
brief correction without weakening Miri:

- `implementation_attempts_started=0` and `failed_implementation_attempts=0` at correction time;
- `invalid_zero_test_miri_invocations=1`;
- `tests_executed_by_invalid_invocation=0`;
- `valid_miri_evidence_invocations=0`.

Before production changes, add only the E1 qualification scaffold in `ffi.rs` while retaining the
whole-Plan/`RefCell` defect. Freeze its exact name as
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render`. A non-Miri `--list` preflight must
find that exact name once. Only then is one replacement valid pre-fix run authorized; it must say
`running 1 test` and fail for the expected whole-Plan alias/data-race defect. An unrelated failure
or pass is STOP. After that red, implementation attempt 1 may begin and exactly one identical
corrected run must execute one test and pass.

Final successful counters are `miri_named_invocations_total=3`,
`invalid_zero_test_miri_invocations=1`, `valid_pre_fix_red_invocations=1`,
`valid_corrected_green_invocations=1`, `valid_miri_evidence_invocations=2`,
`miri_retries_of_valid_workload=0`, `implementation_attempts_started=1`, and
`failed_implementation_attempts=0`. No alternate filter, substitute toolchain or extra retry is
authorized.

## Accepted authority and frozen behavior

Preserve the accepted Issue-113 through Issue-121 implementation, including:

- two-phase protocol transaction and plan reservation order;
- publication/retirement credits, cancellation, provider epochs and lifecycle/drop matrices;
- the active epoch `Release`/`Acquire` handoff and current-to-replacement resource report change at
  the render boundary;
- CAPI-owned render observations/events and the 14 exported symbols;
- atomic caller-output behavior and session/plan destruction in either order after quiescence.

No ABI symbol, numeric result code, public struct layout, protocol, graph, core, fixture, host or
accepted transaction meaning changes in this slice.

## F2: indivisible plan ownership split

`Plan` has three disjoint projected regions:

1. An immutable-after-construction `PlanResourceView` holding an `Arc` clone of the existing
   `SharedPlanState`. It loads the active epoch with the accepted `Acquire` ordering and selects the
   current report from the existing synchronized two-report arena. It must reflect a committed
   replacement after the render boundary; it must not freeze the initial report.
2. `AtomicU32 last_error`, written by render and readable concurrently with `Relaxed` ordering.
3. `PlanState`, exclusively borrowed by the one render thread.

FFI entry points may reach these regions only with raw field projection (`&raw`/`addr_of!`). They
must never form `&Plan` or `&mut Plan`. Handle-kind validation reads/copies only the
`HandleHeader` prefix. `plan_resources` touches only the resource view and never clears the render
diagnostic. The report arena may synchronize with serialized control updates; documentation must
not claim that the query can never block.

Remove the plan's `RefCell<FixedBytes>` diagnostic allocation. Store one fixed code in
`last_error` and copy the associated static byte string on query:

| code | text |
| --- | --- |
| `NONE` | empty |
| `OUTPUT_OVERFLOW` | existing output-overflow text |
| `CONTRACT_REJECTED` | existing contract-rejection text |
| `OUTPUT_PLATFORM` | existing platform-limit text |
| `OUTPUT_LAYOUT` | `render.output.rejected` |
| `PLAN_REJECTED` | existing plan-rejection text |
| `OUTPUT_UNALIGNED` | the new misaligned-output text |
| unknown | `render.internal` |

A successful render stores `NONE`. Removing the diagnostic allocation may change opaque plan size
and `capi_retained_bytes`; rederive those totals with the existing independent primitive-owner
oracle, remove only the diagnostic owner, and keep exact-cap/one-below and lifecycle mutations.

## F3: bounded aligned borrowed memory

All checks happen before `from_raw_parts`, `from_raw_parts_mut`, dereference, or caller-memory copy:

1. `borrowed_bytes(data, bytes, limit)` rejects null, `bytes > limit`, and
   `bytes > isize::MAX as u64`, then performs checked `usize` conversion.
2. TOML keeps its typed `capi.toml.limit` precedence before the bounded helper.
3. Source IDs use `MAX_SOURCE_ID_BYTES = 127`.
4. Control requests use the compiled codec's `max_frame_bytes` before borrowing.
5. `chunk.planes` is nonnull and aligned for `*const f32`; `plane_count` remains `1..=255`, which
   bounds the pointer-array extent.
6. Every plane is nonnull and `f32`-aligned; `frames * size_of::<f32>() <= isize::MAX` precedes
   construction of every plane slice.
7. Render output is nonnull and `f32`-aligned; checked `stride + frames`, required samples no
   greater than declared capacity, checked `usize`, and required byte extent no greater than
   `isize::MAX` all precede `from_raw_parts_mut`.
8. `BytesOut` needs no new alignment rule because `u8` alignment is one; copy only the actual Rust
   slice length, never arbitrary caller capacity.

Misaligned source data returns `source.chunk.unaligned`; oversized source data returns
`source.chunk.oversized`; misaligned render output stores `OUTPUT_UNALIGNED` and returns
`INVALID_ARGUMENT`.

## C header thread and borrowed-memory contract

Document the complete 14-symbol surface:

- ABI version/capabilities: any thread.
- Engine: one serialized control owner.
- Session submit, seek, command, `dequeue_event`, and session error: one serialized control owner.
- Plan render: one exclusive render thread and never concurrent with itself.
- Plan resources/error: callable concurrently with render; resources use only the synchronized
  report view and error uses one atomic word.
- Every destroy requires quiescence; session and plan may be destroyed in either order.
- Borrowed memory lasts for the call only and is never retained; each output region has one
  exclusive writer for the call.

Do not place the ABI mutation script's replacement tokens in comments.

## Allowed tracked paths

- `crates/miso-engine-capi/src/abi.rs`
- `crates/miso-engine-capi/src/ffi.rs`
- `crates/miso-engine-capi/src/runtime.rs`
- `crates/miso-engine-capi/include/miso_engine_v2.h`
- `crates/miso-engine-capi/tests/resource_lifecycle.rs`, only for independently rederived owner
  totals and accepted lifecycle regressions
- `.github/ISSUE_SPECS/022-stable-c-abi-and-host-fed-planar-pcm-render.md`, only for the synchronized
  thread/diagnostic/report decision
- this spec and its tracked brief

Cargo files, protocol, core, graph, hosts, fixtures, C/C++ fixtures, ABI scripts, symbols and every
other path are outside the fence.

## Acceptance gates and red mutations

### E1 — concurrent query/render ownership

Run 2,000 renders while another thread repeatedly calls `plan_resources` and `last_error`; run 16
iterations under pinned Miri. Every call returns `OK`, the active report is exact and the error is
empty. Before the fix, and as a red mutation, restore a whole-plan mutable reference: Miri or the
static gate must fail.

### E2 — resource query is diagnostic-pure

After a rejected render, call `plan_resources`, then `last_error`; the original
`CONTRACT_REJECTED` text remains. Restoring diagnostic clearing in `plan_resources` must fail.

### E3 — oversized borrowed lengths reject before reads

Use dangling pointers with source-ID lengths `u64::MAX`, `128`, and `isize::MAX + 1`; a request one
byte over its compiled cap; and TOML length `isize::MAX + 1` under a `u64::MAX` TOML cap. All reject
before reading, and compile outputs remain null. Removing each cap/isize check must fail or trigger
the debug slice precondition abort.

### E4 — misalignment rejects exactly and retry succeeds

Misalign the plane-array pointer, an individual plane and render output. Each rejects with the
specified typed diagnostic and an aligned retry succeeds. Removing each alignment check must fail
or trigger the debug precondition abort.

### E5 — whole-plan references are statically absent

A source scan rejects whole-plan reference forms while allowing raw projected fields. Construct
forbidden strings with `concat!` so the test does not match itself. Reintroducing
`let plan = unsafe { &mut *plan };` must fail.

### E6 — auto-trait ownership

Const assertions prove `Engine`, `Session`, and `Plan` are `Send`; compile-fail doctests prove
`Session` and `Plan` are not `Sync`. Temporarily adding an unsafe `Sync` implementation must make
the compile-fail example unexpectedly compile.

## Required commands

After adding only the E1 scaffold with the production defect intact, preflight its exact name:

```sh
cargo +nightly-2026-08-20 test --locked -p miso-engine-capi --lib -- --list
```

The output must contain exactly one
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render: test` line. Then run one valid
pre-fix and one corrected pinned-Miri invocation with the identical exact filter; no other valid-
workload retry or tuning:

```sh
rustup +nightly-2026-08-20 component add miri
cargo +nightly-2026-08-20 miri test --locked -p miso-engine-capi --lib -- \
  ffi::tests::plan_queries_are_pure_and_concurrent_with_render --exact --nocapture
```

Then run:

```sh
cargo test --locked -p miso-engine-capi --lib plan_queries
cargo test --locked -p miso-engine-capi --lib plan_resources_does_not
cargo test --locked -p miso-engine-capi --lib oversized_borrowed
cargo test --locked -p miso-engine-capi --lib misaligned_planes
cargo test --locked -p miso-engine-capi --lib ffi_never_forms
cargo test --locked -p miso-engine-capi --doc
cargo test --locked -p miso-engine-capi --test resource_lifecycle
cargo test --locked -p miso-engine-capi --all-targets
cargo clippy --locked -p miso-engine-capi --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p miso-engine-capi
cargo fmt --all -- --check
bash scripts/check-capi-abi.sh
bash scripts/test-capi-abi.sh
bash scripts/check-realtime-policy.sh
bash scripts/test-realtime-policy.sh
bash scripts/check-workspace-policy.sh
git diff --check
git diff --exit-code -- fixtures
```

Miri cannot run the C smoke tests. If pinned Miri cannot be installed for an environmental reason,
record the exact failure and stop before implementation; do not substitute an unpinned toolchain.

## Evidence and completion

Record the base/candidate/tree and exact changed paths; the invalid zero-test invocation and final
Miri counters; old and new Plan ownership; active-report
transition before/after a committed replacement; all 14 header ownership rows; diagnostic table;
every cap/alignment site and exact rejection; E1–E6 results; pre-fix and corrected Miri results;
each red mutation; independent resource-owner totals; format/Clippy/test/doc/policy results; frozen
fixture diff; zero benchmark/timing invocations; and Sol High/Sol XHigh verdicts.

After Sol XHigh PASS and an upstream green evidence commit, report F2/F3 complete on Issue 103 but
keep it open. F1 facade de-duplication and later findings remain separate waves.

## Rollback / fallback

- If the active resource report no longer changes after the render boundary, stop: the view froze
  the initial report and violates Issue 121.
- If a whole-Plan reference is needed, stop and redesign the field projection; do not add `Sync`.
- If removing the diagnostic allocation changes any owner total beyond its independently derived
  delta, stop and report the owner mismatch rather than pinning a production getter.
- If an alignment/cap check cannot reject before a slice is formed, stop and report the exact site.
- Do not weaken Miri, mutation, concurrency, ABI or lifecycle gates.

## Explicit non-goals

F1 CAPI/web facade consolidation; new ABI symbols or result codes; protocol/core/graph/host changes;
render algorithm changes; performance claims; benchmark, timing, target-matrix or fixture work.

## Terminal pre-implementation evidence — 2026-08-23

The first named pinned-Miri command selected zero tests (`0 passed; 18 filtered out`) because E1
had not yet been scaffolded. It exercised no code and was invalid evidence. Sol XHigh authorized
one bounded briefing correction: add only E1 while retaining the production defect, prove its
exact name appears once, then run one replacement pre-fix invocation.

The exact-name preflight found one
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render` test, and pinned Miri reported
`running 1 test`. The scaffold had converted the live plan pointer through an integer and rebuilt
it with `with_exposed_provenance`; Miri stopped in `plan_kind` while constructing a no-provenance
`HandleHeader` reference. It never reached the intended whole-Plan alias/data-race defect. This is
invalid E1 evidence but consumed the sole replacement pre-fix slot under the synchronized
unrelated-failure STOP rule. No retry or alternate filter ran.

Terminal counters are:

- `miri_named_invocations_total=2`;
- `invalid_zero_test_miri_invocations=1`;
- `exact_named_miri_invocations=1`;
- `unrelated_scaffold_failure_miri_invocations=1`;
- `tests_executed_by_unrelated_invocation=1`;
- `authorized_pre_fix_miri_slots_consumed=1`;
- `valid_pre_fix_red_invocations=0`;
- `valid_corrected_green_invocations=0`;
- `valid_miri_evidence_invocations=0`;
- `miri_retries_of_valid_workload=0`;
- `implementation_attempts_started=0`;
- `failed_implementation_attempts=0`;
- `preimplementation_qualification_stops=2`;
- `fresh_miri_invocations_authorized=0`.

Production CAPI code, ABI/header, Issue-022 authority, fixtures and accepted Issue-113–121 behavior
remain unchanged. The flawed E1 scaffold is not retained. No benchmark, timing, target-matrix or
fixture command ran.

A future owner-approved rescope must preserve raw-pointer provenance with a test-local opaque
`SendPlanPtr` around the original `*mut Plan`, an audited test-only `unsafe impl Send`, a method
that recovers the pointer inside `std::thread::scope`, and a join before destruction. It must use
no `.addr()`, integer cast or `with_exposed_provenance`. This record does not authorize that edit,
a fresh Miri run or production implementation. Issue 103 remains open and blocks Issue-125 Step 0.

## Wave-0 implementation record — 2026-08-23 (owner-authorized restart)

The terminal pre-implementation stop above was lifted by the owner, who authorized one fresh,
minimal implementation of F2 and F3 on branch `audit-103-wave0` from `origin/main` at `3be899f`.
The prior Miri capture harness was not inherited, extended, or re-run; the rescope guidance in that
record was honoured on one point that mattered — the concurrency regression test moves the plan
pointer through a test-local opaque `SendPlanPtr` with an audited test-only `unsafe impl Send` and
joins inside `std::thread::scope`, using no integer cast, `.addr()`, or `with_exposed_provenance`,
so the provenance the test exists to check is not laundered.

What landed, all in `crates/miso-engine-capi`:

- F2. `Plan` is now `header | queries | last_error: AtomicU32 | state`. `queries` is an immutable
  any-thread projection of the shared plan state; `last_error` is a relaxed atomic index into the
  frozen `runtime::plan_error` text table; `state` stays exclusive to the render thread. `ffi.rs`
  reaches those fields only through the `plan_state`/`plan_error_slot`/`plan_queries` raw
  projections, so no path forms a reference to the whole `Plan`. `miso_engine_v2_plan_resources` is
  pure — its `clear()` on a `const` plan is gone. The plan no longer retains a `FixedBytes`
  diagnostic buffer, so `capi_retained_bytes` on the pinned nine-track fixture drops from 144537 to
  140425 (active CAPI oracle) and the double-live CAPI oracle from 168926 to 164814; the
  independent live-allocation oracle in `tests/resource_lifecycle.rs` was updated in the same
  checkpoint and still drives its exact/one-below cap rows. No reported diagnostic string changed.
- F2 (c). `include/miso_engine_v2.h` gains the frozen "Thread ownership" block, and every exported
  function in `ffi.rs` carries a one-line thread contract in its `# Safety` doc. The block contains
  none of the three strings `scripts/test-capi-abi.sh` rewrites.
- F3. `borrowed_bytes(data, bytes, limit)` rejects null, over-limit, and `> isize::MAX` lengths
  before the slice exists; `source_submit_planar_f32` rejects a misaligned plane array, an
  oversized `frames * 4` extent, and any null or misaligned plane; `render_f32_planar` rejects a
  misaligned `output.samples` with `MISO_ENGINE_V2_INVALID_ARGUMENT` and the new
  `render.output.unaligned` diagnostic.

Class A throughout: no rendered bit changes, the frozen ABI is untouched (13 symbols, 8 structs,
codes 0-8/255), and the two-thread parity and barrier tests in `runtime.rs` pass unchanged. Every
new test landed with its red mutation proven and named in its commit message. F1 and the wave-4
CAPI/web facade work remain open on this issue.
