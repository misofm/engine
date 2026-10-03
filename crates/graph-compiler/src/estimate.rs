//! The resource estimate a compile is admitted or rejected against.
//!
//! Every term is `checked_*`; `graph_metadata_bytes` computes id lengths arithmetically rather
//! than formatting them (#99 F5).

use super::*;
use crate::canonical::node_text_len;
use crate::ids::is_summing_node;
use crate::pdc::TimingResult;
use effect_contract::{BypassShunt, EffectControlLane, ObservationLane, PreparedAutomationSpan};

/// Eleven inputs because the estimate is a function of that many independent facts about the
/// compile, and bundling them into a struct would only move the argument list. Was inside `lib.rs`
/// before the #99 module split, where the crate-level allow covered it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn resource_estimate(
    quantum: u32,
    session_bytes: u64,
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    buffers: &[BufferAssignment],
    timing: &TimingResult,
    effects: &[EffectPreparedEntry],
    // `sum(delay_samples) * 4` over both lanes of every delayed strip, track or submix (#210
    // phase 2, #1201).
    track_delay_bytes: u64,
    track_delays: &[PreparedTrackDelay],
) -> Option<GraphResourceEstimate> {
    let count = |value: usize| u64::try_from(value).ok();
    let logical_nodes = count(nodes.len())?;
    let logical_edges = count(edges.len())?;
    let materialized_nodes = logical_nodes.checked_add(timing.delay_count)?;
    let materialized_edges = logical_edges.checked_add(timing.delay_count)?;
    let schedule_items = count(schedule.len())?.checked_add(timing.delay_count)?;
    let dependency_levels = count(levels.len())?;
    let mut input_counts: BTreeMap<&GraphNodeId, u64> =
        nodes.iter().map(|node| (&node.id, 0_u64)).collect();
    for edge in edges {
        let count = input_counts.get_mut(&edge.destination.node)?;
        *count = count.checked_add(1)?;
    }
    let reductions = count(
        nodes
            .iter()
            .filter(|node| is_summing_node(&node.id) && input_counts[&node.id] > 1)
            .count(),
    )?;
    let routes = count(
        nodes
            .iter()
            .filter(|node| matches!(node.id, GraphNodeId::Route { .. }))
            .count(),
    )?;
    let effect_count = count(effects.len())?;
    let maximum_inputs = input_counts.values().copied().max().unwrap_or(0);
    let quantum = u64::from(quantum);
    // Node outputs use the deterministic liveness coloring recorded in `buffers`. Edge
    // contributions remain distinct because they carry independent PDC state into reductions.
    let colored_outputs = buffers
        .iter()
        .map(|assignment| assignment.buffer_index)
        .max()
        .map_or(Some(0), |maximum| maximum.checked_add(1))?;
    let audio_buffer_samples = colored_outputs
        .checked_add(logical_edges)?
        .checked_mul(2)?
        .checked_mul(quantum)?
        .checked_add(maximum_inputs)?;
    let audio_bytes = audio_buffer_samples.checked_mul(4)?;
    // Track delay rides the existing `delay_bytes` row, and is added **beside** PDC's term rather
    // than through it. `timing.total_delay` is PDC's own accounting: it also feeds `delay_count`,
    // the materialized node and edge counts, the schedule-item count and the compile report's
    // rows, so folding a track delay into it would invent PDC nodes that do not exist and report
    // compensation the graph never inserted. The bytes are the same kind of bytes and are charged
    // once, here; the *samples* stay out of PDC entirely.
    //
    // `* 8` for PDC because one `CompensationDelay` of `n` samples is two `f32` rings of `n`.
    // Track delay is per lane, so its two rings are sized independently and the caller has already
    // summed `left * 4 + right * 4`.
    let delay_bytes = timing
        .total_delay
        .checked_mul(8)?
        .checked_add(track_delay_bytes)?;
    let mut declared_effect_bytes = 0_u64;
    for effect in effects {
        declared_effect_bytes = declared_effect_bytes
            .checked_add(effect.metadata.state_sizes.total()?)?
            .checked_add(effect.metadata.scratch_bytes)?;
    }
    let graph_metadata_bytes =
        graph_metadata_bytes(nodes, edges, schedule, levels, buffers, timing)?;
    let incremental_plan_bytes = audio_bytes
        .checked_add(delay_bytes)?
        .checked_add(declared_effect_bytes)?
        .checked_add(graph_metadata_bytes)?;
    let lane_bytes = quantum.checked_mul(4)?;
    let mut delay_lane_bytes = 0_u64;
    for delay in &timing.delays {
        delay_lane_bytes = delay_lane_bytes.max(delay.samples.0.checked_mul(4)?);
    }
    // A track-delay ring is one named allocation of one lane's samples, exactly as a PDC ring is,
    // so it participates in `largest_allocation_bytes` on the same terms and the cap that guards
    // that row guards it too.
    for delay in track_delays {
        delay_lane_bytes = delay_lane_bytes
            .max(u64::from(delay.left_samples).checked_mul(4)?)
            .max(u64::from(delay.right_samples).checked_mul(4)?);
    }
    let reduction_bytes = maximum_inputs.checked_mul(4)?;
    let largest_allocation_bytes = graph_metadata_bytes
        .max(lane_bytes)
        .max(delay_lane_bytes)
        .max(reduction_bytes);
    Some(GraphResourceEstimate {
        logical_nodes,
        materialized_nodes,
        edges: materialized_edges,
        schedule_items,
        dependency_levels,
        reductions,
        routes,
        effects: effect_count,
        audio_buffer_samples,
        total_delay_samples: timing.total_delay,
        delay_bytes,
        graph_metadata_bytes,
        declared_effect_bytes,
        effect_bank_count: 0,
        effect_bank_scratch_bytes: 0,
        effect_bank_runtime_buffer_bytes: 0,
        effect_bank_metadata_bytes: 0,
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
        largest_allocation_bytes,
        incremental_plan_bytes,
        session_plus_plan_bytes: session_bytes.checked_add(incremental_plan_bytes)?,
    })
}

