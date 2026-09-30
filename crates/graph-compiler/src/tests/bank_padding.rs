//! Issue #1088 (console strip P2a), gate 2: a padded effect bank through the real planner, the
//! real graph and the real rack.
//!
//! No shipped factory accepts padding yet (P2b-P2e opt them in), so the factory here is a test
//! double over the launch effect: it validates the request, checks the padding contract's clone
//! clause, and binds the launch bank with every lane active. That is the contract's simplest lawful
//! implementation for this input -- the clone lanes run the member's program on `+0.0` and the
//! rack discards what they produce -- and it is enough to show what P2a owns: the planner forms
//! and pads the group, the graph carries the mask, and the rack reads and writes the active lanes
//! only.

use super::*;
use crate::banks::{BankPadding, effect_bank_resource, test_only_with_bank_padding};

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

/// A launch effect whose bank bind accepts padded requests.
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
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        self.delegate.prepare(request)
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
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
        self.delegate
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                active_mask: request.width.full_mask(),
                ..request
            })
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

/// The first `tracks` tracks of the intended console: `eq -> comp` in SIMD rack 1 and the
/// true-peak limiter in SIMD rack 2, every track routed to the main output.
fn intended(tracks: usize) -> session::SessionModel {
    let mut model =
        parse_session_json(CONSOLE_SIXTY_FOUR_TRACK_INTENDED_FIXTURE).expect("intended fixture");
    model.tracks.truncate(tracks);
    model.routes.truncate(tracks);
    model
}

/// The effect banks `model` binds at the host's width under `padding`, straight from the binder:
/// the `GraphPreparedEffectBank`s the graph is handed.
fn bound_effect_banks(
    model: &session::SessionModel,
    levels: &[DependencyLevel],
    registry: &NativeEffectRegistry,
    padding: BankPadding,
) -> Vec<GraphPreparedEffectBank> {
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
    let (banks, _) = test_only_with_bank_padding(padding, || {
        bind_rack_banks_indexed(
            &effects,
            &index,
            &ids,
            levels,
            host_dispatch(),
            &mut classes,
        )
    })
    .expect("the console's banks bind");
    assert!(
        effect_bank_resource(&banks, session.quantum().0).is_some(),
        "the resource estimate accepts every bank the binder returns"
    );
    banks
}

