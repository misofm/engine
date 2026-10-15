//! Issue #1562a (slice S10a of decision 17): the browser host commits live edits through the
//! control plane's `SessionState::command`, reached by the generic command-frame entry of D1, so a
//! `SET_CONTROL` edit advances the committed revision, reaches the plan's live lanes, and renders
//! as a plan prepared from the committed model.
//!
//! Every test drives `AudioWorkletEngineHost` through `control_request_mut` and `control_command`,
//! the calls the exports make, with the frames the C ABI would receive and nothing decoded by the
//! host. A transaction the classifier sends to the rebuild arm is prepared and published like any
//! other (#1562b).

#![cfg(feature = "test-support")]

use controls::{ControlAddress, Lane, MonitorOverlay, Owner, Section};
use host_web::RESULT_INVALID_ARGUMENT;
use protocol::{
    CommandPayload, ControlListRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, SessionEdit,
};
use session::{
    DualMonoFader, ParameterChannel, SessionModel, StableId, Vca, canonical_session_json,
    parse_session_json,
};

mod common;
use common::*;

const RATE: u32 = 48_000;
const FOLLOWS_MUTE: u32 = 7;
const MATRIX_MODE: u32 = 1;
const PAN_WINDOW_SAMPLES: u32 = 3;
const SESSION_MUTE_MS: u32 = 2;
const DELAY_SAMPLES: u32 = 11;
const EQ_BAND_GAIN: u32 = 4;
const COMPRESSOR_THRESHOLD: u32 = 1;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner,
        section,
        control,
        lane,
    }
}

fn strip_row(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Strip(id(strip)), Section::Builtins, control, lane)
}

fn strip_own(strip: &str, control: u32) -> ControlAddress {
    address(
        Owner::Strip(id(strip)),
        Section::Own,
        control,
        ParameterChannel::Both,
    )
}

fn insert_row(strip: &str, effect: &str, control: u32, lane: Lane) -> ControlAddress {
    address(
        Owner::Strip(id(strip)),
        Section::Inserts(id(effect)),
        control,
        lane,
    )
}

fn console_row(strip: &str, slot: &str, control: u32, lane: Lane) -> ControlAddress {
    address(
        Owner::Strip(id(strip)),
        Section::Console(id(slot)),
        control,
        lane,
    )
}

fn route_row(control: u32) -> ControlAddress {
    address(
        Owner::Route(id("t1-b")),
        Section::Own,
        control,
        ParameterChannel::Both,
    )
}

fn vca_row(control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Vca(id(VCA.id)), Section::Own, control, lane)
}

fn set(address: ControlAddress, value: f32, ramp: Option<u32>) -> SessionEdit {
    SessionEdit::SetControl {
        address,
        value,
        ramp_samples: ramp,
    }
}

/// The committed document of a rig, as the snapshot query serves it.
fn document(rig: &mut Rig) -> Vec<u8> {
    snapshot_bytes(|payload, expected| rig.command(payload, expected))
}

/// `blocks` blocks of a host booted from `document`, with the same PCM from frame 0.
fn fresh_blocks(document: &[u8], blocks: u64, prepare: impl FnOnce(&mut Rig)) -> Vec<Vec<u32>> {
    let mut rig = Rig::boot(document, RATE);
    prepare(&mut rig);
    (0..blocks).map(|block| rig.step(block)).collect()
}

/// A run of `blocks` blocks of the parity session where each `(block, edits)` entry is committed
/// before its block. Returns the bits of every block.
fn run_script(
    model: &SessionModel,
    script: &[(u64, Vec<SessionEdit>)],
    blocks: u64,
) -> Vec<Vec<u32>> {
    let mut rig = Rig::new(model);
    (0..blocks)
        .map(|block| {
            for (_, edits) in script.iter().filter(|(at, _)| *at == block) {
                let verdict = rig.transaction(edits);
                assert!(
                    matches!(verdict, Verdict::Applied { .. }),
                    "block {block}: {verdict:?}"
                );
            }
            rig.step(block)
        })
        .collect()
}

