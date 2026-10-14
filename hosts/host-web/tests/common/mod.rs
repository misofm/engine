//! Shared fixtures of the host-web control tests (issue #1562a): the parity session of #1560, its
//! deterministic PCM, and the browser leg's frame helpers.

#![allow(dead_code)]

use host_web::{AudioWorkletEngineHost, RESULT_OK, WebBootOptions};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    SessionSnapshotRequest, StatusCode, TypedCommandFrame,
};
use session::{
    DualMonoFader, EffectIdentity, EffectParam, MatrixOrPan, ParameterChannel, ParameterUnit,
    RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix, Vca,
    canonical_session_json, parse_session_json,
};

pub(crate) const QUANTUM: u32 = 128;
pub(crate) const BLOCKS: u64 = 64;
/// A VCA by its stable ID.
#[derive(Clone, Copy)]
pub(crate) struct VcaRef {
    pub(crate) id: &'static str,
}

pub(crate) const VCA: VcaRef = VcaRef { id: "v" };
/// A VCA the session does not have.
pub(crate) const NO_VCA: VcaRef = VcaRef { id: "nobody" };

/// The strip window of `t1`'s pan, in samples. `t2`'s pan has none (`pan_ms` applies).
pub(crate) const PAN_WINDOW: u32 = 200;
// The builtin and own-row IDs `SET_CONTROL` addresses (`controls`' tables).
pub(crate) const POLARITY_INVERT: u32 = 1;
pub(crate) const TRIM_DB: u32 = 2;
pub(crate) const HPF_HZ: u32 = 3;
pub(crate) const LPF_HZ: u32 = 4;
pub(crate) const FADER_DB: u32 = 5;
pub(crate) const MUTE: u32 = 6;
pub(crate) const MATRIX_LL: u32 = 7;
pub(crate) const PAN: u32 = 12;
pub(crate) const SOLO: u32 = 2;
pub(crate) const ROUTE_GAIN_DB: u32 = 1;
pub(crate) const ROUTE_MUTE: u32 = 2;
pub(crate) const ROUTE_MATRIX_LL: u32 = 3;
pub(crate) const VCA_OFFSET_DB: u32 = 1;
pub(crate) const VCA_MUTE: u32 = 2;

/// A strip of the session and its browser strip index (tracks in ID order, then submixes).
#[derive(Clone, Copy)]
pub(crate) struct Strip {
    pub(crate) id: &'static str,
    pub(crate) index: u32,
}

pub(crate) const T1: Strip = Strip { id: "t1", index: 0 };
pub(crate) const T2: Strip = Strip { id: "t2", index: 1 };
pub(crate) const T3: Strip = Strip { id: "t3", index: 2 };
pub(crate) const BUS: Strip = Strip { id: "b", index: 3 };

/// One insert of a track: its ID, its native identity and its position in the track's inserts
/// (the browser's `effect_index`).
#[derive(Clone, Copy)]
pub(crate) struct Insert {
    pub(crate) track: Strip,
    pub(crate) id: &'static str,
    pub(crate) native: &'static str,
    pub(crate) position: u32,
}

pub(crate) const EQ: Insert = Insert {
    track: T1,
    id: "eq",
    native: "miso.parametric-eq",
    position: 0,
};
pub(crate) const COMPRESSOR: Insert = Insert {
    track: T1,
    id: "comp",
    native: "miso.compressor",
    position: 1,
};
pub(crate) const GATE: Insert = Insert {
    track: T2,
    id: "gate",
    native: "miso.gate-expander",
    position: 0,
};
pub(crate) const CLIP: Insert = Insert {
    track: T2,
    id: "clip",
    native: "miso.soft-clip",
    position: 1,
};
pub(crate) const SHAPER: Insert = Insert {
    track: T2,
    id: "shaper",
    native: "miso.transient-shaper",
    position: 2,
};
pub(crate) const MULTIBAND: Insert = Insert {
    track: T3,
    id: "multi",
    native: "miso.multiband-compressor",
    position: 0,
};
pub(crate) const LIMITER: Insert = Insert {
    track: T3,
    id: "limiter",
    native: "miso.true-peak-limiter",
    position: 1,
};
pub(crate) const DELAY: Insert = Insert {
    track: T3,
    id: "delay",
    native: "miso.delay",
    position: 2,
};
pub(crate) const INSERTS: [Insert; 8] = [
    EQ, COMPRESSOR, GATE, CLIP, SHAPER, MULTIBAND, LIMITER, DELAY,
];

