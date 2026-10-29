//! Issue #1320 gates 3-6: a primed source consumer replays exactly the blocks it would have
//! played, observes no command, carries a shared `prime_required` flag, and allocates nothing.
//!
//! Every ring has two channels and a quantum of 128 frames, and its samples encode
//! `(frame, channel)`, so a block played at the wrong frame or from the wrong channel differs by
//! value. The lead is `k = 3` blocks unless a gate says otherwise.

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use engine::{QuantumFrames, SampleRateHz};
use source::{
    HostChunkProvider, HostPlanarChunk, PcmSourceConsumer, PcmSourceRing, PcmSourceRingConfig,
    SourceCommand, SourceConsumerTelemetry, SourceFrame, SourceGeneration, SourceReadReport,
    SourceSeekError, SourceSeekReport,
};

const RATE: SampleRateHz = SampleRateHz(48_000);
const Q: u32 = 128;
const QUANTUM: u64 = Q as u64;
const K: u32 = 3;
const RING_BLOCKS: u64 = 24;

fn ring() -> (HostChunkProvider, PcmSourceConsumer) {
    let (producer, consumer, _) = PcmSourceRing::prepare(PcmSourceRingConfig {
        channel_count: 2,
        quantum_frames: QuantumFrames(Q),
        frame_capacity: RING_BLOCKS * QUANTUM,
        initial_generation: SourceGeneration(1),
        seek_runway_blocks: 0,
    })
    .expect("ring");
    (producer.into_host_chunk_provider(RATE), consumer)
}

#[allow(clippy::cast_precision_loss)]
fn sample(generation: u64, frame: u64, channel: u64) -> f32 {
    ((generation << 20) + frame * 2 + channel) as f32
}

fn submit(host: &mut HostChunkProvider, generation: u64, start: u64, frames: u32, end: bool) {
    let planes: [Vec<f32>; 2] = core::array::from_fn(|channel| {
        (0..u64::from(frames))
            .map(|frame| sample(generation, start + frame, channel as u64))
            .collect()
    });
    host.submit(HostPlanarChunk {
        sample_rate_hz: RATE,
        generation: SourceGeneration(generation),
        start_frame: SourceFrame(start),
        planes: &[&planes[0], &planes[1]],
        frames,
        end_of_region: end,
    })
    .expect("valid block");
}

/// What one block made observable: both played planes as bits (or `None`), its read report and
/// the consumer's telemetry after it.
#[derive(Debug, PartialEq)]
struct Played {
    planes: [Option<Vec<u32>>; 2],
    report: SourceReadReport,
    telemetry: SourceConsumerTelemetry,
}

fn observe(consumer: &PcmSourceConsumer, report: SourceReadReport) -> Played {
    Played {
        planes: core::array::from_fn(|channel| {
            consumer
                .played_plane(u32::try_from(channel).expect("channel"))
                .map(|plane| plane.iter().map(|value| value.to_bits()).collect())
        }),
        report,
        telemetry: consumer.telemetry(),
    }
}

fn ordinary(consumer: &mut PcmSourceConsumer, first_sample: u64) -> Played {
    let report = consumer.begin_block_at(first_sample);
    observe(consumer, report)
}

fn primed(consumer: &mut PcmSourceConsumer, first_sample: u64) -> Played {
    let report = consumer.prime_block_at(first_sample);
    observe(consumer, report)
}

fn seek_report(host: &HostChunkProvider) -> SourceSeekReport {
    host.seek_report().expect("no publication in flight")
}

/// How a pair of identically fed rings is set up before the window.
#[derive(Clone, Copy, Debug)]
enum Feed {
    /// Generation 1 from frame 0, enough for the window and 8 more blocks.
    Steady,
    /// The region ends with a short block at window block `j = 1`.
    EndsAtJOne,
    /// A plain seek to generation 2 applied 2 blocks before the window.
    SeekTwoBlocksEarlier,
}

const WARM_UP_BLOCKS: u64 = 2;
const SEEK_FRAME: u64 = 10_000;

