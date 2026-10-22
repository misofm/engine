//! Issue #1745 gate 1: the EQ bank's per-lane rest claim, at every vector width the build has.
//!
//! A crate-internal module so that it binds [`PreparedParametricEq`] at `Simd4` and `Simd8` on an
//! AVX2 host (`lane::each_vector_lane!`), reads every state word of a lane, and forks a bank with
//! its bank-wide claim withdrawn. The shared driver is `conformance::rest_probe`.

use super::ramping_elision::{Rng, automatable, draw, request};
use super::*;
use conformance::rest_probe::{self, Probe, Reach, Schedule};
use effect_contract::{EffectTargetRequest, NativeEffectTargetPreparation};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// One EQ bank under the driver.
struct EqProbe<L: Lane, const W: usize> {
    eq: PreparedParametricEq<L, W>,
    /// Frames per block. A block shorter than the 64-sample ramp leaves a ramp in flight at a
    /// block boundary; a quantum-long one never does.
    frames: usize,
    /// Every lane's current prepare-time values, so a retune starts from where the lane is.
    values: Vec<Vec<InitialParameterValue>>,
    /// Render through the collapsed body (`process_bank_mono`) instead of the dual one.
    mono: bool,
    /// Retune a random member between blocks. Off on a fork, whose block must be the claim's.
    retune: Option<Rng>,
    /// A per-lane input gain. The driver's levels never reach the band near `1e-15` where the
    /// per-word flush clears one integrator of a low section and keeps the other; a gain of
    /// `1e-6` or `1e-8` puts its `1e-9` level there. A gain maps `+0.0` to `+0.0`, so a silent
    /// block stays silent.
    gain: Vec<f32>,
}

/// A field-by-field copy of a channel: every word, derived flags included.
fn copy_channel<L: Lane, const W: usize>(channel: &Channel<L, W>) -> Channel<L, W> {
    Channel {
        sections: channel.sections,
        remaining: channel.remaining,
        targets: channel.targets,
        identity: channel.identity,
        dry: channel.dry,
        silence: channel.silence,
    }
}

impl<L: Lane, const W: usize> EqProbe<L, W> {
    /// Retunes one random member's automatable parameter, about one block in ten: a ramp in
    /// flight on that lane, and its neighbours through the ramped kernel with a `+0.0` step.
    fn maybe_retune(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if !rng.chance(0.1) {
            return;
        }
        let members: Vec<usize> = (0..W).filter(|&lane| self.eq.active[lane]).collect();
        let lane = members[rng.below(members.len())];
        let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters.len();
        let index = loop {
            let index = rng.below(parameters);
            if automatable(index) {
                break index;
            }
        };
        let mut candidate = self.values[lane].clone();
        let mut changed = vec![false; parameters * 2];
        for side in 0..2 {
            candidate[index * 2 + side].value = draw(rng, index);
            changed[index * 2 + side] = true;
        }
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_SECTION_COUNT * 2];
        let Ok(count) = ParametricEqFactory.prepare_targets(
            EffectTargetRequest {
                sample_rate: self.eq.sample_rate,
                values: &candidate,
                changed: &changed,
            },
            &mut targets,
        ) else {
            return;
        };
        if targets[..count]
            .iter()
            .all(|target| self.eq.apply_prepared_target_lane(lane, target).is_ok())
        {
            self.values[lane] = candidate;
        }
    }
}

impl<L: Lane, const W: usize> Probe for EqProbe<L, W> {
    fn lanes(&self) -> usize {
        W
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.maybe_retune();
        for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let gain = self.gain[index % W];
            *l *= gain;
            *r *= gain;
        }
        let width = BankWidth::for_lanes(W).expect("a bank width");
        let offsets = vec![0_u32; W + 1];
        let block = EffectBankProcessBlock::new(
            left,
            right,
            None,
            self.frames as u32,
            width,
            0,
            &[],
            &offsets,
            self.eq.quantum(),
        )
        .expect("a bank block");
        if self.mono {
            self.eq.process_bank_mono(block);
            // The seam's copy: a collapsed track's right plane is its left one.
            right.copy_from_slice(left);
        } else {
            self.eq.process_bank(block);
        }
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.eq)
    }

    /// The lane's per-track state payload, both channels, minus the silence counter (the one
    /// clock), plus the ramp target words the payload recomputes rather than carries.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        for channel in [&self.eq.left, &self.eq.right] {
            let mut words = [0_u32; STATE_LANE_WORDS];
            channel.snapshot_track(lane, &mut words);
            out.extend(
                words
                    .iter()
                    .enumerate()
                    .filter(|&(word, _)| word != STATE_SILENCE_WORD)
                    .map(|(_, bits)| *bits),
            );
            for section in &channel.sections {
                for index in 0..EQ_COEFFICIENT_WORDS {
                    out.push(lane_get(coef_word(&section.target, index), lane).to_bits());
                }
            }
        }
    }

    fn fork(&self) -> Self {
        let eq = &self.eq;
        Self {
            eq: PreparedParametricEq {
                sample_rate: eq.sample_rate,
                bypass: eq.bypass,
                initial: eq.initial,
                initial_words: eq.initial_words,
                left: copy_channel(&eq.left),
                right: copy_channel(&eq.right),
                active: eq.active,
                // Withdrawn: the check's block runs the whole kernel.
                silent_fixed_point: false,
                rest: eq.rest.clone(),
            },
            frames: self.frames,
            values: self.values.clone(),
            mono: self.mono,
            retune: None,
            gain: self.gain.clone(),
        }
    }
}

