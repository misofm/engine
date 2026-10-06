//! E13 — the current causal compressor contract.
//!
//! Descriptor, resource, zero-latency, causal sample-zero, bank-fallback, link, sidechain and
//! malformed-block behavior are asserted here. The contract intentionally changes the former
//! lookahead payload and parameter menu under issue #737; root-owned current fixture pins move with
//! that amended descriptor.

mod support;

use compressor::{COMPRESSOR_DESCRIPTOR, COMPRESSOR_PARAMETERS, CompressorFactory};
use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectProcessBlock, LatencySamples,
    LinkMode, NativeEffectFactory, PrepareEffectBankRequest, PreparedSidechainPort,
    expected_prepared_metadata, validate_descriptor,
};
use lane::Backend;

use support::{
    PARAMETER_COUNT, STATE_HEADER_WORDS, initial_values, prepare, render_scalar, request,
    sidechain_port, values_with,
};

/// Descriptor rows, latency, payload sizes, scratch and the resource envelope are frozen.
///
/// Red mutation: `scratch_fixed_bytes: 0` or `STATE_HEADER_WORDS = 26` — RED here, which is the
/// point: the causal resource envelope and exact 37-word channel payload (#1278) are
/// both contract data.
#[test]
fn descriptor_rows_and_resource_envelope_are_frozen() {
    validate_descriptor(&COMPRESSOR_DESCRIPTOR).expect("descriptor");
    assert_eq!(COMPRESSOR_DESCRIPTOR.id.as_str(), "miso.compressor");
    assert_eq!(COMPRESSOR_DESCRIPTOR.state_layout_version, 1);
    assert_eq!(COMPRESSOR_PARAMETERS.len(), PARAMETER_COUNT);
    for (quality, (rate, latency, lane_bytes, total_bytes)) in
        COMPRESSOR_DESCRIPTOR.qualities.iter().zip([
            (44_100_u32, 0_u64, 148_u32, 296_u64),
            (48_000, 0, 148, 296),
            (88_200, 0, 148, 296),
            (96_000, 0, 148, 296),
        ])
    {
        assert_eq!(quality.sample_rate, rate);
        assert_eq!(quality.latency, LatencySamples(latency));
        assert_eq!(quality.maximum_state.common_bytes, 0);
        assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.total(), Some(total_bytes));
        assert_eq!(quality.scratch_fixed_bytes, 64);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
        assert_eq!(lane_bytes as usize, STATE_HEADER_WORDS * 4);
    }
}

