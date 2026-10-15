//! The `RESPONSE_GET` frame as data: the field IDs, wire types and enum codes a client builds its
//! codec from (decision 17, C13; #1623).
//!
//! The browser SDK writes and reads this frame without a Rust codec. Before this table it copied
//! the field IDs and the grid, lane, quality and link codes by hand, and a field or code that moved
//! in `schema.rs` moved in the engine alone. The table is read from the same `schema` specs the
//! codec writes and reads with, and each code table is read from the decoder that refuses any code
//! outside it, so a client generated from the table cannot spell a field or a code the engine does
//! not.
//!
//! Only the names are written here, once, beside the spec each one labels. A name is paired with
//! its spec constant, not with a position, and [`response_frame_table`] refuses a message whose
//! names do not cover its spec in order.

use crate::{
    message_wire::{ParameterChannel, ResponseConfigurationKind, ResponseGrid},
    schema::{
        self, FieldSpec, MessageSpec, response_configuration, response_override, response_request,
        response_result,
    },
    session_wire::{parse_link, parse_quality},
    wire::MessageId,
};

/// One field of a message in the frame table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameField {
    /// The field's name, as a client spells it.
    pub name: &'static str,
    /// The field ID.
    pub id: u16,
    /// The BTLV wire type code.
    pub wire: u8,
    /// The wire type's name.
    pub wire_name: &'static str,
    /// Whether a frame must carry the field.
    pub mandatory: bool,
    /// Whether the field may repeat.
    pub repeated: bool,
    /// The name of the nested message, for a message-typed field.
    pub message: Option<&'static str>,
}

/// One message of the frame: its fields, in field-ID order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameMessage {
    /// The message's key in the table.
    pub key: &'static str,
    /// The schema's name for the message.
    pub name: &'static str,
    /// The fields, in ascending ID order.
    pub fields: Vec<FrameField>,
}

/// One allocated code of a code table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameCode {
    /// The name a client spells the code with.
    pub name: &'static str,
    /// The wire code.
    pub code: u8,
}

/// The codes a one-byte enum field carries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameCodeTable {
    /// The table's key in the table.
    pub key: &'static str,
    /// Every allocated code, ascending.
    pub codes: Vec<FrameCode>,
}

/// The `RESPONSE_GET` command: its message ID, its four messages and its code tables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseFrameTable {
    /// The command's message ID.
    pub command: u16,
    /// The request, the configuration it may carry, one override row and the reply.
    pub messages: Vec<FrameMessage>,
    /// The code tables of the enum fields.
    pub code_tables: Vec<FrameCodeTable>,
}

fn message(
    key: &'static str,
    spec: &'static MessageSpec,
    names: &[(&'static str, FieldSpec)],
) -> FrameMessage {
    assert!(
        names.len() == spec.fields.len()
            && names
                .iter()
                .zip(spec.fields)
                .all(|((_, named), field)| named.id == field.id),
        "the names of {} do not cover its spec in order",
        spec.name
    );
    FrameMessage {
        key,
        name: spec.name,
        fields: names
            .iter()
            .map(|(name, field)| FrameField {
                name,
                id: field.id,
                wire: field.wire.raw(),
                wire_name: field.wire.name(),
                mandatory: field.mandatory,
                repeated: field.repeated,
                message: field.nested.map(|nested| nested.name),
            })
            .collect(),
    }
}

/// The allocated codes of a table, read from the strict reader that refuses the rest.
fn codes<T>(reader: impl Fn(u8) -> Option<T>, name: impl Fn(T) -> &'static str) -> Vec<FrameCode> {
    (0..=u8::MAX)
        .filter_map(|code| {
            reader(code).map(|value| FrameCode {
                name: name(value),
                code,
            })
        })
        .collect()
}

const fn kind_name(kind: ResponseConfigurationKind) -> &'static str {
    match kind {
        ResponseConfigurationKind::Effect => "effect",
        ResponseConfigurationKind::InputFilters => "inputFilters",
    }
}

const fn grid_name(grid: ResponseGrid) -> &'static str {
    match grid {
        ResponseGrid::Linear => "linear",
        ResponseGrid::Logarithmic => "logarithmic",
    }
}

const fn lane_name(lane: ParameterChannel) -> &'static str {
    match lane {
        ParameterChannel::Left => "left",
        ParameterChannel::Right => "right",
        ParameterChannel::Both => "both",
    }
}