fn first_difference(a: &[Vec<u32>], b: &[Vec<u32>]) -> Option<usize> {
    a.iter().zip(b).position(|(x, y)| x != y)
}

/// One row of gate 1: its label, the edits that put the row's control in a state where the value
/// is a change, the row's own edits, and whether the control carries filter or effect state.
struct Row {
    label: &'static str,
    setup: Vec<SessionEdit>,
    edits: Vec<SessionEdit>,
    /// The row moves a coefficient the plan smooths through filter or effect state, so a plan that
    /// was running before the edit converges on the fresh plan's output without being bit-equal
    /// to it at any block.
    stateful: bool,
    /// The first block that is compared: an edit that carries no length of its own ramps over the
    /// session's, behind the plan's latency, so its blocks are compared after the ramp.
    from_block: usize,
}

fn row(label: &'static str, address: ControlAddress, value: f32) -> Row {
    Row {
        label,
        setup: Vec::new(),
        edits: vec![set(address, value, Some(0))],
        stateful: false,
        from_block: 0,
    }
}

/// A row whose control takes no length of its own (`control.ramp.fixed`): the engine smooths it.
fn fixed_row(label: &'static str, address: ControlAddress, value: f32) -> Row {
    Row {
        label,
        setup: Vec::new(),
        edits: vec![set(address, value, None)],
        stateful: matches!(
            label,
            "hpf"
                | "lpf"
                | "effect parameter"
                | "EQ band gain"
                | "pre-insert slot parameter"
                | "post-insert slot parameter"
                | "pre-insert slot bypass"
                | "post-insert slot bypass"
        ),
        from_block: 0,
    }
}

fn live_rows() -> Vec<Row> {
    let upsert_vca = SessionEdit::UpsertVca {
        vca: Vca {
            id: id(VCA.id),
            fader: DualMonoFader {
                left_db: -3.0,
                right_db: -3.0,
                left_mute: false,
                right_mute: false,
            },
            members: vec![id(T2.id), id(T3.id)],
        },
    };
    vec![
        // A track's strip rows.
        row(
            "fader",
            strip_row("t1", FADER_DB, ParameterChannel::Both),
            -9.0,
        ),
        row("mute", strip_row("t3", MUTE, ParameterChannel::Both), 1.0),
        row("pan", strip_row("t2", PAN, ParameterChannel::Left), -0.2),
        row(
            "matrix",
            strip_row("t3", MATRIX_LL, ParameterChannel::Both),
            0.7,
        ),
        row("matrix_mode", strip_own("t3", MATRIX_MODE), 0.0),
        row(
            "trim",
            strip_row("t1", TRIM_DB, ParameterChannel::Both),
            -5.0,
        ),
        row(
            "polarity",
            strip_row("t2", POLARITY_INVERT, ParameterChannel::Right),
            1.0,
        ),
        fixed_row("hpf", strip_row("t3", HPF_HZ, ParameterChannel::Both), 80.0),
        fixed_row(
            "lpf",
            strip_row("t3", LPF_HZ, ParameterChannel::Both),
            8_000.0,
        ),
        // A block-rate effect parameter, an EQ band gain and a bypass.
        fixed_row(
            "effect parameter",
            insert_row("t1", "comp", COMPRESSOR_THRESHOLD, ParameterChannel::Left),
            -30.0,
        ),
        fixed_row(
            "EQ band gain",
            insert_row("t1", "eq", EQ_BAND_GAIN, ParameterChannel::Both),
            -9.0,
        ),
        // An insert's bypass crossfades over the session's `mute_ms` ramp (#1341), which reaches
        // the output after the strip's delay compensation: the plan equals the fresh one from
        // the block the ramp has ended in. The ramp itself is held by `control_parity.rs`.
        Row {
            from_block: 16,
            ..fixed_row(
                "bypass",
                insert_row(
                    "t1",
                    "comp",
                    controls::BYPASS_CONTROL,
                    ParameterChannel::Both,
                ),
                1.0,
            )
        },
        // One console slot in each section (batch 7 review M6): a parameter and a bypass of each.
        fixed_row(
            "pre-insert slot parameter",
            console_row("t1", PRE_SLOT, COMPRESSOR_THRESHOLD, ParameterChannel::Left),
            -30.0,
        ),
        fixed_row(
            "post-insert slot parameter",
            console_row(
                "t1",
                POST_SLOT,
                COMPRESSOR_THRESHOLD,
                ParameterChannel::Both,
            ),
            12.0,
        ),
        fixed_row(
            "pre-insert slot bypass",
            console_row(
                "t1",
                PRE_SLOT,
                controls::BYPASS_CONTROL,
                ParameterChannel::Both,
            ),
            1.0,
        ),
        fixed_row(
            "post-insert slot bypass",
            console_row(
                "t1",
                POST_SLOT,
                controls::BYPASS_CONTROL,
                ParameterChannel::Both,
            ),
            1.0,
        ),
        // The send.
        row("send gain", route_row(ROUTE_GAIN_DB), -6.0),
        row("send mute", route_row(ROUTE_MUTE), 1.0),
        row("send matrix", route_row(ROUTE_MATRIX_LL), 0.5),
        // Following a muted source is the only state in which the toggle changes a gate.
        Row {
            setup: vec![set(
                strip_row("t1", MUTE, ParameterChannel::Both),
                1.0,
                Some(0),
            )],
            ..row("follows_mute", route_row(FOLLOWS_MUTE), 0.0)
        },
        // The VCA.
        row(
            "VCA offset",
            vca_row(VCA_OFFSET_DB, ParameterChannel::Both),
            -3.0,
        ),
        row("VCA mute", vca_row(VCA_MUTE, ParameterChannel::Left), 1.0),
        // A member joins a VCA that already offsets its members, so its fader changes.
        Row {
            label: "VCA member added",
            setup: vec![set(
                vca_row(VCA_OFFSET_DB, ParameterChannel::Both),
                -3.0,
                Some(0),
            )],
            edits: vec![upsert_vca],
            stateful: false,
            from_block: 20,
        },
        // A submix strip.
        row(
            "submix fader",
            strip_row("b", FADER_DB, ParameterChannel::Both),
            -4.0,
        ),
        row(
            "submix mute",
            strip_row("b", MUTE, ParameterChannel::Both),
            1.0,
        ),
        row(
            "submix matrix",
            strip_row("b", MATRIX_LL, ParameterChannel::Both),
            0.6,
        ),
    ]
}

