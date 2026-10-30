//! SIMD-rack cohort planning and homogeneous-bank binding.
//!
//! One cohort former, over whole rack chains: see [`bind_rack_banks`] (#96 F1, #99 F3).

use super::*;
use crate::ids::{AsProjected, EffectInputs, PreparedEffectIndex, diag, prepared_effect_node};
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
/// lowers to [`RackLocation::Simd1`], `post_insert` to [`RackLocation::Simd2`], and a strip's
/// inserts to [`RackLocation::Dynamic`]. No session reaches either SIMD rack any other way
/// (`session::SessionModel::lower_strip`), so a chain or a group in one of them is a console one.
/// A submix's console chains (#1202) are console chains like a track's.
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
/// **An insert group is padded from [`min_padded_insert_members`] members** (issues #888 and #889):
/// inserts still bank opportunistically -- a full insert group banks, an effect whose factory
/// declines padding renders per node, and no insert diagnostic exists -- but an insert remainder
/// of at least that many members binds as one padded bank per slot instead of rendering per node
/// (decision 12, "Banking", as refined by #888). Below it the remainder renders per node, where it
/// is cheaper. Padding never moves an active lane's bits (the coupling rule), so the threshold
/// decides cost only.
pub(crate) fn pads(group: &BankGroup<RackChainId>, width: BankWidth) -> bool {
    is_console_rack(group.rack) || group.active_count() >= min_padded_insert_members(width)
}