/// The `RESPONSE_GET` frame, as the codec writes and reads it.
#[must_use]
pub fn response_frame_table() -> ResponseFrameTable {
    use response_configuration as configuration;
    use response_override as row;
    use response_request as request;
    use response_result as result;
    ResponseFrameTable {
        command: MessageId::ResponseGet as u16,
        messages: vec![
            message(
                "request",
                &request::SPEC,
                &[
                    ("configuration", request::CONFIGURATION),
                    ("address", request::ADDRESS),
                    ("stripId", request::STRIP_ID),
                    ("grid", request::GRID),
                    ("points", request::POINTS),
                    ("minimumHz", request::MINIMUM_HZ),
                    ("maximumHz", request::MAXIMUM_HZ),
                    ("channels", request::CHANNELS),
                    ("sections", request::SECTIONS),
                    ("override", request::OVERRIDE),
                    ("configurationId", request::CONFIGURATION_ID),
                    ("sampleRateHz", request::SAMPLE_RATE_HZ),
                    ("quantumFrames", request::QUANTUM_FRAMES),
                    ("maximumPreparedBytes", request::MAXIMUM_PREPARED_BYTES),
                    ("maximumStateBytes", request::MAXIMUM_STATE_BYTES),
                    ("maximumScratchBytes", request::MAXIMUM_SCRATCH_BYTES),
                    ("maximumAutomationSpans", request::MAXIMUM_AUTOMATION_SPANS),
                    ("maximumReplyBytes", request::MAXIMUM_REPLY_BYTES),
                ],
            ),
            message(
                "configuration",
                &configuration::SPEC,
                &[
                    ("kind", configuration::KIND),
                    ("effect", configuration::EFFECT),
                    ("quality", configuration::QUALITY),
                    ("linkMode", configuration::LINK_MODE),
                ],
            ),
            message(
                "override",
                &row::SPEC,
                &[
                    ("control", row::CONTROL),
                    ("lane", row::LANE),
                    ("value", row::VALUE),
                ],
            ),
            message(
                "result",
                &result::SPEC,
                &[
                    ("requestTag", result::REQUEST_TAG),
                    ("sampleRateHz", result::SAMPLE_RATE_HZ),
                    ("configurationId", result::CONFIGURATION_ID),
                    ("floorDb", result::FLOOR_DB),
                    ("bypass", result::BYPASS),
                    ("sectionCount", result::SECTION_COUNT),
                    ("enabledLeft", result::ENABLED_LEFT),
                    ("enabledRight", result::ENABLED_RIGHT),
                    ("retainedBytes", result::RETAINED_BYTES),
                    ("frequenciesHz", result::FREQUENCIES_HZ),
                    ("leftDb", result::LEFT_DB),
                    ("rightDb", result::RIGHT_DB),
                    ("sectionsLeftDb", result::SECTIONS_LEFT_DB),
                    ("sectionsRightDb", result::SECTIONS_RIGHT_DB),
                ],
            ),
        ],
        code_tables: vec![
            FrameCodeTable {
                key: "configurationKind",
                codes: codes(
                    |code| ResponseConfigurationKind::from_code(code).ok(),
                    kind_name,
                ),
            },
            FrameCodeTable {
                key: "grid",
                codes: codes(|code| ResponseGrid::from_code(code).ok(), grid_name),
            },
            FrameCodeTable {
                key: "lane",
                codes: codes(
                    |code| schema::parameter_channel_from_wire(code).ok(),
                    lane_name,
                ),
            },
            FrameCodeTable {
                key: "quality",
                codes: codes(|code| parse_quality(code).ok(), |quality| quality.token()),
            },
            FrameCodeTable {
                key: "linkMode",
                codes: codes(|code| parse_link(code).ok(), |mode| mode.token()),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session_wire::{enum_link, enum_quality};

    fn table<'a>(frame: &'a ResponseFrameTable, key: &str) -> &'a FrameCodeTable {
        frame
            .code_tables
            .iter()
            .find(|table| table.key == key)
            .unwrap_or_else(|| panic!("no code table {key}"))
    }

    fn code_of(frame: &ResponseFrameTable, key: &str, name: &str) -> u8 {
        table(frame, key)
            .codes
            .iter()
            .find(|row| row.name == name)
            .unwrap_or_else(|| panic!("no code {key}.{name}"))
            .code
    }

    /// Each name is the code the encoder writes for that variant. A table read from the decoders
    /// alone would call a swapped pair of variants consistent; the encoders are the other half.
    #[test]
    fn every_code_is_the_one_the_encoder_writes() {
        let frame = response_frame_table();
        assert_eq!(frame.command, 0x000e);
        for (name, kind) in [
            ("effect", ResponseConfigurationKind::Effect),
            ("inputFilters", ResponseConfigurationKind::InputFilters),
        ] {
            assert_eq!(code_of(&frame, "configurationKind", name), kind.code());
        }
        assert_eq!(table(&frame, "configurationKind").codes.len(), 2);
        for (name, grid) in [
            ("linear", ResponseGrid::Linear),
            ("logarithmic", ResponseGrid::Logarithmic),
        ] {
            assert_eq!(code_of(&frame, "grid", name), grid.code());
        }
        assert_eq!(table(&frame, "grid").codes.len(), 2);
        for (name, lane) in [
            ("left", ParameterChannel::Left),
            ("right", ParameterChannel::Right),
            ("both", ParameterChannel::Both),
        ] {
            assert_eq!(
                code_of(&frame, "lane", name),
                schema::parameter_channel_wire(lane)
            );
        }
        assert_eq!(table(&frame, "lane").codes.len(), 3);
        for quality in [
            session::EffectQuality::Draft,
            session::EffectQuality::Normal,
            session::EffectQuality::High,
        ] {
            assert_eq!(
                code_of(&frame, "quality", quality.token()),
                enum_quality(quality)
            );
        }
        assert_eq!(table(&frame, "quality").codes.len(), 3);
        for mode in [
            session::LinkMode::DualMono,
            session::LinkMode::Maximum,
            session::LinkMode::Average,
        ] {
            assert_eq!(code_of(&frame, "linkMode", mode.token()), enum_link(mode));
        }
        assert_eq!(table(&frame, "linkMode").codes.len(), 3);
    }
}
