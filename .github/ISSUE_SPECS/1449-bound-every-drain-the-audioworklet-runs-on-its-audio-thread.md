# Bound every drain the AudioWorklet runs on its audio thread

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-10).
Root ruling R2 (2026-10-05), on the second review of the realtime-policy tool drafts: the spectrum
reads that the browser bridge calls run on the AudioWorklet's audio thread, so they are render-thread
code and must meet the realtime drain rule. This issue covers the six `try_pop` sites in
`crates/host-core/src/spectrum.rs`. Its sibling *Bound the browser meter poll by each queue's count
at entry* (stream H, #1448) covers `poll_meters`. Together they cover every
drain the worklet runs. The source-submit pop needs no change (see Problem). It also moves the
pop into the loop of the two collection functions that cancel captures (D9), so that the loop can
be marked (root's ruling, 2026-10-05).

No allocation, lock or syscall is added. The rendered PCM does not change.

## Problem (verified on `origin/main` at `6d28a80ec`)

`crates/host-core/src/spectrum.rs`, `hosts/host-web/src/lib.rs` and the worklet module are the same
on `6b9067ede`, on `6d28a80ec` and on `codex/d15-stream-b` at `b8392df66` (stream B batch 1), so
this issue lands on `main` as it is.

- **Port messages run on the audio thread.** The worklet handles its port on the rendering thread,
  between `process()` calls (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:266`,
  `this.port.onmessage = (event) => this.receive(event.data)`).
  - `receiveSpectrum` (`:1255`) calls `miso_engine_web_v1_spectrum_stream_start`, `_stream_stop`
    and `_stream_read` (`:1351-1354`), and `miso_engine_web_v1_spectrum_cancel` and `_read`
    (`:1417-1418`).
  - These reach `AudioWorkletEngineHost::{start,restart,read,stop}_spectrum_stream`,
    `cancel_spectrum` and `read_spectrum` (`hosts/host-web/src/lib.rs:2376-2620`), and through
    `PreparedSpectrumCapture` (`:1400-1500`) the `SpectrumCapture` methods below.
  - The C ABI has no spectrum capture (#1327, Context). So on every host that has one, the
    capture's consumer runs on the audio thread today.
- **The six sites** (`crates/host-core/src/spectrum.rs`, `impl SpectrumCapture`, `:474-797`):

  | Site | Function | Shape today |
  |---|---|---|
  | `:510` | `cancel` | `while self.consumer.try_pop().is_ok() {}`: unbounded |
  | `:535` | `try_read_record` | one pop in a `match` arm, in no loop |
  | `:546` | `try_read_record` | one pop (`let _ = ..`), in no loop |
  | `:637` | `commit_continuous` (from `begin_continuous` and `restart_continuous`) | `while self.consumer.try_pop().is_ok() {}`: unbounded |
  | `:671` | `stop_continuous` | `while self.consumer.try_pop().is_ok() {}`: unbounded |
  | `:761` | `try_read_continuous_record` | `loop { if remaining_pops == 0 { return .. } remaining_pops -= 1; .. try_pop() .. }`, counted by `continuous_available_at_entry()` (`:726`, `:781-783`, `.min(1)`) |

  - Root's R2 names `:510` and `:637` as the two unbounded drains. `:671` has the same shape, and
    the gate measurement below refuses it too. This issue treats all three as unbounded.
  - `:761` is bounded by the count at entry in fact, but in a form the drain rule does not accept:
    a `loop` with a hand-written counter, and a count read through a helper method.
  - `:535` and `:546` are single pops in no loop. The drain rule accepts that shape (the
    self-test base tree's plan-exchange pop, `scripts/test-realtime-policy.sh:78`, has it). They
    need no change.
- **Measured.** On an `origin/main` export, with `// REALTIME_POLICY_BEGIN` and `_END` replacing
  the blank lines around one function at a time, `bash scripts/check-realtime-policy.sh` gives:
  - `cancel` (`:507`/`:521`): refused, `marked realtime unbounded try_pop drain` at `:510`;
  - `try_read_record` (`:526`/`:554`): `ok (90 marked regions in 25 files)`;
  - `commit_continuous` (`:635`/`:662`): refused at `:637`;
  - `stop_continuous` (`:662`/`:678`): refused at `:671`;
  - `try_read_continuous_record` (`:710`/`:779`): refused at `:761`.

  No other class fires on these functions.
- **Why the unbounded loops end today, and why that is not enough.** Each capture's queue has one
  slot (`bounded_spsc(NonZeroUsize::new(1) ..)`, `:1352-1353`). Its only producer is the graph
  observer, which runs inside `render`, on the same thread. While a message handler runs, render
  cannot publish, so each `while` pops at most one record. That is an accident of the thread
  layout, not a bound in the code:
  - *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (H #1332)
    and its successors (#1387, #1381, #1382) move control work to a Worker. #1332 itself keeps
    spectrum and meter reads on the worklet thread (its Non-goals), and its `single` mode keeps
    every control message on the audio thread for good. Once a spectrum read runs on another
    thread than render, a `while` drain chases the producer.
  - The realtime-policy gate cannot accept the shape, so these functions cannot be marked. Slice
    B2b-1 (#1443) of the tool issue requires every non-test pop to be in a marked region or on a
    control-side allowlist, and root ruled that none of these sites goes on the allowlist.
- **Callers that loop.** `SpectrumCaptureCollection::cancel` (`:873-878`, loop `:875-877`) and
  `select` (`:885-937`, loop `:928-932`) call `SpectrumCapture::cancel` once per capture, on the
  same worklet handlers. They hold no pop, so the tool's non-test pop rule does not see them, and
  the call rule (slice B2b-2 (#1444)) refuses them as soon as they are marked: the loop calls `cancel`,
  which pops. Root ruled (2026-10-05) that this issue makes them markable (D9) before the tool
  lands.

## Decisions

- **D1. One cap, named once.** Add `const SPECTRUM_RESULT_SLOTS: usize = 1;` in `spectrum.rs`. The
  capture's queue is built with `NonZeroUsize::new(SPECTRUM_RESULT_SLOTS)` (`:1353`), the resource
  estimate builds its queue the same way (`spectrum_capture_resources_for_id_bytes`, `:331-333`,
  today `NonZeroUsize::new(1)`), and every drain below caps its count with
  `.min(SPECTRUM_RESULT_SLOTS)`. So the production code names the slot count once. The test
  helpers that build a capture queue (`continuous_pair`, `:2815-2843`, and the others at `:3080`,
  `:3145`, `:3197`) may keep their literal `1`.
- **D2. The three unbounded drains** (`cancel`, `commit_continuous`, `stop_continuous`) become,
  each in place of its `while` line:

  ```rust
  let available = self.consumer.available_at_entry().min(SPECTRUM_RESULT_SLOTS);
  for _ in 0..available {
      if self.consumer.try_pop().is_err() {
          break;
      }
  }
  ```

  This is the form the drain rule accepts (`scripts/check-realtime-policy.sh:93-96`, `:206-226` on
  `main`; `:95-98`, `:208-228` on `b8392df66`; tool slices B1a-D6 and B1a-D8). The count is read once, at the line where the `while` stood, so
  the order of the surrounding atomic stores does not change.
  - **Semantics.** A record that render publishes after the count is read stays queued. That is
    already true today one instruction after the `while` ends, so no caller can rely on more. The
    next arm, read or drain sees it as it would see any later record. On the web host today the
    count equals what the `while` popped, because render cannot run during the call.
- **D3. `try_read_continuous_record`** (`:715-779`):
  - The count stays where it is (`:726`), before the status loads, as its comment requires. It is
    spelled `let available = self.consumer.available_at_entry().min(SPECTRUM_RESULT_SLOTS);`.
  - The `loop` with `remaining_pops` becomes `for _ in 0..available { .. }`. Its body is today's
    `match self.consumer.try_pop() { .. }` with the same four arms and the same returns.
  - After the `for`, return what the `remaining_pops == 0` branch returns today (`Warming` while
    the phase is `CONTINUOUS_WARMING`, else `Pending`).
  - `continuous_available_at_entry` (`:780-783`) has no other caller. Delete it. If a test calls
    it, the test reads `capture.consumer.available_at_entry()` directly instead.
  - Behaviour does not change: the old loop popped at most `available` times and returned the
    same values.
- **D4. `try_read_record`** (`:535`, `:546`) does not change.
- **D5. Realtime rules.** The changed functions allocate nothing, take no lock and make no
  syscall, as today. No `[]` index, `.expect(..)`, `.unwrap()` or panic macro is added.
- **D6. No region markers here.** Tool slice B2b-1 marks these functions and D9's
  `cancel_except`. This issue changes only their bodies and adds `cancel_except`. Write each
  function so that it passes `bash scripts/check-realtime-policy.sh` when it is marked (gate 3).

- **D7. Three producer/render concurrency tests**, one per D2 drain, in `spectrum.rs`'s
  `mod tests`, each built on `bench_support::producer::render_while_producing` (`host-core` already
  has the `bench-support` dev-dependency).
  - **Setup.** Build a `SpectrumCapture` around a one-slot queue whose producer the test holds, as
    `continuous_pair` (`:2815`) builds its pair. For `commit_continuous` and `stop_continuous`, put
    the capture in continuous mode first (through `start_continuous`, or the private fields).
  - **Producer.** `produce` pushes one record when the slot is empty and adds one to a shared
    `AtomicU64` of successful pushes. `queued` is "the slot is full".
  - **Each block** (`render`) first puts the capture back in the mode its drain needs, through the
    private fields (`mode`, `shared.active`), with no pop. It then reads, in this order:
    `o0 = consumer.available_at_entry()`, `p0 = pushes`; calls the drain once (`cancel()`,
    `restart_continuous()` or `stop_continuous()`); reads `p1 = pushes`,
    `o1 = consumer.available_at_entry()`. It asserts `o0 + (p1 - p0) - o1 <= 1`, computed in `i64`
    (the value can be negative when a push lands between `p1` and `o1`).
  - **Memory ordering.** The bound holds only under these three conditions. Write each into the
    test, with a comment:
    1. `produce` adds one to `pushes` with `Ordering::Release`, and only after its push returned
       `Ok` (the push's own release store publishes the record first);
    2. `render` loads `pushes` with `Ordering::Acquire` (both `p0` and `p1`);
    3. no push is in flight at `o0`. `render_while_producing` gives this: it calls `render` only
       after `queued` saw the slot full under the producers' lock, `produce` pushes only into an
       empty slot, and only the drain pops, so the slot stays full from that check until the
       drain's first pop.
  - **Why the bound holds after the change and is not a false alarm.** The test thread pops only
    inside the drain. Under the three conditions, the expression counts at most the pops between
    the two occupancy reads, because pushes before `p0` or after `p1` are not counted. After the
    change a drain pops at most its count at entry, and the slot holds at most one record.
  - **Why it fails today.** A `while` drain that pops the slot and finds it refilled pops again.
  - Run 20,000 blocks per test. Record the run time.
- **D8. One regression test for D3**: a continuous capture with one stale record (an epoch at or
  below `invalidated_epoch`) followed by nothing returns `Pending` or `Warming` after one pop, and a
  fresh record is returned on the next read. If an existing test already holds this exact shape
  (`bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop`, `:3111`), name it
  in the Evidence and add nothing.

- **D9. The collection's capture loop holds its own drain.**
  - Add one private function in `impl SpectrumCaptureCollection`, in its own place in the file so
    that B2b-1 can mark it alone:

    ```rust
    fn cancel_except(&mut self, keep: Option<usize>) {
        for (index, capture) in self.captures.iter_mut().enumerate() {
            if keep == Some(index) {
                continue;
            }
            let available = capture.consumer.available_at_entry().min(SPECTRUM_RESULT_SLOTS);
            for _ in 0..available {
                if capture.consumer.try_pop().is_err() {
                    break;
                }
            }
        }
        for (index, capture) in self.captures.iter_mut().enumerate() {
            if keep == Some(index) {
                continue;
            }
            capture.reset_after_cancel();
        }
    }
    ```

  - **Two loops, not one** (fourth review, MAJOR-5). #1418's invariance rule lets the drain loop
    name `capture` only in its binding, its count and its pop. A `capture.reset_after_cancel()` in
    the same loop names it a fourth time, and attempt 2's gate refuses that shape (measured at the
    pop). So the drain loop drains and the reset loop resets. Do not merge them.

  - `SpectrumCapture::reset_after_cancel(&mut self)` is the tail of today's
    `SpectrumCapture::cancel` (`:511-519`: the continuous-mode stores, `state = IDLE`,
    `recovery_pending = false`), moved without change. It holds no pop. `SpectrumCapture::cancel`
    becomes D2's counted drain followed by `self.reset_after_cancel()`, so both paths drain first
    and reset second, as today.
  - `SpectrumCaptureCollection::cancel` becomes `self.cancel_except(None)`. In `select`, the loop
    at `:928-932` becomes `self.cancel_except(Some(index))`, at the same place in the function.
  - **Why this is a restructure, not a wrapper.** The pop moves into the loop's own body, where the
    drain rule judges it: the count is read at entry, the outer loop is the finite form
    `<fields>.iter_mut().enumerate()` (tool slice B1b-D1), and each pass pops a different capture's
    queue (#1418 Amendment 1, D1). The reset loop, `reset_after_cancel` and `cancel_except`'s
    callers hold no pop in a loop. The drain is written twice (here and in `SpectrumCapture::cancel`); that is the
    price of a loop the gate can read.
  - **Behaviour.** Each capture except `keep` is drained and then reset, as `capture.cancel()`
    did. The order across captures changes: today capture 0 is drained and reset before capture 1
    is drained; now every capture is drained before any is reset. Nothing on the worklet thread can
    observe the difference: the captures share no state, and the call holds `&mut self`
    throughout.
  - **One unit test** in `spectrum.rs`'s `mod tests`,
    `collection_cancel_except_drains_every_capture_but_the_kept_one`: build a collection with two
    captures, each with one record queued (through the private fields, as the existing collection
    tests build captures), no capture selected, and capture 1 in one-shot mode with state `IDLE`.
    `select` takes `(target, channels)`, not an index: call it with capture 1's target and
    channels. With nothing selected, `select` arms capture 1 (`arm()`, `spectrum.rs:498-506`),
    which returns `Busy` unless the mode is one-shot and the state is `IDLE`; on `Busy`, `select`
    returns before `cancel_except` runs, and the test proves nothing. So assert that `select`
    returns `Ok`. Then capture 0's queue is empty and capture 1's record is still queued. Then
    `cancel()` leaves both queues empty (fifth review, NIT-4). Test value: red if `cancel_except` skips the
    drain or drains the kept capture. In every existing test the cancelled capture's queue is
    already empty, so removing the drain leaves them all green (fourth review, MAJOR-6, measured).

## Authorized paths

- `crates/host-core/src/spectrum.rs`: D1-D3, D9 and their unit tests in its `mod tests`. This is a
  named exception: stream A owns this file for #1327 and #1395 (STREAMS, stream A "Owns"). The
  hot-file row puts this issue before A #1327.
- This spec

## Non-goals

- `poll_meters` (its sibling issue) and `PcmSourceProducer::take_recycled_block` (one pop per
  submit, in no loop; tool slice B2b-1 marks it as it is).
- Moving any call off the worklet thread (H #1332 and its successors).
- Region markers (tool slice B2b-1).
- The spectrum carry (#1395).

## Hazards

- **Hot file.** A #1327 and A #1395 also edit `spectrum.rs` (#1395 adds `pair_carried` and its
  tests). This issue lands first; they rebase. Find each function by name, not by line.
- **Line numbers in other specs.** #1395 cites `spectrum.rs` line ranges (`:457-472`,
  `:1441-1449`). Lines move after this change. The #1395 implementer re-reads by name.
- **The concurrency tests depend on scheduling.** A defect shows only when the producer refills
  the slot inside the drain. Gate 2 measures how often the old code fails; it does not tune the
  test to pass.

## Objective gates

1. **Unit tests.** `cargo test --locked -p host-core --lib spectrum` passes, with every existing
   spectrum test and the D7, D8 and D9 tests. PR evidence: with D9's drain loop removed, and
   separately with the `keep` check removed from it, D9's test is red.
   `cargo test --locked -p host-core` and `cargo test --locked -p host-web --features test-support`
   pass unchanged (the collection's `select` and `cancel` callers, D9).
2. **Red on today's code (PR evidence).** Apply only this issue's new concurrency tests to the
   parent commit and run `cargo test --locked -p host-core --lib spectrum::tests::<name>` ten times for each
   new concurrency test, on a machine with at least two CPUs. Record how many runs fail. Each test
   must fail at least once in its ten runs. If a test never fails on the parent, stop and report
   it; do not lengthen or tune it to get a failure without root.
3. **The gate accepts the new shapes (PR evidence).** On an export of this change, put
   `// REALTIME_POLICY_BEGIN` and `// REALTIME_POLICY_END` around each of `cancel`,
   `try_read_record`, `commit_continuous`, `stop_continuous`, `try_read_continuous_record` and
   `cancel_except`, one function at a time (in place of blank lines where there are some), and run
   `bash scripts/check-realtime-policy.sh`. Each run prints `realtime policy: ok (..)`. Record the
   six lines. Stream J's tool is not on `main` while this issue runs (its batch pushes once):
   B2b-1 marks these functions and checks them, and B2b-2's gate 3 holds today's collection loop
   red under the call rule.
   - **#1418's rule accepts `cancel_except`.** In the same export, with only `cancel_except`
     marked, replace `scripts/check-realtime-policy.sh` with attempt 2's gate
     (`1418-attempt2-check-realtime-policy.sh` in the probes folder, with this change's floors) and
     run it: it prints `realtime policy: ok (..)`. Record the line.
4. **Worklet chain gates.** The batch's `qualification` run passes `artifact`, `artifact-identity`,
   `artifact-gates` (`check-web-audioworklet.sh`, `check-scalar-oracle-absent.py`,
   `test-web-audioworklet.sh`, the V8 spill gate) and the three browser legs.
   - `artifact-identity` may report ARTIFACT CHANGED. PR evidence: a function-level comparison of
     the base and head named twins, as the stream-J2 batch verdict made it (`wasm-objdump -d`,
     call targets mapped to names, crate hashes normalised). Only the changed `SpectrumCapture`
     and `SpectrumCaptureCollection` functions, their inlined copies in the spectrum exports, and
     `core::panic::Location` line fields may differ. The render closure (`miso_engine_web_v1_render`) is byte-identical.
   - **No PCM moved.** No fixture, pin or expected digest changes, and every rendered-digest gate
     passes with the base's pins (`check-browser-expected-resources.py --artifacts`, the browser
     legs' native-digest gates, `wasm-gates` G5 and G6).
5. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p host-core --all-targets -- -D warnings`,
   `bash scripts/check-realtime-policy.sh` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- Each D7 test is red if its drain pops past the count it read at entry, the producer-chase defect
  that an unbounded `while` has once a spectrum read runs on another thread than render. No
  existing test runs a producer during a drain.
- D8 (or the existing test it names) is red if the rewritten `try_read_continuous_record` drops
  the stale-record branch or the post-loop status.
- D9's test is red if `cancel_except` skips the drain or drains the kept capture: a deselected
  capture would keep a stale window, which is read as the result when it is selected again. Its
  shape is held by B2b-1's marker, which the drain and call rules check.

## Evidence

- Gates 1-5 output; gate 2's failure counts per test; gate 3's six lines and the attempt-2 gate's
  line; gate 4's comparison.

## Dependencies

- After (same stream): none.
- After (other streams): none.
- Before (other streams): A #1327 and A #1395 (hot file); J tool slice B2b-1, which marks these
  functions.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.

## Attempt record

### Attempt 1 (implementer, 2026-10-06; base `af1c55a20`)

**Change** (`crates/host-core/src/spectrum.rs` only):

- D1: `const SPECTRUM_RESULT_SLOTS: usize = 1;` builds the capture queue
  (`prepare_capture_with_handle`) and the resource-estimate queue
  (`spectrum_capture_resources_for_id_bytes`), and caps every drain.
- D2: `cancel`, `commit_continuous` and `stop_continuous` each drain with
  `available_at_entry().min(SPECTRUM_RESULT_SLOTS)` and a `for` of bounded pops, in place of the
  `while`. `cancel` is that drain, then `reset_after_cancel()` (today's tail, moved unchanged).
- D3: `try_read_continuous_record` reads the count in place, runs `for _ in 0..available` over the
  same four arms, and returns `Warming`/`Pending` after the loop. `continuous_available_at_entry`
  is deleted (no other caller, no test called it).
- D4: `try_read_record` unchanged.
- D9: private `SpectrumCaptureCollection::cancel_except(keep)`, two loops (drain, then reset), in
  its own place after `SpectrumCaptureCollection::cancel`. `cancel` is `self.cancel_except(None)`;
  `select` calls `self.cancel_except(Some(index))` where its loop stood.
- Tests: `racing_capture` and `assert_drain_pops_at_most_its_entry_count` (helpers),
  `cancel_pops_at_most_its_entry_count_while_a_producer_refills`,
  `continuous_restart_pops_at_most_its_entry_count_while_a_producer_refills`,
  `continuous_stop_pops_at_most_its_entry_count_while_a_producer_refills` (D7, 20,000 blocks each,
  on `bench_support::producer::render_while_producing`, with the three ordering conditions written
  in the helper's comments), and `collection_cancel_except_drains_every_capture_but_the_kept_one`
  (D9). D8: no new test; `failed_continuous_completion_drains_only_the_failed_queued_window`
  already holds the full shape (corrected in the follow-ups below; the attempt first named
  `bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop`, which holds only
  the first half). No test was superseded.

**Gate 1.** `cargo test --locked -p host-core --lib spectrum`: 41 passed. `cargo test --locked -p
host-core`: every binary ok (lib 68 passed). `cargo test --locked -p host-web --features
test-support`: ok (lib 186 passed, 2 ignored, as before).

Mutation evidence (each applied alone to the change, then reverted; the file was restored byte for
byte, checked with `cmp`):

| Mutation | Test | Result |
|---|---|---|
| `cancel_except`: drain loop removed | `collection_cancel_except_drains_every_capture_but_the_kept_one` | red (capture 0 still queued) |
| `cancel_except`: `keep` check removed from the drain loop | same | red (capture 1's record drained) |
| `try_read_continuous_record`: stale-record arm removed | `bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop` | red |
| `try_read_continuous_record`: post-loop status always `Pending` | same | red (`Warming` expected) |
| each D2 drain back to `while .. try_pop().is_ok() {}` | its D7 test | red (gate 2, the parent's code) |

**Gate 2** (red on the parent's code; 32-CPU host, debug test profile, `cargo test --locked -p
host-core --lib spectrum::tests::<name>` ten times each, with only the new tests applied to
`af1c55a20`):

| Test | Failed runs of 10 | Failure |
|---|---|---|
| `cancel_pops_at_most_its_entry_count_while_a_producer_refills` | 1 | block 14242: popped 2, saw 1 at entry |
| `continuous_restart_pops_at_most_its_entry_count_while_a_producer_refills` | 2 | blocks 999 and 5351: popped 2, saw 1 |
| `continuous_stop_pops_at_most_its_entry_count_while_a_producer_refills` | 1 | block 6577: popped 2, saw 1 |

On the change: 0 failed runs of 10 for each. Run time per test (20,000 blocks): 160-300 ms.
The failure rate is low because the producer must copy a 16 KiB record into the slot between the
drain's pop and its next emptiness check; the tests were not lengthened or tuned.

**Gate 3.** Export of the change (`git ls-files` of `crates hosts tools scripts` from the working
tree), one function marked at a time in place of its surrounding blank lines,
`bash scripts/check-realtime-policy.sh`:

- `cancel`: `realtime policy: ok (90 marked regions in 25 files)`
- `try_read_record`: `realtime policy: ok (90 marked regions in 25 files)`
- `commit_continuous`: `realtime policy: ok (90 marked regions in 25 files)`
- `stop_continuous`: `realtime policy: ok (90 marked regions in 25 files)`
- `try_read_continuous_record`: `realtime policy: ok (90 marked regions in 25 files)`
- `cancel_except`: `realtime policy: ok (90 marked regions in 25 files)`
- `cancel_except` only, with `1418-attempt2-check-realtime-policy.sh` (its floors are 25 files and
  90 regions, which the one added marker meets): `realtime policy: ok (90 marked regions in 25
  files)`.
- Control: the same marker around `cancel` on `af1c55a20` is refused,
  `crates/host-core/src/spectrum.rs:510: while self.consumer.try_pop().is_ok() {}` /
  `marked realtime unbounded try_pop drain`.

**Gate 4** (local; the batch's `qualification` run is still owed). Head and base built with
`scripts/build-web-audioworklet.sh --named-twin`:

- shipped module `9b2b1a0f..` (base) -> `465b78a8..` (head): ARTIFACT CHANGED, as expected.
- `strip-wasm-names.py check`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (digests and exact rows agree; self-test 32 red
  mutations), `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`, the V8 spill
  gate (self-test and module) and `run-wasm-gates.sh --without-v8-spill --without-native` (142
  cases, 0 mismatches): all pass.
- Browser legs, `npm run qualify -- --artifacts <head> --sdk-root sdk --browser <b>
  --check-matrix --self-test-mutations`: chromium 151.0.7922.34, firefox 153.0, webkit 26.5, each
  `all qualification gates passed`. No fixture, pin or expected digest was edited.
- Function-level comparison of the named twins (`wasm-objdump -d`, calls mapped to normalised
  callee names, the 1418-probes `mtr-closure-offset-cmp.py` parser): 2,694 -> 2,695 functions.
  The only new function is `SpectrumCaptureCollection::cancel_except`. The only changed bodies are
  `SpectrumCapture::{cancel, commit_continuous, restart_continuous, stop_continuous}` and
  `SpectrumCaptureCollection::{cancel, select, start_continuous, start_continuous_with_hop,
  stop_continuous, restart_continuous}` (inlining of the changed drains moved between them).
  `try_read_continuous_record` compiled to identical code. The render closure
  (`miso_engine_web_v1_render`, 25 functions) is identical.
- Data section: same size (100,279 bytes); 36 bytes differ, all in the `line` field of 30
  `core::panic::Location` records whose file is `crates/host-core/src/spectrum.rs` (line deltas
  +2, +28, +49, +53). No column or file field changed.

**Gate 5.** `cargo fmt --all -- --check`, `cargo clippy --locked -p host-core --all-targets -- -D
warnings`, `bash scripts/check-realtime-policy.sh` (`ok (89 marked regions in 25 files)`), `bash
scripts/check-workspace-policy.sh` and `bash scripts/check-cross-targets.sh` (PASS): all exit 0.
AArch64 runs only in CI.

*Test value.*
- Each D7 test: red if its drain pops past the count it read at entry (the `while` chase of a
  refilling producer); no existing test runs a producer during a drain.
- D9's test: red if `cancel_except` skips the drain or drains the kept capture; every existing
  collection test cancels an already-empty queue.
- D8 (existing tests, see the follow-ups): red if the rewritten read drops the stale-record arm or
  the post-loop `Warming` status.

### Follow-ups (verifier MINOR/NIT)

Test-only change in `crates/host-core/src/spectrum.rs` `mod tests`; no non-test code changed.

- **MINOR-1 and MINOR-2**: `each_drain_pops_exactly_one_record_of_two_queued`, single-thread. A
  capture around a two-slot queue holds two records; `cancel()` (armed one-shot),
  `start_continuous()` (reaches `commit_continuous`) and `stop_continuous()` (active continuous
  mode) each run on a fresh capture, and exactly one record must be left. It holds
  `.min(SPECTRUM_RESULT_SLOTS)` deterministically (MINOR-1) and that each drain removes a record
  (MINOR-2). The D7 concurrency tests are unchanged.
- **NIT-1**: the test that holds D8's full shape (one stale pop, then `Pending`/`Warming`, then a
  fresh record on the next read) is `failed_continuous_completion_drains_only_the_failed_queued_window`.
  The verifier measured it red on both D3 mutations (stale-record arm removed; post-loop status
  always `Pending`). `bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop`
  holds only the first half and, despite its name, does not detect a chase: D3's bound is held
  only by the realtime-policy marker (B2b-1).
- **NIT-2**: the one-slot builder is now `capture_with_held_producer(slots)`. The D9 test and
  `selected_channels_reports_the_selected_entrys_mask` call it; `racing_capture()` remains as a
  one-line wrapper (`capture_with_held_producer(1)`) used only by the three D7 tests, so their
  bodies are unchanged.

Mutation evidence (each applied alone, `cargo test --locked -p host-core --lib spectrum::tests::`,
then reverted; the file restored byte for byte, checked with `cmp`):

| Mutation | New test | Assertion | Other tests red |
|---|---|---|---|
| `cancel` drain -> `while .. try_pop().is_ok() {}` | red | `cancel`: left 0, right 1 | D7 `cancel_..` (by chance) |
| `cancel` drain removed | red | `cancel`: left 2, right 1 | none |
| `commit_continuous` drain -> `while` | red | `commit_continuous`: left 0, right 1 | none |
| `commit_continuous` drain removed | red | `commit_continuous`: left 2, right 1 | none |
| `stop_continuous` drain -> `while` | red | `stop_continuous`: left 0, right 1 | D7 `continuous_stop_..` (by chance) |
| `stop_continuous` drain removed | red | `stop_continuous`: left 2, right 1 | none |

*Test value.* `each_drain_pops_exactly_one_record_of_two_queued`: red if any of `cancel`,
`commit_continuous` or `stop_continuous` pops more than `min(count at entry, SPECTRUM_RESULT_SLOTS)`
records (a `while` drain, every run, unlike the D7 tests' scheduling-dependent power) or pops none
(a cancelled or stopped capture keeps a stale window); no existing host-core test catches either
deterministically, and none catches a removed `cancel` or `stop_continuous` drain at all.
