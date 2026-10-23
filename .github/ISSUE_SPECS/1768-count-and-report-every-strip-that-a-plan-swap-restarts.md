# Count and report every strip that a plan swap restarts

Owner decision (2026-10-23): add this counter in optimisation batch 4. Found by the design review of 2026-10-23.

## Problem

A structural edit that cannot carry a strip's stage state starts that strip at rest (`resolve_strip_carry` and the effect and strip-delay carries, `crates/host-core/src/prepare.rs` near 3734-3766, 5997-6044, 6252-6304). The fade-in that decision 15 D15-9 asks for is not implemented (#1288, #1324, #1325 are open), so a restart is a step in the output, and nothing counts or reports it: `PreparedHost::restarted_strips()` (`prepare.rs` near 729) is read only by tests, and the C ABI header says values on a lane that a swap restarts are not counted (`crates/capi/include/miso_engine_v1.h` near 70-71). Decision 15 requires an outcome flag or counter for every fallback (D15-17). An app cannot see that an edit restarted strips it did not touch.

## Smallest closable slice

- In the core: when render adopts a plan, add that plan's restarted-strip count to one monotone session counter, `strips_restarted`, next to `live_values_superseded`. The count is decided at prepare (the list exists); render only adds it at adoption, allocation-free. A withdrawn or never-adopted candidate adds nothing.
- Report it as one new protocol counter read with `COUNTERS_GET` on the C ABI, and through the same core counter frame in the browser and the SDK. Hosts stay thin: no host computes it.
- Correct the header sentence and the SDK doc.

## Authorized paths

- `crates/host-core/`, `crates/control-plane/`, `crates/protocol/` (the counter and its frame)
- `crates/capi/` (the counter id, the header doc, its parity row)
- `hosts/host-web/` and `sdk/` (the counter read and its parity row)
- this spec
- `docs/CONTROL_PROTOCOL_REGISTRY.md` and `docs/C_ABI_V1_QUALIFICATION.md` (the counter's entry; root ruling 2026-10-23)

## Objective gates

1. One core test: a structural edit that restarts N strips adds exactly N when render adopts the plan; a candidate that is withdrawn adds 0; an edit that restarts none adds 0.
2. One parity row per host: the C ABI and the browser report the same value for the same edit.
3. Render allocates nothing; `tools/realtime-policy` passes; fmt, the workspace clippy line, `cargo check --target aarch64-unknown-linux-gnu --tests` and `cargo doc -D warnings` on touched crates.

## Test value

The core test turns red when an adopted restart is not counted, or when a candidate that never plays is counted. No current test reads restarts outside host-core's carry tests.

## Dependencies

None. Edit order: #1767 (the fused-stage removal) edits `resolve_strip_carry` in the same file and merges first.

## Decision record (attempt 1, 2026-10-23)

- **D1 Counter.** `CounterId::StripsRestarted = 18` (`STRIPS_RESTARTED`), the next free ID. The
  provider serves it beside `LIVE_VALUES_SUPERSEDED` (`SERVED_COUNTERS`, now five), a saturating
  `u64` from 0. The protocol's counter test now refuses 19 as the next ID. The SDK has no typed
  counter-ID list (it sends `COUNTERS_GET` frames through `control()`/`controlFrame()`), so only
  its doc changes.
- **D2 Where it is added (root ruling 2026-10-23: accepted; D2 replaces the slice's "render
  only adds it at adoption" wording).** In the control plane, where a pending provider is promoted
  (`SessionState::promote_pending_provider`, `crates/control-plane/src/control.rs`): a provider is
  promoted only once render has adopted its plan (the active-epoch atomic render stores after the
  adopting block, the reclaim of the plan it displaced, or a `Taken` withdrawal), each adopted
  candidate exactly once, and a withdrawn candidate never (rebuild pops it). Both hosts run this
  one `SessionState`, so no host computes the value. The refresh that copies
  `live_values_superseded` into the provider copies it too. Render code does not change, so render
  allocation and the realtime policy are unchanged. The spec's "render adds it at adoption" would
  need a per-candidate word through the plan exchange (`crates/engine`, as `superseded` travels)
  or the graph hand-over (`crates/graph`), both outside the authorized paths; the control-side
  promotion observes the same adoption without them. `COUNTERS_GET` reports a restart once the
  adopting block's render call has returned (every session call runs the service step first).
- **D3 Rule.** A strip counts once per adopted plan, whichever carries restart it: the count is
  the length of the plan's deduplicated `restarted` list (`PlanStateInventory::restarted_strips`,
  new accessor; `PreparedHost::restarted_strips` reads it). Added strips are not restarts. A
  discontinuity successor lists none (its base carries no strip state; the jump is declared, not a
  fallback). A candidate withdrawn unadopted adds nothing; its replacement is prepared against the
  running plan, so its list holds the withdrawn edit's strips too.
- **Known limits.** (1) A hand-over that render refuses whole (`CarryOutcome::PredecessorMismatch`)
  or a refused console restore (`carry_refusal_count`, `crates/graph/src/lib.rs` near 4323) starts
  lanes at rest that the list does not name. Both counts are internal (not on the C ABI), and the
  paths are unreachable by construction (#1269 P11); an app-visible defect counter belongs on the
  #1521 model, not here. (2) A refused input-section join (`prepare.rs` near 2481-2503) starts the
  input section at rest, and a send whose record `join_routes` refuses starts at its prepared
  value; neither is listed, and neither is reachable in product today (one session's live lanes
  agree, and a rate change does not come through a successor). (3) The SDK has no typed counter
  read for any counter; #1571 owns it. Per-revision attribution of restarts belongs with #1288 or
  #1324 (watermark outcome).
- **Path extension (root ruling 2026-10-23).** `docs/CONTROL_PROTOCOL_REGISTRY.md` (IDs run 1 to
  18, the `STRIPS_RESTARTED` entry) and `docs/C_ABI_V1_QUALIFICATION.md` (the served-counter
  paragraph now names the five counters `SERVED_COUNTERS` serves; canceled automation, ID 12, was
  stale: retired by #1629, absent from `CounterId`).

## Verifier (attempt 1, 2026-10-23)

PASS. Root corrected the known-limit wording from its findings 2 and 3, and recorded that the SDK
counter read is #1571's and per-revision attribution is #1288's or #1324's. Note for #1521: its
counter now takes the next free ID, 19.
