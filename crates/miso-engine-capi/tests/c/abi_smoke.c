#include "miso_engine_v2.h"

#include <stddef.h>
#include <stdint.h>

#define ABI_ASSERT(expression) _Static_assert((expression), #expression)

ABI_ASSERT(MISO_ENGINE_V2_ABI_VERSION == UINT32_C(0x00010000));
ABI_ASSERT(MISO_ENGINE_V2_OK == 0);
ABI_ASSERT(MISO_ENGINE_V2_INVALID_ARGUMENT == 1);
ABI_ASSERT(MISO_ENGINE_V2_ABI_MISMATCH == 2);
ABI_ASSERT(MISO_ENGINE_V2_WRONG_HANDLE == 3);
ABI_ASSERT(MISO_ENGINE_V2_BUFFER_TOO_SMALL == 4);
ABI_ASSERT(MISO_ENGINE_V2_COMPILE_REJECTED == 5);
ABI_ASSERT(MISO_ENGINE_V2_BACKPRESSURE == 6);
ABI_ASSERT(MISO_ENGINE_V2_UNSUPPORTED == 7);
ABI_ASSERT(MISO_ENGINE_V2_RENDER_REJECTED == 8);
ABI_ASSERT(MISO_ENGINE_V2_INTERNAL == 255);
ABI_ASSERT(MISO_ENGINE_V2_TAIL_FINITE == 0);
ABI_ASSERT(MISO_ENGINE_V2_TAIL_INFINITE == 1);
ABI_ASSERT(MISO_ENGINE_V2_EXACT_LAUNCH_RATE_MASK == 15);
ABI_ASSERT(MISO_ENGINE_V2_FEATURE_MASK == 31);

ABI_ASSERT(sizeof(miso_engine_v2_engine_config) == MISO_ENGINE_V2_ENGINE_CONFIG_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_compile_limits) == MISO_ENGINE_V2_COMPILE_LIMITS_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_bytes_out) == MISO_ENGINE_V2_BYTES_OUT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_source_chunk) == MISO_ENGINE_V2_SOURCE_CHUNK_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_submit_report) == MISO_ENGINE_V2_SUBMIT_REPORT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_planar_output) == MISO_ENGINE_V2_PLANAR_OUTPUT_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_capabilities) == MISO_ENGINE_V2_CAPABILITIES_SIZE);
ABI_ASSERT(sizeof(miso_engine_v2_plan_resource_report) ==
           MISO_ENGINE_V2_PLAN_RESOURCE_REPORT_SIZE);

ABI_ASSERT(_Alignof(miso_engine_v2_engine_config) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_compile_limits) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_bytes_out) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_source_chunk) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_submit_report) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_planar_output) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_capabilities) == 8);
ABI_ASSERT(_Alignof(miso_engine_v2_plan_resource_report) == 8);

ABI_ASSERT(offsetof(miso_engine_v2_compile_limits, maximum_toml_bytes) == 16);
ABI_ASSERT(offsetof(miso_engine_v2_compile_limits, maximum_replay_entries) == 168);
ABI_ASSERT(offsetof(miso_engine_v2_compile_limits, reserved) == 176);
ABI_ASSERT(offsetof(miso_engine_v2_bytes_out, data) == 8);
ABI_ASSERT(offsetof(miso_engine_v2_bytes_out, required_bytes) == 24);
ABI_ASSERT(offsetof(miso_engine_v2_source_chunk, planes) == 24);
ABI_ASSERT(offsetof(miso_engine_v2_source_chunk, reserved0) == 44);
ABI_ASSERT(offsetof(miso_engine_v2_planar_output, samples) == 8);
ABI_ASSERT(offsetof(miso_engine_v2_planar_output, reserved) == 32);
ABI_ASSERT(offsetof(miso_engine_v2_capabilities, reserved) == 24);
ABI_ASSERT(offsetof(miso_engine_v2_plan_resource_report, source_count) == 16);
ABI_ASSERT(offsetof(miso_engine_v2_plan_resource_report, reserved) == 208);

