# Replace exact resource-byte and record-hash pins with ceilings and live reports

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body.** **Owner ruling (2026-09-28, decision 4):** memory tests assert budgets (upper limits) **plus one independent completeness check** that every allocation is counted in the resource report, replacing exact byte counts. The completeness check must go red when an owner row is dropped (the verifier's scenario).

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 1). Base `a9414c0c`. Paths starting `../`
are relative to the audit's handoff folder. **Needs owner ruling R2:** are exact retained-byte counts
of fixtures a product requirement, or are budgets ceilings? The ruling also sets the headroom.

## Problem

Several gates assert an exact byte count, or a hash of a record whose only variable content is byte
counts or constants. Every intended layout change turns them red, and the fix is always a re-pin.

- From 2026-09-01 to 09-28, the pins in this issue caused **16 red CI jobs**
  (`../data/ci-red-jobs.tsv`): `expected.json` rows 6, graph audit hash 4, capi totals 4,
  `layout_total_bytes` 2. None exposed an unintended change.
- The values moved often:
  - the `expected.json` rows 24 times;
  - the capi totals up to 18 times;
  - the graph audit hash 5-8 times;
  - `layout_total_bytes` 3 times, and CI went red twice when one of its copies was missed.

The pins in scope:

1. **`hosts/host-web/tests/browser-v1/expected.json`:** the 21 wasm32 resource byte rows. Also the
   row-equality check and the native witness classification in
   `scripts/check-browser-expected-resources.py`: `check_native_witness` at `:254`, and the native
   `cargo run --release … browser_fixture_resources` build at `:410-420`, about 60 s per PR.
2. **The Issue-544 inline validator,** `.github/workflows/qualification.yml:671`: `layout_entries: 16`
   and `layout_total_bytes: 6472`. Also the Rust copies in `tools/audit/src/source_duration.rs:317-337`:
   `16`, `6_472`, and fnv `0xde96_92fb_4797_b2a5` of the accounting text.
3. **`scripts/trace-builtins-graph-audit.sh:65-69`:** `expected_audit_hash`.
4. **`crates/capi/tests/resource_lifecycle.rs`:**
   - the hand-maintained layout mirror, about 1,850 lines: the `*Mirror` structs at `:605-1287` and
     the owner-row functions at `:1288-2472`;
   - the literal totals it drives, for example `178_514 + …` at `:2804`.

## Outcome

**Budgets become ceilings.** A new test per host checks every retained resource row against its
ceiling: `hosts/host-web/tests/retained_ceilings.rs` for the browser identity fixture, and a capi
equivalent. The ceilings are today's values plus the headroom R2 sets, for example 10 %. The test
checks at the ceiling and one byte over, by lowering the configured cap. Today **no test goes red when
a retained row grows**. The existing budget tests derive their budgets from the live report, so
the ceilings are new coverage, not a rename.

**The exact-report tests stay, driven by live reports instead of mirrors.**
- `crates/capi/tests/resource_lifecycle.rs:2651`,
  `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`, admits the exact total
  and rejects one byte below, across 8 cap rows. It catches a dropped owner row, which a ceiling
  cannot. It keeps its checks, with the expected totals taken from two live reports rather than the
  mirror.
- `:2785` keeps its exact admission at the live report's `capi_retained_bytes` and loses the
  literal.

**Kept unchanged:**
- `:2571` `render_diagnostic_egress_reuses_eager_capi_storage_without_allocation`, the only
  zero-allocation check of C-ABI render plus `dequeue_event`;
- the helpers `compile_c`, `resources_c` and `compile_rejected_c` (`:2473-2569`);
- `:391` (allocation balance).

**Kept real claims elsewhere:**
- the native-against-simd128 PCM digest parity in `check-browser-expected-resources.py`;
- the Issue-544 duration-independence flags and frame/file-size constants;
- the audit's zero allocation, lock and syscall counts (`trace-builtins-graph-audit.sh:60-64`).