/// Exact retained storage for attached effect control lanes and the live-control owners they
/// create.
///
/// Queue payload is charged from the SPSC layout helper using each lane's actual capped capacity.
/// The lane-owned target FIFO is charged once when present. Scalar lanes retain one boxed lane;
/// banked lanes are moved into one boxed `Option<EffectControlLane>` array per bank, so this
/// helper does not count the pre-bank scalar boxes a bank later consumes.
///
/// # The live-control owners (issue #1100)
///
/// A lane -- a live channel, or the channel-less lane that carries a session bypass (#1087) --
/// also makes bind wrap its effect in a live-control owner, and this charges every byte that owner
/// allocates beyond the live-control-free path, in both of its forms:
///
/// * **Per node**, `graph`'s `runtime::LiveControlEffect`: the boxed owner, which the
///   live-control-free `NodeKind::Effect` does not allocate at all; the staging window of
///   `automation_capacity` spans, for a live channel only; and the shunt, always.
/// * **Banked**, `rack::LiveControlEffectBankStage`: its growth over the `rack::EffectBankStage` it
///   replaces; the one-lane staging window and the packed window of `automation_capacity` spans
///   per lane, when any lane has a live channel; and the shunt over the whole AoSoA block, when
///   any lane has a live channel or is bypassed.
///
/// A shunt is [`BypassShunt::allocated_bytes`] at the size its owner builds it with. A channel-less
/// lane holds no window in either form, which is what lets a live-control-free session bypass bind
/// at any automation capacity. Before #1100 none of this was charged: P1's session bypass put a
/// per-node owner on every live-control-free host, and live controls already had the gap.
pub(crate) fn effect_control_resource(
    effects: &[EffectPreparedEntry],
    banks: &[GraphPreparedEffectBank],
) -> Option<GraphScalarOwnerResourceEstimate> {
    let bytes = |value: usize| u64::try_from(value).ok();
    let span_bytes = bytes(core::mem::size_of::<PreparedAutomationSpan>())?;
    // Charge the five fields retained by `graph`'s `runtime::LiveControlEffect`.
    // `bypass_resources`' allocator-observed estimate test sees the box if it
    // ever outgrows this sum.
    let live_control_effect_bytes = bytes(core::mem::size_of::<GraphPreparedEffect>())?
        .checked_add(bytes(core::mem::size_of::<Box<EffectControlLane>>())?)?
        .checked_add(bytes(core::mem::size_of::<Box<[PreparedAutomationSpan]>>())?)?
        .checked_add(bytes(core::mem::size_of::<BypassShunt>())?)?
        .checked_add(bytes(core::mem::size_of::<Option<Box<ObservationLane>>>())?)?;
    let live_control_stage_bytes = bytes(core::mem::size_of::<rack::LiveControlEffectBankStage>())?;
    let live_control_stage_growth = live_control_stage_bytes
        .saturating_sub(bytes(core::mem::size_of::<rack::EffectBankStage>())?);
    let mut total = 0_u64;
    let mut largest = 0_u64;
    let mut banked: BTreeSet<(&str, EffectRack, &str)> = BTreeSet::new();
    for bank in banks {
        for member in &bank.members {
            let rack = match member.rack {
                RackId::Simd1 => EffectRack::Simd1,
                RackId::Dynamic => EffectRack::Dynamic,
                RackId::Simd2 => EffectRack::Simd2,
            };
            banked.insert((member.track_id.as_str(), rack, member.effect_id.as_str()));
        }
    }
    for entry in effects {
        let Some(control) = entry.control.as_deref() else {
            continue;
        };
        let queue = control.retained_queue_payload()?;
        let queue_header = u64::try_from(queue.ring_header_bytes).ok()?;
        let queue_slots = u64::try_from(queue.slot_payload_bytes).ok()?;
        total = total.checked_add(queue_header)?.checked_add(queue_slots)?;
        largest = largest.max(queue_header).max(queue_slots);
        let target = u64::try_from(control.target_staging_retained_bytes()).ok()?;
        total = total.checked_add(target)?;
        largest = largest.max(target);
        let key = (
            entry.track_id.as_str(),
            entry.rack,
            entry.effect_id.as_str(),
        );
        if !banked.contains(&key) {
            let lane =
                u64::try_from(core::mem::size_of::<effect_contract::EffectControlLane>()).ok()?;
            total = total.checked_add(lane)?;
            largest = largest.max(lane);
            // The per-node live-control owner: `runtime::LiveControlEffect::new` at the render
            // quantum.
            let window = if control.has_channel() {
                u64::from(entry.metadata.automation_capacity).checked_mul(span_bytes)?
            } else {
                0
            };
            let frames = usize::try_from(entry.metadata.quantum).ok()?;
            let latency = usize::try_from(entry.metadata.latency.0).ok()?;
            let shunt = bytes(BypassShunt::allocated_bytes(frames, latency)?)?;
            let shunt_largest = bytes(BypassShunt::largest_allocation_bytes(frames, latency)?)?;
            total = total
                .checked_add(live_control_effect_bytes)?
                .checked_add(window)?
                .checked_add(shunt)?;
            largest = largest
                .max(live_control_effect_bytes)
                .max(window)
                .max(shunt_largest);
        }
    }
    // Every entry that carries a control lane, keyed once (issue #962). Asking each bank member
    // "does any entry match me and carry a control" by scanning every entry made this estimate
    // quadratic in the effect count, and so in the track count on any session with a banked
    // effect per track.
    let controlled: BTreeMap<(&str, EffectRack, &str), &EffectControlLane> = effects
        .iter()
        .filter_map(|entry| {
            let control = entry.control.as_deref()?;
            Some((
                (
                    entry.track_id.as_str(),
                    entry.rack,
                    entry.effect_id.as_str(),
                ),
                control,
            ))
        })
        .collect();
    for bank in banks {
        let mut has_control = false;
        let mut live = false;
        let mut shunted = false;
        for member in &bank.members {
            let rack = match member.rack {
                RackId::Simd1 => EffectRack::Simd1,
                RackId::Dynamic => EffectRack::Dynamic,
                RackId::Simd2 => EffectRack::Simd2,
            };
            if let Some(control) =
                controlled.get(&(member.track_id.as_str(), rack, member.effect_id.as_str()))
            {
                has_control = true;
                live |= control.has_channel();
                shunted |= control.has_channel() || control.bypassed();
            }
        }
        if has_control {
            let lane_count = bank.scratch.width().lanes();
            let lanes = u64::from(lane_count);
            let lane_array = lanes.checked_mul(
                u64::try_from(core::mem::size_of::<
                    Option<effect_contract::EffectControlLane>,
                >())
                .ok()?,
            )?;
            total = total.checked_add(lane_array)?;
            largest = largest.max(lane_array);
            // The banked live-control owner: `rack::LiveControlEffectBankStage::new`, which
            // `graph`'s `stage_for` builds in place of a plain `EffectBankStage` for any slot with
            // a lane.
            let metadata = bank.processor.metadata();
            let capacity = u64::from(metadata.program_key.automation_capacity);
            let (staging, packed) = if live {
                (
                    capacity.checked_mul(span_bytes)?,
                    capacity.checked_mul(lanes)?.checked_mul(span_bytes)?,
                )
            } else {
                (0, 0)
            };
            let (shunt, shunt_largest) = if shunted {
                let lane_count = usize::try_from(lane_count).ok()?;
                let frames = usize::try_from(bank.scratch.quantum())
                    .ok()?
                    .checked_mul(lane_count)?;
                let latency = usize::try_from(metadata.program_key.latency.0)
                    .ok()?
                    .checked_mul(lane_count)?;
                (
                    bytes(BypassShunt::allocated_bytes(frames, latency)?)?,
                    bytes(BypassShunt::largest_allocation_bytes(frames, latency)?)?,
                )
            } else {
                (0, 0)
            };
            total = total
                .checked_add(live_control_stage_growth)?
                .checked_add(staging)?
                .checked_add(packed)?
                .checked_add(shunt)?;
            largest = largest
                .max(live_control_stage_bytes)
                .max(staging)
                .max(packed)
                .max(shunt_largest);
        }
    }
    Some(GraphScalarOwnerResourceEstimate {
        total_bytes: total,
        largest_allocation_bytes: largest,
        split_pair_table_bytes: 0,
    })
}

