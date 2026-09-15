//! The emitted ABI layout is the engine's own bytes, not a table somebody typed (issue #243).
//!
//! `render_abi_layout` takes its struct offsets through `offset_of!`, so a field that moves moves
//! the document with it and no assertion here can catch a *rename*. What these tests catch is the
//! other half, which `offset_of!` cannot reach:
//!
//! * the **command record**, whose offsets are a decode rule rather than a `#[repr(C)]` type.
//!   `CommandRecord::decode` reads byte positions out of the staging buffer directly, so the rows
//!   in the document are a transcription of a function body. The test below encodes a record *at
//!   the emitted offsets* and requires the live engine to decode exactly the values written — the
//!   only way to prove a hand-transcribed offset table against a hand-written decoder.
//! * the **source-ring rule**, which the document states as its two inputs rather than as an
//!   answer. The test re-derives the ring from those inputs and requires it to equal
//!   `default_source_ring_frames` at every launch rate and a spread of quanta, so a consumer that
//!   applies the published rule lands where the engine lands.
//! * the **alias table**, which must be exactly the three alias constants and must agree with the
//!   base names on the same values.
//! * the **document's own structure**, so a schema key cannot be dropped silently.

use core::mem::{offset_of, size_of};

use effect_compiler::launch_native_effect_registry;
use host_core::LAUNCH_SAMPLE_RATES;
use host_web::{
    AudioWorkletEngineHost, COMMAND_EFFECT_PARAM, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_TRACK,
    COMMAND_RECORD_BYTES, RESULT_OK, RESULT_UNSUPPORTED, WebBootOptions, WebObservationAdmission,
    WebObservationCaptureIdentity, WebObservationDemand, WebObservationIngressLimits,
    WebObservationPreparationRecord, WebObservationReceipt, WebObservationStatus,
    WebObservationWorkLimits, default_source_ring_frames,
};
use parameter_metadata::abi_layout::{
    ERROR_PHASES, SCHEMA, SOURCE_RING_RESERVE_QUANTA, STAGING_SEQUENCE, render,
};

/// Minimal parsing: the document is generated, so a test that pulled in a JSON crate would be
/// testing the crate. These helpers read the exact shapes this generator emits and panic loudly on
/// anything else, which is itself an assertion that the shape did not change.
fn structure_body<'a>(document: &'a str, structure: &str) -> &'a str {
    let marker = format!("\"{structure}\": {{\n");
    let marker_start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("document names structure {structure}"));
    let indent_start = document[..marker_start]
        .rfind('\n')
        .map_or(0, |newline| newline + 1);
    let indent = &document[indent_start..marker_start];
    let structure_start = marker_start + marker.len();
    let close = format!("\n{indent}}}");
    let structure_end = document[structure_start..]
        .find(&close)
        .unwrap_or_else(|| panic!("structure {structure} is closed"));
    &document[structure_start..structure_start + structure_end]
}

fn field_entry(document: &str, structure: &str, field: &str) -> (usize, String) {
    let body = structure_body(document, structure);
    let row = format!("{{ \"name\": \"{field}\", \"offset\": ");
    let row_start = body
        .find(&row)
        .unwrap_or_else(|| panic!("structure {structure} names field {field}"))
        + row.len();
    let type_marker = ", \"type\": \"";
    let offset_end = body[row_start..]
        .find(type_marker)
        .expect("offset is followed by a type")
        + row_start;
    let type_start = offset_end + type_marker.len();
    let type_end = body[type_start..].find('"').expect("type is closed") + type_start;
    (
        body[row_start..offset_end]
            .trim()
            .parse()
            .expect("offset is an integer"),
        body[type_start..type_end].to_owned(),
    )
}

fn structure_bytes(document: &str, structure: &str) -> u64 {
    let body = structure_body(document, structure);
    let key = "\"bytes\": ";
    let start = body
        .find(key)
        .unwrap_or_else(|| panic!("structure {structure} names bytes"))
        + key.len();
    let end = body[start..]
        .find(',')
        .or_else(|| body[start..].find('\n'))
        .expect("structure bytes is terminated")
        + start;
    body[start..end]
        .trim()
        .parse()
        .expect("structure bytes is an integer")
}

fn field_offset(document: &str, structure: &str, field: &str) -> usize {
    field_entry(document, structure, field).0
}

fn named_constants(document: &str, group: &str) -> Vec<(u32, String)> {
    let key = format!("\"{group}\": [");
    let start = document
        .find(&key)
        .unwrap_or_else(|| panic!("document names constant group {group}"))
        + key.len();
    let end = document[start..].find(']').expect("group is closed") + start;
    let mut rows = Vec::new();
    for entry in document[start..end].split("}, ") {
        let value_key = "\"value\": ";
        let name_key = "\"name\": \"";
        let Some(value_start) = entry.find(value_key) else {
            continue;
        };
        let value_start = value_start + value_key.len();
        let value_end = entry[value_start..].find(',').expect("value then name") + value_start;
        let name_start = entry.find(name_key).expect("row names a name") + name_key.len();
        let name_end = entry[name_start..].find('"').expect("name is closed") + name_start;
        rows.push((
            entry[value_start..value_end]
                .trim()
                .parse()
                .expect("value is an integer"),
            entry[name_start..name_end].to_owned(),
        ));
    }
    rows
}