## Scope

Authorized paths:
- `hosts/host-web/tests/browser-v1/expected.json` and one new `hosts/host-web/tests/` file;
- `scripts/check-browser-expected-resources.py`;
- `.github/workflows/qualification.yml` (the inline validator only);
- `tools/audit/src/source_duration.rs`;
- `scripts/trace-builtins-graph-audit.sh`;
- `crates/capi/tests/resource_lifecycle.rs` and one new `crates/capi/tests/` file;
- this issue's spec.

## Gates

1. **The ceilings discriminate.** In a scratch copy, grow a retained structure counted in the
   browser fixture and in capi (for example one more `u64` in a per-track owner):
   - past its ceiling: the new ceiling test goes red;
   - within its ceiling: it stays green, which is the intended change of behaviour.
2. **A dropped owner row is still caught.** Remove one owner from the capi retained-bytes accounting
   in a scratch copy. `:2651` still goes red.
3. **The real claims keep failing:**
   - `check-browser-expected-resources.py --self-test`'s PCM-digest mutation still fails;
   - `tools/audit` still exits non-zero when the 1-minute and 3-hour layouts differ (seed a
     duration-dependent entry);
   - the graph audit still fails on one allocation during render.
4. **Mutation equivalence on the accounting code,** before and after, with the same arguments.
   The caught set may shrink only by mutants whose sole catcher was an exact literal and whose only
   effect is a retained-size change within the ceiling. List each such mutant in the PR. The runs:
   `../tools/run-mutants.sh host-core 5 10 --features control-provider,test-support --shard 0/4
   --sharding round-robin --test-package capi --test-package host-web`, and the same for
   `graph-compiler --file crates/graph-compiler/src/estimate.rs`.
5. **Historical bugs.** `../tools/revert.py` for #966, #970, #994 and #1015. The red set is
   unchanged, and none of the tests touched here is a reproducer.
6. **Cost.** The artifact-gates step no longer builds `host-web --example browser_fixture_resources`
   natively. Record the step time before and after.

## Saving and risk

- **Saving:** about 60 s of runner time per full PR. About 50 re-pin edits a month go, and about
  1,850 lines of hand-mirrored layout.
- **Risk:** growth of up to the ceiling's headroom goes unseen per PR. The owner sets the headroom.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **The outcome contradicts gate 2 (finding F2), so rewrite it.**
   - **What I did.** In a scratch tree I dropped `checked_layout::<crate::Plan>(1)` from
     `fixed_allocation_rows` (`crates/capi/src/runtime/compile.rs`). That under-reports 416 bytes.
   - **What caught it today.** `:2651` went red twice over:
     - on the frozen literal report at `:2732`;
     - with that literal removed, on the mirror-driven "capi one-below" admission.
   - **What the draft's form missed.** `:2785`, rewritten as this draft proposes (its total taken from
     the live report), stayed **green**.
   - **Why.** A total taken from the live report shrinks together with the dropped row. After the
     outcome as written, nothing catches an under-count.
2. **Keep one independent completeness oracle.** Drop only the literal totals.
   - Preferred: an allocator-observed oracle. This file already installs a counting global
     allocator (`record_allocation`, `:41`). After compile, the bytes allocated and still live for
     the plan and the session must equal the reported `capi_retained_bytes`, or be bounded by it if
     exactness proves infeasible.
   - Otherwise keep the mirror and delete only the literals.
   - `:2785` also compares against the independent oracle, never against the report it tests.
3. **Ceilings do not replace completeness.** Ceilings guard growth budgets only. A ceiling never sees
   a decrease: an owner row dropped, or a `size_of` change the accounting forgot. Such an
   under-count lets admission accept a session above the host's configured memory cap, which is an
   OOM on a phone.
4. **Gate 2 is kept verbatim, and made concrete.**
   - The mutation above turns the new oracle test red.
   - So does adding a new boxed owner to `crate::Plan` that the accounting does not count.
