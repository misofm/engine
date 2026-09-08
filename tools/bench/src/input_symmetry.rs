//! Issue #600's entirely untimed sustained input-trim qualification.
//!
//! The qualification owns no clock, timer, performance metadata, or capture machinery. It
//! prepares the real eight-track W8 graph, publishes the real input records, renders contiguous
//! blocks, and reports deterministic regression digests.

use bench_support::alloc as bench_alloc;
use bench_support::digest::Sha256Sink;
use builtins::BuiltinLaneSelector;
use builtins_compiler::{
    BuiltinCompileCaps, TrackControlProducer, TrackControlRequest, TrackInputRecord,
    prepare_session_builtins_with_console,
};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderReport, RenderTime,
};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use lane::Backend;
use session::{CompileCaps, compile_session, parse_session_json};
use std::num::NonZeroUsize;

pub(crate) const SAMPLE_RATE_HZ: u32 = 48_000;
pub(crate) const QUANTUM: usize = 128;
pub(crate) const TRACKS: usize = 8;
pub(crate) const RECORDS_PER_BLOCK: usize = 8;
pub(crate) const QUEUE_CAPACITY: usize = 16;
pub(crate) const PREPARATION_BLOCKS: u64 = 512;
pub(crate) const QUALIFICATION_BLOCKS: u64 = 4096;
pub(crate) const QUALIFICATION_PHASES: usize = 2;
pub(crate) const SMOOTHING_SAMPLES: u32 = 256;
pub(crate) const PLAN_ID: u64 = 600;
pub(crate) const FIXTURE_ID: &str = "fixtures/session/v1/parametric-eq-bank-console.json";
pub(crate) const FIXTURE: &str =
    include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");

// Filled from the connected independent oracle and then frozen in the qualification evidence.
const REVIEWED_PHASE_DIGESTS: [&str; QUALIFICATION_PHASES] = [
    "75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87",
    "75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87",
];
const CONNECTED_ORACLE_DIGEST: &str =
    "460411969d6f9a9d238beff68a53a1980364d398aaadaf83e79ae6ee758653e4";

struct Source {
    left: [f32; QUANTUM],
    right: [f32; QUANTUM],
}

impl GraphRuntimeProcessor for Source {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
        block.left.copy_from_slice(&self.left);
        block.right.copy_from_slice(&self.right);
        Ok(())
    }
}

struct Identity;

impl GraphRuntimeProcessor for Identity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
        Ok(())
    }
}

