# 078 Canonical effect package, CID, and artifact selection

## Outcome

Bind accepted canonical descriptor bytes to a deterministic non-archive package stream, exact source/
core-Wasm/target-native artifacts, CIDv1 identity and deterministic verified artifact selection.

## Status and attempt budget

Stateless successor after accepted Issue 082. Permit one Terra attempt and one bounded Sol
correction; a second failure stops. Workload/benchmark/timed counts remain zero.

## Scope and gates

Replace the provisional package/CID modules with a Sol-frozen header/table/content byte layout,
canonical lowercase relative paths/targets/sorted feature tokens, exact per-artifact SHA-256,
strict CIDv1 raw codec `0x55` plus SHA2-256 multihash `0x12 0x20`, lowercase base32 text and a single
deterministic most-specific compatible artifact-selection order. Descriptor bytes must verify under
**Close canonical effect descriptor wire, identity, and C inspection ABI** before package
acceptance. Bounded size-query/encode/borrowed-verify/select APIs use caller storage and exact
diagnostics; insufficient output is all-or-none.

Golden package/CID vectors from an independent stdlib reference cover source, core Wasm and native
artifacts. Every descriptor/manifest/content mutation either rejects or produces the frozen changed
CID; selection recomputes content hashes before returning. Native and Wasm nonexecuting gates pass.

## Non-goals

State, migration, signatures, trust, licensing, repository/network resolution, installation,
execution, broad fuzz/target qualification, benchmark or timing.

## Dependencies by exact issue title

- Close canonical effect descriptor wire, identity, and C inspection ABI
- DSP research corpus and conformance harness
- Native effect runtime contract and conformance

## References

- [CIDv1 specification](https://specs.ipfs.tech/cid/)
