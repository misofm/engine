PASS

# #1449 attempt 1: adversarial verdict

Commit `1e658aa65` (parent `af1c55a20`), branch `codex/d15-stream-h`. Reviewed from `git archive`
exports of both commits (no build or edit in the worktree). Binding: the spec at the commit,
`AGENTS.md` at the commit, decision 15 (D15-0, D15-10), the owner's no-shortcuts principle.

Verdict: the change does what D1-D3 and D9 say, only in the authorized paths, and every gate I re-ran
passes. No BLOCKER or MAJOR. Two MINOR findings: one is for root at spec level (the D7 tests have low
power in CI's shape), and one is a coverage gap that existed before this change.

## Scope and claims checked

- Diff: `crates/host-core/src/spectrum.rs` and the spec only. Correct.
- D1: `SPECTRUM_RESULT_SLOTS = 1` (`spectrum.rs:43`) builds the capture queue
  (`prepare_capture_with_handle`) and the resource-estimate queue, and caps every drain. Test helpers
  keep the literal `1`, which the spec allows.
- D2: `cancel` (`:511`), `commit_continuous` (`:654`) and `stop_continuous` (`:690`) each have a
  counted `for` with `break` on a failed pop, at the line where the `while` was. The order of the
  atomic stores around them is unchanged. `reset_after_cancel` (`:528`) is the old tail of `cancel`,
  moved unchanged.
- D3: `try_read_continuous_record` reads its count in the same place, keeps the same four arms, and
  returns the old `remaining_pops == 0` value after the loop. `continuous_available_at_entry` is
  deleted and had no other caller. The named twin compiles this function to identical code.
- D9: `cancel_except` (`:910`) has two loops (drain, then reset) and sits alone with blank lines
  around it. `cancel` and `select` call it at the place where the old loop was.
- Realtime rules: no allocation, lock, syscall, `[]` index, `expect`, `unwrap` or panic added to
  production code.
- Naming and version suffixes: the new names have no version suffix. No test greps source. No test
  is superseded.
- Acked-batch question: these queues carry spectrum results to the control side, and nothing acks
  them. The change can only discard fewer records: a record published after the count stays queued.
  No ack can come before a drop.

## Findings

### MINOR-1 (for root, spec level): the D7 tests are sound but have low power in CI's shape

`spectrum.rs:3236` (helper), `:3299`, `:3316`, `:3344`.

- **Soundness: confirmed.** The mutex in `render_while_producing` makes the producer's `Release`
  increment happen before `p0`. Each counted push is visible to `o1`. So `o0 + (p1-p0) - o1` is
  never more than the real number of pops. The tests cannot raise a false alarm on correct code. On
  the change:
  - 0/50 isolated runs failed for each test;
  - 0/60 test-runs failed with the three tests together on 4 CPUs;
  - 0 failures in 30 full-lib runs on 4 CPUs (plus 10 more earlier) and 30 on 2 CPUs.
- **Red on the parent (gate 2), measured again:**
  - isolated, 50 runs each on 32 CPUs: `cancel` 12/50, `restart` 16/50, `stop` 9/50. All 37
    failures are the intended `popped 2 records; it saw 1 at entry`. Gate 2 (at least 1 in 10) is
    met. The implementer's 1, 2 and 1 of 10 agree with these rates.
  - the three tests together, pinned to 4 CPUs: 28/60 failing test-runs.
  - **CI's shape** (debug profile, the whole `host_core` lib binary in parallel, pinned to 4 CPUs,
    the size of the public `ubuntu-24.04` runner): 3 red runs of 40, one test each time. That is
    about 2.5% per test per run. Pinned to 2 CPUs: 0/30.
- **Effect:** in CI, a `while` regression in one drain turns `qualification` red in about 1 run in
  40. These tests are not a reliable merge gate. The deterministic hold of the bound is B2b-1's
  marker: gate 3 shows the drain rule refuses each old `while` and accepts each new shape.
- **Why not MAJOR:** the spec chose this design (D7), accepted that it depends on scheduling
  (Hazards), and forbids lengthening or tuning without root (gate 2). The implementer did exactly
  that.