/// Every launch rate takes the same direct current-sample path in scalar and the supported native
/// bank. The identity configurations use a sample-zero impulse so a delayed implementation cannot
/// satisfy the bypass, ratio-one, or mix-zero assertions by returning a later sample.
#[test]
fn every_launch_rate_processes_scalar_and_supported_bank_at_zero_latency() {
    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    let factory = CompressorFactory;
    let active_values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (3, 0.1), (6, 1.0)]);
    let identity_values = values_with(&[(0, -40.0), (1, 1.0), (2, 0.0), (5, 0.0), (6, 1.0)]);
    let mix_zero_values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 0.0)]);

    for rate in RATES {
        let mut active_request = request(&active_values);
        active_request.sample_rate = rate;
        active_request.tail_bound = conformance::tail_bound_of(
            Box::new(compressor::CompressorFactory),
            active_request.sample_rate,
            active_request.quality,
        );
        let prepared = support::prepare_with_metadata(active_request);
        assert_eq!(prepared.metadata.latency, LatencySamples(0));
        let mut active = prepared.processor;
        let mut left = vec![0.0_f32; 128];
        let mut right = vec![0.0_f32; 128];
        left[0] = 0.75;
        right[0] = -0.5;
        render_scalar(active.as_mut(), &mut left, &mut right, 128, 128, &[]);
        assert!(left[0].is_finite() && left[0].to_bits() != 0.75_f32.to_bits());
        assert!(right[0].is_finite() && right[0].to_bits() != (-0.5_f32).to_bits());

        let mut bypass_request = request(&active_values);
        bypass_request.sample_rate = rate;
        bypass_request.tail_bound = conformance::tail_bound_of(
            Box::new(compressor::CompressorFactory),
            bypass_request.sample_rate,
            bypass_request.quality,
        );
        bypass_request.bypass = true;
        let mut bypass = prepare(bypass_request);
        let mut bypass_left = vec![0.0_f32; 128];
        let mut bypass_right = vec![0.0_f32; 128];
        bypass_left[0] = 0.75;
        bypass_right[0] = -0.5;
        render_scalar(
            bypass.as_mut(),
            &mut bypass_left,
            &mut bypass_right,
            128,
            128,
            &[],
        );
        assert_eq!(bypass_left[0].to_bits(), 0.75_f32.to_bits());
        assert_eq!(bypass_right[0].to_bits(), (-0.5_f32).to_bits());

        for (name, values) in [
            ("ratio-one", &identity_values),
            ("mix-zero", &mix_zero_values),
        ] {
            let mut identity_request = request(values);
            identity_request.sample_rate = rate;
            identity_request.tail_bound = conformance::tail_bound_of(
                Box::new(compressor::CompressorFactory),
                identity_request.sample_rate,
                identity_request.quality,
            );
            let mut identity = prepare(identity_request);
            let mut identity_left = vec![0.0_f32; 128];
            let mut identity_right = vec![0.0_f32; 128];
            identity_left[0] = 0.75;
            identity_right[0] = -0.5;
            render_scalar(
                identity.as_mut(),
                &mut identity_left,
                &mut identity_right,
                128,
                128,
                &[],
            );
            assert_eq!(
                identity_left[0].to_bits(),
                0.75_f32.to_bits(),
                "{name} left rate {rate}"
            );
            assert_eq!(
                identity_right[0].to_bits(),
                (-0.5_f32).to_bits(),
                "{name} right rate {rate}"
            );
        }

        let Some((backend, width)) = support::native_bank_width() else {
            continue;
        };
        let lanes = width.lanes() as usize;
        let requests: Vec<_> = (0..lanes)
            .map(|_| {
                let mut request = request(&active_values);
                request.sample_rate = rate;
                request.tail_bound = conformance::tail_bound_of(
                    Box::new(compressor::CompressorFactory),
                    request.sample_rate,
                    request.quality,
                );
                request
            })
            .collect();
        let mut bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("supported bank bind")
            .expect("supported bank")
            .processor;
        let mut bank_left = vec![0.75_f32; lanes];
        let mut bank_right = vec![-0.5_f32; lanes];
        let offsets = vec![0_u32; lanes + 1];
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                1,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank impulse"),
        );
        assert!(
            bank_left
                .iter()
                .any(|sample| sample.to_bits() != 0.75_f32.to_bits())
        );

        let bypass_requests: Vec<_> = (0..lanes)
            .map(|_| {
                let mut request = request(&active_values);
                request.sample_rate = rate;
                request.tail_bound = conformance::tail_bound_of(
                    Box::new(compressor::CompressorFactory),
                    request.sample_rate,
                    request.quality,
                );
                request.bypass = true;
                request
            })
            .collect();
        let mut bypass_bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &bypass_requests,
                active_mask: width.full_mask(),
            })
            .expect("supported bypass bank bind")
            .expect("supported bypass bank")
            .processor;
        let mut bypass_bank_left = vec![0.75_f32; lanes];
        let mut bypass_bank_right = vec![-0.5_f32; lanes];
        bypass_bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bypass_bank_left,
                &mut bypass_bank_right,
                None,
                1,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank bypass impulse"),
        );
        assert!(
            bypass_bank_left
                .iter()
                .all(|sample| sample.to_bits() == 0.75_f32.to_bits())
        );
        assert!(
            bypass_bank_right
                .iter()
                .all(|sample| sample.to_bits() == (-0.5_f32).to_bits())
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
    for (index, parameter) in COMPRESSOR_PARAMETERS.iter().enumerate() {
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
    for index in [2_usize, 5, 6] {
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
        effect.metadata.latency,
        expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, request(&values))
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

/// A bank fallback never hides a malformed or incompatible request.
///
/// Rewritten on `lane::Backend`: the "unavailable backend" is a width this build was
/// not compiled for, and the property under test is unchanged — validation happens **before** any
/// `Ok(None)`.
///
/// Red mutation: move the `Backend::current().width() != lanes` check above the per-request
/// validation loop in `bind_homogeneous_bank` — RED on the first case.
#[test]
#[cfg_attr(
    not(target_feature = "avx2"),
    ignore = "a 4-lane (NEON/simd128) build has no vector width it cannot run (#1112)"
)]
fn bank_fallback_never_hides_malformed_or_incompatible_requests() {
    let factory = CompressorFactory;
    // A width this build cannot run, chosen so the fallback path is the one under test: four
    // lanes in the 8-lane (AVX2) build.
    let backend = *Backend::VECTOR
        .iter()
        .find(|&&backend| backend != Backend::current())
        .expect("a build with a second vector width");
    let width = BankWidth::for_backend(backend).expect("a vector backend");
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
                active_mask: width.full_mask(),
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
                active_mask: width.full_mask(),
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
                active_mask: width.full_mask(),
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
                active_mask: width.full_mask(),
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
                active_mask: width.full_mask(),
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
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (3, 0.1), (6, 1.0)]);

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
            .any(|sample| sample.to_bits() != 0.5_f32.to_bits()),
        "a well-formed bank block must render rather than leave the input untouched"
    );
}

