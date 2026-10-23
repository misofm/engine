//! Gate support for the render-time lane swap (issue #1747 gate 1).
//!
//! The one driver each effect's swap test runs, on the model of [`crate::rest_probe`]: the six
//! tests share one rig, one schedule and one check, and each effect's test stays in its own crate
//! and binds its own banks through [`Probe`]. A dev-dependency of each effect crate; nothing on a
//! render path reaches it.
//!
//! The claim, stated per strip: a strip whose lane `effect_contract::swap_bank_lanes` moves,
//! inside one bank (`second = None`) and across two banks of one slot, with its ramps in flight,
//! renders bit for bit as a twin that never moved, holds the twin's payload, re-seeds from its own
//! prepared defaults on a full reset, and the swap reaches the allocator zero times.

use effect_contract::{
    BankProcessReport, EffectBankProcessBlock, PreparedAutomationSpan, PreparedEffectTarget,
    PreparedNativeEffectBank, ResetKind, StatePayloadOutput, StatePayloadSizes, swap_bank_lanes,
    swap_bank_lanes_scratch_bytes,
};

use crate::SplitMix64;

/// How a strip is automated on one block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Automate {
    /// No span and no target.
    Nothing,
    /// The effect's own random draw: some parameters, or none.
    Random,
    /// Every automatable parameter on both channels, at the block's first sample, so every ramp
    /// of the strip opens and is in flight at the end of a block shorter than the ramps.
    Every,
}

/// One block of control for one strip: spans, which the block carries, and prepared targets,
/// which the driver applies to the strip's lane before the block.
#[derive(Clone, Debug, Default)]
pub struct Automation {
    /// The strip's spans, in the order the effect admits them.
    pub spans: Vec<PreparedAutomationSpan>,
    /// The strip's prepared targets, applied in order through
    /// `PreparedNativeEffectBank::apply_prepared_target_lane`.
    pub targets: Vec<PreparedEffectTarget>,
}

/// Two banks of one slot under test, bound by the effect's own test.
///
/// Strip `s` is prepared on bank `s / lanes`, lane `s % lanes`, and every draw is keyed by the
/// strip, never by the lane, so a strip's input and automation follow it to its new lane.
pub trait Probe {
    /// Lanes per bank, padded lanes included. The driver swaps only populated lanes, so every
    /// lane of both banks must be a member.
    fn lanes(&self) -> usize;
    /// The bank width to state in a block.
    fn width(&self) -> effect_contract::BankWidth;
    /// The prepared quantum: the longest block.
    fn quantum(&self) -> u32;
    /// The payload sizes of one lane.
    fn sizes(&self) -> StatePayloadSizes;
    /// The effect's state layout version.
    fn state_layout_version(&self) -> u32;
    /// Binds bank `which` (0 is A, 1 is B). Called twice for each bank, once for the bank that
    /// swaps and once for its twin, and the two binds must be equal.
    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank>;
    /// Fills one block of strip `strip`'s input; each plane is one frame per word.
    fn input(&mut self, strip: usize, left: &mut [f32], right: &mut [f32]);
    /// Strip `strip`'s automation for the block that starts at `first_sample`.
    fn automate(&mut self, strip: usize, how: Automate, first_sample: u64) -> Automation;
    /// Whether every target of the strip's last [`Automation`] applied (identically on the
    /// strip's lane and its twin's, which the driver asserts). The default ignores it.
    fn targets_applied(&mut self, strip: usize, applied: bool) {
        let _ = (strip, applied);
    }
    /// Whether a lane whose payload channel sections are `left` and `right` has a ramp in flight.
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool;
    /// The two lanes of bank A the first swap exchanges, when the effect needs a particular
    /// pair; the default lets the driver draw two distinct lanes.
    fn inside_pair(&mut self) -> Option<(usize, usize)> {
        None
    }
    /// Runs `call` and returns how many times it reached the allocator on this thread
    /// (allocations, frees and reallocations): the crate's test reads `bench_support::alloc`.
    fn heap_calls(&self, call: &mut dyn FnMut()) -> u64;
}

