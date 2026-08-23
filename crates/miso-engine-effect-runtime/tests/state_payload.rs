//! The state-payload codec: round trip, length rejection, version rejection, byte order.

use miso_engine_effect_runtime::state_payload::{
    HEADER_WORDS, STATE_LENGTH_CODE, STATE_VERSION_CODE, StateLayout, StatePayloadInput,
    StatePayloadOutput, StateWords, StateWordsMut, expected_sizes, read_f32, read_header, read_u32,
    restore, snapshot, validate_lengths, write_f32, write_header, write_u32,
};

const LAYOUT: StateLayout = StateLayout {
    version: 3,
    common_words: 2,
    lane_words: 5,
};

fn buffers() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sizes = expected_sizes(&LAYOUT);
    (
        vec![0u8; sizes.common],
        vec![0u8; sizes.left],
        vec![0u8; sizes.right],
    )
}

#[test]
fn expected_sizes_account_for_the_header() {
    let sizes = expected_sizes(&LAYOUT);
    assert_eq!(sizes.common, 4 * (HEADER_WORDS as usize + 2));
    assert_eq!(sizes.left, 4 * 5);
    assert_eq!(sizes.right, sizes.left);
    assert_eq!(sizes.total(), sizes.common + 2 * sizes.left);
    assert_eq!(LAYOUT.data_words(), 2 + 2 * 5);
}

/// Words are little-endian on every target.
///
/// Red mutation: `write_u32` uses `to_be_bytes`, or `read_u32` uses `from_be_bytes`.
#[test]
fn words_are_little_endian() {
    let mut bytes = [0u8; 8];
    write_u32(&mut bytes, 0, 0x1122_3344);
    assert_eq!(&bytes[0..4], &[0x44, 0x33, 0x22, 0x11]);
    write_u32(&mut bytes, 1, 0x0000_00ff);
    assert_eq!(&bytes[4..8], &[0xff, 0x00, 0x00, 0x00]);
    assert_eq!(read_u32(&bytes, 0), 0x1122_3344);
    assert_eq!(read_u32(&bytes, 1), 0x0000_00ff);
}

/// `f32` words round-trip by bit pattern, including the signed zeros and a subnormal.
#[test]
fn float_words_round_trip_by_bits() {
    let values = [
        0.0_f32,
        -0.0,
        1.0,
        -1.0,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::MAX,
        -160.0,
    ];
    let mut bytes = vec![0u8; values.len() * 4];
    for (word, value) in values.iter().enumerate() {
        write_f32(&mut bytes, word, *value);
    }
    for (word, value) in values.iter().enumerate() {
        assert_eq!(
            read_f32(&bytes, word).to_bits(),
            value.to_bits(),
            "word {word}"
        );
    }
}

#[test]
fn snapshot_then_restore_returns_every_word() {
    let (mut common, mut left, mut right) = buffers();
    let source_common = [0xdead_beef_u32, 7];
    let source_left = [1_u32, 2, 3, 4, 5];
    let source_right = [0xffff_ffff_u32, 0, 0x8000_0000, 42, 0x7f80_0000];
    snapshot(
        &LAYOUT,
        &StateWords {
            common: &source_common,
            left: &source_left,
            right: &source_right,
        },
        &mut StatePayloadOutput {
            common: &mut common,
            left: &mut left,
            right: &mut right,
        },
    )
    .expect("snapshot into correctly sized buffers");

    assert_eq!(read_u32(&common, 0), LAYOUT.version, "version word");
    assert_eq!(read_u32(&common, 1), LAYOUT.data_words(), "word count");

    let mut out_common = [0u32; 2];
    let mut out_left = [0u32; 5];
    let mut out_right = [0u32; 5];
    restore(
        &LAYOUT,
        &StatePayloadInput {
            common: &common,
            left: &left,
            right: &right,
        },
        &mut StateWordsMut {
            common: &mut out_common,
            left: &mut out_left,
            right: &mut out_right,
        },
    )
    .expect("restore from a payload this codec wrote");
    assert_eq!(out_common, source_common);
    assert_eq!(out_left, source_left);
    assert_eq!(out_right, source_right);
}