5. **Item 2 is out of scope.** #1035 deletes the Issue-544 validator and `source_duration.rs`'s
   source-duration subject. Drop item 2 and those paths.
6. **Sequence.** #1024, #1033 and #1035 all edit the capi layout mirror. Land this draft after them.
7. **Gate 4.** Before the mutant runs, run the `--test-package` set (capi, host-web) once unmutated,
   with the same environment, and require it green. `cargo mutants` 27.1 baselines only the mutated
   package (F4).
8. **R2, as the verification recommends.** Budgets are ceilings; accounting completeness stays exact
   against an independent live oracle. Neither is a literal pin.

## Attempt 1 evidence

Terra, attempt 1, branch `codex/1060-ceilings-not-byte-pins` from `a509b681`. Code diff outside this
spec: **+1,134 / −2,541 lines** (capi `resource_lifecycle.rs` +641 / −2,098; the native witness
example −93).

### What changed

- **capi completeness oracle (amendments 1–4, 8).**
  `capi_retained_bytes_charge_every_byte_the_compile_retains` uses the file's counting allocator. It
  observes a C ABI compile, then a replay of its host-core half (`parse_host_session` →
  `SessionStore` → `prepare_host_runtime`, the same calls capi makes before it allocates anything
  of its own). The rule: compile-live − host-live == `capi_retained_bytes` − capi's charges for
  host-allocated storage. Those charges are the canonical JSON, `control_retained_bytes` and the
  provider's catalog charge, each read from its owning crate's resource report. The rule holds to the
  byte on the EQ and soft-clip nine-track fixtures, at x86-64 and AArch64, in dev and release, and
  20/20 runs under the parallel harness in each profile. The one derived difference is the
  decode-field `u16` rounding of an odd frame: an over-charge of 1 byte, computed from the limits.
  - The source control table is exact: freed bytes == `control_retained_bytes`.
  - The engine rows are a bound: plan live ≤ engine rows, and store live ≤ model estimate +
    canonical JSON. They are not exact, because the graph estimate over-states the bound plan by
    about 128 KB by design.
- **The mirror is gone.** That is every `*Mirror` struct, the owner-row functions,
  `frozen_scratch_report`, `primitive_replacement_oracle` and every literal total. The
  double-live test, renamed `double_live_oracle_drives_exact_and_one_below_c_caps`, takes its
  requirements from live values:
  - two live reports: the current plan before the swap and the prospective plan after it;
  - both `CompiledSession` estimates;
  - the prospective epoch plus the prepared-protocol owner, from `ReplayCache` and
    `SessionControlProvider` resource reports.

  On x86 these reproduce the old mirror exactly: graph 524,172, source 24,252 / 7,868, effect
  15,120 / 432, builtin 34,902, capi 204,311, largest 58,804. On AArch64 (qemu) graph is 510,900.
  The `#[ignore]` off x86 is removed.
- **The tiny-frame test** now checks its charge against the allocator oracle, not a literal.
- **Budgets.**
  - `reference_session_retained_rows_stay_within_their_budgets` checks 19 rows plus the model
    estimate of the EQ fixture. Each cap is admitted at its budget and refused one byte below the
    requirement.
  - `hosts/host-web/tests/retained_ceilings.rs` checks 12 rows of the browser fixture natively,
    and the `maximum_memory_bytes` admission at the budget and at exact − 1.
  - `expected.json` has `resourceCeilings` for the 8 wasm32 layout rows; its exact rows stay.
- **Browser gate.** `check-browser-expected-resources.py` checks exact rows for equality and
  budgeted rows for 0 < v ≤ ceiling, and prints each row's headroom. The native witness and its
  classification are gone (`examples/browser_fixture_resources.rs` is deleted; nothing else used
  it). The self-test is green at every ceiling and has 32 red mutations, including one byte over
  each ceiling, a zeroed row and each of the 3 PCM digests.
