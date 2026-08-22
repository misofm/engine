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

int main(void) {
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

    puts("effect descriptor C inspection smoke: ok");
    return 0;
}
