//! Plugin-delay compensation: arrival times, per-edge compensation delays and tail propagation.
//!
//! Single-pass longest-path over the level-major schedule, with every add checked, then the tail
//! composition through gain (#1379 Amendment 1 H4; `docs/derivations/1379-graph-tail-composition.md`,
//! "The graph extent").
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
//!
//! The tail extent is the one place the delay line counts (#1379 D2a): after the input stops the
//! delayed strip keeps playing for `delay_samples`, so the strip's `Input` node states that delay
//! as its tail (exact rest after it, no gain, no stall), and both extents add it.

use super::*;
use crate::compile::inflate;
use crate::ids::{NodeTable, diag};

pub(crate) struct TimingResult {
    pub(crate) routes: Vec<RouteTiming>,
    pub(crate) delays: Vec<InsertedDelay>,
    pub(crate) total_delay: u64,
    pub(crate) delay_count: u64,
    pub(crate) output_latency: LatencySamples,
    pub(crate) output_tail: TailSamples,
    pub(crate) output_tail_every_peak: TailSamples,
    pub(crate) output_flush_floor: FlushFloor,
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
/// natural one. Each node's latency and tail bound are the ones its `GraphNode` declares; the two
/// tail extents and the floor are [`compose`]'s.
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
    // `comp(u -> v)` of every edge: the compensation delay PDC inserts on it.
    let mut compensation = vec![0_u64; edges.len()];
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
            compensation[index] = delay;
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
    let composed = compose(table, schedule, &incoming_by_node, &compensation, output)?;
    if let TailSamples::Finite(value) = composed.output_tail_every_peak
        && value > caps.maximum_finite_tail_samples
    {
        return Err(diag("graph.tail.limit", "$.graph"));
    }
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
        output_tail: composed.output_tail,
        output_tail_every_peak: composed.output_tail_every_peak,
        output_flush_floor: composed.output_flush_floor,
        input_arrivals: named(input_arrivals),
        raised_arrivals: named(raised_arrivals),
    })
}

/// The two output extents and the floor of #1379 Amendment 1 H4.
pub(crate) struct Composed {
    pub(crate) output_tail: TailSamples,
    pub(crate) output_tail_every_peak: TailSamples,
    pub(crate) output_flush_floor: FlushFloor,
}

/// `a + b` in millibels, checked (H4 step 9): an overflow is `graph.tail.arithmetic_overflow`.
pub(crate) fn millibel_sum(a: i64, b: i64) -> Result<i64, GraphDiagnostic> {
    a.checked_add(b)
        .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))
}

/// A gain in millibels, `Zero` as `None` (`-infinity`: absorbing in a sum, neutral in a maximum).
fn gain_mb(gain: PeakGain) -> Option<i64> {
    match gain {
        PeakGain::Zero => None,
        PeakGain::Millibels(value) => Some(i64::from(value)),
    }
}

/// `inflate(x)` plus the smallest positive `f64`: an upper bound of the exact result of one rounded
/// non-negative `f64` operation even where it lands subnormal (H4 step 6). `0` stays `0`.
fn up(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        inflate(x) + f64::from_bits(1)
    }
}

/// `10^(mb / 2000)`, rounded up, with [`math::exp`] (the same bits on every target). A value below
/// `10^-300` is stated as `10^-300` (an upper bound); a value past `f64::MAX` is `inf`.
fn linear_up(mb: i64) -> f64 {
    let mb = mb.max(-600_000) as f64;
    up(math::exp(mb * (core::f64::consts::LN_10 / 2000.0)))
}

/// A stall's linear level, rounded up; `Zero` is `0`.
fn stall_linear(stall: FlushStall) -> f64 {
    match stall {
        FlushStall::Zero => 0.0,
        FlushStall::Level(level) => linear_up(i64::from(level)),
    }
}

/// `ceil(2000 log10 n)` for an integer `n >= 1`: the path-count term of H4 step 4.
fn count_mb(n: u64) -> i64 {
    i64::from(ceil_millibels(n as f64, Rounding::Exact))
}

/// The composition values of a stated node: `(D, G_p, G_t, sigma_p, sigma_t)`.
fn stated(node: &GraphNode) -> Option<(u64, PeakGain, PeakGain, FlushStall, FlushStall)> {
    let (peak_gain, peak_stall) = node.bound.composition.peak_clause()?;
    let (TailDecay(decay), tail_gain, tail_stall) = node.bound.composition.tail_clause()?;
    Some((decay, peak_gain, tail_gain, peak_stall, tail_stall))
}

