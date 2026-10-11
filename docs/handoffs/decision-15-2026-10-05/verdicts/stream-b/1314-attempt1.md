PASS

# #1314 attempt 1: verdict (adversarial verifier, opus-xhigh)

Reviewed: `git diff 646d6247d 899db1be7` on `codex/d15-stream-b` (*Publish an applied-revision
watermark and complete edits asynchronously*). The commit was exported with `git archive` to
`/tmp/claude-1002/v1314/tree` and built there with `CARGO_TARGET_DIR=/tmp/claude-1002/v1314/target`.
The worktree was not touched.

Verdict: no BLOCKER and no MAJOR. The design is correct as built. The revision words travel with
their plans. Render reads only its own `Active` cell, after the adoption decision and before any
drain. The seqlock is the standard fenced pattern, with the same orderings as `observe.rs`. Each
commit path writes the revision last. The C ABI matches the spec. There are five MINOR findings.
F1 breaks a required CI step and must be fixed before the batch push. F2, F3 and F4 are test gaps
around requirements that the code meets. F5 is a spec problem that root must authorize.

## Findings

### MINOR F1: `cargo doc -D warnings` fails on a new private intra-doc link (must fix before push)

`crates/engine/src/realtime/watermark.rs:85`: the public `WatermarkBusy` doc links
[`MAXIMUM_READ_ATTEMPTS`], which is a private const. `RUSTDOCFLAGS='-D warnings' cargo doc --locked
--workspace --no-deps` is the "Documentation" step of the required `qualification.yml` lint job
(`:459-460`). It fails with:

```
error: public documentation for `WatermarkBusy` links to private item `MAXIMUM_READ_ATTEMPTS`
```

The fix is to write the name in plain backticks. I made that one-line change in the export only:
the next failure is then a pre-existing one that this slice did not cause. `crates/capi/src/runtime/mod.rs:148`
links the private `limits_are_valid`, which came in with #1309 (`18f3ca30c`). Neither the #1309
verdict nor the #1343 verdict ran `cargo doc`, so the branch already failed this step before #1314.
Both links must be fixed in the follow-ups before the batch push. **For the coordinator:** the #1309
one is a cross-slice item.

### MINOR F2: no test guards D3's load placement; gate 3's named defect is not red

D3 requires render to load the `Active` revision word before `render_inner`
(`plan_exchange.rs:735` and `:759`). Mutation M14 moved both loads after `render_inner`. The
engine lib, capi lib, capi integration tests and loom stayed **all green**. The spec's gate-3 test
value names this exact defect, but gate 3 is specified as a one-thread test. A single-threaded test
cannot commit an edit while `render_inner` runs, so as written the spec cannot satisfy that clause.

The attempt record says "the loom model holds the store/load pairing". That is true only for the
mailbox's `Release` store and `Acquire` load (L1 and L2 below). The loom model never touches
`plan_exchange.rs`, so the placement is not covered. Reading the code confirms the placement is
correct today.

