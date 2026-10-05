#include "miso_engine_v1.h"

#include <stddef.h>
#include <stdint.h>

#define ABI_ASSERT(expression) _Static_assert((expression), #expression)

ABI_ASSERT(MISO_ENGINE_V1_ABI_VERSION == UINT32_C(0x00010000));
ABI_ASSERT(MISO_ENGINE_V1_OK == 0);
ABI_ASSERT(MISO_ENGINE_V1_INVALID_ARGUMENT == 1);
ABI_ASSERT(MISO_ENGINE_V1_ABI_MISMATCH == 2);
ABI_ASSERT(MISO_ENGINE_V1_WRONG_HANDLE == 3);
ABI_ASSERT(MISO_ENGINE_V1_BUFFER_TOO_SMALL == 4);
ABI_ASSERT(MISO_ENGINE_V1_COMPILE_REJECTED == 5);
ABI_ASSERT(MISO_ENGINE_V1_BACKPRESSURE == 6);
ABI_ASSERT(MISO_ENGINE_V1_UNSUPPORTED == 7);
ABI_ASSERT(MISO_ENGINE_V1_RENDER_REJECTED == 8);
ABI_ASSERT(MISO_ENGINE_V1_INTERNAL == 255);
ABI_ASSERT(MISO_ENGINE_V1_TAIL_FINITE == 0);
ABI_ASSERT(MISO_ENGINE_V1_TAIL_INFINITE == 1);
ABI_ASSERT(MISO_ENGINE_V1_EXACT_LAUNCH_RATE_MASK == 15);
ABI_ASSERT(MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT == 32);
ABI_ASSERT(MISO_ENGINE_V1_FEATURE_MASK == 127);
ABI_ASSERT((MISO_ENGINE_V1_FEATURE_MASK & MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK) ==
           MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK);
ABI_ASSERT(MISO_ENGINE_V1_OUTCOME_EXACT == 1);
ABI_ASSERT(MISO_ENGINE_V1_OUTCOME_TRANSITION_FALLBACK == 2);
ABI_ASSERT(MISO_ENGINE_V1_OUTCOME_SUPERSEDED == 4);

ABI_ASSERT(sizeof(miso_engine_v1_engine_config) == MISO_ENGINE_V1_ENGINE_CONFIG_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_compile_limits) == MISO_ENGINE_V1_COMPILE_LIMITS_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_bytes_out) == MISO_ENGINE_V1_BYTES_OUT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_source_chunk) == MISO_ENGINE_V1_SOURCE_CHUNK_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_submit_report) == MISO_ENGINE_V1_SUBMIT_REPORT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_planar_output) == MISO_ENGINE_V1_PLANAR_OUTPUT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_capabilities) == MISO_ENGINE_V1_CAPABILITIES_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_plan_resource_report) ==
           MISO_ENGINE_V1_PLAN_RESOURCE_REPORT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v1_watermark) == MISO_ENGINE_V1_WATERMARK_SIZE);
ABI_ASSERT(MISO_ENGINE_V1_WATERMARK_SIZE == 96);

ABI_ASSERT(_Alignof(miso_engine_v1_engine_config) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_compile_limits) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_bytes_out) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_source_chunk) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_submit_report) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_planar_output) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_capabilities) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_plan_resource_report) == 8);
ABI_ASSERT(_Alignof(miso_engine_v1_watermark) == 8);

