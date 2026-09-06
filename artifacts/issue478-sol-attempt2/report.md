# Issue #478 Sol attempt 2 evidence report

Candidate source is clean pushed `071ac394dabb19cd80b0ffca9d04e314e5fed757`.
The coherent implementation checkpoint was `d5a952116bf7bde487a9c17a324b848fa5c7d14f`;
`071ac394` adds only the bounded `checked_div` Clippy correction in the allocation fixture.

## Source identity

- `crates/rack/src/lib.rs`: SHA-256
  `0f666adb7696a401b2abe098faa1d78c9d7768297406a2e8de90987ad772e10e`, Git blob
  `804a7976a28122ae28cd864968dd7951ccd54549`.
- `crates/builtins-compiler/tests/allocation_tracker.rs`: SHA-256
  `16fe559f89ebcf025345957967a0cdfba407c55836c0ddc825a4df412c6643c1`, Git blob
  `bed3c091a0c4bc1f53c5008d8006ba1c168880b9`.
- `crates/graph/src/runtime.rs`: SHA-256
  `dbba576b96297ab453aee5c317fafa3048ccff01f3c1b4fb5bb89fe3f0119605`, Git blob
  `e2bb02e44addfc7aa5cca37cb200551cd2474b03`.

Every raw command record includes argv, cwd, HEAD, worktree status, environment, and all three
source hashes. Its sibling `.stdout`, `.stderr`, and `.status` files contain the unedited result.

## Implemented proof shape

- The shape gate covers full and partial W4/W8 chains and S=0/1/3/9 with literal expected packed
  masks, caller mutation before transfer, spare input capacity, identity rendering, out-of-range
  decline, collapse-prefix preservation, and inactive-stage render/drop lifetime.
- The dispatch gate uses a fixed-capacity allocation-free event/state record and a compact old
  `mask.iter().any()` reference. It compares ordered begin/dual/mono events, nonzero sample/frame
  values, queue state, arithmetic state, PCM bits, errors at begin/prefix/seam processing, inactive
  leading/middle/trailing stages, and destruction after rendering.
- The mechanism gate executes an ordinary block followed by an actually armed collapsed block. Its
  event and PCM assertions precede the work assertion and prove begin, ordinary, prefix-mono, and
  seam-dual dispatch. Prepared source records eight predicate calls and zero lane inspections.
- The existing allocator observes W4/W8 and S=0/1/3/9. It attributes the stage vector, public slot
  vector, every old mask, private destination, chain mask, and scratch planes; asserts every request
  and release; distinguishes incoming public capacity from private retained count; and uses the
  frozen `NF + NB + 2NP + 2NW <= C` inequality. S=0 is asserted only as direct-caller behavior.
  Repeated actually collapsed and forced-fallback renders make the installed allocation/free
  counter live and remain `(0, 0)`.

## Layout evidence

Root recovered the immutable preimplementation `9f6007da` baseline in an isolated build and ran
the candidate under the identical pinned compiler, target and AVX2/FMA flags. Baseline/candidate:
public `BankSlot` 32 bytes align 8, stage trait-object box 16 bytes align 8, `BankChain` 200 bytes
align 8, and graph `RuntimeUnit` 248 bytes align 8. Candidate private `PreparedSlot` is 24 bytes
align 8. `BankChain` and `RuntimeUnit` are unchanged. Root retains the raw `before.*`, `after.*`,
and `layout-comparison.json` under `/tmp/issue478-root-evidence`. This is explicitly retrospective
baseline evidence, not a contemporaneous prechange capture.

## Corrected three-site control

Exactly one corrected control replaced the three production `has_active_lanes()` calls with
`has_active_lanes_scan(self.lanes)`. The helper counts the same one predicate call per site and
performs actual lane-order, width-bounded, short-circuit bit reads. The exact diff is preserved in
`corrected-control-diff.stdout`. The frozen mechanism test completed its semantic assertions and
then failed only at the unchanged excess-work assertion with `left: 8`, `right: 0`, status 101.
Restored source passed the same test, 1/1, status 0. No other control was run; the earlier invalid
attempt-1 control remains preserved in its immutable attempt-1 evidence.

## Gate results

- Frozen exact debug gates: shape 1/1, trace/errors 1/1, mechanism 1/1, physical 1/1.
- Frozen exact release gates: shape 1/1, trace/errors 1/1, mechanism 1/1. After the Clippy-only
  correction, physical was rerun debug 1/1 and release 1/1.
- Rack affected suites, debug and release: library 30/30, `console_bank` 10/10,
  `mono_reengage` 4/4.
- Allocation tracker after the correction: debug 7/7 and release 7/7.
- Graph direct-bank allocation: debug 1/1 and release 1/1.
- C API resource lifecycle: debug 4/4 and release 4/4.
- Frozen graph-compiler reservation exact test: 1/1; full debug library: 64/64.
- Strict rack all-target/all-feature Clippy passed. Integrated builtins/graph test-support Clippy
  first failed solely on `manual_checked_ops`; that raw status 101 is preserved. The bounded
  `checked_div` correction passed its focused physical gate and the same strict integrated Clippy.
- Isolated `builtins-compiler --features test-support` check passed.
- Rack, builtins, graph, realtime, lane, and workspace policy checks and their gate self-tests all
  passed through explicit `bash` invocations.
- `cargo fmt --all -- --check` and `git diff --check` passed on clean `071ac394`.

No timing was run. Broader immutable workspace, supported-target, ABI, artifact/browser, actual-PR,
CI, GitHub evidence publication, and consolidated Astra verdict remain root-owned.
