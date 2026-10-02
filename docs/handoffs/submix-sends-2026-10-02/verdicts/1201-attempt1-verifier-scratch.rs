// Sol verifier scratch for #1201 attempt 1 (MINOR-1). Append to
// crates/host-core/tests/submix_strip.rs; it uses that file's helpers (empty_session, add_track,
// route, post_pan, into_bus, to_output, bus_output, Strip, document, impulse_feeds, borrowed,
// render, BLOCKS).
//
// Green on 84d26a1d8. RED under the mutation `track_delays[0].process(..)` in the SumDelay arm
// (line-index aliasing between the TrackDelay and SumDelay arms, or between two buses), which the
// committed suite does not catch: L positions [21, 25] instead of [23, 51].

/// `(sample, bits)` of every nonzero sample.
fn nonzero(plane: &[f32]) -> Vec<(usize, u32)> {
    plane
        .iter()
        .enumerate()
        .filter(|(_, sample)| **sample != 0.0)
        .map(|(index, sample)| (index, sample.to_bits()))
        .collect()
}

/// A delayed track into a delayed bus, beside a second delayed bus: every line is its own, and a
/// track's delay and its bus's delay add.
#[test]
fn delayed_tracks_into_two_delayed_buses_each_keep_their_own_line() {
    let build = |t0: [u32; 2], bus: [u32; 2], bus2: [u32; 2]| {
        let (mut model, source, track) = empty_session(48_000);
        add_track(&mut model, &source, &track, "t0");
        add_track(&mut model, &source, &track, "t1");
        model.tracks[0].builtins.left.delay_samples = t0[0];
        model.tracks[0].builtins.right.delay_samples = t0[1];
        model.routes.push(route("t0-bus", post_pan("t0"), into_bus("bus")));
        model.routes.push(route("t1-bus2", post_pan("t1"), into_bus("bus2")));
        model.routes.push(to_output("bus-main", bus_output("bus")));
        model.routes.push(to_output("bus2-main", bus_output("bus2")));
        let mut first = Strip::transparent();
        first.builtins.left.delay_samples = bus[0];
        first.builtins.right.delay_samples = bus[1];
        let mut second = Strip::transparent();
        second.builtins.left.delay_samples = bus2[0];
        second.builtins.right.delay_samples = bus2[1];
        model.submixes = vec![first.submix("bus"), second.submix("bus2")];
        document(&model)
    };
    let feeds = impulse_feeds(); // t0: 0.25 at sample 3; t1: 0.5 at sample 10; both planes.
    let feeds = borrowed(&feeds);
    let reference = render(&build([0, 0], [0, 0], [0, 0]), &feeds, BLOCKS);
    let out = render(&build([11, 0], [37, 5], [13, 0]), &feeds, BLOCKS);
    let word = |plane: &[f32], at: usize| plane[at].to_bits();
    // Left: t1 via bus2 at 10 + 13; t0 via its own 11 and bus's 37 at 3 + 48.
    assert_eq!(
        nonzero(&out[0]),
        vec![(23, word(&reference[0], 10)), (51, word(&reference[0], 3))]
    );
    // Right: t0 at 3 + 0 + 5; t1 at 10 + 0.
    assert_eq!(
        nonzero(&out[1]),
        vec![(8, word(&reference[1], 3)), (10, word(&reference[1], 10))]
    );
}
