//! An entry on a collapsed bank behaves as on a never-collapsed one (issue #2352).
//!
//! The effect contract's mono collapse freezes a bank's right channel from the first
//! `process_bank_mono` until `desymmetrize_channels`. `rack::BankChain` always takes that copy
//! before its first dual block, but the bank's own surface does not require it: every launch
//! bank takes the copy itself when an entry that reads or writes channel state is entered
//! collapsed (`effect_runtime::collapse::MonoCollapse`). This file drives each banking launch
//! effect through its registry factory, as a host binds it, and holds the run that collapses and
//! then, **without** calling `desymmetrize_channels`, meets one boundary -- a plain dual block, or
//! a writer of channel state whose write would be lost to a late copy -- and renders on, against a
//! run that never collapsed: both planes, the rest mask, the report and every lane's state
//! payload, bit for bit. Beside the dual block, the boundaries are the bank's writers of channel
//! state that a later copy could otherwise overwrite: see [`Boundary`].

use effect_compiler::{NEVER_BANKED_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectQuality, EffectTargetRequest,
    InitialParameterValue, LinkMode, NativeEffectFactory, PREPARED_EFFECT_TARGET_WORDS,
    ParameterChannel, PortRole, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectTarget, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ResetKind, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, decimal_to_f32, default_initial_values, parameter_lattice_points,
};
use lane::Backend;

const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// Collapsed blocks before the first dual one.
const COLLAPSED: usize = 8;
/// Dual blocks of signal after them, then dual blocks of exact silence.
const LOUD: usize = 8;
const SILENT: usize = 64;

/// The frozen SplitMix64 step, so every draw is a function of the seed alone.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
    /// Uniform in `[-1, 1)`.
    fn signed(&mut self) -> f32 {
        ((self.next() >> 40) as f32 / (1_u64 << 24) as f32) * 2.0 - 1.0
    }
}

fn ports(descriptor: &'static EffectDescriptor) -> PreparedPorts {
    let sidechain = descriptor
        .ports
        .iter()
        .find(|port| port.role == PortRole::SidechainInput)
        .map_or(PreparedSidechainPort::None, |port| {
            PreparedSidechainPort::Unconnected {
                id: port.id,
                required: port.required,
            }
        });
    PreparedPorts { sidechain }
}

fn request<'a>(
    descriptor: &'static EffectDescriptor,
    initial: &'a [InitialParameterValue],
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: ports(descriptor),
        initial_values: initial,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 26,
            maximum_scratch_bytes: 1 << 22,
            maximum_automation_spans_per_block: 128,
        },
        tail_bound: launch_native_effect_registry()
            .expect("launch registry")
            .tail_bound(descriptor.id, SAMPLE_RATE, EffectQuality::Normal)
            .expect("a declared row"),
    }
}

/// Per lane, one lattice draw per parameter, written to **both** channels' slots: a collapse is
/// lawful only over a track whose two channels are designed alike. Prepared parameters take one
/// draw for every lane, as a homogeneous bank requires. A draw that does not prepare is redrawn.
fn initial_values(
    factory: &dyn NativeEffectFactory,
    lanes: usize,
    seed: u64,
) -> Vec<Vec<InitialParameterValue>> {
    let descriptor = factory.descriptor();
    let lattices: Vec<Vec<f32>> = descriptor
        .parameters
        .iter()
        .map(|parameter| {
            parameter_lattice_points(parameter)
                .expect("a lawful lattice")
                .iter()
                .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
                .collect()
        })
        .collect();
    let defaults: Vec<_> = default_initial_values(descriptor).collect();
    let mut rng = Rng(seed);
    let shared: Vec<f32> = lattices
        .iter()
        .map(|values| values[rng.below(values.len())])
        .collect();
    (0..lanes)
        .map(|_| {
            for attempt in 0..256 {
                let draw: Vec<f32> = lattices
                    .iter()
                    .map(|values| values[rng.below(values.len())])
                    .collect();
                let values: Vec<_> = defaults
                    .iter()
                    .map(|slot| {
                        let index = slot.parameter_index as usize;
                        let value = if descriptor.parameters[index].prepared_reason.is_none() {
                            draw[index]
                        } else if attempt < 128 {
                            shared[index]
                        } else {
                            slot.value
                        };
                        InitialParameterValue { value, ..*slot }
                    })
                    .collect();
                if factory.prepare(request(descriptor, &values)).is_ok() {
                    return values;
                }
            }
            panic!("{}: no lane draw prepares", descriptor.id.as_str())
        })
        .collect()
}

