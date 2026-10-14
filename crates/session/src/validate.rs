//! Semantic validation owned by issue 004, deliberately before graph/DSP/effect resolution.
use crate::{
    Automation, AutomationSegment, AutomationShape, ConsoleEntry, ControlAddress, Diagnostic,
    DiagnosticCode, DiagnosticSet, DualMonoBuiltins, DualMonoFader, Effect, MatrixOrPan, Owner,
    ParameterChannel, ParameterUnit, ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM, ROUTE_GAIN_DB_MAXIMUM,
    ROUTE_GAIN_DB_MINIMUM, Rack, RouteDestination, RouteSource, SESSION_SCHEMA_VERSION_V1, Section,
    SessionModel, Source, StableId, Submix, Track,
    diagnostic::{MAXIMUM_SESSION_DIAGNOSTICS, PathRef},
};
use engine::{SampleRateHz, is_launch_sample_rate};
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy)]
enum GraphEntity<'a> {
    Track(&'a Track),
    Submix(&'a Submix),
    Output,
    /// A VCA group (#1240), by its declared index in `vcas`. It is never a route endpoint, a
    /// sidechain source or an automation target; only a VCA's `members` accept it.
    Vca(usize),
}
struct Index<'a> {
    sources: HashMap<&'a str, &'a Source>,
    routes: HashSet<&'a str>,
    graph: HashMap<&'a str, GraphEntity<'a>>,
}
#[derive(Default)]
struct LocalUniqueness<'a> {
    effect_ids: HashSet<&'a str>,
    parameters: HashSet<(u32, u8)>,
}
pub(crate) fn validate_session(session: &SessionModel) -> Result<(), DiagnosticSet> {
    let mut diagnostics = Vec::new();
    let root = PathRef::ROOT;
    if session.schema_version != SESSION_SCHEMA_VERSION_V1 {
        error(
            &mut diagnostics,
            DiagnosticCode::VersionUnsupported,
            &root.key("schema_version"),
            "only version 1 is accepted",
        );
    }
    if session.revision > crate::MAX_SESSION_REVISION {
        error(
            &mut diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            &root.key("revision"),
            "revision must not exceed 9223372036854775807 (2^63 - 1), the plan revision gate's ceiling",
        );
    }
    if !is_launch_sample_rate(SampleRateHz(session.sample_rate_hz)) {
        error(
            &mut diagnostics,
            DiagnosticCode::SampleRateUnsupportedAtLaunch,
            &root.key("sample_rate_hz"),
            "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz",
        );
    }
    if session.quantum_frames == 0 {
        error(
            &mut diagnostics,
            DiagnosticCode::CapacityZero,
            &root.key("quantum_frames"),
            "quantum_frames must be nonzero",
        );
    }
    if session.output_profile.channels != 2 {
        error(
            &mut diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            &root.key("output_profile").key("channels"),
            "V1 output must contain exactly two dual-mono channels",
        );
    }

    // The live-ramp lengths (#1054): the parser's domain, kept here for a model that was built
    // rather than parsed.
    if let Some(lengths) = &session.control_smoothing {
        let path = root.key("control_smoothing");
        for (key, value) in [
            ("mute_ms", lengths.mute_ms),
            ("fader_ms", lengths.fader_ms),
            ("pan_ms", lengths.pan_ms),
        ] {
            if !(0.0..=crate::CONTROL_SMOOTHING_MS_MAXIMUM).contains(&value) {
                error(
                    &mut diagnostics,
                    DiagnosticCode::NumericOutOfSchemaRange,
                    &path.key(key),
                    "control_smoothing lengths must be finite and in [0, 1000] ms",
                );
            }
        }
    }

    // One pass establishes uniqueness and builds every global cross-reference index.
    let sources_path = root.key("sources");
    let mut sources = HashMap::with_capacity(session.sources.len());
    for (position, source) in session.sources.iter().enumerate() {
        if sources.insert(source.id.as_str(), source).is_some() {
            duplicate(&mut diagnostics, &sources_path.index(position).key("id"));
        }
    }

    let graph_capacity = session
        .tracks
        .len()
        .saturating_add(session.submixes.len())
        .saturating_add(session.outputs.len())
        .saturating_add(session.vcas.len());
    let mut graph = HashMap::with_capacity(graph_capacity);
    let tracks_path = root.key("tracks");
    for (position, track) in session.tracks.iter().enumerate() {
        if graph
            .insert(track.id.as_str(), GraphEntity::Track(track))
            .is_some()
        {
            duplicate(&mut diagnostics, &tracks_path.index(position).key("id"));
        }
    }
    let submixes_path = root.key("submixes");
    for (position, submix) in session.submixes.iter().enumerate() {
        if graph
            .insert(submix.id.as_str(), GraphEntity::Submix(submix))
            .is_some()
        {
            duplicate(&mut diagnostics, &submixes_path.index(position).key("id"));
        }
    }
    let outputs_path = root.key("outputs");
    for (position, output) in session.outputs.iter().enumerate() {
        if graph
            .insert(output.id.as_str(), GraphEntity::Output)
            .is_some()
        {
            duplicate(&mut diagnostics, &outputs_path.index(position).key("id"));
        }
    }
    // VCA IDs join the namespace (#1240 D2). They are indexed after every other entity, so a
    // collision is reported at the VCA and never displaces the strip or output it shadows.
    let vcas_path = root.key("vcas");
    for (position, vca) in session.vcas.iter().enumerate() {
        match graph.entry(vca.id.as_str()) {
            std::collections::hash_map::Entry::Occupied(_) => {
                duplicate(&mut diagnostics, &vcas_path.index(position).key("id"));
            }
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(GraphEntity::Vca(position));
            }
        }
    }

    let routes_path = root.key("routes");
    let mut route_ids = HashSet::with_capacity(session.routes.len());
    for (position, route) in session.routes.iter().enumerate() {
        if !route_ids.insert(route.id.as_str()) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &routes_path.index(position).key("id"),
                "route ID is repeated",
            );
        }
    }
    let automations_path = root.key("automation");
    let mut automation_ids = HashSet::with_capacity(session.automation.len());
    for (position, automation) in session.automation.iter().enumerate() {
        if !automation_ids.insert(automation.id.as_str()) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &automations_path.index(position).key("id"),
                "automation ID is repeated",
            );
        }
    }

    let index = Index {
        sources,
        routes: route_ids,
        graph,
    };
    let mut local = LocalUniqueness::default();
    validate_sources(session, &root, &mut diagnostics);
    validate_console(session, &root, &mut diagnostics);
    validate_tracks(session, &index, &root, &mut diagnostics, &mut local);
    validate_submixes(session, &index, &root, &mut diagnostics, &mut local);
    validate_vcas(session, &index, &root, &mut diagnostics);
    validate_routes(session, &index, &root, &mut diagnostics);
    validate_automation(session, &index, &root, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(DiagnosticSet::from_vec(diagnostics))
    }
}

