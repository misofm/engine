# #1217 *Skip an inactive route in its destination's sum*: Sol verdict, attempt 2

- **Reviewed:** `git diff 0da034dba 1b5034c35` (5 files, +249/-15), judged together with attempt 1
  (`071c6c14a`) against the spec.
  - Branch `codex/batch-submix-k3`, worktree `/home/bl/misofm/wt-submix-k3`.
  - The parent `0da034dba` is #1218, which passed separately.
- **Binding:** `AGENTS.md`, plus `.github/ISSUE_SPECS/1217-skip-an-inactive-route-in-its-destinations-sum.md`
  with its Attempt 1 and Attempt 2 records, and verdict 1 (`1217-attempt1.md`).
- **How I ran it:**
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1217` is OPEN, and its title
    matches the spec.
  - I exported these commits with `git archive` under `/tmp/claude-1002/v1217b/`, each with its own
    target directory:
    - head `1b5034c35`;
    - `f48fe7c74` (#1216, the attempt-1 base);
    - `0da034dba` (#1218);
    - `071c6c14a` (attempt 1);
    - a variant of head with `reduce_group` set back to `#[inline]`.
  - I copied `sdk/node_modules` into the exports.
  - Mutations ran in a separate copy, so the gate tree stayed pristine.
  - My probes are saved beside this file as `1217-attempt2-verifier-scratch.rs`.
  - All scratch was deleted afterwards.

## Verdict: PASS

- **BLOCKER-1 is resolved.** The required web kernel-shape gate is green, and the module's
  `4wide6f32x4` kernel set is identical to `f48fe7c74`'s, by name and by vector:scalar count.
- **The inlining moves no rendered bit.** I checked this directly, on native debug and release, and
  through the browser identity fixture's pinned PCM digests.
- **MINOR-1 and MINOR-2 now bite on audio.** Their mutations are red exactly as `MUTATIONS.md`
  1217-7, 1217-11 and 1217-12 record.
- **The NIT fixes are correct, and NIT-1's tightened test bites.**
- **Every gate is green.** The counts match the record exactly.

There is no BLOCKER and no MAJOR. There is one new MINOR, a test-strength gap that attempt 1 already
had: an active contributor that follows an inactive one is untested. There are also two NITs.

## Gates (head `1b5034c35`, x86-64-v3 AVX2)

| Gate | Result |
|---|---|
| `build-web-audioworklet.sh --named-twin` | rc 0. Shipped module 2,726,858 B; named twin 3,109,932 B |
| `check-web-audioworklet.sh` (full form, metadata regeneration included) | rc 0: `kernels=12`, every roster entry ok, `eight lanes: none`, render/meter/command closures as on base. Boot budget high-water passed with 0 mismatches. Static/object checks passed. For comparison, the same script at `0da034dba` gives rc 1 with eight `reduce_group_into<f32x4,N>` FAIL lines and `kernels=27`; at `f48fe7c74` it gives rc 0 with `kernels=12` |
| `check-browser-expected-resources.py --artifacts` | rc 0. Digests and exact rows agree, every budget row is within its ceiling, and the self-test catches 32 red mutations |
| `test-web-audioworklet.sh` | rc 0 |
| Extra CI `artifact-gates` and `sdk` steps | `check-scalar-oracle-absent.py`, `check-web-audioworklet-v8-spill.py` (Node 22.23.2, the pinned V8) and `check-sdk-headless.sh`: all rc 0 |
| 6: release build of `audit`, `bench`, `capi` and `session-validator` | rc 0 |
| 6: `check-graph-determinism.sh` | PASS (100/100). The output is `cmp`-identical to `f48fe7c74`'s and `0da034dba`'s, each run fresh |
| 6: `graph_fixture -- --check`, `check-console-fixtures.sh`, `check-builtins-fixtures.sh` | rc 0 |
| 6: `audit capi` | `allocations 0`, `deallocations 0`, `syscalls 0`, `total_violations 0`, digest `ff6cdcb96cdcdad5` |
| 6: `cargo test --release -p audit -p bench -p console-workload` | 110 passed, 0 failed, 2 ignored, including `every_standing_workload_folds_one_route_per_track` |
| 7: test-debug-a (the exact DESIGN 7 command, `--no-fail-fast`) | rc 0: **1194 passed, 0 failed, 9 ignored, 106 binaries**, as recorded |
| 7: `cargo fmt --check`; workspace clippy `--all-features -D warnings`; clippy without features on `graph`, `graph-compiler`, `host-core` and `capi` | all rc 0 |
| 7: `check-`/`test-graph-policy.sh`, `check-`/`test-realtime-policy.sh`, `check-workspace-policy.sh`, `check-cross-targets.sh` | all rc 0 |
| 7: `run-aarch64-tests.sh debug` | **not run**: this is an x86 host with no arm64 runner. It is deferred to CI `aarch64-debug` at the K3 push, which the spec permits |