fn bind(
    factory: &dyn NativeEffectFactory,
    width: BankWidth,
    initial: &[Vec<InitialParameterValue>],
) -> Box<dyn PreparedNativeEffectBank> {
    let descriptor = factory.descriptor();
    let requests: Vec<_> = initial
        .iter()
        .map(|values| request(descriptor, values))
        .collect();
    factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .unwrap_or_else(|failure| panic!("{}: bind: {}", descriptor.id.as_str(), failure.code))
        .unwrap_or_else(|| panic!("{}: a full bank binds", descriptor.id.as_str()))
        .processor
}

/// One lane's state payload, its three sections concatenated.
fn payload(bank: &dyn PreparedNativeEffectBank, lane: usize, sizes: StatePayloadSizes) -> Vec<u8> {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("payload shape"),
    )
    .unwrap_or_else(|failure| panic!("lane {lane} payload: {}", failure.code));
    [common, left, right].concat()
}

/// Every lane's state payload, in lane order.
fn payloads(
    bank: &dyn PreparedNativeEffectBank,
    lanes: usize,
    sizes: StatePayloadSizes,
) -> Vec<Vec<u8>> {
    (0..lanes).map(|lane| payload(bank, lane, sizes)).collect()
}

/// What one block left: both output planes, the rest mask and every lane's payload.
#[derive(PartialEq)]
struct Observed {
    left: Vec<u32>,
    right: Vec<u32>,
    rest: u32,
    report: String,
    payloads: Vec<Vec<u8>>,
}

/// What the run does at the boundary, besides rendering: each is an entry a collapsed bank must
/// enter on live state.
#[derive(Clone, Copy, Debug)]
enum Boundary {
    /// Nothing: the first dual block is entered collapsed.
    Dual,
    /// The first dual block carries a right-channel span on every lane, which the block applies
    /// before it renders: the copy must land before the automation, not over it.
    RightSpan,
    /// Lane 0 is restored from a payload whose channels differ before the first dual block: the
    /// copy must land before the restore, not over it at the next dual entry.
    Restore,
    /// Lane 0 is copied from lane 0 of a donor bank whose channels differ (`copy_lane_from`).
    CopyLane,
    /// Lane 0 is swapped with lane 0 of such a donor (`swap_lanes_direct`).
    Swap,
    /// The same swap called on the donor, with this bank as its peer: the peer's copy.
    SwapAsPeer,
    /// Lane 0 took prepared defaults whose channels differ (`exchange_lane_defaults`) at bind, and
    /// the whole bank resets to its defaults (`reset(FullToDefaults)`).
    ResetToDefaults,
    /// Lane 0 takes a prepared target on one right-channel parameter
    /// (`apply_prepared_target_lane`).
    RightTarget,
}

/// The effects whose bank implements `copy_lane_from` and `swap_lanes_direct` itself; every
/// other bank declines both, and the rows for them are vacuous.
const DIRECT_LANE_PATHS: [&str; 2] = ["miso.compressor", "miso.parametric-eq"];

/// One automatable live right-channel slot of `values` moved to another lattice value, or `None`.
fn moved_right(
    descriptor: &'static EffectDescriptor,
    values: &[InitialParameterValue],
) -> Option<(u32, f32)> {
    values.iter().find_map(|slot| {
        let parameter = &descriptor.parameters[slot.parameter_index as usize];
        if slot.channel != ParameterChannel::Right
            || !parameter.automatable
            || parameter.prepared_reason.is_some()
        {
            return None;
        }
        parameter_lattice_points(parameter)
            .expect("a lawful lattice")
            .iter()
            .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
            .rev()
            .find(|value| value.to_bits() != slot.value.to_bits())
            .map(|value| (slot.parameter_index, value))
    })
}

/// The prepared targets that move lane `values`' first movable right-channel slot.
fn right_targets(
    factory: &dyn NativeEffectFactory,
    values: &[InitialParameterValue],
) -> Vec<PreparedEffectTarget> {
    let (Some(preparation), Some((index, value))) = (
        factory.target_preparation(),
        moved_right(factory.descriptor(), values),
    ) else {
        return Vec::new();
    };
    let mut next = values.to_vec();
    let changed: Vec<bool> = next
        .iter_mut()
        .map(|slot| {
            let hit = slot.parameter_index == index && slot.channel == ParameterChannel::Right;
            if hit {
                slot.value = value;
            }
            hit
        })
        .collect();
    let mut out = vec![
        PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; PREPARED_EFFECT_TARGET_WORDS],
        };
        preparation.maximum_targets()
    ];
    let Ok(count) = preparation.prepare_targets(
        EffectTargetRequest {
            sample_rate: SAMPLE_RATE,
            values: &next,
            changed: &changed,
        },
        &mut out,
    ) else {
        return Vec::new();
    };
    out.truncate(count);
    out
}

