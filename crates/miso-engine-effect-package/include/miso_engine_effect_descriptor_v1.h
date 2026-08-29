#ifndef MISO_ENGINE_EFFECT_DESCRIPTOR_V1_H
#define MISO_ENGINE_EFFECT_DESCRIPTOR_V1_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#define MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT static_assert
#define MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF alignof
#else
#define MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT _Static_assert
#define MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF _Alignof
#endif

#define MISO_ENGINE_EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1 UINT32_C(0x00010000)
#define MISO_ENGINE_EFFECT_DESCRIPTOR_UNAVAILABLE_V1 UINT32_MAX

enum miso_engine_effect_descriptor_diagnostic_code_v1 {
    MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1 = 0,
    MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 = 1,
    MISO_ENGINE_EFFECT_DESCRIPTOR_LIMIT_V1 = 2,
    MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1 = 3,
    MISO_ENGINE_EFFECT_DESCRIPTOR_HEADER_V1 = 4,
    MISO_ENGINE_EFFECT_DESCRIPTOR_LENGTH_V1 = 5,
    MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1 = 6,
    MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1 = 7,
    MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1 = 8,
    MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1 = 9,
    MISO_ENGINE_EFFECT_DESCRIPTOR_OFFSET_V1 = 10,
    MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1 = 11,
    MISO_ENGINE_EFFECT_DESCRIPTOR_FLOAT_V1 = 12,
    MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1 = 13,
    MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1 = 14
};

enum miso_engine_effect_parameter_unit_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_DB_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_HZ_V1 = 2,
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_MILLISECONDS_V1 = 3,
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_SAMPLES_V1 = 4,
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_LINEAR_V1 = 5,
    MISO_ENGINE_EFFECT_PARAMETER_UNIT_RATIO_V1 = 6
};

enum miso_engine_effect_parameter_domain_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_CONTINUOUS_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_BOOLEAN_V1 = 2,
    MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_ENUMERATION_V1 = 3
};

enum miso_engine_effect_parameter_mapping_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_MAPPING_LINEAR_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_MAPPING_LOGARITHMIC_V1 = 2,
    MISO_ENGINE_EFFECT_PARAMETER_MAPPING_EXPONENTIAL_V1 = 3,
    MISO_ENGINE_EFFECT_PARAMETER_MAPPING_STEPPED_V1 = 4
};

enum miso_engine_effect_automation_rate_v1 {
    MISO_ENGINE_EFFECT_AUTOMATION_RATE_SAMPLE_V1 = 1,
    MISO_ENGINE_EFFECT_AUTOMATION_RATE_BLOCK_V1 = 2,
    MISO_ENGINE_EFFECT_AUTOMATION_RATE_NONE_V1 = 3
};

enum miso_engine_effect_parameter_channel_policy_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_CHANNEL_POLICY_SHARED_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_CHANNEL_POLICY_PER_LANE_V1 = 2
};

enum miso_engine_effect_smoothing_rule_v1 {
    MISO_ENGINE_EFFECT_SMOOTHING_NONE_V1 = 1,
    MISO_ENGINE_EFFECT_SMOOTHING_LINEAR_V1 = 2,
    MISO_ENGINE_EFFECT_SMOOTHING_ONE_POLE_99_V1 = 3
};

enum miso_engine_effect_parameter_step_unit_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_STEP_ABSOLUTE_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_STEP_CENTS_V1 = 2,
    MISO_ENGINE_EFFECT_PARAMETER_STEP_RATIO_V1 = 3,
    MISO_ENGINE_EFFECT_PARAMETER_STEP_INDEX_V1 = 4
};

/* Packed step_spec fields. The step-unit field stores enum value minus one. */
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_XS_MASK_V1 UINT32_C(0x0000001f)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_SM_MASK_V1 UINT32_C(0x000003e0)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_MD_MASK_V1 UINT32_C(0x00007c00)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_LG_MASK_V1 UINT32_C(0x000f8000)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_XL_MASK_V1 UINT32_C(0x03f00000)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_PRECISION_MASK_V1 UINT32_C(0x3c000000)
#define MISO_ENGINE_EFFECT_PARAMETER_STEP_UNIT_MASK_V1 UINT32_C(0xc0000000)

enum miso_engine_effect_port_role_v1 {
    MISO_ENGINE_EFFECT_PORT_MAIN_INPUT_V1 = 1,
    MISO_ENGINE_EFFECT_PORT_MAIN_OUTPUT_V1 = 2,
    MISO_ENGINE_EFFECT_PORT_SIDECHAIN_INPUT_V1 = 3
};

enum miso_engine_effect_port_layout_v1 {
    MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1 = 1
};

enum miso_engine_effect_quality_v1 {
    MISO_ENGINE_EFFECT_QUALITY_DRAFT_V1 = 1,
    MISO_ENGINE_EFFECT_QUALITY_NORMAL_V1 = 2,
    MISO_ENGINE_EFFECT_QUALITY_HIGH_V1 = 3
};