pub(crate) struct Owner {
    plan: PreparedRenderPlan,
    output: Box<[f32]>,
    controls: Vec<TrackControlProducer>,
    phase_digest: Sha256Sink,
    attempted: u64,
    accepted: u64,
    rendered: u64,
    render_errors: u64,
    output_words: u64,
    nonzero_samples: u64,
    next_sample: u64,
    block_index: u64,
    active: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PhaseResult {
    pub(crate) attempted: u64,
    pub(crate) accepted: u64,
    pub(crate) rendered: u64,
    pub(crate) render_errors: u64,
    pub(crate) output_words: u64,
    pub(crate) nonzero_samples: u64,
    pub(crate) digest: String,
}

impl Owner {
    pub(crate) fn prepare() -> Self {
        let model = parse_session_json(FIXTURE).expect("issue 600 fixture parses");
        assert_eq!(model.sample_rate_hz, SAMPLE_RATE_HZ);
        assert_eq!(model.quantum_frames as usize, QUANTUM);
        assert_eq!(model.tracks.len(), TRACKS);
        let session = compile_session(&model, compile_caps()).expect("issue 600 session compiles");
        let controls: Vec<_> = model
            .tracks
            .iter()
            .map(|track| TrackControlRequest {
                track_id: track.id.to_string(),
                queue_capacity: NonZeroUsize::new(QUEUE_CAPACITY).expect("queue capacity"),
            })
            .collect();
        let builtins =
            prepare_session_builtins_with_console(&session, &[], &controls, builtin_caps())
                .expect("issue 600 builtins prepare");
        assert_eq!(builtins.track_control_count(), TRACKS);
        let registry = launch_native_effect_registry().expect("launch effect registry");
        let effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .expect("issue 600 effects prepare");
        let dispatch = Backend::current();
        assert_eq!(
            dispatch,
            Backend::Simd8,
            "issue 600 requires native W8 dispatch"
        );
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
            dispatch,
        })
        .unwrap_or_else(|_| panic!("issue 600 graph compile"));
        assert_eq!(artifact.prepared_builtin_bank_count(), 3);
        assert!(artifact.graph().builtin_bank_info().all(|bank| {
            bank.backend == dispatch && bank.width.lanes() == 8 && bank.members.len() == TRACKS
        }));
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), source_binding(node)))
            .collect();
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|_| panic!("issue 600 graph bind"));
        assert_eq!(bound.track_controls.len(), TRACKS);
        let mut owner = Self {
            plan: bound.plan,
            output: vec![0.0; QUANTUM * 2].into_boxed_slice(),
            controls: bound.track_controls,
            phase_digest: Sha256Sink::new(),
            attempted: 0,
            accepted: 0,
            rendered: 0,
            render_errors: 0,
            output_words: 0,
            nonzero_samples: 0,
            next_sample: 0,
            block_index: 0,
            active: false,
        };
        owner
            .controls
            .sort_by(|left, right| left.track_id.cmp(&right.track_id));
        owner
    }

    pub(crate) fn publish(&mut self) {
        let db = if self.block_index.is_multiple_of(2) {
            -6.0
        } else {
            -12.0
        };
        for control in &mut self.controls {
            if self.active {
                self.attempted = self.attempted.checked_add(1).expect("attempted overflow");
            }
            let result = control.input.try_push(TrackInputRecord::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db,
                smoothing_samples: SMOOTHING_SAMPLES,
            });
            assert!(
                result.is_ok(),
                "issue 600 input queue accepted every record"
            );
            if self.active {
                self.accepted = self.accepted.checked_add(1).expect("accepted overflow");
            }
        }
    }

    pub(crate) fn render(&mut self) -> RenderReport {
        self.render_observed(|plan, io, time| (0, plan.render(io, time)))
            .1
    }

    pub(crate) fn render_observed<F>(&mut self, observe: F) -> (u64, RenderReport)
    where
        F: FnOnce(
            &mut PreparedRenderPlan,
            RenderIo<'_>,
            RenderTime,
        ) -> (u64, Result<RenderReport, RenderError>),
    {
        let output = PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
            .expect("prepared output buffer");
        let (elapsed, result) = observe(
            &mut self.plan,
            RenderIo {
                input: None,
                output,
            },
            RenderTime {
                absolute_sample: self.next_sample,
            },
        );
        let report = match result {
            Ok(report) => report,
            Err(error) => {
                self.render_errors = self.render_errors.checked_add(1).expect("error overflow");
                panic!("issue 600 render failed: {error:?}");
            }
        };
        assert_eq!(report.plan_id, PLAN_ID);
        assert_eq!(report.frames, QUANTUM as u32);
        self.next_sample = self
            .next_sample
            .checked_add(QUANTUM as u64)
            .expect("sample overflow");
        assert_eq!(report.next_absolute_sample, self.next_sample);
        self.block_index = self.block_index.checked_add(1).expect("block overflow");
        if self.active {
            self.rendered = self.rendered.checked_add(1).expect("render overflow");
            self.output_words = self
                .output_words
                .checked_add((QUANTUM * 2) as u64)
                .expect("word overflow");
        }
        (elapsed, report)
    }

    pub(crate) fn absorb(&mut self) {
        for byte in self
            .output
            .iter()
            .flat_map(|value| value.to_bits().to_le_bytes())
        {
            self.phase_digest.update([byte]);
        }
    }

    pub(crate) fn warmup(&mut self) {
        for _ in 0..PREPARATION_BLOCKS {
            self.publish();
            self.render();
        }
    }

    pub(crate) fn begin_phase(&mut self) {
        self.active = true;
        self.attempted = 0;
        self.accepted = 0;
        self.rendered = 0;
        self.render_errors = 0;
        self.output_words = 0;
        self.nonzero_samples = 0;
        self.phase_digest = Sha256Sink::new();
    }

    pub(crate) fn observe_nonzero(&mut self) {
        let count = self.output.iter().filter(|value| **value != 0.0).count() as u64;
        self.nonzero_samples = self
            .nonzero_samples
            .checked_add(count)
            .expect("nonzero sample overflow");
    }

    pub(crate) fn phase_snapshot(&self) -> PhaseResult {
        PhaseResult {
            attempted: self.attempted,
            accepted: self.accepted,
            rendered: self.rendered,
            render_errors: self.render_errors,
            output_words: self.output_words,
            nonzero_samples: self.nonzero_samples,
            digest: self.phase_digest.snapshot_hex(),
        }
    }

    pub(crate) fn qualification_phase(&mut self, phase: usize) -> PhaseResult {
        self.begin_phase();
        for _ in 0..QUALIFICATION_BLOCKS {
            self.publish();
            self.render();
            self.absorb();
            self.observe_nonzero();
        }
        let result = self.phase_snapshot();
        assert_eq!(
            result.attempted,
            QUALIFICATION_BLOCKS * RECORDS_PER_BLOCK as u64
        );
        assert_eq!(result.accepted, result.attempted);
        assert_eq!(result.rendered, QUALIFICATION_BLOCKS);
        assert_eq!(result.render_errors, 0);
        assert_eq!(
            result.output_words,
            QUALIFICATION_BLOCKS * (QUANTUM * 2) as u64
        );
        assert!(result.nonzero_samples > 0, "qualification PCM is all zero");
        assert_eq!(result.digest, REVIEWED_PHASE_DIGESTS[phase]);
        result
    }
}

