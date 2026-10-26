//! The state payload (#1278): every word a continuation reads, rings whole.
//!
//! The common section holds the control state, the sixteen line taps, the gain ramps, the filter
//! states and the sixteen line rings. Each channel section holds that channel's diffuser cursors
//! and pipeline registers, its pre-delay ring and its four diffuser rings, so a left-only input
//! changes no word of the right section until it has crossed the loop (the dual-mono isolation
//! probe). A ring word the history counter makes unreadable is written as `+0.0`, so the payload is
//! canonical. A restore validates every section as a pure function of the bytes, then commits.

use effect_contract::{
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
};
use effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::{
    RAMP_WORDS, ramp_path_inside, read_f32, read_ramp, read_u32, write_f32, write_ramp, write_u32,
};

use crate::{
    COMMON_HEADER_WORDS, DIFFUSERS, GainRamps, LANE_HEADER_WORDS, LINES, PARAMETER_COUNT,
    PARAMETER_SPECS, PRE_DELAY, PreparedReverb, RAMP_SAMPLES, REVERB_DESCRIPTOR, SCALAR_RAMPS,
    SIZE, TRANSITION_SAMPLES, damping_coefficient, damping_cutoff_max_hz, diffusion_coefficient,
    line_lengths, low_cut_coefficient, pre_delay_samples, tpt_coefficient,
};

const C_HISTORY: usize = 0;
const C_LINE_FADE: usize = 1;
const C_PRE_FADE: usize = 2;
const C_PRE_ACTIVE: usize = 3;
const C_PRE_TRANSITION: usize = 4;
const C_PRE_PENDING: usize = 5;
const C_PRE_CURSOR: usize = 6;
const C_GAIN_REMAINING: usize = 7;
const C_TARGETS: usize = 8;
const C_RAMPS: usize = C_TARGETS + PARAMETER_COUNT;
const C_LINE_ACTIVE: usize = C_RAMPS + SCALAR_RAMPS * RAMP_WORDS;
const C_LINE_TRANSITION: usize = C_LINE_ACTIVE + LINES;
const C_LINE_PENDING: usize = C_LINE_TRANSITION + LINES;
const C_LINE_CURSOR: usize = C_LINE_PENDING + LINES;
const C_GAIN_CURRENT: usize = C_LINE_CURSOR + LINES;
const C_GAIN_STEP: usize = C_GAIN_CURRENT + LINES;
const C_GAIN_TARGET: usize = C_GAIN_STEP + LINES;
const C_LP: usize = C_GAIN_TARGET + LINES;
const C_HP: usize = C_LP + LINES;
const _: () = assert!(C_HP + LINES == COMMON_HEADER_WORDS);

const L_CURSOR: usize = 0;
const L_PIPE: usize = L_CURSOR + DIFFUSERS / 2;
const _: () = assert!(L_PIPE + DIFFUSERS / 2 == LANE_HEADER_WORDS);

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

