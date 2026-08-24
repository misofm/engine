#ifndef MISO_ENGINE_V2_H
#define MISO_ENGINE_V2_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Thread ownership (frozen for ABI 1.0; unchanged by any minor version).
 *
 *   any thread   miso_engine_v2_abi_version and miso_engine_v2_query_capabilities take no handle
 *                and may be called from any thread at any time.
 *   engine       Control thread. At most one thread at a time calls any function taking the engine
 *                (miso_engine_v2_engine_create, miso_engine_v2_compile_session, and
 *                miso_engine_v2_last_error on the engine).
 *   session      Control thread. At most one thread at a time calls
 *                miso_engine_v2_source_submit_planar_f32, miso_engine_v2_source_seek,
 *                miso_engine_v2_submit_command, miso_engine_v2_dequeue_event, or
 *                miso_engine_v2_last_error on the session; they are serialized with each other.
 *   plan         Split ownership.
 *                miso_engine_v2_render_f32_planar: render thread only, never concurrently with
 *                itself, and the exclusive owner of the plan's render state.
 *                miso_engine_v2_plan_resources and miso_engine_v2_last_error on a plan: any thread,
 *                at any time while the plan is live, including concurrently with a render call.
 *                They are pure with respect to the plan handle: the report is copied from the
 *                plan's frozen resource accounting and the diagnostic is one atomic error word.
 *                Neither call writes plan render state, allocates, or blocks the render thread.
 *   *_destroy    miso_engine_v2_engine_destroy, miso_engine_v2_session_destroy and
 *                miso_engine_v2_plan_destroy require quiescence: no other call on that handle is
 *                in flight or will start. A session and its plan may be destroyed in either order.
 *
 * Borrowed pointers (session TOML, source IDs, request frames, chunk planes, output samples, and
 * every out pointer) are read or written only for the duration of the call and are never retained.
 * Every float plane pointer (chunk planes and output samples) must be 4-byte aligned, and the chunk
 * plane array must be pointer-aligned; a null or misaligned pointer returns
 * MISO_ENGINE_V2_INVALID_ARGUMENT before any access. No borrowed byte or sample region may exceed
 * PTRDIFF_MAX bytes; a larger declared length also returns MISO_ENGINE_V2_INVALID_ARGUMENT before
 * any access.
 *
 * miso_engine_v2_last_error on a plan returns a fixed diagnostic selected by the most recent render
 * call, one string per rule -- "render.output.unaligned", "render.output.platform",
 * "render.output.layout", "render.output.shape", "render.time.discontinuity",
 * "render.time.overflow", "render.plan.rejected" -- and is empty after a successful render; on a
 * session or engine it returns the most recent control-thread diagnostic. A rejected source
 * submission or seek likewise names the rule it broke ("source.region.outside",
 * "source.generation.stale", "source.channels.mismatch", ...).
 * MISO_ENGINE_V2_UNSUPPORTED is returned by exactly one entry point: miso_engine_v2_engine_create
 * refuses to create an engine on a CPU that cannot execute the instruction set this library was
 * built for (issue 083, master plan D4 -- the engine dispatches nothing at runtime, so the check
 * happens once at boot instead of inside a render callback). No other entry point returns it. An
 * embedder that receives it must not retry; the library and the CPU do not match.
 */

#define MISO_ENGINE_V2_ABI_VERSION UINT32_C(0x00010000)

#define MISO_ENGINE_V2_OK UINT32_C(0)
#define MISO_ENGINE_V2_INVALID_ARGUMENT UINT32_C(1)
#define MISO_ENGINE_V2_ABI_MISMATCH UINT32_C(2)
#define MISO_ENGINE_V2_WRONG_HANDLE UINT32_C(3)
#define MISO_ENGINE_V2_BUFFER_TOO_SMALL UINT32_C(4)
#define MISO_ENGINE_V2_COMPILE_REJECTED UINT32_C(5)
#define MISO_ENGINE_V2_BACKPRESSURE UINT32_C(6)
#define MISO_ENGINE_V2_UNSUPPORTED UINT32_C(7)
#define MISO_ENGINE_V2_RENDER_REJECTED UINT32_C(8)
#define MISO_ENGINE_V2_INTERNAL UINT32_C(255)

#define MISO_ENGINE_V2_EVENT_LANE_RELIABLE UINT32_C(0)
#define MISO_ENGINE_V2_EVENT_LANE_LOSSY UINT32_C(1)

#define MISO_ENGINE_V2_TAIL_FINITE UINT64_C(0)
#define MISO_ENGINE_V2_TAIL_INFINITE UINT64_C(1)

#define MISO_ENGINE_V2_RATE_44100 UINT64_C(1)
#define MISO_ENGINE_V2_RATE_48000 UINT64_C(2)
#define MISO_ENGINE_V2_RATE_88200 UINT64_C(4)
#define MISO_ENGINE_V2_RATE_96000 UINT64_C(8)
#define MISO_ENGINE_V2_EXACT_LAUNCH_RATE_MASK UINT64_C(15)

#define MISO_ENGINE_V2_FEATURE_IMMUTABLE_SESSION UINT64_C(1)
#define MISO_ENGINE_V2_FEATURE_HOST_PLANAR_SOURCE UINT64_C(2)
#define MISO_ENGINE_V2_FEATURE_SOURCE_SEEK UINT64_C(4)
#define MISO_ENGINE_V2_FEATURE_PLANAR_STEREO_RENDER UINT64_C(8)
#define MISO_ENGINE_V2_FEATURE_CAPABILITY_COMMAND UINT64_C(16)
#define MISO_ENGINE_V2_FEATURE_MASK UINT64_C(31)

