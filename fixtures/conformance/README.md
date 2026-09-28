# Conformance fixtures

`v1/*.mepcm` is planar, channel-major IEEE-754 `f32` PCM. The fixed 48-byte little-endian header is
`MISOEPCM`, version `u16=1`, header length `u16=48`, flags `u32=0`, rate `u32`, channels
`u16`, encoding `u16=1`, frames `u64`, payload bytes `u64`, CRC-32C `u32`, reserved `u32=0`.
The CRC is Castagnoli reflected polynomial `0x82F63B78`, init/final XOR `0xffffffff`, over the complete
file with header CRC bytes zeroed. `MANIFEST.tsv` starts with
`miso-engine-fixture-manifest-v1`; each sorted row contains the canonical stored CRC-32C, byte length,
and safe relative path. It is intentionally a corruption/integrity check, not authenticity.

The seven checked-in files and `MANIFEST.tsv` are byte-frozen. Every file is at a launch rate
(44,100, 48,000, 88,200 or 96,000 Hz), the only rates the parser accepts. Owner ruling R5 (#1036)
removed the four files at the former extended research rates (176,400, 192,000, 352,800 and
384,000 Hz); the remaining files' bytes did not change.

Run `cargo run --locked -p conformance --example conformance_fixtures -- --check`.
Only a maintainer deliberately updating the corpus may run `--write`, then review every checksum change.

Regeneration is pinned to the generating platform's libm (audit #105, finding F9). The sine content
in the corpus was produced with `f32::sin`, whose result is not specified to the last bit across
platforms or toolchain versions, so a `--write` on a different machine can move the CRCs without any
intended change. These bytes are frozen contract bytes: treat any CRC movement under `--write` as a
platform difference to be investigated, never as a fixture to be re-pinned.
