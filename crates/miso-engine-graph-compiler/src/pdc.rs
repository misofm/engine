//! Plugin-delay compensation: arrival times, per-edge compensation delays and tail propagation.
//!
//! Single-pass longest-path over the level-major schedule, with every add checked.
//!
//! # What PDC is not: the track delay (#210 phase 2)
//!
//! PDC exists to equalize **unrequested** arrival-time skew. Everything here is computed from
//! *declared node latency*: a path that arrives late because an effect on it declares latency gets
//! every sibling path delayed to match, so a session sounds the way it was written rather than the
//! way its plugins happen to be implemented.
//!
//! A track's `builtins.*.delay_samples` is the opposite kind of object -- a time shift the session
//! **asked for**, for multi-mic alignment. If it were declared as node latency, this module would
//! faithfully insert compensating delays on every other path and cancel exactly the alignment the
//! user wanted. So it is not latency and it is not here: a track delay contributes zero to
//! `latencies`, produces no `InsertedDelay`, moves no `RouteTiming`, and changes neither
//! `total_delay` nor `delay_count` nor `output_latency`. It shares this module's *ring type and
//! kernel* (`runtime::TrackDelayLine` drives `pdc_delay_block`) and none of its *accounting*: its
//! bytes are added to the estimate's `delay_bytes` beside `total_delay * 8` rather than through
//! it, because `total_delay` is load-bearing for the counts above.
//!
//! The consequence a reader should carry away: a delayed track's **output shifts and its PDC
//! report does not**, which is what eval P2-3 pins.

use super::*;
use crate::ids::diag;

pub(crate) struct TimingResult {
    pub(crate) routes: Vec<RouteTiming>,
    pub(crate) delays: Vec<InsertedDelay>,
    pub(crate) total_delay: u64,
    pub(crate) delay_count: u64,
    pub(crate) output_latency: LatencySamples,
    pub(crate) output_tail: TailSamples,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn timings(
    schedule: &[GraphNodeId],
    edges: &[GraphEdge],
    latencies: &BTreeMap<GraphNodeId, LatencySamples>,
    tails: &BTreeMap<GraphNodeId, TailSamples>,
    caps: &GraphCompileCaps,
) -> Result<TimingResult, GraphDiagnostic> {
    let mut incoming_by_node: BTreeMap<_, Vec<_>> = schedule
        .iter()
        .cloned()
        .map(|node| (node, Vec::new()))
        .collect();
    for edge in edges {
        incoming_by_node
            .get_mut(&edge.destination.node)
            .ok_or_else(|| diag("graph.internal.invariant", &edge.path))?
            .push(edge);
    }
    let mut arrivals = BTreeMap::<GraphNodeId, u64>::new();
    let mut extents = BTreeMap::<GraphNodeId, TailSamples>::new();
    let mut total_delay: u64 = 0;
    let mut delay_count: u64 = 0;
    let mut routes = Vec::new();
    let mut delays = Vec::new();
    for node in schedule {
        let incoming = &incoming_by_node[node];
        let max = incoming
            .iter()
            .filter_map(|edge| arrivals.get(&edge.source.node).copied())
            .max()
            .unwrap_or(0);
        for edge in incoming {
            let source = arrivals.get(&edge.source.node).copied().unwrap_or(0);
            let delay = max
                .checked_sub(source)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            if delay > caps.maximum_delay_samples_per_edge {
                return Err(diag("graph.pdc.edge_limit", &edge.path));
            }
            total_delay = total_delay
                .checked_add(delay)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            if total_delay > caps.maximum_total_delay_samples {
                return Err(diag("graph.pdc.total_limit", &edge.path));
            }
            if delay > 0 {
                delay_count += 1;
                delays.push(InsertedDelay {
                    node: GraphNodeId::CompensationDelay {
                        edge_id: Box::new(edge.id.clone()),
                    },
                    edge_id: edge.id.clone(),
                    samples: LatencySamples(delay),
                });
            }
            if let GraphEdgeId::RouteDestination { route_id } = &edge.id {
                routes.push(RouteTiming {
                    route_id: route_id.clone(),
                    source_arrival: LatencySamples(source),
                    compensation_delay: LatencySamples(delay),
                    destination_arrival: LatencySamples(max),
                });
            }
        }
        let latency = latencies.get(node).copied().unwrap_or(LatencySamples(0)).0;
        arrivals.insert(
            node.clone(),
            max.checked_add(latency)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", "$.graph"))?,
        );
        let mut extent = TailSamples::Finite(0);
        for edge in incoming {
            let source_arrival = arrivals.get(&edge.source.node).copied().unwrap_or(0);
            let compensation_delay = max
                .checked_sub(source_arrival)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            extent = max_tail(
                extent,
                shifted_tail(
                    *extents
                        .get(&edge.source.node)
                        .unwrap_or(&TailSamples::Finite(0)),
                    compensation_delay,
                )?,
            );
        }
        extent = shifted_tail(extent, latency)?;
        extent = match (
            extent,
            tails.get(node).copied().unwrap_or(TailSamples::Finite(0)),
        ) {
            (TailSamples::Infinite, _) | (_, TailSamples::Infinite) => TailSamples::Infinite,
            (TailSamples::Finite(value), TailSamples::Finite(declared_tail)) => value
                .checked_add(declared_tail)
                .map(TailSamples::Finite)
                .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?,
        };
        if let TailSamples::Finite(value) = extent
            && value > caps.maximum_finite_tail_samples
        {
            return Err(diag("graph.tail.limit", "$.graph"));
        }
        extents.insert(node.clone(), extent);
    }
    routes.sort_by(|a, b| a.route_id.cmp(&b.route_id));
    delays.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
    let output = schedule
        .iter()
        .find(|node| matches!(node, GraphNodeId::Output { .. }))
        .ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?;
    let output_latency = LatencySamples(
        *arrivals
            .get(output)
            .ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?,
    );
    let output_tail = *extents
        .get(output)
        .ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?;
    Ok(TimingResult {
        routes,
        delays,
        total_delay,
        delay_count,
        output_latency,
        output_tail,
    })
}
pub(crate) fn shifted_tail(value: TailSamples, add: u64) -> Result<TailSamples, GraphDiagnostic> {
    match value {
        TailSamples::Infinite => Ok(TailSamples::Infinite),
        TailSamples::Finite(v) => v
            .checked_add(add)
            .map(TailSamples::Finite)
            .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph")),
    }
}
pub(crate) fn max_tail(a: TailSamples, b: TailSamples) -> TailSamples {
    match (a, b) {
        (TailSamples::Infinite, _) | (_, TailSamples::Infinite) => TailSamples::Infinite,
        (TailSamples::Finite(a), TailSamples::Finite(b)) => TailSamples::Finite(a.max(b)),
    }
}
