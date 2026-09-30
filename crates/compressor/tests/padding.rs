//! Issue #1090 (console strip P2c): padded compressor banks through the production factory.
//!
//! A console slot banks at every track count, its partial groups padded with inactive lanes
//! (decision 12). These tests bind through `CompressorFactory::bind_homogeneous_bank` at this
//! build's own width (`Simd8` on x86-64, `Simd4` on AArch64 and wasm) and compare every member
//! against the same track rendered per node. The class-A differential at both widths on every
//! host, the silent fast path and planted state are the crate's unit tests
//! (`src/padding_tests.rs`), because they need a width the factory does not bind here and
//! internal witnesses.

mod support;

use conformance::{FixtureLimits, PcmFixture, parse_manifest};
use effect_contract::{
    BankProcessReport, BankWidth, BypassShunt, EffectBankProcessBlock, InitialParameterValue,
    LinkMode, PrepareEffectBankRequest, PrepareEffectRequest, PreparedNativeEffectBank,
    ProcessReport, StatePayloadInput, StatePayloadOutput,
};
use lane::Backend;

use compressor::CompressorFactory;
use dsp_reference::class_a;
use effect_contract::NativeEffectFactory;
use support::{PARAMETER_COUNT, native_bank_width, prepare, request, values_with};

const QUANTUM: u32 = 128;

type Values = [InitialParameterValue; PARAMETER_COUNT * 2];

/// The standing console fixture's first eight compressors
/// (`fixtures/session/v1/console-sixty-four-track-intended.json`, tracks 0-7, in table order:
/// threshold, ratio, knee, attack, release, makeup, mix), as `kernel.rs`'s #1006 scenario reads
/// them.
const CONSOLE_TRACKS: [[f32; PARAMETER_COUNT]; 8] = [
    [-6.0, 1.5, 3.0, 2.0, 40.0, 0.0, 1.0],
    [-7.5, 2.25, 4.5, 3.5, 55.0, 0.5, 1.0],
    [-9.0, 3.0, 6.0, 5.0, 70.0, 1.0, 1.0],
    [-10.5, 3.75, 7.5, 6.5, 85.0, 1.5, 1.0],
    [-12.0, 4.5, 9.0, 8.0, 100.0, 2.0, 1.0],
    [-13.5, 5.25, 3.0, 9.5, 115.0, 2.5, 1.0],
    [-15.0, 6.0, 4.5, 11.0, 130.0, 3.0, 1.0],
    [-16.5, 6.75, 6.0, 12.5, 145.0, 0.0, 1.0],
];

fn console_values(track: usize) -> Values {
    let row = CONSOLE_TRACKS[track % CONSOLE_TRACKS.len()];
    values_with(&core::array::from_fn::<_, PARAMETER_COUNT, _>(
        |parameter| (parameter, row[parameter]),
    ))
}

/// A request per lane: member `k` on the `k`-th lane, member 0's request cloned on every lane past
/// the members (the planner's members-first layout).
fn lane_requests<'a>(
    members: &[PrepareEffectRequest<'a>],
    lanes: usize,
) -> Vec<PrepareEffectRequest<'a>> {
    (0..lanes)
        .map(|lane| *members.get(lane).unwrap_or(&members[0]))
        .collect()
}

fn members_first(members: usize, lanes: usize) -> Vec<bool> {
    (0..lanes).map(|lane| lane < members).collect()
}

fn bind(
    backend: Backend,
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
    mask: &[bool],
) -> Box<dyn PreparedNativeEffectBank> {
    CompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests,
            active_mask: mask,
        })
        .expect("a well-formed padded request")
        .expect("the compressor binds a padded bank at this build's width")
}

/// One member's planes for a whole render.
struct Planes {
    left: Vec<f32>,
    right: Vec<f32>,
}

