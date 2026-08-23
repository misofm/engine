#include "miso_engine_v2.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __cplusplus
#define QUAL_ASSERT(expression) static_assert((expression), #expression)
#define QUAL_ZERO {}
#else
#define QUAL_ASSERT(expression) _Static_assert((expression), #expression)
#define QUAL_ZERO {0}
#endif

QUAL_ASSERT(sizeof(miso_engine_v2_engine_config) == MISO_ENGINE_V2_ENGINE_CONFIG_SIZE);
QUAL_ASSERT(sizeof(miso_engine_v2_compile_limits) == MISO_ENGINE_V2_COMPILE_LIMITS_SIZE);
QUAL_ASSERT(sizeof(miso_engine_v2_plan_resource_report) ==
            MISO_ENGINE_V2_PLAN_RESOURCE_REPORT_SIZE);
QUAL_ASSERT(offsetof(miso_engine_v2_engine_config, struct_size) == 0);
QUAL_ASSERT(offsetof(miso_engine_v2_engine_config, abi_version) == 4);
QUAL_ASSERT(offsetof(miso_engine_v2_engine_config, reserved) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_compile_limits, maximum_toml_bytes) == 16);
QUAL_ASSERT(offsetof(miso_engine_v2_compile_limits, maximum_capi_retained_bytes) == 112);
QUAL_ASSERT(offsetof(miso_engine_v2_compile_limits, maximum_replay_entries) == 168);
QUAL_ASSERT(offsetof(miso_engine_v2_compile_limits, reserved) == 176);
QUAL_ASSERT(offsetof(miso_engine_v2_bytes_out, data) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_bytes_out, capacity_bytes) == 16);
QUAL_ASSERT(offsetof(miso_engine_v2_bytes_out, required_bytes) == 24);
QUAL_ASSERT(offsetof(miso_engine_v2_source_chunk, generation) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_source_chunk, planes) == 24);
QUAL_ASSERT(offsetof(miso_engine_v2_source_chunk, end_of_region) == 40);
QUAL_ASSERT(offsetof(miso_engine_v2_submit_report, accepted_frames) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_submit_report, active_generation) == 24);
QUAL_ASSERT(offsetof(miso_engine_v2_planar_output, samples) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_planar_output, frames) == 24);
QUAL_ASSERT(offsetof(miso_engine_v2_planar_output, reserved) == 32);
QUAL_ASSERT(offsetof(miso_engine_v2_capabilities, exact_launch_rate_mask) == 8);
QUAL_ASSERT(offsetof(miso_engine_v2_capabilities, reserved) == 24);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, source_count) == 16);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, graph_session_plus_plan_bytes) == 56);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, source_total_bytes) == 144);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, capi_retained_bytes) == 192);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, largest_named_allocation_bytes) == 200);
QUAL_ASSERT(offsetof(miso_engine_v2_plan_resource_report, reserved) == 208);

static const uint8_t CAPABILITIES_GET[] = {
    0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x30, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
};

static miso_engine_v2_compile_limits limits(void) {
    miso_engine_v2_compile_limits value = QUAL_ZERO;
    value.struct_size = MISO_ENGINE_V2_COMPILE_LIMITS_SIZE;
    value.source_ring_frames = 1024;
    value.maximum_automation_spans_per_block = 128;
    value.maximum_toml_bytes = UINT64_C(1000000);
    value.maximum_diagnostic_bytes = 4096;
    value.maximum_tracks = 100;
    value.maximum_sources = 100;
    value.maximum_routes = 100;
    value.maximum_effects = 100;
    value.maximum_graph_session_plus_plan_bytes = UINT64_C(100000000);
    value.maximum_source_total_bytes = UINT64_C(10000000);
    value.maximum_source_overhead_bytes = UINT64_C(10000000);
    value.maximum_effect_state_bytes = UINT64_C(100000000);
    value.maximum_effect_scratch_bytes = UINT64_C(100000000);
    value.maximum_builtin_retained_bytes = UINT64_C(100000000);
    value.maximum_capi_retained_bytes = UINT64_C(10000000);
    value.maximum_named_allocation_bytes = UINT64_C(100000000);
    value.maximum_meter_streams = 1;
    value.maximum_meter_items = 1;
    value.maximum_meter_bytes = 1;
    value.maximum_control_frame_bytes = 4096;
    value.maximum_replay_bytes = 8192;
    value.maximum_replay_entries = 16;
    return value;
}

