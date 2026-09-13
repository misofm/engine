# Stem identity v1: canonical PCM

This document is the normative launch contract for content-addressed stems. A stem's identity is
its sound samples, not its delivery wrapper: the same integer audio carried by different external
containers has
the same identity. `MUST`, `MUST NOT`, and `SHOULD` are normative requirements.

## Declaration and interpretation

The source declaration established with this contract supplies `content`, `channels`, `bit_depth`,
and `frames`. The session root supplies the only `sample_rate_hz`; there is no per-source rate and
no implicit sample-rate conversion. `bit_depth` is the closed token set `{16, 24, 32f}`. The
integer tokens retain source-native signed integers; `32f` is IEEE-754 binary32.

Shape is deliberately not part of the hash preimage. A hash alone therefore underdetermines the
sound: the `(declaration, hash)` pair determines it. Reusing the same canonical bytes under a
different declaration in another document is a coherent, harmless reinterpretation, not an
identity collision. Within one declaration, every resolver and decoder MUST reproduce the exact
declared integer samples or float bit patterns, as applicable, or refuse before the stem is
admitted.

## Canonical serialization

For `frames` frames, `channels` channels, and `bit_depth`:

1. Serialize samples only. There is no header, magic, version, length prefix, sample-rate word,
   channel-count word, padding, metadata, checksum, or container data.
2. Samples are interleaved in frame-major order: frame 0 channel 0, frame 0 channel 1, ..., then
   frame 1 channel 0, and so on. Channel indices are zero-based.
3. For `bit_depth = 16` and `bit_depth = 24`, each sample is a signed two's-complement integer at
   the source-native declared depth. `16` uses exactly two little-endian bytes. `24` uses exactly
   three little-endian bytes, packed with no sign-extension or pad byte. For `bit_depth = 32f`,
   each sample is its raw four-byte IEEE-754 binary32 bit pattern in little-endian order. Every NaN
   payload and sign bit is identity-bearing: implementations MUST NOT canonicalize NaNs or
   negative zero.
4. The exact preimage length is
   `frames * channels * bytes_per_sample(bit_depth)` bytes, where the widths for `16`, `24`, and
   `32f` are 2, 3, and 4 respectively. Implementations MUST use checked arithmetic and MUST reject
   a byte-length mismatch. This is also the stem store's mandatory open-time length check.
5. Hash the complete serialization with BLAKE3-256. The identity string is `blake3:` followed by the
   64 lowercase hexadecimal digest characters. The exact grammar is
   `^blake3:[0-9a-f]{64}$`.

Container bytes never join the hash. A WAVE parser or external ingress adapter conforms only when
it produces the exact serialization above before hashing. Integer samples MUST
stay integer-valued through this boundary; an implementation may not make identity depend on a
floating-point decoder's rounding behavior.

BLAKE3-256 is the canonical PCM identity vocabulary. No whole-stem residency is permitted, so a
shipped incremental implementation is mandatory where one-shot WebCrypto cannot cover the input.
The Rust reference oracle uses the pinned workspace `blake3` implementation. Artifact, package,
effect, and render hashes retain their own schemes and are outside this contract.

## Frozen conformance vectors

The complete machine-readable corpus is
[`fixtures/stem-identity/v1/VECTORS.tsv`](../fixtures/stem-identity/v1/VECTORS.tsv). Commas separate
channels inside a frame; vertical bars separate frames. The table below repeats every normative
pin so each answer is independently hand-derivable.

| Vector | Depth | Channels x frames | Samples by frame | Canonical bytes (hex) | Identity |
| --- | ---: | ---: | --- | --- | --- |
| `f32-mono-edge-bits` | 32f | 1 x 3 | `0x7fc00001 \| 0x00000001 \| 0x80000000` | `0100c07f0100000000000080` | `blake3:864e0348bd18954c2804d91b0c3c44fdc023617c2f111bdf7eeb84f7d85ba334` |
| `f32-stereo-edge-bits` | 32f | 2 x 2 | `(0x7fc00001,0x80000000) \| (0x00000001,0xffc12345)` | `0100c07f00000080010000004523c1ff` | `blake3:4cdda9e6378fd2c56bc8aa22c59848729bbc7f9ddb45fc0ace156ee0ec5a60f2` |
| `pcm16-mono-boundaries` | 16 | 1 x 5 | `0 \| 32767 \| -32768 \| 1 \| -1` | `0000ff7f00800100ffff` | `blake3:019f72841a70e06c516205a07deef8a3d4d42ab26faa3cd4bf18275f9b5eabe1` |
| `pcm16-stereo-boundaries` | 16 | 2 x 3 | `(0,32767) \| (-32768,1) \| (-1,0)` | `0000ff7f00800100ffff0000` | `blake3:41b5fff5e18a17133898edad06d13da6504aea16e283378ee6c13f6a3faed9fe` |
| `pcm24-mono-boundaries` | 24 | 1 x 5 | `0 \| 8388607 \| -8388608 \| 1 \| -1` | `000000ffff7f000080010000ffffff` | `blake3:ab6cdfd122c457fca7c78a12b6a532de760905381edad178f234cee443051265` |
| `pcm24-stereo-boundaries` | 24 | 2 x 3 | `(0,8388607) \| (-8388608,1) \| (-1,0)` | `000000ffff7f000080010000ffffff000000` | `blake3:cc7b9b00ad74b505b7e73e097f85cd196030cc954a47f628394ec721b1e8d1f5` |

The `32f` sample text is exact hexadecimal IEEE-754 bits. Those rows cover NaN payloads, the least
positive subnormal, and negative zero without relying on host-language float formatting.

All six rows have committed headerless `.pcm` files. The stereo row at each depth also has a
committed `.wav` fixture. The reference WAVE path MUST strip the two different wrappers and
produce bytes and identity equal to the corresponding `.pcm` row.

## Reference oracle

`stem-hasher` is the publishing and migration oracle. It streams raw PCM or parses
RIFF/WAVE and RF64/WAVE through the engine's own `source` parser, serializes each
sample through the rules above, optionally emits the canonical preimage, and prints the identity.
It never retains a complete stem.

Raw input is little-endian canonical PCM at the explicitly supplied shape: signed two's-complement
for depths 16/24, or raw IEEE-754 bit patterns for `32f`:

```sh
cargo run --locked -p stem-hasher -- raw \
  --input stem.pcm --channels 2 --bit-depth 24 --frames 10617984
```

WAVE supplies its shape through the engine parser and is accepted only for signed PCM16, packed
PCM24, or IEEE float32:

```sh
cargo run --locked -p stem-hasher -- wave --input stem.wav
```

With no `--output`, stdout is the identity. `--output PATH` creates a new canonical-PCM file and
still prints the identity to stdout; it refuses to replace an existing path. `--output -` writes
canonical bytes to stdout and writes the identity to stderr, keeping the binary stream pure.

The corpus gate is:

```sh
python3 fixtures/stem-identity/v1/generate.py --check
cargo test --locked -p stem-hasher
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
identity scheme-prefix rule covers both `blake3:` stems and the CID scheme used by third-party
effect packages. Store layout must reserve a namespace for non-stem artifacts; package fetching
and execution remain deferred to the effect-package workstream.
