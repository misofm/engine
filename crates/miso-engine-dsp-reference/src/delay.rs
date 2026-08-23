//! Independent offline `f64` oracle for the fixed integer-time launch delay.
//!
//! The oracle owns its own circular rings, tap-transition state and feedback matrix. It neither
//! imports production delay helpers nor shares its state encoding.

/// Static controls for the two independently delayed lanes and their explicit feedback matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceDelayParameters {
    /// Left delay time in milliseconds.
    pub left_delay_ms: f64,
    /// Right delay time in milliseconds.
    pub right_delay_ms: f64,
    /// Left signed feedback coefficient.
    pub left_feedback: f64,
    /// Right signed feedback coefficient.
    pub right_feedback: f64,
    /// Left damping coefficient.
    pub left_damping: f64,
    /// Right damping coefficient.
    pub right_damping: f64,
    /// Left wet mix.
    pub left_mix: f64,
    /// Right wet mix.
    pub right_mix: f64,
    /// Shared dual-mono through ping-pong feedback amount.
    pub cross_feedback: f64,
}

/// Reference construction input was invalid for the frozen launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceDelayError {
    /// A rate or parameter was nonfinite or outside its inclusive domain.
    InvalidInput,
}

#[derive(Clone, Debug)]
struct ReferenceLane {
    delay_target_ms: f64,
    active_delay: usize,
    transition_delay: usize,
    pending_delay: usize,
    transition_remaining: u32,
    damping_state: f64,
    valid_history: usize,
    ring: Vec<f64>,
}

impl ReferenceLane {
    fn new(delay_ms: f64, sample_rate_hz: f64, ring_length: usize) -> Self {
        let delay = delay_samples(delay_ms, sample_rate_hz).expect("validated delay");
        Self {
            delay_target_ms: delay_ms,
            active_delay: delay,
            transition_delay: delay,
            pending_delay: delay,
            transition_remaining: 0,
            damping_state: 0.0,
            valid_history: 0,
            ring: vec![0.0; ring_length],
        }
    }

    fn set_delay(
        &mut self,
        milliseconds: f64,
        sample_rate_hz: f64,
    ) -> Result<(), ReferenceDelayError> {
        let delay = delay_samples(milliseconds, sample_rate_hz)?;
        self.delay_target_ms = milliseconds;
        self.pending_delay = delay;
        Ok(())
    }

    fn start_transition(&mut self) {
        if self.transition_remaining == 0 && self.pending_delay != self.active_delay {
            self.transition_delay = self.pending_delay;
            self.transition_remaining = 128;
        }
    }

    fn tap(&self, cursor: usize, delay: usize) -> f64 {
        if delay > self.valid_history {
            0.0
        } else {
            let index = (cursor + self.ring.len() - delay) % self.ring.len();
            self.ring[index]
        }
    }

    fn read_transition(&mut self, cursor: usize) -> f64 {
        let remaining = self.transition_remaining;
        if remaining == 0 {
            return self.tap(cursor, self.active_delay);
        }
        let new = self.tap(cursor, self.transition_delay);
        if remaining == 1 {
            self.active_delay = self.transition_delay;
            self.transition_remaining = 0;
            return new;
        }
        let old = self.tap(cursor, self.active_delay);
        self.transition_remaining -= 1;
        let update = 129 - remaining;
        let alpha = f64::from(update) * (1.0 / 128.0);
        old + alpha * (new - old)
    }

    fn write(&mut self, cursor: usize, value: f64) {
        self.ring[cursor] = value;
        self.valid_history = self.valid_history.saturating_add(1).min(self.ring.len());
    }
}

/// Independent `f64` fixed-two-second circular-delay pair.
#[derive(Clone, Debug)]
pub struct ReferenceDelayPair {
    sample_rate_hz: f64,
    cursor: usize,
    parameters: ReferenceDelayParameters,
    left_damping_g: f64,
    right_damping_g: f64,
    left: ReferenceLane,
    right: ReferenceLane,
}

impl ReferenceDelayPair {
    /// Constructs a pair with the frozen rounded integer-tap mapping and two-second rings.
    pub fn new(
        sample_rate_hz: f64,
        parameters: ReferenceDelayParameters,
    ) -> Result<Self, ReferenceDelayError> {
        validate_parameters(sample_rate_hz, parameters)?;
        let max_delay = usize::try_from((2.0 * sample_rate_hz) as u64)
            .map_err(|_| ReferenceDelayError::InvalidInput)?;
        let ring_length = max_delay
            .checked_add(3)
            .ok_or(ReferenceDelayError::InvalidInput)?;
        Ok(Self {
            sample_rate_hz,
            cursor: 0,
            parameters,
            left_damping_g: damping_coefficient(parameters.left_damping, sample_rate_hz),
            right_damping_g: damping_coefficient(parameters.right_damping, sample_rate_hz),
            left: ReferenceLane::new(parameters.left_delay_ms, sample_rate_hz, ring_length),
            right: ReferenceLane::new(parameters.right_delay_ms, sample_rate_hz, ring_length),
        })
    }

    /// Queues one rounded integer-tap change without interrupting an active crossfade.
    pub fn set_delay(
        &mut self,
        left_milliseconds: f64,
        right_milliseconds: f64,
    ) -> Result<(), ReferenceDelayError> {
        self.left
            .set_delay(left_milliseconds, self.sample_rate_hz)?;
        self.right
            .set_delay(right_milliseconds, self.sample_rate_hz)
    }

