//! The identity chain renders inside an armed render scope with no forbidden operation, two
//! identically warmed chains render the same bits, and a negative-zero input leaves as `+0.0`.
//!
//! Moved here by #1026 from the retired builtins benchmark's untimed
//! `all_render_workloads_arm_only_product_render_without_timing`. Its full-chain and matrix-ramp
//! rows stay armed by `audit builtins` and its meter rows by `audit builtins-graph`; the identity
//! chain (`fixtures/builtins/v1/benchmark/identity_chain-*.toml`: every section disabled or at
//! unity) was armed nowhere else. Its two disabled sections are the arithmetic identity, and
//! their trailing `+ 0.0` normalises a negative zero (#85, class B).

use bench_support::alloc::{Mode, assert_installed, set_mode};
use builtins::{BuiltinChain, BuiltinParameters, DualMonoBlock};
use engine::realtime::audit::{self, AuditSnapshot};

const QUANTUM: usize = 128;

/// A deterministic dual-mono block whose first two frames are negative zero on both channels.
fn input(block: u64) -> ([f32; QUANTUM], [f32; QUANTUM]) {
    let mut left = [0.0; QUANTUM];
    let mut right = [0.0; QUANTUM];
    for frame in 0..QUANTUM {
        let phase = (block as usize * QUANTUM + frame) % 97;
        left[frame] = (phase as f32 - 48.0) / 64.0;
        right[frame] = (48.0 - phase as f32) / 128.0;
    }
    left[..2].fill(-0.0);
    right[..2].fill(-0.0);
    (left, right)
}

/// The identity chain's output: the input, with negative zero turned into positive zero.
fn identity(sample: f32) -> u32 {
    (sample + 0.0).to_bits()
}

#[test]
fn identity_chain_renders_audit_clean_deterministic_and_positive_zero() {
    assert_installed();
    set_mode(Mode::Count);
    audit::warm_up();
    // Non-vacuity: the armed scope sees an allocation.
    audit::reset();
    audit::in_render_scope(|| {
        let probe = Vec::<u8>::with_capacity(core::hint::black_box(64));
        core::hint::black_box(&probe);
    });
    let live = audit::snapshot();
    assert!(live.allocations > 0 && live.deallocations > 0);

    for rate_hz in [48_000, 96_000] {
        let mut chains = [
            BuiltinChain::new(rate_hz, BuiltinParameters::default()).expect("identity chain"),
            BuiltinChain::new(rate_hz, BuiltinParameters::default()).expect("identity chain"),
        ];
        for block in 0..8_u64 {
            let (source_left, source_right) = input(block);
            let mut outputs = Vec::new();
            for chain in &mut chains {
                let (mut left, mut right) = (source_left, source_right);
                let first_sample = block * QUANTUM as u64;
                audit::reset();
                audit::in_render_scope(|| {
                    chain.process_dual_mono(
                        DualMonoBlock::new(&mut left, &mut right, first_sample)
                            .expect("fixed block"),
                    );
                });
                assert_eq!(
                    audit::snapshot(),
                    AuditSnapshot::default(),
                    "{rate_hz} Hz, block {block}"
                );
                outputs.push((left.map(f32::to_bits), right.map(f32::to_bits)));
            }
            assert_eq!(outputs[0], outputs[1], "{rate_hz} Hz, block {block}");
            assert_eq!(outputs[0].0, source_left.map(identity));
            assert_eq!(outputs[0].1, source_right.map(identity));
            assert_eq!(outputs[0].0[0], 0.0_f32.to_bits());
            assert_eq!(outputs[0].1[1], 0.0_f32.to_bits());
        }
    }
}
