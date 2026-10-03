// Sol #1244 attempt 1 probes. Part 1: crates/host-core/tests/sol_probe_1244.rs (run with --release -- --ignored --nocapture).
//! Sol probes for #1244 attempt 1 (scratch, never committed).

use std::time::Instant;

use builtins::BuiltinLaneSelector;
use host_core::{
    HostPrepareCaps, HostShapePolicy, LiveControlSoloState, LiveVcaState, StripMuteSeed,
    compile_host_session,
};
use session::{Console, SessionModel, StableId, Submix, Vca, canonical_session_json, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: u64::MAX,
        maximum_submixes: u64::MAX,
        maximum_vcas: u64::MAX,
        maximum_sources: u64::MAX,
        maximum_routes: u64::MAX,
        maximum_effects: u64::MAX,
        maximum_graph_session_plus_plan_bytes: u64::MAX,
        maximum_source_total_bytes: u64::MAX,
        maximum_source_overhead_bytes: u64::MAX,
        maximum_effect_state_bytes: u64::MAX,
        maximum_effect_scratch_bytes: u64::MAX,
        maximum_builtin_retained_bytes: u64::MAX,
        maximum_named_allocation_bytes: u64::MAX,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn minify(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    let mut in_string = false;
    let mut escaped = false;
    for ch in json.chars() {
        if in_string {
            out.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
        } else if ch == '"' {
            in_string = true;
            out.push(ch);
        } else if !ch.is_whitespace() {
            out.push(ch);
        }
    }
    out
}

/// `strips` submixes (no sources) and a chain of `vcas` VCAs: v0 holds every strip, v{i} holds v{i-1}.
fn chain(strips: usize, vcas: usize, tracks: bool) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.sample_rate_hz = 48_000;
    let source = model.sources[0].clone();
    let mut track = model.tracks[0].clone();
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    let unity = Submix::unity(sid("unused"), &model.console);
    track.builtins = unity.builtins.clone();
    track.console = unity.console.clone();
    track.inserts = unity.inserts.clone();
    track.fader = unity.fader;
    track.matrix_or_pan = unity.matrix_or_pan.clone();
    let mut ids = Vec::new();
    for index in 0..strips {
        let id = format!("s{index:x}");
        if tracks {
            let mut source = source.clone();
            source.id = sid(&id);
            model.sources.push(source);
            let mut track = track.clone();
            track.id = sid(&id);
            track.source_id = sid(&id);
            model.tracks.push(track);
        } else {
            model.submixes.push(Submix::unity(sid(&id), &model.console));
        }
        ids.push(sid(&id));
    }
    for index in 0..vcas {
        model.vcas.push(Vca {
            id: sid(&format!("v{index:x}")),
            fader: session::DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: if index == 0 {
                ids.clone()
            } else {
                vec![sid(&format!("v{:x}", index - 1))]
            },
        });
    }
    model
}

fn sizes(model: &SessionModel) -> (usize, usize) {
    let canonical = canonical_session_json(model).expect("canonical");
    let small = minify(&canonical);
    parse_session_json(&small).expect("minified parses");
    (canonical.len(), small.len())
}

