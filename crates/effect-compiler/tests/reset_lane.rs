//! `PreparedNativeEffectBank::reset_lane` against the whole-bank `reset`, for every banking launch
//! effect (issue #2131 gates 1 and 2).
//!
//! # The claim
//!
//! After `reset_lane(i, kind)` at a block boundary, lane `i` continues exactly as lane `i` of a twin
//! that ran `reset(kind)` on the whole bank at that boundary, and every other lane continues exactly
//! as a twin that was never reset. "Exactly" is checked three ways after the call and after every
//! later block: every output word of both planes, every lane's state payload, and every lane's bit
//! of `lanes_at_rest`. So a lane's bits after the call cannot depend on its bank mates, and the
//! bank's shared bookkeeping (rest claims, fast-path flags, silence counters, shared cursors) cannot
//! go stale without a later word or rest bit disagreeing.
//!
//! Each bank is full and every lane is distinct: its own seeded draw of every live parameter on the
//! lattice, its own signal, and its own automation spans, some landing late in the last block
//! before the call so ramps are in flight across it. The prepared (non-live) parameters are one draw
//! shared by the bank, because they are part of what a homogeneous bank shares. Two schedules run:
//! `running`, where the signal never stops, and `resting`, where every lane falls silent for long
//! enough before and after the call that rest and silence bookkeeping is exercised. Each runs for
//! both reset kinds and for the first, a middle and the last lane.
//!
//! The rows are one per effect so a failure names the effect; `every_banking_effect_has_a_row`
//! keeps the row list equal to the launch effects outside `NEVER_BANKED_EFFECTS`.
//!
//! Gate 2 rides in the same driver: every `reset_lane` call is measured with
//! `bench_support::alloc`'s thread-scoped counters and must reach the allocator zero times.

use bench_support::alloc as bench_alloc;
use effect_compiler::{NEVER_BANKED_EFFECTS, launch_native_effect_registry};
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterDescriptor, PortRole, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort, ResetKind, StatePayloadOutput, StatePayloadSizes,
    decimal_to_f32, default_initial_values, parameter_lattice_points,
};
use lane::Backend;

const SAMPLE_RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// Block lengths, cycled: a full quantum, short odd blocks and a one-frame block.
const FRAMES: [u32; 6] = [128, 61, 128, 1, 97, 128];

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
        ((self.next() >> 40) as f32 / (1_u64 << 24) as f32).mul_add(2.0, -1.0)
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

/// Every lattice value of every parameter, in descriptor order.
fn lattices(descriptor: &'static EffectDescriptor) -> Vec<Vec<f32>> {
    descriptor
        .parameters
        .iter()
        .map(|parameter| {
            parameter_lattice_points(parameter)
                .expect("a lawful lattice")
                .iter()
                .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
                .collect()
        })
        .collect()
}

fn live(parameter: &ParameterDescriptor) -> bool {
    parameter.prepared_reason.is_none()
}

/// The fixture of one effect: the per-lane initial values and the per-lane automation and signal
/// of every block, drawn once so the three runs of every case see the same words.
struct Fixture {
    descriptor: &'static EffectDescriptor,
    lanes: usize,
    width: BankWidth,
    initial: Vec<Vec<InitialParameterValue>>,
    sizes: StatePayloadSizes,
    lattices: Vec<Vec<f32>>,
}

impl Fixture {
    fn new(factory: &dyn NativeEffectFactory, width: BankWidth, seed: u64) -> Self {
        let descriptor = factory.descriptor();
        let lanes = width.lanes() as usize;
        let lattices = lattices(descriptor);
        let mut rng = Rng(seed);
        let defaults: Vec<_> = default_initial_values(descriptor).collect();
        // The bank's shared prepared values: one draw for every lane.
        let shared: Vec<f32> = lattices
            .iter()
            .map(|values| values[rng.below(values.len())])
            .collect();
        let mut initial = Vec::with_capacity(lanes);
        for _ in 0..lanes {
            // Redraw until the member prepares on its own: some parameter pairs carry an ordering
            // rule (a crossover above the one below it), and a refused draw is not this test's
            // subject. The prepared parameters stay at the shared draw, or at their defaults if
            // the shared draw itself is refused.
            let mut chosen = None;
            for attempt in 0..256 {
                let values: Vec<_> = defaults
                    .iter()
                    .map(|slot| {
                        let index = slot.parameter_index as usize;
                        let parameter = &descriptor.parameters[index];
                        let value = if live(parameter) {
                            lattices[index][rng.below(lattices[index].len())]
                        } else if attempt < 128 {
                            shared[index]
                        } else {
                            slot.value
                        };
                        InitialParameterValue { value, ..*slot }
                    })
                    .collect();
                if factory.prepare(request(descriptor, &values)).is_ok() {
                    chosen = Some(values);
                    break;
                }
            }
            initial
                .push(chosen.unwrap_or_else(|| {
                    panic!("{}: no lane draw prepares", descriptor.id.as_str())
                }));
        }
        let sizes = factory
            .prepare(request(descriptor, &initial[0]))
            .expect("lane 0 prepares")
            .metadata
            .state_sizes;
        Self {
            descriptor,
            lanes,
            width,
            initial,
            sizes,
            lattices,
        }
    }

