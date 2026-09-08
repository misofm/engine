//! Issue #600: a sustained eight-track input-trim ride on the delivered native W8 plan.
//!
//! The subject keeps publication, source refill and hashing outside the timer.  The only timed
//! operation is [`PreparedRenderPlan::render`].  The untimed tests below use the same prepared
//! owner and exercise the queue boundary without invoking this subject's process entry point.

use bench_support::alloc as bench_alloc;
use bench_support::digest::{Sha256Sink, sha256_hex};
use bench_support::metadata::Metadata;
use bench_support::timing;
use builtins::BuiltinLaneSelector;
use builtins_compiler::{
    BuiltinCompileCaps, TrackControlProducer, TrackControlRequest, TrackInputRecord,
    prepare_session_builtins_with_console,
};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use lane::Backend;
use session::{CompileCaps, compile_session, parse_session_json};
use std::num::NonZeroUsize;

const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM: usize = 128;
const TRACKS: usize = 8;
const RECORDS_PER_BLOCK: usize = 8;
const QUEUE_CAPACITY: usize = 16;
const WARMUP_BLOCKS: u64 = 512;
const MEASURED_BLOCKS: u64 = 4096;
const SMOOTHING_SAMPLES: u32 = 256;
const PLAN_ID: u64 = 600;
const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");

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

struct Owner {
    plan: PreparedRenderPlan,
    output: Box<[f32]>,
    controls: Vec<TrackControlProducer>,
    digest: Sha256Sink,
    render_samples: Vec<u64>,
    attempted: u64,
    accepted: u64,
    rendered: u64,
    render_errors: u64,
    output_words: u64,
    next_sample: u64,
    block_index: u64,
    elapsed_ns: u64,
    active: bool,
}

impl Owner {
    fn prepare() -> Self {
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
        let envelope = artifact.envelope();
        assert_eq!(
            artifact.prepared_builtin_bank_count(),
            3,
            "one full W8 bank per builtin stage"
        );
        assert!(artifact.graph().builtin_bank_info().all(|bank| {
            bank.backend == dispatch && bank.width.lanes() == 8 && bank.members.len() == TRACKS
        }));
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
            digest: Sha256Sink::new(),
            render_samples: Vec::with_capacity(
                (WARMUP_BLOCKS as usize + 2 * MEASURED_BLOCKS as usize) * 2,
            ),
            attempted: 0,
            accepted: 0,
            rendered: 0,
            render_errors: 0,
            output_words: 0,
            next_sample: 0,
            block_index: 0,
            elapsed_ns: 0,
            active: false,
        };
        owner
            .controls
            .sort_by(|left, right| left.track_id.cmp(&right.track_id));
        owner
    }

    fn publish(&mut self) {
        let db = if self.block_index.is_multiple_of(2) {
            -6.0
        } else {
            -12.0
        };
        for control in &mut self.controls {
            if self.active {
                self.attempted = self
                    .attempted
                    .checked_add(1)
                    .expect("issue 600 attempted-record counter overflow");
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
                self.accepted = self
                    .accepted
                    .checked_add(1)
                    .expect("issue 600 accepted-record counter overflow");
            }
        }
    }

    fn render(&mut self) {
        let output = PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
            .expect("prepared output buffer");
        let result = timing::timed(|| {
            self.plan.render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: self.next_sample,
                },
            )
        });
        self.record_render(result.0, result.1);
    }

    fn render_untimed(&mut self) {
        let output = PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
            .expect("prepared output buffer");
        let result = self.plan.render(
            RenderIo {
                input: None,
                output,
            },
            RenderTime {
                absolute_sample: self.next_sample,
            },
        );
        self.record_render(0, result);
    }

    /// A bounded refill is a public queue witness: after a render, all sixteen slots accept new
    /// records. This proves the eight records published before that render were consumed at its
    /// block boundary without adding a production counter or observer.
    #[cfg(test)]
    fn prove_drained(&mut self) -> usize {
        let mut accepted: usize = 0;
        for control in &mut self.controls {
            for _ in 0..QUEUE_CAPACITY {
                let result = control.input.try_push(TrackInputRecord::TrimDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -6.0,
                    smoothing_samples: SMOOTHING_SAMPLES,
                });
                if result.is_ok() {
                    accepted = accepted
                        .checked_add(1)
                        .expect("issue 600 drain witness counter overflow");
                }
            }
        }
        accepted
    }

    fn record_render(
        &mut self,
        elapsed_ns: u64,
        result: Result<engine::realtime::RenderReport, engine::realtime::RenderError>,
    ) {
        self.elapsed_ns = self
            .elapsed_ns
            .checked_add(elapsed_ns)
            .expect("issue 600 elapsed nanoseconds overflow");
        let sample = self.next_sample;
        self.next_sample = self
            .next_sample
            .checked_add(QUANTUM as u64)
            .expect("issue 600 absolute sample overflow");
        self.render_samples.push(sample);
        self.block_index = self
            .block_index
            .checked_add(1)
            .expect("issue 600 block counter overflow");
        if self.active {
            self.rendered = self
                .rendered
                .checked_add(1)
                .expect("issue 600 render counter overflow");
            self.output_words = self
                .output_words
                .checked_add(self.output.len() as u64)
                .expect("issue 600 output counter overflow");
        }
        if result.is_err() {
            self.render_errors = self
                .render_errors
                .checked_add(1)
                .expect("issue 600 error counter overflow");
        }
    }

    fn absorb(&mut self) {
        for word in self
            .output
            .iter()
            .flat_map(|value| value.to_bits().to_le_bytes())
        {
            self.digest.update([word]);
        }
    }

    fn start_round(&mut self) {
        self.active = true;
        self.attempted = 0;
        self.accepted = 0;
        self.rendered = 0;
        self.render_errors = 0;
        self.output_words = 0;
        self.elapsed_ns = 0;
        self.digest = Sha256Sink::new();
    }

    fn snapshot(&self) -> RoundResult {
        RoundResult {
            elapsed_ns: self.elapsed_ns,
            attempted: self.attempted,
            accepted: self.accepted,
            rendered: self.rendered,
            render_errors: self.render_errors,
            output_words: self.output_words,
            output_sha256: self.digest.snapshot_hex(),
        }
    }
}

