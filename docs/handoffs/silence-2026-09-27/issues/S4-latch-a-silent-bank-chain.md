# Latch a silent bank chain

Draft, slice S4 of the silence architecture issue (A0). Class A. Evidence and a working prototype:
`docs/handoffs/silence-2026-09-27/DESIGN.md` sections 4.5 and 6, and
`whole-bank-latch-prototype.patch` (applies to `3fc79b5a`; its names are not this contract's, and its
probes are not to be copied: use #960's `rack::test_only_bank_phase_profile`).

## Product outcome

A bank chain whose every lane's input is `+0.0` and whose every slot is at rest renders nothing: no
gather, no slot `process`, no scatter. It writes its outputs as constants and advances only what the
skipped kernels would have advanced. Prototype, native eight lanes, same runtime, A/B:

* `sixty_four_track_idle` 29.4 → 4.97 µs p50 (bound feed; the remainder is the bound copies, the
  input scans that S1 removes, and bookkeeping S5 trims);
* all-silent builtins-only strip 23.9 → 3.85 µs (eight lanes), 41.2 → 4.41 µs (four lanes);
* the dogfood song, 64 tracks: console strip 98.1 → 92.5 µs mean, builtins-only 24.5 → 16.8 µs;
* dense and downbeat blocks unchanged within noise.

## Interface contract

* **Effect claims.** `effect_contract::PreparedNativeEffectBank` gains provided methods
  a silent-rest query and an exact skipped-block update (signatures below). Implemented from the
  existing observation-earned claims, with no new claim logic:
  * parametric EQ: `bypass || (silent_fixed_point && both channels have no ramp in flight)`
    (`crates/parametric-eq/src/lib.rs:2071-2141`);
  * compressor: `silent_fixed_point && max_remaining() == 0` on both channels `&& silent_bypass ==
    bypass` (`crates/compressor/src/lib.rs:490-555`). `render`'s fourth leg, `detector` is `Main` or
    `Silent` (`:508-514`), is vacuous for a bank because `process_bank_inner` refuses a sidechain
    (`:1089`); the doc comment says so, and a future bank sidechain must add the leg;
  * true-peak limiter: `silent_fixed_point && silent_bypass == bypass && ramps_are_stationary` on the
    four ramp arrays; `advance_silent_rest(frames, mono)` performs exactly the latched bodies'
    update: `cursors.advance` plus `advance_rest_phase` on both channels when dual
    (`crates/true-peak-limiter/src/lib.rs:2208-2231`), on the left channel only when `mono`
    (`:2302-2312`).
  The provided methods are therefore `fn silent_rest(&self) -> bool { false }` and
  `fn advance_silent_rest(&mut self, frames: u32, mono: bool) {}`.
* **Stages.** `rack::EffectBankStage` forwards `silent_skippable`/`advance_silent` to the two
  methods above. `rack::ConsoleEffectBankStage` returns `false` in this slice whenever its bypass
  shunt feeds a line or any observation tap is armed or its drain admitted anything this block;
  otherwise it forwards (S11 and a follow-up extend it). `BuiltinStage` forwards (S3).
* **Members.** `rack::BankMembers` gains `fn input_silent(&self, lane: usize, frames: usize) -> bool`
  (default: the #942 predicate over both planes of `plane(lane)`) and
  `fn skip_silent(&mut self, active: &[bool], frames: usize, fold: &[bool]) -> bool` (default
  `false`: decline, nothing written). `active` is the chain's active mask; the chain's width is not
  the member count (a partial bank's `bank_outputs` has only `population` entries,
  `crates/graph/src/runtime.rs:2654-2694`), so every access is guarded by `active[lane]` exactly as
  gather and scatter are. `graph`'s `ArenaMembers` (`crates/graph/src/runtime.rs:1520`) overrides:
  * `input_silent`: a source-claim lane answers from `played_silent` (S1) without a scan; any other
    lane scans its gather source;
  * `skip_silent`: every active unfolded lane's output buffer is filled with `+0.0` over `frames`;
    the active folded lanes' constant is applied to the master as A0 D5 states: per plane, the zero
    `route_word([ll, lr, rl, rr], +0.0, +0.0)` of each folded lane in ascending lane order
    (`crates/graph/src/runtime.rs:1947`); when lane 0 stores, the master is filled with the
    left-to-right `f32` sum of those zeros; otherwise, if any is `+0.0`, one `x + (+0.0)` pass over
    the plane, and no write if all are `-0.0`. It declines (returns `false`, nothing written) on the
    fold kernels' own premises: a folded lane exists and the master is not writable, or a lane other
    than lane 0 stores (`fold_resident_tiles` declines on it, `:1843`). The scalar constant arithmetic lives in
    non-generic `#[inline(never)]` functions (rule 3).
* **The chain.** In `rack::BankChain::run_with_input` (`crates/rack/src/lib.rs:2433`), after the
  drain loop and before the collapse decision:
  1. qualify := no resident predecessor, no auxiliary destination, every active slot
     `silent_skippable()`, every active lane `input_silent`;
  2. qualified and sealed (and `frames == sealed_frames`, and the collapse decision this block would
     take equals `sealed_collapsed`): call `skip_silent(&self.active, ...)`; if it returns `true`,
     call `advance_silent(frames, sealed_collapsed)` on every active slot, count one skip, and return
     `Ok(())`. The collapse decision is evaluated cheaply from state that cannot change on a
     qualified block (no drain admitted a record), or the implementer caches it with the seal and
     releases the seal on any witness change;
  3. qualified, not sealed: run the block normally; after the last slot (after the collapse seam
     on a collapsed block), seal iff the whole resident block `[..frames * lanes]` of both planes is
     `+0.0`, and record `sealed_frames = frames` and `sealed_collapsed = collapse`;
  4. not qualified: unseal.
  A skipped block does not touch `transposes`, the collapse state or any scratch word.
* **Oracle and counters.** A test-support thread-local `test_only_set_silent_latch_declined(bool)`
  (the declined arm runs exactly today's code) and `test_only_silent_latch_counts() -> [skips,
  seals]`, re-exported beside graph's existing `test_only_*` knobs. None of it exists in a production
  build.

## Hazards

* **Partial banks.** The chain's width is not its member count (`trailing_active_mask`); qualify,
  skip and fill over the active lanes only. The prototype called `skip_silent` with the width and
  would index past the member list of a partial bank (only full banks were exercised).
* **Evidence counters.** On a skipped block these do not advance: the chain's `transposes`
  (`crates/rack/src/lib.rs:2550`), `collapses` and `transitions` (the collapse decision is not
  taken, `:2521-2523`), the builtin processors' `[process_calls, frames_processed]`. And these may
  change: `TEST_ONLY_RESIDENT_COUNTS` and `test_only_count_source_plane`, because a scanned
  `input_silent` calls `plane()` per lane (`crates/graph/src/runtime.rs:1609-1632`). Rule: the input
  scan reads the gather source through a path that does **not** count as a gather, and every pin of
  these counters on a row that can skip counts skips (`transposes + skips == blocks * chains`). No
  standing test pins them on a silent row today (`tools/console-workload/tests/chain_shape.rs` pins
  tone rows); S2's source-plane pins must be written with this rule.
* **The limiter on a collapsed chain.** It sits on `simd2`, upstream of the seam, so a collapsed
  block runs its mono body, which advances only the left channel. `advance_silent` must receive the
  mode (see the interface); the prototype advanced both channels and its output-only comparison
  could not see it.
* **Drains.** Every stage must have drained in `begin_block` before the qualify check (S3 moves the
  fader and matrix drains); a stage that still drains in `process` must decline.
* **Collapse.** The skip returns before the collapse decision and leaves every collapse flag as it
  was; the seal check runs after the seam copy on a collapsed block.

## Authorized paths

`crates/rack/src/lib.rs`, `crates/graph/src/{lib.rs,runtime.rs}`, `crates/effect-contract/src/lib.rs`
(the two provided methods and docs), `crates/parametric-eq/src/lib.rs`, `crates/compressor/src/lib.rs`,
`crates/true-peak-limiter/src/lib.rs` (each: the two methods only), new tests under
`crates/rack/tests/`, `crates/graph/tests/`, `tools/console-workload/tests/`, their `MUTATIONS.md`,
this spec.

## Non-goals

No generic payload compare (S8), no tail paths (S6), no per-slot skip inside a running chain (S9), no
silence table (S10), no delay lines (S11), no meter shortcut (S12), no regrouping (S7). A console
stage with a shunt line or an armed tap declines.

## Objective gates

1. **Bit identity against the declined oracle.** Two runtimes of the same plan, one declined, render
   the same blocks. On every block compare: the host planes; every chain's resident block (the words
   `final_output_block` exposes); every meter snapshot of the metered row; and, at every seal,
   release and every 64th block, every effect slot's state payload for every lane
   (`snapshot_track_state_payload`, read outside the render call) and every builtin bank's lane
   state words. All must be bit-identical, including during a collapsed run. Corpus, at widths 8
   and 4 (the four-lane run uses builtins-only strips on x86, where effects do not bank at four
   lanes):
   * strips: console, builtins-only, EQ-only, compressor-only, gain/pan, the mono fixture (collapse
     engaged), the metered row;
   * patterns: all silent; tracks 16-63 silent; seeded random on/off runs (1-40 live, 1-900 silent)
     with a common eight-block downbeat every 1,500 blocks; the first 8,000 blocks of the S2 activity
     fixture; a 9-track ragged strip (a partial bank);
   * feeds: source set with silent-bit `Some` blocks, source set with `None` (absent), and the bound
     feed (scan path);
   * hostile routes: a session whose route rows are all negative (`-0.0` contributions, the no-write
     arm), and one whose first bank is silent while a later one is live (a storing lane skipped);
   * a render of one short block (`frames < quantum`) after a sealed run;
   * live records during silence: fader, mute, matrix and trim records (instant and smoothed, `Left`
     only and `Both`) admitted at random blocks of a sealed run on a console-attached plan, and a
     control-attached effect plan with parameter points and bypass toggles.
   The activity-pattern source comes from S2 (the harness's twin of its driver).
   The prototype's version of this corpus renders 0 mismatches with up to 53,393 skips per run.
2. **It fires.** On the idle row every block after the first seal skips all 8 (or 16) chains; on each
   pattern the skip and seal counts are recorded. At least one skip is required on the ragged
   (partial-bank) strip, the mono fixture while collapsed, and the metered row; a zero is a failure,
   not a record.
3. **Red mutations** (each alone turns gate 1 red; record in `MUTATIONS.md`): skip without the seal;
   treat a storing lane as accumulating; drop the `+0.0` fix-up; apply the fix-up when every zero is
   `-0.0`; skip the unfolded-lane fill; drop the limiter's `advance_silent_rest`; advance both
   limiter phases on a collapsed block (both red through gate 1's state-payload comparison, since
   the output cannot see them); fill a lane outside `active`; keep the seal across a block whose
   drain admitted a fader record (needs S3's drain move); keep the seal across a `frames` change or
   a collapse transition.
4. **Worst case.** A downbeat harness (4,000 silent blocks, then 64 loud, six cycles, arms alternated)
   records the first loud block's median and maximum and the steady loud p50 for both arms; the
   latched arm's median may not exceed the declined arm's by more than the declined arm's spread.
   Chunk-interleaved dense A/B (at least 8,000 blocks, arms alternating every 500 blocks) likewise.
   Long-pattern A/B runs must interleave: a back-to-back pair measured 22 % host drift in the
   prototype.
5. **Realtime.** Allocation-free render tests with the latch firing; `scripts/check-realtime-policy.sh`;
   the AudioWorklet callgraph gate (rule 3: list each new function's vector/scalar counts).
6. **Wasm.** `host-web` builds for `wasm32-unknown-unknown` with `simd128`; the wasm console arm's
   digests are unchanged; gate 1's corpus for the console strip runs once in the wasm gate harness
   (`tools/wasm-gates`), because on x86 effects bank only at the native width
   (`has_matching_backend_width`) and the wasm arm is the only four-lane effect bank.
7. fmt, clippy with `-D warnings`, `cargo test -p rack -p graph -p console-workload -p parametric-eq
   -p compressor -p true-peak-limiter`, with and without `graph/test-support`.

## Console benchmark rows

Moves `sixty_four_track_idle` and the S2 sparse rows. Every digest is unchanged. Dense rows must not
move beyond noise.

## Dependencies

S3 (the seam and builtin claims). S1 for the scan-free source path (without it, `input_silent` scans
every lane). S2 lands first so the movement is measured.

## Standing rules for the implementer

- Work only from this body and the prototype patch. Read the cited functions first.
- Class A: every gate that says "bit-identical" is a hard stop.
- The owner's copy rule: this slice removes block-sized work and adds only the constant fills a
  skipped scatter owes.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving; the
  in-process A/B tables are descriptive evidence.
