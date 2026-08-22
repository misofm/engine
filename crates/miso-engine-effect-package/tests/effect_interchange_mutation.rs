//! Deterministic bounded mutation qualification for the three interchange parsers.

use std::panic::{AssertUnwindSafe, catch_unwind};

use miso_engine_effect_package::*;
use sha2::{Digest, Sha256};

const TRIALS: usize = 10_000;
const DESCRIPTOR_SEED: u64 = 0x081d_e5c0_0000_0001;
const PACKAGE_SEED: u64 = 0x081d_e5c0_0000_0002;
const STATE_SEED: u64 = 0x081d_e5c0_0000_0003;

#[derive(Clone, Copy)]
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

#[derive(Default)]
struct Summary {
    accepted: usize,
    rejected: usize,
    normalized: Sha256,
}

fn hex_fixture(text: &str) -> Vec<u8> {
    let compact: Vec<_> = text
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("lowercase hex fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn guarded_candidate(source: &[u8], random: u64) -> Vec<u8> {
    let mut guarded = vec![0x3c; source.len() + 32];
    guarded[16..16 + source.len()].copy_from_slice(source);
    let offset = random as usize % source.len();
    let bit = 1 << ((random >> 32) & 7);
    guarded[16 + offset] ^= bit as u8;
    guarded
}

fn absorb(summary: &mut Summary, parser: &[u8], trial: usize, outcome: &[u8], accepted: bool) {
    summary.accepted += usize::from(accepted);
    summary.rejected += usize::from(!accepted);
    summary
        .normalized
        .update((parser.len() as u64).to_le_bytes());
    summary.normalized.update(parser);
    summary.normalized.update((trial as u64).to_le_bytes());
    summary
        .normalized
        .update((outcome.len() as u64).to_le_bytes());
    summary.normalized.update(outcome);
}

fn assert_descriptor_diagnostic_shape(
    error: EffectDescriptorWireDiagnosticV1,
    candidate_len: usize,
) {
    assert_eq!(error.required_bytes, 0);
    assert!(
        error.byte_offset == EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE
            || (error.byte_offset as usize) < candidate_len
    );
    if error.record_index != EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE {
        assert!(error.record_index < 4_096);
        assert_ne!(error.byte_offset, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE);
    }
}

fn assert_package_diagnostic_shape(error: EffectPackageDiagnosticV1, candidate_len: usize) {
    assert_eq!(error.reserved, 0);
    assert_eq!(error.required_bytes, 0);
    assert!(
        error.byte_offset == EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET
            || error.byte_offset < candidate_len as u64
    );
    if error.artifact_index != EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX {
        assert!(error.artifact_index < 4_096);
        assert_ne!(error.byte_offset, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET);
    }
    if matches!(error.code, EffectPackageDiagnosticCodeV1::Descriptor) {
        assert_eq!(error.artifact_index, EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX);
    }
}

fn assert_state_diagnostic_shape(error: EffectStateDiagnosticV1, candidate_len: usize) {
    assert_eq!(error.reserved, 0);
    if matches!(error.code, EffectStateDiagnosticCodeV1::Limit) {
        assert_eq!(error.detail, 0);
        assert_eq!(error.item_index, EFFECT_STATE_V1_UNAVAILABLE_INDEX);
        assert_eq!(error.byte_offset, EFFECT_STATE_V1_UNAVAILABLE_OFFSET);
        assert!(error.required_bytes > 0);
        return;
    }
    assert_eq!(error.required_bytes, 0);
    assert!(error.byte_offset < candidate_len as u64);
    if error.item_index != EFFECT_STATE_V1_UNAVAILABLE_INDEX {
        assert!(error.item_index < 4_096);
    }
}

fn descriptor_outcome(candidate: &[u8]) -> Vec<u8> {
    match verify_effect_descriptor_wire_v1(candidate, 4_194_304) {
        Ok(view) => {
            assert_eq!(view.as_bytes().as_ptr(), candidate.as_ptr());
            assert_eq!(view.as_bytes().len(), candidate.len());
            let identity = effect_descriptor_identity_v1(candidate, 4_194_304).unwrap();
            let mut row = vec![1];
            row.extend_from_slice(identity.as_bytes());
            for value in [
                view.parameter_count(),
                view.port_count(),
                view.quality_count(),
                view.enum_choice_count(),
                view.state_layout_version(),
                view.supported_link_mode_bits(),
            ] {
                row.extend_from_slice(&value.to_le_bytes());
            }
            row
        }
        Err(error) => {
            assert_descriptor_diagnostic_shape(error, candidate.len());
            let mut row = vec![0];
            for value in [
                error.code as u32,
                error.byte_offset,
                error.record_index,
                error.required_bytes,
            ] {
                row.extend_from_slice(&value.to_le_bytes());
            }
            row
        }
    }
}

fn package_outcome(candidate: &[u8]) -> Vec<u8> {
    match verify_effect_package_v1(candidate, EffectPackageLimitsV1::default()) {
        Ok(view) => {
            assert_eq!(view.as_bytes().as_ptr(), candidate.as_ptr());
            assert_eq!(view.as_bytes().len(), candidate.len());
            let cid = effect_package_cid_v1(candidate, EffectPackageLimitsV1::default()).unwrap();
            let artifacts: Vec<_> = view
                .artifacts()
                .map(|artifact| {
                    let start = candidate.as_ptr() as usize;
                    let end = start + candidate.len();
                    let content = artifact.content().as_ptr() as usize;
                    assert!(content >= start && content + artifact.content().len() <= end);
                    EffectArtifactAuthoringV1 {
                        kind: artifact.kind(),
                        path: artifact.path(),
                        target: artifact.target(),
                        features: artifact.features(),
                        content: artifact.content(),
                    }
                })
                .collect();
            let authoring = EffectPackageAuthoringV1 {
                descriptor: view.descriptor(),
                artifacts: &artifacts,
            };
            let required =
                effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default())
                    .unwrap();
            assert_eq!(required, candidate.len() as u64);
            let mut encoded = vec![0xa5; candidate.len()];
            let written = encode_effect_package_v1(
                &authoring,
                EffectPackageLimitsV1::default(),
                &mut encoded,
            )
            .unwrap();
            assert_eq!(written, candidate.len());
            assert_eq!(encoded, candidate);
            let mut row = vec![1];
            row.extend_from_slice(cid.as_binary());
            row.extend_from_slice(&view.artifact_count().to_le_bytes());
            row
        }
        Err(error) => {
            assert_package_diagnostic_shape(error, candidate.len());
            let mut row = vec![0];
            for value in [
                error.code as u32,
                error.detail,
                error.artifact_index,
                error.reserved,
            ] {
                row.extend_from_slice(&value.to_le_bytes());
            }
            row.extend_from_slice(&error.byte_offset.to_le_bytes());
            row.extend_from_slice(&error.required_bytes.to_le_bytes());
            row
        }
    }
}