/// Read a scalar, searching from `after` so a key that repeats across blocks (`bytes` appears in
/// every structure) is read from the block the caller means.
fn scalar_after(document: &str, after: &str, name: &str) -> u64 {
    let base = document
        .find(after)
        .unwrap_or_else(|| panic!("document names {after}"));
    let key = format!("\"{name}\": ");
    let start = document[base..]
        .find(&key)
        .unwrap_or_else(|| panic!("document names scalar {name} after {after}"))
        + base
        + key.len();
    let end = start
        + document[start..]
            .find([',', '\n', '}'])
            .expect("scalar is terminated");
    document[start..end]
        .trim()
        .parse()
        .expect("scalar is an integer")
}

/// A one-track session whose dynamic rack holds one launch effect, so an `effectParam` record has
/// a real address to resolve against.
fn one_effect_session(effect_id: &str) -> String {
    format!(
        r#"{{
  "schema_version": 1,
  "session_id": "abi-layout",
  "revision": "1",
  "sample_rate_hz": 48000,
  "quantum_frames": 128,
  "render_profile": {{ "id": "native", "mode": "single_thread" }},
  "output_profile": {{ "id": "main", "channels": 2, "sample_format": "f32_planar" }},
  "sources": [{{ "id": "s", "content": "blake3:{}", "channels": 2, "bit_depth": 24, "frames": "48000" }}],
  "tracks": [{{
    "id": "t",
    "source_id": "s",
    "left_source_channel": 0,
    "right_source_channel": 1,
    "builtins": {{
      "left": {{ "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 0.0, "lpf_hz": 0.0, "delay_samples": 0 }},
      "right": {{ "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 0.0, "lpf_hz": 0.0, "delay_samples": 0 }}
    }},
    "simd1": {{ "effects": [] }},
    "dynamic": {{ "effects": [{{
      "id": "e",
      "identity": {{ "kind": "native", "effect_id": "{effect_id}" }},
      "quality": "normal",
      "bypass": false,
      "link_mode": "dual_mono",
      "params": [],
      "sidechain": {{ "kind": "none" }}
    }}] }},
    "simd2": {{ "effects": [] }},
    "fader": {{ "left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false }},
    "pan": {{ "left": -1.0, "right": 1.0, "smoothing_samples": 0 }}
  }}],
  "submixes": [],
  "outputs": [{{ "id": "out" }}],
  "routes": [{{
    "id": "r",
    "source": {{ "kind": "track", "track_id": "t", "tap": "post_matrix" }},
    "destination": {{ "kind": "output_input", "output_id": "out" }},
    "channel_matrix": {{ "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 }},
    "gain_db": 0.0
  }}],
  "automation": []
}}"#,
        "0".repeat(64)
    )
}

/// The emitted command-record offsets decode, on the live engine, to exactly the values written.
///
/// Red mutation: move any row in `command_record_fields` — say `parameterId` to 16 — and the
/// engine reads the smoothing word as the parameter ID; the acknowledgement stops being
/// `UNSUPPORTED_KIND` (address resolved, no write path) and becomes `UNKNOWN_PARAMETER`.
#[test]
fn the_emitted_command_record_offsets_are_the_engine_s_own_decode_rule() {
    let document = render();
    let bytes = scalar_after(&document, "\"commandRecord\": {", "bytes");
    assert_eq!(
        bytes,
        u64::from(COMMAND_RECORD_BYTES),
        "the command record's published size is the engine's"
    );

    let registry = launch_native_effect_registry().expect("launch effect registry");
    let first = registry
        .descriptors()
        .next()
        .expect("the launch registry is non-empty")
        .id
        .as_str();
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        source_ring_frames: 128,
        console_command_queue_records: 64,
        ..WebBootOptions::explicit_defaults()
    };
    let mut host = AudioWorkletEngineHost::boot(one_effect_session(first).as_bytes(), options)
        .expect("the fixture session boots");

    let kind = field_offset(&document, "commandRecord", "kind");
    let rack = field_offset(&document, "commandRecord", "rack");
    let channel = field_offset(&document, "commandRecord", "channel");
    let track_index = field_offset(&document, "commandRecord", "trackIndex");
    let effect_index = field_offset(&document, "commandRecord", "effectIndex");
    let parameter_id = field_offset(&document, "commandRecord", "parameterId");
    let smoothing = field_offset(&document, "commandRecord", "smoothingSamples");
    let values = field_offset(&document, "commandRecord", "values");

    let staging = host.command_staging_mut().expect("console staging exists");
    staging[..COMMAND_RECORD_BYTES as usize].fill(0);
    staging[kind] = u8::try_from(COMMAND_EFFECT_PARAM).expect("kind is a byte");
    // Rack 1 is `dynamic`, which is where the fixture put its one effect.
    staging[rack] = 1;
    staging[channel] = 2;
    staging[track_index..track_index + 4].copy_from_slice(&0_u32.to_le_bytes());
    staging[effect_index..effect_index + 4].copy_from_slice(&0_u32.to_le_bytes());
    // Parameter ID 1 is the first declared parameter of every launch descriptor.
    staging[parameter_id..parameter_id + 4].copy_from_slice(&1_u32.to_le_bytes());
    staging[smoothing..smoothing + 4].copy_from_slice(&0_u32.to_le_bytes());
    staging[values..values + 4].copy_from_slice(&0.0_f32.to_le_bytes());

    let result = host.submit_commands(1);
    let reason = host.command_report().reason;
    // The address resolving is the whole assertion. `RESULT_OK` means the engine also had a write
    // path; `RESULT_UNSUPPORTED`/`unsupportedKind` means it resolved and deliberately has none.
    // Either proves the offsets; an `unknown*` reason proves they are wrong.
    assert!(
        result == RESULT_OK || result == RESULT_UNSUPPORTED,
        "a record written at the published offsets was answered {result} (reason {reason})"
    );
    for (unknown, label) in [
        (COMMAND_REASON_UNKNOWN_TRACK, "track"),
        (COMMAND_REASON_UNKNOWN_RACK, "rack"),
        (COMMAND_REASON_UNKNOWN_EFFECT, "effect"),
        (COMMAND_REASON_UNKNOWN_PARAMETER, "parameter"),
    ] {
        assert_ne!(
            reason, unknown,
            "the published offsets addressed no {label}: the record's {label} word is misplaced"
        );
    }
}

/// The published source-ring rule reproduces the engine's derivation at every launch shape.
///
/// Red mutation: publish `reserveQuanta: 1` and every row below misses by one quantum.
#[test]
fn the_published_source_ring_rule_reproduces_the_engine_derivation() {
    let document = render();
    let key = "\"sourceRing\": { \"stallToleranceMs\": ";
    let start = document.find(key).expect("the document names sourceRing") + key.len();
    let end = document[start..].find(',').expect("two members") + start;
    let tolerance_ms: u64 = document[start..end].trim().parse().expect("an integer");
    assert_eq!(
        u32::try_from(tolerance_ms).expect("a u32"),
        host_web::SOURCE_STALL_TOLERANCE_MS,
        "the published tolerance is the engine's constant"
    );

    for rate in LAUNCH_SAMPLE_RATES {
        let rate = rate.0;
        for quantum in [1_u32, 2, 32, 64, 127, 128, 129, 256, 480, 1024] {
            let stall_frames = u64::from(rate) * tolerance_ms / 1_000;
            let quanta =
                stall_frames.div_ceil(u64::from(quantum)) + u64::from(SOURCE_RING_RESERVE_QUANTA);
            let published = u32::try_from(quanta * u64::from(quantum)).expect("fits a u32");
            assert_eq!(
                published,
                default_source_ring_frames(rate, quantum),
                "the published rule and the engine disagree at {rate} Hz / {quantum} frames"
            );
        }
    }

    // The eval-2 shape, re-derived here so the number in the brief has an independent witness.
    assert_eq!(default_source_ring_frames(96_000, 127), 78 * 127);
    assert_eq!(78 * 127, 9_906);
}

/// The boot alias table is exactly the three alias constants, and every alias value is also a base
/// name in `resultCodes`.
///
/// Red mutation: add `refusedBudget` to the alias table — it is a primary code (5), not an alias,
/// and the length assertion goes red.
#[test]
fn the_boot_alias_table_is_exactly_the_three_alias_constants() {
    let document = render();
    let aliases = named_constants(&document, "bootResultAliases");
    assert_eq!(
        aliases,
        vec![
            (
                host_web::RESULT_REFUSED_DOCUMENT,
                "refusedDocument".to_owned()
            ),
            (
                host_web::RESULT_REFUSED_OPTIONS,
                "refusedOptions".to_owned()
            ),
            (
                host_web::RESULT_REFUSED_LIFECYCLE,
                "refusedLifecycle".to_owned()
            ),
        ],
        "the alias table is the three boot alias constants, in constant order"
    );

    let results = named_constants(&document, "resultCodes");
    for (value, alias) in &aliases {
        let base = results
            .iter()
            .find(|(candidate, _)| candidate == value)
            .unwrap_or_else(|| panic!("alias value {value} has a base name"));
        assert_ne!(
            &base.1, alias,
            "an alias that equals its base name is not an alias"
        );
    }
    assert_eq!(
        results
            .iter()
            .map(|(value, _)| *value)
            .collect::<Vec<_>>()
            .len(),
        11,
        "the base ladder is the eleven frozen result codes"
    );
}

/// Every published struct offset is the engine's, and the document carries its whole schema.
#[test]
fn the_document_carries_its_whole_schema_and_the_engine_s_offsets() {
    let document = render();
    assert!(document.contains(&format!("\"schema\": \"{SCHEMA}\"")));
    for export in STAGING_SEQUENCE {
        assert!(
            document.contains(&format!("\"{export}\"")),
            "the staging sequence names {export}"
        );
    }
    for phase in ERROR_PHASES {
        assert!(
            document.contains(&format!("\"{phase}\"")),
            "the phase vocabulary names {phase}"
        );
    }
    assert_eq!(
        field_offset(&document, "bootOptions", "requireSampleRateHz"),
        8
    );
    assert_eq!(
        field_offset(&document, "bootOptions", "requireQuantumFrames"),
        12
    );
    assert_eq!(
        field_offset(&document, "bootOptions", "sourceRingFrames"),
        16
    );
    assert_eq!(
        field_offset(&document, "bootOptions", "maximumMemoryBytes"),
        24
    );
    assert_eq!(
        field_offset(&document, "bootOptions", "consoleCommandQueueRecords"),
        32
    );
    assert_eq!(
        field_offset(&document, "bootOptions", "consoleMasterTrackPlusOne"),
        56
    );
    assert_eq!(field_offset(&document, "status", "sampleRateHz"), 20);
    assert_eq!(field_offset(&document, "status", "quantumFrames"), 24);
    assert_eq!(
        scalar_after(&document, "\"constants\": {", "maximumDocumentBytes"),
        u64::from(host_web::MAXIMUM_DOCUMENT_BYTES)
    );
    assert_eq!(
        scalar_after(&document, "\"constants\": {", "defaultMaximumMemoryBytes"),
        host_web::DEFAULT_MAXIMUM_MEMORY_BYTES
    );
    assert_eq!(
        scalar_after(&document, "\"constants\": {", "defaultCommandQueueRecords"),
        u64::from(host_web::DEFAULT_COMMAND_QUEUE_RECORDS)
    );
}

/// The work-limit record is published from the actual Rust record, including its scalar types.
///
/// Red mutations: moving a field's `offset_of!` row or changing one of its type strings makes the
/// corresponding assertion fail. Each lookup is bounded to `observationWorkLimits`, so a missing
/// field cannot accidentally resolve to a same-named row in another structure.
#[test]
fn observation_work_limits_layout_matches_the_rust_record() {
    let document = render();
    assert_eq!(
        structure_bytes(&document, "observationWorkLimits"),
        size_of::<WebObservationWorkLimits>() as u64,
        "the published work-limit size is the Rust record size"
    );

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, "observationWorkLimits", $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationWorkLimits, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!(
        "maximumActiveMeterChannels",
        maximum_active_meter_channels,
        u64
    );
    assert_field!(
        "maximumMeterSamplesPerBlock",
        maximum_meter_samples_per_block,
        u64
    );
    assert_field!(
        "maximumMeterPublicationsPerBlock",
        maximum_meter_publications_per_block,
        u64
    );
    assert_field!(
        "maximumMeterPublicationBytesPerBlock",
        maximum_meter_publication_bytes_per_block,
        u64
    );
    assert_field!(
        "maximumActiveSpectrumCaptures",
        maximum_active_spectrum_captures,
        u64
    );
    assert_field!(
        "maximumCaptureInputSamplesPerBlock",
        maximum_capture_input_samples_per_block,
        u64
    );
    assert_field!(
        "maximumCaptureCopySamplesPerBlock",
        maximum_capture_copy_samples_per_block,
        u64
    );
    assert_field!(
        "maximumCapturePublicationsPerBlock",
        maximum_capture_publications_per_block,
        u64
    );
    assert_field!(
        "maximumCaptureBytesPerSecond",
        maximum_capture_bytes_per_second,
        u64
    );
    assert_field!(
        "maximumTransitionEntryVisitsPerBlock",
        maximum_transition_entry_visits_per_block,
        u64
    );
    assert_field!("maximumRetainedBytes", maximum_retained_bytes, u64);
}

/// The ingress-limit record publishes every Rust scalar and its actual alignment hole.
///
/// The padding row is derived from adjacent Rust fields rather than represented by a Rust
/// reserved member. It is nonsemantic: consumers may ignore its bytes and must not require them
/// to be zero.
#[test]
fn observation_ingress_limits_layout_matches_the_rust_record() {
    let document = render();
    assert_eq!(
        structure_bytes(&document, "observationIngressLimits"),
        size_of::<WebObservationIngressLimits>() as u64,
        "the published ingress-limit size is the Rust record size"
    );

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, "observationIngressLimits", $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationIngressLimits, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("maximumControlBytes", maximum_control_bytes, u32);
    assert_field!("maximumObservationRows", maximum_observation_rows, u32);
    assert_field!("maximumResultBytes", maximum_result_bytes, u32);
    assert_field!(
        "ordinaryOperationsPerBoundary",
        ordinary_operations_per_boundary,
        u32
    );
    assert_field!(
        "removalOperationsPerBoundary",
        removal_operations_per_boundary,
        u32
    );

    let removal_end =
        offset_of!(WebObservationIngressLimits, removal_operations_per_boundary) + size_of::<u32>();
    let admission_start = offset_of!(WebObservationIngressLimits, maximum_admission_entry_visits);
    assert_eq!(admission_start - removal_end, 4);
    let (padding_offset, padding_type) =
        field_entry(&document, "observationIngressLimits", "alignmentPadding");
    assert_eq!(padding_offset, removal_end);
    assert_eq!(padding_type, "u8[4]");

    assert_field!(
        "maximumAdmissionEntryVisits",
        maximum_admission_entry_visits,
        u64
    );
    assert_field!(
        "maximumResponseBindingVisits",
        maximum_response_binding_visits,
        u64
    );
    assert_field!(
        "maximumResponseSectionVisits",
        maximum_response_section_visits,
        u64
    );
    assert_field!("maximumResponseCopyBytes", maximum_response_copy_bytes, u64);
    assert_field!(
        "maximumHandlerCopyBytesPerBoundary",
        maximum_handler_copy_bytes_per_boundary,
        u64
    );
    assert_field!(
        "maximumCleanupEntryVisitsPerBoundary",
        maximum_cleanup_entry_visits_per_boundary,
        u64
    );
    assert_field!("maximumRetainedBytes", maximum_retained_bytes, u64);
}

/// The preparation record is published as one flattened, non-overlapping byte layout. The
/// nested records remain authoritative through their own generated rows; this test checks the
/// parent size/boundaries, the prefixed names, and the derived ingress padding without duplicating
/// every child Rust field assertion here.
#[test]
fn observation_preparation_layout_flattens_the_rust_record() {
    let document = render();
    let structure = "observationPreparation";
    let bytes = structure_bytes(&document, structure) as usize;
    assert_eq!(bytes, size_of::<WebObservationPreparationRecord>());

    assert_eq!(
        field_entry(&document, structure, "structSize"),
        (0, "u32".into())
    );
    assert_eq!(
        field_entry(&document, structure, "activationMaximumRetainedBytes"),
        (32, "u64".into())
    );
    assert_eq!(
        field_entry(&document, structure, "workLimits.structSize"),
        (40, "u32".into())
    );
    assert_eq!(
        field_entry(&document, structure, "workLimits.maximumRetainedBytes"),
        (128, "u64".into())
    );
    assert_eq!(
        field_entry(&document, structure, "ingressLimits.structSize"),
        (136, "u32".into())
    );
    assert_eq!(
        field_entry(&document, structure, "ingressLimits.alignmentPadding"),
        (164, "u8[4]".into())
    );
    assert_eq!(
        field_entry(&document, structure, "ingressLimits.maximumRetainedBytes"),
        (216, "u64".into())
    );
    assert_eq!(
        field_entry(&document, structure, "spectrumRequest.structSize"),
        (224, "u32".into())
    );
    assert_eq!(
        field_entry(&document, structure, "spectrumRequest.reserved"),
        (256, "u32[2]".into())
    );
    assert_eq!(
        field_entry(&document, structure, "targetId"),
        (264, "u8[128]".into())
    );

    let names = [
        "structSize",
        "abiVersion",
        "profile",
        "meterCount",
        "residentTaps",
        "spectrumCount",
        "maximumActiveObservers",
        "reserved0",
        "activationMaximumRetainedBytes",
        "workLimits.structSize",
        "workLimits.abiVersion",
        "workLimits.maximumActiveMeterChannels",
        "workLimits.maximumMeterSamplesPerBlock",
        "workLimits.maximumMeterPublicationsPerBlock",
        "workLimits.maximumMeterPublicationBytesPerBlock",
        "workLimits.maximumActiveSpectrumCaptures",
        "workLimits.maximumCaptureInputSamplesPerBlock",
        "workLimits.maximumCaptureCopySamplesPerBlock",
        "workLimits.maximumCapturePublicationsPerBlock",
        "workLimits.maximumCaptureBytesPerSecond",
        "workLimits.maximumTransitionEntryVisitsPerBlock",
        "workLimits.maximumRetainedBytes",
        "ingressLimits.structSize",
        "ingressLimits.abiVersion",
        "ingressLimits.maximumControlBytes",
        "ingressLimits.maximumObservationRows",
        "ingressLimits.maximumResultBytes",
        "ingressLimits.ordinaryOperationsPerBoundary",
        "ingressLimits.removalOperationsPerBoundary",
        "ingressLimits.alignmentPadding",
        "ingressLimits.maximumAdmissionEntryVisits",
        "ingressLimits.maximumResponseBindingVisits",
        "ingressLimits.maximumResponseSectionVisits",
        "ingressLimits.maximumResponseCopyBytes",
        "ingressLimits.maximumHandlerCopyBytesPerBoundary",
        "ingressLimits.maximumCleanupEntryVisitsPerBoundary",
        "ingressLimits.maximumRetainedBytes",
        "spectrumRequest.structSize",
        "spectrumRequest.abiVersion",
        "spectrumRequest.target",
        "spectrumRequest.channels",
        "spectrumRequest.targetIdBytes",
        "spectrumRequest.reserved0",
        "spectrumRequest.maximumCaptureBytes",
        "spectrumRequest.reserved",
        "targetId",
    ];
    let body = structure_body(&document, structure);
    let mut next_offset = 0;
    for (index, name) in names.iter().enumerate() {
        let marker = format!("\"name\": \"{name}\"");
        assert_eq!(
            body.matches(&marker).count(),
            1,
            "field {name} occurs exactly once in the bounded preparation structure"
        );
        let (offset, kind) = field_entry(&document, structure, name);
        assert_eq!(offset, next_offset, "field {name} tiles row {index}");
        next_offset += match kind.as_str() {
            "u8" => 1,
            "u8[4]" => 4,
            "u8[128]" => 128,
            "u32" | "f32" => 4,
            "u32[2]" => 8,
            "u64" => 8,
            other => panic!("unexpected preparation field type {other}"),
        };
    }
    assert_eq!(next_offset, bytes);
}

/// The demand record is published from the six leaves of the authoritative Rust record.
#[test]
fn observation_demand_layout_matches_the_rust_record() {
    let document = render();
    let structure = "observationDemand";
    assert_eq!(
        structure_bytes(&document, structure),
        size_of::<WebObservationDemand>() as u64
    );

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, structure, $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationDemand, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("operation", operation, u32);
    assert_field!("count", count, u32);
    assert_field!("owner", owner, u64);
    let (reserved_offset, reserved_type) = field_entry(&document, structure, "reserved");
    assert_eq!(reserved_offset, offset_of!(WebObservationDemand, reserved));
    assert_eq!(reserved_type, "u32[2]");

    let fields = [
        ("structSize", size_of::<u32>()),
        ("abiVersion", size_of::<u32>()),
        ("operation", size_of::<u32>()),
        ("count", size_of::<u32>()),
        ("owner", size_of::<u64>()),
        ("reserved", size_of::<[u32; 2]>()),
    ];
    let mut next_offset = 0;
    for (name, width) in fields {
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    assert_eq!(next_offset, size_of::<WebObservationDemand>());
}

/// The receipt record is published from every leaf of the authoritative Rust record.
#[test]
fn observation_receipt_layout_matches_the_rust_record() {
    let document = render();
    let structure = "observationReceipt";
    assert_eq!(
        structure_bytes(&document, structure),
        size_of::<WebObservationReceipt>() as u64
    );

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, structure, $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationReceipt, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("domain", domain, u32);
    assert_field!("state", state, u32);
    assert_field!("owner", owner, u64);
    assert_field!("sequence", sequence, u64);
    assert_field!("applicationSample", application_sample, u64);
    assert_field!("result", result, u32);
    assert_field!("reserved", reserved, u32);

    let fields = [
        ("structSize", size_of::<u32>()),
        ("abiVersion", size_of::<u32>()),
        ("domain", size_of::<u32>()),
        ("state", size_of::<u32>()),
        ("owner", size_of::<u64>()),
        ("sequence", size_of::<u64>()),
        ("applicationSample", size_of::<u64>()),
        ("result", size_of::<u32>()),
        ("reserved", size_of::<u32>()),
    ];
    let mut next_offset = 0;
    for (name, width) in fields {
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    assert_eq!(next_offset, size_of::<WebObservationReceipt>());
}

/// The admission record flattens its own leaves and every nested receipt leaf into one complete
/// parent layout. The receipt itself is represented only by its prefixed leaves, so no aggregate
/// row overlaps those bytes.
#[test]
fn observation_admission_layout_flattens_the_rust_record() {
    let document = render();
    let structure = "observationAdmission";
    let bytes = structure_bytes(&document, structure) as usize;
    assert_eq!(bytes, size_of::<WebObservationAdmission>());
    assert_eq!(bytes, 232);

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, structure, $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationAdmission, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("result", result, u32);
    assert_field!("operation", operation, u32);
    assert_field!("flags", flags, u32);
    assert_field!("reason", reason, u32);
    assert_field!("limitBytes", limit_bytes, u32);
    assert_field!("reserved", reserved, u32);
    assert_field!("ingressEpoch", ingress_epoch, u64);
    assert_field!("requested", requested, u64);
    assert_field!("maximum", maximum, u64);

    let (limit_offset, limit_type) = field_entry(&document, structure, "limit");
    assert_eq!(limit_offset, offset_of!(WebObservationAdmission, limit));
    assert_eq!(limit_type, "u8[128]");

    let receipt_offset = offset_of!(WebObservationAdmission, receipt);
    assert_eq!(
        receipt_offset,
        offset_of!(WebObservationAdmission, limit) + size_of::<[u8; 128]>()
    );
    assert_eq!(
        receipt_offset + size_of::<WebObservationReceipt>(),
        size_of::<WebObservationAdmission>()
    );

    let receipt_fields = [
        (
            "receipt.structSize",
            offset_of!(WebObservationReceipt, struct_size),
            size_of::<u32>(),
        ),
        (
            "receipt.abiVersion",
            offset_of!(WebObservationReceipt, abi_version),
            size_of::<u32>(),
        ),
        (
            "receipt.domain",
            offset_of!(WebObservationReceipt, domain),
            size_of::<u32>(),
        ),
        (
            "receipt.state",
            offset_of!(WebObservationReceipt, state),
            size_of::<u32>(),
        ),
        (
            "receipt.owner",
            offset_of!(WebObservationReceipt, owner),
            size_of::<u64>(),
        ),
        (
            "receipt.sequence",
            offset_of!(WebObservationReceipt, sequence),
            size_of::<u64>(),
        ),
        (
            "receipt.applicationSample",
            offset_of!(WebObservationReceipt, application_sample),
            size_of::<u64>(),
        ),
        (
            "receipt.result",
            offset_of!(WebObservationReceipt, result),
            size_of::<u32>(),
        ),
        (
            "receipt.reserved",
            offset_of!(WebObservationReceipt, reserved),
            size_of::<u32>(),
        ),
    ];
    let body = structure_body(&document, structure);
    let mut next_offset = 0;
    for (name, width) in [
        ("structSize", size_of::<u32>()),
        ("abiVersion", size_of::<u32>()),
        ("result", size_of::<u32>()),
        ("operation", size_of::<u32>()),
        ("flags", size_of::<u32>()),
        ("reason", size_of::<u32>()),
        ("limitBytes", size_of::<u32>()),
        ("reserved", size_of::<u32>()),
        ("ingressEpoch", size_of::<u64>()),
        ("requested", size_of::<u64>()),
        ("maximum", size_of::<u64>()),
        ("limit", size_of::<[u8; 128]>()),
    ] {
        let marker = format!("\"name\": \"{name}\"");
        assert_eq!(
            body.matches(&marker).count(),
            1,
            "field {name} occurs exactly once in the bounded admission structure"
        );
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    for (name, child_offset, width) in receipt_fields {
        let marker = format!("\"name\": \"{name}\"");
        assert_eq!(
            body.matches(&marker).count(),
            1,
            "field {name} occurs exactly once in the bounded admission structure"
        );
        assert_eq!(
            field_offset(&document, structure, name),
            receipt_offset + child_offset
        );
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    assert_eq!(next_offset, bytes);
}

/// The status record is published from every leaf of the authoritative Rust record.
#[test]
fn observation_status_layout_matches_the_rust_record() {
    let document = render();
    let structure = "observationStatus";
    let bytes = structure_bytes(&document, structure) as usize;
    assert_eq!(bytes, size_of::<WebObservationStatus>());
    assert_eq!(bytes, 64);

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, structure, $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationStatus, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("profile", profile, u32);
    assert_field!("flags", flags, u32);
    assert_field!("pendingCount", pending_count, u32);
    assert_field!("reserved", reserved, u32);
    assert_field!("owner", owner, u64);
    assert_field!("ingressEpoch", ingress_epoch, u64);
    assert_field!("acceptedGeneration", accepted_generation, u64);
    assert_field!("appliedGeneration", applied_generation, u64);
    assert_field!("selectionEpoch", selection_epoch, u64);

    let fields = [
        ("structSize", size_of::<u32>()),
        ("abiVersion", size_of::<u32>()),
        ("profile", size_of::<u32>()),
        ("flags", size_of::<u32>()),
        ("pendingCount", size_of::<u32>()),
        ("reserved", size_of::<u32>()),
        ("owner", size_of::<u64>()),
        ("ingressEpoch", size_of::<u64>()),
        ("acceptedGeneration", size_of::<u64>()),
        ("appliedGeneration", size_of::<u64>()),
        ("selectionEpoch", size_of::<u64>()),
    ];
    let body = structure_body(&document, structure);
    let mut next_offset = 0;
    for (name, width) in fields {
        let marker = format!("\"name\": \"{name}\"");
        assert_eq!(
            body.matches(&marker).count(),
            1,
            "field {name} occurs exactly once in the bounded status structure"
        );
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    assert_eq!(next_offset, bytes);
}

/// The capture identity record is published from every leaf of the authoritative Rust record.
#[test]
fn observation_capture_identity_layout_matches_the_rust_record() {
    let document = render();
    let structure = "observationCaptureIdentity";
    let bytes = structure_bytes(&document, structure) as usize;
    assert_eq!(bytes, size_of::<WebObservationCaptureIdentity>());
    assert_eq!(bytes, 48);

    macro_rules! assert_field {
        ($name:literal, $field:ident, $ty:ty) => {{
            let (offset, kind) = field_entry(&document, structure, $name);
            assert_eq!(
                offset,
                offset_of!(WebObservationCaptureIdentity, $field),
                "the published offset for {} is the Rust offset",
                $name
            );
            assert_eq!(
                kind,
                core::any::type_name::<$ty>(),
                "the published type for {} is the Rust type",
                $name
            );
        }};
    }

    assert_field!("structSize", struct_size, u32);
    assert_field!("abiVersion", abi_version, u32);
    assert_field!("kind", kind, u32);
    assert_field!("flags", flags, u32);
    assert_field!("owner", owner, u64);
    assert_field!("observationGeneration", observation_generation, u64);
    assert_field!("selectionEpoch", selection_epoch, u64);
    assert_field!("snapshotToken", snapshot_token, u64);

    let fields = [
        ("structSize", size_of::<u32>()),
        ("abiVersion", size_of::<u32>()),
        ("kind", size_of::<u32>()),
        ("flags", size_of::<u32>()),
        ("owner", size_of::<u64>()),
        ("observationGeneration", size_of::<u64>()),
        ("selectionEpoch", size_of::<u64>()),
        ("snapshotToken", size_of::<u64>()),
    ];
    let body = structure_body(&document, structure);
    let mut next_offset = 0;
    for (name, width) in fields {
        let marker = format!("\"name\": \"{name}\"");
        assert_eq!(
            body.matches(&marker).count(),
            1,
            "field {name} occurs exactly once in the bounded capture identity structure"
        );
        assert_eq!(field_offset(&document, structure, name), next_offset);
        next_offset += width;
    }
    assert_eq!(next_offset, bytes);
}

/// Regeneration is deterministic: the same tree renders the same bytes.
#[test]
fn rendering_is_deterministic() {
    assert_eq!(render(), render());
}

/// The checked-in schema-gate fixture is this generator's exact output.
///
/// `scripts/check-abi-layout-v1.py --self-test` runs its fifteen red mutations against that file,
/// so a stale fixture would mean the gate proves its discrimination against a document the engine
/// no longer emits. Regenerate with
/// `cargo run -q -p parameter-metadata -- --print-abi-layout
/// > scripts/fixtures/abi-layout-v1-self-test.json`.
#[test]
fn the_checked_in_self_test_fixture_is_current() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scripts/fixtures/abi-layout-v1-self-test.json");
    let existing = std::fs::read_to_string(&fixture).expect("the self-test fixture exists");
    assert_eq!(
        existing,
        render(),
        "scripts/fixtures/abi-layout-v1-self-test.json is stale; regenerate it with \
         --print-abi-layout"
    );
}

/// The published export set is exactly the set the artifact gate freezes.
///
/// Two lists, one truth: `scripts/check-web-audioworklet.sh` proves the *module* exports exactly
/// these symbols by disassembling it; this proves the *document* names exactly the same ones, by
/// reading that script's own list. An export added to the engine without both lists moving fails
/// one of the two.
#[test]
fn the_published_export_set_is_the_frozen_artifact_set() {
    let gate = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/check-web-audioworklet.sh"),
    )
    .expect("the artifact gate exists");
    let start = gate
        .find("expected_exports=$(printf '%s\\n' \\")
        .expect("the gate declares its expected export set");
    let end = gate[start..].find("| sort)").expect("the list is closed") + start;
    let mut frozen: Vec<&str> = gate[start..end]
        .lines()
        .skip(1)
        .map(|line| line.trim().trim_end_matches('\\').trim())
        .filter(|line| !line.is_empty() && *line != "memory")
        .collect();
    frozen.sort_unstable();

    let document = render();
    let key = "\"exports\": [";
    let list_start = document.find(key).expect("the document names exports") + key.len();
    let list_end = document[list_start..].find(']').expect("closed") + list_start;
    let published: Vec<String> = document[list_start..list_end]
        .split(',')
        .map(|entry| entry.trim().trim_matches(['"', '\n', ' ']).to_owned())
        .filter(|entry| !entry.is_empty())
        .collect();

    assert_eq!(
        published, frozen,
        "the published export set and the artifact gate's frozen set are one list"
    );
}