/// Gate 1: every live family commits to the model through the generic entry.
///
/// For each row, one transaction before the first block: the reply is `live`, the revision
/// advances by one, the plan is the same plan, the committed document holds the value, and every
/// block is the bits of a plan prepared from that committed document and fed the same PCM from
/// frame 0.
///
/// Test value: red if a browser edit reaches the cells without reaching the committed model (the
/// document would not hold the value, and the fresh plan would render the old one), if a family
/// lowers differently in the browser than in the model, or if a row the core calls live is a
/// rebuild in the browser.
#[test]
fn every_live_row_commits_to_the_model_and_renders_as_a_fresh_plan() {
    const BLOCKS_RUN: u64 = 40;
    let model = model_at(RATE);
    let untouched = {
        let mut rig = Rig::new(&model);
        (0..BLOCKS_RUN)
            .map(|block| rig.step(block))
            .collect::<Vec<_>>()
    };
    for Row {
        label,
        setup,
        edits,
        stateful,
        from_block,
    } in live_rows()
    {
        let mut rig = Rig::new(&model);
        if !setup.is_empty() {
            assert!(
                matches!(rig.transaction(&setup), Verdict::Applied { .. }),
                "{label}: the setup commits"
            );
        }
        let before = rig.revision;
        let epoch = rig.host.active_plan_epoch_for_test();
        let verdict = rig.transaction(&edits);
        let Verdict::Applied { path, revision } = verdict else {
            panic!("{label}: refused: {verdict:?}")
        };
        assert_eq!(path, EditPath::Live, "{label}: the reply's path");
        assert_eq!(
            revision,
            before + 1,
            "{label}: the revision advances by one"
        );
        assert_eq!(
            rig.host.committed_revision(),
            revision,
            "{label}: the committed revision"
        );
        assert_eq!(
            rig.host.active_plan_epoch_for_test(),
            epoch,
            "{label}: a live edit is not a plan swap"
        );
        let committed = document(&mut rig);
        let parsed = parse_session_json(std::str::from_utf8(&committed).expect("UTF-8"))
            .expect("the committed document parses");
        for edit in &edits {
            if let SessionEdit::SetControl { address, value, .. } = edit {
                // A per-lane control is read on each of its lanes, never on both at once.
                let lanes: &[Lane] = if address.lane == ParameterChannel::Both
                    && controls::get(&parsed, &MonitorOverlay::default(), address, RATE).is_err()
                {
                    &[ParameterChannel::Left, ParameterChannel::Right]
                } else {
                    &[address.lane]
                };
                for lane in lanes {
                    let read = ControlAddress {
                        lane: *lane,
                        ..address.clone()
                    };
                    let held = controls::get(&parsed, &MonitorOverlay::default(), &read, RATE)
                        .unwrap_or_else(|error| panic!("{label}: {error:?}"));
                    assert!(
                        (held - value).abs() <= value.abs() * 1.0e-6,
                        "{label}: the committed document holds {held} on {lane:?}, not {value}"
                    );
                }
            }
        }
        let first_block = 0;
        let edited: Vec<Vec<u32>> = (first_block..BLOCKS_RUN)
            .map(|block| rig.step(block))
            .collect();
        let fresh = fresh_blocks(&committed, BLOCKS_RUN, |_| {});
        let largest_difference = |from: usize| {
            edited[from..]
                .iter()
                .flatten()
                .zip(fresh[from..].iter().flatten())
                .map(|(a, b)| (f32::from_bits(*a) - f32::from_bits(*b)).abs())
                .fold(0.0_f32, f32::max)
        };
        if stateful {
            // The plan was running before the edit and smooths the coefficient through state, so
            // it approaches the fresh plan's output and is never bit-equal to it. Gate 8 holds
            // these rows bit-equal to the C ABI's, which is the same smoothing.
            // A stateless smoother can land on the fresh plan's bits exactly: zero converges.
            let settled = largest_difference(BLOCKS_RUN as usize - 2);
            assert!(
                settled == 0.0 || settled < largest_difference(8) / 2.0,
                "{label}: the running plan does not converge on the fresh plan"
            );
        } else {
            assert_eq!(
                first_difference(&edited[from_block..], &fresh[from_block..]),
                None,
                "{label}: a block differs from the plan prepared from the committed model"
            );
        }
        if setup.is_empty() {
            assert_ne!(
                first_difference(&edited, &untouched),
                None,
                "{label}: the edit is not heard"
            );
        }
    }
}

