//! Issue #1013 gate 4: a bank-API scenario whose digest was pinned on the unmodified kernel.
//!
//! #1013 starts each of the detector's four Annex-2 accumulators at its first product instead of
//! at `+0.0`. The two forms differ only when that product is `-0.0`, and then only in the sign of
//! a zero that `abs` erases before anything reads it, so the change is class A: no output word, no
//! state payload, no report and no observation may move. This file is the end-to-end statement of
//! that, written and pinned **before** the change (on `52ad1460`, the branch point of #1013, whose
//! `crates/true-peak-limiter` tree is `99fed5cb`, the same as on `220c5db5`), so the pins below are
//! the base kernel's words and not the new one's.
//!
//! The scenario feeds every track, block by block, the inputs the seed can see:
//!
//! * +3 dBFS noise, so every track limits and every detector phase is a full-magnitude chain;
//! * all-signed-zero blocks: every sample `+0.0` or `-0.0`, the sign drawn per lane, channel and
//!   frame, so the history holds mixed signed zeros and a first product is `-0.0` about half the
//!   time;
//! * subnormal blocks: random signs and mantissas with a zero exponent. Most products of a
//!   subnormal and a table coefficient round to a signed zero, the rest stay subnormal;
//! * one NaN in track 1's left input, whose output a lookahead later trips the §4.4 boundary check
//!   (zeroed, reset, counted).
//!
//! It runs under `LinkMode::Maximum` (the #990 linked body while the pair agrees), under
//! `LinkMode::DualMono` (the dual body) and, for the banks, through `process_bank_mono` (the
//! collapsed body), so all three bodies that reach `annex2_phases` are in the pin. After every
//! block the digest folds every output word, every track's process report, every track's state
//! payload and the resident gain-reduction observation. One digest per width.

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ObservationSample, ParameterChannel, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ProcessReport, StatePayloadOutput, StatePayloadSizes,
};
use lane::Backend;
use sha2::{Digest, Sha256};
use true_peak_limiter::{TRUE_PEAK_LIMITER_DESCRIPTOR, TruePeakLimiterFactory};

const FRAMES: usize = 128;
const BLOCKS: usize = 96;
/// Tracks rendered through scalar instances, one instance each: as many as the widest bank.
const SCALAR_TRACKS: usize = 8;
/// The block that carries the one NaN, and where it sits.
const NAN_BLOCK: usize = 56;
const NAN_TRACK: usize = 1;
const NAN_FRAME: usize = 37;

/// SHA-256 of the W8 bank's scenario, recorded on the unmodified kernel.
const W8_DIGEST: &str = "4b57d4daef9181fe499528ad36c114de8c62b0c6fcd1f621d9f7db07fb1898f2";
/// SHA-256 of the W4 bank's scenario, recorded on the unmodified kernel.
const W4_DIGEST: &str = "c578264074e61d11f4bda42b1c43835edd569135c54f012972905fffaadd307a";
/// SHA-256 of the scalar instances' scenario (tracks 0-7), recorded on the unmodified kernel.
const SCALAR_DIGEST: &str = "ec135dac1fd3e82d7d0638130cc1a66879a0c330a515036db33692bf415f7ca2";

/// What block `block` feeds every track.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Content {
    Noise,
    SignedZeros,
    Subnormals,
}

/// The schedule. Each non-noise run follows limiting noise, so the detector history turns over
/// from full-scale words into zeros or subnormals while the gain path is still releasing, and
/// back.
fn content(block: usize) -> Content {
    match block {
        24..=39 | 72..=79 => Content::SignedZeros,
        40..=55 | 80..=87 => Content::Subnormals,
        _ => Content::Noise,
    }
}

/// The console fixture's per-track limiter shape: ceiling, release and a 5 ms lookahead, the same
/// on both channels.
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

