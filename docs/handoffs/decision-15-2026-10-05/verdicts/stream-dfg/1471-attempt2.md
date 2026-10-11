FAIL

# #1471 attempt 2 -- adversarial verdict

Verifier: fresh Opus, 2026-10-11. Reviewed the cumulative #1471 work at `e44b8e8b2` on `codex/d15-batch-dfg`
(commits 28de9b791 dbcf9c46c ecdc3a936 8c61cb459 5ba95c705 6d56b1f15 32241583c 5fa3f0c69 1227b9090
e44b8e8b2; the three unrelated follow-ups 3812a1171, 966f0e82d and 773d93b57 are excluded). The code was
exported with `git archive e44b8e8b2` to `/tmp/claude-1002/v1471/tree` and built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1471/target`. Nothing was built or edited in the worktree.

The product code does what Amendment 2 asks. The attempt fails on one MAJOR: gate 6's test does not
catch one of its three named defects and catches the second only in part. The test's own doc comment
says that it catches both. A test inside the named exceptions catches both, and I ran one (see below).

## MAJOR

**M1. Gate 6 is not met as written. The test's doc comment claims a catch that does not occur.**
`crates/capi/tests/resource_lifecycle.rs:1438-1445` (`the_engine_cache_stays_within_its_stated_ceiling`).
Amendment 2 gate 6 names three defects: an entry-layout change, a cap that no longer applies, and a
journal left open on the engine's cache.
- *Cap*: caught (my M6a: `INPUT_BOUND_CACHE_ENTRIES` = 16,384 makes it red).
- *Journal left open*: **not caught.** My M6b changes `take_journal` to `self.journal.clone()`, so the
  journal stays open after every compile. The test stays green. The doc comment at :1442-1443 still
  says that it catches this defect ("it keeps every design used, uncapped"). That is false:
  `begin_journal` restarts the journal, so the leaked journal holds at most one preparation, which
  fits in the 0.70 MB of headroom. The attempt record (spec :385-392) states this limit. The test
  comment does not.
- *Entry-layout change*: caught only for large growth. With 8 more bytes per value, the derived worst
  case becomes 1,639 x 2,696 + 273 x 96 = 4,444,952 B. That is above the header's 4,300,720 B. The
  measured fill (about 1,381 nodes) stays near 3.72 MB, so the test stays green. A field added to
  `NodeTailBound` or `CachedDesign` is a plausible change, and it would make the header's figure
  false with no test going red.
- The record says that "a sharper check needs an engine-internal test, not in Amendment 2's
  exceptions" (spec :391). That is not true. I added this C-ABI-only test to `resource_lifecycle.rs`
  in the export: engine A compiles a 200-design document (`distinct_design_session(0, 200)`); engine B
  compiles the same document and then a memoryless one (every `hpf_hz`/`lpf_hz` = 0). The test
  asserts that `engine_destroy_freed(A) == engine_destroy_freed(B)`. On the real code it is green
  (84,208 = 84,208 B, 0.9 s). Under M6b it is red (168,048 vs 84,208). It is an allocator-observed
  equality, not a pinned byte count. For the layout defect, measure the node size from a one-design
  compile (the attempt-1 record shows that a fresh engine then frees exactly one leaf, 2,608 B),
  derive 1,639 x leaf + 273 x 96 from it, and assert that the result is at most the stated ceiling.
  Both checks fit in "the engine-ceiling test" exception. Neither needs root.
- Severity: one of gate 6's binding root defects (adopted verbatim) has no catcher, and the test's
  own text claims that it does. AGENTS.md: "there is no PASS without" a true test-value answer. No
  product defect is present: `prepare_runtime` takes the journal on every path.

## MINOR

**m1. The header's allocator-overhead figure is glibc's, not the figure for the C ABI's live targets.**
`crates/capi/include/miso_engine_v1.h:119` ("up to 16 bytes a node") and
`docs/C_ABI_V1_QUALIFICATION.md:565-566` (26,224). On iOS, libmalloc's small region (sizes from about
1 KB) rounds to a 512-byte quantum, so 2,608 and 2,704 both become 3,072. Android's scudo also uses
size classes much coarser than 16 B. The derived 4,300,720 B ceiling on requested heap is correct.
The stated bound on the extra is wrong for iOS and Android, which are the targets the header is for.
Say that the extra depends on the platform allocator's size classes, or drop the figure.

**m2. The header ceiling does not include the journal while a compile runs.** Header :111-119 says
that the cache "holds at most 4,300,720 bytes". During a compile the same `InputBoundCache` also holds
the open journal map: up to 1,805 entries, at most 361 nodes, about 0.95 MB. That memory is transient
and is freed before the call returns. Say "between calls", or state the transient term.

**m3. Gate 4's rebuild timings come from attempt-1 code.** Spec :363-364: "The timing table above is
unchanged ... not re-run". Amendment 2 changed the rebuild path: it now reads a sorted-slice seed
through `with_seed` instead of the session `BTreeMap` that attempt 1 took out with `mem::take`, and it
no longer refills a session cache. The table's rebuild column therefore describes code that is gone.
The benchmark is descriptive, so this is MINOR. Re-record the 9- and 64-track rebuild rows on the
final code, with one invocation as before.

## NIT

- n1. `crates/control-plane/src/compile.rs:297-315`: adding `bound_seed_bytes` to the first array made
  rustfmt re-indent the whole `largest` chain (about 15 lines) in a stream-B hot file that stream B's
  batch 2 also edits. `.chain([bound_seed_bytes])` gives the same result with a one-line hunk.
- n2. Gate 5's refusal uses the pending-plan backpressure (`!pending_providers.is_empty()`, after
  `validate_replacement_peak`). The gate names a test fault after admission or a limit one byte below
  the peak. The effect is the same (refused after preparation and admission, before commit; my M5
  makes it red). Record the choice in the spec.
- n3. Edits outside the literal path lists, each necessary and small: `crates/builtins/src/lib.rs`
  (`get` -> `lookup` with `as_deref_mut`, the `InputBoundRecord` re-export),
  `crates/host-core/src/lib.rs` and `crates/control-plane/src/lib.rs` (re-exports),
  `crates/capi/src/runtime/mod.rs` (the `compile_children` wrapper's parameter). Attempt 1's record
  names most of them. Record them in one line of the Attempt record.

## The points the implementer raised

1. **Gate 6 and a journal left open.** Yes, the test-value claim is false as written (MAJOR M1). A test
   inside the named exceptions catches it: the exact before-and-after destroy comparison above, which
   I ran red and green. It does not need root.
2. **`InputBoundCache<'seed>` / `with_seed`.** Acceptable. The parameter is still spelled
   `Option<&mut InputBoundCache>` (an elided lifetime) on the same single type, so D2 holds in
   substance. There is no second entry point, so it is not a parallel public API variant. A borrowed
   read-only seed is the only shape that meets D1 (read `newest_providers().bound_seed` read-only)
   without a copy (more memory) or a move out of the newest epoch (which mutates it). I count it as
   part of the tail.rs "read-only form" exception.
