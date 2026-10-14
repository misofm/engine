//! Issue #137 E7: the emitted metadata is complete, and every ID in it resolves on a live session.
//!
//! The question worth asking of an addressing scheme is whether the numbers reach the thing they
//! name. Since slice #1565A the only command records are the two observation kinds, so the
//! observation taps still round-trip through an acknowledgement; every parameter ID goes through
//! the control catalog (`controls`), and its round trip is the comparison of the document's rows
//! with the catalog's rows.

use builtins::{BUILTIN_PARAMETER_DESCRIPTORS, builtin_parameter_unit};
use effect_compiler::launch_native_effect_registry;
use effect_contract::ParameterUnit;
use host_web::{
    AudioWorkletEngineHost, COMMAND_REASON_NONE, COMMAND_RECORD_BYTES, RESULT_OK, WebBootOptions,
};

fn builtin_rows(document: &str) -> Vec<&str> {
    let builtins_start = document
        .find("\"builtins\": {")
        .expect("document names builtins");
    let effects_start = document[builtins_start..]
        .find("\"effects\": [")
        .expect("document closes builtins before effects")
        + builtins_start;
    document[builtins_start..effects_start]
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("{ \"id\":") && line.contains("\"scope\":"))
        .collect()
}

fn quoted_field<'a>(row: &'a str, field: &str) -> &'a str {
    let marker = format!("\"{field}\": \"");
    let start = row
        .find(&marker)
        .unwrap_or_else(|| panic!("builtin row names {field}: {row}"))
        + marker.len();
    let end = row[start..].find('"').expect("builtin field closes") + start;
    &row[start..end]
}

fn raw_field<'a>(row: &'a str, field: &str) -> &'a str {
    let marker = format!("\"{field}\": ");
    let start = row
        .find(&marker)
        .unwrap_or_else(|| panic!("row names {field}: {row}"))
        + marker.len();
    let end = row[start..].find([',', '\n']).expect("raw field closes") + start;
    row[start..end].trim()
}

fn effect_parameter_rows<'a>(document: &'a str, effect_id: &str) -> Vec<&'a str> {
    let effect_start = document
        .find(&format!("\"id\": \"{effect_id}\""))
        .unwrap_or_else(|| panic!("metadata names effect {effect_id}"));
    let parameters_start = document[effect_start..]
        .find("\"parameters\": [")
        .expect("effect carries parameters")
        + effect_start;
    let ports_start = document[parameters_start..]
        .find("\"ports\": [")
        .expect("effect closes parameters before ports")
        + parameters_start;
    document[parameters_start..ports_start]
        .split("        {\n")
        .skip(1)
        .map(|row| {
            let end = row.find("\n        }").expect("parameter row closes");
            &row[..end]
        })
        .collect()
}

fn unit_name(unit: ParameterUnit) -> &'static str {
    match unit {
        ParameterUnit::Db => "db",
        ParameterUnit::Hz => "hz",
        ParameterUnit::Milliseconds => "milliseconds",
        ParameterUnit::Samples => "samples",
        ParameterUnit::Linear => "linear",
        ParameterUnit::Ratio => "ratio",
    }
}