/// The fewest members an insert remainder needs before the planner pads it (issues #888 and #889).
///
/// A padded bank costs about what a full bank of the same slots costs, whatever its member count,
/// while per node costs one track's worth per member. H5 measured both at eight lanes
/// (`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, "H5", arithmetic on the `final-0400`
/// record): about 9.3 µs per padded eight-lane bank of three slots against about 4.2 µs per
/// per-node track, so per node wins at one member, is about even at two, and padding wins from
/// three. At four lanes the padded bank is about half as wide, so the break-even falls near one
/// member and padding wins from two (H5's "break-even probably near one member"; unmeasured, a
/// planner decision recorded in #888's spec).
///
/// Reopen when a measured per-node row exists at either width: the constant is that row's
/// break-even, and a new measurement replaces the H5 arithmetic here.
pub(crate) const fn min_padded_insert_members(width: BankWidth) -> usize {
    match width {
        BankWidth::Four => 2,
        #[cfg(target_feature = "avx2")]
        BankWidth::Eight => 3,
    }
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
/// scalar path exactly as before. A partial group that is not padded -- an insert remainder below
/// [`min_padded_insert_members`] -- is likewise unbound, unchanged from #96.
///
/// **Console slots always bank** (S2, #1098; decision 12). Every console group is padded, and on a
/// vector backend a console slot that is not bound fails the compile with `console.slot.unbanked`
/// ([`unbanked_console_slot`]) rather than rendering per node. There is no silent fallback. For a
/// valid session at the build's own width (`Backend::current()`) that is unreachable: every strip
/// -- every track and, since #1202, every submix -- carries every slot in one order, so a console
/// rack's chains share one program, a console slot has no sidechain, and every effect on the
/// console eligibility list binds padded requests at that width. At a foreign vector width (a
/// test or bench dispatch; no production caller passes one) a factory may legally decline the
/// width, since a build's factories bind only the widths it executes (D4), and the compile is then
/// refused with `console.slot.foreign_width` instead (#1694). A
/// console group is formed per (rack, pool class, dependency level of the chain's first slot), so
/// each console slot binds `sum over (pool class, level) of ceil(n / W)` banks. Differing insert
/// counts put `post_insert` at several levels, one group per level, and a cohort whose
/// `post_insert` banks do not line up with its `pre_insert` ones pays a planar/AoSoA round trip
/// where the chain cannot fuse (H2). That cost is recorded in #1098's evidence, not fixed here.
///
/// A padded slot binds the group's members on its active lanes and a clone of its first member's
/// request on every padded lane (issue #1088). The group's `active_mask` travels to the factory and
/// onto the bound bank, so the rack gathers and scatters the active lanes only.
///
/// A *bypassed* slot is not a skipped one (issue #1087). It has its node and its latency, and a
/// session bypass (every effect's since #1340) never reaches the program key this planner
/// compares: preparation lowers it to
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
#[cfg(test)]
pub(crate) fn bind_rack_banks_indexed(
    effects: &EffectPreparedSession,
    prepared: &PreparedEffectIndex<'_>,
    ids: &[Option<EffectNodeId>],
    levels: &[DependencyLevel],
    dispatch: Backend,
    classes: &mut SessionPoolClasses,
) -> Result<(Vec<graph::GraphPreparedEffectBank>, GraphRackBankReport), GraphDiagnostic> {
    let inputs = EffectInputs::prepared(effects);
    let (planned, report) = plan_rack_banks_placed(
        &inputs,
        prepared,
        ids,
        levels,
        &PlacementLevels::single_point(levels),
        dispatch,
        classes,
    )?;
    Ok((
        bind_planned_banks(planned, &inputs, prepared, dispatch)?,
        report,
    ))
}

/// Every strip's rack chains, in session slot order, and each chain's program: the rack planner's
/// candidates (one per `(strip, rack)` whose slots may bank), shared by the bind and by the
/// placement's bank count ([`placement_levels`]) so the two read one candidate set.
pub(crate) fn rack_chains<E: AsProjected>(
    effects: &EffectInputs<'_, E>,
    prepared: &PreparedEffectIndex<'_>,
    ids: &[Option<EffectNodeId>],
) -> Result<RackChains, GraphDiagnostic> {
    let model = effects.session.normalized_model();
    // One chain per (track, bankable rack), in session slot order.
    let mut chains: BTreeMap<RackChainId, Vec<EffectNodeId>> = BTreeMap::new();
    let mut programs: BTreeMap<RackChainId, RackProgram> = BTreeMap::new();
    for strip in model.strips() {
        let lowered = model.lower_strip(&strip);
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
                track_id: strip.id.as_str().to_owned(),
                rack,
                slot: None,
            };
            let mut nodes = Vec::with_capacity(declared.len());
            let mut slots = Vec::with_capacity(declared.len());
            for effect in declared {
                let key = (strip.id.as_str(), rack, effect.id.as_str());
                let Some(slot) = prepared.get(key.0, key.1, key.2) else {
                    return Err(diag("graph.internal.invariant", "$.effects"));
                };
                let Some(entry) = effects.entry(slot.index()) else {
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

    Ok((chains, programs))
}

/// [`rack_chains`]' result: the chains' effect nodes and their programs.
pub(crate) type RackChains = (
    BTreeMap<RackChainId, Vec<EffectNodeId>>,
    BTreeMap<RackChainId, RackProgram>,
);

/// The rack banks of the compile, planned with the cohorts bucketed by `placement` (issue #2136,
/// [`placement_levels`]) rather than by `levels`. `levels` stays the graph's: the path-arithmetic
/// invariant below (#966) is checked on it, and the diagnostics name it.
///
/// Issue #2199: the plan is every bank the factories bind, without its processor. Each slot is
/// bound once here, as before, to learn whether its factory binds it and with which metadata,
/// and the bank is dropped at once, so at most one bank is resident while the plan is formed and
/// no prepared processor is needed; [`bind_planned_banks`] binds the planned banks after the
/// compile's resources are admitted. Binding is a function of the requests, so the two binds
/// agree.
pub(crate) fn plan_rack_banks_placed<E: AsProjected>(
    effects: &EffectInputs<'_, E>,
    prepared: &PreparedEffectIndex<'_>,
    ids: &[Option<EffectNodeId>],
    levels: &[DependencyLevel],
    placement: &PlacementLevels,
    dispatch: Backend,
    classes: &mut SessionPoolClasses,
) -> Result<(Vec<PlannedEffectBank>, GraphRackBankReport), GraphDiagnostic> {
    let (mut chains, programs) = rack_chains(effects, prepared, ids)?;
    let model = effects.session.normalized_model();

    let empty = |dispatch, chains: BTreeMap<RackChainId, Vec<EffectNodeId>>| GraphRackBankReport {
        dispatch,
        plan: BankPlan {
            groups: Vec::new(),
            scalar: Vec::new(),
        },
        bound_slots: Vec::new(),
        chains,
        utilisation: BankUtilisation::default(),
        utilisation_by_rack: RackLocation::ALL.map(|rack| (rack, BankUtilisation::default())),
        windows: BTreeMap::new(),
        planned_levels: placement.planned_levels.clone(),
        leftover_levels: BTreeMap::new(),
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
        // Issue #2136: the cohort is bucketed by the level the placement sweep gave the chain's
        // first slot, which aligns chains of one key class whose level windows intersect.
        let Some(placed) = placement.level_of(&GraphNodeId::Effect(first.clone())) else {
            return Err(diag("graph.internal.invariant", "$.effects"));
        };
        candidates_by_level
            .entry(placed)
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
    // Every chain node's placement level: a slot binds when its members share one (#966, on the
    // placement the cohort was bucketed by).
    let placed_by_node: BTreeMap<&EffectNodeId, u64> = chains
        .values()
        .flatten()
        .filter_map(|node| {
            placement
                .level_of(&GraphNodeId::Effect(node.clone()))
                .map(|level| (node, level))
        })
        .collect();
    // Each chain's window after placement (`[asap', ALAP]` of its first slot), for the report.
    let windows: BTreeMap<RackChainId, (u64, u64)> = chains
        .iter()
        .filter_map(|(chain, nodes)| {
            let first = GraphNodeId::Effect(nodes.first()?.clone());
            Some((chain.clone(), placement.window_of(&first)?))
        })
        .collect();
    let mut levels_in: Vec<_> = candidates_by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let mut plan = plan_bank_groups(&levels_in, width)
        .map_err(|_| diag("graph.effect.bank_members", "$.effects"))?;
    let bind_group = |index: usize, group: &BankGroup<RackChainId>| {
        plan_group_banks(
            index,
            group,
            pads(group, width),
            &chains,
            &placed_by_node,
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
    let stranded = stranded_mono_tracks(&plan, width);
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
    // Issue #2138: the leftover pass, over the insert nodes the chain pass left per node.
    let mut windows = windows;
    let mut leftover_levels = BTreeMap::new();
    leftover_pass(
        &mut plan,
        &mut bound,
        &mut chains,
        &mut windows,
        &programs,
        placement,
        classes,
        effects,
        prepared,
        dispatch,
        width,
        &mut leftover_levels,
    )?;
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
    if let Some(diagnostic) =
        unbanked_console_slot(dispatch, &chains, &bound_slots, &level_by_node, classes)
    {
        return Err(diagnostic);
    }
    // The same guarantee for a console entry the strip walk above never collected (#1202 verdict
    // MINOR-1): the population comes from preparation, not from `chains`, so a walk that skips a
    // strip's console fails the compile instead of rendering that strip's slots per node.
    if let Some(diagnostic) =
        uncollected_console_slot(effects, ids, &chains, &level_by_node, classes)
    {
        return Err(diagnostic);
    }
    // Issue #2132: the plan's lane use, from the plan itself.
    let fold_targets = route_fold_targets(model);
    let lanes = width.lanes() as usize;
    let utilisation = BankUtilisation::of(
        &plan.groups,
        &bound_slots,
        lanes,
        &fold_targets,
        &windows,
        None,
    );
    let utilisation_by_rack = RackLocation::ALL.map(|rack| {
        (
            rack,
            BankUtilisation::of(
                &plan.groups,
                &bound_slots,
                lanes,
                &fold_targets,
                &windows,
                Some(rack),
            ),
        )
    });
    Ok((
        banks,
        GraphRackBankReport {
            dispatch,
            plan,
            bound_slots,
            chains,
            utilisation,
            utilisation_by_rack,
            windows,
            planned_levels: placement.planned_levels.clone(),
            leftover_levels,
        },
    ))
}

/// The leftover pass (issue #2138, `docs/BANKING_DESIGN.md` slice 8): every insert node the chain
/// pass left per node is offered again as a one-slot candidate, and the ones that meet in a level
/// window bind as one-slot banks beside the chain pass's.
///
/// A candidate is a node of a [`RackLocation::Dynamic`] chain that no bound bank carries and whose
/// own key does not block banking (a sidechained slot and the delay stay per node). The candidates
/// are placed by the same sweep as the chain pass's placement (`rack_compiler::place_over_windows`),
/// keyed by the planner's own key (program key, pool class) and counted by the planner's own
/// grouping (`plan_bank_groups` over one-slot candidates, as the chain pass counts its chains),
/// over a sub-problem: the candidates, and every graph neighbour of one fixed at the level the
/// placement gave it. A candidate's window is the levels it may take with those neighbours where
/// they are, so every edge stays strictly upward, every bank (chain, builtin or leftover) still
/// has its members at one placement level, and the graph contracted by bank is acyclic (#2134).
/// The chain pass's bound nodes are not in the sub-problem: they sit in its banks, which a
/// leftover candidate never joins. The placed candidates are then bucketed by level for
/// [`plan_bank_groups`] and bound through the chain pass's binder; a group below
/// [`min_padded_insert_members`] does not bind, and its nodes render per node.
///
/// Banking regroups lanes only, so a leftover bank renders each lane's own arithmetic: the
/// per-node bits.
#[allow(clippy::too_many_arguments)] // The bind's whole context.
fn leftover_pass<E: AsProjected>(
    plan: &mut BankPlan<RackChainId>,
    bound: &mut Vec<BoundGroup>,
    chains: &mut BTreeMap<RackChainId, Vec<EffectNodeId>>,
    windows: &mut BTreeMap<RackChainId, (u64, u64)>,
    programs: &BTreeMap<RackChainId, RackProgram>,
    placement: &PlacementLevels,
    classes: &SessionPoolClasses,
    effects: &EffectInputs<'_, E>,
    prepared: &PreparedEffectIndex<'_>,
    dispatch: Backend,
    width: BankWidth,
    leftover_levels: &mut BTreeMap<EffectNodeId, u64>,
) -> Result<(), GraphDiagnostic> {
    let invariant = || diag("graph.internal.invariant", "$.effects");
    let banked: BTreeSet<&EffectNodeId> = bound
        .iter()
        .flatten()
        .flat_map(|(_, slot)| slot.members.iter())
        .collect();
    // What the key's factory binds at this width, asked once per key with the bind's own request
    // shape: `(a full bank, a padded bank)`. A factory with no bank kernel (the delay, the
    // reverb) or none at this width declines both, and its nodes are no candidates; a factory
    // that has not opted into padding binds only full banks. The probe banks are dropped here, off
    // the render thread.
    let mut binds: BTreeMap<effect_contract::EffectProgramKey, (bool, bool)> = BTreeMap::new();
    let lanes = width.lanes() as usize;
    let mut probe = |key: &effect_contract::EffectProgramKey,
                     node: &EffectNodeId|
     -> Result<(bool, bool), GraphDiagnostic> {
        if let Some(known) = binds.get(key) {
            return Ok(*known);
        }
        let slot = prepared
            .get(node.track_id.as_str(), node.rack, node.effect_id.as_str())
            .ok_or_else(invariant)?;
        let entry = effects.entry(slot.index()).ok_or_else(invariant)?;
        let requests = vec![entry.bank_preparation.request(); lanes];
        let binds_with = |active_mask: &[bool]| {
            entry
                .factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: dispatch,
                    width,
                    requests: &requests,
                    active_mask,
                })
                // An error is a contract violation, never a decline: it refuses the compile,
                // as the bind's own call does.
                .map(|bank| bank.is_some())
                .map_err(|error| diag(error.code, "$.effects"))
        };
        let full = binds_with(&vec![true; lanes])?;
        let padded = full && binds_with(&(0..lanes).map(|lane| lane == 0).collect::<Vec<_>>())?;
        binds.insert(key.clone(), (full, padded));
        Ok((full, padded))
    };
    // (key, class) -> the candidates of that class, by id.
    let mut by_class: BTreeMap<
        (effect_contract::EffectProgramKey, CohortPoolClass),
        Vec<(RackChainId, EffectNodeId)>,
    > = BTreeMap::new();
    for (chain, nodes) in chains.iter() {
        if chain.rack != RackId::Dynamic || chain.slot.is_some() {
            continue;
        }
        for (rank, node) in nodes.iter().enumerate() {
            let key = programs[chain].slots.get(rank).ok_or_else(invariant)?;
            if banked.contains(node) || rack::BankSlotKey::blocks_banking(key) {
                continue;
            }
            let id = RackChainId {
                slot: Some(rank),
                ..chain.clone()
            };
            by_class
                .entry((key.clone(), classes.class_of(&chain.track_id)))
                .or_default()
                .push((id, node.clone()));
        }
    }
    // A key class with fewer candidates than the padding threshold forms no group that binds, so
    // it is dropped without asking its factory; every other class is asked once per key.
    let threshold = min_padded_insert_members(width);
    let mut kept = BTreeMap::new();
    for ((key, class), members) in by_class {
        if members.len() >= threshold && probe(&key, &members[0].1)?.0 {
            kept.insert((key, class), members);
        }
    }
    let by_class = kept;
    if by_class.is_empty() {
        return Ok(());
    }
    // The sub-problem the shared sweep (`rack_compiler::place_over_windows`) places: every
    // candidate, keyed by its planner key (program key, pool class), and every graph neighbour of
    // one, unkeyed and fixed at its placement level. Candidate-to-candidate edges (two leftover
    // slots of one chain) are the sweep's own edges.
    let candidates: Vec<(
        effect_contract::EffectProgramKey,
        CohortPoolClass,
        RackChainId,
        EffectNodeId,
    )> = by_class
        .into_iter()
        .flat_map(|((key, class), members)| {
            members
                .into_iter()
                .map(move |(id, node)| (key.clone(), class, id, node))
        })
        .collect();
    let mut index_of: BTreeMap<GraphNodeId, usize> = BTreeMap::new();
    for (index, (_, _, _, node)) in candidates.iter().enumerate() {
        index_of.insert(GraphNodeId::Effect(node.clone()), index);
    }
    let mut fixed: Vec<(GraphNodeId, u64)> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (index, (_, _, _, node)) in candidates.iter().enumerate() {
        let (producers, consumers) = placement
            .neighbours
            .get(node)
            .map_or((&[][..], &[][..]), |(p, c)| (p.as_slice(), c.as_slice()));
        for (neighbour, is_producer) in producers
            .iter()
            .map(|id| (id, true))
            .chain(consumers.iter().map(|id| (id, false)))
        {
            let other = match index_of.get(neighbour) {
                Some(other) => *other,
                None => {
                    let other = candidates.len() + fixed.len();
                    let level = placement.level_of(neighbour).ok_or_else(invariant)?;
                    fixed.push((neighbour.clone(), level));
                    index_of.insert(neighbour.clone(), other);
                    other
                }
            };
            // A candidate-to-candidate edge is seen from both ends; keep the producer's view.
            if is_producer && other >= candidates.len() {
                edges.push((other, index));
            } else if !is_producer {
                edges.push((index, other));
            }
        }
    }
    edges.sort_unstable();
    edges.dedup();
    let total = candidates.len() + fixed.len();
    let level_at = |index: usize| -> Result<u64, GraphDiagnostic> {
        if index < candidates.len() {
            placement
                .level_of(&GraphNodeId::Effect(candidates[index].3.clone()))
                .ok_or_else(invariant)
        } else {
            Ok(fixed[index - candidates.len()].1)
        }
    };
    // Windows: a fixed node is the point it was placed at; a candidate's is `[ASAP, ALAP]` over
    // the sub-problem, which a fixed point per neighbour bounds. With no recorded neighbours
    // (`PlacementLevels::single_point`) a candidate's window is its own level.
    let mut low: Vec<u64> = (0..total).map(level_at).collect::<Result<_, _>>()?;
    let mut high = low.clone();
    for index in 0..candidates.len() {
        if placement.neighbours.contains_key(&candidates[index].3) {
            low[index] = 0;
            high[index] = u64::MAX;
        }
    }
    for _ in 0..=candidates.len() {
        for &(producer, consumer) in &edges {
            if consumer < candidates.len() {
                low[consumer] = low[consumer].max(low[producer].saturating_add(1));
            }
            if producer < candidates.len() {
                high[producer] = high[producer].min(high[consumer].saturating_sub(1));
            }
        }
    }
    let nodes: Vec<
        rack_compiler::PlacementNode<(effect_contract::EffectProgramKey, CohortPoolClass)>,
    > = (0..total)
        .map(|index| rack_compiler::PlacementNode {
            window: rack_compiler::CohortWindow {
                asap: low[index],
                alap: high[index].max(low[index]),
            },
            key: candidates
                .get(index)
                .map(|(key, class, _, _)| (key.clone(), *class)),
        })
        .collect();
    // The merge counter is the leftover planner's own grouping: `plan_bank_groups` over the
    // set's one-slot candidates (a fixed neighbour has no key and is not counted).
    let binding_groups = |set: &[usize]| -> Vec<Vec<usize>> {
        let candidates: Vec<CohortCandidate<usize>> = set
            .iter()
            .filter_map(|index| {
                let (key, class) = nodes.get(*index)?.key.clone()?;
                Some(CohortCandidate {
                    id: *index,
                    class,
                    program: RackProgram::new(RackLocation::Dynamic, vec![key]),
                })
            })
            .collect();
        plan_bank_groups(
            &[CohortLevel {
                level: 0,
                candidates,
            }],
            width,
        )
        .map_or_else(
            |_| Vec::new(),
            |plan| {
                // Only the groups the bind will bind: full, or padded from the threshold where the
                // factory pads. Every member of a one-slot group runs its slot, so #96 F7 holds.
                plan.groups
                    .iter()
                    .filter(|group| {
                        let padded = binds
                            .get(&group.program[0])
                            .is_some_and(|(_, padded)| *padded);
                        group.is_full() || (padded && group.active_count() >= threshold)
                    })
                    .map(|group| group.members.iter().flatten().copied().collect())
                    .collect()
            },
        )
    };
    let banks_of = |set: &[usize]| -> usize { binding_groups(set).len() };
    let placed_levels =
        rack_compiler::place_over_windows(&nodes, &edges, banks_of).map_err(|_| invariant())?;
    // Every fixed node must stay where it was placed, or the chain pass's banks would move.
    for (offset, (_, level)) in fixed.iter().enumerate() {
        if placed_levels[candidates.len() + offset].level != *level {
            return Err(invariant());
        }
    }
    // What the counter says binds, per level: the report's `leftover_levels`.
    let mut at_level: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (index, placement) in placed_levels.iter().enumerate().take(candidates.len()) {
        at_level.entry(placement.level).or_default().push(index);
    }
    for (level, set) in &at_level {
        for index in binding_groups(set).into_iter().flatten() {
            leftover_levels.insert(candidates[index].3.clone(), *level);
        }
    }
    let mut buckets: BTreeMap<u64, Vec<CohortCandidate<RackChainId>>> = BTreeMap::new();
    let mut placed: BTreeMap<EffectNodeId, u64> = BTreeMap::new();
    for (index, (key, class, id, node)) in candidates.into_iter().enumerate() {
        let level = placed_levels[index].level;
        windows.insert(
            id.clone(),
            (placed_levels[index].floor, nodes[index].window.alap),
        );
        chains.insert(id.clone(), vec![node.clone()]);
        placed.insert(node, level);
        buckets.entry(level).or_default().push(CohortCandidate {
            id,
            class,
            program: RackProgram::new(RackLocation::Dynamic, vec![key]),
        });
    }
    let levels: Vec<CohortLevel<RackChainId>> = buckets
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let leftover = plan_bank_groups(&levels, width)
        .map_err(|_| diag("graph.effect.bank_members", "$.effects"))?;
    let placed: BTreeMap<&EffectNodeId, u64> =
        placed.iter().map(|(node, level)| (node, *level)).collect();
    for group in leftover.groups {
        let index = plan.groups.len();
        bound.push(plan_group_banks(
            index,
            &group,
            pads(&group, width),
            chains,
            &placed,
            effects,
            prepared,
            dispatch,
            width,
        )?);
        plan.groups.push(group);
    }
    Ok(())
}

/// The level every graph node is placed at for bank planning (issue #2136), and each node's
/// window after placement. Built once per compile by [`placement_levels`] and read by both bank
/// planners -- the rack chains here and the builtin strip stages (`builtins-compiler`'s
/// `planned_strip_banks`, through [`PlacementLevels::levels`]) -- so a strip's builtin stages and
/// console racks are aligned by one decision and their chains still fuse (`runtime::chains_into`).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PlacementLevels {
    /// The placement as levels: every node at its placement level, levels ascending, nodes in id
    /// order. Every edge goes strictly upward in it, like the graph's own levels, but it is the
    /// planners' bucketing input, never the graph's schedule.
    levels: Vec<DependencyLevel>,
    by_node: BTreeMap<GraphNodeId, (u64, u64, u64)>,
    /// Every insert node's graph producers and consumers (issue #2138): the leftover pass moves a
    /// node only within the levels these leave it. Empty for [`Self::single_point`], where every
    /// leftover node stays at its level.
    neighbours: BTreeMap<EffectNodeId, (Vec<GraphNodeId>, Vec<GraphNodeId>)>,
    /// Every node a bank planner groups, at its placement level.
    planned_levels: BTreeMap<GraphNodeId, u64>,
}

impl PlacementLevels {
    /// Every node at its own dependency level, its window that single point: the level bucketing
    /// the placement replaced, for callers that plan without a graph.
    #[cfg(test)]
    pub(crate) fn single_point(levels: &[DependencyLevel]) -> Self {
        let by_node = levels
            .iter()
            .flat_map(|level| {
                level
                    .nodes
                    .iter()
                    .map(move |node| (node.clone(), (level.level, level.level, level.level)))
            })
            .collect();
        Self {
            levels: levels.to_vec(),
            by_node,
            neighbours: BTreeMap::new(),
            planned_levels: BTreeMap::new(),
        }
    }

    /// The placement as the level list the planners bucket by.
    pub(crate) fn levels(&self) -> &[DependencyLevel] {
        &self.levels
    }

    fn level_of(&self, node: &GraphNodeId) -> Option<u64> {
        self.by_node.get(node).map(|(level, _, _)| *level)
    }

    /// `[asap', ALAP]` for a segment's first stage, the window it was placed within once its
    /// producers were; the placement level alone for every other node, which follows its segment.
    fn window_of(&self, node: &GraphNodeId) -> Option<(u64, u64)> {
        self.by_node
            .get(node)
            .map(|(_, floor, alap)| (*floor, *alap))
    }
}

/// The key class a node is placed in (issue #2136): a builtin strip stage, or the first slot of a
/// rack chain, per rack, and the strip's pool class. Coarser than the planners' own keys (a
/// program, a leader's subsequences), which only means the sweep may align two chains that then do
/// not bank together; it never splits what the planners would group.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlacementKey {
    Stage(TrackStage, CohortPoolClass),
    Chain(RackId, CohortPoolClass),
}

/// Place every node of the graph for bank planning (issue #2136, `docs/BANKING_DESIGN.md` 2.2).
///
/// Each node's window is `[ASAP, ALAP]`: `ASAP` its dependency level, `ALAP` the latest level it
/// can take without lengthening the longest path to the session output (the output sits at its own
/// level; a node no path joins to the output may sink to the deepest level).
///
/// `rack_compiler::place_over_windows` places the **keyed** nodes earliest deadline first among
/// the nodes of their [`PlacementKey`], and every other node as early as its producers allow, so
/// an unkeyed node keeps its offset from the keyed node before it. Keyed are every strip's
/// post-input builtin stage, the first slot of every rack chain, and a fader that no bankable
/// effect feeds directly (through elided stage boundaries). A chain's later slots, a fader fed by
/// a chain's last slot, and every matrix stage follow: they keep the lane set of the stage before
/// them, so the chains the planners form there still fuse (`runtime::chains_into`). Moving a
/// fader apart from the chain that feeds it would trade a fused stage for a planar round trip per
/// bank (H2), and on C1 that round trip cost what a merged bank saved (#2136 evidence). A chain's
/// slot `k` therefore sits at its first slot's placement plus `k` whenever it sat at its first
/// slot's level plus `k`.
///
/// A merge is taken only where it lowers the count of groups the planners form
/// (`place_over_windows`; a group the bind renders per node still counts as one), and the count
/// is the planners' own: [`PlannedNode`] keys every node a bank planner will group exactly as that
/// planner keys it -- a rack chain's first slot by its candidate (rack, program, pool class), fed
/// through `rack_compiler::plan_bank_groups` as the rack planner feeds it; a builtin strip stage by
/// (stage, pool class), through the same planner as the builtin planner -- whether the sweep keyed
/// it for placement or placed it as a follower. So the set counted at a (planner key, level) is the
/// members the planner will group there, minus the nodes later stabs place at that level; the
/// report's `planned_levels` exposes it and `bank_levels.rs` checks it against the plan. One limit:
/// #971 may move a stranded mono track to the stereo pool after placement, which the count, taken
/// on the classes before the move, does not see. With no bank width nothing merges.
///
/// `chains` and `programs` are [`rack_chains`]' candidates.
///
/// `per_node` is every effect whose slot key blocks banking: it neither heads nor continues a
/// chain a fader would follow.
pub(crate) fn placement_levels(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    levels: &[DependencyLevel],
    classes: &SessionPoolClasses,
    per_node: &BTreeSet<EffectNodeId>,
    (chains, programs): &RackChains,
    width: Option<BankWidth>,
) -> Result<PlacementLevels, GraphDiagnostic> {
    let invariant = || diag("graph.internal.invariant", "$.graph");
    let index_of = |id: &GraphNodeId| nodes.binary_search_by(|node| node.id.cmp(id)).ok();
    let mut asap = vec![0u64; nodes.len()];
    for level in levels {
        for id in &level.nodes {
            asap[index_of(id).ok_or_else(invariant)?] = level.level;
        }
    }
    let mut ends: Vec<(usize, usize)> = Vec::with_capacity(edges.len());
    let mut strip_producer: Vec<Option<usize>> = vec![None; nodes.len()];
    for edge in edges {
        let source = index_of(&edge.source.node).ok_or_else(invariant)?;
        let destination = index_of(&edge.destination.node).ok_or_else(invariant)?;
        ends.push((source, destination));
        if matches!(edge.id, GraphEdgeId::TrackMain { .. })
            && edge.destination.kind == GraphPortKind::MainInput
        {
            strip_producer[destination] = Some(source);
        }
    }
    // ALAP, deepest first: a consumer's level is always above its producer's.
    let depth = levels.last().map_or(0, |level| level.level);
    let output = nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }));
    let mut alap = vec![depth; nodes.len()];
    if let Some(output) = output {
        alap[output] = asap[output];
    }
    let mut by_source: Vec<Vec<usize>> = vec![Vec::new(); nodes.len()];
    for &(source, destination) in &ends {
        by_source[source].push(destination);
    }
    for level in levels.iter().rev() {
        for id in &level.nodes {
            let index = index_of(id).ok_or_else(invariant)?;
            for &consumer in &by_source[index] {
                alap[index] = alap[index].min(alap[consumer].saturating_sub(1));
            }
            alap[index] = alap[index].max(asap[index]);
        }
    }
    // An elided stage boundary: carries the signal without an op.
    let transparent = |index: usize| {
        matches!(
            &nodes[index].id,
            GraphNodeId::TrackStage {
                stage: TrackStage::PostSimd1
                    | TrackStage::PostDynamic
                    | TrackStage::PostSimd2PreFader,
                ..
            }
        )
    };
    // The first producer up the strip's main chain that is not an elided boundary.
    let feeder = |index: usize| {
        let mut cursor = strip_producer[index]?;
        while transparent(cursor) {
            cursor = strip_producer[cursor]?;
        }
        Some(cursor)
    };
    let banked_effect = |index: usize| matches!(&nodes[index].id, GraphNodeId::Effect(effect) if !per_node.contains(effect));
    let key_of = |index: usize| -> Option<PlacementKey> {
        match &nodes[index].id {
            GraphNodeId::TrackStage {
                track_id,
                stage: stage @ (TrackStage::PostInputBuiltins | TrackStage::PostFader),
            } => {
                let follows =
                    *stage == TrackStage::PostFader && feeder(index).is_some_and(banked_effect);
                (!follows).then(|| PlacementKey::Stage(*stage, classes.class_of(track_id.as_str())))
            }
            GraphNodeId::Effect(effect) => {
                // A chain's first slot: its main producer is not a slot of the same chain.
                let continues = strip_producer[index].is_some_and(|producer| {
                    matches!(&nodes[producer].id, GraphNodeId::Effect(previous)
                        if previous.track_id == effect.track_id && previous.rack == effect.rack)
                });
                (!continues).then(|| {
                    PlacementKey::Chain(effect.rack, classes.class_of(effect.track_id.as_str()))
                })
            }
            _ => None,
        }
    };
    let placement_nodes: Vec<rack_compiler::PlacementNode<PlacementKey>> = (0..nodes.len())
        .map(|index| rack_compiler::PlacementNode {
            window: rack_compiler::CohortWindow {
                asap: asap[index],
                alap: alap[index],
            },
            key: key_of(index),
        })
        .collect();
    // What each bank planner groups, keyed as it keys it.
    let mut planned: Vec<Option<PlannedNode>> = vec![None; nodes.len()];
    for (chain, chain_nodes) in chains {
        let Some(first) = chain_nodes.first() else {
            continue;
        };
        let Some(index) = index_of(&GraphNodeId::Effect(first.clone())) else {
            continue;
        };
        // A chain that cannot bank is never in a group (the planner lists it as scalar).
        if !programs[chain].is_bankable() {
            continue;
        }
        planned[index] = Some(PlannedNode::Chain(CohortCandidate {
            id: chain.clone(),
            class: classes.class_of(&chain.track_id),
            program: programs[chain].clone(),
        }));
    }
    for (index, node) in nodes.iter().enumerate() {
        if let GraphNodeId::TrackStage {
            track_id,
            stage:
                stage @ (TrackStage::PostInputBuiltins | TrackStage::PostFader | TrackStage::PostMatrix),
        } = &node.id
        {
            planned[index] = Some(PlannedNode::Stage(
                *stage,
                classes.class_of(track_id.as_str()),
            ));
        }
    }
    let banks_of = |set: &[usize]| -> usize {
        let Some(width) = width else {
            return set
                .iter()
                .filter(|index| planned[**index].is_some())
                .count();
        };
        let mut effect_candidates = Vec::new();
        let mut stage_candidates = Vec::new();
        for &index in set {
            match &planned[index] {
                Some(PlannedNode::Chain(candidate)) => effect_candidates.push(candidate.clone()),
                Some(PlannedNode::Stage(stage, class)) => stage_candidates.push(CohortCandidate {
                    id: index,
                    program: RackProgram::new(RackLocation::Simd1, vec![StageSlot(*stage)]),
                    class: *class,
                }),
                None => {}
            }
        }
        // Candidate ids are unique (one chain per strip and rack, one node per stage index), so
        // the planner's only error cannot occur here.
        let groups = |count: Result<usize, _>| count.unwrap_or(0);
        groups(
            plan_bank_groups(
                &[CohortLevel {
                    level: 0,
                    candidates: effect_candidates,
                }],
                width,
            )
            .map(|plan| plan.groups.len()),
        ) + groups(
            plan_bank_groups(
                &[CohortLevel {
                    level: 0,
                    candidates: stage_candidates,
                }],
                width,
            )
            .map(|plan| plan.groups.len()),
        )
    };
    let placed = rack_compiler::place_over_windows(&placement_nodes, &ends, banks_of)
        .map_err(|_| invariant())?;
    let planned_levels: BTreeMap<GraphNodeId, u64> = placed
        .iter()
        .enumerate()
        // With no bank width nothing banks, so nothing is counted.
        .filter(|(index, _)| width.is_some() && planned[*index].is_some())
        .map(|(index, placement)| (nodes[index].id.clone(), placement.level))
        .collect();
    let mut by_level: BTreeMap<u64, Vec<GraphNodeId>> = BTreeMap::new();
    let mut by_node = BTreeMap::new();
    for (index, placement) in placed.iter().enumerate() {
        by_level
            .entry(placement.level)
            .or_default()
            .push(nodes[index].id.clone());
        // A segment's later stages follow its first, so their window is their placement alone:
        // the sweep can merge them only where their segment heads merge.
        let window = if placement_nodes[index].key.is_some() {
            (placement.floor, alap[index])
        } else {
            (placement.level, placement.level)
        };
        by_node.insert(
            nodes[index].id.clone(),
            (placement.level, window.0, window.1),
        );
    }
    let mut neighbours: BTreeMap<EffectNodeId, (Vec<GraphNodeId>, Vec<GraphNodeId>)> =
        BTreeMap::new();
    for &(source, destination) in &ends {
        if let GraphNodeId::Effect(effect) = &nodes[destination].id
            && effect.rack == RackId::Dynamic
        {
            neighbours
                .entry(effect.clone())
                .or_default()
                .0
                .push(nodes[source].id.clone());
        }
        if let GraphNodeId::Effect(effect) = &nodes[source].id
            && effect.rack == RackId::Dynamic
        {
            neighbours
                .entry(effect.clone())
                .or_default()
                .1
                .push(nodes[destination].id.clone());
        }
    }
    Ok(PlacementLevels {
        neighbours,
        levels: by_level
            .into_iter()
            .map(|(level, mut nodes)| {
                nodes.sort();
                DependencyLevel { level, nodes }
            })
            .collect(),
        by_node,
        planned_levels,
    })
}

