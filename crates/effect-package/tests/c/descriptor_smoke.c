#include "miso_engine_effect_descriptor_v1.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WIRE_BYTES 429u
#define MAXIMUM_WIRE_BYTES 4096u

static void put_u16(uint8_t *bytes, size_t offset, uint16_t value) {
    bytes[offset] = (uint8_t)value;
    bytes[offset + 1] = (uint8_t)(value >> 8);
}

static void put_u32(uint8_t *bytes, size_t offset, uint32_t value) {
    for (size_t index = 0; index < 4; ++index) {
        bytes[offset + index] = (uint8_t)(value >> (8 * index));
    }
}

static void put_u64(uint8_t *bytes, size_t offset, uint64_t value) {
    for (size_t index = 0; index < 8; ++index) {
        bytes[offset + index] = (uint8_t)(value >> (8 * index));
    }
}

static uint32_t get_u32(const uint8_t *bytes, size_t offset) {
    uint32_t value = 0;
    for (size_t index = 0; index < 4; ++index) {
        value |= (uint32_t)bytes[offset + index] << (8 * index);
    }
    return value;
}

static uint64_t get_u64(const uint8_t *bytes, size_t offset) {
    uint64_t value = 0;
    for (size_t index = 0; index < 8; ++index) {
        value |= (uint64_t)bytes[offset + index] << (8 * index);
    }
    return value;
}

static void make_wire(uint8_t wire[WIRE_BYTES]) {
    static const uint32_t rates[4] = {44100, 48000, 88200, 96000};
    memset(wire, 0, WIRE_BYTES);
    memcpy(wire, "MISOEFD1", 8);
    put_u16(wire, 8, 1);
    put_u16(wire, 10, 96);
    put_u32(wire, 16, WIRE_BYTES);
    put_u16(wire, 20, 1);
    put_u16(wire, 22, 0);
    put_u32(wire, 24, 1);
    put_u32(wire, 28, MISO_ENGINE_EFFECT_LINK_DUAL_MONO_V1);
    put_u32(wire, 32, 400);
    put_u32(wire, 36, 7);
    put_u32(wire, 40, 407);
    put_u32(wire, 44, 7);
    put_u32(wire, 48, 0);
    put_u32(wire, 52, 96);
    put_u32(wire, 56, 2);
    put_u32(wire, 60, 96);
    put_u32(wire, 64, 4);
    put_u32(wire, 68, 144);
    put_u32(wire, 72, 0);
    put_u32(wire, 76, 400);
    put_u32(wire, 80, 29);
    put_u32(wire, 84, 400);

    put_u32(wire, 96, 414);
    put_u32(wire, 100, 7);
    put_u32(wire, 104, MISO_ENGINE_EFFECT_PORT_MAIN_INPUT_V1);
    put_u32(wire, 108, 1);
    put_u32(wire, 112, MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1);
    put_u32(wire, 120, 421);
    put_u32(wire, 124, 8);
    put_u32(wire, 128, MISO_ENGINE_EFFECT_PORT_MAIN_OUTPUT_V1);
    put_u32(wire, 132, 1);
    put_u32(wire, 136, MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1);

    for (size_t index = 0; index < 4; ++index) {
        const size_t record = 144 + index * 64;
        put_u32(wire, record, MISO_ENGINE_EFFECT_QUALITY_NORMAL_V1);
        put_u32(wire, record + 4, rates[index]);
        put_u64(wire, record + 8, 0);
        put_u32(wire, record + 16, MISO_ENGINE_EFFECT_TAIL_FINITE_V1);
        put_u64(wire, record + 24, 0);
        put_u32(wire, record + 32, 0);
        put_u32(wire, record + 36, 0);
        put_u32(wire, record + 40, 0);
        put_u64(wire, record + 48, 0);
        put_u64(wire, record + 56, 0);
    }
    memcpy(wire + 400, "c.smoke", 7);
    memcpy(wire + 407, "C Smoke", 7);
    memcpy(wire + 414, "main-in", 7);
    memcpy(wire + 421, "main-out", 8);
}

static void require(int condition, const char *message) {
    if (!condition) {
        fprintf(stderr, "C descriptor smoke failure: %s\n", message);
        exit(1);
    }
}

static int hex_digit(int value) {
    if (value >= '0' && value <= '9') {
        return value - '0';
    }
    if (value >= 'a' && value <= 'f') {
        return value - 'a' + 10;
    }
    return -1;
}

static size_t read_hex_fixture(const char *path, uint8_t **output) {
    FILE *file = fopen(path, "rb");
    require(file != NULL, "open comprehensive fixture");
    require(fseek(file, 0, SEEK_END) == 0, "seek comprehensive fixture");
    const long text_length = ftell(file);
    require(text_length > 0 && fseek(file, 0, SEEK_SET) == 0, "measure comprehensive fixture");
    char *text = malloc((size_t)text_length + 1);
    require(text != NULL, "allocate fixture text");
    require(fread(text, 1, (size_t)text_length, file) == (size_t)text_length,
            "read comprehensive fixture");
    require(fclose(file) == 0, "close comprehensive fixture");
    uint8_t *bytes = malloc((size_t)text_length / 2 + 1);
    require(bytes != NULL, "allocate fixture bytes");
    int high = -1;
    size_t length = 0;
    for (long index = 0; index < text_length; ++index) {
        const int digit = hex_digit((unsigned char)text[index]);
        if (digit < 0) {
            require(text[index] == ' ' || text[index] == '\n' || text[index] == '\r' ||
                        text[index] == '\t',
                    "fixture contains non-hex input");
            continue;
        }
        if (high < 0) {
            high = digit;
        } else {
            bytes[length++] = (uint8_t)((high << 4) | digit);
            high = -1;
        }
    }
    require(high < 0, "fixture has odd hex digit count");
    free(text);
    *output = bytes;
    return length;
}

