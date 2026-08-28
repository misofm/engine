//! E13 — the contract fixtures, unchanged from V1.
//!
//! Every assertion here was already true before #88 and must still be true after it (master plan
//! section 8.2: a re-landing job does not move a contract fixture). The bank-fallback test is the
//! one that changed shape: it named `Backend::Simd8` / `Aarch64Neon` as "the backend
//! that is not available here", and D4 revision 4 removed runtime dispatch, so the unavailable
//! backend is now simply "a bank width this build was not compiled for".

mod support;

use miso_engine_compressor::{
    COMPRESSOR_DESCRIPTOR_V1, COMPRESSOR_PARAMETERS_V1, CompressorFactory,
};
use miso_engine_effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectProcessBlock, LatencySamples,
    LinkMode, NativeEffectFactory, PrepareEffectBankRequest, PreparedSidechainPort, ResetKind,
    StatePayloadOutput, expected_prepared_metadata, validate_descriptor,
};
use miso_engine_effect_runtime::state_payload::read_f32;
use miso_engine_lane::Backend;

use support::{
    PARAMETER_COUNT, STATE_HEADER_WORDS, initial_values, prepare, render_scalar, request,
    sidechain_port, values_with,
};

/// Descriptor rows, latency, payload sizes, scratch and the resource envelope are frozen.
///
/// Red mutation: `scratch_fixed_bytes: 0` (the F10 change #95 owns) or `STATE_HEADER_WORDS = 26`
/// (83c's two-word header) — RED here, which is the point: neither may be smuggled in by #88.
#[test]
fn descriptor_rows_and_resource_envelope_are_frozen() {
    validate_descriptor(&COMPRESSOR_DESCRIPTOR_V1).expect("descriptor");
    assert_eq!(COMPRESSOR_DESCRIPTOR_V1.id.as_str(), "miso.compressor");
    assert_eq!(COMPRESSOR_DESCRIPTOR_V1.state_layout_version, 1);
    assert_eq!(COMPRESSOR_PARAMETERS_V1.len(), PARAMETER_COUNT);
    for (quality, (rate, latency, lane_bytes, total_bytes)) in
        COMPRESSOR_DESCRIPTOR_V1.qualities.iter().zip([
            (44_100_u32, 882_u64, 7_160_u32, 14_320_u64),
            (48_000, 960, 7_784, 15_568),
            (88_200, 1_764, 14_216, 28_432),
            (96_000, 1_920, 15_464, 30_928),
        ])
    {
        let ring_length = latency as usize + 1;
        assert_eq!(quality.sample_rate, rate);
        assert_eq!(quality.latency, LatencySamples(latency));
        assert_eq!(quality.maximum_state.common_bytes, 0);
        assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.total(), Some(total_bytes));
        assert_eq!(quality.scratch_fixed_bytes, 64);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
        assert_eq!(
            lane_bytes as usize,
            (STATE_HEADER_WORDS + 2 * ring_length) * 4
        );
    }
}

/// The runtime's parameter specs describe exactly the descriptor rows they are derived from.
///
/// The crate carries no domain predicate of its own any more: `params::parameter_value_valid` is
/// the workspace's one implementation and `design::PARAMETER_SPECS` is the descriptor expressed in
/// its shape. This test is what stops the two drifting.
///
/// Red mutation: change any `minimum`/`maximum`/`default_value` in one of the two places — RED.
#[test]
fn every_descriptor_row_admits_exactly_its_own_domain() {
    let factory = CompressorFactory;
    for (index, parameter) in COMPRESSOR_PARAMETERS_V1.iter().enumerate() {
        let minimum = parameter.minimum.expect("continuous minimum");
        let maximum = parameter.maximum.expect("continuous maximum");
        assert!(minimum <= parameter.default_value && parameter.default_value <= maximum);
        for (value, admitted) in [
            (minimum, true),
            (maximum, true),
            (parameter.default_value, true),
            (minimum - 1.0, false),
            (maximum + 1.0, false),
            (f32::NAN, false),
            (f32::INFINITY, false),
        ] {
            let values = values_with(&[(index, value)]);
            let accepted = factory.prepare(request(&values)).is_ok();
            assert_eq!(
                accepted, admitted,
                "parameter {index} value {value}: accepted {accepted}, expected {admitted}"
            );
        }
    }
    // `-0.0` is a preparation-time rejection for every parameter whose domain contains zero.
    for index in [2_usize, 5, 6, 7] {
        let values = values_with(&[(index, -0.0)]);
        assert!(
            factory.prepare(request(&values)).is_err(),
            "parameter {index} must reject -0.0 at preparation"
        );
    }
}

