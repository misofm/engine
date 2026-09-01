//! Bit-level repeatability helpers.

/// Repeat-render validation failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeterminismError {
    /// The two outputs have different sizes.
    LengthMismatch,
    /// A specific f32 bit pattern differs.
    BitMismatch {
        /// Sample index.
        index: usize,
        /// First output bits.
        first: u32,
        /// Second output bits.
        second: u32,
    },
}

/// Verifies exact `f32::to_bits` equality, preserving signed zero and NaN payloads.
pub fn verify_bit_exact_repeat(first: &[f32], second: &[f32]) -> Result<(), DeterminismError> {
    if first.len() != second.len() {
        return Err(DeterminismError::LengthMismatch);
    }
    for (index, (left, right)) in first.iter().zip(second).enumerate() {
        if left.to_bits() != right.to_bits() {
            return Err(DeterminismError::BitMismatch {
                index,
                first: left.to_bits(),
                second: right.to_bits(),
            });
        }
    }
    Ok(())
}
