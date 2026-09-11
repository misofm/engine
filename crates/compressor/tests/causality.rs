//! Zero-latency causality: a future suffix cannot alter an already rendered prefix.

mod support;

use effect_contract::{EffectProcessBlock, LinkMode, PreparedSidechainPort};

use support::{initial_values, noise, prepare, request, sidechain_port};

const PREFIX: usize = 256;
const FRAMES: usize = 512;
const QUANTUM: usize = 128;

fn asymmetric_values() -> [effect_contract::InitialParameterValue; support::PARAMETER_COUNT * 2] {
    let mut values = initial_values();
    for (parameter, (left, right)) in [
        (0, (-40.0, -32.0)),
        (1, (20.0, 12.0)),
        (2, (0.0, 3.0)),
        (3, (0.1, 0.2)),
        (4, (40.0, 80.0)),
        (5, (-3.0, 4.0)),
        (6, (0.5, 0.75)),
    ] {
        values[parameter * 2].value = left;
        values[parameter * 2 + 1].value = right;
    }
    values
}

fn render(
    link_mode: LinkMode,
    connected_sidechain: bool,
    vary_main_suffix: bool,
    vary_sidechain_suffix: bool,
) -> (Vec<f32>, Vec<f32>) {
    let values = asymmetric_values();
    let make_effect = || {
        let mut preparation = request(&values);
        preparation.link_mode = link_mode;
        if connected_sidechain {
            preparation.ports.sidechain = PreparedSidechainPort::Connected {
                id: sidechain_port(),
                required: false,
            };
        }
        prepare(preparation)
    };
    let mut left_a = noise(FRAMES, 0xC4_A0_00_01, 0.8);
    let mut right_a = noise(FRAMES, 0xC4_A0_00_02, 0.7);
    let mut left_b = left_a.clone();
    let mut right_b = right_a.clone();
    let main_left_suffix = noise(FRAMES - PREFIX, 0xC4_A0_00_03, 0.8);
    let main_right_suffix = noise(FRAMES - PREFIX, 0xC4_A0_00_04, 0.7);
    if vary_main_suffix {
        left_b[PREFIX..].copy_from_slice(&main_left_suffix);
        right_b[PREFIX..].copy_from_slice(&main_right_suffix);
    } else {
        left_a[PREFIX..].copy_from_slice(&main_left_suffix);
        right_a[PREFIX..].copy_from_slice(&main_right_suffix);
        left_b[PREFIX..].copy_from_slice(&main_left_suffix);
        right_b[PREFIX..].copy_from_slice(&main_right_suffix);
    }

    let side_left_a = noise(FRAMES, 0xC4_A0_00_05, 0.9);
    let side_right_a = noise(FRAMES, 0xC4_A0_00_06, 0.6);
    let mut side_left_b = side_left_a.clone();
    let mut side_right_b = side_right_a.clone();
    if vary_sidechain_suffix {
        side_left_b[PREFIX..].fill(-0.35);
        side_right_b[PREFIX..].fill(0.85);
    }

    let render_one = |effect: &mut dyn effect_contract::PreparedNativeEffect,
                      left: &mut [f32],
                      right: &mut [f32],
                      side_left: &[f32],
                      side_right: &[f32]| {
        for offset in (0..FRAMES).step_by(QUANTUM) {
            let end = offset + QUANTUM;
            let sidechain =
                connected_sidechain.then_some((&side_left[offset..end], &side_right[offset..end]));
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    sidechain,
                    offset as u64,
                    &[],
                    QUANTUM as u32,
                )
                .expect("causal block"),
            );
        }
    };

    // The same prepared effect law is used for both streams; fresh instances isolate the
    // comparison at the common initial state.
    let mut reference = make_effect();
    let mut candidate = make_effect();
    render_one(
        reference.as_mut(),
        &mut left_a,
        &mut right_a,
        &side_left_a,
        &side_right_a,
    );
    render_one(
        candidate.as_mut(),
        &mut left_b,
        &mut right_b,
        &side_left_b,
        &side_right_b,
    );
    for (index, (actual, expected)) in left_a[..PREFIX].iter().zip(&left_b[..PREFIX]).enumerate() {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "future suffix changed left prefix sample {index} for {link_mode:?}, sidechain={connected_sidechain}, main_suffix={vary_main_suffix}, side_suffix={vary_sidechain_suffix}"
        );
    }
    for (index, (actual, expected)) in right_a[..PREFIX].iter().zip(&right_b[..PREFIX]).enumerate()
    {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "future suffix changed right prefix sample {index} for {link_mode:?}, sidechain={connected_sidechain}, main_suffix={vary_main_suffix}, side_suffix={vary_sidechain_suffix}"
        );
    }
    (left_a, right_a)
}

#[test]
fn future_main_and_sidechain_suffixes_cannot_change_the_prefix() {
    for link_mode in [LinkMode::DualMono, LinkMode::Average, LinkMode::Maximum] {
        for connected_sidechain in [false, true] {
            for (vary_main_suffix, vary_sidechain_suffix) in [(true, false), (false, true)] {
                let _ = render(
                    link_mode,
                    connected_sidechain,
                    vary_main_suffix,
                    connected_sidechain && vary_sidechain_suffix,
                );
            }
        }
    }
}
