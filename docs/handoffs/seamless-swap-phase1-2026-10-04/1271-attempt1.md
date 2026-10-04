# #1271 attempt 1 verdict: PASS

PASS with minors: no BLOCKER and no MAJOR findings. Seven MINOR findings and three NITs are listed
below. Most are untested defensive branches. The implementation is correct as written.

## Commit reviewed

- `git diff 22c9bd5a1 1a911e31f` (commits `5488fb2f5` and `1a911e31f` on parent `22c9bd5a1`, #1270),
  exported with `git archive 1a911e31f` to `/tmp/claude-1002/v1271/attempt1/`. All builds and tests
  ran in that export. Mutations ran in a second export, `/tmp/claude-1002/v1271/attempt1-mut/`,
  each with its own `target/`. I did not build in or write to `/home/bl/misofm/wt-swap`. I did not
  judge the concurrent #1272 work there.
- Paths: `crates/source/src/lib.rs`, `crates/graph/src/lib.rs`, `crates/engine/src/realtime/plan.rs`
  (accessor only, 9 lines), the slice spec, and `crates/graph/tests/rt11_swap_carry_alloc.rs`
  (coordinator amendment). Everything is in the authorized set. `crates/graph/src/runtime.rs` is
  untouched. Both commits end with the required `Co-Authored-By` line. No `target/` or artifacts
  were committed.

## Gates re-run (results)

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | green |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | green |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | green |
| `check-workspace-policy.sh`, `test-workspace-policy.sh` | green |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | green (both new render bodies sit in `REALTIME_POLICY` regions) |
| `check-capi-abi.sh` | green |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `scripts/trace-graph-audit.sh target/release/audit` | PASS (1000000 blocks) |
| `cargo test --locked -p source -p graph --features graph/test-support` | green: graph lib 90, rt1/rt9/rt10/rt11, source lib 24 (five new `tests::carry::*`), randomized, seek model |
| CI `test-debug-a` command, verbatim (workspace, `--all-targets`, the seven test-support/realtime-audit features) | see "Late results" |
| Does CI reach rt11? | yes. `ci-path-router.py` routes both changed paths to `full`, and `test-debug-a` runs `--workspace --all-targets` with `graph` not excluded. In my run of that exact command, `Running tests/rt11_swap_carry_alloc.rs` printed `the_swap_block_carry_allocates_and_frees_nothing ... ok`, so it passes with `engine/realtime-audit` and every other CI feature on. |
| `check-cross-targets.sh` | see "Late results" |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) | see "Late results" |

## Interpretations the coordinator asked about

- **Graph identity.** `next_graph_identity()` is `NEXT_GRAPH_IDENTITY.fetch_add(1, Relaxed)` from 1.
  It is called only from `GraphExecutor::new`, and only `bind_with_source_set` calls that (the other
  call is in a runtime test module). So it never runs on render. An atomic RMW gives unique values
  whatever the ordering, so the identity is unique across every engine in the process (several C ABI
  engines share the counter) and nonzero until 2^64 binds. On wasm32 without `+atomics` it lowers to
  plain loads and stores. Accepted (P11, D2). Gate 4 asserts two identities differ and are nonzero.
  Mutation M12 (constant identity) turns gate 4 red.
- **`carry_program_retained_bytes` instead of a charge at bind.** Accepted as a reading of D3. No
  plan report exists after bind (the graph estimate is admitted before bind). Host-core already
  reads `PreparedRenderPlan::observation_retained_bytes()` after bind and adds it to its own report,
  and the accessor follows that precedent. It is a hand-off risk, though: see MINOR-6.
- **Non-graph predecessor returns `NotRequested`.** This matches D4 step 1 word for word, and it is
  unreachable on both hosts. It still conflicts with #1270's own `CarryOutcome::PredecessorMismatch`
  doc, and nothing tests it. See MINOR-4.
