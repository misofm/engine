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

**Red on base.** `collapse_dual_entry.rs`, with the base sources restored under it: 9 of 9 rows
red, `every_banking_effect_has_a_row` green.

```
miso.compressor block 8: the right plane of a dual block entered collapsed
miso.de-esser block 8: the right plane of a dual block entered collapsed
miso.dynamic-eq block 8: the right plane of a dual block entered collapsed
miso.gate-expander block 8: lane 0's state payload differs at byte Some(169)
miso.multiband-compressor block 8: the right plane of a dual block entered collapsed
miso.parametric-eq block 8: the right plane of a dual block entered collapsed
miso.soft-clip block 8: the right plane of a dual block entered collapsed
miso.transient-shaper block 8: the right plane of a dual block entered collapsed
miso.true-peak-limiter block 8: the right plane of a dual block entered collapsed
test result: FAILED. 1 passed; 9 failed
```

Each row also checks that the fixture can fail (the frozen right channel differs from the dual
run's at the boundary). The gate needed a loud-then-quiet schedule across the boundary for that.

**Test value.** Which defect turns it red that no existing test catches: an effect's dual body that
does not take the disengage copy when entered collapsed, so it renders the right channel from the
state the collapse froze. Every existing collapse test desymmetrizes first, as the chain does.

**Mutations** (each run once, tree restored and touched between runs):

| mutation | result |
|---|---|
| compressor: `enter::<false>` removed from `render` | `dual_entry_compressor` red, 9 others green |
| gate: `enter::<MONO>` reduced to `if MONO { enter::<true> }` | `dual_entry_gate_expander` red alone |
| limiter: `enter::<false>` removed from `process_block` | `dual_entry_true_peak_limiter` red alone |
| parametric EQ: `enter::<false>` removed from `render` | `dual_entry_parametric_eq` red alone |
| `MonoCollapse::disengage` clears the record without the copy | all 9 rows red |

**Superseded test deleted.** `true-peak-limiter` `tests::a_collapsed_block_unlinks_the_pair_without_desymmetrize`
(the 990-M4 gate) built exactly the sequence this issue closes off and asserted the pair stays
unlinked. The dual entry now re-establishes the link record through the copy, so the sequence no
longer exists and the test went red. Deleted; `tests/MUTATIONS.md` records why M4 is now equivalent
on every sequence. The `unlink` in `process_block_mono` stays as defence.

**No bit moved (release, one-time, not committed).** A scratch test rendered each of the nine
banks through the registry over four configurations (symmetric and per-channel-asymmetric
parameters, two seeds each): 60 runs of dual blocks (equal and unequal planes), collapsed blocks
each run ended by `desymmetrize_channels`, silent blocks and whole-bank `advance_clocks` skips, odd
block lengths. It hashed every output plane, report, rest mask and state payload, plus 200 blocks of
the scalar instance. Base `900c3d24c` and the fix gave the same FNV-64 digest for every effect:
compressor `54d386345954b9d8`, de-esser `ebf687d7db0664a8`, dynamic EQ `b0e9dd1d046c0eed`, gate
`5674274b85e50f84`, multiband `c98f1d5b4f490aef`, EQ `f219361339ac1345`, soft clip
`13d793483f268c16`, transient shaper `32216b5f9eee54aa`, limiter `97bb533261ad1dc5`.

**Gates** (local, x86-64 AVX2 backend): `cargo test --locked` for effect-runtime, effect-contract,
the nine effect crates and effect-compiler: green. `cargo clippy --locked --all-targets -D warnings`
on those crates: green. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`: green.
`cargo fmt --check`: green. `scripts/check-workspace-policy.sh`: ok. `realtime_policy`: ok (298
marked regions in 48 files). Render cost: one predictable branch per bank block on the dual path.

**Follow-ups (not in scope).** A whole-bank `reset(FullToDefaults)` while collapsed leaves the
record engaged, so a later copy (the chain's or the dual entry's) would write the left channel's
defaults over an asymmetric right one. Not reachable: the chain only calls
`reset_lane(DiscontinuityKeepParameters)` on a bound bank, which writes both channels alike.
`builtins::InputStage` clears its flag on reset (#2274); the effects could do the same.
