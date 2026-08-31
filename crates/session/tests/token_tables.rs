//! Exhaustive public closed-token table contract tests.

use session::{
    AutomationShape, EffectQuality, LinkMode, ParameterChannel, ParameterUnit, RackName,
    RenderMode, SampleFormat, SendTap, SourceBitDepth,
};

macro_rules! assert_map {
    ($type:ty) => {
        for (index, (value, token)) in <$type>::ALL.iter().copied().enumerate() {
            let wire = u8::try_from(index + 1).expect("small closed token table");
            assert_eq!(value.wire(), wire);
            assert_eq!(<$type>::from_wire(wire), Some(value));
            assert_eq!(value.token(), token);
            assert_eq!(<$type>::from_token(token), Some(value));
        }
        assert_eq!(<$type>::from_wire(0), None);
        assert_eq!(<$type>::from_token("not-a-v1-token"), None);
    };
}

#[test]
fn all_closed_tokens_round_trip_in_declaration_and_wire_order() {
    assert_map!(RenderMode);
    assert_map!(SampleFormat);
    assert_map!(EffectQuality);
    assert_map!(LinkMode);
    assert_map!(ParameterChannel);
    assert_map!(ParameterUnit);
    assert_map!(SendTap);
    assert_map!(RackName);
    assert_map!(AutomationShape);
}

/// `SourceBitDepth` is a closed token table like the ones above, but it carries no `ALL` or
/// `from_token`, so `assert_map!` cannot reach it and nothing was checking its wire edges.
///
/// That gap was real: widening `from_wire`'s fallthrough to hand back `Float32` for any unknown
/// code passed the whole suite. A BTLV peer could then spell an unallocated depth token and have
/// it silently decode as f32 -- the exact class of reinterpretation the closed-set doctrine
/// exists to forbid. This row makes the table's edges the contract they are for every other token.
#[test]
fn source_bit_depth_wire_table_is_closed_at_both_edges() {
    const ALL: [(SourceBitDepth, u8, &str); 3] = [
        (SourceBitDepth::Pcm16, 1, "16"),
        (SourceBitDepth::Pcm24, 2, "24"),
        (SourceBitDepth::Float32, 3, "32f"),
    ];
    for (value, wire, token) in ALL {
        assert_eq!(value.wire(), wire, "{token} wire code");
        assert_eq!(
            SourceBitDepth::from_wire(wire),
            Some(value),
            "{token} decode"
        );
        assert_eq!(value.token(), token);
    }
    // Zero is never a legal token code, and 4..=255 is the unallocated tail. Exhaustive rather
    // than sampled: a widened fallthrough is exactly the mutation that hides in an untested gap.
    assert_eq!(
        SourceBitDepth::from_wire(0),
        None,
        "zero is not a token code"
    );
    for wire in 4_u8..=u8::MAX {
        assert_eq!(
            SourceBitDepth::from_wire(wire),
            None,
            "unallocated wire code {wire} must not decode"
        );
    }
}
