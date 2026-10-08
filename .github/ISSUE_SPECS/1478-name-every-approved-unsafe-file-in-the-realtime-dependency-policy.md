# Name every approved unsafe file in the realtime dependency policy

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-08 by root from the #1333 attempt-1 verdict, MINOR 5 (open item 3). The realtime-policy
gate's unsafe allowlist names `hosts/host-web/src/render_lock.rs`, and its header comment sends the
reader to `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", for the full
justification. That section does not name the file, and it has fallen behind the allowlist in
other places too.

Documentation only. No code, gate or allowlist changes.

## Problem (verified on `main` at `1e78d7820`)

- **The authority.** The unsafe allowlist is the exclusion regex of
  `scripts/check-realtime-policy.sh:29` (19 paths). Its header comment (`:16-26`) names
  `render_lock.rs` as "Issue #1333's ... the browser module's `GlobalAlloc` forwarding wrapper" and
  ends: "`docs/REALTIME_DEPENDENCY_POLICY.md`, 'Unsafe-code ownership', carries the full
  justification."
- **The section** (`docs/REALTIME_DEPENDENCY_POLICY.md:45-136`) names 7 of the 19 paths:
  `spsc.rs`, `disjoint.rs`, `softfma.rs`, `fpenv.rs`, `crates/capi/src/ffi.rs`,
  `hosts/host-web/src/ffi.rs`, `hosts/host-web/tests/boot_transient_budget.rs` (and the crate or
  file names `builtins-compiler` and `transient-shaper` loosely). It does not name:
  - `hosts/host-web/src/render_lock.rs` (#1333): the shipped browser module's
    `#[global_allocator]` (`render_lock.rs:87`), a production wrapper, not a test-only one;
  - `crates/builtins-compiler/tests/allocation_tracker.rs`,
    `crates/session/tests/allocation_budget.rs`, `crates/soft-clip/tests/allocation.rs`,
    `crates/transient-shaper/tests/allocation.rs`, `crates/true-peak-limiter/tests/allocation.rs`,
    `crates/multiband-compressor/tests/no_alloc_render.rs`;
  - `crates/capi/tests/resource_lifecycle.rs`, `crates/capi/tests/plan_swap_race.rs`;
  - `tools/bench-support/src/alloc.rs`, `tools/audit/src/capi.rs`,
    `tools/wasm-gate-guest/src/lib.rs`.
- **Stale names.** The section still calls `tools/audit/src/realtime.rs` and
  `tools/audit/src/protocol.rs` approved unsafe exceptions (`:98-108`, `:115-118`). Neither file
  contains `unsafe` today, and neither is on the allowlist. `:120-122` says the paragraph "has
  fallen behind" and that reconciling it "belongs to the #104 evidence triage"; #104 is closed.
- **Not covered elsewhere.** Stream J's tool batch moves the allowlist into
  `tools/realtime-policy` (#1438 D5 takes "`:29` as `main` holds it", so the tool will carry
  `render_lock.rs`), and #1446 D4 renames the gate in four of this section's lines
  (`:52`, `:84`, `:120`, `:132`) without changing what they say. Neither names the missing files
  or removes the stale ones.

## Decisions

- **D1. The section lists the allowlist exactly.** Every path on the allowlist as `main` holds it
  at implementation time (the awk gate's `:29`, or `Policy::workspace()` in `tools/realtime-policy`
  if #1446 has landed) appears once in the section, with its introducing issue and a one-sentence
  justification taken from that file's own header or `SAFETY` comments. Paths are grouped by
  category: realtime primitives, lane intrinsics, C-ABI and browser-ABI boundaries, the browser
  render-locked allocator, test-only counting allocators, and tool-only allocators.
- **D2. `render_lock.rs` is its own category.** It is the only allowlisted global allocator linked
  into a shipped artifact. The paragraph states: it forwards every operation to the inner allocator
  unchanged; its only additions are one thread-local flag read and, inside a locked window, one
  relaxed counter increment; it is the runtime proof behind D15-10, because the static call-graph
  gate cannot see behind the plan executor's `call_indirect` (`render_lock.rs:1-18`).
- **D3. Stale text goes.** The sentences that call `tools/audit/src/realtime.rs` and
  `tools/audit/src/protocol.rs` unsafe exceptions are rewritten in the past tense or removed, with
  the issue that removed their unsafe if `git log -S` shows it. The "fallen behind ... #104"
  paragraph is replaced by one sentence: the gate's allowlist is the authority, and this section
  lists it.
- **D4. No checking test.** A test that compares this prose with the allowlist greps prose and is
  refused (AGENTS.md, "Test value"). Review is the gate.

## Authorized paths

- `docs/REALTIME_DEPENDENCY_POLICY.md` ("Unsafe-code ownership" only)
- this spec

## Non-goals

- Any change to the allowlist, to the gate, or to any unsafe file.
- The rest of the policy document.
- `docs/REALTIME_MEMORY.md`.

## Hazards

- #1446 D4 edits four lines of this section (`:52`, `:84`, `:120`, `:132`). Either order; the
  later slice rebases and keeps the other's wording (STREAMS hot-file row). If #1446 lands first,
  name `tools/realtime-policy` as the gate.
- A path added to the allowlist between filing and implementation is listed too; D1 is "as `main`
  holds it".

## Objective gates

1. **Exact coverage (review, recorded).** The attempt record lists the allowlist's paths (copied
   from the gate at the implementation commit) beside the section's paths; the two sets are equal.
