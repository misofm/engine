# Compare a no-ramp reposition at the old cell's last ramp completion off the fader grid

Filed by root from the #1716 fix D verification (2026-10-25).

## Problem
#1716 fix D compares a no-ramp `Reposition`'s new curve with the old curve at `span.start - 1`. That is exact when the block starts on the 64-sample fader grid, because the last grid ramp has just completed there. Off the grid, the stage's stored target is the old curve at the last grid ramp's completion `g + 63` (with `g + 63 >= span.start`), not at `span.start - 1`. So a table-only rebuild with no move, adopted at an off-grid block, still opens with a J-length adoption jump on a moving ride. This happens in production: the C ABI accepts any quantum (96, 100, 240, 480 ...), and seeks land on any frame. It is not a regression of fix D: the code before D jumped there too.

## Smallest slice
Compare at the old cell's last ramp completion (`ramp_done`, which `reposition` clears today) for a stage with no ramp, held or free. Keep a step of the new table at `span.start` jumping (B17-R13).

## Gates
- The builtins-compiler composed-lane row at an off-grid block (for example 100 frames after a seek) and a no-move table-only rebuild: bit-equal to the uninterrupted engine. Red on main.
- Class B (bits move only in that case); fix D's owner approval of 2026-10-25 covers the same rule ("compare at the same node time"). State that in the PR.

## Decision and implementation (attempt 1, 2026-10-27)

- Root approved this bounded brief; the owner requested Sol 6.1 high for implementation in this batch. Fresh independent Astra review is coordinated by root.
- Read the carried cell's recorded ramp completion before repositioning clears it. `CellState::ramp_completion` exposes that existing word without changing its representation; the common fader adoption rule reads the lone cell's state or the composed output's state, never one constituent cursor's completion.
- A held or free stage with no running ramp compares the successor curve at that recorded completion. With no recorded completion (fresh state or exact-set landing), retain fix D's previous-sample fallback. An unchanged no-ramp comparison restores the captured completion with the existing hold operation, retaining both its target time and grid hold across another no-op adoption before the next grid; intentional resets and jumps retain their existing semantics. Keep a successor-table step at `span.start` comparing and jumping at the block's entry (B17-R13), and leave running-stage-ramp adoption unchanged.
- This is Class B: bits may move only for a no-ramp reposition whose recorded completion differs from the old previous-sample comparison and changes the adoption decision. #1716 fix D's owner approval of 2026-10-25 covers the same rule, **compare at the same node time**; this corrects that rule off the grid rather than changing the product contract. The batch PR must state that authority.
- Render still reads fixed state and evaluates existing tables: no allocation, lock, syscall, DSP algorithm, parameter, wire or ABI change.

## Regression and test value

Fold the new cases into the existing carry regression, renamed `a_carried_automated_lane_renders_the_uninterrupted_bits` for its composed and lone rows, retaining its existing continued/repositioned, running-ramp and grid-aligned rows. One claim remains owned here: a table-only no-move adoption renders the uninterrupted lane's bits.

Test value: a no-ramp reposition that compares at `span.start - 1` instead of its carried target's completion now fails after the settled muted lane unmutes at 1,120 (the swap is at 1,100, its last grid target belongs to 1,151); an identical exact stage set on the predecessor and uninterrupted runs exercises the free/no-ramp branch, a second muted no-op adoption one frame later checks that a lone cell keeps its completion history (comparing remembered target bits as well as the later audible output), and a successor-table step at adoption protects B17-R13's jump instead of suppressing it with the earlier comparison.

The full post-adoption sample arrays include the audible unmute, so comparing muted zeros cannot satisfy this regression. The owning test is renamed and expanded; no additional test function or fixture framework is added.

## Evidence

Evidence directory: `/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/2096/` (outside the worktree). Every build/test/format command runs through the batch's `run-serial.sh` wrapper, which serializes CPU-heavy work, sets Cargo jobs to two and test threads to one, and uses the shared batch target directory.