/// Blocks of random automation before the first swap.
const WARM: usize = 6;
/// Blocks of random automation between the block after the first swap and the second swap.
const BETWEEN: usize = 4;
/// Blocks of random automation after the block after the second swap.
const AFTER: usize = 8;
/// The longest block that ends with a strip's ramps in flight: shorter than every effect's
/// ramps. The check that the ramps are in flight is the driver's, not this constant's.
const SHORT: usize = 48;
/// The most quantum-long blocks the destination bank of the second swap gets to settle.
const SETTLE: usize = 8;

/// A lane's payload: common, left and right sections.
type Payload = [Vec<u8>; 3];

fn payload(bank: &dyn PreparedNativeEffectBank, lane: usize, sizes: StatePayloadSizes) -> Payload {
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
            .expect("the prepared sizes"),
    )
    .expect("a lane snapshot");
    [common, left, right]
}

/// The rig: banks 0 and 1 (A and B) swap, banks 2 and 3 are their twins and never do.
struct Rig<'p, P: Probe> {
    probe: &'p mut P,
    banks: [Box<dyn PreparedNativeEffectBank>; 4],
    /// Strip `s` sits at `place[s]` (bank, lane) in A and B; its twin sits at home.
    place: Vec<(usize, usize)>,
    lanes: usize,
    sizes: StatePayloadSizes,
    scratch: Vec<u8>,
    first_sample: u64,
    block: usize,
    context: String,
}