- **Recommendation for root:**
  - record the CI-shape power in the spec's evidence;
  - consider a deterministic complement. My probe: a capture around a two-slot queue holding two
    records, `cancel()`, then assert one record is left. It was green on the change and red on a
    `while` revert in every run (`ev/mut/deterministic-probe.txt`). It holds
    `.min(SPECTRUM_RESULT_SLOTS)`, which D1 requires.

### MINOR-2 (existed before this change): no test checks that a drain drains anything

The D7 tests check only an upper bound. If the drain loop is removed from `SpectrumCapture::cancel`
or `stop_continuous` (or the `for` is written `1..available`), every `host-core` (`--features
test-support`) and `host-web` (`--features test-support`) test stays green. I measured the same
result on the parent with the `while` lines deleted, so this change did not create the gap. It
leaves the gap in the drains it rewrote.

- `commit_continuous` is covered: `host-web` `ffi.rs:5381`
  `collection_selection_is_atomic_and_keeps_one_active_capture` goes red when its drain is removed.
- The possible defect: a cancelled or stopped capture keeps a stale window, which the next read
  returns. D9's test value names this same hazard for the collection.
- **Recommendation:** a successor issue, or a cheap single-thread assertion in a later attempt (one
  queued record, then `cancel()` or `stop_continuous()`, then the queue is empty). This is outside
  D7-D9, so it does not fail the attempt.

### NIT-1: name the full D8 shape correctly

D8's shape is one stale pop, then `Pending` or `Warming`, then a fresh record on the next read.
`bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop` (`:3160`) holds only the
first half. `failed_continuous_completion_drains_only_the_failed_queued_window` (`:4320`) holds the
whole shape, and it also goes red on both D3 mutations (measured). Name it in the Evidence too.

Also, despite its name, the named test cannot detect a chase. With D3's `for` replaced by an
unbounded `loop`, every `host-core` and `host-web` test stays green. This name came before this
change. D3's bound is held only by the realtime-policy marker (B2b-1).

### NIT-2: the D9 test uses a helper named for racing

The D9 test (`:3276`) builds its captures with `racing_capture` (`:3191`), but it does not race. A
neutral name such as `capture_with_held_producer` would read better.

## Test value (one sentence per new test)

- `cancel_pops_at_most_its_entry_count_while_a_producer_refills`: red when `SpectrumCapture::cancel`
  pops past its count at entry, as the `while` drain does when a producer refills the slot during
  the drain. No existing test runs a producer during a drain. Measured 12/50 isolated on the parent,
  about 2.5% per run in CI's shape (MINOR-1).
- `continuous_restart_pops_at_most_its_entry_count_while_a_producer_refills`: the same defect in
  `commit_continuous`, reached through `restart_continuous`. 16/50 isolated on the parent.
- `continuous_stop_pops_at_most_its_entry_count_while_a_producer_refills`: the same defect in
  `stop_continuous`. 9/50 isolated on the parent.
- `collection_cancel_except_drains_every_capture_but_the_kept_one`: red when `cancel_except` skips
  the drain (M1) or drains the kept capture (M2). Measured red on both. Every other host-core test
  binary (`--no-fail-fast`, all 29) and every host-web test stays green on M1 and M2.
- Related: removing the `keep` check from the reset loop (M3) is caught by an existing test,
  `tests/spectrum.rs:651` `prepared_collection_switches_exact_taps_without_audio_or_render_allocation`.

## Mutations (each applied alone to an export of the change, then reverted and checked with `cmp`)

| Mutation | Result |
|---|---|
| M1 `cancel_except` drain loop removed | red: only the D9 test |
| M2 `cancel_except` drain ignores `keep` | red: only the D9 test |
| M3 `cancel_except` reset ignores `keep` | red: `tests/spectrum.rs` `prepared_collection_switches_exact_taps_...` |
| M4 D3 stale arm removed | red: the named D8 test, `failed_continuous_completion_...`, `repeated_failed_completions_...`, `continuous_failed_block_with_multiple_completions_...` |
| M5 D3 post-loop always `Pending` | red: 5 tests, including the named D8 test |
| M6 D3 post-loop always `Warming` | red: 3 tests |
| M7 `SpectrumCapture::cancel` drain removed | green everywhere (same on the parent; MINOR-2) |
| M8 `commit_continuous` drain removed | red: host-web `collection_selection_is_atomic_...` |
| M9 `stop_continuous` drain removed | green everywhere (same on the parent; MINOR-2) |
| M11 D3 `for` replaced by an unbounded `loop` | green everywhere (NIT-1) |
| D2 drains back to `while` (the parent code) | D7 tests red at the rates in MINOR-1 |

