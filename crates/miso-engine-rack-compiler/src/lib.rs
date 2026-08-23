//! Deterministic off-render cohort selection for homogeneous AoSoA racks.
//!
//! [`plan_bank_groups`] is the **single** cohort planner in the workspace (#96 F1). Everything that
//! forms SIMD-rack or builtin banks calls it:
//!
//! * `miso-engine-graph-compiler::bind_rack_banks` (SIMD rack 1 / rack 2 effect banks) passes one
//!   candidate per effect node with a one-slot program, `id = EffectNodeId`, and levels taken from
//!   the graph's dependency levels. It binds only [`BankGroup::is_full`] groups, because every
//!   effect factory rejects `requests.len() != lanes` today (#96 F7); padded groups' members render
//!   on the per-node scalar path, exactly as they did before #96.
//! * #86 (builtin banks) passes one candidate per track with
//!   `id = GraphNodeId::TrackStage { .., PostInputBuiltins }` and a one-slot program holding the
//!   fixed builtin stage key. Both full and padded groups are bindable there: the frozen #85
//!   constructor takes the active member count and treats lanes `[active, W)` as padding.
//!   Padding lanes are always the highest lane indices, which is exactly what that contract
//!   requires.
//! * #99 (chain-level lowering) passes one candidate per `(track, rack)` whose `slots` are the
//!   track's rack chain **in session order** (`track.simd1.effects`), *not* the
//!   `EffectPreparedSession::entries` order, which is sorted by effect id. Each slot `s` binds one
//!   bank with lane mask `active_slots[*][s]`; `BankChain` skips slots that
//!   [`BankGroup::slot_is_identity_everywhere`].
#![allow(missing_docs)]

use core::cmp::Ordering;
use miso_engine_effect_contract::{BankWidth, EffectProgramKeyV1};
use miso_engine_rack::{RackLocationV1, RackProgramV1};

/// One track's ordered rack program, addressed by a caller-chosen stable id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CohortCandidate<Id> {
    pub id: Id,
    pub program: RackProgramV1,
}

/// Candidates already partitioned by dependency level by the caller: a bank never crosses a level,
/// because its members must all be ready in the same wave (#96 F12).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CohortLevel<Id> {
    pub level: u64,
    pub candidates: Vec<CohortCandidate<Id>>,
}

/// One planned bank: a cohort leader's program plus the lanes that run it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankGroup<Id> {
    pub level: u64,
    pub rack: RackLocationV1,
    /// The cohort leader's ordered program: a chain of `program.len()` slots.
    pub program: Box<[EffectProgramKeyV1]>,
    /// `len == width.lanes()`. `None` is a padding lane; padding lanes are always the highest
    /// lane indices.
    pub members: Box<[Option<Id>]>,
    /// `active_mask[i] == members[i].is_some()`.
    pub active_mask: Box<[bool]>,
    /// `active_slots[i][s]`: lane `i` runs slot `s`; `false` is an identity slot for that lane.
    /// Padding lanes are all `false`.
    pub active_slots: Box<[Box<[bool]>]>,
}

impl<Id> BankGroup<Id> {
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.active_mask.iter().all(|lane| *lane)
    }
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.active_mask.iter().filter(|lane| **lane).count()
    }
    /// A slot no lane runs. `BankChain` skips exactly these.
    #[must_use]
    pub fn slot_is_identity_everywhere(&self, slot: usize) -> bool {
        !self
            .active_slots
            .iter()
            .any(|lane| lane.get(slot).copied().unwrap_or(false))
    }
    /// The leader program as a [`RackProgramV1`], for callers that re-derive masks.
    #[must_use]
    pub fn program_v1(&self) -> RackProgramV1 {
        RackProgramV1 {
            rack: self.rack,
            slots: self.program.clone(),
        }
    }
}

/// The planner's whole output: banked groups plus the candidates that never bank.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankPlan<Id> {
    pub groups: Vec<BankGroup<Id>>,
    /// Candidates that never bank (empty program, or a connected sidechain), in `id` order.
    pub scalar: Vec<Id>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RackCompileError {
    DuplicateId,
}

