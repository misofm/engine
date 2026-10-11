FAIL

# #1478 attempt 1 -- adversarial verdict

Commit `859fb98a8` (parent `9d7722276`), worktree `/home/bl/misofm/wt-d15-misc`, reviewed on an
exported tree (`git archive`), never in the worktree. The diff touches only the two authorized paths
(`docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership" lines 45-207, and the spec's attempt
record, additions only). `origin/main` holds the same `scripts/check-realtime-policy.sh` and policy doc
as the parent, and `tools/realtime-policy` does not exist, so the awk gate is the right authority.

Gate 1 (exact coverage) holds: the 19 allowlist paths of `scripts/check-realtime-policy.sh:31` equal
the section's full `.rs` paths before "Retired exceptions" (the only extras are the cited test
`crates/lane/tests/fp_env.rs` and the short names `fpenv.rs`/`softfma.rs`). D2 (render_lock.rs) is
accurate. Gate 2 (every justification true against its file) fails on two entries.

## MAJOR

**M1. The `softfma.rs` entry justifies unsafe code that was deleted six weeks ago.**
`docs/REALTIME_DEPENDENCY_POLICY.md:81-85` says the file carries unsafe for "the wasm `simd128`
promote/demote intrinsics of the software FMA" and the x86 MXCSR read/write. #163 phase 2
(`477dc15ee`, 2026-08-26) retired the software FMA and removed every wasm intrinsic from the file
(`f64x2_promote_low_f32x4`, `f32x4_demote_f64x2_zero` and the rest). The file's own header says so:
`crates/lane/src/softfma.rs:1` "The MXCSR helpers gate G6 needs", `:5-14` "Until issue #163 phase 2
this module held the workspace's exact software FMA ... the emulation was retired". Its only unsafe
today is two x86 sites, `_mm_getcsr` (`:90`) and `_mm_setcsr` (`:105`). D1 requires each justification
to come "from that file's own header or `SAFETY` comments" and gate 2 requires the reviewer to check
it against the file. The text was carried over unchanged from the old section, and the attempt record
claims every justification was taken from the file. Also, the entry names only gate G6 as the user.
`fpenv.rs:141,148` calls these helpers at every native x86 render entry, so they are on the render
path. The fix: describe the MXCSR helpers, name both users (G6 and `fpenv.rs`), and add a past-tense
note that #163 phase 2 (`477dc15ee`) removed the wasm FMA intrinsics.

**M2. "`fpenv.rs` is the one allowlisted file that is reachable from a render path" is false.**
`docs/REALTIME_DEPENDENCY_POLICY.md:89`. This attempt changed the old wording ("the one place in the
workspace") so that the claim now covers the 19-path list this section gives, and it is false for
several of those paths:
- `crates/engine/src/realtime/spsc.rs` has unsafe at `:381` and `:449`, inside its
  `REALTIME_POLICY_BEGIN/END` region (`:295-459`).
- `crates/engine/src/realtime/disjoint.rs` has unsafe at `:208`, `:303` and `:334`, inside its marked
  region (`:106-347`). The section's own entry at `:67-68` says "which the sequential executor still
  renders through".
- The C render entry `miso_engine_v1_render_f32_planar` (`crates/capi/src/ffi.rs:807-843`) has
  unsafe derefs, and the same is true of the web render export.
- `softfma.rs`'s MXCSR helpers run on the render path through `fpenv.rs` on x86.
- `render_lock.rs`'s allocator hook sits under every render-locked export.

A realtime policy that gets wrong which approved unsafe code is on the render path misleads the
review it exists to support. The fix is a one-word scope change, for example "the one lane file", or
a sentence that names the render-reachable entries.

## MINOR

**m1. The `hosts/host-web/src/ffi.rs` entry leaves out the file's second unsafe owner.**
`docs:121-124` describes only the export boundary and says "Each unsafe block carries a `SAFETY`
comment that names the bounds check or the liveness it relies on". The file also has a
`#[cfg(test)]` `#[global_allocator]` `CountingAllocator` (`hosts/host-web/src/ffi.rs:4777-4842`, in
`live_response_ffi_tests`, added by `708036a76`). Its SAFETY comments name neither a bounds check nor
liveness. The test-allocator category text (`docs:154-155`, "in a `tests/` file") also implies that no
`src/` file holds a counting allocator. D2's "only allowlisted global allocator linked into a shipped
artifact" stays true, because this one is cfg(test).

**m2. The retired paragraph keeps a false count.** `docs:188-190` says "The source-policy checker
originally accepted unsafe syntax in exactly four source files". The first gate
(`68ff477e4:scripts/check-realtime-policy.sh:19`) excluded five, including
`tools/miso-engine-effect-contract-bench/src/main.rs`. Only its failure message said "four". That
fifth exemption's removal (one of the eleven tool paths `d9a66a952` dropped) is not recorded. The
sentence is pre-existing, but this slice rewrote the paragraph as the record of retired exceptions.

## NIT

- n1. `docs:118-120` (capi `ffi.rs`): "Each dereference carries a SAFETY comment that names the
  caller's pointer contract and the capacity check made before the write" overstates. Several name a
  live-kind check or projection disjointness instead (`crates/capi/src/ffi.rs:161,172,182,837`).
- n2. `tools/wasm-gate-guest/src/lib.rs`'s entry in the realtime gate is as inert as the four stale
  ones: the gate regex (`unsafe\s+(impl|fn|extern)|unsafe\s*\{`) never matches `#[unsafe(no_mangle)]`.
  The real approval is `scripts/check-bench-policy.sh`'s exact `tools/` set. The doc's description is
  true, but the successor below should decide this entry too.