/// Feed `host` and play the warm-up blocks; returns the first sample of the window.
fn prepare(feed: Feed, host: &mut HostChunkProvider, consumer: &mut PcmSourceConsumer) -> u64 {
    match feed {
        Feed::Steady => {
            for block in 0..WARM_UP_BLOCKS + u64::from(K) + 8 {
                submit(host, 1, block * QUANTUM, Q, false);
            }
        }
        Feed::EndsAtJOne => {
            for block in 0..=WARM_UP_BLOCKS {
                submit(host, 1, block * QUANTUM, Q, false);
            }
            submit(host, 1, (WARM_UP_BLOCKS + 1) * QUANTUM, Q / 2, true);
        }
        Feed::SeekTwoBlocksEarlier => {
            for block in 0..4 {
                submit(host, 1, block * QUANTUM, Q, false);
            }
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(SEEK_FRAME),
            })
            .expect("seek");
            for block in 0..WARM_UP_BLOCKS + u64::from(K) + 8 {
                submit(host, 2, SEEK_FRAME + block * QUANTUM, Q, false);
            }
        }
    }
    let mut warm_up = Vec::new();
    for block in 0..WARM_UP_BLOCKS {
        warm_up.push(ordinary(consumer, block * QUANTUM));
    }
    if matches!(feed, Feed::SeekTwoBlocksEarlier) {
        assert!(
            warm_up[0].report.generation_changed,
            "the seek began 2 blocks earlier"
        );
    }
    WARM_UP_BLOCKS * QUANTUM
}

const FEEDS: [Feed; 3] = [Feed::Steady, Feed::EndsAtJOne, Feed::SeekTwoBlocksEarlier];

/// The reference: `k` ordinary blocks over the window, then 8 more.
fn reference(feed: Feed) -> (Vec<Played>, Vec<Played>, SourceSeekReport) {
    let (mut host, mut consumer) = ring();
    let first_sample = prepare(feed, &mut host, &mut consumer);
    let window = (0..u64::from(K))
        .map(|j| ordinary(&mut consumer, first_sample + j * QUANTUM))
        .collect();
    let report = seek_report(&host);
    let after = (u64::from(K)..u64::from(K) + 8)
        .map(|j| ordinary(&mut consumer, first_sample + j * QUANTUM))
        .collect();
    (window, after, report)
}

/// #1320 gate 3: on its twin `k` ordinary blocks, on this ring `k` primes after a true
/// readiness. Every played plane, read report and telemetry are bit-identical, as is the seek
/// report, and so are the next 8 ordinary blocks on both. Steady, with the region ending at
/// `j = 1`, and with a generation a seek began 2 blocks earlier.
#[test]
fn the_prime_equals_the_blocks_the_consumer_would_have_played() {
    for feed in FEEDS {
        let (window, after, reference_report) = reference(feed);
        let (mut host, mut consumer) = ring();
        let first_sample = prepare(feed, &mut host, &mut consumer);
        assert!(consumer.prime_ready(first_sample, K), "{feed:?}");
        let primed_window: Vec<Played> = (0..u64::from(K))
            .map(|j| primed(&mut consumer, first_sample + j * QUANTUM))
            .collect();
        assert_eq!(primed_window, window, "{feed:?}: the primed window");
        assert_eq!(
            seek_report(&host),
            reference_report,
            "{feed:?}: the seek report"
        );
        let primed_after: Vec<Played> = (u64::from(K)..u64::from(K) + 8)
            .map(|j| ordinary(&mut consumer, first_sample + j * QUANTUM))
            .collect();
        assert_eq!(primed_after, after, "{feed:?}: the blocks after the prime");
        let ended = matches!(feed, Feed::EndsAtJOne);
        assert_eq!(window[1].report.end_of_region, ended, "{feed:?}");
        assert!(
            window
                .iter()
                .all(|played| played.report.underrun_frames == 0),
            "{feed:?}: no window block underruns"
        );
        assert_eq!(
            window
                .iter()
                .filter(|played| played.planes[0].is_some())
                .count(),
            if ended { 2 } else { K as usize },
            "{feed:?}: the window plays PCM until the region's end"
        );
    }
}