/// SplitMix64 of one track, channel and absolute frame. Seeded by track and channel, never by
/// width, so track `k` hears the same samples in every arm.
fn hash(track: usize, channel: usize, frame: usize) -> u64 {
    let mut state = 0x1013_0000_u64 ^ ((track as u64) << 32) ^ ((channel as u64) << 24);
    state = state.wrapping_add((frame as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut mixed = state;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^ (mixed >> 31)
}

/// The input sample of one track, channel and absolute frame.
fn sample(track: usize, channel: usize, frame: usize) -> f32 {
    let block = frame / FRAMES;
    if block == NAN_BLOCK && track == NAN_TRACK && channel == 0 && frame % FRAMES == NAN_FRAME {
        return f32::NAN;
    }
    let bits = hash(track, channel, frame);
    match content(block) {
        // +3 dBFS peak.
        Content::Noise => {
            let unit = ((bits >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0;
            unit * 1.412_537_5
        }
        Content::SignedZeros => f32::from_bits(((bits >> 63) as u32) << 31),
        // Sign and mantissa random, exponent field zero. A zero mantissa (a signed zero) is
        // possible and kept.
        Content::Subnormals => f32::from_bits((bits as u32) & 0x807F_FFFF),
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

/// The fixed latency at 48 kHz, `N + 6` samples.
fn latency() -> usize {
    TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate")
        .latency
        .0 as usize
}

/// How many whole blocks of line fill an empty line puts out before the first input word arrives.
fn fill_blocks() -> usize {
    latency() / FRAMES
}

/// The blocks a §4.4 reset zeroes: the block the NaN reaches the output in, which the check zeroes,
/// and the line fill of the reset instance after it.
fn reset_blocks() -> Vec<usize> {
    let reset = (NAN_BLOCK * FRAMES + NAN_FRAME + latency()) / FRAMES;
    (reset..=reset + fill_blocks()).collect()
}

/// What the non-vacuity checks read: the deepest reduction observed, how many output words were
/// `-0.0` and subnormal, and which blocks came out entirely `+0.0`.
///
/// The limiter's §4.4 reset is not in its `ProcessReport` (the counter is the instance's own
/// instrumentation), so the host-visible trace of it is the one it leaves in the audio: a whole
/// block of `+0.0`, then the line fill of an emptied line. Past the first line fill no other block
/// of this scenario is entirely `+0.0`: a noise block's words are nonzero, a signed-zero block's
/// carry `-0.0` and a subnormal block's carry subnormals.
#[derive(Default)]
struct Witness {
    deepest: f32,
    negative_zeros: usize,
    subnormals: usize,
    zeroed_blocks: Vec<usize>,
}

impl Witness {
    fn block(&mut self, block: usize, planes: &[&[f32]]) {
        let mut zeroed = true;
        for word in planes.iter().flat_map(|plane| plane.iter()) {
            if word.to_bits() == 0x8000_0000 {
                self.negative_zeros += 1;
            }
            if word.is_subnormal() {
                self.subnormals += 1;
            }
            zeroed &= word.to_bits() == 0;
        }
        if block >= fill_blocks() && zeroed {
            self.zeroed_blocks.push(block);
        }
    }
}

/// How a bank renders: through `process_bank` under a link mode, or collapsed.
#[derive(Clone, Copy)]
enum Body {
    Dual(LinkMode),
    Collapsed,
}

/// Runs the scenario through one bank of `width` under `body`, folding into `hasher`.
fn bank_run(width: BankWidth, backend: Backend, body: Body, hasher: &mut Sha256) -> Witness {
    let lanes = width.lanes() as usize;
    let link_mode = match body {
        Body::Dual(link_mode) => link_mode,
        Body::Collapsed => LinkMode::Maximum,
    };
    let tables: Vec<_> = (0..lanes).map(values).collect();
    let requests: Vec<_> = tables
        .iter()
        .map(|values| request(values, link_mode))
        .collect();
    let mut bank: Box<dyn PreparedNativeEffectBank> = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("valid bank request")
        .expect("the width must bind");
    if let Body::Collapsed = body {
        assert!(bank.supports_mono_collapse(), "the limiter collapses");
    }
    let sizes = bank.metadata().program_key.state_sizes;
    let offsets = vec![0_u32; lanes + 1];
    let mut witness = Witness::default();
    for block in 0..BLOCKS {
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                let at = block * FRAMES + frame;
                left[frame * lanes + lane] = sample(lane, 0, at);
                right[frame * lanes + lane] = match body {
                    Body::Dual(_) => sample(lane, 1, at),
                    // Never gathered by the collapsed body; a finite poison makes a read visible.
                    Body::Collapsed => f32::from_bits(0x7F7F_FFFF),
                };
            }
        }
        let block_request = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            width,
            (block * FRAMES) as u64,
            &[],
            &offsets,
            FRAMES as u32,
        )
        .expect("bank block");
        let report = match body {
            Body::Dual(_) => bank.process_bank(block_request),
            Body::Collapsed => bank.process_bank_mono(block_request),
        };
        hasher.update([block as u8]);
        match body {
            Body::Dual(_) => {
                for word in left.iter().chain(right.iter()) {
                    hasher.update(word.to_bits().to_le_bytes());
                }
                witness.block(block, &[&left, &right]);
            }
            Body::Collapsed => {
                for word in &left {
                    hasher.update(word.to_bits().to_le_bytes());
                }
                witness.block(block, &[&left]);
                // A collapsed bank's right section is stale until the disengage copy; take it
                // before every snapshot, which is always sound and leaves the left run alone.
                bank.desymmetrize_channels();
            }
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
            witness.deepest = witness.deepest.max(sample.left).max(sample.right);
        }
    }
    witness
}

/// Runs the scenario's track `track` through one scalar instance under `link_mode`.
fn scalar_run(track: usize, link_mode: LinkMode, hasher: &mut Sha256) -> Witness {
    let table = values(track);
    let mut effect: Box<dyn PreparedNativeEffect> = TruePeakLimiterFactory
        .prepare(request(&table, link_mode))
        .expect("prepare");
    let sizes = effect.metadata().state_sizes;
    let mut witness = Witness::default();
    for block in 0..BLOCKS {
        let mut left: Vec<f32> = (0..FRAMES)
            .map(|frame| sample(track, 0, block * FRAMES + frame))
            .collect();
        let mut right: Vec<f32> = (0..FRAMES)
            .map(|frame| sample(track, 1, block * FRAMES + frame))
            .collect();
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (block * FRAMES) as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        hasher.update([block as u8]);
        for word in left.iter().chain(right.iter()) {
            hasher.update(word.to_bits().to_le_bytes());
        }
        witness.block(block, &[&left, &right]);
        fold_report(hasher, &report);
        fold_payload(hasher, sizes, |output| {
            effect.snapshot_state_payload(output).expect("snapshot");
        });
        let mut observed = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut observed), "resident tap");
        fold_observation(hasher, &observed);
        witness.deepest = witness.deepest.max(observed.left).max(observed.right);
    }
    witness
}

/// Non-vacuity of one run. The scenario is about a limiter that limits, that carries signed zeros
/// and subnormals through its line, and whose §4.4 reset fires once.
fn check_witness(label: &str, witness: &Witness, expect_reset: bool) {
    assert!(
        witness.deepest > 0.1,
        "{label}: the scenario never limited (deepest {})",
        witness.deepest
    );
    assert!(
        witness.negative_zeros > 0,
        "{label}: no -0.0 reached the output"
    );
    assert!(
        witness.subnormals > 0,
        "{label}: no subnormal reached the output"
    );
    let expected = if expect_reset {
        reset_blocks()
    } else {
        Vec::new()
    };
    assert_eq!(
        witness.zeroed_blocks, expected,
        "{label}: blocks zeroed by the §4.4 reset"
    );
}

fn bank_digest(width: BankWidth, backend: Backend) -> String {
    let mut hasher = Sha256::new();
    for (name, body) in [
        ("maximum", Body::Dual(LinkMode::Maximum)),
        ("dual_mono", Body::Dual(LinkMode::DualMono)),
        ("collapsed", Body::Collapsed),
    ] {
        let witness = bank_run(width, backend, body, &mut hasher);
        // The NaN is in track 1, which is lane 1 of the bank at both widths.
        check_witness(&format!("{width:?} {name}"), &witness, true);
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    bench_support::digest::hex(&bytes)
}

fn scalar_digest() -> String {
    let mut hasher = Sha256::new();
    for (name, link_mode) in [
        ("maximum", LinkMode::Maximum),
        ("dual_mono", LinkMode::DualMono),
    ] {
        for track in 0..SCALAR_TRACKS {
            let witness = scalar_run(track, link_mode, &mut hasher);
            check_witness(
                &format!("scalar {name} track {track}"),
                &witness,
                track == NAN_TRACK,
            );
        }
    }
    let bytes: [u8; 32] = hasher.finalize().into();
    bench_support::digest::hex(&bytes)
}

fn check(label: &str, digest: String, pin: &str) {
    if std::env::var_os("MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_SEEDLESS").is_some() {
        println!("{label}: {digest}");
        return;
    }
    assert_eq!(digest, pin, "{label}: the seedless-scenario digest moved");
}

#[test]
fn the_seedless_scenario_renders_the_pinned_base_words() {
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