    /// Processes one finite input pair using the fixed damping and explicit feedback matrix.
    pub fn process_sample(&mut self, left_input: f64, right_input: f64) -> (f64, f64) {
        self.left.start_transition();
        self.right.start_transition();
        let left_tap = self.left.read_transition(self.cursor);
        let right_tap = self.right.read_transition(self.cursor);
        let left_filtered = damp(left_tap, self.left_damping_g, &mut self.left.damping_state);
        let right_filtered = damp(right_tap, self.right_damping_g, &mut self.right.damping_state);
        let left_gain = self.parameters.left_feedback * left_filtered;
        let right_gain = self.parameters.right_feedback * right_filtered;
        let (left_feedback, right_feedback) =
            matrix(self.parameters.cross_feedback, left_gain, right_gain);
        self.left.write(self.cursor, left_input + left_feedback);
        self.right.write(self.cursor, right_input + right_feedback);
        self.cursor = (self.cursor + 1) % self.left.ring.len();
        (
            mix(left_input, left_tap, self.parameters.left_mix),
            mix(right_input, right_tap, self.parameters.right_mix),
        )
    }

    /// Returns the exact rounded integer tap for an accepted millisecond value.
    pub fn rounded_delay_samples(
        milliseconds: f64,
        sample_rate_hz: f64,
    ) -> Result<usize, ReferenceDelayError> {
        delay_samples(milliseconds, sample_rate_hz)
    }
}

fn validate_parameters(
    sample_rate_hz: f64,
    parameters: ReferenceDelayParameters,
) -> Result<(), ReferenceDelayError> {
    if !matches!(sample_rate_hz as u32, 44_100 | 48_000 | 88_200 | 96_000)
        || sample_rate_hz != (sample_rate_hz as u32) as f64
        || !in_range(parameters.left_delay_ms, 1.0, 2000.0)
        || !in_range(parameters.right_delay_ms, 1.0, 2000.0)
        || !in_range(parameters.left_feedback, -0.95, 0.95)
        || !in_range(parameters.right_feedback, -0.95, 0.95)
        || !in_range(parameters.left_damping, 0.0, 0.995)
        || !in_range(parameters.right_damping, 0.0, 0.995)
        || !in_range(parameters.left_mix, 0.0, 1.0)
        || !in_range(parameters.right_mix, 0.0, 1.0)
        || !in_range(parameters.cross_feedback, 0.0, 1.0)
    {
        return Err(ReferenceDelayError::InvalidInput);
    }
    Ok(())
}

fn in_range(value: f64, minimum: f64, maximum: f64) -> bool {
    value.is_finite() && (minimum..=maximum).contains(&value)
}

fn delay_samples(milliseconds: f64, sample_rate_hz: f64) -> Result<usize, ReferenceDelayError> {
    if !in_range(milliseconds, 1.0, 2000.0) || !sample_rate_hz.is_finite() || sample_rate_hz <= 0.0
    {
        return Err(ReferenceDelayError::InvalidInput);
    }
    let rounded = (milliseconds * sample_rate_hz / 1000.0 + 0.5).floor();
    let delay = usize::try_from(rounded as u64).map_err(|_| ReferenceDelayError::InvalidInput)?;
    let maximum = usize::try_from((2.0 * sample_rate_hz) as u64)
        .map_err(|_| ReferenceDelayError::InvalidInput)?;
    if (1..=maximum).contains(&delay) {
        Ok(delay)
    } else {
        Err(ReferenceDelayError::InvalidInput)
    }
}

/// Topology-preserving one-pole low pass, written from the equations.
///
/// Zavalishin, *The Art of VA Filter Design*, chapter 3: the trapezoidal integrator of a one-pole
/// low pass with `g = G / (1 + G)` resolved for its instantaneous feedback is
/// `v = g * (x - s)`, `y = s + v`, `s' = y + v`. `g == 0` is the exact identity.
fn damp(tap: f64, g: f64, state: &mut f64) -> f64 {
    if g == 0.0 {
        *state = tap;
        return tap;
    }
    let v = g * (tap - *state);
    let out = *state + v;
    *state = out + v;
    out
}

/// Sample rate the frozen damping control keeps its meaning at.
const DAMPING_REFERENCE_RATE_HZ: f64 = 48_000.0;

/// `0.45 * 44_100`: strictly below Nyquist at every launch rate.
const DAMPING_MAX_CUTOFF_HZ: f64 = 19_845.0;

/// Maps the frozen linear damping control to the one-pole coefficient at `sample_rate_hz`.
///
/// The frozen `y = (1 - c) * x + c * y` recurrence has its pole at `-ln(c) * fs / (2 pi)`; holding
/// that cutoff — evaluated once at the 48 kHz reference rate — fixed in hertz is what makes the
/// control rate invariant. Written here from the definition, with the platform `ln` and `tan`, so
/// that the oracle shares no code with the engine.
fn damping_coefficient(c: f64, sample_rate_hz: f64) -> f64 {
    if c == 0.0 {
        return 0.0;
    }
    let cutoff =
        (-c.ln() * DAMPING_REFERENCE_RATE_HZ / (2.0 * core::f64::consts::PI)).min(DAMPING_MAX_CUTOFF_HZ);
    let big_g = (core::f64::consts::PI * cutoff / sample_rate_hz).tan();
    big_g / (1.0 + big_g)
}

fn matrix(cross_feedback: f64, left: f64, right: f64) -> (f64, f64) {
    if cross_feedback == 0.0 {
        (left, right)
    } else if cross_feedback == 1.0 {
        (right, left)
    } else {
        let opposite = 1.0 - cross_feedback;
        (
            opposite * left + cross_feedback * right,
            cross_feedback * left + opposite * right,
        )
    }
}

fn mix(dry: f64, wet: f64, amount: f64) -> f64 {
    if amount == 0.0 {
        dry
    } else if amount == 1.0 {
        wet
    } else {
        dry + amount * (wet - dry)
    }
}
