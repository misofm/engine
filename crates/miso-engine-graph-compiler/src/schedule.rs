//! Topological order, dependency levels, cycle witnesses and output-buffer colouring.
//!
//! Levels are emitted in ascending node-ID order and the sequential schedule is their
//! concatenation -- the contract graph binding enforces (#99 F1).

use super::*;
use crate::ids::port;

pub(crate) fn topo(nodes: &[GraphNode], edges: &[GraphEdge]) -> Option<Vec<DependencyLevel>> {
    let mut degree: BTreeMap<_, u64> = nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    let mut successors: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    let mut predecessors: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    for edge in edges {
        *degree.get_mut(&edge.destination.node)? += 1;
        successors
            .get_mut(&edge.source.node)?
            .push(edge.destination.node.clone());
        predecessors
            .get_mut(&edge.destination.node)?
            .push(edge.source.node.clone());
    }
    let mut ready: BTreeSet<_> = degree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();
    let mut processed = 0_usize;
    let mut levels = BTreeMap::<u64, Vec<GraphNodeId>>::new();
    let mut node_levels = BTreeMap::new();
    while let Some(node) = ready.pop_first() {
        let level = predecessors[&node]
            .iter()
            .filter_map(|predecessor| node_levels.get(predecessor))
            .copied()
            .max()
            .map_or(0, |value| value + 1);
        node_levels.insert(node.clone(), level);
        levels.entry(level).or_default().push(node.clone());
        processed += 1;
        for successor in &successors[&node] {
            let degree = degree.get_mut(successor)?;
            *degree -= 1;
            if *degree == 0 {
                ready.insert(successor.clone());
            }
        }
    }
    if processed != nodes.len() {
        None
    } else {
        for nodes in levels.values_mut() {
            nodes.sort();
        }
        Some(
            levels
                .into_iter()
                .map(|(level, nodes)| DependencyLevel { level, nodes })
                .collect(),
        )
    }
}
#[cfg(test)]
pub(crate) fn cycle_witness(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    cycle_witnesses(nodes, edges).into_iter().next()
}
pub(crate) fn cycle_witnesses(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> Vec<(Vec<GraphNodeId>, Vec<String>)> {
    let mut adjacency: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    let mut reverse: BTreeMap<_, Vec<_>> = nodes
        .iter()
        .map(|node| (node.id.clone(), Vec::new()))
        .collect();
    for edge in edges {
        let Some(outgoing) = adjacency.get_mut(&edge.source.node) else {
            return Vec::new();
        };
        outgoing.push(edge);
        let Some(incoming) = reverse.get_mut(&edge.destination.node) else {
            return Vec::new();
        };
        incoming.push(edge);
    }
    for outgoing in adjacency.values_mut() {
        outgoing.sort_by(|left, right| left.id.cmp(&right.id));
    }
    for incoming in reverse.values_mut() {
        incoming.sort_by(|left, right| left.id.cmp(&right.id));
    }

    let mut visited = BTreeSet::new();
    let mut finish = Vec::with_capacity(nodes.len());
    for start in nodes.iter().map(|node| &node.id) {
        if !visited.insert(start.clone()) {
            continue;
        }
        let mut stack = vec![(start.clone(), 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[node];
            if *next_edge == outgoing.len() {
                finish.push(stack.pop().expect("nonempty DFS stack").0);
                continue;
            }
            let destination = outgoing[*next_edge].destination.node.clone();
            *next_edge += 1;
            if visited.insert(destination.clone()) {
                stack.push((destination, 0));
            }
        }
    }

    let mut assigned = BTreeSet::new();
    let mut components = Vec::new();
    for start in finish.into_iter().rev() {
        if !assigned.insert(start.clone()) {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            component.push(node.clone());
            for edge in reverse[&node].iter().rev() {
                let predecessor = edge.source.node.clone();
                if assigned.insert(predecessor.clone()) {
                    stack.push(predecessor);
                }
            }
        }
        component.sort();
        let cyclic = component.len() > 1
            || adjacency[&component[0]]
                .iter()
                .any(|edge| edge.destination.node == component[0]);
        if cyclic {
            components.push(component);
        }
    }
    components.sort_by(|left, right| left[0].cmp(&right[0]));
    components
        .into_iter()
        .filter_map(|component| cycle_witness_in_component(&component, &adjacency))
        .collect()
}
pub(crate) fn cycle_witness_in_component(
    component: &[GraphNodeId],
    adjacency: &BTreeMap<GraphNodeId, Vec<&GraphEdge>>,
) -> Option<(Vec<GraphNodeId>, Vec<String>)> {
    let members: BTreeSet<_> = component.iter().cloned().collect();
    let start = component.first()?;
    {
        let mut nodes_path = vec![start.clone()];
        let mut edge_path = Vec::new();
        let mut on_path = BTreeSet::from([start.clone()]);
        let mut stack = vec![(start.clone(), 0usize)];
        while let Some((node, next_edge)) = stack.last_mut() {
            let outgoing = &adjacency[node];
            while *next_edge < outgoing.len()
                && !members.contains(&outgoing[*next_edge].destination.node)
            {
                *next_edge += 1;
            }
            if *next_edge == outgoing.len() {
                stack.pop();
                if let Some(removed) = nodes_path.pop() {
                    on_path.remove(&removed);
                }
                if !edge_path.is_empty() {
                    edge_path.pop();
                }
                continue;
            }
            let edge = outgoing[*next_edge];
            *next_edge += 1;
            if edge.destination.node == *start {
                let mut witness = nodes_path.clone();
                witness.push(start.clone());
                let mut witness_edges = edge_path.clone();
                witness_edges.push(edge.path.clone());
                return Some((witness, witness_edges));
            }
            if on_path.insert(edge.destination.node.clone()) {
                nodes_path.push(edge.destination.node.clone());
                edge_path.push(edge.path.clone());
                stack.push((edge.destination.node.clone(), 0));
            }
        }
    }
    None
}
pub(crate) fn buffer_assignments(
    schedule: &[GraphNodeId],
    edges: &[GraphEdge],
) -> Vec<BufferAssignment> {
    let positions: BTreeMap<_, _> = schedule
        .iter()
        .cloned()
        .enumerate()
        .map(|(position, node)| (node, position))
        .collect();
    let mut consumer_counts = vec![0_usize; schedule.len()];
    let mut last_consumers: Vec<_> = schedule
        .iter()
        .enumerate()
        .map(|(position, node)| {
            if matches!(node, GraphNodeId::Output { .. }) {
                schedule.len()
            } else {
                position
            }
        })
        .collect();
    let mut main_input_counts = vec![0_usize; schedule.len()];
    let mut main_input_sources = vec![None; schedule.len()];
    for edge in edges {
        let source = positions[&edge.source.node];
        let destination = positions[&edge.destination.node];
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
    for (position, node) in schedule.iter().enumerate() {
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
