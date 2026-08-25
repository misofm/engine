//! SIMD-rack cohort planning and homogeneous-bank binding.
//!
//! One cohort former, over whole rack chains: see [`bind_rack_banks`] (#96 F1, #99 F3).

use super::*;
use crate::ids::{diag, rack_id};

/// The `RackLocationV1` a graph rack id addresses.
///
/// Total, and deliberately so. This used to return `Option`, with `RackId::Dynamic => None`, and
/// that `None` was the only reason a dynamic-rack effect never banked: the candidate loop skipped
/// the rack outright, so a native effect carrying the homogeneous-bank kernel contract ran scalar
/// at width 1 purely because of where the session had placed it.
///
/// AGENTS.md does not say that. It says a native effect *may* run track-locally in the dynamic
/// rack, and that opaque third-party Wasm is per-instance because it "breaks the known
/// homogeneous/fused SIMD bank contract". The disqualifier is **opacity, not location**, so the
/// gate now lives on identity (see [`banks_are_permitted`]) and every rack has a location.
pub(crate) const fn rack_location(rack: RackId) -> RackLocationV1 {
    match rack {
        RackId::Simd1 => RackLocationV1::Simd1,
        RackId::Simd2 => RackLocationV1::Simd2,
        RackId::Dynamic => RackLocationV1::Dynamic,
    }
}

/// Whether a session effect may ever be considered for a homogeneous bank.
///
/// The AGENTS.md boundary, made structural: "Third-party core Wasm ... is permitted only in the
/// dynamic rack: opaque per-instance Wasm breaks the known homogeneous/fused SIMD bank contract."
/// A bank binds one kernel over `width` lanes of *known* arithmetic; an opaque module supplies no
/// such kernel, so it is per-instance wherever it sits. This is checked for **every** rack, not
/// just the dynamic one, so the boundary cannot be re-opened by a future rack gaining a location.
///
/// Non-native identities cannot reach preparation at all today
/// (`effect.third_party.unavailable_at_launch`), so this predicate is currently unreachable-false
/// in a compiling session. That is exactly why it is written as a gate rather than left implicit:
/// when third-party execution does land, candidacy must already refuse it, and
/// `third_party_dynamic_effects_are_never_bank_candidates` pins that it does.
pub(crate) fn banks_are_permitted(identity: &miso_engine_session::EffectIdentity) -> bool {
    match identity {
        miso_engine_session::EffectIdentity::Native { .. } => true,
        miso_engine_session::EffectIdentity::ThirdPartyCid { .. } => false,
    }
}

/// Plan the SIMD-rack cohorts over whole rack chains, and bind every slot that can be bound.
///
/// The planner is `miso_engine_rack_compiler::plan_bank_groups` -- the single cohort planner in
/// the workspace (#96 F1). #99 F3 changes *what is handed to it*: one candidate per
/// `(track, rack)` whose slots are that track's rack program **in session order**
/// (`track.simd1.effects` / `track.simd2.effects`), not `EffectPreparedSession::entries` order,
/// which is sorted by effect id. That is AGENTS.md's cohort model -- a signature over slot
/// types/order with absent slots as identity kernels -- and it is what makes a multi-slot bank
/// expressible at all: #96's per-effect candidates carry one-slot programs, so they can only ever
/// form single-slot banks.
///
/// A slot is bound when the group is full and **every** lane runs that slot. A slot some lane
/// skips would need a per-lane bypass mask in the effect contract, which does not exist (#96 F7);
/// those members render on the per-node scalar path exactly as before.
/// Padded (non-full) groups are likewise unbound, unchanged from #96.
///
/// Level bucketing: slot `k` of every chain in a bucket sits at `level + k`, because a rack chain
/// is a path and a sidechain source never raises a chain member's level. A bank may not cross a
/// dependency level (#96 F12), so chains are bucketed by the level of their *first* slot and the
/// arithmetic is asserted rather than assumed.
pub(crate) fn bind_rack_banks(
    effects: &EffectPreparedSession,
    ids: &BTreeMap<(String, RackId, String), EffectNodeId>,
    levels: &[DependencyLevel],
    dispatch: Backend,
) -> Result<
    (
        Vec<miso_engine_graph::GraphPreparedEffectBank>,
        GraphRackBankReport,
    ),
    GraphDiagnostic,