typedef struct parameter_canary {
    uint64_t before;
    miso_engine_effect_parameter_record_v1 rows[6];
    uint64_t after;
} parameter_canary;

typedef struct port_canary {
    uint64_t before;
    miso_engine_effect_port_record_v1 rows[3];
    uint64_t after;
} port_canary;

typedef struct quality_canary {
    uint64_t before;
    miso_engine_effect_quality_record_v1 rows[12];
    uint64_t after;
} quality_canary;

typedef struct choice_canary {
    uint64_t before;
    miso_engine_effect_enum_choice_record_v1 rows[3];
    uint64_t after;
} choice_canary;

static void require_exact_projection(
    const uint8_t *wire,
    const parameter_canary *parameters,
    const port_canary *ports,
    const quality_canary *qualities,
    const choice_canary *choices) {
    const size_t parameter_offset = get_u32(wire, 52);
    const size_t port_offset = get_u32(wire, 60);
    const size_t quality_offset = get_u32(wire, 68);
    const size_t choice_offset = get_u32(wire, 76);
    for (size_t index = 0; index < get_u32(wire, 48); ++index) {
        const size_t record = parameter_offset + index * 80;
        const miso_engine_effect_parameter_record_v1 *row = &parameters->rows[index];
        const uint32_t actual[20] = {
            row->id, row->unit, row->domain, row->mapping, row->automation_rate,
            row->channel_policy, row->smoothing, row->smoothing_samples, row->flags,
            row->minimum_bits, row->maximum_bits, row->default_bits, row->enum_start,
            row->enum_count, row->display_name_offset, row->display_name_length,
            row->display_unit_offset, row->display_unit_length, row->step_bits, row->step_spec};
        for (size_t field = 0; field < 20; ++field) {
            require(actual[field] == get_u32(wire, record + field * 4),
                    "complete parameter projection");
        }
    }
    for (size_t index = 0; index < get_u32(wire, 56); ++index) {
        const size_t record = port_offset + index * 24;
        const miso_engine_effect_port_record_v1 *row = &ports->rows[index];
        const uint32_t actual[6] = {row->id_offset, row->id_length, row->role,
                                    row->required, row->layout, row->reserved};
        for (size_t field = 0; field < 6; ++field) {
            require(actual[field] == get_u32(wire, record + field * 4),
                    "complete port projection");
        }
    }
    for (size_t index = 0; index < get_u32(wire, 64); ++index) {
        const size_t record = quality_offset + index * 64;
        const miso_engine_effect_quality_record_v1 *row = &qualities->rows[index];
        require(row->quality == get_u32(wire, record) &&
                    row->sample_rate == get_u32(wire, record + 4) &&
                    row->latency_samples == get_u64(wire, record + 8) &&
                    row->tail_kind == get_u32(wire, record + 16) &&
                    row->reserved0 == get_u32(wire, record + 20) &&
                    row->tail_samples == get_u64(wire, record + 24) &&
                    row->common_state_bytes == get_u32(wire, record + 32) &&
                    row->left_state_bytes == get_u32(wire, record + 36) &&
                    row->right_state_bytes == get_u32(wire, record + 40) &&
                    row->reserved1 == get_u32(wire, record + 44) &&
                    row->scratch_fixed_bytes == get_u64(wire, record + 48) &&
                    row->scratch_bytes_per_frame == get_u64(wire, record + 56),
                "complete quality projection");
    }
    for (size_t index = 0; index < get_u32(wire, 72); ++index) {
        const size_t record = choice_offset + index * 16;
        const miso_engine_effect_enum_choice_record_v1 *row = &choices->rows[index];
        require(row->value_bits == get_u32(wire, record) &&
                    row->label_offset == get_u32(wire, record + 4) &&
                    row->label_length == get_u32(wire, record + 8) &&
                    row->reserved == get_u32(wire, record + 12),
                "complete enum-choice projection");
    }
}

static uint32_t inspect_comprehensive(
    const uint8_t *wire,
    size_t wire_len,
    miso_engine_effect_descriptor_summary_v1 *summary,
    parameter_canary *parameters,
    uint32_t parameter_capacity,
    port_canary *ports,
    uint32_t port_capacity,
    quality_canary *qualities,
    uint32_t quality_capacity,
    choice_canary *choices,
    uint32_t choice_capacity,
    uint32_t required[4],
    miso_engine_effect_descriptor_diagnostic_v1 *diagnostic) {
    return miso_engine_effect_descriptor_v1_inspect(
        wire, wire_len, MAXIMUM_WIRE_BYTES, summary,
        parameters == NULL ? NULL : parameters->rows, parameter_capacity,
        ports == NULL ? NULL : ports->rows, port_capacity,
        qualities == NULL ? NULL : qualities->rows, quality_capacity,
        choices == NULL ? NULL : choices->rows, choice_capacity,
        required == NULL ? NULL : &required[0], required == NULL ? NULL : &required[1],
        required == NULL ? NULL : &required[2], required == NULL ? NULL : &required[3], diagnostic);
}

