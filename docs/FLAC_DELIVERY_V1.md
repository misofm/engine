# FLAC delivery v1

FLAC is a lossless transport for canonical PCM; it is never a stem identity. The identity remains
the `sha256:` digest of the samples-only serialization in
[`STEM_IDENTITY_V1.md`](STEM_IDENTITY_V1.md). Encoder version, settings, comments, padding, frame
partition, and the FLAC file's own bytes may all change without changing identity. Lossy codecs
cannot satisfy this class-A contract and are not admitted.

## Shipped decoder

The client decoder is `miso-engine-flac-decoder`, built as a standalone
`wasm32-unknown-unknown` artifact. It uses Symphonia 0.6.1, pinned exactly in `Cargo.toml` and by the
registry checksum in `Cargo.lock`. `scripts/build-flac-decoder.sh` uses the repository's pinned
Rust 1.97.1 release profile, strips only debug information, verifies the committed artifact
SHA-256, and emits the Wasm with its JavaScript loader and declaration.

The loader hashes the exact fetched bytes before compiling those same bytes. A mismatch refuses
with `miso.flac.decoder.artifact_mismatch`; it is never a warning or fallback. Decoder format,
shape, corruption, length, and resource refusals are likewise typed. Browser audio APIs, including
`decodeAudioData`, are outside this path.

The Worker copies FLAC transport bytes into the decoder's bounded staging region once. Decoded
output is pumped one FLAC frame at a time as exact interleaved integer canonical PCM. The Worker
incrementally hashes and persists each block, then discards the transport; playback reads stored
PCM and never invokes the decoder.

`scripts/check-flac-decoder.sh` rebuilds and checks the exact artifact pin, Node-hosted Wasm ABI,
all shared conformance vectors, typed artifact/sample mutations, and the `decodeAudioData` ban. It
is an explicit `scripts/sweep.sh` row. The browser qualification workflow consumes that same
built artifact and repeats the vectors in a real Worker under Chromium, Firefox, and WebKit.

For the engine pump only, an integer sample becomes planar `f32` by division by exactly
`2^(bit_depth - 1)`: PCM16 divides by `32768`; PCM24 divides by `8388608`. Both denominators are
powers of two and every launch integer value is exactly representable in `f32`, so the conversion
is bit-exact. Identity is established over the integer bytes before this conversion.

## Publisher

`miso-engine-stem-publisher` accepts a signed PCM16 or packed PCM24 WAVE master. It asks the shared
`miso-engine-stem-hasher` library for the canonical identity, encodes with the exact pinned
`flacenc 0.5.1` single-thread configuration recorded in the emitted JSON row, decodes the result
through the same `miso-engine-flac-decoder` core shipped to clients, and asks the shared hasher to
recompute the decoded identity. It creates the delivery object only after shape and identity both
round-trip. A corrupt or shortened encode is therefore not publishable.

The default publisher block size is 4096 frames. A nondefault `--block-frames` is available for
conformance and reproducibility checks and is always recorded. Changing it is expected to change
FLAC transport bytes while leaving the canonical identity unchanged.

## One-way catalog migration

`miso-engine-catalog-migrate` consumes an explicit pre-launch catalog of WAVE masters plus a
complete embedding inventory. It verifies each old `sha256:` value against the old container-byte
law, computes the new canonical-PCM identity through the shared hasher, and emits:

- `identity-mapping.tsv`, the old-to-new oracle;
- `manifest.tsv`, regenerated canonical identity and shape rows; and
- `document-replacements.tsv`, every manifest row, mix document, app fixture, package pin, and
  server record that #246 must rewrite.

There is no reverse mapping, alias, fallback, or dual-identity compatibility window. This tool
does not edit the app repository; #246 applies the pinned mapping and performs the app-side
regeneration in one stroke.

The non-gating cold-ingest sanity runner is `npm run flac-throughput` in
`hosts/miso-engine-host-web/qualification`. It accepts a pinned decoder artifact directory, one
FLAC delivery object, its canonical byte count and identity, and a browser name; decode and SHA-256
both execute in the Worker and the runner logs elapsed time and MiB/s. Its input is deliberately
temporary and it never writes a sealed benchmark artifact.
