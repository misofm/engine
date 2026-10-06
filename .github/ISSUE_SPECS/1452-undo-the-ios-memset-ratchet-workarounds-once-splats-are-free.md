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

1. Up to six commits, one per undo (D2; undo 6 by Amendment 1), each with its gate evidence in the
   commit message or this spec's attempt record.
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
- `crates/lane/src/kernels.rs` (undo 2: `route_mix_ramp_block`'s index only; undo 6, Amendment 1:
  `silence_skip_block`, `silence_skip_settle` and their docs only)
- `crates/parametric-eq/src/lib.rs` (undo 3: `lanes_mask` and its doc only)
- `crates/true-peak-limiter/src/lib.rs` (undo 4: `clear_runtime`'s attribute and doc only)
- `crates/soft-clip/src/kernel.rs` (undo 5: `soft_clip_block` only)
- `scripts/lib/aarch64-known-defects.py` (the ceilings and their comment, D4)
- *(Amendment 2, ratified.)* `crates/lane/tests/input_chain_arming.rs` (the removed argument),
  the `INPUT_FILTER_LEADING_UPDATES` import line in `crates/builtins/src/lib.rs`, step 1 of the
  soft clip module doc (`crates/soft-clip/src/kernel.rs`), and the section header at
  `crates/builtins/tests/filter_liveness.rs:934`
- *(Amendment 2, authorized.)* The stale "21 vector" prose for the route kernel:
  `scripts/check-web-audioworklet-callgraph.py` (`SCALAR_SLACK`'s comment and the route roster
  row's comment), `scripts/check-web-audioworklet.sh` (the `--kernel-min` comment) and the
  `route_mix_settled_tail` doc in `crates/lane/src/kernels.rs`
- *(Batch follow-up, verdict NIT 4.)* `crates/lane/src/kernels/builtins.rs`
  (`INPUT_FILTER_RAMP_UPDATES` and `FILTER_LEADING_FLOOR`) and `crates/builtins/src/lib.rs` (a
  const assertion beside the `lane` import and a `debug_assert!` in `load_filter_countdown`)
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
   run` on the shipped module of base and head for undos 1, 2, 5 and 6 (the browser documents that
   reach the site; undo 6 by Amendment 1), one warmup and two measured rounds each, interleaved, pinned to one CPU, host
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

## Amendment 1 (root ruling, 2026-10-06, before attempt 1)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15, on #1451's verifier MINOR 3
(2026-10-06). It governs where the sections above seem to disagree; Deliverables, Authorized paths
and gate 4 are updated to match.

- **A1. A sixth undo: the outlined `silence_skip_settle`.** The #1328 follow-up (`6293440ff`,
  `91724e66c`, recorded in `0725a8949`; landed) wrote `lane::kernels::silence_skip_block`'s
  settle path, the backward scan over a block whose last frame is zero on some lane, as one
  `#[inline(never)]` function per lane width (`silence_skip_settle`,
  `crates/lane/src/kernels.rs`), chosen while every constant vector cost a `memset_pattern16`
  call on `aarch64-apple-ios` (#1018); its doc says only that "the outlining stays as it is". It
  has no recorded reason of its own. It joins the undo list as **undo 6**, done last in D2's
  order (2, 1, 5, 3, 4, 6), in its own commit.
  - **Natural shape:** the scan inlined into `silence_skip_block` (no `#[inline(never)]`: the
    body in place, or an `#[inline(always)]` helper, whichever reads better); the live form
    unchanged.
  - **Keep-or-undo:** D3's rule applies unchanged. The site is on the frame path of the builtins
    input chain's and the EQ's unarmed form (every block whose last frame is zero on some lane,
    including a partial bank's padding lanes), so its p50 is measured (gate 4, browser
    documents). The counts are those of the callers that inline `silence_skip_block` (the
    builtins and EQ unarmed bodies) on x86-64-v3 and `simd128`, and the TurboFan count of the held
    rows. The outlining stays only if its own merit is measured (the inlined form is worse by D3),
    and the attempt record states the reason either way; the doc at the site then states that
    reason in one sentence, or goes with the outlining.
  - **Gates:** 1-5 as for every undo; gate 2 also covers `builtins` and `parametric-eq`, the
    crates that inline the function (a count that rises is a revert, D4).

## Amendment 2 (root rulings, 2026-10-06, after attempt 1's verdict)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), on the attempt-1 verifier's items for root. It governs where
D3 and gate 4 seem to disagree with it.

- **R1. Codegen is the keep criterion for a site no benchmark document reaches.** None of the
  four browser documents reaches undos 2, 1 or 5 (no soft clip, no input filter retarget, no route
  coefficient ramp; attempt 1's finding, verified by the verifier). For such a site the natural
  shape is kept when it is not worse in codegen: a shorter loop on every target, no rendered bit
  moved and no iOS count raised. Each increase is still named (D3). **p50 is not decisive there,
  and why:** a document that never executes the site measures only the module's layout and the
  host, so its p50 swings both ways (tables A to C: from -18.6 % to +4.9 % on code the documents
  never run) and is not evidence about the site; D3's "+2 % of base" clause applies to sites a
  document reaches (table D, undo 6, is flat). Undos 2, 1 and 5 stay kept on this rule; a
  measurement that reaches the sites would be its own tooling issue.
- **R2. Ratified path deviations** (each forced, none changes behaviour):
  `crates/lane/tests/input_chain_arming.rs` (compile), the `INPUT_FILTER_LEADING_UPDATES` import
  line in `crates/builtins/src/lib.rs` (the unused-import lint), step 1 of the soft clip module doc
  (`crates/soft-clip/src/kernel.rs`, it described the removed branch), and the section header at
  `crates/builtins/tests/filter_liveness.rs:934` (beside the authorized gate-8 doc).
- **R3. Authorized:** the stale "21 vector" prose for the route kernel, now 22 vector and 0 scalar,
  in the four places the verifier found (Authorized paths).

## Attempt record

### Attempt 1 (2026-10-06, implementer; base `045a0dbb3`, code at `6a2c5216a`)

Commits: `045a0dbb3` (Amendment 1 of this spec and of #1454), `862afc03a` (undo 2), `ca57f942c`
(undo 1), `00a0445c5` (undo 5), `271fdaca8` (undo 3, reverted: doc only), `766d6c95e` (undo 4,
reverted: doc only), `0f7c90ba9` (undo 6, reverted: doc only), `45cf6506b` (undo 1 doc
follow-up: a public doc linked a private constant, which `cargo doc -D warnings` refuses),
`517b339e2` (D4 comment). Each undo was built on the commit before it. Nothing below was tuned or
re-run, except that a scratch-example build failure in `check-cross-targets.sh` (the scratch
differentials sat in the tree and do not build for AArch64) was re-run after they were moved out.

**Outcome.** Kept: undos 2, 1 and 5. Reverted on their own measured merit (the current shape
stays, its #1018 doc replaced by the measured reason): undos 3, 4 and 6. No rendered bit moved; no
iOS count moved; no ceiling row moved.

| undo | site | verdict | iOS count (crate) | codegen, base -> natural shape | p50 |
|---|---|---|---|---|---|
| 2 (#1220) | `route_mix_ramp_block` index | **kept** | graph 0 -> 0 | ramp loop: x86-64-v3 (`f32x8`) 25 -> 22, AArch64 iOS (`f32x4`) 24 -> 21, `simd128` 62 -> 57 (function 291 -> 290); worklet kernel roster 21 -> 22 vector, 0 scalar | table A |
| 1 (#1407) | filter ramp leading word (two commits: `ca57f942c` needs `45cf6506b` for `cargo doc -D warnings`; revert or bisect them together) | **kept** | builtins 5 -> 5 | filter-ramp frame loops: x86-64-v3 `f32x8` dual 292/283/295 -> 287/278/290, mono 284/273 -> 278/267; `f32x4` dual 293/282/294 -> 289/279/291, mono 283/272 -> 279/267; `simd128` dual 775/709/827/757 -> 756/694/808/742, mono 447/496 -> 442/491. Functions: `simd128` `InputStage::process` 15,986 -> 15,876, `process_mono` 8,713 -> 8,685; iOS 6,977 -> 6,930, 4,067 -> 4,027; x86 `f32x8` `process_mono` 4,665 -> 4,678 (+13 outside the loops; named) | table B |
| 5 (#1409 D5) | `soft_clip_block` two copies | **kept** | soft-clip 1 -> 1 | frame loop: x86-64-v3 `f32x8` 201 -> 187 ramping / 171 settled; `simd128` 441 -> 421 / 374. `Channel::process` grows by the second copy (named, the cost of the two bodies): `simd128` 1,548 -> 2,034, `f32` 1,056 -> 1,667; x86 `f32x8` 805 -> 960. Scalar `Channel<f32>::process` (the per-node instance in the shipped worklet, when a soft clip insert does not bank; added by the batch follow-up from the verifier's measurement): ramping frame loop `simd128` 509 -> **512 (+3, named)**, settled copy 448; x86-64-v3 one latch path of the ramping loop 246 -> **247 (+1)**, settled copy 194 / 191 | table C |
| 3 (#1089) | EQ `lanes_mask` | **reverted** | parametric-eq 0 -> 0 | `recover_failed_lanes` (where it inlines): `simd128` 351 -> 360, AArch64 iOS 95 -> 104, x86-64-v3 `f32x4` 89 -> 77, `f32x8` 147 -> 126. Longer on both shipped targets | not measured, off the frame loop (D7's failing path) |
| 4 (#1091) | limiter `clear_runtime` `#[inline(never)]` | **reverted** | true-peak-limiter 6 -> **9** (D4 refuses) | inlined into `ChannelState::new`: x86-64-v3 469 -> 570, `simd128` 948 -> 1,316; the out-of-line copy stays for the reset callers | not measured, off the frame loop |
| 6 (A1, #1328 follow-up) | `silence_skip_settle` outlining | **reverted** | builtins 5 -> 5, parametric-eq 0 -> 0 | inlined at every call site: `simd128` `InputStage::process` 15,876 -> 20,902, `process_mono` 8,685 -> 11,248, EQ process bodies +230 to +596; x86-64-v3 `InputStage::process` `f32x4` 7,805 -> 9,615, `f32x8` 7,357 -> 9,005, `f32` 8,168 -> 11,169; iOS `f32x4` 6,930 -> 8,780. A duplicated body, not a simpler one | table D: flat |

Counting: x86-64-v3 and AArch64 from `cargo rustc --release -p <crate> --lib --crate-type rlib
--emit asm` (fat LTO, one CGU; the iOS one is `check-cross-targets.sh`'s command), instructions
per function, and per loop as the span from a loop label to the backward branch to it (so an outer
loop includes its inner ones); `simd128` from `wasm-objdump -d` of the worklet's named twin, every
instruction line counted, loops from `loop` to its `end`. TurboFan: the V8 spill gate's held and
reported rows are identical to base after every undo (every row clean; no site is in a held row).

**Gate 1, no bit moves.** Five scratch differentials (examples copied into
`crates/effect-compiler/examples/` for the run and removed again; never committed), run on each
undo's base and head:
- route ramp (`route_mix_ramp_block` at `f32`, `Simd4`, `Simd8`; 60,000 scenarios: random ramps,
  lengths near `2^22`, positions before, inside and past the ramp, hostile planes);
- `silence_skip_block` (#1328 follow-up's harness, 4,000 seeds x three widths);
- `bitid2` (#1451's 908 scenarios; the head copy from undo 1 on drops the removed argument only);
- all effects (#1451 verifier's harness, 6,630 scenarios), extended here with both reset kinds and,
  from undo 5 on, automation points on the scalar and bank paths (without them no soft clip block
  ramped);
- new owner-level filter-ramp differential (900 scenarios: `BuiltinInputBank` at every width, dual
  and collapsed, and the scalar `InputBuiltins`; staggered HPF/LPF retargets, disables, trim ramps,
  lane export/import mid-ramp, blocks of 1 to 128 frames).

Every one identical at every kept undo (and at the undo-6 build). Sensitivity: route mutant
(advance `W^2/4`) moves 23,823 rows (`f32` and `Simd8`; `Simd4` unchanged, as that mutant leaves
four lanes alone); filter floor `61` moves 861 of 900 filter rows (0 of 908 `bitid2` rows: that
harness never ramps a filter); soft clip ramping blocks sent to the settled copy move 209 of 390
soft-clip rows. Undo 5's bit argument: the settled copy holds the three words where the old arm
added a zero step; `x + (+-0.0)` is `x` for every word a settled lane can hold, because no ramp
word is ever `-0.0` (refused at preparation and restore, normalized at runtime points, never
produced by `ramp_toward` or the snap) or non-finite. Pinned artifacts: gate 5.

**Gate 2.** `check-cross-targets.sh` passes after each kept undo; counts in the table (builtins 5,
host-core 4, soft-clip 1, true-peak-limiter 6 at head, as at base). **Gate 3.**
`run-wasm-gates.sh` exits 0 after each kept undo; the spill gate's rows are those of base.

**Gate 4, p50** (`web-mixing-automation-benchmark.mjs run`, controls
`scratchpad/bench-controls.json`, one warmup and two measured rounds each, interleaved base,
head, base 1, head 1, head 2, base 2; `taskset -c 31`, `node --no-liftoff`; one invocation per
undo, not retried; every output digest equal between base and head). Modules (SHA-256 prefixes):
base `e25b045d`, after undo 2 `d13e0003`, after undo 1 `b2cfb8a7`, after undo 5 `1ff05d10`,
undo-6 build `a5d0b497`.

**Finding for root: no browser document reaches sites 2, 1 or 5.** None of the four fixtures
holds a soft clip; the controls move only EQ gain, compressor threshold and limiter ceiling, so no
input filter retargets and no route coefficient ramps (the sends document's routes are live but
static; `route_mix_ramp_block` runs with zero ramping frames). For these three undos the timing
measures the module's layout and the host, not the site. The host was shared: another agent's
21-core test run took load to 10 during table A. The differences swing both ways by more than 2 %
in tables A to C; table D (the one site the documents reach) is flat. I kept undos 2, 1 and 5 on
codegen (every changed loop shorter on every target but one: the scalar `Channel<f32>::process`
ramping loop of undo 5 grows by 3 on `simd128` and by 1 on one x86-64-v3 latch path, beside a
settled copy 12 % shorter; corrected by the batch follow-up) and on this finding, and did not
re-run.
Each is its own commit if root reads D3 otherwise.

Table A, undo 2 (load 10.10 -> 8.37):

| p50 | base r1 / r2 (ns) | head r1 / r2 (ns) | head vs base |
|---|---|---|---|
| mono console, quiet | 150,666 / 164,463 | 158,081 / 150,927 | +4.9 % / -8.2 % |
| mono console, restated | 154,525 / 167,899 | 161,999 / 154,764 | +4.8 % / -7.8 % |
| mono console, automated | 158,471 / 171,597 | 165,074 / 158,652 | +4.2 % / -7.5 % |
| sixty-four-track console | 238,785 / 237,152 | 247,050 / 237,963 | +3.5 % / +0.3 % |
| app shape | 152,531 / 152,380 | 156,168 / 152,631 | +2.4 % / +0.2 % |
| bus-and-send console | 344,316 / 342,943 | 352,431 / 342,121 | +2.4 % / -0.2 % |

Table B, undo 1 (load 4.83 -> 3.73):

| p50 | base r1 / r2 (ns) | head r1 / r2 (ns) | head vs base |
|---|---|---|---|
| mono console, quiet | 151,068 / 150,697 | 151,288 / 150,837 | +0.1 % / +0.1 % |
| mono console, restated | 155,165 / 154,474 | 154,323 / 154,594 | -0.5 % / +0.1 % |
| mono console, automated | 160,255 / 158,832 | 158,341 / 158,421 | -1.2 % / -0.3 % |
| sixty-four-track console | 239,486 / 267,149 | 245,277 / 240,628 | +2.4 % / -9.9 % |
| app shape | 155,426 / 187,227 | 158,823 / 152,420 | +2.2 % / -18.6 % |
| bus-and-send console | 343,905 / 382,749 | 347,862 / 344,436 | +1.2 % / -10.0 % |

Table C, undo 5 (load 2.56 -> 2.03):

| p50 | base r1 / r2 (ns) | head r1 / r2 (ns) | head vs base |
|---|---|---|---|
| mono console, quiet | 150,798 / 150,587 | 151,358 / 154,174 | +0.4 % / +2.4 % |
| mono console, restated | 155,086 / 154,505 | 154,966 / 158,502 | -0.1 % / +2.6 % |
| mono console, automated | 160,306 / 158,813 | 159,464 / 162,810 | -0.5 % / +2.5 % |
| sixty-four-track console | 244,737 / 239,847 | 246,721 / 242,733 | +0.8 % / +1.2 % |
| app shape | 155,286 / 153,272 | 156,158 / 159,374 | +0.6 % / +4.0 % |
| bus-and-send console | 348,956 / 346,300 | 351,631 / 344,217 | +0.8 % / -0.6 % |

Table D, undo 6 (inlined scan against the outlined one; load 2.70 -> 2.54):

| p50 | base r1 / r2 (ns) | head r1 / r2 (ns) | head vs base |
|---|---|---|---|
| mono console, quiet | 150,868 / 150,808 | 151,127 / 151,608 | +0.2 % / +0.5 % |
| mono console, restated | 154,965 / 154,895 | 155,416 / 155,667 | +0.3 % / +0.5 % |
| mono console, automated | 158,902 / 158,883 | 159,674 / 159,554 | +0.5 % / +0.4 % |
| sixty-four-track console | 239,606 / 238,635 | 241,931 / 240,288 | +1.0 % / +0.7 % |
| app shape | 154,334 / 156,288 | 153,663 / 154,344 | -0.4 % / -1.2 % |
| bus-and-send console | 345,278 / 343,715 | 343,254 / 344,988 | -0.6 % / +0.4 % |

**Gate 5** (at `517b339e2`): every command in gate 5 passes except `check-lane-policy.sh`: the effect-crate
`cargo test` (here the CI `test-debug-b` superset plus `graph`; G5, `BUILTINS_DIGESTS`, `E9_DIGESTS`,
multiband `DIGESTS`, the soft clip and limiter corpora), the release `lane`/`math`/`wasm-gates`
tests, `test-debug-a`, `conformance_fixtures --check`, `check-builtins-fixtures.sh`,
`check-builtins-policy.sh`, `check-graph-determinism.sh`, `check-workspace-policy.sh`,
`check-realtime-policy.sh`, the known-defect self-test, clippy, `cargo doc -D warnings`, fmt,
`check-cross-targets.sh`, `run-wasm-gates.sh` and the worklet chain (build with the named twin,
check, expected resources, V8 spill gate, `test-web-audioworklet.sh`). `check-lane-policy.sh`
fails on `crates/builtins/tests/tail_contract.rs:161` (`mul_add` outside `crates/lane`), a line
#1329 attempt 3 added at `6a2c5216a`, this slice's base; this slice does not touch that file. A
final run of the five differentials at head equals base (route, skip, `bitid2`), the base of
undo 1 (filter) and the base of undo 5 (all effects with automation).

**Test value.** No test added. Two tests are edited only where they named the removed argument
(`filter_ramp_line.rs` drops its leading-word builder; `input_chain_arming.rs` drops the argument),
and `filter_liveness.rs`'s gate-8 doc names the new source of the leading window. Mutation: the
filter floor at `61` turns `filter_ramp_line` and `filter_liveness` gate 8 red; the soft clip
dispatch swap turns three `soft-clip/tests/ramp_law.rs` tests red.

**For root.**
1. **Path deviation:** `crates/lane/tests/input_chain_arming.rs` calls the two filter-ramp bodies
   and had to lose the removed argument; it is not in Authorized paths (only `filter_ramp_line.rs`
   is). Also the `INPUT_FILTER_LEADING_UPDATES` import line in `crates/builtins/src/lib.rs` (beside
   the named builder and call sites), and undo 5's module doc step 1 in
   `crates/soft-clip/src/kernel.rs` (it described the per-frame branch).
2. **Gate 4 premise:** no browser document reaches sites 1, 2 or 5 (above). Kept on codegen.
3. **Stale prose after undo 2 (outside the authorized paths):** "21 vector" for the route kernel in
   `scripts/check-web-audioworklet-callgraph.py:242-245`, `scripts/check-web-audioworklet.sh:463-465`
   and the `route_mix_settled_tail` doc in `crates/lane/src/kernels.rs`; the roster now reads 22
   vector, 0 scalar (budget 8; the gate passes).
4. **Pre-existing red gate:** `check-lane-policy.sh` fails at this slice's base on
   `crates/builtins/tests/tail_contract.rs:161` (#1329 attempt 3's `mul_add`); #1329 owns it.

## Follow-up record

### Batch follow-up part B (2026-10-06, stream G; attempt 1 verdict and Amendment 2)

- **Root rulings recorded** as Amendment 2 above (R1 codegen keep criterion and why p50 is not
  decisive for sites no document reaches; R2 the four ratified path deviations; R3 the prose fix).
- **MINOR 1.** The undo-5 row names the scalar `Channel<f32>::process` loops (`simd128` ramping
  509 -> 512, settled 448; x86-64-v3 latch 246 -> 247, settled 194 / 191, the verifier's
  measurement), and the record's "every changed loop shorter on every target" sentence is
  corrected in place. The +3 is 0.6 % of a loop whose settled sibling is 12 % shorter; undo 5 stays
  kept under R1.
- **NIT 1 / R3: "21 vector" -> 22, four places.** `scripts/check-web-audioworklet-callgraph.py`
  (`SCALAR_SLACK`'s comment: 22 vector, about 88 scalar when de-vectorised; the route roster row's
  comment and its budget expression `max(0.10 x 22, 8) = 8`), `scripts/check-web-audioworklet.sh`
  (the `--kernel-min` comment) and the `route_mix_settled_tail` doc. Its "18 scalar" figure was
  **re-measured**: a worklet built with `route_mix_settled_tail` marked `#[inline(always)]` (scratch,
  reverted) reads `route-mix-ramp f32x4: vector=22 scalar=18`, a roster FAIL against its budget of
  8, so the outlining's reason stands; the head roster reads `vector=22 scalar=0`.
- **NIT 2.** The `filter_liveness.rs:934` header is in R2.
- **NIT 3.** The undo-1 row notes that `ca57f942c` needs `45cf6506b`.
- **NIT 4: `FILTER_LEADING_FLOOR` tied to the ramp length.** Undo 1's equivalence rests on every
  filter countdown the bodies read being at most 64, which no code asserted, and the lane floor
  spelled the ramp length as a literal `64`. Now `lane::kernels::builtins::INPUT_FILTER_RAMP_UPDATES`
  (64) names it and `FILTER_LEADING_FLOOR` is
  `INPUT_FILTER_RAMP_UPDATES - INPUT_FILTER_LEADING_UPDATES`; `crates/builtins/src/lib.rs` has
  `const _: () = assert!(INPUT_FILTER_RAMP_SAMPLES == INPUT_FILTER_RAMP_UPDATES)` (compile time)
  and `load_filter_countdown` a `debug_assert!` that every countdown is at most
  `INPUT_FILTER_RAMP_SAMPLES` (compiled out of release builds: no render cost). No bit moves (the
  constant's value is unchanged). Mutation runs, each alone, then restored:
  - `INPUT_FILTER_RAMP_SAMPLES` 64 -> 65 -> `cargo build -p builtins` **fails** (E0080, "the
    builtins' filter ramp length is the lane filter-ramp bodies' ramp length");
  - a retarget writes `INPUT_FILTER_RAMP_SAMPLES + 1` -> debug `--test filter_liveness`: **10 of 14
    tests panic** at the new assertion ("a filter countdown exceeds the ramp length").
- Gates: listed with the stream G batch follow-ups (part B).