/// Preparation metadata matches the contract's own derivation, and one byte below either limit
/// rejects.
#[test]
fn preparation_has_expected_metadata_and_one_byte_below_rejects() {
    let values = initial_values();
    let factory = CompressorFactory;
    let effect = factory.prepare(request(&values)).expect("prepare");
    assert_eq!(
        effect.metadata().latency,
        expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR_V1, request(&values))
            .expect("metadata")
            .latency
    );

    let mut below = request(&values);
    below.limits.maximum_total_state_bytes -= 1;
    assert_eq!(
        factory.prepare(below).err().expect("state limit").code,
        "effect.resource.limit"
    );

    let mut below_scratch = request(&values);
    below_scratch.limits.maximum_scratch_bytes -= 1;
    assert_eq!(
        factory
            .prepare(below_scratch)
            .err()
            .expect("scratch limit")
            .code,
        "effect.resource.limit"
    );
}

/// `D = N - L` is derived at prepare and at a full reset, and only there.
///
/// `L = floor(ms * Fs / 1000 + 0.5)` clamped to `N`, `D = N - L`. At 48 kHz `N = 960`, so
/// `lookahead = 20 ms` gives `L = 960` and `D = 0`: the detector reads the entry written **this**
/// frame, 960 frames ahead of the output it gains, and the envelope is fully settled by the time
/// the first sample leaves the latency. `lookahead = 0` gives `D = N`: the detector is aligned
/// with the output, so at sample 960 the envelope has had exactly one sample of attack. The gap
/// between the two is the whole content of the lookahead tap.
///
/// Red mutation: `gather_detector` uses `D[0]` for every lane, or `D = L` instead of `N - L` —
/// RED, the two configurations become indistinguishable.
#[test]
fn lookahead_taps_are_derived_only_at_prepare_restore_and_full_reset() {
    let mut first_audible = Vec::new();
    for lookahead_ms in [20.0_f32, 0.0] {
        let values = values_with(&[
            (0, -40.0),
            (1, 20.0),
            (2, 0.0),
            (3, 0.1),
            (6, 1.0),
            (7, lookahead_ms),
        ]);
        let mut effect = prepare(request(&values));
        let mut left = vec![0.5_f32; 1_152];
        let mut right = vec![0.5_f32; 1_152];
        render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
        // Sample 960 is the first sample the latency lets through.
        first_audible.push((left[960], left[1_100]));
    }
    let (settled, late) = (first_audible[0], first_audible[1]);
    assert!(
        settled.0 < late.0,
        "a 20 ms lookahead must have settled by sample 960: {settled:?} vs {late:?}"
    );
    // By sample 1,100 both have settled to the same gain.
    assert!((settled.1 - late.1).abs() < 1.0e-6, "{settled:?} {late:?}");
    assert!(settled.0 < 0.5 && late.0 < 0.5);

    // The payload carries the lookahead value itself, and a full reset re-derives from it.
    let values = values_with(&[(7, 12.5)]);
    let mut effect = prepare(request(&values));
    effect.reset(ResetKind::FullToDefaults);
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    assert_eq!(read_f32(&left, 1).to_bits(), 12.5_f32.to_bits());
}

/// A bank fallback never hides a malformed or incompatible request.
///
/// Rewritten on `miso_engine_lane::Backend`: the "unavailable backend" is a width this build was
/// not compiled for, and the property under test is unchanged — validation happens **before** any
/// `Ok(None)`.
///
/// Red mutation: move the `Backend::current().width() != lanes` check above the per-request
/// validation loop in `bind_homogeneous_bank` — RED on the first case.
#[test]
fn bank_fallback_never_hides_malformed_or_incompatible_requests() {
    let factory = CompressorFactory;
    // A width this build cannot run, chosen so the fallback path is the one under test.
    let (backend, width) = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        (Backend::Simd4, BankWidth::Four)
    } else {
        (Backend::Simd8, BankWidth::Eight)
    };
    let lanes = width.lanes() as usize;

    let mut malformed = vec![initial_values(); lanes];
    malformed[lanes - 1][0].value = f32::NAN;
    let malformed_requests = malformed.iter().map(|v| request(v)).collect::<Vec<_>>();
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &malformed_requests,
            })
            .err()
            .expect("malformed values must not be hidden")
            .code,
        "effect.parameter.initial"
    );

    let connected_values = vec![initial_values(); lanes];
    let mut connected = connected_values
        .iter()
        .map(|v| request(v))
        .collect::<Vec<_>>();
    for item in &mut connected {
        item.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &connected,
            })
            .expect("connected fallback")
            .is_none()
    );
    connected[lanes - 1].limits.maximum_total_state_bytes -= 1;
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &connected,
            })
            .err()
            .expect("connected fallback still validates every request")
            .code,
        "effect.resource.limit"
    );

    let heterogeneous_values = vec![initial_values(); lanes];
    let mut heterogeneous = heterogeneous_values
        .iter()
        .map(|v| request(v))
        .collect::<Vec<_>>();
    heterogeneous[lanes - 1].bypass = true;
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &heterogeneous,
            })
            .expect("heterogeneous fallback")
            .is_none()
    );

    // A backend and a width that do not describe the same lane count is malformed, not a fallback.
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::Scalar,
                width,
                requests: &heterogeneous,
            })
            .err()
            .expect("mismatched backend and width")
            .code,
        "effect.bank.requests"
    );
}