- **All-or-none at the swap block.** Reading the code, it holds. `adopt_sources` checks every move
  against the pre-swap state before any swap: quantum, both indices in range, successor vacant,
  predecessor occupied, equal channel counts. Install already refuses repeated successor and
  predecessor indices, so the swap loop touches distinct entries and cannot half-apply. A failure
  returns `false` before any `mem::swap`, and the executor reports `PredecessorMismatch`. The
  identity check runs before the source sets are touched. No committed test defends it, though: see
  MINOR-1.
- **A vacant entry refuses seek preparation.** Correct. `can_prepare_source_seek` is false, and
  `prepare_source_seek` cannot reach a consumer. This is the safe choice: before the swap, a host's
  seek is prepared on the active predecessor, and the pending flag is carried (gate 3). Untested:
  see NIT-1.
- **A second install replaces the first; a refused install keeps the earlier program.** Correct:
  validation runs before `executor.carry = Some(program)`.
- **`sourceTotalBytes` +8 B per entry.** `GraphSourceEntry` gains a `u32` `channel_count`, padded
  to 8; `Option<PcmSourceConsumer>` uses a niche. The size is charged automatically through
  `allocation_class::<GraphSourceEntry>`. The figure is checked in the worklet chain (Late results).
- **ARTIFACT CHANGED, pin untouched.** This matches `docs/RELEASE.md` "Between releases": nothing
  re-pins between releases.

## Realtime and correctness

- **Swap-block carry.** It downcasts through `as_any_mut`, compares identities, makes two bounded
  passes over `moves`, swaps `Option<PcmSourceConsumer>`, and ORs one flag. No allocation, free,
  lock or syscall. Neither plan's ring is dropped: the successor's `None` is swapped out, the
  predecessor keeps `None`, and the retiring plan frees nothing it carried.
- **Independent proof on the real driver.** I added a scratch test in the mutation export (not part
  of the candidate). It binds two real `PcmSourceRing` sources, carries both through `plan_exchange`
  with `bench_support::alloc` linked, and runs 4 warm blocks first. The swap block gave
  `(allocations, deallocations) == (0, 0)`, `Applied`/`Carried`, and nonzero output. So the claim
  holds for the shipped driver too.
- **Accepted PCM.** The carried ring keeps its queued blocks, generation and read position (gate 1
  is bit-exact). A refused carry leaves the ring in the predecessor. P11 and #1272's D6 call that
  case unreachable and say it renders `+0.0`. Nothing in this slice acknowledges anything, so no ack
  can precede a drop here.
- **Vacant entry.** It renders `+0.0` on the copy path. On the #918 in-place gather,
  `played_planes` is `None`, so the gather reads `ARENA_SILENCE_BUFFER` (`runtime.rs:1959-1980`),
  the same path an underrun takes, which rt10 already covers. It also flags `source_underrun`,
  reports zero telemetry, and charges no ring.
- **One shape on every target.** There is no `cfg` in implementation code. The `mod carry` tests are
  `cfg(not(target_arch = "wasm32"))`, like the fan-out test they copy. Bank width is not involved:
  the tests bind no banks, and the vacant in-place gather reuses the underrun path. Slice 3's gate 1
  covers both widths.

## Test value (one sentence each, confirmed by mutation in my export)

- `a_carried_consumer_continues_the_predecessor_audio_gap_free` (gate 1): turns red if the carry
  reports success but moves nothing, or moves after the successor's first `begin_block` (silence or
  an underrun at block 5). No other test renders a real ring across a graph swap. Confirmed by M1,
  `adopt_sources` returning `true` without swapping: RED.
- `a_vacant_source_without_a_matching_carry_renders_silence_and_an_underrun` (gate 2): turns red if
  a carry ignores the predecessor identity (M2: RED) or a vacant entry stops flagging the underrun
  (M8: RED).