ABI_ASSERT(offsetof(miso_engine_v1_compile_limits, maximum_document_bytes) == 16);
ABI_ASSERT(offsetof(miso_engine_v1_compile_limits, maximum_replay_entries) == 168);
ABI_ASSERT(offsetof(miso_engine_v1_compile_limits, maximum_submixes) == 176);
ABI_ASSERT(offsetof(miso_engine_v1_compile_limits, maximum_vcas) == 184);
ABI_ASSERT(offsetof(miso_engine_v1_compile_limits, reserved) == 192);
ABI_ASSERT(offsetof(miso_engine_v1_bytes_out, data) == 8);
ABI_ASSERT(offsetof(miso_engine_v1_bytes_out, required_bytes) == 24);
ABI_ASSERT(offsetof(miso_engine_v1_source_chunk, planes) == 24);
ABI_ASSERT(offsetof(miso_engine_v1_source_chunk, reserved0) == 44);
ABI_ASSERT(offsetof(miso_engine_v1_planar_output, samples) == 8);
ABI_ASSERT(offsetof(miso_engine_v1_planar_output, reserved) == 32);
ABI_ASSERT(offsetof(miso_engine_v1_capabilities, reserved) == 24);
ABI_ASSERT(offsetof(miso_engine_v1_plan_resource_report, source_count) == 16);
ABI_ASSERT(offsetof(miso_engine_v1_plan_resource_report, reserved) == 208);
ABI_ASSERT(offsetof(miso_engine_v1_watermark, revision) == 8);
ABI_ASSERT(offsetof(miso_engine_v1_watermark, reserved) == 56);

static uint32_t (*const abi_version_signature)(void) = miso_engine_v1_abi_version;
static uint32_t (*const query_capabilities_signature)(miso_engine_v1_capabilities *) =
    miso_engine_v1_query_capabilities;
static uint32_t (*const engine_create_signature)(const miso_engine_v1_engine_config *,
                                                 miso_engine_v1_engine **) =
    miso_engine_v1_engine_create;
static void (*const engine_destroy_signature)(miso_engine_v1_engine *) =
    miso_engine_v1_engine_destroy;
static uint32_t (*const compile_session_signature)(miso_engine_v1_engine *,
                                                   const uint8_t *,
                                                   uint64_t,
                                                   const miso_engine_v1_compile_limits *,
                                                   miso_engine_v1_bytes_out *,
                                                   miso_engine_v1_session **,
                                                   miso_engine_v1_plan **) =
    miso_engine_v1_compile_session;
static uint32_t (*const source_submit_signature)(miso_engine_v1_session *,
                                                 const uint8_t *,
                                                 uint64_t,
                                                 const miso_engine_v1_source_chunk *,
                                                 miso_engine_v1_submit_report *) =
    miso_engine_v1_source_submit_planar_f32;
static uint32_t (*const source_seek_signature)(miso_engine_v1_session *,
                                               const uint8_t *,
                                               uint64_t,
                                               uint64_t,
                                               uint64_t) = miso_engine_v1_source_seek;
static uint32_t (*const source_seek_at_signature)(miso_engine_v1_session *,
                                                  const uint8_t *,
                                                  uint64_t,
                                                  uint64_t,
                                                  uint64_t,
                                                  uint64_t) = miso_engine_v1_source_seek_at;
static uint32_t (*const submit_command_signature)(miso_engine_v1_session *,
                                                  const uint8_t *,
                                                  uint64_t,
                                                  miso_engine_v1_bytes_out *) =
    miso_engine_v1_submit_command;
static uint32_t (*const dequeue_event_signature)(miso_engine_v1_session *,
                                                 uint32_t,
                                                 miso_engine_v1_bytes_out *) =
    miso_engine_v1_dequeue_event;
static uint32_t (*const render_signature)(miso_engine_v1_plan *,
                                          uint64_t,
                                          const miso_engine_v1_planar_output *) =
    miso_engine_v1_render_f32_planar;
static uint32_t (*const resources_signature)(const miso_engine_v1_plan *,
                                             miso_engine_v1_plan_resource_report *) =
    miso_engine_v1_plan_resources;
static uint32_t (*const watermark_signature)(const miso_engine_v1_plan *,
                                             miso_engine_v1_watermark *) =
    miso_engine_v1_plan_watermark;
static uint32_t (*const last_error_signature)(const void *, miso_engine_v1_bytes_out *) =
    miso_engine_v1_last_error;
static void (*const session_destroy_signature)(miso_engine_v1_session *) =
    miso_engine_v1_session_destroy;
