//! The typed source-diagnostic table is pinned, variant by variant (audit #103 F6).
//!
//! F6 was exactly this collapse: seventeen distinct source rejections reaching a host as one of
//! two strings. A table with no test is one careless edit away from being that collapse again, and
//! the verifier proved it: rewriting a single arm to `"engine.invalid_argument"` survived every
//! other gate in this crate and in `capi`, because those gates assert the error
//! *value* and never the string it reports.
//!
//! Each row is pinned three ways, so no single edit can move a string unnoticed:
//!
//! 1. **The exact string.** `diagnostic()` must return the recorded text.
//! 2. **The classification.** `is_backpressure()` and `is_internal()` must match, so a string
//!    cannot be quietly retargeted at a variant a host treats differently.
//! 3. **The reverse map.** Every code maps back to exactly the variants recorded here, so an arm
//!    cannot be merged into a neighbour's string and cannot be split away from a documented pair.
//!
//! `variant_index` is the fourth guard and a compile-time one: it matches exhaustively, so adding
//! a variant to `SourceControlError` fails to build until this table gains its row.

use engine::SampleRateHz;
use host_core::SourceControlError;
use source::{HostChunkError, SourceFrame, SourceGeneration, SourceSeekError};

/// `(variant, diagnostic code, is_backpressure, is_internal)`, in `diagnostic()`'s arm order.
///
/// Payloads are arbitrary: a diagnostic names a rule, never a value.
const TABLE: &[(SourceControlError, &str, bool, bool)] = &[
    (
        SourceControlError::UnknownSource,
        "source.id.unknown",
        false,
        false,
    ),
    (
        SourceControlError::RegionOverflow,
        "source.region.overflow",
        false,
        false,
    ),
    (
        SourceControlError::OutsideRegion,
        "source.region.outside",
        false,
        false,
    ),
    (
        SourceControlError::EndOfRegionMismatch,
        "source.region.end_mismatch",
        false,
        false,
    ),
    (
        SourceControlError::GenerationZero,
        "source.generation.zero",
        false,
        false,
    ),
    (
        SourceControlError::Seek(SourceSeekError::GenerationZero),
        "source.generation.zero",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::WrongSampleRate {
            expected: SampleRateHz(48_000),
            actual: SampleRateHz(44_100),
        }),
        "source.rate.mismatch",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::StaleGeneration {
            active: SourceGeneration(2),
            submitted: SourceGeneration(1),
        }),
        "source.generation.stale",
        false,
        false,
    ),
    (
        SourceControlError::Seek(SourceSeekError::GenerationNotStrictlyIncreasing {
            active: SourceGeneration(2),
            requested: SourceGeneration(2),
        }),
        "source.generation.stale",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::ChannelCount {
            expected: 2,
            actual: 3,
        }),
        "source.channels.mismatch",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::PlaneLength {
            expected_frames: 128,
        }),
        "source.plane.length",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::FrameCount {
            quantum_frames: 128,
            submitted_frames: 64,
            end_of_region: false,
        }),
        "source.frames.shape",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::NonContiguous {
            expected: SourceFrame(128),
            actual: SourceFrame(256),
        }),
        "source.frame.noncontiguous",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::EndOfRegionAlreadySubmitted),
        "source.region.ended",
        false,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::Full { full_count: 1 }),
        "source.backpressure",
        true,
        false,
    ),
    (
        SourceControlError::Chunk(HostChunkError::InternalInvariant),
        "source.internal",
        false,
        true,
    ),
    (
        SourceControlError::Seek(SourceSeekError::Backpressure { full_count: 1 }),
        "source.seek.backpressure",
        true,
        false,
    ),
];

/// The two codes a host must see from more than one variant, and why. Anything else sharing a
/// code is a collapse, which is what F6 removed.
const SHARED_CODES: &[(&str, usize)] = &[
    // Generation `0` is invalid whether it arrives on a submission or on a seek.
    ("source.generation.zero", 2),
    // A generation that does not advance is stale whichever call carries it.
    ("source.generation.stale", 2),
];