3. **`settle` and the third rebuild's Backpressure.** This behaviour is already on `main` and is
   correct. I ran a probe on a `main` export (`dbd508504`; protocol, control-plane and capi `src` are
   the same as the slice base): two rebuilds with renders between them gave Ok and Ok. The third,
   without a drain, gave C result OK with a NonOk `Backpressure` response. A submit at revision +3
   then gave `RevisionConflict`, and after a drain a retry at revision +2 gave `Ok`. So the refused
   transaction did not commit. It is typed backpressure before commit, and no ack precedes a drop. It
   is not a defect and needs no root follow-up.
4. **The derived ceiling.** It is correct as an upper bound on requested heap. On the pinned 1.97.1
   toolchain I measured key 124 B (align 4), value 112 B (align 8), pair 240, std `BTreeMap` leaf
   2,608 B and internal node 2,704 B, with a counting allocator on stand-in types of the same layout.
   std's `splitpoint` leaves at least 5 entries in each half of a split (B = 6, CAPACITY = 11), and
   the cache only inserts and clears, so every non-root node keeps 5 to 11 entries. That gives
   N <= 1 + 8,191/5 = 1,639 nodes and I <= (N - 3)/6 + 1 = 273 internal nodes. Ascending and
   descending orders give 3,578,640 B, the largest of 60 orders I tried; the bound is 4,300,720. Only
   the allocator-overhead wording is wrong (m1).
5. **Acked-batch question for the seed hand-off.** No ack precedes a drop. The candidate
   `ProviderEpoch` owns the record from `prepare_runtime` onward. Every refusal before
   `prepared.commit` (test faults, `validate_replacement_peak`, `pending_providers` not empty,
   reservation failures, a full report table) drops the candidate with its record, and no code writes
   to the newest seed. After the protocol commit, `pending_providers.push` cannot fail (the queue was
   empty and its capacity was checked before the commit), and the response is written after the push.
   A live commit prepares nothing and changes no seed.

## Other checks (all hold)

- Attempt 1's pre-admission fill is gone (no `mem::take`, no session cache). `SessionState` holds no
  cache.
- `prepare_runtime` calls `begin_journal`, prepares (no `?` in between), and calls `take_journal`
  before `prepare_failure`'s `?` and before `prepared_capi_resources`, so the journal is closed on
  every path.
- The seed is an `epoch_rows` row and a `largest` candidate, so `active_retained` and
  `validate_replacement_peak` refuse with the existing `ResourceFault::Limit` before commit (M1 and M3
  below show this).
- The header change is comment only (every added line is a `*` comment line; `check-capi-abi.sh`
  passes).
- Render reads neither cache: `Engine::bound_cache` and `ProviderEpoch::bound_seed` are control-side
  only, and the audit shows 0 allocations, 0 syscalls and an unchanged digest.
- The control-plane and capi hunks stay inside the named exceptions, apart from n1 and n3.