- n3. The attempt record says `allow(unsafe_code)` appears in a doc comment in
  `crates/host-core/src/lib.rs`. It does not: `:22` mentions `#[unsafe(no_mangle)]`. Only
  `tools/bench-support/src/lib.rs:13` has such a comment.
- n4. The four stale paths are named without their introducing issues (D1 asks for one for every
  path). Giving the removal commits is the more useful record, but say that the omission is
  deliberate.

## Verified true

- Every introducing issue checked against the adding commit: spsc 003 (`68ff477e4`); disjoint #100
  (`ab0aea713`); softfma #83 (`c018e2547`); fpenv #146 (`27e8299c6`); capi `ffi.rs` 022 (`947d7996f`);
  web `ffi.rs` 024 (`ef925e6cb`); `plan_swap_race.rs` #1042/#1273 (header, `1338b063c`);
  `resource_lifecycle.rs` 119 (`1a3dde27e`); audit `capi.rs` 022 (header, `434551bc0`); `render_lock.rs`
  #1333 (`4830bed93`); `allocation_tracker.rs` 007 (`ff7546734`); `allocation_budget.rs` #107
  (`c94f659b1`); `boot_transient_budget.rs` #240 (`3d91830b3`); bench-support `alloc.rs` #104
  (`d9a66a952`); wasm-gate-guest #83 G5 (`3ef329b14`).
- Every other entry matches its file's SAFETY comments and header: spsc, disjoint, `fpenv.rs` (the only
  `asm!` in the workspace), `plan_swap_race.rs`, `resource_lifecycle.rs` (`LifecycleAllocator`), audit
  `capi.rs`, the three test allocators, bench-support `alloc.rs` (abort by default, `Mode::Count`), and
  wasm-gate-guest (`u32`-only exports, no unsafe block, fn or impl).
- D2 / `render_lock.rs`: it forwards all four operations unchanged (`:57-81`), with one `try_with` flag
  read and one `Relaxed` `fetch_add` only when locked (`:44-53`). It is the D15-10 runtime proof
  behind `call_indirect` (`:1-9`). Qualification asserts zero (`hosts/host-web/qualification/run.mjs:242-246`).
  The registration is `cfg(all(target_family = "wasm", not(test)))` (`:86`). It is the only allowlisted
  global allocator in a shipped artifact: bench-support is a dev-dependency only in every
  `crates/`/`hosts/` manifest, and the other allocators are in `tests/` or `cfg(test)`.
- Retired paragraph: `d9a66a952` removed `tools/miso-engine-{realtime,protocol}-audit/src/main.rs` from
  the allowlist and stripped their allocators. Today's `tools/audit/src/{realtime,protocol}.rs` contain
  no `unsafe`. `5e15bc962` (#1033) removed `tools/native-pcm-runner/src/lib.rs` from the allowlist, and
  `9b5c3c346` (#1075) removed `tools/bench/src/protocol.rs`.
- The four stale entries are truly stale. The soft-clip, transient-shaper and true-peak-limiter
  `tests/allocation.rs` files and the multiband-compressor `tests/no_alloc_render.rs` file contain no
  `unsafe` and no `allow(unsafe_code)`, and they use `bench_support::alloc`'s
  `current_thread_counters`/`current_thread_delta_since`. The wrappers were removed by `568ad4087`,
  `cb4898437` (#1046), `a6da0cade` and `39c4651b1`, as stated (`git log -S'unsafe impl GlobalAlloc'
  --follow`).
- The final paragraph is true: `scripts/test-realtime-policy.sh:546-547`
  (`unsafe-outside-capi-audit-main`) writes `tools/audit/src/other.rs`, and `:548-559` cover the
  retired paths `tools/native-pcm-runner`, `tools/bench/src/protocol.rs` and `crates/engine/src/arch`.
  No case covers `tools/audit/src/{realtime,protocol}.rs`, but the sentence does not claim one.
- Companion scripts: `check-bench-policy.sh:209-229` holds the exact three-file `tools/` set.
  `check-builtins-policy.sh:23` names `allocation_tracker.rs`. `check-lane-policy.sh` names `fpenv.rs`
  and `softfma.rs` through `scripts/policies/lane-source.toml`.

## Stale entries: document here, and file a successor

This slice should only document the four stale entries: changing the allowlist is a non-goal. A
successor issue is still needed. The workspace lint is overridable per file, and the realtime gate's
exact-file list is the only mechanical check on new unsafe owners under `crates/` and `hosts/`. So
today a change that adds `#![allow(unsafe_code)]` plus unsafe code to any of those four test files
passes the gate unreviewed. The successor should do three things:
- remove the four entries;
- decide the inert wasm-gate-guest entry (n2);
- delete the doc's "Four further entries" paragraph.

#1438 D5 copies the allowlist verbatim and #1438 says that two gates hold one allowlist until C2.
Sequence the successor after #1446, or amend #1438 D5, so that only one allowlist is edited.

## Test value

No test added (D4). That is correct: a test that compares this prose with the allowlist would grep
prose. No queue is touched, so the acked-batch question does not apply.

## Gates run (exported tree, `CARGO_TARGET_DIR=/tmp/claude-1002/v1478/target`)

- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok` (exit 0)
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)` (exit 0)
- `bash scripts/check-dsp-research.sh`: `dsp research corpus: ok` (exit 0)
- `bash scripts/check-builtins-listening.sh`: `issue-007 listening preregistrations: ok (human evidence pending)` (exit 0)
- Gate 1 set comparison (`diff` of the sorted allowlist and the sorted section paths): equal, 19/19.
