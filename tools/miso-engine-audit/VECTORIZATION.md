# Native vectorization report — operating notes

The contract, the linkage argument and its limits are in
[`docs/NATIVE_VECTORIZATION_V1.md`](../../docs/NATIVE_VECTORIZATION_V1.md). This file is how to run
it, what the registries mean, and what would have to be true to promote it from a report to a gate.

## Running it

```sh
bash scripts/run-native-vectorization-report.sh                 # writes target/ci/native-vectorization/report.json
bash scripts/test-native-vectorization-report.sh                # 15 red mutations, rebuilds its binary
```

The runner builds three things from this tree and hands their paths to the subject:

1. the probe crate for the host x86-64-v3 backend, emitting fresh LLVM IR and a fresh object;
2. the same crate for AArch64, guarded on the cross target's standard library being installed;
3. `libmiso_engine_capi.so` and `libmiso_engine_host_web.so` under the release profile as it ships.

Exit status is `0` for a clean report, `1` for a red one, `2` for a usage or environment error. The
JSON report is written either way; the failures array names every rule that fired.

## The three registries

| File | One row per | Fails when |
| --- | --- | --- |
| `vectorization-families.tsv` | public item in the lane kernel modules | it and the parsed lane sources differ in either direction, or an `exempt` row has no reason |
| `vectorization-allowlist.tsv` | (backend, certified family) | a certified family lacks a rule at a backend, or a rule names a family that is not certified |
| `vectorization-shipped.tsv` | (product, rule, symbol) | a shipped symbol is missing, duplicated, below its floor, or not vector dominated |

**Adding a kernel family** is therefore three edits: a probe in
`tools/miso-engine-vectorization-probes/src/lib.rs` and a call to it from `run_all`, a row in the
families registry, and one row per backend in the allowlist. Until then the report is red, which is
the point: the completeness check is what stops a new bank shipping uncertified.

## Red mutations

See [`VECTORIZATION_MUTATIONS.md`](VECTORIZATION_MUTATIONS.md). Two properties of the suite are
deliberate:

- **It rebuilds the binary, and asserts that the rebuilt binary carries the subcommand.** This suite
  has twice in this repository's history been run against a stale `target/release/miso_engine_audit`
  that predated the subcommand under test; it then "passed" by refusing an argument it did not
  understand. The rebuild and the subcommand assertion are the fix.
- **Each mutation must fail for *its own* reason.** A mutation that goes red for an unrelated reason
  proves nothing about the rule it names, so every case greps for the specific message.

## Promotion criteria: report to gate

The job is `continue-on-error: true` and uploads its receipt. It should stay that way until all of
the following hold; each is a real reason, not ceremony.

1. **A false-positive record across compiler bumps.** The instruction-level rules are the only ones
   whose truth depends on the optimizer's choices rather than on our source. Two clean rebuilds on
   one toolchain (recorded) is a floor, not a record. The gate should be promoted after it has
   survived one *toolchain* change without a red run that turned out to be drift.
2. **The kernel-host registry derived, not curated.** Today seven production symbols per product are
   named by hand. Until the roster of shipped banks is derived from the effect registry the way the
   family roster is derived from the lane sources, a new bank can ship uncovered, and a gate would be
   claiming coverage it does not have.
3. **The AArch64 leg exercised in the CI image.** The skip is honest, but a gate whose second backend
   is routinely skipped is a one-backend gate with extra words. Promotion requires the AArch64 target
   standard library present in the job.
4. **The floors reviewed once against a deliberate regression.** The `scalar-bank` mutation proves
   the floor rule fires; it does not prove the floors are set where a *real* regression would cross
   them. One deliberate de-vectorization of one bank, measured, would settle it.

Until then it is what its `status` field says: evidence, gathered from bytes, with its limits
written down.
