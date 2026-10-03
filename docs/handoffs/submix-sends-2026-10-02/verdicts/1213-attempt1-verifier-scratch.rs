// Sol verifier probes for #1213 attempt 1 (c13ac5e1d). Not committed anywhere.
//
// To run the Rust probes: append them to hosts/host-web/src/tests.rs and replace
// `fn solo_bus_host(drums_muted: bool)` with this pair (the second takes per-lane mutes):
//
//   fn solo_bus_host(drums_muted: bool) -> AudioWorkletEngineHost {
//       solo_bus_host_lanes([drums_muted, drums_muted])
//   }
//   fn solo_bus_host_lanes(drums_muted: [bool; 2]) -> AudioWorkletEngineHost { ...body of the
//       old solo_bus_host, with drums.fader.left_mute = drums_muted[0] and
//       drums.fader.right_mute = drums_muted[1] ... }
//
// Results on c13ac5e1d: all three probes green. Under mutation P1
// (`let lane = 0_usize;` in kind 4) probe 1 goes red and the committed suite stays green.
// Under mutation P2 (`prepared_queue_address`'s Eq base spelled `ready.tracks.len() * 3`) or P3
// (the admission EQ marker `queue_slot: (ready.tracks.len() * 3 + effect)`), probe 3 goes red and
// the committed suite stays green. Probe 2 does not discriminate P1 (the solo term masks it) and
// is kept only as a record.
#[test]
fn verifier_scratch_right_lane_mute_on_a_bus_equals_booted_right_mute() {
    for (channel, lanes) in [(1_u8, [false, true]), (0_u8, [true, false])] {
        let mut live = solo_bus_host_lanes([false, false]);
        let mut booted = solo_bus_host_lanes(lanes);
        stage_command(&mut live, 0, COMMAND_MUTE, 255, channel, 3, 0, 0, STRIP_QUANTUM, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(live.submit_commands(1), RESULT_OK);
        for block in 0..2 {
            solo_bus_render(&mut live, block);
            solo_bus_render(&mut booted, block);
        }
        assert!(solo_bus_compare(&mut live, &mut booted, 2, 4, "lane mute live vs booted"));
    }
}

#[test]
fn verifier_scratch_right_lane_mute_on_a_track_under_solo() {
    // Track b right-muted live while track a is soloed, then a unsoloed: b's right lane must stay muted.
    let mut live = solo_bus_host_lanes([false, false]);
    let mut reference = solo_bus_host_lanes([false, false]);
    stage_solo(&mut live, 0, 0, true, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_solo(&mut reference, 0, 0, true, 0);
    assert_eq!(reference.submit_commands(1), RESULT_OK);
    // live: right-lane user mute on c (index 2); reference: both-lane mute on c then left unmute
    stage_command(&mut live, 0, COMMAND_MUTE, 255, 1, 2, 0, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_command(&mut reference, 0, COMMAND_MUTE, 255, 2, 2, 0, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    stage_command(&mut reference, 1, COMMAND_MUTE, 255, 0, 2, 0, 0, 0, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(reference.submit_commands(2), RESULT_OK);
    stage_solo(&mut live, 0, 0, false, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_solo(&mut reference, 0, 0, false, 0);
    assert_eq!(reference.submit_commands(1), RESULT_OK);
    for block in 0..2 {
        solo_bus_render(&mut live, block);
        solo_bus_render(&mut reference, block);
    }
    assert!(solo_bus_compare(&mut live, &mut reference, 2, 4, "c right-muted"));
}

#[test]
fn verifier_scratch_prepared_eq_on_a_bus_equals_the_track() {
    let (_, _, _, mut eq, _) = strip_base();
    eq.id = strip_id("eq");
    eq.identity = session::EffectIdentity::Native {
        effect_id: strip_id("miso.parametric-eq"),
    };
    eq.params = Vec::new();
    let (bus, reference) = strip_pair_documents(Some(eq));
    let mut a = strip_boot(&bus, strip_options(64, 0, 0));
    let mut b = strip_boot(&reference, strip_options(64, 0, 0));
    let mut block = 0;
    for _ in 0..2 {
        strip_render_pair(&mut a, &mut b, block, "before");
        block += 1;
    }
    for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
        stage_prepared_eq_parameter(host, 0, index, RACK_INSERTS, 0, 4, -12.0);
        assert_eq!(host.submit_prepared_commands(1, 104), RESULT_OK, "prepared EQ at strip {index}");
    }
    let mut audible = false;
    for _ in 0..6 {
        audible |= strip_render_pair(&mut a, &mut b, block, "prepared EQ");
        block += 1;
    }
    assert!(audible);
}

// ---------------------------------------------------------------------------------------------
// JS probes (scripts/test-web-audioworklet.mjs). Insert inside `testProcessor()`, just before
// `const explicitHopOptions = ...`:
//
//     {
//       const { processor, fake } = makeProcessor();
//       fake.exports.miso_engine_web_v1_eq_target_config_copy = () => 1;
//       fake.exports.miso_engine_web_v1_input_filters_config_copy = () => 1;
//       const classify = (trackIndex, rack) => {
//         processor.port.posts.length = 0;
//         processor.receiveEqTargetConfig({ requestId: 7, trackIndex, rack, effectIndex: 9 });
//         const reply = processor.port.posts.at(-1).message;
//         assert.equal(reply.tag, "miso.eq-target-config.v1");
//         assert.equal(reply.result, 1);
//         return reply.reason;
//       };
//       assert.equal(classify(1, 1), 4, "a track's missing insert is unknownEffect");
//       assert.equal(classify(2, 1), 4, "the bus (T + 0) missing insert is unknownEffect");
//       assert.equal(classify(2, 0), 3, "the bus with a retired rack is unknownRack");
//       assert.equal(classify(3, 1), 2, "T + S is unknownTrack");
//     }
//
// Green on c13ac5e1d; red ("the bus (T + 0) missing insert is unknownEffect") when the worklet
// classifier is reverted to `message.trackIndex >= this.trackCount`, which the committed
// harness does not notice (rc 0).
//
// frameSlot probe: insert after `assert.equal(observeAck.tag, "miso.observe.v1");`
//
//     const map = await liveControlHost.sessionMap();
//     const busAck = await liveControlHost.observe({ subscriptions: [{ trackIndex: map.tracks.length,
//       rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: true }] });
//     console.log(JSON.stringify(busAck.bindings.find((b) => b.trackIndex === map.tracks.length)));
//
// prints {"trackIndex":2,...,"frameSlot":2,...} with 2 tracks: past the end of trackGrDb.
