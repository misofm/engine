//! Non-timed Issue-022 audit plan for the exported C render entrypoint.
//!
//! Every [`LIVE_EDIT_PERIOD`]th call first commits a live edit through
//! `miso_engine_v1_submit_command` (#1258 gate 2, D3), cycling through five kinds: a mute toggle
//! on one track, a pan move on another, a compressor threshold change (#1264), a parametric EQ
//! band gain change (#1265) and a compressor bypass toggle (#1266). So the audited render drains
//! non-empty fader and matrix lanes, effect parameter records, designed EQ targets and a live
//! bypass record. Each edit and each event dequeue runs on this thread outside the audited scope;
//! only the render call is inside it, one scope per call.

#![allow(unsafe_code)]

use bench_support::alloc as bench_alloc;
use std::ptr;

use capi::{
    ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits, ENGINE_CONFIG_SIZE,
    EVENT_LANE_RELIABLE, Engine, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput,
    RESULT_INTERNAL, RESULT_OK, SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SubmitReport, miso_engine_v1_compile_session, miso_engine_v1_dequeue_event,
    miso_engine_v1_engine_create, miso_engine_v1_engine_destroy, miso_engine_v1_plan_destroy,
    miso_engine_v1_render_f32_planar, miso_engine_v1_session_destroy,
    miso_engine_v1_source_submit_planar_f32, miso_engine_v1_submit_command,
};
use engine::realtime::audit::{self, AuditSnapshot};
use protocol::{
    CommandPayload, DecodeScratch, DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec,
    RequestId, SessionEdit, SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{
    DualMonoFader, EffectIdentity, EffectParam, MatrixOrPan, ParameterChannel, ParameterUnit,
    RackName, StableId,
};

const CALLS: u64 = 100_000;
/// Every this many render calls, one live edit precedes the call (#1258 D3).
const LIVE_EDIT_PERIOD: u64 = 64;
/// The fixture's committed revision.
const INITIAL_REVISION: u64 = 42;
const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM_FRAMES: usize = 128;
const FIXTURE_JSON: &str =
    include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
/// The track that carries the audited compressor insert.
const COMPRESSOR_TRACK: &str = "eq2";
/// The audited compressor insert's instance ID.
const COMPRESSOR: &str = "comp";

/// The nine-track EQ fixture with an enabled compressor insert (`comp`) on `eq2`, so the live
/// editor also reaches a non-EQ effect lane. Every track keeps its console EQ slot, whose band 1
/// is enabled on `eq0`.
fn audit_session_json() -> String {
    let mut model = session::parse_session_json(FIXTURE_JSON).expect("nine-track EQ fixture");
    let mut compressor = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    compressor.id = StableId::parse(COMPRESSOR).expect("compressor instance");
    compressor.identity = EffectIdentity::Native {
        effect_id: StableId::parse("miso.compressor").expect("compressor ID"),
    };
    compressor.params.clear();
    compressor.bypass = false;
    model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == COMPRESSOR_TRACK)
        .expect("compressor track")
        .inserts
        .effects
        .push(compressor);
    session::canonical_session_json(&model).expect("canonical audit session")
}

struct AuditHandles {
    engine: *mut Engine,
    session: *mut Session,
    plan: *mut Plan,
}

impl AuditHandles {
    const fn empty() -> Self {
        Self {
            engine: ptr::null_mut(),
            session: ptr::null_mut(),
            plan: ptr::null_mut(),
        }
    }

    fn prepare() -> Result<Self, u32> {
        let mut handles = Self::empty();
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        // SAFETY: Configuration and output-pointer storage are valid for this complete call.
        let created = unsafe { miso_engine_v1_engine_create(&config, &mut handles.engine) };
        if created != RESULT_OK {
            return Err(created);
        }

        let limits = audit_limits();
        let session_json = audit_session_json();
        let mut diagnostic_storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: diagnostic_storage.as_mut_ptr(),
            capacity_bytes: diagnostic_storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The live engine, immutable JSON, fixed limits, diagnostic storage, and both
        // output locations remain valid throughout transactional compilation.
        let compiled = unsafe {
            miso_engine_v1_compile_session(
                handles.engine,
                session_json.as_ptr(),
                session_json.len() as u64,
                &limits,
                &mut diagnostics,
                &mut handles.session,
                &mut handles.plan,
            )
        };
        if compiled != RESULT_OK {
            return Err(compiled);
        }

        let (submitted, accepted_frames) = submit_source_chunk(handles.session, 0);
        if submitted != RESULT_OK || accepted_frames != QUANTUM_FRAMES as u64 {
            return Err(if submitted != RESULT_OK {
                submitted
            } else {
                RESULT_INTERNAL
            });
        }
        Ok(handles)
    }
}

