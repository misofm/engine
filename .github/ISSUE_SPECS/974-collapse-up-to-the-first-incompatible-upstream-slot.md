# Collapse up to the first incompatible upstream slot


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A. Base `6ca203f8`. Lower priority than the per-effect
mono-body issues; it matters for chains that keep an incompatible or asymmetric slot after them.

## Problem

`BankChain::collapse_prefix_of` (`crates/rack/src/lib.rs:1919-1937`) returns `0` (never collapse)
unless **every** upstream-of-seam slot supports a one-plane body, and the per-block decision
conjoins the witness of **every** slot (`lane_symmetry_bank`, `:2003`). One slot that cannot run
one plane, or one slot whose L/R words differ, therefore keeps the whole chain dual, including the
input builtins and the effects before it, which could have run one plane exactly.

## Outcome

The collapsed prefix is the **leading** run of upstream slots that support a one-plane body. The
seam copy (`:2400-2401`) moves to the end of that run; every later slot, upstream or seam-side,
runs its dual body on the duplicated planes. The per-block witness is conjoined over the prefix
slots only; disengage (`disengage_collapse`, `:2438`) and the M3 proof (`channels_agree`) already
cover only the prefix. A chain whose first upstream slot is incompatible keeps prefix `0`.

Exactness argument (put it in the doc comment): prefix slots see bit-identical planes and
symmetric words, so their one-plane body is exact by the existing contract; the copy makes the
later slots' inputs bit-identical to the dual run's, and they run the dual body, so every later word
is the dual run's.

## Read first

`crates/rack/src/lib.rs:1651-1701` (the collapse), `:1897-1937` (`collapse_prefix_of` and its four
clauses), `:2258-2420` (`run_with_input`), `:2424-2462` (`disengage_collapse`), and the rack's own
mono tests (`crates/rack/tests/mono_reengage.rs`, the hand-built stages in the lib tests).

## Authorized paths

`crates/rack/src/lib.rs`, `crates/rack/tests/`, this issue's spec.

## Gates

* Hand-built chain `[mono-capable, incompatible, seam-side]`: collapses (prefix `1`), and its planes
  are bit-identical to the forced-off run for 64 blocks, with an incompatible slot whose two
  channels' words differ.
* `[incompatible, mono-capable, seam-side]` still never collapses.
* A disengage and re-engage across the new seam renders the never-collapsed bits
  (`mono_reengage.rs` pattern).
* `collapse_prefix_of` clause 2 (a seam-side-only chain is vacuous) and clause 4 (lane agreement)
  unchanged; `chain_shape` counters on every standing row unchanged.
* `cargo test -p rack`, `-p console-workload --test chain_shape`, fmt, clippy `-D warnings`.

## Non-goals

The planner's pool class (today a track whose later slot is asymmetric or has no symmetry claim
pools as stereo, so this change reaches console sessions only once the class is computed over the
collapsible prefix; separate issue); a per-block moving seam; cross-chain collapse.

