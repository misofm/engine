# 081 Canonical effect interchange qualification, fuzzing, and benchmark

## Outcome

Qualify the accepted descriptor, package/CID, state and migration products as one portable
interchange boundary without changing their bytes or APIs.

## Status and attempt budget

Stateless qualification successor after Issues 078 and 080. Permit one Terra attempt and one bounded
Sol correction; a second failure stops. Benchmark invocation count starts at zero.

## Scope and gates

Run the frozen independent-reference vectors in exactly 100 fresh processes; at least 10,000 seeded
deterministic mutations per descriptor/package/state parser; migration chain matrices; C/Rust record
agreement; allocation/canary/read-only audits; and native x86-64, AArch64 Android/iOS and
wasm32-unknown-unknown compile/conformance rows. Verify no parser/hash/selector/migration path is
render-reachable and publish exact corpus/tool/source hashes.

After every nonbenchmark gate passes, freeze one address-free representative descriptor/package/state/
migration workload and preflight persistence/no-clobber semantics without executing it. Sol may then
authorize exactly one descriptive invocation with one warmup and two measured rounds, no threshold,
tuning or retry.

## Non-goals

Byte/API changes, runtime DSP, third-party execution, repository/network/trust/signing, optimization,
subjective claims or release benchmark substitution.

## Dependencies by exact issue title

- Canonical effect package, CID, and artifact selection
- Effect state migration registry and bounded chains
- DSP research corpus and conformance harness
