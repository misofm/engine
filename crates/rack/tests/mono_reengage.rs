//! Mono-collapse M3: the re-engage rule.
//!
//! The chain's dispatch reads two things -- the channel-symmetry witness, and
//! [`BankChain::collapse_channels_agree`], the invariant that says whether the two channels' state
//! is where the engage direction needs it to be. M2 had only the first and stood a one-way latch in
//! for the second, so **no M2 test ever rendered a re-engaged block.** These do.
//!
//! # Why this file is here and not in `src/lib.rs`'s inline module
//!
//! The stage these tests need has to be reachable from the test body *after* the chain has taken
//! ownership of it -- a test moves the witness and the designed words between blocks, the way a
//! drain does, and reads the counters back afterwards. `BankStage` is `Send`, so the handle is an
//! `Arc<Mutex<..>>`, and `scripts/check-rack-policy.sh` forbids `std::sync` anywhere under
//! `crates/rack/src` -- rightly, because that directory is render code. An integration
//! test is not, so the sharing lives here, next to `console_bank.rs`, and everything below goes
//! through the crate's public surface.

use std::sync::{Arc, Mutex};

use effect_contract::{BankWidth, ChannelSymmetryWitness, SeamSide};
use engine::realtime::RenderError;
use rack::{AoSoaScratch, BankBlock, BankChain, BankMembers, BankSlot, BankStage};

/// Planar per-lane buffers, the shape a chain gathers from and scatters to.
struct Planes {
    left: Vec<Vec<f32>>,
    right: Vec<Vec<f32>>,
}
impl BankMembers for Planes {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        (&self.left[lane], &self.right[lane])
    }
    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        (&mut self.left[lane], &mut self.right[lane])
    }
}

fn slot(active_lanes: Vec<bool>, stage: Box<dyn BankStage>) -> BankSlot {
    BankSlot {
        stage,
        active_lanes: active_lanes.into_boxed_slice(),
    }
}

/// A seam-side 2x2 in the strip's frozen operation order, so the chain has a legal seam suffix.
///
/// `yl = ll*l + lr*r`, never `(ll + lr) * l`: the inline suite's
/// `a_collapsed_seam_keeps_the_matrixs_operation_order` is the gate on that, and this is the same
/// body so that these tests sit behind the same seam the collapse actually ships with.
struct Matrix {
    coefficients: [f32; 4],
}
impl BankStage for Matrix {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let [ll, lr, rl, rr] = self.coefficients;
        for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
            let (l, r) = (*left, *right);
            *left = ll * l + lr * r;
            *right = rl * l + rr * r;
        }
        Ok(())
    }
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
    fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
        ChannelSymmetryWitness::SYMMETRIC
    }
}

fn mono_chain(lanes: usize, frames: u32, slots: Vec<Box<dyn BankStage>>) -> BankChain {
    let active: Vec<bool> = vec![true; lanes];
    let width = if lanes == 4 {
        BankWidth::Four
    } else {
        BankWidth::Eight
    };
    let slots = slots
        .into_iter()
        .map(|stage| slot(active.clone(), stage))
        .collect();
    BankChain::new(
        AoSoaScratch::new(width, frames).expect("scratch"),
        active.into_boxed_slice(),
        slots,
    )
    .expect("chain")
}

/// Finite, identical, per-block-distinct planes: the controlled input the M3 tests need.
///
/// The inline suite's `identical_planes` salts NaN, both infinities and both zeros across every lane, which is
/// what the seam's operation-order gates exist for and exactly wrong here: a recursive state
/// fed an infinity saturates, and two channels that have saturated agree again however far
/// apart their coefficients drove them. These are ordinary numbers, identical on the two
/// planes -- the `SOURCE` term, made concrete.
fn finite_planes(lanes: usize, frames: u32, block: u64) -> Planes {
    let plane: Vec<Vec<f32>> = (0..lanes)
        .map(|lane| {
            (0..frames)
                .map(|frame| {
                    let step = block * u64::from(frames) + u64::from(frame);
                    let salt = (lane as u64 * 7 + step * 13) % 97;
                    0.125 + salt as f32 / 512.0
                })
                .collect()
        })
        .collect();
    Planes {
        left: plane.clone(),
        right: plane,
    }
}