static void (*const plan_destroy_signature)(miso_engine_v1_plan *) = miso_engine_v1_plan_destroy;

/* A one-track session: one stereo source routed straight to the output. */
static const char one_track_session[] =
    "{\"schema_version\":1,\"session_id\":\"abi.smoke\",\"revision\":\"3\",\"sample_rate_hz"
    "\":48000,\"quantum_frames\":128,\"render_profile\":{\"id\":\"single\",\"mode\":\"singl"
    "e_thread\"},\"output_profile\":{\"id\":\"main\",\"channels\":2,\"sample_format\":\"f32"
    "_planar\"},\"sources\":[{\"id\":\"voice\",\"content\":\"blake3:2a2a2a2a2a2a2a2a2a2a2a2"
    "a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a\",\"channels\":2,\"bit_depth\":\"32f\",\"fra"
    "mes\":\"48000\"}],\"console\":{\"pre_insert\":[],\"post_insert\":[]},\"tracks\":[{\"id"
    "\":\"vocal\",\"source_id\":\"voice\",\"left_source_channel\":0,\"right_source_channel"
    "\":1,\"builtins\":{\"left\":{\"polarity_invert\":false,\"trim_db\":0.0,\"hpf_hz\":0.0,"
    "\"lpf_hz\":0.0,\"delay_samples\":0},\"right\":{\"polarity_invert\":false,\"trim_db\":0"
    ".0,\"hpf_hz\":0.0,\"lpf_hz\":0.0,\"delay_samples\":0}},\"console\":[],\"inserts\":{\"e"
    "ffects\":[]},\"fader\":{\"left_db\":0.0,\"right_db\":0.0,\"left_mute\":false,\"right_m"
    "ute\":false},\"pan\":{\"left\":1.0,\"right\":1.0,\"smoothing_samples\":16}}],\"submixe"
    "s\":[],\"vcas\":[],\"outputs\":[{\"id\":\"main-out\"}],\"routes\":[{\"id\":\"to-main\""
    ",\"source\":{\"kind\":\"track\",\"track_id\":\"vocal\",\"tap\":\"post_pan\"},\"destina"
    "tion\":{\"kind\":\"output_input\",\"output_id\":\"main-out\"},\"channel_matrix\":{\"ll"
    "\":1.0,\"lr\":0.0,\"rl\":0.0,\"rr\":1.0},\"gain_db\":0.0,\"mute\":false,\"follows_mute"
    "\":false}],\"automation\":[]}";

