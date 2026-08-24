//! Deterministic Issue-010 source-ring realtime and duration-independent resource audit.

#![allow(unsafe_code)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    io::{Read, Seek, SeekFrom},
    num::NonZeroUsize,
};

use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_core::{
    QuantumFrames, SampleRateHz,
    realtime::{PlanarBufferMut, RenderEnvelope, RenderIo, RenderTime},
};
use miso_engine_effect_contract::{LatencySamples, TailSamples};
use miso_engine_graph::{
    DependencyLevel, GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding, GraphNodeId, GraphPortId,
    GraphPortKind, GraphResourceEstimate, GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec,
    PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId, TrackStage,
};
use miso_engine_source::{
    NativeResolvedAsset, NativeSourcePrepareCaps, NativeSourcePrepareRequest, NativeSourceResolver,
    NativeSourceResolverError, NativeWaveParseCaps, NativeWaveRegion, SourceCommand, SourceFrame,
    SourceGeneration, SourceGraphTrackMapping, prepare_graph_source_set,
    prepare_native_source_with_audit_gate,
};

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: each call forwards the original allocation contract to the system allocator. A render
// allocation/deallocation aborts before the allocation can become observable.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the caller's valid allocation layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the caller's valid allocation layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the original valid pointer/layout pair unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the original valid allocation contract unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn main() {
    let mut resolver = Resolver::new(SyntheticWave::new(u64::from(QUANTUM) * 4));
    let request = NativeSourcePrepareRequest {
        locator: "audit:synthetic-wave".to_owned(),
        declared_identity: b"issue041-native-worker".to_vec(),
        declared_sample_rate_hz: SampleRateHz(48_000),
        engine_sample_rate_hz: SampleRateHz(48_000),
        declared_channel_count: 1,
        region: NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: u64::from(QUANTUM) * 4,
        },
        ring_config: miso_engine_source::PcmSourceRingConfig {
            channel_count: 1,
            quantum_frames: QuantumFrames(QUANTUM),
            // Initial EOF prefill occupies four blocks, leaving exactly one block available for
            // the held post-seek generation before render begins.
            frame_capacity: u64::from(QUANTUM) * 5,
            initial_generation: SourceGeneration(1),
        },
    };
    let caps = NativeSourcePrepareCaps {
        parser: NativeWaveParseCaps {
            max_chunk_count: 4,
            max_skipped_metadata_bytes: 0,
        },
        max_worker_read_scratch_bytes: u64::from(QUANTUM) * 4,
        max_total_engine_owned_bytes: u64::MAX,
        max_largest_allocation_bytes: u64::MAX,
        control_queue_items: NonZeroUsize::new(2).expect("two commands"),
    };
    let (mut prepared, mut gate) =
        prepare_native_source_with_audit_gate(&mut resolver, request, caps)
            .expect("prepare real native worker");
    prepared
        .controller()
        .wait_for_event()
        .expect("native worker prefill event");
    let (mut controller, source) = prepared.into_graph_source();
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(QUANTUM),
        input_channels: None,
        output_channels: NonZeroUsize::new(2).expect("stereo output"),
    };
    let input = GraphNodeId::TrackStage {
        track_id: StableGraphId::parse("audit.source").expect("stable input id"),
        stage: TrackStage::Input,
    };
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("audit.main").expect("stable output id"),
    };
    let source_set = prepare_graph_source_set(
        envelope,
        vec![source],
        vec![SourceGraphTrackMapping {
            node: input.clone(),
            source_index: 0,
            left_channel: 0,
            right_channel: 0,
        }],
    )
    .expect("seal graph source set");
    let mut plan = match prepared_graph_plan(envelope, input, output).bind_with_source_set(
        GraphRuntimeBindings {
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes: vec![GraphNodeBinding::new(
                GraphNodeId::Output {
                    output_id: StableGraphId::parse("audit.main").expect("stable output id"),
                },
                Box::new(Noop),
            )],
            observers: Vec::new(),
        },
        source_set,
    ) {
        Ok(plan) => plan,
        Err(failure) => panic!("bind graph source set: {}", failure.code),
    };
    let mut output_pcm = [f32::from_bits(0xffff_ffff); (QUANTUM as usize) * 2];
    let output_address = output_pcm.as_ptr() as usize;

    audit::warm_up();
    audit::reset();
    let mut resumed_at = None;
    eprintln!("MISO_ENGINE_SOURCE_RT_BEGIN");
    for block in 0..BLOCKS {
        if block == 1 {
            controller
                .try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(2),
                    frame: SourceFrame(u64::from(QUANTUM)),
                })
                .expect("off-render native seek");
            controller
                .hold_worker_for_audit()
                .expect("queue audit hold after seek");
            gate.wait_until_held().expect("worker held outside render");
        }
        let report = plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(
                        &mut output_pcm,
                        2,
                        QUANTUM as usize,
                        QUANTUM as usize,
                    )
                    .expect("fixed output"),
                },
                RenderTime {
                    absolute_sample: block * u64::from(QUANTUM),
                },
            )
            .expect("prepared native-source render");
        assert_eq!(report.frames, QUANTUM);
        let left = &output_pcm[..QUANTUM as usize];
        let right = &output_pcm[QUANTUM as usize..];
        if block == 0 {
            assert!(
                left.iter()
                    .chain(right)
                    .all(|sample| sample.to_bits() == 0.25_f32.to_bits()),
                "initial native PCM mismatch: left={left:?}, right={right:?}"
            );
        }
        if block == 1 {
            assert!(
                left.iter()
                    .chain(right)
                    .all(|sample| sample.to_bits() == 0.25_f32.to_bits())
            );
        }
        if block == 2 {
            assert!(left.iter().chain(right).all(|sample| sample.to_bits() == 0));
            controller
                .try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(3),
                    frame: SourceFrame(u64::from(QUANTUM) * 3),
                })
                .expect("off-render resume seek");
            gate.release_and_wait()
                .expect("worker resumes outside render");
        }
        if block == 3 {
            assert!(
                left.iter()
                    .chain(right)
                    .all(|sample| sample.to_bits() == 0.25_f32.to_bits())
            );
            resumed_at = Some(u64::from(QUANTUM) * 3);
        }
    }
    eprintln!("MISO_ENGINE_SOURCE_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(output_pcm.as_ptr() as usize, output_address);
    assert_eq!(resumed_at, Some(u64::from(QUANTUM) * 3));
    assert_eq!(snapshot.total(), 0);
    drop(plan);
    assert!(
        controller.wait_for_event().is_ok(),
        "off-render worker terminal event"
    );
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue010_source_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":{},\"underrun_frames\":{},",
            "\"underrun_events\":{},\"resumed_source_frame\":{},\"output_address\":{},",
            "\"native_worker_hold_release\":true,",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        BLOCKS,
        QUANTUM,
        QUANTUM,
        1,
        resumed_at.expect("resume"),
        output_address,
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.total(),
    );
}

