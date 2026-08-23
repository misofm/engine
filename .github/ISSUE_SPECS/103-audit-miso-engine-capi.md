# 103 Audit: miso-engine-capi

## Outcome and readiness

Close audit findings F2 and F3 as one indivisible wave-0 C ABI soundness checkpoint: plan queries
must be legal concurrently with the exclusive render owner, and every caller-owned byte/plane/
output region must be rejected before Rust constructs a slice when its length, extent, or alignment
is invalid.

**TERMINAL PRE-IMPLEMENTATION STOP — QUALIFICATION UNPROVEN.** Fresh explicit owner authorization
on 2026-08-23 resumed this slice from synchronized `main` at `3be899f`, but the sole newly
authorized pre-fix Miri invocation has now been consumed without a persisted completion result.
Pinned Miri installed, the build succeeded and the exact E1 reported `running 1 test`; the last
delivered output showed the test still in progress with no setup or provenance failure. The
implementer's reporting turn was then interrupted by an external platform safety-classifier error.
No completion output, exit status or final diagnostic was delivered or persisted, and no
Cargo/Miri process survived for recovery. The invocation is neither an intended red nor a green.
No rerun, corrected invocation or production implementation is authorized by this stopped brief.

Issue 103 remains open. After this F2/F3 slice passes, F1 and the later wave-4 CAPI/web facade work
remain separate scope.

### Consumed owner-rescope qualification budget

The immutable terminal counters are:

- `miri_named_invocations_total=3`;
- `invalid_zero_test_miri_invocations=1`;
- `unrelated_scaffold_failure_miri_invocations=1`;
- `incomplete_external_reporting_miri_invocations=1`;
- `exact_named_miri_invocations=2`;
- `owner_rescope_pre_fix_invocations_launched=1`;
- `owner_rescope_pre_fix_slots_consumed=1`;
- `owner_rescope_pre_fix_red_invocations=0`;
- `valid_pre_fix_red_invocations=0`;
- `valid_corrected_green_invocations=0`;
- `valid_miri_evidence_invocations=0`;
- `miri_retries_of_valid_workload=0`;
- `implementation_attempts_started=0`;
- `failed_implementation_attempts=0`;
- `preimplementation_qualification_stops=3`;
- `corrected_green_slots_available=0`;
- `fresh_miri_invocations_authorized=0`.

The command launch consumes the slot under the synchronized one-run law even though the external
interruption erased its completion evidence. `running 1 test` proves exact selection, not that E1
completed or reached the intended whole-`Plan` conflict. The absent result cannot be reconstructed
from the absence of a surviving process. No scaffold edit or rerun occurred after launch.
Implementation attempt 1 never began because no valid intended red was captured and production was
never edited. This is the third failed qualification shape in the Issue-103 lineage, not a failed
implementation attempt and not permission to reset the counters.

### Historical briefing/preflight correction

The first Sol High turn stopped before implementation after the originally named pre-fix Miri
command ran zero tests (`0 passed; 18 filtered out`): E1 had not yet been scaffolded. It exercised
no code and is invalid evidence, not an implementation attempt. Sol XHigh approved one bounded
brief correction without weakening Miri:

- `implementation_attempts_started=0` and `failed_implementation_attempts=0` at correction time;
- `invalid_zero_test_miri_invocations=1`;
- `tests_executed_by_invalid_invocation=0`;
- `valid_miri_evidence_invocations=0`.

The prior correction required only the E1 qualification scaffold in `ffi.rs` while retaining the
whole-Plan/`RefCell` defect, froze its exact name as
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render`, and required a non-Miri `--list`
preflight before one replacement run. That run was required to reach the expected whole-Plan
alias/data-race defect; an unrelated failure or pass was STOP.

That correction terminated without valid evidence, as recorded in the terminal evidence below.
Its obsolete projected-success counters are superseded by the cumulative fresh-rescope counters
above; the underlying invocation history is unchanged.

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

## Qualification-only scaffold law

Before any production edit, add E1 only inside `ffi.rs`'s existing `#[cfg(test)] mod tests`. The
test must keep the current production whole-`Plan`/`RefCell` defect intact. Its sole cross-thread
pointer carrier is this test-local shape:

