//! Native resolver, preparation, and one-file-per-worker source delivery.
//!
//! This module is control/worker-only and cfg-excluded from browser Wasm. The started worker owns
//! the opened reader, decoder, prepared source producer, and all decoded staging storage; it never
//! shares those mutable objects with a render worker.

use core::{cell::Cell, marker::PhantomData, num::NonZeroUsize};
use std::{
    io::{Read, Seek},
    sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
    thread::{self, JoinHandle},
};

use miso_engine_core::SampleRateHz;

use crate::{
    HostChunkError, HostChunkProvider, NativeWaveDecoder, NativeWaveError, NativeWaveMetadata,
    NativeWaveParseCaps, NativeWaveRegion, PcmSourceConsumer, PcmSourceRing, PcmSourceRingConfig,
    PcmSourceRingError, SourceCommand, SourceDiagnosticCode, SourceFrame, SourceGeneration,
    SourceResourceReport, SourceSeekError, parse_native_wave,
};

/// Opaque native resolver failure before a source asset can be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceResolverError {
    /// The opaque locator could not be resolved to an opened native asset.
    Unresolved,
}

/// An opened native seekable asset and its resolver-observed opaque identity.
pub struct NativeResolvedAsset<R: Read + Seek + Send + 'static> {
    /// Identity bytes compared exactly with the declaration; their encoding remains opaque.
    pub observed_identity: Vec<u8>,
    /// Opened reader moved into the sole worker after preparation succeeds.
    pub reader: R,
}

/// Opaque native source resolver.
pub trait NativeSourceResolver {
    /// Opened asset type for one source worker.
    type Asset: Read + Seek + Send + 'static;

    /// Open `opaque_locator` without interpreting the locator or identity formats.
    fn resolve(
        &mut self,
        opaque_locator: &str,
    ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError>;
}

/// One fully declared native source preparation input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeSourcePrepareRequest {
    /// Opaque locator delivered unchanged to the resolver.
    pub locator: String,
    /// Opaque declared content identity compared byte-for-byte after resolution.
    pub declared_identity: Vec<u8>,
    /// Declared source sample rate; implicit conversion is forbidden.
    pub declared_sample_rate_hz: SampleRateHz,
    /// Render plan sample rate, which must exactly equal the source rate.
    pub engine_sample_rate_hz: SampleRateHz,
    /// Declared source channel count.
    pub declared_channel_count: u16,
    /// Exact finite decoded source region.
    pub region: NativeWaveRegion,
    /// Exact prepared ring storage shape.
    pub ring_config: PcmSourceRingConfig,
}

/// Fixed native source preparation caps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSourcePrepareCaps {
    /// Parser traversal and skipped-metadata limits.
    pub parser: NativeWaveParseCaps,
    /// Maximum fixed native decoder interleaved read scratch bytes.
    pub max_worker_read_scratch_bytes: u64,
    /// Maximum exact ring plus fixed native worker allocation total.
    pub max_total_engine_owned_bytes: u64,
    /// Maximum exact source-ring, decoder, or staging allocation.
    pub max_largest_allocation_bytes: u64,
    /// Bounded worker command queue item count.
    pub control_queue_items: NonZeroUsize,
}

/// Exact source preparation accounting, independent of asset duration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSourceResourceReport {
    /// Exact prepared source-ring accounting, including the session-charged PCM payload.
    pub ring: SourceResourceReport,
    /// One native decoder interleaved fixed read scratch allocation.
    pub decoder_read_scratch_bytes: u64,
    /// One worker-owned planar `[channel][quantum]` staging allocation.
    pub worker_planar_staging_bytes: u64,
    /// Bounded worker command count; `std` channel implementation bytes are not claimed exact.
    pub worker_control_queue_items: u64,
    /// Exact source ring plus fixed decoder/staging allocation total.
    pub total_engine_owned_bytes: u64,
    /// Largest exact allocation among ring, decoder scratch, and planar staging.
    pub largest_allocation_bytes: u64,
}