fn duplicate(diagnostics: &mut Vec<Diagnostic>, path: &PathRef<'_>) {
    error(
        diagnostics,
        DiagnosticCode::DuplicateId,
        path,
        "entity ID is repeated",
    );
}

fn validate_sources(session: &SessionModel, root: &PathRef<'_>, diagnostics: &mut Vec<Diagnostic>) {
    let sources_path = root.key("sources");
    for (position, source) in session.sources.iter().enumerate() {
        let path = sources_path.index(position);
        if !valid_source_content_identity(&source.content) {
            error(
                diagnostics,
                DiagnosticCode::SourceContentIdentityFormat,
                &path.key("content"),
                "source content must match blake3:[0-9a-f]{64}",
            );
        }
        if source.channels == 0 {
            error(
                diagnostics,
                DiagnosticCode::CapacityZero,
                &path.key("channels"),
                "source channels must be nonzero",
            );
        }
        if source.frames == 0 {
            error(
                diagnostics,
                DiagnosticCode::CapacityZero,
                &path.key("frames"),
                "source frames must be nonzero",
            );
        }
    }
}

fn valid_source_content_identity(value: &str) -> bool {
    value.strip_prefix("blake3:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

/// The session console's own rules (decision 12): slot IDs are unique across both sections,
/// because a console address names the slot and not its section, and a slot is native.
///
/// Whether a native slot is on the console eligibility list is a registry question, answered
/// where native identities resolve (`effect_compiler`'s `console.slot.ineligible_effect`).
fn validate_console(session: &SessionModel, root: &PathRef<'_>, diagnostics: &mut Vec<Diagnostic>) {
    let console_path = root.key("console");
    let mut slots = HashSet::with_capacity(
        session.console.pre_insert.len() + session.console.post_insert.len(),
    );
    for (section, declared) in [
        ("pre_insert", &session.console.pre_insert),
        ("post_insert", &session.console.post_insert),
    ] {
        let section_path = console_path.key(section);
        for (position, slot) in declared.iter().enumerate() {
            let slot_path = section_path.index(position);
            if !slots.insert(slot.slot.as_str()) {
                error(
                    diagnostics,
                    DiagnosticCode::DuplicateId,
                    &slot_path.key("slot"),
                    "console slot ID is repeated; slot IDs are unique across pre_insert and \
                     post_insert",
                );
            }
            if let crate::EffectIdentity::ThirdPartyCid { .. } = slot.identity {
                error(
                    diagnostics,
                    DiagnosticCode::ConsoleSlotNotNative,
                    &slot_path.key("identity"),
                    "a console slot is a native effect; third-party code is never a console slot",
                );
            }
        }
    }
}

/// One strip's console entries against the session's slots: exactly one entry per slot, in slot
/// order (`pre_insert`, then `post_insert`), each carrying valid params. It runs for every strip,
/// track or submix, at the strip's index path (#1202 D2).
///
/// `declared` is the session's slot IDs, built once for every strip, so a strip costs one lookup
/// per entry and per slot rather than a scan of the other side (#1093 verdict L3).
fn validate_console_entries<'a>(
    session: &SessionModel,
    declared: &HashSet<&str>,
    entries: &'a [ConsoleEntry],
    path: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
    local: &mut LocalUniqueness<'a>,
) {
    let console_path = path.key("console");
    local.effect_ids.clear();
    let mut expected = session.console.slots();
    for (position, entry) in entries.iter().enumerate() {
        let entry_path = console_path.index(position);
        let slot_path = entry_path.key("slot");
        let expected_slot = expected.next();
        if !declared.contains(entry.slot.as_str()) {
            error(
                diagnostics,
                DiagnosticCode::MissingEntityReference,
                &slot_path,
                "console entry names no declared console slot",
            );
        } else if !local.effect_ids.insert(entry.slot.as_str()) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &slot_path,
                "console entry repeats a slot; a strip carries each slot exactly once",
            );
        } else if expected_slot.is_none_or(|slot| slot.slot != entry.slot) {
            error(
                diagnostics,
                DiagnosticCode::ConsoleEntryOrder,
                &slot_path,
                "console entries follow the session's slot order: pre_insert, then post_insert",
            );
        }
        validate_params(diagnostics, &entry.params, &entry_path.key("params"), local);
    }
    // Every entry that names a declared slot is in `local.effect_ids` now, so a slot missing from
    // it has no entry.
    for slot in session.console.slots() {
        if !local.effect_ids.contains(slot.slot.as_str()) {
            error(
                diagnostics,
                DiagnosticCode::ConsoleEntryMissing,
                &console_path,
                "strip console has no entry for a declared slot; every strip carries every slot",
            );
        }
    }
}