impl<P: Probe> Rig<'_, P> {
    fn home(&self, strip: usize) -> (usize, usize) {
        (2 + strip / self.lanes, strip % self.lanes)
    }

    fn strip_at(&self, at: (usize, usize)) -> usize {
        self.place
            .iter()
            .position(|&place| place == at)
            .expect("a strip at every place")
    }

    fn in_flight(&self, bank: usize, lane: usize) -> bool {
        let [_, left, right] = payload(self.banks[bank].as_ref(), lane, self.sizes);
        self.probe.ramp_in_flight(&left, &right)
    }

    /// One block of `frames` frames through all four banks, strip `s` automated by `how(s)`;
    /// every strip's output bits and report equal its twin's.
    fn block(&mut self, frames: usize, how: impl Fn(usize) -> Automate) {
        let lanes = self.lanes;
        let strips = 2 * lanes;
        let context = format!("{} block {} ({frames} frames)", self.context, self.block);
        let mut left = vec![vec![0.0_f32; frames * lanes]; 4];
        let mut right = left.clone();
        let mut spans: Vec<Vec<Vec<PreparedAutomationSpan>>> = vec![vec![Vec::new(); lanes]; 4];
        let (mut strip_left, mut strip_right) = (vec![0.0_f32; frames], vec![0.0_f32; frames]);
        for strip in 0..strips {
            self.probe.input(strip, &mut strip_left, &mut strip_right);
            let automation = self.probe.automate(strip, how(strip), self.first_sample);
            let at = [self.place[strip], self.home(strip)];
            let mut applied = true;
            for target in &automation.targets {
                let moved = self.banks[at[0].0].apply_prepared_target_lane(at[0].1, target);
                let still = self.banks[at[1].0].apply_prepared_target_lane(at[1].1, target);
                assert_eq!(
                    moved, still,
                    "{context}: strip {strip}'s target applies alike in both runs"
                );
                applied &= moved.is_ok();
            }
            if !automation.targets.is_empty() {
                self.probe.targets_applied(strip, applied);
            }
            for (bank, lane) in at {
                for frame in 0..frames {
                    left[bank][frame * lanes + lane] = strip_left[frame];
                    right[bank][frame * lanes + lane] = strip_right[frame];
                }
                spans[bank][lane].clone_from(&automation.spans);
            }
        }
        let width = self.probe.width();
        let quantum = self.probe.quantum();
        let reports: Vec<BankProcessReport> = (0..4)
            .map(|bank| {
                let mut offsets = vec![0_u32];
                for lane in &spans[bank] {
                    offsets.push(offsets[offsets.len() - 1] + lane.len() as u32);
                }
                let flat = spans[bank].concat();
                let process = EffectBankProcessBlock::new(
                    &mut left[bank],
                    &mut right[bank],
                    None,
                    frames as u32,
                    width,
                    self.first_sample,
                    &flat,
                    &offsets,
                    quantum,
                )
                .expect("a bank block");
                self.banks[bank].process_bank(process)
            })
            .collect();
        for strip in 0..strips {
            let ((bank, lane), (twin, twin_lane)) = (self.place[strip], self.home(strip));
            for frame in 0..frames {
                for (plane, name) in [(&left, "left"), (&right, "right")] {
                    assert_eq!(
                        plane[bank][frame * lanes + lane].to_bits(),
                        plane[twin][frame * lanes + twin_lane].to_bits(),
                        "{context}: strip {strip} at {:?}: {name} frame {frame} differs from its \
                         twin",
                        self.place[strip],
                    );
                }
            }
            assert_eq!(
                reports[bank].reports[lane], reports[twin].reports[twin_lane],
                "{context}: strip {strip}'s report"
            );
        }
        self.first_sample += frames as u64;
        self.block += 1;
    }

    /// Every strip's payload, where it sits now, equals its twin's.
    fn assert_payloads(&self, when: &str) {
        for strip in 0..2 * self.lanes {
            let ((bank, lane), (twin, twin_lane)) = (self.place[strip], self.home(strip));
            assert_eq!(
                payload(self.banks[bank].as_ref(), lane, self.sizes),
                payload(self.banks[twin].as_ref(), twin_lane, self.sizes),
                "{} {when}: strip {strip}'s payload differs from its twin's",
                self.context
            );
        }
    }

    /// Swaps lane `first` of A with lane `second` of `bank` (0 is A itself, `second = None`)
    /// once to warm the process statics and once back, then once measured: exactly zero calls to
    /// the allocator. The strips follow.
    fn swap(&mut self, first: usize, bank: usize, second: usize) {
        let context = format!("{} swap (0, {first}) <-> ({bank}, {second})", self.context);
        let version = self.probe.state_layout_version();
        let sizes = self.sizes;
        let [a, b, ..] = &mut self.banks;
        let scratch = &mut self.scratch;
        let mut call = || {
            let second_bank: Option<&mut dyn PreparedNativeEffectBank> =
                if bank == 0 { None } else { Some(b.as_mut()) };
            swap_bank_lanes(
                a.as_mut(),
                first,
                second_bank,
                second,
                version,
                sizes,
                scratch,
            )
        };
        assert_eq!(call(), Ok(()), "{context}: the warm-up swap");
        assert_eq!(call(), Ok(()), "{context}: the warm-up swap back");
        let mut result = None;
        let heap = self.probe.heap_calls(&mut || result = Some(call()));
        assert_eq!(result, Some(Ok(())), "{context}: the swap");
        assert_eq!(heap, 0, "{context}: the swap reached the allocator");
        for place in &mut self.place {
            if *place == (0, first) {
                *place = (bank, second);
            } else if *place == (bank, second) {
                *place = (0, first);
            }
        }
        self.assert_payloads(&format!("after swap (0, {first}) <-> ({bank}, {second})"));
    }
}