/// Submits one generation-1 quantum of constant source at `start_frame`, returning the result
/// and the accepted frame count.
fn submit_source_chunk(session: *mut Session, start_frame: u64) -> (u32, u64) {
    let left = [0.25_f32; QUANTUM_FRAMES];
    let right = [-0.5_f32; QUANTUM_FRAMES];
    let planes = [left.as_ptr(), right.as_ptr()];
    let chunk = SourceChunk {
        struct_size: SOURCE_CHUNK_SIZE,
        sample_rate_hz: SAMPLE_RATE_HZ,
        generation: 1,
        start_frame,
        planes: planes.as_ptr(),
        plane_count: 2,
        frames: QUANTUM_FRAMES as u32,
        end_of_region: 0,
        reserved0: 0,
    };
    let mut report = SubmitReport {
        struct_size: SUBMIT_REPORT_SIZE,
        reserved0: 0,
        accepted_frames: 0,
        cumulative_written_frames: 0,
        active_generation: 0,
    };
    // SAFETY: The session is live and used by this thread alone, and all borrowed source
    // bytes/planes and report storage remain valid until the synchronous copy completes.
    let submitted = unsafe {
        miso_engine_v1_source_submit_planar_f32(
            session,
            b"fixture-source".as_ptr(),
            14,
            &chunk,
            &mut report,
        )
    };
    (submitted, report.accepted_frames)
}

impl Drop for AuditHandles {
    fn drop(&mut self) {
        // SAFETY: Every nonnull child is the unique quiescent handle published into this owner.
        // Destruction occurs after any render audit scope has ended, plan then session then engine.
        unsafe {
            miso_engine_v1_plan_destroy(self.plan);
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_engine_destroy(self.engine);
        }
        self.plan = ptr::null_mut();
        self.session = ptr::null_mut();
        self.engine = ptr::null_mut();
    }
}

/// How many kinds of live edit the editor cycles through.
const LIVE_EDIT_KINDS: u64 = 5;

/// The control-thread side of the audit: live fader-mute, pan, effect parameter, EQ parameter and
/// effect bypass edits, committed through the C entry point between render calls.
struct LiveEditor {
    session: *mut Session,
    revision: u64,
    request_id: u64,
    edits: u64,
    muted: bool,
    panned: bool,
    thresholded: bool,
    boosted: bool,
    bypassed: bool,
    command: Vec<u8>,
    response: [u8; 4_096],
    event: [u8; 4_096],
}

impl LiveEditor {
    fn new(session: *mut Session) -> Self {
        Self {
            session,
            revision: INITIAL_REVISION,
            request_id: 0,
            edits: 0,
            muted: false,
            panned: false,
            thresholded: false,
            boosted: false,
            bypassed: false,
            command: vec![0; 4_096],
            response: [0; 4_096],
            event: [0; 4_096],
        }
    }

    /// The next edit, in a cycle of [`LIVE_EDIT_KINDS`]: a mute toggle on `eq0`, a pan move on
    /// `eq1`, the compressor's threshold on `eq2`'s left lane, band 1's left gain on `eq0`'s
    /// console EQ (an enabled band, so it designs a target), and the compressor's bypass. Every
    /// edit changes its value, so each one pushes a record or a target to a lane.
    fn next_edit(&mut self) -> SessionEdit {
        let track = |id: &str| StableId::parse(id).expect("fixture track");
        let edit = match self.edits % LIVE_EDIT_KINDS {
            0 => {
                self.muted = !self.muted;
                SessionEdit::SetTrackFader {
                    track_id: track("eq0"),
                    fader: DualMonoFader {
                        left_db: 0.0,
                        right_db: 0.0,
                        left_mute: self.muted,
                        right_mute: self.muted,
                    },
                }
            }
            1 => {
                self.panned = !self.panned;
                let left = if self.panned { 0.5 } else { 1.0 };
                SessionEdit::SetTrackMatrixOrPan {
                    track_id: track("eq1"),
                    matrix_or_pan: MatrixOrPan::Pan {
                        left,
                        right: 1.0,
                        smoothing_samples: 16,
                    },
                }
            }
            2 => {
                self.thresholded = !self.thresholded;
                SessionEdit::UpsertEffectParam {
                    track_id: track(COMPRESSOR_TRACK),
                    rack_name: RackName::Inserts,
                    effect_id: StableId::parse(COMPRESSOR).expect("compressor instance"),
                    param: EffectParam {
                        parameter_id: 1,
                        channel: ParameterChannel::Left,
                        unit: ParameterUnit::Db,
                        value: if self.thresholded { -30.0 } else { -20.0 },
                    },
                }
            }
            3 => {
                self.boosted = !self.boosted;
                SessionEdit::UpsertEffectParam {
                    track_id: track("eq0"),
                    rack_name: RackName::Console,
                    effect_id: StableId::parse("eq").expect("console EQ slot"),
                    param: EffectParam {
                        parameter_id: 4,
                        channel: ParameterChannel::Left,
                        unit: ParameterUnit::Db,
                        value: if self.boosted { 3.0 } else { -3.0 },
                    },
                }
            }
            _ => {
                self.bypassed = !self.bypassed;
                SessionEdit::SetEffectBypass {
                    track_id: track(COMPRESSOR_TRACK),
                    rack_name: RackName::Inserts,
                    effect_id: StableId::parse(COMPRESSOR).expect("compressor instance"),
                    bypass: self.bypassed,
                }
            }
        };
        self.edits += 1;
        edit
    }