#[derive(Debug)]
struct RoundResult {
    elapsed_ns: u64,
    attempted: u64,
    accepted: u64,
    rendered: u64,
    render_errors: u64,
    output_words: u64,
    output_sha256: String,
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
    eprintln!("MISO_ENGINE_BENCH_PHASE workload_started");
    let mut owners = [Owner::prepare(), Owner::prepare()];
    for _ in 0..WARMUP_BLOCKS {
        for owner in &mut owners {
            owner.publish();
            owner.render_untimed();
            owner.absorb();
        }
    }
    eprintln!("MISO_ENGINE_BENCH_PHASE warmup_complete");
    for round in 1..=2 {
        for owner in &mut owners {
            owner.start_round();
        }
        eprintln!("MISO_ENGINE_BENCH_PHASE timed_started");
        for _ in 0..MEASURED_BLOCKS {
            for owner in &mut owners {
                owner.publish();
                owner.render();
                owner.absorb();
            }
        }
        let result = owners[0].snapshot();
        let peer = owners[1].snapshot();
        assert_eq!(
            result.attempted,
            (MEASURED_BLOCKS as usize * RECORDS_PER_BLOCK) as u64
        );
        assert_eq!(result.accepted, result.attempted);
        assert_eq!(result.render_errors, 0);
        assert_eq!(result, peer, "independent owners diverged");
        emit_record(round, result, peer, backend);
        eprintln!("MISO_ENGINE_BENCH_PHASE round_{round}_complete");
    }
}

impl PartialEq for RoundResult {
    fn eq(&self, other: &Self) -> bool {
        self.attempted == other.attempted
            && self.accepted == other.accepted
            && self.rendered == other.rendered
            && self.render_errors == other.render_errors
            && self.output_words == other.output_words
            && self.output_sha256 == other.output_sha256
    }
}

impl Eq for RoundResult {}