2. **Justifications are true (review).** For each path, the stated reason matches the file's own
   header or `SAFETY` comment; the reviewer checks each against the file.
3. `bash scripts/check-workspace-policy.sh` and the docs job's checks (`qualification.yml`, "docs
   and research evidence gates") pass.

*Test value.* No test is added (D4).

## Evidence

- Gate 1's two path lists, the `git log -S` lines used for D3.

## Dependencies

- After: none. Either order with #1446 (hot file).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under two hours.

## Attempt record

### Attempt 1 (implementer, 2026-10-08)

**Change.** `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", rewritten: one entry per
allowlisted path, grouped as D1 asks (realtime primitives, lane intrinsics, C-ABI and browser-ABI
boundaries, the browser render-locked allocator, test-only counting allocators, tool-only
allocators and guests), with the introducing issue and a justification taken from the file's own
header or `SAFETY` comments. `render_lock.rs` is its own category (D2). The stale
`tools/audit/src/realtime.rs`/`protocol.rs` text moved to a "Retired exceptions" paragraph in the
past tense (D3); the "fallen behind ... #104" paragraph is replaced by the statement that the gate's
allowlist is the authority and the section lists it. The last paragraph's stale "exact
protocol-audit `main.rs` allowlist" now names the real mutation case (`tools/audit/src/other.rs`).
#1446 had not landed (`tools/realtime-policy` does not exist), so the awk gate is named.

**Gate 1 (exact coverage).** Allowlist extracted from the `unsafe source exclusions` line
(`scripts/check-realtime-policy.sh:31` at this commit; the spec's `:29` is the comment line before
it), sorted; the section's backticked `.rs` paths before "Retired exceptions", sorted, minus the
cited test `crates/lane/tests/fp_env.rs`. `diff` of the two: equal, 19 paths each:

```
  crates/builtins-compiler/tests/allocation_tracker.rs
  crates/capi/src/ffi.rs
  crates/capi/tests/plan_swap_race.rs
  crates/capi/tests/resource_lifecycle.rs
  crates/engine/src/realtime/disjoint.rs
  crates/engine/src/realtime/spsc.rs
  crates/lane/src/fpenv.rs
  crates/lane/src/softfma.rs
  crates/multiband-compressor/tests/no_alloc_render.rs
  crates/session/tests/allocation_budget.rs
  crates/soft-clip/tests/allocation.rs
  crates/transient-shaper/tests/allocation.rs
  crates/true-peak-limiter/tests/allocation.rs
  hosts/host-web/src/ffi.rs
  hosts/host-web/src/render_lock.rs
  hosts/host-web/tests/boot_transient_budget.rs
  tools/audit/src/capi.rs
  tools/bench-support/src/alloc.rs
  tools/wasm-gate-guest/src/lib.rs
```

**Code survey (not the spec's list alone).** `rg` of the gate's own regex over `crates hosts tools`
finds unsafe sites in 14 of the 19 files. Five allowlisted files have none:

- `tools/wasm-gate-guest/src/lib.rs` has `#![allow(unsafe_code)]` and `#[unsafe(no_mangle)]`
  only, which the gate regex does not match; it is still a real boundary (and on
  `scripts/check-bench-policy.sh`'s exact `tools/` set). Documented as such.
- `crates/soft-clip/tests/allocation.rs`, `crates/transient-shaper/tests/allocation.rs`,
  `crates/true-peak-limiter/tests/allocation.rs` and
  `crates/multiband-compressor/tests/no_alloc_render.rs` carry no unsafe at all; they use
  `bench_support::alloc`. Their wrappers were removed by (`git log -S'unsafe impl GlobalAlloc'`):
  `568ad4087` (soft-clip), `cb4898437` (#1046, transient-shaper), `a6da0cade` (limiter),
  `39c4651b1` (multiband). The section lists them as stale entries that approve nothing.

`allow(unsafe_code)` also appears only in the 19 files plus doc comments in
`crates/host-core/src/lib.rs` and `tools/bench-support/src/lib.rs` (prose, not allowances).

**D3 `git log -S` lines.** `git log -S'unsafe' --follow -- tools/audit/src/realtime.rs` and the same
for `protocol.rs` both give `d9a66a952 refactor(tools): one audited allocator, escaper, percentile
and timer (#104 phase B, F4/F1)` as the commit that removed their unsafe.

**Introducing issues** not stated in a file header were taken from the adding commit and the
`.github/ISSUE_SPECS/README.md` text of that time: `crates/capi/src/ffi.rs` and
`tools/audit/src/capi.rs` issue 022 (`947d7996f`, `434551bc0`); `hosts/host-web/src/ffi.rs` issue
024 (`ef925e6cb`); `crates/capi/tests/resource_lifecycle.rs` issue 119 (checkpoint `1a3dde2`);
`crates/builtins-compiler/tests/allocation_tracker.rs` issue 007 (`ff7546734`);
`crates/session/tests/allocation_budget.rs` #107 (`c94f659b1`).

**Gate 3.** `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
`bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`.
Docs job (`qualification.yml`, "docs and research evidence gates"): `bash scripts/check-dsp-research.sh`
ok; `bash scripts/check-builtins-listening.sh` ok.

**Test value.** No test added (D4).

**Open item for root.** The four stale allowlist entries above (the three `allocation.rs` files and
`no_alloc_render.rs`) admit unsafe code to files that need none. Removing them is a gate change
and a non-goal here; it needs a successor issue (or folds into #1438/#1446's
`tools/realtime-policy` allowlist). When it lands, the "Four further entries" paragraph of the
section goes with it.