static uint32_t (*const abi_version_signature)(void) = miso_engine_v2_abi_version;
static uint32_t (*const query_capabilities_signature)(miso_engine_v2_capabilities *) =
    miso_engine_v2_query_capabilities;
static uint32_t (*const engine_create_signature)(const miso_engine_v2_engine_config *,
                                                 miso_engine_v2_engine **) =
    miso_engine_v2_engine_create;
static void (*const engine_destroy_signature)(miso_engine_v2_engine *) =
    miso_engine_v2_engine_destroy;
static uint32_t (*const compile_session_signature)(miso_engine_v2_engine *,
                                                   const uint8_t *,
                                                   uint64_t,
                                                   const miso_engine_v2_compile_limits *,
                                                   miso_engine_v2_bytes_out *,
                                                   miso_engine_v2_session **,
                                                   miso_engine_v2_plan **) =
    miso_engine_v2_compile_session;
static uint32_t (*const source_submit_signature)(miso_engine_v2_session *,
                                                 const uint8_t *,
                                                 uint64_t,
                                                 const miso_engine_v2_source_chunk *,
                                                 miso_engine_v2_submit_report *) =
    miso_engine_v2_source_submit_planar_f32;
static uint32_t (*const source_seek_signature)(miso_engine_v2_session *,
                                               const uint8_t *,
                                               uint64_t,
                                               uint64_t,
                                               uint64_t) = miso_engine_v2_source_seek;
static uint32_t (*const submit_command_signature)(miso_engine_v2_session *,
                                                  const uint8_t *,
                                                  uint64_t,
                                                  miso_engine_v2_bytes_out *) =
    miso_engine_v2_submit_command;
static uint32_t (*const render_signature)(miso_engine_v2_plan *,
                                          uint64_t,
                                          const miso_engine_v2_planar_output *) =
    miso_engine_v2_render_f32_planar;
static uint32_t (*const resources_signature)(const miso_engine_v2_plan *,
                                             miso_engine_v2_plan_resource_report *) =
    miso_engine_v2_plan_resources;
static uint32_t (*const last_error_signature)(const void *, miso_engine_v2_bytes_out *) =
    miso_engine_v2_last_error;
static void (*const session_destroy_signature)(miso_engine_v2_session *) =
    miso_engine_v2_session_destroy;
static void (*const plan_destroy_signature)(miso_engine_v2_plan *) = miso_engine_v2_plan_destroy;

int main(void) {
    miso_engine_v2_capabilities capabilities = {0};
    miso_engine_v2_engine_config config = {0};
    miso_engine_v2_engine *engine = NULL;

    capabilities.struct_size = MISO_ENGINE_V2_CAPABILITIES_SIZE;
    if (abi_version_signature() != MISO_ENGINE_V2_ABI_VERSION ||
        query_capabilities_signature(&capabilities) != MISO_ENGINE_V2_OK ||
        capabilities.abi_version != MISO_ENGINE_V2_ABI_VERSION ||
        capabilities.exact_launch_rate_mask != MISO_ENGINE_V2_EXACT_LAUNCH_RATE_MASK ||
        capabilities.feature_mask != MISO_ENGINE_V2_FEATURE_MASK) {
        return 1;
    }

    config.struct_size = MISO_ENGINE_V2_ENGINE_CONFIG_SIZE;
    config.abi_version = MISO_ENGINE_V2_ABI_VERSION;
    if (engine_create_signature(&config, &engine) != MISO_ENGINE_V2_OK || engine == NULL) {
        return 2;
    }
    engine_destroy_signature(engine);

    return compile_session_signature == NULL || source_submit_signature == NULL ||
                   source_seek_signature == NULL || submit_command_signature == NULL ||
                   render_signature == NULL || resources_signature == NULL ||
                   last_error_signature == NULL || session_destroy_signature == NULL ||
                   plan_destroy_signature == NULL
               ? 3
               : 0;
}