/// #1320 gate 4: a seek pushed after a true readiness, with its generation's PCM behind it, is
/// not observed by the prime: the primed planes equal gate 3's, the command is still queued (a
/// second seek meets backpressure), and the next ordinary block applies it.
#[test]
fn the_prime_observes_no_command() {
    for feed in [Feed::Steady, Feed::EndsAtJOne] {
        let (window, _, reference_report) = reference(feed);
        let (mut host, mut consumer) = ring();
        let first_sample = prepare(feed, &mut host, &mut consumer);
        assert!(consumer.prime_ready(first_sample, K), "{feed:?}");
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(7),
            frame: SourceFrame(SEEK_FRAME),
        })
        .expect("seek");
        // At the region's end the prime's acquire would reach these blocks: it must stop before
        // them rather than observe their seek.
        for block in 0..2 {
            submit(&mut host, 7, SEEK_FRAME + block * QUANTUM, Q, false);
        }
        let primed_window: Vec<Played> = (0..u64::from(K))
            .map(|j| primed(&mut consumer, first_sample + j * QUANTUM))
            .collect();
        assert_eq!(primed_window, window, "{feed:?}: the primed window");
        assert_eq!(seek_report(&host), reference_report, "{feed:?}");
        assert!(
            matches!(
                host.try_seek(SourceCommand::Seek {
                    generation: SourceGeneration(8),
                    frame: SourceFrame(0),
                }),
                Err(SourceSeekError::Backpressure { .. })
            ),
            "{feed:?}: the seek is still queued"
        );
        let next = ordinary(&mut consumer, first_sample + u64::from(K) * QUANTUM);
        assert!(next.report.generation_changed, "{feed:?}");
        assert_eq!(next.report.active_generation, SourceGeneration(7));
        assert_eq!(next.report.copied_frames, Q, "{feed:?}");
        assert_eq!(
            next.planes[1].as_deref().map(|plane| plane[0]),
            Some(sample(7, SEEK_FRAME, 1).to_bits()),
            "{feed:?}: the sought PCM plays"
        );
    }
}

/// #1320 gate 5: the control thread's handle and the render consumer share one flag.
#[test]
fn the_prime_flag_is_shared_between_the_handle_and_the_consumer() {
    let (producer, consumer, _) = PcmSourceRing::prepare(PcmSourceRingConfig {
        channel_count: 2,
        quantum_frames: QuantumFrames(Q),
        frame_capacity: 4 * QUANTUM,
        initial_generation: SourceGeneration(1),
        seek_runway_blocks: 0,
    })
    .expect("ring");
    assert!(!consumer.prime_required());
    let from_producer = producer.prime_flag();
    let host = producer.into_host_chunk_provider(RATE);
    let from_host = host.prime_flag();
    from_producer.set(true);
    assert!(consumer.prime_required());
    from_host.set(false);
    assert!(!consumer.prime_required());
    from_host.set(true);
    assert!(consumer.prime_required());
}

/// #1320 gate 6: readiness with `k = 16` and 16 primes allocate and free nothing on the render
/// thread, measured after a warm-up window on the same ring.
#[test]
fn readiness_and_sixteen_primes_allocate_nothing() {
    assert_installed();
    const LEAD: u32 = 16;
    let (mut host, mut consumer) = ring();
    for block in 0..u64::from(LEAD) + 1 {
        submit(&mut host, 1, block * QUANTUM, Q, false);
    }
    // Warm-up: one readiness and one prime, so every lazily touched static is warmed.
    assert!(consumer.prime_ready(0, LEAD));
    let _ = consumer.prime_block_at(0);
    let first_sample = QUANTUM;
    submit(&mut host, 1, (u64::from(LEAD) + 1) * QUANTUM, Q, false);
    let mark = current_thread_counters();
    let ready = consumer.prime_ready(first_sample, LEAD);
    let mut copied = 0_u32;
    for j in 0..u64::from(LEAD) {
        copied += consumer
            .prime_block_at(first_sample + j * QUANTUM)
            .copied_frames;
    }
    let delta = current_thread_delta_since(mark);
    assert!(ready);
    assert_eq!(copied, LEAD * Q);
    assert_eq!((delta.allocations, delta.deallocations), (0, 0));
}

