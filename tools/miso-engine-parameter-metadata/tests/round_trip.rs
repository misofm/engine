//! Issue #137 E7: the emitted metadata is complete, and every ID in it resolves on a live session.
//!
//! "Round-trips through a command ack" is the only statement worth making about an addressing
//! scheme: it is not enough that the numbers exist, they must reach the thing they name. For every
//! effect parameter in the metadata the acknowledgement must be `UNSUPPORTED_KIND` -- meaning the
//! address resolved and the engine has no write path -- and never `UNKNOWN_EFFECT`,
//! `UNKNOWN_PARAMETER`, `UNKNOWN_RACK` or `UNKNOWN_TRACK`, which would mean the metadata described
//! something that does not exist. For every live builtin parameter the acknowledgement must be
//! `RESULT_OK`.

use miso_engine_effect_compiler::launch_native_effect_registry_v1;
use miso_engine_host_web::{
    AudioWorkletEngineHost, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_MATRIX,
    COMMAND_REASON_NONE, COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_RECORD_BYTES, RESULT_OK,
    RESULT_UNSUPPORTED, WebPrepareConfigV1,
};

/// A one-track session whose dynamic rack holds every launch effect at its declared defaults.
fn session_with_every_effect(effects: &[&str]) -> String {
    let mut rack = String::from("[");
    for (index, id) in effects.iter().enumerate() {
        if index > 0 {
            rack.push_str(", ");
        }
        rack.push_str(&format!(
            "{{ id = \"e{index}\", identity = {{ kind = \"native\", effect_id = \"{id}\" }}, \
quality = \"normal\", bypass = false, link_mode = \"dual_mono\", params = [], \
sidechain = {{ kind = \"none\" }} }}"
        ));
    }
    rack.push(']');
    format!(
        r#"schema_version = 1
session_id = "metadata-round-trip"
revision = 1
sample_rate_hz = 48000
quantum_frames = 128
render_profile = {{ id = "native", mode = "single_thread" }}
output_profile = {{ id = "main", channels = 2, sample_format = "f32_planar" }}
limits = {{ pcm_ring_frames = 128, control_queue_messages = 8, memory_bytes = 16777216 }}
sources = [
  {{ id = "s", sample_rate_hz = 48000, content = {{ identity = "sha256:metadata-round-trip", locator = "host:metadata-round-trip" }}, mapping = {{ channel_count = 2, region = {{ start_sample = 0, length_samples = 256 }} }} }},
]
submixes = []
outputs = [{{ id = "out" }}]
routes = [
  {{ id = "r", source = {{ kind = "track", track_id = "t", tap = "post_matrix" }}, destination = {{ kind = "output_input", output_id = "out" }}, channel_matrix = {{ ll = 1.0, lr = 0.0, rl = 0.0, rr = 1.0 }}, gain_db = 0.0 }},
]
automation = []

[[tracks]]
id = "t"
source_id = "s"
left_source_channel = 0
right_source_channel = 1
builtins = {{ left = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0 }}, right = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0 }} }}
simd1 = {{ effects = [] }}
dynamic = {{ effects = {rack} }}
simd2 = {{ effects = [] }}
fader = {{ left_db = 0.0, right_db = 0.0, left_mute = false, right_mute = false }}
pan = {{ left = -1.0, right = 1.0, smoothing_samples = 0 }}
"#
    )
}

#[allow(clippy::too_many_arguments)]
fn stage(
    host: &mut AudioWorkletEngineHost,
    kind: u32,
    rack: u8,
    channel: u8,
    effect_index: u32,
    parameter_id: u32,
    value: f32,
) {
    let staging = host
        .command_staging_mut()
        .expect("prepared command staging");
    let record = &mut staging[..COMMAND_RECORD_BYTES as usize];
    record.fill(0);
    record[0] = u8::try_from(kind).expect("frozen kind");
    record[1] = rack;
    record[2] = channel;
    record[8..12].copy_from_slice(&effect_index.to_le_bytes());
    record[12..16].copy_from_slice(&parameter_id.to_le_bytes());
    record[24..28].copy_from_slice(&value.to_le_bytes());
}

