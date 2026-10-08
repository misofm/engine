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

**Correction (attempt 2).** The survey line above is wrong about `crates/host-core/src/lib.rs`: its
`:22` mentions `#[unsafe(no_mangle)]`, not `allow(unsafe_code)`. Only
`tools/bench-support/src/lib.rs:13` has an `allow(unsafe_code)` doc comment.

### Attempt 2 (implementer, 2026-10-08)

Fixes the attempt-1 verdict (FAIL: M1, M2, m1, m2, n1-n4). Every entry was re-checked against the
file's current code, not against attempt 1's text. Only "Unsafe-code ownership" changed.

- **M1 (`softfma.rs`).** The entry now describes what the file holds today: `read_mxcsr` and
  `write_mxcsr`, two unsafe blocks calling `_mm_getcsr` (`softfma.rs:90`) and `_mm_setcsr`
  (`:105`), with their `SAFETY` reasons. It names both users: gate G6 and `fpenv.rs`
  (`fpenv.rs:141,148`), which calls them at every native `x86` render entry
  (`crates/capi/src/ffi.rs:817`, `crates/host-core/src/render_session.rs:121`). It adds that
  #163 phase 2 (`477dc15ee`) retired the software FMA and its wasm intrinsics. Found while
  checking: the helpers' own doc comments (`softfma.rs:82,95`) still say "never called from a
  render path", which is false on `x86` since #146; the doc says so. The file is a non-goal here
  (open item).
- **M2 (render-path reachability).** The false sentence is now "`fpenv.rs` is reachable from a
  render path deliberately". A new "Render-path reachability" subsection states, per entry,
  checked against the code: `spsc.rs` yes (unsafe at `:381`, `:449` inside the marked region
  `:295-459`; render-side users are `plan_exchange.rs` and the graph's control rings);
  `disjoint.rs` yes (`:208`, `:303`, `:334` inside `:106-347`); `fpenv.rs` yes on native;
  `softfma.rs` yes on `x86_64` through `fpenv.rs`; capi `ffi.rs` yes (`plan_kind`, `&*output`,
  `plan_state`, `plan_error_slot` derefs in `miso_engine_v1_render_f32_planar`, `:818-841`);
  web `ffi.rs` yes, with a correction to the verdict's wording: `miso_engine_web_v1_render`'s body
  (`:3949-3957`) holds no unsafe block and `with_host_mut` is safe, but the render-locked
  `spectrum_read`, `spectrum_stream_read` and `track_response_capture` reach the bounds-checked
  `copy_live_record` (`:870`) and `read_live_record` (`:1770`) through `write_spectrum_window`,
  `run_spectrum_analysis`, `run_live_response_capture` and `run_live_response_analysis`
  (the only other unsafe-bearing functions in the file are `source_submit`, `source_seek` and
  `response_header_bytes`, none render-locked); `render_lock.rs`: its allocator methods run on the
  render thread only if render calls the allocator, which is what they count; `render_locked` is
  safe. Every other entry is test or tool code.
- **m1.** The web `ffi.rs` entry names the `#[cfg(test)]` `CountingAllocator` in
  `live_response_ffi_tests` (`:4777-4842`, `708036a76`) and what its `SAFETY` comments say. The
  test-allocator category text now says the three listed files are `tests/` files and this one is
  the `src/` exception.
- **m2.** The retired paragraph now says the first gate (`68ff477e4:scripts/check-realtime-policy.sh:19`)
  excluded five files although its message said "four", names the fifth
  (`tools/miso-engine-effect-contract-bench/src/main.rs`, its own `unsafe impl GlobalAlloc`), and
  records its retirement: `d9a66a952` (#104 phase B) replaced its allocator and dropped the entry;
  `6349258fa` (#136) later deleted the package.
- **n1.** The capi `ffi.rs` entry lists what its `SAFETY` comments actually name: pointer or
  handle contract, size/capacity check, live-kind check on `HandleHeader`, field-projection
  disjointness, and `Box` ownership transfer (`:120-183`, `:207-299`).
- **n2.** The wasm-gate-guest entry says its realtime-gate entry is inert (the pattern
  `unsafe\s+(impl|fn|extern)|unsafe\s*\{` never matches `#[unsafe(no_mangle)]`; the file has zero
  `unsafe {`) and that `scripts/check-bench-policy.sh`'s exact `tools/` set is the binding
  approval.
- **n3.** Corrected in the attempt-1 record above.
- **n4.** The four stale paths now carry their introducing issues, from the commit that first
  added each file (`git log --follow --diff-filter=A`) and the gate commit that first listed it
  (`git log -S<path> -- scripts/check-realtime-policy.sh`): soft-clip #91 (`651903b85`,
  `b49b9c885`); transient-shaper #92 (`3a4bc6cbe`, the checkpoint after `9670016a8` "re-land ...
  (#92)"; gate `72328a917`); true-peak-limiter #90 (`4c0293fe5`); multiband-compressor issue 018,
  re-landed under audit #94 (`e59859edc`).

**Gate 1 (exact coverage).** Allowlist from `scripts/check-realtime-policy.sh:31`
(`unsafe source exclusions`), sorted; the section's backticked `.rs` paths with a `/` before
"Retired exceptions", sorted. `diff`: one extra line on the doc side,
`crates/lane/tests/fp_env.rs` (the cited test); otherwise equal, 19 paths each (the list in
attempt 1 is unchanged).

**Gates.** `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
`bash scripts/check-realtime-policy.sh`: `realtime policy: ok (89 marked regions in 25 files)`.
Docs job (`qualification.yml`, "DSP research corpus and listening-evidence packet validators"):
`check-dsp-research.sh` ok; `check-builtins-listening.sh` ok.

**Test value.** No test added (D4).

**Open items for root.**
1. Successor issue (unchanged from attempt 1, widened per the verdict): remove the four stale
   allowlist entries, decide the inert wasm-gate-guest entry, and delete the doc's "Four further
   entries" paragraph; sequence it after #1446 or amend #1438 D5 so only one allowlist is edited.
2. `crates/lane/src/softfma.rs:1,26-27,82,95` say G6 is the helpers' only user and "never called
   from a render path"; since #146 `fpenv.rs` calls them at every native `x86` render entry. A
   one-line doc-comment fix in that file is outside this slice.
3. The gate's own header comment (`scripts/check-realtime-policy.sh:19-23`) says `fpenv.rs`'s one
   unsafe site is the `mrs`/`msr` pair; the file also has the empty `asm!` barrier (`:326`).
   Gate edits are a non-goal here.