/// #1379 Amendment 1 H4: the decay extent `output_tail`, the floor `P*_graph` and the every-peak
/// extent `output_tail_every_peak`, in `O(nodes + edges)` on the control thread.
///
/// Signal edges (main and route edges) carry magnitude; a sidechain edge enters only the two time
/// recurrences (step 1). `comp` is each edge's compensation delay. A node from which the output is
/// not reachable takes no part. The proof is `docs/derivations/1379-graph-tail-composition.md`,
/// "The graph extent".
pub(crate) fn compose(
    table: &NodeTable<'_>,
    schedule: &[usize],
    incoming_by_node: &[Vec<usize>],
    comp: &[u64],
    output: usize,
) -> Result<Composed, GraphDiagnostic> {
    let count = table.len();
    let edges = table.edges();
    let ends = table.ends();
    let signal = |edge: usize| edges[edge].destination.kind != GraphPortKind::SidechainInput;
    // Which nodes reach the output, by any edge kind (step 8).
    let mut reaches = vec![false; count];
    reaches[output] = true;
    for &node in schedule.iter().rev() {
        if reaches[node] {
            for &edge in &incoming_by_node[node] {
                reaches[ends[edge].source()] = true;
            }
        }
    }
    // Step 8: an `Infinite` tail or an `Unstated` composition that reaches the output wins.
    let mut values = vec![None; count];
    for &node in schedule {
        if !reaches[node] {
            continue;
        }
        let declared = table.node(node);
        let (TailSamples::Finite(tail), Some(composition)) =
            (declared.bound.tail, stated(declared))
        else {
            return Ok(Composed {
                output_tail: TailSamples::Infinite,
                output_tail_every_peak: TailSamples::Infinite,
                output_flush_floor: FlushFloor::Unstated,
            });
        };
        values[node] = Some((tail, composition));
    }
    let value = |node: usize| values[node].expect("a node that reaches the output is stated");
    let decays = |node: usize| u64::from(value(node).1.0 > 0);
    // Step 2, backward over signal edges: `down` (`None` is `-infinity`) and `nd`.
    let mut down = vec![None::<i64>; count];
    let mut nd = vec![0_u64; count];
    down[output] = Some(0);
    for &node in schedule.iter().rev() {
        if !reaches[node] {
            continue;
        }
        let (_, (_, _, tail_gain, _, _)) = value(node);
        let through = match (down[node], gain_mb(tail_gain)) {
            (Some(below), Some(gain)) => Some(millibel_sum(below, gain)?),
            _ => None,
        };
        let count_through = nd[node] + decays(node);
        for &edge in incoming_by_node[node].iter().filter(|&&edge| signal(edge)) {
            let source = ends[edge].source();
            down[source] = down[source].max(through);
            nd[source] = nd[source].max(count_through);
        }
    }
    // Steps 3-6, forward: `up`, `nu`, `k`, the decay extent, `S_in`, `S_out` and the stall sum.
    let mut up_mb = vec![None::<i64>; count];
    let mut nu = vec![0_u64; count];
    let mut extent = vec![0_u64; count];
    let mut s_in = vec![0.0_f64; count];
    let mut s_out = vec![0.0_f64; count];
    let mut sigma = 0.0_f64;
    let three_quarters_eps = up(0.75 * math::tail::TAIL_FLOOR);
    for &node in schedule {
        if !reaches[node] {
            continue;
        }
        let (tail, (decay, peak_gain, _, peak_stall, tail_stall)) = value(node);
        let incoming = &incoming_by_node[node];
        let mut fan_in = 0_usize;
        let mut above: Option<Option<i64>> = None;
        let mut above_count = 0_u64;
        let mut stall_in = 0.0_f64;
        for &edge in incoming.iter().filter(|&&edge| signal(edge)) {
            fan_in += 1;
            let source = ends[edge].source();
            let (_, (_, source_peak, _, _, _)) = value(source);
            let path = match (up_mb[source], gain_mb(source_peak)) {
                (Some(upstream), Some(gain)) => Some(millibel_sum(upstream, gain)?),
                _ => None,
            };
            above = Some(above.map_or(path, |best| best.max(path)));
            above_count = above_count.max(nu[source] + decays(source));
            stall_in = stall_in.max(s_out[source]);
        }
        if decay > 0 && fan_in > 1 {
            // Step 4: every node with `D > 0` has one signal input; fan-in needs an amendment.
            return Err(diag("graph.internal.invariant", "$.graph"));
        }
        // `0` at a source.
        up_mb[node] = above.unwrap_or(Some(0));
        nu[node] = above_count;
        s_in[node] = stall_in;
        // Step 4: `A = up + down + ceil(2000 log10(nu + nd + 1))`, `k = max(0, ceil(A / 2000))`.
        let k = match (decay > 0, up_mb[node], down[node]) {
            (true, Some(upstream), Some(below)) => {
                let paths = nu[node]
                    .checked_add(nd[node])
                    .and_then(|total| total.checked_add(1))
                    .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
                let a = millibel_sum(millibel_sum(upstream, below)?, count_mb(paths))?;
                if a > 0 { (a + 1999) / 2000 } else { 0 }
            }
            _ => 0,
        };
        // Step 5: the decay extent.
        let mut latest = 0_u64;
        for &edge in incoming {
            let shifted = extent[ends[edge].source()]
                .checked_add(comp[edge])
                .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
            latest = latest.max(shifted);
        }
        let k = u64::try_from(k).map_err(|_| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
        extent[node] = k
            .checked_mul(decay)
            .and_then(|decades| decades.checked_add(tail))
            .and_then(|own| own.checked_add(table.node(node).latency.0))
            .and_then(|own| own.checked_add(latest))
            .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
        // Step 6: `S_out = g_p max_u S_out(u) + sigma_p`, the effective stall `sigma'` and its
        // share of `Sigma`, carried by the downstream tail gain.
        let gain = gain_mb(peak_gain).map_or(0.0, linear_up);
        s_out[node] = up(up(gain * stall_in) + stall_linear(peak_stall));
        let mut effective = stall_linear(tail_stall);
        if decay > 0 {
            let decades = linear_up(-2000 * i64::try_from(k).unwrap_or(i64::MAX / 2000));
            effective = up(effective + up(up(three_quarters_eps * decades) * stall_in));
        }
        if let Some(below) = down[node]
            && effective > 0.0
        {
            sigma = up(sigma + up(linear_up(below) * effective));
        }
    }
    let output_tail = extent[output];
    // Step 6: `P*_graph = 4 Sigma / eps`.
    let floor_linear = up(up(4.0 * sigma) / math::tail::TAIL_FLOOR);
    let any_infinite =
        !floor_linear.is_finite() || s_out.iter().chain(&s_in).any(|value| !value.is_finite());
    let floor = if sigma == 0.0 && !any_infinite {
        FlushFloor::Zero
    } else if any_infinite {
        FlushFloor::AboveRange
    } else {
        FlushFloor::Millibels(ceil_millibels(floor_linear, Rounding::Computed))
    };
    // Step 7: the rest branch.
    let every_peak = match floor {
        FlushFloor::Zero => output_tail,
        FlushFloor::Unstated => unreachable!("a stated graph states its floor"),
        FlushFloor::AboveRange | FlushFloor::Millibels(_) => {
            // The `f32` word of +24 dB, never above `10^(24/20)`.
            let plus_24 = f64::from(
                builtins::checked_fader_gain(24.0).expect("+24 dB is inside the fader domain"),
            )
            .min(math::pow(10.0, 1.2));
            let floor_mb = match floor {
                FlushFloor::Millibels(value) => Some(i64::from(value)),
                _ => None,
            };
            let mut rest = vec![0_u64; count];
            for &node in schedule {
                if !reaches[node] {
                    continue;
                }
                let RestBound::Bounded(samples) = table.node(node).bound.rest else {
                    // Step 8: an `Unstated` rest that reaches the output, with a floor not `Zero`.
                    return Ok(Composed {
                        output_tail: TailSamples::Finite(output_tail),
                        output_tail_every_peak: TailSamples::Infinite,
                        output_flush_floor: floor,
                    });
                };
                let quiet = match floor_mb {
                    None => false,
                    Some(floor_mb) => {
                        let peak = match up_mb[node] {
                            None => 0.0,
                            Some(upstream) => up(linear_up(floor_mb) * linear_up(upstream)),
                        };
                        up(peak + s_in[node]) <= plus_24
                    }
                };
                let own = if quiet {
                    samples.peak_plus_24_dbfs
                } else {
                    samples.any_sanitized_input
                };
                let mut latest = 0_u64;
                for &edge in &incoming_by_node[node] {
                    let shifted = rest[ends[edge].source()]
                        .checked_add(comp[edge])
                        .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
                    latest = latest.max(shifted);
                }
                rest[node] = own
                    .checked_add(table.node(node).latency.0)
                    .and_then(|own| own.checked_add(latest))
                    .ok_or_else(|| diag("graph.tail.arithmetic_overflow", "$.graph"))?;
            }
            output_tail.max(rest[output])
        }
    };
    Ok(Composed {
        output_tail: TailSamples::Finite(output_tail),
        output_tail_every_peak: TailSamples::Finite(every_peak),
        output_flush_floor: floor,
    })
}