/// Every member rendered alone, in blocks of `partitions`: the per-node oracle's output words and
/// per-block reports.
fn per_node(
    members: &[PrepareEffectRequest<'_>],
    inputs: &[Planes],
    partitions: &[usize],
) -> Vec<(Vec<u32>, Vec<ProcessReport>)> {
    members
        .iter()
        .zip(inputs)
        .map(|(request, input)| {
            let mut effect = prepare(*request);
            let (mut left, mut right) = (input.left.clone(), input.right.clone());
            let mut reports = Vec::new();
            let mut offset = 0;
            for frames in partitions {
                let range = offset..offset + frames;
                reports.push(
                    effect.process(
                        effect_contract::EffectProcessBlock::new(
                            &mut left[range.clone()],
                            &mut right[range],
                            None,
                            offset as u64,
                            &[],
                            QUANTUM,
                        )
                        .expect("a bounded block"),
                    ),
                );
                offset += frames;
            }
            let words = left
                .iter()
                .chain(&right)
                .map(|sample| class_a::bits(*sample))
                .collect();
            (words, reports)
        })
        .collect()
}

/// A padded bank's render: each member's output words and per-block reports, after `restore`
/// has rewritten any lane it chooses (the rack's bypass shunt, below). Padded lanes are asserted
/// to stay `+0.0` and unreported on every block.
fn banked(
    bank: &mut dyn PreparedNativeEffectBank,
    width: BankWidth,
    inputs: &[Planes],
    partitions: &[usize],
    mut restore: impl FnMut(&BypassShunt, &mut [f32], &mut [f32], usize),
) -> Vec<(Vec<u32>, Vec<ProcessReport>)> {
    let lanes = width.lanes() as usize;
    let members = inputs.len();
    let mut shunt = BypassShunt::new(QUANTUM as usize * lanes, 0);
    let mut outputs = vec![(Vec::new(), Vec::new(), Vec::new()); members];
    let offsets = vec![0_u32; lanes + 1];
    let mut offset = 0;
    for (block, frames) in partitions.iter().copied().enumerate() {
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        for (lane, input) in inputs.iter().enumerate() {
            for frame in 0..frames {
                left[frame * lanes + lane] = input.left[offset + frame];
                right[frame * lanes + lane] = input.right[offset + frame];
            }
        }
        shunt.capture(&left, &right);
        let report: BankProcessReport = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                frames as u32,
                width,
                offset as u64,
                &[],
                &offsets,
                QUANTUM,
            )
            .expect("a bounded bank block"),
        );
        restore(&shunt, &mut left, &mut right, frames);
        for lane in members..lanes {
            assert!(
                (0..frames).all(|frame| left[frame * lanes + lane].to_bits() == 0
                    && right[frame * lanes + lane].to_bits() == 0),
                "block {block}: padded lane {lane} is not +0.0"
            );
            assert_eq!(
                report.reports[lane],
                ProcessReport::default(),
                "block {block}: padded lane {lane} was reported"
            );
        }
        for (lane, output) in outputs.iter_mut().enumerate() {
            output
                .0
                .extend((0..frames).map(|frame| class_a::bits(left[frame * lanes + lane])));
            output
                .1
                .extend((0..frames).map(|frame| class_a::bits(right[frame * lanes + lane])));
            output.2.push(report.reports[lane]);
        }
        offset += frames;
    }
    outputs
        .into_iter()
        .map(|(mut left, right, reports)| {
            left.extend(right);
            (left, reports)
        })
        .collect()
}