static void comprehensive_smoke(const char *fixture_path) {
    static const uint8_t expected_identity[32] = {
        0x7d, 0x2f, 0x1e, 0xe7, 0x9a, 0xa5, 0x83, 0x3c,
        0x54, 0x6e, 0xa0, 0x65, 0x48, 0xcb, 0x29, 0xe1,
        0x3b, 0x37, 0xf4, 0xab, 0x69, 0x0e, 0x90, 0x24,
        0xf1, 0x48, 0x0d, 0x2f, 0xdf, 0xad, 0xe2, 0x98};
    uint8_t *wire = NULL;
    const size_t wire_len = read_hex_fixture(fixture_path, &wire);
    require(wire_len == 1587, "comprehensive fixture byte length");
    parameter_canary parameters;
    port_canary ports;
    quality_canary qualities;
    choice_canary choices;
    memset(&parameters, 0, sizeof parameters);
    memset(&ports, 0, sizeof ports);
    memset(&qualities, 0, sizeof qualities);
    memset(&choices, 0, sizeof choices);
    parameters.before = parameters.after = UINT64_C(0x1122334455667788);
    ports.before = ports.after = UINT64_C(0x2233445566778899);
    qualities.before = qualities.after = UINT64_C(0x33445566778899aa);
    choices.before = choices.after = UINT64_C(0x445566778899aabb);
    miso_engine_effect_descriptor_summary_v1 summary;
    miso_engine_effect_descriptor_diagnostic_v1 diagnostic;
    uint32_t required[4] = {UINT32_MAX, UINT32_MAX, UINT32_MAX, UINT32_MAX};
    uint32_t result = inspect_comprehensive(
        wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities, 12, &choices, 3,
        required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1, "comprehensive status");
    require(required[0] == 6 && required[1] == 3 && required[2] == 12 && required[3] == 3,
            "comprehensive counts");
    require(memcmp(summary.identity, expected_identity, sizeof expected_identity) == 0,
            "comprehensive identity");
    require(parameters.rows[0].id == 1 && parameters.rows[0].unit == 1 &&
                parameters.rows[5].id == 6 && parameters.rows[5].unit == 6,
            "parameter projection");
    require(ports.rows[0].role == 1 && ports.rows[1].role == 2 && ports.rows[2].role == 3,
            "canonical port projection");
    require(qualities.rows[0].quality == 1 && qualities.rows[4].quality == 2 &&
                qualities.rows[8].quality == 3,
            "all quality projections");
    require(choices.rows[0].value_bits == UINT32_C(0xbf800000) &&
                choices.rows[1].value_bits == 0 &&
                choices.rows[2].value_bits == UINT32_C(0x3f800000),
            "enum choice projection");
    require_exact_projection(wire, &parameters, &ports, &qualities, &choices);
    require(parameters.before == UINT64_C(0x1122334455667788) &&
                parameters.after == UINT64_C(0x1122334455667788) &&
                ports.before == UINT64_C(0x2233445566778899) &&
                ports.after == UINT64_C(0x2233445566778899) &&
                qualities.before == UINT64_C(0x33445566778899aa) &&
                qualities.after == UINT64_C(0x33445566778899aa) &&
                choices.before == UINT64_C(0x445566778899aabb) &&
                choices.after == UINT64_C(0x445566778899aabb),
            "per-array exact-capacity canaries");

    const uint32_t capacities[4][4] = {
        {5, 3, 12, 3}, {6, 2, 12, 3}, {6, 3, 11, 3}, {6, 3, 12, 2}};
    for (size_t short_index = 0; short_index < 4; ++short_index) {
        parameter_canary parameters_before;
        port_canary ports_before;
        quality_canary qualities_before;
        choice_canary choices_before;
        miso_engine_effect_descriptor_summary_v1 summary_before;
        memset(&summary, 0xa5, sizeof summary);
        memset(parameters.rows, 0x5a, sizeof parameters.rows);
        memset(ports.rows, 0x6b, sizeof ports.rows);
        memset(qualities.rows, 0x7c, sizeof qualities.rows);
        memset(choices.rows, 0x8d, sizeof choices.rows);
        memcpy(&summary_before, &summary, sizeof summary);
        memcpy(&parameters_before, &parameters, sizeof parameters);
        memcpy(&ports_before, &ports, sizeof ports);
        memcpy(&qualities_before, &qualities, sizeof qualities);
        memcpy(&choices_before, &choices, sizeof choices);
        result = inspect_comprehensive(
            wire, wire_len, &summary, &parameters, capacities[short_index][0], &ports,
            capacities[short_index][1], &qualities, capacities[short_index][2], &choices,
            capacities[short_index][3], required, &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1 &&
                    diagnostic.required_bytes == 1368,
                "per-array short status/required bytes");
        require(required[0] == 6 && required[1] == 3 && required[2] == 12 && required[3] == 3,
                "per-array short required counts");
        require(memcmp(&summary, &summary_before, sizeof summary) == 0 &&
                    memcmp(&parameters, &parameters_before, sizeof parameters) == 0 &&
                    memcmp(&ports, &ports_before, sizeof ports) == 0 &&
                    memcmp(&qualities, &qualities_before, sizeof qualities) == 0 &&
                    memcmp(&choices, &choices_before, sizeof choices) == 0,
                "per-array short no partial writes");
    }

    miso_engine_effect_descriptor_summary_v1 null_summary_before;
    parameter_canary null_parameters_before;
    port_canary null_ports_before;
    quality_canary null_qualities_before;
    choice_canary null_choices_before;
    memcpy(&null_summary_before, &summary, sizeof summary);
    memcpy(&null_parameters_before, &parameters, sizeof parameters);
    memcpy(&null_ports_before, &ports, sizeof ports);
    memcpy(&null_qualities_before, &qualities, sizeof qualities);
    memcpy(&null_choices_before, &choices, sizeof choices);
    result = inspect_comprehensive(wire, wire_len, &summary, NULL, 1, &ports, 3, &qualities, 12,
                                   &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required[0] == 0 &&
                required[1] == 0 && required[2] == 0 && required[3] == 0,
            "nonnull capacity requires record pointer");
    require(memcmp(&summary, &null_summary_before, sizeof summary) == 0 &&
                memcmp(&parameters, &null_parameters_before, sizeof parameters) == 0 &&
                memcmp(&ports, &null_ports_before, sizeof ports) == 0 &&
                memcmp(&qualities, &null_qualities_before, sizeof qualities) == 0 &&
                memcmp(&choices, &null_choices_before, sizeof choices) == 0,
            "nonnull-capacity null has no partial writes");
    memcpy(&null_parameters_before, &parameters, sizeof parameters);
    memcpy(&null_ports_before, &ports, sizeof ports);
    memcpy(&null_qualities_before, &qualities, sizeof qualities);
    memcpy(&null_choices_before, &choices, sizeof choices);
    result = inspect_comprehensive(wire, wire_len, NULL, &parameters, 6, &ports, 3, &qualities, 12,
                                   &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required[0] == 0 &&
                required[1] == 0 && required[2] == 0 && required[3] == 0,
            "mandatory summary null");
    require(memcmp(&parameters, &null_parameters_before, sizeof parameters) == 0 &&
                memcmp(&ports, &null_ports_before, sizeof ports) == 0 &&
                memcmp(&qualities, &null_qualities_before, sizeof qualities) == 0 &&
                memcmp(&choices, &null_choices_before, sizeof choices) == 0,
            "mandatory summary null has no record writes");

    result = inspect_comprehensive(NULL, 1, &summary, &parameters, 6, &ports, 3, &qualities, 12,
                                   &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required[0] == 0 &&
                required[1] == 0 && required[2] == 0 && required[3] == 0,
            "null nonempty wire");

    for (size_t null_array = 0; null_array < 4; ++null_array) {
        parameter_canary *parameter_pointer = null_array == 0 ? NULL : &parameters;
        port_canary *port_pointer = null_array == 1 ? NULL : &ports;
        quality_canary *quality_pointer = null_array == 2 ? NULL : &qualities;
        choice_canary *choice_pointer = null_array == 3 ? NULL : &choices;
        result = inspect_comprehensive(
            wire, wire_len, &summary, parameter_pointer, 6, port_pointer, 3, quality_pointer, 12,
            choice_pointer, 3, required, &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required[0] == 0 &&
                    required[1] == 0 && required[2] == 0 && required[3] == 0,
                "nonnull capacity requires each record pointer");
    }

    for (size_t null_count = 0; null_count < 4; ++null_count) {
        uint32_t counts[4] = {UINT32_MAX, UINT32_MAX, UINT32_MAX, UINT32_MAX};
        result = miso_engine_effect_descriptor_v1_inspect(
            wire, wire_len, MAXIMUM_WIRE_BYTES, &summary, parameters.rows, 6, ports.rows, 3,
            qualities.rows, 12, choices.rows, 3, null_count == 0 ? NULL : &counts[0],
            null_count == 1 ? NULL : &counts[1], null_count == 2 ? NULL : &counts[2],
            null_count == 3 ? NULL : &counts[3], &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1,
                "each required-count pointer is mandatory");
        for (size_t index = 0; index < 4; ++index) {
            require(counts[index] == (index == null_count ? UINT32_MAX : 0),
                    "mandatory-null count publication");
        }
    }

    for (size_t zero_capacity = 0; zero_capacity < 4; ++zero_capacity) {
        miso_engine_effect_descriptor_summary_v1 summary_before;
        parameter_canary parameter_before;
        port_canary port_before;
        quality_canary quality_before;
        choice_canary choice_before;
        memcpy(&parameter_before, &parameters, sizeof parameters);
        memcpy(&port_before, &ports, sizeof ports);
        memcpy(&quality_before, &qualities, sizeof qualities);
        memcpy(&choice_before, &choices, sizeof choices);
        memset(&summary, 0xa5, sizeof summary);
        memcpy(&summary_before, &summary, sizeof summary);
        result = inspect_comprehensive(
            wire, wire_len, &summary, &parameters, zero_capacity == 0 ? 0 : 6, &ports,
            zero_capacity == 1 ? 0 : 3, &qualities, zero_capacity == 2 ? 0 : 12, &choices,
            zero_capacity == 3 ? 0 : 3, required, &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1,
                "nonnull zero-capacity record pointer is legal and short");
        require(memcmp(&summary, &summary_before, sizeof summary) == 0 &&
                    memcmp(&parameters, &parameter_before, sizeof parameters) == 0 &&
                    memcmp(&ports, &port_before, sizeof ports) == 0 &&
                    memcmp(&qualities, &quality_before, sizeof qualities) == 0 &&
                    memcmp(&choices, &choice_before, sizeof choices) == 0,
                "zero-capacity short call is all-or-none");
    }

    const size_t parameter_offset = get_u32(wire, 52);
    const size_t port_offset = get_u32(wire, 60);
    const size_t quality_offset = get_u32(wire, 68);
    typedef struct mutation_case {
        size_t field;
        uint32_t value;
        uint32_t code;
        uint32_t byte_offset;
        uint32_t record_index;
    } mutation_case;
    const mutation_case mutations[] = {
        {8, 2, MISO_ENGINE_EFFECT_DESCRIPTOR_HEADER_V1, 8, UINT32_MAX},
        {10, 104, MISO_ENGINE_EFFECT_DESCRIPTOR_HEADER_V1, 10, UINT32_MAX},
        {28, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1, 28, UINT32_MAX},
        {28, 8, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1, 28, UINT32_MAX},
        {28, 9, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1, 28, UINT32_MAX},
        {95, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1, 95, UINT32_MAX},
        {parameter_offset + 4, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 4, 0},
        {parameter_offset + 8, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 8, 0},
        {parameter_offset + 12, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 12, 0},
        {parameter_offset + 16, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 16, 0},
        {parameter_offset + 20, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 20, 0},
        {parameter_offset + 24, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)parameter_offset + 24, 0},
        {port_offset + 8, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)port_offset + 8, 0},
        {port_offset + 12, 2, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)port_offset + 12, 0},
        {port_offset + 16, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)port_offset + 16, 0},
        {quality_offset, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)quality_offset, 0},
        {quality_offset + 16, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1,
         (uint32_t)quality_offset + 16, 0},
        {parameter_offset + 32, 16, MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1,
         (uint32_t)parameter_offset + 32, 0},
        {parameter_offset + 3 * 80 + 36, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1,
         (uint32_t)parameter_offset + 3 * 80 + 36, 3},
        {parameter_offset + 72, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1,
         (uint32_t)parameter_offset + 72, 0},
        {port_offset + 20, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1,
         (uint32_t)port_offset + 20, 0},
        {quality_offset + 20, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1,
         (uint32_t)quality_offset + 20, 0},
        {get_u32(wire, 76) + 12, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1,
         get_u32(wire, 76) + 12, 0},
        {parameter_offset + 80, 1, MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1,
         (uint32_t)parameter_offset + 80, 1},
        {quality_offset + 64 + 4, 44100, MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1,
         (uint32_t)quality_offset + 64, 1},
        {get_u32(wire, 76) + 16, UINT32_C(0xc0000000), MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1,
         get_u32(wire, 76) + 16, 1},
        {parameter_offset + 44, UINT32_C(0x7fc00000), MISO_ENGINE_EFFECT_DESCRIPTOR_FLOAT_V1,
         (uint32_t)parameter_offset + 44, 0},
        {parameter_offset + 44, UINT32_C(0x80000000), MISO_ENGINE_EFFECT_DESCRIPTOR_FLOAT_V1,
         (uint32_t)parameter_offset + 44, 0},
        {get_u32(wire, 76), UINT32_C(0x7fc00000), MISO_ENGINE_EFFECT_DESCRIPTOR_FLOAT_V1,
         get_u32(wire, 76), 0},
        {parameter_offset + 44, UINT32_C(0x7f7fffff), MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1,
         (uint32_t)parameter_offset + 4, 0},
        {port_offset + 12, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1,
         (uint32_t)port_offset, UINT32_MAX},
        {quality_offset + 40, 17, MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1,
         (uint32_t)quality_offset + 36, 0},
        {20, 2, MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1, 20, UINT32_MAX},
        {48, UINT32_MAX, MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1, 48, UINT32_MAX},
        {56, UINT32_MAX, MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1, 56, UINT32_MAX},
        {64, UINT32_MAX, MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1, 64, UINT32_MAX},
        {72, UINT32_MAX, MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1, 72, UINT32_MAX},
        {80, UINT32_MAX, MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1, 80, UINT32_MAX}};
    for (size_t index = 0; index < sizeof mutations / sizeof mutations[0]; ++index) {
        const uint32_t saved = get_u32(wire, mutations[index].field);
        put_u32(wire, mutations[index].field, mutations[index].value);
        result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                       &qualities, 12, &choices, 3, required, &diagnostic);
        require(result == mutations[index].code && diagnostic.code == mutations[index].code &&
                    diagnostic.byte_offset == mutations[index].byte_offset &&
                    diagnostic.record_index == mutations[index].record_index &&
                    diagnostic.required_bytes == 0 && required[0] == 0 && required[1] == 0 &&
                    required[2] == 0 && required[3] == 0,
                "raw diagnostic matrix");
        put_u32(wire, mutations[index].field, saved);
    }

    const uint32_t saved_second_port_role = get_u32(wire, port_offset + 24 + 8);
    const size_t second_port_text = get_u32(wire, port_offset + 24);
    const uint8_t saved_second_port_text = wire[second_port_text];
    put_u32(wire, port_offset + 24 + 8, 1);
    wire[second_port_text] = 'a';
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1 &&
                diagnostic.byte_offset == port_offset + 24 + 8 &&
                diagnostic.record_index == 1,
            "port order diagnostic");
    put_u32(wire, port_offset + 24 + 8, saved_second_port_role);
    wire[second_port_text] = saved_second_port_text;

    for (uint32_t link_bits = 0; link_bits < 256; ++link_bits) {
        if (link_bits == 1 || link_bits == 3 || link_bits == 5 || link_bits == 7) {
            continue;
        }
        put_u32(wire, 28, link_bits);
        result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                       &qualities, 12, &choices, 3, required, &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1 && diagnostic.byte_offset == 28 &&
                    diagnostic.record_index == UINT32_MAX,
                "bounded invalid link-bit matrix");
    }
    for (uint32_t bit = 8; bit < 32; ++bit) {
        for (uint32_t base = 0; base <= 1; ++base) {
            put_u32(wire, 28, (UINT32_C(1) << bit) | base);
            result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                           &qualities, 12, &choices, 3, required, &diagnostic);
            require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1 &&
                        diagnostic.byte_offset == 28 && diagnostic.record_index == UINT32_MAX,
                    "high invalid link-bit matrix");
        }
    }
    put_u32(wire, 28, 7);

    const size_t effect_text = get_u32(wire, 32);
    const size_t first_port_text = get_u32(wire, port_offset);
    const struct {
        size_t offset;
        uint8_t value;
        uint32_t field;
        uint32_t index;
    } id_mutations[] = {
        {effect_text, 'F', 32, UINT32_MAX},
        {effect_text + 7, '/', 32, UINT32_MAX},
        {first_port_text, 'M', (uint32_t)port_offset, 0},
        {first_port_text + 4, '/', (uint32_t)port_offset, 0}};
    for (size_t index = 0; index < sizeof id_mutations / sizeof id_mutations[0]; ++index) {
        const uint8_t saved = wire[id_mutations[index].offset];
        wire[id_mutations[index].offset] = id_mutations[index].value;
        result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                       &qualities, 12, &choices, 3, required, &diagnostic);
        require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1 &&
                    diagnostic.byte_offset == id_mutations[index].field &&
                    diagnostic.record_index == id_mutations[index].index,
                "constructor-sealed raw ID matrix");
        wire[id_mutations[index].offset] = saved;
    }

    const uint32_t saved_display_offset = get_u32(wire, 40);
    put_u32(wire, 40, (uint32_t)effect_text);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OFFSET_V1 &&
                diagnostic.byte_offset == 40 && diagnostic.record_index == UINT32_MAX,
            "aliased string range diagnostic");
    put_u32(wire, 40, saved_display_offset + 1);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OFFSET_V1 &&
                diagnostic.byte_offset == 40 && diagnostic.record_index == UINT32_MAX,
            "gapped string range diagnostic");
    put_u32(wire, 40, saved_display_offset);

    const size_t display_text = get_u32(wire, 40);
    const uint8_t saved_display_text = wire[display_text];
    wire[display_text] = UINT8_C(0xff);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1 &&
                diagnostic.byte_offset == 40 && diagnostic.record_index == UINT32_MAX,
            "invalid UTF-8 diagnostic");
    wire[display_text] = '\n';
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1 &&
                diagnostic.byte_offset == 40 && diagnostic.record_index == UINT32_MAX,
            "control text diagnostic");
    wire[display_text] = saved_display_text;

    const uint64_t saved_tail = get_u64(wire, quality_offset + 8 * 64 + 24);
    put_u64(wire, quality_offset + 8 * 64 + 24, 1);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities,
                                   12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1 &&
                diagnostic.byte_offset == quality_offset + 8 * 64 + 24 &&
                diagnostic.record_index == 8,
            "infinite tail requires zero samples");
    put_u64(wire, quality_offset + 8 * 64 + 24, saved_tail);

    result = inspect_comprehensive(wire, wire_len - 1, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_LENGTH_V1 && diagnostic.byte_offset == 16,
            "truncated wire diagnostic");
    wire[wire_len] = 0;
    result = inspect_comprehensive(wire, wire_len + 1, &summary, &parameters, 6, &ports, 3,
                                   &qualities, 12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_LENGTH_V1 && diagnostic.byte_offset == 16,
            "trailing wire diagnostic");

    const uint32_t saved_flags = get_u32(wire, parameter_offset + 32);
    const uint32_t saved_reserved = get_u32(wire, parameter_offset + 72);
    put_u32(wire, parameter_offset + 32, 16);
    put_u32(wire, parameter_offset + 72, 1);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities,
                                   12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1 &&
                diagnostic.byte_offset == parameter_offset + 32,
            "record flags precede later incomplete lattice field");
    put_u32(wire, parameter_offset + 32, saved_flags);
    put_u32(wire, parameter_offset + 72, saved_reserved);

    const uint32_t saved_effect_offset = get_u32(wire, 32);
    const uint32_t saved_parameter_offset = get_u32(wire, 52);
    put_u32(wire, 32, saved_effect_offset + 1);
    put_u32(wire, 52, saved_parameter_offset + 4);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities,
                                   12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OFFSET_V1 && diagnostic.byte_offset == 32,
            "header text offset precedes later table offset");
    put_u32(wire, 32, saved_effect_offset);
    put_u32(wire, 52, saved_parameter_offset);

    const size_t port_text = get_u32(wire, port_offset);
    const size_t choice_offset = get_u32(wire, 76);
    const size_t choice_text = get_u32(wire, choice_offset + 4);
    const uint8_t saved_port_text = wire[port_text];
    const uint8_t saved_choice_text = wire[choice_text];
    wire[port_text] = 'A';
    wire[choice_text] = '\n';
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities,
                                   12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1 &&
                diagnostic.byte_offset == port_offset && diagnostic.record_index == 0,
            "port text precedes later choice text");
    wire[port_text] = saved_port_text;
    wire[choice_text] = saved_choice_text;
    free(wire);
}

