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
    BankWidth, EffectBankProcessBlock, EffectDescriptor, EffectQuality, EffectTargetRequest,
    InitialParameterValue, LinkMode, NativeEffectFactory, NativeEffectTargetPreparation,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, ParameterDescriptor, PortRole,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedEffectTarget, PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
    ResetKind, StatePayloadOutput, StatePayloadSizes, decimal_to_f32, default_initial_values,
    parameter_lattice_points,
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

/// One block's input and control for every lane.
struct Block {
    frames: u32,
    left: Vec<f32>,
    right: Vec<f32>,
    spans: Vec<PreparedAutomationSpan>,
    offsets: Vec<u32>,
    /// Per lane: prepared targets applied to the lane before the block.
    targets: Vec<Vec<PreparedEffectTarget>>,
    /// Per lane: a detector-link glide started before the block.
    links: Vec<Option<(LinkMode, u32)>>,
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
    /// Dynamic signal, a long quiet passage, the call, dynamic signal again. Every control is in
    /// flight across the call: a span on the last frame before it, a prepared target and a link
    /// glide started on the block before it.
    Running,
    /// The same, with exact silence on both sides of the call, so rest and silence bookkeeping is
    /// what the call meets.
    Resting,
}

/// Dynamic blocks at the start and after the call.
const DYNAMIC: usize = 24;
/// Quiet full-quantum blocks before the call: over a second at 48 kHz, longer than the gate's
/// longest hold (1000 ms), so a gate is closing and every detector releasing when the call lands.
const QUIET: usize = 420;
/// Exact-silence blocks on each side of the call in the resting schedule.
const SILENT: usize = 48;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// Level drawn per lane per block from -66 dB to +6 dB, with spans and targets.
    Dynamic,
    /// -66 dB to -40 dB, no control: below most thresholds, so the detectors release.
    Quiet,
    /// Exact `+0.0`, no control.
    Silent,
}

