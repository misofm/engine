# Host core: a session seek whose push was refused would publish the other pushes, then report failure

Item HC-9 of the implicit-state-machine tracker #2258 (audit of 2026-11-06, agent-reported).
Re-verified here on `fd33447dc`. Follows #2313, which made each source entry's producer a
`Producer<P>` state, and #2273, which added `carried`.

## Problem
`SourceControlSet::seek_session_landing` (`crates/host-core/src/source.rs:514-548` on base) seeks
the session timeline and every source under one generation, in three steps: open the seek above
the session's published word, check every producer with nothing pushed
(`check_session_producers`), then push every command and publish once. Step 3 was written as if
a push could still be refused:

```rust
let mut pushed = self.timeline.owned_mut()
    .is_some_and(|timeline| seek.push_timeline(timeline, timeline_sample).is_ok());
for source in &mut self.sources {
    pushed &= source.provider.owned_mut()
        .is_some_and(|provider| seek.push_source(provider, frame).is_ok());
}
seek.publish();
if !pushed {
    debug_assert!(false, "a checked session seek pushes to every slot");
    return Err(SourceControlError::Chunk(HostChunkError::InternalInvariant));
}
```

So a refused push (or a vacant entry) would have published every command that did land and then
returned `source.internal`. Render would play a seek on some producers and not others, and the
host would hear a failure for a seek that had partly happened. Only a `debug_assert!(false)` said
this cannot happen. The rule that made it impossible ("check every producer before you push to
any, and change nothing in between") was a comment, not a type.

## Reachability (verified: not reachable)
Every push in step 3 is one its own check in step 2 accepted, and nothing the check read changes
before the push:
- **No call in between.** `seek_session_landing` holds `&mut self` from the check to the publish,
  and step 3 runs no other code. Only `adopt_persisting`, `reclaim_from_withdrawn` and the
  donation move a producer (`source.rs`), and each needs `&mut` on the set, so no entry can
  change from `Owned` to vacant in between.
- **Same command, same rules.** The check (`SessionSeek::check_timeline`/`check_source` ->
  `SeekProducer::check_seek`) and the push (`push_timeline`/`push_source` -> `SeekProducer::push`)
  build the same command (`SessionSeek::command(frame)`, with the same frame) and apply the same
  rules (`SeekProducer::check_rules`, `crates/source/src/lib.rs`). Those rules read the command,
  the producer's `active_generation`, its quantum and its frame limit. The last two never change.
  `active_generation` changes only when that producer's own push lands. Each producer (the
  timeline's, and each source's `HostChunkProvider`) owns its own `SeekProducer`, so one push
  cannot change another producer. No push publishes the word: `SeekProducer::push` and
  `PcmSourceProducer::push_seek` store nothing to it.
- **Same session.** `SessionSeek::own` compares the producer's `PublishedGeneration` with the
  seek's. A producer's word is fixed when it is prepared.
- **The slot still has room.** `check_seek` refuses a full slot through
  `Producer::available_capacity` (`crates/engine/src/realtime/spsc.rs`), which loads the
  consumer's cursor once. Only render's pop moves that cursor, and a pop only frees a slot. Only
  this producer fills the slot, and its first fill comes after the check. So `try_push`'s
  `is_full_at` (which reloads the cursor when the cached one looks full) sees room.

So the `InternalInvariant` branch could be reached only through a broken SPSC ring. This is
hardening, as with #2257, #2273 and #2313.

**Cross-thread.** Render sees a session seek through the command slots and the published word
(`PublishedGeneration`: a `Release` `fetch_max` after the last push, and an `Acquire` load at block
entry). This issue does not change that protocol: the same commands are pushed in the same order,
then published once by the same `SessionSeek::publish`. No loom model applies.

## Contract
- `CheckedSessionSeek<'s>` (host-core, private) is a session seek that every producer of its set
  has accepted. `CheckedSessionSeek::check` is its only constructor. It takes the timeline's
  producer (a vacant timeline is `Vacated`, as before), runs `check_session_producers`, and keeps
  `&'s mut` borrows of the timeline producer and the source entries. So the compiler enforces two
  things:
  - a seek reaches its pushes only after every check has passed;
  - nothing can change a producer between its check and its push.
