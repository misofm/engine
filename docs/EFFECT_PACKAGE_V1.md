# Effect package V1

Packages are a canonical binary stream, never an archive. Each source, core-Wasm, or target-native
artifact carries a SHA-256 of exact content. CIDv1 uses raw codec and full SHA2-256 multihash over
the canonical package stream; its canonical text is lowercase unpadded base32 with `b` multibase.

A CID identifies bytes only. It does not assert trust, license status, safety, quality, or
cross-backend bit-identical audio. Package resolution, signatures, and execution are external.