/// One right-channel span per lane, at the block's first sample (the only one every effect
/// admits), moving an automatable live parameter to a lattice value its lane does not hold.
fn right_spans(
    descriptor: &'static EffectDescriptor,
    initial: &[Vec<InitialParameterValue>],
    first: u64,
) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32];
    for values in initial {
        let slot = values.iter().find(|slot| {
            let parameter = &descriptor.parameters[slot.parameter_index as usize];
            slot.channel == ParameterChannel::Right
                && parameter.automatable
                && parameter.prepared_reason.is_none()
        });
        if let Some(slot) = slot {
            let parameter = &descriptor.parameters[slot.parameter_index as usize];
            let lattice = parameter_lattice_points(parameter).expect("a lawful lattice");
            let value = lattice
                .iter()
                .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
                .rev()
                .find(|value| value.to_bits() != slot.value.to_bits());
            if let Some(value) = value {
                spans.push(PreparedAutomationSpan {
                    channel: ParameterChannel::Right,
                    parameter_index: slot.parameter_index,
                    sample: first,
                    value,
                });
            }
        }
        offsets.push(spans.len() as u32);
    }
    (spans, offsets)
}

fn check(effect: &str) {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_ascii(effect).expect("a launch effect");
    let Some(width) = BankWidth::for_backend(Backend::current()) else {
        return;
    };
    let lanes = width.lanes() as usize;
    for boundary in [
        Boundary::Dual,
        Boundary::RightSpan,
        Boundary::Restore,
        Boundary::CopyLane,
        Boundary::Swap,
        Boundary::SwapAsPeer,
        Boundary::ResetToDefaults,
        Boundary::RightTarget,
    ] {
        check_case(factory, width, lanes, boundary);
    }
}