/// [`finite_planes`] at exactly `+0.0`: the input a silence fixed point is earned on.
fn silent_planes(lanes: usize, frames: u32) -> Planes {
    let plane: Vec<Vec<f32>> = (0..lanes).map(|_| vec![0.0; frames as usize]).collect();
    Planes {
        left: plane.clone(),
        right: plane,
    }
}

/// A stage with genuine per-channel recursive state, a rest state, and a movable witness.
///
/// [`Scale`] cannot exercise M3: its two channels differ only in what they *write*, so a
/// window rendered dual under a declining witness leaves them exactly as symmetric as it found
/// them and every re-engage looks sound. The rule is about state that has been driven apart,
/// so the stage that tests it has to have some.
///
/// `state = state * coefficient[channel] + input`, per lane, flushed to `+0.0` below
/// [`FLUSH`](RecursiveCore::FLUSH). Two coefficients, so a dual window with the `DESIGNED` term
/// down genuinely separates the channels; a flush floor, so a window of silence brings them
/// back to one exact rest state -- which is the shipped effects' `silent_fixed_point` in
/// miniature, and what [`BankStage::channels_agree`] is able to prove.
///
/// The core is shared so a test can move the witness and the coefficients between blocks, the
/// way a drain does, and read the counters back afterwards.
#[derive(Default)]
struct RecursiveCore {
    coefficient: [f32; 2],
    state: [[f32; 8]; 2],
    witness: ChannelSymmetryWitness,
    desymmetrized: usize,
    agreement_queries: usize,
}
impl RecursiveCore {
    /// The denormal-ish floor that gives this kernel an exact rest state.
    const FLUSH: f32 = 1e-12;

    fn advance(&mut self, channel: usize, lane: usize, input: f32) -> f32 {
        let mut next = self.state[channel][lane] * self.coefficient[channel] + input;
        if next.abs() < Self::FLUSH {
            next = 0.0;
        }
        self.state[channel][lane] = next;
        next
    }

    /// Whether the two channels' state -- the whole of what `desymmetrize` copies -- is
    /// bit-equal. The comparison is on raw bits, so `+0.0` and `-0.0` disagree.
    fn states_agree(&self) -> bool {
        self.state[0]
            .iter()
            .zip(self.state[1].iter())
            .all(|(left, right)| left.to_bits() == right.to_bits())
    }
}

#[derive(Clone)]
struct Recursive(Arc<Mutex<RecursiveCore>>);
impl Recursive {
    fn new(coefficient: [f32; 2]) -> Self {
        Self(Arc::new(Mutex::new(RecursiveCore {
            coefficient,
            witness: ChannelSymmetryWitness::SYMMETRIC,
            ..RecursiveCore::default()
        })))
    }
    fn core(&self) -> std::sync::MutexGuard<'_, RecursiveCore> {
        self.0.lock().expect("recursive core")
    }
}
impl BankStage for Recursive {
    fn process(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let mut core = self.core();
        for frame in 0..block.frames as usize {
            for lane in 0..block.lanes {
                let index = frame * block.lanes + lane;
                block.left[index] = core.advance(0, lane, block.left[index]);
                block.right[index] = core.advance(1, lane, block.right[index]);
            }
        }
        Ok(())
    }
    fn process_mono(&mut self, block: BankBlock<'_>) -> Result<(), RenderError> {
        let mut core = self.core();
        for frame in 0..block.frames as usize {
            for lane in 0..block.lanes {
                let index = frame * block.lanes + lane;
                block.left[index] = core.advance(0, lane, block.left[index]);
            }
        }
        Ok(())
    }
    fn supports_mono_collapse(&self) -> bool {
        true
    }
    fn desymmetrize(&mut self) {
        let mut core = self.core();
        core.state[1] = core.state[0];
        core.desymmetrized += 1;
    }
    fn channels_agree(&self) -> bool {
        let mut core = self.core();
        core.agreement_queries += 1;
        core.states_agree()
    }
    fn lane_symmetry(&self, _lane: usize) -> ChannelSymmetryWitness {
        self.core().witness
    }
}