/// Runs one case on `probe`.
///
/// The frozen schedule: [`WARM`] blocks of random lengths and random automation; one block shorter
/// than the ramps on which every strip automates every parameter; a swap of two lanes of A
/// (`second = None`), both with a ramp in flight; one block that automates nothing; [`BETWEEN`]
/// random blocks. Then, for the swap of a lane of A with a lane of B: one of the two strips (the
/// mover) automates every parameter on quantum-long blocks while every other strip automates
/// nothing, until the other bank (the destination) has no ramp in flight on any lane; one more
/// short block of the same; the swap; and one block that automates nothing, so a ramping lane
/// lands in a settled bank and must keep ramping there with no new target to restart it. Then
/// [`AFTER`] random blocks, and last a full reset (`ResetKind::FullToDefaults`) of all four banks
/// and one block that automates nothing, so a strip whose prepared defaults stayed behind
/// re-seeds from another strip's values. Each swap is measured at zero allocator calls.
///
/// # Panics
///
/// Panics when a strip's output bits, report or payload differ from its twin's, when a swap is
/// refused or reaches the allocator, or when the schedule does not reach a swapped strip with a
/// ramp in flight or a settled destination bank.
pub fn check<P: Probe>(probe: &mut P, seed: u64) {
    let lanes = probe.lanes();
    assert!(lanes >= 2, "a bank of at least two lanes");
    let quantum = probe.quantum() as usize;
    let sizes = probe.sizes();
    let banks = [probe.bind(0), probe.bind(1), probe.bind(0), probe.bind(1)];
    let scratch_bytes =
        swap_bank_lanes_scratch_bytes(banks[0].as_ref(), sizes).expect("declared defaults");
    let mut draw = SplitMix64::new(seed);
    let mut below = |bound: usize| (draw.next_u64() % bound.max(1) as u64) as usize;
    let mut rig = Rig {
        probe,
        banks,
        place: (0..2 * lanes).map(|s| (s / lanes, s % lanes)).collect(),
        lanes,
        sizes,
        scratch: vec![0_u8; scratch_bytes],
        first_sample: 0,
        block: 0,
        context: format!("W{lanes} seed {seed:#x}"),
    };
    let random = |_: usize| Automate::Random;
    let nothing = |_: usize| Automate::Nothing;
    let short = SHORT.min(quantum);

    for _ in 0..WARM {
        rig.block(1 + below(quantum), random);
    }
    let (i, j) = rig.probe.inside_pair().unwrap_or_else(|| {
        let i = below(lanes);
        (i, (i + 1 + below(lanes - 1)) % lanes)
    });
    assert_ne!(i, j, "two distinct lanes");
    rig.block(1 + below(short), |_| Automate::Every);
    for lane in [i, j] {
        assert!(
            rig.in_flight(0, lane),
            "{}: lane {lane} of A has no ramp in flight at the first swap",
            rig.context
        );
    }
    rig.swap(i, 0, j);
    rig.block(1 + below(quantum), nothing);
    for _ in 0..BETWEEN {
        rig.block(1 + below(quantum), random);
    }

    let across = (below(lanes), below(lanes));
    let (mover_at, destination) = if below(2) == 0 {
        ((0, across.0), 1)
    } else {
        ((1, across.1), 0)
    };
    let mover = rig.strip_at(mover_at);
    let mover_only = |strip: usize| {
        if strip == mover {
            Automate::Every
        } else {
            Automate::Nothing
        }
    };
    let mut settled = false;
    for _ in 0..SETTLE {
        rig.block(quantum, mover_only);
        settled = (0..lanes).all(|lane| !rig.in_flight(destination, lane));
        if settled {
            break;
        }
    }
    assert!(
        settled,
        "{}: bank {destination} did not settle",
        rig.context
    );
    rig.block(1 + below(short), mover_only);
    assert!(
        rig.in_flight(mover_at.0, mover_at.1),
        "{}: the mover has no ramp in flight at the second swap",
        rig.context
    );
    rig.swap(across.0, 1, across.1);
    rig.block(1 + below(quantum), nothing);
    for _ in 0..AFTER {
        rig.block(1 + below(quantum), random);
    }
    rig.assert_payloads("at the end");
    assert_ne!(
        rig.place,
        (0..2 * lanes)
            .map(|s| (s / lanes, s % lanes))
            .collect::<Vec<_>>(),
        "{}: the strips moved",
        rig.context
    );

    for bank in &mut rig.banks {
        bank.reset(ResetKind::FullToDefaults);
    }
    rig.block(1 + below(quantum), nothing);
    rig.assert_payloads("after a full reset");
}