/// The metadata must carry the same explicit unit authority that the builtin lattice uses.
///
/// Red mutations: dropping `unit`/`unitName` makes the row lookup fail, changing delay to linear
/// makes the fixed semantic anchors fail, and duplicating a name violates the emitted table check.
#[test]
fn builtin_metadata_has_authoritative_units_and_unique_keys() {
    let document = parameter_metadata::render();
    let rows = builtin_rows(&document);
    assert_eq!(rows.len(), BUILTIN_PARAMETER_DESCRIPTORS.len());

    let mut names = std::collections::HashSet::new();
    for row in &rows {
        let name = quoted_field(row, "name");
        assert!(!name.is_empty(), "builtin keys are nonempty");
        assert!(names.insert(name), "builtin keys are unique: {name}");
    }

    for descriptor in &BUILTIN_PARAMETER_DESCRIPTORS {
        let row = rows
            .iter()
            .find(|row| row.contains(&format!("\"name\": \"{}\"", descriptor.name)))
            .unwrap_or_else(|| panic!("metadata emits builtin {}", descriptor.name));
        let expected = match descriptor.name {
            "hpf_hz" | "lpf_hz" => ParameterUnit::Hz,
            "trim_db" | "fader_db" => ParameterUnit::Db,
            "delay_samples" => ParameterUnit::Samples,
            "polarity_invert" | "mute" | "matrix_ll" | "matrix_lr" | "matrix_rl" | "matrix_rr"
            | "pan" => ParameterUnit::Linear,
            name => panic!("unexpected builtin parameter {name}"),
        };
        assert_eq!(
            builtin_parameter_unit(descriptor),
            expected,
            "{descriptor:?}"
        );
        assert!(
            row.contains(&format!(
                "\"unit\": {}, \"unitName\": \"{}\"",
                expected as u32,
                unit_name(expected)
            )),
            "{} carries its explicit unit",
            descriptor.name
        );
    }
}

/// Issue #807: the six live cut controls are appended after the historical 24-band catalog.
///
/// The old IDs and defaults are part of the prepared session compatibility surface. The cut rows
/// are deliberately present in the catalog so the SDK can author them, but remain prepared-only:
/// they use the same block-rate, linear-64 live path as the existing numeric rows.
#[test]
fn parametric_eq_catalog_preserves_old_rows_and_appends_prepared_cuts() {
    let document = parameter_metadata::render();
    let rows = effect_parameter_rows(&document, "miso.parametric-eq");
    let old = [
        (1, "band-1-enabled", "0.0"),
        (2, "band-1-kind", "1.0"),
        (3, "band-1-frequency", "80.0"),
        (4, "band-1-gain", "0.0"),
        (5, "band-1-q", "0.70710677"),
        (6, "band-1-shelf-slope", "1.0"),
        (17, "band-2-enabled", "0.0"),
        (18, "band-2-kind", "1.0"),
        (19, "band-2-frequency", "400.0"),
        (20, "band-2-gain", "0.0"),
        (21, "band-2-q", "0.70710677"),
        (22, "band-2-shelf-slope", "1.0"),
        (33, "band-3-enabled", "0.0"),
        (34, "band-3-kind", "1.0"),
        (35, "band-3-frequency", "2000.0"),
        (36, "band-3-gain", "0.0"),
        (37, "band-3-q", "0.70710677"),
        (38, "band-3-shelf-slope", "1.0"),
        (49, "band-4-enabled", "0.0"),
        (50, "band-4-kind", "1.0"),
        (51, "band-4-frequency", "10000.0"),
        (52, "band-4-gain", "0.0"),
        (53, "band-4-q", "0.70710677"),
        (54, "band-4-shelf-slope", "1.0"),
    ];
    assert_eq!(
        rows.len(),
        30,
        "the EQ catalog has 24 old rows plus six cuts"
    );
    for (row, (id, name, default)) in rows.iter().zip(old) {
        assert_eq!(raw_field(row, "id"), id.to_string());
        assert_eq!(quoted_field(row, "name"), name);
        assert_eq!(raw_field(row, "default"), default);
    }

    let cuts = [
        (
            65,
            "hpf-enabled",
            "on/off",
            "linear",
            "boolean",
            "stepped",
            "null",
            "null",
            "0.0",
        ),
        (
            66,
            "hpf-frequency",
            "Hz",
            "hz",
            "continuous",
            "logarithmic",
            "10.0",
            "20000.0",
            "80.0",
        ),
        (
            67,
            "hpf-q",
            "Q",
            "ratio",
            "continuous",
            "logarithmic",
            "0.1",
            "18.0",
            "0.70710677",
        ),
        (
            81,
            "lpf-enabled",
            "on/off",
            "linear",
            "boolean",
            "stepped",
            "null",
            "null",
            "0.0",
        ),
        (
            82,
            "lpf-frequency",
            "Hz",
            "hz",
            "continuous",
            "logarithmic",
            "10.0",
            "20000.0",
            "18000.0",
        ),
        (
            83,
            "lpf-q",
            "Q",
            "ratio",
            "continuous",
            "logarithmic",
            "0.1",
            "18.0",
            "0.70710677",
        ),
    ];
    for (row, (id, name, display_unit, unit, domain, mapping, minimum, maximum, default)) in
        rows[old.len()..].iter().zip(cuts)
    {
        assert_eq!(raw_field(row, "id"), id.to_string());
        assert_eq!(quoted_field(row, "name"), name);
        assert_eq!(quoted_field(row, "displayUnit"), display_unit);
        assert_eq!(quoted_field(row, "unitName"), unit);
        assert_eq!(quoted_field(row, "domainName"), domain);
        assert_eq!(quoted_field(row, "mappingName"), mapping);
        assert_eq!(raw_field(row, "minimum"), minimum);
        assert_eq!(raw_field(row, "maximum"), maximum);
        assert_eq!(raw_field(row, "default"), default);
        assert_eq!(quoted_field(row, "channelPolicyName"), "perLane");
        assert_eq!(quoted_field(row, "automationRateName"), "block");
        assert_eq!(quoted_field(row, "smoothingName"), "linear");
        assert_eq!(raw_field(row, "smoothingSamples"), "64");
        assert_eq!(raw_field(row, "readable"), "true");
        assert_eq!(raw_field(row, "automatable"), "true");
        assert_eq!(raw_field(row, "liveUpdatable"), "true");
    }
}