## Gates run (on exports, `CARGO_TARGET_DIR` under `/tmp/claude-1002/v1449`)

1. `cargo test --locked -p host-core --lib spectrum`: 41 passed. `cargo test --locked -p host-core`:
   every binary ok (lib 68). `cargo test --locked -p host-web --features test-support`: lib 186
   passed, 2 ignored, other binaries ok.
2. Red on the parent: see MINOR-1. 50 isolated runs per test on the parent plus the new tests, and
   on the change. Also CI-shape runs pinned to 4 and 2 CPUs.
3. Realtime-policy marking. One function marked at a time, with markers in place of blank lines;
   each of the six runs prints `realtime policy: ok (90 marked regions in 25 files)`:
   - `cancel`
   - `try_read_record`
   - `commit_continuous`
   - `stop_continuous`
   - `try_read_continuous_record`
   - `cancel_except`

   Parent controls:
   - `cancel` (`:510`), `commit_continuous` (`:637`), `stop_continuous` (`:671`) and
     `try_read_continuous_record` (`:761`) are refused with `marked realtime unbounded try_pop
     drain`;
   - `try_read_record` is ok.

   `1418-attempt2-check-realtime-policy.sh` with only `cancel_except` marked prints `ok (90 marked
   regions in 25 files)`. The same gate refuses the merged-loop variant (reset inside the drain
   loop) at the pop (`:920`), which confirms the spec's "two loops" rationale.
4. Worklet chain:
   - **Named-twin builds:** `build-web-audioworklet.sh --named-twin` for head and base. Shipped
     `465b78a8...` (head) and `9b2b1a0f...` (base), the same digests the implementer reports, so
     the build is reproducible: ARTIFACT CHANGED, as expected.
   - **Artifact gates, all pass:**
     - `strip-wasm-names.py` `--self-test` and `check`;
     - `check-web-audioworklet.sh --without-metadata-regeneration`;
     - `check-browser-expected-resources.py --artifacts` (digests and exact rows agree; 32 red
       mutations);
     - `check-scalar-oracle-absent.py --wasm`;
     - `test-web-audioworklet.sh`;
     - the V8 spill gate on Node v22.23.2 (self-test and module);
     - `run-wasm-gates.sh --without-v8-spill --without-native` (142 cases, 250 comparisons, 0
       mismatches).
   - **Function-level comparison of the named twins** (`wasm-objdump -d`, call targets mapped to
     names with hashes normalised):
     - 2,694 -> 2,695 functions;
     - only new function: `SpectrumCaptureCollection::cancel_except`;
     - changed bodies: only `SpectrumCapture::{cancel, commit_continuous, restart_continuous,
       stop_continuous}` and `SpectrumCaptureCollection::{cancel, select, start_continuous,
       start_continuous_with_hop, stop_continuous, restart_continuous}`;
     - the closure of `miso_engine_web_v1_render` (25 functions) is IDENTICAL.
   - **Data section:** the same size (100,279 B). 36 bytes differ, all in the `line` field of 30
     `core::panic::Location` records that share one file pointer (`spectrum.rs`), with deltas +2,
     +28, +49 and +53. No column or file field changed.
   - **Other sections:** the function and element sections differ only because the new function
     shifts the indices.
   - **Browser legs** (`npm run qualify -- --artifacts <head> --sdk-root <tree>/sdk --browser <b>
     --check-matrix --self-test-mutations`, with a private PulseAudio null sink as CI uses):
     chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 each print `all qualification gates
     passed`.
   - **Pins:** no fixture, pin or expected digest was edited.
5. Lint and policy gates, all exit 0:
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked -p host-core --all-targets -- -D warnings`;
   - `bash scripts/check-realtime-policy.sh` (`ok (89 marked regions in 25 files)`);
   - `bash scripts/check-workspace-policy.sh`.

Not run: AArch64 (CI only). The batch's `qualification` run is still owed, as the attempt record
says.

Evidence (small files): `/home/bl/misofm/submix-verdicts/evidence/1449-attempt1/`.