    fn bind(&self, factory: &dyn NativeEffectFactory) -> Box<dyn PreparedNativeEffectBank> {
        let requests: Vec<_> = self
            .initial
            .iter()
            .map(|values| request(self.descriptor, values))
            .collect();
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::current(),
                width: self.width,
                requests: &requests,
                active_mask: self.width.full_mask(),
            })
            .unwrap_or_else(|failure| {
                panic!("{}: bind: {}", self.descriptor.id.as_str(), failure.code)
            })
            .unwrap_or_else(|| {
                panic!(
                    "{}: a full distinct bank binds",
                    self.descriptor.id.as_str()
                )
            })
            .processor
    }

    /// Every lane's state payload, in lane order.
    fn payloads(&self, bank: &dyn PreparedNativeEffectBank) -> Vec<Vec<u8>> {
        (0..self.lanes)
            .map(|lane| {
                let mut common = vec![0_u8; self.sizes.common_bytes as usize];
                let mut left = vec![0_u8; self.sizes.left_bytes as usize];
                let mut right = vec![0_u8; self.sizes.right_bytes as usize];
                bank.snapshot_track_state_payload(
                    lane as u32,
                    StatePayloadOutput::new(&mut common, &mut left, &mut right, self.sizes)
                        .expect("payload shape"),
                )
                .unwrap_or_else(|failure| panic!("lane {lane} payload: {}", failure.code));
                [common, left, right].concat()
            })
            .collect()
    }
}

/// One block's input and automation for every lane.
struct Block {
    frames: u32,
    left: Vec<f32>,
    right: Vec<f32>,
    spans: Vec<PreparedAutomationSpan>,
    offsets: Vec<u32>,
}

/// What one block of a run left: both output planes, the rest mask and every lane's payload.
struct Observed {
    left: Vec<f32>,
    right: Vec<f32>,
    rest: u32,
    payloads: Vec<Vec<u8>>,
}

#[derive(Clone, Copy)]
enum Schedule {
    /// Signal on every block.
    Running,
    /// Signal, then silence long enough to settle, the call, more silence, then signal again.
    Resting,
}

/// Blocks before the call and after it.
const BEFORE: usize = 10;
const AFTER: usize = 10;
const RESTING_SILENCE: usize = 48;