/// A payload that is one byte short, or one byte long, is rejected — both, and with the same code.
///
/// Red mutation: `validate_lengths` compares with `<` instead of `!=`, which accepts a payload
/// with trailing bytes.
#[test]
fn wrong_section_lengths_are_rejected() {
    let sizes = expected_sizes(&LAYOUT);
    for delta in [-1_isize, 1] {
        let short = sizes.common.wrapping_add_signed(delta);
        assert_eq!(
            validate_lengths(&LAYOUT, (short, sizes.left, sizes.right))
                .expect_err("common section length must be exact")
                .code,
            STATE_LENGTH_CODE
        );
        assert_eq!(
            validate_lengths(
                &LAYOUT,
                (
                    sizes.common,
                    sizes.left.wrapping_add_signed(delta),
                    sizes.right
                )
            )
            .expect_err("left section length must be exact")
            .code,
            STATE_LENGTH_CODE
        );
        assert_eq!(
            validate_lengths(
                &LAYOUT,
                (
                    sizes.common,
                    sizes.left,
                    sizes.right.wrapping_add_signed(delta)
                )
            )
            .expect_err("right section length must be exact")
            .code,
            STATE_LENGTH_CODE
        );
    }
    assert!(validate_lengths(&LAYOUT, (sizes.common, sizes.left, sizes.right)).is_ok());
}

/// A payload written by another layout version is rejected as a version error, not a length one.
#[test]
fn wrong_version_is_rejected() {
    let (mut common, _, _) = buffers();
    let older = StateLayout {
        version: LAYOUT.version - 1,
        ..LAYOUT
    };
    write_header(&older, &mut common);
    assert_eq!(
        read_header(&LAYOUT, &common)
            .expect_err("a payload from version 2 must not restore into version 3")
            .code,
        STATE_VERSION_CODE
    );
}

/// A payload whose word count disagrees with its layout is a length error even when the version
/// matches — the case a version bump alone would not catch.
#[test]
fn wrong_word_count_is_a_length_error() {
    let (mut common, _, _) = buffers();
    let wider = StateLayout {
        lane_words: LAYOUT.lane_words + 1,
        ..LAYOUT
    };
    write_header(&wider, &mut common);
    assert_eq!(
        read_header(&LAYOUT, &common)
            .expect_err("word count must match the layout")
            .code,
        STATE_LENGTH_CODE
    );
}

/// The word slices an effect hands the codec are checked too, so a mismatch between an effect's
/// own arrays and its declared layout cannot write past the end of a section.
#[test]
fn word_slice_lengths_are_checked() {
    let (mut common, mut left, mut right) = buffers();
    let short_left = [0u32; 4];
    let error = snapshot(
        &LAYOUT,
        &StateWords {
            common: &[0, 0],
            left: &short_left,
            right: &[0u32; 5],
        },
        &mut StatePayloadOutput {
            common: &mut common,
            left: &mut left,
            right: &mut right,
        },
    )
    .expect_err("a four-word lane slice does not fit a five-word layout");
    assert_eq!(error.code, STATE_LENGTH_CODE);
}

/// A rejected restore leaves the effect's words untouched.
#[test]
fn rejected_restore_writes_nothing() {
    let (mut common, left, right) = buffers();
    write_header(
        &StateLayout {
            version: 99,
            ..LAYOUT
        },
        &mut common,
    );
    let mut out_common = [7u32; 2];
    let mut out_left = [7u32; 5];
    let mut out_right = [7u32; 5];
    assert!(
        restore(
            &LAYOUT,
            &StatePayloadInput {
                common: &common,
                left: &left,
                right: &right,
            },
            &mut StateWordsMut {
                common: &mut out_common,
                left: &mut out_left,
                right: &mut out_right,
            },
        )
        .is_err()
    );
    assert_eq!(out_common, [7, 7]);
    assert_eq!(out_left, [7; 5]);
    assert_eq!(out_right, [7; 5]);
}
