//! The resource estimate a compile is admitted or rejected against.
//!
//! Every term is `checked_*`; `graph_metadata_bytes` computes id lengths arithmetically rather
//! than formatting them (#99 F5).

use super::*;
use crate::canonical::node_text_len;
use crate::pdc::TimingResult;

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
    // `sum(delay_samples) * 4` over both lanes of every delayed track (#210 phase 2).
    track_delay_bytes: u64,
    track_delays: &[PreparedTrackDelayV1],
) -> Option<GraphResourceEstimate> {
    let count = |value: usize| u64::try_from(value).ok();
    let logical_nodes = count(nodes.len())?;
    let logical_edges = count(edges.len())?;
    let materialized_nodes = logical_nodes.checked_add(timing.delay_count)?;
    let materialized_edges = logical_edges.checked_add(timing.delay_count)?;
    let schedule_items = count(schedule.len())?.checked_add(timing.delay_count)?;
    let dependency_levels = count(levels.len())?;
    let mut input_counts: BTreeMap<_, u64> =
        nodes.iter().map(|node| (node.id.clone(), 0_u64)).collect();
    for edge in edges {
        let count = input_counts.get_mut(&edge.destination.node)?;
        *count = count.checked_add(1)?;
    }
    let reductions = count(
        nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.id,
                    GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
                ) && input_counts[&node.id] > 1
            })
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