/// One armed chain of a single [`Recursive`] prefix slot behind the frozen seam matrix.
fn recursive_chain(stage: &Recursive) -> BankChain {
    let mut chain = mono_chain(
        4,
        8,
        vec![
            Box::new(stage.clone()),
            Box::new(Matrix {
                coefficients: [1.0, 0.0, 0.0, 1.0],
            }),
        ],
    );
    chain.arm_mono_collapse(true);
    chain
}

/// The switch coming back re-engages, and the whole cycle renders the never-collapsed bits.
///
/// # What replaced what
///
/// M2 latched the disengage: `collapse_retired` was set by the block that stopped and never
/// cleared, so a chain that stopped stayed dual for the life of the plan. The latch was not
/// timidity -- engaging on a re-equal witness is genuinely unsound, and
/// `re_equal_words_after_a_desymmetrised_episode_do_not_re_engage` is the session that proves
/// it -- it was that M2 had no way to tell this case from that one.
///
/// This is the case the latch was wrong about. The force-off switch is the paired
/// measurement's second arm: it never touches a designed word, so the witness holds for every
/// block of the forced-off window, and the disengage copy that opened the window left the two
/// channels holding one state. Equal inputs over equal state with equal words leave equal
/// state, block after block, so at the moment the switch comes back the premise the engage
/// direction needs is exactly as true as it was the first time. The chain re-engages, and the
/// oracle -- the same chain, same input, never collapsed -- says it rendered the same bits.
///
/// # Red mutation
///
/// Delete `self.collapse_channels_agree = true;` from `disengage_collapse` and the chain never
/// comes back: the transition triple reads `[1, 0, 0]` and `collapses()` is 4 rather than 8.
#[test]
fn the_forced_off_window_re_engages_and_renders_the_never_collapsed_bits() {
    let stage = Recursive::new([0.5, 0.5]);
    let oracle_stage = Recursive::new([0.5, 0.5]);
    let mut chain = recursive_chain(&stage);
    let mut oracle = recursive_chain(&oracle_stage);
    oracle.force_mono_collapse_off(true);

    let mut observed = Vec::new();
    let mut expected = Vec::new();
    for block in 0..12_u64 {
        if block == 4 {
            chain.force_mono_collapse_off(true);
        }
        if block == 8 {
            chain.force_mono_collapse_off(false);
        }
        let mut planes = finite_planes(4, 8, block);
        let mut oracle_planes = finite_planes(4, 8, block);
        chain.run(&mut planes, 8, block * 8).expect("render");
        oracle
            .run(&mut oracle_planes, 8, block * 8)
            .expect("render");
        for lane in 0..4 {
            for frame in 0..8 {
                observed.push((
                    planes.left[lane][frame].to_bits(),
                    planes.right[lane][frame].to_bits(),
                ));
                expected.push((
                    oracle_planes.left[lane][frame].to_bits(),
                    oracle_planes.right[lane][frame].to_bits(),
                ));
            }
        }
    }

    assert_eq!(
        chain.collapse_transitions(),
        [1, 1, 0],
        "one disengage, one re-engage, and no agreement proof: the disengage copy is what \
         re-established the premise, so the recovery path was never asked"
    );
    assert_eq!(
        stage.core().agreement_queries,
        0,
        "the recovery query is only asked inside a recovery window, and there was none"
    );
    assert_eq!(
        chain.collapses(),
        8,
        "four blocks either side of the window"
    );
    assert_eq!(oracle.collapses(), 0, "the oracle never collapsed");
    assert_eq!(
        observed, expected,
        "a session that collapsed, stopped and started again must render the bits of one \
         that never collapsed at all"
    );
}