/* Issue 1314: the watermark of a live plan, read the way a host reads it. Returns 0 on success. */
static int check_plan_watermark(miso_engine_v1_engine *engine) {
    miso_engine_v1_compile_limits limits = {0};
    miso_engine_v1_bytes_out diagnostics = {0};
    miso_engine_v1_session *session = NULL;
    miso_engine_v1_plan *plan = NULL;
    miso_engine_v1_watermark watermark = {0};
    int failed = 0;

    limits.struct_size = MISO_ENGINE_V1_COMPILE_LIMITS_SIZE;
    limits.source_ring_frames = 1024;
    limits.maximum_automation_spans_per_block = 128;
    limits.maximum_document_bytes = 1000000;
    limits.maximum_diagnostic_bytes = 4096;
    limits.maximum_tracks = 100;
    limits.maximum_sources = 100;
    limits.maximum_routes = 100;
    limits.maximum_effects = 100;
    limits.maximum_graph_session_plus_plan_bytes = 100000000;
    limits.maximum_source_total_bytes = 10000000;
    limits.maximum_source_overhead_bytes = 10000000;
    limits.maximum_effect_state_bytes = 100000000;
    limits.maximum_effect_scratch_bytes = 100000000;
    limits.maximum_builtin_retained_bytes = 100000000;
    limits.maximum_capi_retained_bytes = 10000000;
    limits.maximum_named_allocation_bytes = 100000000;
    limits.maximum_meter_streams = 1;
    limits.maximum_meter_items = 1;
    limits.maximum_meter_bytes = 1;
    limits.maximum_control_frame_bytes = 4096;
    limits.maximum_replay_bytes = 8192;
    limits.maximum_replay_entries = 16;
    diagnostics.struct_size = MISO_ENGINE_V1_BYTES_OUT_SIZE;
    if (compile_session_signature(engine, (const uint8_t *)one_track_session,
                                  sizeof(one_track_session) - 1, &limits, &diagnostics, &session,
                                  &plan) != MISO_ENGINE_V1_OK ||
        session == NULL || plan == NULL) {
        return 1;
    }

    /* A valid struct: the initial watermark is the session's revision, from sample 0, exact. */
    watermark.struct_size = MISO_ENGINE_V1_WATERMARK_SIZE;
    if (watermark_signature(plan, &watermark) != MISO_ENGINE_V1_OK ||
        watermark.struct_size != MISO_ENGINE_V1_WATERMARK_SIZE || watermark.revision != 3 ||
        watermark.first_sample != 0 || watermark.outcome_flags != MISO_ENGINE_V1_OUTCOME_EXACT ||
        watermark.exact_count != 0 || watermark.transition_fallback_count != 0 ||
        watermark.superseded_count != 0) {
        failed = 2;
    }

    /* A wrong struct size is refused. */
    watermark.struct_size = MISO_ENGINE_V1_WATERMARK_SIZE - 8;
    if (failed == 0 && watermark_signature(plan, &watermark) != MISO_ENGINE_V1_INVALID_ARGUMENT) {
        failed = 3;
    }

    plan_destroy_signature(plan);
    session_destroy_signature(session);
    return failed;
}

int main(void) {
    miso_engine_v1_capabilities capabilities = {0};
    miso_engine_v1_engine_config config = {0};
    miso_engine_v1_engine *engine = NULL;

    capabilities.struct_size = MISO_ENGINE_V1_CAPABILITIES_SIZE;
    if (abi_version_signature() != MISO_ENGINE_V1_ABI_VERSION ||
        query_capabilities_signature(&capabilities) != MISO_ENGINE_V1_OK ||
        capabilities.abi_version != MISO_ENGINE_V1_ABI_VERSION ||
        capabilities.exact_launch_rate_mask != MISO_ENGINE_V1_EXACT_LAUNCH_RATE_MASK ||
        capabilities.feature_mask != MISO_ENGINE_V1_FEATURE_MASK) {
        return 1;
    }

    config.struct_size = MISO_ENGINE_V1_ENGINE_CONFIG_SIZE;
    config.abi_version = MISO_ENGINE_V1_ABI_VERSION;
    if (engine_create_signature(&config, &engine) != MISO_ENGINE_V1_OK || engine == NULL) {
        return 2;
    }
    /* A host checks the watermark bit before calling the symbol (issue 1314). */
    if ((capabilities.feature_mask & MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK) == 0 ||
        check_plan_watermark(engine) != 0) {
        engine_destroy_signature(engine);
        return 5;
    }
    engine_destroy_signature(engine);

    /* A host checks the anchored-seek bit before calling the symbol (issue 1275); a null session
     * is refused before any other argument is read. */
    if ((capabilities.feature_mask & MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT) == 0 ||
        source_seek_at_signature(NULL, (const uint8_t *)"s", 1, 2, 0, 0) !=
            MISO_ENGINE_V1_INVALID_ARGUMENT) {
        return 4;
    }

    return compile_session_signature == NULL || source_submit_signature == NULL ||
                   source_seek_signature == NULL || submit_command_signature == NULL ||
                   dequeue_event_signature == NULL ||
                   render_signature == NULL || resources_signature == NULL ||
                   watermark_signature == NULL ||
                   last_error_signature == NULL || session_destroy_signature == NULL ||
                   plan_destroy_signature == NULL
               ? 3
               : 0;
}
