//! A latency-growing edit during playback in the browser host (issue #1361 gate 3): the control
//! half and the render half on two threads, as the Worker and the AudioWorklet hold them.
//!
//! The warm classification, the deadline step and the prime are the control plane's and the
//! engine's (#1360, #1403, #1355), and their tests own them. This binary holds what the browser host
//! adds: its source feed reaches the warm successor's rings with no service step
//! (`submit_unserviced`), its render half adopts the successor with no control call, its
//! [`AudioWorkletEngineHost::service`] runs the deadline step, and its render status words
//! (`watermark_*`, #1349) report the outcome.
//!
//! The session is the nine-track fixture at 48 kHz in 128-frame blocks, with the default source
//! ring. The growth adds a muted copy of the first track with a true-peak limiter insert and its
//! route to the output, as `crates/capi/tests/latency_growth.rs` does: `P` = 512. A muted track adds
//! exact `+0.0` to the output sum, so an exact adoption leaves the predecessor's output continued
//! bit for bit (D15-8).

#![cfg(feature = "test-support")]

use host_web::{AudioWorkletEngineHost, RESULT_OK, RenderHost, WebBootOptions, WebStatus};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision, StatusCode,
    TypedCommandFrame,
};
use session::{SessionModel, StableId};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM: u32 = 128;
const Q: u64 = QUANTUM as u64;
/// Blocks rendered before the growth.
const BEFORE: u64 = 6;
/// How far past the render clock the exact case queues the source before the growth: more than
/// `2P + 2q` (the adoption moves the read position `P` ahead and the prime needs `P + q` past it),
/// and inside the default ring.
const FEED_AHEAD: u64 = 4_096;
/// The session's mute ramp: the fixture's 10 ms at 48 kHz.
const MUTE_RAMP: u64 = 480;

fn model() -> SessionModel {
    let model = session::parse_session_json(FIXTURE).expect("fixture");
    assert_eq!(
        (model.sample_rate_hz, model.quantum_frames),
        (48_000, QUANTUM)
    );
    model
}

/// The growth: a muted copy of the first track named `grown`, with a true-peak limiter insert,
/// and its route to the output.
fn growth(model: &SessionModel) -> Vec<SessionEdit> {
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse("grown").expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    track.inserts.effects.push(session::Effect {
        id: StableId::parse("grown-limiter").expect("effect ID"),
        identity: session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.true-peak-limiter").expect("effect ID"),
        },
        quality: session::EffectQuality::Normal,
        bypass: false,
        link_mode: session::LinkMode::Maximum,
        params: Vec::new(),
        sidechain: session::SidechainDeclaration::None,
    });
    let mut route = model.routes[0].clone();
    route.id = StableId::parse("grown-main").expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    vec![
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

/// The source's frame `frame`: never zero, different from frame to frame.
fn pcm(frame: u64) -> [f32; 2] {
    let step = (frame.wrapping_mul(7_919) % 1_999) as f32;
    [(step - 999.5) / 2_000.0, (999.5 - step) / 4_000.0]
}

/// The control half, on the test's thread: the Worker's side.
struct Control {
    host: AudioWorkletEngineHost,
    request: u64,
    revision: u64,
    /// The next source frame to submit.
    fed: u64,
}

/// The render half and what it rendered.
struct Render {
    half: RenderHost,
    /// Every rendered block's output bits.
    blocks: Vec<Vec<u32>>,
}

fn boot() -> (Control, Render) {
    let model = model();
    let document = session::canonical_session_json(&model).expect("canonical session");
    let options = WebBootOptions {
        require_quantum_frames: QUANTUM,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options)
        .unwrap_or_else(|failure| panic!("boot: {:?}", failure.diagnostic()));
    let half = host.release_render().expect("the render half");
    (
        Control {
            host,
            request: 0,
            revision: model.revision,
            fed: 0,
        },
        Render {
            half,
            blocks: Vec::new(),
        },
    )
}

impl Control {
    /// Applies `edits` through the production control entry: the transaction must commit with
    /// path `rebuild`. Returns its revision.
    fn apply(&mut self, edits: &[SessionEdit]) -> u64 {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let mut frame = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut frame,
            )
            .expect("command frame");
        self.host
            .control_request_mut(length as u32)
            .expect("the control staging")[..length]
            .copy_from_slice(&frame[..length]);
        assert_eq!(self.host.control_command(length as u32), RESULT_OK);
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
        } = codec
            .decode_typed_response(
                self.host.control_response(),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("transaction reply")
        else {
            panic!("the growth is refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        assert_eq!(applied.edit_path, EditPath::Rebuild);
        self.revision += 1;
        assert_eq!(header.revision, SessionRevision(self.revision));
        self.revision
    }

    /// Submits whole blocks of the source until `until` frames are queued.
    fn feed_to(&mut self, until: u64) {
        while self.fed < until {
            let mut planes = [vec![0.0_f32; QUANTUM as usize], vec![0.0; QUANTUM as usize]];
            for index in 0..QUANTUM as usize {
                [planes[0][index], planes[1][index]] = pcm(self.fed + index as u64);
            }
            let views: [&[f32]; 2] = [&planes[0], &planes[1]];
            let result = self
                .host
                .submit_source(SOURCE_ID, 1, self.fed, 48_000, &views, QUANTUM, false);
            assert_eq!(result, RESULT_OK, "frame {}", self.fed);
            self.fed += Q;
        }
    }

    fn service(&mut self) {
        assert_eq!(self.host.service(), RESULT_OK);
    }

    fn dispose(mut self, render: Render) {
        assert!(
            self.host.adopt_render(render.half).is_ok(),
            "the host takes its render half back"
        );
        assert_eq!(self.host.dispose(), RESULT_OK);
    }
}

impl Render {
    /// Renders `blocks` blocks on a render thread of its own, as the worklet renders.
    fn render_on_thread(&mut self, blocks: u64) {
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    for _ in 0..blocks {
                        let at = self.half.status().next_absolute_sample;
                        assert_eq!(self.half.render_next(), RESULT_OK, "render at {at}");
                        self.blocks.push(
                            self.half
                                .output_pcm()
                                .expect("output")
                                .iter()
                                .map(|sample| sample.to_bits())
                                .collect(),
                        );
                    }
                })
                .join()
                .expect("the render thread completes");
        });
    }

    fn at(&self) -> u64 {
        self.half.status().next_absolute_sample
    }

    fn status(&self) -> &WebStatus {
        self.half.status()
    }
}