fn check_case(
    factory: &dyn NativeEffectFactory,
    width: BankWidth,
    lanes: usize,
    boundary: Boundary,
) {
    let descriptor = factory.descriptor();
    let effect = descriptor.id.as_str();
    let case = format!("{effect} {boundary:?}");
    let seed = effect.bytes().fold(0x2352_0000_u64, |hash, byte| {
        hash.rotate_left(7) ^ u64::from(byte)
    });
    let initial = initial_values(factory, lanes, seed);
    let sizes = factory
        .prepare(request(descriptor, &initial[0]))
        .expect("lane 0 prepares")
        .metadata
        .state_sizes;
    // One plane per block, the track's two channels alike: a tone and noise at a level drawn per
    // lane and block, then exact silence. Loud, then quiet across the boundary, then loud again,
    // so every detector is releasing (and a gate closing) when the first dual block lands.
    let mut rng = Rng(seed ^ 0x5eed);
    let shapes: Vec<(f32, f32)> = (0..lanes)
        .map(|_| (8.0 + rng.below(400) as f32, rng.signed().abs()))
        .collect();
    let mut sample = 0_usize;
    let planes: Vec<Vec<f32>> = (0..COLLAPSED + LOUD + SILENT)
        .map(|block| {
            let (floor, span) = match block {
                _ if block < COLLAPSED / 2 => (-6.0, 12.0),
                _ if block < COLLAPSED + LOUD / 2 => (-80.0, 10.0),
                _ => (-12.0, 18.0),
            };
            let gains: Vec<f32> = (0..lanes)
                .map(|_| math::powf(10.0, (floor + span * rng.signed().abs()) / 20.0))
                .collect();
            let mut plane = vec![0.0_f32; QUANTUM as usize * lanes];
            if block < COLLAPSED + LOUD {
                for (frame, words) in plane.chunks_mut(lanes).enumerate() {
                    let t = (sample + frame) as f32;
                    for (lane, word) in words.iter_mut().enumerate() {
                        let (period, noise) = shapes[lane];
                        let tone = math::sin(f64::from(core::f32::consts::TAU * t / period)) as f32;
                        *word = gains[lane] * ((1.0 - noise) * tone + noise * rng.signed());
                    }
                }
            }
            sample += QUANTUM as usize;
            plane
        })
        .collect();
    let bits = |plane: &[f32]| plane.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
    let empty = vec![0_u32; lanes + 1];
    let (spans, span_offsets) =
        right_spans(descriptor, &initial, (COLLAPSED * QUANTUM as usize) as u64);
    // The restored payload: lane 0 of a donor bank that rendered the collapsed span dual with its
    // right plane at half the left, so its two channels' state differ.
    let donor_bank = || {
        let mut bank = bind(factory, width, &initial);
        for (index, plane) in planes.iter().take(COLLAPSED).enumerate() {
            let mut left = plane.clone();
            let mut right: Vec<f32> = plane.iter().map(|word| word * 0.5).collect();
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                QUANTUM,
                width,
                (index * QUANTUM as usize) as u64,
                &[],
                &empty,
                QUANTUM,
            )
            .expect("block shape");
            let _ = bank.process_bank(block);
        }
        bank
    };
    let donor = payload(&*donor_bank(), 0, sizes);
    // Lane 0's defaults with one right-channel slot moved, read out of a bank bound with them.
    let asymmetric_defaults = moved_right(descriptor, &initial[0]).and_then(|(index, value)| {
        let mut lanes_values = initial.clone();
        for slot in &mut lanes_values[0] {
            if slot.parameter_index == index && slot.channel == ParameterChannel::Right {
                slot.value = value;
            }
        }
        factory
            .prepare(request(descriptor, &lanes_values[0]))
            .ok()?;
        let mut bank = bind(factory, width, &lanes_values);
        let mut bytes = vec![0_u8; bank.lane_default_bytes()?];
        bank.exchange_lane_defaults(0, &mut bytes).then_some(bytes)
    });
    let targets = right_targets(factory, &initial[0]);
    // Whether the boundary's writer did anything: a bank that declines a direct lane path, or an
    // effect with no prepared targets or no lane defaults, makes its row vacuous, and the rows the
    // effect is known to implement must not be.
    let applies = match boundary {
        Boundary::CopyLane | Boundary::Swap | Boundary::SwapAsPeer => {
            DIRECT_LANE_PATHS.contains(&effect)
        }
        Boundary::ResetToDefaults => asymmetric_defaults.is_some(),
        Boundary::RightTarget => !targets.is_empty(),
        Boundary::Dual | Boundary::RightSpan | Boundary::Restore => true,
    };
    if effect == "miso.parametric-eq" && matches!(boundary, Boundary::RightTarget) {
        assert!(applies, "{case}: the EQ designs a right-channel target");
    }
    if !applies {
        return;
    }
    let run = |collapse: bool| -> (Vec<Observed>, Vec<Vec<u8>>) {
        let mut bank = bind(factory, width, &initial);
        let mut peer = donor_bank();
        assert!(
            bank.supports_mono_collapse(),
            "{effect}: a banking launch effect collapses"
        );
        let mut out = Vec::new();
        let mut frozen = Vec::new();
        for (index, plane) in planes.iter().enumerate() {
            let mut automation: (&[PreparedAutomationSpan], &[u32]) = (&[], &empty);
            // Defaults are no state word, so a bank holding asymmetric ones still collapses
            // lawfully; the reset at the boundary is what writes them into the right channel.
            if index == 0 && matches!(boundary, Boundary::ResetToDefaults) {
                let mut bytes = asymmetric_defaults.clone().expect("lane defaults");
                assert!(
                    bank.exchange_lane_defaults(0, &mut bytes),
                    "{case}: exchange"
                );
            }
            if index == COLLAPSED {
                frozen = payloads(&*bank, lanes, sizes);
                match boundary {
                    Boundary::Dual => {}
                    Boundary::RightSpan => automation = (&spans, &span_offsets),
                    Boundary::Restore => {
                        let (common, rest) = donor.split_at(sizes.common_bytes as usize);
                        let (left, right) = rest.split_at(sizes.left_bytes as usize);
                        bank.restore_track_state_payload(
                            0,
                            descriptor.state_layout_version,
                            StatePayloadInput::new(common, left, right, sizes)
                                .expect("payload shape"),
                        )
                        .unwrap_or_else(|failure| panic!("{case}: restore: {}", failure.code));
                    }
                    Boundary::CopyLane => {
                        // `Some(false)`: copied, and the lane's two sections disagree.
                        let copied = bank.copy_lane_from(0, &*peer, 0);
                        assert_eq!(copied, Some(false), "{case}: the direct copy");
                    }
                    Boundary::Swap => {
                        assert!(
                            bank.swap_lanes_direct(0, Some(&mut *peer), 0),
                            "{case}: the direct swap"
                        );
                    }
                    Boundary::SwapAsPeer => {
                        assert!(
                            peer.swap_lanes_direct(0, Some(&mut *bank), 0),
                            "{case}: the direct swap"
                        );
                    }
                    Boundary::ResetToDefaults => bank.reset(ResetKind::FullToDefaults),
                    Boundary::RightTarget => {
                        for target in &targets {
                            bank.apply_prepared_target_lane(0, target)
                                .unwrap_or_else(|failure| panic!("{case}: target: {failure:?}"));
                        }
                    }
                }
            }
            let mut left = plane.clone();
            let mut right = plane.clone();
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                QUANTUM,
                width,
                (index * QUANTUM as usize) as u64,
                automation.0,
                automation.1,
                QUANTUM,
            )
            .expect("block shape");
            // The collapsed run never calls `desymmetrize_channels`: the boundary's entries meet
            // it collapsed.
            let mono = collapse && index < COLLAPSED;
            let report = if mono {
                bank.process_bank_mono(block)
            } else {
                bank.process_bank(block)
            };
            out.push(Observed {
                left: bits(&left),
                // A collapsed block's right plane is not its output: the seam writes the left.
                right: if mono { Vec::new() } else { bits(&right) },
                rest: bank.lanes_at_rest(),
                report: format!("{report:?}"),
                payloads: if mono {
                    Vec::new()
                } else {
                    payloads(&*bank, lanes, sizes)
                },
            });
        }
        (out, frozen)
    };
    let (dual, dual_boundary) = run(false);
    let (collapsed, frozen) = run(true);
    // The check must be able to fail: the collapsed run's right channel, frozen since its first
    // block, differs from the dual run's at the boundary, so an entry that read it, or a copy
    // that landed after the boundary's write, would show.
    assert_ne!(
        frozen, dual_boundary,
        "{case}: the collapsed blocks moved no state, so this fixture cannot see a frozen right channel"
    );
    if matches!(boundary, Boundary::RightSpan) {
        assert!(
            !spans.is_empty(),
            "{case}: no lane has a right-channel span to carry"
        );
    }
    for (index, (got, want)) in collapsed.iter().zip(&dual).enumerate() {
        let at = format!("{case} block {index}");
        assert_eq!(got.left, want.left, "{at}: the left plane");
        assert_eq!(got.rest, want.rest, "{at}: the rest mask");
        assert_eq!(got.report, want.report, "{at}: the report");
        if index < COLLAPSED {
            continue;
        }
        assert_eq!(
            got.right, want.right,
            "{at}: the right plane of a dual block entered collapsed"
        );
        for lane in 0..lanes {
            let first = got.payloads[lane]
                .iter()
                .zip(&want.payloads[lane])
                .position(|(got, want)| got != want);
            assert!(
                first.is_none(),
                "{at}: lane {lane}'s state payload differs at byte {first:?}"
            );
        }
    }
}

