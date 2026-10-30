# Charge the browser boot's projection without processors for banked members

Filed by root from the #2199 verify (attempt 1 MINORs). Follow-up; not launch-blocking.

## Problem
After #2199 the browser boot admits on its projection (`RetainedProjection`, #2199 D4), a ceiling of the exact aggregate. Two of its rows are loose:
- the effect row still charges a processor for every banked member, although #2199 slice 2 prepares no processor for a banked member;
- the send-lane, inventory and later-carry rows are charged at their count bound.

On the dense shape (`dense_document(6_000)`) the projection is 47.8 MB (8.2 %) above the exact total. So the browser boot refuses a session that fits the budget by up to about 8 %, while a replace of the same session at the same budget is admitted (the replace pre-check charges exact rows, #2199 D3).

## Scope
1. Charge the effect row of the browser boot projection with no processor for a member the bank plan banks (the same rule `compile_projected` follows).
2. Tighten the remaining ceiling rows where the count is known before preparation; state the residual slack for each row that stays a ceiling.
3. Size the dense gate shape with margin: today its exact total is only 0.22 % over the 512 MiB budget, so tightening the projection can turn the gate's "exceeds the budget" expectation red. Pick a shape whose exact total is clearly over the budget, and re-measure the native and wasm legs.
4. One builder for the dense shape: today it is copied in `hosts/host-web/tests/boot_transient_budget.rs`, `hosts/host-web/examples/dense_document.rs` and `crates/control-plane/tests/session_replace.rs` (the realtime policy refuses a `#[path]` with `..`). Share it through a crate the tests already depend on, or record why three copies must stay.

## Gates
- The browser boot admits every fixture session at a budget equal to its exact total plus the stated residual slack, and the slack is under 1 % on the dense shape.
- The dense gate still refuses before its rows are allocated, native and wasm.
- No rendered bit moves (one-time comparison against the pre-change base, PR evidence only).