fn validate_tracks<'a>(
    session: &'a SessionModel,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
    local: &mut LocalUniqueness<'a>,
) {
    let tracks_path = root.key("tracks");
    let declared = declared_slots(session);
    for (position, track) in session.tracks.iter().enumerate() {
        let path = tracks_path.index(position);
        let source = index.sources.get(track.source_id.as_str()).copied();
        if source.is_none() {
            error(
                diagnostics,
                DiagnosticCode::MissingEntityReference,
                &path.key("source_id"),
                "track source_id is not a declared source",
            );
        }
        if let Some(source) = source {
            for (field, channel) in [
                ("left_source_channel", track.left_source_channel),
                ("right_source_channel", track.right_source_channel),
            ] {
                if channel >= source.channels {
                    error(
                        diagnostics,
                        DiagnosticCode::SourceChannelIndexOutOfRange,
                        &path.key(field),
                        "track source channel index exceeds declared source channels",
                    );
                }
            }
        }

        validate_strip(
            diagnostics,
            index,
            &path,
            StripValues {
                builtins: &track.builtins,
                inserts: &track.inserts,
                fader: &track.fader,
                matrix_or_pan: &track.matrix_or_pan,
            },
            local,
        );
        validate_console_entries(
            session,
            &declared,
            &track.console,
            &path,
            diagnostics,
            local,
        );
    }
}

/// The session's console slot IDs, for [`validate_console_entries`].
fn declared_slots(session: &SessionModel) -> HashSet<&str> {
    session
        .console
        .slots()
        .map(|slot| slot.slot.as_str())
        .collect()
}

/// Every submix strip, at its index path, with the track's strip rules (#1199 D3) and the track's
/// console-entry rules (#1202 D2).
fn validate_submixes<'a>(
    session: &'a SessionModel,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
    local: &mut LocalUniqueness<'a>,
) {
    let submixes_path = root.key("submixes");
    let declared = declared_slots(session);
    for (position, submix) in session.submixes.iter().enumerate() {
        let path = submixes_path.index(position);
        validate_strip(
            diagnostics,
            index,
            &path,
            StripValues {
                builtins: &submix.builtins,
                inserts: &submix.inserts,
                fader: &submix.fader,
                matrix_or_pan: &submix.matrix_or_pan,
            },
            local,
        );
        validate_console_entries(
            session,
            &declared,
            &submix.console,
            &path,
            diagnostics,
            local,
        );
    }
}

/// The strip half every strip shares: its input section, fader, pan or matrix, and inserts.
struct StripValues<'a> {
    builtins: &'a DualMonoBuiltins,
    inserts: &'a Rack,
    fader: &'a DualMonoFader,
    matrix_or_pan: &'a MatrixOrPan,
}

/// Validate one strip's values at `path`, the strip's index path (`$.tracks[<i>]` or
/// `$.submixes[<i>]`). The source half of a track and every strip's console entries are
/// validated by the caller.
fn validate_strip<'a>(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    path: &PathRef<'_>,
    strip: StripValues<'a>,
    local: &mut LocalUniqueness<'a>,
) {
    validate_strip_values(diagnostics, path, &strip);
    validate_rack(
        diagnostics,
        index,
        strip.inserts,
        &path.key("inserts"),
        local,
    );
}