/// The session at `rate`: tracks `t1`..`t3` over their own sources, submix `b` routed to the
/// output, `t1` sending `pre_fader` into `b` with `follows_mute`, VCA `v` holding `t2`, and one
/// instance of every launch native effect over the tracks.
pub(crate) fn model_at(rate: u32) -> SessionModel {
    let mut model = parse_session_json(include_str!("../browser-v1/command-session.json"))
        .expect("accepted command fixture");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM;
    let source = model.sources[0].clone();
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    // The fixture's EQ has band 1 enabled, as a low shelf at 1 kHz, so its gain is audible.
    let eq = track.inserts.effects[0].clone();
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    for (index, strip) in [T1, T2, T3].into_iter().enumerate() {
        let mut source = source.clone();
        source.id = StableId::parse(&format!("src-{}", strip.id)).expect("source ID");
        source.frames = u64::from(QUANTUM) * (BLOCKS + 8);
        let mut track = track.clone();
        track.id = StableId::parse(strip.id).expect("track ID");
        track.source_id = source.id.clone();
        track.fader.left_db = -2.0 - index as f32;
        track.fader.right_db = -2.0 - index as f32;
        track.matrix_or_pan = match strip.id {
            "t1" => MatrixOrPan::Pan {
                left: -0.8,
                right: 0.6,
                smoothing_samples: PAN_WINDOW,
            },
            "t2" => MatrixOrPan::Pan {
                left: -0.5,
                right: 0.4,
                smoothing_samples: 0,
            },
            _ => MatrixOrPan::Matrix {
                ll: 0.9,
                lr: 0.1,
                rl: 0.2,
                rr: 0.8,
                smoothing_samples: 0,
            },
        };
        track.inserts.effects = INSERTS
            .iter()
            .filter(|insert| insert.track.id == strip.id)
            .map(|insert| {
                let mut effect = eq.clone();
                effect.id = StableId::parse(insert.id).expect("effect ID");
                effect.identity = EffectIdentity::Native {
                    effect_id: StableId::parse(insert.native).expect("native effect ID"),
                };
                if insert.id != "eq" {
                    // Every other insert takes its default parameters, but for the gate's hold:
                    // its default (100 ms) keeps an open gate open for longer than the run's tail
                    // at 96 kHz, so a threshold edit would never be heard there.
                    effect.params.clear();
                }
                if insert.id == "gate" {
                    effect.params.push(EffectParam {
                        parameter_id: 6,
                        channel: ParameterChannel::Both,
                        unit: ParameterUnit::Milliseconds,
                        value: 0.0,
                    });
                }
                effect
            })
            .collect();
        let mut route = route.clone();
        route.id = StableId::parse(&format!("{}-main", strip.id)).expect("route ID");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
    }
    let bus = StableId::parse(BUS.id).expect("submix ID");
    model
        .submixes
        .push(Submix::unity(bus.clone(), &model.console));
    let mut send = model.routes[0].clone();
    send.id = StableId::parse("t1-b").expect("route ID");
    send.source = RouteSource::Track {
        track_id: StableId::parse(T1.id).expect("track ID"),
        tap: SendTap::PreFader,
    };
    send.destination = RouteDestination::SubmixInput {
        submix_id: bus.clone(),
    };
    send.follows_mute = true;
    let mut out = model.routes[0].clone();
    out.id = StableId::parse("b-main").expect("route ID");
    out.source = RouteSource::Submix {
        submix_id: bus,
        tap: SendTap::PostPan,
    };
    model.routes.push(send);
    model.routes.push(out);
    model.vcas.push(Vca {
        id: StableId::parse(VCA.id).expect("VCA ID"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![StableId::parse(T2.id).expect("track ID")],
    });
    model
}

/// A deterministic, never-silent sample for source `feed`, lane `lane`, absolute frame `frame`.
pub(crate) fn sample(feed: u64, lane: u64, frame: u64) -> f32 {
    let mut z = (feed + 1)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(lane.wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add(frame.wrapping_mul(0x94D0_49BB_1331_11EB));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let unit = (z >> 40) as f32 / (1_u64 << 24) as f32 * 2.0 - 1.0;
    // Source 1 (`t2`) is quiet, so a gate threshold edit has a level to act on; the others are loud.
    unit * if feed == 1 { 0.25 } else { 0.8 }
}

pub(crate) fn planes(feed: u64, block: u64) -> [Vec<f32>; 2] {
    let first = block * u64::from(QUANTUM);
    [0, 1].map(|lane| {
        (0..u64::from(QUANTUM))
            .map(|offset| sample(feed, lane, first + offset))
            .collect()
    })
}

pub(crate) fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
}

/// What the control plane said about one transaction.
#[derive(Debug, PartialEq)]
pub(crate) enum Verdict {
    /// Committed, with the path and the revision the reply names.
    Applied { path: EditPath, revision: u64 },
    /// Refused with the control codes of its diagnostics, and the revision the reply reports.
    Refused { codes: Vec<String>, revision: u64 },
}

/// Decode a `SESSION_TRANSACTION_APPLY` response.
pub(crate) fn decode_verdict(response: &[u8]) -> Verdict {
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("transaction response")
    {
        DecodedTypedResponseFrame::Success { header, payload } => {
            let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload else {
                panic!("expected a transaction success")
            };
            assert_eq!(header.status, StatusCode::Ok);
            Verdict::Applied {
                path: applied.edit_path,
                revision: header.revision.0,
            }
        }
        DecodedTypedResponseFrame::NonOk { header, payload } => Verdict::Refused {
            codes: payload
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect(),
            revision: header.revision.0,
        },
    }
}

/// The browser leg: a native `AudioWorkletEngineHost` driven through the generic command-frame
/// entry (issue #1562a D1), as the worklet's message handler drives it, with one request-ID
/// allocator for every frame it sends.
pub(crate) struct Rig {
    pub(crate) host: AudioWorkletEngineHost,
    pub(crate) rate: u32,
    /// The committed revision the last reply named.
    pub(crate) revision: u64,
    request: u64,
}

impl Rig {
    pub(crate) fn new(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical session");
        Self::boot(document.as_bytes(), model.sample_rate_hz)
    }

    pub(crate) fn boot(document: &[u8], rate: u32) -> Self {
        let options = WebBootOptions {
            require_sample_rate_hz: rate,
            require_quantum_frames: QUANTUM,
            source_ring_frames: QUANTUM * 2,
            live_control_command_queue_records: 64,
            ..WebBootOptions::explicit_defaults()
        };
        let host = AudioWorkletEngineHost::boot(document, options)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())));
        let revision = host.committed_revision();
        Self {
            host,
            rate,
            revision,
            request: 0,
        }
    }

    /// Encode one command frame with the next request ID.
    pub(crate) fn encode(
        &mut self,
        payload: CommandPayload<'_>,
        expected: ExpectedRevision,
    ) -> Vec<u8> {
        self.request += 1;
        let mut bytes = vec![0_u8; 16 << 10];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: expected,
                    payload,
                },
                &mut bytes,
            )
            .expect("command frame");
        bytes.truncate(length);
        bytes
    }

    /// Stage `frame` and run it. Returns the export's result and the response bytes (the response
    /// frame after `RESULT_OK`, else the refusal's diagnostic).
    pub(crate) fn send_frame(&mut self, frame: &[u8]) -> (u32, Vec<u8>) {
        let length = u32::try_from(frame.len()).expect("frame length");
        let staging = self
            .host
            .control_request_mut(length)
            .expect("the control request staging");
        staging[..frame.len()].copy_from_slice(frame);
        let result = self.host.control_command(length);
        (result, self.host.control_response().to_vec())
    }

    /// One command whose call must be ok, with its response frame.
    pub(crate) fn command(
        &mut self,
        payload: CommandPayload<'_>,
        expected: ExpectedRevision,
    ) -> Vec<u8> {
        let frame = self.encode(payload, expected);
        let (result, response) = self.send_frame(&frame);
        assert_eq!(result, RESULT_OK, "{}", String::from_utf8_lossy(&response));
        response
    }

    /// One transaction at the committed revision.
    pub(crate) fn transaction(&mut self, edits: &[SessionEdit]) -> Verdict {
        let response = self.command(
            CommandPayload::SessionTransactionApply(edits),
            ExpectedRevision::Exact(SessionRevision(self.revision)),
        );
        let verdict = decode_verdict(&response);
        if let Verdict::Applied { revision, .. } = &verdict {
            self.revision = *revision;
        }
        // A commit queues its reliable events, whose reader is #1570b's.
        self.host.drain_reliable_events_for_test();
        verdict
    }

    /// Submit block `block`'s PCM for the three tracks and render it. Returns the output's bits.
    pub(crate) fn step(&mut self, block: u64) -> Vec<u32> {
        for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
            let [left, right] = planes(feed as u64, block);
            assert_eq!(
                self.host.submit_source(
                    format!("src-{}", strip.id).as_bytes(),
                    1,
                    block * u64::from(QUANTUM),
                    self.rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                RESULT_OK,
                "browser source {}",
                strip.id
            );
        }
        assert_eq!(self.host.render_next(), RESULT_OK, "browser render");
        bits(self.host.output_pcm().expect("browser output"))
    }
}

/// The committed document, paged through `SESSION_SNAPSHOT_GET` over one host's command entry.
pub(crate) fn snapshot_bytes(
    mut send: impl FnMut(CommandPayload<'_>, ExpectedRevision) -> Vec<u8>,
) -> Vec<u8> {
    let mut snapshot = Vec::new();
    loop {
        let bytes = send(
            CommandPayload::SessionSnapshotGet(SessionSnapshotRequest {
                offset: snapshot.len() as u64,
                maximum_bytes: 2_048,
            }),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SessionSnapshot(chunk),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("a snapshot chunk")
        else {
            panic!("expected a snapshot chunk")
        };
        snapshot.extend_from_slice(chunk.canonical_json_chunk);
        if chunk.eof {
            break;
        }
    }
    snapshot
}