/// A bypass episode re-engages: it never moved the two channels apart.
///
/// `UNBYPASSED` is the one witness term that comes back within a plan (a live `Bypass(true)`
/// followed by a `Bypass(false)`), and it is the one term whose absence does **not** separate
/// the channels -- a bypassed lane still runs the bank on both planes and the shunt restores
/// the same dry block into both. `ChannelSymmetryWitness::AGREEING` is that distinction, and
/// this is what it buys: a chain that would otherwise be retired for the duration of a bypass
/// it rendered correctly on both channels throughout.
///
/// # Red mutation
///
/// Make `AGREEING` equal to `ALL` -- fold `UNBYPASSED` back into the invariant -- and the
/// bypass window retires this chain: `collapses()` falls to 4 and the transition triple loses
/// its re-engage.
#[test]
fn a_bypass_episode_re_engages_because_it_never_moved_the_channels_apart() {
    let stage = Recursive::new([0.5, 0.5]);
    let oracle_stage = Recursive::new([0.5, 0.5]);
    let mut chain = recursive_chain(&stage);
    let mut oracle = recursive_chain(&oracle_stage);
    oracle.force_mono_collapse_off(true);

    let mut observed = Vec::new();
    let mut expected = Vec::new();
    for block in 0..12_u64 {
        let bypassed = (4..8).contains(&block);
        let witness = if bypassed {
            ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::UNBYPASSED)
        } else {
            ChannelSymmetryWitness::SYMMETRIC
        };
        stage.core().witness = witness;
        oracle_stage.core().witness = witness;
        let mut planes = finite_planes(4, 8, block);
        let mut oracle_planes = finite_planes(4, 8, block);
        chain.run(&mut planes, 8, block * 8).expect("render");
        oracle
            .run(&mut oracle_planes, 8, block * 8)
            .expect("render");
        for lane in 0..4 {
            for frame in 0..8 {
                observed.push((
                    planes.left[lane][frame].to_bits(),
                    planes.right[lane][frame].to_bits(),
                ));
                expected.push((
                    oracle_planes.left[lane][frame].to_bits(),
                    oracle_planes.right[lane][frame].to_bits(),
                ));
            }
        }
    }

    assert_eq!(chain.collapse_transitions(), [1, 1, 0]);
    assert_eq!(chain.collapses(), 8);
    assert_eq!(observed, expected);
}

/// Re-equal designed words after a de-symmetrised episode do **not** re-engage.
///
/// # The one design flaw this rule exists to exclude
///
/// "The witness came back, so collapse again" is unsound, and this is the session that shows
/// it. Blocks 4..8 render dual with the `DESIGNED` term down and the two coefficients
/// genuinely different, so the two channels' recursive state separates. At block 8 the
/// coefficients are made equal again -- a `ParameterChannel::Both` retarget reaching its
/// target is exactly this -- and the witness is eligible once more. Every term of it holds.
/// The states still differ, and a collapse here would publish the left channel's state as the
/// right channel's.
///
/// The decline is not vacuous: this asserts the states really do disagree at that boundary, so
/// the test would still fail if the episode stopped separating them.
///
/// # Red mutation
///
/// Drop `&& self.collapse_channels_agree` from `BankChain::run`'s dispatch -- which is exactly
/// what M2 would do if its latch were simply deleted -- and this fails on the output
/// comparison, not on a counter: the re-engaged blocks publish the left plane on both channels
/// while the never-collapsed oracle publishes two channels that have been apart since block 4.
#[test]
fn re_equal_words_after_a_desymmetrised_episode_do_not_re_engage() {
    let stage = Recursive::new([0.5, 0.5]);
    let oracle_stage = Recursive::new([0.5, 0.5]);
    let mut chain = recursive_chain(&stage);
    let mut oracle = recursive_chain(&oracle_stage);
    oracle.force_mono_collapse_off(true);

    let mut observed = Vec::new();
    let mut expected = Vec::new();
    for block in 0..12_u64 {
        for core in [&stage, &oracle_stage] {
            let mut core = core.core();
            if block == 4 {
                // The episode: one channel's word moves, and the witness says so.
                core.coefficient[1] = 0.25;
                core.witness =
                    ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED);
            }
            if block == 10 {
                // The words agree again. The states do not.
                core.coefficient[1] = core.coefficient[0];
                core.witness = ChannelSymmetryWitness::SYMMETRIC;
            }
        }
        let mut planes = finite_planes(4, 8, block);
        let mut oracle_planes = finite_planes(4, 8, block);
        chain.run(&mut planes, 8, block * 8).expect("render");
        oracle
            .run(&mut oracle_planes, 8, block * 8)
            .expect("render");
        for lane in 0..4 {
            for frame in 0..8 {
                observed.push((
                    planes.left[lane][frame].to_bits(),
                    planes.right[lane][frame].to_bits(),
                ));
                expected.push((
                    oracle_planes.left[lane][frame].to_bits(),
                    oracle_planes.right[lane][frame].to_bits(),
                ));
            }
        }
    }

    assert!(
        !stage.core().states_agree(),
        "the episode must actually have separated the channels, or the decline proves nothing"
    );
    assert!(!chain.collapse_channels_agree());
    assert_eq!(
        chain.collapse_transitions(),
        [1, 0, 0],
        "one disengage, no re-engage, and no proof: the query was asked on the two eligible \
         blocks after the words re-agreed and refused both times"
    );
    assert_eq!(
        stage.core().agreement_queries,
        2,
        "the recovery window is exactly the eligible blocks the invariant declined"
    );
    assert_eq!(
        chain.collapses(),
        4,
        "only the four blocks before the episode collapsed"
    );
    assert_eq!(
        observed, expected,
        "a chain that declined to re-engage renders the never-collapsed bits"
    );
}

