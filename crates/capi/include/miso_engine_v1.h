#ifndef MISO_ENGINE_V1_H
#define MISO_ENGINE_V1_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Thread ownership (frozen for ABI 1.0; unchanged by any minor version).
 *
 *   any thread   miso_engine_v1_abi_version and miso_engine_v1_query_capabilities take no handle
 *                and may be called from any thread at any time.
 *   engine       Control thread. At most one thread at a time calls any function taking the engine
 *                (miso_engine_v1_engine_create, miso_engine_v1_compile_session, and
 *                miso_engine_v1_last_error on the engine).
 *   session      Control thread. At most one thread at a time calls
 *                miso_engine_v1_source_submit_planar_f32, miso_engine_v1_source_seek,
 *                miso_engine_v1_source_seek_at, miso_engine_v1_submit_command,
 *                miso_engine_v1_dequeue_event, miso_engine_v1_service, or
 *                miso_engine_v1_last_error on the session; they are serialized with each other.
 *   plan         Split ownership.
 *                miso_engine_v1_render_f32_planar: render thread only, never concurrently with
 *                itself, and the exclusive owner of the plan's render state.
 *                miso_engine_v1_plan_resources, miso_engine_v1_plan_watermark and
 *                miso_engine_v1_last_error on a plan: any thread, at any time while the plan is
 *                live, including concurrently with a render call. They are pure with respect to
 *                the plan handle: the report is copied from the plan's frozen resource
 *                accounting, the watermark from the record render publishes, and the diagnostic
 *                is one atomic error word. None of them writes plan render state, allocates, or
 *                blocks the render thread.
 *   *_destroy    miso_engine_v1_engine_destroy, miso_engine_v1_session_destroy and
 *                miso_engine_v1_plan_destroy require quiescence: no other call on that handle is
 *                in flight or will start. A session and its plan may be destroyed in either order.
 *
 * Live edits. A committed SESSION_TRANSACTION_APPLY whose changes are all live is applied to the
 * running plan: no plan is prepared, and the source rings, the effect state and the render
 * position continue, so the host goes on submitting with no seek. The live changes are a track's
 * fader levels, mutes, and pan or matrix values; a changed value of a track effect parameter
 * whose automation rate is Block, on a console-slot entry or an insert, the parametric EQ's live
 * parameters included (applied through prepared targets); and a track effect's bypass, except the
 * delay's and the multiband compressor's, which rebuild. A transaction that changes only the
 * session ID, a profile's id or the stored automation table commits with no replacement and no
 * live value. Any other change takes the replacement path exactly as before: every submix strip
 * value (its fader, its effects), any edit while the session declares a VCA, the mute of a track
 * that a follows_mute send reads, a fader outside its domain or effect parameters that
 * preparation refuses (refused as MISO_ENGINE_V1_COMPILE_REJECTED with the same diagnostic as
 * before), and every other field. A live edit applies no later than the first render call that
 * begins after miso_engine_v1_submit_command returns, and may apply one block earlier, so the
 * values of one transaction can take effect up to one quantum apart; it is heard up to
 * latency_samples later. Each track's fader and matrix lanes hold 16 pending live values each,
 * and each effect's lane the smaller of 16 and its automation capacity; an effect edit that
 * could never fit its lane rebuilds. Past a lane's room the edit returns
 * MISO_ENGINE_V1_BACKPRESSURE with the diagnostic "control.live.backpressure", the session and
 * its revision are unchanged, and the host retries after a render call. An
 * acknowledged live edit is never dropped. A live edit commits the same response and the same
 * reliable events as a replacement, so the host drains the reliable event lane after each edit
 * either way. After either one the host goes on submitting every source the transaction left
 * unchanged, with no seek; after a replacement it acts only on a source the transaction added,
 * changed or removed (see "Sources across a structural transaction" and "Starting an added stem
 * in time" below). The ABI gives no signal that tells a live edit from a replacement, and a host
 * needs none. (miso_engine_v1_plan_resources describes whichever plan is active, so a replacement
 * may change it, but an unchanged report does not mean that no replacement happened.)
 *
 * Completion (issue 1314). miso_engine_v1_submit_command is synchronous only for what can fail:
 * it validates and classifies the transaction, prepares a replacement plan and reserves its
 * publication and retirement credit, commits, and returns; it never waits for a render call or a
 * plan swap. Every committed revision is pending until the plan watermark covers it.
 * miso_engine_v1_plan_watermark copies (revision, first_sample, outcome_flags) and per-outcome
 * counters: revision is the highest committed revision in effect together with every revision
 * before it, and first_sample the absolute render sample at the start of the block that reported
 * it. The watermark is never early: a revision is in effect no later than the block it names.
 * first_sample is the start of the first block that begins after the commit's last write, so at
 * most one block after the submit returns (while a replacement plan is pending: the block that
 * adopts it). A live value of that revision can apply earlier, in any block that
 * ran between the submit's first push and its revision store. One transaction's values can
 * spread across those blocks until issues 1502, 1503, 1504, 1312 and 1345 have all landed,
 * which together make it exact. A replacement plan renders from first_sample. A host that
 * renders nothing completes nothing: a paused host's revisions stay pending until render
 * resumes. The watermark is a level, not a queue: render overwrites it, so a host that polls it
 * late still reads the newest revision, and the counters still count every revision.
 * outcome_flags is the OR
 * over the revisions the last advance covered of MISO_ENGINE_V1_OUTCOME_EXACT,
 * MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK and MISO_ENGINE_V1_OUTCOME_SUPERSEDED. The host
 * polls the watermark to learn that an edit is audible, never the event lane. The caller zeroes
 * reserved0 and reserved[] and sets struct_size, or the call returns
 * MISO_ENGINE_V1_INVALID_ARGUMENT; a copy that keeps landing inside render's publication
 * returns MISO_ENGINE_V1_BACKPRESSURE with out untouched, and the host retries. A host checks
 * MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK before calling the symbol.
 *
 * Control service (issue 1348). The engine owns no thread, so the host drives the control work
 * between edits. miso_engine_v1_service(session) runs one bounded step of it: it reclaims plans
 * render has retired, brings the session's view of the running plan up to render, refreshes the
 * session counters, and stages render telemetry. Every other session call (source submit, seek,
 * seek_at, submit_command, dequeue_event) that passes its argument checks runs the same step
 * first; a call refused for its arguments returns before the step. So a host that makes those
 * calls anyway need not also call service. The step never waits for render, never commits or
 * acknowledges anything, and never takes or drops a pending edit; its cost does not grow with the
 * time since the last call. It returns MISO_ENGINE_V1_OK; a null session as
 * MISO_ENGINE_V1_INVALID_ARGUMENT and another handle as MISO_ENGINE_V1_WRONG_HANDLE; and, should
 * the session's plan epochs fail to synchronize, MISO_ENGINE_V1_INTERNAL with the session
 * diagnostic "capi.source.epoch". The host's duty: a pending edit progresses only while the host
 * makes session calls, the same duty as draining events. A host that renders but never calls
 * keeps retired plans allocated, and its next structural edit waits for that reclaim; a warm
 * successor that misses its deadline falls back only at the host's next session call. The plan
 * watermark never covers an edit before it is in effect. While edits are pending, call
 * miso_engine_v1_service (or another session call) at least once per render-buffer period. A host
 * checks MISO_ENGINE_V1_FEATURE_SERVICE before calling the symbol.
 *
 * Borrowed pointers (session JSON, source IDs, request frames, chunk planes, output samples, and
 * every out pointer) are read or written only for the duration of the call and are never retained.
 * Every float plane pointer (chunk planes and output samples) must be 4-byte aligned, and the chunk
 * plane array must be pointer-aligned; a null or misaligned pointer returns
 * MISO_ENGINE_V1_INVALID_ARGUMENT before any access. No borrowed byte or sample region may exceed
 * PTRDIFF_MAX bytes; a larger declared length also returns MISO_ENGINE_V1_INVALID_ARGUMENT before
 * any access.
 *
 * miso_engine_v1_last_error on a plan returns a fixed diagnostic selected by the most recent render
 * call, one string per rule -- "render.output.unaligned", "render.output.platform",
 * "render.output.layout", "render.output.shape", "render.time.discontinuity",
 * "render.time.overflow", "render.plan.rejected" -- and is empty after a successful render; on a
 * session or engine it returns the most recent control-thread diagnostic. A rejected source
 * submission or seek likewise names the rule it broke ("source.region.outside",
 * "source.generation.stale", "source.channels.mismatch", ...).
 * MISO_ENGINE_V1_UNSUPPORTED is returned by exactly one entry point: miso_engine_v1_engine_create
 * refuses to create an engine on a CPU that cannot execute the instruction set this library was
 * built for (issue 083, master plan D4 -- the engine dispatches nothing at runtime, so the check
 * happens once at boot instead of inside a render callback). No other entry point returns it. An
 * embedder that receives it must not retry; the library and the CPU do not match.
 *
 * Sources across a structural transaction (issue 1273). A SESSION_TRANSACTION_APPLY that replaces
 * the plan keeps every source it leaves unchanged playing: the source keeps its ring, its
 * generation and its read position, so the host neither seeks nor refills it, and must not reseek
 * it. From the moment the command returns OK, source submissions and seeks address the newest
 * committed session, whether or not its plan has been swapped in yet. A source the transaction
 * removed is refused as "source.id.unknown"; PCM already accepted for it is discarded with its
 * plan. A source the transaction added starts at generation 1, frame 0, until the host seeks it.
 * A source whose declaration the transaction changed restarts like an added one: in a new ring at
 * generation 1, frame 0, with the PCM accepted for it before the command discarded with its plan;
 * the host restarts its feed.
 *
 * Starting an added stem in time (issue 1275). miso_engine_v1_source_seek_at is
 * miso_engine_v1_source_seek with an anchor: source_frame enters the graph in the block that starts
 * at absolute render sample anchor_sample, the clock miso_engine_v1_render_f32_planar takes and
 * every plan swap continues; the output hears it the plan's latency later. The anchor must be a
 * multiple of the plan's quantum, or the call returns MISO_ENGINE_V1_INVALID_ARGUMENT with
 * "source.seek.anchor_unaligned". A host checks MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT before
 * calling it. To start a stem a transaction added, after the command returns OK: pick A, a
 * quantum multiple a few quanta past the last rendered block; call
 * miso_engine_v1_source_seek_at(id, g, F, A), where g is above the source's generation (2 for a
 * just-added source) and F is the source frame the playing stems read at A; then submit generation
 * g from F before the block at A renders. Until A the stem renders silence, and those blocks
 * count as source underruns; from A it plays F on. Should the render reach the seek only after A,
 * the stem starts at F plus the lateness, so it stays in time.
 */