    /// Commits the next edit, then dequeues every reliable event so the next transaction finds
    /// room for its `SESSION_COMMITTED` event (#1258 D3). Any refusal fails the audit.
    fn commit_next(&mut self) {
        let edit = self.next_edit();
        self.request_id += 1;
        self.command.resize(4_096, 0);
        let len = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request_id).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
                },
                &mut self.command,
            )
            .expect("live edit command");
        self.command.truncate(len);
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: self.response.as_mut_ptr(),
            capacity_bytes: self.response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; the command and response
        // buffers are owned and complete for the synchronous call.
        let result = unsafe {
            miso_engine_v1_submit_command(
                self.session,
                self.command.as_ptr(),
                self.command.len() as u64,
                &mut output,
            )
        };
        assert_eq!(result, RESULT_OK, "live edit {}", self.edits);
        let mut fields = [0_u16; 64];
        let header = match ProtocolCodec::default()
            .decode_typed_response(
                &self.response[..output.required_bytes as usize],
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("live edit response")
        {
            DecodedTypedResponseFrame::Success { header, .. }
            | DecodedTypedResponseFrame::NonOk { header, .. } => header,
        };
        assert_eq!(header.status, StatusCode::Ok, "live edit {}", self.edits);
        self.revision += 1;
        assert_eq!(header.revision, SessionRevision(self.revision));
        loop {
            let mut event = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: self.event.as_mut_ptr(),
                capacity_bytes: self.event.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: As above; the event descriptor names complete owned storage.
            let result = unsafe {
                miso_engine_v1_dequeue_event(self.session, EVENT_LANE_RELIABLE, &mut event)
            };
            assert_eq!(result, RESULT_OK, "reliable dequeue");
            if event.required_bytes == 0 {
                break;
            }
        }
    }
}

struct PreparedAudit {
    handles: AuditHandles,
    output: [f32; QUANTUM_FRAMES * 2],
}

impl PreparedAudit {
    fn prepare() -> Result<Self, u32> {
        Ok(Self {
            handles: AuditHandles::prepare()?,
            output: [0.0; QUANTUM_FRAMES * 2],
        })
    }