/// A one-track session whose inserts hold every launch effect at its declared defaults.
fn session_with_every_effect(effects: &[&str]) -> String {
    let mut rack = String::from("[");
    for (index, id) in effects.iter().enumerate() {
        if index > 0 {
            rack.push_str(", ");
        }
        rack.push_str(&format!(
            r#"{{"id":"e{index}","identity":{{"kind":"native","effect_id":"{id}"}},"quality":"normal","bypass":false,"link_mode":"dual_mono","params":[],"sidechain":{{"kind":"none"}}}}"#
        ));
    }
    rack.push(']');
    format!(
        r#"{{"schema_version":1,"session_id":"metadata-round-trip","revision":"1","sample_rate_hz":48000,"quantum_frames":128,
"render_profile":{{"id":"native","mode":"single_thread"}},"output_profile":{{"id":"main","channels":2,"sample_format":"f32_planar"}},
"sources":[{{"id":"s","content":"blake3:0000000000000000000000000000000000000000000000000000000000000000","channels":2,"bit_depth":"32f","frames":"256"}}],
"console":{{"pre_insert":[],"post_insert":[]}},
"tracks":[{{"id":"t","source_id":"s","left_source_channel":0,"right_source_channel":1,
"builtins":{{"left":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}},"right":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}}}},
"console":[],"inserts":{{"effects":{rack}}},
"fader":{{"left_db":0.0,"right_db":0.0,"left_mute":false,"right_mute":false}},"pan":{{"left":-1.0,"right":1.0,"smoothing_samples":0}}}}],
"submixes":[],"vcas":[],"control_smoothing":{{"kind":"default"}},"outputs":[{{"id":"out"}}],"routes":[{{"id":"r","source":{{"kind":"track","track_id":"t","tap":"post_pan"}},"destination":{{"kind":"output_input","output_id":"out"}},"channel_matrix":{{"ll":1.0,"lr":0.0,"rl":0.0,"rr":1.0}},"gain_db":0.0,"mute":false,"follows_mute":false}}],"automation":[]}}"#
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

