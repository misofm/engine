//! Issues #1088 (console strip P2a) and #1098 (S2): a padded effect bank through the real planner,
//! the real graph and the real rack, under the production policy.
//!
//! Every console group is padded (S2), so a console remainder binds as one padded bank. An insert
//! remainder is not: inserts bank opportunistically, and their remainder renders per node
//! (decision 12). The factory here is a test double over each launch effect: it validates the
//! request, checks the padding contract's clone clause, logs the mask it was asked for, and then
//! hands the request unchanged to the launch effect, which binds its own padded bank (P2b-P2e).

use super::*;
use crate::banks::effect_bank_resource;

/// Past the limiter's latency line, with room for its release.
const BLOCKS: u64 = 12;

/// How many times a [`PaddingDouble`] was asked to bind each mask, indexed by the mask's bits
/// (lane `l` is bit `l`; a bank has at most eight lanes).
type SeenMasks = Arc<[AtomicU64; 256]>;

/// A mask's index in [`SeenMasks`].
fn mask_bits(mask: &[bool]) -> usize {
    mask.iter()
        .enumerate()
        .map(|(lane, active)| usize::from(*active) << lane)
        .sum()
}

/// The masks that were asked for, as `(mask bits, times)`.
fn asked(seen: &SeenMasks) -> Vec<(usize, u64)> {
    seen.iter()
        .enumerate()
        .map(|(bits, count)| (bits, count.load(Ordering::SeqCst)))
        .filter(|(_, count)| *count != 0)
        .collect()
}

/// A launch effect whose bank bind logs its mask and checks the clone clause.
struct PaddingDouble {
    delegate: Arc<dyn NativeEffectFactory>,
    seen: SeenMasks,
}

/// Whether `padded` is a clone of `member`: every field of the prepared request, with the initial
/// values compared bit for bit.
fn is_clone(padded: &PrepareEffectRequest<'_>, member: &PrepareEffectRequest<'_>) -> bool {
    padded.sample_rate == member.sample_rate
        && padded.quantum == member.quantum
        && padded.quality == member.quality
        && padded.bypass == member.bypass
        && padded.link_mode == member.link_mode
        && padded.ports == member.ports
        && padded.limits == member.limits
        && padded.initial_values.len() == member.initial_values.len()
        && padded
            .initial_values
            .iter()
            .zip(member.initial_values)
            .all(|(padded, member)| {
                padded.parameter_index == member.parameter_index
                    && padded.channel == member.channel
                    && padded.value.to_bits() == member.value.to_bits()
            })
}

impl NativeEffectFactory for PaddingDouble {
    fn descriptor(&self) -> &'static effect_contract::EffectDescriptor {
        self.delegate.descriptor()
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<effect_contract::PreparedEffect, EffectPrepareError> {
        self.delegate.prepare(request)
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<effect_contract::PreparedEffectBank>, EffectPrepareError> {
        request.validate_shape()?;
        let members: Vec<_> = request
            .requests
            .iter()
            .zip(request.active_mask)
            .filter(|(_, active)| **active)
            .map(|(member, _)| member)
            .collect();
        for (lane, padded) in request.requests.iter().enumerate() {
            assert!(
                request.active_mask[lane] || members.iter().any(|member| is_clone(padded, member)),
                "padded lane {lane} carries a clone of an active member's request, never zeros"
            );
        }
        self.seen[mask_bits(request.active_mask)].fetch_add(1, Ordering::SeqCst);
        self.delegate.bind_homogeneous_bank(request)
    }
}

/// The intended console's three strip effects, each behind a [`PaddingDouble`] sharing one log.
fn padding_registry() -> (NativeEffectRegistry, SeenMasks) {
    let launch = launch_native_effect_registry().expect("launch registry");
    let seen: SeenMasks = Arc::new(core::array::from_fn(|_| AtomicU64::new(0)));
    let registry = NativeEffectRegistry::new(
        [
            "miso.parametric-eq",
            "miso.compressor",
            "miso.true-peak-limiter",
        ]
        .map(|id| {
            Box::new(PaddingDouble {
                delegate: launch.get_shared_ascii(id).expect("launch effect"),
                seen: Arc::clone(&seen),
            }) as Box<dyn NativeEffectFactory>
        }),
    )
    .expect("padding registry");
    (registry, seen)
}

/// The first `tracks` tracks of the intended console: `eq -> comp` in `console.pre_insert` and the
/// true-peak limiter in `console.post_insert`, every track routed to the main output.
fn intended(tracks: usize) -> session::SessionModel {
    let mut model =
        parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE).expect("intended fixture");
    model.tracks.truncate(tracks);
    model.routes.truncate(tracks);
    model
}

