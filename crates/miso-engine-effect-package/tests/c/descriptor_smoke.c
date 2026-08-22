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
        {28, 0, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1, 28, UINT32_MAX},
        {28, 8, MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1, 28, UINT32_MAX},
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

    const uint32_t saved_flags = get_u32(wire, parameter_offset + 32);
    const uint32_t saved_reserved = get_u32(wire, parameter_offset + 72);
    put_u32(wire, parameter_offset + 32, 16);
    put_u32(wire, parameter_offset + 72, 1);
    result = inspect_comprehensive(wire, wire_len, &summary, &parameters, 6, &ports, 3, &qualities,
                                   12, &choices, 3, required, &diagnostic);
    require(result == MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1 &&
                diagnostic.byte_offset == parameter_offset + 32,
            "flags precede later reserved field");
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

int main(int argc, char **argv) {
    require(argc == 2, "expected comprehensive fixture path");
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
    puts("effect descriptor C inspection smoke: ok");
    return 0;
}