/// Issue #137 E7, rebuilt for the control plane (#1565A): every ID the metadata declares for an
/// effect parameter is a row of the control catalog the engine serves, with the same liveness.
///
/// The old test sent each ID through a value command and read the acknowledgement. The value
/// commands are gone, and `SET_CONTROL` reaches the engine through `controls::ControlAddress`, whose
/// effect rows are `controls::effect_table_rows`. So the claim "an ID in the document reaches the
/// engine" is the claim that the document's row and the catalog's row agree on the ID and on
/// whether the control is automatable, and that the document's `liveUpdatable` is that flag. The
/// host half (a `SET_CONTROL` on an insert row commits, renders and matches a fresh plan; a
/// rebuild-class row is refused before commit) is held by `hosts/host-web/tests/control_admission.rs`
/// and `crates/controls/tests/effect_rows.rs`.
///
/// Red mutation: renumber a parameter in the generator's `effects` block (or give it a different
/// `automatable` flag) without the catalog -> the catalog lookup or the flag comparison fails; an
/// effect the catalog does not carry fails the lookup.
#[test]
fn every_metadata_effect_parameter_is_a_control_row_with_the_same_liveness() {
    let document = parameter_metadata::render();
    let registry = launch_native_effect_registry().expect("launch registry");
    assert!(!registry.is_empty());
    for descriptor in registry.descriptors() {
        let effect = descriptor.id.as_str();
        assert!(
            document.contains(&format!("\"id\": \"{effect}\"")),
            "every registered effect is in the metadata: {effect}"
        );
        let catalog = controls::effect_table_rows(effect)
            .unwrap_or_else(|| panic!("the control catalog carries {effect}"));
        let rows = effect_parameter_rows(&document, effect);
        assert_eq!(
            rows.len(),
            descriptor.parameters.len(),
            "{effect}: the document lists every declared parameter"
        );
        assert_eq!(catalog.parameters.len(), descriptor.parameters.len());
        for (row, parameter) in rows.iter().zip(descriptor.parameters) {
            assert_eq!(raw_field(row, "id"), parameter.id.0.to_string());
            let control = catalog
                .parameters
                .iter()
                .find(|control| control.control == parameter.id.0)
                .unwrap_or_else(|| panic!("{effect}#{} is a control row", parameter.id.0));
            let automatable = control.descriptor.automatable();
            assert_eq!(
                raw_field(row, "automatable"),
                automatable.to_string(),
                "{effect}#{}: the document and the catalog disagree on automation",
                parameter.id.0
            );
            assert_eq!(
                raw_field(row, "liveUpdatable"),
                automatable.to_string(),
                "{effect}#{}: liveUpdatable is the automation flag",
                parameter.id.0
            );
        }
    }
    // The negative case: an effect the registry does not carry has no rows.
    assert!(controls::effect_table_rows("miso.no-such-effect").is_none());
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
    use host_web::{
        COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_REASON_UNKNOWN_TAP,
    };

    let document = parameter_metadata::render();
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

    let json = session_with_every_effect(&ids);
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        live_control_meter_blocks: 4,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(json.as_bytes(), options).expect("boot");

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
                // A tap's queue slot holds one subscription until a block drains it, and these are
                // separate submissions, so the next one waits for a render (flow control, #1345
                // D2); one submission cannot name the tap twice.
                assert_eq!(host.render_next(), RESULT_OK);
            }
        }
        assert_eq!(host.render_next(), RESULT_OK);
    }

    // The negative case: a tap id no effect declares does not resolve, and says so as a *tap*.
    stage(&mut host, COMMAND_OBSERVE_SUBSCRIBE, 1, 255, 0, 4_242, 0.0);
    assert_eq!(host.submit_commands(1), host_web::RESULT_INVALID_ARGUMENT);
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
    use effect_contract::{PortLayout, PortRole};

    let document = parameter_metadata::render();
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
    let json = format!(
        r#"{{"schema_version":1,"session_id":"port-table-round-trip","revision":"1","sample_rate_hz":48000,"quantum_frames":128,
"render_profile":{{"id":"native","mode":"single_thread"}},"output_profile":{{"id":"main","channels":2,"sample_format":"f32_planar"}},
"sources":[{{"id":"s","content":"blake3:0000000000000000000000000000000000000000000000000000000000000000","channels":2,"bit_depth":"32f","frames":"256"}}],
"console":{{"pre_insert":[],"post_insert":[]}},
"tracks":[
{{"id":"a","source_id":"s","left_source_channel":0,"right_source_channel":1,"builtins":{{"left":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}},"right":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}}}},"console":[],"inserts":{{"effects":[]}},"fader":{{"left_db":0.0,"right_db":0.0,"left_mute":false,"right_mute":false}},"pan":{{"left":-1.0,"right":1.0,"smoothing_samples":0}}}},
{{"id":"b","source_id":"s","left_source_channel":0,"right_source_channel":1,"builtins":{{"left":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}},"right":{{"polarity_invert":false,"trim_db":0.0,"hpf_hz":0.0,"lpf_hz":0.0,"delay_samples":0}}}},"console":[],"inserts":{{"effects":[{{"id":"e0","identity":{{"kind":"native","effect_id":"{effect_id}"}},"quality":"normal","bypass":false,"link_mode":"dual_mono","params":[],"sidechain":{{"kind":"routed","source":{{"kind":"track","track_id":"a","tap":"post_fader"}},"port_id":"{port_id}"}}}}]}},"fader":{{"left_db":0.0,"right_db":0.0,"left_mute":false,"right_mute":false}},"pan":{{"left":-1.0,"right":1.0,"smoothing_samples":0}}}}],
"submixes":[],"vcas":[],"control_smoothing":{{"kind":"default"}},"outputs":[{{"id":"out"}}],"routes":[{{"id":"r","source":{{"kind":"track","track_id":"b","tap":"post_pan"}},"destination":{{"kind":"output_input","output_id":"out"}},"channel_matrix":{{"ll":1.0,"lr":0.0,"rl":0.0,"rr":1.0}},"gain_db":0.0,"mute":false,"follows_mute":false}}],"automation":[]}}"#
    );
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(json.as_bytes(), options).is_ok()
}