/// A group of fewer members than lanes binds as one padded bank of a factory that accepts padding,
/// under the test-only policy, and renders what the per-node path renders.
///
/// For `1`, `W - 1` and `W + 3` tracks of the intended console (a lone track, the widest partial
/// group, and a full group beside a partial one):
///
/// * **Policy.** Under the production policy nothing asks for padding: the partial group's slots
///   render per node and the factory is never asked to bind a padded lane. Under the test-only
///   policy every slot of every group binds, and nothing renders per node.
/// * **The request.** A padded bank is asked for with the group's mask, `true` on lanes
///   `0..members` and `false` after, and each padded lane carries a clone of a member's request
///   (the double asserts it on every bind).
/// * **The graph carries the mask.** Each bound `GraphPreparedEffectBank` holds its members only,
///   and its `active_mask` is `true` on exactly those lanes; the resource estimate accepts it.
/// * **The rack reads and writes the active lanes only.** The padded plan renders the bits of the
///   bank-free per-node oracle and of the production plan.
///
/// Red if the planner never pads, or pads under the production policy; if a padded lane is fed
/// zeros or defaults instead of a clone; if the bound bank's mask or members disagree with the
/// group; if the estimate still refuses a partial mask (the compile fails); or if the rack gathers
/// or scatters a padded lane -- the graph runtime hands the chain one plane per member, so a
/// planted scatter of lane `members` indexes past them and the render panics, and a lane swapped
/// with a padded one moves a track's bits.
#[test]
fn a_partial_group_binds_one_padded_bank_and_renders_the_per_node_bits() {
    let Some(width) = BankWidth::for_backend(host_dispatch()) else {
        return;
    };
    let lanes = width.lanes() as usize;
    for tracks in [1, lanes - 1, lanes + 3] {
        let model = intended(tracks);
        let what = format!("{tracks} tracks at {lanes} lanes");
        let partial = tracks % lanes;

        let oracle =
            compile_console_model_with_builtins(&model, 1_088, &[], &scalar_console_registry());
        let levels = oracle.graph().dependency_levels.clone();
        let oracle =
            render_armed_console_blocks(oracle, BLOCKS, &BTreeSet::new(), &BTreeSet::new());

        // The production policy: nothing asks, so the remainder renders per node.
        let (registry, seen) = padding_registry();
        let production = compile_console_model_with_builtins(&model, 1_088, &[], &registry);
        let report = &production.report().rack_cohorts;
        let full = (1_usize << lanes) - 1;
        assert!(
            asked(&seen).iter().all(|(bits, _)| *bits == full),
            "{what}: no group asks for padding, so no padded request is made: {:?}",
            asked(&seen)
        );
        assert_eq!(
            report.scalar_in(RackLocation::Simd1).len()
                + report.scalar_in(RackLocation::Simd2).len(),
            3 * partial,
            "{what}: the partial group's three slots render per node"
        );
        let production =
            render_armed_console_blocks(production, BLOCKS, &BTreeSet::new(), &BTreeSet::new());
        assert_pcm_bits_equal(&production.pcm, &oracle.pcm, &format!("{what}, production"));

        // The test-only policy: every partial group is padded.
        let (registry, seen) = padding_registry();
        let padded = test_only_with_bank_padding(BankPadding::EveryGroup, || {
            compile_console_model_with_builtins(&model, 1_088, &[], &registry)
        });
        let report = &padded.report().rack_cohorts;
        assert!(
            report.scalar_in(RackLocation::Simd1).is_empty()
                && report.scalar_in(RackLocation::Simd2).is_empty(),
            "{what}: every slot binds"
        );
        let slots: usize = report
            .plan
            .groups
            .iter()
            .map(|group| group.program.len())
            .sum();
        assert_eq!(report.bound_slots.len(), slots, "{what}: one bank per slot");
        // The partial group's three slots are asked for with its mask, `true` on lanes
        // `0..partial`, and every full group's slots with the full mask; nothing else is asked.
        let mut expected = Vec::new();
        if partial != 0 {
            expected.push(((1_usize << partial) - 1, 3));
        }
        if tracks >= lanes {
            expected.push((full, 3 * (tracks / lanes) as u64));
        }
        assert_eq!(asked(&seen), expected, "{what}: the masks asked for");
        let padded =
            render_armed_console_blocks(padded, BLOCKS, &BTreeSet::new(), &BTreeSet::new());
        assert_pcm_bits_equal(&padded.pcm, &oracle.pcm, &format!("{what}, padded"));

        // What the graph is handed, straight from the binder.
        for (padding, binds_partial) in [
            (BankPadding::AsRequested, false),
            (BankPadding::EveryGroup, true),
        ] {
            let (registry, _) = padding_registry();
            let banks = bound_effect_banks(&model, &levels, &registry, padding);
            let expected = 3 * (tracks / lanes) + if binds_partial && partial != 0 { 3 } else { 0 };
            assert_eq!(banks.len(), expected, "{what}, {padding:?}: bound banks");
            for bank in &banks {
                let members = bank.members.len();
                assert_eq!(bank.scratch.width(), width, "{what}, {padding:?}");
                assert_eq!(
                    bank.active_mask.as_ref(),
                    (0..lanes).map(|lane| lane < members).collect::<Vec<_>>(),
                    "{what}, {padding:?}: the mask is true on exactly the members' lanes"
                );
                assert!(
                    members == lanes || (binds_partial && members == partial),
                    "{what}, {padding:?}: a bank of {members} members"
                );
            }
        }
    }
}