macro_rules! rows {
    ($($name:ident => $effect:literal,)*) => {
        const ROWS: &[&str] = &[$($effect),*];
        $(
            #[test]
            fn $name() {
                check($effect);
            }
        )*
    };
}

rows! {
    dual_entry_compressor => "miso.compressor",
    dual_entry_de_esser => "miso.de-esser",
    dual_entry_dynamic_eq => "miso.dynamic-eq",
    dual_entry_gate_expander => "miso.gate-expander",
    dual_entry_multiband_compressor => "miso.multiband-compressor",
    dual_entry_parametric_eq => "miso.parametric-eq",
    dual_entry_soft_clip => "miso.soft-clip",
    dual_entry_transient_shaper => "miso.transient-shaper",
    dual_entry_true_peak_limiter => "miso.true-peak-limiter",
}

/// Red mutation: a launch effect that binds a bank and has no row here -> the set comparison names
/// it, so a new collapsing bank cannot ship without this oracle.
#[test]
fn every_banking_effect_has_a_row() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut banking: Vec<&str> = registry
        .descriptors()
        .map(|descriptor| descriptor.id.as_str())
        .filter(|id| !NEVER_BANKED_EFFECTS.contains(id))
        .collect();
    banking.sort_unstable();
    let mut rows = ROWS.to_vec();
    rows.sort_unstable();
    assert_eq!(banking, rows);
}
