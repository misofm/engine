# Conformance fixtures

`v1/*.mepcm` is planar, channel-major IEEE-754 `f32` PCM. The fixed 48-byte little-endian header is
`MISOEPCM`, version `u16=1`, header length `u16=48`, flags `u32=0`, rate `u32`, channels
`u16`, encoding `u16=1`, frames `u64`, payload bytes `u64`, CRC-32C `u32`, reserved `u32=0`.
The CRC is Castagnoli reflected polynomial `0x82F63B78`, init/final XOR `0xffffffff`, over the complete
file with header CRC bytes zeroed. `MANIFEST.tsv` starts with
`miso-engine-fixture-manifest-v1`; each sorted row contains the canonical stored CRC-32C, byte length,
and safe relative path. It is intentionally a corruption/integrity check, not authenticity.

The eleven checked-in files and `MANIFEST.tsv` are byte-frozen. The four files at 44,100, 48,000,
88,200, and 96,000 Hz are launch conformance inputs. Files at 176,400, 192,000, 352,800, and
384,000 Hz remain readable extended compatibility evidence only; they do not make those rates
launch engine, host, effect, or release support.

Run `cargo run --locked -p miso-engine-conformance --example miso_engine_conformance_fixtures -- --check`.
Only a maintainer deliberately updating the corpus may run `--write`, then review every checksum change.