/// An earned agreement proof brings back a chain the witness alone could not.
///
/// The same de-symmetrised episode as above, followed by a window of silence. The kernel has a
/// rest state -- the flush floor -- so both channels reach exactly `+0.0` and
/// `BankStage::channels_agree` can say so from the state words rather than from a theory about
/// where the recursion settles. That is the shipped effects' `silent_fixed_point` argument at
/// one word instead of four rings, and it is the only route back that does not go through the
/// disengage copy.
///
/// # Red mutation
///
/// Make `Recursive::channels_agree` return `true` unconditionally and the *previous* test
/// fails on its output comparison. Make it return `false` unconditionally and this one loses
/// its re-engage and its proof.
#[test]
fn an_earned_agreement_proof_re_engages_a_chain_the_witness_could_not() {
    let stage = Recursive::new([0.5, 0.5]);
    let oracle_stage = Recursive::new([0.5, 0.5]);
    let mut chain = recursive_chain(&stage);
    let mut oracle = recursive_chain(&oracle_stage);
    oracle.force_mono_collapse_off(true);

    let mut observed = Vec::new();
    let mut expected = Vec::new();
    for block in 0..40_u64 {
        for handle in [&stage, &oracle_stage] {
            let mut core = handle.core();
            if block == 4 {
                core.coefficient[1] = 0.25;
                core.witness =
                    ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::DESIGNED);
            }
            if block == 8 {
                core.coefficient[1] = core.coefficient[0];
                core.witness = ChannelSymmetryWitness::SYMMETRIC;
            }
        }
        // Silence from block 8 on: the recursion decays into the flush floor and both channels
        // land on the one rest state.
        let silent = block >= 8;
        let mut planes = if silent {
            silent_planes(4, 8)
        } else {
            finite_planes(4, 8, block)
        };
        let mut oracle_planes = if silent {
            silent_planes(4, 8)
        } else {
            finite_planes(4, 8, block)
        };
        chain.run(&mut planes, 8, block * 8).expect("render");
        oracle
            .run(&mut oracle_planes, 8, block * 8)
            .expect("render");
        for lane in 0..4 {
            for frame in 0..8 {
                observed.push((
                    planes.left[lane][frame].to_bits(),
                    planes.right[lane][frame].to_bits(),
                ));
                expected.push((
                    oracle_planes.left[lane][frame].to_bits(),
                    oracle_planes.right[lane][frame].to_bits(),
                ));
            }
        }
    }

    let transitions = chain.collapse_transitions();
    assert_eq!(transitions[0], 1, "one disengage");
    assert_eq!(transitions[1], 1, "one re-engage, through the proof");
    assert_eq!(
        transitions[2], 1,
        "and exactly one proof: it is latched by the invariant"
    );
    assert!(chain.collapse_channels_agree());
    assert!(
        chain.collapses() > 4,
        "the chain collapsed again after the proof"
    );
    assert_eq!(
        observed, expected,
        "a chain that re-engaged on a proven agreement renders the never-collapsed bits"
    );
}
