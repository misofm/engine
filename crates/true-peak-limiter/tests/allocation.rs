//! A1: the render path allocates nothing.
//!
//! The audited allocator's current-thread counters watch `process` and `process_bank` over a
//! hundred blocks that include automation, a boundary-check failure and both resets, and a padded
//! bank whose failures recover one lane (#1091). The limiter allocates at preparation -- control
//! plane -- and never again: since #1278 a restore validates and commits in place, and the
//! randomized differential (`tests/randomized.rs`) audits every payload call. Before #90 the
//! render path was allocation-free too, and this gate is what keeps it so now that the arena, the
//! ramps and the payload codec all changed hands.

use bench_support::alloc;
use lane::Backend;

use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedPorts, PreparedSidechainPort, ResetKind,
};
use true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR, TRUE_PEAK_LIMITER_PARAMETERS, TruePeakLimiterFactory,
};

fn measure(operation: impl FnOnce()) -> (u64, u64) {
    alloc::assert_installed();
    let mark = alloc::current_thread_counters();
    operation();
    let delta = alloc::current_thread_delta_since(mark);
    (delta.allocations, delta.deallocations)
}

#[test]
fn the_own_thread_allocator_observes_an_allocate_and_free() {
    let (allocations, deallocations) = measure(|| {
        let mut bytes = Vec::with_capacity(64);
        bytes.extend_from_slice(&[0xA5; 64]);
        std::hint::black_box(&bytes);
        drop(bytes);
    });

    assert!(
        allocations > 0,
        "positive allocator control did not allocate"
    );
    assert!(
        deallocations > 0,
        "positive allocator control did not deallocate"
    );
    assert_eq!(
        allocations, deallocations,
        "positive allocator control leaked"
    );
}

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRUE_PEAK_LIMITER_PARAMETERS[index / 2].default_value,
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
        quantum: 128,
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

fn automation(block: usize) -> [PreparedAutomationSpan; 1] {
    let value = if block.is_multiple_of(2) { -3.0 } else { -9.0 };
    [PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Left,
        parameter_index: 0,
        start_sample: (block * 128) as u64,
        end_sample: (block * 128) as u64,
        start_value: value,
        end_value: value,
    }]
}