#define MISO_ENGINE_V1_ABI_VERSION UINT32_C(0x00010000)

#define MISO_ENGINE_V1_OK UINT32_C(0)
#define MISO_ENGINE_V1_INVALID_ARGUMENT UINT32_C(1)
#define MISO_ENGINE_V1_ABI_MISMATCH UINT32_C(2)
#define MISO_ENGINE_V1_WRONG_HANDLE UINT32_C(3)
#define MISO_ENGINE_V1_BUFFER_TOO_SMALL UINT32_C(4)
#define MISO_ENGINE_V1_COMPILE_REJECTED UINT32_C(5)
#define MISO_ENGINE_V1_BACKPRESSURE UINT32_C(6)
#define MISO_ENGINE_V1_UNSUPPORTED UINT32_C(7)
#define MISO_ENGINE_V1_RENDER_REJECTED UINT32_C(8)
#define MISO_ENGINE_V1_INTERNAL UINT32_C(255)

#define MISO_ENGINE_V1_EVENT_LANE_RELIABLE UINT32_C(0)
#define MISO_ENGINE_V1_EVENT_LANE_LOSSY UINT32_C(1)

#define MISO_ENGINE_V1_TAIL_FINITE UINT64_C(0)
#define MISO_ENGINE_V1_TAIL_INFINITE UINT64_C(1)