/// Resolution or source preparation rejection before any consumer is published.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourcePrepareError {
    /// Resolver could not open an opaque asset.
    Resolver(NativeSourceResolverError),
    /// Observed and declared opaque identities differ.
    ContentIdentityMismatch,
    /// Declared, observed, or engine sample rates differ.
    RateMismatch,
    /// Declared, observed, or ring channel counts differ.
    ChannelMismatch,
    /// Declared region is not within parsed source frames.
    RegionOutOfBounds,
    /// Prepared ring setup rejected its fixed shape.
    Ring(PcmSourceRingError),
    /// Native WAVE parsing/decoder preparation rejected the asset.
    Wave(NativeWaveError),
    /// One explicit worker resource cap rejected preparation.
    ResourceLimit,
    /// The off-render native worker could not be started.
    WorkerStart,
}

impl NativeSourcePrepareError {
    /// Stable source diagnostic code.
    #[must_use]
    pub const fn diagnostic_code(self) -> SourceDiagnosticCode {
        match self {
            Self::Resolver(_) => SourceDiagnosticCode::AssetUnresolved,
            Self::ContentIdentityMismatch => SourceDiagnosticCode::ContentIdentityMismatch,
            Self::RateMismatch => SourceDiagnosticCode::RateMismatch,
            Self::ChannelMismatch => SourceDiagnosticCode::ChannelsMismatch,
            Self::RegionOutOfBounds => SourceDiagnosticCode::RegionOutOfBounds,
            Self::Ring(error) => error.diagnostic_code(),
            Self::Wave(error) => error.diagnostic_code(),
            Self::ResourceLimit | Self::WorkerStart => SourceDiagnosticCode::ResourceLimit,
        }
    }
}

/// Worker lifecycle event delivered only through a bounded non-render channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerEvent {
    /// First PCM chunk or zero-frame end marker has been submitted to the prepared ring.
    SourceReady,
}

/// Worker terminal state returned after an off-render join.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerExit {
    /// Explicit stop or controller disconnect stopped the worker.
    Stopped,
    /// File/decoder work failed outside render.
    DecodeFailed(NativeWaveError),
    /// The native path violated shared host/ring submission semantics.
    SubmitFailed(HostChunkError),
    /// A queued source seek could not be delivered through the bounded source command state.
    SeekFailed(SourceSeekError),
}

/// Typed non-render worker command failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSourceWorkerControlError {
    /// Worker has already stopped or its command receiver disconnected.
    Stopped,
    /// A seek generation is zero.
    GenerationZero,
    /// A seek generation does not strictly increase from the last accepted command.
    GenerationNotStrictlyIncreasing {
        /// Last accepted generation.
        active: SourceGeneration,
        /// Rejected generation.
        requested: SourceGeneration,
    },
    /// A seek frame lies outside the prepared finite region.
    RegionOutOfBounds,
    /// Bounded worker command storage is full; the command was not accepted.
    Backpressure,
    /// Worker thread panicked; no render-thread recovery is attempted.
    WorkerPanicked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkerCommand {
    Seek {
        generation: SourceGeneration,
        frame: SourceFrame,
    },
    Wake,
    Stop,
}

#[derive(Clone, Copy)]
struct PendingBlock {
    generation: SourceGeneration,
    start_frame: SourceFrame,
    frames: u32,
    end_of_region: bool,
}

/// A started native worker owning one reader, decoder, source producer, and fixed staging block.
pub struct NativeSourceWorker {
    commands: SyncSender<WorkerCommand>,
    events: Receiver<NativeSourceWorkerEvent>,
    join: Option<JoinHandle<NativeSourceWorkerExit>>,
    next_requested_generation: SourceGeneration,
    region: NativeWaveRegion,
    stopped: bool,
    _not_sync: PhantomData<Cell<()>>,
}

impl NativeSourceWorker {
    /// Queue a strictly increasing region-bounded source seek without blocking.
    pub fn try_seek(
        &mut self,
        command: SourceCommand,
    ) -> Result<(), NativeSourceWorkerControlError> {
        if self.stopped {
            return Err(NativeSourceWorkerControlError::Stopped);
        }
        let SourceCommand::Seek { generation, frame } = command;
        if !generation.is_valid() {
            return Err(NativeSourceWorkerControlError::GenerationZero);
        }
        if generation <= self.next_requested_generation {
            return Err(
                NativeSourceWorkerControlError::GenerationNotStrictlyIncreasing {
                    active: self.next_requested_generation,
                    requested: generation,
                },
            );
        }
        validate_seek_frame(self.region, frame)?;
        self.try_send(WorkerCommand::Seek { generation, frame })?;
        self.next_requested_generation = generation;
        Ok(())
    }

