//! Deterministic Issue-010 source-ring realtime and duration-independent resource audit.
//!
//! The native worker's ring feeds a plan compiled the way every host compiles one (#963): a
//! one-track session through [`GraphCompiler::compile_with_builtins`] at the build's own
//! [`Backend::current`], its track input claimed by the source set and its builtin strip attached
//! as banks. The strip is configured as the exact identity (no filters, 0 dB, the identity matrix
//! and a unity route), so the source's PCM reaches the output bit for bit and the audit's
//! per-block PCM assertions keep their meaning.

use bench_support::alloc as bench_alloc;
use std::{
    io::{Read, Seek, SeekFrom},
    num::NonZeroUsize,
};

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::EffectPreparedSession;
use engine::realtime::audit;
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{PlanarBufferMut, RenderIo, RenderTime},
};
use graph::{
    GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use session::{ChannelBuiltins, CompileCaps, MatrixOrPan, compile_session, parse_session_json};
use source::{
    NativeResolvedAsset, NativeSourcePrepareCaps, NativeSourcePrepareRequest, NativeSourceResolver,
    NativeSourceResolverError, NativeWaveParseCaps, NativeWaveRegion, SourceCommand, SourceFrame,
    SourceGeneration, SourceGraphTrackMapping, prepare_graph_source_set,
    prepare_native_source_with_audit_gate,
};

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let mut resolver = Resolver::new(SyntheticWave::new(u64::from(QUANTUM) * 4));
    let request = NativeSourcePrepareRequest {
        locator: "audit:synthetic-wave".to_owned(),
        declared_identity: b"issue041-native-worker".to_vec(),
        declared_sample_rate_hz: SampleRateHz(48_000),
        engine_sample_rate_hz: SampleRateHz(48_000),
        declared_channel_count: 1,
        declared_bit_depth: session::SourceBitDepth::Float32,
        region: NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: u64::from(QUANTUM) * 4,
        },
        ring_config: source::PcmSourceRingConfig {
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
    let artifact = prepared_graph_artifact();
    let envelope = artifact.envelope();
    assert_eq!(envelope.sample_rate, SampleRateHz(48_000));
    assert_eq!(envelope.quantum, QuantumFrames(QUANTUM));
    let (input, output): (Vec<_>, Vec<_>) =
        artifact
            .external_binding_nodes()
            .cloned()
            .partition(|node| {
                matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                )
            });
    let ([input], [output]) = (input.as_slice(), output.as_slice()) else {
        panic!("one track input and the session output are external: {input:?} {output:?}");
    };
    assert!(matches!(output, GraphNodeId::Output { .. }));
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
    let mut plan = match artifact.into_bound_with_source_set(
        GraphRuntimeBindings {
            envelope,
            nodes: vec![GraphNodeBinding::new(output.clone(), Box::new(Noop))],
            observers: Vec::new(),
        },
        source_set,
    ) {
        Ok(bound) => bound.plan,
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
        _block: graph::GraphBindingBlock<'_>,
    ) -> Result<(), engine::realtime::RenderError> {
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

/// The one-track session the worker feeds, compiled with its builtins at the host's width.
///
/// The canonical session's track, stripped of its effects and automation, with a mono source
/// read into both lanes and every builtin stage set to its exact identity: no polarity flip, 0 dB
/// trim and fader, both filters off (a `0.0` cutoff is the identity section), no delay, the
/// identity 2x2 matrix (a pan is a position, not a gain) and the canonical unity route. The bank
/// arithmetic then multiplies by one and adds zero, so the output is the source's PCM bit for
/// bit.
fn prepared_graph_artifact() -> PreparedGraphBuiltinsArtifact {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("canonical session");
    model.quantum_frames = QUANTUM;
    model.sources[0].channels = 1;
    let track = &mut model.tracks[0];
    track.left_source_channel = 0;
    track.right_source_channel = 0;
    track.dynamic.effects.clear();
    let identity = ChannelBuiltins {
        polarity_invert: false,
        trim_db: 0.0,
        hpf_hz: 0.0,
        lpf_hz: 0.0,
        delay_samples: 0,
    };
    track.builtins.left = identity.clone();
    track.builtins.right = identity;
    track.fader.left_db = 0.0;
    track.fader.right_db = 0.0;
    track.fader.left_mute = false;
    track.fader.right_mute = false;
    track.matrix_or_pan = MatrixOrPan::Matrix {
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
        smoothing_samples: 0,
    };
    model.automation.clear();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled source audit session");
    let builtins = prepare_session_builtins(
        &session,
        &[],
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
        },
    )
    .expect("sealed source audit builtins");
    let dispatch = Backend::current();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id: 41,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|failure| panic!("source audit graph compile: {:?}", failure.diagnostics));
    // The strip renders in banks at a SIMD width, as on every host; at scalar width no bank
    // attaches.
    let banks = if dispatch.width() > 1 { 3 } else { 0 };
    assert_eq!(artifact.prepared_builtin_bank_count(), banks);
    artifact
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