fn blocks(fixture: &Fixture, schedule: Schedule, seed: u64) -> (Vec<Block>, usize) {
    let mut rng = Rng(seed);
    let lanes = fixture.lanes;
    let (before, after, silent) = match schedule {
        Schedule::Running => (BEFORE, AFTER, 0..0),
        Schedule::Resting => (
            BEFORE + RESTING_SILENCE,
            RESTING_SILENCE + AFTER,
            BEFORE..BEFORE + 2 * RESTING_SILENCE,
        ),
    };
    // Per-lane signal shape: a level, a tone period and a noise share.
    let shapes: Vec<(f32, f32, f32)> = (0..lanes)
        .map(|_| {
            (
                0.05 + 1.9 * rng.signed().abs(),
                8.0 + rng.below(400) as f32,
                rng.signed().abs(),
            )
        })
        .collect();
    let automatable: Vec<InitialParameterValue> = default_initial_values(fixture.descriptor)
        .filter(|slot| {
            let parameter = &fixture.descriptor.parameters[slot.parameter_index as usize];
            parameter.automatable && live(parameter)
        })
        .collect();
    let mut sample = 0_u64;
    let mut out = Vec::new();
    for index in 0..before + after {
        let frames = FRAMES[index % FRAMES.len()];
        let quiet = silent.contains(&index);
        let size = frames as usize * lanes;
        let mut left = vec![0.0_f32; size];
        let mut right = vec![0.0_f32; size];
        if !quiet {
            for frame in 0..frames as usize {
                let t = (sample + frame as u64) as f32;
                for (lane, &(level, period, noise)) in shapes.iter().enumerate() {
                    let tone = (core::f32::consts::TAU * t / period).sin();
                    left[frame * lanes + lane] =
                        level * (tone * (1.0 - noise) + noise * rng.signed());
                    right[frame * lanes + lane] =
                        level * (0.7 * tone * (1.0 - noise) + noise * rng.signed());
                }
            }
        }
        // Spans: up to two per lane, and on the last block before the call one lands on that
        // block's last frame on every lane, so a ramp is in flight across the boundary.
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        for _lane in 0..lanes {
            if !quiet && !automatable.is_empty() {
                let count = if index + 1 == before { 2 } else { rng.below(3) };
                let mut at: Vec<u64> = (0..count)
                    .map(|_| sample + rng.below(frames as usize) as u64)
                    .collect();
                if index + 1 == before {
                    at[count - 1] = sample + u64::from(frames) - 1;
                }
                at.sort_unstable();
                for when in at {
                    let slot = automatable[rng.below(automatable.len())];
                    let values = &fixture.lattices[slot.parameter_index as usize];
                    spans.push(PreparedAutomationSpan {
                        channel: slot.channel,
                        parameter_index: slot.parameter_index,
                        sample: when,
                        value: values[rng.below(values.len())],
                    });
                }
            }
            offsets.push(spans.len() as u32);
        }
        out.push(Block {
            frames,
            left,
            right,
            spans,
            offsets,
        });
        sample += u64::from(frames);
    }
    (out, before)
}

/// Runs `blocks` on `bank`, calling `at_boundary` before block `boundary`, and records what each
/// block from the boundary on left (the first record is the state the call itself left).
fn run(
    fixture: &Fixture,
    bank: &mut dyn PreparedNativeEffectBank,
    blocks: &[Block],
    boundary: usize,
    at_boundary: &mut dyn FnMut(&mut dyn PreparedNativeEffectBank),
) -> Vec<Observed> {
    let mut sample = 0_u64;
    let mut out = Vec::new();
    for (index, block) in blocks.iter().enumerate() {
        if index == boundary {
            at_boundary(bank);
            out.push(Observed {
                left: Vec::new(),
                right: Vec::new(),
                rest: bank.lanes_at_rest(),
                payloads: fixture.payloads(bank),
            });
        }
        let mut left = block.left.clone();
        let mut right = block.right.clone();
        let _ = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                block.frames,
                fixture.width,
                sample,
                &block.spans,
                &block.offsets,
                QUANTUM,
            )
            .expect("block shape"),
        );
        sample += u64::from(block.frames);
        if index >= boundary {
            out.push(Observed {
                left,
                right,
                rest: bank.lanes_at_rest(),
                payloads: fixture.payloads(bank),
            });
        }
    }
    out
}

fn kind_name(kind: ResetKind) -> &'static str {
    match kind {
        ResetKind::FullToDefaults => "full",
        ResetKind::DiscontinuityKeepParameters => "discontinuity",
    }
}

