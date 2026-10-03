
/// Verifier probe (#1209 attempt 1, not committed): the browser's `SAMPLE_PEAK` bus meters
/// against the feeder sum with a varying, sign-changing source whose peak lands at a random
/// sample inside the two-block window. Every track lane is a positive multiple of the one source
/// lane, so the peak of a bus is exactly the sum of its feeders' peaks (to rounding).
#[test]
fn verifier_probe_bus_sample_peak_varying_source() {
    const QUANTUM: u32 = 128;
    for seed in 1..=6_u64 {
        let mut twin = bus_meter_host(QUANTUM, false);
        let mut host = bus_meter_host(QUANTUM, true);
        assert_eq!(twin.set_meter_lease(true), RESULT_OK);
        assert_eq!(host.set_meter_lease(true), RESULT_OK);
        let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            ((state >> 40) as f32 / (1_u64 << 24) as f32).mul_add(2.0, -1.0)
        };
        let q = QUANTUM as usize;
        let mut windows = 0;
        for block in 0..8_u64 {
            let mut left: Vec<f32> = (0..q).map(|_| next() * 0.2).collect();
            let mut right: Vec<f32> = (0..q).map(|_| next() * 0.2).collect();
            // One spike per lane, at a drawn sample, sometimes negative.
            let at_l = (next().abs() * (q as f32 - 1.0)) as usize;
            let at_r = (next().abs() * (q as f32 - 1.0)) as usize;
            left[at_l] = if block % 3 == 0 { -0.9 } else { 0.7 + 0.01 * block as f32 };
            right[at_r] = if block % 2 == 0 { 0.6 } else { -0.8 };
            let planes: [&[f32]; 2] = [&left, &right];
            for h in [&mut twin, &mut host] {
                assert_eq!(
                    h.submit_source(
                        b"fixture-source",
                        1,
                        block * u64::from(QUANTUM),
                        h.status().sample_rate_hz,
                        &planes,
                        QUANTUM,
                        false,
                    ),
                    RESULT_OK
                );
                assert_eq!(h.render_next(), RESULT_OK);
            }
            if block % 2 == 1 {
                let a = twin.poll_meters();
                let b = host.poll_meters();
                assert_eq!(a, b, "seed {seed} block {block}: same window count");
                if a == 0 {
                    continue;
                }
                windows += 1;
                let today = twin.meter_frame().to_vec();
                let frame = host.meter_frame().to_vec();
                assert_eq!(twin.meter_header().first_sample, host.meter_header().first_sample);
                let (tracks, strips) = (3_usize, 5_usize);
                for word in 0..2 * tracks {
                    assert_eq!(frame[word].to_bits(), today[word].to_bits(), "track word {word}");
                }
                let close = |x: f32, y: f32| (x - y).abs() <= y.abs() * 2e-6;
                for lane in 0..2 {
                    let aaa = frame[2 * tracks + lane];
                    let zzz = frame[2 * (tracks + 1) + lane];
                    assert!(
                        close(aaa, frame[lane] + frame[2 + lane]),
                        "seed {seed} block {block} lane {lane}: aaa {aaa} vs {}",
                        frame[lane] + frame[2 + lane]
                    );
                    assert!(close(zzz, frame[4 + lane]), "seed {seed} lane {lane}: zzz {zzz}");
                    let master = frame[2 * strips + lane];
                    assert!(close(master, today[2 * tracks + lane]), "master {master}");
                }
            }
        }
        assert!(windows >= 2, "seed {seed}: {windows} windows compared");
    }
}
