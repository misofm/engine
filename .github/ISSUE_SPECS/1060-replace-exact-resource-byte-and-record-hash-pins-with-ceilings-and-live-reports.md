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

## Sol verdict, attempt 1

**FAIL.** The capi-owned half of the completeness oracle is exact and strong, and the ceilings
discriminate. But the oracle takes two of the charges it should check from the accounting under test,
and both hide real under-counts that have no owner. The branch also does not compile `-p capi` on
the batch head.

Reviewer: Sol, 2026-09-29. I merged `a568b121` into a scratch detached checkout of
`codex/batch-slim-3` at `1c3b0531` (main plus #1029, #1030, #1031, #1033, #1039, #1059, #1061). No
timed workload was run.

### Findings, by severity

1. **HIGH: the oracle trusts two host-allocated charges, and both hide under-counts with no owner
   (amendment 2: "never against the report it tests").** `HostHalf::capi_charges()` subtracts
   two figures from the accounting instead of the bytes the owners hold:
   - `catalog_charge()`, the provider's own `resource_report`, which is the number capi adds into
     `capi_retained_bytes`;
   - `canonical_bytes()`, the `len` capi charges.

   Neither term is observed. The two under-counts:
   - **Parameter catalog.** It holds 129,708 bytes and is charged 125,100: 4,608 bytes are uncharged in
     capi's own row on the EQ fixture, on x86 and on AArch64 under qemu. The only test that sees this
     is `#[ignore]`d, and its reason says "host-core follow-up to #1060". No issue number or
     spec exists for it (`gh issue list` and `.github/ISSUE_SPECS/`).
   - **Canonical JSON.** `session::canonical::write_canonical` grows a `String` and never
     shrinks it. The EQ canonical is 16,712 bytes in a 32,768-byte allocation, so 16,056 bytes are
     uncharged. Soft-clip leaves 14,315 uncharged and the browser identity fixture 129. This is the
     evidence's "compiled-model estimate under-count", but the estimate itself is right. With
     `shrink_to_fit()` added in `write_canonical`, the observed store fits its estimate: 22,013 ≤
     23,039 (EQ) and 2,867 ≤ 2,869 (identity).
     - capi stays covered only because it charges that one allocation twice: in the graph cap
       through `compiled_model_bytes`, and in its epoch row.
     - host-web charges it once, so the browser under-counts by up to about the canonical length.
       For the 290 KB console fixture that is on the order of 200 KB.
     - No test documents this, and no issue owns it.

   **Fix:**
   - The oracle observes the catalog and canonical allocations (`freed_by_drop`, as it already does
     for the source control table) and compares them with their charges.
   - Each under-count is then either fixed under a scope amendment, or carried as a named, derived
     expected delta that cites its own issue number (like the decode-field rounding). The fixes are
     small: in host-core, shrink the enum-choice vectors or charge their capacity; in session,
     `shrink_to_fit` or `into_boxed_str`.
   - Root files the issues.
2. **HIGH (merge): semantic conflict with #1059.** `Budget::ceiling` matches `Backend::Scalar`
   exhaustively, but #1059 gates that variant behind `lane/test-support` and forbids exhaustive
   matches outside `lane`.
   - On the merge, `cargo test --locked -p capi` fails with E0599. So does gate 4's corrected
     command (`--test-package capi --test-package host-web` with the host-core features): its test
     build has no `lane/test-support`.
   - CI's `test-debug-a`, `aarch64-debug` and `--all-features` legs compile it anyway, because
     feature unification enables the variant (`cargo tree -e features -i lane`).
   - **Fix:** `match Backend::current().width() { 8 => …, 4 => …, _ => panic!(…) }`. Everything
     below was run with that one-line change.
3. **MEDIUM: the engine-plan half is a loose bound, and the graph runtime has no allocator
   oracle.**
   - **Slack** (engine rows minus the observed plan): 128,317 bytes on EQ (the plan holds 138,973
     against 267,290 charged), 128,834 on soft-clip and 12,233 on the browser identity fixture.
   - **Plant, uncharged table.** An uncharged `Box<[u64]>` of 16 words per unit in graph `Runtime`
     (1,808 bytes) left every capi and host-web test green. At 2,048 words per unit (229,392 bytes)
     the oracle went red.
   - **Plant, dropped row.** `estimate.rs` `graph_metadata_bytes → Some(0)` drops a 56 KB row. After
     #1060 no capi or host-web test sees it. Only graph-compiler's own fixture and digest pins do
     (3 tests).
   - **Builtins is covered.** A dropped builtin vector charge was caught only by builtins-compiler's
     own allocator oracle (`tests/allocation_tracker.rs`).
   - This is not a regression: base only re-pinned `size_of` moves, and amendment 2 allows a bound.
     But decision 4's "every allocation is counted" does not hold for the prepared plan.
   - **Needs:** a tracked successor (a graph-runtime allocator oracle, or per-owner attribution),
     cited in the spec and in the oracle's doc. The browser identity fixture should join the oracle
     loop now.
4. **LOW:** `direct-oracle.mjs`'s non-print `deepEqual`, and the WebDriver harness's run and
   `--check` modes, now always fail. This is disclosed. #1050 retires the harness, but
   `direct-oracle.mjs` stays, so its non-print path needs removing or teaching the ceilings.
5. **LOW:** the only stated reason for 10 % headroom is that the brief gives it as an example. The
   history supports it: past moves were +1.2 %, +2.3 %, +11 % and +48 %. Cite that, or get an owner
   figure.

### What holds

- **Merge.** It is textually clean: `docs/TARGET_MATRIX.md` auto-merged and reads correctly. The
  only semantic conflict is finding 2. The retained rows on the merge equal the base values at every
  width, so #1059 moved none.
- **Completeness of capi-owned storage is exact.**
  - Dropping the `crate::Plan` row is red: 131,378 against 130,978, and tiny-frame 57,700 against
    57,300.
  - An 8-byte over-charge is red: 131,378 against 131,386. This is the right result, because the
    check is an equality between two measurements.
  - The source control table is exact.
  - The counting allocator is thread-local and armed only around the observed window. Compile
    spawns no thread (every `spawn` in the path is test-only), so harness allocations can neither
    mask nor add.
- **The ceilings discriminate.**
  - capi: doubling the telemetry tables (configuration items counted per byte instead of per
    `u16`) is red, 338,988 > 300,800, while the oracle stays green, as it should for charged
    growth.
  - browser: `MAXIMUM_COMMAND_RECORDS` 256 → 512 is red, bridge metadata 1,238,279 > 1,160,384 and
    bridge retained 1,258,788 > 1,182,976.
  - The self-test catches all 32 of its mutations.
- **Per-width rows.**
  - Simd8 (x86): the values match the table.
  - Simd4 (AArch64 under qemu-user, rust-std sysroot in scratch): capi 8 passed, host-web 1 passed,
    and the values equal the table. The double-live graph requirement is 510,900.
  - wasm32 `simd128`: all 8 rows are within their ceilings.
- **Double-live.** The live requirements reproduce the old totals: 524,172, 24,252, 7,868, 15,120,
  432, 34,902, 204,311 and 58,804.
- **Gate 4, re-run with corrected arguments.**
  - `estimate.rs`: 6 caught → 4. The two lost are `graph_metadata_bytes → Some(0)` and `Some(1)`,
    both still caught by graph-compiler (finding 3).
  - host-core, 36 accounting mutants (`resource_report`, `bytes`, the control table, the ID arena,
    `retained_bytes`, the factory bytes): caught 12 → 12, the identical set.
  - capi `runtime/compile.rs`, 78 mutants: 56 → 58. None is lost, and two are new.
- **Audit hash.** It still prints `dbac3f3d…`. Nothing live asserts it; the only mentions are
  historical specs. The fields of the record that jq does not check are constants, and the
  `accepted_*_sha256` strings are never compared with the fixtures. The audit's own PCM and meter
  comparison stays, and the probe mutations pass (9 operations). The ruling permits removing the
  hash.
- **Gates on the merge:**
  - fmt; clippy `--workspace --all-targets --all-features -D warnings`; `cargo check` with all
    features;
  - capi and host-web tests in dev (with and without `host-web/test-support`), under
    `test-debug-a`'s features and in release;
  - `check-browser-expected-resources.py --artifacts` and its `--self-test`;
    `check-web-audioworklet.sh`; `test-web-audioworklet.sh`; `check-scalar-oracle-absent.py --wasm`
    (module `a4a2383f…`);
  - `check-capi-abi.sh` and its `--self-test`;
  - `console-workload` in release; `trace-builtins-graph-audit.sh`;
  - all 54 lint and routing policy commands, run with `python3 -B`;
  - the x86 `cargo test --no-run` of the `run-aarch64-tests.sh debug` package and feature set.

**Attempt 2 passes when:**
- finding 2 is fixed;
- the oracle observes the catalog and canonical allocations;
- each of the three gaps (catalog, canonical and graph runtime) is either fixed under a scope
  amendment or tracked by an issue number cited in the test and in this spec.

## Attempt 2 evidence

Terra, attempt 2. `codex/batch-slim-3` (`3ba8982a`: #1059, #1039, #1043, #1049) is merged first
(`79564472`). Code diff against `3ba8982a`, outside this spec: **+1,268 / −2,555 lines**. Root
extended the paths to host-core and session for finding 1.

### Finding 1 (HIGH): the oracle observes every owner, and both hidden under-counts are fixed

`resource_lifecycle`'s oracle now takes no charge from the accounting it checks. It replays the
compile's host-core half and observes each owner by dropping it. The drop order is the attribution:
the source producers go first, so a ring they share with the plan is counted with the plan. It
also asserts that the four owners (producers, catalog, plan, store) hold every byte the half
retains. Then:

- `capi_retained_bytes` == capi's own allocations + observed producers + observed catalog (+ the
  derived decode-field rounding), to the byte;
- the source producers == `control_retained_bytes`, exactly;
- the session store ≤ `compiled_model_bytes` (the graph cap's model row);
- the plan ≤ its engine rows (a bound; see finding 3).

The browser identity fixture joins the loop, beside the EQ and soft-clip fixtures.

**Red before, green after** (x86, dev). Each defect is re-injected alone into the final tree.

| defect | red on | observed vs charged |
|---|---|---|
| catalog enum capacity (the `Result<Vec>` collect) | oracle, tiny-frame, catalog test | 261,348 vs 256,740; 187,670 vs 183,062; 129,708 vs 125,100 |
| canonical spare capacity (no shrink) | oracle's store check | store 38,069 > estimate 23,039 |
| canonical double charge put back in capi's epoch row | oracle, tiny-frame, double-live | 256,740 vs 273,452; 183,062 vs 199,774 |
| `checked_layout::<Plan>(1)` dropped | oracle, tiny-frame | 256,740 vs 256,340 |
| uncharged `Box<[u64; 32]>` in `Plan` | oracle, tiny-frame | 257,004 vs 256,748 |

With the fixes, all green at both widths: EQ store 22,013 ≤ 23,039, soft-clip 23,901 ≤ 24,927,
identity 2,867 ≤ 2,869. The store's slack is the estimate's 128 bytes per entity for indexes; the
compiled session builds one index, over its sources.

**Fixes, and why each form.**
- **Catalog.** host-core's `build_parameter_catalog` reserves each descriptor's enum-choice vector
  exactly, then pushes, as every other catalog allocation already does. I chose that over charging
  capacity: the growth `collect` picks is not a contract, and the spare memory is useless. The
  catalog test is un-ignored and passes.
- **Canonical JSON.** session's `write_canonical` calls `shrink_to_fit` before it returns: one
  control-plane `realloc`. I chose that over charging capacity: the doubling writer leaves up to
  half its allocation spare, and charging it would make phones reserve it. The shrink sits in the
  writer rather than in `compile_session` because `check-session-policy.sh` anchors on the exact
  line `let canonical_json = write_canonical(session)`.
- **capi's double charge is removed.** The epoch row charged the canonical JSON, and the compiled
  model's graph-cap charge already includes it.

**Ceilings the fixes move.**
- capi `capi_retained_bytes` on the EQ fixture: 273,452 → 256,740 at both widths (−16,712, the
  canonical length). Its budget is re-derived at value + 10 %, rounded to 64: 300,800 → 282,432.
- The double-live capi requirement: 204,311 → 167,414 (−18,453 current and −18,444 prospective
  canonical). It is derived, not pinned.
- No other row moves:
  - host-web charges the length-based estimate, so the shrink changes only real memory;
  - the native browser and all 8 wasm32 rows are unchanged, and `expected.json` is untouched.

### Finding 2 (HIGH): the #1059 conflict

`Budget::ceiling` matches on `Backend::current().width()`: 8 or 4, anything else panics. There is
no scalar variant and no `cfg(target_arch)`. `retained_ceilings.rs` does the same, so `lane`
joins host-web's dev-dependencies (one `Cargo.lock` line). `cargo test --locked -p capi` compiles
on the merged tree.

### Finding 3 (MEDIUM): the plan bound. It is justified, not tightened; it does not catch the table

- **The store now joins the oracle and is observed.** The prepared plan stays a bound, because its
  rows are the graph compiler's admission estimate of the preparation: the compile-time graph
  metadata, bank scratch per slot where a merged chain keeps one slot's, and #511's and #936's
  reservations at their bound.
- **Measured slack** (eight lanes / four):

  | fixture | slack (bytes) |
  |---|---|
  | EQ | 128,317 / 128,405 |
  | soft-clip | 128,834 / 128,730 |
  | browser identity | 12,233 / 8,109 |

- **Sol's plant.** An uncharged `Box<[u64]>` in graph `Runtime`:
  - at 16 words per unit (1,808 bytes on EQ), every test stays green. **The observing check does
    not catch it.**
  - at 2,048 words per unit it is red: 368,365 > 267,322.
- **Why it cannot be caught here.** Nothing in the report or in a public API separates the plan's
  retained bytes from its admission margin. The margin does not follow from report rows alone:
  on the identity fixture it is smaller than the graph metadata plus bank scratch. A slack band
  would be a pin that moves whenever an unretained compile-time layout moves.
- **Proposed successor, for root to file.** Give the bound plan per-owner attribution: a
  retained-bytes walk over the bound runtime, as `observation_retained_bytes` already does for the
  observation lanes, checked by an allocator oracle in graph's own tests. The capi oracle can then
  require plan-observed == walked. The oracle's doc comment names this gap. I have no issue number
  to cite: I may not file one.

### The LOW findings
- **Finding 4.** `direct-oracle.mjs`'s check mode applies the two classes: exact rows equal, and
  retained rows 0 < v ≤ `resourceCeilings`. It passes on the built module and goes red on a ceiling
  one byte under (`builtinRetainedBytes 1817 is over its ceiling 1816`). The WebDriver harness's
  run and `--check` modes still compare the old whole block; #1050 retires the harness.
- **Finding 5.** Both budget docs cite the review's sampled history (+1.2 %, +2.3 %, +11 %,
  +48 %): 10 % passes the routine moves and stops the structural ones.

### Gates (final tree `3f9a111e` unless noted)

- `cargo check --workspace --all-targets --all-features`, workspace clippy `-D warnings`, fmt.
- `cargo test -p capi -p host-core -p host-web`: dev 443 passed and release 443 passed, 0 failed,
  4 ignored.
- With session, protocol and the test-support features: 636 passed. session in release: 59 passed.
- `console-workload` in release (the console digests).
- `check-browser-expected-resources.py --artifacts` and `--self-test` (32 mutations); the
  direct-oracle check mode; `check-web-audioworklet.sh`; `test-web-audioworklet.sh`;
  `check-scalar-oracle-absent.py --wasm` and `--native`.
- `check-capi-abi.sh` and its `--self-test`.
- The session, host-core, workspace, realtime and env-vocabulary policies, routing, and
  test-support-ci.
- All 49 lint-job commands plus routing and `test-env-vocabulary.sh`, run with `python3 -B`, on
  `8bc6ac40`. Since then only the shrink moved from `compile.rs` to `canonical.rs` and one doc
  comment changed; on the final tree `check-session-policy` is re-run green (it had caught the
  first placement).

**AArch64.**
- The x86 `cargo test --no-run` of `run-aarch64-tests.sh debug`'s 25 product crates plus
  dsp-reference, conformance and target-smoke, with the leg's features, resolves 218 test
  executables.
- Under qemu-user, `resource_lifecycle` has 9 passed and `retained_ceilings` 1 passed. Real arm64 CI
  verifies the rest.

**Shipped module:** CHANGED. It was `a4a2383f…` at `3ba8982a` and is `390fada9…` after, from the
canonical shrink in `session`. The PCM digests, exact rows and `memoryBytes` are unchanged.

**`--list` diff against `3ba8982a`:**
- `resource_lifecycle`:
  - removed `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`;
  - added `double_live_oracle_drives_exact_and_one_below_c_caps`,
    `capi_retained_bytes_charge_every_byte_the_compile_retains`,
    `reference_session_retained_rows_stay_within_their_budgets` and
    `prepared_parameter_catalog_charge_covers_its_allocations` (none ignored);
  - 6 → 9 tests.
- host-web: added `browser_identity_fixture_retained_rows_stay_within_their_budgets`.

**Not rerun:** the gate-4 mutation runs and the gate-5 reverts from attempt 1.

**My own error, fixed.** A paragraph edit in `docs/C_ABI_V1_QUALIFICATION.md` dropped that
paragraph's later sentences (the C response vectors and the RT artifact history); `8bc6ac40`
restores them word for word.

## Sol verdict, attempt 2

**PASS, with one tracked successor.**
- Both HIGH findings are closed. The completeness oracle now observes every host-core owner. Plants
  in the catalog, the canonical JSON and three owners the implementer did not name all go red.
- The residual plan-bound gap is an honest limit of the one owner that has no exact attribution.
  It is not a hidden under-count.
- **Before #1060 closes,** root files the successor below and cites its number in this spec and in
  the doc comment of `assert_host_owners_are_charged`. That is bookkeeping, not another attempt.

Reviewer: Sol, 2026-09-29. I merged `6ccbe7cf` into a scratch detached checkout of main
`a8955ad4`. The merge was clean, and its tree is identical to `6ccbe7cf`, because the branch already
carries batch 3 and main's tree equals `3ba8982a`. No timed workload was run.

### Findings, by severity

1. **MEDIUM, accepted with a successor: the prepared plan is only bounded.** The check still covers
   the plan: `plan ≤ engine rows`, and the four observed owners must sum to every byte the host-core
   half retains.
   - **Slack on AArch64 under qemu:** 128,405 bytes (EQ), 128,730 (soft-clip) and 8,109 (identity).
     On x86 the evidence reports 128,317, 128,834 and 12,233.
   - **What the bound catches.** An uncharged plan table goes red above the smallest slack, as my
     attempt-1 plant at 229 KB did.
   - **What it misses.** A table below the slack stays green; the evidence reproduces my 1.8 KB
     plant. Estimate mutants such as `graph_metadata_bytes → Some(0)` are still caught only by
     graph-compiler's own tests.
   - **Why this is not a FAIL.**
     - Amendment 2 allows a bound where exactness is infeasible. It is infeasible here: the plan's
       rows are the graph compiler's pre-allocation admission estimate, and no API separates what
       the bound runtime retains from that margin.
     - Base had no allocator check on the plan at all, so this is new coverage, not a regression.
     - Admission stays safe (charge ≥ live) on every fixture observed. The risk is a future
       uncharged table in a session shape with less margin than the fixtures, which is what the
       successor closes.
     - Exact attribution is a graph-crate product slice, outside this issue's crates. AGENTS.md
       says to split such work.
   - **Successor scope, for root to file:**
     > Give the bound graph runtime a per-owner `retained_bytes()` walk (units, banks, tables,
     > delays), checked equal to its observed allocations by an allocator oracle in graph's tests.
     > Then tighten capi's `resource_lifecycle` oracle from `plan ≤ engine rows` to
     > `plan == walked ≤ engine rows`, so an uncharged plan table of any size goes red.
2. **LOW, informational: the C report now carries no canonical-JSON bytes.**
   - The canonical JSON is charged once, correctly, inside `compiled_model_bytes` in the graph cap,
     at initial admission and in `validate_replacement_peak`. The budget and double-live tests prove
     both at exact and one below.
   - `PlanResourceReport` has never had a compiled-model row. So a host that sums report rows, rather
     than configuring caps, now sees 16,712 fewer bytes on EQ.
   - The C ABI qualification doc states this. No action is needed unless a host sums the rows; if
     one does, the fix is a model row in a later ABI.

### HIGH 1 (completeness trusted two charges): closed

- **Re-planted, both red.**
  - Reverting the catalog to `collect::<Result<Vec<_>>>` fails three tests: the oracle (261,348
    against 256,740), tiny-frame (187,670 against 183,062) and the catalog test (129,708 against
    125,100).
  - Removing `shrink_to_fit` fails the store check: 38,069 > 23,039.
- **Owners the implementer did not name, all red.**
  - **Source producers:** an uncharged `Box<[u64; 4]>` in `SourceControlSet`. The oracle and
    tiny-frame go red, off by exactly 32 bytes.
  - **Compiled session (store):** an uncharged `Vec<u64>` of 8 in `CompiledSession`. The store check
    on the browser identity fixture goes red, 2,931 > 2,869. That fixture's store slack is 2 bytes;
    EQ's is 1,026.
  - **capi-owned protocol state:** an uncharged `Box<[u64; 4]>` in `ReplayCache`. The oracle and
    tiny-frame go red, left 256,788 against right 256,756: the struct grew 16 bytes, which is
    charged, and the 32-byte heap buffer is not.
- **`shrink_to_fit` runs off the render path.** `write_canonical` has one production caller,
  `compile_session`, reached from `SessionStore::new` and apply and from host-core's compile, all
  control-plane. Every other `canonical_session_json` call is `#[cfg(test)]`. The render-path
  checks pass: the egress no-allocation test, and `check-realtime-policy.sh` with its tests.
- **The capi row moves only by the removed double charge.** The drop equals each fixture's canonical
  length: EQ −16,712, soft-clip 160,893 → 142,440 (−18,453) and identity 133,559 → 131,640 (−1,919).
  Tiny-frame drops 16,712, and the double-live capi requirement 36,897 (18,453 + 18,444). The catalog
  fix changes the bytes allocated, not the charge, which is by length and was already 125,100.

### HIGH 2 (#1059 conflict): closed

- `cargo test --locked -p capi` compiles and passes on the merge without `lane/test-support`.
- `lane` is a plain host-web dev-dependency: the workspace entry has no features. `cargo tree -e
  features -i lane` shows only `default` for host-web's normal graph, native and wasm32, and for its
  dev graph.
- `check-scalar-oracle-absent.py --wasm` passes on the release module (2,649 symbols, none of the
  17 scalar identifiers), and so do `--native libcapi.so` and `--self-test`.

### The module change is explained

- The merge builds `390fada9…` at 3,415,176 bytes. Main builds `a4a2383f…` at 3,415,107.
- With only `crates/session/src/canonical.rs` set back to main's version, the merge rebuilds
  `a4a2383f…` byte for byte. So the whole change, +69 bytes, comes from the shrink.
- The catalog change cannot reach the module: host-web's wasm graph enables host-core with only
  `default`, and `control_provider` sits behind `control-provider`. The dev-dependency moves no byte.
- The browser rows, the PCM digests and `memoryBytes` are unchanged.

### Gates on the merge (tree = `6ccbe7cf`)

- fmt; clippy `--workspace --all-targets --all-features -D warnings`; `cargo check --workspace
  --all-targets --all-features`.
- `cargo test -p capi -p host-core -p host-web`: dev 443 passed and release 443 passed, 0 failed.
  The 4 ignored are the unchanged release-budget and descriptive-timing tests.
- AArch64 under qemu-user: `resource_lifecycle` 9 passed and `retained_ceilings` 1 passed. The
  values equal the table: capi 256,740 of 282,432, graph 230,845 of 253,952, and double-live capi
  167,414.
- `check-browser-expected-resources.py --artifacts`, green with 32 self-test mutations;
  `direct-oracle.mjs` in check mode; `check-web-audioworklet.sh`.
- `check-capi-abi.sh` and its `--self-test`; `console-workload` in release (62 passed).
- All 57 lint and routing policy commands, run with `python3 -B`, including the three self-tests
  #1043 made conditional.
- The x86 `cargo test --no-run` of the `run-aarch64-tests.sh debug` set (28 packages, the leg's
  features, 218 executables) and of the release leg's `lane`/`math`.
- **Gate 4, re-run on this tree.** capi `runtime/compile.rs`: 58 caught of 78, against 56 at base
  `a509b681`, with none lost. The 36 host-core accounting mutants: 12 → 12, the identical set.
  `build_parameter_catalog`'s 20 mutants, not in the base run: 9 caught.
