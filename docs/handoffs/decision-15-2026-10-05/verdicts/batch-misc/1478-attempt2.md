FAIL

# #1478 attempt 2 -- adversarial verdict

Commit `e403684d9` (parent `e0804b9b2`), worktree `/home/bl/misofm/wt-d15-misc`. Reviewed on an
exported tree (`git archive e403684d9`), never in the worktree. Cumulative section reviewed:
`git diff a059cdd03 e403684d9 -- docs/REALTIME_DEPENDENCY_POLICY.md` (commits `859fb98a8` and
`e403684d9`). Attempt 2 touches only the two authorized paths: the doc's "Unsafe-code ownership"
section (hunks at lines 78-256 only) and the spec's attempt record. `origin/main` is still
`a059cdd03`: its `scripts/check-realtime-policy.sh` is the same as the tree's, and
`tools/realtime-policy` does not exist. So the awk gate's allowlist is the right authority.

Attempt-1 findings: m1, m2, n1, n2, n3 and n4 are fixed. M2's false "the one allowlisted file"
sentence is gone. M1 is fixed only in part, and the new "Render-path reachability" subsection that
fixes M2 contains false reachability claims. Gate 2 fails on three entries (M1-M3 below). The
attempt budget is two, so this attempt is the last one.

## MAJOR

**M1. The web `ffi.rs` reachability entry says that three render-locked exports reach
`read_live_record`. None of them does.**
`docs/REALTIME_DEPENDENCY_POLICY.md:220-223`. `spectrum_read` (`hosts/host-web/src/ffi.rs:2979`),
`spectrum_stream_read` (`:3103`) and `track_response_capture` (`:3472`) reach only
`copy_live_record`. They do this through `write_spectrum_window` (`:1266`), `spectrum_failure`
(`:1071`), `run_live_response_capture` (`:1764`), `live_response_failure` (`:1536`) and
`LiveResponseCaptureSink` (`:1594`, `:1615`). Only these functions call `read_live_record` (non-test):
- `imported_stream_configuration` (`:1173`), called by `spectrum_stream_analysis_configure` (`:3274`);
- `run_spectrum_analysis` (`:1317`), called by `spectrum_analysis` (`:2746`) and
  `spectrum_stream_analysis` (`:3256`);
- `parse_live_sections` and `parse_live_snapshot` (`:1825`, `:1850`, `:1895`), called by
  `run_live_response_analysis`, which `track_response_analysis` (`:3493`) calls.

