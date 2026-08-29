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

use miso_engine_effect_compiler::launch_native_effect_registry;
use miso_engine_host_core::LAUNCH_SAMPLE_RATES_HZ;
use miso_engine_host_web::{
    AudioWorkletEngineHost, COMMAND_EFFECT_PARAM, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_TRACK,
    COMMAND_RECORD_BYTES, RESULT_OK, RESULT_UNSUPPORTED, WebBootOptions,
    default_source_ring_frames,
};
use miso_engine_parameter_metadata::abi_layout::{
    ERROR_PHASES, SCHEMA, SOURCE_RING_RESERVE_QUANTA, STAGING_SEQUENCE, render,
};

/// Minimal parsing: the document is generated, so a test that pulled in a JSON crate would be
/// testing the crate. These helpers read the exact shapes this generator emits and panic loudly on
/// anything else, which is itself an assertion that the shape did not change.
fn field_offset(document: &str, structure: &str, field: &str) -> usize {
    let structure_start = document
        .find(&format!("\"{structure}\": {{"))
        .unwrap_or_else(|| panic!("document names structure {structure}"));
    let row = format!("{{ \"name\": \"{field}\", \"offset\": ");
    let row_start = document[structure_start..]
        .find(&row)
        .unwrap_or_else(|| panic!("structure {structure} names field {field}"))
        + structure_start
        + row.len();
    let row_end = document[row_start..]
        .find(',')
        .expect("offset is followed by a type")
        + row_start;
    document[row_start..row_end]
        .trim()
        .parse()
        .expect("offset is an integer")
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
        "schema_version = 1\n\
         session_id = \"abi-layout\"\n\
         revision = 1\n\
         sample_rate_hz = 48000\n\
         quantum_frames = 128\n\
         render_profile = {{ id = \"native\", mode = \"single_thread\" }}\n\
         output_profile = {{ id = \"main\", channels = 2, sample_format = \"f32_planar\" }}\n\
         sources = [{{ id = \"s\", content = \"sha256:{}\", channels = 2, bit_depth = 24, \
         frames = 48000 }}]\n\
         submixes = []\n\
         outputs = [{{ id = \"out\" }}]\n\
         routes = [{{ id = \"r\", source = {{ kind = \"track\", track_id = \"t\", \
         tap = \"post_matrix\" }}, destination = {{ kind = \"output_input\", output_id = \"out\" }}, \
         channel_matrix = {{ ll = 1.0, lr = 0.0, rl = 0.0, rr = 1.0 }}, gain_db = 0.0 }}]\n\
         automation = []\n\
         \n\
         [[tracks]]\n\
         id = \"t\"\n\
         source_id = \"s\"\n\
         left_source_channel = 0\n\
         right_source_channel = 1\n\
         builtins = {{ left = {{ polarity_invert = false, trim_db = 0.0, hpf_hz = 0.0, \
         lpf_hz = 0.0, delay_samples = 0 }}, right = {{ polarity_invert = false, trim_db = 0.0, \
         hpf_hz = 0.0, lpf_hz = 0.0, delay_samples = 0 }} }}\n\
         simd1 = {{ effects = [] }}\n\
         dynamic = {{ effects = [{{ id = \"e\", identity = {{ kind = \"native\", \
         effect_id = \"{effect_id}\" }}, quality = \"normal\", bypass = false, \
         link_mode = \"dual_mono\", params = [], sidechain = {{ kind = \"none\" }} }}] }}\n\
         simd2 = {{ effects = [] }}\n\
         fader = {{ left_db = 0.0, right_db = 0.0, left_mute = false, right_mute = false }}\n\
         pan = {{ left = -1.0, right = 1.0, smoothing_samples = 0 }}\n",
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
        miso_engine_host_web::SOURCE_STALL_TOLERANCE_MS,
        "the published tolerance is the engine's constant"
    );

    for rate in LAUNCH_SAMPLE_RATES_HZ {
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
                miso_engine_host_web::RESULT_REFUSED_DOCUMENT,
                "refusedDocument".to_owned()
            ),
            (
                miso_engine_host_web::RESULT_REFUSED_OPTIONS,
                "refusedOptions".to_owned()
            ),
            (
                miso_engine_host_web::RESULT_REFUSED_LIFECYCLE,
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
        u64::from(miso_engine_host_web::MAXIMUM_DOCUMENT_BYTES)
    );
    assert_eq!(
        scalar_after(&document, "\"constants\": {", "defaultMaximumMemoryBytes"),
        miso_engine_host_web::DEFAULT_MAXIMUM_MEMORY_BYTES
    );
    assert_eq!(
        scalar_after(&document, "\"constants\": {", "defaultCommandQueueRecords"),
        u64::from(miso_engine_host_web::DEFAULT_COMMAND_QUEUE_RECORDS)
    );
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
/// `cargo run -q -p miso-engine-parameter-metadata -- --print-abi-layout
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