// Decision 17, slice S2b: the `controls` object is the `controls` crate's row tables, row for row.

mod controls_object {
    use controls::{
        ControlClass, ControlKind, Family, Lane, LiveRampRow, Motion, OwnerKind, RebuildReason,
        RowSource, SectionKind, TableRow, Taper, effect_table_rows, table_rows,
    };
    use effect_compiler::launch_native_effect_registry;
    use parameter_metadata::{LAUNCH_RATES_HZ, render};
    use serde_json::Value;

    /// The JSON number as the `f32` it was written from.
    fn f32_of(value: &Value) -> f32 {
        value.as_f64().expect("a number") as f32
    }

    fn optional_f32(value: &Value) -> Option<f32> {
        (!value.is_null()).then(|| f32_of(value))
    }

    fn reason_name(reason: RebuildReason) -> (&'static str, Option<u32>) {
        match reason {
            RebuildReason::NewMemory => ("newMemory", None),
            RebuildReason::NewGraph => ("newGraph", None),
            RebuildReason::ProcessingOrder => ("processingOrder", None),
            RebuildReason::Latency => ("latency", None),
            RebuildReason::Glitch => ("glitch", None),
            RebuildReason::Optimisation => ("optimisation", None),
            RebuildReason::Correctness => ("correctness", None),
            RebuildReason::RuledLiveNotBuilt { issue } => ("ruledLiveNotBuilt", Some(issue)),
        }
    }

    /// The class fields of `object` equal `class`, and its `automatable` equals what the
    /// descriptor computes under that class.
    fn assert_class(object: &Value, class: ControlClass, automatable: bool, what: &str) {
        let (name, reason, issue, reopen) = match class {
            ControlClass::Live => ("live", None, None, None),
            ControlClass::Derived => ("derived", None, None, None),
            ControlClass::RebuildOnly { reason, reopen } => {
                let (name, issue) = reason_name(reason);
                ("rebuildOnly", Some(name), issue, Some(reopen))
            }
        };
        assert_eq!(object["class"], name, "{what} class");
        assert_eq!(object["reason"].as_str(), reason, "{what} reason");
        assert_eq!(
            object["reasonIssue"].as_u64(),
            issue.map(u64::from),
            "{what} issue"
        );
        assert_eq!(object["reopen"].as_str(), reopen, "{what} reopen");
        assert_eq!(object["automatable"], automatable, "{what} automatable");
    }