/// A node a bank planner groups, keyed exactly as that planner keys it ([`placement_levels`]).
#[derive(Clone, Debug)]
enum PlannedNode {
    /// A rack chain's first slot: the rack planner's candidate.
    Chain(CohortCandidate<RackChainId>),
    /// A builtin strip stage of a strip in a pool class: the builtin planner's candidate.
    Stage(TrackStage, CohortPoolClass),
}

/// The builtin planner's slot key, one per stage: it plans each stage in a call of its own with
/// a fieldless key, which groups exactly as one call keyed by stage.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StageSlot(TrackStage);

impl rack::BankSlotKey for StageSlot {}

/// Strip ID -> the sorted destination IDs of its routes (issue #2132): the fold target
/// [`BankUtilisation::of`] compares. Every route counts, muted or not, because a muted route keeps
/// its edge.
fn route_fold_targets(model: &session::SessionModel) -> BTreeMap<String, Vec<String>> {
    let mut targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for route in &model.routes {
        let source = match &route.source {
            session::RouteSource::Track { track_id, .. } => track_id,
            session::RouteSource::Submix { submix_id, .. } => submix_id,
        };
        let destination = match &route.destination {
            session::RouteDestination::SubmixInput { submix_id } => submix_id,
            session::RouteDestination::OutputInput { output_id } => output_id,
        };
        targets
            .entry(source.as_str().to_owned())
            .or_default()
            .push(destination.as_str().to_owned());
    }
    for destinations in targets.values_mut() {
        destinations.sort();
        destinations.dedup();
    }
    targets
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
///   for an insert remainder, which renders per node below [`min_padded_insert_members`] by choice
///   and is padded from it (#888).
/// * **The mono pool is the larger win.** A padded mono bank collapses to one plane, and the pool
///   is worth about 36 % on the standing console row (M3). A restated objective (planes x banks)
///   would move a mono remainder only when it fits in the stereo pool's padding at every slot and
///   builtin stage, to save one one-plane padded bank per stage; weighing that against an insert
///   remainder the move might fill needs a per-node cost that nothing has measured at four lanes.
///
/// A session with no console keeps #971's rule for an insert remainder below
/// [`min_padded_insert_members`]: it still renders per node, so a stranded mono track still banks
/// nowhere, and moving it can still gain a bank. A remainder at or above the threshold is padded
/// (#888), binds, and strands nobody.
fn stranded_mono_tracks(plan: &BankPlan<RackChainId>, width: BankWidth) -> Vec<String> {
    let mut stranded: BTreeMap<&str, bool> = BTreeMap::new();
    for group in &plan.groups {
        let binds_nothing = group.class == CohortPoolClass::MonoSymmetricAtPrepare
            && !group.is_full()
            && !pads(group, width);
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
/// at the build's own width this is unreachable ([`bind_rack_banks_indexed`] says why); it guards a
/// regression in the planner, the binder or a factory, which would otherwise surface only as a
/// slower render.
///
/// At a `dispatch` other than `Backend::current()` a factory may legally decline the width (D4),
/// so the same slot is refused with `console.slot.foreign_width` at
/// `$.console.<section>[slot=<id>].bank[pool=<class>,level=<level>,width=<lanes>]` instead: the
/// condition is the width, not a lost slot (#1694).
///
/// Only a vector backend reaches this. A plan compiled at the test-only `Scalar` oracle (#1059)
/// binds no bank at all, and it is exempt by construction: it is the per-node reference that the
/// banked render is compared with.
fn unbanked_console_slot(
    dispatch: Backend,
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
    let slot = node.effect_id.as_str();
    if dispatch != Backend::current() {
        let Some(width) = BankWidth::for_backend(dispatch) else {
            return Some(diag("graph.internal.invariant", "$.dispatch"));
        };
        return Some(diag(
            "console.slot.foreign_width",
            &format!(
                "$.console.{section}[slot={slot}].bank[pool={pool},level={level},width={}]",
                width.lanes()
            ),
        ));
    }
    Some(diag(
        "console.slot.unbanked",
        &format!("$.console.{section}[slot={slot}].bank[pool={pool},level={level}]"),
    ))
}

/// The diagnostic for the first prepared console entry, in preparation order, that no collected
/// chain holds, or `None` when the strip walk collected every one.
///
/// [`unbanked_console_slot`] sees only the chains the walk collected, so a walk that skipped a
/// strip's console racks would leave those slots per node with no refusal. This takes the console
/// population from the prepared entries instead and refuses a missing one with the same
/// `console.slot.unbanked` spelling. A prepared entry with no node, or a node with no level, is
/// `graph.internal.invariant`.
fn uncollected_console_slot<E: AsProjected>(
    effects: &EffectInputs<'_, E>,
    ids: &[Option<EffectNodeId>],
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<&EffectNodeId, u64>,
    classes: &SessionPoolClasses,
) -> Option<GraphDiagnostic> {
    let collected: BTreeSet<&EffectNodeId> = chains.values().flatten().collect();
    for (index, entry) in effects.iter().enumerate() {
        if !is_console_rack(rack_location(crate::ids::rack_id(entry.rack))) {
            continue;
        }
        let Some(node) = ids.get(index).and_then(Option::as_ref) else {
            return Some(diag("graph.internal.invariant", "$.effects"));
        };
        if collected.contains(node) {
            continue;
        }
        let section = match node.rack {
            RackId::Simd1 => "pre_insert",
            _ => "post_insert",
        };
        let pool = match classes.class_of(node.track_id.as_str()) {
            CohortPoolClass::MonoSymmetricAtPrepare => "mono",
            CohortPoolClass::Stereo => "stereo",
        };
        let Some(level) = level_by_node.get(node) else {
            return Some(diag("graph.internal.invariant", "$.effects"));
        };
        return Some(diag(
            "console.slot.unbanked",
            &format!(
                "$.console.{section}[slot={}].bank[pool={pool},level={level}]",
                node.effect_id.as_str()
            ),
        ));
    }
    None
}

/// The banks planned from one plan group's slots, each with its report entry, in slot order.
type BoundGroup = Vec<(PlannedEffectBank, GraphRackBoundSlot)>;

/// One bank the compile's factories bind, planned without its processor (issue #2199): its members
/// on lanes `0..members`, its active mask, the metadata its factory's bind returned and its cohort.
/// [`bind_planned_banks`] binds it.
#[derive(Clone, Debug)]
pub(crate) struct PlannedEffectBank {
    pub(crate) members: Box<[EffectNodeId]>,
    pub(crate) active_mask: Box<[bool]>,
    pub(crate) metadata: effect_contract::PreparedBankMetadata,
    pub(crate) cohort: graph::GraphBankCohort,
}

/// The shape of a bank the estimate charges: a planned bank, or a bound one.
pub(crate) trait BankLayout {
    fn members(&self) -> &[EffectNodeId];
    fn active_mask(&self) -> &[bool];
    fn metadata(&self) -> &effect_contract::PreparedBankMetadata;
}

impl BankLayout for PlannedEffectBank {
    fn members(&self) -> &[EffectNodeId] {
        &self.members
    }
    fn active_mask(&self) -> &[bool] {
        &self.active_mask
    }
    fn metadata(&self) -> &effect_contract::PreparedBankMetadata {
        &self.metadata
    }
}

impl BankLayout for graph::GraphPreparedEffectBank {
    fn members(&self) -> &[EffectNodeId] {
        &self.members
    }
    fn active_mask(&self) -> &[bool] {
        &self.active_mask
    }
    fn metadata(&self) -> &effect_contract::PreparedBankMetadata {
        &self.metadata
    }
}

/// The one request per lane a bank of `members` is bound with: each member's own on its active
/// lane and, on every padded lane, a clone of the first member's -- never zeros (issue #1088, the
/// padding contract). The planner emits members before padding, so the members are exactly lanes
/// `0..members`.
fn bank_requests<'e, E: AsProjected>(
    members: &[EffectNodeId],
    effects: &'e EffectInputs<'_, E>,
    prepared: &PreparedEffectIndex<'_>,
    width: BankWidth,
) -> Result<
    (
        &'e EffectProjectedEntry,
        Vec<effect_contract::PrepareEffectRequest<'e>>,
    ),
    GraphDiagnostic,
> {
    let invariant = || diag("graph.internal.invariant", "$.effects");
    let mut requests = Vec::with_capacity(width.lanes() as usize);
    let mut first = None;
    for node in members {
        let slot = prepared
            .get(node.track_id.as_str(), node.rack, node.effect_id.as_str())
            .ok_or_else(invariant)?;
        let entry = effects.entry(slot.index()).ok_or_else(invariant)?;
        first.get_or_insert(entry);
        requests.push(entry.bank_preparation.request());
    }
    let first = first.ok_or_else(invariant)?;
    requests.resize(width.lanes() as usize, requests[0]);
    Ok((first, requests))
}

/// Plans every slot of plan group `group_index` that can be one homogeneous bank and whose
/// factory consents. `padded` says whether a partial group is padded (issue #1088).
///
/// The factory's consent is its bind's own answer: each slot is bound and the bank dropped at once
/// (issue #2199), keeping its metadata, so the plan is exactly the banks the bind will return.
#[allow(clippy::too_many_arguments)] // The binder's whole context, passed from one closure.
fn plan_group_banks<E: AsProjected>(
    group_index: usize,
    group: &BankGroup<RackChainId>,
    padded: bool,
    chains: &BTreeMap<RackChainId, Vec<EffectNodeId>>,
    level_by_node: &BTreeMap<&EffectNodeId, u64>,
    effects: &EffectInputs<'_, E>,
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
        let (first, requests) = bank_requests(&members, effects, prepared, width)?;
        let request = PrepareEffectBankRequest {
            backend: dispatch,
            width,
            requests: &requests,
            active_mask: &group.active_mask,
        };
        // Equal program key implies the same registry factory: the registry maps one
        // `EffectId` to one `Arc` (#96 F12), so a per-chunk `Arc::ptr_eq` scan proved nothing.
        let Some(effect_contract::PreparedEffectBank { metadata, .. }) = first
            .factory
            .bind_homogeneous_bank(request)
            .map_err(|error| diag(error.code, "$.effects"))?
        else {
            continue;
        };
        // Issue #1461: the bind result carries the bank's metadata beside the processor, which
        // holds no copy of it.
        if metadata.width != width || metadata.program_key != group.program[slot] {
            return Err(diag("graph.effect.bank_metadata", "$.effects"));
        }
        // `rack::AoSoaScratch::new`'s refusals, checked here so the plan holds only banks whose
        // scratch [`bind_planned_banks`] can allocate.
        let quantum = effects.session.quantum().0;
        if quantum == 0
            || (quantum as usize)
                .checked_mul(width.lanes() as usize)
                .is_none()
        {
            return Err(diag("graph.resource.arithmetic_overflow", "$.graph"));
        }
        let bank = PlannedEffectBank {
            members: members.clone().into_boxed_slice(),
            active_mask: group.active_mask.clone(),
            metadata,
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

/// Bind every planned bank (issue #2199), in plan order: each slot's members' requests, the
/// planned mask, the factory of its first member. A bank its factory declines, or binds with other
/// metadata, contradicts the plan the compile's resources were admitted for, and is
/// `graph.effect.bank_metadata`; a factory error is its own code, as the plan's bind's was.
pub(crate) fn bind_planned_banks<E: AsProjected>(
    planned: Vec<PlannedEffectBank>,
    effects: &EffectInputs<'_, E>,
    prepared: &PreparedEffectIndex<'_>,
    dispatch: Backend,
) -> Result<Vec<graph::GraphPreparedEffectBank>, GraphDiagnostic> {
    let mut banks = Vec::with_capacity(planned.len());
    for plan in planned {
        let width = plan.metadata.width;
        let (first, requests) = bank_requests(&plan.members, effects, prepared, width)?;
        let Some(effect_contract::PreparedEffectBank {
            processor,
            metadata,
        }) = first
            .factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: dispatch,
                width,
                requests: &requests,
                active_mask: &plan.active_mask,
            })
            .map_err(|error| diag(error.code, "$.effects"))?
        else {
            return Err(diag("graph.effect.bank_metadata", "$.effects"));
        };
        if metadata != plan.metadata {
            return Err(diag("graph.effect.bank_metadata", "$.effects"));
        }
        let scratch = rack::AoSoaScratch::new(width, effects.session.quantum().0)
            .map_err(|_| diag("graph.resource.arithmetic_overflow", "$.graph"))?;
        banks.push(graph::GraphPreparedEffectBank {
            members: plan.members,
            active_mask: plan.active_mask,
            processor,
            metadata,
            response_snapshot_declared: first.factory.response_analysis().is_some(),
            native_id: first.factory.descriptor().id.as_str(),
            scratch,
            cohort: plan.cohort,
        });
    }
    Ok(banks)
}

/// #96 F7, the one predicate: a slot can be one homogeneous bank only when every member lane runs
/// it, because the effect contract has no per-lane identity slot. `lanes` are the member lanes'
/// slot masks over the leader program (padding lanes excluded). Shared by the binder
/// ([`bindable_slot_members`]) and the utilisation report's merge test
/// (`BankUtilisation::of`), so the report never counts a merge the binder would refuse.
pub(crate) fn every_member_runs<'a>(
    lanes: impl IntoIterator<Item = &'a [bool]>,
    slot: usize,
) -> bool {
    lanes
        .into_iter()
        .all(|lane| lane.get(slot).copied().unwrap_or(false))
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
    if !every_member_runs(
        group
            .active_slots
            .iter()
            .zip(&group.active_mask)
            .filter(|(_, active)| **active)
            .map(|(lane, _)| &lane[..]),
        slot,
    ) {
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
/// padded lane still occupies its AoSoA column; member metadata for the members only. The runtime
/// buffers are the chain's staging planes, plus a padded bank's `+0.0` plane (issue #887).
pub(crate) fn effect_bank_resource<B: BankLayout>(
    banks: &[B],
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
        // A bound bank's scratch is `AoSoaScratch::new(metadata.width, quantum)`
        // (`bind_planned_banks`), so the planned bank's width and the session quantum are its
        // shape.
        let lanes = u64::from(bank.metadata().width.lanes());
        let members = u64::try_from(bank.members().len()).ok()?;
        if members == 0
            || members > lanes
            || u64::try_from(bank.active_mask().len()).ok()? != lanes
            || bank
                .active_mask()
                .iter()
                .enumerate()
                .any(|(lane, active)| *active != (lane < bank.members().len()))
        {
            return None;
        }
        let scratch_plane_samples = u64::from(quantum).checked_mul(lanes)?;
        let scratch_plane_bytes = scratch_plane_samples.checked_mul(4)?;
        // L and R only: the sidechain planes were never read (#96 F9).
        let scratch_samples = scratch_plane_samples.checked_mul(2)?;
        let scratch_bytes = scratch_samples.checked_mul(4)?;
        // The chain's two lane-major staging planes, and for a padded bank the `quantum`-word
        // `+0.0` plane its tiled gather reads for an inactive lane (issue #887).
        let zero_plane_samples = if members < lanes {
            u64::from(quantum)
        } else {
            0
        };
        let runtime_buffer_samples = scratch_plane_samples
            .checked_mul(2)?
            .checked_add(zero_plane_samples)?;
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
        for member in bank.members() {
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
