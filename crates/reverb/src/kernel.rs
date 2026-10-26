//! The per-sample kernel (R1-R12 of `dsp-research/reverb.md`), one body generic over `Lane`.
//!
//! # Loop shape
//!
//! A plain per-sample loop. Each line has its own length, so the delay's chunked copy-out (which
//! exists to hoist two lanes' tap windows out of the frame loop) would need sixteen windows per
//! chunk and buys nothing here: the ring reads and writes are scalar per line in either shape. A
//! wrap is one compare-and-subtract. The only per-sample branches are on control state (a
//! crossfade or a ramp in flight), never on a sample value.
//!
//! # The butterfly and the re-layout
//!
//! Position `p` of the 16-word slot vector set is lane `p % W` of vector `p / W`. A layout maps
//! each position bit `b` to the slot bit `tau[b]` it carries. Stage `k` pairs the slots that
//! differ in bit `k`: when bit `k` sits on a vector bit, that is a pair of whole vectors and the
//! stage is `W`-wide add/sub. When it sits on a lane bit, the kernel first stores the vectors to a
//! 16-word stack array and reloads them in a layout that puts every remaining stage bit on the
//! highest position bits. A re-layout only moves words, and every stage computes
//! `(w_j + w_m, w_j - w_m)` on the same two words at every width, so every `W` gives the same bits.
//! `W = 1` never re-lays out, `W = 4` once (before stage 1) and `W = 8` before each of stages 2, 1
//! and 0, as the note states. The schedule is a `const` per width.

use lane::kernels::ramp_toward;
use lane::{Lane, flush};

use crate::{
    DIFFUSERS, INPUT_GAIN, LINES, OUTPUT_LEFT, OUTPUT_RIGHT, PreparedReverb, RAMP_DAMPING,
    RAMP_DIFFUSION, RAMP_LOW_CUT, RAMP_MIX, RAMP_WIDTH, TRANSITION_SAMPLES, slot_of_line,
};

/// One butterfly stage of the schedule.
#[derive(Clone, Copy)]
struct Stage {
    /// Re-lay the vectors out (through `gather`) before this stage.
    relayout: bool,
    /// After a re-layout, new position `p` takes old position `gather[p]`.
    gather: [u8; LINES],
    /// The stage pairs vector `v` (bit clear) with `v + stride`.
    stride: usize,
}

/// The butterfly schedule of one lane width.
#[derive(Clone, Copy)]
pub(crate) struct Schedule {
    stages: [Stage; 4],
    /// After the last stage: the matrix output (line) index at each position.
    slot_of_position: [u8; LINES],
    /// After the last stage: the position of each matrix output (line) index.
    position_of_slot: [u8; LINES],
}

const fn slot_at(tau: &[u8; 4], position: usize) -> usize {
    let mut slot = 0;
    let mut b = 0;
    while b < 4 {
        slot |= ((position >> b) & 1) << tau[b];
        b += 1;
    }
    slot
}

const fn position_of(tau: &[u8; 4], slot: usize) -> usize {
    let mut position = 0;
    let mut b = 0;
    while b < 4 {
        position |= ((slot >> tau[b]) & 1) << b;
        b += 1;
    }
    position
}

/// The schedule at `width` lanes (1, 4 or 8).
pub(crate) const fn matrix_schedule(width: usize) -> Schedule {
    let lane_bits = width.trailing_zeros() as usize;
    assert!(width == 1 << lane_bits && lane_bits <= 3);
    let mut tau: [u8; 4] = [0, 1, 2, 3];
    let empty = Stage {
        relayout: false,
        gather: [0; LINES],
        stride: 0,
    };
    let mut stages = [empty; 4];
    let mut index = 0;
    while index < 4 {
        let k = 3 - index;
        let mut b = 0;
        while tau[b] as usize != k {
            b += 1;
        }
        let mut stage = empty;
        if b < lane_bits {
            // Remaining stage bits k, k-1, .., 0 go to position bits 3, 2, ..; the done bits to the
            // lowest position bits.
            let mut next: [u8; 4] = [0; 4];
            let mut r = 0;
            while r <= k {
                next[3 - r] = (k - r) as u8;
                r += 1;
            }
            let mut d = 0;
            while d < 3 - k {
                next[d] = (k + 1 + d) as u8;
                d += 1;
            }
            let mut p = 0;
            while p < LINES {
                stage.gather[p] = position_of(&tau, slot_at(&next, p)) as u8;
                p += 1;
            }
            stage.relayout = true;
            tau = next;
            b = 3;
        }
        stage.stride = 1 << (b - lane_bits);
        stages[index] = stage;
        index += 1;
    }
    let mut slot_of_position = [0; LINES];
    let mut position_of_slot = [0; LINES];
    let mut p = 0;
    while p < LINES {
        slot_of_position[p] = slot_at(&tau, p) as u8;
        position_of_slot[slot_at(&tau, p)] = p as u8;
        p += 1;
    }
    Schedule {
        stages,
        slot_of_position,
        position_of_slot,
    }
}

