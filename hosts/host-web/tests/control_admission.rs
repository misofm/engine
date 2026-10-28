//! Issue #1562a (slice S10a of decision 17): the browser host commits live edits through the
//! control plane's `SessionState::command`, reached by the generic command-frame entry of D1.
//! The tests here hold the browser-side gates that no other test holds: a rebuild keeps the solo
//! (gate 3's structural part), a refused transaction changes nothing (gate 4), the ramp table
//! follows the committed model (gate 6), and the entry serves queries and refuses bad frames (D1).
//! Gates 1, 2 and 3's live part (every live family, the per-edit ramp and the solo overlay) are
//! held in host-core and reach the browser through `tests/control_parity.rs`.
//!
//! Every test drives `AudioWorkletEngineHost` through `control_request_mut` and `control_command`,
//! the calls the exports make, with the frames the C ABI would receive and nothing decoded by the
//! host.

#![cfg(feature = "test-support")]

use controls::{ControlAddress, Lane, Owner, Section};
use host_web::RESULT_INVALID_ARGUMENT;
use protocol::{
    CommandPayload, ControlListRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, SessionEdit,
};
use session::{ParameterChannel, SessionModel, StableId, canonical_session_json};

mod common;
use common::*;

const RATE: u32 = 48_000;
const PAN_WINDOW_SAMPLES: u32 = 3;
const SESSION_MUTE_MS: u32 = 2;
const DELAY_SAMPLES: u32 = 11;

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

fn set(address: ControlAddress, value: f32, ramp: Option<u32>) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address,
        value,
        ramp_samples: ramp,
    }
}

/// The committed document of a rig, as the snapshot query serves it.
fn document(rig: &mut Rig) -> Vec<u8> {
    snapshot_bytes(|payload, expected| rig.command(payload, expected))
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
