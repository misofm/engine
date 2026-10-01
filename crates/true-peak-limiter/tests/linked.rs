//! Issue #990 gate 3: a bank-API scenario whose digest was pinned on the unmodified kernel.
//!
//! The linked gain computer (#990) computes a `LinkMode::Maximum` pair's gain path once when the
//! two channels provably agree, and mirrors every ring write into the right channel. It is class
//! A: nothing observable may move. This file is the end-to-end statement of that, written and
//! pinned **before** the change (on `6ca203f8`, whose limiter crate is byte-identical to the
//! branch point of #990), so the pins below are the base kernel's words and not the new one's.
//!
//! The scenario walks the linked-agreement record through each of its transitions:
//!
//! * blocks 0-29: loud noise on every track, every block limiting (the pair engages);
//! * block 30: a ceiling retarget of **both** channels, delivered as a left span and a right span
//!   with equal values in one block, so the designed words stay equal while ramping (it stays
//!   linked through the ramping dispatch);
//! * block 60: a release retarget of track 0's **left** channel only (the bank disengages, and
//!   stays disengaged: the gain state of that lane diverges);
//! * block 90: a `FullToDefaults` reset (the record is re-established and the pair re-engages).
//!
//! After every block the digest folds every output word, every track's state payload, every
//! track's process report and the resident gain-reduction observation. One digest per width.
//! The engagement pattern itself is asserted by the crate-local witness (gate 2), which can see
//! the kernel's choice; this file sees only what a host sees.

use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ObservationSample, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
    ProcessReport, ResetKind, StatePayloadOutput, StatePayloadSizes,
};
use lane::Backend;
use sha2::{Digest, Sha256};
use true_peak_limiter::{TRUE_PEAK_LIMITER_DESCRIPTOR, TruePeakLimiterFactory};

const FRAMES: usize = 128;
const BLOCKS: usize = 128;
const BOTH_CEILINGS_AT: usize = 30;
const LEFT_RELEASE_AT: usize = 60;
const RESET_AT: usize = 90;

/// SHA-256 of the W8 bank's scenario, recorded on the unmodified kernel.
const W8_DIGEST: &str = "f4892e75a54a09e6ebbc86a84793b8c906d0e5b4aed980da01b9d4e2b3164134";
/// SHA-256 of the W4 bank's scenario, recorded on the unmodified kernel.
const W4_DIGEST: &str = "987746c7d9d1e8081a4d0a767b8bf9d255bd22da6df4b7e3ae350b3f3fee59db";
/// SHA-256 of the scalar instance's scenario (track 0), recorded on the unmodified kernel.
const SCALAR_DIGEST: &str = "cac0d1fd704c1a90d3590b73672c1e1829f36f49031dea520431b4af3c58039c";