fn validate_lengths(
    common: usize,
    left: usize,
    right: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common != sizes.common_bytes as usize
        || left != sizes.left_bytes as usize
        || right != sizes.right_bytes as usize
    {
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

/// `true` if ring cell `index` holds history a tap may read: written at most `history` samples
/// ago (the delay's rule; the cell at the cursor is the oldest, `len` samples old).
fn valid_cell(cursor: u32, history: u32, len: usize, index: usize) -> bool {
    if history as usize >= len {
        return true;
    }
    let cursor = cursor as usize;
    let age = if index < cursor {
        cursor - index
    } else {
        cursor + len - index
    };
    age <= history as usize
}

fn normal_or_zero(value: f32) -> bool {
    value.is_normal() || value == 0.0
}

/// Writes one ring at word `word`, its unreadable cells as `+0.0`.
fn write_ring(bytes: &mut [u8], word: usize, ring: &[f32], cursor: u32, history: u32) {
    for (index, value) in ring.iter().copied().enumerate() {
        let value = if valid_cell(cursor, history, ring.len(), index) {
            value
        } else {
            0.0
        };
        write_f32(bytes, word + index, value);
    }
}

/// Validates one ring at word `word` without writing it.
fn validate_ring(
    bytes: &[u8],
    word: usize,
    len: usize,
    cursor: u32,
    history: u32,
) -> Result<(), StatePayloadError> {
    for index in 0..len {
        let value = read_f32(bytes, word + index);
        if !normal_or_zero(value)
            || (!valid_cell(cursor, history, len, index) && value.to_bits() != 0)
        {
            return Err(state_error("effect.state.history"));
        }
    }
    Ok(())
}

/// Reads and validates one carried ramp (the delay's rule, #1278 D2a, #1409, #1411).
fn read_carried_ramp(
    bytes: &[u8],
    word: usize,
    spec: &ParameterSpec,
) -> Result<LinearRamp, StatePayloadError> {
    let read = read_ramp(bytes, word);
    if !parameter_value_valid(spec, read.current)
        || !parameter_value_valid(spec, read.target)
        || !ramp_path_inside(read, (spec.minimum, spec.maximum), RAMP_SAMPLES)
        || (read.remaining == 0 && read.current != read.target)
    {
        return Err(state_error("effect.state.parameter"));
    }
    Ok(LinearRamp {
        current: normalize_zero(read.current),
        target: normalize_zero(read.target),
        ..read
    })
}

/// A validated common header, held until every section has been accepted.
struct CommonHeader {
    history: u32,
    line_fade: u32,
    pre_fade: u32,
    pre_active: u32,
    pre_transition: u32,
    pre_pending: u32,
    pre_cursor: u32,
    targets: [f32; PARAMETER_COUNT],
    ramps: [LinearRamp; SCALAR_RAMPS],
    line_active: [u32; LINES],
    line_transition: [u32; LINES],
    line_pending: [u32; LINES],
    line_cursor: [u32; LINES],
    gains: GainRamps,
    lp: [f32; LINES],
    hp: [f32; LINES],
}

fn read_words<const N: usize>(bytes: &[u8], word: usize) -> [u32; N] {
    core::array::from_fn(|i| read_u32(bytes, word + i))
}

fn read_floats<const N: usize>(bytes: &[u8], word: usize) -> [f32; N] {
    core::array::from_fn(|i| read_f32(bytes, word + i))
}

impl PreparedReverb {
    pub(crate) fn snapshot(&self, output: StatePayloadOutput<'_>) -> Result<(), StatePayloadError> {
        validate_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.state_sizes,
        )?;
        let g = &self.geometry;
        let common = output.common;
        write_u32(common, C_HISTORY, self.history);
        write_u32(common, C_LINE_FADE, self.line_fade);
        write_u32(common, C_PRE_FADE, self.pre_fade);
        write_u32(common, C_PRE_ACTIVE, self.pre_active);
        write_u32(common, C_PRE_TRANSITION, self.pre_transition);
        write_u32(common, C_PRE_PENDING, self.pre_pending);
        write_u32(common, C_PRE_CURSOR, self.pre_cursor);
        write_u32(common, C_GAIN_REMAINING, self.gains.remaining);
        for i in 0..PARAMETER_COUNT {
            write_f32(common, C_TARGETS + i, self.targets[i]);
        }
        for (i, ramp) in self.ramps.iter().enumerate() {
            write_ramp(common, C_RAMPS + i * RAMP_WORDS, *ramp);
        }
        for i in 0..LINES {
            write_u32(common, C_LINE_ACTIVE + i, self.line_active[i]);
            write_u32(common, C_LINE_TRANSITION + i, self.line_transition[i]);
            write_u32(common, C_LINE_PENDING + i, self.line_pending[i]);
            write_u32(common, C_LINE_CURSOR + i, self.line_cursor[i]);
            write_f32(common, C_GAIN_CURRENT + i, self.gains.current[i]);
            write_f32(common, C_GAIN_STEP + i, self.gains.step[i]);
            write_f32(common, C_GAIN_TARGET + i, self.gains.target[i]);
            write_f32(common, C_LP + i, self.lp[i]);
            write_f32(common, C_HP + i, self.hp[i]);
        }
        for line in 0..LINES {
            let len = g.line_len[line] as usize;
            write_ring(
                common,
                COMMON_HEADER_WORDS + g.line_offset[line],
                &self.lines[g.line_offset[line]..][..len],
                self.line_cursor[line],
                self.history,
            );
        }
        let pre_len = g.pre_len as usize;
        for (channel, bytes) in [output.left, output.right].into_iter().enumerate() {
            let lanes = channel * DIFFUSERS / 2..(channel + 1) * DIFFUSERS / 2;
            for (i, lane) in lanes.clone().enumerate() {
                write_u32(bytes, L_CURSOR + i, self.diffuser_cursor[lane]);
                write_f32(bytes, L_PIPE + i, self.pipe[lane]);
            }
            write_ring(
                bytes,
                LANE_HEADER_WORDS,
                &self.pre[channel * pre_len..][..pre_len],
                self.pre_cursor,
                self.history,
            );
            let mut word = LANE_HEADER_WORDS + pre_len;
            for lane in lanes {
                let len = g.diffuser_len[lane] as usize;
                write_ring(
                    bytes,
                    word,
                    &self.diffusers[g.diffuser_offset[lane]..][..len],
                    self.diffuser_cursor[lane],
                    self.history,
                );
                word += len;
            }
            for padding in word..bytes.len() / 4 {
                write_f32(bytes, padding, 0.0);
            }
        }
        Ok(())
    }

    /// Validates the common header as a pure function of the bytes.
    fn read_common_header(&self, bytes: &[u8]) -> Result<CommonHeader, StatePayloadError> {
        let g = &self.geometry;
        let rate = self.sample_rate;
        let lane_error = || state_error("effect.state.lane");
        let header = CommonHeader {
            history: read_u32(bytes, C_HISTORY),
            line_fade: read_u32(bytes, C_LINE_FADE),
            pre_fade: read_u32(bytes, C_PRE_FADE),
            pre_active: read_u32(bytes, C_PRE_ACTIVE),
            pre_transition: read_u32(bytes, C_PRE_TRANSITION),
            pre_pending: read_u32(bytes, C_PRE_PENDING),
            pre_cursor: read_u32(bytes, C_PRE_CURSOR),
            targets: read_floats(bytes, C_TARGETS),
            ramps: [LinearRamp::fixed(0.0); SCALAR_RAMPS],
            line_active: read_words(bytes, C_LINE_ACTIVE),
            line_transition: read_words(bytes, C_LINE_TRANSITION),
            line_pending: read_words(bytes, C_LINE_PENDING),
            line_cursor: read_words(bytes, C_LINE_CURSOR),
            gains: GainRamps {
                current: read_floats(bytes, C_GAIN_CURRENT),
                target: read_floats(bytes, C_GAIN_TARGET),
                step: read_floats(bytes, C_GAIN_STEP),
                remaining: read_u32(bytes, C_GAIN_REMAINING),
            },
            lp: read_floats(bytes, C_LP),
            hp: read_floats(bytes, C_HP),
        };
        for (index, target) in header.targets.iter().enumerate() {
            if !parameter_value_valid(&PARAMETER_SPECS[index], *target) || is_negative_zero(*target)
            {
                return Err(state_error("effect.state.parameter"));
            }
        }
        let pre_ok = |d: u32| d <= g.pre_max;
        if header.history > g.history_cap
            || header.line_fade > TRANSITION_SAMPLES
            || header.pre_fade > TRANSITION_SAMPLES
            || header.pre_cursor >= g.pre_len
            || !pre_ok(header.pre_active)
            || !pre_ok(header.pre_transition)
            || (header.pre_fade == 0 && header.pre_active != header.pre_transition)
            || pre_delay_samples(header.targets[PRE_DELAY], rate) != Some(header.pre_pending)
            || line_lengths(header.targets[SIZE], rate, &self.sieve) != Some(header.line_pending)
            || (header.line_fade == 0 && header.line_active != header.line_transition)
        {
            return Err(lane_error());
        }
        for line in 0..LINES {
            let in_range = |d: u32| (self.min_lengths[line]..=self.max_lengths[line]).contains(&d);
            if !in_range(header.line_active[line])
                || !in_range(header.line_transition[line])
                || header.line_cursor[line] >= g.line_len[line]
                || !normal_or_zero(header.lp[line])
                || !normal_or_zero(header.hp[line])
            {
                return Err(lane_error());
            }
        }
        let gains = &header.gains;
        for line in 0..LINES {
            let ramp = LinearRamp {
                current: gains.current[line],
                target: gains.target[line],
                step: gains.step[line],
                remaining: gains.remaining,
            };
            if !ramp_path_inside(ramp, (0.0, 0.25), RAMP_SAMPLES)
                || !(gains.current[line] > 0.0 && gains.target[line] > 0.0)
                || (gains.remaining == 0
                    && (gains.current[line] != gains.target[line] || gains.step[line] != 0.0))
            {
                return Err(state_error("effect.state.parameter"));
            }
        }
        // The coefficient ramps hold designed words, so their domains are the designs' ranges.
        let damping_max = tpt_coefficient(damping_cutoff_max_hz(rate), rate);
        let specs = [
            ParameterSpec::continuous(damping_coefficient(1_000.0, rate), damping_max, 0.0),
            ParameterSpec::continuous(
                low_cut_coefficient(20.0, rate),
                low_cut_coefficient(500.0, rate),
                0.0,
            ),
            ParameterSpec::continuous(0.0, diffusion_coefficient(1.0), 0.0),
            PARAMETER_SPECS[crate::WIDTH],
            PARAMETER_SPECS[crate::MIX],
        ];
        let mut header = header;
        for (index, spec) in specs.iter().enumerate() {
            header.ramps[index] = read_carried_ramp(bytes, C_RAMPS + index * RAMP_WORDS, spec)?;
        }
        Ok(header)
    }

    pub(crate) fn restore(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != REVERB_DESCRIPTOR.state_layout_version {
            return Err(state_error("effect.state.version"));
        }
        validate_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.state_sizes,
        )?;
        let g = self.geometry;
        let header = self.read_common_header(input.common)?;
        for line in 0..LINES {
            validate_ring(
                input.common,
                COMMON_HEADER_WORDS + g.line_offset[line],
                g.line_len[line] as usize,
                header.line_cursor[line],
                header.history,
            )?;
        }
        let pre_len = g.pre_len as usize;
        let mut cursors = [0_u32; DIFFUSERS];
        let mut pipe = [0.0_f32; DIFFUSERS];
        for (channel, bytes) in [input.left, input.right].into_iter().enumerate() {
            validate_ring(
                bytes,
                LANE_HEADER_WORDS,
                pre_len,
                header.pre_cursor,
                header.history,
            )?;
            let mut word = LANE_HEADER_WORDS + pre_len;
            for i in 0..DIFFUSERS / 2 {
                let lane = channel * DIFFUSERS / 2 + i;
                let len = g.diffuser_len[lane];
                cursors[lane] = read_u32(bytes, L_CURSOR + i);
                pipe[lane] = read_f32(bytes, L_PIPE + i);
                if cursors[lane] >= len || !normal_or_zero(pipe[lane]) {
                    return Err(state_error("effect.state.lane"));
                }
                validate_ring(bytes, word, len as usize, cursors[lane], header.history)?;
                word += len as usize;
            }
            if (word..bytes.len() / 4).any(|padding| read_u32(bytes, padding) != 0) {
                return Err(state_error("effect.state.lane"));
            }
        }

        // Commit: every section has been accepted.
        self.history = header.history;
        self.line_fade = header.line_fade;
        self.pre_fade = header.pre_fade;
        self.pre_active = header.pre_active;
        self.pre_transition = header.pre_transition;
        self.pre_pending = header.pre_pending;
        self.pre_cursor = header.pre_cursor;
        self.targets = header.targets;
        self.ramps = header.ramps;
        self.line_active = header.line_active;
        self.line_transition = header.line_transition;
        self.line_pending = header.line_pending;
        self.line_cursor = header.line_cursor;
        self.line_queued = header.line_pending != header.line_active;
        self.gains = header.gains;
        self.lp = header.lp;
        self.hp = header.hp;
        self.diffuser_cursor = cursors;
        self.pipe = pipe;
        for line in 0..LINES {
            let base = g.line_offset[line];
            for index in 0..g.line_len[line] as usize {
                self.lines[base + index] =
                    read_f32(input.common, COMMON_HEADER_WORDS + base + index);
            }
        }
        for (channel, bytes) in [input.left, input.right].into_iter().enumerate() {
            for index in 0..pre_len {
                self.pre[channel * pre_len + index] = read_f32(bytes, LANE_HEADER_WORDS + index);
            }
            let mut word = LANE_HEADER_WORDS + pre_len;
            for i in 0..DIFFUSERS / 2 {
                let lane = channel * DIFFUSERS / 2 + i;
                let base = g.diffuser_offset[lane];
                for index in 0..g.diffuser_len[lane] as usize {
                    self.diffusers[base + index] = read_f32(bytes, word + index);
                }
                word += g.diffuser_len[lane] as usize;
            }
        }
        Ok(())
    }
}
