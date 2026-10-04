# #1276 attempt 1 verdict: FAIL

One MAJOR, four MINORs and four NITs. The lane copy, the drain, the all-or-nothing check, the D1
rule and the realtime proof are sound, and every gate passes. The one MAJOR is in the D5
agreement inheritance. It reads a predecessor chain's `collapse_channels_agree` flag, but rack
maintains that flag only on a chain that can collapse. Inherited from a chain that cannot
collapse, the flag is always `true`. So a successor can collapse a carried lane whose two channels
hold different state, and that moves output bits. Probes showed it at both widths, on one path
with no live record that the C ABI can reach today. A one-line fix closes it; I checked the fix
against the probes and the suite.

- **Commits reviewed:** `807b48547` (implementation, parent `0297efa8c`) and `f41388939` (attempt
  record). `c14fde0ce` (#1274 follow-ups) is in the tree but outside this review. All three come
  from `/home/bl/misofm/wt-swap`, exported with `git archive f41388939` to
  `/tmp/claude-1002/v1276/attempt1/`. Every gate below ran on that tree.
- **Scope:** seven files, all on the slice's authorized list. No capi file or other shared touch
  point is edited.
- **Hygiene:** exact paths, attribution lines present, no artifacts.

## Gates re-run on the export

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (58 marked regions in 16 files) / ok |
| Focused: `cargo test --locked -p builtins -p builtins-compiler -p graph -p rack -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support` | exit 0, no failures; `successor_swap` 24/24 including both widths (avx2 build, so the `Simd8` rows ran) |
| `cargo test -p graph --features graph/test-support,engine/realtime-audit --test rt11_swap_carry_alloc` | 1/1 |
| `cargo test --locked -p capi` | 49 passed, 0 failed |
| `cargo test --locked -p console-workload` | 66 passed, 0 failed (static digests unmoved) |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 0 allocations, 0 deallocations, 0 syscalls, 0 violations |
| `trace-builtins-audit.sh` / `trace-builtins-graph-audit.sh` | PASS / PASS |
| `check-builtins-fixtures.sh . target/release/audit` / `test-builtins-fixtures.sh` | ok (50 files) / ok |
| `check-capi-abi.sh` | ok (shared and static) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` | all exit 0; graphSessionPlusPlanBytes 29794 of 35648, sourceTotalBytes 3358 of 3648 |
| `check-cross-targets.sh` | PASS; only the known #1018 iOS `memset_pattern16` expected failures |

ARTIFACT CHANGED is confirmed. The base `0297efa8c` builds `751122a9…` (2,870,096 B), which matches
the attempt record. `f41388939` builds `17429f47…` (2,914,664 B). That tree includes
`c14fde0ce`, so it is about 69 B larger than the implementer's `807b48547` module (2,914,595 B).
Where the bytes go: see MINOR-4.

## Test value: one sentence per new test (mutations run by me unless marked)

- `filtered_strips_keep_their_state_across_a_swap_at_{eight,four}_lanes`: red when a carried input
  lane restarts at rest or lands on the wrong lane. My run of the implementer's M1 (skip the input
  moves) turned both widths red, plus every other #1276 gap-free test.
- `a_pending_trim_record_survives_the_swap_at_{eight,four}_lanes`: red when the carry exports a lane
  without draining the predecessor's queue first. My M3 (`drain_for_carry` drains nothing) turned
  both widths red.
- `a_high_pass_ramp_in_flight_finishes_after_the_swap_at_{eight,four}_lanes`: red when the
  coefficient target, step or countdown is not carried. That is the implementer's M7; my M1 also
  turns it red.
- `collapsed_strips_keep_collapsing_across_a_swap_at_{eight,four}_lanes`:
  - The forced-dual arm is red when the disengage copy is skipped (implementer's M5).
  - The collapsing arm is red when the inherited flag is always cleared, but only through the
    proof counter, not through bits (implementer's M4). That counter is the claim (collapse
    savings kept), so it is acceptable.
  - It does **not** defend the AND's `false` arm; see MAJOR-1.
- `a_strip_whose_filter_changed_starts_at_rest`: red when a section whose `hpf_hz` changed is
  carried (implementer's M6). It defends only that one field; see MINOR-1.
- `the_input_carry_allocates_and_frees_nothing`: red when the carry or the first successor block
  allocates or frees (implementer's M8).
- `a_one_channel_record_keeps_its_chain_dual_after_the_swap`: red when the lane's `LIVE` symmetry
  term is not copied (implementer's M9). My M3 also turns it red.
- `a_bad_input_move_is_refused_whole`: red when the all-or-nothing pre-check is dropped or install
  accepts a padding lane (implementer's M10 and M11).
- The two byte assertions changed in the #1272 tests now count the two carried input sections. They
  are written with `size_of::<GraphLaneMove>()`, so they state a move count, which is the claim, and
  are not a digest. Acceptable.

## Findings

### MAJOR-1: the inherited agreement flag comes from chains that never maintain it, so a successor collapses a carried lane whose channels disagree

The code:

- `crates/graph/src/runtime.rs:4189` reads `let agree = chain.collapse_channels_agree();`.
- `crates/rack/src/lib.rs:2817` (`inherit_channel_agreement`) ANDs that value into the successor
  chain.
- Rack maintains the flag only when `can_collapse()` holds (`rack/src/lib.rs:2515-2524`: the
  maintenance step is guarded by `self.can_collapse()`). A chain that is not armed keeps `true`
  from bind for its whole life, whatever its channels hold.
- A chain is unarmed when any lane's track is not in the mono-source set. That includes a
  stereo-source neighbour and a mono-source track with an asymmetric input delay
  (`runtime.rs:2661-2674`, `prepare.rs:1763-1769`).

So a lane whose L and R input-section state diverged can sit in an unarmed predecessor chain
whose flag says "agree". A transaction then pools that lane into an armed successor chain. The
successor inherits `true`, its witness is symmetric (equal words, `LIVE` intact), and it collapses
on the swap block: the right channel is replaced by the left channel's processing. Collapse is
class A ("this moves no bit", D5; AGENTS.md: an optimisation may couple cost, never bits). Here it
moves bits.

Probes are in `/tmp/claude-1002/v1276/attempt1/logs/probe_1276.rs` (copy it into
`crates/host-core/tests/` to run). Both widths:

- **Live-record path.** All-mono filtered session. `eq6` is prepared with trim R = L − 6 dB, so it
  pools Stereo next to `eq5`, which reads two channels; that chain is unarmed. A `Both` trim record
  equalises `eq6` two blocks before the swap. `B` = committed model + muted track sorting first.
  - The forced-dual successor equals A's no-swap continuation bit for bit.
  - The collapsing successor differs from block 6: max |Δ| = 1.3e-3 on a peak of 8.25.
  - With the record five blocks earlier: max |Δ| = 1.1e-5, still not bit-exact.
  - Successor collapse counters show the extra chain collapsing (`[18, 3]` against the forced-dual
    `[0, 3]` at W4).
- **No live record; reachable on the C ABI today.** `eq6` has `delay_samples` R = 37 in A, so no
  chain holding it is armed and its HPF/LPF integrators diverge. B makes the delay symmetric and
  adds a muted track. Delay is not one of D1's compared values, so the section carries.
  - On a C ABI structural transaction, `control.rs:761-767` prepares this successor exactly so.
  - Collapsing against forced-dual successor: first difference at block 6, max |Δ| = 0.50 on a
    peak of 8.3.
- **Control case.** The same record path with an *armed* predecessor chain (no stereo neighbour)
  is correct. The predecessor's flag is maintained, so it is `false`.

The AND's `false` arm is also untested. Mutant M-a, where `inherit_channel_agreement` ignores
`agree`, passes all 24 `successor_swap` tests; only my armed-chain probe turns red.

**Fix.**

- At `runtime.rs:4189`, use `let agree = chain.can_collapse() && chain.collapse_channels_agree();`.
  Better, add a rack accessor that answers `false` for a chain that never maintains the flag, so
  the premise lives in rack.
- With that one-line change, all 12 of my probes and all 24 `successor_swap` tests pass. Gate 3 is
  unaffected, because its predecessor chains are armed.
- Add two regression tests at both widths:
  - an unarmed predecessor holding a channel-diverged mono lane (the delay case needs no live
    queue), asserting that the collapsing successor equals the forced-dual successor bit for bit;
  - the armed-and-cleared case, so the AND's `false` arm is defended.
- Update the flag's documentation:
  - the `BankChain` "Coming back" clause list (`rack/src/lib.rs:1847-1860`) gains a fifth clause,
    the carry's AND, with its premise;
  - `collapse_channels_agree()` (`:2783-2791`), still documented as "Evidence only", now feeds
    production.

### MINOR-1: D1's comparator is defended for one field of four, and for none of the kind clauses

Each of these mutants passes all 24 `successor_swap` tests:

- Mc, Mc2, Mc3: delete the `trim_db`, `lpf_hz` or `polarity_invert` comparison in
  `same_input_section` (`crates/host-core/src/prepare.rs:1808`).
- Md: replace `row.live == live_input` (`:1160`) with `true`.
- Mk: delete `same_strip_kind`.

A comparator that forgets trim keeps the old trim on a carried strip after an acknowledged
structural edit, which loses an acked edit. The code is correct today; the gate is too narrow.

**Fix:** extend `a_strip_whose_filter_changed_starts_at_rest` to change trim, lpf and polarity on
three other strips in the same transaction, each asserted at rest. Add one control-kind-mismatch
case.

### MINOR-2: `drain_for_carry` swallows a drain error and silently drops the rest of the bank's records

`crates/builtins-compiler/src/lib.rs:572-573` discards the error. `drain_controls` returns at the
first record whose apply fails, so the remaining records of that lane, and every later lane of the
bank, stay in a queue that never renders again. Admission validation makes this unreachable today.
Still, it is a silent drop path on an acked record, which the acked-batch question rejects.

**Fix:** a carry drain that applies every record present at entry and keeps going past a failed
apply (count it, or `debug_assert!`), so every record is consumed.

### MINOR-3: doc drift on the flag

The flag's documentation does not mention the carry; this is covered under MAJOR-1's fix.

### MINOR-4: the browser module grows by +44.5 KB, and about two thirds of it is prepare-time generic plumbing

I measured the growth with `twiggy diff` between the base and candidate named twins
(`/tmp/claude-1002/v1276/attempt1/logs/twiggy-diff.json`). Net code growth, excluding the
`name` section (+16.4 KB, not shipped):

| Part | Growth |
|---|---|
| Sort monomorphizations | about +23.6 KB |
| `BTreeMap` collects | about +6.4 KB |
| host-core prepare closures | about +5.2 KB |
| `export_lane` / `import_lane` | +4.8 KB |
| graph | +3.5 KB, of which `adopt_predecessor` is +1.7 KB |
| builtins-compiler | +1.0 KB |

The sort and collect instantiations are new: `(&str, StripRef)`, `(&str, GraphLaneLocation)`,
`InputSectionInventoryRow`, `GraphLaneMove`.

**Judgement against AGENTS.md "no code for modes production never runs": not a violation.** The C
ABI runs this carry in production on this branch (`control.rs:761-767`), P13 makes it portable core
code, and B2 is the planned browser caller. The browser does pay two costs it does not use: the
inventory build on every prepare (`builtin_input_lanes`, the row sort), and about 30 KB of join
instantiations.

**Fix (follow-up acceptable):**

- Look the committed strips up by binary search; the normalized model is already sorted by ID.
- Push inventory rows in strip order instead of re-sorting.
- Avoid the `BTreeMap` collects.

### NITs

- **NIT-1.** `prepare.rs:1692` weakened `debug_assert_eq!` to `<=`. In a shipped plan every strip
  banks, so `moves.len() == carried_inputs.len()` holds. Asserting it would catch a join that
  silently drops a carried strip (that strip would restart at rest).
- **NIT-2.** Several render-thread helpers sit outside `REALTIME_POLICY` regions:
  - `builtin_processor_mut` and `input_lane_mut` (`runtime.rs:4071`, `:4083`);
  - `InputStage` / `BuiltinInputBank` `export_lane` / `import_lane`;
  - `BankChain::disengage_for_carry`, `inherit_channel_agreement`, `slot_stage_mut`.

  There is precedent: `disengage_collapse` is unmarked too. The zero-allocation counters cover
  them; marking them would let the policy scan see them.
- **NIT-3.** The cache re-derivations in `import_lane` are untested:
  - Mutant Mj drops `refresh_filter_plan`; Ms drops `refresh_channel_symmetry_lane`. Both survive
    every test and probe, including my ramp-to-disabled probe.
  - Mj looks behaviour-neutral: the ramping body ignores the plan, and a settled carried lane
    holds its successor's own coefficients.
  - The debug assertion in `lane_channel_symmetry` backs Ms.
  - Keep both refreshes as defence, and say so in a comment.
- **NIT-4 (concern 3, forward note).** On this branch no host can admit an input record to a
  predecessor after a successor is published:
  - the C ABI prepares no strip controls;
  - only `hosts/host-web` builds a `TrackInputRecord`, and it has no successor path.

  The first slice that gives the C ABI input live edits, or B2/B3 for the browser, must route
  admission to the newest plan once it is published (P5, bullet 2). A record pushed to the
  predecessor after the carry's `available_at_entry` snapshot is otherwise lost with the retired
  plan.

## Judged and found sound

- **Drain.** It is bounded: `available_at_entry` snapshots at most the queue capacity per lane,
  each apply is O(1) and allocation-free, and each predecessor bank drains once (moves sorted by
  location). It cannot double-apply: the predecessor never renders again, its consumer is not
  inherited, and the successor drains only its own queue. #1253 is not needed for the input
  section, as the implementer says.
- **Disengage, then drain.** Disengaging and then draining commutes with the normal
  drain-witness-disengage order for the input stage. `desymmetrize` copies only integrators, and
  the drain writes only coefficient and ramp words.
- **`import_lane` coverage.** It copies every per-lane word of `InputStage`. It leaves
  `filter_initial` (the successor's own reset endpoint) and the telemetry counters, which is
  correct.
- **All or nothing.** It holds: `can_carry_input_lanes` runs before any source or lane moves, and
  `adopt_input_lane` cannot fail after the check passes.
- **Submix input sections carry.** The 64-track, 10-submix sends fixture records 74 inventory
  rows. My probe on it (effects stripped, per-channel filters on every track and submix, a muted
  track added) is bit-identical gap-free at W4 and W8.
- **`as_any_mut` defaulted to `None`.** It is safe: a type that does not opt in reports no carried
  lanes, so it never enters the inventory and cannot cause a mismatch.

Export `target/` deleted after this verdict; sources, logs, the probe file and the mutation logs
(`logs/mut/`) are kept.
