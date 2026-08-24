//! Canonical TOML writer; it intentionally never delegates formatting to Serde or `toml` display.

use core::fmt::Write;

use crate::{
    Automation, AutomationSegment, Effect, MatrixOrPan, Rack, SessionTomlV1,
    validate::validate_session, value::write_f32,
};

/// Produce canonical V1 TOML bytes as UTF-8 text with LF line endings and one final newline.
///
/// Entity sets are sorted by stable ID. Rack effect order and automation/segment order retain
/// their declared meaning. Parameters sort by `(parameter_id, channel)`.
pub fn canonical_session_toml(session: &SessionTomlV1) -> Result<String, crate::DiagnosticSet> {
    validate_session(session)?;
    Ok(write_canonical(session))
}

pub(crate) fn write_canonical(session: &SessionTomlV1) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "schema_version = {}", session.schema_version);
    line_string(&mut output, "session_id", session.session_id.as_str());
    let _ = writeln!(output, "revision = {}", session.revision);
    let _ = writeln!(output, "sample_rate_hz = {}", session.sample_rate_hz);
    let _ = writeln!(output, "quantum_frames = {}", session.quantum_frames);
    output.push_str("render_profile = { id = ");
    output.push_str(&quoted(session.render_profile.id.as_str()));
    output.push_str(", mode = ");
    output.push_str(&quoted(session.render_profile.mode.token()));
    output.push_str(" }\n");
    output.push_str("output_profile = { id = ");
    output.push_str(&quoted(session.output_profile.id.as_str()));
    let _ = write!(
        output,
        ", channels = {}, sample_format = ",
        session.output_profile.channels
    );
    output.push_str(&quoted(session.output_profile.sample_format.token()));
    output.push_str(" }\n");
    let _ = writeln!(
        output,
        "limits = {{ pcm_ring_frames = {}, control_queue_messages = {}, memory_bytes = {} }}",
        session.limits.pcm_ring_frames,
        session.limits.control_queue_messages,
        session.limits.memory_bytes
    );
    array(
        &mut output,
        "sources",
        sorted_by_id(&session.sources),
        write_source,
    );
    array(
        &mut output,
        "tracks",
        sorted_by_id(&session.tracks),
        write_track,
    );
    array(
        &mut output,
        "submixes",
        sorted_by_id(&session.submixes),
        |output, submix| {
            output.push_str("{ id = ");
            output.push_str(&quoted(submix.id.as_str()));
            output.push_str(" }");
        },
    );
    array(
        &mut output,
        "outputs",
        sorted_by_id(&session.outputs),
        |output, item| {
            output.push_str("{ id = ");
            output.push_str(&quoted(item.id.as_str()));
            output.push_str(" }");
        },
    );
    array(
        &mut output,
        "routes",
        sorted_by_id(&session.routes),
        |output, route| {
            output.push_str("{ id = ");
            output.push_str(&quoted(route.id.as_str()));
            output.push_str(", source = ");
            write_route_source(output, &route.source);
            output.push_str(", destination = ");
            write_route_destination(output, &route.destination);
            output.push_str(", channel_matrix = { ll = ");
            let _ = write_f32(output, route.channel_matrix.ll);
            output.push_str(", lr = ");
            let _ = write_f32(output, route.channel_matrix.lr);
            output.push_str(", rl = ");
            let _ = write_f32(output, route.channel_matrix.rl);
            output.push_str(", rr = ");
            let _ = write_f32(output, route.channel_matrix.rr);
            output.push_str(" }");
            output.push_str(", gain_db = ");
            let _ = write_f32(output, route.gain_db);
            output.push_str(" }");
        },
    );
    array(
        &mut output,
        "automation",
        sorted_by_id(&session.automation),
        write_automation,
    );
    output
}

fn line_string(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push_str(" = ");
    output.push_str(&quoted(value));
    output.push('\n');
}

fn array<T>(output: &mut String, key: &str, values: Vec<&T>, write_item: impl Fn(&mut String, &T)) {
    output.push_str(key);
    output.push_str(" = [\n");
    for value in values {
        output.push_str("  ");
        write_item(output, value);
        output.push_str(",\n");
    }
    output.push_str("]\n");
}

trait HasId {
    fn id(&self) -> &crate::StableId;
}

impl HasId for crate::Source {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}
impl HasId for crate::Track {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}
impl HasId for crate::Submix {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}
impl HasId for crate::Output {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}
impl HasId for crate::Route {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}
impl HasId for crate::Automation {
    fn id(&self) -> &crate::StableId {
        &self.id
    }
}