/// The whole check for one effect.
fn check(effect: &str) {
    bench_alloc::assert_installed();
    let registry = launch_native_effect_registry().expect("launch registry");
    let factory = registry.get_ascii(effect).expect("a launch effect");
    let Some(width) = BankWidth::for_backend(Backend::current()) else {
        return;
    };
    let seed = effect.bytes().fold(0x2131_0000_u64, |hash, byte| {
        hash.rotate_left(7) ^ u64::from(byte)
    });
    let fixture = Fixture::new(factory, width, seed);
    let lanes = fixture.lanes;
    let picks = [0, lanes / 2, lanes - 1];
    for (schedule, schedule_name) in [
        (Schedule::Running, "running"),
        (Schedule::Resting, "resting"),
    ] {
        let (blocks, boundary) = blocks(&fixture, schedule, seed ^ 0x5eed);
        let untouched = run(
            &fixture,
            &mut *fixture.bind(factory),
            &blocks,
            boundary,
            &mut |_| {},
        );
        for kind in [
            ResetKind::FullToDefaults,
            ResetKind::DiscontinuityKeepParameters,
        ] {
            let whole = run(
                &fixture,
                &mut *fixture.bind(factory),
                &blocks,
                boundary,
                &mut |bank| bank.reset(kind),
            );
            for &target in &picks {
                let case = format!("{effect} {schedule_name} {} lane {target}", kind_name(kind));
                let mut heap = 0_u64;
                let one = run(
                    &fixture,
                    &mut *fixture.bind(factory),
                    &blocks,
                    boundary,
                    &mut |bank| {
                        let mark = bench_alloc::current_thread_counters();
                        bank.reset_lane(target, kind);
                        let delta = bench_alloc::current_thread_delta_since(mark);
                        heap = delta.allocations + delta.deallocations + delta.reallocations;
                    },
                );
                assert_eq!(heap, 0, "{case}: reset_lane reached the allocator");
                compare(&case, lanes, target, &one, &untouched, &whole);
            }
            // The check must be able to fail: on the running schedule every picked lane of the
            // whole-reset twin differs from the untouched twin somewhere, or an empty
            // `reset_lane` would pass.
            if matches!(schedule, Schedule::Running) {
                for &target in &picks {
                    let differs = whole.iter().zip(&untouched).any(|(reset, kept)| {
                        reset.payloads[target] != kept.payloads[target]
                            || lane_words(&reset.left, lanes, target)
                                .ne(lane_words(&kept.left, lanes, target))
                    });
                    assert!(
                        differs,
                        "{effect} {}: a whole-bank reset left lane {target} exactly as an \
                         untouched run, so this fixture cannot see an empty reset_lane",
                        kind_name(kind)
                    );
                }
            }
        }
    }
}

fn lane_words(plane: &[f32], lanes: usize, lane: usize) -> impl Iterator<Item = u32> + '_ {
    plane
        .iter()
        .skip(lane)
        .step_by(lanes)
        .map(|word| word.to_bits())
}

/// Lane `target` of `one` against `whole`, every other lane against `untouched`, bit for bit.
fn compare(
    case: &str,
    lanes: usize,
    target: usize,
    one: &[Observed],
    untouched: &[Observed],
    whole: &[Observed],
) {
    for (step, ((got, kept), reset)) in one.iter().zip(untouched).zip(whole).enumerate() {
        let at = if step == 0 {
            "right after the call".to_owned()
        } else {
            format!("after block {step} from the call")
        };
        for lane in 0..lanes {
            let (reference, which) = if lane == target {
                (reset, "the whole-bank reset")
            } else {
                (kept, "the untouched run")
            };
            // The first differing byte, not two whole payloads: a payload is kilobytes.
            let first = got.payloads[lane]
                .iter()
                .zip(&reference.payloads[lane])
                .position(|(got, reference)| got != reference);
            assert!(
                first.is_none(),
                "{case}: lane {lane}'s state payload {at} differs from {which} at byte {first:?}"
            );
            assert!(
                lane_words(&got.left, lanes, lane).eq(lane_words(&reference.left, lanes, lane))
                    && lane_words(&got.right, lanes, lane).eq(lane_words(
                        &reference.right,
                        lanes,
                        lane
                    )),
                "{case}: lane {lane}'s output {at} differs from {which}"
            );
            assert_eq!(
                (got.rest >> lane) & 1,
                (reference.rest >> lane) & 1,
                "{case}: lane {lane}'s rest bit {at} differs from {which}"
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
    reset_lane_compressor => "miso.compressor",
    reset_lane_de_esser => "miso.de-esser",
    reset_lane_dynamic_eq => "miso.dynamic-eq",
    reset_lane_gate_expander => "miso.gate-expander",
    reset_lane_multiband_compressor => "miso.multiband-compressor",
    reset_lane_parametric_eq => "miso.parametric-eq",
    reset_lane_soft_clip => "miso.soft-clip",
    reset_lane_transient_shaper => "miso.transient-shaper",
    reset_lane_true_peak_limiter => "miso.true-peak-limiter",
}

/// Red mutation: a launch effect that binds a bank and has no row here (the reverb, #2114, landing
/// with a bank kernel) -> the set comparison names it. `NEVER_BANKED_EFFECTS` is held to exactly the
/// launch factories that decline a bank by `graph-compiler`'s `bypass_shunt_identity` test, so its
/// complement is the banking set.
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