/// The predecessor rendered on with no edit: before each block the source is queued to
/// `feed(block)` frames.
fn predecessor(blocks: u64, feed: impl Fn(u64) -> u64) -> Vec<Vec<u32>> {
    let (mut control, mut render) = boot();
    for block in 0..blocks {
        control.feed_to(feed(block).max(control.fed));
        render.render_on_thread(1);
    }
    let bits = core::mem::take(&mut render.blocks);
    control.dispose(render);
    bits
}

/// The first block whose bits differ, if any.
fn first_divergence(left: &[Vec<u32>], right: &[Vec<u32>]) -> Option<usize> {
    assert_eq!(left.len(), right.len());
    left.iter().zip(right).position(|(a, b)| a != b)
}

/// Exact with no service call. After [`BEFORE`] blocks the control thread queues the source
/// [`FEED_AHEAD`] past the render clock (at least `P + q`) and applies the growth. The render thread
/// then renders 16 blocks while the control thread makes no call: the render status's watermark
/// words read the growth's revision with `EXACT`, and the output equals the predecessor rendered on.
///
/// Test value: a browser feed that does not reach the warm successor's rings without a service
/// step, or a render half that needs a control call to adopt, leaves the revision pending; a render
/// half that does not copy the report into its status words reads the initial revision; a growth
/// published as an ordinary successor floors the output by `P`.
#[test]
fn a_growth_fed_ahead_is_adopted_exact_with_no_service_call() {
    const AFTER: u64 = 16;
    let (mut control, mut render) = boot();
    for _ in 0..BEFORE {
        control.feed_to(render.at() + Q);
        render.render_on_thread(1);
    }
    control.feed_to(render.at() + FEED_AHEAD);
    let revision = control.apply(&growth(&model()));
    assert!(
        render.status().watermark_revision < revision,
        "pending at submit"
    );
    render.render_on_thread(AFTER);
    let status = *render.status();
    assert_eq!(
        (status.watermark_revision, status.watermark_outcome_flags),
        (revision, capi::OUTCOME_EXACT)
    );
    let continued = predecessor(BEFORE + AFTER, |block| {
        if block < BEFORE {
            (block + 1) * Q
        } else {
            BEFORE * Q + FEED_AHEAD
        }
    });
    assert_eq!(first_divergence(&render.blocks, &continued), None);
    control.dispose(render);
}

/// The transition only at a service call past the deadline. The source is fed only the block
/// render reads next, so the growth is never ready. After the growth the render thread renders for
/// twice the deadline while the control thread makes no call (the source runs dry after one
/// block): the revision stays pending in the render status, and every block equals the predecessor
/// rendered on with the same feed. The next [`AudioWorkletEngineHost::service`] publishes the
/// transition, which render adopts at its `S`, and the status words report
/// `TRANSITION_FALLBACK`.
///
/// Test value: a browser service loop that does not reach the deadline step leaves the revision
/// pending after the service call, and a fallback started on the render thread or counted in wall
/// time completes it with no service call or at another sample.
#[test]
fn a_growth_never_ready_falls_back_at_the_next_service_call() {
    let deadline = host_core::warm::prime_deadline_samples(48_000, QUANTUM);
    let withheld = 2 * deadline / Q;
    let (mut control, mut render) = boot();
    for _ in 0..BEFORE {
        control.feed_to(render.at() + Q);
        render.render_on_thread(1);
    }
    control.feed_to(render.at() + Q);
    let revision = control.apply(&growth(&model()));
    render.render_on_thread(withheld);
    assert!(
        render.status().watermark_revision < revision,
        "pending at {}, past the deadline",
        render.at()
    );
    let continued = predecessor(BEFORE + withheld, |block| (block + 1).min(BEFORE + 1) * Q);
    assert_eq!(first_divergence(&render.blocks, &continued), None);

    control.service();
    let published = render.at();
    // The transition ducks the grown strips and is adopted at its `S`: ceil_q(clock + q + N).
    let s = (published + Q + MUTE_RAMP).div_ceil(Q) * Q;
    render.render_on_thread((s - published) / Q + 1);
    let status = *render.status();
    assert_eq!(
        (
            status.watermark_revision,
            status.watermark_first_sample,
            status.watermark_outcome_flags
        ),
        (revision, s, capi::OUTCOME_TRANSITION_FALLBACK)
    );
    control.dispose(render);
}
