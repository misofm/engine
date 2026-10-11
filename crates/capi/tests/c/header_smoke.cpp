#include "miso_engine_v1.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

static_assert(MISO_ENGINE_V1_ABI_VERSION == UINT32_C(0x00010000));
static_assert(sizeof(miso_engine_v1_engine_config) == MISO_ENGINE_V1_ENGINE_CONFIG_SIZE);
static_assert(sizeof(miso_engine_v1_compile_limits) == MISO_ENGINE_V1_COMPILE_LIMITS_SIZE);
static_assert(sizeof(miso_engine_v1_capabilities) == MISO_ENGINE_V1_CAPABILITIES_SIZE);
static_assert(std::is_standard_layout<miso_engine_v1_plan_resource_report>::value);
static_assert(sizeof(miso_engine_v1_watermark) == MISO_ENGINE_V1_WATERMARK_SIZE);
static_assert(MISO_ENGINE_V1_WATERMARK_SIZE == 96);
static_assert(std::is_standard_layout<miso_engine_v1_watermark>::value);
static_assert(offsetof(miso_engine_v1_watermark, struct_size) == 0);
static_assert(offsetof(miso_engine_v1_watermark, reserved0) == 4);
static_assert(offsetof(miso_engine_v1_watermark, revision) == 8);
static_assert(offsetof(miso_engine_v1_watermark, first_sample) == 16);
static_assert(offsetof(miso_engine_v1_watermark, outcome_flags) == 24);
static_assert(offsetof(miso_engine_v1_watermark, exact_count) == 32);
static_assert(offsetof(miso_engine_v1_watermark, transition_fallback_count) == 40);
static_assert(offsetof(miso_engine_v1_watermark, superseded_count) == 48);
static_assert(offsetof(miso_engine_v1_watermark, reserved) == 56);
static_assert((MISO_ENGINE_V1_FEATURE_MASK & MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK) ==
              MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK);
static_assert((MISO_ENGINE_V1_FEATURE_MASK & MISO_ENGINE_V1_FEATURE_SERVICE) ==
              MISO_ENGINE_V1_FEATURE_SERVICE);
static_assert(std::is_same<decltype(&miso_engine_v1_service),
                           uint32_t (*)(miso_engine_v1_session *)>::value);