static uint8_t *read_file(const char *path, size_t *size) {
    FILE *file = fopen(path, "rb");
    long length;
    uint8_t *bytes;
    if (file == NULL || fseek(file, 0, SEEK_END) != 0 || (length = ftell(file)) <= 0 ||
        fseek(file, 0, SEEK_SET) != 0) {
        if (file != NULL) {
            fclose(file);
        }
        return NULL;
    }
    bytes = (uint8_t *)malloc((size_t)length);
    if (bytes == NULL || fread(bytes, 1, (size_t)length, file) != (size_t)length) {
        free(bytes);
        fclose(file);
        return NULL;
    }
    fclose(file);
    *size = (size_t)length;
    return bytes;
}

static int qualify_one(const uint8_t *toml, size_t toml_bytes, int plan_first) {
    miso_engine_v2_engine_config config = QUAL_ZERO;
    miso_engine_v2_engine *engine = NULL;
    miso_engine_v2_session *session = NULL;
    miso_engine_v2_plan *plan = NULL;
    miso_engine_v2_compile_limits compile_limits = limits();
    uint8_t diagnostics_storage[4096] = {0};
    miso_engine_v2_bytes_out diagnostics = {MISO_ENGINE_V2_BYTES_OUT_SIZE, 0,
                                            diagnostics_storage, sizeof(diagnostics_storage), 0};
    float left[128] = {0};
    float right[128] = {0};
    const float *planes[2] = {left, right};
    miso_engine_v2_source_chunk chunk = QUAL_ZERO;
    miso_engine_v2_submit_report submit = QUAL_ZERO;
    float pcm[256] = {0};
    miso_engine_v2_planar_output output = QUAL_ZERO;
    miso_engine_v2_plan_resource_report resources = QUAL_ZERO;
    uint8_t response_storage[4096] = {0};
    uint8_t replay_storage[4096] = {0};
    miso_engine_v2_bytes_out response = {MISO_ENGINE_V2_BYTES_OUT_SIZE, 0, response_storage,
                                         sizeof(response_storage), 0};
    miso_engine_v2_bytes_out replay = {MISO_ENGINE_V2_BYTES_OUT_SIZE, 0, replay_storage,
                                       sizeof(replay_storage), 0};
    uint8_t canary[47];
    miso_engine_v2_bytes_out short_response = {MISO_ENGINE_V2_BYTES_OUT_SIZE, 0, canary,
                                               sizeof(canary), 0};
    miso_engine_v2_bytes_out empty_event = {MISO_ENGINE_V2_BYTES_OUT_SIZE, 0, NULL, 0, 0};
    size_t index;

    memset(canary, 0xa5, sizeof(canary));
    config.struct_size = MISO_ENGINE_V2_ENGINE_CONFIG_SIZE;
    config.abi_version = MISO_ENGINE_V2_ABI_VERSION;
    config.reserved[0] = 1;
    if (miso_engine_v2_engine_create(&config, &engine) != MISO_ENGINE_V2_INVALID_ARGUMENT ||
        engine != NULL) {
        return 10;
    }
    config.reserved[0] = 0;
    if (miso_engine_v2_engine_create(&config, &engine) != MISO_ENGINE_V2_OK || engine == NULL) {
        return 11;
    }
    if (miso_engine_v2_compile_session(engine, toml, (uint64_t)toml_bytes, &compile_limits,
                                       &diagnostics, &session, &plan) != MISO_ENGINE_V2_OK ||
        session == NULL || plan == NULL) {
        miso_engine_v2_engine_destroy(engine);
        return 12;
    }
    chunk.struct_size = MISO_ENGINE_V2_SOURCE_CHUNK_SIZE;
    chunk.sample_rate_hz = 48000;
    chunk.generation = 1;
    chunk.planes = planes;
    chunk.plane_count = 2;
    chunk.frames = 128;
    submit.struct_size = MISO_ENGINE_V2_SUBMIT_REPORT_SIZE;
    if (miso_engine_v2_source_submit_planar_f32(session, (const uint8_t *)"fixture-source", 14,
                                                &chunk, &submit) != MISO_ENGINE_V2_OK ||
        submit.accepted_frames != 128 || submit.active_generation != 1) {
        return 14;
    }
    output.struct_size = MISO_ENGINE_V2_PLANAR_OUTPUT_SIZE;
    output.channels = 2;
    output.samples = pcm;
    output.sample_capacity = 256;
    output.frames = 128;
    output.plane_stride_samples = 128;
    if (miso_engine_v2_render_f32_planar(plan, 0, &output) != MISO_ENGINE_V2_OK) {
        return 15;
    }
    if (miso_engine_v2_source_seek(session, (const uint8_t *)"fixture-source", 14, 2, 128) !=
        MISO_ENGINE_V2_OK) {
        return 13;
    }
    chunk.generation = 2;
    chunk.start_frame = 128;
    if (miso_engine_v2_source_submit_planar_f32(session, (const uint8_t *)"fixture-source", 14,
                                                &chunk, &submit) != MISO_ENGINE_V2_OK ||
        submit.accepted_frames != 128 || submit.active_generation != 2 ||
        miso_engine_v2_render_f32_planar(plan, 128, &output) != MISO_ENGINE_V2_OK) {
        return 22;
    }
    resources.struct_size = MISO_ENGINE_V2_PLAN_RESOURCE_REPORT_SIZE;
    if (miso_engine_v2_plan_resources(plan, &resources) != MISO_ENGINE_V2_OK ||
        resources.abi_version != MISO_ENGINE_V2_ABI_VERSION || resources.sample_rate_hz != 48000 ||
        resources.quantum_frames != 128 || resources.source_count != 1 ||
        resources.track_count != 9) {
        return 16;
    }
    if (miso_engine_v2_submit_command(session, CAPABILITIES_GET, sizeof(CAPABILITIES_GET),
                                      &short_response) != MISO_ENGINE_V2_BUFFER_TOO_SMALL ||
        short_response.required_bytes <= sizeof(canary)) {
        return 17;
    }
    for (index = 0; index < sizeof(canary); ++index) {
        if (canary[index] != 0xa5) {
            return 18;
        }
    }
    if (miso_engine_v2_submit_command(session, CAPABILITIES_GET, sizeof(CAPABILITIES_GET),
                                      &response) != MISO_ENGINE_V2_OK ||
        response.required_bytes <= sizeof(CAPABILITIES_GET)) {
        return 19;
    }
    if (miso_engine_v2_submit_command(session, CAPABILITIES_GET, sizeof(CAPABILITIES_GET),
                                      &replay) != MISO_ENGINE_V2_OK ||
        replay.required_bytes != response.required_bytes ||
        memcmp(response_storage, replay_storage, (size_t)response.required_bytes) != 0) {
        return 20;
    }
    if (miso_engine_v2_dequeue_event(session, UINT32_C(2), &empty_event) !=
            MISO_ENGINE_V2_INVALID_ARGUMENT ||
        miso_engine_v2_dequeue_event(session, MISO_ENGINE_V2_EVENT_LANE_RELIABLE, &empty_event) !=
            MISO_ENGINE_V2_OK ||
        empty_event.required_bytes != 0) {
        return 21;
    }
    if (plan_first) {
        miso_engine_v2_plan_destroy(plan);
        miso_engine_v2_session_destroy(session);
    } else {
        miso_engine_v2_session_destroy(session);
        miso_engine_v2_plan_destroy(plan);
    }
    miso_engine_v2_engine_destroy(engine);
    return 0;
}

int main(int argc, char **argv) {
    miso_engine_v2_capabilities capabilities = QUAL_ZERO;
    uint8_t *toml;
    size_t toml_bytes = 0;
    int first;
    int second;
    if (argc != 2 || miso_engine_v2_abi_version() != MISO_ENGINE_V2_ABI_VERSION) {
        return 1;
    }
    capabilities.struct_size = MISO_ENGINE_V2_CAPABILITIES_SIZE;
    if (miso_engine_v2_query_capabilities(&capabilities) != MISO_ENGINE_V2_OK ||
        capabilities.abi_version != MISO_ENGINE_V2_ABI_VERSION ||
        capabilities.exact_launch_rate_mask != MISO_ENGINE_V2_EXACT_LAUNCH_RATE_MASK ||
        capabilities.feature_mask != MISO_ENGINE_V2_FEATURE_MASK) {
        return 2;
    }
    toml = read_file(argv[1], &toml_bytes);
    if (toml == NULL) {
        return 3;
    }
    first = qualify_one(toml, toml_bytes, 1);
    second = qualify_one(toml, toml_bytes, 0);
    free(toml);
    return first != 0 ? first : second;
}