/// Gate 1's `delay_samples` row (#1562b): a `SET_CONTROL` on `delay_samples` is a rebuild. The
/// browser prepares the successor and publishes it, the reply is `rebuild` at the next revision
/// with the plan still the boot plan, and from the block that adopts it every block is the bits of
/// a plan prepared from the committed document and fed the same PCM from frame 0.
///
/// Test value: red if the browser still refuses a rebuild-path frame, answers it live, acknowledges
/// it without a plan that renders it (the document would hold the delay and the blocks would not
/// have it), or adopts a plan that is not the committed model's.
#[test]
fn a_delay_samples_edit_is_a_rebuild_and_renders_as_a_fresh_plan() {
    const BLOCKS_RUN: u64 = 24;
    let model = model_at(RATE);
    let mut rig = Rig::new(&model);
    let before = rig.host.committed_revision();
    let epoch = rig.host.active_plan_epoch_for_test().expect("a plan");
    let verdict = rig.transaction(&[set(
        strip_row("t1", DELAY_SAMPLES, ParameterChannel::Both),
        64.0,
        None,
    )]);
    assert_eq!(
        verdict,
        Verdict::Applied {
            path: EditPath::Rebuild,
            revision: before + 1
        },
        "the reply is the rebuild path at the next revision"
    );
    assert_eq!(
        rig.host.active_plan_epoch_for_test(),
        Some(epoch),
        "the submit never waits for render: the boot plan still renders"
    );
    let committed = document(&mut rig);
    let edited: Vec<Vec<u32>> = (0..BLOCKS_RUN).map(|block| rig.step(block)).collect();
    assert_eq!(
        rig.host.active_plan_epoch_for_test(),
        Some(epoch + 1),
        "render adopted the successor"
    );
    let parsed = parse_session_json(std::str::from_utf8(&committed).expect("UTF-8"))
        .expect("the committed document parses");
    let held = controls::get(
        &parsed,
        &MonitorOverlay::default(),
        &strip_row("t1", DELAY_SAMPLES, ParameterChannel::Left),
        RATE,
    )
    .expect("the delay row reads");
    assert!((held - 64.0).abs() < 1.0e-6, "the document holds {held}");
    let fresh = fresh_blocks(&committed, BLOCKS_RUN, |_| {});
    assert_eq!(
        first_difference(&edited, &fresh),
        None,
        "a block differs from the plan prepared from the committed model"
    );
    let untouched = fresh_blocks(&document_of(&model), BLOCKS_RUN, |_| {});
    assert_ne!(
        first_difference(&edited, &untouched),
        None,
        "the delay is heard"
    );
}