pub(crate) fn main() {
    bench_alloc::assert_installed();
    assert!(
        std::env::args_os().nth(1).is_none(),
        "input-symmetry takes no arguments"
    );
    let backend = Backend::current();
    assert_eq!(
        backend,
        Backend::Simd8,
        "issue 600 requires native W8 dispatch"
    );
    let mut owners = [Owner::prepare(), Owner::prepare()];
    owners[0].warmup();
    owners[1].warmup();
    let mut results = Vec::with_capacity(QUALIFICATION_PHASES);
    for phase in 0..QUALIFICATION_PHASES {
        let left = owners[0].qualification_phase(phase);
        let right = owners[1].qualification_phase(phase);
        assert_eq!(left, right, "independent owners diverged");
        results.push(left);
    }
    for (phase, result) in results.iter().enumerate() {
        println!(
            "{{\"issue\":600,\"workload\":\"input_symmetry\",\"fixture\":\"{FIXTURE_ID}\",\"backend\":\"{backend:?}\",\"sample_rate_hz\":{SAMPLE_RATE_HZ},\"quantum_frames\":{QUANTUM},\"tracks\":{TRACKS},\"records_per_block\":{RECORDS_PER_BLOCK},\"smoothing_samples\":{SMOOTHING_SAMPLES},\"preparation_blocks_per_owner\":{PREPARATION_BLOCKS},\"qualification_phase\":{},\"qualification_blocks\":{QUALIFICATION_BLOCKS},\"attempted_records\":{},\"accepted_records\":{},\"rendered_blocks\":{},\"render_errors\":{},\"output_words\":{},\"nonzero_samples\":{},\"phase_digest\":\"{}\",\"connected_oracle_digest\":\"{CONNECTED_ORACLE_DIGEST}\"}}",
            phase + 1,
            result.attempted,
            result.accepted,
            result.rendered,
            result.render_errors,
            result.output_words,
            result.nonzero_samples,
            result.digest,
        );
    }
}

fn source_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
    if let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    {
        let index: usize = track_id
            .as_str()
            .trim_start_matches(|character: char| !character.is_ascii_digit())
            .parse()
            .expect("fixture track index");
        let mut source = Source {
            left: [0.0; QUANTUM],
            right: [0.0; QUANTUM],
        };
        for frame in 0..QUANTUM {
            let value = ((frame + index * QUANTUM) as f32 * 0.013).sin() * 0.5;
            source.left[frame] = value;
            source.right[frame] = -value * 0.75;
        }
        Box::new(source)
    } else {
        Box::new(Identity)
    }
}

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

fn effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 28,
        maximum_scratch_bytes: 1 << 28,
        maximum_automation_spans_per_block: 32,
    }
}

