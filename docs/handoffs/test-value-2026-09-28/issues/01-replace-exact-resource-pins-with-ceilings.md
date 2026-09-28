# Replace exact resource-byte and record-hash pins with ceilings and live reports

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 1). Base `a9414c0c`. **Needs owner ruling R2**
("are exact retained-byte counts of fixtures a product requirement, or are budgets ceilings?").

## Problem

Several gates assert an exact byte count or a hash of a record whose only variable content is a byte
count. Every intended layout change turns them red, and the fix is always a re-pin:

- From 2026-09-01 to 09-28, pin refreshes were 78 of the 132 red CI jobs (`../data/ci-red-jobs.tsv`).
  No pin exposed an unintended change.
- The pins in this issue moved often:
  - `expected.json` resource rows 24 times;
  - `crates/capi/tests/resource_lifecycle.rs` up to 18 times;
  - the graph audit record hash 5-8 times;
  - `layout_total_bytes` 3 times, and CI went red twice when one of its three copies was missed.

The pins in scope:

1. `hosts/host-web/tests/browser-v1/expected.json`: the 21 wasm32 resource byte rows. Also the
   row-equality check and the native "witness" classification in
   `scripts/check-browser-expected-resources.py` (`:254-297`), which pays for a native fat-LTO build
   of `host-web --example browser_fixture_resources` (`:378-413`, about 60 s per PR).
2. The Issue-544 inline validator, `.github/workflows/qualification.yml:671`: `layout_entries: 16`
   and `layout_total_bytes: 6472`. Also the Rust copies in `tools/audit/src/source_duration.rs:317-337`:
   `assert_eq!(capture.layout.len(), 16)`, `6_472`, and fnv `0xde96_92fb_4797_b2a5` of the
   accounting text.
3. `scripts/trace-builtins-graph-audit.sh:62`: the expected audit record hash, whose variable content
   is three Rust constants.
4. `crates/capi/tests/resource_lifecycle.rs:605-2732`: about 2,100 lines that mirror the engine's
   layouts by hand, driving literal totals at `:2694-2732`.

## Outcome

- Each budget claim is asserted as a **ceiling**, checked exactly at the budget and one byte over,
  or by an existing **live-report** test.
- No exact byte count of a fixture remains in a test, gate or workflow.
- The claims that are real stay:
  - the PCM digest parity between native and simd128 in `check-browser-expected-resources.py`;
  - the duration-independence equality flags and the frame/file-size constants of the Issue-544
    record;
  - the audit's zero allocation, lock and syscall counts.

Surviving guards of the budget claims, which already run:
- `hosts/host-web/src/tests.rs:650`, `:984` and `:1079`;
- `hosts/host-web/tests/boot_transient_budget.rs`;
- `scripts/check-web-boot-budget.mjs`;
- `crates/capi/tests/resource_lifecycle.rs:2785` (the live-report form);
- `crates/host-core/tests/prepare.rs:161`.

## Scope

Authorized paths:
- `hosts/host-web/tests/browser-v1/expected.json`;
- `scripts/check-browser-expected-resources.py`;
- `.github/workflows/qualification.yml` (the inline validator only);
- `tools/audit/src/source_duration.rs`;
- `scripts/trace-builtins-graph-audit.sh`;
- `crates/capi/tests/resource_lifecycle.rs`;
- this issue's spec.

## Gates

1. **Every budget still refuses.** For each removed exact row, name the ceiling test or live-report
   test that now owns the claim. In a scratch copy, grow the corresponding retained structure past
   its budget by 64 bytes: the named test goes red. Grow it by 64 bytes inside the budget: every gate
   stays green, which is the intended change of behaviour.
2. **The real claims keep failing:**
   - `check-browser-expected-resources.py --self-test`'s PCM-digest mutation still fails;
   - `tools/audit` still exits non-zero when the 1-minute and 3-hour layouts differ (seed a
     duration-dependent entry);
   - the graph audit still fails on one allocation during render.
3. **Mutation equivalence on capi**, using `../tools/run-mutants.sh capi`, before and after. The
   caught set may shrink only by mutants whose sole catcher was an exact-byte literal and whose only
   effect is a retained-size change under the ceiling. List each such mutant in the PR.
4. **Historical bugs.** `../tools/revert.py` for #966, #970, #994 and #1015. Each still turns at
   least one test red. None of the tests touched here is a reproducer.
5. **Cost.** The artifact-gates step no longer builds `host-web --example browser_fixture_resources`
   natively. Record the step time before and after.

## Saving and risk

- **Saving:** about 60 s of runner time per full PR, and roughly 50 re-pin edits a month.
- **Risk:** growth of up to the ceiling's headroom goes unseen per PR. Set the ceilings at today's
  value plus the headroom the owner accepts.
