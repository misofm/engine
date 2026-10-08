#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E8 / A1 — the render path allocates nothing.
//!
//! `prepare` and `bind_homogeneous_bank` allocate (three histories, the ramp table, the boxed
//! instance) and that is the whole budget. After that, `process`, `process_bank`, `reset`,
//! `snapshot_*` and `restore_*` must not reach the allocator at all, because the render thread has
//! no lock, no syscall and no allocator (`AGENTS.md`, master plan §1.5).
//!
//! The shared audited allocator's current-thread counters attribute allocations, reallocations
//! and frees to the measured calls without counting work on the harness's other threads.

mod support;

use bench_support::alloc;
use effect_contract::{BankWidth, NativeEffectFactory, ParameterChannel, ResetKind};
use soft_clip::SoftClipFactory;
use support::{initial_values, prepare, prepare_bank, process, process_bank};

/// Measures this thread after checking and warming the installed allocator outside the region.
fn count(body: impl FnOnce()) -> (u64, u64) {
    alloc::assert_installed();
    let mark = alloc::current_thread_counters();
    body();
    let delta = alloc::current_thread_delta_since(mark);
    (delta.allocations, delta.deallocations)
}

#[test]
fn the_scalar_render_path_never_allocates() {
    let values = initial_values();
    let prepared = SoftClipFactory
        .prepare(support::request(&values))
        .expect("prepare");
    let sizes = prepared.metadata.state_sizes;
    let mut effect = prepared.processor;
    let mut left = vec![0.0_f32; 128];
    let mut right = vec![0.0_f32; 128];
    for (index, sample) in left.iter_mut().enumerate() {
        *sample = (index as f32 * 0.05).sin() * 0.8;
    }
    right.copy_from_slice(&left);
    let spans = [support::point(0, ParameterChannel::Left, 12.0, 0)];
    // Warm once outside the guard so nothing lazily initialises inside it.
    process(effect.as_mut(), &mut left, &mut right, 0, &spans);

    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut snapshot_left = vec![0_u8; sizes.left_bytes as usize];
    let mut snapshot_right = vec![0_u8; sizes.right_bytes as usize];

    let events = count(|| {
        let mut first_sample = 128_u64;
        for round in 0..1_000 {
            let automation: &[_] = if round % 128 == 0 { &spans } else { &[] };
            process(
                effect.as_mut(),
                &mut left,
                &mut right,
                first_sample,
                automation,
            );
            first_sample += 128;
        }
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        effect.reset(ResetKind::FullToDefaults);
        effect
            .snapshot_state_payload(
                effect_contract::StatePayloadOutput::new(
                    &mut common,
                    &mut snapshot_left,
                    &mut snapshot_right,
                    sizes,
                )
                .expect("sizes"),
            )
            .expect("snapshot");
        effect
            .restore_state_payload(
                1,
                effect_contract::StatePayloadInput {
                    common: &common,
                    left: &snapshot_left,
                    right: &snapshot_right,
                },
            )
            .expect("restore");
    });
    assert_eq!(events, (0, 0), "scalar render allocated or freed");
}

#[test]
fn the_bank_render_path_never_allocates() {
    let width = BankWidth::for_backend(lane::Backend::current()).expect("a vector build");
    let lanes = width.lanes() as usize;
    let values = initial_values();
    let per_lane: Vec<Vec<_>> = (0..lanes).map(|_| values.to_vec()).collect();
    let mut bank = prepare_bank(width, &per_lane).expect("bank binds");
    let mut left = vec![0.0_f32; 128 * lanes];
    let mut right = vec![0.0_f32; 128 * lanes];
    for (index, sample) in left.iter_mut().enumerate() {
        *sample = (index as f32 * 0.03).sin() * 0.7;
    }
    right.copy_from_slice(&left);
    let offsets = vec![0_u32; lanes + 1];
    process_bank(
        bank.as_mut(),
        width,
        &mut left,
        &mut right,
        128,
        0,
        &[],
        &offsets,
    );

    let events = count(|| {
        let mut first_sample = 128_u64;
        for _ in 0..1_000 {
            process_bank(
                bank.as_mut(),
                width,
                &mut left,
                &mut right,
                128,
                first_sample,
                &[],
                &offsets,
            );
            first_sample += 128;
        }
        bank.reset(ResetKind::DiscontinuityKeepParameters);
    });
    assert_eq!(events, (0, 0), "bank render allocated or freed");
}

/// Preparation's bounded allocation and drop prove both counters can observe allocator events.
#[test]
fn preparation_has_bounded_allocations_and_the_counter_observes_frees() {
    let values = initial_values();
    let (allocations, deallocations) = count(|| {
        let _effect = prepare(&values);
        std::hint::black_box(&_effect);
    });
    assert!(
        (1..=32).contains(&allocations),
        "prepare made {allocations} allocations"
    );
    assert!(deallocations > 0, "positive free control observed nothing");
}