/// `i - len` when `i >= len`: the wrap of an index known to be below `2 len`.
#[inline(always)]
const fn wrap(index: usize, len: usize) -> usize {
    if index >= len { index - len } else { index }
}

/// One output sample: `dry + mix * (wet - dry)`, the dry and wet identities and bypass as
/// bitwise selects (the delay's `mix_sample`).
#[inline(always)]
fn mix_sample(dry: f32, wet: f32, mix: f32, bypass: bool) -> f32 {
    let blended = mix.fma(wet - dry, dry);
    let wet_or_blend = <f32 as Lane>::select(<f32 as Lane>::eq(mix, 1.0), wet, blended);
    let dry_mask = <f32 as Lane>::mask_or(
        <f32 as Lane>::eq(mix, 0.0),
        <f32 as Lane>::eq(0.0, if bypass { 0.0 } else { 1.0 }),
    );
    <f32 as Lane>::select(dry_mask, dry, wet_or_blend)
}

/// A crossfade update's weight `j / 128`, exact, for `remaining = 129 - j`.
#[inline(always)]
fn fade_alpha(remaining: u32) -> f32 {
    (TRANSITION_SAMPLES + 1 - remaining) as f32 * (1.0 / TRANSITION_SAMPLES as f32)
}

/// One tap blend: the new word on the last update, otherwise `old + alpha (new - old)`.
#[inline(always)]
fn blend(old: f32, new: f32, remaining: u32) -> f32 {
    if remaining == 1 {
        new
    } else {
        fade_alpha(remaining).fma(new - old, old)
    }
}

