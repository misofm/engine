//! Offline processor boundary deliberately independent from production kernels.

use crate::{F64PlanarBuffer, ReferenceBlockError};

/// A test-only processor of complete planar `f64` buffers.
pub trait OfflineF64Processor {
    /// Processes `input` into `output`; implementations must preserve shape.
    fn process(&mut self, input: &F64PlanarBuffer, output: &mut F64PlanarBuffer);
}

/// Runs one independent offline processor.
pub fn render_planar_f64<P: OfflineF64Processor>(
    processor: &mut P,
    input: &F64PlanarBuffer,
) -> Result<F64PlanarBuffer, ReferenceBlockError> {
    let mut output = F64PlanarBuffer::zeros(input.channels(), input.frames())?;
    processor.process(input, &mut output);
    Ok(output)
}

/// The intentionally simple baseline oracle used by issue 002.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdentityProcessor;

impl OfflineF64Processor for IdentityProcessor {
    fn process(&mut self, input: &F64PlanarBuffer, output: &mut F64PlanarBuffer) {
        output.samples_mut().copy_from_slice(input.samples());
    }
}