fn blocks(
    fixture: &Fixture,
    preparation: Option<&dyn NativeEffectTargetPreparation>,
    schedule: Schedule,
    seed: u64,
) -> (Vec<Block>, usize) {
    let mut rng = Rng(seed);
    let lanes = fixture.lanes;
    let mut phases = vec![Phase::Dynamic; DYNAMIC];
    phases.extend([Phase::Quiet; QUIET]);
    if matches!(schedule, Schedule::Resting) {
        phases.extend([Phase::Silent; SILENT]);
    }
    let before = phases.len();
    if matches!(schedule, Schedule::Resting) {
        phases.extend([Phase::Silent; SILENT]);
    }
    phases.extend([Phase::Dynamic; DYNAMIC]);
    // The last block before the call carries every control in the running schedule.
    let primed = matches!(schedule, Schedule::Running).then_some(before - 1);

    // Per-lane signal shape: a tone period and a noise share.
    let shapes: Vec<(f32, f32)> = (0..lanes)
        .map(|_| (8.0 + rng.below(400) as f32, rng.signed().abs()))
        .collect();
    let automatable: Vec<InitialParameterValue> = default_initial_values(fixture.descriptor)
        .filter(|slot| {
            let parameter = &fixture.descriptor.parameters[slot.parameter_index as usize];
            parameter.automatable && live(parameter)
        })
        .collect();
    let link_modes: Vec<LinkMode> = [LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .filter(|&mode| {
            fixture.descriptor.lane_link && fixture.descriptor.supported_link_modes.contains(mode)
        })
        .collect();
    // Each lane's committed candidate, which its prepared targets are designed from.
    let mut candidates = fixture.initial.clone();
    let mut sample = 0_u64;
    let mut out = Vec::new();
    for (index, &phase) in phases.iter().enumerate() {
        let prime = primed == Some(index);
        // The primed block is shorter than the shortest ramp a span starts (the limiter's 64
        // updates), so a ramp it starts is still in flight at the call.
        let frames = match phase {
            _ if prime => 32,
            Phase::Dynamic => FRAMES[index % FRAMES.len()],
            Phase::Quiet | Phase::Silent => QUANTUM,
        };
        let size = frames as usize * lanes;
        let mut left = vec![0.0_f32; size];
        let mut right = vec![0.0_f32; size];
        if phase != Phase::Silent {
            let (spread, centre) = match phase {
                Phase::Dynamic => (36.0, -30.0),
                _ => (13.0, -53.0),
            };
            let levels: Vec<f32> = (0..lanes)
                .map(|_| math::powf(10.0, (rng.signed() * spread + centre) / 20.0))
                .collect();
            for frame in 0..frames as usize {
                let t = (sample + frame as u64) as f32;
                for (lane, (&(period, noise), &level)) in shapes.iter().zip(&levels).enumerate() {
                    let tone = math::sin(f64::from(core::f32::consts::TAU * t / period)) as f32;
                    left[frame * lanes + lane] =
                        level * (tone * (1.0 - noise) + noise * rng.signed());
                    right[frame * lanes + lane] =
                        level * (0.7 * tone * (1.0 - noise) + noise * rng.signed());
                }
            }
        }
        let controlled = phase == Phase::Dynamic || prime;
        // Spans: up to two per lane. On the primed block every lane takes two, on its first frame
        // (the limiter admits a span only there) and on its last, so a ramp is in flight across
        // the call.
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        let mut targets = vec![Vec::new(); lanes];
        let mut links = vec![None; lanes];
        for lane in 0..lanes {
            if controlled && !automatable.is_empty() {
                let count = if prime { 2 } else { rng.below(3) };
                let mut at: Vec<u64> = (0..count)
                    .map(|_| sample + rng.below(frames as usize) as u64)
                    .collect();
                if prime {
                    at[0] = sample;
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
            // A prepared target on a third of the dynamic blocks and on the primed one: one live
            // parameter of the lane's candidate moves on both channels.
            if let Some(preparation) = preparation
                && controlled
                && !automatable.is_empty()
                && (prime || rng.below(3) == 0)
            {
                targets[lane] = target(
                    fixture,
                    preparation,
                    &mut candidates[lane],
                    &automatable,
                    &mut rng,
                );
            }
            // On the middle lane only, which is a picked lane: every switch on, on the full block
            // before the primed one, so each switched section settles away from identity; then
            // every switch off on the primed block, so each is ramping to identity across the
            // call. Resetting that lane ends its ramp at identity, which flips its dry bit in each
            // dedicated cut (the EQ's HPF and LPF). No other lane ramps those sections, so nothing
            // but the reset itself re-derives the flags, and a stale one meets the EQ's debug
            // check of its flags on the next stationary block.
            if let Some(preparation) = preparation
                && lane == lanes / 2
            {
                if primed == Some(index + 1) {
                    targets[lane] = switch(fixture, preparation, &mut candidates[lane], true);
                } else if prime {
                    let off = switch(fixture, preparation, &mut candidates[lane], false);
                    targets[lane].extend(off);
                }
            }
            // A link glide on the primed block, long enough to be in flight across the call.
            if prime && !link_modes.is_empty() {
                links[lane] = Some((link_modes[rng.below(link_modes.len())], 4096));
            }
        }
        out.push(Block {
            frames,
            left,
            right,
            spans,
            offsets,
            targets,
            links,
        });
        sample += u64::from(frames);
    }
    (out, before)
}

/// Moves one automatable parameter of `candidate` to a fresh lattice draw and designs its targets.
/// A refused design keeps the candidate and returns no target.
fn target(
    fixture: &Fixture,
    preparation: &dyn NativeEffectTargetPreparation,
    candidate: &mut Vec<InitialParameterValue>,
    automatable: &[InitialParameterValue],
    rng: &mut Rng,
) -> Vec<PreparedEffectTarget> {
    let parameter = automatable[rng.below(automatable.len())].parameter_index;
    let values = &fixture.lattices[parameter as usize];
    let value = values[rng.below(values.len())];
    design(preparation, candidate, &mut |slot| {
        (slot.parameter_index == parameter).then_some(value)
    })
}

/// Sets every switch (a boolean `*_enabled` parameter) of `candidate` to `on` and designs its
/// targets: every section a switch governs ramps to or from the identity words.
fn switch(
    fixture: &Fixture,
    preparation: &dyn NativeEffectTargetPreparation,
    candidate: &mut Vec<InitialParameterValue>,
    on: bool,
) -> Vec<PreparedEffectTarget> {
    let descriptor = fixture.descriptor;
    design(preparation, candidate, &mut |slot| {
        let parameter = &descriptor.parameters[slot.parameter_index as usize];
        (parameter.automatable && live(parameter) && parameter.name.ends_with("_enabled"))
            .then_some(if on { 1.0 } else { 0.0 })
    })
}

/// Applies `change` to every slot of `candidate` it names and designs the targets of the result.
/// A refused design keeps the candidate and returns no target.
fn design(
    preparation: &dyn NativeEffectTargetPreparation,
    candidate: &mut Vec<InitialParameterValue>,
    change: &mut dyn FnMut(&InitialParameterValue) -> Option<f32>,
) -> Vec<PreparedEffectTarget> {
    let mut next = candidate.clone();
    let mut changed = vec![false; next.len()];
    for (slot, touched) in next.iter_mut().zip(&mut changed) {
        if let Some(value) = change(slot) {
            slot.value = value;
            *touched = true;
        }
    }
    if !changed.contains(&true) {
        return Vec::new();
    }
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
    *candidate = next;
    out.truncate(count);
    out
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
        for (lane, (targets, link)) in block.targets.iter().zip(&block.links).enumerate() {
            for target in targets {
                bank.apply_prepared_target_lane(lane, target)
                    .unwrap_or_else(|failure| panic!("lane {lane} target: {failure:?}"));
            }
            if let Some((mode, samples)) = *link {
                assert!(
                    bank.retarget_link(lane, mode, samples),
                    "lane {lane} link glide"
                );
            }
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

/// How the reset lane's state payload is judged against the whole-bank reset's.
#[derive(Clone, Copy, PartialEq)]
enum LanePayload {
    /// Byte for byte.
    Exact,
    /// Not at all: the payload carries the bank's shared clocks (the limiter's ring cursors, and
    /// the van Herk phase and ring rotation measured from them). The whole-bank reset rewinds
    /// those clocks and `reset_lane` must not, because every other lane's payload carries them
    /// too (spec item 3: the limiter's shared cursors stay where per-lane D7 recovery leaves
    /// them). The reset lane is then judged by what the clocks cannot change: every output word
    /// and its rest bit. Every other lane's payload is still compared byte for byte.
    SharedClocks,
}

/// The whole check for one effect.
fn check(effect: &str, lane_payload: LanePayload) {
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
        let (blocks, boundary) = blocks(
            &fixture,
            factory.target_preparation(),
            schedule,
            seed ^ 0x5eed,
        );
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
                compare(&case, lanes, target, lane_payload, &one, &untouched, &whole);
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
    lane_payload: LanePayload,
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
            let judged = lane != target || lane_payload == LanePayload::Exact;
            let first = got.payloads[lane]
                .iter()
                .zip(&reference.payloads[lane])
                .position(|(got, reference)| got != reference)
                .filter(|_| judged);
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
    ($($name:ident => $effect:literal $payload:ident,)*) => {
        const ROWS: &[&str] = &[$($effect),*];
        $(
            #[test]
            fn $name() {
                check($effect, LanePayload::$payload);
            }
        )*
    };
}

rows! {
    reset_lane_compressor => "miso.compressor" Exact,
    reset_lane_de_esser => "miso.de-esser" Exact,
    reset_lane_dynamic_eq => "miso.dynamic-eq" Exact,
    reset_lane_gate_expander => "miso.gate-expander" Exact,
    reset_lane_multiband_compressor => "miso.multiband-compressor" Exact,
    reset_lane_parametric_eq => "miso.parametric-eq" Exact,
    reset_lane_soft_clip => "miso.soft-clip" Exact,
    reset_lane_transient_shaper => "miso.transient-shaper" Exact,
    reset_lane_true_peak_limiter => "miso.true-peak-limiter" SharedClocks,
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
