Verdict: PASS

# #1258 attempt 1: Sol's adversarial verdict

- Commit reviewed: `0fdb4f886`, parent `ef5ac335c`.
- Export: `/tmp/claude-1002/v1258-a1`, on x86-64-v3.
- Spec: `.github/ISSUE_SPECS/1258-qualify-live-c-abi-edits-against-a-concurrently-rendering-plan.md`.
- Also held against umbrella #1053 D2, D4, D6 and D8.

No BLOCKER and no MAJOR finding. There is one MINOR and there are two NITs. Every gate I re-ran
passes. Every mutation I re-ran went red on the new test and green again once reverted.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **The race's mute edits almost never change a value, so a lost mute record survives the race.**
   - Where: `crates/capi/tests/resource_lifecycle.rs:2523-2525`. The formula is
     `let mute = (index / 3).is_multiple_of(3);`.
   - The same claim is made in three places:
     - the `RaceEdits::edit` doc at `:2480` ("a value no earlier edit of the same kind on the same
       track used");
     - the `RACE_EDITS` doc at `:2554` ("the run ends with four live edits");
     - the Attempt record (spec `:198`, "with distinct values", and `:219`).
   - **The arithmetic.** A mute edit has `index = 9*step + track` with `step % 3 == 2`. So
     `index / 3 = 3*step + track/3`, and `mute` is `(track / 3) % 3 == 0` for every mute edit of a
     track:
     - `eq0`-`eq2` are always left-muted;
     - `eq3`-`eq8` are always unmuted.
   - **What follows** (simulated over the exact edit sequence):
     - Of the 43 mute edits per run, 3 push a record. These are edits 18-20, the first mute of
       `eq0`-`eq2`.
     - The other 40 are live commits with zero records.
     - Edits 153-155, three of the four "live edits after the last structural edit", are such
       no-ops. Only edit 152, a pan on `eq8`, can be lost in the final block.
   - **Mutation I ran.** The live arm skips every `TrackFaderRecord::Mute` while it pushes. This
     is an ack-then-drop of every mute record.
     - The race stays **green** on 3 of 3 invocations, 60 runs in all.
     - The suite is not blind: #1257's `live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly`
       and `a_live_edit_while_a_candidate_is_pending_reaches_the_candidate` both go red.
     - What fails is the race's own claim. It does not race any mute record against a swap, and it
       rests its lost-edit detection on one edit.
     - M1-a is still caught, red in run 0 on 5 of 5 invocations. So the gate's stated test value
       holds, and this is MINOR, not MAJOR.
   - **Fix.**
     - Make each mute edit flip its track's left mute. An example is
       `let mute = (step / 3 + track as u64).is_multiple_of(2);`: 36 of 43 mute edits then change
       a value, edits 153 and 155 push records, and 4 of 9 tracks end left-muted, with no right
       lane ever muted, so the final mix stays non-zero.
     - Correct the three comments and the Attempt record.
     - Re-run M1-a and the mute-drop mutation above.

### NIT

1. **The Attempt record overstates the race's cost** (spec `:226`).
   - It says "Each run takes about 7 s in debug".
   - The whole 20-run test takes 7.0-9.3 s here, which is about 0.35 s per run.
   - Write "the 20-run test takes about 7 s".
2. **"40-41 retries" is mostly a counter the test sets itself** (spec `:224`; test `:2948`, `:2950`).
   - `paused_bursts` adds 2 to `retries` per run, whether or not the resubmission is refused. That
     makes 40 per 20 runs.
   - The bounded retry loop in `RaceControl::commit` actually ran 0-2 times per 20 runs in my 10
     sequential invocations.
   - Count only refused resubmissions, or say in the record that the 40 are the forced paused-phase
     resubmissions.

## Notes the caller asked for

### Hang paths (lesson d)

**The justification holds.** `bench_support` installs its own `#[global_allocator]`
(`tools/bench-support/src/alloc.rs:169`), and this file has its own (`resource_lifecycle.rs:24`).
Two global allocators cannot link into one binary. capi has no `bench-support` dependency, and its
`Cargo.toml` is outside the authorized paths. #1251 D4 anticipated the same clash.

**The pattern applied by hand holds.** I planted four failures in the export and ran each under
`timeout -s KILL 120`. Each one failed instead of hanging:

| Plant | Exit | Time | Message |
|---|---|---|---|
| Render thread panics at block 52 | 101 | 10.06 s | "planted render panic", then the control thread's 10 s deadline and `expect("the render thread does not panic")` |
| Control thread panics at edit 40, outside any `Result` | 101 | 0.10 s | "planted control panic" (`StopOnDrop` released the render thread) |
| Panic while the render thread is parked | 101 | 0.16 s | "planted paused panic" (the parked loop re-checks `stop`) |
| `Err` while the render thread is parked | 101 | 0.16 s | "run 0: a control call failed: planted paused error" |

Other paths, all bounded:
- Every control-side wait is bounded:
  - `await_block` checks `stop` and a 10 s deadline;
  - the park wait has a 10 s deadline;
  - `fill` and `drain_events_c` are bounded loops;
  - `commit` is bounded by 256 blocks, and its progress wait by the 10 s deadline.
- A failed render code sets `stop`, and the control thread then returns an error.
- A panic inside `miso_engine_v1_render_f32_planar` aborts the process, because the entry point is
  `extern "C"`.

The one unrecoverable case is a render call that never returns. The join then blocks. That holds
equally for `render_while_producing`, and no in-process test can escape it. It is not a finding.

### Flakiness and CI cost

I ran 56 invocations, which is 1,120 race runs, with 0 failures:

| Condition | Invocations | Time each | Result |
|---|---|---|---|
| Sequential | 10 | 7.0-9.3 s | all pass |
| 40 at once on 32 cores (80 spinning threads) | 40 | 26.6-31.4 s | all pass |
| Pinned to 1 CPU | 3 | 13.9-14.9 s | all pass |
| Pinned to 2 CPUs | 3 | 6.9-7.7 s | all pass |

- Even on one CPU, `overlapped` was about 3,150, so a control call genuinely overlaps a render call.
- Every refusal comes from the deterministic paused phase:
  - `live_backpressure` is always 20 per 20 runs;
  - `plan_backpressure` is 20-22;
  - `event_backpressure` is always 0.
- So a loaded runner cannot starve the retry bound.
- The whole `resource_lifecycle` binary takes 7.33 s, and the race is its longest test.
- The CI router sends this diff to `route=full`, so the race also runs in `aarch64-debug`, natively.
  The cost is acceptable.

### `pcm_digest`

No CI validator and no committed record pins `ff6cdcb96cdcdad5`.
- The only mentions are historical verdict prose under `docs/handoffs/submix-sends-2026-10-02/verdicts/`.
- The `qualification.yml` step "Validate Issue-544 runtime audit records" checks the digest only
  against `[0-9a-f]{16}`.
- `scripts/run-aarch64-tests.sh:170-185` does not read the digest at all.
- The expected key set is unchanged.
- I extracted both validators and ran them on my record. Both pass.
- Negative controls fail as they should: `allocations` set to 1, and the `pcm_digest` key removed.

### The capi row with a pending candidate

The implementer's argument holds, from `compile.rs:282-399`:
- The structural peak is `cur.capi + cand.epoch + cand.protocol`.
- The live pending peak is `cand.capi + cur.epoch + cand.protocol`.
- They are equal when `cur.protocol == cand.protocol`, which a catalog-keeping `SetSourceContent`
  guarantees.
- So "one byte below" refuses the structural edit first and cannot be driven through the C ABI.

Gate 3 says "once with a structural candidate pending", and the implementer covered that once, on
the graph row. Reading it as a single case is reasonable. #1257's unit test covers the capi row's
arithmetic.

### #1256 MINOR-1 fold-in

`two_track_routed_submix_session` routes `eq0` to `bus0` and `bus0` to the original destination.
That gives two tracks, three submix strips and a route into a submix.

I re-ran **O3**: the strip charge folds over `normalized_model().tracks` instead of `.strips()`.
- It is red on "routed submix" only: capi's observed bytes against `capi_retained_bytes` at
  `resource_lifecycle.rs:734`.
- The three earlier sessions stay green.
- This closes the gap the #1256 verdict found.

### Authorized paths and the acked-batch question

The diff touches four files: the spec, `live_tests.rs` (gate 4 only), `resource_lifecycle.rs` and
`tools/audit/src/capi.rs`. All four are authorized, and no product code changed.

The acked-batch question:
- The race's final-block differential is the ack-then-drop check for raced edits. MINOR-1 limits it
  in practice to pans and faders.
- Gate 3's refused runs show that a cap refusal leaves no record and no revision.
- Gate 4 shows that a refusal by the protocol pushes nothing.

## Test value: one sentence per new or rewritten test

- **`live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`.** It turns
  red in two cases:
  - A fader or matrix drain allocates or frees on the render thread while live records stream in.
    I re-ran M1-b (a `Box` per record in `drain_matrix_controls`): `Snapshot { allocations: 47, .. }`.
    The implementer recorded the rest of capi green under it.
  - A live edit committed while a plan swap is in flight is lost. I re-ran M1-a (push to
    `self.providers`): red in run 0 on 5 of 5 invocations.
  - Mutes are not covered; see MINOR-1.
- **`live_edits_are_admitted_at_their_exact_graph_and_capi_peaks_and_refused_one_byte_below`.** It
  turns red if the live admission is fed the wrong inputs: the wrong prospective model, or no pending
  candidate. #1257's arithmetic unit test cannot see either. I re-ran M3-e (the current model passed
  as the prospective one): "graph: one byte below" admitted, while all 47 capi lib tests stayed
  green.
