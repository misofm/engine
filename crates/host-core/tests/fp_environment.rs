//! Issue #146 E1/E2 at the host facade: a render entry pins the environment it renders in.
//!
//! The fixture is nine parametric-EQ tracks fed a signal whose samples are all subnormal -- the
//! tail of a fade, which is exactly where hardware flush-to-zero and denormals-are-zero become
//! audible. Three arms render the same blocks:
//!
//! * **canonical** -- the caller's control word already has FTZ and DAZ clear, and the plan is
//!   rendered directly. This is the pin: the bytes the frozen corpus and every browser leg agree on.
//! * **guarded** -- the caller sets FTZ+DAZ and renders through [`StartedRenderSession`]. It must
//!   produce the canonical bytes, which is the claim issue #146 exists to make true.
//! * **unguarded control** -- the caller sets FTZ+DAZ and renders the plan directly, bypassing the
//!   entry. It must *differ*, or the guarded arm proves nothing and the test is vacuous.
//!
//! Red mutation (recorded in `tests/MUTATIONS.md`): delete the `CanonicalFpEnv::enter()` line from
//! `StartedRenderSession::render_contiguous`. The guarded arm then equals the unguarded control
//! arm and differs from the canonical pin.

use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo};
use host_core::{
    HostPrepareCaps, HostShapePolicy, SourceControlSet, SourceSubmission, StartedRenderSession,
    prepare_host_session,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
const QUANTUM: usize = 128;
const BLOCKS: usize = 16;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
    }
}

/// One block of the fade tail: every sample is a distinct `f32` subnormal.
fn tail_block(block: usize) -> ([f32; QUANTUM], [f32; QUANTUM]) {
    let mut left = [0.0_f32; QUANTUM];
    let mut right = [0.0_f32; QUANTUM];
    for (frame, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
        let step = (block * QUANTUM + frame) as u32;
        *left = f32::from_bits(0x0004_0000_u32.wrapping_add(step.wrapping_mul(1_031)));
        *right = -f32::from_bits(0x0002_0000_u32.wrapping_add(step.wrapping_mul(613)));
    }
    (left, right)
}

fn submit(sources: &mut SourceControlSet, block: usize) {
    let (left, right) = tail_block(block);
    sources
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: (block * QUANTUM) as u64,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: QUANTUM as u32,
                end_of_region: false,
            },
        )
        .expect("source block");
}

/// Renders `BLOCKS` quanta straight into the plan, with no render entry and so no guard.
fn render_unguarded(plan: &mut PreparedRenderPlan, sources: &mut SourceControlSet) -> Vec<u32> {
    let mut rendered = Vec::with_capacity(BLOCKS * QUANTUM * 2);
    for block in 0..BLOCKS {
        submit(sources, block);
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        plan.render_contiguous(
            RenderIo {
                input: None,
                output,
            },
            (block * QUANTUM) as u64,
        )
        .expect("render");
        rendered.extend(samples.iter().map(|value| value.to_bits()));
    }
    rendered
}