#define MISO_ENGINE_V1_RATE_44100 UINT64_C(1)
#define MISO_ENGINE_V1_RATE_48000 UINT64_C(2)
#define MISO_ENGINE_V1_RATE_88200 UINT64_C(4)
#define MISO_ENGINE_V1_RATE_96000 UINT64_C(8)
#define MISO_ENGINE_V1_EXACT_LAUNCH_RATE_MASK UINT64_C(15)

#define MISO_ENGINE_V1_FEATURE_IMMUTABLE_SESSION UINT64_C(1)
#define MISO_ENGINE_V1_FEATURE_HOST_PLANAR_SOURCE UINT64_C(2)
#define MISO_ENGINE_V1_FEATURE_SOURCE_SEEK UINT64_C(4)
#define MISO_ENGINE_V1_FEATURE_PLANAR_STEREO_RENDER UINT64_C(8)
#define MISO_ENGINE_V1_FEATURE_CAPABILITY_COMMAND UINT64_C(16)
#define MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT UINT64_C(32)
#define MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK UINT64_C(64)
#define MISO_ENGINE_V1_FEATURE_SERVICE UINT64_C(128)
#define MISO_ENGINE_V1_FEATURE_MASK UINT64_C(255)

#define MISO_ENGINE_V1_OUTCOME_EXACT UINT64_C(1)
#define MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK UINT64_C(2)
#define MISO_ENGINE_V1_OUTCOME_SUPERSEDED UINT64_C(4)