    fn assert_row(json: &Value, row: &TableRow, what: &str) {
        let descriptor = &row.descriptor;
        assert_eq!(json["control"], row.control, "{what} control");
        assert_eq!(json["name"], descriptor.name, "{what} name");
        assert_eq!(
            json["displayName"], descriptor.display_name,
            "{what} displayName"
        );
        assert_eq!(
            json["displayUnit"], descriptor.display_unit,
            "{what} displayUnit"
        );
        assert_eq!(
            json["unit"],
            match descriptor.unit {
                effect_contract::ParameterUnit::Db => "db",
                effect_contract::ParameterUnit::Hz => "hz",
                effect_contract::ParameterUnit::Milliseconds => "milliseconds",
                effect_contract::ParameterUnit::Samples => "samples",
                effect_contract::ParameterUnit::Linear => "linear",
                effect_contract::ParameterUnit::Ratio => "ratio",
            },
            "{what} unit"
        );
        assert_eq!(
            json["kind"],
            match descriptor.kind {
                ControlKind::Continuous => "continuous",
                ControlKind::Toggle => "toggle",
                ControlKind::Enumeration => "enumeration",
            },
            "{what} kind"
        );
        assert_eq!(
            f32_of(&json["minimum"]),
            descriptor.minimum,
            "{what} minimum"
        );
        // A maximum that depends on the rate is an object keyed by each launch rate.
        let maxima = LAUNCH_RATES_HZ.map(|rate| row.at_rate(rate).maximum);
        if maxima.iter().all(|maximum| *maximum == maxima[0]) {
            assert_eq!(f32_of(&json["maximum"]), maxima[0], "{what} maximum");
        } else {
            let by_rate = json["maximum"].as_object().expect("a rate-keyed maximum");
            assert_eq!(by_rate.len(), LAUNCH_RATES_HZ.len(), "{what} maximum rates");
            for (rate, maximum) in LAUNCH_RATES_HZ.iter().zip(maxima) {
                assert_eq!(
                    f32_of(&by_rate[&rate.to_string()]),
                    maximum,
                    "{what} maximum {rate}"
                );
            }
        }
        assert_eq!(
            optional_f32(&json["disabledValue"]),
            descriptor.disabled_value,
            "{what}"
        );
        assert_eq!(
            f32_of(&json["default"]),
            descriptor.default,
            "{what} default"
        );
        assert_eq!(
            json["taper"],
            match descriptor.taper {
                Taper::Linear => "linear",
                Taper::Logarithmic => "logarithmic",
                Taper::Exponential => "exponential",
                Taper::Decibel => "decibel",
                Taper::Stepped => "stepped",
            },
            "{what} taper"
        );
        let lattice = descriptor.lattice;
        assert_eq!(
            json["step"]["size"],
            lattice.step.to_string(),
            "{what} step"
        );
        assert_eq!(
            json["step"]["precision"], lattice.precision,
            "{what} precision"
        );
        for (size, key) in [
            (effect_contract::StepSize::Xs, "xs"),
            (effect_contract::StepSize::Sm, "sm"),
            (effect_contract::StepSize::Md, "md"),
            (effect_contract::StepSize::Lg, "lg"),
            (effect_contract::StepSize::Xl, "xl"),
        ] {
            assert_eq!(
                json["step"]["ladder"][key],
                lattice.ladder.multiple(size),
                "{what} {key}"
            );
        }
        let choices = json["choices"].as_array().expect("choices");
        assert_eq!(choices.len(), descriptor.choices.len(), "{what} choices");
        for (json, choice) in choices.iter().zip(descriptor.choices) {
            assert_eq!(f32_of(&json["value"]), choice.value, "{what} choice value");
            assert_eq!(json["label"], choice.label, "{what} choice label");
        }
        assert_eq!(
            json["ramp"],
            match descriptor.motion {
                Motion::Fixed => "fixed",
                Motion::Ramp(ramp) => match ramp {
                    LiveRampRow::Fader => "fader",
                    LiveRampRow::InputTrim => "inputTrim",
                    LiveRampRow::SendGain => "sendGain",
                    LiveRampRow::VcaOffset => "vcaOffset",
                    LiveRampRow::LinkGlide => "linkGlide",
                    LiveRampRow::Mute => "mute",
                    LiveRampRow::Solo => "solo",
                    LiveRampRow::PolarityInvert => "polarityInvert",
                    LiveRampRow::SendMute => "sendMute",
                    LiveRampRow::SendFollowsMute => "sendFollowsMute",
                    LiveRampRow::VcaMute => "vcaMute",
                    LiveRampRow::BypassCrossfade => "bypassCrossfade",
                    LiveRampRow::Pan => "pan",
                    LiveRampRow::Matrix => "matrix",
                    LiveRampRow::SendMatrix => "sendMatrix",
                },
            },
            "{what} ramp"
        );
        assert_eq!(
            json["scope"],
            match descriptor.scope {
                controls::ControlScope::Model => "model",
                controls::ControlScope::Monitor => "monitor",
            },
            "{what} scope"
        );
        assert_eq!(
            json["family"],
            match descriptor.family {
                Family::StripLevel => "stripLevel",
                Family::StripMatrix => "stripMatrix",
                Family::StripInput => "stripInput",
                Family::Route => "route",
                Family::Effect => "effect",
                Family::Setting => "setting",
            },
            "{what} family"
        );
        assert_eq!(
            json["lanes"],
            match descriptor.lanes {
                controls::Lanes::PerLane => "perLane",
                controls::Lanes::Shared => "shared",
            },
            "{what} lanes"
        );
        if let Some(output) = row.into_output {
            // A route row states its class under each destination, and none at the top.
            assert!(json.get("class").is_none(), "{what} has no top-level class");
            let into_submix = &json["intoSubmix"];
            assert_class(
                into_submix,
                descriptor.class,
                descriptor.automatable(),
                what,
            );
            assert_eq!(into_submix["active"], true, "{what} active into a submix");
            let into_output = &json["intoOutput"];
            let under_output = controls::ControlDescriptor {
                class: output.class,
                ..*descriptor
            };
            assert_class(into_output, output.class, under_output.automatable(), what);
            assert_eq!(
                into_output["active"], output.active,
                "{what} active into the output"
            );
        } else {
            assert_class(json, descriptor.class, descriptor.automatable(), what);
            assert_eq!(
                json.get("onSubmix").map(|value| value == false),
                (!row.on_submix).then_some(true),
                "{what} onSubmix"
            );
        }
    }

