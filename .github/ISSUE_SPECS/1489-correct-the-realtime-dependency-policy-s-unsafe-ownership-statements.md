# Correct the realtime dependency policy's unsafe-ownership statements

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-08 by root as the bounded successor of #1478 (*Name every approved unsafe file in the
realtime dependency policy*), which failed its two-attempt budget. #1478's commits (`859fb98a8`,
`e403684d9`) were reverted, so `main` carries no unverified policy text; its spec and both verdicts
(`/home/bl/misofm/submix-verdicts/1478-attempt1.md`, `1478-attempt2.md`) are the evidence this
issue starts from.

Root's ruling (2026-10-08), verbatim:

> (1) #1478: rescope once. Revert its two doc commits in this batch (main gets no unverified policy
> text; keep the verdicts as evidence) and file a new bounded stream J issue: "Correct the realtime
> dependency policy's unsafe-ownership statements" — coverage of the 19 allowlisted paths as
> attempt 2 had it, plus explicit decisions for M1 (ffi.rs reachability: render-locked exports
> reach only copy_live_record), M2 (state exactly which exports run inside render_locked, after the
> H issue below lands, or name the exception until then), M3 (the x86 MXCSR description:
> memory-operand STMXCSR/LDMXCSR, and the write_mxcsr SAFETY premise restated to what fpenv
> actually writes) and (a) softfma.rs's stale comments and SAFETY premise; each claim checked
> against code by its verifier.

Documentation and comments only. No code, gate or allowlist change.

## Problem (verified on `main` at `a059cdd03`)