    /// Wake a worker waiting for ring capacity after an off-render consumer-drain notification.
    pub fn try_wake(&mut self) -> Result<(), NativeSourceWorkerControlError> {
        if self.stopped {
            return Err(NativeSourceWorkerControlError::Stopped);
        }
        self.try_send(WorkerCommand::Wake)
    }

    /// Wait outside render for the initial prepared source data event.
    pub fn wait_for_event(
        &self,
    ) -> Result<NativeSourceWorkerEvent, NativeSourceWorkerControlError> {
        self.events
            .recv()
            .map_err(|_| NativeSourceWorkerControlError::Stopped)
    }

    /// Stop and join the worker outside render.
    pub fn stop_and_join(
        &mut self,
    ) -> Result<NativeSourceWorkerExit, NativeSourceWorkerControlError> {
        if !self.stopped {
            self.try_send(WorkerCommand::Stop)?;
            self.stopped = true;
        }
        let Some(join) = self.join.take() else {
            return Err(NativeSourceWorkerControlError::Stopped);
        };
        join.join()
            .map_err(|_| NativeSourceWorkerControlError::WorkerPanicked)
    }

    fn try_send(&self, command: WorkerCommand) -> Result<(), NativeSourceWorkerControlError> {
        match self.commands.try_send(command) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(_)) => Err(NativeSourceWorkerControlError::Backpressure),
            Err(TrySendError::Disconnected(_)) => Err(NativeSourceWorkerControlError::Stopped),
        }
    }
}

/// Resolve, validate, prepare, and start one native source worker transactionally.
pub fn prepare_native_source<S: NativeSourceResolver>(
    resolver: &mut S,
    request: NativeSourcePrepareRequest,
    caps: NativeSourcePrepareCaps,
) -> Result<
    (
        NativeSourceWorker,
        PcmSourceConsumer,
        NativeSourceResourceReport,
    ),
    NativeSourcePrepareError,
> {
    let mut asset = resolver
        .resolve(&request.locator)
        .map_err(NativeSourcePrepareError::Resolver)?;
    if asset.observed_identity != request.declared_identity {
        return Err(NativeSourcePrepareError::ContentIdentityMismatch);
    }
    let metadata = parse_native_wave(&mut asset.reader, caps.parser)
        .map_err(NativeSourcePrepareError::Wave)?;
    if metadata.sample_rate_hz != request.declared_sample_rate_hz
        || metadata.sample_rate_hz != request.engine_sample_rate_hz
    {
        return Err(NativeSourcePrepareError::RateMismatch);
    }
    if metadata.channel_count != request.declared_channel_count
        || request.ring_config.channel_count != u32::from(request.declared_channel_count)
    {
        return Err(NativeSourcePrepareError::ChannelMismatch);
    }
    validate_region(metadata, request.region)?;
    let report = source_resource_report(metadata, request.ring_config, caps)?;
    let quantum = usize::try_from(request.ring_config.quantum_frames.0)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    let quantum = NonZeroUsize::new(quantum).ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let decoder = NativeWaveDecoder::prepare(asset.reader, metadata, request.region, quantum)
        .map_err(NativeSourcePrepareError::Wave)?;
    let staging_samples = usize::from(metadata.channel_count)
        .checked_mul(quantum.get())
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let mut planar_staging = Vec::new();
    planar_staging
        .try_reserve_exact(staging_samples)
        .map_err(|_| NativeSourcePrepareError::ResourceLimit)?;
    planar_staging.resize(staging_samples, 0.0);
    let (producer, consumer, ring_report) =
        PcmSourceRing::prepare_at_source_frame(request.ring_config, request.region.start_frame)
            .map_err(NativeSourcePrepareError::Ring)?;
    debug_assert_eq!(ring_report, report.ring);
    let provider = producer.into_host_chunk_provider(metadata.sample_rate_hz);
    let (command_sender, command_receiver) = mpsc::sync_channel(caps.control_queue_items.get());
    let (event_sender, event_receiver) = mpsc::sync_channel(1);
    let initial_generation = request.ring_config.initial_generation;
    let join = thread::Builder::new()
        .name("miso-engine-source".to_owned())
        .spawn(move || {
            run_worker(
                command_receiver,
                event_sender,
                provider,
                decoder,
                planar_staging.into_boxed_slice(),
                initial_generation,
            )
        })
        .map_err(|_| NativeSourcePrepareError::WorkerStart)?;
    Ok((
        NativeSourceWorker {
            commands: command_sender,
            events: event_receiver,
            join: Some(join),
            next_requested_generation: initial_generation,
            region: request.region,
            stopped: false,
            _not_sync: PhantomData,
        },
        consumer,
        report,
    ))
}

