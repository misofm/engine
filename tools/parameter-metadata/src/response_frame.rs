//! Build-time transcription of the `RESPONSE_GET` frame (issue #1623).
//!
//! The browser SDK writes the `RESPONSE_GET` request and reads its reply without the Rust codec.
//! Before this document it held the field IDs and the grid, lane, quality and link codes in a hand
//! copy, and the same copy refused requests the core also refuses. The frame table of the
//! `protocol` crate ([`protocol::response_frame_table`]) is read from the codec's own schema specs
//! and strict readers; this module writes it down as the SDK's source asset, and `--check`-style
//! comparison (`sdk/codegen/assets.mjs`, `response_frame_asset_is_current`) makes a hand edit or a
//! schema change that did not reach the SDK fail before a consumer can observe it.

use protocol::{FrameCodeTable, FrameField, FrameMessage, response_frame_table};

/// The emitted file name. It is an SDK source asset, not a member of the release artifact set.
pub const OUTPUT_NAME: &str = "miso-engine-v1-response-frame.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.response.frame.v1";

fn field(out: &mut String, row: &FrameField, last: bool) {
    let message = row
        .message
        .map_or_else(|| "null".to_owned(), |name| format!("\"{name}\""));
    append!(
        out,
        "        \"{}\": {{ \"id\": {}, \"wire\": {}, \"wireName\": \"{}\", \"mandatory\": {}, \"repeated\": {}, \"message\": {message} }}{}\n",
        row.name,
        row.id,
        row.wire,
        row.wire_name,
        row.mandatory,
        row.repeated,
        if last { "" } else { "," }
    );
}

fn message(out: &mut String, row: &FrameMessage, last: bool) {
    append!(out, "    \"{}\": {{\n", row.key);
    append!(out, "      \"name\": \"{}\",\n", row.name);
    out.push_str("      \"fields\": {\n");
    for (index, entry) in row.fields.iter().enumerate() {
        field(out, entry, index + 1 == row.fields.len());
    }
    out.push_str("      }\n");
    append!(out, "    }}{}\n", if last { "" } else { "," });
}

fn codes(out: &mut String, table: &FrameCodeTable, last: bool) {
    append!(out, "    \"{}\": [\n", table.key);
    for (index, row) in table.codes.iter().enumerate() {
        append!(
            out,
            "      {{ \"name\": \"{}\", \"code\": {} }}{}\n",
            row.name,
            row.code,
            if index + 1 == table.codes.len() {
                ""
            } else {
                ","
            }
        );
    }
    append!(out, "    ]{}\n", if last { "" } else { "," });
}

/// The document `--print-response-frame` writes.
#[must_use]
pub fn render() -> String {
    let frame = response_frame_table();
    let mut out = String::new();
    out.push_str("{\n");
    append!(&mut out, "  \"schema\": \"{SCHEMA}\",\n");
    append!(&mut out, "  \"command\": {},\n", frame.command);
    out.push_str("  \"messages\": {\n");
    for (index, row) in frame.messages.iter().enumerate() {
        message(&mut out, row, index + 1 == frame.messages.len());
    }
    out.push_str("  },\n  \"codes\": {\n");
    for (index, table) in frame.code_tables.iter().enumerate() {
        codes(&mut out, table, index + 1 == frame.code_tables.len());
    }
    out.push_str("  }\n}\n");
    out
}