/// Every checked-in conformance PCM fixture, as `(name, rate, left, right)`; a mono fixture feeds
/// both channels.
fn pcm_fixtures() -> Vec<(String, u32, Vec<f32>, Vec<f32>)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/conformance");
    let entries = parse_manifest(&std::fs::read(root.join("MANIFEST.tsv")).expect("manifest"))
        .expect("a valid manifest");
    entries
        .iter()
        .map(|entry| {
            let bytes = std::fs::read(root.join(&entry.path)).expect("a listed fixture");
            let fixture = PcmFixture::parse(&bytes, FixtureLimits::default()).expect("a fixture");
            let frames = fixture.frames() as usize;
            let samples = fixture.samples();
            let left = samples[..frames].to_vec();
            let right = if fixture.channels() > 1 {
                samples[frames..2 * frames].to_vec()
            } else {
                left.clone()
            };
            (entry.path.clone(), fixture.rate().0, left, right)
        })
        .collect()
}

/// Gate 1 on the fixtures: every conformance PCM fixture, at its own launch rate, through a padded
/// bank of the standing console's compressors, at every active count `1..W` and under every link
/// mode. Each member hears the fixture four times over at its own level and offset (an exact power
/// of two, a rotation), in blocks that do not divide the quantum, and renders the bits, reports
/// and state of its own per-node instance.
///
/// Red for anything that couples a member to its bank-mates or to the padded lanes: a whole-bank
/// recovery, a link across lanes, a padded lane that writes anything but `+0.0`.
#[test]
fn the_fixtures_render_through_a_padded_bank_as_they_do_per_node() {
    let Some((backend, width)) = native_bank_width() else {
        println!("scalar-only build: no bank width");
        return;
    };
    let lanes = width.lanes() as usize;
    let fixtures = pcm_fixtures();
    assert_eq!(fixtures.len(), 7, "the seven checked-in fixtures");
    let values: Vec<Values> = (0..lanes).map(console_values).collect();
    for (name, rate, left, right) in &fixtures {
        let length = left.len() * 4;
        let partitions: Vec<usize> = [QUANTUM as usize, 100, 37, 64, 1, 128]
            .iter()
            .copied()
            .cycle()
            .scan(0, |taken, frames| {
                let frames = frames.min(length - *taken);
                *taken += frames;
                (frames > 0).then_some(frames)
            })
            .collect();
        for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            for members in 1..lanes {
                let requests: Vec<_> = values[..members]
                    .iter()
                    .map(|values| {
                        let mut request = request(values);
                        request.sample_rate = *rate;
                        request.link_mode = link;
                        request
                    })
                    .collect();
                let inputs: Vec<Planes> = (0..members)
                    .map(|member| {
                        let gain = f32::from_bits((127 + member as u32 % 4 - 1) << 23);
                        let shift = member * 17;
                        let plane = |source: &[f32]| -> Vec<f32> {
                            (0..length)
                                .map(|frame| gain * source[(frame + shift) % source.len()])
                                .collect()
                        };
                        Planes {
                            left: plane(left),
                            right: plane(right),
                        }
                    })
                    .collect();
                let oracle = per_node(&requests, &inputs, &partitions);
                let mut bank = bind(
                    backend,
                    width,
                    &lane_requests(&requests, lanes),
                    &members_first(members, lanes),
                );
                let rendered = banked(bank.as_mut(), width, &inputs, &partitions, |_, _, _, _| {});
                for member in 0..members {
                    let context = format!("{name} {link:?} {members} of {lanes}: member {member}");
                    let (want, got) = (&oracle[member].0, &rendered[member].0);
                    if let Some(word) = want.iter().zip(got).position(|(a, b)| a != b) {
                        panic!(
                            "{context}: word {word} per node {:#010x}, banked {:#010x}",
                            want[word], got[word]
                        );
                    }
                    assert_eq!(oracle[member].1, rendered[member].1, "{context}: reports");
                    assert!(
                        want.iter().any(|word| *word != 0),
                        "{context}: rendered nothing"
                    );
                }
            }
        }
    }
}