enum miso_engine_effect_tail_kind_v1 {
    MISO_ENGINE_EFFECT_TAIL_FINITE_V1 = 1,
    MISO_ENGINE_EFFECT_TAIL_INFINITE_V1 = 2
};

enum miso_engine_effect_link_mode_bits_v1 {
    MISO_ENGINE_EFFECT_LINK_DUAL_MONO_V1 = 1,
    MISO_ENGINE_EFFECT_LINK_MAXIMUM_V1 = 2,
    MISO_ENGINE_EFFECT_LINK_AVERAGE_V1 = 4
};

enum miso_engine_effect_parameter_flags_v1 {
    MISO_ENGINE_EFFECT_PARAMETER_READABLE_V1 = 1,
    MISO_ENGINE_EFFECT_PARAMETER_AUTOMATABLE_V1 = 2,
    MISO_ENGINE_EFFECT_PARAMETER_HAS_MINIMUM_V1 = 4,
    MISO_ENGINE_EFFECT_PARAMETER_HAS_MAXIMUM_V1 = 8
};

typedef struct miso_engine_effect_parameter_record_v1 {
    uint32_t id;
    uint32_t unit;
    uint32_t domain;
    uint32_t mapping;
    uint32_t automation_rate;
    uint32_t channel_policy;
    uint32_t smoothing;
    uint32_t smoothing_samples;
    uint32_t flags;
    uint32_t minimum_bits;
    uint32_t maximum_bits;
    uint32_t default_bits;
    uint32_t enum_start;
    uint32_t enum_count;
    uint32_t display_name_offset;
    uint32_t display_name_length;
    uint32_t display_unit_offset;
    uint32_t display_unit_length;
    uint32_t step_bits;
    uint32_t step_spec;
} miso_engine_effect_parameter_record_v1;

typedef struct miso_engine_effect_port_record_v1 {
    uint32_t id_offset;
    uint32_t id_length;
    uint32_t role;
    uint32_t required;
    uint32_t layout;
    uint32_t reserved;
} miso_engine_effect_port_record_v1;

typedef struct miso_engine_effect_quality_record_v1 {
    uint32_t quality;
    uint32_t sample_rate;
    uint64_t latency_samples;
    uint32_t tail_kind;
    uint32_t reserved0;
    uint64_t tail_samples;
    uint32_t common_state_bytes;
    uint32_t left_state_bytes;
    uint32_t right_state_bytes;
    uint32_t reserved1;
    uint64_t scratch_fixed_bytes;
    uint64_t scratch_bytes_per_frame;
} miso_engine_effect_quality_record_v1;

typedef struct miso_engine_effect_enum_choice_record_v1 {
    uint32_t value_bits;
    uint32_t label_offset;
    uint32_t label_length;
    uint32_t reserved;
} miso_engine_effect_enum_choice_record_v1;

/*
 * One declared observation tap (issue #143), projected from the 32-byte wire record.
 *
 * The wire packs the six vocabularies and the two string lengths into single bytes because the
 * record is 32 bytes; this projection widens each of them to uint32_t so a C caller reads one
 * uniform record shape.
 */
typedef struct miso_engine_effect_observation_record_v1 {
    uint32_t id;
    uint32_t kind;
    uint32_t unit;
    uint32_t cost;
    uint32_t cadence;
    uint32_t fold;
    uint32_t channels;
    uint32_t minimum_bits;
    uint32_t maximum_bits;
    uint32_t display_name_offset;
    uint32_t display_name_length;
    uint32_t display_unit_offset;
    uint32_t display_unit_length;
    uint32_t reserved;
} miso_engine_effect_observation_record_v1;

typedef struct miso_engine_effect_descriptor_summary_v1 {
    uint32_t abi_version;
    uint32_t total_bytes;
    uint32_t parameter_count;
    uint32_t port_count;
    uint32_t quality_count;
    uint32_t enum_choice_count;
    uint32_t state_layout_version;
    uint32_t supported_link_mode_bits;
    uint8_t identity[32];
} miso_engine_effect_descriptor_summary_v1;

typedef struct miso_engine_effect_descriptor_diagnostic_v1 {
    uint32_t code;
    uint32_t byte_offset;
    uint32_t record_index;
    uint32_t required_bytes;
} miso_engine_effect_descriptor_diagnostic_v1;

