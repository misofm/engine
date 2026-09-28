//! Issue #1014 gate 3: a bank-API scenario whose digest was pinned on the unmodified kernel.
//!
//! #1014 cuts the uniform body's segments where the van Herk block completes, runs the release
//! recursion as a pass of its own, and walks a linked pair's steady frames over bounds-check-free
//! ring streams, in the stationary dispatch. It is class A: no output word, no state payload, no
//! report and no observation may move. This file is the end-to-end statement of that, written and
//! pinned **before** the change (on `0e4732c0`, the branch point of #1014, whose
//! `crates/true-peak-limiter` tree is `6da64966`: #1013's kernel, and the same tree as on the
//! slice's parent `bfec4bba`), so the pins below are the base kernel's words and not the new
//! one's.
//!
//! Four arms per width, each a bank of fixture-shaped tracks at 48 kHz with 128-frame blocks of
//! +3 dBFS noise (every track limits):
//!
//! * **`Maximum` and `DualMono` with a lookahead mix per channel**: left 5 ms (`Wb = 241`), right
//!   9.9 ms (`Wb = 476`, so `R - Wb` is 5). The channels' windows differ, so the pair is not linked
//!   and the dual body cuts its segments at whichever channel completes first;
//! * **`Maximum` with 5 ms on both channels**: the #990 linked body, whose steady frames take the
//!   ring streams (`R - Wb = 240`);
//! * **`Maximum` with 9.9 ms on both channels**: linked, with `R - Wb = 5`, so a run of steady
//!   frames takes the streams up to five frames and the checked loop beyond.
//!
//! Every arm retargets the ceiling of both channels of every track at block 40 (a left span and a
//! right span of one value: the ramping dispatch, still linked), and resets to defaults at
//! block 80. After every block the digest folds every output word, every track's process report,
//! every track's state payload and the resident gain-reduction observation. One digest per width.

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
const RETARGET_AT: usize = 40;
const RESET_AT: usize = 80;
/// Tracks rendered through scalar instances, one instance each: as many as the widest bank.
const SCALAR_TRACKS: usize = 8;

/// SHA-256 of the W8 banks' scenario, recorded on the unmodified kernel.
const W8_DIGEST: &str = "8b20d428e499689557b2a4f27f7567091dd5c04c055ffde2119f9b22bb299552";
/// SHA-256 of the W4 banks' scenario, recorded on the unmodified kernel.
const W4_DIGEST: &str = "f72aa85702645d5286316e0796630cf0d29f38062654f7958beac4abad17a0c8";
/// SHA-256 of the scalar instances' scenario (tracks 0-7), recorded on the unmodified kernel.
const SCALAR_DIGEST: &str = "ea06a84cd472426f396e9cdd2502f111706825a3e2dd9dd669697cba22847a17";

/// One arm: a link mode and each channel's lookahead in milliseconds.
#[derive(Clone, Copy, Debug)]
struct Arm {
    link_mode: LinkMode,
    left_lookahead: f32,
    right_lookahead: f32,
}

const ARMS: [Arm; 4] = [
    Arm {
        link_mode: LinkMode::Maximum,
        left_lookahead: 5.0,
        right_lookahead: 9.9,
    },
    Arm {
        link_mode: LinkMode::DualMono,
        left_lookahead: 5.0,
        right_lookahead: 9.9,
    },
    Arm {
        link_mode: LinkMode::Maximum,
        left_lookahead: 5.0,
        right_lookahead: 5.0,
    },
    Arm {
        link_mode: LinkMode::Maximum,
        left_lookahead: 9.9,
        right_lookahead: 9.9,
    },
];

/// The console fixture's per-track ceiling and release, with the arm's lookahead per channel.
fn values(track: usize, arm: Arm) -> [InitialParameterValue; 6] {
    let ceiling = -0.5 - 0.031_25 * track as f32;
    let release = 60.0 + 1.25 * track as f32;
    core::array::from_fn(|index| {
        let left = index % 2 == 0;
        let lookahead = if left {
            arm.left_lookahead
        } else {
            arm.right_lookahead
        };
        InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if left {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: [ceiling, release, lookahead][index / 2],
        }
    })
}

fn request(values: &[InitialParameterValue], link_mode: LinkMode) -> PrepareEffectRequest<'_> {
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
        link_mode,
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