fn document_of(model: &SessionModel) -> Vec<u8> {
    canonical_session_json(model)
        .expect("canonical session")
        .into_bytes()
}

/// Gate 3's structural part (#1562b): a structural transaction keeps the solo. Track `t3` is
/// muted by the solo of `t2`, so its delay changes nothing the output carries; the rebuild's
/// successor must still render under the overlay, or the soloed-out tracks would be heard.
///
/// Test value: red if a rebuild drops or recomputes the monitor overlay (the others would unmute
/// from the swap block, which the bits against a run without the rebuild show), or if the solo
/// moved the revision or the document.
#[test]
fn a_structural_transaction_keeps_the_solo() {
    const BLOCKS_RUN: u64 = 24;
    const REBUILD_BEFORE: u64 = 8;
    let model = without_inserts(RATE);
    let solo = set(strip_own_solo("t2"), 1.0, Some(0));

    let mut without = Rig::new(&model);
    without.transaction(std::slice::from_ref(&solo));
    let reference: Vec<Vec<u32>> = (0..BLOCKS_RUN).map(|block| without.step(block)).collect();

    let mut rig = Rig::new(&model);
    rig.transaction(std::slice::from_ref(&solo));
    let mut blocks = Vec::new();
    for block in 0..BLOCKS_RUN {
        if block == REBUILD_BEFORE {
            let before = rig.host.committed_revision();
            let verdict = rig.transaction(&[set(
                strip_row("t3", DELAY_SAMPLES, ParameterChannel::Both),
                64.0,
                None,
            )]);
            assert_eq!(
                verdict,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    revision: before + 1
                },
                "the structural transaction rebuilds"
            );
        }
        blocks.push(rig.step(block));
    }
    assert_eq!(
        rig.host.active_plan_epoch_for_test(),
        Some(1),
        "render adopted the successor"
    );
    assert_eq!(
        first_difference(&blocks, &reference),
        None,
        "the successor renders under the solo: every block equals a run with no rebuild"
    );
    // The solo is still a monitor write after the swap.
    let after = rig.host.committed_revision();
    let verdict = rig.transaction(&[set(strip_own_solo("t2"), 0.0, Some(0))]);
    assert_eq!(
        verdict,
        Verdict::Applied {
            path: EditPath::Monitor,
            revision: after
        },
        "clearing the solo after the swap is still a monitor write"
    );
}

