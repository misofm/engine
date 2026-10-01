# Make the safe arena API enforce its ownership and aliasing rules in release builds

## Finding and scope

The crate-by-crate housekeeping audit of engine (#1115) found that the public safe `ArenaLease` API relies on caller obligations that safe Rust cannot enforce. In `crates/engine/src/realtime/disjoint.rs`, `checked_write` checks the write capability with `debug_assert!`; `write_read`, `write_read2` and `write_read_stereo` guard output/input aliases only in debug. `read` permits foreign-buffer access after validating the ID, while the builder's wave ordering does not establish the documented E1 happens-before obligation. Leases are Send and the backing arena is Sync, so safe multi-lease callers can violate that obligation. The current single-lease production executor satisfies its intended access order; that fact does not make the public safe constructors/accessors sound for other safe callers. This is a separate safety correction, not a claimed result of behavior-preserving housekeeping.

## Smallest closable slice and decisions

Audit every public arena access method, including plane, offset, shape, write-set and alias checks, and construct safe-only release/Miri reproducers. Select a surface that mechanically guarantees spatial disjointness and execution ordering: checked safe admission and prevalidated capabilities, or a deliberately narrowed/unsafe multi-lease API with explicit obligations. The issue must freeze the chosen API/error behavior before implementation and state whether any cross-crate caller needs migration. Preserve validated single-thread render behavior, bits, planar layout, stable ordering and exact retained-byte accounting. Do not reintroduce multicore scheduling.

Root Sol approves investigation and a bounded brief. An API direction that changes an existing public source surface is collected for the owner's final housekeeping report; no implementation is authorized by this issue yet.

## Objective gates

- Release-mode safe-only misuse reproducers cover out-of-write-set mutation, same-buffer output/read aliases, invalid plane/shape and concurrent foreign writer/read access. Run Miri where available; never intentionally execute undefined behavior in an ordinary test process.
- The selected surface rejects misuse before references are formed, or makes the required unsafe obligation visible at the actual call/ownership boundary. Documentation alone and debug-only assertions are insufficient.
- Existing valid arena, graph reduction/scatter, zero-copy and allocation/RT gates pass, with one-time unchanged-output evidence if implementation changes. Runtime checks are bounded and introduce no allocation/free, lock, syscall or structural render work.
- Focused native/target builds and proportional clippy/fmt gates; each new regression records the concrete bug's revert-red evidence or its unique plausible defect. No source/prose grep or resource-byte pin tests.
- Maximum five coherent implementation attempts after a frozen Sol brief, one adversarial verdict each. Root checkpoints and pushes exact owned paths, synchronizes this issue and closes only after PASS evidence is upstream.

## Evidence and delivery state

Identified independently by GPT-6.1 Sol xhigh worker A and confirmed by root while reviewing #1115 on 2026-10-01. Current relevant locations: `checked_write`, `read`, `write`/`write_stereo`, `write_read`/`write_read2`/`write_read_stereo`, and `ArenaLeaseSetBuilder::finish`. Representative consumers are in `crates/graph/src/runtime.rs`. Formal release/Miri reproduction and the API ruling remain pending.
