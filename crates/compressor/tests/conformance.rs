//! Effect-contract conformance (issue 011 harness) against the production factory.
//!
//! A linked detector, causal current-sample processing, and a zero-allocation render path.
//!
//! Issue #105 phase 2 F1: the harness runs against every production `NativeEffectFactory`, not
//! just its own reference mock. The whole test is the macro -- see
//! `conformance::effect_conformance_test!` for what it gates and why the two
//! dev-dependencies are load-bearing.
mod support;

use std::hint::black_box;
use std::sync::{Arc, Barrier};

use bench_support::alloc as bench_alloc;
use effect_contract::{EffectBankProcessBlock, EffectProcessBlock, ParameterChannel};
use engine::realtime::audit;

conformance::effect_conformance_test!(compressor::CompressorFactory);

/// The allocator hooks feed the same thread-local audit used by the render guard. A deliberately
/// counted heap probe proves positive same-thread attribution (capacity growth uses realloc, which
/// the existing allocator records as an Allocation) and a worker allocation proves that unrelated
/// coordinating-thread activity cannot enter the render thread's snapshot.
#[test]
fn scoped_allocator_attribution_controls_are_live_and_isolated() {
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    audit::warm_up();
    audit::reset();

    let (allocated, grown, freed) = audit::in_render_scope(|| {
        let mut probe = Vec::with_capacity(1);
        let allocated = audit::snapshot();
        probe.push(1_u8);
        probe.reserve(128);
        assert!(
            probe.capacity() > 1,
            "allocation-control probe did not grow"
        );
        let grown = audit::snapshot();
        black_box(&probe);
        drop(probe);
        let freed = audit::snapshot();
        (allocated, grown, freed)
    });
    assert!(
        allocated.allocations > 0,
        "same-thread allocation was not audited"
    );
    assert!(
        grown.allocations > allocated.allocations,
        "same-thread capacity growth/reallocation was not audited"
    );
    assert!(
        freed.deallocations > grown.deallocations,
        "same-thread deallocation was not audited"
    );

    audit::reset();
    let ready = Arc::new(Barrier::new(2));
    let start = Arc::new(Barrier::new(2));
    let done = Arc::new(Barrier::new(2));
    let worker_ready = Arc::clone(&ready);
    let worker_start = Arc::clone(&start);
    let worker_done = Arc::clone(&done);
    let worker = std::thread::spawn(move || {
        worker_ready.wait();
        worker_start.wait();
        let mut probe = Vec::with_capacity(1);
        probe.push(1_u8);
        probe.reserve(128);
        assert!(probe.capacity() > 1, "worker probe did not grow");
        black_box(&probe);
        drop(probe);
        worker_done.wait();
    });
    ready.wait();
    audit::in_render_scope(|| {
        start.wait();
        done.wait();
    });
    worker.join().expect("allocation-control worker");
    let other_thread = audit::snapshot();
    assert_eq!(other_thread, audit::AuditSnapshot::default());
}

/// The installed allocator is live for both allocation and free, while repeated production
/// renders through uniform and ragged bank paths move neither counter.
#[test]
fn uniform_and_ragged_render_paths_allocate_and_free_nothing() {
    const CHILD: &str = "MISO_ENGINE_COMPRESSOR_ALLOCATION_AUDIT_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg("uniform_and_ragged_render_paths_allocate_and_free_nothing")
            .arg("--nocapture")
            .env(CHILD, "1")
            .status()
            .expect("isolated allocation audit child");
        assert!(status.success(), "isolated allocation audit child failed");
        return;
    }

    bench_alloc::set_mode(bench_alloc::Mode::Count);
    bench_alloc::assert_installed();
    let liveness_mark = bench_alloc::counters();
    let probe = black_box(vec![0_u8; 4_096]);
    drop(probe);
    let liveness = bench_alloc::delta_since(liveness_mark);
    assert!(liveness.allocations > 0, "allocation counter is not live");
    assert!(liveness.deallocations > 0, "free counter is not live");

    let uniform_values = support::values_with(&[(0, -24.0), (1, 4.0)]);
    let live_values = support::values_with(&[(0, -24.0), (1, 4.0)]);
    let mut uniform = support::prepare(support::request(&uniform_values));
    let mut live = support::prepare(support::request(&live_values));
    let mut uniform_left = [0.375_f32; 128];
    let mut uniform_right = [-0.625_f32; 128];
    let mut live_left = [-0.0_f32; 128];
    let mut live_right = [0.5_f32; 128];

    let (bank_width, bank_lanes, mut ragged, mut ragged_left, mut ragged_right, offsets) = {
        let (_, width) = support::native_bank_width().expect("native bank required by this gate");
        let lanes = width.lanes() as usize;
        let values: Vec<_> = (0..lanes)
            .map(|lane| {
                support::values_with(&[(0, -20.0 - lane as f32), (1, 3.0 + lane as f32 * 0.25)])
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

    audit::warm_up();
    audit::reset();
    audit::in_render_scope(|| {
        let targets = [3.0_f32, -3.0, 6.0, -6.0];
        for block in 0..32_u64 {
            let target = targets[block as usize % targets.len()];
            assert_eq!(
                uniform.apply_parameter_point(5, ParameterChannel::Left, target),
                Ok(())
            );
            let accepted = uniform
                .parameter_state(5, ParameterChannel::Left)
                .expect("native point state");
            assert_eq!(accepted.target_value.to_bits(), target.to_bits());
            assert_eq!(
                uniform.apply_parameter_point(5, ParameterChannel::Left, f32::NAN),
                Err(effect_contract::ParameterAccessError::InvalidValue)
            );
            let after_rejection = uniform
                .parameter_state(5, ParameterChannel::Left)
                .expect("native point state");
            assert_eq!(
                after_rejection.current_value.to_bits(),
                accepted.current_value.to_bits()
            );
            assert_eq!(
                after_rejection.target_value.to_bits(),
                accepted.target_value.to_bits()
            );
            assert_eq!(
                uniform.parameter_state(u32::MAX, ParameterChannel::Right),
                Err(effect_contract::ParameterAccessError::InvalidParameterIndex)
            );
            let silent = (8..16).contains(&(block as usize));
            if silent {
                uniform_left.fill(0.0);
                uniform_right.fill(0.0);
            } else {
                uniform_left.fill(0.375);
                uniform_right.fill(-0.625);
            }
            uniform.process(
                EffectProcessBlock::new(
                    &mut uniform_left,
                    &mut uniform_right,
                    None,
                    block * 128,
                    &[],
                    128,
                )
                .expect("uniform block"),
            );
            live.process(
                EffectProcessBlock::new(
                    &mut live_left,
                    &mut live_right,
                    None,
                    block * 128,
                    &[],
                    128,
                )
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
            ragged.process_bank_mono(
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
                .expect("mono block"),
            );
        }
    });
    black_box((
        &uniform_left,
        &uniform_right,
        &live_left,
        &live_right,
        &ragged_left,
        &ragged_right,
        bank_lanes,
    ));
    let render = audit::snapshot();
    assert_eq!(render.allocations, 0, "render allocated");
    assert_eq!(render.deallocations, 0, "render freed heap storage");
    assert_eq!(render.total(), 0, "render reported forbidden operations");
}