    fn assert_rows(json: &Value, rows: &[TableRow], what: &str) {
        let json = json
            .as_array()
            .unwrap_or_else(|| panic!("{what} is a list"));
        assert_eq!(json.len(), rows.len(), "{what} row count");
        for (json, row) in json.iter().zip(rows) {
            assert_row(json, row, &format!("{what}/{}", row.descriptor.name));
        }
    }

    #[test]
    fn the_controls_object_is_the_row_tables_row_for_row() {
        let document: Value = serde_json::from_str(&render()).expect("the document is JSON");
        let controls = &document["controls"];
        for (key, source) in [
            ("stripOwn", RowSource::StripOwn),
            ("builtins", RowSource::Builtins),
            ("routeOwn", RowSource::RouteOwn),
            ("vcaOwn", RowSource::VcaOwn),
            ("sessionOwn", RowSource::SessionOwn),
        ] {
            assert_rows(&controls[key], &table_rows(source), key);
        }
        // A rate-keyed maximum is emitted for the two filters and for no other row.
        let rate_keyed: Vec<_> = controls["builtins"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["maximum"].is_object())
            .map(|row| row["name"].as_str().unwrap())
            .collect();
        assert_eq!(rate_keyed, ["hpf_hz", "lpf_hz"]);

        let registry = launch_native_effect_registry().expect("launch registry");
        let effects = controls["effects"].as_object().expect("effects by ID");
        assert_eq!(effects.len(), registry.len(), "one entry per launch effect");
        for descriptor in registry.descriptors() {
            let id = descriptor.id.as_str();
            let rows = effect_table_rows(id).expect("a launch effect has rows");
            let json = &effects[id];
            assert_row(&json["bypass"], &rows.bypass, &format!("{id}/bypass"));
            assert_rows(&json["parameters"], &rows.parameters, id);
            // Every parameter the effect declares is a row, by its declared machine name.
            let names: Vec<_> = json["parameters"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["name"].as_str().unwrap())
                .collect();
            let declared: Vec<_> = descriptor.parameters.iter().map(|p| p.name).collect();
            assert_eq!(names, declared, "{id} parameter names");
            match &rows.link_mode {
                Some(row) => assert_row(&json["linkMode"], row, &format!("{id}/link_mode")),
                None => assert!(json["linkMode"].is_null(), "{id} has no link-mode row"),
            }
        }
    }
    /// A frontend keys a control by its machine name and the wire by its ID, so both must be unique
    /// in the set an address picks from: each of the five tables, and each effect's bypass,
    /// parameters and link mode together. Red if two rows of a set share a name (the generated
    /// SDK type would merge them) or an ID (one address would name two rows).
    #[test]
    fn names_and_ids_are_unique_per_table_and_per_effect() {
        let document: Value = serde_json::from_str(&render()).expect("the document is JSON");
        let controls = &document["controls"];
        let check = |what: &str, rows: Vec<&Value>| {
            let mut names = std::collections::BTreeSet::new();
            let mut ids = std::collections::BTreeSet::new();
            for row in rows {
                let name = row["name"].as_str().expect("a name");
                assert!(names.insert(name), "{what}: the name `{name}` is repeated");
                let id = row["control"].as_u64().expect("a control ID");
                assert!(ids.insert(id), "{what}: the control ID {id} is repeated");
            }
        };
        for key in ["stripOwn", "builtins", "routeOwn", "vcaOwn", "sessionOwn"] {
            check(key, controls[key].as_array().unwrap().iter().collect());
        }
        for (id, effect) in controls["effects"].as_object().unwrap() {
            let mut rows = vec![&effect["bypass"]];
            rows.extend(effect["parameters"].as_array().unwrap());
            if !effect["linkMode"].is_null() {
                rows.push(&effect["linkMode"]);
            }
            check(id, rows);
        }
    }