/// The numeric half of a strip's validation: the input section, the fader and the pan or matrix.
/// It reads the strip alone, so a host that changes values can check the strip it changed without
/// the session's cross-reference index (#1613).
fn validate_strip_values(
    diagnostics: &mut Vec<Diagnostic>,
    path: &PathRef<'_>,
    strip: &StripValues<'_>,
) {
    for (channel, values) in [
        ("left", &strip.builtins.left),
        ("right", &strip.builtins.right),
    ] {
        let builtins_path = path.key("builtins");
        let channel_path = builtins_path.key(channel);
        validate_finite(diagnostics, values.trim_db, &channel_path.key("trim_db"));
        validate_nonnegative_finite(diagnostics, values.hpf_hz, &channel_path.key("hpf_hz"));
        validate_nonnegative_finite(diagnostics, values.lpf_hz, &channel_path.key("lpf_hz"));
        // A flat integer domain, checked here alongside the finite checks. The upper bound is
        // the schema's, not the DSP's: it is what bounds the ring allocation a hostile
        // session can demand, which is why it is stage-2 schema work rather than issue-007
        // Nyquist work.
        if values.delay_samples > crate::CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM {
            error(
                diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &channel_path.key("delay_samples"),
                "builtin delay_samples exceeds the schema maximum of 48000",
            );
        }
    }
    let fader_path = path.key("fader");
    validate_finite(diagnostics, strip.fader.left_db, &fader_path.key("left_db"));
    validate_finite(
        diagnostics,
        strip.fader.right_db,
        &fader_path.key("right_db"),
    );
    match *strip.matrix_or_pan {
        MatrixOrPan::Pan { left, right, .. } => {
            let pan_path = path.key("pan");
            validate_finite_range(diagnostics, left, -1.0, 1.0, &pan_path.key("left"));
            validate_finite_range(diagnostics, right, -1.0, 1.0, &pan_path.key("right"));
        }
        MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
            let matrix_path = path.key("matrix");
            for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                validate_finite(diagnostics, value, &matrix_path.key(field));
            }
        }
    }
}

fn validate_rack<'a>(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    rack: &'a Rack,
    path: &PathRef<'_>,
    local: &mut LocalUniqueness<'a>,
) {
    let effects_path = path.key("effects");
    local.effect_ids.clear();
    for (position, effect) in rack.effects.iter().enumerate() {
        let effect_path = effects_path.index(position);
        if !local.effect_ids.insert(effect.id.as_str()) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &effect_path.key("id"),
                "effect ID is repeated in a rack",
            );
        }
        validate_effect(diagnostics, index, effect, &effect_path, local);
    }
}

fn validate_effect(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    effect: &Effect,
    path: &PathRef<'_>,
    local: &mut LocalUniqueness<'_>,
) {
    if let crate::EffectIdentity::ThirdPartyCid { cid } = &effect.identity
        && cid.is_empty()
    {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            &path.key("identity").key("cid"),
            "third-party CID must be nonempty",
        );
    }
    if let crate::SidechainDeclaration::Routed(sidechain) = &effect.sidechain {
        let sidechain_path = path.key("sidechain");
        validate_route_source(
            diagnostics,
            index,
            &sidechain.source,
            &sidechain_path.key("source"),
        );
    }
    validate_params(diagnostics, &effect.params, &path.key("params"), local);
}

fn validate_params(
    diagnostics: &mut Vec<Diagnostic>,
    params: &[crate::EffectParam],
    params_path: &PathRef<'_>,
    local: &mut LocalUniqueness<'_>,
) {
    local.parameters.clear();
    for (position, parameter) in params.iter().enumerate() {
        let parameter_path = params_path.index(position);
        let channel = match parameter.channel {
            crate::ParameterChannel::Left => 0_u8,
            crate::ParameterChannel::Right => 1,
            crate::ParameterChannel::Both => 2,
        };
        if !local.parameters.insert((parameter.parameter_id, channel)) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &parameter_path.key("parameter_id"),
                "parameter ID/channel is repeated",
            );
        }
        validate_unit_value(
            diagnostics,
            parameter.value,
            parameter.unit,
            &parameter_path.key("value"),
        );
    }
}

/// A VCA's members, offset range and acyclicity (#1240 D2).
fn validate_vcas(
    session: &SessionModel,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let vcas_path = root.key("vcas");
    // Resolvable VCA-to-VCA membership edges, by declared index.
    let mut edges: Vec<Vec<usize>> = Vec::with_capacity(session.vcas.len());
    let mut seen = HashSet::new();
    for (position, vca) in session.vcas.iter().enumerate() {
        let path = vcas_path.index(position);
        validate_vca_values(diagnostics, &path, vca);
        let members_path = path.key("members");
        let mut nested = Vec::new();
        seen.clear();
        for (member_position, member) in vca.members.iter().enumerate() {
            let member_path = members_path.index(member_position);
            if !seen.insert(member.as_str()) {
                error(
                    diagnostics,
                    DiagnosticCode::DuplicateId,
                    &member_path,
                    "VCA member is repeated",
                );
                continue;
            }
            match index.graph.get(member.as_str()) {
                Some(GraphEntity::Track(_) | GraphEntity::Submix(_)) => {}
                Some(GraphEntity::Vca(target)) => nested.push(*target),
                Some(GraphEntity::Output) | None => error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &member_path,
                    "VCA member must be a declared track, submix or VCA",
                ),
            }
        }
        edges.push(nested);
    }
    for (position, on_cycle) in vcas_on_cycles(&edges).into_iter().enumerate() {
        if on_cycle {
            error(
                diagnostics,
                DiagnosticCode::VcaCycle,
                &vcas_path.index(position),
                "VCA membership must be acyclic",
            );
        }
    }
}