/// Gate 4 (the acked-batch rule): a transaction of a valid fader edit, a solo and an out-of-domain
/// pan is refused whole.
///
/// Test value: red if any write precedes a later refusal: the fader cell, the solo overlay, the
/// model or the revision would move, and the next blocks would differ.
#[test]
fn a_refused_transaction_leaves_model_revision_overlay_and_cells_alone() {
    const BLOCKS_RUN: u64 = 8;
    let model = model_at(RATE);
    let mut rig = Rig::new(&model);
    let mut control = Rig::new(&model);
    let before = document(&mut rig);
    let revision = rig.host.committed_revision();
    let edits = [
        set(
            strip_row("t1", FADER_DB, ParameterChannel::Both),
            -12.0,
            Some(0),
        ),
        set(strip_own_solo("t2"), 1.0, Some(0)),
        set(strip_row("t1", PAN, ParameterChannel::Left), 5.0, Some(0)),
    ];
    let verdict = rig.transaction(&edits);
    let Verdict::Refused {
        codes,
        revision: reported,
    } = verdict
    else {
        panic!("the transaction must be refused: {verdict:?}")
    };
    assert!(
        codes.iter().any(|code| code == "control.domain"),
        "{codes:?}"
    );
    assert_eq!(
        reported, revision,
        "the refusal reports the unchanged revision"
    );
    assert_eq!(rig.host.committed_revision(), revision);
    assert_eq!(document(&mut rig), before);
    for block in 0..BLOCKS_RUN {
        assert_eq!(
            rig.step(block),
            control.step(block),
            "block {block} differs from a run without the refused call"
        );
    }
}

fn strip_own_solo(strip: &str) -> ControlAddress {
    strip_own(strip, SOLO)
}

/// Gate 3: solo is the core's monitor overlay.
///
/// Test value: red if the browser's solo advances the revision, writes the model, or is lost by a
/// later live transaction: the reply path, the revision, the document and the bits against a plan
/// prepared from the committed model with the same solo would show it.
#[test]
fn solo_is_the_monitor_overlay_and_a_later_live_edit_keeps_it() {
    const BLOCKS_RUN: u64 = 8;
    let model = model_at(RATE);
    let mut rig = Rig::new(&model);
    let before = document(&mut rig);
    let revision = rig.host.committed_revision();
    let verdict = rig.transaction(&[set(strip_own_solo("t2"), 1.0, Some(0))]);
    assert_eq!(
        verdict,
        Verdict::Applied {
            path: EditPath::Monitor,
            revision
        },
        "solo replies monitor with the revision unchanged"
    );
    assert_eq!(document(&mut rig), before, "solo is never in the document");
    let soloed: Vec<Vec<u32>> = (0..BLOCKS_RUN).map(|block| rig.step(block)).collect();
    let plain = fresh_blocks(&before, BLOCKS_RUN, |_| {});
    assert_ne!(
        first_difference(&soloed, &plain),
        None,
        "solo mutes the others"
    );
    let fresh_solo = fresh_blocks(&before, BLOCKS_RUN, |fresh| {
        fresh.transaction(&[set(strip_own_solo("t2"), 1.0, Some(0))]);
    });
    assert_eq!(
        first_difference(&soloed, &fresh_solo),
        None,
        "soloing through the browser renders as the core's overlay on a fresh plan"
    );

    // A live transaction after the solo keeps it, and the document holds the new fader.
    let verdict = rig.transaction(&[set(
        strip_row("t3", FADER_DB, ParameterChannel::Both),
        -20.0,
        Some(0),
    )]);
    assert!(matches!(
        verdict,
        Verdict::Applied {
            path: EditPath::Live,
            ..
        }
    ));
    let committed = document(&mut rig);
    let after: Vec<Vec<u32>> = (BLOCKS_RUN..2 * BLOCKS_RUN)
        .map(|block| rig.step(block))
        .collect();
    // The fresh plan renders from frame 0 under the same solo, and the blocks after the first
    // run's are compared: the fader edit is a step, and the fresh plan has it from the start.
    let fresh_after = fresh_blocks(&committed, 2 * BLOCKS_RUN, |fresh| {
        fresh.transaction(&[set(strip_own_solo("t2"), 1.0, Some(0))]);
    });
    assert_eq!(
        first_difference(&after, &fresh_after[BLOCKS_RUN as usize..]),
        None,
        "the solo survives a live edit"
    );
}

/// The parity session with no insert and no console slot, so no plan latency delays an edit's
/// effect at the output and no effect state makes a running plan differ from a fresh one.
fn without_inserts(rate: u32) -> SessionModel {
    let mut model = model_at(rate);
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.inserts.effects.clear();
        track.console.clear();
    }
    for submix in &mut model.submixes {
        submix.console.clear();
    }
    model
}

