//! Topological order, dependency levels, cycle witnesses and output-buffer colouring.
//!
//! Levels are emitted in ascending node-ID order and the sequential schedule is their
//! concatenation -- the contract graph binding enforces (#99 F1).

use super::*;
use crate::ids::{NodeTable, port};

/// `topo`'s result: the dependency levels, and the sequential schedule (their concatenation) as
/// dense indices of the [`NodeTable`], so later passes need no ID lookup (#1384 D2).
#[derive(Debug)]
pub(crate) struct Topo {
    pub(crate) levels: Vec<DependencyLevel>,
    pub(crate) schedule: Vec<usize>,
}

pub(crate) fn topo(table: &NodeTable<'_>) -> Option<Topo> {
    // Dense indices follow GraphNodeId order, so integer ready membership preserves the
    // existing deterministic tie-break without owning graph identities in scratch state.
    let count = table.len();
    let mut degree = vec![0_u64; count];
    let mut successors = vec![Vec::<usize>::new(); count];
    let mut predecessors = vec![Vec::<usize>::new(); count];
    for ends in table.ends() {
        let (source, destination) = (ends.source(), ends.destination());
        degree[destination] = degree[destination].checked_add(1)?;
        successors[source].push(destination);
        predecessors[destination].push(source);
    }
    let mut ready: BTreeSet<usize> = degree
        .iter()
        .enumerate()
        .filter_map(|(dense, degree)| (*degree == 0).then_some(dense))
        .collect();
    let mut processed = 0_usize;
    let mut levels = Vec::<Vec<usize>>::new();
    let mut node_levels = vec![None::<u64>; count];
    while let Some(node) = ready.pop_first() {
        let level = predecessors[node]
            .iter()
            .filter_map(|&predecessor| node_levels[predecessor])
            .max()
            .map_or(Some(0), |value| value.checked_add(1))?;
        let level_index = usize::try_from(level).ok()?;
        let level_count = level_index.checked_add(1)?;
        if levels.len() < level_count {
            levels.resize_with(level_count, Vec::new);
        }
        node_levels[node] = Some(level);
        levels[level_index].push(node);
        processed += 1;
        for successor in &successors[node] {
            let degree = degree.get_mut(*successor)?;
            *degree = degree.checked_sub(1)?;
            if *degree == 0 {
                ready.insert(*successor);
            }
        }
    }
    if processed != count {
        return None;
    }

    let mut schedule = Vec::with_capacity(count);
    let levels = levels
        .into_iter()
        .enumerate()
        .map(|(level, mut dense_nodes)| {
            dense_nodes.sort_unstable();
            schedule.extend_from_slice(&dense_nodes);
            DependencyLevel {
                level: level as u64,
                nodes: dense_nodes
                    .into_iter()
                    .map(|dense| table.id(dense).clone())
                    .collect(),
            }
        })
        .collect();
    Some(Topo { levels, schedule })
}
#[cfg(test)]
pub(crate) fn cycle_witness(table: &NodeTable<'_>) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    cycle_witnesses(table).into_iter().next()
}
/// A cycle's host-visible path: the first route or sidechain edge of its witness, else its first
/// edge. A host shows only this path, so it names an edge an author wired: a witness can start on
/// a strip's chain edge, whose sealed path is the bare collection (`$.submixes`) or an effect path
/// in the internal rack spelling (`simd2`), and neither names the loop (#1200 and #1203 verdict
/// MINOR-1, #1205 verdict NIT-5).
///
/// `edge_paths[i]` is the edge from `witness[i]` to `witness[i + 1]`. A route edge's path is its
/// route's. A sidechain edge's sealed path is its strip's (`<strip>.sidechain`), so it is
/// re-spelled to name the keyed effect at its session position: an insert,
/// `<strip>.inserts.effects[id=<effect>].sidechain` (a console slot has no sidechain).
pub(crate) fn cycle_primary_path(witness: &[GraphNodeId], edge_paths: &[String]) -> String {
    for (index, path) in edge_paths.iter().enumerate() {
        if path.starts_with("$.routes[") {
            return path.clone();
        }
        if let Some(strip) = path.strip_suffix(".sidechain") {
            return match witness.get(index + 1) {
                Some(GraphNodeId::Effect(effect)) if effect.rack == RackId::Dynamic => format!(
                    "{strip}.inserts.effects[id={}].sidechain",
                    effect.effect_id.as_str()
                ),
                _ => path.clone(),
            };
        }
    }
    edge_paths
        .first()
        .cloned()
        .unwrap_or_else(|| "$".to_owned())
}
/// Every cyclic strongly connected component's witness, sorted by the component's least node ID:
/// the witness starts and ends at that node. Kosaraju over dense indices (#1384 D2); each node's
/// edge lists are sorted by edge ID, so the walk is independent of edge order.
pub(crate) fn cycle_witnesses(table: &NodeTable<'_>) -> Vec<(Vec<GraphNodeId>, Vec<String>)> {
    let edges = table.edges();
    let ends = table.ends();
    let count = table.len();
    let mut adjacency = vec![Vec::<usize>::new(); count];
    let mut reverse = vec![Vec::<usize>::new(); count];
    for (index, end) in ends.iter().enumerate() {
        adjacency[end.source()].push(index);
        reverse[end.destination()].push(index);
    }
    for outgoing in &mut adjacency {
        outgoing.sort_by(|left, right| edges[*left].id.cmp(&edges[*right].id));
    }
    for incoming in &mut reverse {
        incoming.sort_by(|left, right| edges[*left].id.cmp(&edges[*right].id));
    }

    let mut visited = vec![false; count];
    let mut finish = Vec::with_capacity(count);
    for start in 0..count {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        let mut stack = vec![(start, 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[*node];
            if *next_edge == outgoing.len() {
                finish.push(stack.pop().expect("nonempty DFS stack").0);
                continue;
            }
            let destination = ends[outgoing[*next_edge]].destination();
            *next_edge += 1;
            if !visited[destination] {
                visited[destination] = true;
                stack.push((destination, 0));
            }
        }
    }

    let mut assigned = vec![false; count];
    let mut components = Vec::new();
    for start in finish.into_iter().rev() {
        if assigned[start] {
            continue;
        }
        assigned[start] = true;
        let mut component = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            component.push(node);
            for &edge in reverse[node].iter().rev() {
                let predecessor = ends[edge].source();
                if !assigned[predecessor] {
                    assigned[predecessor] = true;
                    stack.push(predecessor);
                }
            }
        }
        // Dense order is ID order, so this is the ID sort.
        component.sort_unstable();
        let cyclic = component.len() > 1
            || adjacency[component[0]]
                .iter()
                .any(|&edge| ends[edge].destination() == component[0]);
        if cyclic {
            components.push(component);
        }
    }
    components.sort_by_key(|component| component[0]);
    let mut members = vec![false; count];
    let mut on_path = vec![false; count];
    components
        .into_iter()
        .filter_map(|component| {
            cycle_witness_in_component(table, &component, &adjacency, &mut members, &mut on_path)
        })
        .collect()
}
/// `members` and `on_path` are all-false scratch on entry and on return.
pub(crate) fn cycle_witness_in_component(
    table: &NodeTable<'_>,
    component: &[usize],
    adjacency: &[Vec<usize>],
    members: &mut [bool],
    on_path: &mut [bool],
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    let edges = table.edges();
    let ends = table.ends();
    let start = *component.first()?;
    for &node in component {
        members[node] = true;
    }
    let mut result = None;
    let mut nodes_path = vec![start];
    let mut edge_path = Vec::new();
    on_path[start] = true;
    let mut stack = vec![(start, 0usize)];
    while let Some((node, next_edge)) = stack.last_mut() {
        let outgoing = &adjacency[*node];
        while *next_edge < outgoing.len() && !members[ends[outgoing[*next_edge]].destination()] {
            *next_edge += 1;
        }
        if *next_edge == outgoing.len() {
            stack.pop();
            if let Some(removed) = nodes_path.pop() {
                on_path[removed] = false;
            }
            if !edge_path.is_empty() {
                edge_path.pop();
            }
            continue;
        }
        let edge = outgoing[*next_edge];
        *next_edge += 1;
        let destination = ends[edge].destination();
        if destination == start {
            let mut witness: Vec<GraphNodeId> = nodes_path
                .iter()
                .map(|&dense| table.id(dense).clone())
                .collect();
            witness.push(table.id(start).clone());
            let mut witness_edges: Vec<String> = edge_path
                .iter()
                .map(|&index: &usize| edges[index].path.clone())
                .collect();
            witness_edges.push(edges[edge].path.clone());
            result = Some((witness, witness_edges));
            break;
        }
        if !on_path[destination] {
            on_path[destination] = true;
            nodes_path.push(destination);
            edge_path.push(edge);
            stack.push((destination, 0));
        }
    }
    // Only the nodes still on the path are marked: every other mark was cleared when its node
    // was popped.
    for &node in &nodes_path {
        on_path[node] = false;
    }
    for &node in component {
        members[node] = false;
    }
    result
}
/// `schedule` is dense indices of `table`; a node of `table` absent from it is never a source or
/// destination of an edge.
pub(crate) fn buffer_assignments(
    table: &NodeTable<'_>,
    schedule: &[usize],
) -> Vec<BufferAssignment> {
    let mut positions = vec![usize::MAX; table.len()];
    for (position, &dense) in schedule.iter().enumerate() {
        positions[dense] = position;
    }
    let mut consumer_counts = vec![0_usize; schedule.len()];
    let mut last_consumers: Vec<_> = schedule
        .iter()
        .enumerate()
        .map(|(position, &dense)| {
            if matches!(table.id(dense), GraphNodeId::Output { .. }) {
                schedule.len()
            } else {
                position
            }
        })
        .collect();
    let mut main_input_counts = vec![0_usize; schedule.len()];
    let mut main_input_sources = vec![None; schedule.len()];
    for (edge, ends) in table.edges().iter().zip(table.ends()) {
        let source = positions[ends.source()];
        let destination = positions[ends.destination()];
        consumer_counts[source] += 1;
        last_consumers[source] = last_consumers[source].max(destination);
        if edge.destination.kind == GraphPortKind::MainInput {
            main_input_counts[destination] += 1;
            main_input_sources[destination] = Some(source);
        }
    }

    let mut next_buffer = 0_u64;
    let mut free = BTreeSet::new();
    let mut live_until = Vec::<usize>::new();
    let mut expirations = vec![Vec::<u64>::new(); schedule.len() + 1];
    let mut node_buffers = vec![0_u64; schedule.len()];
    let mut assignments = Vec::with_capacity(schedule.len());
    for (position, &dense) in schedule.iter().enumerate() {
        let node = table.id(dense);
        if position != 0 {
            for buffer in expirations[position - 1].drain(..) {
                if live_until[buffer as usize] == position - 1 {
                    free.insert(buffer);
                }
            }
        }

        let alias = is_identity_boundary(node)
            .then_some(position)
            .filter(|position| main_input_counts[*position] == 1)
            .and_then(|position| main_input_sources[position])
            .filter(|source| consumer_counts[*source] == 1)
            .map(|source| node_buffers[source]);
        let buffer_index = if let Some(buffer) = alias {
            free.remove(&buffer);
            buffer
        } else if let Some(buffer) = free.pop_first() {
            buffer
        } else {
            let buffer = next_buffer;
            next_buffer = next_buffer.checked_add(1).expect("node count fits u64");
            live_until.push(position);
            buffer
        };
        let last_consumer = last_consumers[position];
        live_until[buffer_index as usize] = last_consumer;
        expirations[last_consumer].push(buffer_index);
        node_buffers[position] = buffer_index;
        assignments.push(BufferAssignment {
            port: port(node.clone(), GraphPortKind::MainOutput),
            buffer_index,
        });
    }
    assignments
}

pub(crate) fn is_identity_boundary(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::PostSimd1 | TrackStage::PostDynamic | TrackStage::PostSimd2PreFader,
            ..
        }
    )
}