- `a_seek_prepared_before_the_swap_is_reported_in_the_successors_first_block` (gate 3): turns red if
  the carry drops the predecessor's pending generation change (M3: RED).
- `install_refuses_out_of_range_occupied_and_repeated_sources` (gate 4): turns red if install
  accepts an occupied successor entry (M4: RED) or identities stop being unique (M12: RED).
- `a_vacant_source_charges_no_ring_and_is_counted_apart` (D1): turns red if vacant sources are not
  counted apart (M16, `vacant_source_count` returning 0: RED) or are charged ring bytes.
- `rt11 the_swap_block_carry_allocates_and_frees_nothing`: turns red on an allocation or free in
  `GraphExecutor::adopt_predecessor` or the engine's swap-block carry path. Confirmed by M18, a
  `Box` in `adopt_predecessor`: RED with `(1, 1)`. It cannot see the source driver (MINOR-5).

Mutations that left every committed test green (all reverted): M5a, M6, M7, M9, M10, M11, M13, M14.
They are the findings below.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **All-or-none has no defending test.** M6 makes `adopt_sources` check and swap one move at a time
   and stop at the first bad move, which leaves a partial move reported as `PredecessorMismatch`. All
   five source tests and rt11 stay green, because every carry test uses a single move.
   (`crates/source/src/lib.rs:1679-1703`.)
   **Fix:** add a two-source case: predecessor with two real rings, successor `bind` with two vacant
   entries, program `[(0, 0), (1, 5)]`. Assert `PredecessorMismatch` and that the successor's block
   is all `+0.0`. My scratch version, `zz_a_refused_second_move_moves_nothing`, is red under M6.