/// Gate 4, the P1 verdict's M2: a bypassed lane fed a tripping value moves no bank-mate's bit.
///
/// A session bypass keeps its track in the bank (#1087): the effect runs the lane with prepared
/// `bypass = false`, and the rack's `BypassShunt` copies the dry words back over that lane's
/// output, so the bypassed lane's wet path still runs and still meets D7. Here the bypassed lane
/// carries a legal gain of +24 dB (ratio 1, makeup +24) and is fed `1e29` on some blocks, a legal
/// sample below the `1e30` bound that its wet block pushes past it. The test drives the real
/// `BypassShunt` the same way `rack::ConsoleEffectBankStage` does: capture the dry block, run the
/// bank, restore the bypassed lane.
///
/// For every active count `2..=W` (full banks too) and the first and last member bypassed:
/// * every enabled member renders the bits and reports of the render in which the bypassed lane
///   was fed ordinary noise instead, and of its own per-node instance;
/// * the bypassed lane's output is its dry input, bit for bit;
/// * its wet block really was rejected (the report is the non-vacuity check), and no other lane
///   reports anything.
///
/// Red for the pre-#1090 whole-bank recovery: every enabled member of the bank is silenced for as
/// long as the bypassed lane stays hot (Sol's probe measured 43,008 of 43,008 words).
#[test]
fn a_bypassed_lane_fed_a_tripping_value_moves_no_bank_mates_bit() {
    let Some((backend, width)) = native_bank_width() else {
        println!("scalar-only build: no bank width");
        return;
    };
    let lanes = width.lanes() as usize;
    const BLOCKS: usize = 8;
    const TRIPPING_BLOCKS: [usize; 3] = [2, 3, 5];
    let partitions = vec![QUANTUM as usize; BLOCKS];
    let frames = QUANTUM as usize * BLOCKS;
    let hot = values_with(&[(0, 0.0), (1, 1.0), (2, 0.0), (5, 24.0), (6, 1.0)]);
    let cool: Vec<Values> = (0..lanes).map(console_values).collect();
    for members in 2..=lanes {
        for bypassed in [0, members - 1] {
            let values: Vec<&Values> = (0..members)
                .map(|member| {
                    if member == bypassed {
                        &hot
                    } else {
                        &cool[member]
                    }
                })
                .collect();
            let requests: Vec<_> = values.iter().map(|values| request(*values)).collect();
            let noise = |member: usize, channel: u64| {
                support::noise(frames, 0x1090_0000 + member as u64 * 2 + channel, 0.5)
            };
            let benign: Vec<Planes> = (0..members)
                .map(|member| Planes {
                    left: noise(member, 0),
                    right: noise(member, 1),
                })
                .collect();
            let mut tripping: Vec<Planes> = (0..members)
                .map(|member| Planes {
                    left: noise(member, 0),
                    right: noise(member, 1),
                })
                .collect();
            for block in TRIPPING_BLOCKS {
                let range = block * QUANTUM as usize..(block + 1) * QUANTUM as usize;
                tripping[bypassed].left[range.clone()].fill(1.0e29);
                tripping[bypassed].right[range].fill(-1.0e29);
            }
            let shunted = |inputs: &[Planes]| {
                let mut bank = bind(
                    backend,
                    width,
                    &lane_requests(&requests, lanes),
                    &members_first(members, lanes),
                );
                banked(
                    bank.as_mut(),
                    width,
                    inputs,
                    &partitions,
                    |shunt, left, right, frames| {
                        // `ConsoleEffectBankStage`'s restore: the bypassed lane's dry words, by copy.
                        let (dry_left, dry_right) = shunt.dry();
                        for frame in 0..frames {
                            let word = frame * lanes + bypassed;
                            left[word] = dry_left[word];
                            right[word] = dry_right[word];
                        }
                    },
                )
            };
            let hot_render = shunted(&tripping);
            let cool_render = shunted(&benign);
            let oracle = per_node(&requests, &tripping, &partitions);
            let context = format!("{members} of {lanes}, lane {bypassed} bypassed");
            for member in 0..members {
                if member == bypassed {
                    let dry: Vec<u32> = tripping[member]
                        .left
                        .iter()
                        .chain(&tripping[member].right)
                        .map(|sample| sample.to_bits())
                        .collect();
                    assert_eq!(
                        hot_render[member].0, dry,
                        "{context}: the bypassed lane is dry"
                    );
                    let rejected: u64 = hot_render[member]
                        .1
                        .iter()
                        .map(|report| report.nonfinite_left_blocks + report.nonfinite_right_blocks)
                        .sum();
                    assert!(
                        rejected >= 2 * TRIPPING_BLOCKS.len() as u64,
                        "{context}: the bypassed lane's wet block was not rejected ({rejected})"
                    );
                    continue;
                }
                assert_eq!(
                    hot_render[member], cool_render[member],
                    "{context}: enabled member {member} moved with the bypassed lane's input"
                );
                assert_eq!(
                    hot_render[member], oracle[member],
                    "{context}: enabled member {member} is not its per-node render"
                );
                assert!(
                    hot_render[member]
                        .1
                        .iter()
                        .all(|report| *report == ProcessReport::default()),
                    "{context}: enabled member {member} was reported"
                );
            }
        }
    }
}