impl PreparedReverb {
    /// Renders one block in place. `FULL` is "every ring word is valid history", which holds for
    /// the whole block when it holds at its start (the counter only grows).
    ///
    /// Out of line on purpose: the symbol then carries the lane type (`4wide6f32x4` in the
    /// browser build), which is how `scripts/check-web-audioworklet-callgraph.py` finds and
    /// rosters the reverb's render (#2114 S4). Inlined, the whole body sat in
    /// `PreparedReverb::process`, whose name carries no lane type, so the gate never saw it.
    #[inline(never)]
    pub(crate) fn render<L: Lane, const FULL: bool>(
        &mut self,
        io_left: &mut [f32],
        io_right: &mut [f32],
    ) {
        let schedule = const { matrix_schedule(L::WIDTH) };
        let w = L::WIDTH;
        let vectors = LINES / w;
        let diffuser_vectors = DIFFUSERS / w;
        let geometry = self.geometry;
        let pre_len = geometry.pre_len as usize;
        let bypass = self.bypass;

        // Recursive state into vectors, slot order.
        let mut lp = [L::zero(); LINES];
        let mut hp = [L::zero(); LINES];
        for v in 0..vectors {
            lp[v] = L::load(&self.lp[v * w..]);
            hp[v] = L::load(&self.hp[v * w..]);
        }
        // Gains in the butterfly's final layout, and the per-position injection side.
        let mut gain_words = [[0.0_f32; LINES]; 3];
        let mut side_words = [0.0_f32; LINES];
        for p in 0..LINES {
            let line = schedule.slot_of_position[p] as usize;
            gain_words[0][p] = self.gains.current[line];
            gain_words[1][p] = self.gains.step[line];
            gain_words[2][p] = self.gains.target[line];
            side_words[p] = line as f32;
        }
        let mut gain = [L::zero(); LINES];
        let mut gain_step = [L::zero(); LINES];
        let mut gain_target = [L::zero(); LINES];
        let mut from_left = [L::zero().eq(L::zero()); LINES];
        for v in 0..vectors {
            gain[v] = L::load(&gain_words[0][v * w..]);
            gain_step[v] = L::load(&gain_words[1][v * w..]);
            gain_target[v] = L::load(&gain_words[2][v * w..]);
            from_left[v] = L::load(&side_words[v * w..]).lt(L::splat((LINES / 2) as f32));
        }
        let mut gain_remaining = self.gains.remaining;

        let mut history = self.history;
        let mut slots = [0.0_f32; LINES];
        let mut scratch = [0.0_f32; LINES];
        let mut gathered = [0.0_f32; LINES];
        let mut x = [L::zero(); LINES];

        for frame in 0..io_left.len() {
            // Queued crossfades start on the first sample after the running one ends.
            if self.pre_fade == 0 && self.pre_pending != self.pre_active {
                self.pre_transition = self.pre_pending;
                self.pre_fade = TRANSITION_SAMPLES;
            }
            if self.line_fade == 0 && self.line_queued {
                self.line_transition = self.line_pending;
                self.line_fade = TRANSITION_SAMPLES;
                self.line_queued = false;
            }

            // R12: every ramped word steps once, then this sample uses it.
            let damping = L::splat(self.ramps[RAMP_DAMPING].next_value());
            let low_cut = L::splat(self.ramps[RAMP_LOW_CUT].next_value());
            let diffusion = L::splat(self.ramps[RAMP_DIFFUSION].next_value());
            let width = self.ramps[RAMP_WIDTH].next_value();
            let mix = self.ramps[RAMP_MIX].next_value();
            if gain_remaining != 0 {
                for v in 0..vectors {
                    gain[v] = if gain_remaining == 1 {
                        gain_target[v]
                    } else {
                        ramp_toward(gain[v], gain_step[v], gain_target[v])
                    };
                }
                gain_remaining -= 1;
            }

            // R1: pre-delay, write then read (`P = 0` reads this sample).
            let dry_left = io_left[frame];
            let dry_right = io_right[frame];
            let cursor = self.pre_cursor as usize;
            self.pre[cursor] = dry_left;
            self.pre[pre_len + cursor] = dry_right;
            let pre_read = |pre: &[f32], d: u32| -> (f32, f32) {
                if FULL || d <= history {
                    let i = wrap(cursor + pre_len - d as usize, pre_len);
                    (pre[i], pre[pre_len + i])
                } else {
                    (0.0, 0.0)
                }
            };
            let (old_left, old_right) = pre_read(&self.pre, self.pre_active);
            let (pre_left, pre_right) = if self.pre_fade == 0 {
                (old_left, old_right)
            } else {
                let (new_left, new_right) = pre_read(&self.pre, self.pre_transition);
                (
                    blend(old_left, new_left, self.pre_fade),
                    blend(old_right, new_right, self.pre_fade),
                )
            };

            // R2: eight pipelined allpasses, one group of vector operations.
            let mut inputs = [0.0_f32; DIFFUSERS];
            let mut delayed = [0.0_f32; DIFFUSERS];
            inputs[0] = pre_left;
            inputs[1..4].copy_from_slice(&self.pipe[0..3]);
            inputs[4] = pre_right;
            inputs[5..8].copy_from_slice(&self.pipe[4..7]);
            for (lane, word) in delayed.iter_mut().enumerate() {
                let len = geometry.diffuser_len[lane];
                *word = if FULL || len <= history {
                    self.diffusers
                        [geometry.diffuser_offset[lane] + self.diffuser_cursor[lane] as usize]
                } else {
                    0.0
                };
            }
            let mut written = [0.0_f32; DIFFUSERS];
            for v in 0..diffuser_vectors {
                let a = L::load(&inputs[v * w..]);
                let d = L::load(&delayed[v * w..]);
                let stored = diffusion.fma(d, a);
                let out = d.sub(diffusion.mul(stored));
                flush(stored).store(&mut written[v * w..]);
                out.store(&mut self.pipe[v * w..]);
            }
            for (lane, word) in written.iter().enumerate() {
                let cursor = self.diffuser_cursor[lane];
                self.diffusers[geometry.diffuser_offset[lane] + cursor as usize] = *word;
                let next = cursor + 1;
                self.diffuser_cursor[lane] = if next == geometry.diffuser_len[lane] {
                    0
                } else {
                    next
                };
            }
            let injected_left = INPUT_GAIN * self.pipe[3];
            let injected_right = INPUT_GAIN * self.pipe[7];

            // R3-R4: clear-on-read taps into slot order.
            for line in 0..LINES {
                let len = geometry.line_len[line] as usize;
                let base = geometry.line_offset[line];
                let cursor = self.line_cursor[line] as usize;
                let old_delay = self.line_active[line];
                let old_index = base + wrap(cursor + len - old_delay as usize, len);
                let old = if FULL || old_delay <= history {
                    self.lines[old_index]
                } else {
                    0.0
                };
                let tap = if self.line_fade == 0 {
                    self.lines[old_index] = 0.0;
                    old
                } else {
                    let new_delay = self.line_transition[line];
                    let new_index = base + wrap(cursor + len - new_delay as usize, len);
                    let new = if FULL || new_delay <= history {
                        self.lines[new_index]
                    } else {
                        0.0
                    };
                    self.lines[old_index] = 0.0;
                    self.lines[new_index] = 0.0;
                    blend(old, new, self.line_fade)
                };
                slots[slot_of_line(line)] = tap;
            }

            // R5-R6: the in-loop low-pass and high-pass, slot order.
            for v in 0..vectors {
                let s = L::load(&slots[v * w..]);
                let delta = s.sub(lp[v]);
                let h = damping.mul(delta);
                let l = h.add(lp[v]);
                lp[v] = flush(l.add(h));
                let delta2 = l.sub(hp[v]);
                let h2 = low_cut.mul(delta2);
                let m = h2.add(hp[v]);
                hp[v] = flush(m.add(h2));
                x[v] = l.sub(m);
            }

            // R7: the butterfly, stages 3, 2, 1, 0.
            for stage in &schedule.stages {
                if stage.relayout {
                    for v in 0..vectors {
                        x[v].store(&mut scratch[v * w..]);
                    }
                    for p in 0..LINES {
                        gathered[p] = scratch[stage.gather[p] as usize];
                    }
                    for v in 0..vectors {
                        x[v] = L::load(&gathered[v * w..]);
                    }
                }
                for v in 0..vectors {
                    if v & stage.stride == 0 {
                        let m = v + stage.stride;
                        let (a, b) = (x[v], x[m]);
                        x[v] = a.add(b);
                        x[m] = a.sub(b);
                    }
                }
            }

            // R8-R9: outputs, then the gain and injection on the line being written.
            for v in 0..vectors {
                x[v].store(&mut scratch[v * w..]);
            }
            let out_left = 0.25 * scratch[schedule.position_of_slot[OUTPUT_LEFT] as usize];
            let out_right = 0.25 * scratch[schedule.position_of_slot[OUTPUT_RIGHT] as usize];
            let q_left = L::splat(injected_left);
            let q_right = L::splat(injected_right);
            for v in 0..vectors {
                let q = L::select(from_left[v], q_left, q_right);
                flush(gain[v].fma(x[v], q)).store(&mut gathered[v * w..]);
            }
            for line in 0..LINES {
                let cursor = self.line_cursor[line];
                self.lines[geometry.line_offset[line] + cursor as usize] =
                    gathered[schedule.position_of_slot[line] as usize];
                let next = cursor + 1;
                self.line_cursor[line] = if next == geometry.line_len[line] {
                    0
                } else {
                    next
                };
            }

            // R10-R11: width, then mix.
            let mid = 0.5 * (out_left + out_right);
            let side = width * (0.5 * (out_left - out_right));
            io_left[frame] = mix_sample(dry_left, mid + side, mix, bypass);
            io_right[frame] = mix_sample(dry_right, mid - side, mix, bypass);

            // Advance the clocks.
            let next = self.pre_cursor + 1;
            self.pre_cursor = if next == geometry.pre_len { 0 } else { next };
            if self.pre_fade != 0 {
                self.pre_fade -= 1;
                if self.pre_fade == 0 {
                    self.pre_active = self.pre_transition;
                }
            }
            if self.line_fade != 0 {
                self.line_fade -= 1;
                if self.line_fade == 0 {
                    self.line_active = self.line_transition;
                    self.line_queued = self.line_pending != self.line_active;
                }
            }
            if !FULL {
                history = (history + 1).min(geometry.history_cap);
            }
        }

        self.history = history;
        for v in 0..vectors {
            lp[v].store(&mut self.lp[v * w..]);
            hp[v].store(&mut self.hp[v * w..]);
            gain[v].store(&mut gain_words[0][v * w..]);
        }
        for (word, line) in gain_words[0].iter().zip(schedule.slot_of_position) {
            self.gains.current[line as usize] = *word;
        }
        self.gains.remaining = gain_remaining;
        if gain_remaining == 0 {
            self.gains.step = [0.0; LINES];
        }
    }
}
