//! SIMD-rack cohort planning and homogeneous-bank binding.
//!
//! One cohort former, over whole rack chains: see [`bind_rack_banks`] (#96 F1, #99 F3).

use super::*;
use crate::ids::{PreparedEffectIndex, diag, prepared_effect_node};
use rack_compiler::CohortPoolClass;

/// The `RackLocation` a graph rack id addresses.
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
pub(crate) const fn rack_location(rack: RackId) -> RackLocation {
    match rack {
        RackId::Simd1 => RackLocation::Simd1,
        RackId::Simd2 => RackLocation::Simd2,
        RackId::Dynamic => RackLocation::Dynamic,
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
pub(crate) fn banks_are_permitted(identity: &session::EffectIdentity) -> bool {
    match identity {
        session::EffectIdentity::Native { .. } => true,
        session::EffectIdentity::ThirdPartyCid { .. } => false,
    }
}

/// Plan the SIMD-rack cohorts over whole rack chains, and bind every slot that can be bound.
///
/// The planner is `rack_compiler::plan_bank_groups` -- the single cohort planner in
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
///
/// That `k` is the lane's own chain position, not the leader's slot index (issue #966). A lane
/// whose program skips an earlier leader slot runs a later one at a lower rank, so the bucket
/// aligns every lane's slot 0 and nothing after it. A slot binds only when its members also share
/// a level; the misaligned ones render per node.
///
/// `classes` is the compile's one pool-class map, and this is the only place it is changed: a
/// mono track that would strand in a partial mono group is moved to the stereo pool before the
/// kept plan is formed (issue #971, [`stranded_mono_tracks`]), and the builtin-stage planner reads
/// the map afterwards, so it pools that track exactly as the rack chains were pooled.
pub(crate) fn bind_rack_banks_indexed(
    effects: &EffectPreparedSession,
    prepared: &PreparedEffectIndex<'_>,
    ids: &[Option<EffectNodeId>],
    levels: &[DependencyLevel],
    dispatch: Backend,
    classes: &mut SessionPoolClasses,
) -> Result<(Vec<graph::GraphPreparedEffectBank>, GraphRackBankReport), GraphDiagnostic> {
    let model = effects.session.normalized_model();
    // One chain per (track, bankable rack), in session slot order.
    let mut chains: BTreeMap<RackChainId, Vec<EffectNodeId>> = BTreeMap::new();
    let mut programs: BTreeMap<RackChainId, RackProgram> = BTreeMap::new();
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
            // (`EffectProgramKey::blocks_banking`). A bank is a single kernel over the chain
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
                let Some(slot) = prepared.get(key.0, key.1, key.2) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                let Some(entry) = effects.entries.get(slot.index()) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                let Some(node) = prepared_effect_node(ids, slot) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                nodes.push(node.clone());
                slots.push(entry.metadata.program_key());
            }
            programs.insert(chain.clone(), RackProgram::new(location, slots));
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
                // Mono-collapse M1: the *track's* class, from the one map `GraphCompiler` derived
                // for this compile and also handed to the strip-bank planner. A chain is one
                // track's rack program, so the track's class is the chain's; deriving it here
                // from anything else is exactly the two-planner disagreement
                // `SessionPoolClasses` exists to make impossible.
                class: classes.class_of(&chain.track_id),
                program: programs[chain].clone(),
            });
    }
    let mut levels_in: Vec<_> = candidates_by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let mut plan = plan_bank_groups(&levels_in, width)
        .map_err(|_| diag("graph.effect.bank_members", "$.effects"))?;
    let bind = |plan: &BankPlan<RackChainId>| {
        bind_planned_banks(
            plan,
            &chains,
            &level_by_node,
            effects,
            prepared,
            dispatch,
            width,
        )
    };
    let (mut banks, mut bound_slots) = bind(&plan)?;

    // Issue #971: keep the mono pool a whole number of cohorts. The plan above is the trial.
    // A mono track every one of whose effect groups is a *partial* mono group banks nowhere in
    // it, so it is moved to the stereo pool, where it renders dual but can fill a bank; the
    // classes are then re-read and the session is planned again.
    //
    // The move is kept only if the new plan **binds** more effect banks than the trial did, and
    // both counts are the banks the factories actually bound, not the slots the planner formed:
    // a factory may decline a full group (the delay never banks; an effect built for another
    // width declines that width), so a planned slot proves nothing. The check exists because a
    // move can cost: a track in the stereo pool gives up its builtin stages' collapse, and it can
    // join a stereo cohort in a way that loses a bank (a longer program taking the leader of
    // stereo tracks whose programs are disjoint subsequences of it). The check is one decision
    // for the whole move set, not one per stranded track (a follow-up recorded in #971's spec).
    // Either way the map is updated **before** the builtin-stage planner reads it, so both
    // planners still see one class per track.
    let stranded = stranded_mono_tracks(&plan);
    if !stranded.is_empty() {
        let mut demoted = classes.clone();
        for track in &stranded {
            demoted.pool_as_stereo(track);
        }
        for level in &mut levels_in {
            for candidate in &mut level.candidates {
                candidate.class = demoted.class_of(&candidate.id.track_id);
            }
        }
        let replan = plan_bank_groups(&levels_in, width)
            .map_err(|_| diag("graph.effect.bank_members", "$.effects"))?;
        // Issue #1001: the re-plan is speculative, and the trial above is already bound and
        // valid, so a factory error while binding the re-plan keeps the unmoved plan and its banks
        // rather than failing the compile (#95: a cohort a factory cannot bank never costs the
        // user the compile). The re-plan's banks bound before the error are dropped with it, and
        // `classes` is still the unmoved map: only the clone was changed.
        if let Ok((replan_banks, replan_slots)) = bind(&replan)
            && replan_banks.len() > banks.len()
        {
            *classes = demoted;
            plan = replan;
            banks = replan_banks;
            bound_slots = replan_slots;
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

/// The mono-class tracks the trial `plan` strands (issue #971): every effect group each one sits in
/// is a **partial** group of the mono pool, so none of its effect slots can bind.
///
/// The test is per track over all its groups, not per program. A track whose simd1 chain fills a
/// mono cohort while its simd2 chain is a partial one is not stranded: moving it would break the
/// full cohort, and its builtin stages' collapse with it, to gain a bank that may not exist. A
/// track in no effect group at all (builtins only, or every chain on the per-node path) is not
/// stranded either: builtin banks pad a partial cohort, so its remainder still banks and still
/// collapses where it is. A group is class-homogeneous, so a member of a partial mono group is a
/// mono-class track.
fn stranded_mono_tracks(plan: &BankPlan<RackChainId>) -> Vec<String> {
    let mut stranded: BTreeMap<&str, bool> = BTreeMap::new();
    for group in &plan.groups {
        let partial_mono =
            group.class == CohortPoolClass::MonoSymmetricAtPrepare && !group.is_full();
        for id in group.members.iter().flatten() {
            let entry = stranded.entry(id.track_id.as_str()).or_insert(true);
            *entry = *entry && partial_mono;
        }
    }
    stranded
        .into_iter()
        .filter(|(_, stranded)| *stranded)
        .map(|(track, _)| track.to_owned())
        .collect()
}

/// Binds every slot of `plan` that can be one homogeneous bank and whose factory consents:
/// the banks and the report's `(group, slot, members)` entries, in plan order.
fn bind_planned_banks(
    plan: &BankPlan<RackChainId>,
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<GraphNodeId, u64>,
    effects: &EffectPreparedSession,
    prepared: &PreparedEffectIndex<'_>,
    dispatch: Backend,
    width: BankWidth,
) -> Result<(Vec<graph::GraphPreparedEffectBank>, Vec<GraphRackBoundSlot>), GraphDiagnostic> {
    let mut banks = Vec::new();
    let mut bound_slots = Vec::new();
    for (group_index, group) in plan.groups.iter().enumerate() {
        for slot in 0..group.program.len() {
            let Some(members) = bindable_slot_members(group, slot, chains, level_by_node)? else {
                continue;
            };
            let entries: Vec<&EffectPreparedEntry> = members
                .iter()
                .map(|node| {
                    let slot = prepared
                        .get(node.track_id.as_str(), node.rack, node.effect_id.as_str())
                        .expect("prepared effect node has an entry");
                    &effects.entries[slot.index()]
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
            let scratch = rack::AoSoaScratch::new(width, effects.session.quantum().0)
                .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?;
            banks.push(graph::GraphPreparedEffectBank {
                members: members.clone().into_boxed_slice(),
                active_mask: group.active_mask.clone(),
                processor,
                response_snapshot_declared: entries[0].factory.response_analysis().is_some(),
                native_id: entries[0].factory.descriptor().id.as_str(),
                scratch,
                // Issue #181: the group this slot came out of, carried forward so the runtime can
                // build one chain per cohort instead of one per slot. It is the same `(group,
                // slot)` pair `bound_slots` already reports; the report was the only consumer.
                cohort: graph::GraphBankCohort {
                    group: u32::try_from(group_index)
                        .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?,
                    slot: u32::try_from(slot)
                        .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?,
                },
            });
            bound_slots.push(GraphRackBoundSlot {
                group: group_index,
                slot,
                members,
            });
        }
    }
    Ok((banks, bound_slots))
}

/// The effect node each lane of `group` runs at leader slot `slot`, when that slot can be one
/// homogeneous bank; `None` when it cannot, and its members render per node.
fn bindable_slot_members(
    group: &BankGroup<RackChainId>,
    slot: usize,
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<GraphNodeId, u64>,
) -> Result<Option<Vec<EffectNodeId>>, GraphDiagnostic> {
    // A bank binds only a full group (#96 F7): every launch effect factory refuses
    // `requests.len() != lanes`.
    if !group.is_full() || group.slot_is_identity_everywhere(slot) {
        return Ok(None);
    }
    // Every lane must run this slot: the effect contract has no per-lane bypass mask
    // (#96 F7), so a bank whose lanes disagree cannot be expressed.
    if !group.active_slots.iter().all(|lane| lane[slot]) {
        return Ok(None);
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
    // Issue #966: every member must sit at one dependency level, or bind refuses the
    // plan (`graph.scheduler.layout`). An identity slot is a planner fiction with no
    // graph node, so a lane's member for leader slot `slot` sits at `group.level + rank`,
    // not at `group.level + slot`: a lane that skips an earlier slot another lane runs
    // reaches this one a level early. Such a bank would run as one unit at its first
    // member's position, before the late lanes' producers had written their blocks. The
    // slot is left unbound instead and its members render per node, exactly as a slot
    // some lane skips does. Equal ranks and equal levels are the same condition (the path
    // arithmetic in `bind_rack_banks_indexed` asserts `slot_level == level + offset` for every
    // bankable chain), and this reads the levels because they are what bind checks.
    let member_level = |node: &EffectNodeId| {
        level_by_node
            .get(&GraphNodeId::Effect(node.clone()))
            .copied()
    };
    let first_level = member_level(&members[0]);
    if first_level.is_none()
        || members
            .iter()
            .any(|member| member_level(member) != first_level)
    {
        return Ok(None);
    }
    Ok(Some(members))
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
    banks: &[graph::GraphPreparedEffectBank],
    quantum: u32,
) -> Option<EffectBankResourceEstimate> {
    let bank_count = u64::try_from(banks.len()).ok()?;
    let mut resource = EffectBankResourceEstimate {
        bank_count,
        ..EffectBankResourceEstimate::default()
    };
    let bank_array_bytes = u64::try_from(core::mem::size_of::<graph::GraphPreparedEffectBank>())
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
