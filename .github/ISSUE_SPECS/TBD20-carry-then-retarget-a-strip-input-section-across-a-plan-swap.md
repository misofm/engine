# Carry, then retarget a strip input section across a plan swap

Successor found by the batch-10 carry-chain check, decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `12d4a4f9d`.
Issue number: assigned at filing (written here as TBD20). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review.

## Product outcome

A structural edit that also changes a strip's live input value (trim, polarity, an input filter)
keeps the strip's input section running: the successor carries the predecessor's input state and
then ramps to the new value, on both hosts, instead of restarting the section.

## Context

- The input join requires every input value to be equal: `same_input_section`
  (`crates/host-core/src/prepare.rs:2726`, used at `:1803`). Any live input difference in the same
  transaction as a structural edit restarts the input section (its filter state and ramps).
- The effect carry follows the same pattern: #1280a carries, #1280c retargets a live difference
  after the carry (D4).
- The C ABI behaves the same today, so this does not block #1562b.

## Decisions frozen for this slice

- **D1.** The input join keys on the section's structure (which builtins exist, their designs'
  shape), not on live values; a live difference carries the state and retargets.
- **D2.** The retarget writes through the successor's input producer, stamped on its gate, in the
  block that writes the strip retargets, with the session's ramps; filter retargets use the
  existing live input-filter path (#1556).
- **D3.** The carry is allocation-, lock- and syscall-free on the swap block.

## Objective gates

1. A host-core successor-swap test: a structural edit plus a trim change on a surviving strip; the
   input section's state carries (no restart transient) and the trim ramps to the new value,
   bit-equal to a plan that made the same edit live.
2. The same for an input filter cutoff change and a polarity change.
3. Zero allocations and frees on the swap block; `cargo test --locked -p host-core --all-features
   -p capi -p host-web --features host-web/test-support`; clippy.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the input join and retarget), `crates/host-core/tests/successor_swap.rs`,
  the builtins input stage's carry entry if it lacks one, this spec's decision record.

## Dependencies

- #1280c (the retarget pattern). Lane K.

## Size

Half a day to a day.
