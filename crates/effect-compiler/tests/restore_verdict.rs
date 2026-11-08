//! `PreparedNativeEffectBank::validate_track_state_payload` against the restore it answers for,
//! for every banking launch effect (issue #2324).
//!
//! # The claim
//!
//! For every payload and every track index, the write-free validation returns exactly what
//! `restore_track_state_payload` then returns. `effect_contract::swap_bank_lanes` validates both
//! lanes' own payloads before its first write and relies on that answer to know the undo it may
//! need takes, so a validation that misses one of its restore's refusals (or invents one) would
//! let a lane swap half-finish, or refuse a swap the restore would take.
//!
//! Each bank is full, every lane draws its own live parameters on the lattice (the prepared ones
//! are one shared draw), and the bank renders a few blocks of loud noise so its state is off rest.
//! Then every lane's payload is offered as itself, as another lane's, under a wrong layout version
//! and with words overwritten by values a restore must judge (NaN, the infinities, `-0.0`,
//! subnormals, out-of-range magnitudes, all-ones bits and small integers), and every offer is
//! validated and then restored on the same bank. Out-of-range track indices and, where the factory
//! pads, a padded lane are offered too. The verdicts must be equal, and every lane's own untouched
//! payload must validate.

use effect_compiler::{NEVER_BANKED_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, PortRole, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, decimal_to_f32,
    default_initial_values, parameter_lattice_points,
};
use lane::Backend;

const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// Words overwritten per lane payload.
const REWRITES: usize = 48;
/// The values a rewritten word takes, as bits.
const WORDS: [u32; 12] = [
    0x7FC0_0000, // NaN
    0x7F80_0000, // +inf
    0xFF80_0000, // -inf
    0x8000_0000, // -0.0
    0x0000_0001, // the smallest subnormal
    0x7149_F2CA, // 1e30
    0xF149_F2CA, // -1e30
    0xFFFF_FFFF,
    0x0000_0000,
    0x3F80_0000, // 1.0
    0x0000_0007,
    0x4B80_0000, // 2^24
];

/// The frozen SplitMix64 step.
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

