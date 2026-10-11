//! E6: the render path, both resets and both state-payload directions allocate nothing.
//!
//! Version 1's `reset(FullToDefaults)` rebuilt boxed history per lane and its restore replaced the
//! boxes instead of writing into them (#94 F9). `reset` is a `PreparedNativeEffect` method owned by
//! the realtime plane, so that was a latent violation of the AGENTS.md rule that render performs no
//! allocation. Everything below now allocates only in `prepare`.

mod support;

use bench_support::alloc::{self, Counters};
use std::hint::black_box;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, NativeEffectFactory, ParameterChannel,
    PreparedAutomationSpan, ResetKind, StatePayloadInput, StatePayloadOutput,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{new_sections, point, process, request_with, varied_values};

/// Counts allocation/free events on the calling thread. The audited allocation count includes
/// reallocations, so a realloc also makes a zero-event assertion fail.
fn events(operation: impl FnOnce()) -> u64 {
    let counts = allocator_counts(operation);
    counts.allocations + counts.deallocations
}

/// Warms the audited allocator before measuring only this thread's operation.
fn allocator_counts(operation: impl FnOnce()) -> Counters {
    alloc::assert_installed();
    let mark = alloc::current_thread_counters();
    operation();
    alloc::current_thread_delta_since(mark)
}

#[test]
fn audited_allocator_proves_own_thread_allocation_and_free() {
    let counts = allocator_counts(|| {
        let value = Box::new([black_box(0x5Au8); 64]);
        black_box(value.as_ptr());
        black_box(&value[..]);
        drop(value);
    });
    assert!(
        counts.allocations > 0,
        "own-thread control saw no allocation"
    );
    assert!(counts.deallocations > 0, "own-thread control saw no free");
}

#[test]
fn the_scalar_render_path_allocates_nothing() {
    let initial = varied_values(1);
    let prepared = MultibandCompressorFactory
        .prepare(request_with(&initial, LinkMode::Maximum, 128, false))
        .expect("prepare");
    let sizes = prepared.metadata.state_sizes;
    let mut effect = prepared.processor;
    let mut left = support::signal(128, 0x0BAD_C0DE);
    let mut right = support::signal(128, 0x0BAD_BEEF);
    let spans = [point(1, ParameterChannel::Left, 128, -30.0)];
    // Warm the allocator's own lazy state outside the measured region.
    process(effect.as_mut(), &mut left, &mut right, 0, &[], 128);
    let mut sections = new_sections(sizes);

    let counted = events(|| {
        process(effect.as_mut(), &mut left, &mut right, 128, &spans, 128);
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        process(effect.as_mut(), &mut left, &mut right, 256, &[], 128);
        effect.reset(ResetKind::FullToDefaults);
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&sections.0, &sections.1, &sections.2, sizes)
                    .expect("payload"),
            )
            .expect("restore");
    });
    assert_eq!(
        counted, 0,
        "the scalar render path allocated {counted} times"
    );
}

#[test]
fn the_bank_render_path_allocates_nothing() {
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        {
            let sets = (0..lanes).map(varied_values).collect::<Vec<_>>();
            let requests = sets
                .iter()
                .map(|set| request_with(set, LinkMode::Average, 128, false))
                .collect::<Vec<_>>();
            let prepared = support::bank(width, &requests);
            let sizes = prepared.metadata.program_key.state_sizes;
            let mut bank = prepared.processor;
            let mut left = support::signal(128 * lanes, 0x00C0_FFEE);
            let mut right = support::signal(128 * lanes, 0x00DE_CAF0);
            let offsets = vec![0u32; lanes + 1];
            let automation = [point(1, ParameterChannel::Left, 128, -30.0)];
            let mut automation_offsets = vec![0u32; lanes + 1];
            automation_offsets[1..].fill(1);
            let mut sections = new_sections(sizes);
            let run = |bank: &mut dyn effect_contract::PreparedNativeEffectBank,
                       left: &mut [f32],
                       right: &mut [f32],
                       first: u64,
                       spans: &[PreparedAutomationSpan],
                       offsets: &[u32]| {
                bank.process_bank(
                    EffectBankProcessBlock::new(
                        left, right, None, 128, width, first, spans, offsets, 128,
                    )
                    .expect("bank block"),
                );
            };
            run(bank.as_mut(), &mut left, &mut right, 0, &[], &offsets);

            let counted = events(|| {
                run(
                    bank.as_mut(),
                    &mut left,
                    &mut right,
                    128,
                    &automation,
                    &automation_offsets,
                );
                bank.reset(ResetKind::DiscontinuityKeepParameters);
                bank.reset(ResetKind::FullToDefaults);
                run(bank.as_mut(), &mut left, &mut right, 256, &[], &offsets);
                bank.snapshot_track_state_payload(
                    1,
                    StatePayloadOutput::new(
                        &mut sections.0,
                        &mut sections.1,
                        &mut sections.2,
                        sizes,
                    )
                    .expect("payload"),
                )
                .expect("snapshot");
                bank.restore_track_state_payload(
                    1,
                    1,
                    StatePayloadInput::new(&sections.0, &sections.1, &sections.2, sizes)
                        .expect("payload"),
                )
                .expect("restore");
            });
            assert_eq!(counted, 0, "{width:?} allocated {counted} times");
        }
    }
}
