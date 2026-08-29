//! Render a session natively and print its PCM digest, for the SDK's cross-check (issue #243).
//!
//! # Why the oracle is a separate process rather than a fixture
//!
//! Issue #243 eval 2 asks that a document rendered *through the SDK* produce the same bits as the
//! same document rendered natively. A checked-in expected digest could not carry that claim: it
//! would pin whatever the SDK produced on the day it was written, and the two would agree forever
//! by construction. So the native side renders here, on the host CPU, through
//! `AudioWorkletEngineHost` — the same safe facade the browser module wraps — and the SDK asserts
//! against *this* answer.
//!
//! The two sides must therefore agree on the input as well as the output, which is what makes the
//! source generator below load-bearing. It is a plain 32-bit linear congruential generator with
//! the constants of Numerical Recipes, evaluated in `f64` and narrowed once to `f32`, and it is
//! mirrored character for character by `ramp()` in `sdk/test/support.mjs`. Nothing about it is
//! musical; what matters is that it is exactly reproducible in both languages, that it fills the
//! `f32` mantissa (so a bit-level difference cannot hide in a run of round numbers), and that it
//! needs no fixture file to travel between them.
//!
//! Equality is over `to_bits` — SHA-256 of the little-endian `f32` words — never a tolerance, in
//! keeping with every other parity gate in this repository.
//!
//! Usage: `sdk_render_oracle QUANTA [SEED] < session.toml`, printing one lowercase hex line.

use std::io::{Read as _, Write as _};

use miso_engine_host_web::{AudioWorkletEngineHost, RESULT_OK, WebBootOptions};
use sha2::{Digest, Sha256};

/// The shared deterministic source generator.
///
/// Mirrored by `ramp()` in `sdk/test/support.mjs`. Both sides step the same `u32` state with the
/// same constants and narrow through the same `f64` expression, so a divergence here is a
/// divergence in the test rather than in the engine — which is why the SDK's eval asserts the
/// generators agree before it asserts the digests do.
fn ramp(frames: usize, seed: u32) -> Vec<f32> {
    let mut state = seed;
    (0..frames)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((f64::from(state) / 4_294_967_296.0) * 2.0 - 1.0) as f32
        })
        .collect()
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let quanta: usize = arguments
        .next()
        .and_then(|value| value.parse().ok())
        .expect("usage: sdk_render_oracle QUANTA [SEED] < session.toml");
    let seed: u32 = arguments
        .next()
        .map_or(1, |value| value.parse().expect("SEED is a u32"));
    assert!(arguments.next().is_none(), "too many arguments");

    let mut document = Vec::new();
    std::io::stdin()
        .read_to_end(&mut document)
        .expect("read the session document from stdin");

    // Both `require_*` words are zero, exactly as the SDK's headless boot writes them: the native
    // oracle has no physical shape to satisfy either, so it accepts whatever the document declares
    // and the two boots differ in nothing at all.
    let mut host = AudioWorkletEngineHost::boot(&document, WebBootOptions::explicit_defaults())
        .unwrap_or_else(|failure| {
            panic!(
                "the oracle's document did not boot: result {} diagnostic {}",
                failure.result(),
                String::from_utf8_lossy(failure.diagnostic())
            )
        });

    let sample_rate_hz = host.status().sample_rate_hz;
    let quantum = host.status().quantum_frames as usize;

    // Every declared source is fed the same generator, keyed by its index, so a multi-source
    // document gives each source distinct content and the digest is sensitive to routing.
    let sources: Vec<(String, u32)> = (0..host.session_source_count())
        .map(|index| {
            let index = u32::try_from(index).expect("source index fits a u32");
            let id = host
                .session_source_id(index)
                .expect("a declared source has an id")
                .to_owned();
            let shape = host
                .session_source_shape(index)
                .expect("a declared source has a shape");
            (id, shape.channel_count)
        })
        .collect();

    let mut digest = Sha256::new();
    for block in 0..quanta {
        for (source_index, (id, channels)) in sources.iter().enumerate() {
            let planes: Vec<Vec<f32>> = (0..*channels)
                .map(|channel| {
                    ramp(
                        quantum,
                        seed.wrapping_add(
                            u32::try_from(source_index * 16 + channel as usize + block * 1024)
                                .expect("plane key fits a u32"),
                        ),
                    )
                })
                .collect();
            let borrowed: Vec<&[f32]> = planes.iter().map(Vec::as_slice).collect();
            let submitted = host.submit_source(
                id.as_bytes(),
                1,
                (block * quantum) as u64,
                sample_rate_hz,
                &borrowed,
                u32::try_from(quantum).expect("quantum fits a u32"),
                false,
            );
            assert_eq!(submitted, RESULT_OK, "source {id} refused block {block}");
        }
        assert_eq!(
            host.render_next(),
            RESULT_OK,
            "render refused block {block}"
        );
        let output = host.output_pcm().expect("a rendered quantum has output");
        for sample in output {
            digest.update(sample.to_le_bytes());
        }
    }

    let hex: String = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    writeln!(std::io::stdout(), "{hex}").expect("write the digest");
}