/// Renders `BLOCKS` quanta through the started-session render entry, which pins the environment.
fn render_guarded(session: &mut StartedRenderSession, sources: &mut SourceControlSet) -> Vec<u32> {
    let mut rendered = Vec::with_capacity(BLOCKS * QUANTUM * 2);
    for block in 0..BLOCKS {
        submit(sources, block);
        let mut samples = [0.0_f32; QUANTUM * 2];
        session
            .render_planar(&mut samples, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
            .expect("render");
        rendered.extend(samples.iter().map(|value| value.to_bits()));
    }
    rendered
}

fn prepare() -> (PreparedRenderPlan, SourceControlSet) {
    let (_compiled, prepared) = prepare_host_session(SESSION, &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    (prepared.plan, prepared.sources)
}

#[test]
fn a_started_session_hands_the_plan_back_when_it_stops() {
    let (plan, _sources) = prepare();
    let session = StartedRenderSession::start(plan)
        .unwrap_or_else(|(_plan, rejection)| panic!("attestation: {rejection}"));
    assert_eq!(session.next_absolute_sample(), 0);
    let plan = session.stop();
    assert_eq!(plan.next_absolute_sample(), 0);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::{
        BLOCKS, PreparedRenderPlan, QUANTUM, SourceControlSet, StartedRenderSession, prepare,
        render_guarded, render_unguarded,
    };
    use lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};

    struct Restore(u32);

    impl Drop for Restore {
        fn drop(&mut self) {
            write_mxcsr(self.0);
        }
    }

    /// The MXCSR control bits: DAZ, the six exception masks, the rounding-control field and FTZ.
    /// The low six bits are sticky *status* flags that any arithmetic sets, so an arm that renders
    /// without a guard legitimately returns with more of them set than it started with.
    const MXCSR_CONTROL_BITS: u32 = 0xFFC0;

    fn arm(ftz: bool, guarded: bool) -> (Vec<u32>, u32, u32) {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let caller = if ftz {
            (saved | MXCSR_FTZ | MXCSR_DAZ) & !0x6000
        } else {
            saved & !(MXCSR_FTZ | MXCSR_DAZ)
        };
        write_mxcsr(caller);
        assert_eq!(read_mxcsr(), caller, "the arm's caller word must install");

        let (plan, mut sources) = prepare();
        let rendered = if guarded {
            let mut session = StartedRenderSession::start(plan)
                .unwrap_or_else(|(_plan, rejection)| panic!("attestation: {rejection}"));
            let rendered = render_guarded(&mut session, &mut sources);
            // The plan leaves the render thread the way the contract says it must.
            let plan: PreparedRenderPlan = session.stop();
            drop(plan);
            rendered
        } else {
            let mut plan = plan;
            render_unguarded(&mut plan, &mut sources)
        };
        let observed = read_mxcsr();
        drop(sources);
        (rendered, observed, caller)
    }

    #[test]
    fn a_caller_with_ftz_and_daz_renders_the_canonical_bytes() {
        let (canonical, canonical_word, canonical_caller) = arm(false, false);
        let (guarded, guarded_word, guarded_caller) = arm(true, true);
        let (unguarded, unguarded_word, unguarded_caller) = arm(true, false);

        // Each arm prepares, submits and drops around its renders, and that work sets sticky status
        // flags of its own, so this comparison is over the control bits. The bit-exact claim --
        // status flags included -- is `the_callers_word_is_restored_bit_exactly_after_every_block`,
        // which reads MXCSR the instant each render returns.
        assert_eq!(
            guarded_word & MXCSR_CONTROL_BITS,
            guarded_caller & MXCSR_CONTROL_BITS,
            "the guarded arm must return the caller's control bits"
        );
        assert_eq!(
            canonical_word & MXCSR_CONTROL_BITS,
            canonical_caller & MXCSR_CONTROL_BITS,
            "no arm may change a control bit"
        );
        assert_eq!(
            unguarded_word & MXCSR_CONTROL_BITS,
            unguarded_caller & MXCSR_CONTROL_BITS,
            "no arm may change a control bit"
        );

        assert_eq!(canonical.len(), BLOCKS * QUANTUM * 2);
        assert_ne!(
            unguarded, canonical,
            "issue #146 is vacuous here: FTZ+DAZ did not move this fixture's unguarded render, so \
             the guarded arm proves nothing"
        );
        let divergent = guarded
            .iter()
            .zip(&canonical)
            .filter(|(left, right)| left != right)
            .count();
        assert_eq!(
            divergent,
            0,
            "a caller's FTZ+DAZ must not reach a guarded render: {divergent} of {} words moved",
            canonical.len()
        );
    }

    #[test]
    fn the_callers_word_is_restored_bit_exactly_after_every_block() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        // A word no engine would choose: flush-to-zero, denormals-are-zero, round-toward-zero, and
        // a sticky precision flag that must survive every block.
        let caller = (saved & !0x6000) | MXCSR_FTZ | MXCSR_DAZ | 0x6000 | 0x0020;
        write_mxcsr(caller);

        let (plan, mut sources) = prepare();
        let mut session = StartedRenderSession::start(plan)
            .unwrap_or_else(|(_plan, rejection)| panic!("attestation: {rejection}"));
        assert_eq!(
            read_mxcsr(),
            caller,
            "the start-of-session attestation must leave the caller's word alone"
        );
        for block in 0..BLOCKS {
            super::submit(&mut sources, block);
            let mut samples = [0.0_f32; QUANTUM * 2];
            session
                .render_planar(&mut samples, 2, QUANTUM, QUANTUM, (block * QUANTUM) as u64)
                .expect("render");
            assert_eq!(
                read_mxcsr(),
                caller,
                "block {block} did not restore the caller's exact control word"
            );
        }

        // A rejected render restores it too: a discontinuous absolute sample never reaches the plan.
        let mut samples = [0.0_f32; QUANTUM * 2];
        let rejected = session.render_planar(&mut samples, 2, QUANTUM, QUANTUM, 7);
        assert!(rejected.is_err(), "a discontinuous block must be rejected");
        assert_eq!(
            read_mxcsr(),
            caller,
            "a rejected render must restore the caller's exact control word"
        );

        // And so does a render whose output layout is refused before the plan is touched.
        let mut short = [0.0_f32; QUANTUM];
        let refused = session.render_planar(&mut short, 2, QUANTUM, QUANTUM, 0);
        assert!(refused.is_err(), "a short output must be refused");
        assert_eq!(
            read_mxcsr(),
            caller,
            "a refused output layout must restore the caller's exact control word"
        );

        let plan: PreparedRenderPlan = session.stop();
        drop(plan);
        let _ = &mut sources as *mut SourceControlSet;
    }
}