/// A padded lane is not a track: a state payload call that names one is `effect.state.track`,
/// exactly as a lane past the width is, while every member's payload round-trips.
///
/// Red if a padded lane's payload can be snapshotted or restored. A restored envelope would move
/// the lane off rest: it would still write `+0.0` for `+0.0` in, but it would hold the whole bank
/// off the silent fast path until it released, and it has no track whose state it could be.
#[test]
fn a_padded_lane_is_not_a_track() {
    let Some((backend, width)) = native_bank_width() else {
        println!("scalar-only build: no bank width");
        return;
    };
    let lanes = width.lanes() as usize;
    let values: Vec<Values> = (0..lanes).map(console_values).collect();
    let requests: Vec<_> = values.iter().map(|values| request(values)).collect();
    let sizes = prepare(requests[0]).metadata().state_sizes;
    for members in 1..lanes {
        let mask = members_first(members, lanes);
        let mut bank = bind(
            backend,
            width,
            &lane_requests(&requests[..members], lanes),
            &mask,
        );
        for lane in 0..=lanes {
            let (mut left, mut right) = (
                vec![0_u8; sizes.left_bytes as usize],
                vec![0_u8; sizes.right_bytes as usize],
            );
            let snapshot = bank.snapshot_track_state_payload(
                lane as u32,
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            );
            let restore = |bank: &mut dyn PreparedNativeEffectBank, lane: usize| {
                bank.restore_track_state_payload(
                    lane as u32,
                    1,
                    StatePayloadInput {
                        common: &[],
                        left: &left,
                        right: &right,
                    },
                )
            };
            if lane < members {
                snapshot.expect("a member's snapshot");
                restore(bank.as_mut(), lane).expect("a member's payload restores");
            } else {
                assert_eq!(
                    snapshot.err().map(|error| error.code),
                    Some("effect.state.track"),
                    "{members} of {lanes}: snapshot of lane {lane}"
                );
                // A payload that restores onto a member must still be refused on this lane.
                let (mut member_left, mut member_right) = (
                    vec![0_u8; sizes.left_bytes as usize],
                    vec![0_u8; sizes.right_bytes as usize],
                );
                bank.snapshot_track_state_payload(
                    0,
                    StatePayloadOutput::new(&mut [], &mut member_left, &mut member_right, sizes)
                        .expect("sizes"),
                )
                .expect("member 0's snapshot");
                assert_eq!(
                    bank.restore_track_state_payload(
                        lane as u32,
                        1,
                        StatePayloadInput {
                            common: &[],
                            left: &member_left,
                            right: &member_right,
                        },
                    )
                    .err()
                    .map(|error| error.code),
                    Some("effect.state.track"),
                    "{members} of {lanes}: restore of lane {lane}"
                );
            }
        }
    }
}
