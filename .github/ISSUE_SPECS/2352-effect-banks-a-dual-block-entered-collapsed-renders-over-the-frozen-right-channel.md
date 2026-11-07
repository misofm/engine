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
- `MonoCollapse::disengage` is the disengage copy: the owner's copy, then the record cleared. It is
  the only way to clear the record, so the record can never be cleared with the right channel stale.
- `MonoCollapse::enter::<MONO>` at the top of each bank body, after the shape guard and **before**
  the block's automation, admission or silent-claim work (attempt 2): a collapsed block engages the
  record, and a dual block entered collapsed takes the copy there, so the block's own right-channel
  spans land on live state. The parametric EQ and the multiband already entered there (the EQ admits
  no bank automation); the limiter's core bodies keep their own entry for the core-level tests.
- `MonoCollapse::enter::<false>` at the top of every bank entry that writes channel state (attempt
  2): `reset`, `reset_lane`, `apply_prepared_target_lane`, `retarget_link`,
  `restore_track_state_payload`, `copy_lane_from`, `swap_lanes_direct` (on both banks) and
  `exchange_lane_defaults`. A write made while engaged therefore lands on live state and is never
  overwritten by a later copy, the `builtins::InputStage` rule (#2274).
- A whole-bank reset takes the copy rather than calling a `clear()`: a reset that keeps parameters
  keeps each channel's ramps and targets, and the right channel's are the frozen ones, so clearing
  after it would leave them. One copy at a reset costs nothing that matters.
- `engaged()` replaces every read of the old flag (rest scans, clocks, the limiter's relink tick).
- The contract text on `process_bank_mono` and `desymmetrize_channels` records which entries take
  the copy, and that a `&self` read (a snapshot) cannot.

From the chain, the dual entry is one untaken branch per bank block (it has already copied). A
writer the chain calls on a still-collapsed bank (a lane reset, a target, a link retarget, all
before the block's mode is decided) now takes the copy, which the next collapsed block's frozen
right channel never reads and the eventual disengage copy overwrites: no published bit moves (the
no-bit-moved probe below calls these writers on collapsed runs).

Self-repair rather than `debug_assert!(!collapsed)`, as the tracker first proposed: the copy is
always sound on a collapsed bank (the counterfactual right channel is the left one, the contract's
own argument), so the dual block renders exactly what a never-collapsed run renders, in release as
in debug. An assertion would leave release builds rendering from the frozen state, and would copy
the same rule ten times.

## Non-goals
`builtins::InputStage` (already self-repairing, #2274) and the chain's `CollapseMode` (GR-4). Any
change to what a collapsed body computes. A `&self` read of a collapsed bank (a snapshot, a resident
observation) still reads the frozen right channel, as the contract documents; it cannot take the
copy.

## Gates
- A regression test red on base: `crates/effect-compiler/tests/collapse_dual_entry.rs`, one row
  per banking launch effect, plus a set check that every banking effect has a row.
- Mutation: removing one effect's dual entry, putting it back after the automation, or removing
  one writer's entry turns that row red.
- Rows for a right-channel span on the first dual block and a restore while engaged (attempt 2),
  and for a direct lane copy, a direct swap (either side), a reset to asymmetric defaults and a
  right-channel prepared target (attempt 3). Each writer either has the row that turns red, or is
  recorded as an equivalent mutant with its reason (attempt 3 evidence).
- A one-time release-mode no-bit-moved comparison against base, as evidence (not a committed test).
- `cargo test --locked` for the touched crates, clippy `-D warnings`, `cargo fmt --check`, rustdoc
  `-D warnings`, `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Attempt record

- **Attempt 1** (`ccfe090bd`): **FAIL**, two MAJOR findings. (1) In seven effects the repair copy
  ran in the render body, after the block's automation had written both channels, so a
  right-channel span in the first dual block was overwritten: one `ParameterChannel::Right` span on
  lane 0 was red against a never-collapsed run in the compressor, de-esser, dynamic EQ, gate, soft
  clip, transient shaper and limiter. (2) The copy waited for the next dual entry, so a per-channel
  write made while engaged was overwritten: 8 collapsed blocks, an asymmetric
  `restore_track_state_payload`, then `process_bank` was red in all nine; base got that lane right.
  Minor: an `impl` between `LaneRestore`'s doc comment and the struct in the gate. The reviewer
  agreed with self-repair and with deleting the limiter's M4 gate.
- **Attempt 2** (`047a6a9a6`): **FAIL**, one MAJOR finding. The fix was judged correct, along with
  both departures (copy-on-reset rather than `clear()`, which would be wrong for a reset that keeps
  parameters; the parametric-EQ test edits, since that raw right section is not product-observable).
  The failure was coverage. Deleting the compressor's `copy_lane_from` or `swap_lanes_direct` entry,
  or the EQ's `apply_prepared_target_lane` entry, left every suite green, so the gate "removing one
  writer's entry turns that row red" was not met. Minor findings: the `Maximum` rows' comment
  claimed to cover the limiter's `link.restate(true)`, but removing it left the row green; and the
  chain-path cost was not recorded.
- **Attempt 3**: below.

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

## Evidence (attempt 2)

Base `900c3d24c`. `origin/main` @ `707dc52c2` merged first (#2350, test-only rack). Effect
sources are unchanged by the merge.

**Test.** `collapse_dual_entry.rs` now runs three boundaries per row: `Dual` (a plain dual
block), `RightSpan` (the first dual block carries one `ParameterChannel::Right` span on every lane,
at its first sample) and `Restore` (lane 0 is restored, before the first dual block, from a donor
whose two channels differ). Each runs at `DualMono`, and at `Maximum` too where the effect links
lanes: the compressor, de-esser, dynamic EQ, gate, multiband, transient shaper and limiter. The
reports are now compared as well.

**Red on base and on attempt 1** (the test run with older effect sources restored under it):

| sources | boundaries run | result |
|---|---|---|
| base `900c3d24c` | all | 9 of 9 red, all at `DualMono Dual block 8` |
| attempt 1 `ccfe090bd` | `RightSpan` only | 7 red: compressor, de-esser, dynamic EQ, gate, soft clip, transient shaper, limiter. The EQ and multiband are green, as the review found. |
| attempt 1 `ccfe090bd` | `Restore` only | 9 of 9 red |

**Mutations** (each run once, then restored and touched):

| mutation | result |
|---|---|
| compressor: `enter` moved after the automation loop | `dual_entry_compressor` red alone (`DualMono RightSpan block 8`) |
| gate: `restore_track_state_payload` loses its entry | `dual_entry_gate_expander` red alone (`DualMono Restore block 8`) |
| limiter: `restore_track_state_payload` loses its entry | `dual_entry_true_peak_limiter` red alone (`DualMono Restore block 8`) |

There is no `clear()` row: a whole-bank reset takes the copy (see Decision), so there is no
`clear()` to drop.

**Parametric-EQ tests that read a collapsed bank's right section.** The EQ's padded-bank scenario
and four `tests/bank.rs` transcripts render `process_bank_mono` over tracks whose two channels are
designed apart. They then compare a collapsed bank's raw right section across bank placements and
against bind, or restore a payload snapshotted mid-collapse. That right section is what the
collapse froze, or the left one once a writer took the copy, so with writer copies it depends on
which bank a writer reached. No rendered bit depends on it. The member checks of the same scenario
already judged only `..2` sections in mono. The padded-lane check now does the same, and
`tests/bank.rs` gains `collapsed_view`, which judges and restores a collapsed track by its left
section (the counterfactual right). These were the only failures across the touched crates.

**No bit moved (release, one-time, not committed).** The scratch probe was extended on two
points:
- inside collapsed runs it calls `reset_lane(DiscontinuityKeepParameters)` and `retarget_link`
  (the writers the chain calls on a collapsed bank), hashing their answers;
- it adds two `Maximum` configurations for the linking effects.

Base `900c3d24c` and this head give the same FNV-64 for all nine banks: compressor
`4ef35951ad8e98f0`, de-esser `b8925818f8838b22`, dynamic EQ `d110117e28f781e6`, gate
`c0183815fc89581a`, multiband `85d2ad431836f004`, EQ `ea65b568f9488d03`, soft clip
`171255ed03ffb4d1`, transient shaper `99fcb0362d4d88a1`, limiter `f67f548881bb8886`.

**Gates** (local, AVX2, one crate at a time):
- `cargo test --locked --no-fail-fast`: green for effect-runtime, effect-contract, the nine
  effect crates, effect-compiler, graph-compiler and host-core. The parametric EQ is green after the
  test change above.
- `cargo clippy --locked --all-targets -D warnings` on the touched crates: green.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`: green.
- `cargo fmt --check`: green.
- `scripts/check-workspace-policy.sh`: ok.
- `realtime_policy`: ok (298 regions in 48 files).

**Minor findings.** The gate's `impl CollapsibleChannels` now sits above `LaneRestore`'s doc
comment. The contract text and the `collapse` module doc now say which entries take the copy and
what `InputStage` does (dual body and lane import copy; reset clears).

## Evidence (attempt 3)

`origin/main` @ `8147ab6b8` merged first. That merge changed control-plane, host-web and the web
test script; no effect crate. No effect source changed in this attempt, only
`crates/effect-compiler/tests/collapse_dual_entry.rs`.

**New boundaries in the same table.** Each one is skipped as vacuous where its writer does not
apply, and asserted to apply where it must:
- `CopyLane` (`copy_lane_from` from a donor whose channels differ; the answer is `Some(false)`);
- `Swap` (`swap_lanes_direct` with that donor);
- `SwapAsPeer` (the same swap called on the donor, with this bank as the peer);
- `ResetToDefaults`: lane 0 takes asymmetric defaults through `exchange_lane_defaults` before the
  collapse. Defaults are no state word, so the collapse stays lawful. At the boundary the bank
  takes `reset(FullToDefaults)`.
- `RightTarget` (`apply_prepared_target_lane` with the targets that move one right-channel
  parameter; asserted to apply on the EQ).

`CopyLane`, `Swap` and `SwapAsPeer` apply to the compressor and the EQ, the only banks that
implement the direct lane paths.

**Per writer: the row that turns red, or why it is an equivalent mutant.** Mutations are recorded
in this attempt unless marked; each deletes one entry copy, then the tree is restored and touched.

| writer entry | covered by |
|---|---|
| dual entry in each bank body | `Dual` (base, 9 of 9) and `RightSpan` (attempt 1, 7 of 7). Mutations in attempts 1-2: compressor, gate, limiter, EQ. |
| `restore_track_state_payload` | `Restore`. Compressor, EQ (this attempt), gate, limiter (attempt 2): red. |
| `copy_lane_from` | `CopyLane`. Compressor and EQ: red. |
| `swap_lanes_direct` (own bank) | `Swap`. Compressor and EQ: red. |
| `swap_lanes_direct` (peer) | `SwapAsPeer`. Compressor and EQ: red. |
| `reset` | `ResetToDefaults`. Compressor, EQ, gate, limiter: red. |
| `apply_prepared_target_lane` | `RightTarget`. EQ, the one bank that applies prepared targets: red. |
| `exchange_lane_defaults` | **Equivalent mutant.** It changes no state word and no payload word (contract), so a copy before it moves nothing; compressor and EQ mutants green. Kept for uniformity. |
| `reset_lane(DiscontinuityKeepParameters)` | **Equivalent on a lawful collapse.** It writes both channels of the lane alike, from parameters the witness made equal; EQ mutant green. Kept for uniformity. |
| `retarget_link` | **Equivalent on a lawful collapse.** It moves both channels' link weights alike; compressor mutant green. Kept for uniformity. |

**The `Maximum` rows are gone.** They caught nothing. The limiter's `link.restate(true)` in
`copy_left_channel_onto_right` moves no bit, because the linked body is bit-identical to the
unlinked one, and that word only re-arms the fast path. Removing it leaves `collapse_dual_entry`
green and turns the limiter's own liveness gates red: `the_linked_body_engages_exactly_where_the_record_allows`
and `randomized_scenarios_match_the_current_frame_law`, which are 990-M14 in `tests/MUTATIONS.md`.
That claim is tested once, in the crate that owns it.

**Chain-path cost.** These figures are the reviewer's. `rack` calls three writers on a bank whose
chain is `Collapsed`:
- `reset_lane` (DKP) at rack ~2874, when a lane leaves settled bypass;
- `retarget_link` at ~2884, on a link switch;
- `apply_prepared_target_lane` at ~3111, when a target cell changes.

Each such call now takes the whole-bank copy if the bank is engaged. The first one disengages, and
the block's `process_bank_mono` engages again, so there is at most one copy per bank per block,
and only on blocks that carry such a control. The EQ's copy is about 4 KB at AVX2 width. It is
bit-exact: the no-bit-moved probe of attempt 2 calls `reset_lane` and `retarget_link` inside
collapsed runs and matches base for all nine banks.

**Gates** (local, AVX2):
- `cargo test --locked --no-fail-fast -p effect-compiler`: green.
- clippy `-D warnings` and rustdoc `-D warnings` on effect-compiler: green.
- `cargo fmt --check`: green.
- `scripts/check-workspace-policy.sh`: ok.
- `realtime_policy`: ok (298 regions in 48 files).
- No effect source changed since attempt 2, whose per-crate gates stand.

**Recorded follow-up (not in scope).** A snapshot of an engaged bank could write the
counterfactual right section, the left channel's, instead of the frozen one. That would remove the
`&self` exception in the contract and make a collapsed bank's snapshot independent of placement, so
the parametric EQ's `collapsed_view` would no longer be needed. It needs a per-effect check that the
payload's right section holds only words the copy covers.
