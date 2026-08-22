#include "miso_engine_v2.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

static_assert(MISO_ENGINE_V2_ABI_VERSION == UINT32_C(0x00010000));
static_assert(sizeof(miso_engine_v2_engine_config) == MISO_ENGINE_V2_ENGINE_CONFIG_SIZE);
static_assert(sizeof(miso_engine_v2_compile_limits) == MISO_ENGINE_V2_COMPILE_LIMITS_SIZE);
static_assert(sizeof(miso_engine_v2_capabilities) == MISO_ENGINE_V2_CAPABILITIES_SIZE);
static_assert(std::is_standard_layout<miso_engine_v2_plan_resource_report>::value);
