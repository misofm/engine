# Sol implementation brief — issue 103 F2/F3 C ABI soundness

## Decision

**OWNER-RESCOPED / READY — SOL XHIGH PASS.** Fresh explicit owner authorization resumes F2/F3 from
`main` at `3be899f`. The two historical invalid-evidence Miri invocations and two qualification
stops remain counted; production and implementation-attempt counters remain unchanged at zero.
The fresh workflow authorizes exactly one provenance-preserving pre-fix red and, only after that
intended red, one identical corrected green. It authorizes no other Miri invocation.

Issue 103 remains the final Issue-125 Step-0 gate. Step 1 does not begin merely because this brief
is READY; F2/F3 still requires pushed implementation, Sol XHigh PASS and green synchronized
evidence.

## Qualification scaffold

Before production edits, add only
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render` inside `ffi.rs`'s test module. Carry
the original `*mut Plan` in a test-local opaque `SendPlanPtr`; give only that wrapper an audited
`unsafe impl Send`, do not implement `Copy`, `Clone` or `Sync`, and recover the pointer through a
method called inside `std::thread::scope`. Keep render on the parent thread, explicitly join the
query thread inside the scope, and destroy the handles afterward.

No `.addr()`, pointer/integer round-trip, `expose_provenance`, `with_exposed_provenance`,
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

Cumulative successful counters are: four named Miri invocations total; one historical zero-test
invocation; one historical unrelated scaffold failure; one valid fresh pre-fix red; one valid
corrected green; two valid evidence invocations; zero valid-workload retries; one implementation
attempt started and zero failed. Implementation attempt 1 begins only after the intended red and
the first production edit. The historical failures remain preimplementation qualification stops,
not implementation attempts.

## Fence and stop conditions

This rebrief checkpoint changes exactly the Issue-103 spec and brief. Relative to that checkpoint,
the qualification scaffold changes only test code inside `crates/miso-engine-capi/src/ffi.rs`.
Only after the intended red may implementation attempt 1 use the broader fence below.

Edit only the CAPI implementation/header, the bounded resource-lifecycle evidence, Issue-022's
single decision amendment, and Issue-103 spec/brief. Do not change Cargo, symbols, scripts,
fixtures, protocol, core, graph or hosts. No benchmark or timing run.

Stop on a pinned-Miri environment failure, lost active-report transition, whole-Plan reference,
unexplained resource-owner delta, or any cap/alignment check that cannot precede slice creation.
Also stop if either fresh Miri invocation misses its required outcome. The three-attempt rule does
not create extra Miri slots or permit stale Miri evidence after an ownership/projection revision.
Do not weaken a gate or land a partial F2/F3 checkpoint.

## Terminal verdict

The first named Miri command selected zero tests. The one authorized replacement ran the exact E1
test but stopped on pointer provenance erased by the scaffold's integer round-trip before reaching
the intended whole-Plan alias/data-race red. Sol XHigh returned terminal pre-implementation STOP:
two named Miri invocations, zero valid red/green evidence, zero retries of a valid workload, and
zero implementation attempts. Production is unchanged and the flawed scaffold is not retained.

The owner subsequently supplied that explicit fresh authorization. The terminal facts and counters
remain historical evidence; the READY decision and qualification law above govern the new workflow.