fn graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: 100_000,
        maximum_edges: 100_000,
        maximum_schedule_items: 100_000,
        maximum_dependency_levels: 100_000,
        maximum_audio_buffer_samples: 100_000_000,
        maximum_delay_samples_per_edge: 1_000_000,
        maximum_total_delay_samples: 100_000_000,
        maximum_graph_bytes: 100_000_000,
        maximum_plan_bytes: 1_000_000_000,
        maximum_single_allocation_bytes: 100_000_000,
        maximum_finite_tail_samples: 10_000_000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::realtime::audit;

    fn source_value(track: usize, frame: usize) -> f32 {
        ((frame + track * QUANTUM) as f32 * 0.013).sin() * 0.5
    }

    fn expected_block(current: &mut f32, target: f32) -> [f32; QUANTUM * 2] {
        let step = (target - *current) / SMOOTHING_SAMPLES as f32;
        let mut expected = [0.0; QUANTUM * 2];
        for frame in 0..QUANTUM {
            *current += step;
            let source: f32 = (0..TRACKS).map(|track| source_value(track, frame)).sum();
            expected[frame] = source * *current;
            expected[QUANTUM + frame] = -source * 0.75 * *current;
        }
        expected
    }

    fn assert_connected_block(owner: &Owner, expected: &[f32; QUANTUM * 2], report: RenderReport) {
        assert_eq!(report.plan_id, PLAN_ID);
        assert_eq!(report.frames, QUANTUM as u32);
        assert!(owner.output.iter().any(|value| *value != 0.0));
        for (actual, expected) in owner.output.iter().zip(expected) {
            assert!(
                (actual - expected).abs() <= 1.0e-6,
                "PCM mismatch actual={actual} expected={expected}"
            );
        }
    }

    #[test]
    fn connected_runtime_oracle_covers_retargeted_both_channel_ramp() {
        let mut ridden = Owner::prepare();
        let mut current = 1.0_f32;
        let mut actual_digest = Sha256Sink::new();
        for block in 0..4 {
            ridden.publish();
            let report = ridden.render();
            let target = 10.0_f32.powf(if block % 2 == 0 { -6.0 } else { -12.0 } / 20.0);
            let expected = expected_block(&mut current, target);
            assert_connected_block(&ridden, &expected, report);
            for byte in ridden
                .output
                .iter()
                .flat_map(|value| value.to_bits().to_le_bytes())
            {
                actual_digest.update([byte]);
            }
            assert_ne!(
                current.to_bits(),
                target.to_bits(),
                "retarget must precede settle"
            );
        }
        assert_eq!(actual_digest.snapshot_hex(), CONNECTED_ORACLE_DIGEST);

        let mut no_record = Owner::prepare();
        let mut differs = false;
        let mut expected_current = 1.0_f32;
        for block in 0..4 {
            let target = 10.0_f32.powf(if block % 2 == 0 { -6.0 } else { -12.0 } / 20.0);
            let expected = expected_block(&mut expected_current, target);
            let report = no_record.render();
            assert_eq!(
                report.next_absolute_sample,
                (block + 1) as u64 * QUANTUM as u64
            );
            differs |= no_record
                .output
                .iter()
                .zip(expected)
                .any(|(actual, expected)| (actual - expected).abs() > 1.0e-6);
        }
        assert!(
            differs,
            "no-record owner escaped connected PCM discriminator"
        );
    }

    #[test]
    fn separate_capacity_sixteen_drain_witness_has_no_pending_records() {
        let mut witness = Owner::prepare();
        witness.publish();
        witness.render();
        for _ in 0..QUEUE_CAPACITY {
            for control in &mut witness.controls {
                assert!(
                    control
                        .input
                        .try_push(TrackInputRecord::TrimDb {
                            lanes: BuiltinLaneSelector::Both,
                            db: -6.0,
                            smoothing_samples: SMOOTHING_SAMPLES
                        })
                        .is_ok()
                );
            }
        }
        witness.render();
        for _ in 0..QUEUE_CAPACITY {
            for control in &mut witness.controls {
                assert!(
                    control
                        .input
                        .try_push(TrackInputRecord::TrimDb {
                            lanes: BuiltinLaneSelector::Both,
                            db: -12.0,
                            smoothing_samples: SMOOTHING_SAMPLES
                        })
                        .is_ok()
                );
            }
        }
    }

    #[test]
    fn qualification_phase_constants_and_zero_render_allocations() {
        let mut owner = Owner::prepare();
        owner.warmup();
        owner.begin_phase();
        audit::warm_up();
        audit::reset();
        let counters = bench_alloc::current_thread_counters();
        for _ in 0..4 {
            owner.publish();
            owner.render();
            owner.absorb();
        }
        let delta = bench_alloc::current_thread_delta_since(counters);
        assert_eq!(delta.allocations, 0);
        assert_eq!(delta.deallocations, 0);
        assert_eq!(delta.reallocations, 0);
        assert_eq!(audit::snapshot().total(), 0);
        let result = owner.qualification_phase(0);
        assert_eq!(
            result.attempted,
            QUALIFICATION_BLOCKS * RECORDS_PER_BLOCK as u64
        );
        assert_eq!(result.accepted, result.attempted);
        assert_eq!(result.rendered, QUALIFICATION_BLOCKS);
        assert_eq!(result.render_errors, 0);
    }

    #[test]
    fn owners_are_w8_nonzero_and_phase_results_match() {
        let mut first = Owner::prepare();
        let mut second = Owner::prepare();
        first.warmup();
        second.warmup();
        let left = first.qualification_phase(0);
        let right = second.qualification_phase(0);
        assert_eq!(left, right);
        assert_ne!(left.digest, "0".repeat(64));
    }
}