#[test]
#[ignore = "probe"]
fn sol_probe_worst_case_pairs_and_command_cost() {
    const MIB: usize = 1 << 20;
    for tracks in [false, true] {
        let base = sizes(&chain(1, 1, tracks));
        let per_strip = {
            let a = sizes(&chain(101, 1, tracks));
            ((a.0 - base.0) / 100, (a.1 - base.1) / 100)
        };
        let per_vca = {
            let a = sizes(&chain(1, 101, tracks));
            ((a.0 - base.0) / 100, (a.1 - base.1) / 100)
        };
        println!(
            "tracks={tracks}: base {base:?}, per strip (canonical, minified) {per_strip:?}, per chained VCA {per_vca:?}"
        );
        for (label, a, b, fixed) in [
            ("canonical", per_strip.0, per_vca.0, base.0),
            ("minified", per_strip.1, per_vca.1, base.1),
        ] {
            let budget = MIB - fixed;
            let mut strips = budget / (2 * a);
            let mut vcas = budget / (2 * b);
            // Shrink until the actual document fits.
            loop {
                let model = chain(strips, vcas, tracks);
                let size = if label == "canonical" {
                    canonical_session_json(&model).expect("c").len()
                } else {
                    minify(&canonical_session_json(&model).expect("c")).len()
                };
                if size <= MIB {
                    println!(
                        "  {label}: {strips} strips x {vcas} chained VCAs = {} pairs, {size} bytes",
                        strips * vcas
                    );
                    let document = if label == "canonical" {
                        canonical_session_json(&model).expect("c")
                    } else {
                        minify(&canonical_session_json(&model).expect("c"))
                    };
                    let started = Instant::now();
                    let compiled = compile_host_session(&document, &caps()).unwrap_or_else(|f| {
                        panic!("compile: {}", String::from_utf8_lossy(f.as_bytes()))
                    });
                    let compile_time = started.elapsed();
                    let model = compiled.normalized_model().clone();
                    let started = Instant::now();
                    let reach = model.vca_reach();
                    let reach_time = started.elapsed();
                    let total: usize = reach.iter().map(Vec::len).sum();
                    drop(reach);
                    let started = Instant::now();
                    let mut state = LiveVcaState::try_new(&model).expect("state");
                    let build = started.elapsed();
                    let s = model.tracks.len() + model.submixes.len();
                    let v = model.vcas.len();
                    let wasm32 = 4 * (s + 1) + 4 * total + 4 * (v + 1) + 4 * total + 2 * (v * 8 + v * 2) + 2 * (s * 8 + s * 8);
                    println!(
                        "    compile {compile_time:?}, one vca_reach {reach_time:?}, try_new {build:?}; pairs {total}; retained native {} B ({:.1} MiB), wasm32 est {wasm32} B ({:.1} MiB), largest {} B",
                        state.retained_bytes(),
                        state.retained_bytes() as f64 / MIB as f64,
                        wasm32 as f64 / MIB as f64,
                        state.largest_allocation_bytes()
                    );
                    let seeds: Vec<StripMuteSeed> = (0..s)
                        .map(|_| StripMuteSeed {
                            mutes: [false; 2],
                            solo_safe: true,
                            vca_mute: [false; 2],
                        })
                        .collect();
                    let mut solo = LiveControlSoloState::try_new(&seeds).expect("solo");
                    // The top VCA (and every one in a chain) reaches every strip.
                    let top = v - 1;
                    assert_eq!(state.reached_by(top).len(), s);
                    let started = Instant::now();
                    let mut records = 0;
                    assert!(state.set_vca_db(top, BuiltinLaneSelector::Both, -6.0));
                    for index in 0..state.reached_by(top).len() {
                        let strip = state.reached_by(top)[index];
                        for (lanes, db) in state.fader_delta(strip).into_iter().flatten() {
                            state.record_emitted_db(strip, lanes, db);
                            records += 1;
                        }
                    }
                    state.commit();
                    let db_move = started.elapsed();
                    let started = Instant::now();
                    assert!(state.set_vca_mute(top, BuiltinLaneSelector::Both, true));
                    for index in 0..state.reached_by(top).len() {
                        let strip = state.reached_by(top)[index];
                        solo.set_vca_mute(strip, state.vca_mute(strip));
                    }
                    let mut mutes = 0;
                    for strip in 0..s {
                        for (lanes, muted) in solo.strip_delta(strip).into_iter().flatten() {
                            solo.record_emitted(strip, lanes, muted);
                            mutes += 1;
                        }
                    }
                    state.commit();
                    solo.commit();
                    let mute_move = started.elapsed();
                    println!(
                        "    one VCA dB move: {db_move:?} ({records} records); one VCA mute move: {mute_move:?} ({mutes} records)"
                    );
                    break;
                }
                strips -= strips / 50 + 1;
                vcas -= vcas / 50 + 1;
            }
        }
    }
}

/// A NaN or out-of-domain offset reaches the state unrefused; record what it owes.
#[test]
#[ignore = "probe"]
fn sol_probe_non_finite_offset_owes_forever() {
    let model = compile_host_session(
        &canonical_session_json(&chain(2, 1, false)).expect("c"),
        &caps(),
    )
    .unwrap_or_else(|f| panic!("{}", String::from_utf8_lossy(f.as_bytes())))
    .normalized_model()
    .clone();
    let mut state = LiveVcaState::try_new(&model).expect("state");
    println!("set NaN accepted: {}", state.set_vca_db(0, BuiltinLaneSelector::Both, f32::NAN));
    let delta = state.fader_delta(0);
    println!("delta after NaN: {delta:?}");
    for (lanes, db) in delta.into_iter().flatten() {
        state.record_emitted_db(0, lanes, db);
    }
    println!("delta after recording it: {:?}", state.fader_delta(0));
    println!("set 500 accepted: {}", state.set_vca_db(0, BuiltinLaneSelector::Both, 500.0));
    println!("effective at +500 offset: {}", state.effective_db(0, 0));
    println!("set_member_db -1000 accepted: {}", state.set_member_db(0, BuiltinLaneSelector::Both, -1000.0));
    println!("effective: {}", state.effective_db(0, 0));
}

