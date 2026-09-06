//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! 882 samples of lookahead at 44.1 kHz, a linked detector and a lookahead ring whose write index advances on silence.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
mod support;

use std::hint::black_box;

use bench_support::alloc as bench_alloc;
use effect_contract::{EffectBankProcessBlock, EffectProcessBlock};

conformance::effect_conformance_test!(compressor::CompressorFactory);

/// The installed allocator is live for both allocation and free, while repeated production
/// renders through the uniform staged, uniform D=0 and ragged bank paths move neither counter.
#[test]
fn uniform_and_ragged_render_paths_allocate_and_free_nothing() {
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let liveness_mark = bench_alloc::counters();
    let probe = black_box(vec![0_u8; 4_096]);
    drop(probe);
    let liveness = bench_alloc::delta_since(liveness_mark);
    assert!(liveness.allocations > 0, "allocation counter is not live");
    assert!(liveness.deallocations > 0, "free counter is not live");

    let staged_values = support::values_with(&[(0, -24.0), (1, 4.0), (7, 0.0)]);
    let live_values = support::values_with(&[(0, -24.0), (1, 4.0), (7, 20.0)]);
    let mut staged = support::prepare(support::request(&staged_values));
    let mut live = support::prepare(support::request(&live_values));
    let mut staged_left = [0.375_f32; 128];
    let mut staged_right = [-0.625_f32; 128];
    let mut live_left = [-0.0_f32; 128];
    let mut live_right = [0.5_f32; 128];

    let (bank_width, bank_lanes, mut ragged, mut ragged_left, mut ragged_right, offsets) = {
        let (_, width) = support::native_bank_width().expect("native bank required by this gate");
        let lanes = width.lanes() as usize;
        let values: Vec<_> = (0..lanes)
            .map(|lane| {
                support::values_with(&[
                    (0, -20.0 - lane as f32),
                    (1, 3.0 + lane as f32 * 0.25),
                    (7, 2.5 * lane as f32),
                ])
            })
            .collect();
        let requests: Vec<_> = values
            .iter()
            .map(|values| support::request(values))
            .collect();
        let bank = support::bind_bank(&requests).expect("ragged bank");
        (
            width,
            lanes,
            bank,
            vec![0.25_f32; 128 * lanes],
            vec![-0.75_f32; 128 * lanes],
            vec![0_u32; lanes + 1],
        )
    };

    let mark = bench_alloc::counters();
    for block in 0..32_u64 {
        staged.process(
            EffectProcessBlock::new(
                &mut staged_left,
                &mut staged_right,
                None,
                block * 128,
                &[],
                128,
            )
            .expect("staged block"),
        );
        live.process(
            EffectProcessBlock::new(&mut live_left, &mut live_right, None, block * 128, &[], 128)
                .expect("D=0 block"),
        );
        ragged.process_bank(
            EffectBankProcessBlock::new(
                &mut ragged_left,
                &mut ragged_right,
                None,
                128,
                bank_width,
                block * 128,
                &[],
                &offsets,
                128,
            )
            .expect("ragged block"),
        );
    }
    black_box((
        &staged_left,
        &staged_right,
        &live_left,
        &live_right,
        &ragged_left,
        &ragged_right,
        bank_lanes,
    ));
    let render = bench_alloc::delta_since(mark);
    assert_eq!(render.allocations, 0, "render allocated");
    assert_eq!(render.deallocations, 0, "render freed heap storage");
    assert_eq!(render.reallocations, 0, "render reallocated");
}