/// The three link laws are exact, and a connected sidechain detects something different from the
/// main input.
#[test]
fn links_are_exact_and_connected_sidechain_is_distinct_from_main_detection() {
    // 0.5 * |l| + 0.5 * |r| in the frozen product order, checked through the rendered output of a
    // configuration whose gain is a pure function of the detector level.
    let values = values_with(&[
        (0, -40.0),
        (1, 20.0),
        (2, 0.0),
        (3, 0.1),
        (6, 1.0),
        (7, 20.0),
    ]);

    let mut outputs = Vec::new();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let mut preparation = request(&values);
        preparation.link_mode = link;
        let mut effect = prepare(preparation);
        let mut left = vec![0.25_f32; 1_024];
        let mut right = vec![0.75_f32; 1_024];
        render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
        outputs.push((left[1_000], right[1_000]));
    }
    // Dual mono: the quieter channel is compressed less than the louder one.
    assert!(outputs[0].0 / 0.25 > outputs[0].1 / 0.75);
    // Maximum: both channels ride the louder one, so both have the louder one's gain.
    assert!((outputs[1].0 / 0.25 - outputs[1].1 / 0.75).abs() < 1.0e-6);
    assert!(outputs[1].0 / 0.25 < outputs[0].0 / 0.25);
    // Average: 0.5*0.25 + 0.5*0.75 = 0.5, between the two, and again equal on both channels.
    assert!((outputs[2].0 / 0.25 - outputs[2].1 / 0.75).abs() < 1.0e-6);
    assert!(outputs[2].0 / 0.25 > outputs[1].0 / 0.25);

    // A connected but silent sidechain detects silence, so the output is the dry signal exactly.
    let mut connected_request = request(&values);
    connected_request.ports.sidechain = PreparedSidechainPort::Connected {
        id: sidechain_port(),
        required: false,
    };
    let mut connected = prepare(connected_request);
    let mut left = vec![0.25_f32; 1_024];
    let mut right = vec![0.25_f32; 1_024];
    let sidechain_left = vec![0.0_f32; 128];
    let sidechain_right = vec![0.0_f32; 128];
    let mut offset = 0;
    while offset < left.len() {
        connected.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                Some((&sidechain_left, &sidechain_right)),
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        offset += 128;
    }
    assert_eq!(left[1_000].to_bits(), 0.25_f32.to_bits());
    assert_eq!(right[1_000].to_bits(), 0.25_f32.to_bits());
}

/// The bank's per-block guard rejects a malformed block before it indexes anything.
///
/// The pre-audit guard was four inline conditions that accepted `frames == 0`, never checked the
/// slice lengths and indexed `automation_offsets` before checking them. 83c deferred the shared
/// validator to #95; this is the strengthened form in the meantime. A rejected block returns an
/// empty report and leaves the buffers untouched — it never panics and never renders.
///
/// Red mutation: `offsets_are_ordered` returning `true` unconditionally — RED (index out of
/// bounds), which is exactly the failure mode the check exists to prevent.
#[test]
fn a_malformed_bank_block_is_rejected_before_it_is_indexed() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("scalar-only build: no bank to guard");
        return;
    };
    let lanes = width.lanes() as usize;
    let values = vec![initial_values(); lanes];
    let requests: Vec<_> = values.iter().map(|v| request(v)).collect();
    let mut bank = support::bind_bank(&requests).expect("bank");

    let frames = 64_u32;
    let mut left = vec![0.5_f32; frames as usize * lanes];
    let mut right = vec![0.5_f32; frames as usize * lanes];
    let good_offsets = vec![0_u32; lanes + 1];

    // Offsets that run past the end of the span slice.
    let mut bad_offsets = vec![0_u32; lanes + 1];
    bad_offsets[lanes] = 4;
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &bad_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));
    assert!(
        left.iter()
            .all(|sample| sample.to_bits() == 0.5_f32.to_bits())
    );

    // Descending offsets.
    let mut descending = vec![0_u32; lanes + 1];
    descending[0] = 1;
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &descending,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // Zero frames.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames: 0,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // A slice that does not match `frames * lanes`.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames: frames + 1,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // Too few offsets to describe the lanes.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets[..lanes],
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // And a well-formed block still renders.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));
    assert!(
        left.iter()
            .all(|sample| sample.to_bits() == 0.0_f32.to_bits())
    );
}
