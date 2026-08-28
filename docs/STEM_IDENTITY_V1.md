# Stem identity v1: canonical PCM

This document is the normative launch contract for content-addressed stems. A stem's identity is
its sound samples, not its delivery wrapper: the same integer audio delivered as WAV or FLAC has
the same identity. `MUST`, `MUST NOT`, and `SHOULD` are normative requirements.

## Declaration and interpretation

The source declaration established with this contract supplies `content`, `channels`, `bit_depth`,
and `frames`. The session root supplies the only `sample_rate_hz`; there is no per-source rate and
no implicit sample-rate conversion. Launch `bit_depth` is the closed integer set `{16, 24}`. A
future 32-bit float token would be an additive schema change, not an interpretation of either
launch token.

Shape is deliberately not part of the hash preimage. A hash alone therefore underdetermines the
sound: the `(declaration, hash)` pair determines it. Reusing the same canonical bytes under a
different declaration in another document is a coherent, harmless reinterpretation, not an
identity collision. Within one declaration, every resolver and decoder MUST reproduce the exact
declared integer samples or refuse before the stem is admitted.

## Canonical serialization

For `frames` frames, `channels` channels, and `bit_depth`:

1. Serialize samples only. There is no header, magic, version, length prefix, sample-rate word,
   channel-count word, padding, metadata, checksum, or container data.
2. Samples are interleaved in frame-major order: frame 0 channel 0, frame 0 channel 1, ..., then
   frame 1 channel 0, and so on. Channel indices are zero-based.
3. Each sample is a signed two's-complement integer at the source-native declared depth.
   `bit_depth = 16` uses exactly two little-endian bytes. `bit_depth = 24` uses exactly three
   little-endian bytes, packed with no sign-extension or pad byte.
4. The exact preimage length is
   `frames * channels * (bit_depth / 8)` bytes. Implementations MUST use checked arithmetic and
   MUST reject a byte-length mismatch. This is also the stem store's mandatory open-time length
   check.
5. Hash the complete serialization with SHA-256. The identity string is `sha256:` followed by the
   64 lowercase hexadecimal digest characters. The exact grammar is
   `^sha256:[0-9a-f]{64}$`.

Container bytes never join the hash. A WAVE parser, FLAC decoder, publisher, and store ingest path
conform only when they produce the exact serialization above before hashing. Integer samples MUST
stay integer-valued through this boundary; an implementation may not make identity depend on a
floating-point decoder's rounding behavior.

SHA-256 is the existing repository identity vocabulary and is verifiable in the Sui framework via
`std::hash::sha2_256`; Blake3 is not available there. No whole-stem residency is permitted, so a
shipped incremental implementation is mandatory where one-shot WebCrypto cannot cover the input.
The Rust reference oracle uses the pinned workspace `sha2` implementation. The choice does not
rest on a claim of zero shipped hashing code.

## Frozen conformance vectors

The complete machine-readable corpus is
[`fixtures/stem-identity/v1/VECTORS.tsv`](../fixtures/stem-identity/v1/VECTORS.tsv). Commas separate
channels inside a frame; vertical bars separate frames. The table below repeats every normative
pin so each answer is independently hand-derivable.

| Vector | Depth | Channels x frames | Samples by frame | Canonical bytes (hex) | Identity |
| --- | ---: | ---: | --- | --- | --- |
| `pcm16-mono-boundaries` | 16 | 1 x 5 | `0 \| 32767 \| -32768 \| 1 \| -1` | `0000ff7f00800100ffff` | `sha256:342f56e6d16f7cbcd69bbc003e4e16d0fa45335f3756701db3a6649f19d6042c` |
| `pcm16-stereo-boundaries` | 16 | 2 x 3 | `(0,32767) \| (-32768,1) \| (-1,0)` | `0000ff7f00800100ffff0000` | `sha256:0320b11905302eb840cd06ab90b0549114e6ee1c89233e928ebe21b8c4964ef2` |
| `pcm24-mono-boundaries` | 24 | 1 x 5 | `0 \| 8388607 \| -8388608 \| 1 \| -1` | `000000ffff7f000080010000ffffff` | `sha256:de48b490bab45d06c72b240d7e46efa95d07deb216eb7f1f2afc7a7e14a4b832` |
| `pcm24-stereo-boundaries` | 24 | 2 x 3 | `(0,8388607) \| (-8388608,1) \| (-1,0)` | `000000ffff7f000080010000ffffff000000` | `sha256:f014aa907c6c9894ab1a1d3b05a82f31b6ddb82f5cbc1e61fdc2d7c35245e4c6` |

All four rows have committed headerless `.pcm` files. The stereo row at each depth also has a
committed `.wav` fixture. The reference WAVE path MUST strip the two different wrappers and
produce bytes and identity equal to the corresponding `.pcm` row.

## Reference oracle

`miso-engine-stem-hasher` is the publishing and migration oracle. It streams raw PCM or parses
RIFF/WAVE and RF64/WAVE through the engine's own `miso-engine-source` parser, serializes each
sample through the rules above, optionally emits the canonical preimage, and prints the identity.
It never retains a complete stem.

Raw input is signed, little-endian PCM at the explicitly supplied shape:

```sh
cargo run --locked -p miso-engine-stem-hasher -- raw \
  --input stem.pcm --channels 2 --bit-depth 24 --frames 10617984
```

WAVE supplies its shape through the engine parser and is accepted only for signed PCM16 or packed
PCM24:

```sh
cargo run --locked -p miso-engine-stem-hasher -- wave --input stem.wav
```

With no `--output`, stdout is the identity. `--output PATH` creates a new canonical-PCM file and
still prints the identity to stdout; it refuses to replace an existing path. `--output -` writes
canonical bytes to stdout and writes the identity to stderr, keeping the binary stream pure.

The corpus gate is:

```sh
python3 fixtures/stem-identity/v1/generate.py --check
cargo test --locked -p miso-engine-stem-hasher
```

It exercises every row through the raw library and CLI paths and both WAVE fixtures through the
engine parser and CLI. Reversing sample endianness makes every pinned vector fail; changing stereo
channel order makes both stereo vectors fail.

## Render identity and future backends

Once a canonical session document embeds each source identity, its own digest transitively pins
the canonical source bytes under that document's declarations. For honest resolvers on class-A
CPU legs, **the document digest is the complete identity of the command-free render; live sessions
extend it by the command stream**. This claim does not promise byte identity for a future
class-P/GPU backend.

Any future class-P render cache MUST key on `(digest, backend-leg)`, not on the digest alone. The
parameter lattice introduced alongside this work makes persisted parameter space finite, which is
what makes a future complete render-cache key well-defined.

## Artifact namespaces

The ingest gate is phrased over every referenced content-addressed artifact, not only stems. The
identity scheme-prefix rule covers both `sha256:` stems and the CID scheme used by third-party
effect packages. Store layout must reserve a namespace for non-stem artifacts; package fetching
and execution remain deferred to the effect-package workstream.
