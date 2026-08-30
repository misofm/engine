//! Issue #137 E7: the emitted metadata is complete, and every ID in it resolves on a live session.
//!
//! "Round-trips through a command ack" is the only statement worth making about an addressing
//! scheme: it is not enough that the numbers exist, they must reach the thing they name. For every
//! effect parameter in the metadata the acknowledgement must be `UNSUPPORTED_KIND` -- meaning the
//! address resolved and the engine has no write path -- and never `UNKNOWN_EFFECT`,
//! `UNKNOWN_PARAMETER`, `UNKNOWN_RACK` or `UNKNOWN_TRACK`, which would mean the metadata described
//! something that does not exist. For every live builtin parameter the acknowledgement must be
//! `RESULT_OK`.

use miso_engine_effect_compiler::launch_native_effect_registry;
use miso_engine_host_web::{
    AudioWorkletEngineHost, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_MATRIX,
    COMMAND_REASON_NONE, COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_RECORD_BYTES, RESULT_OK,
    RESULT_UNSUPPORTED, WebBootOptions,
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
sources = [
  {{ id = "s", content = "sha256:0000000000000000000000000000000000000000000000000000000000000000", channels = 2, bit_depth = "32f", frames = 256 }},
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
builtins = {{ left = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }}, right = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }} }}
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
    let registry = launch_native_effect_registry().expect("launch registry");
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
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        console_command_queue_records: 64,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(toml.as_bytes(), options).expect("boot");

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

/// Issue #143 E9: every declared observation tap in the document resolves on a live session.
///
/// The metadata's per-effect `observations` array is a *promise*: it says an app may address that
/// tap by that number. This walks every one of them through the real command path and requires
/// each to resolve -- `RESULT_OK` for a `subscribable` (resident) tap and `UNSUPPORTED_KIND` for a
/// computed one -- and **never** an `Unknown*` reason, which would mean the document described
/// something that does not exist.
///
/// The completeness half is structural: the taps come from
/// `NativeEffectRegistry::descriptors`, which is the same list the generator walks, so an effect
/// whose menu is missing from the document cannot exist.
///
/// Red mutation: hand-edit a tap id in the document (or, equivalently, offset the id in the
/// lowering) -> the tap stops resolving and the `UNKNOWN_TAP` assertion fires.
#[test]
fn every_metadata_observation_tap_resolves_through_a_command_acknowledgement() {
    use miso_engine_host_web::{
        COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_REASON_UNKNOWN_TAP,
    };

    let document = miso_engine_parameter_metadata::render();
    let registry = launch_native_effect_registry().expect("launch registry");
    let ids: Vec<&'static str> = registry
        .descriptors()
        .map(|descriptor| descriptor.id.as_str())
        .collect();

    // Every declared tap is in the document, and every effect has the key even when empty.
    let mut declared = 0_usize;
    for descriptor in registry.descriptors() {
        assert!(
            document.contains("\"observations\": ["),
            "the observations key is never absent"
        );
        for tap in descriptor.observations {
            assert!(
                document.contains(&format!(
                    "\"id\": {}, \"name\": \"{}\"",
                    tap.id.0, tap.display_name
                )),
                "{} tap {} is in the metadata",
                descriptor.id.as_str(),
                tap.id.0
            );
            declared += 1;
        }
    }
    assert_eq!(
        declared, 4,
        "the four dynamics effects declare one tap each"
    );

    let toml = session_with_every_effect(&ids);
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        console_command_queue_records: 64,
        console_meter_blocks: 4,
        console_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(toml.as_bytes(), options).expect("boot");

    for (effect_index, descriptor) in registry.descriptors().enumerate() {
        let index = u32::try_from(effect_index).expect("effect index");
        for tap in descriptor.observations {
            for kind in [COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE] {
                stage(&mut host, kind, 1, 255, index, tap.id.0, 0.0);
                let result = host.submit_commands(1);
                let reason = host.command_report().reason;
                // Every launch tap is resident, so every one of them binds. The rule the test
                // states is the general one: resolved, never `Unknown*`.
                assert_ne!(
                    reason,
                    COMMAND_REASON_UNKNOWN_TAP,
                    "{} tap {} did not resolve",
                    descriptor.id.as_str(),
                    tap.id.0
                );
                assert_eq!(
                    reason,
                    COMMAND_REASON_NONE,
                    "{} tap {} is subscribable and must bind",
                    descriptor.id.as_str(),
                    tap.id.0
                );
                assert_eq!(result, RESULT_OK);
            }
        }
        assert_eq!(host.render_next(), RESULT_OK);
    }

    // The negative case: a tap id no effect declares does not resolve, and says so as a *tap*.
    stage(&mut host, COMMAND_OBSERVE_SUBSCRIBE, 1, 255, 0, 4_242, 0.0);
    assert_eq!(
        host.submit_commands(1),
        miso_engine_host_web::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TAP);
}

