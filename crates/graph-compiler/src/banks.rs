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

/// Whether a graph rack holds a console section (decision 12, "Class A by lowering").
///
/// After S1a (#1093) the two SIMD racks hold exactly the session's console slots: `pre_insert`
/// lowers to [`RackLocation::Simd1`], `post_insert` to [`RackLocation::Simd2`], and a track's
/// inserts to [`RackLocation::Dynamic`]. No session reaches either SIMD rack any other way
/// (`session::SessionModel::lower_track`), so a chain or a group in one of them is a console one.
pub(crate) const fn is_console_rack(rack: RackLocation) -> bool {
    matches!(rack, RackLocation::Simd1 | RackLocation::Simd2)
}

/// Whether the planner pads `group` to a whole bank when it is partial (issues #1088 and #1098;
/// decision 12, "Banking").
///
/// A *partial* group has fewer members than the bank width. Padding binds it as one bank anyway:
/// the absent lanes carry a clone of an active member's request and run as padded lanes under the
/// contract on `effect_contract::PrepareEffectBankRequest`. A full group needs no padding and is
/// bound whatever this says.
///
/// **A console group is always padded** (S2, #1098): console slots bank on every target and for
/// every track count, with no member threshold. The owner accepted that a one- or two-member
/// remainder costs more padded than per node at eight lanes (H5). Every effect on the console
/// eligibility list accepts padded requests (P2b-P2e, #1089-#1092), so for a valid session every
/// console group binds, and [`unbanked_console_slot`] refuses a compile in which one does not.
///
/// **An insert group is never padded:** inserts bank opportunistically, as they did before
/// decision 12, so a full insert group banks and an insert remainder renders per node (decision 12,
/// "Inserts bank opportunistically, as today").
pub(crate) const fn pads(group: &BankGroup<RackChainId>) -> bool {
    is_console_rack(group.rack)
}