static uint32_t inspect(
    const uint8_t *wire,
    miso_engine_effect_descriptor_summary_v1 *summary,
    miso_engine_effect_port_record_v1 *ports,
    uint32_t port_capacity,
    miso_engine_effect_quality_record_v1 *qualities,
    uint32_t quality_capacity,
    uint32_t *required_parameters,
    uint32_t *required_ports,
    uint32_t *required_qualities,
    uint32_t *required_choices,
    miso_engine_effect_descriptor_diagnostic_v1 *diagnostic) {
    return miso_engine_effect_descriptor_v1_inspect(
        wire, WIRE_BYTES, MAXIMUM_WIRE_BYTES, summary, NULL, 0, ports, port_capacity, qualities,
        quality_capacity, NULL, 0, required_parameters, required_ports, required_qualities,
        required_choices, diagnostic);
}

typedef struct observation_canary {
    uint64_t before;
    miso_engine_effect_observation_record_v1 rows[2];
    uint64_t after;
} observation_canary;

/* Issue #143: the additive observation projection, over one zero-tap and one tap-bearing wire. */
static void observation_smoke(const char *zero_tap_path, const char *tap_bearing_path) {
    uint8_t *zero_tap = NULL;
    const size_t zero_tap_len = read_hex_fixture(zero_tap_path, &zero_tap);
    uint8_t *tap_bearing = NULL;
    const size_t tap_bearing_len = read_hex_fixture(tap_bearing_path, &tap_bearing);
    miso_engine_effect_descriptor_diagnostic_v1 diagnostic;
    observation_canary observations;
    uint32_t required = UINT32_MAX;

    /* A descriptor that declares no tap keeps header bytes 88..96 at zero and projects nothing. */
    for (size_t index = 88; index < 96; ++index) {
        require(zero_tap[index] == 0, "zero-tap header stays reserved-zero");
    }
    uint32_t result = miso_engine_effect_descriptor_v1_inspect_observations(
        zero_tap, zero_tap_len, MAXIMUM_WIRE_BYTES, NULL, 0, &required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1 && required == 0,
            "zero-tap observation count");

    memset(&observations, 0, sizeof observations);
    observations.before = observations.after = UINT64_C(0x556677889900aabb);
    required = UINT32_MAX;
    result = miso_engine_effect_descriptor_v1_inspect_observations(
        tap_bearing, tap_bearing_len, MAXIMUM_WIRE_BYTES, observations.rows, 2, &required,
        &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1 && required == 2,
            "tap-bearing observation count");
    require(observations.before == UINT64_C(0x556677889900aabb) &&
                observations.after == UINT64_C(0x556677889900aabb),
            "observation exact-capacity canaries");
    const size_t observation_offset = get_u32(tap_bearing, 92);
    for (size_t index = 0; index < 2; ++index) {
        const size_t record = observation_offset + index * 32;
        const miso_engine_effect_observation_record_v1 *row = &observations.rows[index];
        require(row->id == get_u32(tap_bearing, record) &&
                    row->kind == tap_bearing[record + 4] &&
                    row->unit == tap_bearing[record + 5] &&
                    row->cost == tap_bearing[record + 6] &&
                    row->cadence == tap_bearing[record + 7] &&
                    row->fold == tap_bearing[record + 8] &&
                    row->channels == tap_bearing[record + 9] &&
                    row->display_name_length == tap_bearing[record + 10] &&
                    row->display_unit_length == tap_bearing[record + 11] &&
                    row->minimum_bits == get_u32(tap_bearing, record + 12) &&
                    row->maximum_bits == get_u32(tap_bearing, record + 16) &&
                    row->display_name_offset == get_u32(tap_bearing, record + 20) &&
                    row->display_unit_offset == get_u32(tap_bearing, record + 24) &&
                    row->reserved == get_u32(tap_bearing, record + 28),
                "complete observation projection");
    }
    require(observations.rows[0].id == 1 && observations.rows[1].id == 7,
            "observation ids ascend");

    /* A short array publishes the required count and the diagnostic, and writes no record. */
    observation_canary before_short;
    memset(observations.rows, 0x9e, sizeof observations.rows);
    memcpy(&before_short, &observations, sizeof observations);
    required = UINT32_MAX;
    result = miso_engine_effect_descriptor_v1_inspect_observations(
        tap_bearing, tap_bearing_len, MAXIMUM_WIRE_BYTES, observations.rows, 1, &required,
        &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1 && required == 2 &&
                diagnostic.required_bytes == 64,
            "short observation array status");
    require(memcmp(&observations, &before_short, sizeof observations) == 0,
            "short observation array writes nothing");

    required = UINT32_MAX;
    result = miso_engine_effect_descriptor_v1_inspect_observations(
        tap_bearing, tap_bearing_len, MAXIMUM_WIRE_BYTES, NULL, 1, &required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required == 0,
            "nonzero capacity requires a record pointer");

    required = UINT32_C(0x0badf00d);
    result = miso_engine_effect_descriptor_v1_inspect_observations(
        tap_bearing, tap_bearing_len, MAXIMUM_WIRE_BYTES, observations.rows, 2, &required, NULL);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 && required == UINT32_C(0x0badf00d),
            "null diagnostic performs no writes");

    uint8_t *invalid = malloc(tap_bearing_len);
    require(invalid != NULL, "allocate invalid observation wire");
    memcpy(invalid, tap_bearing, tap_bearing_len);
    invalid[12] = 1;
    required = UINT32_MAX;
    result = miso_engine_effect_descriptor_v1_inspect_observations(
        invalid, tap_bearing_len, MAXIMUM_WIRE_BYTES, observations.rows, 2, &required,
        &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1 && required == 0 &&
                diagnostic.byte_offset == 12,
            "invalid wire projects nothing");
    free(invalid);
    free(zero_tap);
    free(tap_bearing);
}