- `CheckedSessionSeek::push_and_publish(self)` returns `()`. It pushes the timeline, then the
  sources in ID order, then publishes. Its only failure branches are `unreachable!`, each stating
  the invariant that rules it out (the type's doc gives the proof above). A panic is an engine
  defect. A release build aborts (`panic = "abort"`, `docs/REALTIME_DEPENDENCY_POLICY.md`) rather
  than seek some producers and not others.
- `seek_session` and `seek_session_at_once` lose the post-publish `InternalInvariant` return.
  Every refusal now comes from steps 1 and 2, with nothing pushed. `check_session_seek` runs the
  same checks.
- No refusal, diagnostic, order of refusals, wire format, rendered bit or publish order changes.
  The `ForeignSession` mapping to `InternalInvariant` in step 2 stays: it is a refusal with
  nothing pushed.

## Non-goals
- The source crate's `SessionSeek` API (`check_*`/`push_*`/`publish`). It stays a fallible
  primitive. A primitive cannot hold every producer of a set without a per-seek allocation, so
  only the set's owner can prove its pushes infallible.
- Render.
- `crates/host-core/src/spectrum.rs`.
- The other #2258 items.

## Gates
- Unreachable, so there is no red-on-base test. Instead: the proof above, plus the type-level
  change that keeps the failure out.
- Mutations of the new check (skip the source checks, skip the timeline check) turn existing
  gates red.
- A one-time no-bit-moved comparison (not committed) of the session-seek tests' rendered output,
  base against head.
- `cargo test --locked -p host-core --features test-support,control-provider`; `-p capi` and
  `-p host-web` session-seek tests; clippy `-D warnings`; `cargo fmt --check`; `cargo doc -D
  warnings`; `scripts/check-workspace-policy.sh`; `scripts/check-host-core-policy.sh`.

## Evidence (attempt 1)

**Verified on `fd33447dc`.** The branch is as quoted under Problem (`source.rs:529-547` on base).
No push can be refused after the check (proof under Reachability). So there is no red-on-base
test: no input reaches the branch. The defect was in the shape: the check-all-then-push-all rule
was a comment, and a dead error path published before it reported failure.

**Model rejected: a per-producer push token from the check.** A token that proves one producer's
slot has room must borrow that producer from its check until its push. Holding one for every
producer of a set at once needs storage per source, so a seek would allocate. Seeks are
allocation-free today. A token that borrows nothing cannot be tied to its producer without a
runtime check, and that check would be a new failure path. The exclusive borrow of the whole set
is O(1), and only the set's owner (host-core) can hold it. So the source crate's
`SessionSeek::push_*` stay fallible primitives.

**Fix** (`crates/host-core/src/source.rs`).
- `CheckedSessionSeek<'s>` is built only by `CheckedSessionSeek::check`. That takes the
  timeline's producer `&mut`, runs `check_session_producers`, and keeps `&'s mut` borrows of the
  timeline producer and the source entries.
- `push_and_publish(self) -> ()` pushes the timeline, then the sources in ID order, then calls
  `SessionSeek::publish`. Its three failure branches are `unreachable!`, each with its invariant
  named. The type's doc carries the proof.
- `seek_session_landing` is now two lines: open the seek (step 1), then
  `CheckedSessionSeek::check(..)?.push_and_publish()`. The `pushed` flag, the `debug_assert!(false)`
  and the post-publish `Err(InternalInvariant)` are gone.
- `check_session_producers` now takes `&TimelineProducer`. Both callers (`CheckedSessionSeek::check`
  and `check_session_seek`) refuse a vacant timeline `Vacated` first, as before, so the order of
  refusals is unchanged.
- The `seek_session` doc's step 3 is reworded to match.

**Test value.** No new test. No input reaches the removed branch, and the existing gates already
pin every refusal and the whole-seek outcome: `source::tests::a_refusal_pushes_nothing_and_publishes_nothing`,
`a_vacant_entry_refuses_the_whole_seek`, `a_generation_not_above_the_word_is_stale_when_every_producer_takes_it`,
and the session-seek gates in `crates/host-core/tests`. No test is made useless: none exercised the
removed branch.

**Mutations (each reverted).** Each one breaks the check that `CheckedSessionSeek` relies on, and
an existing gate goes red at one of the new `unreachable!` branches. On base, the same mutations
would have published a partial seek and returned `source.internal` in a release build.
- `check_session_producers` checks no source (`sources.iter().take(0)`):
  `a_refusal_pushes_nothing_and_publishes_nothing` panics at the source push's `unreachable!`
  (the short source's full slot), and `a_vacant_entry_refuses_the_whole_seek` panics at the
  vacant-source `unreachable!`.
- `check_session_producers` ignores the timeline's check:
  `a_refusal_pushes_nothing_and_publishes_nothing` panics at the timeline push's `unreachable!`
  (`TIMELINE_SAMPLE_LIMIT + 1`).
- `CheckedSessionSeek::check` drops the check's result (`let _ = ..`): both tests panic.

**No bit moved (one-time, not committed).** For this run only, a temporary patch to
`crates/engine` hashed every block's output buffer after each successful
`PreparedRenderPlan::render` (FNV-1a over the `f32` bits) into a per-thread digest, written as
`thread name, digest, blocks` when the thread exits. It was applied to base `fd33447dc` (in a
detached worktree) and to head, then reverted. The suites:
- `cargo test -p host-core --features test-support,control-provider`;
- `cargo test -p capi --lib`;
- `cargo test -p host-web --features test-support --test control_parity`;
- `cargo test -p control-plane --features test-support --test mute_automation_realtime`.

300 rendering threads, 125,172 blocks. Of these, 297 named test threads (78,464 blocks) are
identical at base and head. They include every rendering session-seek gate:
- `a_session_seek_sets_the_fader_lane_exactly`;
- `a_session_seek_sets_a_latent_fader_lane_exactly_where_it_reaches_the_node`;
- the four `a_seek_*_unmute_ramps_end` gates;
- `a_seek_queued_at_the_swap_block_lands_in_the_successors_first_block`;
- `a_donated_ring_plays_its_acked_pcm_and_held_seek`;
- `a_source_and_the_timeline_given_the_same_seeks_read_the_same_frame_in_every_block`;
- `seek_generation_invalidates_a_partial_spectrum_window`.

Three threads differ, and none is deterministic:
- two unnamed render threads of concurrent tests (such as `session_seek.rs`'s gate 1b), whose
  block counts differ too;
- `live_routes::live_sends_render_without_allocating`, whose producer thread races render. Two
  runs on base alone gave two different digests (`200e5eb7…`, `e273441e…`), and it does no seek.

**Gates (head).**
- `cargo test --locked -p host-core --features test-support,control-provider`: 58 binaries,
  624 passed, 0 failed, 8 ignored.
- `cargo test --locked -p capi --lib`: 190 passed.
- `-p host-web --test control_parity`: 46 passed.
- `-p control-plane --test mute_automation_realtime`: 2 passed. These three ran in the head
  digest run, with the digest patch on top of this change.
- `cargo clippy --locked -p host-core --all-targets -- -D warnings`, with and without
  `test-support,control-provider`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-core --features control-provider`,
  also with `--document-private-items`: clean. Without `control-provider`, `-p host-core` alone
  fails on base too: `prepare.rs:262` links `crate::warm::production_config`, which is
  feature-gated. The workspace doc job unifies the feature.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`: ok.
- The realtime policy tool was not run: no render code changed (`source.rs` is control-plane
  code, outside every `REALTIME_POLICY` region).

**Open.** None in scope. The source crate's `SessionSeek` can still be misused by a caller that
checks and pushes producers interleaved, or that changes a producer between its check and its
push. Host-core is its only caller outside the source crate's tests. A future caller should use a
set-level owner like `CheckedSessionSeek`.

## Verdict (attempt 1): PASS
The adversarial review confirmed that:
- the unreachability proof holds, including the SPSC cursor argument;
- abort in release is the right choice, because no typed error can undo a push that has landed;
- no new test is needed: no input reaches the removed branch, and the existing refusal gates go
  red under every mutation of the check;
- the branch merges cleanly with `main`.

Folded after the review: the `CheckedSessionSeek` doc also names
`SourceControlSet::reclaim_from_withdrawn` among the producer moves the borrow keeps out, and the
`push_and_publish` doc names the residual case a broken SPSC ring. The coordinator tracks
control-plane's `session_seek_landing` cross-call invariant in #2258, outside this issue.
