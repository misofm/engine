//! The emitted ABI layout is the engine's own bytes, not a table somebody typed (issue #243).
//!
//! `render_abi_layout` takes its struct offsets through `offset_of!`, so a field that moves moves
//! the document with it and no assertion here can catch a *rename*. What these tests catch is the
//! other half, which `offset_of!` cannot reach:
//!
//! * the **source-ring rule** (#1406 D3), which the document states as its inputs rather than as
//!   an answer. The test re-derives the ring from those inputs and requires it to equal
//!   `default_source_ring_frames` at every launch rate and a spread of quanta, so a consumer that
//!   applies the published rule lands where the engine lands, and requires the host to boot on
//!   that ring.
//! * the **alias table**, which must be exactly the three alias constants and must agree with the
//!   base names on the same values.
//! * the **document's own structure**, so a schema key cannot be dropped silently.

use host_core::{LAUNCH_SAMPLE_RATES, p_max_samples, stall_ring_frames};
use host_web::{AudioWorkletEngineHost, WebBootOptions, default_source_ring_frames};
use parameter_metadata::abi_layout::{ERROR_PHASES, SCHEMA, STAGING_SEQUENCE, render};

/// The document parsed as JSON. Each helper below panics loudly on a missing key, which is itself
/// an assertion that the schema did not change.
fn parse(document: &str) -> serde_json::Value {
    serde_json::from_str(document).expect("the document is JSON")
}

/// A structure's `fields` rows: a member of `structures`.
fn field_offset(document: &str, structure: &str, field: &str) -> usize {
    let document = parse(document);
    let block = document["structures"]
        .get(structure)
        .unwrap_or_else(|| panic!("document names structure {structure}"));
    let row = block["fields"]
        .as_array()
        .expect("a structure has fields")
        .iter()
        .find(|row| row["name"] == field)
        .unwrap_or_else(|| panic!("structure {structure} names field {field}"));
    usize::try_from(row["offset"].as_u64().expect("offset is an integer")).expect("a usize")
}

fn named_constants(document: &str, group: &str) -> Vec<(u32, String)> {
    parse(document)["constants"][group]
        .as_array()
        .unwrap_or_else(|| panic!("document names constant group {group}"))
        .iter()
        .map(|row| {
            (
                u32::try_from(row["value"].as_u64().expect("value is an integer")).expect("a u32"),
                row["name"].as_str().expect("row names a name").to_owned(),
            )
        })
        .collect()
}

/// Read the integer `name` of the top-level object `block`.
fn scalar(document: &str, block: &str, name: &str) -> u64 {
    parse(document)[block][name]
        .as_u64()
        .unwrap_or_else(|| panic!("document names scalar {name} in {block}"))
}

/// The published source-ring rule reproduces the engine's derivation at every launch shape (#1406
/// gates 2 and 3), and the host boots on the ring the zero override derives.
///
/// Red mutations: publish `reserveQuanta: 1` or `primeReserveQuanta: 0` and every row misses by
/// one quantum; take `p_max_samples` at quantum 128 for every quantum and the 96 kHz / 127 row is
/// 14,129 frames, not a whole number of quanta, which the boot refuses.
#[test]
fn the_published_source_ring_rule_reproduces_the_engine_derivation() {
    let document = render();
    let ring = &parse(&document)["constants"]["sourceRing"];
    let input = |name: &str| {
        ring[name]
            .as_u64()
            .unwrap_or_else(|| panic!("the document names sourceRing.{name}"))
    };
    let tolerance_ms = input("stallToleranceMs");
    assert_eq!(
        u32::try_from(tolerance_ms).expect("a u32"),
        host_web::SOURCE_STALL_TOLERANCE_MS,
        "the published tolerance is the engine's constant"
    );
    let latencies = ring["primeLatencySamples"]
        .as_array()
        .expect("the document names sourceRing.primeLatencySamples");
    assert_eq!(
        latencies.len(),
        LAUNCH_SAMPLE_RATES.len(),
        "one row per launch rate"
    );
    let minimal = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/session/v1/canonical-minimal.json"),
    )
    .expect("the minimal fixture exists");

    for (rate, row) in LAUNCH_SAMPLE_RATES.into_iter().zip(latencies) {
        let rate = rate.0;
        assert_eq!(row["sampleRateHz"].as_u64(), Some(u64::from(rate)));
        let latency = row["samples"].as_u64().expect("a latency row's samples");
        for quantum in [1_u32, 2, 32, 64, 127, 128, 129, 256, 480, 1024] {
            let q = u64::from(quantum);
            let stall_frames = u64::from(rate) * tolerance_ms / 1_000;
            let quanta = stall_frames.div_ceil(q)
                + input("reserveQuanta")
                + input("primeGrowths") * latency.div_ceil(q)
                + input("primeReserveQuanta");
            let published = u32::try_from(quanta * q).expect("fits a u32");
            let engine = default_source_ring_frames(rate, quantum);
            assert_eq!(
                published, engine,
                "the published rule and the engine disagree at {rate} Hz / {quantum} frames"
            );
            let floor = u64::from(stall_ring_frames(rate, quantum))
                + u64::from(p_max_samples(rate, quantum))
                + q;
            assert!(
                engine.is_multiple_of(quantum) && u64::from(engine) >= floor,
                "{rate} Hz / {quantum} frames: {engine} frames"
            );
            // The boot takes the zero override as `default_source_ring_frames` and refuses a ring
            // that preparation would.
            let session = minimal
                .replacen(
                    "\"sample_rate_hz\": 48000",
                    &format!("\"sample_rate_hz\": {rate}"),
                    1,
                )
                .replacen(
                    "\"quantum_frames\": 128",
                    &format!("\"quantum_frames\": {quantum}"),
                    1,
                );
            let options = WebBootOptions {
                require_sample_rate_hz: rate,
                require_quantum_frames: quantum,
                ..WebBootOptions::explicit_defaults()
            };
            AudioWorkletEngineHost::boot(session.as_bytes(), options)
                .unwrap_or_else(|_| panic!("{rate} Hz / {quantum} frames: the zero ring boots"));
        }
    }
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
        results.len(),
        12,
        "the base ladder is the twelve frozen result codes (#1562a added compileRejected)"
    );
}

/// Every published struct offset is the engine's, and the document carries its whole schema.
#[test]
fn the_document_carries_its_whole_schema_and_the_engine_s_offsets() {
    let document = render();
    let json = parse(&document);
    assert_eq!(json["schema"], SCHEMA);
    assert_eq!(
        json["stagingSequence"],
        serde_json::json!(STAGING_SEQUENCE),
        "the staging sequence is the four boot exports, in call order"
    );
    assert_eq!(
        json["errorPhases"],
        serde_json::json!(ERROR_PHASES),
        "the phase vocabulary is the six phases, in order"
    );
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
    assert_eq!(field_offset(&document, "bootOptions", "maximumControlFrameBytes"), 32);
    assert_eq!(field_offset(&document, "bootOptions", "reserved2"), 56);
    assert_eq!(field_offset(&document, "status", "sampleRateHz"), 20);
    assert_eq!(field_offset(&document, "status", "quantumFrames"), 24);
    assert_eq!(field_offset(&document, "bootOptions", "maximumDocumentBytes"), 48);
    assert_eq!(
        scalar(&document, "constants", "parseTransientMultiplier"),
        host_web::PARSE_TRANSIENT_MULTIPLIER
    );
    assert_eq!(
        scalar(&document, "constants", "defaultMaximumMemoryBytes"),
        host_web::DEFAULT_MAXIMUM_MEMORY_BYTES
    );
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