> {
    let model = effects.session.normalized_model();
    let entry_by_key: BTreeMap<(&str, RackId, &str), &EffectPreparedEntry> = effects
        .entries
        .iter()
        .map(|entry| {
            (
                (
                    entry.track_id.as_str(),
                    rack_id(entry.rack),
                    entry.effect_id.as_str(),
                ),
                entry,
            )
        })
        .collect();

    // One chain per (track, bankable rack), in session slot order.
    let mut chains: BTreeMap<RackChainId, Vec<EffectNodeId>> = BTreeMap::new();
    let mut programs: BTreeMap<RackChainId, RackProgramV1> = BTreeMap::new();
    for track in &model.tracks {
        for (rack, declared) in [
            (RackId::Simd1, &track.simd1.effects),
            (RackId::Dynamic, &track.dynamic.effects),
            (RackId::Simd2, &track.simd2.effects),
        ] {
            let location = rack_location(rack);
            if declared.is_empty() {
                continue;
            }
            // AGENTS.md's opacity boundary, applied before a chain is even a candidate: one
            // opaque slot makes the whole chain per-node, exactly as one sidechained slot does
            // (`EffectProgramKeyV1::blocks_banking`). A bank is a single kernel over the chain
            // program, so it cannot straddle a slot it has no kernel for.
            if !declared
                .iter()
                .all(|effect| banks_are_permitted(&effect.identity))
            {
                continue;
            }
            let chain = RackChainId {
                track_id: track.id.as_str().to_owned(),
                rack,
            };
            let mut nodes = Vec::with_capacity(declared.len());
            let mut slots = Vec::with_capacity(declared.len());
            for effect in declared {
                let key = (track.id.as_str(), rack, effect.id.as_str());
                let Some(entry) = entry_by_key.get(&key).copied() else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                let Some(node) = ids.get(&(
                    track.id.as_str().to_owned(),
                    rack,
                    effect.id.as_str().to_owned(),
                )) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                nodes.push(node.clone());
                slots.push(entry.metadata.program_key());
            }
            programs.insert(chain.clone(), RackProgramV1::new(location, slots));
            chains.insert(chain, nodes);
        }
    }

    let empty = |dispatch, chains: BTreeMap<RackChainId, Vec<EffectNodeId>>| GraphRackBankReport {
        dispatch,
        plan: BankPlan {
            groups: Vec::new(),
            scalar: Vec::new(),
        },
        bound_slots: Vec::new(),
        chains,
    };
    let Some(width) = BankWidth::for_backend(dispatch) else {
        return Ok((Vec::new(), empty(dispatch, chains)));
    };

    let level_by_node: BTreeMap<_, _> = levels
        .iter()
        .flat_map(|level| {
            level
                .nodes
                .iter()
                .cloned()
                .map(move |node| (node, level.level))
        })
        .collect();

    let mut candidates_by_level: BTreeMap<u64, Vec<CohortCandidate<RackChainId>>> = BTreeMap::new();
    'chain: for (chain, nodes) in &chains {
        let Some(first) = nodes.first() else {
            continue;
        };
        let Some(level) = level_by_node
            .get(&GraphNodeId::Effect(first.clone()))
            .copied()
        else {
            continue;
        };
        // Slot k sits at level + k: a rack chain is a path, so each slot depends on the previous.
        for (offset, node) in nodes.iter().enumerate() {
            let Some(slot_level) = level_by_node
                .get(&GraphNodeId::Effect(node.clone()))
                .copied()
            else {
                return Err(diag("graph.internal.invariant", "$.effects"));
            };
            if slot_level != level + offset as u64 {
                // A connected sidechain is the one edge that feeds a chain slot from outside the
                // path, and it lifts a *later* slot past `level + k` when its source is scheduled
                // late. `level` is read from the chain's first slot, so a sidechain on slot 0
                // lifts the chain uniformly and this arithmetic still holds; it takes a sidechain
                // on slot 1 or beyond to break it. Such a slot already blocks banking (#96 F9),
                // so the chain is simply not a candidate: it renders per node and `scalar_in`
                // still reports it, because `chains` keeps it.
                //
                // This is reachable, not defensive. Opening the dynamic rack is what makes it
                // matter -- sidechained compressors live there -- and before this branch existed
                // such a session was rejected outright with `graph.internal.invariant`.
                // `a_sidechain_lifted_chain_slot_falls_back_instead_of_failing_the_compile`
                // constructs one (a two-slot dynamic chain whose second slot sidechains from
                // another track's post-matrix tap, lifting it 4 levels past the path arithmetic)
                // and asserts the lift explicitly, so this branch cannot go quietly unreachable.
                //
                // A *bankable* chain that breaks the path arithmetic is still a real invariant
                // violation and still fails the compile.
                if !programs[chain].is_bankable() {
                    continue 'chain;
                }
                return Err(diag("graph.internal.invariant", "$.effects"));
            }
        }
        candidates_by_level
            .entry(level)
            .or_default()
            .push(CohortCandidate {
                id: chain.clone(),
                program: programs[chain].clone(),
            });
    }
    let levels_in: Vec<_> = candidates_by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let plan = plan_bank_groups(&levels_in, width)
        .map_err(|_| diag("graph.effect.bank_members", "$.effects"))?;

    let mut banks = Vec::new();
    let mut bound_slots = Vec::new();
    for (group_index, group) in plan.groups.iter().enumerate() {
        if !group.is_full() {
            continue;
        }
        for slot in 0..group.program.len() {
            if group.slot_is_identity_everywhere(slot) {
                continue;
            }
            // Every lane must run this slot: the effect contract has no per-lane bypass mask
            // (#96 F7), so a bank whose lanes disagree cannot be expressed.
            if !group.active_slots.iter().all(|lane| lane[slot]) {
                continue;
            }
            // Lane `i` runs its own chain in order, so the leader slot maps to the lane's slot by
            // the rank of `slot` among that lane's active positions.
            let mut members = Vec::with_capacity(group.members.len());
            for (lane, id) in group.members.iter().enumerate() {
                let id = id.as_ref().expect("full group");
                let rank = group.active_slots[lane][..slot]
                    .iter()
                    .filter(|active| **active)
                    .count();
                let Some(node) = chains[id].get(rank) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                members.push(node.clone());
            }
            let entries: Vec<&EffectPreparedEntry> = members
                .iter()
                .map(|node| {
                    entry_by_key[&(node.track_id.as_str(), node.rack, node.effect_id.as_str())]
                })
                .collect();
            let requests: Vec<_> = entries
                .iter()
                .map(|entry| entry.bank_preparation.request())
                .collect();
            let request = PrepareEffectBankRequest {
                backend: dispatch,
                width,
                requests: &requests,
            };
            // Equal program key implies the same registry factory: the registry maps one
            // `EffectId` to one `Arc` (#96 F12), so a per-chunk `Arc::ptr_eq` scan proved nothing.
            let Some(processor) = entries[0]
                .factory
                .bind_homogeneous_bank(request)
                .map_err(|error| diag(error.code, "$.effects"))?
            else {
                continue;
            };
            if processor.metadata().width != width
                || processor.metadata().program_key != group.program[slot]
            {
                return Err(diag("graph.effect.bank_metadata", "$.effects"));
            }
            let scratch = miso_engine_rack::AoSoaScratch::new(width, effects.session.quantum().0)
                .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?;
            banks.push(miso_engine_graph::GraphPreparedEffectBank {
                members: members.clone().into_boxed_slice(),
                active_mask: group.active_mask.clone(),
                processor,
                scratch,
            });
            bound_slots.push(GraphRackBoundSlot {
                group: group_index,
                slot,
                members,
            });
        }
    }
    Ok((
        banks,
        GraphRackBankReport {
            dispatch,
            plan,
            bound_slots,
            chains,
        },
    ))
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct EffectBankResourceEstimate {
    pub(crate) bank_count: u64,
    pub(crate) scratch_samples: u64,
    pub(crate) scratch_bytes: u64,
    pub(crate) runtime_buffer_samples: u64,
    pub(crate) runtime_buffer_bytes: u64,
    pub(crate) metadata_bytes: u64,
    pub(crate) largest_allocation_bytes: u64,
}

