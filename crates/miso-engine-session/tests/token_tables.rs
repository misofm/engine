//! Exhaustive public closed-token table contract tests.

use miso_engine_session::{
    AutomationShape, EffectQuality, LinkMode, ParameterChannel, ParameterUnit, RackName,
    RenderMode, SampleFormat, SendTap,
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
