FAIL

# #1489 attempt 1 -- adversarial verdict

Commit `30cc36ef9` on `codex/d15-batch-misc2` (worktree `/home/bl/misofm/wt-d15-misc2`). Its real
parent is `b0256b89d` (#1492 attempt 1), not `453f3038a`: `40a1ead6f` (#1488 follow-ups) and
`b0256b89d` sit between them. The diff was reviewed against `b0256b89d`. The commit and the branch
head `fb303f836` were both exported with `git archive`, and nothing was built in the worktree.

Scope is clean. The commit touches 4 paths: the spec (attempt record only), `crates/lane/src/softfma.rs`
(only `//` lines change; `git diff -U0` shows no code token), `docs/REALTIME_DEPENDENCY_POLICY.md`
(hunks only inside "Unsafe-code ownership"), and `scripts/check-realtime-policy.sh` (old `:20-23` ->
new `:20-23`, only `#` lines, same line count). The gate-script edit is within the spec's Amendment
(`:19-23`, D-hdr), and STREAMS gives stream J `scripts/check-realtime-policy.sh` "until #1446 deletes
them" (`STREAMS.md:487`). Hot-file row 108 does not list the gate script because #1446 deletes it, and
D-hdr has a lapse clause for that order. So there is no conflict.

D-M1, D-M2, the D-M3 instruction forms, D-hdr and gate 1 are all true (see "Verified true"). Gate 2
fails because the restated `write_mxcsr` `SAFETY` premise contains a false clause, and the policy
quotes it word for word.

## MAJOR

**M1. The restated `SAFETY` premise says that fpenv "never installs an environment the caller did not
already run under". This is false for the entry write.**
`crates/lane/src/softfma.rs:105-108`, quoted in `docs/REALTIME_DEPENDENCY_POLICY.md:91-93`: "On the
render path `fpenv` writes only `CANONICAL_MXCSR` (0x1F80, the architectural default) on entry and the
word it read from this thread on exit, so it never installs an environment the caller did not already
run under."

On entry `CanonicalFpEnv::enter` (`fpenv.rs:336-338`) installs `0x1F80` whatever word the caller had.
Doing that to a caller with a different word is the purpose of the guard:
- `fpenv.rs:16-17` says "every DAW audio callback arrives with FTZ and DAZ already set";
- `fpenv.rs:43-45` says installing the whole word "removes a caller's directed rounding mode and any
  unmasked exception";
- the tests run exactly this case: `crates/lane/tests/fp_env.rs:73-97` (hostile word -> guard reads
  `0x1F80`) and `crates/capi/src/runtime/tests.rs` `the_c_render_entry_is_canonical_under_a_caller_that_set_ftz_and_daz`.

Only the exit write restores an environment the caller already ran under. The spec's D-M3 asked for the
real soundness ground. The true version is: the entry write installs only the architectural default,
which is the state Rust assumes, and the exit write gives back the caller's own word. Neither word sets
a reserved bit. This clause is the core of D-M3, and the policy quotes it, so it fails gate 2 ("Any
false statement fails the attempt"). The fix is one clause in both files.

## MINOR

**m1. `softfma.rs:97-98`: "so every caller restores the previous value before it returns" is false for
the only non-test caller.** The direct caller `fpenv::write_fp_control_word` (`fpenv.rs:147-149`, a
safe `pub fn`) writes and returns. Its caller `CanonicalFpEnv::enter` also returns with the canonical
word still installed, by design: the restore is in the guard's `Drop` (`:357-360`). The tests do
restore. A true version is: "every user restores the previous word: `fpenv` in its guard's `Drop`,
tests before they return". This is graded MINOR because the guard as a unit does restore. Under gate 2's
"any false statement" rule, root may regrade it.

**m2. `docs/REALTIME_DEPENDENCY_POLICY.md:292-294`: "their own unsafe is the caller side of the C
contract" is false for `resource_lifecycle.rs`.** That file's own unsafe also includes
`LifecycleAllocator` (`crates/capi/tests/resource_lifecycle.rs:55-88`), which runs on the same test
thread as its render calls. The section's own entry for the file (`:157-161`) names this allocator.
Fix: "apart from `resource_lifecycle.rs`'s counting allocator".

## NIT

- n1. `docs:289-291` (`render_lock.rs` reachability): "calls made during boot and by `dispose` are not
  counted, by design" is less precise than the mechanism. During boot the worklet calls render-locked
  exports:
  - the eleven accessors in `initialize`/`bindLiveControls`;
  - `spectrum_target_id_ptr`/`_capacity` in `stageSpectrumRequest` (worklet `:455-456`, before the boot
    export at `:330`).

  An allocator call inside any of these would be counted, because the counter is cumulative from
  instantiation. Boot is uncounted only because boot allocation happens outside windows
  (`ffi.rs:617-626`). This is true of today's calls, but the sentence names the wrong rule.
- n2. `softfma.rs:96-97` keeps "FTZ and DAZ are *observed*, never relied on (D7)" from the G6
  context. Next to the new "Called by `fpenv` at every native `x86_64` render entry", a reader may
  take it to mean that fpenv does not depend on FTZ/DAZ. In fact fpenv exists because the render
  depends on both being clear (`fpenv.rs:10-17`).
- n3. Line citations at the head (`fb303f836`). #1492's follow-up reflowed the `render_lock.rs` header
  and added one line to it:
  - `render_lock.rs:87-90` (`docs:178`) is now `:88-91`, so the citation is stale by one line;
  - `render_lock.rs:6-10` (`docs:213`) was exact on `30cc36ef9` (the sentence begins "This module" at
    the end of `:5`). At the head the passage is `:5-11` (set and exceptions `:5-9`, qualification
    `:9-11`), so `:6-10` still contains the set claim (`:6-8`) but is no longer exact.

  All other citations are still exact at the head:
  - the `ffi.rs` header reflow kept its line count, so `ffi.rs:14-39`, `:878`, `:894`, `:1805`,
    `:3929`, `:3936`, `:3939`, `:3982` and `:4881` are unchanged;
  - `run.mjs:266-270`;
  - the feed worklet's `:306-328`, `:386`, `:445` and `:539`;
  - every `fpenv.rs`, `softfma.rs`, `disjoint.rs`, `spsc.rs`, capi `ffi.rs:807` and `Cargo.toml`
    citation.

## Verified true

- **Gate 1.** The allowlist at `scripts/check-realtime-policy.sh:31` has 19 paths. They are equal to
  the section's paths before "Render-path reachability"; the only extras are the three cited tests
  (`crates/capi/src/runtime/tests.rs`, `crates/lane/tests/fp_env.rs`,
  `crates/lane/tests/g6_ftz_inert.rs`). Fifteen paths have their own entry, and the four stale paths
  are grouped. Those four files and `tools/wasm-gate-guest/src/lib.rs` have 0 gate-regex matches. The
  guest has only `#[unsafe(no_mangle)]` and `u32`-only exports, and `tools/audit/src/{realtime,protocol}.rs`
  have no `unsafe`.
- **D-M1** (a call-graph script over non-test `ffi.rs`, at the commit and at the head):
  - `copy_live_record` is called from `spectrum_failure`, `write_spectrum_window`,
    `run_spectrum_analysis`, `live_response_failure`, `LiveResponseCaptureSink::{append_section,
    write_owner}`, `run_live_response_capture` and `run_live_response_analysis`. So the render-locked
    `spectrum_read`, `spectrum_stream_read` and `track_response_capture` reach it only through the
    chain the doc names.
  - `read_live_record` is reached only from `spectrum_analysis`, `spectrum_stream_analysis`,
    `spectrum_stream_analysis_configure` and `track_response_analysis`.
  - `response_header_bytes` is reached only from `response_query`.
  - None of these exports is render-locked, and neither worklet calls them. No export that the
    worklet calls during boot reaches an unsafe site. "Every such site runs inside a render-locked
    window" is true.
- **D-M2, recounted at the head** (#1492's select wrap is already in `b0256b89d`, so the counts are
  the same at the commit and at the head):
  - 123 exports, 42 with a `render_locked(|| ...)` body.
  - The post-boot calls of the engine worklet (`receive*`, `readSpectrumStreamMetadata`,
    `postMeterFrame`, `process`; `initialize`/`bindLiveControls` are called only from the constructor)
    plus the feed's two calls give 33 exports, the same as the doc's list.
  - Set 1 minus set 2 = {`dispose`, `render_allocation_count`}.
  - Set 2 minus set 1 = the eleven named accessors, all called only in `initialize`/`bindLiveControls`.
  - `spectrum_request_*`/`spectrum_collection_*` are not wrapped and are called only from
    `stage*Request` (`:320-321`, before `boot` at `:330`).
  - `spectrum_selection_epoch` (`:3004`) is not wrapped and neither worklet calls it.
  - Neither worklet reaches an export through a stored or dynamic reference; the only non-call
    references are `:293-294`, inside `initialize`.
  - The feed's `:386`/`:445` are reached from `process()` `:306-328` (through `drainSharedRing` and
    `applySharedSeek`), and the prepare-seek branch `:539-540` reaches `prepareSharedSeeks` and then
    `applySharedSeek`.
  - The `source_submit`/`source_seek` unsafe slices are inside `render_locked`.
- **D-M3 instruction forms.**
  - `core_arch/src/x86/sse.rs` (nightly-2026-08-20 rust-src) has `_mm_getcsr` at `:1513-1519` =
    `stmxcsr(addr_of_mut!(result))` and `_mm_setcsr` at `:1662-1664` = `ldmxcsr(addr_of!(val))`. Its
    "immediate Undefined Behavior" paragraph (`:1542-1546`) names the masking flags, rounding mode and
    DAZ, which matches the SAFETY text.
  - The emitted x86_64 release assembly (`--emit asm`) has `read_mxcsr` as `movl $0,-4(%rsp);
    vstmxcsr -4(%rsp); movl -4(%rsp),%eax` and `write_mxcsr` as `movl %edi,-4(%rsp); vldmxcsr
    -4(%rsp)`.
  - In the shipped `libcapi.so` (fat LTO), `miso_engine_v1_render_f32_planar` calls both helpers out
    of line through the GOT, at `0x80897` and `0x808a4` (`mov $0x1f80,%edi`). The helpers are exactly
    those two stack-slot forms, so the parenthetical in the doc holds for the shipped render path.
  - AArch64: `mrs`/`msr` use `out(reg)`/`in(reg)` with `nostack` (`fpenv.rs:166-168`, `:181-183`).
    `cargo check -p lane --target aarch64-unknown-linux-gnu` is ok.
  - The only `asm!` in crates, hosts and tools is `fpenv.rs:167`, `:182` and `:327`.
- **The `write_mxcsr` callers, against the SAFETY text.**
  - The only non-test caller is `fpenv.rs:148`, through `enter` `:338` (`CANONICAL_MXCSR`) and
    `Drop` `:359` (the saved word). Its users are `capi/src/ffi.rs:817`,
    `host-core/src/render_session.rs:121` and `attest_fp_environment`.
  - Direct test callers are `lane/tests/fp_env.rs` and `capi/src/runtime/tests.rs` (`#[cfg(test)]`,
    `runtime/mod.rs:54-55`).
  - Indirect test callers, through `write_fp_control_word`, are `g6_ftz_inert.rs:115-117`,
    `host-core/tests/fp_environment.rs` and `wasm-gates/tests/g6_full_corpus_ftz.rs`.
  - Every word written is `0x1F80`, a read word, or a read word OR'd or masked with `0x003F`,
    `0x0020`, `0x0040`, `0x4000`, `0x6000` or `0x8000`. No word sets bits 16-31.
  - Tests restore through `Restore`/`WordGuard` guards or an explicit write-back. G6 reaches the word
    through `lane::fpenv` (`g6_ftz_inert.rs:10`, `:105`).
  - SSE is guaranteed by the compile guard at `lane/src/lib.rs:78-85`.
- **D-a.** `softfma.rs:1`, `:26-32` and `:83` are true. The dropped "forbids inline assembly" was false
  since #146.
- **D-hdr.** `fpenv.rs` reuses the softfma helpers on x86_64 (`:141`, `:148`). Its own unsafe sites
  are the AArch64 pair `:166`/`:181` and the barrier `:326`, which is under
  `cfg(any(x86_64, aarch64))` (`:320`).
- **Other entries.**
  - Workspace lints: `Cargo.toml:90`, `:91`, `:102`.
  - `lane-source.toml:28` (softfma and fpenv only), `check-builtins-policy.sh:23`, and the three-file
    `tools/` set in `check-bench-policy.sh:220-234`.
  - spsc `:381`/`:449` are inside `:295-459`.
  - disjoint: `:208`, `:303` and `:334` are inside `:106-347`; `read_stereo` is at `:145`,
    `write_stereo` at `:169` and `write_read_stereo` at `:317`. There is no `UnsafeCell` and no
    `unsafe impl`.
  - The capi render entry pins at `:817`, then the unsafe `plan_kind`, `&*output`, `plan_state` and
    `plan_error_slot` come before `render`.
  - Web `ffi.rs` unsafe sites: `:878`, `:894`, `:1805`, `:3929`, `:3936`, `:3939`, `:3982`. The
    `CountingAllocator` is at `:4881` (module) / `:4896` (`#[global_allocator]`), counts only
    successful operations while armed, and forwards to `System`.
  - `render_lock.rs` forwards all four operations unchanged and increments the counter with one
    `Relaxed` `fetch_add` only when locked. `render_locked` is used only in `ffi.rs` and its own tests.
  - `run.mjs:266-270` asserts `count === 0`.
  - `allocation_tracker.rs` has `std::alloc` calls at `:802`, `:805` and `:809`. `allocation_budget.rs`
    runs `1/256/4_096` tracks. `bench-support` has `Mode::Abort` (default) and `Mode::Count`.
  - The retired paragraph (`68ff477e4` lists five files and says "four"; `d9a66a952`; #1075; #1033)
    and the mutation cases (`test-realtime-policy.sh:547-559`, `:581`; `test-lane-policy.sh:302`).
  - `crates/graph` has no `unsafe`. The introducing issues are as the #1478 verdicts verified them.
- Attempt-record spot checks (`ffi.rs:3045`, `:1293`, `:1098`, `:3276`, `:3522`, `:1791`, `:1563`,
  `:1621`, `:1642`, `:3312`, `:2778`, `:3297`, `:3535`, `:2467`, `:3652`, `:4570`, `:4896`,
  `:3004`): all exact.

## Open items for root (outside this slice)

1. The record's open item 1 is a real soundness gap, not a nicety. `softfma::write_mxcsr` and
   `fpenv::write_fp_control_word` are safe `pub fn`s whose SAFETY argument is about what their callers
   pass. Any safe caller can pass a reserved bit (#GP) or a non-default mask, rounding or DAZ word (UB,
   according to `core::arch`). Making them `unsafe fn`, or keeping only the guard and a test-only path,
   needs its own issue.
2. Extend open item 3: the claim in `fpenv.rs:73-75` "no call at all: the bodies are `#[inline]`" is
   false on the shipped x86_64 cdylib. Each render entry makes two out-of-line GOT calls (`read_mxcsr`,
   `write_mxcsr`) and `Drop` makes a third. "Two register writes" is also false on x86_64 (stack-slot
   `LDMXCSR`). The policy section correctly no longer repeats either claim.

## Test value

No test is added (D-test), which is correct, so there is no mutation run. No queue is touched, so the
acked-batch question does not apply.

## Gates run (exported `30cc36ef9`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1489/target`)

- `cargo build --locked -p lane`: ok
- `cargo fmt --all -- --check`: exit 0
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`
- `bash scripts/check-lane-policy.sh`: `lane policy: ok`
- `bash scripts/check-dsp-research.sh`: `dsp research corpus: ok`
- `bash scripts/check-builtins-listening.sh`: `issue-007 listening preregistrations: ok (human evidence pending)`
- Also run: `bash scripts/test-realtime-policy.sh` ok; `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane --no-deps` ok;
  `cargo clippy --locked -p lane --all-targets -- -D warnings` ok;
  `cargo check --locked -p lane --target aarch64-unknown-linux-gnu` ok;
  `cargo rustc --locked --release -p lane --lib -- --emit asm`; `cargo build --locked --release -p capi` (for the disassembly).
- At the head `fb303f836`: `check-workspace-policy.sh`, `check-realtime-policy.sh` and `check-lane-policy.sh` all ok.

Evidence files: `/tmp/claude-1002/v1489/{allow.txt,doc.txt,exports_tree.txt,exports_head.txt,exports.py,callers.py}`.