/// A VCA's offset range (#1240 D2): the numeric half of its validation.
fn validate_vca_values(diagnostics: &mut Vec<Diagnostic>, path: &PathRef<'_>, vca: &crate::Vca) {
    let fader_path = path.key("fader");
    for (field, value) in [
        ("left_db", vca.fader.left_db),
        ("right_db", vca.fader.right_db),
    ] {
        validate_finite_range(diagnostics, value, -144.0, 24.0, &fader_path.key(field));
    }
}

/// Whether each node lies on a cycle (a self-edge included): Tarjan's strongly connected
/// components, iterative so a deep VCA chain cannot exhaust the stack. A node is on a cycle exactly
/// when its component has more than one node or it has an edge to itself, so a diamond is not one.
fn vcas_on_cycles(edges: &[Vec<usize>]) -> Vec<bool> {
    const UNVISITED: usize = usize::MAX;
    let count = edges.len();
    let mut order = vec![UNVISITED; count];
    let mut low = vec![0; count];
    let mut on_stack = vec![false; count];
    let mut stack = Vec::new();
    let mut on_cycle = vec![false; count];
    let mut next = 0;
    // `(node, next edge to follow)`, the explicit recursion stack.
    let mut frames: Vec<(usize, usize)> = Vec::new();
    for start in 0..count {
        if order[start] != UNVISITED {
            continue;
        }
        order[start] = next;
        low[start] = next;
        next += 1;
        stack.push(start);
        on_stack[start] = true;
        frames.push((start, 0));
        while let Some(frame) = frames.last_mut() {
            let node = frame.0;
            if let Some(&target) = edges[node].get(frame.1) {
                frame.1 += 1;
                if target == node {
                    on_cycle[node] = true;
                } else if order[target] == UNVISITED {
                    order[target] = next;
                    low[target] = next;
                    next += 1;
                    stack.push(target);
                    on_stack[target] = true;
                    frames.push((target, 0));
                } else if on_stack[target] {
                    low[node] = low[node].min(order[target]);
                }
                continue;
            }
            frames.pop();
            if let Some(&(parent, _)) = frames.last() {
                low[parent] = low[parent].min(low[node]);
            }
            if low[node] == order[node] {
                let root = stack
                    .iter()
                    .rposition(|&member| member == node)
                    .expect("component root is on the stack");
                let cyclic = stack.len() - root > 1;
                for &member in &stack[root..] {
                    on_stack[member] = false;
                    on_cycle[member] |= cyclic;
                }
                stack.truncate(root);
            }
        }
    }
    on_cycle
}

/// Validates each route's endpoints, `follows_mute` and value domain. The gain domain
/// ([`ROUTE_GAIN_DB_MINIMUM`], [`ROUTE_GAIN_DB_MAXIMUM`]) and the coefficient domain
/// ([`ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`]) are the session model's constants (issue #1237); the
/// graph compiler's `route_values` reads the same constants for the lowering and every live record.
fn validate_routes(
    session: &SessionModel,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let routes_path = root.key("routes");
    for (position, route) in session.routes.iter().enumerate() {
        let path = routes_path.index(position);
        validate_route_source(diagnostics, index, &route.source, &path.key("source"));
        validate_route_destination(
            diagnostics,
            index,
            &route.destination,
            &path.key("destination"),
        );
        validate_route_values(diagnostics, &path, route);
    }
}

/// A route's numeric half of validation: the gain range, where `follows_mute` is legal and the
/// matrix coefficients (#1237, #1218).
fn validate_route_values(
    diagnostics: &mut Vec<Diagnostic>,
    path: &PathRef<'_>,
    route: &crate::Route,
) {
    validate_finite_range(
        diagnostics,
        route.gain_db,
        ROUTE_GAIN_DB_MINIMUM,
        ROUTE_GAIN_DB_MAXIMUM,
        &path.key("gain_db"),
    );
    // A route into the output is never live, so a follow there could only be prepared, and a
    // live unmute of its source would leave the strip's main route silent (#1218 D1, DESIGN
    // P11). The boolean is legal in general and illegal in this context.
    if route.follows_mute && matches!(route.destination, RouteDestination::OutputInput { .. }) {
        error(
            diagnostics,
            DiagnosticCode::InvalidEnum,
            &path.key("follows_mute"),
            "follows_mute applies only to a route into a submix",
        );
    }
    let matrix_path = path.key("channel_matrix");
    for (field, value) in [
        ("ll", route.channel_matrix.ll),
        ("lr", route.channel_matrix.lr),
        ("rl", route.channel_matrix.rl),
        ("rr", route.channel_matrix.rr),
    ] {
        validate_finite_range(
            diagnostics,
            value,
            -ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM,
            ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM,
            &matrix_path.key(field),
        );
    }
}