#define MISO_ENGINE_V2_ENGINE_CONFIG_SIZE UINT32_C(40)
#define MISO_ENGINE_V2_COMPILE_LIMITS_SIZE UINT32_C(208)
#define MISO_ENGINE_V2_BYTES_OUT_SIZE UINT32_C(32)
#define MISO_ENGINE_V2_SOURCE_CHUNK_SIZE UINT32_C(48)
#define MISO_ENGINE_V2_SUBMIT_REPORT_SIZE UINT32_C(32)
#define MISO_ENGINE_V2_PLANAR_OUTPUT_SIZE UINT32_C(48)
#define MISO_ENGINE_V2_CAPABILITIES_SIZE UINT32_C(56)
#define MISO_ENGINE_V2_PLAN_RESOURCE_REPORT_SIZE UINT32_C(240)

typedef struct miso_engine_v2_engine miso_engine_v2_engine;
typedef struct miso_engine_v2_session miso_engine_v2_session;
typedef struct miso_engine_v2_plan miso_engine_v2_plan;

typedef struct miso_engine_v2_engine_config {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t reserved[4];
} miso_engine_v2_engine_config;

typedef struct miso_engine_v2_compile_limits {
    uint32_t struct_size;
    uint32_t source_ring_frames;
    uint32_t maximum_automation_spans_per_block;
    uint32_t reserved0;
    uint64_t maximum_toml_bytes;
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
    uint64_t reserved[4];
} miso_engine_v2_compile_limits;

typedef struct miso_engine_v2_bytes_out {
    uint32_t struct_size;
    uint32_t reserved0;
    uint8_t *data;
    uint64_t capacity_bytes;
    uint64_t required_bytes;
} miso_engine_v2_bytes_out;

typedef struct miso_engine_v2_source_chunk {
    uint32_t struct_size;
    uint32_t sample_rate_hz;
    uint64_t generation;
    uint64_t start_frame;
    const float *const *planes;
    uint32_t plane_count;
    uint32_t frames;
    uint32_t end_of_region;
    uint32_t reserved0;
} miso_engine_v2_source_chunk;

typedef struct miso_engine_v2_submit_report {
    uint32_t struct_size;
    uint32_t reserved0;
    uint64_t accepted_frames;
    uint64_t cumulative_written_frames;
    uint64_t active_generation;
} miso_engine_v2_submit_report;

typedef struct miso_engine_v2_planar_output {
    uint32_t struct_size;
    uint32_t channels;
    float *samples;
    uint64_t sample_capacity;
    uint32_t frames;
    uint32_t plane_stride_samples;
    uint64_t reserved[2];
} miso_engine_v2_planar_output;

typedef struct miso_engine_v2_capabilities {
    uint32_t struct_size;
    uint32_t abi_version;
    uint64_t exact_launch_rate_mask;
    uint64_t feature_mask;
    uint64_t reserved[4];
} miso_engine_v2_capabilities;

typedef struct miso_engine_v2_plan_resource_report {
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
} miso_engine_v2_plan_resource_report;

uint32_t miso_engine_v2_abi_version(void);
uint32_t miso_engine_v2_query_capabilities(miso_engine_v2_capabilities *out);
uint32_t miso_engine_v2_engine_create(const miso_engine_v2_engine_config *config,
                                      miso_engine_v2_engine **out_engine);
void miso_engine_v2_engine_destroy(miso_engine_v2_engine *engine);
uint32_t miso_engine_v2_compile_session(miso_engine_v2_engine *engine,
                                        const uint8_t *toml,
                                        uint64_t toml_bytes,
                                        const miso_engine_v2_compile_limits *limits,
                                        miso_engine_v2_bytes_out *diagnostics,
                                        miso_engine_v2_session **out_session,
                                        miso_engine_v2_plan **out_plan);
uint32_t miso_engine_v2_source_submit_planar_f32(miso_engine_v2_session *session,
                                                 const uint8_t *source_id,
                                                 uint64_t source_id_bytes,
                                                 const miso_engine_v2_source_chunk *chunk,
                                                 miso_engine_v2_submit_report *out_report);
uint32_t miso_engine_v2_source_seek(miso_engine_v2_session *session,
                                    const uint8_t *source_id,
                                    uint64_t source_id_bytes,
                                    uint64_t generation,
                                    uint64_t source_frame);
uint32_t miso_engine_v2_submit_command(miso_engine_v2_session *session,
                                       const uint8_t *request,
                                       uint64_t request_bytes,
                                       miso_engine_v2_bytes_out *response);
uint32_t miso_engine_v2_dequeue_event(miso_engine_v2_session *session,
                                      uint32_t lane,
                                      miso_engine_v2_bytes_out *event);
uint32_t miso_engine_v2_render_f32_planar(miso_engine_v2_plan *plan,
                                          uint64_t absolute_sample,
                                          const miso_engine_v2_planar_output *output);
uint32_t miso_engine_v2_plan_resources(const miso_engine_v2_plan *plan,
                                       miso_engine_v2_plan_resource_report *out);
uint32_t miso_engine_v2_last_error(const void *live_handle, miso_engine_v2_bytes_out *out);
void miso_engine_v2_session_destroy(miso_engine_v2_session *session);
void miso_engine_v2_plan_destroy(miso_engine_v2_plan *plan);

#ifdef __cplusplus
}
#endif

#endif