fn sorted_by_id<T: HasId>(items: &[T]) -> Vec<&T> {
    let mut items: Vec<_> = items.iter().collect();
    items.sort_by(|left, right| left.id().cmp(right.id()));
    items
}

fn write_source(output: &mut String, source: &crate::Source) {
    output.push_str("{ id = ");
    output.push_str(&quoted(source.id.as_str()));
    output.push_str(", sample_rate_hz = ");
    let _ = write!(output, "{}", source.sample_rate_hz);
    output.push_str(", content = { identity = ");
    output.push_str(&quoted(&source.content.identity));
    output.push_str(", locator = ");
    output.push_str(&quoted(&source.content.locator));
    output.push_str(" }, mapping = { channel_count = ");
    let _ = write!(output, "{}", source.mapping.channel_count);
    output.push_str(", region = { start_sample = ");
    let _ = write!(output, "{}", source.mapping.region.start_sample);
    output.push_str(", length_samples = ");
    let _ = write!(output, "{}", source.mapping.region.length_samples);
    output.push_str(" } } }");
}

fn write_track(output: &mut String, track: &crate::Track) {
    output.push_str("{ id = ");
    output.push_str(&quoted(track.id.as_str()));
    output.push_str(", source_id = ");
    output.push_str(&quoted(track.source_id.as_str()));
    let _ = write!(
        output,
        ", left_source_channel = {}, right_source_channel = {}",
        track.left_source_channel, track.right_source_channel
    );
    output.push_str(", builtins = { left = ");
    write_builtins_channel(output, &track.builtins.left);
    output.push_str(", right = ");
    write_builtins_channel(output, &track.builtins.right);
    output.push_str(" }, simd1 = ");
    write_rack(output, &track.simd1);
    output.push_str(", dynamic = ");
    write_rack(output, &track.dynamic);
    output.push_str(", simd2 = ");
    write_rack(output, &track.simd2);
    output.push_str(", fader = { left_db = ");
    let _ = write_f32(output, track.fader.left_db);
    output.push_str(", right_db = ");
    let _ = write_f32(output, track.fader.right_db);
    output.push_str(", left_mute = ");
    output.push_str(if track.fader.left_mute {
        "true"
    } else {
        "false"
    });
    output.push_str(", right_mute = ");
    output.push_str(if track.fader.right_mute {
        "true"
    } else {
        "false"
    });
    output.push_str(" }, ");
    match track.matrix_or_pan {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => {
            output.push_str("pan = { left = ");
            let _ = write_f32(output, left);
            output.push_str(", right = ");
            let _ = write_f32(output, right);
            let _ = write!(output, ", smoothing_samples = {} }}", smoothing_samples);
        }
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => {
            output.push_str("matrix = { ll = ");
            let _ = write_f32(output, ll);
            output.push_str(", lr = ");
            let _ = write_f32(output, lr);
            output.push_str(", rl = ");
            let _ = write_f32(output, rl);
            output.push_str(", rr = ");
            let _ = write_f32(output, rr);
            let _ = write!(output, ", smoothing_samples = {} }}", smoothing_samples);
        }
    }
    output.push_str(" }");
}

fn write_builtins_channel(output: &mut String, channel: &crate::ChannelBuiltins) {
    output.push_str("{ polarity_invert = ");
    output.push_str(if channel.polarity_invert {
        "true"
    } else {
        "false"
    });
    output.push_str(", trim_db = ");
    let _ = write_f32(output, channel.trim_db);
    output.push_str(", hpf_hz = ");
    let _ = write_f32(output, channel.hpf_hz);
    output.push_str(", lpf_hz = ");
    let _ = write_f32(output, channel.lpf_hz);
    output.push_str(" }");
}

fn write_rack(output: &mut String, rack: &Rack) {
    output.push_str("{ effects = [");
    for (index, effect) in rack.effects.iter().enumerate() {
        if index != 0 {
            output.push_str(", ");
        }
        write_effect(output, effect);
    }
    output.push_str("] }");
}

