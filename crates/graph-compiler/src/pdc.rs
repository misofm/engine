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
use crate::ids::{NodeTable, diag};

pub(crate) struct TimingResult {
    pub(crate) routes: Vec<RouteTiming>,
    pub(crate) delays: Vec<InsertedDelay>,
    pub(crate) total_delay: u64,
    pub(crate) delay_count: u64,
    pub(crate) output_latency: LatencySamples,
    pub(crate) output_tail: TailSamples,
    /// Every node whose input arrival (its floored `max`) is nonzero, sorted by node ID (#1285 D2).
    pub(crate) input_arrivals: Vec<(GraphNodeId, LatencySamples)>,
    /// Every node whose floor raised its input arrival, with its latest incoming arrival, sorted
    /// by node ID (#1323 D3).
    pub(crate) raised_arrivals: Vec<(GraphNodeId, LatencySamples)>,
}

/// Arrivals, compensation and tails for `schedule`, which is dense indices of `table` (#1384 D2).
///
/// `floors` holds a lower bound on a node's input arrival (#1285 D1): a node's `max` is the larger
/// of its latest incoming arrival and its floor, and every incoming edge is compensated to that
/// `max`. A floor that names no node of `schedule` is ignored. With no floors the result is the
/// natural one. Each node's latency and tail are the ones its `GraphNode` declares.
pub(crate) fn timings(
    table: &NodeTable<'_>,
    schedule: &[usize],
    floors: &BTreeMap<GraphNodeId, LatencySamples>,
    caps: &GraphCompileCaps,
) -> Result<TimingResult, GraphDiagnostic> {
    let count = table.len();
    let edges = table.edges();
    let ends = table.ends();
    let mut in_schedule = vec![false; count];
    for &dense in schedule {
        in_schedule[dense] = true;
    }
    let mut incoming_by_node = vec![Vec::<usize>::new(); count];
    for (index, edge) in edges.iter().enumerate() {
        let destination = ends[index].destination();
        if !in_schedule[destination] {
            return Err(diag("graph.internal.invariant", &edge.path));
        }
        incoming_by_node[destination].push(index);
    }
    let mut floor_of = vec![0_u64; count];
    for (node, floor) in floors {
        if let Some(dense) = table.dense_of(node)
            && in_schedule[dense]
        {
            floor_of[dense] = floor.0;
        }
    }
    let mut arrivals = vec![None::<u64>; count];
    let mut extents = vec![None::<TailSamples>; count];
    let mut total_delay: u64 = 0;
    let mut delay_count: u64 = 0;
    let mut routes = Vec::new();
    let mut delays = Vec::new();
    let mut input_arrivals = Vec::<(usize, LatencySamples)>::new();
    let mut raised_arrivals = Vec::<(usize, LatencySamples)>::new();
    for &node in schedule {
        let incoming = &incoming_by_node[node];
        let latest = incoming
            .iter()
            .filter_map(|&edge| arrivals[ends[edge].source()])
            .max()
            .unwrap_or(0);
        let max = latest.max(floor_of[node]);
        if max > latest {
            raised_arrivals.push((node, LatencySamples(latest)));
        }
        if max > 0 {
            input_arrivals.push((node, LatencySamples(max)));
        }
        for &index in incoming {
            let edge = &edges[index];
            let source = arrivals[ends[index].source()].unwrap_or(0);
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
        let declared = table.node(node);
        let latency = declared.latency.0;
        arrivals[node] = Some(
            max.checked_add(latency)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", "$.graph"))?,
        );
        let mut extent = TailSamples::Finite(0);
        for &index in incoming {
            let edge = &edges[index];
            let source = ends[index].source();
            let source_arrival = arrivals[source].unwrap_or(0);
            let compensation_delay = max
                .checked_sub(source_arrival)
                .ok_or_else(|| diag("graph.pdc.arithmetic_overflow", &edge.path))?;
            extent = max_tail(
                extent,
                shifted_tail(
                    extents[source].unwrap_or(TailSamples::Finite(0)),
                    compensation_delay,
                )?,
            );
        }
        extent = shifted_tail(extent, latency)?;
        extent = match (extent, declared.tail) {
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
        extents[node] = Some(extent);
    }
    routes.sort_by(|a, b| a.route_id.cmp(&b.route_id));
    delays.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
    // Dense order is ID order, so these are the ID sorts.
    input_arrivals.sort_unstable_by_key(|entry| entry.0);
    raised_arrivals.sort_unstable_by_key(|entry| entry.0);
    let output = *schedule
        .iter()
        .find(|&&node| matches!(table.id(node), GraphNodeId::Output { .. }))
        .ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?;
    let output_latency = LatencySamples(
        arrivals[output].ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?,
    );
    let output_tail =
        extents[output].ok_or_else(|| diag("graph.internal.invariant", "$.outputs"))?;
    let named = |entries: Vec<(usize, LatencySamples)>| {
        entries
            .into_iter()
            .map(|(dense, arrival)| (table.id(dense).clone(), arrival))
            .collect()
    };
    Ok(TimingResult {
        routes,
        delays,
        total_delay,
        delay_count,
        output_latency,
        output_tail,
        input_arrivals: named(input_arrivals),
        raised_arrivals: named(raised_arrivals),
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