/// #2205 gate 5: blocks that hold a live plain seek for its runway, and the block that lands it,
/// allocate and free nothing on the render thread.
///
/// Test value: red when the readiness check, the hold or the landing allocates or frees.
#[test]
fn a_held_live_seek_and_its_landing_allocate_nothing() {
    assert_installed();
    let (producer, mut consumer, _) = PcmSourceRing::prepare(PcmSourceRingConfig {
        channel_count: 2,
        quantum_frames: QuantumFrames(Q),
        frame_capacity: RING_BLOCKS * QUANTUM,
        initial_generation: SourceGeneration(1),
        seek_runway_blocks: 2,
    })
    .expect("ring");
    let mut host = producer.into_host_chunk_provider(RATE);
    for block in 0..4 {
        submit(&mut host, 1, block * QUANTUM, Q, false);
    }
    // Warm-up: one ordinary block.
    let _ = consumer.begin_block_at(0);
    host.try_seek(SourceCommand::Seek {
        generation: SourceGeneration(2),
        frame: SourceFrame(SEEK_FRAME),
    })
    .expect("seek");
    let mut reports = Vec::with_capacity(3);
    let mut generations = [0_u64; 3];
    let mut delta = (0, 0);
    for (index, block) in (1..4_u64).enumerate() {
        let mark = current_thread_counters();
        let report = consumer.begin_block_at(block * QUANTUM);
        let moved = current_thread_delta_since(mark);
        delta.0 += moved.allocations;
        delta.1 += moved.deallocations;
        generations[index] = report.active_generation.0;
        reports.push(report.underrun_frames);
        submit(&mut host, 2, SEEK_FRAME + index as u64 * QUANTUM, Q, false);
    }
    assert_eq!(generations, [1, 1, 2], "held twice, then landed");
    assert_eq!(reports, [0, 0, 0]);
    assert_eq!(delta, (0, 0));
}

/// #1320 D3: a prime reads on the caller's source-read clock, as `begin_block_at` does, even when
/// that clock jumped since the last block: a seek prepared between blocks before the window
/// applies at the window's first block, so the seek reports the window's first sample on both
/// rings, not the consumer's own clock from before the jump.
#[test]
fn the_prime_continues_the_callers_clock_across_a_jump() {
    const JUMPED: u64 = 64 * QUANTUM;
    let mut reports = Vec::new();
    for prime in [false, true] {
        let (mut host, mut consumer) = ring();
        let first_sample = prepare(Feed::Steady, &mut host, &mut consumer);
        assert!(JUMPED > first_sample);
        host.try_seek(SourceCommand::Seek {
            generation: SourceGeneration(2),
            frame: SourceFrame(SEEK_FRAME),
        })
        .expect("seek");
        for block in 0..=u64::from(K) {
            submit(&mut host, 2, SEEK_FRAME + block * QUANTUM, Q, false);
        }
        assert!(consumer.prepare_seek(SourceGeneration(2), SourceFrame(SEEK_FRAME)));
        assert!(consumer.prime_ready(JUMPED, K));
        for j in 0..u64::from(K) {
            let sample = JUMPED + j * QUANTUM;
            let _ = if prime {
                consumer.prime_block_at(sample)
            } else {
                consumer.begin_block_at(sample)
            };
        }
        reports.push(seek_report(&host));
    }
    assert_eq!(reports[0].generation, 2);
    assert_eq!(reports[0].first_sample, JUMPED);
    assert_eq!(reports[1], reports[0]);
}
