//! Exhaustive identity proof for MA-3's E1 argument reduction.
//!
//! The reference below is a test-local copy of the pre-E1 `exp2_lane` body. The ignored sweep
//! compares it to the current scalar implementation for every `f32` bit pattern and also checks
//! Simd4 and Simd8 against that current scalar result. This includes NaN payloads, infinities,
//! subnormals, clamp rails, and the negative fractions whose `x - floor(x)` rounds to `1.0`.

use std::thread;

use lane::Lane;

/// Pre-E1 `exp2_lane`, retained here as an independent operation-order reference.
#[inline(always)]
#[allow(clippy::excessive_precision)]
fn pre_e1_exp2_lane<L: Lane>(x: L) -> L {
    const COEFFICIENTS: [f32; 6] = [
        1.535336188319500E-4,
        1.339887440266574E-3,
        9.618437357674640E-3,
        5.550332471162809E-2,
        2.402264791363012E-1,
        6.931472028550421E-1,
    ];

    let x = x.max(L::splat(-126.0)).min(L::splat(127.0));
    let xi = x.floor();
    let f = x.sub(xi);

    let fold = f.gt(L::splat(0.5));
    let xi = L::select(fold, xi.add(L::splat(1.0)), xi);
    let f = L::select(fold, f.sub(L::splat(1.0)), f);

    let mut p = L::splat(COEFFICIENTS[0]);
    let mut index = 1;
    while index < COEFFICIENTS.len() {
        p = p.mul(f).add(L::splat(COEFFICIENTS[index]));
        index += 1;
    }
    let p = L::splat(1.0).add(f.mul(p));

    p.mul(L::exp2_int_in_range(xi))
}

#[test]
#[ignore = "all 2^32 f32 bit patterns; run in release mode with --ignored"]
fn e1_exp2_lane_is_bit_identical_over_all_patterns_and_widths() {
    const TOTAL: u64 = 1_u64 << 32;
    // Patterns per batch: eight, so every vector width covers a batch in whole vectors.
    const WIDTH: u64 = 8;
    let workers = thread::available_parallelism()
        .map_or(1, |count| count.get())
        .min(4);
    let batches = TOTAL / WIDTH;
    let batches_per_worker = batches.div_ceil(workers as u64);

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for worker in 0..workers {
            let first_batch = worker as u64 * batches_per_worker;
            let end_batch = ((worker as u64 + 1) * batches_per_worker).min(batches);
            handles.push(scope.spawn(move || {
                let mut inputs = [0.0_f32; WIDTH as usize];
                // `(lane count, result bits)` per vector width this build has, narrowest first:
                // Simd4, and Simd8 where `avx2` is enabled (issue #1112).
                let mut results = [(0_usize, [0_u32; WIDTH as usize]); 2];

                for batch in first_batch..end_batch {
                    let first_pattern = batch * WIDTH;
                    for (index, input) in inputs.iter_mut().enumerate() {
                        *input = f32::from_bits((first_pattern + index as u64) as u32);
                    }

                    let mut widths = 0;
                    lane::each_vector_lane!(|L, N| {
                        results[widths].0 = N;
                        for offset in (0..inputs.len()).step_by(N) {
                            math::exp2_lane::<L>(L::load(&inputs[offset..]))
                                .store_bits(&mut results[widths].1[offset..]);
                        }
                        widths += 1;
                    });

                    for index in 0..inputs.len() {
                        let input = inputs[index];
                        let expected = math::exp2_lane::<f32>(input).to_bits();
                        let old = pre_e1_exp2_lane::<f32>(input).to_bits();
                        assert_eq!(
                            old,
                            expected,
                            "pre-E1 scalar identity failed at {:#010x}",
                            input.to_bits()
                        );

                        for (width, got) in &results[..widths] {
                            assert_eq!(
                                got[index],
                                expected,
                                "Simd{width} differs from scalar at {:#010x}",
                                input.to_bits()
                            );
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("exhaustive identity worker panicked");
        }
    });

    println!("E1 identity: 4,294,967,296 patterns; Scalar and every vector width: 0 mismatches");
}
