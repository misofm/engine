# Effect banks: a dual block entered collapsed renders over the frozen right channel

Tracker: #2258, item FX-4 ("`collapsed` is mirrored in 10 effect crates with no guard"). Found by
the implicit-state-machine audit of 2026-11-06.

## Problem
Nine launch effect banks implement the mono-collapse trio (`process_bank_mono`,
`desymmetrize_channels`, `supports_mono_collapse`): the compressor, de-esser, dynamic EQ,
gate/expander, multiband compressor, parametric EQ, soft clip, transient shaper and true-peak
limiter. Each keeps its own loose `collapsed: bool` (base `900c3d24c`: compressor `lib.rs:671`,
de-esser `:520`, dynamic EQ `:661`, gate `:526`, multiband `:1073`, EQ `:3450`, soft clip `:1228`,
transient shaper `:908`, limiter `:4282`). The one-plane body sets it and freezes the right
channel's state; only `desymmetrize_channels` copies the left channel onto the right and clears it.
Rest scans and clock advances read the left channel alone while it is set.

No dual body reads the flag. A dual block on a bank whose flag is set renders the right channel from
state frozen at the collapse (stale envelopes, filter integrators, delay lines, ramps), and its rest
scan still reads the left channel alone. The contract (`effect-contract` `process_bank_mono`)
makes the disengage copy the caller's obligation; nothing in the bank holds it.

**Reachability.** Not reachable from the engine today. The only production caller that
collapses is `rack::BankChain`, whose explicit `CollapseMode` (#2274) leaves `Collapsed` only
through `disengage_collapse`, which desymmetrizes every prefix stage first (`rack/src/lib.rs`
~4511, ~4797); `disengage_for_carry` and `swap_lanes` take the same boundary. `graph-compiler`'s
wrapper forwards. It is reachable through the bank's own public surface, the one the contract
defines: `process_bank_mono` and then `process_bank`. The regression test below drives that
surface through the launch registry.

`builtins::InputStage` already solved the same problem for the input section (#2274): its dual
body takes the disengage copy itself when entered collapsed.

## Decision
Make the stale read unreachable by construction, with the rule in one place, as #2320 did for the
silent claim:

- `effect_runtime::collapse::MonoCollapse` (private word) and the `CollapsibleChannels` owner trait
  (`mono_collapse()`, `copy_left_channel_onto_right()`).
- `MonoCollapse::enter::<true>` at the top of each one-plane body engages the record.
- `MonoCollapse::disengage` is the disengage copy: the owner's copy, then the record cleared. It is
  the only way to clear the record, so the record can never be cleared with the right channel stale.
- `MonoCollapse::enter::<false>` at the top of each dual body takes that copy when the record is
  engaged. From the chain, which has already copied, this is one untaken branch per bank block.
- `engaged()` replaces every read of the old flag (rest scans, clocks, the limiter's relink tick).
- The contract text on `process_bank_mono` and `desymmetrize_channels` records that a dual block
  entered collapsed takes the copy itself.

Self-repair rather than `debug_assert!(!collapsed)`, as the tracker first proposed: the copy is
always sound on a collapsed bank (the counterfactual right channel is the left one, the contract's
own argument), so the dual block renders exactly what a never-collapsed run renders, in release as
in debug. An assertion would leave release builds rendering from the frozen state, and would copy
the same rule ten times.

## Non-goals
`builtins::InputStage` (already self-repairing, #2274) and the chain's `CollapseMode` (GR-4). The
other entry points that write right-channel state while collapsed (reset, restore, lane swap); the
chain disengages before each of them today. Any change to what a collapsed body computes.

## Gates
- A regression test red on base: `crates/effect-compiler/tests/collapse_dual_entry.rs`, one row
  per banking launch effect, plus a set check that every banking effect has a row.
- Mutation: removing one effect's dual entry turns that row red.
- A one-time release-mode no-bit-moved comparison against base, as evidence (not a committed test).
- `cargo test --locked` for the touched crates, clippy `-D warnings`, `cargo fmt --check`, rustdoc
  `-D warnings`, `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)

Base: `main` @ `900c3d24c`.

(filled in below as the attempt proceeds)