/// An edit of one control that takes an optional length of its own.
type RampedEdit = fn(Option<u32>) -> SessionEdit;

/// One plane of every block, concatenated.
fn plane(blocks: &[Vec<u32>], plane: usize) -> Vec<u32> {
    blocks
        .iter()
        .flat_map(|block| block[plane * QUANTUM as usize..(plane + 1) * QUANTUM as usize].to_vec())
        .collect()
}

/// Gate 2: the ramp of an edit is the edit's own length, and an absent one is the session's.
///
/// Sample-exact on a session with no insert, so the edit reaches the output in the block it is
/// made before: 0 steps at the first sample of that block, 480 has reached the target 480 samples
/// later, and an absent length is the session's own.
///
/// Test value: red if the browser drops a per-edit ramp (a zero would not step, 480 would not
/// reach the target at its sample), or reads an absent ramp as a length instead of the session's.
#[test]
fn ramps_carry_end_to_end() {
    const EDIT_BLOCK: u64 = 4;
    const BLOCKS_RUN: u64 = 14;
    let first_sample = (EDIT_BLOCK * u64::from(QUANTUM)) as usize;
    let model = without_inserts(RATE);
    let plain = {
        let mut rig = Rig::new(&model);
        (0..BLOCKS_RUN)
            .map(|block| rig.step(block))
            .collect::<Vec<_>>()
    };
    let smoothing = model.control_smoothing_samples();
    let edits: [(&str, RampedEdit, u32); 3] = [
        (
            "fader",
            |ramp| {
                set(
                    strip_row("t1", FADER_DB, ParameterChannel::Both),
                    -20.0,
                    ramp,
                )
            },
            smoothing.fader,
        ),
        (
            "mute",
            |ramp| set(strip_row("t3", MUTE, ParameterChannel::Both), 1.0, ramp),
            smoothing.mute,
        ),
        (
            "pan",
            |ramp| set(strip_row("t2", PAN, ParameterChannel::Left), 0.9, ramp),
            smoothing.pan,
        ),
    ];
    // The left plane: every edit above reaches it.
    let plane_index = 0;
    for (label, edit, session_length) in edits {
        let run =
            |ramp: Option<u32>| run_script(&model, &[(EDIT_BLOCK, vec![edit(ramp)])], BLOCKS_RUN);
        let committed = {
            let mut rig = Rig::new(&model);
            rig.transaction(&[edit(Some(0))]);
            document(&mut rig)
        };
        let target = plane(&fresh_blocks(&committed, BLOCKS_RUN, |_| {}), plane_index);
        let untouched = plane(&plain, plane_index);

        // 0 steps at the first sample of the next block: nothing before it moved, everything from
        // it on is the committed model's.
        let zero = plane(&run(Some(0)), plane_index);
        assert_eq!(
            zero[..first_sample],
            untouched[..first_sample],
            "{label}: nothing moves early"
        );
        assert_eq!(
            zero[first_sample..],
            target[first_sample..],
            "{label}: 0 steps at the next block"
        );

        // 480 samples: the target is the 480th sample of the ramp, and no sample before it is.
        let ramped = plane(&run(Some(480)), plane_index);
        assert_eq!(ramped[..first_sample], untouched[..first_sample]);
        let reached = (first_sample..ramped.len())
            .find(|&at| ramped[at..] == target[at..])
            .expect("the ramp reaches the target");
        assert_eq!(
            reached - first_sample,
            479,
            "{label}: the 480th sample of the ramp is the target"
        );
        assert_ne!(
            ramped[first_sample + 240],
            target[first_sample + 240],
            "{label}: half way"
        );

        // Absent: the session's length, which is the same as saying it, and not a step.
        let absent = run(None);
        let explicit = run(Some(session_length));
        assert_eq!(
            first_difference(&absent, &explicit),
            None,
            "{label}: an absent ramp is the session's {session_length} samples"
        );
        assert_ne!(
            plane(&absent, plane_index),
            zero,
            "{label}: the session ramp is not a step"
        );
    }
}