struct WorkingMember<Id> {
    id: Id,
    program: RackProgramV1,
    mask: Box<[bool]>,
}

impl<Id> WorkingMember<Id> {
    fn active_count(&self) -> usize {
        self.mask.iter().filter(|slot| **slot).count()
    }
}

struct WorkingGroup<Id> {
    program: RackProgramV1,
    members: Vec<WorkingMember<Id>>,
}

fn order_members<Id: Ord>(members: &mut [WorkingMember<Id>]) {
    // F5.2: tracks that run the whole leader program fill banks first, so a partial group's
    // padding displaces the *least* work.
    members.sort_by(|a, b| {
        b.active_count()
            .cmp(&a.active_count())
            .then_with(|| a.id.cmp(&b.id))
    });
}

/// The single cohort planner.
///
/// There is deliberately **no** remainder-placement pass. A member stranded in a partial group can
/// never fit an earlier partial group: within one cohort every group but the last is full, and a
/// member of a *later* cohort is by construction not a subsequence of an earlier cohort's leader --
/// otherwise step 2's pooling, which is exhaustive, would already have placed it there.
/// `pooling_is_exhaustive_so_no_member_is_stranded` gates that argument.
///
/// Deterministic: the output depends only on the multiset of `(level, id, program)` and on `width`,
/// never on input order. Partial groups are padded rather than dropped (#96 F6); a program that is
/// a subsequence of its cohort leader's joins that cohort with identity slots where it has no
/// effect (#96 F1/F5).
///
/// # Errors
/// [`RackCompileError::DuplicateId`] if the same id appears twice, including across levels.
pub fn plan_bank_groups<Id: Ord + Clone>(
    members_by_level: &[CohortLevel<Id>],
    width: BankWidth,
) -> Result<BankPlan<Id>, RackCompileError> {
    let lanes = width.lanes() as usize;
    let mut all_ids: Vec<&Id> = members_by_level
        .iter()
        .flat_map(|level| level.candidates.iter().map(|candidate| &candidate.id))
        .collect();
    all_ids.sort();
    if all_ids.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(RackCompileError::DuplicateId);
    }
    let mut by_level: std::collections::BTreeMap<u64, Vec<&CohortCandidate<Id>>> =
        std::collections::BTreeMap::new();
    for level in members_by_level {
        by_level
            .entry(level.level)
            .or_default()
            .extend(level.candidates.iter());
    }

    let mut groups = Vec::new();
    let mut scalar = Vec::new();
    for (level, candidates) in by_level {
        for rack in [RackLocationV1::Simd1, RackLocationV1::Simd2] {
            // No canonicalising sort here: leader selection is a total `max_by` over unique ids,
            // `order_members` fixes every group's lane order, and `scalar` is sorted on the way
            // out, so the plan cannot depend on pool order. `output_is_input_order_invariant`
            // is the gate on that claim.
            let mut pool: Vec<&CohortCandidate<Id>> = candidates
                .iter()
                .copied()
                .filter(|candidate| candidate.program.rack == rack)
                .collect();
            pool.retain(|candidate| {
                if candidate.program.is_bankable() {
                    true
                } else {
                    scalar.push(candidate.id.clone());
                    false
                }
            });

            let mut rack_groups: Vec<WorkingGroup<Id>> = Vec::new();
            while !pool.is_empty() {
                let leader = pool
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| {
                        a.program
                            .slots
                            .len()
                            .cmp(&b.program.slots.len())
                            .then_with(|| b.program.slots.cmp(&a.program.slots))
                            .then_with(|| b.id.cmp(&a.id))
                    })
                    .map(|(index, _)| index)
                    .unwrap_or(0);
                let leader_program = pool[leader].program.clone();
                let mut compatible = Vec::new();
                let mut rest = Vec::new();
                for candidate in pool {
                    match candidate.program.subsequence_mask(&leader_program) {
                        Some(mask) => compatible.push(WorkingMember {
                            id: candidate.id.clone(),
                            program: candidate.program.clone(),
                            mask,
                        }),
                        None => rest.push(candidate),
                    }
                }
                order_members(&mut compatible);
                let mut remaining = compatible.into_iter().peekable();
                while remaining.peek().is_some() {
                    let members: Vec<_> = remaining.by_ref().take(lanes).collect();
                    rack_groups.push(WorkingGroup {
                        program: leader_program.clone(),
                        members,
                    });
                }
                pool = rest;
            }

            groups.extend(
                rack_groups
                    .into_iter()
                    .map(|group| materialize(level, rack, group, lanes)),
            );
        }
    }
    scalar.sort();
    let plan = BankPlan { groups, scalar };
    debug_assert!(plan_invariants_hold(&plan, lanes));
    Ok(plan)
}

