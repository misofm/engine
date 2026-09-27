# Report dual-mono stems in the stem hasher


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A (tooling; no render change). Base `6ca203f8`.

## Problem

The engine processes both channels of a stereo stem whose channels are bit-identical, because the
collapse is driven by the session's source mapping, not by content. The exact remedy exists:
`session_validator fold-mono` (#744) turns such a source into a one-channel source and remaps its
tracks to `(0, 0)`, bit-exactly. But it needs a caller that has *proved* `L == R` and knows the
mono identity. Only the release CLI (misofm/cli `prepare-sparse`) does that today, in TypeScript.
The engine's reference stem tool cannot, so agent-built sessions (the dogfood first-listen mix has
18 dual-mono stems of 81, all declared stereo), SDK users and a future browser import have no
engine-side way to find them.

## Outcome

`stem_hasher wave --input <file> --report-channels` (and the `raw` mode with the same flag) prints,
after the identity line, one line:

    channels_identical=<true|false>[ mono_identity=blake3:<64 hex>]

computed in the **same streamed pass** as the identity (no second read, no whole-file buffer).
`true` only for a 2-channel stem whose two channel words are byte-equal in every frame (so for
`32f`, `-0.0` and `+0.0` differ and NaN payloads differ); `mono_identity` is the canonical identity
of channel 0 alone (the identity a one-channel stem of those samples would have).
One-channel and 3+-channel stems print `channels_identical=false`. Without the flag, output is
byte-for-byte what it is today.

## Read first

* `tools/stem-hasher/src/lib.rs:173-330` (`canonicalize_raw_pcm`, `canonicalize_wave`, the chunked
  serializer) and `:341-431` (`parse_cli`, `run_cli`).
* `docs/STEM_IDENTITY_V1.md` (canonical bytes, the mono vectors at lines 63-67).

## Authorized paths

`tools/stem-hasher/src/lib.rs`, `tools/stem-hasher/src/main.rs`, `tools/stem-hasher/tests/`, and a
short paragraph in `docs/STEM_IDENTITY_V1.md` naming the report as informative (not part of the
identity contract).

## Gates

* Vectors, each asserted on both `wave` and `raw` input: identical stereo at 16, 24 and `32f`
  (true, and `mono_identity` equals the identity of the same samples written as a one-channel
  stem); a single one-LSB difference in the **last** frame (false); `32f` with `+0.0` left and
  `-0.0` right in one frame (false); a stem whose right channel is all zeros (false); one-channel
  stem (false). A chunk-boundary case: the difference falls exactly at a 48 KiB chunk edge.
* Without `--report-channels` the existing `tools/stem-hasher/tests/conformance.rs` output is
  unchanged.
* Memory stays bounded: no allocation proportional to the stem (the existing streaming structure).
* `cargo test -p stem-hasher`, `cargo fmt --check`, clippy `-D warnings`.

## Non-goals

Changing the session or the render path; folding (that is `fold-mono`); near-mono or one-sided
stems (a separate ruling); any browser or CLI adoption (separate repositories).

## Evidence

Measured cost of the comparison on the 81 dogfood stems (native, naive per-frame compare,
early exit): 3.0 ns per frame for a full compare, 0.56 s for all 81 files
(`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md` §4.5).