    fn run(&mut self) -> AuditEvidence {
        let output_address = self.output.as_ptr() as usize;
        let plan = self.handles.plan;
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: self.output.as_mut_ptr(),
            sample_capacity: self.output.len() as u64,
            frames: QUANTUM_FRAMES as u32,
            plane_stride_samples: QUANTUM_FRAMES as u32,
            reserved: [0; 2],
        };
        let mut render_errors = 0_u64;
        let mut output_address_changes = 0_u64;
        let mut pcm_digest = 0xcbf2_9ce4_8422_2325_u64;
        let mut editor = LiveEditor::new(self.handles.session);
        audit::warm_up();
        audit::reset();
        for call in 0..CALLS {
            // The edit, its allocations and its event dequeues are control-thread work: they
            // run before the call's audited scope opens (#1258 D3).
            if call.is_multiple_of(LIVE_EDIT_PERIOD) {
                editor.commit_next();
            }
            // SAFETY: The plan is live and exclusive, the descriptor points to the same complete
            // writable output for every synchronous call, and exact time is bounded.
            let result = audit::in_render_scope(|| unsafe {
                miso_engine_v1_render_f32_planar(plan, call * QUANTUM_FRAMES as u64, &output)
            });
            if result != RESULT_OK {
                render_errors = render_errors.saturating_add(1);
            }
            if self.output.as_ptr() as usize != output_address {
                output_address_changes = output_address_changes.saturating_add(1);
            }
            for sample in &self.output {
                pcm_digest ^= u64::from(sample.to_bits());
                pcm_digest = pcm_digest.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        let snapshot = audit::snapshot();
        assert_eq!(editor.edits, CALLS.div_ceil(LIVE_EDIT_PERIOD));
        // The liveness witness: every edit was live, so the first plan still renders and its
        // source ring takes the next contiguous generation-1 chunk with no seek. After a
        // replacement the ring would refuse it until a seek, so an edit that silently rebuilt
        // fails the audit instead of auditing a path it never took.
        assert_eq!(
            submit_source_chunk(self.handles.session, QUANTUM_FRAMES as u64).0,
            RESULT_OK,
            "a live edit rebuilt the plan"
        );
        assert_eq!(render_errors, 0);
        assert_eq!(output_address_changes, 0);
        assert_eq!(snapshot.total(), 0);
        AuditEvidence {
            calls: CALLS,
            stable_output_address: output_address_changes == 0,
            pcm_digest,
            render_errors,
            snapshot,
        }
    }
}

#[derive(Clone, Copy)]
struct AuditEvidence {
    calls: u64,
    stable_output_address: bool,
    pcm_digest: u64,
    render_errors: u64,
    snapshot: AuditSnapshot,
}

impl AuditEvidence {
    fn to_json(self) -> String {
        format!(
            concat!(
                "{{\"schema_version\":1,\"kind\":\"issue022_capi_render_audit\",",
                "\"calls\":{},\"sample_rate_hz\":48000,\"quantum_frames\":128,",
                "\"stable_output_address\":{},\"pcm_digest\":\"{:016x}\",",
                "\"render_errors\":{},\"allocations\":{},\"deallocations\":{},",
                "\"locks\":{},\"feature_detection\":{},\"logs\":{},\"file_io\":{},",
                "\"network_io\":{},\"syscalls\":{},\"panic_unwinds\":{},",
                "\"total_violations\":{}}}"
            ),
            self.calls,
            self.stable_output_address,
            self.pcm_digest,
            self.render_errors,
            self.snapshot.allocations,
            self.snapshot.deallocations,
            self.snapshot.locks,
            self.snapshot.feature_detection,
            self.snapshot.logs,
            self.snapshot.file_io,
            self.snapshot.network_io,
            self.snapshot.syscalls,
            self.snapshot.panic_unwinds,
            self.snapshot.total(),
        )
    }
}

const fn audit_limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let mut prepared = PreparedAudit::prepare().expect("prepare Issue-022 C audit plan");
    let evidence = prepared.run();
    println!("{}", evidence.to_json());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializer_names_all_nine_forbidden_counters_exactly() {
        let json = AuditEvidence {
            calls: CALLS,
            stable_output_address: true,
            pcm_digest: 0x1234,
            render_errors: 0,
            snapshot: AuditSnapshot {
                allocations: 1,
                deallocations: 2,
                locks: 3,
                feature_detection: 4,
                logs: 5,
                file_io: 6,
                network_io: 7,
                syscalls: 8,
                panic_unwinds: 9,
            },
        }
        .to_json();
        assert_eq!(
            json,
            concat!(
                "{\"schema_version\":1,\"kind\":\"issue022_capi_render_audit\",",
                "\"calls\":100000,\"sample_rate_hz\":48000,\"quantum_frames\":128,",
                "\"stable_output_address\":true,\"pcm_digest\":\"0000000000001234\",",
                "\"render_errors\":0,\"allocations\":1,\"deallocations\":2,",
                "\"locks\":3,\"feature_detection\":4,\"logs\":5,\"file_io\":6,",
                "\"network_io\":7,\"syscalls\":8,\"panic_unwinds\":9,",
                "\"total_violations\":45}"
            )
        );
    }

    #[test]
    fn lifecycle_prepares_and_destroys_without_entering_render() {
        let prepared = PreparedAudit::prepare().expect("prepared lifecycle");
        assert!(!prepared.handles.engine.is_null());
        assert!(!prepared.handles.session.is_null());
        assert!(!prepared.handles.plan.is_null());
        assert!(!audit::is_render_scope_active());
        drop(prepared);
        assert!(!audit::is_render_scope_active());
    }
}
