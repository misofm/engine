//! A dual block on a collapsed bank renders what a never-collapsed run renders (issue #2352).
//!
//! The effect contract's mono collapse freezes a bank's right channel from the first
//! `process_bank_mono` until `desymmetrize_channels`. `rack::BankChain` always takes that copy
//! before its first dual block, but the bank's own surface does not require it: every launch
//! bank's dual body takes the copy itself when it is entered collapsed
//! (`effect_runtime::collapse::MonoCollapse`). This file drives each banking launch effect through
//! its registry factory, as a host binds it, and holds the run that collapses and then renders dual
//! **without** calling `desymmetrize_channels` to a run that never collapsed: both planes, the rest
//! mask and every lane's state payload, bit for bit.

use effect_compiler::{NEVER_BANKED_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, PortRole, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
    StatePayloadOutput, StatePayloadSizes, decimal_to_f32, default_initial_values,
    parameter_lattice_points,
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

/// Every lane's state payload, in lane order.
fn payloads(
    bank: &dyn PreparedNativeEffectBank,
    lanes: usize,
    sizes: StatePayloadSizes,
) -> Vec<Vec<u8>> {
    (0..lanes)
        .map(|lane| {
            let mut common = vec![0_u8; sizes.common_bytes as usize];
            let mut left = vec![0_u8; sizes.left_bytes as usize];
            let mut right = vec![0_u8; sizes.right_bytes as usize];
            bank.snapshot_track_state_payload(
                lane as u32,
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                    .expect("payload shape"),
            )
            .unwrap_or_else(|failure| panic!("lane {lane} payload: {}", failure.code));
            [common, left, right].concat()
        })
        .collect()
}

/// What one block left: both output planes, the rest mask and every lane's payload.
#[derive(PartialEq)]
struct Observed {
    left: Vec<u32>,
    right: Vec<u32>,
    rest: u32,
    payloads: Vec<Vec<u8>>,
}

fn check(effect: &str) {
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_ascii(effect).expect("a launch effect");
    let Some(width) = BankWidth::for_backend(Backend::current()) else {
        return;
    };
    let lanes = width.lanes() as usize;
    let seed = effect.bytes().fold(0x2352_0000_u64, |hash, byte| {
        hash.rotate_left(7) ^ u64::from(byte)
    });
    let initial = initial_values(factory, lanes, seed);
    let sizes = factory
        .prepare(request(factory.descriptor(), &initial[0]))
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
    let offsets = vec![0_u32; lanes + 1];
    let run = |collapse: bool| -> (Vec<Observed>, Vec<Vec<u8>>) {
        let mut bank = bind(factory, width, &initial);
        assert!(
            bank.supports_mono_collapse(),
            "{effect}: a banking launch effect collapses"
        );
        let mut out = Vec::new();
        let mut boundary = Vec::new();
        for (index, plane) in planes.iter().enumerate() {
            if index == COLLAPSED {
                boundary = payloads(&*bank, lanes, sizes);
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
                &[],
                &offsets,
                QUANTUM,
            )
            .expect("block shape");
            // The collapsed run never calls `desymmetrize_channels`: its first dual block is
            // entered collapsed.
            let mono = collapse && index < COLLAPSED;
            if mono {
                let _ = bank.process_bank_mono(block);
            } else {
                let _ = bank.process_bank(block);
            }
            out.push(Observed {
                left: bits(&left),
                // A collapsed block's right plane is not its output: the seam writes the left.
                right: if mono { Vec::new() } else { bits(&right) },
                rest: bank.lanes_at_rest(),
                payloads: if mono {
                    Vec::new()
                } else {
                    payloads(&*bank, lanes, sizes)
                },
            });
        }
        (out, boundary)
    };
    let (dual, dual_boundary) = run(false);
    let (collapsed, frozen) = run(true);
    // The check must be able to fail: the collapsed run's right channel, frozen since its first
    // block, differs from the dual run's at the boundary, so a dual body that read it would show.
    assert_ne!(
        frozen, dual_boundary,
        "{effect}: the collapsed blocks moved no state, so this fixture cannot see a frozen right channel"
    );
    for (index, (got, want)) in collapsed.iter().zip(&dual).enumerate() {
        let at = format!("{effect} block {index}");
        assert_eq!(got.left, want.left, "{at}: the left plane");
        assert_eq!(got.rest, want.rest, "{at}: the rest mask");
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