#define MISO_ENGINE_V1_ENGINE_CONFIG_SIZE UINT32_C(40)
#define MISO_ENGINE_V1_COMPILE_LIMITS_SIZE UINT32_C(208)
#define MISO_ENGINE_V1_BYTES_OUT_SIZE UINT32_C(32)
#define MISO_ENGINE_V1_SOURCE_CHUNK_SIZE UINT32_C(48)
#define MISO_ENGINE_V1_SUBMIT_REPORT_SIZE UINT32_C(32)
#define MISO_ENGINE_V1_PLANAR_OUTPUT_SIZE UINT32_C(48)
#define MISO_ENGINE_V1_CAPABILITIES_SIZE UINT32_C(56)
#define MISO_ENGINE_V1_PLAN_RESOURCE_REPORT_SIZE UINT32_C(240)
#define MISO_ENGINE_V1_WATERMARK_SIZE UINT32_C(96)

typedef struct miso_engine_v1_engine miso_engine_v1_engine;
typedef struct miso_engine_v1_session miso_engine_v1_session;
typedef struct miso_engine_v1_plan miso_engine_v1_plan;

typedef struct miso_engine_v1_engine_config {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t reserved[4];
} miso_engine_v1_engine_config;

typedef struct miso_engine_v1_compile_limits {
    uint32_t struct_size;
    uint32_t source_ring_frames;
    uint32_t maximum_automation_spans_per_block;
    uint32_t reserved0;
    uint64_t maximum_document_bytes;
    uint64_t maximum_diagnostic_bytes;
    uint64_t maximum_tracks;
    uint64_t maximum_sources;
    uint64_t maximum_routes;
    uint64_t maximum_effects;
    uint64_t maximum_graph_session_plus_plan_bytes;
    uint64_t maximum_source_total_bytes;
    uint64_t maximum_source_overhead_bytes;
    uint64_t maximum_effect_state_bytes;
    uint64_t maximum_effect_scratch_bytes;
    uint64_t maximum_builtin_retained_bytes;
    uint64_t maximum_capi_retained_bytes;
    uint64_t maximum_named_allocation_bytes;
    uint64_t maximum_meter_streams;
    uint64_t maximum_meter_items;
    uint64_t maximum_meter_bytes;
    uint64_t maximum_control_frame_bytes;
    uint64_t maximum_replay_bytes;
    uint64_t maximum_replay_entries;
    /* Maximum session submix strips. Zero means "use maximum_tracks"; it never means
       "no submixes" or "unbounded". Formerly reserved[0]; the layout is unchanged. */
    uint64_t maximum_submixes;
    /* Maximum session VCA groups. Zero means "use maximum_tracks"; it never means
       "no VCAs" or "unbounded". Formerly reserved[1] of the original reserved[4]
       (offset 184); the layout is unchanged. */
    uint64_t maximum_vcas;
    uint64_t reserved[2]; /* Must be zero in ABI V1. */
} miso_engine_v1_compile_limits;

typedef struct miso_engine_v1_bytes_out {
    uint32_t struct_size;
    uint32_t reserved0;
    uint8_t *data;
    uint64_t capacity_bytes;
    uint64_t required_bytes;
} miso_engine_v1_bytes_out;

typedef struct miso_engine_v1_source_chunk {
    uint32_t struct_size;
    uint32_t sample_rate_hz;
    uint64_t generation;
    uint64_t start_frame;
    const float *const *planes;
    uint32_t plane_count;
    uint32_t frames;
    uint32_t end_of_region;
    uint32_t reserved0;
} miso_engine_v1_source_chunk;

typedef struct miso_engine_v1_submit_report {
    uint32_t struct_size;
    uint32_t reserved0;
    uint64_t accepted_frames;
    uint64_t cumulative_written_frames;
    uint64_t active_generation;
} miso_engine_v1_submit_report;

typedef struct miso_engine_v1_planar_output {
    uint32_t struct_size;
    uint32_t channels;
    float *samples;
    uint64_t sample_capacity;
    uint32_t frames;
    uint32_t plane_stride_samples;
    uint64_t reserved[2];
} miso_engine_v1_planar_output;

typedef struct miso_engine_v1_capabilities {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t exact_launch_rate_mask;
    uint64_t feature_mask;
    uint64_t reserved[4];
} miso_engine_v1_capabilities;

