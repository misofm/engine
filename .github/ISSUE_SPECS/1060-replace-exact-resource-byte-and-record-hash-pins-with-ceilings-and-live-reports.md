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
