# Sol implementation brief — issue 103 F2/F3 C ABI soundness

## Decision

**READY / SOL XHIGH REBRIEF PASS.** Land F2 and F3 as one indivisible checkpoint against accepted
Issues 113–121. The old plan's 13-symbol and frozen-initial-report assumptions are stale. Preserve
all 14 symbols and the dynamic active resource report selected through the accepted epoch handoff.

The first Miri command selected zero tests before E1 existed. It is recorded as one invalid
zero-test invocation and consumed no implementation attempt. Add only the exact E1 scaffold first,
with the production defect intact; use non-Miri `--list` to prove
`ffi::tests::plan_queries_are_pure_and_concurrent_with_render` exists once. Then run exactly one
valid pre-fix Miri command with `--exact --nocapture`, require `running 1 test` and the expected
alias/data-race red, implement F2/F3, and run exactly one identical corrected green. The invalid
invocation remains counted; no extra valid-workload retry is authorized.

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

E1 is the 2,000-render concurrent query test plus the corrected preflight sequence, one valid
pre-fix red and one corrected pinned-Miri green.
E2 proves resource queries preserve the last render diagnostic. E3/E4 prove oversized and
misaligned dangling inputs reject before reads and aligned retries succeed. E5 forbids whole-plan
references. E6 proves handles are `Send` while Session/Plan are not `Sync`. Execute and revert every
named red mutation, then run the complete command list in the issue spec.

## Fence and stop conditions

Edit only the CAPI implementation/header, the bounded resource-lifecycle evidence, Issue-022's
single decision amendment, and Issue-103 spec/brief. Do not change Cargo, symbols, scripts,
fixtures, protocol, core, graph or hosts. No benchmark or timing run.

Stop on a pinned-Miri environment failure, lost active-report transition, whole-Plan reference,
unexplained resource-owner delta, or any cap/alignment check that cannot precede slice creation.
Do not weaken a gate or land a partial F2/F3 checkpoint.