    /// Red if the address vocabulary the SDK builds its types from differs from the core's owner
    /// kinds, section kinds and lanes, or loses a code.
    #[test]
    fn the_address_vocabulary_is_the_cores() {
        let document: Value = serde_json::from_str(&render()).expect("the document is JSON");
        let address = &document["controls"]["address"];
        let listed = |key: &str| -> Vec<(String, u64)> {
            address[key]
                .as_array()
                .unwrap_or_else(|| panic!("{key} is a list"))
                .iter()
                .map(|entry| {
                    (
                        entry["name"].as_str().unwrap().to_owned(),
                        entry["code"].as_u64().unwrap(),
                    )
                })
                .collect()
        };
        let expected = |entries: Vec<(&str, u8)>| -> Vec<(String, u64)> {
            entries
                .into_iter()
                .map(|(name, code)| (name.to_owned(), u64::from(code)))
                .collect()
        };
        assert_eq!(
            listed("ownerKinds"),
            expected(OwnerKind::ALL.map(|k| (k.name(), k.code())).to_vec())
        );
        assert_eq!(
            listed("sectionKinds"),
            expected(SectionKind::ALL.map(|k| (k.name(), k.code())).to_vec())
        );
        assert_eq!(
            listed("lanes"),
            expected(Lane::ALL.iter().map(|(l, n)| (*n, l.wire())).collect())
        );
        // The pinned codes: owner kinds session 1, strip 2, route 3, VCA 4; sections inserts 2,
        // builtins 4, console 5, own 6; lanes left 1, right 2, both 3.
        assert_eq!(
            listed("ownerKinds")
                .iter()
                .map(|(_, c)| *c)
                .collect::<Vec<_>>(),
            [1, 2, 3, 4]
        );
        assert_eq!(
            listed("sectionKinds")
                .iter()
                .map(|(_, c)| *c)
                .collect::<Vec<_>>(),
            [2, 4, 5, 6]
        );
        assert_eq!(
            listed("lanes").iter().map(|(_, c)| *c).collect::<Vec<_>>(),
            [1, 2, 3]
        );
    }
}