```rust
struct SendPlanPtr(*mut Plan);

// SAFETY: This test-only token moves the original pointer without dereferencing or changing its
// provenance. The scoped query thread uses it only under E1's documented concurrent-query
// contract, joins before destruction, and Miri verifies production's projected accesses.
unsafe impl Send for SendPlanPtr {}

impl SendPlanPtr {
    fn new(plan: *mut Plan) -> Self {
        Self(plan)
    }

    fn into_ptr(self) -> *mut Plan {
        self.0
    }
}
```

Do not derive or implement `Copy`, `Clone` or `Sync`. Construct `SendPlanPtr::new(plan)` from the
original compile output on the parent thread. Move the opaque wrapper into a closure created by
`std::thread::scope`; recover the raw pointer only by calling `into_ptr()` inside that closure.
Keep the render loop on the parent thread, retain the two-boundary barrier protocol, explicitly
`join()` the query handle inside the scope, and destroy session/plan only after the scope returns.

The scaffold may contain no `plan.addr()`, pointer-to-integer/integer-to-pointer cast,
`expose_provenance`, `with_exposed_provenance`, `without_provenance` or equivalent reconstruction.
It may not add any production helper or unsafe trait implementation outside the test-only wrapper.
Before Miri, Sol must inspect the `ffi.rs` diff and record that every added hunk is inside the test
module, the exact test name occurs once, production paths are unchanged from the rebrief base, and
the prohibited provenance operations are absent. A scaffold defect discovered before Miri may be
corrected without consuming the Miri budget; once Miri launches, that invocation is consumed.

## Acceptance gates and red mutations

### E1 — concurrent query/render ownership

Run 2,000 renders while another thread repeatedly calls `plan_resources` and `last_error`; run 16
iterations under pinned Miri. Every call returns `OK`, the active report is exact and the error is
empty. Before the fix, and as a red mutation, restore a whole-plan mutable reference: Miri or the
static gate must fail. The fresh pre-fix transcript is valid only if it reports `running 1 test`
and reaches the production whole-plan alias/data-race conflict between render and a plan query.
A failure in the wrapper, handle header, barrier, fixture setup, allocator or toolchain is unrelated
and consumes the fresh pre-fix slot without satisfying E1.

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

After adding only the provenance-preserving E1 scaffold with the production defect intact, first
format/compile it and preflight its exact name without executing the test:

```sh
cargo fmt --all -- --check
cargo +nightly-2026-08-20 clippy --locked --no-deps \
  -p miso-engine-capi --lib --tests -- \
  -D warnings -A clippy::chunks_exact_to_as_chunks
cargo +nightly-2026-08-20 test --locked -p miso-engine-capi --lib -- --list
```

This is a qualification-only command correction. `--no-deps` prevents the pinned nightly from
turning six pre-existing `miso-engine-protocol` diagnostics outside Issue 103's path fence into a
CAPI scaffold failure. The single named allowance applies only to the pinned nightly's pre-existing
`miso-engine-capi/src/runtime.rs` `chunks_exact(2)` occurrence; that production occurrence predates
the scaffold and cannot be changed during the test-only qualification checkpoint. `-D warnings`
still denies every other CAPI warning, including `undocumented_unsafe_blocks` in the new scaffold.
The final stable all-targets Clippy command below remains unchanged, has no allowance, and must pass
before F2/F3 acceptance.

The following command sequence is retained as the consumed owner-rescope authority, not as current
permission to launch it again. Its output had to contain exactly one
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render: test` line. Sol must also complete
and record the qualification-only diff inspection defined above. Only then run the one newly
authorized pre-fix pinned-Miri qualification. That slot is now consumed:

```sh
rustup +nightly-2026-08-20 component add miri
cargo +nightly-2026-08-20 miri test --locked -p miso-engine-capi --lib -- \
  ffi::tests::plan_queries_are_pure_and_concurrent_with_render --exact --nocapture