int main(int argc, char **argv) {
    require(argc == 3, "expected zero-tap and tap-bearing fixture paths");
    static const uint8_t expected_identity[32] = {
        0x69, 0xf8, 0x50, 0xcc, 0xd9, 0x4c, 0x0d, 0x6e,
        0xca, 0x0a, 0xf6, 0xcc, 0x6a, 0x2d, 0xa8, 0x04,
        0x61, 0x29, 0x70, 0xd3, 0x11, 0x2b, 0x59, 0xd4,
        0x0b, 0x0d, 0x68, 0x4f, 0xcd, 0x82, 0x8b, 0xa7
    };
    uint8_t wire[WIRE_BYTES];
    miso_engine_effect_descriptor_summary_v1 summary;
    miso_engine_effect_port_record_v1 ports[2];
    miso_engine_effect_quality_record_v1 qualities[4];
    miso_engine_effect_descriptor_diagnostic_v1 diagnostic;
    uint32_t required_parameters = UINT32_MAX;
    uint32_t required_ports = UINT32_MAX;
    uint32_t required_qualities = UINT32_MAX;
    uint32_t required_choices = UINT32_MAX;
    make_wire(wire);

    uint32_t result = inspect(wire, &summary, ports, 2, qualities, 4, &required_parameters,
                              &required_ports, &required_qualities, &required_choices, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1, "valid inspection status");
    require(diagnostic.code == MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1, "valid diagnostic");
    require(diagnostic.byte_offset == UINT32_MAX && diagnostic.record_index == UINT32_MAX &&
                diagnostic.required_bytes == 0,
            "valid diagnostic fields");
    require(summary.abi_version == MISO_ENGINE_EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1,
            "ABI version");
    require(summary.total_bytes == WIRE_BYTES && summary.parameter_count == 0 &&
                summary.port_count == 2 && summary.quality_count == 4 &&
                summary.enum_choice_count == 0 && summary.state_layout_version == 1 &&
                summary.supported_link_mode_bits == MISO_ENGINE_EFFECT_LINK_DUAL_MONO_V1,
            "summary projection");
    require(required_parameters == 0 && required_ports == 2 && required_qualities == 4 &&
                required_choices == 0,
            "required counts");
    require(ports[0].id_offset == 414 && ports[0].id_length == 7 &&
                ports[0].role == MISO_ENGINE_EFFECT_PORT_MAIN_INPUT_V1 && ports[0].required == 1 &&
                ports[0].layout == MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1,
            "input port projection");
    require(ports[1].id_offset == 421 && ports[1].id_length == 8 &&
                ports[1].role == MISO_ENGINE_EFFECT_PORT_MAIN_OUTPUT_V1 &&
                ports[1].required == 1 &&
                ports[1].layout == MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1,
            "output port projection");
    require(qualities[0].quality == MISO_ENGINE_EFFECT_QUALITY_NORMAL_V1 &&
                qualities[0].sample_rate == 44100 && qualities[3].sample_rate == 96000 &&
                qualities[3].tail_kind == MISO_ENGINE_EFFECT_TAIL_FINITE_V1 &&
                qualities[3].scratch_bytes_per_frame == 0,
            "quality projection");
    if (memcmp(summary.identity, expected_identity, sizeof expected_identity) != 0) {
        fprintf(stderr, "C descriptor smoke identity:");
        for (size_t index = 0; index < sizeof summary.identity; ++index) {
            fprintf(stderr, "%02x", summary.identity[index]);
        }
        fputc('\n', stderr);
        return 2;
    }

    miso_engine_effect_descriptor_summary_v1 summary_before;
    miso_engine_effect_port_record_v1 ports_before[2];
    miso_engine_effect_quality_record_v1 qualities_before[4];
    memset(&summary, 0xa5, sizeof summary);
    memset(ports, 0x5a, sizeof ports);
    memset(qualities, 0x3c, sizeof qualities);
    memcpy(&summary_before, &summary, sizeof summary);
    memcpy(ports_before, ports, sizeof ports);
    memcpy(qualities_before, qualities, sizeof qualities);
    result = inspect(wire, &summary, ports, 1, qualities, 4, &required_parameters, &required_ports,
                     &required_qualities, &required_choices, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1,
            "short inspection status");
    require(diagnostic.code == MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1 &&
                diagnostic.byte_offset == UINT32_MAX && diagnostic.record_index == UINT32_MAX &&
                diagnostic.required_bytes == 304,
            "short diagnostic");
    require(required_parameters == 0 && required_ports == 2 && required_qualities == 4 &&
                required_choices == 0,
            "short required counts");
    require(memcmp(&summary, &summary_before, sizeof summary) == 0 &&
                memcmp(ports, ports_before, sizeof ports) == 0 &&
                memcmp(qualities, qualities_before, sizeof qualities) == 0,
            "short all-or-none publication");

    wire[12] = 1;
    required_parameters = required_ports = required_qualities = required_choices = UINT32_MAX;
    result = inspect(wire, &summary, ports, 2, qualities, 4, &required_parameters, &required_ports,
                     &required_qualities, &required_choices, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1 &&
                diagnostic.code == MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1 &&
                diagnostic.byte_offset == 12 && diagnostic.record_index == UINT32_MAX &&
                diagnostic.required_bytes == 0,
            "first-error diagnostic");
    require(required_parameters == 0 && required_ports == 0 && required_qualities == 0 &&
                required_choices == 0,
            "invalid required counts");

    required_parameters = UINT32_C(0x12345678);
    result = miso_engine_effect_descriptor_v1_inspect(
        wire, WIRE_BYTES, MAXIMUM_WIRE_BYTES, &summary, NULL, 0, ports, 2, qualities, 4, NULL, 0,
        &required_parameters, &required_ports, &required_qualities, &required_choices, NULL);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1 &&
                required_parameters == UINT32_C(0x12345678),
            "null diagnostic performs no writes");

    comprehensive_smoke(argv[1]);
    observation_smoke(argv[1], argv[2]);
    puts("effect descriptor C inspection smoke: ok");
    return 0;
}
