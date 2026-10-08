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