- **`a_live_edit_without_reliable_event_room_is_protocol_backpressure_and_pushes_nothing`.** It
  turns red if a live transaction commits, or moves a strip queue, while the reliable-event lane has
  no room for its `SESSION_COMMITTED`. One example is a future live fast path that skips the
  protocol's structural prepare. The pinned `EVENT_FULL` vector covers only a rebuild edit and
  checks no lane.
- **`audit capi` with live edits (gate 2).** It turns red (abort, exit 134) if draining a non-empty
  matrix lane allocates on the render thread. I re-ran G2-M1, then its control: the pre-#1258 audit
  under the same mutation stays green, with digest `ff6cdcb96cdcdad5` and 0 violations.
- **The `capi_retained_bytes_charge_every_byte_the_compile_retains` fold-in.** It turns red if the
  capi strip charge leaves out submix strips. I re-ran O3: red on the routed-submix case only.

## Gates I re-ran (export of `0fdb4f886`)

| Gate | Result |
|---|---|
| `cargo test --locked -p capi` | pass: lib 47; `resource_lifecycle` 13, in 7.33 s |
| Race, 10 sequential + 40 concurrent + 6 CPU-pinned | 56 of 56 pass |
| `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | ok |
| `audit capi` | rc 0. `calls` 100,000, `pcm_digest` `18e56b897a3abf17`, every forbidden counter 0, `total_violations` 0 |
| `qualification.yml` "Validate Issue-544 runtime audit records" (extracted) | pass; negative controls fail as they should |
| `run-aarch64-tests.sh` capi record check (extracted) | pass |
| `cargo test --locked --release -p audit -p bench -p console-workload` | pass: 113 tests |
| `bash scripts/check-capi-abi.sh` and `--self-test` | ok, and "C ABI mutation tests: ok" |
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked -p capi -p audit --all-targets --all-features -- -D warnings` (lean; the touched crates) | ok |
| `check-` and `test-` `realtime` and `workspace` `-policy.sh` | ok |
| `cargo test --locked -p protocol` | pass: 146 |
| `scripts/check-ci-path-routing.py`, and the router on this diff | contract passed; `route=full` |

Not run:
- `check-cross-targets.sh`: no product code changed.
- The worklet chain: nothing compiled into the browser module changed.
- AArch64: CI-only.
- Full-workspace clippy: I ran the lean form on the touched crates instead.

## Mutations I ran

Each one was applied in the export, run, and restored from a saved copy. I diffed `crates/` and
`tools/` clean against `git archive 0fdb4f886` afterwards.

| Mutation | Result |
|---|---|
| M1-a: the live arm pushes to `self.providers` | race red in run 0, on 5 of 5 invocations |
| M1-b / G2-M1: a `Box` per record in `drain_matrix_controls` | race red (47 allocations); `audit capi` aborts (134); the pre-#1258 audit is green |
| M3-e: `live_admission` given the current model as the prospective one | gate 3 red ("graph: one byte below"); capi lib 47 of 47 green |
| O3: the strip charge over `tracks` | oracle red on "routed submix" only |
| Mute-drop (mine): the live arm skips `Mute` records | **race green** on 3 of 3 (MINOR-1); two #1257 live tests red |
| Planted panics, four of them (hang probe) | each fails in at most 10.1 s; none hangs |