None of these exports is render-locked, and the worklet calls none of them. They are the
analysis-worker exports: "A browser analysis worker has no render-host handle", `:3268-3270`. The
attempt record repeats the false chain ("through `write_spectrum_window`, `run_spectrum_analysis`,
`run_live_response_capture` and `run_live_response_analysis`"). The fix: name `copy_live_record`
only, and say that `read_live_record` and `response_header_bytes` are reached only from the
worker-side analysis and query exports.

**M2. The browser entries describe the render-locked set as if it were the render thread. Unsafe
code in `source_submit` and `source_seek` runs inside `process()`, outside any render-locked
window.**
The SDK ships `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`. It is staged into
`dist/assets` (`sdk/codegen/stage-package.mjs:66-67`, `sdk/src/assets.ts:34`) and loaded by
`prepareEngineFeed`. The file wraps the engine processor. At the top of `process()` (`:306-328`), for
every block, it calls `miso_engine_web_v1_source_submit` (`:386`) and `miso_engine_web_v1_source_seek`
(`:445`), and only then calls `super.process()`, which renders. Without the feed, the engine
worklet's own port handlers call these same exports (`receiveSource` `:1525`, `receiveSeek` `:1746`)
on the AudioWorklet thread. `hosts/host-web/src/lib.rs:3180` says: "This web host owns both producer
and render plan on one exclusive thread." Neither export is render-locked: `ffi.rs:3846`, `:3922`,
and the set in the header at `:14-23`. Both carry unsafe: three `slice::from_raw_parts` at
`:3891`, `:3898`, `:3901`, and one at `:3942`. As a result:
- `:220-223` (web `ffi.rs`, "yes") does not name the file's most frequent render-thread unsafe code,
  which runs every block on the SDK feed path. The attempt record leaves it out because it is
  "not render-locked". That reason mixes up the render-locked set and the render thread.
- `:224-226` (`render_lock.rs`): "they run on the render thread only if render calls the allocator
  -- which is exactly what they count" is false. An allocator call from `source_submit` or
  `source_seek` inside `process()` (or from boot, `dispose` or the single-mode control work that
  D15-10's recorded resolution puts in the worklet's message handler) goes through
  `RenderLockedAllocator` on that thread, and nothing counts it.
- `:161-162` (the D2 paragraph, kept from attempt 1, copied from `render_lock.rs:6-7`): "every export
  the worklet calls on its render thread runs inside `render_locked`" is false for the same two
  exports. Attempt 1's verdict missed this.

The fix: say which meaning of render thread the subsection uses, and name `source_submit` and
`source_seek` as unsafe code that runs on the AudioWorklet render thread, inside `process()` on the
SDK feed path and outside the render-locked count. Open item for root (out of slice): the D15-10
runtime proof does not cover these two exports, and `render_lock.rs:6-7` and the header of
`ffi.rs` make the same overclaim.

**M3. The x86 MXCSR path is still misdescribed (softfma and fpenv entries).**
- `:99`: `fpenv.rs` "is three register accesses ... with no memory operand". This is false on
  `x86_64`. The control word is reached only through `_mm_getcsr`/`_mm_setcsr`, which lower to
  `STMXCSR m32`/`LDMXCSR m32` through a stack slot (`core_arch/src/x86/sse.rs:1513-1519`,
  `:1662-1664`). x86 has no register form of these instructions. The sentence is true only for
  AArch64 `mrs`/`msr`. Attempt 2 rewrote this sentence. The `fpenv.rs` header (`:73-75`) does not
  make this claim.
- `:84-85`: the entry gives the `write_mxcsr` `SAFETY` premise ("a word previously read, with at
  most the FTZ and DAZ bits changed", `softfma.rs:102-104`, which goes on: "so no rounding mode or
  exception mask is disturbed") as the justification. The next sentence names `fpenv.rs` as a user.
  But `fpenv.rs` writes `CANONICAL_MXCSR` = `0x1F80` at every native x86 render entry
  (`fpenv.rs:338`), and the purpose of that write is to clear a caller's directed rounding and
  unmasked exceptions (`fpenv.rs:40-45`). The entry flags the stale "never called from a render
  path" doc comment, but it does not flag this stale premise. The sound reason is that the value is
  always either a word read from this thread's MXCSR or the architectural default, and neither sets
  a reserved bit.
- `:86-89`: "They have two users: gate G6 ... and `fpenv.rs`". Since #1017 (`92b1def5e`), G6
  (`crates/lane/tests/g6_ftz_inert.rs:10,105`) reaches the control word only through
  `lane::fpenv`. The direct callers are `fpenv.rs`, `crates/capi/src/runtime/tests.rs:3094` and
  `crates/lane/tests/fp_env.rs`. Attempt 1's verdict prescribed this wording, but it is still
  inaccurate.

The fix: state that the x86 accesses use a stack memory operand, give the real `_mm_setcsr` safety
ground, and say that `fpenv.rs` is the helpers' only non-test caller. Add `softfma.rs:102-104` to
open item 2.

## MINOR

**m1. `:234-236`, "the tools now at ... `tools/bench/src/protocol.rs`".** No tool is now at that
path: #1075 (`9b5c3c346`) deleted it, and the same paragraph says so at `:244-246`. Attempt 2
introduced the word "now". It is graded MINOR rather than MAJOR because it is retirement history,
not an allowlist justification, a reachability claim or an issue/commit citation, and the paragraph
itself states the truth. Root may regrade it under the "any false statement" rule. Fix: "the tools
that became ...".

## NIT

- n1. `:227-229`: `crates/capi/tests/resource_lifecycle.rs` also calls
  `miso_engine_v1_render_f32_planar` (9 sites), but only `plan_swap_race.rs` and the audit
  `capi.rs` are named.
- n2. `:169-171` and the `allocation_tracker.rs` entry describe only the `GlobalAlloc` impl. The file
  also has three direct `std::alloc` unsafe calls in its realloc liveness test (`:801-808`).
- n3. `:69` (text from before this slice): "Single-buffer and stereo access use safe slices". This
  is true of `read_stereo`/`write_stereo`, but `write_read_stereo` is a raw-slice stereo borrow
  (`disjoint.rs:317-345`). The reachability bullet at `:213-214` is accurate.

## Verified true

- **Gate 1.** The 19 paths of the allowlist (`scripts/check-realtime-policy.sh:31`) equal the
  section's backticked `/`-paths before "Retired exceptions". The one extra path is the cited test
  `crates/lane/tests/fp_env.rs`. Evidence: `/tmp/claude-1002/v1478/{allow,doc}.txt`.
- **Introducing issues and commits, all re-checked against `git log --diff-filter=A --follow` and
  the spec index of that time.** spsc 003 (`68ff477e4`); disjoint #100 (`ab0aea713`); softfma 083
  (`c018e2547`); fpenv #146 (`27e8299c6`); capi `ffi.rs` 022 (`947d7996f`); web `ffi.rs` 024
  (`ef925e6cb`); resource_lifecycle 119 (`1a3dde27e`); plan_swap_race #1042/#1273 (header,
  `1338b063c`); audit `capi.rs` 022 (header, `434551bc0`); render_lock #1333 (`4830bed93`);
  allocation_tracker 007 (`ff7546734`); allocation_budget #107 (`c94f659b1`); boot_transient_budget
  #240 (`3d91830b3`); bench-support #104 (`d9a66a952`); wasm-gate-guest #83 G5 (`3ef329b14`);
  soft-clip #91 (`651903b85`); transient-shaper #92 (`3a4bc6cbe`, after `9670016a8`); limiter #90
  (`4c0293fe5`); multiband 018/#94 (`e59859edc`, which also amends spec 018); wrapper removals
  `568ad4087`, `cb4898437`, `a6da0cade`, `39c4651b1`; host-web `CountingAllocator` `708036a76`;
  #163 phase 2 `477dc15ee` (2026-08-26). "Older than #146" is true: the helper doc comments date
  from `c018e2547` (2026-08-23), and #146 is from 2026-08-25.
- **Retired exceptions.** `68ff477e4`'s exclusion list has five files, and its message says "four".
  The fifth is `tools/miso-engine-effect-contract-bench/src/main.rs`, with its own
  `#[global_allocator]`. `d9a66a952` dropped the effect-contract-bench, realtime-audit and
  protocol-audit entries and added bench-support. #136 (`6349258fa`) folded the bench into
  `tools/bench/src/effect_contract.rs`.
- **Reachability.** These bullets are correct:
  - spsc: unsafe at `:381` and `:449`, inside `:295-459`. Render-side users are `plan_exchange`, the
    protocol queues, the effect control rings and the source rings.
  - disjoint: `write_read_many` (`graph/runtime.rs:488`), `write_stereo_many` (`:2020`, `:2029`) and
    `write_read_stereo` (`:3681`).
  - fpenv: the guard is pinned at `capi/src/ffi.rs:817` and `host-core/src/render_session.rs:121`,
    and it is a zero-sized type with no `Drop` on wasm.
  - softfma: on `x86_64`, through `fpenv.rs:141,148`.
  - capi render entry: `plan_kind`, `&*output`, `plan_state` and `plan_error_slot` before
    `state.render`.
  - The body of `miso_engine_web_v1_render` has no unsafe code, and `with_host_mut` has none.
- **Entries matching their files.** The capi `SAFETY` categories match `:51-563`. The web `ffi.rs`
  `CountingAllocator` is described correctly. Also correct: `plan_swap_race.rs` (bench_support
  allocator), `LifecycleAllocator`, audit `capi.rs` (only render is in scope), the three test
  allocators, bench-support (Abort by default, `Mode::Count`, the only `#[global_allocator]` under
  `tools/`) and wasm-gate-guest (no `unsafe {`/`fn`/`impl`, only `#[unsafe(no_mangle)]`, inert in
  the regex, binding in `check-bench-policy.sh:216-219`).
- **Other facts.** The workspace lints are in `Cargo.toml:90,91,102`. `crates/graph` has no unsafe
  code. The only `asm!` is in `fpenv.rs`. Mutation cases for a third lane file exist in both gates
  (`test-realtime-policy.sh:580`, `test-lane-policy.sh:301`).

## Test value

No test was added (D4), which is correct. No queue is touched, so the acked-batch question does not
apply.

## Gates run (exported tree; no build)

- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok` (exit 0)
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)` (exit 0)
- `bash scripts/check-dsp-research.sh`: `dsp research corpus: ok` (exit 0)
- `bash scripts/check-builtins-listening.sh`: `issue-007 listening preregistrations: ok (human evidence pending)` (exit 0)
- Gate 1 set comparison: equal, 19/19, plus the cited test.

## Open items for root

1. The D15-10 coverage gap (M2). The SDK feed calls `source_submit` and `source_seek` inside
   `process()`, outside `render_locked`, so the render-allocation count cannot see an allocation
   there. `render_lock.rs:6-7` and the header of `ffi.rs` (`:14-23`) overclaim. This needs an
   issue: either lock these exports or state the exclusion.
2. `softfma.rs:102-104` (`SAFETY`), `:1`, `:26-27`, `:82` and `:95` are stale since #146 and #1017.
3. The successor for the stale allowlist entries, as in attempt 1.