pub(crate) fn effect_bank_resource(
    banks: &[miso_engine_graph::GraphPreparedEffectBank],
    quantum: u32,
) -> Option<EffectBankResourceEstimate> {
    let bank_count = u64::try_from(banks.len()).ok()?;
    let mut resource = EffectBankResourceEstimate {
        bank_count,
        ..EffectBankResourceEstimate::default()
    };
    let bank_array_bytes = u64::try_from(core::mem::size_of::<
        miso_engine_graph::GraphPreparedEffectBank,
    >())
    .ok()?
    .checked_mul(bank_count)?;
    resource.metadata_bytes = bank_array_bytes;
    resource.largest_allocation_bytes = bank_array_bytes;
    for bank in banks {
        let lanes = u64::from(bank.scratch.width().lanes());
        if bank.scratch.quantum() != quantum
            || u64::try_from(bank.members.len()).ok()? != lanes
            || u64::try_from(bank.active_mask.len()).ok()? != lanes
            || !bank.active_mask.iter().all(|lane| *lane)
            || bank.processor.metadata().width != bank.scratch.width()
        {
            return None;
        }
        let scratch_plane_samples = u64::from(quantum).checked_mul(lanes)?;
        let scratch_plane_bytes = scratch_plane_samples.checked_mul(4)?;
        // L and R only: the sidechain planes were never read (#96 F9).
        let scratch_samples = scratch_plane_samples.checked_mul(2)?;
        let scratch_bytes = scratch_samples.checked_mul(4)?;
        let runtime_buffer_samples = scratch_plane_samples.checked_mul(2)?;
        let runtime_buffer_bytes = runtime_buffer_samples.checked_mul(4)?;
        resource.scratch_samples = resource.scratch_samples.checked_add(scratch_samples)?;
        resource.scratch_bytes = resource.scratch_bytes.checked_add(scratch_bytes)?;
        resource.runtime_buffer_samples = resource
            .runtime_buffer_samples
            .checked_add(runtime_buffer_samples)?;
        resource.runtime_buffer_bytes = resource
            .runtime_buffer_bytes
            .checked_add(runtime_buffer_bytes)?;

        let member_array_bytes = u64::try_from(core::mem::size_of::<EffectNodeId>())
            .ok()?
            .checked_mul(lanes)?;
        // One `bool` per lane for the bank's active mask, mirroring the builtin-bank accounting.
        let active_mask_bytes = lanes;
        resource.metadata_bytes = resource
            .metadata_bytes
            .checked_add(member_array_bytes)?
            .checked_add(active_mask_bytes)?;
        resource.largest_allocation_bytes = resource
            .largest_allocation_bytes
            .max(member_array_bytes)
            .max(scratch_plane_bytes)
            .max(u64::from(quantum).checked_mul(4)?);
        for member in &bank.members {
            for id in [&member.track_id, &member.effect_id] {
                let string_bytes = u64::try_from(id.as_str().len()).ok()?;
                resource.metadata_bytes = resource.metadata_bytes.checked_add(string_bytes)?;
                resource.largest_allocation_bytes =
                    resource.largest_allocation_bytes.max(string_bytes);
            }
        }
    }
    Some(resource)
}