#[test]
fn the_render_path_allocates_nothing() {
    let values = values();
    let mut scalar = TruePeakLimiterFactory
        .prepare(request(&values))
        .expect("prepare");
    let mut left = vec![0.0_f32; 128];
    let mut right = vec![0.0_f32; 128];
    // Warm every lazily initialised path once before arming the counter.
    scalar
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"));

    let (allocations, deallocations) = measure(|| {
        for block in 0..100 {
            for (index, sample) in left.iter_mut().enumerate() {
                // A NaN every twentieth block: it reaches the output a lookahead later, where the
                // once-per-block boundary check zeroes, resets and counts — still without a heap.
                // (#1091: the `3.0e38` this used to feed never failed the check. The detector's
                // infinite peak asks for a gain of zero, and `g <= r` delivers it on that sample.)
                *sample = if block % 20 == 19 {
                    f32::NAN
                } else {
                    ((index % 17) as f32 - 8.0) * 0.25
                };
            }
            right.copy_from_slice(&left);
            let spans = automation(block);
            scalar.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * 128) as u64,
                    &spans,
                    128,
                )
                .expect("block"),
            );
            if block % 33 == 32 {
                scalar.reset(ResetKind::DiscontinuityKeepParameters);
            }
        }
        scalar.reset(ResetKind::FullToDefaults);
    });
    assert_eq!(
        (allocations, deallocations),
        (0, 0),
        "scalar render path allocated"
    );

    // A bank of this build's native width: eight lanes on x86-64-v3, four on AArch64 NEON (#1017).
    let backend = Backend::current();
    let width = BankWidth::for_backend(backend).expect("every product target has a bank width");
    let lanes = width.lanes() as usize;
    let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes).map(|_| request(&values)).collect();
    let mut bank = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("bank binding")
        .expect("bank available");
    let mut left = vec![0.0_f32; 128 * lanes];
    let mut right = vec![0.0_f32; 128 * lanes];
    // Lane 0 carries the one span; every other lane carries none.
    let offsets: Vec<u32> = (0..=lanes).map(|lane| u32::from(lane > 0)).collect();
    let spans = automation(0);
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left, &mut right, None, 128, width, 0, &spans, &offsets, 128,
        )
        .expect("bank block"),
    );

    let (allocations, deallocations) = measure(|| {
        for block in 0..100 {
            for (index, sample) in left.iter_mut().enumerate() {
                *sample = if block % 20 == 19 {
                    f32::NAN
                } else {
                    ((index % 23) as f32 - 11.0) * 0.2
                };
            }
            right.copy_from_slice(&left);
            let spans = automation(block);
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    128,
                    width,
                    (block * 128) as u64,
                    &spans,
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            if block % 33 == 32 {
                bank.reset(ResetKind::DiscontinuityKeepParameters);
            }
        }
        bank.reset(ResetKind::FullToDefaults);
    });
    assert_eq!(
        (allocations, deallocations),
        (0, 0),
        "bank render path allocated"
    );

    let mut mono_bank = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("mono bank binding")
        .expect("mono bank available");
    let mut mono_left = vec![0.0_f32; 128 * lanes];
    let mut mono_right = vec![0.0_f32; 128 * lanes];
    mono_bank.process_bank_mono(
        EffectBankProcessBlock::new(
            &mut mono_left,
            &mut mono_right,
            None,
            128,
            width,
            0,
            &spans,
            &offsets,
            128,
        )
        .expect("mono bank block"),
    );

    let (allocations, deallocations) = measure(|| {
        for block in 0..100 {
            for (index, sample) in mono_left.iter_mut().enumerate() {
                *sample = if block % 20 == 19 {
                    f32::NAN
                } else {
                    ((index % 23) as f32 - 11.0) * 0.2
                };
            }
            mono_right.copy_from_slice(&mono_left);
            let spans = automation(block);
            mono_bank.process_bank_mono(
                EffectBankProcessBlock::new(
                    &mut mono_left,
                    &mut mono_right,
                    None,
                    128,
                    width,
                    (block * 128) as u64,
                    &spans,
                    &offsets,
                    128,
                )
                .expect("mono bank block"),
            );
            if block % 33 == 32 {
                mono_bank.reset(ResetKind::DiscontinuityKeepParameters);
            }
        }
        mono_bank.reset(ResetKind::FullToDefaults);
    });
    assert_eq!(
        (allocations, deallocations),
        (0, 0),
        "mono bank render path allocated"
    );

    // Issue #1091: a padded bank, and its per-lane §4.4 recovery. Every lane but the last carries
    // a member and the last a clone, fed `+0.0`; only lane 0 is hostile, so each failure recovers
    // one lane and leaves the others running, which is the path a full bank's all-lanes failure
    // above never takes.
    let mask: Vec<bool> = (0..lanes).map(|lane| lane + 1 < lanes).collect();
    for mono in [false, true] {
        let mut padded = TruePeakLimiterFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: &mask,
            })
            .expect("padded bank binding")
            .expect("padded bank available");
        let mut left = vec![0.0_f32; 128 * lanes];
        let mut right = vec![0.0_f32; 128 * lanes];
        let fill = |left: &mut [f32], right: &mut [f32], block: usize| {
            for (index, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
                let lane = index % lanes;
                let value = if lane + 1 == lanes {
                    0.0
                } else if lane == 0 && block % 20 == 19 {
                    f32::NAN
                } else {
                    ((index % 23) as f32 - 11.0) * 0.2
                };
                *left = value;
                *right = value;
            }
        };
        let mut render = |left: &mut [f32], right: &mut [f32], block: usize| {
            let spans = automation(block);
            let request = EffectBankProcessBlock::new(
                left,
                right,
                None,
                128,
                width,
                (block * 128) as u64,
                &spans,
                &offsets,
                128,
            )
            .expect("padded bank block");
            if mono {
                padded.process_bank_mono(request);
            } else {
                padded.process_bank(request);
            }
        };
        fill(&mut left, &mut right, 0);
        render(&mut left, &mut right, 0);
        let (allocations, deallocations) = measure(|| {
            for block in 1..100 {
                fill(&mut left, &mut right, block);
                render(&mut left, &mut right, block);
            }
        });
        assert_eq!(
            (allocations, deallocations),
            (0, 0),
            "padded bank render path allocated (mono {mono})"
        );
    }
}