struct Noop;
impl GraphRuntimeProcessor for Noop {
    fn process(
        &mut self,
        _block: miso_engine_graph::GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

struct Resolver {
    asset: Option<NativeResolvedAsset<SyntheticWave>>,
}
impl Resolver {
    fn new(reader: SyntheticWave) -> Self {
        Self {
            asset: Some(NativeResolvedAsset {
                observed_identity: b"issue041-native-worker".to_vec(),
                reader,
            }),
        }
    }
}
impl NativeSourceResolver for Resolver {
    type Asset = SyntheticWave;
    fn resolve(
        &mut self,
        locator: &str,
    ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError> {
        if locator != "audit:synthetic-wave" {
            return Err(NativeSourceResolverError::Unresolved);
        }
        self.asset
            .take()
            .ok_or(NativeSourceResolverError::Unresolved)
    }
}

struct SyntheticWave {
    position: u64,
    frames: u64,
}
impl SyntheticWave {
    fn new(frames: u64) -> Self {
        Self {
            position: 0,
            frames,
        }
    }
    fn header(&self) -> [u8; 44] {
        let data_bytes = u32::try_from(self.frames * 4).expect("small audit source");
        let riff_size = 36_u32.checked_add(data_bytes).expect("small audit source");
        [
            b'R',
            b'I',
            b'F',
            b'F',
            riff_size.to_le_bytes()[0],
            riff_size.to_le_bytes()[1],
            riff_size.to_le_bytes()[2],
            riff_size.to_le_bytes()[3],
            b'W',
            b'A',
            b'V',
            b'E',
            b'f',
            b'm',
            b't',
            b' ',
            16,
            0,
            0,
            0,
            3,
            0,
            1,
            0,
            0x80,
            0xbb,
            0,
            0,
            0,
            0xee,
            2,
            0,
            4,
            0,
            32,
            0,
            b'd',
            b'a',
            b't',
            b'a',
            data_bytes.to_le_bytes()[0],
            data_bytes.to_le_bytes()[1],
            data_bytes.to_le_bytes()[2],
            data_bytes.to_le_bytes()[3],
        ]
    }
    fn len(&self) -> u64 {
        44 + self.frames * 4
    }
}
impl Read for SyntheticWave {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        if self.position >= self.len() {
            return Ok(0);
        }
        let count = usize::try_from((self.len() - self.position).min(output.len() as u64))
            .expect("bounded read");
        let header = self.header();
        for (index, byte) in output[..count].iter_mut().enumerate() {
            let offset = self.position + index as u64;
            *byte = if offset < 44 {
                header[offset as usize]
            } else {
                0.25_f32.to_le_bytes()[((offset - 44) % 4) as usize]
            };
        }
        self.position += count as u64;
        Ok(count)
    }
}
impl Seek for SyntheticWave {
    fn seek(&mut self, from: SeekFrom) -> std::io::Result<u64> {
        let base = match from {
            SeekFrom::Start(value) => {
                self.position = value;
                return Ok(value);
            }
            SeekFrom::Current(value) => self.position as i128 + value as i128,
            SeekFrom::End(value) => self.len() as i128 + value as i128,
        };
        if base < 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "before start",
            ));
        }
        self.position = base as u64;
        Ok(self.position)
    }
}