fn validate_route_source(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    source: &RouteSource,
    path: &PathRef<'_>,
) {
    let (valid, leaf) = match source {
        RouteSource::Track { track_id, .. } => (
            matches!(
                index.graph.get(track_id.as_str()),
                Some(GraphEntity::Track(_))
            ),
            "track_id",
        ),
        RouteSource::Submix { submix_id, .. } => (
            matches!(
                index.graph.get(submix_id.as_str()),
                Some(GraphEntity::Submix(_))
            ),
            "submix_id",
        ),
    };
    if !valid {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &path.key(leaf),
            "route source entity is not declared with the required role",
        );
    }
}

fn validate_route_destination(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    destination: &RouteDestination,
    path: &PathRef<'_>,
) {
    let (valid, leaf) = match destination {
        RouteDestination::SubmixInput { submix_id } => (
            matches!(
                index.graph.get(submix_id.as_str()),
                Some(GraphEntity::Submix(_))
            ),
            "submix_id",
        ),
        RouteDestination::OutputInput { output_id } => (
            matches!(
                index.graph.get(output_id.as_str()),
                Some(GraphEntity::Output)
            ),
            "output_id",
        ),
    };
    if !valid {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &path.key(leaf),
            "route destination entity is not declared with the required role",
        );
    }
}

/// Check that an automation target names things that exist (#1566 D5). Structure only: whether the
/// row at the address exists, is automatable and takes the lane is `controls`' rule, which reads
/// the row table this crate cannot see.
fn validate_automation_target(
    target: &ControlAddress,
    index: &Index<'_>,
    path: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let owner_path = path.key("owner_id");
    // A strip carries inserts and a console entry per slot; a route and a VCA have an own section
    // and nothing else.
    let strip = match &target.owner {
        Owner::Session => {
            error(
                diagnostics,
                DiagnosticCode::InvalidEnum,
                &path.key("owner"),
                "a session control is not automatable; the owner is a strip, a route or a VCA",
            );
            return;
        }
        Owner::Strip(id) => match index.graph.get(id.as_str()) {
            Some(GraphEntity::Track(track)) => Some((&track.inserts, track.console.as_slice())),
            Some(GraphEntity::Submix(submix)) => Some((&submix.inserts, submix.console.as_slice())),
            _ => {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &owner_path,
                    "automation target must be a declared track or submix",
                );
                return;
            }
        },
        Owner::Route(id) => {
            if !index.routes.contains(id.as_str()) {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &owner_path,
                    "automation target must be a declared route",
                );
            }
            None
        }
        Owner::Vca(id) => {
            if !matches!(index.graph.get(id.as_str()), Some(GraphEntity::Vca(_))) {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &owner_path,
                    "automation target must be a declared VCA",
                );
            }
            None
        }
    };
    let Some((inserts, console)) = strip else {
        if target.section != Section::Own {
            error(
                diagnostics,
                DiagnosticCode::InvalidEnum,
                &path.key("section"),
                "a route or VCA target uses the own section",
            );
        }
        return;
    };
    // Rack size is resource-bounded; these are intentional local searches.
    match &target.section {
        Section::Own | Section::Builtins => {}
        Section::Console(slot) => {
            if !console.iter().any(|entry| entry.slot == *slot) {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &path.key("slot_id"),
                    "console slot is absent from the strip",
                );
            }
        }
        Section::Inserts(slot) => {
            if !inserts.effects.iter().any(|effect| effect.id == *slot) {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &path.key("slot_id"),
                    "effect ID is absent from the strip's inserts",
                );
            }
        }
    }
}