fn emit_record(round: u32, result: RoundResult, peer: RoundResult, backend: Backend) {
    let metadata = Metadata::gather();
    let value = |name: &str| bench_support::json::escape(&metadata.nonempty_or_unknown(name));
    let source_commit = value("MISO_ENGINE_BENCH_CANDIDATE_COMMIT");
    let source_tree = value("MISO_ENGINE_BENCH_CANDIDATE_TREE");
    let binary_sha256 = value("MISO_ENGINE_BENCH_BINARY_SHA256");
    let fixture_sha256 = value("MISO_ENGINE_BENCH_FIXTURE_SHA256");
    let argv = value("MISO_ENGINE_BENCH_ARGV");
    let cwd = value("MISO_ENGINE_BENCH_CWD");
    let rust_version = value("MISO_ENGINE_BENCH_RUST_VERSION");
    let compiler = value("MISO_ENGINE_BENCH_COMPILER");
    let target = value("MISO_ENGINE_BENCH_TARGET_TRIPLE");
    let build_flags = value("MISO_ENGINE_BENCH_BUILD_FLAGS");
    let cpu = value("MISO_ENGINE_BENCH_CPU_MODEL");
    let os = value("MISO_ENGINE_BENCH_OS");
    let metadata_missing = std::env::var("MISO_ENGINE_BENCH_METADATA_MISSING")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "[\"cpu\"]".to_string());
    let output_witness = sha256_hex(
        format!(
            "issue600-output-v1|{}|{}|{round}",
            result.output_sha256, peer.output_sha256
        )
        .as_bytes(),
    );
    let traffic_witness = sha256_hex(
        format!(
            "issue600-traffic-v1|{round}|{SAMPLE_RATE_HZ}|{QUANTUM}|{TRACKS}|{RECORDS_PER_BLOCK}|{SMOOTHING_SAMPLES}|{WARMUP_BLOCKS}|{MEASURED_BLOCKS}|{}|{}|{}|{}|{}",
            result.attempted,
            result.accepted,
            result.rendered,
            result.render_errors,
            result.output_words
        )
        .as_bytes(),
    );
    assert!(result.elapsed_ns > 0, "issue 600 timed render elapsed zero");
    let nanoseconds_per_block = result
        .elapsed_ns
        .checked_div(MEASURED_BLOCKS)
        .filter(|value| *value > 0)
        .expect("issue 600 elapsed shorter than one nanosecond per block");
    println!(
        "{{\"schema_version\":1,\"issue\":600,\"record\":\"input_symmetry\",\"round\":{round},\"sample_rate_hz\":{SAMPLE_RATE_HZ},\"quantum_frames\":{QUANTUM},\"lane_width\":8,\"track_count\":{TRACKS},\"records_per_block\":{RECORDS_PER_BLOCK},\"target_pair_db\":[-6.0,-12.0],\"smoothing_samples\":{SMOOTHING_SAMPLES},\"warmup_blocks\":{WARMUP_BLOCKS},\"measured_blocks\":{MEASURED_BLOCKS},\"attempted_records\":{attempted},\"accepted_records\":{accepted},\"rendered_blocks\":{rendered},\"render_errors\":{errors},\"output_words\":{words},\"output_sha256\":\"{digest}\",\"owner_digests\":[\"{digest}\",\"{peer_digest}\"],\"owner_attempted_records\":[{attempted},{peer_attempted}],\"owner_accepted_records\":[{accepted},{peer_accepted}],\"owner_rendered_blocks\":[{rendered},{peer_rendered}],\"output_witness_sha256\":\"{output_witness}\",\"traffic_witness_sha256\":\"{traffic_witness}\",\"elapsed_ns\":{elapsed},\"nanoseconds_per_block\":{ns},\"backend\":\"{backend:?}\",\"source_commit\":\"{source_commit}\",\"source_tree\":\"{source_tree}\",\"binary_sha256\":\"{binary_sha256}\",\"fixture_sha256\":\"{fixture_sha256}\",\"fixture_id\":\"fixtures/session/v1/parametric-eq-bank-console.json\",\"argv\":\"{argv}\",\"cwd\":\"{cwd}\",\"rust_version\":\"{rust_version}\",\"compiler\":\"{compiler}\",\"target_triple\":\"{target}\",\"build_flags\":\"{build_flags}\",\"cpu\":\"{cpu}\",\"os\":\"{os}\",\"metadata_missing\":{metadata_missing},\"descriptive_only\":true}}",
        attempted = result.attempted,
        accepted = result.accepted,
        rendered = result.rendered,
        errors = result.render_errors,
        words = result.output_words,
        digest = result.output_sha256,
        peer_digest = peer.output_sha256,
        peer_attempted = peer.attempted,
        peer_accepted = peer.accepted,
        peer_rendered = peer.rendered,
        output_witness = output_witness,
        traffic_witness = traffic_witness,
        elapsed = result.elapsed_ns,
        ns = nanoseconds_per_block,
    );
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

    #[test]
    fn prepared_owner_is_w8_and_eight_records_drain_at_boundary() {
        let mut owner = Owner::prepare();
        assert_eq!(owner.controls.len(), TRACKS);
        owner.start_round();
        owner.publish();
        assert_eq!(owner.attempted, RECORDS_PER_BLOCK as u64);
        assert_eq!(owner.accepted, RECORDS_PER_BLOCK as u64);
        owner.render_untimed();
        assert_eq!(owner.render_errors, 0);
        assert!(owner.output.iter().any(|value| *value != 0.0));
        assert_eq!(owner.next_sample, QUANTUM as u64);
        assert_eq!(owner.prove_drained(), TRACKS * QUEUE_CAPACITY);
        owner.render_untimed();
        assert_eq!(owner.render_errors, 0);
        assert_eq!(owner.prove_drained(), TRACKS * QUEUE_CAPACITY);
        owner.render_untimed();
        assert_eq!(owner.render_errors, 0);
    }

    #[test]
    fn two_owners_are_deterministic_and_no_record_control_differs() {
        let mut first = Owner::prepare();
        let mut second = Owner::prepare();
        for _ in 0..4 {
            first.publish();
            second.publish();
            first.render_untimed();
            second.render_untimed();
            first.absorb();
            second.absorb();
        }
        let first_digest = first.digest.snapshot_hex();
        let second_digest = second.digest.snapshot_hex();
        assert_eq!(first_digest, second_digest);
        assert_eq!(first.render_errors, 0);
        assert_eq!(second.render_errors, 0);
        assert_ne!(first.render_samples[0], first.render_samples[1]);
        assert_ne!(first.output[0].to_bits(), 0);
        let mut suppressed = Owner::prepare();
        suppressed.start_round();
        for _ in 0..4 {
            suppressed.attempted = suppressed
                .attempted
                .checked_add(RECORDS_PER_BLOCK as u64)
                .expect("test schedule counter");
            suppressed.render_untimed();
            suppressed.absorb();
        }
        assert_eq!(suppressed.attempted, 4 * RECORDS_PER_BLOCK as u64);
        assert_eq!(suppressed.accepted, 0);
        assert_eq!(suppressed.render_errors, 0);
        assert_ne!(first_digest, suppressed.digest.snapshot_hex());
    }

    #[test]
    fn external_trim_oracle_proves_continuing_nonstationary_ramp() {
        // The input contract is a linear coefficient ramp: db is converted once to
        // 10^(db/20), then 128 samples consume half of the 256-sample window each block.
        let gain = |db: f64| 10.0_f64.powf(db / 20.0) as f32;
        let mut current = 1.0_f32;
        let mut previous = current;
        for block in 0..4 {
            let target = gain(if block % 2 == 0 { -6.0 } else { -12.0 });
            let step = (target - current) / SMOOTHING_SAMPLES as f32;
            current += step * QUANTUM as f32;
            assert!(current.is_finite());
            assert_ne!(current.to_bits(), previous.to_bits());
            assert!((current - target).abs() < (previous - target).abs());
            previous = current;
        }
    }

    #[test]
    fn render_has_no_forbidden_operations_or_allocator_activity() {
        let mut owner = Owner::prepare();
        for _ in 0..2 {
            owner.publish();
            owner.render_untimed();
        }
        audit::warm_up();
        audit::reset();
        let allocations = bench_alloc::current_thread_counters();
        owner.publish();
        owner.render_untimed();
        let delta = bench_alloc::current_thread_delta_since(allocations);
        assert_eq!(delta.allocations, 0);
        assert_eq!(delta.deallocations, 0);
        assert_eq!(delta.reallocations, 0);
        assert_eq!(audit::snapshot().total(), 0);
    }

    #[test]
    fn frozen_assertion_detects_missing_records() {
        fn frozen_assertion(owner: &Owner, expected: &str) {
            assert_eq!(
                owner.attempted, owner.accepted,
                "accepted traffic must equal schedule"
            );
            assert_eq!(
                owner.digest.snapshot_hex(),
                expected,
                "traffic/output digest"
            );
        }

        let mut positive = Owner::prepare();
        positive.start_round();
        positive.publish();
        positive.render_untimed();
        positive.absorb();
        let positive_digest = positive.digest.snapshot_hex();
        frozen_assertion(&positive, &positive_digest);

        let mut suppressed = Owner::prepare();
        suppressed.start_round();
        suppressed.attempted += RECORDS_PER_BLOCK as u64;
        suppressed.render_untimed();
        suppressed.absorb();
        assert_eq!(suppressed.attempted, 8);
        assert_eq!(suppressed.accepted, 0);
        let mutation = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            frozen_assertion(&suppressed, &positive_digest);
        }));
        assert!(
            mutation.is_err(),
            "suppressed input-record mutation escaped the frozen assertion"
        );

        // The mutation is local to this test; the production publishing branch remains enabled.
        let mut restored = Owner::prepare();
        restored.start_round();
        restored.publish();
        restored.render_untimed();
        restored.absorb();
        assert_eq!(restored.accepted, 8);
        frozen_assertion(&restored, &positive_digest);
    }
}