/// One lane's initial values: every live parameter drawn on its lattice, every prepared one at
/// `shared`, redrawn until the member prepares on its own.
fn draw_lane(
    factory: &dyn NativeEffectFactory,
    shared: &[f32],
    rng: &mut Rng,
) -> Vec<InitialParameterValue> {
    let descriptor = factory.descriptor();
    let defaults: Vec<_> = default_initial_values(descriptor).collect();
    for attempt in 0..256 {
        let values: Vec<_> = defaults
            .iter()
            .map(|slot| {
                let index = slot.parameter_index as usize;
                let parameter = &descriptor.parameters[index];
                let value = if parameter.prepared_reason.is_none() {
                    let lattice = lattice(parameter);
                    lattice[rng.below(lattice.len())]
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
    panic!("{}: no lane draw prepares", descriptor.id.as_str());
}

fn lattice(parameter: &effect_contract::ParameterDescriptor) -> Vec<f32> {
    parameter_lattice_points(parameter)
        .expect("a lawful lattice")
        .iter()
        .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
        .collect()
}

fn payload(bank: &dyn PreparedNativeEffectBank, lane: usize, sizes: StatePayloadSizes) -> Vec<u8> {
    let mut payload = vec![0_u8; sizes.total().expect("a payload total") as usize];
    let (common, rest) = payload.split_at_mut(sizes.common_bytes as usize);
    let (left, right) = rest.split_at_mut(sizes.left_bytes as usize);
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(common, left, right, sizes).expect("the prepared sizes"),
    )
    .expect("a member's snapshot");
    payload
}

/// Validates `payload` for `track`, then restores it: both verdicts. A bank that swaps lanes
/// answers the validation once more through its capability
/// ([`LaneSwap::validate_lane_payload`](effect_contract::LaneSwap::validate_lane_payload)),
/// which must agree (issue #2392).
fn offer(
    bank: &mut dyn PreparedNativeEffectBank,
    track: u32,
    version: u32,
    payload: &[u8],
    sizes: StatePayloadSizes,
) -> [Result<(), StatePayloadError>; 2] {
    let (common, rest) = payload.split_at(sizes.common_bytes as usize);
    let (left, right) = rest.split_at(sizes.left_bytes as usize);
    let input = || StatePayloadInput::new(common, left, right, sizes).expect("the prepared sizes");
    let validated = bank.validate_track_state_payload(track, version, &input());
    if let Some(swap) = bank.lane_swap() {
        assert_eq!(
            swap.validate_lane_payload(track, version, &input()),
            validated,
            "track {track}: the capability's verdict is the bank's"
        );
    }
    let restored = bank.restore_track_state_payload(track, version, input());
    [validated, restored]
}

fn check(factory: &dyn NativeEffectFactory, seed: u64) {
    let descriptor = factory.descriptor();
    let id = descriptor.id.as_str();
    let version = descriptor.state_layout_version;
    let width = BankWidth::for_lanes(Backend::current().width()).expect("the build's width");
    let lanes = width.lanes() as usize;
    let mut rng = Rng(seed);
    let shared: Vec<f32> = descriptor
        .parameters
        .iter()
        .map(|parameter| {
            let lattice = lattice(parameter);
            lattice[rng.below(lattice.len())]
        })
        .collect();
    let initial: Vec<_> = (0..lanes)
        .map(|_| draw_lane(factory, &shared, &mut rng))
        .collect();
    let requests: Vec<_> = initial
        .iter()
        .map(|values| request(descriptor, values))
        .collect();
    let sizes = factory
        .prepare(request(descriptor, &initial[0]))
        .expect("lane 0 prepares")
        .metadata
        .state_sizes;
    let bind = |requests: &[PrepareEffectRequest<'_>], mask: &[bool]| {
        factory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::current(),
            width,
            requests,
            active_mask: mask,
        })
    };
    let mut bank = bind(&requests, width.full_mask())
        .unwrap_or_else(|failure| panic!("{id}: bind: {}", failure.code))
        .unwrap_or_else(|| panic!("{id}: a full distinct bank binds"))
        .processor;
    let offsets = vec![0_u32; lanes + 1];
    for block in 0..4_u64 {
        let words = QUANTUM as usize * lanes;
        let mut left: Vec<f32> = (0..words).map(|_| 2.0 * rng.signed()).collect();
        let mut right: Vec<f32> = (0..words).map(|_| 2.0 * rng.signed()).collect();
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                QUANTUM,
                width,
                block * u64::from(QUANTUM),
                &[],
                &offsets,
                QUANTUM,
            )
            .expect("a bank block"),
        );
    }
    let total_words = sizes.total().expect("a payload total") as usize / 4;
    let mut offered = [0_usize; 2];
    for lane in 0..lanes {
        let own = payload(bank.as_ref(), lane, sizes);
        let at = format!("{id} seed {seed:#x} lane {lane}");
        let [validated, restored] = offer(bank.as_mut(), lane as u32, version, &own, sizes);
        assert_eq!(validated, Ok(()), "{at}: its own payload validates");
        assert_eq!(restored, Ok(()), "{at}: its own payload restores");
        let other = payload(bank.as_ref(), (lane + 1) % lanes, sizes);
        let mut cases: Vec<(String, u32, Vec<u8>)> = vec![
            ("another lane's payload".into(), version, other),
            (
                "a wrong layout version".into(),
                version.wrapping_add(1),
                own.clone(),
            ),
        ];
        for _ in 0..REWRITES {
            let word = rng.below(total_words);
            let bits = WORDS[rng.below(WORDS.len())];
            let mut rewritten = own.clone();
            rewritten[4 * word..4 * word + 4].copy_from_slice(&bits.to_le_bytes());
            cases.push((format!("word {word} = {bits:#010x}"), version, rewritten));
        }
        for (what, version, offered_payload) in cases {
            let [validated, restored] =
                offer(bank.as_mut(), lane as u32, version, &offered_payload, sizes);
            assert_eq!(validated, restored, "{at}: {what}");
            offered[usize::from(restored.is_err())] += 1;
            // Back to the lane's own state, so every case starts from the same lane.
            let [_, back] = offer(
                bank.as_mut(),
                lane as u32,
                descriptor.state_layout_version,
                &own,
                sizes,
            );
            assert_eq!(back, Ok(()), "{at}: its own payload restores after {what}");
        }
    }
    assert!(
        offered[0] > lanes && offered[1] > lanes,
        "{id}: the rewrites reach both verdicts ({offered:?} restored, refused)"
    );
    let own = payload(bank.as_ref(), 0, sizes);
    for track in [lanes as u32, u32::MAX] {
        let [validated, restored] = offer(bank.as_mut(), track, version, &own, sizes);
        assert!(restored.is_err(), "{id}: track {track} is out of range");
        assert_eq!(validated, restored, "{id}: track {track}");
    }
    // A padded lane's request is a clone of a member's.
    let mut padded_requests = requests.clone();
    padded_requests[lanes - 1] = requests[0];
    let mut padded_mask = width.full_mask().to_vec();
    padded_mask[lanes - 1] = false;
    if let Ok(Some(padded)) = bind(&padded_requests, &padded_mask) {
        let mut padded = padded.processor;
        let [validated, restored] =
            offer(padded.as_mut(), (lanes - 1) as u32, version, &own, sizes);
        assert_eq!(validated, restored, "{id}: a padded lane");
    }
}

/// Test value: an effect whose `validate_track_state_payload` skips one of its restore's checks
/// (a word range, a ramp path, the link mode agreement, a padded or out-of-range lane), refuses
/// what its restore takes, or is left at the trait's declining default; or a bank that swaps
/// lanes whose capability's verdict is not its `validate_track_state_payload` (issue #2392: an
/// override beside the capability, the drift the type pairing leaves open).
#[test]
fn every_banking_effect_validates_what_it_restores() {
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut banking = 0;
    for descriptor in registry.descriptors() {
        let id = descriptor.id.as_str();
        if NEVER_BANKED_EFFECTS.contains(&id) {
            continue;
        }
        let factory = registry.get_ascii(id).expect("a registered factory");
        for seed in 0..3_u64 {
            check(
                factory,
                0x2324_0000 ^ seed.wrapping_mul(0x9E37_79B9_7F4A_7C15),
            );
        }
        banking += 1;
    }
    assert!(banking >= 9, "every banking launch effect ran ({banking})");
}