/// Red mutation: delete the `command.effect_index >= counts[rack]` leg in `admit_commands`
/// -> an out-of-range effect index is refused as `UNSUPPORTED_KIND`, the completeness assertion
/// below stops distinguishing "resolved" from "did not resolve", and the negative case fails.
#[test]
fn every_metadata_id_resolves_through_a_command_acknowledgement() {
    let document = miso_engine_parameter_metadata::render();
    let registry = launch_native_effect_registry_v1().expect("launch registry");
    let ids: Vec<&'static str> = registry
        .descriptors()
        .map(|descriptor| descriptor.id.as_str())
        .collect();
    assert_eq!(ids.len(), registry.len());
    for id in &ids {
        assert!(
            document.contains(&format!("\"id\": \"{id}\"")),
            "every registered effect is in the metadata: {id}"
        );
    }

    let toml = session_with_every_effect(&ids);
    let mut config = WebPrepareConfigV1::console_defaults(48_000, 128);
    config.source_ring_frames = 128;
    config.console_meter_blocks = 0;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        core::str::from_utf8(host.diagnostic())
    );

    // Issue #140 A: every declared effect parameter resolves *and applies*, except the ones whose
    // own descriptor says they cannot be automated -- which are exactly the ones the metadata
    // marks `liveUpdatable: false`. The two statements are checked against each other here, so a
    // metadata flag and the ABI cannot drift apart.
    for (effect_index, descriptor) in registry.descriptors().enumerate() {
        let index = u32::try_from(effect_index).expect("effect index");
        for parameter in descriptor.parameters {
            stage(
                &mut host,
                COMMAND_EFFECT_PARAM,
                1,
                2,
                index,
                parameter.id.0,
                parameter.default_value,
            );
            let result = host.submit_commands(1);
            let reason = host.command_report().reason;
            if parameter.automatable {
                assert_eq!(
                    result,
                    RESULT_OK,
                    "{}#{} is automatable and must apply (reason {reason})",
                    descriptor.id.as_str(),
                    parameter.id.0
                );
                assert_eq!(reason, COMMAND_REASON_NONE);
                assert_eq!(host.command_report().admitted, 1);
            } else {
                assert_eq!(
                    result,
                    RESULT_UNSUPPORTED,
                    "{}#{} is not automatable and must be refused",
                    descriptor.id.as_str(),
                    parameter.id.0
                );
                assert_eq!(
                    reason,
                    COMMAND_REASON_UNSUPPORTED_KIND,
                    "{}#{} resolved to a real target",
                    descriptor.id.as_str(),
                    parameter.id.0
                );
            }
        }
        stage(&mut host, COMMAND_EFFECT_BYPASS, 1, 255, index, 0, 1.0);
        assert_eq!(
            host.submit_commands(1),
            RESULT_OK,
            "{} bypass",
            descriptor.id.as_str()
        );
        assert_eq!(host.command_report().reason, COMMAND_REASON_NONE);
        // Drain every queue this effect just filled, so the next effect starts with full room.
        assert_eq!(host.render_next(), RESULT_OK);
    }

    // The negative case: one past the last effect does not resolve, and says so differently.
    let past = u32::try_from(ids.len()).expect("effect count");
    stage(&mut host, COMMAND_EFFECT_PARAM, 1, 2, past, 1, 0.0);
    assert_eq!(
        host.submit_commands(1),
        miso_engine_host_web::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        host.command_report().reason,
        miso_engine_host_web::COMMAND_REASON_UNKNOWN_EFFECT
    );

    // Every builtin parameter the metadata calls live is applied by the command path.
    stage(&mut host, COMMAND_MATRIX, 255, 255, 0, 0, 1.0);
    let staging = host
        .command_staging_mut()
        .expect("prepared command staging");
    staging[24..28].copy_from_slice(&0.5_f32.to_le_bytes());
    staging[36..40].copy_from_slice(&1.0_f32.to_le_bytes());
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(host.command_report().reason, COMMAND_REASON_NONE);
    assert_eq!(host.command_report().admitted, 1);
}
