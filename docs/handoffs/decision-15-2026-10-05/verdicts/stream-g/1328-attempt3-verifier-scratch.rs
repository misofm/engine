// #1328 attempt 3 verifier scratch (not committed). Appended to the export of ad96a6327; see 1328-attempt3.md.
// (1) appended to crates/parametric-eq/tests/mono_collapse.rs
/// VERIFIER SCRATCH (not committed): after the collapsed off->on HPF ramp and desymmetrize, a
/// left-only retarget turns the left HPF off; input carries no -0.0, so the elision gate can admit.
#[test]
fn verifier_scratch_stale_identity_after_left_only_change() {
    let Some((width, backend)) = native_bank() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let bind_hpf = |enabled: bool| {
        let values_by_track: Vec<_> = (0..lanes)
            .map(|track| configured_with_hpf(track, enabled))
            .collect();
        let requests: Vec<_> = values_by_track
            .iter()
            .map(|values| request(values, false))
            .collect();
        ParametricEqFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
                active_mask: width.full_mask(),
            })
            .expect("valid HPF bank request")
            .expect("the native width must bind")
    };
    let clean = |base: usize| -> Vec<f32> {
        (0..FRAMES * lanes)
            .map(|word| match (base + word / lanes) % 4 {
                0 => 0.0,
                1 => 0.6,
                2 => -0.35,
                _ => 0.125,
            })
            .collect()
    };
    let mut mixed = bind_hpf(false);
    let mut never = bind_hpf(false);
    let mut moved = 0usize;
    for step in 0..(BLOCKS * 2) {
        let collapsed_half = step < BLOCKS / 2;
        if step == BLOCKS / 2 {
            mixed.desymmetrize_channels();
        }
        if step == 2 || step == BLOCKS / 2 + 2 {
            let after = step == 2;
            for bank in [mixed.as_mut(), never.as_mut()] {
                for lane in 0..lanes {
                    let target_values = configured_with_hpf(lane, after);
                    let mut changed = vec![false; target_values.len()];
                    changed[HPF_ENABLED * 2] = true;
                    if step == 2 {
                        changed[HPF_ENABLED * 2 + 1] = true;
                    }
                    apply_prepared_targets_lane(bank, lane, 48_000, &target_values, &changed);
                }
            }
        }
        let mut never_left = clean(step * FRAMES);
        let mut never_right = never_left.clone();
        let mut mixed_left = never_left.clone();
        let mut mixed_right = if collapsed_half {
            vec![f32::from_bits(0x7F7F_FFFF); FRAMES * lanes]
        } else {
            never_left.clone()
        };
        let first = (step * FRAMES) as u64;
        run_block(never.as_mut(), &mut never_left, &mut never_right, width, first, step, false, false);
        run_block(mixed.as_mut(), &mut mixed_left, &mut mixed_right, width, first, step, false, collapsed_half);
        let mut planes = vec![(&mixed_left, &never_left)];
        if !collapsed_half {
            planes.push((&mixed_right, &never_right));
        }
        for (m, n) in planes {
            for (a, b) in m.iter().zip(n.iter()) {
                if a.to_bits() != b.to_bits() {
                    if moved == 0 {
                        eprintln!("first move: block {step}: {a} vs {b}");
                    }
                    moved += 1;
                }
            }
        }
    }
    assert_eq!(moved, 0, "moved words");
}