/// Deep chains and a lattice (every VCA at level k holds two at level k+1), far past the gate's
/// depth 4, edited at random: live composition equals `effective_strip_faders()` bit for bit.
#[test]
#[ignore = "probe"]
fn sol_probe_deep_chain_and_lattice_equal_preparation() {
    let mut rng = 0x1244_u64;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut checks = 0_u64;
    for shape in 0..40 {
        let strips = 3 + (shape % 5);
        let depth = 20 + shape * 3;
        let mut model = chain(strips, depth, shape % 2 == 0);
        if shape % 3 == 0 {
            // A lattice: v{i} also holds v{i-2}, so every strip reaches every VCA along many paths.
            for index in 2..depth {
                let extra = sid(&format!("v{:x}", index - 2));
                model.vcas[index].members.push(extra);
            }
        }
        let document = canonical_session_json(&model).expect("c");
        let mut model = compile_host_session(&document, &caps())
            .unwrap_or_else(|f| panic!("{}", String::from_utf8_lossy(f.as_bytes())))
            .normalized_model()
            .clone();
        let mut state = LiveVcaState::try_new(&model).expect("state");
        let lanes_of = [BuiltinLaneSelector::Left, BuiltinLaneSelector::Right, BuiltinLaneSelector::Both];
        let value = |bits: u64| -> f32 {
            match bits % 5 {
                0 => [-144.0, 24.0, 1.0e-30, -1.0e-30, -0.0][(bits >> 8) as usize % 5],
                1 => -144.0 + (((bits >> 16) % 16801) as f32) / 100.0,
                _ => -6.0 + (((bits >> 16) % 1201) as f32) / 100.0,
            }
        };
        for _ in 0..200 {
            let lanes = lanes_of[(next() % 3) as usize];
            let v = value(next());
            let s = model.tracks.len() + model.submixes.len();
            if next() % 3 == 0 {
                let strip = (next() % s as u64) as usize;
                assert!(state.set_member_db(strip, lanes, v));
                let tracks = model.tracks.len();
                let fader = if strip < tracks { &mut model.tracks[strip].fader } else { &mut model.submixes[strip - tracks].fader };
                if lanes != BuiltinLaneSelector::Right { fader.left_db = v; }
                if lanes != BuiltinLaneSelector::Left { fader.right_db = v; }
            } else {
                let vca = (next() % model.vcas.len() as u64) as usize;
                assert!(state.set_vca_db(vca, lanes, v));
                let fader = &mut model.vcas[vca].fader;
                if lanes != BuiltinLaneSelector::Right { fader.left_db = v; }
                if lanes != BuiltinLaneSelector::Left { fader.right_db = v; }
            }
            let prepared = model.effective_strip_faders();
            for (strip, fader) in prepared.iter().enumerate() {
                for lane in 0..2 {
                    assert_eq!(state.effective_db(strip, lane).to_bits(), fader.db[lane].to_bits(), "shape {shape} strip {strip} lane {lane}");
                    checks += 1;
                }
            }
            if next() % 2 == 0 { state.commit(); } else if next() % 5 == 0 { state.rollback(); state = LiveVcaState::try_new(&model).expect("rebuild"); }
        }
        for vca in 0..model.vcas.len() {
            assert_eq!(state.reached_by(vca).len(), model.tracks.len() + model.submixes.len());
        }
    }
    println!("{checks} lane checks");
}

// Part 2: appended to crates/host-core/tests/vca_live.rs (uses its helpers).
/// Sol probe: an effective value that flips only the sign of zero owes no record (`!=`); does the
/// live plan still render the bits of a fresh plan that bakes the other zero?
#[test]
fn sol_probe_signed_zero_effective_flip_renders_as_fresh() {
    let (mut model, source, track) = empty_session(QUANTUM * 4);
    add_track(&mut model, &source, &track, "t0");
    model.tracks[0].fader.left_db = -0.0;
    model.tracks[0].fader.right_db = -0.0;
    model
        .routes
        .push(to_output("t0-main", track_tap("t0", SendTap::PostPan)));
    model.vcas = vec![Vca {
        id: sid("z"),
        fader: UNITY,
        members: vec![sid("t0")],
    }];
    let model = normalized(&model);
    let feeds: Vec<(String, Planes)> = vec![(
        "t0".to_owned(),
        [
            (0..QUANTUM * 4).map(|i| -0.25 - (i as f32) * 1.0e-3).collect(),
            (0..QUANTUM * 4).map(|i| if i % 2 == 0 { -1.0e-30 } else { -0.5 }).collect(),
        ],
    )];
    let mut state = LiveVcaState::try_new(&model).expect("state");
    println!("prepared effective bits {:#x}", state.effective_db(0, 0).to_bits());
    let mut live = Live::new(&model);
    let _ = live.render_block(&feeds);
    assert!(state.set_vca_db(0, BuiltinLaneSelector::Both, -0.0));
    println!("live effective bits {:#x}", state.effective_db(0, 0).to_bits());
    println!("delta {:?}", state.fader_delta(0));
    assert_eq!(state.fader_delta(0), [None, None]);
    let actual = live.render_block(&feeds);
    let mut edited = model.clone();
    edited.vcas[0].fader.left_db = -0.0;
    edited.vcas[0].fader.right_db = -0.0;
    println!("fresh bakes {:?}", edited.effective_strip_faders()[0].db.map(f32::to_bits));
    let mut fresh = Live::new(&edited);
    let _ = fresh.render_block(&feeds);
    let expected = fresh.render_block(&feeds);
    for plane in 0..2 {
        assert_eq!(first_difference(&actual[plane], &expected[plane]), None, "plane {plane}");
    }
}