fn state_outcome(candidate: &[u8]) -> Vec<u8> {
    match inspect_effect_state_selector_v1(candidate, EffectStateLimitsV1::default()) {
        Ok(selector) => {
            let mut row = vec![1];
            row.extend_from_slice(selector.descriptor_identity().as_bytes());
            row.extend_from_slice(&selector.state_layout_version().to_le_bytes());
            row
        }
        Err(error) => {
            assert_state_diagnostic_shape(error, candidate.len());
            let mut row = vec![0];
            for value in [
                error.code as u32,
                error.detail,
                error.item_index,
                error.reserved,
            ] {
                row.extend_from_slice(&value.to_le_bytes());
            }
            row.extend_from_slice(&error.byte_offset.to_le_bytes());
            row.extend_from_slice(&error.required_bytes.to_le_bytes());
            row
        }
    }
}

fn run_parser(
    name: &[u8],
    seed: u64,
    trials: usize,
    fixtures: &[Vec<u8>],
    parser: fn(&[u8]) -> Vec<u8>,
) -> Summary {
    let fixture_hashes: Vec<_> = fixtures.iter().map(Sha256::digest).collect();
    let mut random = SplitMix64(seed);
    let mut summary = Summary::default();
    for trial in 0..trials {
        let source = &fixtures[trial % fixtures.len()];
        let value = random.next();
        let guarded = guarded_candidate(source, value);
        let input = &guarded[16..16 + source.len()];
        let first = catch_unwind(AssertUnwindSafe(|| parser(input))).expect("parser panic");
        let second = catch_unwind(AssertUnwindSafe(|| parser(input))).expect("parser replay panic");
        assert_eq!(
            first, second,
            "nondeterministic parser outcome at trial {trial}"
        );
        assert!(guarded[..16].iter().all(|byte| *byte == 0x3c));
        assert!(
            guarded[16 + source.len()..]
                .iter()
                .all(|byte| *byte == 0x3c)
        );
        assert_eq!(
            Sha256::digest(source),
            fixture_hashes[trial % fixtures.len()]
        );
        absorb(&mut summary, name, trial, &first, first.first() == Some(&1));
    }
    assert_eq!(summary.accepted + summary.rejected, trials);
    summary
}

fn campaigns(trials: usize) -> [(String, Summary); 3] {
    let descriptors = [
        hex_fixture(include_str!(
            "../../../fixtures/effect-descriptor/v1/comprehensive-a.wire.hex"
        )),
        hex_fixture(include_str!(
            "../../../fixtures/effect-descriptor/v1/comprehensive-b.wire.hex"
        )),
    ];
    let packages = [
        hex_fixture(include_str!(
            "../../../fixtures/effect-package/v1/comprehensive-a.package.hex"
        )),
        hex_fixture(include_str!(
            "../../../fixtures/effect-package/v1/comprehensive-b.package.hex"
        )),
    ];
    let states = [include_bytes!("../../../fixtures/effect-state/v1/canonical.state.bin").to_vec()];
    [
        (
            "descriptor".into(),
            run_parser(
                b"descriptor",
                DESCRIPTOR_SEED,
                trials,
                &descriptors,
                descriptor_outcome,
            ),
        ),
        (
            "package".into(),
            run_parser(b"package", PACKAGE_SEED, trials, &packages, package_outcome),
        ),
        (
            "state".into(),
            run_parser(b"state", STATE_SEED, trials, &states, state_outcome),
        ),
    ]
}

#[test]
fn tiny_deterministic_mutation_smoke() {
    for (_, summary) in campaigns(4) {
        assert_eq!(summary.accepted + summary.rejected, 4);
    }
}

#[test]
#[ignore = "Issue 081 exact 30,000-trial one-shot qualification campaign"]
fn exact_deterministic_mutation_campaign() {
    for (name, summary) in campaigns(TRIALS) {
        let digest = summary.normalized.finalize();
        let digest_hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        println!(
            "parser={name} trials={} accepted={} rejected={} panics=0 normalized_sha256={digest_hex}",
            TRIALS, summary.accepted, summary.rejected
        );
    }
}