/// The console fixture's per-track limiter shape: ceiling, release and a 5 ms lookahead, the same
/// on both channels (`channel: "both"`).
fn values(track: usize) -> [InitialParameterValue; 6] {
    let ceiling = -0.5 - 0.031_25 * track as f32;
    let release = 60.0 + 1.25 * track as f32;
    let lookahead = 5.0;
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: [ceiling, release, lookahead][index / 2],
    })
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// SplitMix64 noise at +3 dBFS peak, one independent stream per track and channel.
///
/// Seeded by track and channel, never by width, so track `k` hears the same samples in every arm.
fn noise(track: usize, channel: usize, frame: usize) -> f32 {
    let mut state = 0x0990_0000_u64 ^ ((track as u64) << 32) ^ ((channel as u64) << 24);
    state = state.wrapping_add((frame as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut mixed = state;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^= mixed >> 31;
    let unit = ((mixed >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0;
    unit * 1.412_537_5
}

/// The spans of block `block` for track `track`, in the effect's ascending validation order.
fn spans_for(block: usize, track: usize) -> Vec<PreparedAutomationSpan> {
    let first = (block * FRAMES) as u64;
    let span = |parameter: u32, channel, value: f32| PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: first,
        end_sample: first,
        start_value: value,
        end_value: value,
    };
    match block {
        BOTH_CEILINGS_AT => vec![
            span(0, ParameterChannel::Left, -3.0),
            span(0, ParameterChannel::Right, -3.0),
        ],
        LEFT_RELEASE_AT if track == 0 => vec![span(1, ParameterChannel::Left, 250.0)],
        _ => Vec::new(),
    }
}

fn fold_report(hasher: &mut Sha256, report: &ProcessReport) {
    for word in [
        report.sanitized_main_samples,
        report.sanitized_sidechain_samples,
        report.invalid_spans,
        report.nonfinite_left_blocks,
        report.nonfinite_right_blocks,
    ] {
        hasher.update(word.to_le_bytes());
    }
}

fn fold_observation(hasher: &mut Sha256, sample: &ObservationSample) {
    hasher.update(sample.left.to_bits().to_le_bytes());
    hasher.update(sample.right.to_bits().to_le_bytes());
}

fn fold_payload(
    hasher: &mut Sha256,
    sizes: StatePayloadSizes,
    write: impl FnOnce(StatePayloadOutput<'_>),
) {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    write(StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"));
    hasher.update(&common);
    hasher.update(&left);
    hasher.update(&right);
}

/// Runs the scenario through one bank of `width` and returns its digest and the largest reduction
/// word it observed (the non-vacuity witness).
fn bank_digest(width: BankWidth, backend: Backend) -> (String, f32) {
    let lanes = width.lanes() as usize;
    let tables: Vec<_> = (0..lanes).map(values).collect();
    let requests: Vec<_> = tables.iter().map(|values| request(values)).collect();
    let mut bank: Box<dyn PreparedNativeEffectBank> = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("valid bank request")
        .expect("the width must bind");
    let sizes = bank.metadata().program_key.state_sizes;
    let mut hasher = Sha256::new();
    let mut deepest = 0.0_f32;
    for block in 0..BLOCKS {
        if block == RESET_AT {
            bank.reset(ResetKind::FullToDefaults);
        }
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                let at = block * FRAMES + frame;
                left[frame * lanes + lane] = noise(lane, 0, at);
                right[frame * lanes + lane] = noise(lane, 1, at);
            }
        }
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        for lane in 0..lanes {
            spans.extend(spans_for(block, lane));
            offsets[lane + 1] = spans.len() as u32;
        }
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                width,
                (block * FRAMES) as u64,
                &spans,
                &offsets,
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        for word in left.iter().chain(right.iter()) {
            hasher.update(word.to_bits().to_le_bytes());
        }
        for lane in 0..lanes {
            fold_report(&mut hasher, &report.reports[lane]);
            fold_payload(&mut hasher, sizes, |output| {
                bank.snapshot_track_state_payload(lane as u32, output)
                    .expect("snapshot");
            });
        }
        let mut observed = vec![ObservationSample::default(); lanes];
        assert!(bank.observe_resident_bank(0, &mut observed), "resident tap");
        for sample in &observed {
            fold_observation(&mut hasher, sample);
            deepest = deepest.max(sample.left).max(sample.right);
        }
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    (bench_support::digest::hex(&bytes), deepest)
}

/// Runs the scenario's track 0 through one scalar instance.
fn scalar_digest() -> (String, f32) {
    let table = values(0);
    let mut effect: Box<dyn PreparedNativeEffect> = TruePeakLimiterFactory
        .prepare(request(&table))
        .expect("prepare");
    let sizes = effect.metadata().state_sizes;
    let mut hasher = Sha256::new();
    let mut deepest = 0.0_f32;
    for block in 0..BLOCKS {
        if block == RESET_AT {
            effect.reset(ResetKind::FullToDefaults);
        }
        let mut left: Vec<f32> = (0..FRAMES)
            .map(|frame| noise(0, 0, block * FRAMES + frame))
            .collect();
        let mut right: Vec<f32> = (0..FRAMES)
            .map(|frame| noise(0, 1, block * FRAMES + frame))
            .collect();
        let spans = spans_for(block, 0);
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (block * FRAMES) as u64,
                &spans,
                FRAMES as u32,
            )
            .expect("block"),
        );
        for word in left.iter().chain(right.iter()) {
            hasher.update(word.to_bits().to_le_bytes());
        }
        fold_report(&mut hasher, &report);
        fold_payload(&mut hasher, sizes, |output| {
            effect.snapshot_state_payload(output).expect("snapshot");
        });
        let mut observed = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut observed), "resident tap");
        fold_observation(&mut hasher, &observed);
        deepest = deepest.max(observed.left).max(observed.right);
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    (bench_support::digest::hex(&bytes), deepest)
}

fn check(label: &str, (digest, deepest): (String, f32), pin: &str) {
    // Non-vacuity: the scenario is about a limiter that is limiting. A reduction word that never
    // left zero would make every gain word `1.0` and the pin would say nothing about the gain path.
    assert!(
        deepest > 0.1,
        "{label}: the scenario never limited (deepest {deepest})"
    );
    if std::env::var_os("MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_LINKED").is_some() {
        println!("{label}: {digest}");
        return;
    }
    assert_eq!(digest, pin, "{label}: the linked-scenario digest moved");
}

#[test]
fn the_linked_scenario_renders_the_pinned_base_words() {
    // Every bank width this build has, widest first, each against its own pin (issue #1112).
    for &width in BankWidth::ALL.iter().rev() {
        let (label, pin) = if width.lanes() == 8 {
            ("W8", W8_DIGEST)
        } else {
            ("W4", W4_DIGEST)
        };
        check(label, bank_digest(width, width.backend()), pin);
    }
    check("scalar", scalar_digest(), SCALAR_DIGEST);
}