fn validate_automation(
    session: &SessionModel,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let automations_path = root.key("automation");
    let mut first_ids = HashSet::with_capacity(session.automation.len());
    // The entries that reach the lane check: target accepted, `id` the first of its value.
    let mut lane_entries = Vec::with_capacity(session.automation.len());
    for (position, automation) in session.automation.iter().enumerate() {
        let path = automations_path.index(position);
        let first_of_id = first_ids.insert(automation.id.as_str());
        if automation.segments.is_empty() {
            error(
                diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &path.key("segments"),
                "automation must declare at least one segment",
            );
        }

        let before_target = diagnostics.len();
        validate_automation_target(&automation.target, index, &path.key("target"), diagnostics);
        if first_of_id && diagnostics.len() == before_target {
            lane_entries.push((position, automation));
        }

        let segments_path = path.key("segments");
        let mut previous_start = None;
        let mut previous_end = None;
        let mut ordered = true;
        for (segment_position, segment) in automation.segments.iter().enumerate() {
            let segment_path = segments_path.index(segment_position);
            if segment.start_sample >= segment.end_sample {
                ordered = false;
                error(
                    diagnostics,
                    DiagnosticCode::AutomationInvalidRange,
                    &segment_path.key("end_sample"),
                    "end_sample must be greater than start_sample",
                );
            }
            if previous_start.is_some_and(|start| segment.start_sample < start) {
                ordered = false;
                error(
                    diagnostics,
                    DiagnosticCode::AutomationOutOfOrder,
                    &segment_path.key("start_sample"),
                    "segment starts before its predecessor",
                );
            } else if previous_end.is_some_and(|end| segment.start_sample < end) {
                ordered = false;
                error(
                    diagnostics,
                    DiagnosticCode::AutomationSegmentOverlap,
                    &segment_path.key("start_sample"),
                    "segment overlaps its predecessor",
                );
            }
            previous_start = Some(segment.start_sample);
            previous_end = Some(segment.end_sample);
            validate_unit_value(
                diagnostics,
                segment.start_value,
                segment.unit,
                &segment_path.key("start_value"),
            );
            validate_unit_value(
                diagnostics,
                segment.end_value,
                segment.unit,
                &segment_path.key("end_value"),
            );
            if segment.shape == AutomationShape::Exponential {
                for (field, value) in [
                    ("start_value", segment.start_value),
                    ("end_value", segment.end_value),
                ] {
                    if value.is_finite() && value <= 0.0 {
                        error(
                            diagnostics,
                            DiagnosticCode::AutomationInvalidRange,
                            &segment_path.key(field),
                            "exponential values must be positive",
                        );
                    }
                }
            }
        }
        if ordered {
            validate_discontinuity_spacing(&automation.segments, &segments_path, diagnostics);
        }
    }
    validate_lane_uniqueness(&lane_entries, &automations_path, diagnostics);
}

/// The least grid period of stored automation, in samples (#1626 D3, note A1.4 of #1058): render
/// evaluates a moving curve on a fixed 64-sample grid, and a lane takes at most one jump in any
/// window of this length. A fixed engine constant, not a session value.
const DISCONTINUITY_SPACING_SAMPLES: u64 = 64;

/// Refuse two discontinuities of one entry closer than [`DISCONTINUITY_SPACING_SAMPLES`] (#1626
/// D3). The caller guarantees the segments are ordered and disjoint.
///
/// A discontinuity is a sample where the value, by `f32` bits, differs from the value just before
/// it (the hold rule in `docs/SESSION_SCHEMA_V1.md`). Only segment boundaries can be one: a
/// segment's start whose `start_value` differs from what the lane held before it (never the first
/// segment's, which is held before it), and the end of a `step` segment whose two values differ
/// and that no segment follows at once. Where a segment follows at once its start is the one
/// boundary at that sample, so two boundaries at one sample are one discontinuity. Of a close pair
/// the later one is named: `(sample, segment, field)`.
fn validate_discontinuity_spacing(
    segments: &[AutomationSegment],
    segments_path: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Boundaries are pushed in ascending sample order: a step's end comes before the start of
    // any later segment.
    let mut jumps: Vec<(u64, usize, &'static str)> = Vec::new();
    for (position, segment) in segments.iter().enumerate() {
        if let Some(previous) = position.checked_sub(1).map(|at| &segments[at]) {
            // Just before this start a step still holds its `start_value` when it ends here;
            // every other segment, and a step after its end, holds `end_value`.
            let held = if previous.shape == AutomationShape::Step
                && previous.end_sample == segment.start_sample
            {
                previous.start_value
            } else {
                previous.end_value
            };
            if held.to_bits() != segment.start_value.to_bits() {
                jumps.push((segment.start_sample, position, "start_sample"));
            }
        }
        let next_starts_here = segments
            .get(position + 1)
            .is_some_and(|next| next.start_sample == segment.end_sample);
        if segment.shape == AutomationShape::Step
            && !next_starts_here
            && segment.end_value.to_bits() != segment.start_value.to_bits()
        {
            jumps.push((segment.end_sample, position, "end_sample"));
        }
    }
    for pair in jumps.windows(2) {
        if pair[1].0 - pair[0].0 < DISCONTINUITY_SPACING_SAMPLES {
            error(
                diagnostics,
                DiagnosticCode::AutomationDiscontinuitySpacing,
                &segments_path.index(pair[1].1).key(pair[1].2),
                "discontinuities of one automation entry must be at least 64 samples apart",
            );
        }
    }
}

/// Refuse an automation entry whose lane an earlier entry (by `id`) already drives (#1626 D1).
///
/// A lane is one control of one owner's section; `both` overlaps both channels, `left` and `right`
/// only themselves. Of an overlapping pair the greater `id` is refused, once however many entries
/// it overlaps, so the refused entry does not depend on declared order. One sort, then one pass.
fn validate_lane_uniqueness(
    entries: &[(usize, &Automation)],
    automations_path: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    type LaneKey<'a> = (u8, Option<&'a str>, u8, Option<&'a str>, u32);
    fn key(target: &ControlAddress) -> LaneKey<'_> {
        let slot = match &target.section {
            Section::Console(slot) | Section::Inserts(slot) => Some(slot.as_str()),
            Section::Own | Section::Builtins => None,
        };
        (
            target.owner.kind().code(),
            target.owner.id().map(StableId::as_str),
            target.section.code(),
            slot,
            target.control,
        )
    }
    let mut sorted: Vec<&(usize, &Automation)> = entries.iter().collect();
    sorted.sort_by(|left, right| {
        key(&left.1.target)
            .cmp(&key(&right.1.target))
            .then_with(|| left.1.id.cmp(&right.1.id))
    });
    let mut group = None;
    let (mut left_taken, mut right_taken) = (false, false);
    for (position, automation) in sorted {
        let this = key(&automation.target);
        if group != Some(this) {
            group = Some(this);
            (left_taken, right_taken) = (false, false);
        }
        let (overlaps, now_taken) = match automation.target.lane {
            ParameterChannel::Left => (left_taken, (true, right_taken)),
            ParameterChannel::Right => (right_taken, (left_taken, true)),
            ParameterChannel::Both => (left_taken || right_taken, (true, true)),
        };
        if overlaps {
            error(
                diagnostics,
                DiagnosticCode::AutomationDuplicateLane,
                &automations_path.index(*position).key("target").key("lane"),
                "an earlier automation entry already drives this lane",
            );
        }
        (left_taken, right_taken) = now_taken;
    }
}