// (2) appended to crates/parametric-eq/tests/exact_rest.rs; run once as is (joint) and once with crates/lane/src/lib.rs flush_pair's rest = mask_and(a1.lt(flush_eps), a2.lt(flush_eps)) (per-word)
/// VERIFIER SCRATCH (not committed): four low shelves 10 Hz +24 dB S 1 at 96 kHz in one EQ,
/// fed a 3.84 Hz square just under one section's dead-zone limit; prints the output peak.
#[test]
fn verifier_scratch_cascade_dead_zone() {
    use effect_contract::{EffectProcessBlock, NativeEffectFactory, ParameterChannel};
    use parametric_eq::{EqBandKind, ParametricEqFactory};
    let mut configured = support::values();
    for band in 0..4 {
        let base = band * 6;
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            support::set_initial(&mut configured, base, channel, 1.0);
            support::set_initial(&mut configured, base + 1, channel, EqBandKind::LowShelf as u32 as f32);
            support::set_initial(&mut configured, base + 2, channel, 10.0);
            support::set_initial(&mut configured, base + 3, channel, 24.0);
            support::set_initial(&mut configured, base + 4, channel, 0.707_106_8);
            support::set_initial(&mut configured, base + 5, channel, 1.0);
        }
    }
    for (freq, amp) in [(10.0_f32, 3.0e-11_f32), (10.0, 2.0e-11), (10.0, 1.0e-11), (100.0, 3.0e-12)] {
        for band in 0..4 {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                support::set_initial(&mut configured, band * 6 + 2, channel, freq);
            }
        }
        let mut effect = ParametricEqFactory
            .prepare(support::request_at_rate(&configured, false, 96_000))
            .expect("prepare");
        let mut peak = 0f32;
        let total = 96_000 + 192_000;
        let mut first = 0usize;
        while first < total {
            let mut left = [0f32; 128];
            for (i, x) in left.iter_mut().enumerate() {
                let n = first + i;
                *x = if n < 96_000 { if (n / 12_488) % 2 == 0 { amp } else { -amp } } else { 0.0 };
            }
            let mut right = left;
            effect.process(EffectProcessBlock::new(&mut left, &mut right, None, first as u64, &[], 128).expect("block"));
            for x in left { peak = peak.max(x.abs()); }
            first += 128;
        }
        eprintln!("VERIFIER cascade {freq} Hz input {amp:e}: output peak {peak:e} ({:.1} dBFS)", 20.0 * f64::from(peak).log10());
    }
}

// (3) standalone f32 simulation, rustc -O casc.rs
// const FLUSH_EPS: f32 = 1.0e-20;
// const REST_EPS: f32 = 1.0e-14;
// fn ls(f: f64, gain_db: f64, s: f64, fs: f64) -> [f32; 6] {
//     let a = 10f64.powf(gain_db / 40.0);
//     let w = (std::f64::consts::PI * f / fs).tan();
//     let sk = ((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0).sqrt();
//     let (g, k, m0, m1, m2) = (w / a.sqrt(), sk, 1.0, sk * (a - 1.0), a * a - 1.0);
//     let t = g * (g + k);
//     let c1 = t / (1.0 + t);
//     let a2 = g * (1.0 - c1);
//     let a3 = g * a2;
//     [c1 as f32, a2 as f32, a3 as f32, m0 as f32, m1 as f32, m2 as f32]
// }
// fn run(w: [f32; 6], input: &[f32], joint: bool) -> Vec<f32> {
//     let [c1, a2, a3, m0, m1, m2] = w;
//     let (mut s1, mut s2) = (0f32, 0f32);
//     let fl = |x: f32| if x.abs() < FLUSH_EPS { 0.0 } else { x };
//     input.iter().map(|&v0| {
//         let v3 = v0 - s2;
//         let d1 = ((-c1) * s1) + (a2 * v3);
//         let v1 = s1 + d1;
//         let d2 = (a3 * v3) + (a2 * s1);
//         let v2 = s2 + d2;
//         let n1 = s1 + (d1 + d1);
//         let n2 = s2 + (d2 + d2);
//         let rest = joint && n1.abs() < REST_EPS && n2.abs() < REST_EPS;
//         s1 = if rest { 0.0 } else { fl(n1) };
//         s2 = if rest { 0.0 } else { fl(n2) };
//         (m2 * v2) + ((m1 * v1) + (m0 * v0))
//     }).collect()
// }
// fn main() {
//     for (f, sections) in [(10.0, 1usize), (10.0, 2), (10.0, 4), (100.0, 4)] {
//         let w = ls(f, 24.0, 1.0, 96_000.0);
//         let lstar = REST_EPS as f64 / (2.0 * w[1] as f64);
//         for frac in [0.99f64] {
//             let amp = lstar * frac;
//             let mut input: Vec<f32> = (0..96_000).map(|n| (if (n / 12_488) % 2 == 0 { amp } else { -amp }) as f32).collect();
//             input.resize(96_000 + 400_000, 0.0);
//             let (mut a, mut b) = (input.clone(), input.clone());
//             for _ in 0..sections { a = run(w, &a, false); b = run(w, &b, true); }
//             let worst = a.iter().zip(&b).map(|(x, y)| (x - y).abs()).fold(0f32, f32::max);
//             let peak_pw = a.iter().fold(0f32, |p, x| p.max(x.abs()));
//             let peak_j = b.iter().fold(0f32, |p, x| p.max(x.abs()));
//             println!("{sections} x low shelf {f} Hz +24 dB @96k, input {:.3e} ({:.2} dBFS): per-word peak {:.3e}, joint peak {:.3e}, worst change {:.3e} ({:.1} dBFS)",
//                 amp, 20.0*amp.log10(), peak_pw, peak_j, worst, 20.0*(worst as f64).log10());
//         }
//     }
// }