/// One member's prepare-time values; `symmetric` gives both channels the same ones.
fn configuration(rng: &mut Rng, symmetric: bool) -> Vec<InitialParameterValue> {
    let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters.len();
    let mut values = Vec::with_capacity(parameters * 2);
    for index in 0..parameters {
        let left = draw(rng, index);
        let right = if symmetric || (index < 24 && index % 6 < 2) {
            left
        } else {
            draw(rng, index)
        };
        for (channel, value) in [
            (ParameterChannel::Left, left),
            (ParameterChannel::Right, right),
        ] {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    values
}

/// A bank of random members (the rest padded with clones), at a random launch rate.
fn probe<L: Lane, const W: usize>(rng: &mut Rng, mono: bool, frames: usize) -> EqProbe<L, W> {
    loop {
        let rate = LAUNCH_RATES[rng.below(LAUNCH_RATES.len())];
        let members = 1 + rng.below(W);
        let configured: Vec<Vec<InitialParameterValue>> = (0..members)
            .map(|_| {
                let symmetric = mono || rng.chance(0.3);
                configuration(rng, symmetric)
            })
            .collect();
        let values: Vec<Vec<InitialParameterValue>> = (0..W)
            .map(|lane| configured[lane.min(members - 1)].clone())
            .collect();
        let requests: Vec<_> = values.iter().map(|lane| request(lane, rate)).collect();
        let mask: Vec<bool> = (0..W).map(|lane| lane < members).collect();
        let Ok(metadata) = expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, requests[0])
        else {
            continue;
        };
        let Ok(eq) = prepare_width::<L, W>(metadata, &requests, &mask) else {
            continue;
        };
        let retune = (!mono).then(|| Rng(rng.next() | 1));
        let gain = (0..W)
            .map(|_| [1.0, 1.0, 1.0e-6, 1.0e-8][rng.below(4)])
            .collect();
        return EqProbe {
            eq,
            frames,
            values,
            mono,
            retune,
            gain,
        };
    }
}

fn claim_holds<L: Lane, const W: usize>(label: &str) {
    // The same span of time at both block lengths: about 0.6 s at 48 kHz, the silence runs long
    // enough for the joint flush to arm (85 ms) and for most designs to come to rest after it.
    let schedule = |frames: usize| Schedule {
        blocks: 30_720 / frames,
        longest_signal: 768 / frames,
        longest_silence: 11_520 / frames,
    };
    let mut reach = Reach::default();
    for seed in 0..4_u64 {
        let mut rng = Rng(0x1745_0001 + seed * 0x9e37);
        let mono = seed == 3;
        let frames = if seed % 2 == 0 { 32 } else { 128 };
        let mut bank = probe::<L, W>(&mut rng, mono, frames);
        reach.add(rest_probe::check(
            &mut bank,
            0x1745 + seed,
            schedule(frames),
        ));
    }
    eprintln!("{label}: {reach:?}");
    reach.assert_reached(label);
}

/// Gate 1: at every block after which the EQ bank sets a lane's rest bit, one more block of
/// `+0.0` through the whole kernel leaves that lane's state bits (the payload minus the silence
/// clock, plus the ramp targets) where they were and writes `+0.0` on both planes. Random members
/// and padded clones, random rates, retunes between blocks (ramps in flight, and neighbours
/// through the ramped kernel), the dual body and, on one seed per width, the collapsed one.
#[test]
fn a_lane_the_eq_bank_calls_at_rest_stays_there_on_a_silent_block() {
    lane::each_vector_lane!(|L, W| claim_holds::<L, W>(&width_label(W)));
}
