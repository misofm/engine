# Sol implementation brief — issue 103 F2/F3 C ABI soundness

## Decision

**TERMINAL PRE-IMPLEMENTATION STOP — QUALIFICATION UNPROVEN.** Fresh explicit owner authorization
resumed F2/F3 from `main` at `3be899f`, but its sole pre-fix Miri slot is consumed without a
persisted completion result. Pinned Miri installed, the build succeeded and the exact E1 reported
`running 1 test`; an external platform safety-classifier interruption then ended the reporting
turn while the test was still in progress. No completion output, exit status or final diagnostic
was delivered or persisted, and no Cargo/Miri process survived. This is neither valid red nor
green evidence. No rerun, corrected invocation or implementation is authorized by this stopped
brief.

Issue 103 remains the final Issue-125 Step-0 gate. Step 1 does not begin while this brief is stopped;
F2/F3 still requires pushed implementation, Sol XHigh PASS and green synchronized evidence.

## Qualification scaffold

Before production edits, add only
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render` inside `ffi.rs`'s test module. Carry
the original `*mut Plan` in a test-local opaque `SendPlanPtr`; give only that wrapper an audited
`unsafe impl Send`, do not implement `Copy`, `Clone` or `Sync`, and recover the pointer through a
method called inside `std::thread::scope`. Keep render on the parent thread, explicitly join the
query thread inside the scope, and destroy the handles afterward.

The preserved candidate is `crates/miso-engine-capi/src/ffi.rs` blob
`d09e3f289e85770a41335fdd0bfdb58a771173da` at stopped-checkpoint HEAD `1b36d7a`. It passed the
final PRE-MIRI review and remains byte-for-byte unchanged. No `.addr()`, pointer/integer round-trip,
`expose_provenance`, `with_exposed_provenance`,
`without_provenance` or equivalent reconstruction is permitted. Sol must inspect the test-only
diff, format/Clippy-compile it, and prove the exact test name appears once before Miri. The current
whole-Plan/`RefCell` production defect must remain unchanged for the red run.

Use this exact pinned-nightly scaffold lint command:

```sh
cargo +nightly-2026-08-20 clippy --locked --no-deps \
  -p miso-engine-capi --lib --tests -- \
  -D warnings -A clippy::chunks_exact_to_as_chunks
```

`--no-deps` excludes six pre-existing protocol diagnostics outside the fence. The sole named
allowance covers only the pre-existing CAPI runtime `chunks_exact(2)` occurrence; all other CAPI
warnings remain denied, including scaffold unsafe-documentation failures. This qualification-only
allowance does not alter the final stable all-targets `-D warnings` gate, which retains no allowance.

## Exact implementation

Split `Plan` into a raw-projectable `PlanResourceView` over an `Arc<SharedPlanState>`, one
`AtomicU32` diagnostic code, and render-exclusive `PlanState`. Every plan FFI entry projects only
the needed field; no whole `&Plan`/`&mut Plan` is formed. Resource queries keep the current-to-
replacement report transition and do not clear diagnostics. Replace the plan `RefCell<FixedBytes>`
with the spec's fixed code/text table and rederive the resource delta independently.

Before any slice construction, apply semantic cap, `isize::MAX`, checked conversion/extent and
alignment checks to TOML, source IDs, control frames, plane arrays, planes and render output exactly
as the issue spec states. Preserve diagnostic precedence and allow aligned retry after every
rejection.

Update the header's ownership table for all 14 exports and Issue-022's decision record. Do not
claim the synchronized report query can never block.

## Proof

E1 is the 2,000-render concurrent query test plus one fresh pinned-Miri run at 16 iterations that
must reach the retained production whole-Plan alias/data-race defect, followed by one identical
corrected green. A wrapper/header/barrier/setup/toolchain failure or unexpected pass consumes the
fresh pre-fix slot without satisfying E1 and is STOP. A failure of the sole corrected run is also
STOP; no retry, alternate filter, substitute toolchain or tuning is authorized.
E2 proves resource queries preserve the last render diagnostic. E3/E4 prove oversized and
misaligned dangling inputs reject before reads and aligned retries succeed. E5 forbids whole-plan
references. E6 proves handles are `Send` while Session/Plan are not `Sync`. Execute and revert every
named red mutation, then run the complete command list in the issue spec.

Terminal counters are: three named invocations total; one historical zero-test invocation; one
historical unrelated scaffold failure; one exact fresh invocation with externally interrupted
result delivery; two exact-name selections total; zero valid pre-fix red, corrected green or valid
Miri evidence; zero valid-workload retries; three preimplementation qualification stops; and zero
implementation attempts started or failed. The current command launch consumed its `1/1` fresh
pre-fix slot. `running 1 test` does not establish completion or the intended whole-Plan diagnostic.

## Fence and stop conditions

This terminal record changes exactly the Issue-103 spec and brief while preserving the existing
test-only `ffi.rs` candidate byte-for-byte. The scaffold is candid stopped-workflow evidence, not
accepted implementation. No production fence is open.

The broader historical implementation fence comprised only the CAPI implementation/header, bounded
resource-lifecycle evidence, Issue-022's single decision amendment, and Issue-103 spec/brief. A
future successor must restate any fence it opens. Cargo, symbols, scripts, fixtures, protocol, core,
graph and hosts remain outside this stopped brief. No benchmark or timing run.

Stop on a pinned-Miri environment failure, lost active-report transition, whole-Plan reference,
unexplained resource-owner delta, or any cap/alignment check that cannot precede slice creation.
Also stop if either fresh Miri invocation misses its required outcome. The three-attempt rule does
not create extra Miri slots or permit stale Miri evidence after an ownership/projection revision.
Do not weaken a gate or land a partial F2/F3 checkpoint.

The current workflow has stopped. The owner has explicitly directed a retry rather than moving on,
but that requires a materially respecified successor, not another amendment authorizing the same
run. Its authority must own and preflight durable combined-output and atomic exit-status capture
independent of the reporting turn, persistent and recoverable process identity, explicit
interrupted-run semantics, a fresh exact budget/fence and a new Sol XHigh PRE-MIRI review. Until
that synchronized successor exists, no Miri or production edit may start.

## Terminal verdict

The first named Miri command selected zero tests. The one authorized replacement ran the exact E1
test but stopped on pointer provenance erased by the scaffold's integer round-trip before reaching
the intended whole-Plan alias/data-race red. Sol XHigh returned terminal pre-implementation STOP:
two named Miri invocations, zero valid red/green evidence, zero retries of a valid workload, and
zero implementation attempts. Production is unchanged and the flawed scaffold is not retained.

The owner subsequently supplied that explicit fresh authorization. The terminal facts and counters
remain historical evidence. The fresh workflow then consumed its one pre-fix slot: pinned Miri
reported `running 1 test`, but an external reporting interruption left no persisted completion,
status or diagnostic and no surviving process. No rerun or edit occurred. This is the third failed
qualification shape, qualification remains unproven, and implementation attempts remain zero.
Preserve exact scaffold blob `d09e3f2` in the candid stopped checkpoint. The owner's subsequent
explicit retry direction permits preparation and synchronization of a materially respecified
durable-capture successor; it does not permit a direct fourth qualification retry under this brief.