/// Gate 6: the ramp table follows the committed model. A session setting committed through the
/// same entry governs the next edit that gives no length.
///
/// Test value: red if any browser path resolves an absent ramp from a table built at boot.
#[test]
fn the_ramp_table_follows_the_committed_model() {
    const BLOCKS_RUN: u64 = 14;
    let model = model_at(RATE);
    // Leg 1: `mute_ms` 5 is 240 samples at 48 kHz.
    let mute = |ramp| set(strip_row("t3", MUTE, ParameterChannel::Both), 1.0, ramp);
    let session_mute_ms = set(
        address(
            Owner::Session,
            Section::Own,
            SESSION_MUTE_MS,
            ParameterChannel::Both,
        ),
        5.0,
        None,
    );
    let governed = run_script(
        &model,
        &[(2, vec![session_mute_ms]), (4, vec![mute(None)])],
        BLOCKS_RUN,
    );
    let explicit = run_script(&model, &[(4, vec![mute(Some(240))])], BLOCKS_RUN);
    let default = run_script(&model, &[(4, vec![mute(None)])], BLOCKS_RUN);
    assert_eq!(
        first_difference(&governed, &explicit),
        None,
        "a mute with no length ramps over the committed 5 ms"
    );
    assert_ne!(
        first_difference(&governed, &default),
        None,
        "the setting changed the ramp"
    );

    // Leg 2: `t1`'s pan window 100 governs its next pan.
    let pan = |ramp| set(strip_row("t1", PAN, ParameterChannel::Left), 0.5, ramp);
    let window = set(strip_own("t1", PAN_WINDOW_SAMPLES), 100.0, None);
    let governed = run_script(
        &model,
        &[(2, vec![window]), (4, vec![pan(None)])],
        BLOCKS_RUN,
    );
    let explicit = run_script(&model, &[(4, vec![pan(Some(100))])], BLOCKS_RUN);
    let default = run_script(&model, &[(4, vec![pan(None)])], BLOCKS_RUN);
    assert_eq!(
        first_difference(&governed, &explicit),
        None,
        "a pan with no length ramps over the strip's committed window"
    );
    assert_ne!(
        first_difference(&governed, &default),
        None,
        "the window changed the ramp"
    );
}

/// D1: the entry serves every frame `SessionState::command` accepts, with the C ABI's bytes.
///
/// Test value: red if the browser decodes or filters the frame, answers a query itself, or serves
/// the committed document other than as the model's canonical JSON.
#[test]
fn the_entry_serves_queries_and_refuses_bad_frames_without_failing_the_host() {
    let model = model_at(RATE);
    let mut rig = Rig::new(&model);
    // The committed document is the model's canonical JSON.
    assert_eq!(
        document(&mut rig),
        canonical_session_json(&model)
            .expect("canonical")
            .into_bytes()
    );
    // The control list: the first row is the first strip's owner header.
    let request = ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 4,
        owner: None,
    };
    let response = rig.command(
        CommandPayload::ControlListGet(&request),
        ExpectedRevision::Any,
    );
    let mut fields = [0_u16; 4096];
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::ControlListPage(page),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("a control page")
    else {
        panic!("expected a control list page")
    };
    assert_eq!(page.entries.len(), 4);
    assert_eq!(page.entries[0].handle, 1);
    assert!(format!("{:?}", page.entries[0]).contains("OwnerHeader"));

    // A frame the core cannot correlate is an invalid argument, and the host stays ready.
    let (result, _) = rig.send_frame(&[0xff; 24]);
    assert_eq!(result, RESULT_INVALID_ARGUMENT);
    assert!(
        rig.host.control_request_mut(0).is_none(),
        "an empty frame has no staging"
    );
    assert!(
        rig.host.control_request_mut(16_385).is_none(),
        "a frame over maximum_control_frame_bytes has no staging"
    );
    assert!(rig.host.control_request_mut(16_384).is_some());
    let verdict = rig.transaction(&[set(
        strip_row("t1", FADER_DB, ParameterChannel::Both),
        -3.0,
        None,
    )]);
    assert!(matches!(
        verdict,
        Verdict::Applied {
            path: EditPath::Live,
            ..
        }
    ));
    rig.step(0);
}
