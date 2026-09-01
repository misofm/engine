//! Non-timed Issue-022 audit plan for the exported C render entrypoint.

#![allow(unsafe_code)]

use bench_support::alloc as bench_alloc;
use std::ptr;

use capi::{
    ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits, ENGINE_CONFIG_SIZE,
    Engine, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput, RESULT_INTERNAL, RESULT_OK,
    SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk, SubmitReport,
    miso_engine_v2_compile_session, miso_engine_v2_engine_create, miso_engine_v2_engine_destroy,
    miso_engine_v2_plan_destroy, miso_engine_v2_render_f32_planar, miso_engine_v2_session_destroy,
    miso_engine_v2_source_submit_planar_f32,
};
use engine::realtime::audit::{self, AuditSnapshot};

const CALLS: u64 = 100_000;
const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM_FRAMES: usize = 128;
const SESSION_TOML: &[u8] =
    include_bytes!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

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
        let created = unsafe { miso_engine_v2_engine_create(&config, &mut handles.engine) };
        if created != RESULT_OK {
            return Err(created);
        }

        let limits = audit_limits();
        let mut diagnostic_storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: diagnostic_storage.as_mut_ptr(),
            capacity_bytes: diagnostic_storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The live engine, immutable TOML, fixed limits, diagnostic storage, and both
        // output locations remain valid throughout transactional compilation.
        let compiled = unsafe {
            miso_engine_v2_compile_session(
                handles.engine,
                SESSION_TOML.as_ptr(),
                SESSION_TOML.len() as u64,
                &limits,
                &mut diagnostics,
                &mut handles.session,
                &mut handles.plan,
            )
        };
        if compiled != RESULT_OK {
            return Err(compiled);
        }

        let left = [0.25_f32; QUANTUM_FRAMES];
        let right = [-0.5_f32; QUANTUM_FRAMES];
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: SAMPLE_RATE_HZ,
            generation: 1,
            start_frame: 0,
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
        // SAFETY: The session is live and all borrowed source bytes/planes and report storage
        // remain valid until the synchronous copy completes.
        let submitted = unsafe {
            miso_engine_v2_source_submit_planar_f32(
                handles.session,
                b"fixture-source".as_ptr(),
                14,
                &chunk,
                &mut report,
            )
        };
        if submitted != RESULT_OK || report.accepted_frames != QUANTUM_FRAMES as u64 {
            return Err(if submitted != RESULT_OK {
                submitted
            } else {
                RESULT_INTERNAL
            });
        }
        Ok(handles)
    }
}

impl Drop for AuditHandles {
    fn drop(&mut self) {
        // SAFETY: Every nonnull child is the unique quiescent handle published into this owner.
        // Destruction occurs after any render audit scope has ended, plan then session then engine.
        unsafe {
            miso_engine_v2_plan_destroy(self.plan);
            miso_engine_v2_session_destroy(self.session);
            miso_engine_v2_engine_destroy(self.engine);
        }
        self.plan = ptr::null_mut();
        self.session = ptr::null_mut();
        self.engine = ptr::null_mut();
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
        audit::warm_up();
        audit::reset();
        audit::in_render_scope(|| {
            for call in 0..CALLS {
                // SAFETY: The plan is live and exclusive, the descriptor points to the same
                // complete writable output for every synchronous call, and exact time is bounded.
                let result = unsafe {
                    miso_engine_v2_render_f32_planar(plan, call * QUANTUM_FRAMES as u64, &output)
                };
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
        });
        let snapshot = audit::snapshot();
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
        maximum_toml_bytes: 1_000_000,
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
        reserved: [0; 4],
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

    #[test]
    fn audit_plan_is_fixed_non_timed_and_calls_the_c_entrypoint() {
        const SOURCE: &str = include_str!("capi.rs");
        assert_eq!(CALLS, 100_000);
        assert!(SOURCE.contains("miso_engine_v2_render_f32_planar("));
        for forbidden in [
            concat!("std", "::time"),
            concat!("Instant", "::"),
            concat!("SystemTime", "::"),
            concat!("Duration", "::"),
            concat!(".", "elapsed()"),
        ] {
            assert!(!SOURCE.contains(forbidden), "timer surface: {forbidden}");
        }
    }
}