fn source_resource_report(
    metadata: NativeWaveMetadata,
    ring_config: PcmSourceRingConfig,
    caps: NativeSourcePrepareCaps,
) -> Result<NativeSourceResourceReport, NativeSourcePrepareError> {
    let ring =
        PcmSourceRing::resource_report(ring_config).map_err(NativeSourcePrepareError::Ring)?;
    let decoder_read_scratch_bytes = u64::from(ring_config.quantum_frames.0)
        .checked_mul(u64::from(metadata.block_align_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let worker_planar_staging_bytes = u64::from(ring_config.quantum_frames.0)
        .checked_mul(u64::from(metadata.channel_count))
        .and_then(|samples| {
            samples.checked_mul(u64::try_from(core::mem::size_of::<f32>()).expect("f32 size"))
        })
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    if decoder_read_scratch_bytes > caps.max_worker_read_scratch_bytes {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    let total_engine_owned_bytes = ring
        .total_engine_owned_bytes
        .checked_add(decoder_read_scratch_bytes)
        .and_then(|total| total.checked_add(worker_planar_staging_bytes))
        .ok_or(NativeSourcePrepareError::ResourceLimit)?;
    let largest_allocation_bytes = ring
        .largest_allocation_bytes
        .max(decoder_read_scratch_bytes)
        .max(worker_planar_staging_bytes);
    if total_engine_owned_bytes > caps.max_total_engine_owned_bytes
        || largest_allocation_bytes > caps.max_largest_allocation_bytes
    {
        return Err(NativeSourcePrepareError::ResourceLimit);
    }
    Ok(NativeSourceResourceReport {
        ring,
        decoder_read_scratch_bytes,
        worker_planar_staging_bytes,
        worker_control_queue_items: u64::try_from(caps.control_queue_items.get())
            .expect("usize fits u64"),
        total_engine_owned_bytes,
        largest_allocation_bytes,
    })
}

fn validate_region(
    metadata: NativeWaveMetadata,
    region: NativeWaveRegion,
) -> Result<(), NativeSourcePrepareError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeSourcePrepareError::RegionOutOfBounds)?;
    if end > metadata.total_frames {
        return Err(NativeSourcePrepareError::RegionOutOfBounds);
    }
    Ok(())
}

fn validate_seek_frame(
    region: NativeWaveRegion,
    frame: SourceFrame,
) -> Result<(), NativeSourceWorkerControlError> {
    let end = region
        .start_frame
        .0
        .checked_add(region.length_frames)
        .ok_or(NativeSourceWorkerControlError::RegionOutOfBounds)?;
    if frame.0 < region.start_frame.0 || frame.0 > end {
        return Err(NativeSourceWorkerControlError::RegionOutOfBounds);
    }
    Ok(())
}

fn run_worker<R: Read + Seek>(
    commands: Receiver<WorkerCommand>,
    events: SyncSender<NativeSourceWorkerEvent>,
    mut provider: HostChunkProvider,
    mut decoder: NativeWaveDecoder<R>,
    mut planar_staging: Box<[f32]>,
    mut generation: SourceGeneration,
) -> NativeSourceWorkerExit {
    let mut pending: Option<PendingBlock> = None;
    let mut end_submitted = false;
    let mut source_ready_sent = false;
    loop {
        match commands.try_recv() {
            Ok(command) => {
                match apply_command(command, &mut provider, &mut decoder, &mut generation) {
                    Ok(CommandResult::Seek) => {
                        pending = None;
                        end_submitted = false;
                        continue;
                    }
                    Ok(CommandResult::Wake) => {}
                    Ok(CommandResult::Stop) => return NativeSourceWorkerExit::Stopped,
                    Err(exit) => return exit,
                }
            }
            Err(TryRecvError::Disconnected) => return NativeSourceWorkerExit::Stopped,
            Err(TryRecvError::Empty) => {}
        }
        if let Some(block) = pending.take() {
            match provider.submit_native_planar(
                block.generation,
                block.start_frame,
                &planar_staging,
                block.frames,
                block.end_of_region,
            ) {
                Ok(_) => {
                    end_submitted = block.end_of_region;
                    if !source_ready_sent {
                        let _ = events.try_send(NativeSourceWorkerEvent::SourceReady);
                        source_ready_sent = true;
                    }
                    continue;
                }
                Err(HostChunkError::Full { .. }) => {
                    match commands.recv() {
                        Ok(command) => match apply_command(
                            command,
                            &mut provider,
                            &mut decoder,
                            &mut generation,
                        ) {
                            Ok(CommandResult::Seek) => {
                                pending = None;
                                end_submitted = false;
                            }
                            Ok(CommandResult::Wake) => pending = Some(block),
                            Ok(CommandResult::Stop) => return NativeSourceWorkerExit::Stopped,
                            Err(exit) => return exit,
                        },
                        Err(_) => return NativeSourceWorkerExit::Stopped,
                    }
                    continue;
                }
                Err(error) => return NativeSourceWorkerExit::SubmitFailed(error),
            }
        }
        if end_submitted {
            match commands.recv() {
                Ok(command) => {
                    match apply_command(command, &mut provider, &mut decoder, &mut generation) {
                        Ok(CommandResult::Seek) => end_submitted = false,
                        Ok(CommandResult::Wake) => {}
                        Ok(CommandResult::Stop) => return NativeSourceWorkerExit::Stopped,
                        Err(exit) => return exit,
                    }
                }
                Err(_) => return NativeSourceWorkerExit::Stopped,
            }
            continue;
        }
        let start_frame = decoder.next_source_frame();
        let decoded = match decoder.decode_quantum_into_planar(&mut planar_staging) {
            Ok(report) => report,
            Err(error) => return NativeSourceWorkerExit::DecodeFailed(error),
        };
        pending = Some(PendingBlock {
            generation,
            start_frame,
            frames: decoded.decoded_frames,
            end_of_region: decoded.end_of_region,
        });
    }
}

enum CommandResult {
    Wake,
    Seek,
    Stop,
}

fn apply_command<R: Read + Seek>(
    command: WorkerCommand,
    provider: &mut HostChunkProvider,
    decoder: &mut NativeWaveDecoder<R>,
    generation: &mut SourceGeneration,
) -> Result<CommandResult, NativeSourceWorkerExit> {
    match command {
        WorkerCommand::Wake => Ok(CommandResult::Wake),
        WorkerCommand::Stop => Ok(CommandResult::Stop),
        WorkerCommand::Seek {
            generation: requested,
            frame,
        } => {
            decoder
                .seek_to_source_frame(frame)
                .map_err(NativeSourceWorkerExit::DecodeFailed)?;
            provider
                .try_seek(SourceCommand::Seek {
                    generation: requested,
                    frame,
                })
                .map_err(NativeSourceWorkerExit::SeekFailed)?;
            *generation = requested;
            Ok(CommandResult::Seek)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use crate::{HostPlanarChunk, PcmSourceRing, QuantumFrames, SourceReadReport};

    struct Resolver {
        asset: Option<NativeResolvedAsset<Cursor<Vec<u8>>>>,
    }

    impl NativeSourceResolver for Resolver {
        type Asset = Cursor<Vec<u8>>;

        fn resolve(
            &mut self,
            opaque_locator: &str,
        ) -> Result<NativeResolvedAsset<Self::Asset>, NativeSourceResolverError> {
            if opaque_locator != "opaque:stem" {
                return Err(NativeSourceResolverError::Unresolved);
            }
            self.asset
                .take()
                .ok_or(NativeSourceResolverError::Unresolved)
        }
    }

    fn resolver(samples: &[f32], identity: &[u8]) -> Resolver {
        Resolver {
            asset: Some(NativeResolvedAsset {
                observed_identity: identity.to_vec(),
                reader: Cursor::new(float32_wave(samples)),
            }),
        }
    }

    fn request(region: NativeWaveRegion) -> NativeSourcePrepareRequest {
        NativeSourcePrepareRequest {
            locator: "opaque:stem".to_owned(),
            declared_identity: b"exact-identity".to_vec(),
            declared_sample_rate_hz: SampleRateHz(48_000),
            engine_sample_rate_hz: SampleRateHz(48_000),
            declared_channel_count: 1,
            region,
            ring_config: PcmSourceRingConfig {
                channel_count: 1,
                quantum_frames: QuantumFrames(4),
                frame_capacity: 8,
                initial_generation: SourceGeneration(1),
            },
        }
    }

    fn caps() -> NativeSourcePrepareCaps {
        NativeSourcePrepareCaps {
            parser: NativeWaveParseCaps {
                max_chunk_count: 8,
                max_skipped_metadata_bytes: 32,
            },
            max_worker_read_scratch_bytes: 64,
            max_total_engine_owned_bytes: u64::MAX,
            max_largest_allocation_bytes: u64::MAX,
            control_queue_items: NonZeroUsize::new(2).expect("two"),
        }
    }

    #[test]
    fn resolver_preparation_validates_identity_rate_channels_region_and_fixed_caps() {
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut wrong_identity = resolver(&[0.0; 4], b"other");
        let identity_error =
            match prepare_native_source(&mut wrong_identity, request(region), caps()) {
                Ok(_) => panic!("identity should reject"),
                Err(error) => error,
            };
        assert_eq!(
            identity_error,
            NativeSourcePrepareError::ContentIdentityMismatch
        );
        assert_eq!(
            identity_error.diagnostic_code(),
            SourceDiagnosticCode::ContentIdentityMismatch
        );

        let mut wrong_rate = resolver(&[0.0; 4], b"exact-identity");
        let mut rate_request = request(region);
        rate_request.engine_sample_rate_hz = SampleRateHz(44_100);
        let rate_error = match prepare_native_source(&mut wrong_rate, rate_request, caps()) {
            Ok(_) => panic!("rate should reject"),
            Err(error) => error,
        };
        assert_eq!(rate_error, NativeSourcePrepareError::RateMismatch);

        let mut wrong_channels = resolver(&[0.0; 4], b"exact-identity");
        let mut channel_request = request(region);
        channel_request.declared_channel_count = 2;
        let channel_error =
            match prepare_native_source(&mut wrong_channels, channel_request, caps()) {
                Ok(_) => panic!("channels should reject"),
                Err(error) => error,
            };
        assert_eq!(channel_error, NativeSourcePrepareError::ChannelMismatch);

        let mut wrong_region = resolver(&[0.0; 4], b"exact-identity");
        let region_error = match prepare_native_source(
            &mut wrong_region,
            request(NativeWaveRegion {
                start_frame: SourceFrame(3),
                length_frames: 2,
            }),
            caps(),
        ) {
            Ok(_) => panic!("region should reject"),
            Err(error) => error,
        };
        assert_eq!(region_error, NativeSourcePrepareError::RegionOutOfBounds);

        let mut capped = resolver(&[0.0; 4], b"exact-identity");
        let mut too_small = caps();
        too_small.max_worker_read_scratch_bytes = 15;
        let cap_error = match prepare_native_source(&mut capped, request(region), too_small) {
            Ok(_) => panic!("read scratch cap should reject"),
            Err(error) => error,
        };
        assert_eq!(cap_error, NativeSourcePrepareError::ResourceLimit);
    }

    #[test]
    fn native_worker_and_host_provider_produce_identical_prepared_ring_pcm() {
        let samples = [0.25, -0.5, 0.75, -1.0];
        let region = NativeWaveRegion {
            start_frame: SourceFrame(0),
            length_frames: 4,
        };
        let mut native_resolver = resolver(&samples, b"exact-identity");
        let (mut worker, mut native_consumer, report) =
            prepare_native_source(&mut native_resolver, request(region), caps()).expect("prepare");
        assert_eq!(report.decoder_read_scratch_bytes, 16);
        assert_eq!(report.worker_planar_staging_bytes, 16);
        assert_eq!(
            worker.wait_for_event().expect("ready"),
            NativeSourceWorkerEvent::SourceReady
        );
        let native = read_one(&mut native_consumer);

        let (producer, mut host_consumer, _) = PcmSourceRing::prepare(PcmSourceRingConfig {
            channel_count: 1,
            quantum_frames: QuantumFrames(4),
            frame_capacity: 8,
            initial_generation: SourceGeneration(1),
        })
        .expect("ring");
        let mut host = producer.into_host_chunk_provider(SampleRateHz(48_000));
        host.submit(HostPlanarChunk {
            sample_rate_hz: SampleRateHz(48_000),
            generation: SourceGeneration(1),
            start_frame: SourceFrame(0),
            planes: &[&samples],
            frames: 4,
            end_of_region: true,
        })
        .expect("host submit");
        let host_pcm = read_one(&mut host_consumer);
        assert_eq!(native, host_pcm);
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
    }

    #[test]
    fn worker_seek_stop_wake_and_join_are_bounded_and_typed() {
        let mut native_resolver = resolver(&[0.0; 8], b"exact-identity");
        let mut lifecycle_caps = caps();
        lifecycle_caps.control_queue_items = NonZeroUsize::new(3).expect("three");
        let (mut worker, _consumer, _) = prepare_native_source(
            &mut native_resolver,
            request(NativeWaveRegion {
                start_frame: SourceFrame(0),
                length_frames: 8,
            }),
            lifecycle_caps,
        )
        .expect("prepare");
        worker.wait_for_event().expect("ready");
        worker
            .try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            })
            .expect("seek");
        assert!(matches!(
            worker.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(4),
            }),
            Err(NativeSourceWorkerControlError::GenerationNotStrictlyIncreasing { .. })
        ));
        assert!(matches!(
            worker.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(3),
                frame: SourceFrame(9),
            }),
            Err(NativeSourceWorkerControlError::RegionOutOfBounds)
        ));
        let _ = worker.try_wake();
        assert_eq!(
            worker.stop_and_join().expect("stop"),
            NativeSourceWorkerExit::Stopped
        );
    }

    fn read_one(consumer: &mut PcmSourceConsumer) -> ([f32; 4], SourceReadReport) {
        let mut output = [0.0; 4];
        let report = {
            let mut planes = [&mut output[..]];
            consumer.read_block(&mut planes).expect("read")
        };
        (output, report)
    }

    fn float32_wave(samples: &[f32]) -> Vec<u8> {
        let mut format = Vec::new();
        format.extend_from_slice(&3_u16.to_le_bytes());
        format.extend_from_slice(&1_u16.to_le_bytes());
        format.extend_from_slice(&48_000_u32.to_le_bytes());
        format.extend_from_slice(&192_000_u32.to_le_bytes());
        format.extend_from_slice(&4_u16.to_le_bytes());
        format.extend_from_slice(&32_u16.to_le_bytes());
        let mut data = Vec::new();
        for sample in samples {
            data.extend_from_slice(&sample.to_le_bytes());
        }
        let mut wave = Vec::new();
        wave.extend_from_slice(b"RIFF");
        wave.extend_from_slice(&0_u32.to_le_bytes());
        wave.extend_from_slice(b"WAVE");
        append_chunk(&mut wave, b"fmt ", &format);
        append_chunk(&mut wave, b"data", &data);
        let riff_size = u32::try_from(wave.len() - 8).expect("len");
        wave[4..8].copy_from_slice(&riff_size.to_le_bytes());
        wave
    }

    fn append_chunk(out: &mut Vec<u8>, id: &[u8; 4], payload: &[u8]) {
        out.extend_from_slice(id);
        out.extend_from_slice(&u32::try_from(payload.len()).expect("len").to_le_bytes());
        out.extend_from_slice(payload);
        if payload.len() % 2 == 1 {
            out.push(0);
        }
    }
}