2. **Install's repeated-predecessor refusal is untested.** M5a changes `.any(|&(s, p)| s ==
   successor || p == predecessor)` to check only `s`, and stays green
   (`crates/graph/src/lib.rs:2767`). This guard matters. With `[(0, 0), (1, 0)]`, both moves pass the
   swap-block check, because each sees predecessor 0 occupied. The second `mem::swap` then exchanges
   two `None`s, the executor reports `Carried`, and successor source 1 stays silent.
   **Fix:** assert `DuplicateSourceIndex` for `[(0, 0), (1, 0)]` on a two-vacant successor. My
   scratch version, `zz_install_refuses_repeated_predecessor_index`, is red under M5a.
3. **D1's "`+0.0` on every claim" is undefended on the copy path.** M7 deletes `left.fill(0.0);
   right.fill(0.0);` from the vacant branch of `copy_track_input` (`crates/source/src/lib.rs:1582`)
   and stays green. In the test graph, the track-input arena buffers are never dirtied. Gate 2's doc
   comment says it catches "a vacant entry renders anything but `+0.0`"; for this path it does not.
   In a production plan whose input buffer is reused, the defect would replay stale audio.
   **Fix:** a driver-level test that calls `begin_block` and `copy_track_input` on a vacant entry
   with NaN- or garbage-filled `left`/`right`, and asserts every word is `0`. My scratch version,
   `zz_vacant_claim_writes_positive_zero_over_dirty_buffers`, is red under M7.
4. **A non-graph predecessor with an installed program returns `NotRequested`, and nothing tests
   it.** This follows D4.1 literally, and it is unreachable on both hosts.
   (`crates/graph/src/lib.rs:2994-2999`.) But #1270's `CarryOutcome::PredecessorMismatch` doc says
   "the successor asked for state but its predecessor was not a shape it can take state from", which
   is exactly this case. P11 also wants a counter to record a carry that moved nothing while vacant
   sources stay silent, and `NotRequested` skips `carry_mismatch_count`. M9 returns `Carried` here
   and stays green.
   **Fix:** when a program is installed, return `PredecessorMismatch` (amend D4.1), or record why
   `NotRequested` is preferred. Either way, add a test with a plain `PreparedRenderPlan`
   predecessor.
5. **The committed allocation proof covers only the graph half of the carry.** rt11 swaps a stub
   driver (`CarriedSlot`), so it cannot see `SourceGraphSourceSetDriver::adopt_sources`. M14 stages
   the moves in a `Vec` inside the real driver: rt11 stays green, and my real-driver probe goes red
   with `(1, 1)`. The authorized paths explain the gap: `source` has no `bench-support`
   dev-dependency and no authorized test file. The claim holds today (my probe gives `(0, 0)`).
   **Fix:** no change to this attempt. The coordinator should make sure #1272 gate 5 measures a swap
   block that carries at least one real source, which its gate 1 shape already does.
6. **No one charges the carry program's bytes yet.** `carry_program_retained_bytes` is correct (8 B
   per pair, 0 without a program), but #1272's brief (D4, the caps, its authorized paths) never says
   to add it to the host's plan report. The slice spec's attempt record says "Slice 3 adds it", and
   only this slice's spec records that. **Fix (coordinator):** amend #1272 D4 or gate 4 so the
   successor's report and cap include `graph::carry_program_retained_bytes(&mut plan)`.
7. **The channel-count and quantum guards at the swap block are untested.** Removing them (M10, M11)
   leaves everything green (`crates/source/src/lib.rs:1679`, `:1688`). The channel guard is what
   keeps a carried consumer with fewer channels than the successor's validated mappings from causing
   a sticky `RenderError::InvalidEnvelope`. **Fix:** a sync-form case with a one-channel predecessor
   source (mapping `(0, 0)`) carried into a two-channel vacancy (mapping `(0, 1)`), asserting
   `PredecessorMismatch` and `+0.0`.

### NIT

1. M13 is untested: `can_prepare_source_seek` returning `true` for a vacant entry stays green
   (`crates/source/src/lib.rs:1494`). Add one assert that `prepare_source_seek` on a vacant
   successor returns `false`.
2. `pending_generation_change` is per set, not per source. A seek pending on a predecessor source
   that the program does not carry still flags the successor's first block as a generation change.
   That over-reports, which is safe, and it is what D4 step 3 specifies.
3. `install_carry_program` refuses a plan without a source set (`NoSourceSet`). Later slices that
   carry non-source state for a plan with no sources will need to relax that; worth a sentence in
   their briefs.

## Late results

- **CI `test-debug-a`, run verbatim** (`cargo test --locked --workspace --all-targets` with the
  same `--exclude` list and the features `builtins-compiler/test-support`, `graph/test-support`,
  `host-web/test-support`, `host-core/test-support`, `effect-compiler/test-support`,
  `protocol/test-support`, `engine/realtime-audit`): exit 0. 116 test binaries, 1305 passed,
  0 failed. This covers capi, host-core, host-web, builtins-compiler and rt11.
- **`check-cross-targets.sh`**: exit 0, `cross-target matrix: PASS`. The only failures are the known
  #1018 iOS `memset_pattern16` expected failures, which the script accepts.
- **Worklet chain**, built into fresh directories:
  - `build-web-audioworklet.sh --named-twin`: green.
  - `check-web-audioworklet.sh`: green, run in its full form with metadata regeneration. That
    covers no atomics opcodes and no shared memory, so the `AtomicU64` identity lowers to plain
    memory operations.
  - `check-browser-expected-resources.py --artifacts`: green, with `sourceTotalBytes 3294 of 3648
    (354 bytes free)`.
  - `test-web-audioworklet.sh`: green.
- **Module digest.** The shipped module is
  `92415199cfb8bf6e7a08926acce85bf1737d1e4619eb658bb3fda47fb6b5d335`, the digest the implementer
  reported. The pin file is untouched, at `6c952a2c…`. **ARTIFACT CHANGED** is confirmed, and leaving
  the pin follows `docs/RELEASE.md` "Between releases". I did not rebuild the parent's module. The
  module compiles the changed graph and source code, so its bytes must change.