typedef struct miso_engine_v1_plan_resource_report {
    uint32_t struct_size;
    uint32_t abi_version;
    uint32_t sample_rate_hz;
    uint32_t quantum_frames;
    uint64_t source_count;
    uint64_t track_count;
    uint64_t latency_samples;
    uint64_t tail_kind;
    uint64_t tail_samples;
    uint64_t graph_session_plus_plan_bytes;
    uint64_t graph_incremental_plan_bytes;
    uint64_t graph_metadata_bytes;
    uint64_t graph_delay_bytes;
    uint64_t effect_bank_scratch_bytes;
    uint64_t effect_bank_runtime_buffer_bytes;
    uint64_t effect_bank_metadata_bytes;
    uint64_t builtin_bank_bytes;
    uint64_t builtin_bank_scratch_bytes;
    uint64_t source_pcm_payload_bytes;
    uint64_t source_overhead_bytes;
    uint64_t source_total_bytes;
    uint64_t effect_scalar_state_bytes;
    uint64_t effect_scalar_scratch_bytes;
    uint64_t builtin_processor_payload_bytes;
    uint64_t builtin_meter_payload_bytes;
    uint64_t builtin_retained_payload_bytes;
    uint64_t capi_retained_bytes;
    uint64_t largest_named_allocation_bytes;
    uint64_t reserved[4];
} miso_engine_v1_plan_resource_report;

typedef struct miso_engine_v1_watermark {
    uint32_t struct_size;
    uint32_t reserved0; /* Must be zero in ABI V1. */
    uint64_t revision;
    uint64_t first_sample;
    uint64_t outcome_flags;
    uint64_t exact_count;
    uint64_t transition_fallback_count;
    uint64_t superseded_count;
    uint64_t reserved[5]; /* Must be zero in ABI V1. */
} miso_engine_v1_watermark;

uint32_t miso_engine_v1_abi_version(void);
uint32_t miso_engine_v1_query_capabilities(miso_engine_v1_capabilities *out);
uint32_t miso_engine_v1_engine_create(const miso_engine_v1_engine_config *config,
                                      miso_engine_v1_engine **out_engine);
void miso_engine_v1_engine_destroy(miso_engine_v1_engine *engine);
uint32_t miso_engine_v1_compile_session(miso_engine_v1_engine *engine,
                                        const uint8_t *document,
                                        uint64_t document_bytes,
                                        const miso_engine_v1_compile_limits *limits,
                                        miso_engine_v1_bytes_out *diagnostics,
                                        miso_engine_v1_session **out_session,
                                        miso_engine_v1_plan **out_plan);
uint32_t miso_engine_v1_source_submit_planar_f32(miso_engine_v1_session *session,
                                                 const uint8_t *source_id,
                                                 uint64_t source_id_bytes,
                                                 const miso_engine_v1_source_chunk *chunk,
                                                 miso_engine_v1_submit_report *out_report);
uint32_t miso_engine_v1_source_seek(miso_engine_v1_session *session,
                                    const uint8_t *source_id,
                                    uint64_t source_id_bytes,
                                    uint64_t generation,
                                    uint64_t source_frame);
uint32_t miso_engine_v1_source_seek_at(miso_engine_v1_session *session,
                                       const uint8_t *source_id,
                                       uint64_t source_id_bytes,
                                       uint64_t generation,
                                       uint64_t source_frame,
                                       uint64_t anchor_sample);
uint32_t miso_engine_v1_submit_command(miso_engine_v1_session *session,
                                       const uint8_t *request,
                                       uint64_t request_bytes,
                                       miso_engine_v1_bytes_out *response);
uint32_t miso_engine_v1_dequeue_event(miso_engine_v1_session *session,
                                      uint32_t lane,
                                      miso_engine_v1_bytes_out *event);
uint32_t miso_engine_v1_service(miso_engine_v1_session *session);
uint32_t miso_engine_v1_render_f32_planar(miso_engine_v1_plan *plan,
                                          uint64_t absolute_sample,
                                          const miso_engine_v1_planar_output *output);
uint32_t miso_engine_v1_plan_resources(const miso_engine_v1_plan *plan,
                                       miso_engine_v1_plan_resource_report *out);
uint32_t miso_engine_v1_plan_watermark(const miso_engine_v1_plan *plan,
                                       miso_engine_v1_watermark *out);
uint32_t miso_engine_v1_last_error(const void *live_handle, miso_engine_v1_bytes_out *out);
void miso_engine_v1_session_destroy(miso_engine_v1_session *session);
void miso_engine_v1_plan_destroy(miso_engine_v1_plan *plan);

#ifdef __cplusplus
}
#endif

#endif