Recommendation, foldable into follow-ups because capi tests are in the authorized paths: add a
concurrent C ABI test on `bench_support::producer::render_while_producing`. A control thread
toggles a track's mute live while render runs. Whenever a block advances the watermark to `r`
(`first_sample` equals that block's first sample), the block must reflect `r`'s mute state. Under
M14 an edit committed during `render_inner` is reported one block early. The alternative is for
root to amend gate 3's test value.

### MINOR F3: "a block that errors publishes nothing" (D3) has no test

Mutation M15 called `advance` before propagating `render_inner`'s error, in both render paths.
Every suite stayed green. A deterministic engine test is easy:
1. Publish A at revision 7.
2. Render one block that `render_inner` rejects (for example, a wrong output shape). The
   watermark must stay at the initial revision.
3. Render the next valid block. It must publish `(7, that block's first sample, EXACT)`.

### MINOR F4: the reader's cached `Active` index after a lost claim is unguarded

`spsc.rs:849` sets `self.active = observed.full` only after a successful claim compare-and-swap.
That is correct. Mutation L3 moved the assignment above the compare-and-swap, so a claim that loses
to `withdraw` leaves render pointing at the withdrawn, now `Empty`, cell. Render would then report
the withdrawn candidate's revision, and later the revision of whatever is republished there,
before adoption. This is the hazard the spec names for D3. Loom, the engine suite and the capi
suite all stayed green.

Recommendation: extend `spsc_loom_plan_mailbox_claim_races_withdrawal` (`spsc.rs:952`). Write a
distinct revision into the candidate's cell, and when the withdrawal wins, assert that
`reader.active_revision()` is still the initial cell's revision.

### MINOR F5 (spec problem for root): realtime-policy floors not raised

`scripts/check-realtime-policy.sh` passes: `realtime policy: ok (91 marked regions in 26 files)`.
`watermark.rs` is under the scan, because files are found by their `REALTIME_POLICY_BEGIN` marker
(`rg -l`). The forbidden-body predicate therefore applies to `note_claim`, `advance` and `store`.
`active_revision` (`spsc.rs:808`) and the render paths are inside regions that already existed.

The floors are still 25 files and 89 regions (`:73-74`). Deleting the new marker would therefore
pass the gate. The script's own rule (`:72`) is "Raising a floor is part of the change that adds a
marker", so the raise is required, not optional. The script is outside #1314's authorized paths,
so root must authorize raising the floors to 26 files and 91 regions in the follow-ups. The branch
was already one region over the floor at the parent `646d6247d` (25/90, from #1309 or #1343); main
is at exactly 25/89.

### NIT N1: claim accumulation in `note_claim` is untested

Mutation M17 overwrote the pending values instead of adding to them, and nothing went red. Two
claims between two advances cannot happen through the C ABI today: a second rebuild needs
`pending_providers` to be empty, and that needs a successful block after the first adoption. Also,
when the claims are merged and one of them is a fallback, every remaining revision is counted as a
fallback. Either test this or document that it cannot happen.

### NIT N2: D2's debug-assertion text in the spec should be amended (spec text)

See deviation 2 below. The direction in the spec is racy. The implementation uses the sound
converse. Root should correct the text of D2.

### NIT N3: the reserved-word and BACKPRESSURE paths are only partly exercised

`plan_watermark_refuses_reserved_words_and_is_pure` (`ffi.rs:2575`) poisons only `reserved0` and
`reserved[4]`. The `RESULT_BACKPRESSURE` path (`out` untouched) is not exercised through the ABI.
Both paths are correct by inspection.

### NIT N4: seqlock code is duplicated

The `watermark.rs` seqlock duplicates `observe.rs`, including its `MAXIMUM_READ_ATTEMPTS` const.
The spec asked for a new module, so this is acceptable. If #1316 reuses "the watermark's
primitive", it is a candidate for one shared primitive.

## The declared deviations, judged

1. **`plan_exchange()` stays at revision 0, and `plan_exchange_at_revision()` is added. Acceptable,
   not a shortcut.** The revision-less form is complete and documented for a host that numbers no
   revisions (`plan_exchange.rs:364-371`). Its only callers are tests and `tools/audit`; I checked
   that no production `src/` calls it. The only production exchange, `compile.rs:761`, uses
   `_at_revision` with `store.revision()`. Nothing about it is a placeholder or needs removing
   later. Collapsing to one constructor would need edits outside the authorized paths, which is
   root's call and not needed for correctness.
2. **D2's assertion direction. Correct, and the race claim is verified.** `command()` calls
   `synchronize_plan_epochs` first. `pending_providers` is pruned only when render's `active_epoch`
   store is seen, and render makes that store after a successful block (`plan.rs:259-262`). Render
   can adopt between the synchronize and `set_revision`. `store_revision` then correctly writes the
   now-`Active` candidate cell while the candidate is still recorded, so the spec's direction would
   fire falsely. The converse is sound: `Pending` means a `Full` cell, and the rebuild path records
   the provider (`control.rs:1159`) before `reservation.commit()`. The spec text needs amending
   (N2).
3. **A reservation without `set_revision` carries the publisher's newest revision. Acceptable.** An
   engine-level publication that commits no revision adds none. The C ABI always sets the revision
   (`control.rs:1163`), and gate 2 goes red if it does not (M10).
4. **`PlanReplacementReservation::set_superseded`. Acceptable.** It is a setter for #1310's producer,
   used by gate 4, and is not itself a producer. It writes only before publication, as D1 requires.
   `superseded` and `outcome` are carried in the cell's payload instead of separate words. They
   live in the cell's memory and are charged with it through `Layout::<SharedMailboxAllocation<T>>`,
   so this is equivalent to D1.
5. **Gate 2 versus a session-wide word. Accepted as declared, with one correction.** Mutation M2
   made one word for all cells. Gates 1 and 8 went red and the capi gate-2 test stayed green, as
   declared: one thread adopts at the next block, so no old-plan block falls in between. The loom
   claim does not cover the `render_inner` placement (F2).

## Scrutiny items

- **Seqlock (`watermark.rs`).** Safe Rust, one writer (render). The writer stores the odd counter
  (`Relaxed`), then `fence(Release)`, then the six `Relaxed` words, then the even counter
  (`Release`). The reader loads the counter with `Acquire`, then the `Relaxed` words, then
  `fence(Acquire)`, then the counter again with `Relaxed`. This is Boehm's fenced seqlock and
  matches `observe.rs:131-184` exactly. A read is bounded at 64 attempts and then returns
  `WatermarkBusy`. A torn read is impossible: if any word read came from a later publication, the
  fence pair orders that publication's odd counter store before the second counter load, so the two
  counter values differ. The x86 stress test cannot see a missing fence (TSO), but the orderings
  match a pattern already reviewed.
- **Render cost.** Each block adds one `Acquire` load and one compare. An advance adds eight atomic
  stores and one fence. A claim adds two register updates. There is no allocation, lock or syscall,
  and the advance happens only after `render_inner` returns `Ok`. Evidence: `audit capi` reports 0
  allocations, deallocations, locks and syscalls, with `"total_violations":0`, while its live edits
  and its carrying swap advance the watermark. The realtime, graph and builtins-graph syscall traces
  (1,000,000 blocks) pass.
- **D8 (can an ack precede a drop?).** No queue is added, and the watermark is a level. The revision
  store comes after every push, target publication, `commit_owner` and the readback
  (`control.rs:1492`, read in full). The rebuild sets the reservation's revision before `commit()`.
  The response is written afterwards. A withdrawn candidate takes its word with it, which M3 checks.
  An ack never precedes a drop.
- **C ABI.** 96 bytes, with `struct_size`, `reserved0` and `reserved[5]` checked before any write.
  Busy returns `RESULT_BACKPRESSURE` with `out` untouched. The query reads only `plan_queries`, so it
  is pure, which the test checks. Thread text: any thread, like `plan_resources`. The feature bit is
  `1 << 6`, the mask is 127, and the outcome macros are 1, 2 and 4, const-asserted equal to the
  engine's values. Bit 6 is free on main and on every `d15-stream-*` branch. The frozen list has 16
  symbols, and `C_ABI_V1_QUALIFICATION.md` says 16. The header's D7 paragraph has every required
  clause (synchronous only for what can fail, pending until the watermark covers a revision, a
  paused host stays pending, poll the watermark and never the event lane) and sits beside the
  live-edit paragraph.
- **Version suffixes.** The ABI identities (`miso_engine_v1_plan_watermark`,
  `miso_engine_v1_watermark`, `MISO_ENGINE_V1_*`) carry V1. The internal names (`PlanWatermark`,
  `WatermarkWriter`, `RevisionTarget`, `CandidateOutcome`) have no version.

## Test value (one sentence per new test)

- `a_revision_completes_at_adoption_not_at_publication` (`mod.rs:621`, gate 1): red if render reads
  a cell other than `Active` (M1), if the revision is one session-wide word (M2), if a withdrawn
  candidate loses its revision (M3), or if render publishes on every block (M13).
- `superseded_and_fallback_revisions_complete_once_per_adoption` (`mod.rs:661`, gate 4): red if
  `superseded` is added on every advance (M4), if `exact` is not reduced by it (M4b), or if a
  withdrawn candidate's outcome does not travel with it (M5).
- `a_revision_follows_a_held_or_published_candidate` (`mod.rs:734`, gate 8): red if a revision
  committed while a candidate is published goes to the `Active` cell (M6), or if a held candidate's
  `set_revision` drops it (M7).
- `a_watermark_read_is_one_whole_publication_or_busy` (`watermark.rs`, gate 5): red in the CI debug
  profile if a word is stored after the closing counter (M8a, 3 of 3 runs) or if the reader skips
  its second counter check (M8b, 5 of 5 runs).
- loom `spsc_loom_plan_mailbox_revision_follows_the_pending_candidate`: red if the revision store
  (L1) or the render load (L2) is `Relaxed` ("revision 8 read without its record").
- `a_live_edit_on_a_pending_candidate_completes_at_the_swap` (`live_tests.rs:842`, gate 2): red if
  the C ABI path writes a pending revision to the `Active` cell (M6, capi) or if the rebuild does not
  set the reservation's revision (M10).
- `a_live_edit_completes_in_the_next_block` (`live_tests.rs:908`, gate 3): red if the live commit
  stores no revision (M12) or if render publishes on every block (M13). It is not red on the load
  placement (F2).
- `plan_watermark_refuses_reserved_words_and_is_pure` (`ffi.rs:2575`): red if the reserved words are
  not checked (M19).
- `abi_smoke.c` and `header_smoke.cpp` (gate 7): red if the bit is left out of `FEATURE_MASK`
  (`check-capi-abi.sh` rc 1) or if the header swaps `exact_count` and
  `transition_fallback_count` (`static_assert` offsets 40 and 32).

No pre-existing test caught any named mutation. In every run, only the new tests listed above went
red across the engine lib, the capi lib and the capi integration tests.

## Mutation runs (verifier re-runs)

| ID | Mutation | Result |
|---|---|---|
| M1 | `active_revision` reads `revisions[1 - active]` | red: gates 1, 4 and 8; capi gates 2 and 3 |
| M2 | one session-wide word (all indices 0) | red: gates 1 and 8; capi green (deviation 5) |
| M3 | withdrawal returns revision 0 | red: gates 1 and 4 |
| M4 | `pending_superseded` not taken | red: gate 4 |
| M4b | `rest = covered` | red: gate 4 |
| M5 | withdrawn outcome reset to `Exact` | red: gate 4 |
| M6 | `store_revision` writes `Active` while a cell is `Full` | red: gate 8; capi gate 2 |
| M7 | `UnadoptedCandidate::set_revision` is a no-op | red: gate 8 |
| M8a | `superseded` stored after the closing counter (debug) | red: gate 5, 3 of 3 runs |
| M8b | reader skips the second counter check (debug) | red: gate 5, 5 of 5 runs |
| L1 | `store_revision` `Relaxed` (loom) | red |
| L2 | `active_revision` `Relaxed` (loom) | red |
| M10 | rebuild does not set the reservation's revision | red: capi gate 2 |
| M12 | live commit stores no revision | red: capi gates 2 and 3 |
| M13 | `advance` uses `<` instead of `<=` | red: gates 1 and 8; capi gates 2 and 3 |
| M19 | query skips the reserved-word check | red: `plan_watermark_refuses_reserved_words_and_is_pure` |
| mask | `FEATURE_PLAN_WATERMARK` left out of `FEATURE_MASK` | red: `check-capi-abi.sh` rc 1 |
| header | `exact_count` and `transition_fallback_count` swapped | red: `header_smoke.cpp` |
| **M14** | revision loaded after `render_inner` (both paths) | **all green, including loom (F2)** |
| **M15** | advance before the render error propagates | **all green (F3)** |
| **L3** | `self.active` set before the claim compare-and-swap | **all green, including loom (F4)** |
| **M17** | `note_claim` overwrites instead of adding | **all green (N1)** |

## Gates run (x86_64 Linux, export of `899db1be7`)

| Gate | Result |
|---|---|
| `cargo test --locked -p engine --features engine/realtime-audit` | ok (48 + 4 + 1) |
| loom `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` | 5 passed |
| `cargo test --locked -p capi` | ok (76 + 2 + 11) |
| `cargo test --locked -p control-plane --features test-support` | ok (0 tests) |
| CI workspace debug test command (`qualification.yml:609-619`, all listed features) | rc 0, 1451 passed, 0 failed |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (91 regions, 26 files) / ok |
| `check-capi-abi.sh` / `--self-test` | ok (shared and static) / ok |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, `total_violations` 0 |
| `trace-realtime-audit.sh` (1M), `trace-graph-audit.sh`, `trace-builtins-graph-audit.sh` | ok, PASS, PASS |
| `check-cross-targets.sh` | PASS |
| `check-workspace-policy.sh` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (also without `--all-features`) | clean |
| `cargo fmt --all -- --check` | clean |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | **FAIL** (F1, plus the pre-existing #1309 link) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all ok; module `87c03c64...1e28cf`, the same as the attempt record |

Not run: AArch64 (CI only, per the rules), and the browser `npm run qualify` legs, which belong to
the batch verifier.
