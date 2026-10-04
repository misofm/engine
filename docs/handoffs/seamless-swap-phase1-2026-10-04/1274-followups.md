# #1274 attempt 1 follow-ups verdict: PASS

PASS-WITH-MINORS: no BLOCKER and no MAJOR. One MINOR and four NITs, none blocking. The race fix
(attempt-1 MINOR-3) is sound. I checked it by reasoning, by deterministic probes and by a two-thread
stress probe:

- With the fix, the stress probe lost 0 acked blocks in 30 runs, each with about 4,000 hits in the
  race window.
- Without the fix, it lost 2998 of 2999 (plain seeks) and 998 of 999 (anchored seeks).

- **Commit reviewed:** `c14fde0ce` ("Close #1274 attempt-1 minors: keep PCM popped before its
  seek"), parent `807b48547` (#1276, ignored). It comes from `/home/bl/misofm/wt-swap` and was
  exported with `git archive` to `/tmp/claude-1002/v1274/followups/`. Every gate below ran on that
  exact tree.
- **Files:** three, all authorized.
  - The slice spec.
  - `crates/source/src/lib.rs`.
  - `crates/host-core/src/source.rs`, a doc change only.
- **Hygiene:** the commit uses exact paths, ends with the attribution line, and has no artifacts.

## Gates re-run on the exported commit

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | exit 0 / exit 0 |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | exit 0 ("58 marked regions in 16 files"; the +2/+1 come from #1276) / exit 0 |
| `check-capi-abi.sh` | ok (shared and static) |
| `audit capi` (release) | 100000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `check-cross-targets.sh` | PASS. The only failures are the known #1018 `memset_pattern16` expected ones |
| `cargo test --locked -p source -p host-core --features host-core/test-support,graph/test-support` | 268 passed, 0 failed, 2 ignored. That is 256 plus #1276's 12 host-core tests, including both widths |
| `cargo test --locked -p source` (all targets, including `randomized`) | exit 0 |
| `cargo test --locked -p capi -p host-web` | 233 passed, 0 failed. This includes the pinned capi and host-web digests |
| `cargo test --locked -p parameter-metadata` | exit 0. It is the only other dependent of `source`, besides `audit` |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all exit 0. `browser-correctness expected.json digests and exact rows agree`. `sourceTotalBytes 3358 of 3648` and `graphSessionPlusPlanBytes 29794 of 35648`, both unchanged |
| **ARTIFACT CHANGED** | Yes. The committed tree's shipped module is `17429f4702f300c5100dce4b97f9fbd75127bc481a454b2d741ac67f380a1c7c`, 2914664 B, and it includes #1276. The pin is correctly left alone (`docs/RELEASE.md` "Between releases"). See NIT-3 |

The console and conformance digests are not affected. `cargo tree -i source` shows that only
`capi`, `audit`, `host-core`, `host-web` and `parameter-metadata` (dev) depend on `source`.
`conformance` and `console-workload` do not, so their digests cannot move. Every digest-bearing test
in the dependents is green.

## Mutations (each applied alone; restored and checked with `cmp`; clean suite green after)

These ran in `/tmp/claude-1002/v1274/followups/mut` with `cargo test -p source --lib`. My probes
(below) were appended to the module. The definitions are in
`/tmp/claude-1002/v1274/followups/mutations/mutate.py`.

| # | Mutation | Candidate tests red | Verifier probes red |
|---|---|---|---|
| F2 | Remove the re-observe in `acquire_current_block` (pre-fix code) | `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm` | all 8 race probes; the threaded stress loses 2998/2999 (plain) and 998/999 (anchored) |
| F2-plain / F2-anchored | F2 with the test reduced to one arm | red either way, so each arm catches it alone (confirms the record) | n/a |
| F1 (V10) | Drop the end-of-region note in `apply_seek`'s keep branch | `an_on_time_anchored_seek_to_a_last_short_block_ends_its_region` only | none |
| F3 (V9) | Keep the replaced seek's pending block | `a_newer_anchored_seek_replaces_a_held_one` only | none |
| V6 | A newer `SeekAt` does not replace a held one | `a_newer_anchored_seek_replaces_a_held_one`, so the rewritten test keeps its original catch | `probe_anchored_seek_in_window_replaces_held_anchored` |
| M7 | Re-observe, but still discard the popped block | `a_seek_whose_pcm_arrives_inside...` | 7 probes |
| M4-now | Re-observe with `SeekClock::Now` instead of the block's clock | `a_seek_whose_pcm_arrives_inside...` (the anchored arm applies early) | 2 probes |
| **M4-hold** | Re-observe with `SeekClock::Hold` instead of the block's clock | **survives the candidate suite** | `probe_late_anchored_seek_in_window_applies_in_time`. **MINOR-1** |
| M5 | `is_unobserved_generation` ignores the held seek | survives | survives. Benign: it only observes an already-admitted command earlier, still in FIFO order and still at most once per popped block |
| V11 | Remove `held_seek.is_some()` from `prepare_seek` | survives | `probe_prepare_playing_generation_with_held_seek_is_not_ready`. **NIT-2** |

### Verifier probes

The probes are not part of the candidate. They are in
`/tmp/claude-1002/v1274/followups/mutations/verifier_probes.rs`, and all ten are green on the
candidate.

- **Late anchored seek in the window.** It applies at once, in time: block 8 plays frame 104.
- **A window seek replaces a held anchored seek.** Tested with a plain seek and with an anchored
  one. The replaced generation never applies, and nothing is discarded.
- **A second `try_seek` inside the window gets `Backpressure`.** The one-slot invariant holds.
- **`u64::MAX` generation in the window.** It plays with no wrap.
- **The race branch allocates and frees nothing.** Checked with thread-scoped counting, over 20
  plain and anchored rounds.
- **Threaded stress, plain and anchored.** A real producer thread races the render. The render is
  "preempted" by a spin between `begin_block_observe` and `begin_block_play`.
- **`prepare_seek`.** Two probes cover the guard rationale and the window, for NIT-1 and NIT-2.

My first stress version showed a rare single discard on the candidate. Tracing it showed a probe
artefact, not the race. The producer was descheduled between `try_seek` and `submit`. The plain seek
was applied, the source underran past frame 0, and the late block was correctly discarded as behind
`next_frame` (pre-existing JIT rule, counted). The probe now counts only discards whose seek was
still unobserved at the data pop. On the candidate that count is 0 in 30 of 30 runs.

## Test value (one sentence per new or rewritten test)

- **`an_on_time_anchored_seek_to_a_last_short_block_ends_its_region`.** It turns red if an on-time
  anchored apply keeps the primed block without noting the region end it carries. Such a stem
  underruns forever after its last short block instead of reaching the end of region. F1 makes only
  this test red.
- **`a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm`.** It turns red if the render
  discards, as stale, an acked block whose seek command it has not popped yet. This is the
  ack-before-drop race. Either arm alone catches it, and nothing else in the suite does (F2).
- **`a_newer_anchored_seek_replaces_a_held_one` (rewritten).** It turns red if observing a newer seek
  keeps the replaced seek's pending block. That block then holds the newer generation one block
  short of its admission depth before its anchor. F3 makes only this test red. The rewrite still
  catches V6, and its corrected doc no longer overclaims.

## The race fix (MINOR-3), point by point

**Why one `try_pop` is enough.**

- `try_seek` advances the producer's generation only after the command push succeeds.
- `submit` refuses any generation but the producer's own. So for every generation `G`, the push of
  command `G` is sequenced before the push of any `G` block, on the single producer endpoint.
- The command queue has one slot, so command `G` can be pushed only after the consumer has popped
  every earlier command.
- `max(active_generation, held.generation)` never decreases. `apply_seek` and a held seek's apply
  only raise it, and a replacing command is always newer.
- So when a popped block's generation is above both values, its own command is the only unobserved
  command, and it must still be in the queue.
- It is also visible to the consumer.
  - The consumer's `Acquire` load of the data cursor synchronizes with the producer's `Release`
    store. This holds even when the pop is served from `cached_producer`: the cache was filled by
    an earlier `Acquire` load of that store or a later one.
  - The command cursor's `Release` store is sequenced before the data push, so by coherence the
    consumer's later `Acquire` reload in `is_drained` sees it.
  - This is the language memory model, not x86 TSO, so it holds on AArch64 too.
- If `try_pop` ever came back empty, the code would fall back to the old discard. It never pops a
  wrong command.

**Multi-command cases.**

- *A second seek queued behind.* This cannot happen: the slot is full until the consumer pops, and
  the probe shows `Backpressure`.
- *Plain replaces anchored.* `held_seek = None` and the new seek applies (probe).
- *Anchored replaces anchored.* The new seek is held, and its block is kept pending. The replaced
  generation's leftovers fail `is_unobserved` and are discarded as stale (probe).
- *Anchored after plain.* This is the candidate test's anchored arm.
- *Late anchored.* It applies at once with the lateness (probe). See MINOR-1.
- *A held seek with its pending block in `current`.* `acquire` returns early, so nothing is popped
  and the newer command waits for the next boundary, as before.
- *Generation wrap.* This cannot happen: `try_seek` refuses a generation that is not strictly
  increasing, and all comparisons are `u64`.

**Bounded, zero allocation.**

- The loop is still `0..transfer_block_count`, with at most one extra command pop per popped block.
  Observe and apply are O(1).
- A discard recycles into a preallocated queue, whose push never fails.
- The allocation probe confirms 0 allocations and 0 frees, and `audit capi` reports 0.

**Plain seeks are unchanged outside the race.**

- The branch needs a producer push between a call's command pop and its data pop.
- In a single-threaded host (the browser's `seek_source` and `render_next`), every observe is
  immediately followed by its acquire in the same call, so the branch is unreachable there.
- The browser expected.json digests agree, and every capi, host-web and host-core digest test is
  green.

**The split is neutral.** `begin_block_with` still runs `end_block`, `flush_deferred_recycle`,
`generation_changed = false`, observe, then acquire, in the old order. `acquire` only gains the
clock argument for the re-observe.

**Gate 2 interaction.** The fix also covers #1275's multi-threaded C ABI host, where a `seek_at` and
its first `submit` can land inside one render's window.

## Other follow-ups

- **MINOR-1 test.** Correct. F1 makes it red alone.
- **MINOR-2 spec amendment.** It lists both forced files with their scope ("table row and
  exhaustive-match arms only"). This is correct.
- **NIT-1 pin.** It pins the discard through admission depth, not a private counter. This is good.
- **NIT-3 docs.** Both #917 blocks now name the held seek's pending block. This is accurate.
- **NIT-4 `take_if`.** It removes the `expect` panic site and is behaviour-identical.
- **NIT-4 kept guard.** The rationale is correct. Without the guard,
  `prepare_seek(playing generation, next_frame)` returns `true` while a newer anchored seek is held.
  The guard is not pinned (NIT-2).

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

**MINOR-1. The late-anchor race path is untested, and M4-hold ships green.**

- **Where.** `crates/source/src/lib.rs:1395-1402` passes the block's `clock` to the re-observe. The
  clock is the only thing that makes a past-anchor `SeekAt` popped inside the window apply now, at
  `frame + late`.
- **Defect.** Replacing it with `SeekClock::Hold`, which is what `prepare_seek` passes, survives the
  whole candidate suite.
- **Effect.** An added stem whose anchor has already passed (gate 2's case in a threaded C ABI
  host) renders one block of silence and an underrun instead of frame 104. My probe shows
  `[0.0; 4]` against the expected `[2.5; 4]`.
- **Fix.** Add a third arm to
  `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm`:
  - `seek_at(2, 100, 4)` inside the window at `SeekClock::At(8)`;
  - submit frames 100 and 104;
  - assert that block 8 plays 104's PCM with `generation_changed`;
  - assert `stale_generation_discard_count == 1` (the late-skipped block).

  Record M4-hold red in the attempt record.

### NIT

**NIT-1. A `prepare_seek` call can now observe an anchored seek and still return `true`, against
D4 and its own doc (`lib.rs:1045-1061`).**

- **Cause.** The verdict is computed before `acquire_current_block(SeekClock::Hold)`. If a producer
  pushes a `SeekAt` and its block between prepare's command pop and its data pop, the re-observe
  holds the seek and keeps the block pending, but the call returns `true`.
  `probe_prepare_seek_window_observes_anchored_seek` shows this. A newer plain seek inside the
  window likewise leaves the source on a newer generation than the one `true` was granted for.
- **Severity.** It is reachable only with a producer running concurrently with `prepare_seek`.
  - The browser is single-threaded.
  - The C ABI never calls `prepare_source_seek`.
  - Before this commit, the same window silently lost the acked block, so this is strictly better.
- **Fix.** Either return `self.held_seek.is_none() && self.active_generation == generation` after
  the acquire, or amend the doc and D4 to say "a `SeekAt` observed before preparation". Add one
  line to the spec's decisions for the new race rule, which is a behaviour change for plain seeks
  too.

**NIT-2. The kept guard is unpinned.** The `held_seek.is_some()` test in `prepare_seek` is justified,
because V11 is not equivalent, but no candidate test fails without it. **Fix (optional):** adopt
`probe_prepare_playing_generation_with_held_seek_is_not_ready` (8 lines) as a unit test.

**NIT-3. The attempt record's evidence describes a tree that is not the commit.** The follow-up's
gates ran on `0297efa8c` plus two files. `c14fde0ce` also contains #1276. So the recorded counts and
module identity are not this commit's:

| | Recorded | This commit |
|---|---|---|
| Tests passed | 256 | 268 |
| Module | `cdb03d38...`, 2870165 B | `17429f47...`, 2914664 B |

Everything is green on the real commit (this verdict). **Fix:** name the measured tree in the
record, or add the committed tree's module hash.

**NIT-4. Formatting.**

- `crates/source/src/lib.rs:980` is 125 columns, and it is the only line in the file over 100.
  rustfmt does not wrap comments.
- `crates/host-core/src/source.rs:218-219` leaves a ragged line ("... before it pops the seek.
  Should the").

**Fix:** reflow both.

## Observation

The probe artefact above is the pre-existing plain-seek rule. A block submitted after its plain
seek was applied and the source has underrun past its frame is acked and then discarded, but it is
counted in `stale_generation_discard_count`. This is the documented JIT behaviour ("underrun emits
zero plus a counter"), and the anchored seek is the remedy for planned starts. It is not a finding
against this slice.