- **Graph audit.** `expected_audit_hash` is removed. The hash is still printed in the PASS line;
  the audit's own PCM and meter comparison and the jq claims are unchanged.

**Headroom rule.** Every budget is the row's value on 2026-09-28 at that lane width, plus 10 %,
rounded up to 64 bytes. For the two bridge rows the 10 % applies above the fixed 1 MiB
live-response capture. A zero budget is a claim: this fixture has no delay, no scalar-scratch effect
and no meter.

**Pins replaced.**

| pin (old) | now |
|---|---|
| capi `frozen_scratch_report` (graph 231,060+…, bank rows, source 3,934/12,126, builtin 17,451, largest 49,167, capi 160,893) | live report equality under each exact cap; EQ-fixture budgets: graph 261,248 / 253,952 (x86 / A64), metadata 61,696 / 62,528, bank 15,680 / 21,056, bank scratch 54,080 / 40,576, effect-bank 9,024 / 9,024 / 704 / 832, source 9,024 / 4,352 / 13,376, effect state 9,280, builtin 19,200 ×2, capi 300,800, largest 99,840, model 25,344, zero rows 0 |
| double-live 511,956+…, 24,252, 7,868, 15,120, 432, 34,902, 204,311, 58,804 | live-derived requirements (above) |
| mirror totals 160,893 / 18,706 / 24,712, EQ catalog 108 / 18,144 / 1,296 / 1,908, executor 328 / 1,312, response owner 24 / 1,908, observation 240 / 256, slot 952 / 224, canonical 18,453 / 18,444 | the allocator oracle; the canonical lengths become the rename relation |
| tiny-frame 178,426 + EQ growth | allocator oracle |
| `expected.json` bridgeMetadata 1,149,255, bridgeRetained 1,169,764, sourceTotal 3,286, sourceOverhead 2,262, builtinRetained 1,817, graph ×2 32,402, graphMetadata 4,131 | ceilings 1,159,360, 1,181,888, 3,648, 2,496, 2,048, 35,648, 4,608 |
| native witness rows | `retained_ceilings.rs`: bridge 1 MiB + 111,808 / + 134,400, source 3,840 / 2,688, builtin 2,176, graph 52,864 / 38,784, metadata 7,424 / 7,360 |
| graph audit `dbac3f3d…` | live report (the record is still `dbac3f3d…`) |

### Gates

1. **Ceilings discriminate.** The plant is `StripPreparation` (per track) gaining `[u64; N]`.
   - N = 1 (within the ceiling): green in capi, the native browser test and the wasm32 gate
     (builtinRetained 1,825 of 2,048).
   - N = 256 (past it): red in capi (builtin processor/retained 35,883 > 19,200), native browser
     (4,005 > 2,176) and wasm32 (3,865 > 2,048).
2. **Dropped row.** Removing `checked_layout::<crate::Plan>(1)` turns the oracle red (131,378 vs
   130,978) and so does tiny-frame. An uncharged `Box<[u64; 32]>` added to `Plan` is red too
   (131,642 vs 131,386). As amendment 1 predicted, the live-report double-live test stays green.
3. **Real claims.**
   - The self-test's three PCM-digest mutations are red.
   - `test-realtime-audit-probes.sh builtins-graph` passes (9 operations). This harness injects
     allocations and other probes into the audit and requires the audit to fail on each one.
   - The graph trace passes.
   - The Issue-544 item is N/A (amendment 5).
4. **Mutation equivalence.** The unmutated capi+host-web set is green at opt-level 1 in both trees.
   - As written, `--features control-provider,test-support` makes all 359 host-core mutants
     unviable in both trees. cargo-mutants 27.1 tests only the `--test-package` packages, and
     neither declares the feature. Rerun as `host-core/control-provider,host-core/test-support`:
     137 → 138 caught. None is lost; `prepare.rs:1279 > → ==` is newly caught by the budget test.
   - `estimate.rs` (15 mutants): 6 → 4 caught by capi+host-web. The two lost are
     `graph_metadata_bytes → Some(0) / Some(1)`, whose sole capi catcher was the literal report.
     graph-compiler's own tests still catch both (3 tests each), so the overall caught set is
     unchanged. They are graph-estimate under-counts that capi's engine bound is too loose to see.