/// Plan the SIMD-rack cohorts over whole rack chains, and bind every slot that can be bound.
///
/// The planner is `rack_compiler::plan_bank_groups` -- the single cohort planner in
/// the workspace (#96 F1). #99 F3 changes *what is handed to it*: one candidate per
/// `(track, rack)` whose slots are that track's rack program **in session order**
/// (each lowered rack of `SessionModel::lower_track`), not `EffectPreparedSession::entries` order,
/// which is sorted by effect id. That is AGENTS.md's cohort model -- a signature over slot
/// types/order with absent slots as identity kernels -- and it is what makes a multi-slot bank
/// expressible at all: #96's per-effect candidates carry one-slot programs, so they can only ever
/// form single-slot banks.
///
/// A slot is bound when the group is full, or padded under [`pads`], and **every** member runs
/// that slot. A slot some member skips would need a per-lane identity slot in the effect contract,
/// which does not exist (#96 F7; #888's identity half); those members render on the per-node
/// scalar path exactly as before. A partial group that is not padded -- an insert remainder -- is
/// likewise unbound, unchanged from #96.
///
/// **Console slots always bank** (S2, #1098; decision 12). Every console group is padded, and on a
/// vector backend a console slot that is not bound fails the compile with `console.slot.unbanked`
/// ([`unbanked_console_slot`]) rather than rendering per node. There is no silent fallback. For a
/// valid session that is unreachable: every track carries every slot in one order, so a console
/// rack's chains share one program, a console slot has no sidechain, and every effect on the
/// console eligibility list binds padded requests. A console group is formed per (rack, pool class,
/// dependency level of the chain's first slot), so each console slot binds
/// `sum over (pool class, level) of ceil(n / W)` banks. Differing insert counts put `post_insert`
/// at several levels, one group per level, and a cohort whose `post_insert` banks do not line up
/// with its `pre_insert` ones pays a planar/AoSoA round trip where the chain cannot fuse (H2). That
/// cost is recorded in #1098's evidence, not fixed here.
///
/// A padded slot binds the group's members on its active lanes and a clone of its first member's
/// request on every padded lane (issue #1088). The group's `active_mask` travels to the factory and
/// onto the bound bank, so the rack gathers and scatters the active lanes only.
///
/// A *bypassed* slot is not a skipped one (issue #1087). It has its node and its latency, and a
/// lowered session bypass (every effect but the delay and the multiband;
/// `effect_compiler::lowers_session_bypass`) never reaches the program key this planner compares:
/// preparation lowers it to
/// a prepared `bypass = false` plus a bypassed lane on the rack's latency-preserving shunt
/// (`EffectControlLane::without_channel`), so a cohort's tracks group, and its slots bind, whatever
/// mix of them is bypassed. The runtime builds the bank's `rack::LiveControlEffectBankStage` from
/// those lanes, and it restores each bypassed lane's delayed dry signal after the bank runs.
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
/// mono track that would bank nowhere is moved to the stereo pool before the kept plan is formed
/// (issue #971, [`stranded_mono_tracks`]), and the builtin-stage planner reads the map afterwards,
/// so it pools that track exactly as the rack chains were pooled. A console slot always banks, so a
/// track that carries one -- in a session with a console, every track -- is never moved (#1098).
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
        let lowered = model.lower_track(track);
        let [pre_insert, inserts, post_insert] = lowered.in_chain_order();
        for (rack, declared) in [
            (RackId::Simd1, pre_insert),
            (RackId::Dynamic, inserts),
            (RackId::Simd2, post_insert),
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
                // Issue #1087: a lowered session bypass is not in this key -- preparation turned
                // it into shunt state -- so a bypassed track keeps its cohort. A bypass that stays
                // prepared (the multiband's, issue #1100, and the delay's) is in the key.
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

    let level_by_node: BTreeMap<&EffectNodeId, u64> = levels
        .iter()
        .flat_map(|level| {
            level.nodes.iter().filter_map(move |node| match node {
                GraphNodeId::Effect(node) => Some((node, level.level)),
                _ => None,
            })
        })
        .collect();

    let mut candidates_by_level: BTreeMap<u64, Vec<CohortCandidate<RackChainId>>> = BTreeMap::new();
    'chain: for (chain, nodes) in &chains {
        let Some(first) = nodes.first() else {
            continue;
        };
        let Some(level) = level_by_node.get(first).copied() else {
            continue;
        };
        // Slot k sits at level + k: a rack chain is a path, so each slot depends on the previous.
        for (offset, node) in nodes.iter().enumerate() {
            let Some(slot_level) = level_by_node.get(node).copied() else {
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
    let bind_group = |index: usize, group: &BankGroup<RackChainId>| {
        bind_group_banks(
            index,
            group,
            pads(group),
            &chains,
            &level_by_node,
            effects,
            prepared,
            dispatch,
            width,
        )
    };
    // One entry per plan group, in plan order: the banks bound from that group's slots.
    let mut bound: Vec<BoundGroup> = plan
        .groups
        .iter()
        .enumerate()
        .map(|(index, group)| bind_group(index, group))
        .collect::<Result<_, _>>()?;

    // Issue #971: keep the mono pool a whole number of cohorts. The plan above is the trial.
    // A mono track every one of whose effect groups is a *partial* mono group that is not padded
    // banks nowhere in it, so it is moved to the stereo pool, where it renders dual but can fill a
    // bank; the classes are then re-read and the session is planned again. A console group is
    // always padded and always binds (#1098), so a track carrying a console slot banks somewhere
    // and is never moved: see [`stranded_mono_tracks`] for why the demotion is retired there.
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

        // Issue #1002: only the groups the move changed are bound for the comparison. A re-plan
        // group equal to a trial group (same level, rack, class, leader program, members in the
        // same lanes and the same active slots) binds to the same banks: the bind reads nothing
        // but the group, the chains, the levels, the prepared entries, the width and the
        // dispatch, and none of those moved. So the trial's banks for such a group are reused,
        // and the two plans' bank counts differ only by the changed groups: the re-plan's new
        // ones (bound here) against the trial's vanished ones (bound above). That is the
        // comparison #971 made over whole plans, without binding the unchanged majority twice.
        // Every member of a plan sits in exactly one group, so a group's first member finds the
        // one trial group it could equal.
        let trial_group_of: BTreeMap<&RackChainId, usize> = plan
            .groups
            .iter()
            .enumerate()
            .filter_map(|(index, group)| Some((group.members.first()?.as_ref()?, index)))
            .collect();
        let unchanged: Vec<Option<usize>> = replan
            .groups
            .iter()
            .map(|group| {
                let first = group.members.first()?.as_ref()?;
                trial_group_of
                    .get(first)
                    .copied()
                    .filter(|index| plan.groups[*index] == *group)
            })
            .collect();
        // Issue #1001: the re-plan is speculative, and the trial above is already bound and
        // valid, so a factory error while binding the re-plan's changed groups keeps the unmoved
        // plan and its banks rather than failing the compile. This is the one exception to #95's
        // rule that an `Err` fails the compile, and it rests on #95's own reasoning (a planner
        // bug must not cost the user their session) applied to a plan that was only ever
        // speculative. The banks bound before the error are dropped with it, and `classes` is
        // still the unmoved map: only the clone was changed.
        let fresh: Result<Vec<Option<BoundGroup>>, GraphDiagnostic> = replan
            .groups
            .iter()
            .zip(&unchanged)
            .enumerate()
            .map(|(index, (group, unchanged))| match unchanged {
                Some(_) => Ok(None),
                None => bind_group(index, group).map(Some),
            })
            .collect();
        if let Ok(mut fresh) = fresh {
            let reused: BTreeSet<usize> = unchanged.iter().flatten().copied().collect();
            let gained: usize = fresh.iter().flatten().map(Vec::len).sum();
            let lost: usize = bound
                .iter()
                .enumerate()
                .filter(|(index, _)| !reused.contains(index))
                .map(|(_, group)| group.len())
                .sum();
            if gained > lost {
                let mut trial = core::mem::take(&mut bound);
                for (index, (unchanged, fresh)) in unchanged.iter().zip(&mut fresh).enumerate() {
                    let mut group = match unchanged {
                        Some(trial_index) => core::mem::take(&mut trial[*trial_index]),
                        None => fresh.take().unwrap_or_default(),
                    };
                    let cohort_group = u32::try_from(index)
                        .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?;
                    for (bank, slot) in &mut group {
                        bank.cohort.group = cohort_group;
                        slot.group = index;
                    }
                    bound.push(group);
                }
                *classes = demoted;
                plan = replan;
            }
        }
    }
    // Issue #1002: a bank reused from the trial plan carries the index its group had there, and
    // acceptance renumbers it. The report's slot index is gated by a test; the bank's cohort index
    // has no public reader, so it is tied to the same index here.
    debug_assert!(bound.iter().enumerate().all(|(index, group)| {
        group.iter().all(|(bank, slot)| {
            slot.group == index && usize::try_from(bank.cohort.group).ok() == Some(index)
        })
    }));
    let (banks, bound_slots): (Vec<_>, Vec<_>) = bound.into_iter().flatten().unzip();
    // S2 (#1098): the guarantee. This is a vector backend (the test-only `Scalar` oracle returned
    // with no banks above), so every console slot renders banked or the compile fails.
    if let Some(diagnostic) = unbanked_console_slot(&chains, &bound_slots, &level_by_node, classes)
    {
        return Err(diagnostic);
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
/// is a **partial** group of the mono pool that is not padded, so none of its effect slots binds.
///
/// The test is per track over all its groups, not per program. A track whose `pre_insert` chain
/// fills a mono cohort while its inserts are a partial one is not stranded: moving it would break
/// the full cohort, and its builtin stages' collapse with it, to gain a bank that may not exist. A
/// track in no effect group at all (builtins only, or every chain on the per-node path) is not
/// stranded either: builtin banks pad a partial cohort, so its remainder still banks and still
/// collapses where it is. A group is class-homogeneous, so a member of a partial mono group is a
/// mono-class track.
///
/// # A console slot never strands a track (#1098, amendment 10)
///
/// A console group is padded ([`pads`]), so a partial mono console group binds one padded mono
/// bank per slot: its members bank there, and a track that carries a console slot -- in a session
/// with a console, every track -- is never stranded and never moved. That retires the demotion for
/// console slots, rather than restating its objective, for three reasons:
///
/// * **Its premise cannot hold.** The demotion rescues a track that banks nowhere. Under padding a
///   console slot always banks, so no console track is ever in that state.
/// * **Its objective measures nothing for console groups** (M3). The move is kept when it binds
///   more banks. Moving a console remainder can only merge console groups -- the mono remainder
///   leaves one padded bank per slot and the stereo pool gains at most one -- so their count never
///   rises, and a merge that saves a padded bank scores as a loss. The count rises only when the
///   move also fills an insert group, and that trades the track's collapse on every console slot
///   for an insert remainder, which decision 12 leaves per node by choice.
/// * **The mono pool is the larger win.** A padded mono bank collapses to one plane, and the pool
///   is worth about 36 % on the standing console row (M3). A restated objective (planes x banks)
///   would move a mono remainder only when it fits in the stereo pool's padding at every slot and
///   builtin stage, to save one one-plane padded bank per stage; weighing that against an insert
///   remainder the move might fill needs a per-node cost that nothing has measured at four lanes.
///
/// A session with no console keeps #971's rule unchanged: its insert remainders still render per
/// node, so a stranded mono track still banks nowhere, and moving it can still gain a bank.
fn stranded_mono_tracks(plan: &BankPlan<RackChainId>) -> Vec<String> {
    let mut stranded: BTreeMap<&str, bool> = BTreeMap::new();
    for group in &plan.groups {
        let binds_nothing = group.class == CohortPoolClass::MonoSymmetricAtPrepare
            && !group.is_full()
            && !pads(group);
        for id in group.members.iter().flatten() {
            let entry = stranded.entry(id.track_id.as_str()).or_insert(true);
            *entry = *entry && binds_nothing;
        }
    }
    stranded
        .into_iter()
        .filter(|(_, stranded)| *stranded)
        .map(|(track, _)| track.to_owned())
        .collect()
}

/// The no-fallback guarantee (S2, #1098; decision 12, "Banking"): the diagnostic for the first
/// console slot, in chain order, that no bound bank carries, or `None` when every console slot
/// binds banked.
///
/// A console slot never renders per node on a vector backend. When one would -- a factory declined
/// its group, or the group did not form -- the compile fails with `console.slot.unbanked` at
/// `$.console.<section>[slot=<id>].bank[pool=<class>,level=<level>]`, naming the slot, its track's
/// pool class and the slot's dependency level: the group that did not bind. For a valid session
/// this is unreachable ([`bind_rack_banks_indexed`] says why); it guards a regression in the
/// planner, the binder or a factory, which would otherwise surface only as a slower render.
///
/// Only a vector backend reaches this. A plan compiled at the test-only `Scalar` oracle (#1059)
/// binds no bank at all, and it is exempt by construction: it is the per-node reference that the
/// banked render is compared with.
fn unbanked_console_slot(
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    bound_slots: &[GraphRackBoundSlot],
    level_by_node: &BTreeMap<&EffectNodeId, u64>,
    classes: &SessionPoolClasses,
) -> Option<GraphDiagnostic> {
    let banked: BTreeSet<&EffectNodeId> = bound_slots
        .iter()
        .flat_map(|bound| bound.members.iter())
        .collect();
    let (chain, node) = chains
        .iter()
        .filter(|(chain, _)| is_console_rack(rack_location(chain.rack)))
        .flat_map(|(chain, nodes)| nodes.iter().map(move |node| (chain, node)))
        .find(|(_, node)| !banked.contains(node))?;
    let section = match chain.rack {
        RackId::Simd1 => "pre_insert",
        _ => "post_insert",
    };
    let pool = match classes.class_of(&chain.track_id) {
        CohortPoolClass::MonoSymmetricAtPrepare => "mono",
        CohortPoolClass::Stereo => "stereo",
    };
    let Some(level) = level_by_node.get(node) else {
        return Some(diag("graph.internal.invariant", "$.effects"));
    };
    Some(diag(
        "console.slot.unbanked",
        &format!(
            "$.console.{section}[slot={}].bank[pool={pool},level={level}]",
            node.effect_id.as_str()
        ),
    ))
}

/// The banks bound from one plan group's slots, each with its report entry, in slot order.
type BoundGroup = Vec<(graph::GraphPreparedEffectBank, GraphRackBoundSlot)>;

/// Binds every slot of plan group `group_index` that can be one homogeneous bank and whose
/// factory consents. `padded` says whether a partial group is padded (issue #1088).
#[allow(clippy::too_many_arguments)] // The binder's whole context, passed from one closure.
fn bind_group_banks(
    group_index: usize,
    group: &BankGroup<RackChainId>,
    padded: bool,
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<&EffectNodeId, u64>,
    effects: &EffectPreparedSession,
    prepared: &PreparedEffectIndex<'_>,
    dispatch: Backend,
    width: BankWidth,
) -> Result<BoundGroup, GraphDiagnostic> {
    let mut bound = Vec::new();
    for slot in 0..group.program.len() {
        let Some(members) = bindable_slot_members(group, slot, padded, chains, level_by_node)?
        else {
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
        // One request per lane: each member's own on its active lane and, on every padded lane, a
        // clone of the first member's -- never zeros (issue #1088, the padding contract). The
        // planner emits members before padding, so the members are exactly lanes `0..members`.
        let mut requests: Vec<_> = entries
            .iter()
            .map(|entry| entry.bank_preparation.request())
            .collect();
        requests.resize(width.lanes() as usize, requests[0]);
        let request = PrepareEffectBankRequest {
            backend: dispatch,
            width,
            requests: &requests,
            active_mask: &group.active_mask,
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
        let bank = graph::GraphPreparedEffectBank {
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
        };
        bound.push((
            bank,
            GraphRackBoundSlot {
                group: group_index,
                slot,
                members,
            },
        ));
    }
    Ok(bound)
}

/// The effect node each member of `group` runs at leader slot `slot`, in lane order, when that
/// slot can be one homogeneous bank; `None` when it cannot, and its members render per node.
///
/// `padded` says whether a partial group binds padded (issue #1088). The nodes are the active
/// lanes' only, so a padded group returns fewer nodes than lanes.
fn bindable_slot_members(
    group: &BankGroup<RackChainId>,
    slot: usize,
    padded: bool,
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<&EffectNodeId, u64>,
) -> Result<Option<Vec<EffectNodeId>>, GraphDiagnostic> {
    // A partial group binds only when it is padded (issue #1088). Unpadded, its members render
    // per node, as every partial group's did before (#96 F7).
    if !(group.is_full() || padded) || group.slot_is_identity_everywhere(slot) {
        return Ok(None);
    }
    // Every member must run this slot: the effect contract has no per-lane identity slot
    // (#96 F7), so a bank some of whose members skip the slot cannot be expressed. A padded lane
    // has no member and no slots, and is the padding contract's, not an identity. (Lanes that
    // disagree only about bypass all run it: issue #1087 carries bypass on the rack's shunt.)
    if !group
        .active_slots
        .iter()
        .zip(&group.active_mask)
        .all(|(lane, active)| !*active || lane[slot])
    {
        return Ok(None);
    }
    // Padding lanes come after every member (`rack_compiler::BankGroup::members`), so the
    // members are exactly lanes `0..members`: the layout the padded request's clone lanes and the
    // graph runtime's gather and scatter both rely on. The planner asserts it in debug builds;
    // it is checked here as well because a bank bound from any other layout would feed a member
    // on a padded lane.
    let active = group.active_count();
    if group.members.iter().enumerate().any(|(lane, id)| {
        id.is_some() != (lane < active) || group.active_mask[lane] != id.is_some()
    }) {
        return Err(diag("graph.internal.invariant", "$.effects"));
    }
    // Lane `i` runs its own chain in order, so the leader slot maps to the lane's slot by
    // the rank of `slot` among that lane's active positions.
    let mut members = Vec::with_capacity(active);
    for (lane, id) in group.members.iter().flatten().enumerate() {
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
    let member_level = |node: &EffectNodeId| level_by_node.get(node).copied();
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

/// The retained cost of the bound effect banks, or `None` when a bank's shape is not one this
/// compiler binds.
///
/// A bank may be padded (issue #1088): it has `1..=lanes` members and its active mask is `true` on
/// exactly lanes `0..members`, the planner's members-before-padding layout, which is also the
/// layout the graph runtime gathers and scatters by. Scratch is charged for every lane, because a
/// padded lane still occupies its AoSoA column; member metadata for the members only.
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
        let members = u64::try_from(bank.members.len()).ok()?;
        if bank.scratch.quantum() != quantum
            || members == 0
            || members > lanes
            || u64::try_from(bank.active_mask.len()).ok()? != lanes
            || bank
                .active_mask
                .iter()
                .enumerate()
                .any(|(lane, active)| *active != (lane < bank.members.len()))
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
            .checked_mul(members)?;
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