pub(crate) fn checked_add_effect_banks(
    estimate: &mut GraphResourceEstimate,
    resource: EffectBankResourceEstimate,
) -> Option<()> {
    let mut next = estimate.clone();
    next.effect_bank_count = next.effect_bank_count.checked_add(resource.bank_count)?;
    next.effect_bank_scratch_bytes = next
        .effect_bank_scratch_bytes
        .checked_add(resource.scratch_bytes)?;
    next.effect_bank_runtime_buffer_bytes = next
        .effect_bank_runtime_buffer_bytes
        .checked_add(resource.runtime_buffer_bytes)?;
    next.effect_bank_metadata_bytes = next
        .effect_bank_metadata_bytes
        .checked_add(resource.metadata_bytes)?;
    next.audio_buffer_samples = next
        .audio_buffer_samples
        .checked_add(resource.scratch_samples)?
        .checked_add(resource.runtime_buffer_samples)?;
    next.graph_metadata_bytes = next
        .graph_metadata_bytes
        .checked_add(resource.metadata_bytes)?;
    let retained = resource
        .scratch_bytes
        .checked_add(resource.runtime_buffer_bytes)?
        .checked_add(resource.metadata_bytes)?;
    next.incremental_plan_bytes = next.incremental_plan_bytes.checked_add(retained)?;
    next.session_plus_plan_bytes = next.session_plus_plan_bytes.checked_add(retained)?;
    next.largest_allocation_bytes = next
        .largest_allocation_bytes
        .max(resource.largest_allocation_bytes);
    *estimate = next;
    Some(())
}