5. **Historical bugs.** revert.py for #966, #970, #994 and #1015. `resource_lifecycle` (base and
   after) and `retained_ceilings` (after) stay green under each one, so no touched test is a
   reproducer, and no other test differs between the trees.
6. **Cost.** The step builds nothing native now. `check-browser-expected-resources.py
   --artifacts` took 85.6 s before on a cold target (0.55 s warm; load average 70) and takes 0.36 s
   after.

Also green:
- `cargo check --workspace --all-targets --all-features`, workspace clippy `-D warnings`, fmt;
- capi and host-web tests in dev, dev with `host-web/test-support`, and release (252 passed, 0
  failed, 3 ignored);
- `console-workload` release (the console digests);
- `check-web-audioworklet.sh`, `test-web-audioworklet.sh`;
- `check-capi-abi.sh` and its `--self-test`;
- the routing check and test;
- every lint-job policy pair, run with `python3 -B`.

On AArch64:
- the `run-aarch64-tests.sh debug` package and feature set resolves with `cargo test --no-run` on
  x86;
- `resource_lifecycle` (8 passed, 1 ignored) and `retained_ceilings` pass under qemu-user, using a
  scratch rust-std sysroot and not rustup.

Real arm64 CI verifies the rest.

The shipped module is unchanged: `01dd58be…` both at base and after.

`--list` diff:
- `resource_lifecycle`:
  - removed `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`;
  - added `double_live_oracle_drives_exact_and_one_below_c_caps`,
    `capi_retained_bytes_charge_every_byte_the_compile_retains`,
    `reference_session_retained_rows_stay_within_their_budgets` and
    `prepared_parameter_catalog_charge_covers_its_allocations` (ignored);
  - 6 → 9 tests.
- host-web: added `browser_identity_fixture_retained_rows_stay_within_their_budgets`.

### Findings for follow-up (the oracle found two under-counts; production code is outside this issue's paths)

1. **The catalog under-counts its enum choices.** `build_parameter_catalog` collects the choices
   through `Result<Vec<_>>`, so the vector keeps capacity 8 for the EQ's 6 kinds.
   `resource_report` charges by length. On the EQ fixture that is 72 × 2 × 32 = 4,608 retained bytes
   `capi_retained_bytes` does not charge. `prepared_parameter_catalog_charge_covers_its_allocations`
   reproduces it; it is ignored with that reason.
2. **The compiled-model estimate under-states `CompiledSession`'s live bytes.**
   - On the EQ fixture the session is 38,069 live against an estimate of 23,039. capi's total
     charge (estimate + canonical JSON, counted twice) still covers it.
   - host-web charges the estimate once. It under-counts by 127 bytes on the identity fixture, and
     about 15 KB on the EQ fixture.

### Deviations and notes

- **Placement.** The capi budgets live in `resource_lifecycle.rs`, not in a new file. That file is
  already on the `unsafe` allowlist and owns the allocator and C helpers.
- **Files outside the authorized list.** The example is deleted (scope item 1). One line each is
  synced in `docs/TARGET_MATRIX.md`, `docs/C_ABI_V1_QUALIFICATION.md` and
  `hosts/host-web/MUTATIONS.md`.
- **Manual modes that now fail.** The superseded WebDriver harness's run and `--check` modes, and
  `direct-oracle.mjs`'s non-print `deepEqual`, compare the old whole resource block, so they no
  longer match. CI runs neither; #1050 retires the harness.
- **Pins still in place.** The `memoryBytes` status pins (1,376,256 / 1,441,792) are outside the
  21 rows and unchanged.