#define MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(type, field, offset)                  \
    MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(offsetof(type, field) == (offset),       \
                                                #type "." #field " offset")

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_parameter_record_v1) == 80,
                                            "parameter record size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_parameter_record_v1) == 4,
    "parameter record alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, id, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, unit, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, domain, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, mapping, 12);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, automation_rate, 16);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, channel_policy, 20);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, smoothing, 24);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, smoothing_samples, 28);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, flags, 32);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, minimum_bits, 36);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, maximum_bits, 40);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, default_bits, 44);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, enum_start, 48);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, enum_count, 52);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, display_name_offset, 56);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, display_name_length, 60);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, display_unit_offset, 64);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, display_unit_length, 68);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, step_bits, 72);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_parameter_record_v1, step_spec, 76);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_port_record_v1) == 24,
                                            "port record size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_port_record_v1) == 4,
    "port record alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, id_offset, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, id_length, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, role, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, required, 12);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, layout, 16);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_port_record_v1, reserved, 20);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_quality_record_v1) == 64,
                                            "quality record size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_quality_record_v1) == 8,
    "quality record alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, quality, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, sample_rate, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, latency_samples, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, tail_kind, 16);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, reserved0, 20);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, tail_samples, 24);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, common_state_bytes, 32);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, left_state_bytes, 36);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, right_state_bytes, 40);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, reserved1, 44);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, scratch_fixed_bytes, 48);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_quality_record_v1, scratch_bytes_per_frame, 56);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_enum_choice_record_v1) == 16,
                                            "enum choice record size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_enum_choice_record_v1) == 4,
    "enum choice record alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_enum_choice_record_v1, value_bits, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_enum_choice_record_v1, label_offset, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_enum_choice_record_v1, label_length, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_enum_choice_record_v1, reserved, 12);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_descriptor_summary_v1) == 64,
                                            "summary size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_descriptor_summary_v1) == 4,
    "summary alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, abi_version, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, total_bytes, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, parameter_count, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, port_count, 12);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, quality_count, 16);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, enum_choice_count, 20);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, state_layout_version, 24);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, supported_link_mode_bits, 28);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_summary_v1, identity, 32);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_observation_record_v1) == 56,
                                            "observation record size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_observation_record_v1) == 4,
    "observation record alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, id, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, kind, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, unit, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, cost, 12);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, cadence, 16);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, fold, 20);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, channels, 24);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, minimum_bits, 28);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, maximum_bits, 32);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, display_name_offset, 36);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, display_name_length, 40);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, display_unit_offset, 44);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, display_unit_length, 48);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_observation_record_v1, reserved, 52);

MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(sizeof(miso_engine_effect_descriptor_diagnostic_v1) == 16,
                                            "diagnostic size");
MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT(
    MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF(miso_engine_effect_descriptor_diagnostic_v1) == 4,
    "diagnostic alignment");
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_diagnostic_v1, code, 0);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_diagnostic_v1, byte_offset, 4);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_diagnostic_v1, record_index, 8);
MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD(miso_engine_effect_descriptor_diagnostic_v1, required_bytes, 12);

/*
 * Verify one complete canonical wire value and project it into caller-owned records.
 * Summary, all four required-count pointers, and diagnostic are mandatory. Wire may be null only
 * when wire_len is zero; a record pointer may be null only when its capacity is zero. Every
 * pointed-to region must be valid for the call and must not overlap another input/output region.
 * No pointer is retained. A short array publishes only required counts and diagnostic; any other
 * failure publishes only zero required counts and diagnostic. Summary and record arrays are
 * published only after complete verification and capacity checks.
 */
uint32_t miso_engine_effect_descriptor_v1_inspect(
    const uint8_t *wire,
    size_t wire_len,
    uint32_t maximum_wire_bytes,
    miso_engine_effect_descriptor_summary_v1 *summary,
    miso_engine_effect_parameter_record_v1 *parameters,
    uint32_t parameter_capacity,
    miso_engine_effect_port_record_v1 *ports,
    uint32_t port_capacity,
    miso_engine_effect_quality_record_v1 *qualities,
    uint32_t quality_capacity,
    miso_engine_effect_enum_choice_record_v1 *enum_choices,
    uint32_t enum_choice_capacity,
    uint32_t *required_parameters,
    uint32_t *required_ports,
    uint32_t *required_qualities,
    uint32_t *required_enum_choices,
    miso_engine_effect_descriptor_diagnostic_v1 *diagnostic);

/*
 * Project the observation menu of one complete canonical wire value (issue #143).
 *
 * A separate entry point rather than four more arguments on the frozen inspect above: #143 changes
 * no existing field, offset, size or signature. `required_observations` and `diagnostic` are
 * mandatory; `observations` may be null only when `observation_capacity` is zero. A zero-tap
 * descriptor writes zero records and returns OK.
 */
uint32_t miso_engine_effect_descriptor_v1_inspect_observations(
    const uint8_t *wire,
    size_t wire_len,
    uint32_t maximum_wire_bytes,
    miso_engine_effect_observation_record_v1 *observations,
    uint32_t observation_capacity,
    uint32_t *required_observations,
    miso_engine_effect_descriptor_diagnostic_v1 *diagnostic);

#undef MISO_ENGINE_EFFECT_DESCRIPTOR_ASSERT_FIELD

#ifdef __cplusplus
}
#endif

#undef MISO_ENGINE_EFFECT_DESCRIPTOR_STATIC_ASSERT
#undef MISO_ENGINE_EFFECT_DESCRIPTOR_ALIGNOF

#endif
