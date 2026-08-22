#ifndef MISO_ENGINE_V2_H
#define MISO_ENGINE_V2_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

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
#define MISO_ENGINE_V2_COMPILE_LIMITS_SIZE UINT32_C(200)
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
    uint64_t maximum_single_allocation_bytes;
    uint64_t maximum_meter_streams;
    uint64_t maximum_meter_items;
    uint64_t maximum_meter_bytes;
    uint64_t maximum_control_frame_bytes;
    uint64_t maximum_replay_bytes;
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
    uint64_t largest_allocation_bytes;
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
