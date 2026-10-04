# #1274 attempt 1 verdict: PASS

PASS-WITH-MINORS: no BLOCKER and no MAJOR. Three MINORs and four NITs below. None of them blocks
closing the slice. MINOR-1 is a one-test fix and should land with the slice or in the next slice
that touches `crates/source`.

- **Commit reviewed:** `0297efa8c` ("Hold an anchored source seek until its render sample (#1274)"),
  parent `1338b063c`, from the worktree `/home/bl/misofm/wt-swap`. I exported it with `git archive`
  to `/tmp/claude-1002/v1274/attempt1/` and built and tested it there.
- **Spec:** `.github/ISSUE_SPECS/1274-hold-an-anchored-source-seek-until-its-render-sample.md`.
- **Umbrella:** P4 of `1269-swap-a-rebuilt-plan-without-an-audio-gap.md`.
- **Files changed:** six. The slice spec, `crates/source/src/lib.rs`, `crates/host-core/src/source.rs`,
  `crates/host-core/tests/successor_swap.rs`, and the two out-of-path files covered in MINOR-2.
- **Commit hygiene:** the commit uses exact paths, ends with the attribution line, and contains no
  `target/` or artifacts.

## Gates re-run in the export

All of these ran on the exported tree. None was taken from the attempt record.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh`, `test-workspace-policy.sh` | exit 0, exit 0 |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | exit 0 ("56 marked regions in 15 files"), exit 0 |
| `check-capi-abi.sh` | ok (shared and static) |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `check-cross-targets.sh` | PASS, with only the known #1018 `memset_pattern16` expected failures |
| `cargo test --locked -p source -p host-core --features host-core/test-support,graph/test-support` | 254 passed, 0 failed. This includes `..._at_eight_lanes` (AVX2 host) and `..._at_four_lanes` |
| `cargo test --locked -p capi -p host-web` | 233 passed, 0 failed |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all exit 0 |
| Browser resources | `sourceTotalBytes 3358 of 3648` (290 B free) and `graphSessionPlusPlanBytes 29794 of 35648`, both as claimed |
| **ARTIFACT CHANGED** | Confirmed. The shipped module is `751122a9ecdd05e96c8d5055bbefbb56ca4c75948d9692ade32ed01fa3959743`, 2870096 B, matching the record. The pin (`6c952a2c...`) is untouched, which is correct per `docs/RELEASE.md` "Between releases" |

## Independent mutation runs

I made a separate copy for mutations,
`/tmp/claude-1002/v1274/attempt1/mut`, and kept its definitions in `.../mutations/`. Each mutation
was applied alone. I ran `--lib --test successor_swap` for `source` and `host-core`, restored the
file and checked it with `cmp`. The final clean run was green.

| # | Mutation | Result |
|---|---|---|
| V1 | Held generation's blocks discarded (the D3 discard defect) | red: gate 1 at both widths |
| V2 | Apply when observed (any first sample) | red: gate 1 at both widths, gate 3 |
| V3 | Lateness not added | red: `a_past_anchor_starts_the_added_stem_in_time` |
| V4 | `current_matches_next_frame` without the generation compare | red: `a_primed_block_waits_for_its_anchor` |
| V5 | `prepare_seek` drops the held seek | red: `prepare_seek_holds_an_anchored_seek` |
| V6 | A newer `SeekAt` does not replace a held one | red: `a_newer_anchored_seek_replaces_a_held_one` |
| V7 | A late apply discards the primed block without noting its end | red: `a_late_anchored_seek_past_the_region_end_is_the_end_of_region` |
| V8 | Driver calls the untimed `begin_block()` | red: gates 1, 2 and 3 |
| V12 | Facade sends `Seek` for `seek_at` | red: `seek_at_refuses_an_unaligned_anchor`, gates 1, 2 and 3 |
| V13 | Alignment check removed | red: `seek_at_refuses_an_unaligned_anchor` |
| V15 | Driver clock off by one block | red: gates 1, 2 and 3 |
| V16 | `Box` allocated and freed when a `SeekAt` is observed | red: `holding_and_applying_an_anchored_seek_allocates_nothing`, "block 7: allocator calls (1, 0, 1)" |
| V17 | A held seek stops the old generation's queued PCM from playing | red: `two_playing_sources_move_to_one_block`, `prepare_seek_holds_an_anchored_seek` |
| V9 | Remove the observe-time discard of the replaced seek's pending block | **survives** (NIT-1) |
| V10 | Remove the end note in `apply_seek`'s keep branch | **survives** the whole suite (MINOR-1) |
| V11 | Remove `held_seek.is_some()` from `prepare_seek` | survives. It is an equivalent mutation: the held generation is always newer than the active one |
| V14 | `>=` to `==` in `apply_seek`'s keep test | survives. It is equivalent because of producer contiguity |

I also ran verifier probes, which are not part of the candidate:
`/tmp/claude-1002/v1274/attempt1/mutations/verifier_probe_1274.rs`. All seven are green on the
candidate:

- An anchor far behind, by more blocks than the ring holds: no panic, and every played block carries
  frame `F + (s - A)`.
- A backward anchored seek while the old generation plays: the old generation plays until the
  anchor, then the new one from `F`.
- A plain `Seek` replaces a held one: the held generation never plays.
- Anchor `u64::MAX` rounded down to the quantum, and a late frame that saturates: no panic.
- A deep anchor with a full ring: the host gets typed backpressure, and every accepted block plays
  in order.
- Two on-time end-of-region cases.

## Test value (one sentence per new test)

- **`a_future_anchor_starts_the_added_stem_on_its_block_at_{eight,four}_lanes`** fail if a held seek
  discards the primed generation (V1) or applies when observed (V2). Either way, an added stem
  starts silent or early at its anchor across a real swap, and no other test checks that
  end-to-end.
- **`a_past_anchor_starts_the_added_stem_in_time`** (both widths) fails if the lateness is not added
  (V3) or is measured on any clock but the absolute sample the swap continues (V15).
- **`two_playing_sources_move_to_one_block`** (both widths) fails if a held seek stops the old
  generation's queued PCM from playing (V17), which the added-source gates cannot see because their
  old generation is empty. It also fails if a seek applies when observed (V2).
- **`holding_and_applying_an_anchored_seek_allocates_nothing`** fails if holding or applying the
  seek allocates or frees on render (V16).
- **`seek_at_refuses_an_unaligned_anchor`** fails if the facade routes `seek_at` as a plain `Seek`
  (V12) or drops the alignment refusal (V13). It also checks that a refusal leaves the generation
  unswitched.
- **`an_unaligned_anchor_is_refused_without_a_generation_switch`** (source unit) fails if the
  refusal comes after the command is queued and the producer switched. That would be a refusal that
  still applies. It overlaps the facade test, but gate 4 requires it at the source level.
- **`a_newer_anchored_seek_replaces_a_held_one`** fails if a newer command does not replace a held
  one (V6). See NIT-1 for its second claim.
- **`prepare_seek_holds_an_anchored_seek`** fails if preparation pops and drops a `SeekAt` (V5). It
  also fails if a held seek stops the old generation playing (V17).
- **`a_late_anchored_seek_past_the_region_end_is_the_end_of_region`** fails if a late apply discards
  the primed end block without noting the region end (V7).
- **`a_primed_block_waits_for_its_anchor`** fails if `current_matches_next_frame` ignores the
  generation (V4). No integration gate catches that: there, the added source's underrun never
  reaches the primed frame early.

## Specific questions from the brief

- **Out-of-path edits.** Both are necessary and minimal; see MINOR-2.
  - `crates/host-core/tests/source_diagnostics.rs` gains one `TABLE` row and one `variant_index`
    arm. `variant_index` is an exhaustive `const fn` match, and the file documents that "adding a
    variant ... fails to build until this table gains its row".
  - `crates/source/tests/randomized.rs` gains one `unreachable!` arm in an exhaustive match on
    `SourceSeekError`. A plain `Seek` can never return `AnchorUnaligned`.
  - Without either edit, `cargo clippy --all-targets` fails to build.
- **Gate 3 without a swap.** This is acceptable. Gate 3's text ("Two playing sources moved to one
  block") does not ask for a swap, and the product outcome names moving playing sources as its own
  use. Gates 1 and 2 cover the swap.
- **Acked-batch question and the pre-existing race.** The anchored seek does not make the race
  reachable in a new way. The details are in MINOR-3, with a follow-up recommendation.
  - The only window is between `observe_seek_at_block_boundary`'s command pop and
    `acquire_current_block`'s data pop inside one `begin_block_with`. It is the same window, with the
    same precondition, as a plain `Seek`.
  - A held seek actually shields a playing source. `acquire_current_block` takes the first
    old-generation block and stops, so a new-generation block that arrives in the window stays
    queued.
  - Exposure is therefore the same as a plain `Seek` on a starved ring, which is exactly the
    added-stem case.
  - A newer `SeekAt` replacing a held one with no pending block is the same class: a block popped
    before its command.
  - The swap path adds no window. A seek issued after commit sits in the successor ring's command
    queue and is popped by the successor's first `begin_block_at` before any data pop. The carry
    `mem::swap`s the whole `PcmSourceConsumer` (`lib.rs:1841-1844`), so `held_seek` and the pending
    block move with it.
  - The other drops are all spec-sanctioned (D3), not acked-then-lost:
    - the primed blocks a late seek skips;
    - old-generation PCM still queued at the anchor;
    - a replaced seek's primed PCM.
- **The held seek allocates nothing.** It is a `Copy` field of 24 B plus the discriminant
  (`Option<HeldSeek>`). V16 and gate 5 prove it.
- **Late past the region end** reports `end_of_region` and no underrun: the unit test and V7.
- **Anchor arithmetic.** Everything is `u64`.
  - `late_by` is computed only when `first_sample >= anchor_sample`, so it cannot underflow.
  - `frame + late_by` saturates. The extreme-value probe is green.
  - The alignment check `is_multiple_of(quantum)` runs before the push, and the quantum is
    validated nonzero.
  - Production `first_sample` values are always quantum multiples:
    - The C ABI renders through `render_contiguous`, from 0 in whole quanta
      (`crates/capi/src/runtime/plan.rs:208`).
    - The browser uses its own `next_absolute_sample`, from 0 in whole quanta
      (`hosts/host-web/src/lib.rs:3229-3238`).
    - The swap adopts the clock (`plan.rs` `adopt_absolute_sample`).
  - So `late_by` is always a whole number of quanta and lands on a block start.
  - An anchor many blocks ahead holds the seek, and the host gets backpressure once the ring is
    primed. An anchor many blocks behind discards up to `transfer_block_count` blocks per render and
    keeps alignment (probe).
- **#1275 can export it cleanly.** A `SessionState::seek_at` mirrors `SessionState::seek`
  (`crates/capi/src/runtime/control.rs:1016-1028`) through `newest_providers_mut().sources.seek_at`.
  `SourceFailure::report` (`control.rs:1045-1061`) already maps the new variant generically to
  `RESULT_INVALID_ARGUMENT` + `source.seek.anchor_unaligned`. The C host already supplies the
  absolute sample it renders. No change to the exhaustive capi or browser maps is needed: both
  classify through `is_backpressure` and `is_internal`. `control.rs` is a #1053 shared touch point,
  so #1275 should keep that edit minimal.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

**MINOR-1. One branch of `apply_seek` is untested, and a plausible defect in it ships green.**
`crates/source/src/lib.rs:1356-1358` notes the region end for the kept primed block on an on-time
apply.

- Removing those three lines (V10) passes all 254 focused tests.
- My probe `probe_on_time_single_block_region_ends_cleanly` shows the effect. When the primed block
  is the region's last, short block (a region of at most one quantum from `F`, such as a short
  one-shot), every block after the stem ends reports `underrun_frames == 4` and a new underrun
  event instead of a clean end of region. `end_frame` stays `None`, so `end_reached()` is never
  true.
- **Fix:** add a source unit test next to `a_late_anchored_seek_past_the_region_end_is_the_end_of_region`.
  Hold a `SeekAt` with a 2-frame `end_of_region` primed block, render to the anchor, and assert the
  block after it has `end_of_region` and `underrun_frames == 0`. Then record V10 red in the attempt
  record.

**MINOR-2. The out-of-path edits were made instead of stopping.**
`crates/host-core/tests/source_diagnostics.rs:157-162,196` and
`crates/source/tests/randomized.rs:437-439` are outside the slice's authorized paths. `rules.md`
says "stop and report why instead".

- The edits are forced by D1's new variant: both files match exhaustively. They are the minimum
  possible and correct.
- `source_diagnostics.rs`' own header prescribes exactly this row-plus-arm procedure.
- **Fix:** amend the slice spec's authorized paths to list both files, so the record matches the
  commit. No code change is needed.

**MINOR-3. The documented promise outruns the pre-existing data-before-command race.**
This is pre-existing and not widened by this commit; I recommend a follow-up issue.

- `crates/host-core/src/source.rs:217-218` says "submit it before the anchor block renders and the
  source plays it from exactly that block".
- For an added source, which has an empty old generation, the race described above can still
  discard the primed block. The window is nanoseconds per block. The loss is accepted PCM counted as
  `stale_generation_discard_count` and is silent to the host. That is an ack preceding a drop. It
  is not caused by a swap, so it falls outside #1269's rule, and it is identical for a plain `Seek`.
- **Fix in a stateless follow-up issue:** in `acquire_current_block`, a popped block whose
  generation is newer than both `active_generation` and the held seek can only exist once its
  command has been pushed. `try_seek` pushes the command before any `submit` of that generation on
  the same producer thread, and the SPSC is release/acquire. So before discarding such a block,
  re-run the command observation for that block. This is bounded: one extra `try_pop`, at most once
  per block. Prove it with `render_while_producing`.
- Until then, either soften the sentence or leave it as is and cite the follow-up.

### NIT

**NIT-1. The observe-time discard of a replaced seek's pending block is unpinned.**
`crates/source/src/lib.rs:1319-1324` discards that block when the newer seek is observed.

- Removing it (V9) passes everything. It changes only admission depth: the replaced generation's
  queued blocks stay in the ring until the newer anchor and delay the host's priming.
- `a_newer_anchored_seek_replaces_a_held_one`'s doc also claims "Red if ... the replaced seek's
  pending block survives to play". That does not depend on this branch: `apply_seek` and the
  generation compare already prevent it.
- **Fix:** tighten the doc. Optionally assert that, after the replacement, the newer generation can
  be primed `transfer_block_count - 1` blocks before its anchor.

**NIT-2. Harness duplication.**
`render_block` and `exchange` (`crates/host-core/tests/successor_swap.rs:432-475`) duplicate the
private `render` and `exchange` in `tests/support/successor.rs:173-206`. The support file was
outside the authorized paths. **Fix:** make the support helpers `pub(crate)` in the next slice that
owns `support/successor.rs`, and delete the copies.

**NIT-3. #917 ownership doc drift.**
`crates/source/src/lib.rs:650-652` and `:975-978` still say the consumer holds a pre-fetched
`current` block "whenever its `start_frame` is ahead of `next_frame`". It now also holds a held
anchored seek's pending block, which is of another generation and may be behind `next_frame` after
a backward seek. **Fix:** add "or a held anchored seek's pending block".

**NIT-4. Two small cleanups.**
- `.expect("checked current block")` on the render path (`lib.rs:1322`) could be
  `self.current.take_if(..)`, which removes a panic site.
- The `held_seek.is_some()` guard in `prepare_seek` (`lib.rs:1051`) is logically redundant (V11 is
  equivalent). It is harmless as defense in depth.

## Observation

`sourceTotalBytes` now has 290 B of headroom under its 3648 B ceiling. Each new per-consumer field
costs `size_of` × the source count (here +64 B for the held seek). Later slices that add consumer
state should expect to touch that ceiling.