/// Issue #278: every port the document publishes is a port the engine actually accepts.
///
/// The port table is a *promise* in exactly the sense the observation menu is: it says a session
/// may name that `port_id` in a routed sidechain. Publishing it is what lets an authoring layer
/// refuse a misspelling before boot, so the table being wrong would be worse than its being
/// absent -- a builder would confidently refuse a legal port, or confidently accept an illegal
/// one. This walks the published sidechain rows through a real preparation and requires each to
/// prepare, and requires a `port_id` the table does NOT publish to be refused.
///
/// The completeness half is structural: the rows come from `NativeEffectRegistry::descriptors`,
/// the same list the generator walks, so an effect whose ports are missing cannot exist.
///
/// Red mutation: rename a port in the generator's output (or in the descriptor) without renaming
/// the other -> the published id stops preparing and the first assertion fires. Delete the
/// `effect.sidechain.unknown_port` leg in `prepare_native_session_effects` -> the negative case
/// prepares and the last assertion fires.
#[test]
fn every_published_sidechain_port_prepares_and_an_unpublished_one_does_not() {
    use miso_engine_effect_contract::{PortLayout, PortRole};

    let document = miso_engine_parameter_metadata::render();
    let registry = launch_native_effect_registry().expect("launch registry");

    // Every declared port is in the document, and every effect has the key.
    let mut rows = 0_usize;
    let mut sidechains: Vec<(&'static str, &'static str)> = Vec::new();
    for descriptor in registry.descriptors() {
        assert!(
            document.contains("\"ports\": ["),
            "the ports key is never absent"
        );
        for port in descriptor.ports {
            let role = match port.role {
                PortRole::MainInput => "mainInput",
                PortRole::MainOutput => "mainOutput",
                PortRole::SidechainInput => "sidechainInput",
            };
            assert_eq!(port.layout, PortLayout::DualMonoPlanar);
            assert!(
                document.contains(&format!(
                    "{{ \"id\": \"{}\", \"role\": {}, \"roleName\": \"{role}\", \
\"required\": {}, \"layout\": 1, \"layoutName\": \"dualMonoPlanar\" }}",
                    port.id.as_str(),
                    port.role as u32,
                    port.required
                )),
                "{} port {} is in the metadata",
                descriptor.id.as_str(),
                port.id.as_str()
            );
            if port.role == PortRole::SidechainInput {
                assert!(!port.required, "a launch sidechain input is never required");
                sidechains.push((descriptor.id.as_str(), port.id.as_str()));
            }
            rows += 1;
        }
    }
    assert_eq!(rows, 18, "eight effects, two of which declare a sidechain");
    assert_eq!(
        sidechains,
        vec![
            ("miso.compressor", "sidechain-in"),
            ("miso.gate-expander", "sidechain-in"),
        ]
    );

    for (effect_id, port_id) in sidechains {
        assert!(
            prepares(effect_id, port_id),
            "{effect_id} publishes '{port_id}' and it must prepare"
        );
        // The negative case, on the same effect: a port the table does not publish is refused.
        assert!(
            !prepares(effect_id, "not-a-port"),
            "{effect_id} accepted a port it never published"
        );
    }
}

/// Boot a two-track session whose second track's compressor-class effect routes a sidechain from
/// the first, and report whether preparation accepted it.
#[cfg(test)]
fn prepares(effect_id: &str, port_id: &str) -> bool {
    let toml = format!(
        r#"schema_version = 1
session_id = "port-table-round-trip"
revision = 1
sample_rate_hz = 48000
quantum_frames = 128
render_profile = {{ id = "native", mode = "single_thread" }}
output_profile = {{ id = "main", channels = 2, sample_format = "f32_planar" }}
sources = [
  {{ id = "s", content = "sha256:0000000000000000000000000000000000000000000000000000000000000000", channels = 2, bit_depth = "32f", frames = 256 }},
]
submixes = []
outputs = [{{ id = "out" }}]
routes = [
  {{ id = "r", source = {{ kind = "track", track_id = "b", tap = "post_matrix" }}, destination = {{ kind = "output_input", output_id = "out" }}, channel_matrix = {{ ll = 1.0, lr = 0.0, rl = 0.0, rr = 1.0 }}, gain_db = 0.0 }},
]
automation = []

[[tracks]]
id = "a"
source_id = "s"
left_source_channel = 0
right_source_channel = 1
builtins = {{ left = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }}, right = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }} }}
simd1 = {{ effects = [] }}
dynamic = {{ effects = [] }}
simd2 = {{ effects = [] }}
fader = {{ left_db = 0.0, right_db = 0.0, left_mute = false, right_mute = false }}
pan = {{ left = -1.0, right = 1.0, smoothing_samples = 0 }}

[[tracks]]
id = "b"
source_id = "s"
left_source_channel = 0
right_source_channel = 1
builtins = {{ left = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }}, right = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }} }}
simd1 = {{ effects = [] }}
dynamic = {{ effects = [{{ id = "e0", identity = {{ kind = "native", effect_id = "{effect_id}" }}, quality = "normal", bypass = false, link_mode = "dual_mono", params = [], sidechain = {{ kind = "routed", source = {{ kind = "track", track_id = "a", tap = "post_fader" }}, port_id = "{port_id}" }} }}] }}
simd2 = {{ effects = [] }}
fader = {{ left_db = 0.0, right_db = 0.0, left_mute = false, right_mute = false }}
pan = {{ left = -1.0, right = 1.0, smoothing_samples = 0 }}
"#
    );
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        console_command_queue_records: 64,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(toml.as_bytes(), options).is_ok()
}