fn validate_unit_value(
    diagnostics: &mut Vec<Diagnostic>,
    value: f32,
    unit: ParameterUnit,
    path: &PathRef<'_>,
) {
    validate_finite(diagnostics, value, path);
    if matches!(
        unit,
        ParameterUnit::Hz
            | ParameterUnit::Milliseconds
            | ParameterUnit::Samples
            | ParameterUnit::Ratio
    ) && value.is_finite()
        && value < 0.0
    {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "unit requires a non-negative value",
        );
    }
    if unit == ParameterUnit::Samples && value.is_finite() && value.fract() != 0.0 {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "sample values must be integral",
        );
    }
}

fn validate_finite_range(
    diagnostics: &mut Vec<Diagnostic>,
    value: f32,
    minimum: f32,
    maximum: f32,
    path: &PathRef<'_>,
) {
    if !value.is_finite() || value < minimum || value > maximum {
        error(
            diagnostics,
            if value.is_finite() {
                DiagnosticCode::NumericOutOfSchemaRange
            } else {
                DiagnosticCode::NumericNonFinite
            },
            path,
            "value is not finite f32 or outside its schema-local range",
        );
    }
}

fn validate_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &PathRef<'_>) {
    if !value.is_finite() {
        error(
            diagnostics,
            DiagnosticCode::NumericNonFinite,
            path,
            "value must be finite",
        );
    }
}

fn validate_nonnegative_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &PathRef<'_>) {
    validate_finite(diagnostics, value, path);
    if value.is_finite() && value < 0.0 {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "value must be non-negative",
        );
    }
}

fn error(
    diagnostics: &mut Vec<Diagnostic>,
    code: DiagnosticCode,
    path: &PathRef<'_>,
    message: &str,
) {
    if diagnostics.len() >= MAXIMUM_SESSION_DIAGNOSTICS {
        return;
    }
    diagnostics.push(Diagnostic::at(code, path, None, message));
}

/// Whether the numeric values of one owner pass the checks `validate_session` makes of them
/// (#1613): a strip's input section, fader, pan or matrix and the parameters of its console entries
/// and inserts; a VCA's offsets; a route's gain, matrix and `follows_mute`. The owner's references
/// to other entities are not read.
pub(crate) fn owner_values_valid(owner: crate::OwnerRef<'_>) -> bool {
    let mut diagnostics = Vec::new();
    let root = PathRef::ROOT;
    let mut local = LocalUniqueness::default();
    let mut strip = |values: StripValues<'_>, console: &[ConsoleEntry]| {
        validate_strip_values(&mut diagnostics, &root, &values);
        for entry in console {
            validate_params(&mut diagnostics, &entry.params, &root, &mut local);
        }
        for effect in &values.inserts.effects {
            validate_params(&mut diagnostics, &effect.params, &root, &mut local);
        }
    };
    match owner {
        crate::OwnerRef::Track(track) => strip(
            StripValues {
                builtins: &track.builtins,
                inserts: &track.inserts,
                fader: &track.fader,
                matrix_or_pan: &track.matrix_or_pan,
            },
            &track.console,
        ),
        crate::OwnerRef::Submix(submix) => strip(
            StripValues {
                builtins: &submix.builtins,
                inserts: &submix.inserts,
                fader: &submix.fader,
                matrix_or_pan: &submix.matrix_or_pan,
            },
            &submix.console,
        ),
        crate::OwnerRef::Vca(vca) => validate_vca_values(&mut diagnostics, &root, vca),
        crate::OwnerRef::Route(route) => validate_route_values(&mut diagnostics, &root, route),
    }
    diagnostics.is_empty()
}