fn materialize<Id>(
    level: u64,
    rack: RackLocationV1,
    group: WorkingGroup<Id>,
    lanes: usize,
) -> BankGroup<Id> {
    let slots = group.program.slots.len();
    let mut members: Vec<Option<Id>> = Vec::with_capacity(lanes);
    let mut active_slots: Vec<Box<[bool]>> = Vec::with_capacity(lanes);
    for member in group.members {
        members.push(Some(member.id));
        active_slots.push(member.mask);
    }
    while members.len() < lanes {
        members.push(None);
        active_slots.push(vec![false; slots].into_boxed_slice());
    }
    let active_mask: Vec<bool> = members.iter().map(Option::is_some).collect();
    BankGroup {
        level,
        rack,
        program: group.program.slots,
        members: members.into_boxed_slice(),
        active_mask: active_mask.into_boxed_slice(),
        active_slots: active_slots.into_boxed_slice(),
    }
}

fn plan_invariants_hold<Id: Ord + Clone>(plan: &BankPlan<Id>, lanes: usize) -> bool {
    let mut seen: Vec<Id> = plan.scalar.clone();
    for group in &plan.groups {
        if group.members.len() != lanes
            || group.active_mask.len() != lanes
            || group.active_slots.len() != lanes
            || group.active_count() == 0
        {
            return false;
        }
        let mut padding = false;
        for lane in 0..lanes {
            let present = group.members[lane].is_some();
            if group.active_mask[lane] != present
                || group.active_slots[lane].len() != group.program.len()
            {
                return false;
            }
            if present && padding {
                return false;
            }
            if !present {
                padding = true;
                if group.active_slots[lane].iter().any(|slot| *slot) {
                    return false;
                }
            }
            if let Some(id) = &group.members[lane] {
                seen.push(id.clone());
            }
        }
    }
    seen.sort();
    !seen.windows(2).any(|pair| pair[0] == pair[1])
}