/// SplitMix64 noise at +3 dBFS peak, one independent stream per track and channel. Seeded by
/// track and channel, never by width, so track `k` hears the same samples in every arm.
fn noise(track: usize, channel: usize, frame: usize) -> f32 {
    let mut state = 0x1014_0000_u64 ^ ((track as u64) << 32) ^ ((channel as u64) << 24);
    state = state.wrapping_add((frame as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut mixed = state;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^= mixed >> 31;
    let unit = ((mixed >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0;
    unit * 1.412_537_5
}

/// Block `block`'s spans for one track: at the retarget, the ceiling of both channels, as a left
/// span and a right span of one value, in the effect's ascending validation order.
fn spans_for(block: usize, track: usize) -> Vec<PreparedAutomationSpan> {
    if block != RETARGET_AT {
        return Vec::new();
    }
    let first = (block * FRAMES) as u64;
    let value = -3.0 - 0.25 * track as f32;
    [ParameterChannel::Left, ParameterChannel::Right]
        .into_iter()
        .map(|channel| PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index: 0,
            start_sample: first,
            end_sample: first,
            start_value: value,
            end_value: value,
        })
        .collect()
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

/// Runs one arm through one bank of `width`, folding into `hasher`, and returns the largest
/// reduction word it observed (the non-vacuity witness).
fn bank_run(width: BankWidth, backend: Backend, arm: Arm, hasher: &mut Sha256) -> f32 {
    let lanes = width.lanes() as usize;
    let tables: Vec<_> = (0..lanes).map(|track| values(track, arm)).collect();
    let requests: Vec<_> = tables
        .iter()
        .map(|values| request(values, arm.link_mode))
        .collect();
    let mut bank: Box<dyn PreparedNativeEffectBank> = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("valid bank request")
        .expect("the width must bind");
    let sizes = bank.metadata().program_key.state_sizes;
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
        hasher.update([block as u8]);
        for word in left.iter().chain(right.iter()) {
            hasher.update(word.to_bits().to_le_bytes());
        }
        for lane in 0..lanes {
            fold_report(hasher, &report.reports[lane]);
            fold_payload(hasher, sizes, |output| {
                bank.snapshot_track_state_payload(lane as u32, output)
                    .expect("snapshot");
            });
        }
        let mut observed = vec![ObservationSample::default(); lanes];
        assert!(bank.observe_resident_bank(0, &mut observed), "resident tap");
        for sample in &observed {
            fold_observation(hasher, sample);
            deepest = deepest.max(sample.left).max(sample.right);
        }
    }
    deepest
}

/// Runs one arm's track `track` through one scalar instance, folding into `hasher`.
fn scalar_run(track: usize, arm: Arm, hasher: &mut Sha256) -> f32 {
    let table = values(track, arm);
    let mut effect: Box<dyn PreparedNativeEffect> = TruePeakLimiterFactory
        .prepare(request(&table, arm.link_mode))
        .expect("prepare");
    let sizes = effect.metadata().state_sizes;
    let mut deepest = 0.0_f32;
    for block in 0..BLOCKS {
        if block == RESET_AT {
            effect.reset(ResetKind::FullToDefaults);
        }
        let mut left: Vec<f32> = (0..FRAMES)
            .map(|frame| noise(track, 0, block * FRAMES + frame))
            .collect();
        let mut right: Vec<f32> = (0..FRAMES)
            .map(|frame| noise(track, 1, block * FRAMES + frame))
            .collect();
        let spans = spans_for(block, track);
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
        hasher.update([block as u8]);
        for word in left.iter().chain(right.iter()) {
            hasher.update(word.to_bits().to_le_bytes());
        }
        fold_report(hasher, &report);
        fold_payload(hasher, sizes, |output| {
            effect.snapshot_state_payload(output).expect("snapshot");
        });
        let mut observed = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut observed), "resident tap");
        fold_observation(hasher, &observed);
        deepest = deepest.max(observed.left).max(observed.right);
    }
    deepest
}

/// Non-vacuity: the scenario is about a limiter that limits. A reduction word that never left
/// zero would make every gain word `1.0` and the pin would say nothing about the gain path.
fn check_deepest(label: &str, deepest: f32) {
    assert!(
        deepest > 0.1,
        "{label}: the scenario never limited (deepest {deepest})"
    );
}

fn bank_digest(width: BankWidth, backend: Backend) -> String {
    let mut hasher = Sha256::new();
    for arm in ARMS {
        let deepest = bank_run(width, backend, arm, &mut hasher);
        check_deepest(&format!("{width:?} {arm:?}"), deepest);
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    bench_support::digest::hex(&bytes)
}

fn scalar_digest() -> String {
    let mut hasher = Sha256::new();
    for arm in ARMS {
        for track in 0..SCALAR_TRACKS {
            let deepest = scalar_run(track, arm, &mut hasher);
            check_deepest(&format!("scalar {arm:?} track {track}"), deepest);
        }
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    bench_support::digest::hex(&bytes)
}

fn check(label: &str, digest: String, pin: &str) {
    if std::env::var_os("MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_SEGMENTS").is_some() {
        println!("{label}: {digest}");
        return;
    }
    assert_eq!(digest, pin, "{label}: the segments-scenario digest moved");
}

#[test]
fn the_segments_scenario_renders_the_pinned_base_words() {
    let backend = Backend::current();
    if backend.width() >= 8 {
        check(
            "W8",
            bank_digest(BankWidth::Eight, Backend::Simd8),
            W8_DIGEST,
        );
    }
    if backend.width() >= 4 {
        check(
            "W4",
            bank_digest(BankWidth::Four, Backend::Simd4),
            W4_DIGEST,
        );
    }
    check("scalar", scalar_digest(), SCALAR_DIGEST);
}
