# Astra #511 C API numerical integration ruling — approved, bounded

Reviewed clean frozen `0f30be0a76f89994059e9ef435b8ddb6edcc152f` in `/home/bl/misofm/engine-slot-reservation`, the accepted #511 source contract, authentic `/tmp/511-immutable-workspace.{command.json,log,status}`, `crates/capi/tests/resource_lifecycle.rs`, the production replacement-cap addition and graph preparation/report paths. The workspace run is terminal with status 101; its first failing integration binary reports 3 passed/1 failed. The ordinary artifact builder is also terminal with its separately ruled status-1 hash mismatch. No pending immutable-run permission is inferred from elapsed time.

The failure is a stale independent test mirror of the newly accepted reservation, not evidence of another production defect. Approve the exact test-only arithmetic integration below in `crates/capi/tests/resource_lifecycle.rs`, with corresponding issue/evidence documentation. This does not reopen #511's three production implementation attempts or authorize a disguised fourth revision.

## Independent derivation

The existing scratch fixture is nine homogeneous soft-clip tracks on its frozen eight-lane native model. It prepares one full effect bank (floor(9/8)=1; the ninth effect is scalar) and two banks for each of the three builtin strip stages (3*ceil(9/8)=6). Therefore the charged membership count is N=1+6=7, irrespective of later adjacent builtin pairing.

The stage owner is a boxed trait object, independently represented by its data/vtable pointer pair: F=16 on this native target. A current BankSlot consists of that stage owner and a boxed bool slice (data pointer plus length), yielding B=32. Its eight-lane bool mask has W=8 with bool size 1. Thus:

- C=7*(16+3*32+3*8)=952 bytes per plan.
- L=max(7*16,7*32,8)=224 bytes for the named largest component request.
- Retained and conservative coexistence reservations must not be confused; this is the exact accepted C allowance, not a claim of 952 physically retained bytes.

The observed report agrees exactly: graph session-plus-plan and incremental totals are 227,148 rather than 226,196, and metadata is 51,247 rather than 50,295. Each difference is 952; all other fields in the full compared report are identical. L=224 is below the existing graph largest 49,167 and double-live maximum 58,804, so neither largest expectation moves.

The production replacement gate explicitly adds current and prospective graph session-plus-plan totals and both retained compiled models. Both plans have the same bank population; shortening the session ID affects canonical/model ownership, not this reservation. Therefore the existing double-live graph/model oracle increases by 2*C=1,904: 502,228 -> 504,132. This second correction is derived from the already observed per-plan component and inspected cap equation; execution has not yet reached that later assertion/admission, and no claim that it already passed is made.

## Exact allowed correction

Keep the existing independent primitive-owner style. A small private test helper may derive N from the fixture's nine tracks/eight lanes/three builtin stages and calculate C/L from primitive native layouts. Restate F as a two-pointer boxed trait-object footprint and B as F plus the boxed bool-slice footprint, or use one small test-local BankSlot field-list mirror with those same two owners. No dependency or Cargo change is needed. Do not invoke `GraphBankSlotResourceEstimate`, its fold, the production resource report or an observed difference as the oracle. Assert the frozen fixture's N/F/B/W and C/L values so this narrow native derivation remains explicit.

1. In `frozen_scratch_report`, add this independent C only to the three authorized graph fields, retaining their reviewed base literals or documenting the exact resulting literals above. Leave every other report field and the full-struct equality unchanged.
2. Append one positively charged row named as a runtime bank-slot coexistence reservation to `graph_owners()`, using the independent C. Append it after the existing rows so the existing `[5..13]` graph-metadata allocation authority retains its meaning. The existing clone into the prospective graph then charges C once for each plan automatically. Do not add another separate double-live charge.
3. Update only the two graph/model total expectations from 502,228 to `502_228 + 2*C` (504,132): `assert_effective_owner_mutations` inside `primitive_replacement_oracle` and the later `oracle.graph` assertion in the frozen exact test. The added row must participate in the existing omission and one-byte-miscount controls.
4. Preserve the existing eight cap rows and their exact/minus-one behavior, canary, atomic report, actual replacement/render and ownership destruction assertions. The graph row already consumes `oracle.graph` and will therefore exercise the derived 504,132/504,131 boundary without another literal cap change. Add a narrow assertion that independently derived L does not displace the existing graph-largest authority; keep 49,167 and 58,804 unchanged.

Source totals/overhead, effect state/scratch, builtin payload, C API retained storage, PCM, latency/tail, canonical document lengths/identities, all non-graph cap thresholds and existing allocator/lifecycle mechanisms remain frozen. No browser resource expectation is authorized by this ruling; any actual downstream mismatch must be separately observed and derived. Preserve the raw initial workspace failure unchanged.

## Objective gates and delivery

Run the existing exact test with nonzero selection in both profiles:

```
cargo test --locked -p capi --test resource_lifecycle external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps -- --exact
cargo test --locked --release -p capi --test resource_lifecycle external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps -- --exact
```

Each must execute one passing test, thereby reaching all eight cap rows and their existing negative controls. Run the existing complete `resource_lifecycle` integration target in both profiles (four tests each), strict affected C API all-targets Clippy, fmt/diff, and then resume the existing immutable delivery route on the identified integrated source. Capture exact source identity/commands/numeric statuses. No new test corpus or numerical sweep is required. A further discrepancy is a returned concrete finding, not authority for automatic additional repins.

The separate exact artifact-pin/current-consumer ruling remains valid; these test-only changes cannot justify any new artifact digest. Final integrated review, required CI and GitHub synchronization remain root's obligations. Read-only review: no builds/tests, timing, source/spec/Git/GitHub mutations performed. Only this requested temporary ruling was written.