/// Total order over program keys, exposed so callers can reproduce cohort order in tests.
#[must_use]
pub fn compare_programs(a: &RackProgramV1, b: &RackProgramV1) -> Ordering {
    a.rack.cmp(&b.rack).then_with(|| a.slots.cmp(&b.slots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::{
        EffectId, EffectQuality, LatencySamples, LinkMode, PortId, PreparedPortsV1,
        PreparedSidechainPort, StatePayloadSizes, TailSamples,
    };
    use std::collections::BTreeMap;

    fn key(index: usize) -> EffectProgramKeyV1 {
        key_with(index, PreparedSidechainPort::None)
    }

    fn key_with(index: usize, sidechain: PreparedSidechainPort) -> EffectProgramKeyV1 {
        let effect_id = match index {
            0 => EffectId::new("fixture.a"),
            1 => EffectId::new("fixture.b"),
            2 => EffectId::new("fixture.c"),
            _ => EffectId::new("fixture.d"),
        }
        .expect("static id");
        EffectProgramKeyV1 {
            effect_id,
            contract_major: 1,
            state_layout_version: 1,
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 { sidechain },
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
            state_sizes: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_bytes: 0,
            automation_capacity: 0,
        }
    }

    fn program(slots: &[usize]) -> RackProgramV1 {
        RackProgramV1::new(
            RackLocationV1::Simd1,
            slots.iter().copied().map(key).collect(),
        )
    }

    fn candidate(id: u32, slots: &[usize]) -> CohortCandidate<u32> {
        CohortCandidate {
            id,
            program: program(slots),
        }
    }

    fn one_level(candidates: Vec<CohortCandidate<u32>>) -> Vec<CohortLevel<u32>> {
        vec![CohortLevel {
            level: 0,
            candidates,
        }]
    }

    fn members(group: &BankGroup<u32>) -> Vec<Option<u32>> {
        group.members.to_vec()
    }

    fn full_group_members(plan: &BankPlan<u32>) -> Vec<Vec<u32>> {
        plan.groups
            .iter()
            .filter(|group| group.is_full())
            .map(|group| group.members.iter().map(|id| id.expect("full")).collect())
            .collect()
    }

    fn splitmix(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// The rule `bind_rack_banks` used before #96, reimplemented over the same inputs: group by
    /// `(rack, program)`, chunk in id order, drop partial chunks and level-mixed chunks.
    fn legacy_full_banks(
        candidates: &[(u64, CohortCandidate<u32>)],
        lanes: usize,
    ) -> Vec<Vec<u32>> {
        let mut by_program: BTreeMap<RackProgramV1, Vec<(u64, u32)>> = BTreeMap::new();
        for (level, candidate) in candidates {
            if !candidate.program.is_bankable() {
                continue;
            }
            by_program
                .entry(candidate.program.clone())
                .or_default()
                .push((*level, candidate.id));
        }
        let mut banks = Vec::new();
        for entries in by_program.values_mut() {
            entries.sort_by_key(|(_, id)| *id);
            for chunk in entries.chunks(lanes) {
                if chunk.len() == lanes && chunk.iter().all(|(level, _)| *level == chunk[0].0) {
                    banks.push(chunk.iter().map(|(_, id)| *id).collect());
                }
            }
        }
        banks
    }

    /// P1: on one-slot programs the planner reproduces the pre-#96 chunking exactly whenever no
    /// `(rack, program)` cohort spans two dependency levels, and otherwise partitions by level
    /// first (#96 F12) — the only intended membership difference.
    #[test]
    fn single_slot_programs_reproduce_exact_equal_chunking() {
        let mut state = 0x5eed_0096_u64;
        for case in 0..200u32 {
            let width = if case % 2 == 0 {
                BankWidth::Four
            } else {
                BankWidth::Eight
            };
            let lanes = width.lanes() as usize;
            let count = 1 + (splitmix(&mut state) % 40) as u32;
            let programs = 1 + (splitmix(&mut state) % 3) as usize;
            let levels = 1 + (splitmix(&mut state) % 3);
            let mut flat = Vec::new();
            for id in 0..count {
                let slot = (splitmix(&mut state) % programs as u64) as usize;
                let level = splitmix(&mut state) % levels;
                flat.push((level, candidate(id, &[slot])));
            }
            let mut by_level: BTreeMap<u64, Vec<CohortCandidate<u32>>> = BTreeMap::new();
            for (level, candidate) in &flat {
                by_level.entry(*level).or_default().push(candidate.clone());
            }
            let input: Vec<_> = by_level
                .into_iter()
                .map(|(level, candidates)| CohortLevel { level, candidates })
                .collect();
            let plan = plan_bank_groups(&input, width).expect("plan");

            // Every candidate lands exactly once, padding is trailing, masks agree.
            let mut seen: Vec<u32> = plan.scalar.clone();
            for group in &plan.groups {
                assert_eq!(group.members.len(), lanes);
                let active = group.active_count();
                assert!(active > 0);
                for lane in 0..lanes {
                    assert_eq!(group.active_mask[lane], lane < active);
                    assert_eq!(group.members[lane].is_some(), lane < active);
                    if let Some(id) = group.members[lane] {
                        seen.push(id);
                    }
                }
            }
            seen.sort_unstable();
            assert_eq!(seen, (0..count).collect::<Vec<_>>(), "case={case}");

            // Every group is single-level and single-program.
            for group in &plan.groups {
                let level = group.level;
                for id in group.members.iter().flatten() {
                    let (candidate_level, candidate) = flat
                        .iter()
                        .find(|(_, candidate)| candidate.id == *id)
                        .expect("known id");
                    assert_eq!(*candidate_level, level);
                    assert_eq!(candidate.program.slots.as_ref(), group.program.as_ref());
                }
            }

            let mixed = {
                let mut level_by_program: BTreeMap<RackProgramV1, u64> = BTreeMap::new();
                let mut mixed = false;
                for (level, candidate) in &flat {
                    match level_by_program.get(&candidate.program) {
                        Some(seen) if seen != level => mixed = true,
                        _ => {
                            level_by_program.insert(candidate.program.clone(), *level);
                        }
                    }
                }
                mixed
            };
            let legacy = legacy_full_banks(&flat, lanes);
            let mut planned = full_group_members(&plan);
            let mut legacy_sorted = legacy.clone();
            planned.sort();
            legacy_sorted.sort();
            if !mixed {
                assert_eq!(planned, legacy_sorted, "case={case} width={lanes}");
            } else {
                // F12: level-mixed cohorts were dropped wholesale before #96; they are now
                // partitioned by level. Every planned full group must still be a level-uniform,
                // program-uniform, id-ordered chunk.
                for group in full_group_members(&plan) {
                    let mut sorted = group.clone();
                    sorted.sort_unstable();
                    assert_eq!(group, sorted, "case={case}");
                }
            }
        }
    }

    /// P2: empty programs and connected sidechains never bank.
    #[test]
    fn empty_programs_and_connected_sidechains_never_bank() {
        let connected = RackProgramV1::new(
            RackLocationV1::Simd1,
            vec![key_with(
                0,
                PreparedSidechainPort::Connected {
                    id: PortId::new("sidechain").expect("static id"),
                    required: true,
                },
            )],
        );
        let unconnected = RackProgramV1::new(
            RackLocationV1::Simd1,
            vec![key_with(
                0,
                PreparedSidechainPort::Unconnected {
                    id: PortId::new("sidechain").expect("static id"),
                    required: false,
                },
            )],
        );
        let candidates = vec![
            CohortCandidate {
                id: 0,
                program: program(&[]),
            },
            CohortCandidate {
                id: 1,
                program: connected,
            },
            CohortCandidate {
                id: 2,
                program: unconnected.clone(),
            },
            CohortCandidate {
                id: 3,
                program: unconnected.clone(),
            },
            CohortCandidate {
                id: 4,
                program: unconnected.clone(),
            },
            CohortCandidate {
                id: 5,
                program: unconnected,
            },
        ];
        let plan = plan_bank_groups(&one_level(candidates), BankWidth::Four).expect("plan");
        assert_eq!(plan.scalar, vec![0, 1]);
        assert_eq!(full_group_members(&plan), vec![vec![2, 3, 4, 5]]);
    }

    /// P3: subsequence membership uses program-key equality, never an occurrence index.
    #[test]
    fn subsequence_uses_program_equality_not_occurrence() {
        let leader = program(&[1, 0, 1]);
        assert_eq!(
            program(&[0, 1]).subsequence_mask(&leader).as_deref(),
            Some(&[false, true, true][..])
        );
        assert_eq!(
            program(&[1, 1]).subsequence_mask(&leader).as_deref(),
            Some(&[true, false, true][..])
        );
        assert!(program(&[0, 0]).subsequence_mask(&leader).is_none());
        assert!(program(&[1, 0, 1, 0]).subsequence_mask(&leader).is_none());
        let other_rack = RackProgramV1::new(RackLocationV1::Simd2, vec![key(1)]);
        assert!(other_rack.subsequence_mask(&leader).is_none());
    }

    /// P4: the longest program leads its cohort and full-program tracks fill banks first. The
    /// short programs deliberately carry the *smaller* ids, so a plan that sorted by id alone
    /// would produce a different lane assignment.
    #[test]
    fn longest_program_leads_and_full_programs_fill_first() {
        let mut candidates = Vec::new();
        for id in 0..4u32 {
            candidates.push(candidate(id, &[0, 2]));
        }
        for id in 4..9u32 {
            candidates.push(candidate(id, &[0, 1, 2]));
        }
        let plan = plan_bank_groups(&one_level(candidates), BankWidth::Four).expect("plan");
        assert_eq!(plan.groups.len(), 3);
        assert_eq!(
            plan.groups[0].program.len(),
            3,
            "the longest program leads the cohort"
        );
        assert_eq!(
            members(&plan.groups[0]),
            vec![Some(4), Some(5), Some(6), Some(7)],
            "full-program tracks fill the first bank even though their ids are larger"
        );
        assert_eq!(
            members(&plan.groups[1]),
            vec![Some(8), Some(0), Some(1), Some(2)]
        );
        assert_eq!(members(&plan.groups[2]), vec![Some(3), None, None, None]);
        assert_eq!(
            plan.groups[1].active_slots[1].as_ref(),
            &[true, false, true],
            "a short program runs its own slots and takes identity elsewhere"
        );
        assert_eq!(
            plan.groups[2].active_slots[1].as_ref(),
            &[false, false, false]
        );
        assert!(!plan.groups[2].is_full());
        assert_eq!(plan.groups[2].active_count(), 1);
    }

    /// P5: pooling is exhaustive, so no member is ever stranded behind an earlier free lane.
    ///
    /// This is what makes a remainder-placement pass unnecessary rather than merely unused: over a
    /// seeded corpus, no partial group ever precedes a member of the same `(level, rack)` whose
    /// program is a subsequence of that group's leader.
    #[test]
    fn pooling_is_exhaustive_so_no_member_is_stranded() {
        let mut state = 0x0517_2600_u64;
        for case in 0..200u32 {
            let width = if case.is_multiple_of(2) {
                BankWidth::Four
            } else {
                BankWidth::Eight
            };
            let lanes = width.lanes() as usize;
            let count = 1 + (splitmix(&mut state) % 30) as u32;
            let mut candidates = Vec::new();
            for id in 0..count {
                let length = 1 + (splitmix(&mut state) % 3) as usize;
                let slots: Vec<usize> = (0..length)
                    .map(|_| (splitmix(&mut state) % 3) as usize)
                    .collect();
                candidates.push(CohortCandidate {
                    id,
                    program: RackProgramV1::new(
                        RackLocationV1::Simd1,
                        slots.into_iter().map(key).collect(),
                    ),
                });
            }
            let by_id: BTreeMap<u32, RackProgramV1> = candidates
                .iter()
                .map(|candidate| (candidate.id, candidate.program.clone()))
                .collect();
            let plan = plan_bank_groups(&one_level(candidates), width).expect("plan");
            for (index, group) in plan.groups.iter().enumerate() {
                if group.active_count() == lanes {
                    continue;
                }
                for later in plan.groups.iter().skip(index + 1) {
                    for member in later.members.iter().flatten() {
                        assert!(
                            by_id[member]
                                .subsequence_mask(&group.program_v1())
                                .is_none(),
                            "case={case}: id {member} could have filled a free lane in group {index}"
                        );
                    }
                }
            }
        }
    }

    /// P6: the plan depends only on the multiset of candidates, never on input order.
    #[test]
    fn output_is_input_order_invariant() {
        let mut state = 0x00c0_ffee_u64;
        let base: Vec<CohortCandidate<u32>> = (0..23u32)
            .map(|id| candidate(id, &[(id % 3) as usize]))
            .collect();
        let reference = plan_bank_groups(&one_level(base.clone()), BankWidth::Four).expect("plan");
        for _ in 0..16 {
            let mut shuffled = base.clone();
            for index in (1..shuffled.len()).rev() {
                let swap = (splitmix(&mut state) % (index as u64 + 1)) as usize;
                shuffled.swap(index, swap);
            }
            let plan = plan_bank_groups(&one_level(shuffled), BankWidth::Four).expect("plan");
            assert_eq!(plan, reference);
        }
    }

    /// P7: duplicate ids are rejected, including across levels.
    #[test]
    fn duplicate_ids_are_rejected_across_levels() {
        let input = vec![
            CohortLevel {
                level: 0,
                candidates: vec![candidate(7, &[0])],
            },
            CohortLevel {
                level: 1,
                candidates: vec![candidate(7, &[0])],
            },
        ];
        assert_eq!(
            plan_bank_groups(&input, BankWidth::Four).err(),
            Some(RackCompileError::DuplicateId)
        );
        assert_eq!(
            plan_bank_groups(
                &one_level(vec![candidate(1, &[0]), candidate(1, &[1])]),
                BankWidth::Four
            )
            .err(),
            Some(RackCompileError::DuplicateId)
        );
    }

    /// P8: the structural invariants hold over a seeded corpus of mixed-length programs.
    #[test]
    fn invariants_hold_on_seeded_corpus() {
        let mut state = 0x1234_5678_u64;
        for case in 0..200u32 {
            let width = if case % 2 == 0 {
                BankWidth::Four
            } else {
                BankWidth::Eight
            };
            let lanes = width.lanes() as usize;
            let count = 1 + (splitmix(&mut state) % 25) as u32;
            let mut by_level: BTreeMap<u64, Vec<CohortCandidate<u32>>> = BTreeMap::new();
            for id in 0..count {
                let length = (splitmix(&mut state) % 4) as usize;
                let slots: Vec<usize> = (0..length)
                    .map(|_| (splitmix(&mut state) % 3) as usize)
                    .collect();
                let rack = if splitmix(&mut state).is_multiple_of(2) {
                    RackLocationV1::Simd1
                } else {
                    RackLocationV1::Simd2
                };
                let level = splitmix(&mut state) % 3;
                by_level.entry(level).or_default().push(CohortCandidate {
                    id,
                    program: RackProgramV1::new(rack, slots.into_iter().map(key).collect()),
                });
            }
            let input: Vec<_> = by_level
                .into_iter()
                .map(|(level, candidates)| CohortLevel { level, candidates })
                .collect();
            let plan = plan_bank_groups(&input, width).expect("plan");
            assert!(plan_invariants_hold(&plan, lanes), "case={case}");
            for group in &plan.groups {
                for lane in 0..lanes {
                    let Some(id) = &group.members[lane] else {
                        continue;
                    };
                    let candidate = input
                        .iter()
                        .flat_map(|level| level.candidates.iter())
                        .find(|candidate| candidate.id == *id)
                        .expect("known id");
                    assert_eq!(candidate.program.rack, group.rack);
                    assert_eq!(
                        candidate
                            .program
                            .subsequence_mask(&group.program_v1())
                            .as_deref(),
                        Some(group.active_slots[lane].as_ref()),
                        "case={case} lane={lane}"
                    );
                }
            }
            let scalar_ids: Vec<u32> = plan.scalar.clone();
            let mut sorted = scalar_ids.clone();
            sorted.sort_unstable();
            assert_eq!(scalar_ids, sorted, "scalar members are id-ordered");
        }
    }

    /// The planner's `(rack, program)` order is a total order; the graph compiler relies on it for
    /// a deterministic bank vector.
    #[test]
    fn program_comparison_is_a_total_order() {
        assert_eq!(
            compare_programs(&program(&[0]), &program(&[0])),
            Ordering::Equal
        );
        assert_eq!(
            compare_programs(&program(&[0]), &program(&[1])),
            Ordering::Less
        );
        assert_eq!(
            compare_programs(
                &program(&[0]),
                &RackProgramV1::new(RackLocationV1::Simd2, vec![key(0)])
            ),
            Ordering::Less
        );
    }
}