// Part 3: the mutation driver (one mutation at a time, file restored after each).
// import subprocess, sys, pathlib, shutil
// root = pathlib.Path('/tmp/claude-1002/v1244/mut')
// M = {
//  'S1': ('crates/host-core/src/solo.rs',
//         "        self.shadow();\n        self.vca_mute[strip] = lanes;\n",
//         "        self.shadow();\n        self.vca_mute[strip][0] |= lanes[0];\n        self.vca_mute[strip][1] |= lanes[1];\n"),
//  'S2': ('crates/host-core/src/vca.rs',
//         "            return;\n        }\n        self.shadow();\n        set_lanes(&mut self.emitted_db[strip], lanes, db);",
//         "            return;\n        }\n        set_lanes(&mut self.emitted_db[strip], lanes, db);"),
//  'S3': ('crates/host-core/src/vca.rs',
//         "        self.own_db_shadow.copy_from_slice(&self.own_db);\n        self.emitted_db_shadow.copy_from_slice(&self.emitted_db);\n        self.open = true;",
//         "        self.emitted_db_shadow.copy_from_slice(&self.emitted_db);\n        self.open = true;"),
//  'S4': ('crates/host-core/src/vca.rs',
//         "        for &vca in self.reach_of(strip) {\n            muted[0]",
//         "        for &vca in self.reach_of(strip).iter().take(1) {\n            muted[0]"),
//  'S5': ('crates/host-core/src/vca.rs',
//         "        match (left != emitted[0], right != emitted[1]) {",
//         "        match (left.to_bits() != emitted[0].to_bits(), right.to_bits() != emitted[1].to_bits()) {"),
//  'S6': ('crates/host-core/src/solo.rs',
//         "        self.emitted_shadow.copy_from_slice(&self.emitted);\n        self.vca_mute_shadow.copy_from_slice(&self.vca_mute);\n",
//         "        self.emitted_shadow.copy_from_slice(&self.emitted);\n"),
// }
// 
// M['H12'] = ('crates/host-core/src/vca.rs',
//         "        vca_effective_db(\n            *own,\n            self.reach_of(strip)\n                .iter()\n                .map(|&vca| self.vca_db[vca][lane]),\n        )",
//         "        let offsets: Vec<f32> = self.reach_of(strip).iter().map(|&vca| self.vca_db[vca][lane]).collect();\n        vca_effective_db(*own, offsets)")
// M['H13'] = ('crates/host-core/src/vca.rs',
//         "let emitted_db = try_boxed(strip_count, effective.iter().map(|fader| fader.db))?;",
//         "let emitted_db = try_boxed(strip_count, own_db.iter().copied())?;")
// M['S7'] = ('crates/host-core/src/vca.rs',
//         "        for (strip, list) in reach_lists.iter().enumerate() {\n            for &vca in list {\n                reached[cursor[vca]] = strip;",
//         "        for (strip, list) in reach_lists.iter().enumerate() {\n            for &vca in list.iter().take(1) {\n                reached[cursor[vca]] = strip;")
// 
// name = sys.argv[1]
// path, old, new = M[name]
// f = root / path
// orig = f.read_text()
// assert orig.count(old) == 1, (name, orig.count(old))
// f.write_text(orig.replace(old, new))
// try:
//     env = dict(__import__('os').environ, CARGO_TARGET_DIR='/tmp/claude-1002/v1244/target-m')
//     r = subprocess.run(['cargo','test','--locked','-p','host-core','--features','host-core/test-support','--test','vca_live','--lib','--no-fail-fast'], cwd=root, env=env, capture_output=True, text=True)
//     out = r.stdout + r.stderr
//     (pathlib.Path('/tmp/claude-1002/v1244/mutlogs')/f'{name}.log').write_text(out)
//     fails = [l for l in out.splitlines() if l.endswith('FAILED') or ' panicked at' in l or l.startswith('test result')]
//     print(name, 'rc', r.returncode)
//     print('\n'.join(fails[:40]))
// finally:
//     f.write_text(orig)
//     assert f.read_text() == orig