/// Issue #1090 (console strip P2c): the compressor accepts the padding contract. A padded request
/// binds at every active count, and only after every lane's request has been validated.
///
/// The malformed request is a **non-first** member (lane 1), because lane 0's request is read
/// before the member loop: a malformed lane 0 is refused even when a fallback has been hoisted
/// above the loop, so it cannot tell. Each fallback is tried with a padded mask: a width this
/// build does not execute, and a connected sidechain.
///
/// Red if the #1088 guard comes back (a padded request is declined); if any fallback (the #1088
/// guard, the width gate, the sidechain gate) is hoisted above the member loop, where it declines
/// a padded request with a malformed lane 1 instead of refusing it; if a padded lane's own request
/// is not validated; or if a padded lane whose request is not the members' program binds. That lane
/// would run another program under the bank's one metadata, so it is the heterogeneous `Ok(None)`.
#[test]
fn a_padded_request_binds_after_every_lane_is_validated() {
    let backend = lane::Backend::current();
    let Some(width) = BankWidth::for_backend(backend) else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = initial_values();
    let requests = vec![request(&values); lanes];
    let bind = |backend: Backend,
                width: BankWidth,
                requests: &[effect_contract::PrepareEffectRequest<'_>],
                mask: &[bool]| {
        CompressorFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
            active_mask: mask,
        })
    };
    let prefix = |members: usize, lanes: usize| -> Vec<bool> {
        (0..lanes).map(|lane| lane < members).collect()
    };
    assert!(
        bind(backend, width, &requests, width.full_mask())
            .expect("a full bank")
            .is_some(),
        "the control: the same members bind as a full bank"
    );
    for members in 1..lanes {
        let bank = bind(backend, width, &requests, &prefix(members, lanes))
            .expect("a padded request is well formed")
            .unwrap_or_else(|| panic!("{members} of {lanes} members must bind"));
        assert_eq!(bank.metadata.width, width);
    }
    // P2a's verdict L3: members come first, and any other mask is malformed.
    let mut scattered = prefix(1, lanes);
    scattered.swap(0, lanes - 1);
    assert_eq!(
        bind(backend, width, &requests, &scattered)
            .err()
            .map(|error| error.code),
        Some("effect.bank.mask_not_prefix")
    );

    let mut malformed_values = initial_values();
    malformed_values[0].value = f32::NAN;
    let refusal = CompressorFactory
        .prepare(request(&malformed_values))
        .err()
        .expect("a malformed request")
        .code;
    let mut malformed = requests.clone();
    malformed[1] = request(&malformed_values);
    let two_members = prefix(2, lanes);
    assert_eq!(
        bind(backend, width, &malformed, &two_members)
            .err()
            .map(|error| error.code),
        Some(refusal),
        "a padded request validates a non-first member"
    );
    let one_member = prefix(1, lanes);
    assert_eq!(
        bind(backend, width, &malformed, &one_member)
            .err()
            .map(|error| error.code),
        Some(refusal),
        "a padded lane's request is validated like a member's"
    );
    // Every width this build does not execute: the fallback comes after the member loop. Four
    // lanes in the 8-lane (AVX2) build; a 4-lane (NEON/simd128) build has none (#1112).
    for &other_backend in Backend::VECTOR {
        if other_backend == Backend::current() {
            continue;
        }
        let other_width = BankWidth::for_backend(other_backend).expect("a vector backend");
        let other_lanes = other_width.lanes() as usize;
        let mut other = vec![request(&values); other_lanes];
        other[1] = request(&malformed_values);
        assert_eq!(
            bind(other_backend, other_width, &other, &prefix(2, other_lanes))
                .err()
                .map(|error| error.code),
            Some(refusal),
            "an unavailable width never hides a malformed non-first member"
        );
    }
    // A connected sidechain: the same.
    let mut connected = malformed.clone();
    for item in &mut connected {
        item.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    assert_eq!(
        bind(backend, width, &connected, &two_members)
            .err()
            .map(|error| error.code),
        Some(refusal),
        "a connected sidechain never hides a malformed non-first member"
    );
    connected[1] = connected[0];
    assert!(
        bind(backend, width, &connected, &two_members)
            .expect("a well-formed keyed request")
            .is_none(),
        "the control: the well-formed keyed request is declined, not refused"
    );

    let mut foreign = requests.clone();
    foreign[lanes - 1].link_mode = LinkMode::Maximum;
    assert!(
        bind(backend, width, &foreign, &prefix(lanes - 1, lanes))
            .expect("a well-formed request")
            .is_none(),
        "a padded lane that is not a clone of the members' program is declined"
    );
}