## Mutations re-run by this verifier (each applied to the export and then reverted)

| # | Mutant | Red | Green |
|---|---|---|---|
| M1 (gate 1) | seed bytes passed as 0 to `prepared_capi_resources` | `capi_retained_bytes_charge_every_byte_...`, `tiny_control_frame_...`, `double_live_oracle_...` | the other 10 `resource_lifecycle` tests |
| M2 (gate 2) | `retained_bytes` = min(len, 1) x 240 | `capi_retained_bytes_...` only | `tiny_control_frame_...`, `double_live_...` |
| M3 (gate 3) | seed charge 0 on a rebuild only | `double_live_oracle_...` only | the other 12 `resource_lifecycle` tests, all of `design_bound_cache`, capi lib (81) |
| M4 (gate 4) | `begin_journal` starts from the seed (growth by union) | `a_rebuild_keeps_only_the_designs_its_plan_uses` | the other 4 `design_bound_cache` tests |
| M5 (gate 5) | the rebuild writes its record into the newest seed before admission | `a_refused_rebuild_leaves_the_seed_as_it_was` | the other 4 `design_bound_cache` tests, all 13 `resource_lifecycle` tests |
| M6a (gate 6) | `INPUT_BOUND_CACHE_ENTRIES` = 16,384 | `the_engine_cache_stays_within_its_stated_ceiling` | -- |
| M6b (gate 6) | `take_journal` leaves the journal open | only my probe (168,048 vs 84,208 B) | **`the_engine_cache_stays_within_its_stated_ceiling`** |

I did not re-run attempt 1's journal mutants (tail_contract and design_bound_cache tests 1-3). For
those, I rely on the attempt record.

## Test-value sentences (new or extended tests)

- `tail_contract::a_journal_holds_every_design_a_preparation_used_through_a_clear`: catches a journal
  that loses designs when the cache clears in the middle of a preparation.
- `tail_contract::a_journal_holds_the_hits_and_the_stopped_walks_a_preparation_used`: catches a journal
  that records only computed designs (no hits) or drops a stopped walk's horizon.
- `tail_contract::a_cache_over_a_record_finds_every_design_and_journals_only_what_it_used`: catches a
  record lookup that misses entries of a large record, or a journal that omits seed hits or copies
  unused seed entries.
- `design_bound_cache::a_rebuild_and_a_recompile_compute_no_design_bound`: catches an engine or session
  cache that is not wired into the compile or the rebuild.
- `design_bound_cache::a_compile_that_clears_the_engine_cache_still_seeds_its_session`: catches seeding
  the session from the engine cache after the compile (Amendment 1).
- `design_bound_cache::a_session_past_the_budget_rebuilds_without_walking`: catches a seed or hit that
  loses a stopped design's horizon.
- `design_bound_cache::a_rebuild_keeps_only_the_designs_its_plan_uses`: catches a seed that grows by
  union instead of following the newest plan (M4).
- `design_bound_cache::a_refused_rebuild_leaves_the_seed_as_it_was`: catches a rebuild that writes its
  designs into the newest seed before commit (M5).
- `resource_lifecycle` "three input designs" row: catches a seed charge that is right for one design but
  wrong per design (M2).
- `resource_lifecycle` `capi_epoch_terms` seed term and its non-zero assertion: catches a successor seed
  that is left out of the prospective epoch's charge (M3).
- `resource_lifecycle::the_engine_cache_stays_within_its_stated_ceiling`: catches a cap that no longer
  applies (M6a), and an entry that grows by many tens of bytes. It does **not** catch a journal left
  open (M6b) or a small entry growth (M1 above).

## Gates run (on the export at e44b8e8b2)

- `cargo fmt --all --check`: ok.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: ok.
- `cargo test` for `builtins --features test-support`, `builtins-compiler`, `host-core`,
  `control-plane --features test-support` and `capi` (all suites): 63 suites ok, 0 failed. This
  includes `design_bound_cache` (5), `resource_lifecycle` (12, 35.9 s), `plan_swap_race` (3) and
  `tail_contract` (23 run, 2 ignored).
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (93 marked regions
  in 26 files).
- `scripts/check-capi-abi.sh`: ok (x86_64-unknown-linux-gnu, shared and static), run with
  `MISO_ENGINE_CAPI_SKIP_BUILD=1` against the release `libcapi.so`/`.a` in the external target dir.
  The header diff is comment lines only.
- Release `audit capi`: calls 100,000, allocations 0, deallocations 0, locks 0, syscalls 0,
  `pcm_digest` `cb10fbface44a3a4` (unchanged), total_violations 0.
- Not re-run: `check-cross-targets.sh` and the worklet chain (they were not in this verification's
  gate list). I did not re-check the browser sha256 recorded in the spec.

## What attempt 3 needs

Fix M1 inside the engine-ceiling test's exception: add the exact destroy comparison for the open
journal, and derive the ceiling from a measured node size for the layout defect. Correct the test's
doc comment and the spec's "needs an engine-internal test" sentence. Fold m1-m3 and n1-n3.
