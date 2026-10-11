//! Issue #1503 D3: an exchange cannot start above the plan gate's revision ceiling, and the
//! refusal comes before the exchange allocates anything.
//!
//! This binary holds only this test: it links the audited allocator, whose counters are the
//! evidence, so no unrelated test may share the process.

#![allow(missing_docs)]

use bench_support::alloc::{assert_installed, current_thread_counters};
use core::num::NonZeroUsize;
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        GATE_REVISION_MAX, PlanExchangeConfig, PlanExchangeError, PlanRevisionGate,
        PrepareRenderPlan, PreparedRenderPlan, RenderEnvelope, plan_exchange_at_revision,
    },
};

fn plan_with_gate() -> (PreparedRenderPlan, PlanRevisionGate) {
    let mut plan = PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: 1,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(2),
            output_channels: NonZeroUsize::new(1).expect("one"),
        },
        scratch: &[],
    })
    .expect("plan");
    let gate = plan.take_revision_gate().expect("a fresh plan's gate");
    (plan, gate)
}

fn config() -> PlanExchangeConfig {
    PlanExchangeConfig {
        retirement_capacity: NonZeroUsize::new(1).expect("one"),
    }
}

/// Red if the ceiling check sits after `plan_exchange_inner`, which allocates the mailbox, the
/// retirement queue and the watermark before the refusal, or if the ceiling itself is refused.
#[test]
fn an_exchange_above_the_gate_ceiling_is_refused_before_it_allocates() {
    assert_installed();

    // The probe is live: an accepted exchange at the ceiling allocates on this thread.
    let (plan, mut gate) = plan_with_gate();
    let mark = current_thread_counters();
    let exchange = plan_exchange_at_revision(plan, &mut gate, GATE_REVISION_MAX, config());
    let delta = current_thread_counters().allocations - mark.allocations;
    let (_publisher, _owner, _retirer) = exchange.expect("the ceiling itself is accepted");
    assert!(delta > 0, "the exchange allocates its queues: {delta}");
    assert_eq!(gate.revision(), GATE_REVISION_MAX);

    for revision in [GATE_REVISION_MAX + 1, u64::MAX] {
        let (plan, mut gate) = plan_with_gate();
        let before = gate.revision();
        let mark = current_thread_counters();
        let refused = plan_exchange_at_revision(plan, &mut gate, revision, config());
        let after = current_thread_counters();
        assert!(
            matches!(
                refused,
                Err(PlanExchangeError::RevisionAboveCeiling { revision: r }) if r == revision
            ),
            "{revision} must be refused"
        );
        assert_eq!(
            after.allocations - mark.allocations,
            0,
            "the refusal allocated (the dropped plan's frees are expected)"
        );
        assert_eq!(gate.revision(), before, "nothing was published");
    }
}
