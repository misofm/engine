//! Issue #1051: the compressor's randomized bank differential, with restored in-flight ramps whose
//! two channels differ in exactly one field.
//!
//! The shared harness (`conformance::run_effect_differential`) renders every bank this build binds
//! against its scalar instances, chunked, collapsed and restored. What only the compressor can draw
//! is its own payload: one channel section is 37 words (#1278), the gain
//! reduction, then `current`, `target`, `step` and `remaining` for each of the seven smoothed
//! parameters, then the same four for the attack and release coefficient ramps. [`craft`] writes a
//! lane whose left and right sections are identical except for one parameter-ramp field,
//! mid-flight, with the step `set_target` would derive:
//!
//! * **only `target`**, one ulp apart, with `current` far enough away that both channels derive
//!   the same `step` bits;
//! * **only `remaining`**, on a ramp already at its target, so both steps are `+0.0`;
//! * or no difference at all.
//!
//! The channel-symmetry witness (`designed_channel_symmetry`, the `DESIGNED` term of the mono
//! collapse) must answer `false` for the first two and `true` for the third, on the scalar instance
//! and on the bank lane. A witness that folds its per-field comparison with `&&` instead of `||`
//! answers `true` on a lane whose channels differ in one field -- the #970 class, one step earlier
//! (issue #1051 amendment 2).

use compressor::{CompressorFactory, STATE_HEADER_WORDS};
use conformance::Payload;
use dsp_reference::randomized::Draw;
use effect_contract::PreparedEffectMetadata;

fn word(section: &[u8], index: usize) -> [u8; 4] {
    section[index * 4..index * 4 + 4]
        .try_into()
        .expect("a four-byte word")
}

fn set(section: &mut [u8], index: usize, bytes: [u8; 4]) {
    section[index * 4..index * 4 + 4].copy_from_slice(&bytes);
}

/// `(target - current) / remaining`, as `LinearRamp::set_target` derives a ramp's step.
fn step(current: f32, target: f32, remaining: u32) -> u32 {
    if remaining == 0 {
        0.0_f32.to_bits()
    } else {
        ((target - current) / remaining as f32).to_bits()
    }
}

/// Rewrites one lane's payload so its channels differ in exactly one ramp field (or in none), and
/// returns what the witness must answer.
fn craft(
    draw: &mut Draw,
    metadata: &PreparedEffectMetadata,
    payload: &mut Payload,
) -> Option<bool> {
    assert_eq!(payload.left.len(), STATE_HEADER_WORDS * 4);
    // Both channels start from the left section: symmetric running state and designed words.
    payload.right = payload.left.clone();
    let parameter = draw.below(7);
    let spec = &metadata.descriptor.parameters[parameter];
    let (low, high) = (
        spec.minimum.expect("continuous"),
        spec.maximum.expect("continuous"),
    );
    let smoothing = spec.smoothing_samples;
    let base = 1 + parameter * 4;
    match draw.below(3) {
        // Only `target` differs: one ulp, with the same derived step.
        0 => {
            for _ in 0..64 {
                let inside = draw.in_domain(low, high);
                let current = draw.pick(&[low, high, inside]);
                let target = draw.in_domain(low, high);
                let other = if target < high {
                    target.next_up()
                } else {
                    target.next_down()
                };
                let remaining = 1 + draw.below(smoothing as usize) as u32;
                if current == target
                    || other.to_bits() == 0x8000_0000
                    || step(current, target, remaining) != step(current, other, remaining)
                {
                    continue;
                }
                for (section, target) in [(&mut payload.left, target), (&mut payload.right, other)]
                {
                    set(section, base, current.to_le_bytes());
                    set(section, base + 1, target.to_le_bytes());
                    set(
                        section,
                        base + 2,
                        step(current, target, remaining).to_le_bytes(),
                    );
                    set(section, base + 3, remaining.to_le_bytes());
                }
                return Some(false);
            }
            Some(true)
        }
        // Only `remaining` differs, on a ramp at its target: both steps are `+0.0`.
        1 => {
            let value = draw.in_domain(low, high);
            let left = 1 + draw.below(smoothing as usize) as u32;
            let right = (left + 1 + draw.below(smoothing as usize) as u32) % (smoothing + 1);
            for (section, remaining) in [(&mut payload.left, left), (&mut payload.right, right)] {
                set(section, base, value.to_le_bytes());
                set(section, base + 1, value.to_le_bytes());
                set(section, base + 2, 0_u32.to_le_bytes());
                set(section, base + 3, remaining.to_le_bytes());
            }
            Some(left == right)
        }
        // No difference: a symmetric in-flight ramp.
        _ => {
            let (current, target) = (draw.in_domain(low, high), draw.in_domain(low, high));
            let remaining = draw.below(smoothing as usize + 1) as u32;
            let (current, remaining) = if remaining == 0 {
                (target, 0)
            } else {
                (current, remaining)
            };
            for section in [&mut payload.left, &mut payload.right] {
                set(section, base, current.to_le_bytes());
                set(section, base + 1, target.to_le_bytes());
                set(
                    section,
                    base + 2,
                    step(current, target, remaining).to_le_bytes(),
                );
                set(section, base + 3, remaining.to_le_bytes());
            }
            debug_assert_eq!(word(&payload.left, base), word(&payload.right, base));
            Some(true)
        }
    }
}

conformance::randomized_effect_test!(
    the_bank_renders_its_scalar_instances_under_random_state,
    CompressorFactory,
    seeds: 32,
    blocks: 24,
    craft: Some(craft),
    known: &[],
    banks_natively: true,
    witness: true,
);

/// #1278: the plan-swap carry restores every lane it carries, so a restore must accept every state
/// the effect itself reaches -- including a smoothed ramp to a domain edge whose iterated
/// `current + step` has rounded past the edge, at every launch rate. Red on a restore that holds a
/// moving ramp's `current` (or a subnormal step) to the strict domain.
#[test]
fn the_effects_own_edge_ramp_snapshots_restore() {
    conformance::EffectDifferential::assert_edge_ramps_restore(&compressor::CompressorFactory);
}
