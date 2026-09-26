//! Issue #918 gate 2: a compiled session whose banked tracks gather their input straight from the
//! played transfer block renders the bits it renders with every claim copied into the arena.
//!
//! End to end through the production pieces: the host facade prepares the session, the real
//! `PcmSourceRing` carries the PCM, the production graph source driver lends the played block, and
//! the test feeds the ring through `SourceControlSet::submit` (no native worker). The copy arm is
//! the same session prepared with `graph::test_only_set_source_in_place_declined`, which binds every
//! claim on the ring-to-arena copy exactly as before the issue.

use host_core::{
    HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission, prepare_host_session,
};
use session::{canonical_session_json, parse_session_json};

const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const NINE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &[u8] = b"fixture-source";
/// The fixture source's mapped region, `0..REGION_END` source frames.
const REGION_END: u64 = 48_000;
/// Frames in the region's last, short chunk.
const SHORT: u32 = 50;
const BLOCKS: usize = 12;

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

/// The bank fixture with both dual-mono lanes of every track reading source channel 0.
fn mono_bank() -> String {
    let mut model = parse_session_json(BANK).expect("fixture parses");
    for track in &mut model.tracks {
        track.right_source_channel = track.left_source_channel;
    }
    canonical_session_json(&model).expect("mutated fixture canonicalizes")
}

/// One source chunk: `frames` frames of a per-chunk pattern starting at `start_frame`, the two
/// channels different, a negative zero in each.
fn submit(prepared: &mut PreparedHost, generation: u64, start_frame: u64, frames: u32) {
    let pattern = |channel: u64| -> Vec<f32> {
        (0..u64::from(frames))
            .map(|frame| {
                if frame == 3 {
                    -0.0
                } else {
                    let value = (start_frame + frame) * (channel + 3) % 211;
                    (value as f32 - 105.0) * 0.004_75
                }
            })
            .collect()
    };
    let (left, right) = (pattern(0), pattern(1));
    let end = start_frame + u64::from(frames);
    prepared
        .sources
        .submit(
            SOURCE,
            SourceSubmission {
                generation,
                start_frame,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames,
                end_of_region: end == REGION_END,
            },
        )
        .expect("source chunk admitted");
}

/// What one arm rendered: every block's master bits,
/// `[claims copied, gathers from a played block, gathers of silence]` over the render, and the
/// blocks its banks rendered mono-collapsed (one plane gathered and processed).
struct Run {
    masters: Vec<Vec<u32>>,
    counts: [u64; 3],
    collapsed: u64,
}

/// Prepare `document` in place or, `declined`, on the copy, then render [`BLOCKS`] blocks while
/// feeding the ring:
///
/// * blocks 0 to 3, 5 and 6: one whole chunk each, generation 1, just ahead of its render;
/// * block 4: nothing submitted, so the consumer underruns. Its chunk arrives before block 5 (a
///   generation's chunks are contiguous) and is discarded as stale;
/// * before block 7: a seek to generation 2 near the region's end, then that generation's two
///   whole chunks and the region's short last chunk (`end_of_region`);
/// * blocks 7 and 8 play whole chunks, block 9 plays the short one (its tail zeroed), and blocks
///   10 and 11 are past the region's end.
fn render(document: &str, declined: bool) -> Run {
    graph::test_only_set_source_in_place_declined(declined);
    let prepared = prepare_host_session(document, &caps());
    graph::test_only_set_source_in_place_declined(false);
    let (_, mut prepared) = prepared.unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let quantum = prepared.report.quantum_frames;
    let frames = quantum as usize;
    let seek_to = REGION_END - 2 * u64::from(quantum) - u64::from(SHORT);
    graph::test_only_source_plane_reset();
    let mut masters = Vec::with_capacity(BLOCKS);
    for block in 0..BLOCKS {
        match block {
            4 => {}
            5 => {
                // The withheld chunk arrives late: the consumer is past it and discards it.
                submit(&mut prepared, 1, (4 * frames) as u64, quantum);
                submit(&mut prepared, 1, (5 * frames) as u64, quantum);
            }
            0..=6 => submit(&mut prepared, 1, (block * frames) as u64, quantum),
            7 => {
                prepared
                    .sources
                    .seek(SOURCE, 2, seek_to)
                    .expect("seek admitted");
                submit(&mut prepared, 2, seek_to, quantum);
                submit(&mut prepared, 2, seek_to + u64::from(quantum), quantum);
                submit(&mut prepared, 2, seek_to + 2 * u64::from(quantum), SHORT);
            }
            _ => {}
        }
        let mut samples = vec![f32::from_bits(0x7fc0_0918); 2 * frames];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, frames, frames)
            .expect("output planes");
        prepared
            .plan
            .render(
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
                engine::realtime::RenderTime {
                    absolute_sample: (block * frames) as u64,
                },
            )
            .expect("render");
        masters.push(samples.iter().map(|word| word.to_bits()).collect());
    }
    let counts = graph::test_only_source_plane_counts();
    Run {
        masters,
        counts,
        collapsed: prepared.plan.bank_collapse_counters()[0],
    }
}

/// FNV-1a over every master word, in render order.
fn digest(masters: &[Vec<u32>]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for word in masters.iter().flatten() {
        for byte in word.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    hash
}

/// Gate 2 of issue #918, on the eight-track bank console, its mono-mapped twin (whose bank
/// collapses on every block and so gathers one plane), and the nine-track fixture: the in-place
/// arm's master is the
/// copy arm's in every block -- whole blocks, the underrun, the first blocks after the seek's
/// generation change, the short end-of-region block and the blocks past the end -- and the mode
/// counters show which arm gathered in place: every claim of these sessions is bound in place,
/// served its played block on the nine blocks that play one and silence on the other three.
#[test]
fn a_ring_fed_banked_session_gathers_in_place_with_the_copy_bits() {
    const PLAYED: u64 = 9;
    let mono = mono_bank();
    for (name, document, claims, collapsed) in [
        ("bank console", BANK, 8_u64, 0),
        ("mono bank console", mono.as_str(), 8, BLOCKS as u64),
        ("nine-track", NINE, 9, 0),
    ] {
        let in_place = render(document, false);
        let copy = render(document, true);
        assert_eq!(
            [in_place.collapsed, copy.collapsed],
            [collapsed; 2],
            "{name}: collapsed blocks"
        );
        for (block, (actual, expected)) in in_place.masters.iter().zip(&copy.masters).enumerate() {
            assert_eq!(
                actual, expected,
                "{name}, block {block}: the in-place master is the copy arm's"
            );
        }
        assert_eq!(
            digest(&in_place.masters),
            digest(&copy.masters),
            "{name}: the output digests agree"
        );
        for block in [0, 5, 7, 8, 9] {
            assert!(
                copy.masters[block].iter().any(|word| *word != 0),
                "{name}, block {block}: a played block carries audio"
            );
        }
        assert_eq!(
            copy.counts,
            [claims * BLOCKS as u64, 0, 0],
            "{name}: the copy arm copies every claim every block"
        );
        assert_eq!(
            in_place.counts,
            [0, claims * PLAYED, claims * (BLOCKS as u64 - PLAYED)],
            "{name}: [claims copied, played gathers, silent gathers] in place"
        );
    }
}