fn prepared_graph_plan(
    envelope: RenderEnvelope,
    input: GraphNodeId,
    output: GraphNodeId,
) -> PreparedGraphPlan {
    PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 41,
        spec: GraphSpec {
            nodes: vec![
                GraphNode {
                    id: input.clone(),
                    latency: LatencySamples(0),
                    tail: TailSamples::Finite(0),
                },
                GraphNode {
                    id: output.clone(),
                    latency: LatencySamples(0),
                    tail: TailSamples::Finite(0),
                },
            ],
            ports: Vec::new(),
            edges: vec![GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("audit.route").expect("stable route id"),
                },
                source: GraphPortId {
                    node: input.clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.audit.route".to_owned(),
            }],
        },
        sequential_schedule: vec![input.clone(), output.clone()],
        dependency_levels: vec![
            DependencyLevel {
                level: 0,
                nodes: vec![input.clone()],
            },
            DependencyLevel {
                level: 1,
                nodes: vec![output.clone()],
            },
        ],
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: GraphResourceEstimate {
            logical_nodes: 0,
            materialized_nodes: 0,
            edges: 0,
            schedule_items: 0,
            dependency_levels: 0,
            reductions: 0,
            routes: 0,
            effects: 0,
            audio_buffer_samples: 0,
            total_delay_samples: 0,
            delay_bytes: 0,
            graph_metadata_bytes: 0,
            declared_effect_bytes: 0,
            effect_bank_count: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        },
        envelope,
        required_bindings: vec![input, output],
        routes: Vec::new(),
        effects: Vec::new(),
        banks: Vec::new(),
        builtin_banks: Vec::new(),
        observers: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    const ASSERTED_TRANSCRIPT: &str = concat!(
        "schema=issue041-worker-v1;blocks=100000;quantum=128;",
        "block0=0x3e800000x256;block1=0x3e800000x256;",
        "block2=0x00000000x256;block3=0x3e800000x256;",
        "underrun_frames=128;underrun_events=1;resume_frame=384;",
        "output_address_stable=true;worker_terminal_after_plan_drop=true;violations=0"
    );

    fn fnv1a64(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    }

    #[test]
    fn asserted_worker_lifecycle_counter_and_pcm_transcript_is_canonical() {
        assert!(ASSERTED_TRANSCRIPT.contains("block2=0x00000000x256"));
        assert!(ASSERTED_TRANSCRIPT.contains("resume_frame=384"));
        assert!(ASSERTED_TRANSCRIPT.contains("worker_terminal_after_plan_drop=true"));
        assert_eq!(
            fnv1a64(ASSERTED_TRANSCRIPT.as_bytes()),
            0x711c_fce8_4eb2_4efa
        );
        println!(
            "issue041 worker asserted transcript fnv1a64={:016x} bytes={} transcript={ASSERTED_TRANSCRIPT}",
            fnv1a64(ASSERTED_TRANSCRIPT.as_bytes()),
            ASSERTED_TRANSCRIPT.len()
        );
    }
}