/// The effect banks `model` binds at the host's width, straight from the binder -- the
/// `GraphPreparedEffectBank`s the graph is handed -- and the session quantum.
fn bound_effect_banks(
    model: &session::SessionModel,
    levels: &[DependencyLevel],
    registry: &NativeEffectRegistry,
) -> (Vec<GraphPreparedEffectBank>, u32) {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled console model");
    let effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 22,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        },
    )
    .expect("prepared console effects");
    let (index, ids) = indexed_effect_ids(&effects);
    let mut classes = SessionPoolClasses::from_session(&effects.session);
    let (banks, _) = bind_rack_banks_indexed(
        &effects,
        &index,
        &ids,
        levels,
        host_dispatch(),
        &mut classes,
    )
    .expect("the console's banks bind");
    assert!(
        effect_bank_resource(&banks, session.quantum().0).is_some(),
        "the resource estimate accepts every bank the binder returns"
    );
    (banks, session.quantum().0)
}

/// A console remainder binds as one padded bank of each slot, and an insert remainder renders per
/// node, under the production policy, and both render the per-node oracle's bits.
///
/// For `1`, `W - 1` and `W + 3` tracks of the intended console (a lone track, the widest partial
/// group, and a full group beside a partial one), at the host's width:
///
/// * **Console slots pad.** Every slot of every group binds, and nothing renders per node. The
///   partial group's three slots are asked for with its mask, `true` on lanes `0..members` and
///   `false` after, and each padded lane carries a clone of a member's request (the double asserts
///   it on every bind).
/// * **Inserts do not.** The same strip folded into every track's inserts binds only its full
///   groups: no padded request is made, and the remainder's three slots render per node.
/// * **The graph carries the mask.** Each bound `GraphPreparedEffectBank` holds its members only,
///   and its `active_mask` is `true` on exactly those lanes; the resource estimate accepts it.
/// * **The rack reads and writes the active lanes only.** Both plans render the bits of the plan
///   compiled at the test-only `Scalar` oracle, where nothing banks.
///
/// Red if console groups stop padding, or insert groups start; if a padded lane is fed zeros or
/// defaults instead of a clone; if the bound bank's mask or members disagree with the group; if the
/// estimate refuses a partial mask (the compile fails); or if the rack gathers or scatters a padded
/// lane -- the graph runtime hands the chain one plane per member, so a planted scatter of lane
/// `members` indexes past them and the render panics, and a lane swapped with a padded one moves a
/// track's bits.
#[test]
fn a_console_remainder_binds_one_padded_bank_and_an_insert_remainder_renders_per_node() {
    let width = BankWidth::for_backend(host_dispatch()).expect("a vector host");
    let lanes = width.lanes() as usize;
    let full = (1_usize << lanes) - 1;
    for tracks in [1, lanes - 1, lanes + 3] {
        let model = intended(tracks);
        let what = format!("{tracks} tracks at {lanes} lanes");
        let partial = tracks % lanes;

        let oracle = try_compile_console_model_at(
            &model,
            1_088,
            &[],
            Backend::Scalar,
            &launch_native_effect_registry().expect("launch registry"),
        )
        .unwrap_or_else(|_| panic!("{what}: the Scalar oracle compiles"));
        let oracle =
            render_armed_console_blocks(oracle, BLOCKS, &BTreeSet::new(), &BTreeSet::new());

        // Console slots: every group binds, the partial one padded.
        let (registry, seen) = padding_registry();
        let console = compile_console_model_with_builtins(&model, 1_088, &[], &registry);
        let levels = console.graph().dependency_levels.clone();
        let report = &console.report().rack_cohorts;
        assert!(
            report.scalar_in(RackLocation::Simd1).is_empty()
                && report.scalar_in(RackLocation::Simd2).is_empty(),
            "{what}: every console slot binds"
        );
        let slots: usize = report
            .plan
            .groups
            .iter()
            .map(|group| group.program.len())
            .sum();
        assert_eq!(report.bound_slots.len(), slots, "{what}: one bank per slot");
        let mut expected = Vec::new();
        if partial != 0 {
            expected.push(((1_usize << partial) - 1, 3));
        }
        if tracks >= lanes {
            expected.push((full, 3 * (tracks / lanes) as u64));
        }
        assert_eq!(asked(&seen), expected, "{what}: the masks asked for");
        let console =
            render_armed_console_blocks(console, BLOCKS, &BTreeSet::new(), &BTreeSet::new());
        assert_pcm_bits_equal(&console.pcm, &oracle.pcm, &format!("{what}, console"));

        // What the graph is handed, straight from the binder.
        let (registry, _) = padding_registry();
        let (banks, _) = bound_effect_banks(&model, &levels, &registry);
        assert_eq!(
            banks.len(),
            3 * tracks.div_ceil(lanes),
            "{what}: bound banks"
        );
        for bank in &banks {
            let members = bank.members.len();
            assert_eq!(bank.scratch.width(), width, "{what}");
            assert_eq!(
                bank.active_mask.as_ref(),
                (0..lanes).map(|lane| lane < members).collect::<Vec<_>>(),
                "{what}: the mask is true on exactly the members' lanes"
            );
            assert!(
                members == lanes || members == partial,
                "{what}: a bank of {members} members"
            );
        }

        // Inserts: only the full groups bind, and the remainder renders per node.
        let mut inserts = model.clone();
        fold_console_into_inserts(&mut inserts);
        let (registry, seen) = padding_registry();
        let folded = compile_console_model_with_builtins(&inserts, 1_088, &[], &registry);
        assert!(
            asked(&seen).iter().all(|(bits, _)| *bits == full),
            "{what}: an insert group never asks for padding: {:?}",
            asked(&seen)
        );
        assert_eq!(
            folded
                .report()
                .rack_cohorts
                .scalar_in(RackLocation::Dynamic)
                .len(),
            3 * partial,
            "{what}: the insert remainder's three slots render per node"
        );
        let folded =
            render_armed_console_blocks(folded, BLOCKS, &BTreeSet::new(), &BTreeSet::new());
        assert_pcm_bits_equal(&folded.pcm, &oracle.pcm, &format!("{what}, inserts"));
    }
}