fn write_effect(output: &mut String, effect: &Effect) {
    output.push_str("{ id = ");
    output.push_str(&quoted(effect.id.as_str()));
    output.push_str(", identity = ");
    match &effect.identity {
        crate::EffectIdentity::Native { effect_id } => {
            output.push_str("{ kind = \"native\", effect_id = ");
            output.push_str(&quoted(effect_id.as_str()));
            output.push_str(" }");
        }
        crate::EffectIdentity::ThirdPartyCid { cid } => {
            output.push_str("{ kind = \"cid\", cid = ");
            output.push_str(&quoted(cid));
            output.push_str(" }");
        }
    }
    output.push_str(", quality = ");
    output.push_str(&quoted(effect.quality.token()));
    output.push_str(", bypass = ");
    output.push_str(if effect.bypass { "true" } else { "false" });
    output.push_str(", link_mode = ");
    output.push_str(&quoted(effect.link_mode.token()));
    output.push_str(", params = [");
    let mut params: Vec<_> = effect.params.iter().collect();
    params.sort_by(|left, right| {
        left.parameter_id
            .cmp(&right.parameter_id)
            .then(left.channel.cmp(&right.channel))
    });
    for (index, param) in params.iter().enumerate() {
        if index != 0 {
            output.push_str(", ");
        }
        let _ = write!(output, "{{ parameter_id = {}", param.parameter_id);
        output.push_str(", channel = ");
        output.push_str(&quoted(param.channel.token()));
        output.push_str(", unit = ");
        output.push_str(&quoted(param.unit.token()));
        output.push_str(", value = ");
        let _ = write_f32(output, param.value);
        output.push_str(" }");
    }
    output.push(']');
    match &effect.sidechain {
        crate::SidechainDeclaration::None => output.push_str(", sidechain = { kind = \"none\" }"),
        crate::SidechainDeclaration::Routed(sidechain) => {
            output.push_str(", sidechain = { kind = \"routed\", source = ");
            write_route_source(output, &sidechain.source);
            output.push_str(", port_id = ");
            output.push_str(&quoted(sidechain.port_id.as_str()));
            output.push_str(" }");
        }
    }
    output.push_str(" }");
}

fn write_automation(output: &mut String, automation: &Automation) {
    output.push_str("{ id = ");
    output.push_str(&quoted(automation.id.as_str()));
    output.push_str(", target = { entity_id = ");
    output.push_str(&quoted(automation.target.entity_id.as_str()));
    output.push_str(", rack = ");
    output.push_str(&quoted(automation.target.rack.token()));
    output.push_str(", effect_id = ");
    output.push_str(&quoted(automation.target.effect_id.as_str()));
    let _ = write!(
        output,
        ", parameter_id = {}",
        automation.target.parameter_id
    );
    output.push_str(", channel = ");
    output.push_str(&quoted(automation.target.channel.token()));
    output.push_str(" }, segments = [");
    for (index, segment) in automation.segments.iter().enumerate() {
        if index != 0 {
            output.push_str(", ");
        }
        write_segment(output, segment);
    }
    output.push_str("] }");
}

fn write_segment(output: &mut String, segment: &AutomationSegment) {
    output.push_str("{ shape = ");
    output.push_str(&quoted(segment.shape.token()));
    let _ = write!(
        output,
        ", start_sample = {}, end_sample = {}",
        segment.start_sample, segment.end_sample
    );
    output.push_str(", start_value = ");
    let _ = write_f32(output, segment.start_value);
    output.push_str(", end_value = ");
    let _ = write_f32(output, segment.end_value);
    output.push_str(", unit = ");
    output.push_str(&quoted(segment.unit.token()));
    output.push_str(" }");
}

fn write_route_source(output: &mut String, source: &crate::RouteSource) {
    match source {
        crate::RouteSource::Track {
            track_id,
            tap: source_tap,
        } => {
            output.push_str("{ kind = \"track\", track_id = ");
            output.push_str(&quoted(track_id.as_str()));
            output.push_str(", tap = ");
            output.push_str(&quoted(source_tap.token()));
            output.push_str(" }");
        }
        crate::RouteSource::SubmixOutput { submix_id } => {
            output.push_str("{ kind = \"submix_output\", submix_id = ");
            output.push_str(&quoted(submix_id.as_str()));
            output.push_str(" }");
        }
    }
}

fn write_route_destination(output: &mut String, destination: &crate::RouteDestination) {
    match destination {
        crate::RouteDestination::SubmixInput { submix_id } => {
            output.push_str("{ kind = \"submix_input\", submix_id = ");
            output.push_str(&quoted(submix_id.as_str()));
            output.push_str(" }");
        }
        crate::RouteDestination::OutputInput { output_id } => {
            output.push_str("{ kind = \"output_input\", output_id = ");
            output.push_str(&quoted(output_id.as_str()));
            output.push_str(" }");
        }
    }
}

fn quoted(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\u{0C}' => output.push_str("\\f"),
            '\r' => output.push_str("\\r"),
            character if character.is_control() => {
                if u32::from(character) <= 0xffff {
                    let _ = write!(output, "\\u{:04X}", u32::from(character));
                } else {
                    let _ = write!(output, "\\U{:08X}", u32::from(character));
                }
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}