pub(crate) fn graph_metadata_bytes(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    buffers: &[BufferAssignment],
    timing: &TimingResult,
) -> Option<u64> {
    let sized = |count: usize, bytes: usize| {
        u64::try_from(count)
            .ok()?
            .checked_mul(u64::try_from(bytes).ok()?)
    };
    let mut total = sized(nodes.len(), core::mem::size_of::<GraphNode>())?
        .checked_add(sized(edges.len(), core::mem::size_of::<GraphEdge>())?)?
        .checked_add(sized(schedule.len(), core::mem::size_of::<GraphNodeId>())?)?
        .checked_add(sized(
            levels.len(),
            core::mem::size_of::<DependencyLevel>(),
        )?)?
        .checked_add(sized(
            buffers.len(),
            core::mem::size_of::<BufferAssignment>(),
        )?)?
        .checked_add(sized(
            timing.routes.len(),
            core::mem::size_of::<RouteTiming>(),
        )?)?
        .checked_add(sized(
            timing.delays.len(),
            core::mem::size_of::<InsertedDelay>(),
        )?)?;
    // Lengths are computed arithmetically: this runs on every production compile and must not
    // allocate a `String` per node and three per edge only to read `.len()` (#99 F5).
    for node in nodes {
        total = total.checked_add(u64::try_from(node_text_len(&node.id)).ok()?)?;
    }
    for edge in edges {
        total = total
            .checked_add(u64::try_from(edge.path.len()).ok()?)?
            .checked_add(u64::try_from(node_text_len(&edge.source.node)).ok()?)?
            .checked_add(u64::try_from(node_text_len(&edge.destination.node)).ok()?)?;
    }
    Some(total)
}

pub(crate) fn estimate_fits_platform(estimate: &GraphResourceEstimate) -> bool {
    [
        estimate.materialized_nodes,
        estimate.edges,
        estimate.schedule_items,
        estimate.audio_buffer_samples,
        estimate.total_delay_samples,
        estimate.delay_bytes,
        estimate.graph_metadata_bytes,
        estimate.declared_effect_bytes,
        estimate.effect_bank_count,
        estimate.effect_bank_scratch_bytes,
        estimate.effect_bank_runtime_buffer_bytes,
        estimate.effect_bank_metadata_bytes,
        estimate.largest_allocation_bytes,
        estimate.incremental_plan_bytes,
        estimate.session_plus_plan_bytes,
    ]
    .into_iter()
    .all(|value| usize::try_from(value).is_ok() && isize::try_from(value).is_ok())
}
