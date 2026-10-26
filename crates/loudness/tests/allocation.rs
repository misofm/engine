//! The render half allocates and frees nothing, on any sample value.

use bench_support::alloc;
use loudness::LoudnessObserver;

#[test]
fn observe_allocates_nothing() {
    alloc::assert_installed();
    let rate = 96_000;
    let mut observer = LoudnessObserver::new(rate).expect("launch rate");
    let mut left = vec![0.0_f32; 1_000];
    let mut right = vec![0.0_f32; 1_000];
    for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
        *l = ((index * 37) % 200) as f32 / 200.0 - 0.5;
        *r = 0.25 - ((index * 11) % 100) as f32 / 200.0;
    }
    left[17] = f32::NAN;
    right[400] = f32::INFINITY;
    left[999] = f32::NEG_INFINITY;
    right[3] = f32::MAX;
    let mut records = 0_u64;
    // Warm every process static the loop could touch.
    observer.observe(&left, &right, 0, &mut |_| records += 1);

    let mark = alloc::current_thread_counters();
    let mut first = left.len() as u64;
    for _ in 0..200 {
        observer.observe(&left, &right, first, &mut |record| {
            records += 1;
            std::hint::black_box(record);
        });
        first += left.len() as u64;
    }
    // A discontinuous block restarts the observer, which must not allocate either.
    observer.observe(&left, &right, first + 7, &mut |_| records += 1);
    let delta = alloc::current_thread_delta_since(mark);

    assert!(records > 200, "the loop completed {records} hops");
    assert_eq!(delta.allocations, 0, "observe allocated");
    assert_eq!(delta.deallocations, 0, "observe freed");
}