```

It must run exactly one test and fail specifically at the retained production whole-Plan
alias/data-race conflict. An unrelated failure or unexpected pass is STOP: preserve the transcript,
do not edit production, do not invoke Miri again, and synchronize the consumed-slot counters.

Had the intended red been captured, implementation attempt 1 would have retained the identical
test name, wrapper, filter and iteration count. Once F2/F3 and all non-Miri gates are green, run the
exact command above once more as the sole corrected invocation. It must run one test and pass. Any
corrected-run failure is STOP with no retry, alternate filter, toolchain substitution or tuning. A
later implementation revision may not reuse stale Miri evidence if it changes F2 ownership/
projection code; no further Miri slot is implicit in the three-attempt rule.

Because no intended red was captured, none of those implementation or corrected-run steps are now
authorized. The command list below is retained only as the acceptance contract for a future
materially respecified successor.

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

Miri cannot run the C smoke tests. If pinned Miri cannot be installed or the named test cannot run
for an environmental reason, record the exact failure and stop before implementation; do not
substitute an unpinned toolchain.

## Evidence and completion

Record the base/candidate/tree and exact changed paths; both historical invalid-evidence
invocations, the fresh owner-rescope authorization, qualification-only diff inspection, exact-name
preflight and final cumulative Miri counters; old and new Plan ownership; active-report
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
- If the fresh pre-fix run does not reach the intended production whole-Plan conflict, stop; it
  consumes the new slot and cannot be reframed as E1 evidence.
- If the sole corrected run fails, stop with implementation attempt 1 failed; the three-attempt
  limit does not authorize another Miri invocation.
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

## Owner-rescope decision — 2026-08-23

The owner subsequently gave explicit autonomous authorization to resume. That fresh decision
supersedes only the terminal record's prospective no-authorization sentence; it does not rewrite
the terminal facts or counters. This rebrief checkpoint itself may change only this spec and its
tracked brief. The subsequent qualification checkpoint may change only test code inside
`crates/miso-engine-capi/src/ffi.rs` as specified above. Only after the intended pre-fix Miri red
may implementation attempt 1 use the full allowed tracked-path fence.

Issue 125 remains open and Step 1 remains unstarted until Issue-103 F2/F3 has a pushed Sol XHigh
PASS and green synchronized evidence. No #83 status update is due merely for this rebrief.

## Owner-rescope terminal qualification evidence — 2026-08-23

The pre-Miri checkpoint was HEAD `1b36d7a` with the sole uncommitted candidate
`crates/miso-engine-capi/src/ffi.rs` at blob
`d09e3f289e85770a41335fdd0bfdb58a771173da`. Sol XHigh verified the exact test-only fence, one
exact-name test among 19 listed tests, the opaque provenance-preserving `SendPlanPtr`, barrier,
join and destruction ordering, retention of the production whole-`Plan`/`RefCell` defect, and
passing format plus the synchronized bounded Clippy preflight. The candidate remained byte-for-byte
unchanged after that review and after the Miri launch.

Exactly one synchronized fresh pre-fix Miri command then launched. Pinned Miri was installed, the
build succeeded and its delivered output reported `running 1 test`. The exact E1 remained in
progress without any delivered setup/provenance failure when an external platform
safety-classifier error interrupted the implementer's reporting turn. No completion output, exit
status or final diagnostic was delivered or persisted. Root subsequently found no surviving
Cargo/Miri process. No rerun, alternate filter, test edit or production edit occurred. Therefore
the invocation consumed the sole slot but supplies zero valid Miri evidence. Qualification is
unproven and the corrected-green slot never opened.

The valid scaffold is candid failed-qualification evidence and must be preserved at its exact blob
in the stopped-branch checkpoint rather than discarded or represented as accepted implementation.
This checkpoint must not be merged as an F2/F3 PASS. It leaves the accepted Issue-113–121 behavior,
production defect, fixtures and implementation-attempt counters unchanged.

The three failed qualification shapes are now: the original zero-test briefing defect, the
integer/provenance scaffold defect, and this externally interrupted result-delivery shape. Under
the AGENTS.md three-attempt rule, another amendment granting the same command would be a disguised
fourth retry. The owner has explicitly directed that work retry rather than move on; that direction
requires a newly synchronized, materially respecified durable-capture successor and does not
retroactively reopen this stopped brief.

Before any further Miri launch, the successor must own and preflight a durable capture mechanism
independent of the reporting turn: a persistent runner, predeclared transcript and exit-status
destinations, combined stdout/stderr capture, atomic final-status publication, recoverable process
identity and an explicit interrupted-run classification. It must freeze a new exact invocation
budget, path fence and stop conditions and receive a fresh Sol XHigh PRE-MIRI review. This terminal
record itself authorizes no Miri run, scaffold change or production implementation. Issue 103
remains open and continues to block Issue-125 Step 0.