/// A padded bank charges its member metadata per active member, and its scratch per lane (P2a
/// verdict L2; #1098 gate 6).
///
/// `W + 1` tracks of the intended console bind, for each slot, one full bank and one padded bank
/// of one member. The two banks have the same width, quantum and lane count, so everything they
/// charge alike cancels in the difference, and what is left is the claim: the full bank carries
/// `W - 1` more members, each one `EffectNodeId` and its two id strings, and no more. Both charge
/// the same scratch, because a padded lane still occupies its AoSoA column.
///
/// Red if the member factor in `effect_bank_resource` becomes the lane count (the difference is
/// then the id strings alone), or if a padded bank stops charging a padded lane's scratch.
#[test]
fn a_padded_bank_charges_member_metadata_per_active_member() {
    let lanes = BankWidth::for_backend(host_dispatch())
        .expect("a vector host")
        .lanes() as usize;
    let model = intended(lanes + 1);
    let levels = compile_console_model_with_builtins(
        &model,
        1_088,
        &[],
        &launch_native_effect_registry().expect("launch registry"),
    )
    .graph()
    .dependency_levels
    .clone();
    let (registry, _) = padding_registry();
    let (banks, quantum) = bound_effect_banks(&model, &levels, &registry);
    let node = core::mem::size_of::<EffectNodeId>() as u64;
    let id_strings = |bank: &GraphPreparedEffectBank| -> u64 {
        bank.members
            .iter()
            .map(|member| (member.track_id.as_str().len() + member.effect_id.as_str().len()) as u64)
            .sum()
    };
    for effect in ["eq", "comp", "limiter"] {
        let of = |members: usize| {
            banks
                .iter()
                .find(|bank| {
                    bank.members.len() == members && bank.members[0].effect_id.as_str() == effect
                })
                .unwrap_or_else(|| panic!("a {effect} bank of {members} members"))
        };
        let (full, padded) = (of(lanes), of(1));
        let estimate = |bank: &GraphPreparedEffectBank| {
            effect_bank_resource(core::slice::from_ref(bank), quantum)
                .expect("the estimate accepts the bank")
        };
        let (full_estimate, padded_estimate) = (estimate(full), estimate(padded));
        assert_eq!(
            (full_estimate.metadata_bytes - id_strings(full))
                - (padded_estimate.metadata_bytes - id_strings(padded)),
            (lanes as u64 - 1) * node,
            "{effect}: each member the full bank adds is one node's metadata"
        );
        assert_eq!(
            full_estimate.scratch_bytes, padded_estimate.scratch_bytes,
            "{effect}: a padded lane still charges its scratch column"
        );
    }
}