- Baseline red: `cargo test -p builtins-compiler a_carried_composed_lane_renders_the_uninterrupted_bits` against unchanged production code, adding only the natural held/unmute row. Exit 101; bit equality fails for `continued 0b0, muted true, Simd8, 100-frame blocks`. Full output: `baseline-red.log`. This is the defect's revert reproducer, not a source/prose inspection or a new output digest.
- Focused green: `cargo test --locked -p automation -p builtins-compiler --features builtins-compiler/test-support` completed with exit 0: 102 tests passed, three existing tests ignored, no failures, and both crates' doc-tests passed. This includes `a_carried_automated_lane_renders_the_uninterrupted_bits` on both lone/composed layouts and both native bank widths, plus the existing allocation gates. Full output: `green.log`.
- Formatting: `cargo fmt --package automation --package builtins-compiler` completed with exit 0 (`fmt-final.log`). The final implementation is locally checkpointed; independent adversarial review and remote delivery remain root's responsibility.
- Exploratory completion-history mutation logs are retained as development evidence, but are **not** counted as a successful discriminator: those runs stopped on the subsequently corrected fixture assertions or the coincident moving-own/entry-step observation below. Fresh independent review owns the final narrow preservation mutation check; the baseline natural-held reproducer above is the completed red-on-revert evidence.
- The first attempt at a target-bit assertion also covered the 128-frame row while its unmute ramp was already running; that was outside the no-ramp premise and intentionally uses the existing entry-time adoption comparison. Restrict repeated-adoption/held-target assertions to the off-grid 100-frame no-ramp row; retain the running row's audible continuity check.
- A boundary-step fixture with a simultaneously moving own ride exposed different pre-existing composed arithmetic at the first post-swap sample (lane 1 left: bits 1049322181 versus uninterrupted 1049321647). The trace shows the lane still has 52 ramp samples, so #2096's no-ramp branch is not involved. Keep B17-R13's existing entry comparison and use a static own curve for this issue's boundary-step guard. The moving-ride observation and minimal reintroducer are preserved in `coincident-step-observation.md`, `coincident-step-reproducer.patch` and `debug-comparison.log`; root/fresh review must assess against baseline before deciding whether a stateless successor is needed. No product approval or PASS is inferred for the observed difference.

## Independent adversarial review — attempt 1 PASS (Astra, 2026-10-27)

Fresh review of implementation `e72b5c969979e41ba93a6c831774e9cffee93a85` and evidence head
`08fd9fa7277af1729506dab46762cfdd0904645e` finds the bounded no-ramp fix correct. Completion
is captured before reposition, comes from the composed output state, and is restored only
when the unchanged comparison keeps that target. Running ramps and B17-R13 entry jumps keep
their existing rules. No state layout, ABI, allocation, lock, syscall or new unbounded loop
is introduced. Class B authority remains #1716 fix D's compare-at-the-same-node-time ruling.

Test value: the folded `a_carried_automated_lane_renders_the_uninterrupted_bits` catches an
off-grid no-ramp comparison at the previous sample and loss of completion across repeated
adoption, both audible after unmute and absent from the old running-ramp/grid-aligned rows.
The original test is replaced, not duplicated.

Independent logs are in `/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/fresh-astra/`:

- `completion-preservation-mutation.log`: removing only the unchanged-adoption
  `hold_until(completion)` restoration is RED (exit 101), on the final audible bit comparison
  for `muted-ride`, lone lane, Simd8, 100-frame blocks. The exact mutation patch is preserved.
- `owner-green-rebuilt.log`: the owning regression is GREEN on integrated source (exit 0);
  the implementation's full automation/builtins suite remains 102 passed, three existing
  ignored, as recorded above. No duplicate full debug suite was run.
- A first integrated focused invocation reused the scratch baseline binary from the shared
  Cargo target and repeated its entry-step failure without compiling. Its log is preserved
  in `reachability-and-owner.log`; invalidating the two reviewed source mtimes forced the
  correct rebuild, without changing file contents. This is not a product failure.
- `clippy.log`: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
  passed. Workspace formatting/policy, realtime policy (283 regions/45 files), builtins policy
  and unfused seal passed (`static-gates.log`, `static-gates-corrected.log`). The first command
  named a nonexistent policy shell script; the corrected command runs `realtime-policy`.
  Its first run rejected a local dependency symlink; regular ignored dependency directories
  fixed that setup problem. No production workaround was made.

The coincident moving-own/VCA-entry-step observation is a distinct confirmed baseline defect,
now tracked by [#2158](https://github.com/misofm/engine/issues/2158). Baseline `7be08dd18`
production plus the narrowed entry-step fixture is RED (`coincident-step-base.log` and
`coincident-step-base.patch`): first post-swap block, lane 1 left, sample 0 is 1049322181
versus uninterrupted 1049321647. The author's retained trace has 52 ramp samples remaining,
so this is outside the no-ramp branch. The successor must preserve the required boundary
jump, not suppress it. This review does not approve that differing behavior.

All heavy checks used the shared serial wrapper; offline compilation was paused and resumed
while unrelated builds ran (`cpu-pause.log`). Full required target/AArch64/browser matrices
remain the batch PR qualification gate; remote evidence synchronization and closure are root's.
