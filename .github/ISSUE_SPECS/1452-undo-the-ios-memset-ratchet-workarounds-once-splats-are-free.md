# Undo the iOS memset ratchet workarounds once splats are free

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (Amendment 1 A3 of *Let the builtins splat their chain constants
without iOS memset calls*, #1451). Code anchors verified on `codex/d15-stream-g` at `6bc14d704`
(the commit that filed #1451). Ordered directly after #1451.

## Product outcome

Five render and reset sites are written in a second-choice shape for one reason only: their natural
shape put more `memset_pattern16` libc calls in the `aarch64-apple-ios` release assembly (known
defect #1018), and the iOS ratchet refused it. Once #1451 removes the cause of those calls, each
site goes back to its natural shape when that shape is better, with no rendered bit moved and no
iOS count raised. A site whose natural shape is not better keeps its current shape, and this issue
records why. After this slice no kernel carries a shape that exists only for the ratchet without a
recorded reason.

## Context

- **#1451 (open, Stream G, directly before this slice).** Its D1 finds why a splatted `f32`
  constant lowers to `bl _memset_pattern16` on `aarch64-apple-ios`; its D2 fixes that cause in one
  shape (expected in `crates/lane/src/wide_impl.rs`, `Lane::splat`); its D3 returns the builtin
  input chain's carried constants (`InputChainConstants`, the EQ's `flush_eps` word) to splats.
  Its D3 leaves the five shapes below as they are, and its Amendment 1 A3 orders this issue for
  them. Root ruled on 2026-10-06 that #1018 is widened to every crate and that #1451 executes it.
- **The ratchet.** `scripts/check-cross-targets.sh` (`:100-139`) counts `^\tbl\t_memset_pattern16$`
  in each product crate's `aarch64-apple-ios` release assembly (`cargo rustc --release --target
  aarch64-apple-ios --crate-type rlib --emit asm`; no Xcode, no link).
  `scripts/lib/aarch64-known-defects.py` `IOS_MEMSET_CEILINGS` (`:70-81`) holds each crate's
  ceiling: a count above its row fails, a count below asks for the row to be lowered, a crate at
  zero with a row fails until the row is deleted. Today's rows (before #1451): `builtins` 71,
  `graph` 10, `parametric-eq` 48, `soft-clip` 22, `true-peak-limiter` 104 among them. The check
  runs locally (the `aarch64-apple-ios` target is installed on the development host).
- **The five workarounds (current shape, natural shape, why it was chosen).**
  1. **#1407, the filter ramp's leading countdown carried as a per-lane word.**
     `INPUT_FILTER_LEADING_UPDATES` and its doc naming #1018
     (`crates/lane/src/kernels/builtins.rs:1266-1274`); the `filter_leading: [[L; 2]; 2]`
     parameter of `input_chain_ramp_block_filter` (`:1031`, `:1050`, `:1067`, `:1088`, decremented
     and compared at `:1162-1164`) and of `input_chain_ramp_block_filter_mono` (`:1290`, `:1306`,
     `:1321`, `:1341`, `:1403-1405`); the `leading` mask that `filter_ramp_words` takes
     (`:1236-1256`). The owner builds the words per block in `InputStage::load_filter_leading`
     (`crates/builtins/src/lib.rs:1336-1350`, `remaining - 60` per lane) and passes them at `:1786`
     and `:1933`, beside `load_filter_countdown` (`:1357`). Natural shape: the bodies derive
     `leading` from the ramp countdown they already carry (`filter_remaining`), compared with a
     splatted constant, and the extra per-lane words, their per-frame decrement and the owner's
     builder go. The doc says why it is carried: a splatted `60.0` is a vector constant, and "on
     Apple targets each one is a `memset_pattern16` call in the render function". Commits
     `f44cf54bf`, `0890f8882` (#1407; `crates/builtins/src/lib.rs`, `filter_liveness.rs`). Test
     users: `crates/lane/tests/filter_ramp_line.rs:22`, `:191-192`;
     `crates/builtins/tests/filter_liveness.rs:1088` (doc).
  2. **#1220, the route ramp's frame index built from a `u32` counter.** `route_mix_ramp_block`
     (`crates/lane/src/kernels.rs:1675`), comment and counter at `:1700-1708`. Commit `8c6268967`
     (#1220 Amendment A2; its kernel change is in `crates/lane/src/kernels.rs`, the rest of the
     commit is unrelated verdict minors) replaced `let advance = L::splat(L::WIDTH as f32)` and a loop-carried
     `index = index.add(advance)` with `L::splat(first as f32).add(offsets)` and `first +=
     L::WIDTH as u32` per chunk, because the splatted `4.0` raised `graph`'s count from 10 to 11.
     Natural shape: the loop-carried vector add (one add per chunk in place of a convert, a
     broadcast and an add).
  3. **#1089, the EQ's `lanes_mask` from one flag vector.** `lanes_mask`
     (`crates/parametric-eq/src/lib.rs:1063-1078`, doc naming #1018), called once from
     `recover_failed_lanes` (`:1784`), D7's failing path, out of line and never reached by the
     frame loops. Commit `858ccb848` (`crates/parametric-eq/src/lib.rs` only) replaced an `or` of
     `lane_mask::<L>(lane)` one-hot compares, each against a splatted `1.0`, with one flag array
     loaded and compared with `+0.0`, because the outlined `or` form raised the crate from 151 to
     160. Natural shape: the `or` of `lane_mask`s.
  4. **#1091, the limiter's out-of-line `clear_runtime`.** `#[inline(never)] fn clear_runtime`
     with its doc naming #1018 (`crates/true-peak-limiter/src/lib.rs:624-636`), called from
     `reset_to_defaults` (`:583`) and `reset_keeping_parameters` (`:621`): preparation, a reset and
     a failed block, never the frame loop. Commit `cd1f02bb2` (`crates/true-peak-limiter/src/lib.rs`
     only) added the attribute because inlining put three more calls in the crate (104 to 107).
     Its body fills buffers: `self.required_ring.fill(1.0)` and `self.box_ring.fill(1.0)`. Natural
     shape: no attribute; LLVM decides.
  5. **#1409 D5, soft clip's loop-invariant branch in place of two body copies.**
     `soft_clip_block` (`crates/soft-clip/src/kernel.rs:186-240`): one frame loop with `if ramping`
     on every frame, the settled arm still adding a zero step to `drive`, `output` and `mix`. The
     comment at `:186-189` says why: "a second copy of this kernel doubles its stored splat
     constants" (#1018). Commit `8deee2c5e` (#1409 attempt 1, on `codex/d15-stream-g`; #1409 is
     open). Natural shape: two copies of the body chosen once per block, a ramping one with
     `ramp_toward` and a settled one with no ramp arithmetic.
- **Not in scope.** The multiband compressor's single body with the counter and joint flush on
  every frame (#1328 A9) is not a carried constant; #1451's attempt record holds root's question
  whether it should gain an unarmed form. The carried constants #1451 D3 removes are #1451's.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled that #1018 is widened to every
  crate and executed by #1451, which keeps to the cause, the fix and the builtins constants; and
  ordered this issue, filed now and placed in Stream G directly after #1451, to undo the five
  earlier ratchet workarounds. Each undo is gated bit-identical (a base-versus-head differential),
  with its crate's iOS ceiling unchanged or lower and the V8 spill gate clean. Each undo states
  whether the natural shape is better, by codegen and a descriptive p50; an undo that is not
  better stays out, and this issue records why. Reason: a shape that exists only to satisfy a
  ratchet whose cause is fixed is cost without a claim, but a shape that is also better on its own
  stays.
- **D1. Start from #1451's measured tree.** Take the per-crate iOS counts of `builtins`, `graph`,
  `parametric-eq`, `soft-clip` and `true-peak-limiter` on the tree after #1451 merges, and #1451's
  D1 root-cause sentence. If #1451 leaves a construction that still makes calls, read its attempt
  record first: an undo whose natural shape is that construction is expected to raise the count
  and is recorded, not attempted against the ratchet.
- **D2. One undo per commit, in this order:** 2 (#1220), 1 (#1407), 5 (#1409 D5), 3 (#1089),
  4 (#1091). Each commit is its own base for the next. Each undo restores the natural shape named
  in Context, with the #1018 doc removed or replaced by one sentence that states the shape's own
  reason. No other change rides along.
- **D3. Keep-or-revert rule, decided per undo before timing.** An undo lands when all of these
  hold: gates 1-3 pass; the instruction count of the affected loop or function on x86-64-v3 and on
  `simd128` (and the TurboFan count where the site is in the worklet's held rows) is not higher than
  the base, or each increase is named and is the cost of a simpler body; and the descriptive p50
  is at most +2 % of the base (`two-percent-allowance-for-better-code`). Otherwise the undo is
  reverted, and the attempt record gives the counts, the p50 and the reason. A site off the frame
  loop (3: D7's failing path; 4: preparation, reset and a failed block) has no production render
  path a benchmark reaches; its record says "p50: not measured, off the frame loop" and codegen
  and the iOS count decide (`benchmark-real-paths-only`).
- **D4. The ceilings only fall.** An undo that raises its crate's iOS count is reverted (gate 2).
  Each row the check reports below its ceiling is lowered, and a row at zero is deleted, as the
  check asks. No row is raised.
- **D5. Expected outcomes, not gates.** Undo 4 fills a buffer (`[f32]::fill(1.0)`), which LLVM's
  loop-idiom pass may still turn into `memset_pattern16` on Darwin after #1451's splat fix: if the
  inlined form raises `true-peak-limiter`'s count, it stays out of line and the record says so.
  Undo 3's current form (one load and one compare) may already be the better form: codegen
  decides.

## DSP evidence (AGENTS.md)

- **Equations, coefficients, update rules:** none change. Each undo computes the same words in the
  same order: the filter ramp's leading predicate selects the same frames (the four updates that
  leave a countdown of 63 down to 60), the route ramp's index words are the same exact integers
  (`position + vectored < length <= 2^22`), the failed-lane mask selects the same lanes, the
  limiter's reset writes the same words. Soft clip's settled arm today adds a zero step to
  `drive`, `output` and `mix`; `x + (+0.0)` is `x` for every `x` except `-0.0`, which becomes
  `+0.0`, and `x + (-0.0)` is `x` for every `x`. A settled copy that drops the add is therefore
  bit-identical only if no settled word is `-0.0` while its step is `+0.0`, or if the copy keeps
  that add: undo 5's commit states which, and gate 1 checks it.
- **Numerical limits, latency, tail, units, smoothing, denormal/NaN:** unchanged.
- **Fixtures and objective tests:** gates 1-5. **Benchmarks:** gate 4, descriptive. **Listening:**
  none; no bit moves.

## Deliverables

1. Up to five commits, one per undo (D2), each with its gate evidence in the commit message or this
   spec's attempt record.
2. Per undo, a row in the attempt record: kept or reverted, base and head iOS counts of the crate,
   x86-64-v3 and `simd128` instruction counts of the affected loop or function, TurboFan count where
   held, p50 table (or the D3 off-loop note), and the reason.
3. `scripts/lib/aarch64-known-defects.py` rows lowered or deleted per D4, with the comment above
   `IOS_MEMSET_CEILINGS` updated.
4. The PR evidence: the base-versus-head differential per undo (gate 1).

## Authorized paths

- `crates/lane/src/kernels/builtins.rs` (undo 1: `INPUT_FILTER_LEADING_UPDATES`, the
  `filter_leading` parameters, `filter_ramp_words`'s `leading` input) and
  `crates/lane/tests/filter_ramp_line.rs` where it names the removed items
- `crates/builtins/src/lib.rs` (undo 1: `InputStage::load_filter_leading` and its two call sites
  only; stream A's file, named exception) and the doc at
  `crates/builtins/tests/filter_liveness.rs:1088`
- `crates/lane/src/kernels.rs` (undo 2: `route_mix_ramp_block`'s index only)
- `crates/parametric-eq/src/lib.rs` (undo 3: `lanes_mask` and its doc only)
- `crates/true-peak-limiter/src/lib.rs` (undo 4: `clear_runtime`'s attribute and doc only)
- `crates/soft-clip/src/kernel.rs` (undo 5: `soft_clip_block` only)
- `scripts/lib/aarch64-known-defects.py` (the ceilings and their comment, D4)
- this spec

## Non-goals

- Anything #1451 owns: the cause, the lane-crate fix, the builtin chain constants, the EQ's
  `flush_eps` word, closing #1018.
- An unarmed form for the multiband compressor (#1328 A9; root's question in #1451).
- Any other kernel restructuring, tuning around an undo's result, or a second timing run.
- Native AArch64 timing (CI-only, no funded runner).
- Rejected alternatives:
  - Undo all five in one commit: a regression could not be traced to its site, and one bad undo
    would block four good ones.
  - Keep every workaround because it is already correct: D0 rules that a shape kept only for a
    fixed defect must earn its place by D3.
  - Raise a ceiling to admit a natural shape: the ratchet only falls.

## Hazards

- This slice does nothing until #1451 has merged. If #1451 stops after its D1 (no single
  construction causes the calls), this issue waits for the rescoped fix.
- #1409 is open and owns `soft_clip_block`; undo 5 rebases onto #1409's final D5 shape, and it
  lands after #1409 and #1411's Stream G pull request if they have not merged by then.
- Hot files: `crates/lane/src/kernels/builtins.rs` and `crates/builtins/src/lib.rs` (stream A's
  file; STREAMS.md hot-file note of 2026-10-06), `crates/parametric-eq/src/lib.rs` (STREAMS.md row:
  G #1328 → A → G #1337 → G #1372). This slice touches only the five sites.
- Splitting soft clip's body may change register allocation in the worklet; the V8 spill gate
  (gate 3) decides, and a spill is a revert, not a tuning task.
- `check-lane-policy.sh` refuses some spellings inside `crates/lane`; use the crate's accepted forms.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** Per undo, a base-versus-head differential over the scenario set
   #1451 uses (724 scenarios, or a superset) that reaches the site: every output identical. PR
   evidence, not a committed test. Every pinned artifact is unchanged:
   `g5_native_digests_match_pins`, `BUILTINS_DIGESTS`, `E9_DIGESTS`, multiband `DIGESTS`, the soft
   clip and limiter corpus digests, `conformance_fixtures --check`, `check-builtins-fixtures.sh`,
   `check-graph-determinism.sh`. `crates/builtins/tests/filter_liveness.rs`,
   `crates/lane/tests/filter_ramp_line.rs`, the route ramp tests and `crates/soft-clip/tests/`
   pass.
2. **iOS ceilings unchanged or lower.** After each undo, `bash scripts/check-cross-targets.sh`
   passes, and the touched crate's count is at or below its count on the base of that undo. The
   before and after count of every touched crate is in the PR.
3. **V8 spill gate clean.** `bash scripts/run-wasm-gates.sh` exits 0 after each kept undo (native,
   `simd128`, V8 spill gate with every held row clean).
4. **Better or recorded (D3), descriptive.** Codegen: x86-64-v3 and `simd128` instruction counts of
   the affected loop or function, base and head. p50: `scripts/web-mixing-automation-benchmark.mjs
   run` on the shipped module of base and head for undos 1, 2 and 5 (the browser documents that
   reach the site), one warmup and two measured rounds each, interleaved, pinned to one CPU, host
   load average recorded; not retried, not tuned.
5. **Workspace gates.**
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p soft-clip -p true-peak-limiter -p parametric-eq -p builtins -p graph -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - the worklet chain, as in `qualification.yml` (`build-web-audioworklet.sh --named-twin`,
     `check-web-audioworklet.sh --without-metadata-regeneration`,
     `check-browser-expected-resources.py`, `test-web-audioworklet.sh`)
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-graph-determinism.sh`, `bash scripts/check-workspace-policy.sh`,
     `python3 -B scripts/lib/aarch64-known-defects.py --self-test`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

This slice adds no new test. Its claims are held by existing gates: the iOS ratchet turns red if an
undo brings a libc call back; the pinned digests, the filter-ramp tests
(`filter_ramp_line.rs`, `filter_liveness.rs`) and the differential turn red if an undo moves a
word; the V8 spill gate turns red if a held loop spills. A test that only names a removed item
(`INPUT_FILTER_LEADING_UPDATES`) is updated, not added.

## Dependencies

#1451; #1409 (root, 2026-10-06: a real dependency edge, not only a Hazards note, so undo 5 cannot land before #1409's final D5 shape; #1409 and #1411 merge in one pull request).

## Attempt record

None yet.