- **The authority.** The realtime gate's unsafe allowlist is the exclusion regex at
  `scripts/check-realtime-policy.sh:31` (19 paths). Its header comment (`:16-26`) sends the reader
  to `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", for "the full justification".
- **The section** (`docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership") names 7 of the
  19 paths and still justifies unsafe code that no longer exists: it describes `softfma.rs` as
  carrying wasm `simd128` promote/demote intrinsics of the software FMA, which #163 phase 2
  (`477dc15ee`) removed, and it calls `tools/audit/src/realtime.rs` and `tools/audit/src/protocol.rs`
  approved unsafe exceptions; neither contains `unsafe` (removed by `d9a66a952`, #104 phase B).
  #1478's spec, "Problem", lists the twelve unnamed paths.
- **Four allowlist entries approve nothing.** `crates/soft-clip/tests/allocation.rs`,
  `crates/transient-shaper/tests/allocation.rs`, `crates/true-peak-limiter/tests/allocation.rs` and
  `crates/multiband-compressor/tests/no_alloc_render.rs` hold no text that the gate's regex
  (`unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{`) matches; their wrappers were removed by
  `568ad4087`, `cb4898437` (#1046), `a6da0cade` and `39c4651b1`. `tools/wasm-gate-guest/src/lib.rs`
  holds only `#[unsafe(no_mangle)]`, which the regex does not match either. #1438's Amendment
  (root, 2026-10-08) removes these five entries from the allowlist in the J tool batch.
- **Web `ffi.rs` reachability (attempt-2 verdict M1).** The render-locked exports
  `miso_engine_web_v1_spectrum_read` (`hosts/host-web/src/ffi.rs:2979`), `spectrum_stream_read`
  (`:3103`) and `track_response_capture` (`:3472`) reach only the unsafe-bearing
  `copy_live_record` (`:870`), through `write_spectrum_window`, `spectrum_failure`,
  `run_live_response_capture`, `live_response_failure` and `LiveResponseCaptureSink`.
  `read_live_record` (`:1770`) is reached only from the analysis-worker exports
  (`spectrum_analysis`, `spectrum_stream_analysis`, `spectrum_stream_analysis_configure`,
  `track_response_analysis`), none render-locked and none called by the worklet. Attempt 2 said
  the three render-locked exports reach `read_live_record`.
- **The render thread is wider than the render-locked set (attempt-2 verdict M2).** The SDK's
  PCM-feed worklet (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`) calls
  `miso_engine_web_v1_source_submit` (`:386`) and `miso_engine_web_v1_source_seek` (`:445`)
  from `process()` (`:306-328`, through `drainSharedRing` and `applySharedSeek`) before it renders,
  and the engine worklet's port handlers call them on the same thread (`receiveSource`,
  `receiveSeek`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1525`, `:1746`). Neither export
  is render-locked (`ffi.rs:3846`, `:3922`), and both carry unsafe (`slice::from_raw_parts` at
  `:3891`, `:3898`, `:3901`, `:3942`). The header comments `hosts/host-web/src/render_lock.rs:6-7`
  and `ffi.rs:14-23` say every export the worklet calls on its render thread runs inside
  `render_locked`, which is false. Stream H's issue *Bring the PCM-feed worklet's source submit
  and seek under the render-locked allocation count* (#1488) fixes the code and those two headers.
- **The x86 MXCSR path (attempt-2 verdict M3).** On `x86_64` the control word is reached only
  through `_mm_getcsr`/`_mm_setcsr`, which lower to `STMXCSR m32`/`LDMXCSR m32` through a stack
  slot; x86 has no register form. The `write_mxcsr` `SAFETY` comment
  (`crates/lane/src/softfma.rs:102-104`) says the value "is a control word previously read by
  `read_mxcsr` with at most the FTZ and DAZ bits changed, so no rounding mode or exception mask is
  disturbed". Its non-test caller, `fpenv.rs` (`write_fp_control_word`, `:148`, from
  `CanonicalFpEnv::enter`, `:338`), writes `CANONICAL_MXCSR` = `0x1F80` at every native x86 render
  entry, precisely to remove a caller's directed rounding and unmasked exceptions
  (`fpenv.rs:40-45`), and restores the saved word on exit. The tests
  (`crates/lane/tests/fp_env.rs:69-70`, `crates/capi/src/runtime/tests.rs:3094`) write words with
  `RC` set to round-up and a status flag set.
- **`softfma.rs`'s stale comments (#1478 attempt-2 open item 2).** `:1` ("The MXCSR helpers gate
  G6 needs"), `:25-27` (the helpers "are used by gate G6"), `:82` and `:95` ("Gate G6 support,
  never called from a render path"). Since #146 `fpenv.rs` calls them at every native x86 render
  entry; since #1017 (`92b1def5e`) G6 (`crates/lane/tests/g6_ftz_inert.rs:10`, `:105`) reaches
  the control word only through `lane::fpenv`. The direct callers are `fpenv.rs`,
  `crates/capi/src/runtime/tests.rs:3094` and `crates/lane/tests/fp_env.rs:49`.

## Decisions

- **D1. Coverage (as attempt 2 had it).** The section lists every path on the allowlist as `main`
  holds it at implementation time (the awk gate's `unsafe source exclusions` line, or
  `Policy::workspace()` in `tools/realtime-policy` if the J tool batch has landed), once each,
  grouped by category (realtime primitives, lane intrinsics, C-ABI and browser-ABI boundaries, the
  browser render-locked allocator, test-only counting allocators, tool-only allocators and guests),
  with its introducing issue and a justification taken from the file's own header or `SAFETY`
  comments. If the four stale entries and the wasm-gate-guest entry are still on the allowlist,
  they are listed as entries that approve nothing, with the reason and the pending removal
  (#1438's Amendment); if they are gone, they are not listed. Attempt 2's text
  (`git show e403684d9:docs/REALTIME_DEPENDENCY_POLICY.md`) is the starting draft; every sentence
  taken from it is re-checked against the code, and the attempt-2 verdict's MINOR and NIT findings
  (m1: "the tools now at ... `tools/bench/src/protocol.rs`"; n1: `resource_lifecycle.rs` also calls
  the C render entry; n2: `allocation_tracker.rs`'s three direct `std::alloc` unsafe calls,
  `:801-808`; n3: `write_read_stereo` is a raw-slice stereo borrow, `disjoint.rs:317-345`) are
  fixed.
- **D-M1. Web `ffi.rs` reachability.** The section states that the render-locked exports reach
  only `copy_live_record`, and that `read_live_record` and `response_header_bytes` are reached only
  from the worker-side analysis and query exports. The chain is named by function, as the Problem
  gives it, and re-checked at implementation time.
- **D-M2. Which exports run inside `render_locked`.** The section separates two sets and names
  each export in each: the exports the worklet calls on its AudioWorklet thread after boot (the
  render thread), and the exports whose bodies run inside `render_locked`. It states that the
  render-allocation count covers exactly the second set.
  - If #1488 is on `main` at implementation time (preferred; see Dependencies), the two sets are
    equal for the worklet's post-boot calls, and the section says so.
  - If it is not, the section names `miso_engine_web_v1_source_submit` and
    `miso_engine_web_v1_source_seek` as the exception: render-thread code with unsafe, called in
    `process()` on the SDK feed path and from the worklet's port handlers, outside the
    render-locked count until #1488 lands. #1488 then removes that sentence (its Authorized
    paths).
  - The spectrum request and collection accessors, which the worklet calls only before boot, are
    named as outside both sets, as `ffi.rs:20-23` says.
- **D-M3. The x86 MXCSR description.** The `fpenv.rs` and `softfma.rs` entries say that on
  `x86_64` the control word is read and written with `STMXCSR m32`/`LDMXCSR m32` through a memory
  operand (a stack slot), and that the register-only description holds for AArch64 `mrs`/`msr`
  only. The `write_mxcsr` `SAFETY` premise is restated to what its callers actually write: on the
  render path, `fpenv.rs` writes only `CANONICAL_MXCSR` (`0x1F80`, the architectural default) and
  the word it read from the same thread on entry; tests also write words with other `RC`, flag and
  FTZ/DAZ bits. The restated premise gives the real soundness ground (no reserved MXCSR bit is
  ever set, and any non-default environment is restored before the caller returns to code that
  assumes the default), checked against the `core::arch` documentation of `_mm_setcsr` and the
  Intel SDM's MXCSR description. The doc entry quotes the restated premise, not the old one.
- **D-a. `softfma.rs`'s stale comments.** `crates/lane/src/softfma.rs:1`, `:25-27`, `:82`, `:95`
  and `:102-104` (comments and `SAFETY` text only) are corrected: the helpers' only non-test caller
  is `fpenv.rs`, which runs at every native x86 render entry; gate G6 reaches the word through
  `lane::fpenv`; the `SAFETY` premise is D-M3's. No code token in the file changes.
- **D-hdr. The gate header's `fpenv.rs` sentence (Amendment (root, 2026-10-09)).**
  `scripts/check-realtime-policy.sh:19-23` says `fpenv.rs` carries no `unsafe` of its own on
  `x86` and that "its one unsafe site is the AArch64 `mrs`/`msr FPCR` pair". Both are false on
  `e9798393e`: `fpenv.rs` also has the empty `asm!` scheduling barrier (`scheduling_barrier`,
  `crates/lane/src/fpenv.rs:322-329`, its `unsafe` block at `:326`), compiled on both `x86_64`
  and `aarch64`. The comment lines are corrected to name both sites (the AArch64 `mrs`/`msr`
  pair at `:166` and `:181`, and the barrier on both architectures) and to keep the `x86` reuse
  of `softfma.rs`'s MXCSR helpers (`fpenv.rs:141`, `:148`). Comment lines only; no code token,
  pattern or allowlist entry changes. If #1446 has deleted the gate before this slice lands,
  D-hdr lapses and the attempt record says so.
- **D-test. No checking test.** A test that compares this prose with the allowlist or the code
  greps prose and is refused (AGENTS.md, "Test value"). The verifier is the gate.

## Authorized paths

- `docs/REALTIME_DEPENDENCY_POLICY.md` ("Unsafe-code ownership" only)
- `crates/lane/src/softfma.rs` (comments and `SAFETY` comments only; stream G's file, named
  exception in STREAMS)
- `scripts/check-realtime-policy.sh`, the header comment lines `:19-23` only (D-hdr). Amendment
  (root, 2026-10-09).
- this spec

## Non-goals

- Any change to the allowlist, to either gate, or to any code token (#1438's Amendment removes
  the five entries).
- `render_lock.rs`, `hosts/host-web/src/ffi.rs` and the browser assets (#1488).
- ~~The awk gate's header comment (`scripts/check-realtime-policy.sh:16-26`, including its
  "one unsafe site" wording for `fpenv.rs`, which also has an empty `asm!` barrier at `:326`):
  #1446 deletes the gate.~~ Struck by the Amendment (root, 2026-10-09): lines `:19-23` are in
  scope (D-hdr). The rest of the header (`:16-18`, `:24-26`) stays out of scope.
- `crates/lane/src/fpenv.rs`, the rest of the policy document, `docs/REALTIME_MEMORY.md`.

## Hazards

- #1446 D4 edits comment lines of this section and of `softfma.rs`, keeping each line count.
  Either order; the later slice rebases and keeps the other's wording (STREAMS hot-file row).
- #1438's Amendment changes the allowlist inside the J tool batch. D1 is "as `main` holds it".
- Line numbers above are on `a059cdd03`; re-read each cited site before writing.

## Objective gates

1. **Exact coverage (verifier, recorded).** The attempt record lists the allowlist's paths
   (copied from the gate at the implementation commit) beside the section's paths; the two sets
   are equal, apart from cited tests that the record names.
2. **Every claim checked against code (verifier).** The verifier checks each sentence of the
   section and each changed comment in `softfma.rs` against the code it describes: each path's
   justification, each introducing issue and commit, each reachability statement (D-M1's chain,
   D-M2's two sets export by export, against `ffi.rs` and both worklet files), D-M3's instruction
   forms (against the `core::arch` source or the emitted assembly) and its restated premise
   (against every `write_mxcsr` caller). Any false statement fails the attempt.
3. `cargo build --locked -p lane` (the comment edit compiles),
   `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`,
   `bash scripts/check-realtime-policy.sh`, `bash scripts/check-lane-policy.sh` and the docs job's
   checks (`bash scripts/check-dsp-research.sh`, `bash scripts/check-builtins-listening.sh`) exit 0.

*Test value.* No test is added (D-test).

## Evidence

- Gate 1's two path lists; gate 2's per-claim check table (claim, code site, verdict); the
  assembly or `core::arch` lines used for D-M3.

## Dependencies

- **After (other streams), preferred:** H #1488. If it has not landed, D-M2's exception clause
  applies and this slice does not wait.
- Either order with #1446 (hot file) and with the J tool batch (D1).

## Standing rules for the implementer

- Work only from this body. Read the cited lines and both #1478 verdicts first.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under three hours.

### Amendment record (root, 2026-10-09)

- Scope gains D-hdr and its Authorized-paths entry (`scripts/check-realtime-policy.sh:19-23`,
  comment lines only); the matching Non-goals bullet is struck. Verifier gate 2 checks D-hdr's
  corrected sentence against `fpenv.rs` like every other claim.
- The GitHub #1489 body predates this amendment and needs a sync from this spec.