## BLOCKER-1: verified resolved, and the extra attribute is justified

**The kernel set.** I listed every arithmetic-carrying `4wide6f32x4` function from `wasm-objdump -d`
of each named twin, with crate hashes normalized.

- **`1b5034c35` against `f48fe7c74`:** an empty diff. The same 12 kernels, the same names and the
  same vector/scalar counts.
- **`0da034dba` (attempt 1 plus #1218):** 27 kernels. They are:
  - the 12 kernels above;
  - 8 `reduce_group_into<f32x4,1..8>` instantiations, at vector:scalar 1:3 up to 15:45, which fail
    the rule;
  - 7 `reduce_group<f32x4,2..8>` instantiations, at 2:1, which pass.
- **Head with `reduce_group` set back to `#[inline]`:** 19 kernels, all passing. So the gate does
  not require the second attribute.
  - Without it, those 7 outlined instantiations count toward the `--kernel-min` ratchet. They give
    it seven kernels of slack, and each one costs a call per reduction group.
  - With it, the module has the parent's kernel set and inlined shape.

That is a sound reason, and the attribute costs 3.9 KB (below).

**No rendered bit moves.**

- **The cited evidence does not show this.** The record cites `fresh-process-determinism.json`, but
  that file is `graph_fixture`'s compile fingerprint, not render output (NIT-2).
- **My own check, native.** I ran a render-digest probe (in the scratch file):
  - one to twelve drawn-gain routes into a bus, and into the session output, from `post_pan` taps,
    with signed-zero frames;
  - with the route fold active, and with it declined, so that `reduce_plane`, `reduce_many` and
    `reduce_group` run at N = 1..12 in both the arena and the host form;
  - in debug and `--release`.

  The digest files are identical across `f48fe7c74`, `0da034dba` and `1b5034c35`. With muted
  patterns added (every third route, or the first and last), which exercises `reduce_gated` and
  `reduce_gated_into`'s `Store` and `Add` arms, `0da034dba` and `1b5034c35` are identical too.
- **Wasm.** `check-browser-expected-resources.py --artifacts` holds the browser identity fixture's
  three PCM digests, equal to native, and it passes on head.

**Code size and boot budget.** Shipped module sizes in bytes:

| Commit | Shipped bytes | Change |
|---|---|---|
| `f48fe7c74` | 2,696,563 | base |
| `071c6c14a` (attempt 1) | 2,706,544 | +9,981 |
| `0da034dba` (+ #1218) | 2,716,545 | +10,001 |
| head with only `reduce_group_into` always-inlined | 2,722,979 | +6,434 |
| `1b5034c35` | 2,726,858 | +3,879 |

- #1217's total over `f48fe7c74` is about 20.3 KB, or 0.75% of the shipped module. The extra
  `reduce_group` attribute is 3.9 KB, or 0.14%.
- The module grew by 15,253 opcodes (+1.3%), and `f32x4_arith` went from 9254 to 9350.
- No gate bounds module bytes. The boot budget gate measures memory high-water, and it passes.
- I judge this acceptable for the gated reduction path. A future size pass could use the #926 form
  (a non-generic `#[inline(never)]` `f32` tail), but nothing here requires it.

## The verdict-1 findings, checked

I applied each mutation alone to the head copy, ran `route_mute` (12 tests) plus graph-compiler's
`route_activity` and `route_coefficients`, and reverted it.

| Mutation | Result |
|---|---|
| MINOR-1 / 1217-11: the host form stores nothing for an inactive first input (`SumSegment::Zero => {}` in `reduce_gated_into`) | **RED** in two tests: `an_inactive_route_is_neither_mixed_nor_read` (`seed 0, Output, r0 muted, plane 0: sample 0: 7.0 != 0.0`) and `a_destination_whose_every_route_is_muted_renders_positive_zero` (`Output, 2 muted routes: plane 0 frame 0: 7.0, expected 0.0`). This is the attempt-1 hole, now closed by `HOST_SENTINEL` |
| MINOR-2 / 1217-7: a delayed muted route goes inactive (`&& input.delay.is_none()` dropped) | **RED on audio**: `a_muted_delayed_routes_line_carries_its_zero_mix` (`plane 0: e's input, d-e's delayed zero mix alone: sample 486: 0.0 != -0.0`). Also red by counter in `a_muted_delayed_route_stays_active` and #1218's `a_fully_follow_muted_delayed_send_stays_active` |
| 1217-12: the host form skips a sum whose every input is inactive (early `return` in `reduce_gated_into`) | **RED**, 1 of 12: `a_destination_whose_every_route_is_muted_renders_positive_zero` (`Output, 2 muted routes: ... 7.0, expected 0.0`) |
| The same skip in the arena form (`reduce_gated`) | **RED**, 1 of 12: the in-place test (`-0.15693776, expected 0.0`, the raw tap). That test is `count == 1`. See NIT-1 for `count >= 2` |
| NIT-1 / 1217-13: the estimate charges twice the bound | **RED**: `a_muted_route_seals_one_route_mute_row_after_its_transform`. Gate 4 (`route_activity`) stays green, being a lower bound |

**NIT-1, the estimate row.** It is checked against `canonical.rs`'s writer and `estimate.rs`:

- Field 0 is `logical_nodes`, 11 is `graph_metadata_bytes`, 17 is `largest_allocation_bytes`, and 18
  and 19 are `incremental_plan_bytes` and `session_plus_plan_bytes`. The row has 20 fields.
- `largest = max(metadata, lane, delay_lane, reduction)`. The test's `open_largest.max(open_metadata + charge)`
  is therefore exact.

**NIT-2, the tag bit.** `index` now refuses a value with bit 31 set, via `expect`, which is a panic
at bind on the control thread. This is unreachable under any configured route cap, and the parse
precedence `(value & ROUTE_DESTINATION) == 0` is right. It is acceptable.

## My own mutations

| # | Mutation | Result |
|---|---|---|
| MA | Arena form `SumSegment::Zero => {}` (no `+0.0` fill) | RED, 3 tests: gate 1 (`seed 0, Bus, r0 muted ... 0.902406 != -0.3599777`), the in-place test, and #1218's `a_fully_follow_muted_undelayed_send_is_inactive` |
| MB | `route_activity` stops skipping the fold master (`Some(op_index) != master_op` replaced by `true`) | GREEN, but an **equivalent mutant**. A master's contributors are all open (a gated route declines the fold), and its runtime inputs are `[its own output]`. `route_segments(1, all-active pairs)` therefore yields `Store(0, 1)`, which is `reduce_plane`'s in-place no-op, exactly the ungated path |
| **MC** | `route_segments` forgets `stored = true` after an opening `Store`, so the next active run *stores* over the earlier active run instead of adding to it | **GREEN on every committed test.** That is the 12 `route_mute` tests plus the graph-compiler route tests, and also 649 tests over `graph`, `graph-compiler`, `host-core`, `capi`, `host-web` and `engine` with the test-debug-a features. **RED** on my probe `probe_middle_and_scattered_inactive_runs_match_d3` (`Bus, 3 inputs, muted [1], plane 0: sample 0: 0.29300022 != 0.36220396`). See MINOR-1 |
| V3c | The arena form skips its sum when `count >= 2` and every input is inactive | GREEN on every committed test. See NIT-1 |
| E1 | The estimate charges one route too many | RED: the tightened #1216 gate 5 |

**The head is correct where MC hides.** My probe renders against the D3 oracle bit for bit. It runs
the patterns below, each into a bus and into the session output, with drawn gains and matrices, and
checks per-route mix counts:

- `[open, muted, open]`;
- `o m o m o`;
- 12 inputs with positions 3 and 9 muted (a later run crossing the 8-input group boundary);
- 12 inputs with positions 0, 5 and 10 muted;
- 11 inputs with the 9th muted;
- 12 inputs with positions 2 to 10 muted.

All pass on `1b5034c35`.

## Findings

### MINOR-1 (new; present since attempt 1): no committed test has an active input after an inactive one that follows an active one

**Evidence.**

- Every committed D3 case falls into one of three segment shapes:
  - `[muted, open]`, which is `Zero, Add`;
  - `[open, muted]` or `[open, muted, muted]`, which is `Store` alone;
  - all muted, which is `Zero`.
- No case produces **`Store` followed by `Add`**, the commonest real shape: one send muted among
  several open ones.
- So mutation MC, which drops every active contributor before the muted route, survives the whole
  suite. That is a bus silently losing audio whenever a middle route is muted.
- Verdict 1's "middle inactive" check was a verifier probe, never committed.

**Fix (a few lines).** Add `&[false, true, false]` to gate 2's cases with a non-`-0.0` oracle, or
better, a three-or-more-contributor variant of gate 1 against `d3_sum`.

`probe_middle_and_scattered_inactive_runs_match_d3` in the scratch file is a ready form. It also
covers runs that cross the 8-input group boundary.

Recommended before the K3 batch closes, or at the latest in #1220, which rewrites activity per block.

### NIT-1: the "every contributor muted" bus half cannot see an arena-form skip

**Evidence.**

- With 2 or 9 muted routes into bus `b`, `b`'s arena buffer is never written by another op, so it
  holds zeros whether or not the sum stores `+0.0`.
- V3c (skip when `count >= 2` and all inputs are inactive) and MA are both green on that half. Only
  the `Output` half (the sentinel) and the `count == 1` in-place test discriminate.

**Fix.** The natural place is #1220. There, a bus that was active in one block and inactive in the
next pre-dirties its own buffer, which is exactly that slice's hazard. Assert `+0.0` there.

### NIT-2: the attempt-2 record's "inlining moves no bit" cites a compile fingerprint

`fresh-process-determinism.json` is `graph_fixture`'s compiled-graph fingerprint, so it cannot
witness render bits.

The claim is true: my render-digest probe and the browser identity PCM digests show it. Cite those,
or the browser expected-resources pass, instead.

## Test value (attempt 2's new or tightened tests, one sentence each)

- **`an_inactive_route_is_neither_mixed_nor_read`, now over `HOST_SENTINEL` planes:** red if the
  host-master form leaves an inactive first input's store unwritten (1217-11). Nothing else caught
  that in attempt 1.
- **`a_destination_whose_every_route_is_muted_renders_positive_zero`:** red if the session output
  whose every route is inactive skips its sum or its `+0.0` store (1217-12, red in this test alone).
  Its bus half is weaker (NIT-1).
- **`a_muted_delayed_routes_line_carries_its_zero_mix`:** red on the audio if a delayed muted route
  goes inactive, if its line carries anything but the zero-coefficient mix, or if the 486-sample
  delay moves (1217-7). Before this test, only a counter defended that.
- **`a_muted_route_seals_one_route_mute_row_after_its_transform`, tightened:** red if the estimate
  row moves by anything other than exactly `route_activity_bound_bytes(R, N)` in its three charged
  fields (1217-13, E1). Gate 4's lower bound misses both.

## Not verified here

- **`run-aarch64-tests.sh debug`, the 4-lane native run.** This is an x86 host with no arm64 runner.
  It is deferred to CI `aarch64-debug` at the K3 push.
  - The wasm simd128 4-lane module passes every web gate above.
  - The cross-target matrix (iOS and Android aarch64 check plus clippy) passes.
