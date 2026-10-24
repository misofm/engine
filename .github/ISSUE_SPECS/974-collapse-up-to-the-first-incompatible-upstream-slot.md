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


## Evidence and decision record (attempt 1, 2026-10-24)

* Scope note accepted (root): `collapse_prefix_of` takes the leading run of upstream slots with a
  one-plane body; clauses 1, 2 and 4 are unchanged. The decision witness and the agreement
  maintenance are conjoined over `..collapse_prefix`; the evidence helpers (`lane_symmetry`,
  `all_lanes_symmetric`, `active_lane_eligibility`, `symmetry_counters`) stay whole-chain and say
  so. Since #975 the first slot after the prefix takes the seam through `process_from_left`; an
  incompatible upstream slot uses the default (copy, then `process`), counted in `seam_copies`.
  Authorized paths narrowed to `crates/rack/src/lib.rs`, `crates/rack/tests/mono_reengage.rs`,
  `crates/rack/tests/MUTATIONS.md` and this spec.
* Gates: `mono_reengage.rs` `a_collapsible_prefix_before_an_incompatible_slot_collapses_exactly`
  (`[collapsible, incompatible with L != R words, seam]`, 64 blocks, forced-off window 24..40:
  48 collapses, transitions `[1, 1, 0]`, planes bit-identical to the forced-off arm); the lib test
  `only_a_seam_suffix_over_a_collapsible_prefix_can_collapse` adds `[upstream, PassThrough, seam]`
  -> `1` and `[PassThrough, upstream, seam]` -> `0`. Mutations 974-R1..R3 in
  `crates/rack/tests/MUTATIONS.md`.
* `prepared_slot_dispatch_uses_constant_activity_checks` now counts 9 activity queries, not 10:
  a collapsed block publishes the prefix stage's witness only, not the seam-side slot's.
* Class A, PR evidence: a one-off digest of the 16 console-workload rows (`WORKLOADS` plus
  `DRIVER_FED_WORKLOADS`, `PlanConfig::BASELINE`, 64 blocks, release) is identical at base
  `3b5740cdd` and at this change. No committed digest test.