/// Exhaustive by construction: adding a `SourceControlError` variant stops this crate's tests from
/// compiling until [`TABLE`] gains its row.
const fn variant_index(error: SourceControlError) -> usize {
    match error {
        SourceControlError::UnknownSource => 0,
        SourceControlError::RegionOverflow => 1,
        SourceControlError::OutsideRegion => 2,
        SourceControlError::EndOfRegionMismatch => 3,
        SourceControlError::GenerationZero => 4,
        SourceControlError::Seek(SourceSeekError::GenerationZero) => 5,
        SourceControlError::Chunk(HostChunkError::WrongSampleRate { .. }) => 6,
        SourceControlError::Chunk(HostChunkError::StaleGeneration { .. }) => 7,
        SourceControlError::Seek(SourceSeekError::GenerationNotStrictlyIncreasing { .. }) => 8,
        SourceControlError::Chunk(HostChunkError::ChannelCount { .. }) => 9,
        SourceControlError::Chunk(HostChunkError::PlaneLength { .. }) => 10,
        SourceControlError::Chunk(HostChunkError::FrameCount { .. }) => 11,
        SourceControlError::Chunk(HostChunkError::NonContiguous { .. }) => 12,
        SourceControlError::Chunk(HostChunkError::EndOfRegionAlreadySubmitted) => 13,
        SourceControlError::Chunk(HostChunkError::Full { .. }) => 14,
        SourceControlError::Chunk(HostChunkError::InternalInvariant) => 15,
        SourceControlError::Seek(SourceSeekError::Backpressure { .. }) => 16,
    }
}

#[test]
fn every_source_rejection_reports_its_own_pinned_diagnostic() {
    for (index, (error, code, backpressure, internal)) in TABLE.iter().enumerate() {
        assert_eq!(
            error.diagnostic(),
            *code,
            "row {index} ({error:?}) must report its recorded code"
        );
        assert_eq!(
            error.is_backpressure(),
            *backpressure,
            "row {index} ({error:?}) backpressure classification"
        );
        assert_eq!(
            error.is_internal(),
            *internal,
            "row {index} ({error:?}) internal classification"
        );
        assert_eq!(
            variant_index(*error),
            index,
            "row {index} ({error:?}) is out of table order"
        );
        assert!(
            code.starts_with("source."),
            "row {index}: a source rejection must stay in the `source.` namespace, not {code}"
        );
    }
}

#[test]
fn only_the_documented_pairs_share_a_code() {
    for (row, (error, code, _, _)) in TABLE.iter().enumerate() {
        let sharing: Vec<usize> = TABLE
            .iter()
            .enumerate()
            .filter(|(_, (_, other, _, _))| other == code)
            .map(|(index, _)| index)
            .collect();
        let expected = SHARED_CODES
            .iter()
            .find_map(|(shared, count)| (shared == code).then_some(*count))
            .unwrap_or(1);
        assert_eq!(
            sharing.len(),
            expected,
            "row {row} ({error:?}): {code} is reported by rows {sharing:?}, but exactly {expected} \
             variant(s) may report it"
        );
    }
    for (code, count) in SHARED_CODES {
        assert!(
            TABLE
                .iter()
                .filter(|(_, other, _, _)| other == code)
                .count()
                == *count,
            "{code} is documented as shared by {count} variants but the table disagrees"
        );
    }
}

/// A rejection is a caller mistake unless it is bounded backpressure or an engine invariant
/// failure. Hosts map exactly those two classes to something other than "invalid argument", so a
/// variant silently joining or leaving either class changes what a host does.
#[test]
fn classification_partitions_the_table() {
    let backpressure: Vec<&str> = TABLE
        .iter()
        .filter(|(error, _, _, _)| error.is_backpressure())
        .map(|(_, code, _, _)| *code)
        .collect();
    assert_eq!(
        backpressure,
        ["source.backpressure", "source.seek.backpressure"]
    );

    let internal: Vec<&str> = TABLE
        .iter()
        .filter(|(error, _, _, _)| error.is_internal())
        .map(|(_, code, _, _)| *code)
        .collect();
    assert_eq!(internal, ["source.internal"]);

    assert!(
        TABLE
            .iter()
            .all(|(error, _, _, _)| !(error.is_backpressure() && error.is_internal())),
        "no rejection may be both backpressure and an internal invariant failure"
    );
}
