# #1202 *Carry every console slot on every submix strip*: Sol verdict, attempt 1

- Reviewed: `git diff 84d26a1d8 b0c7e29fb` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`): 30 files, +1341/-227.
- Binding: `AGENTS.md` (decision 12: console slots always bank, padded, on every target; a bypassed
  lane stays in its bank) and `.github/ISSUE_SPECS/1202-carry-every-console-slot-on-every-submix-strip.md`,
  including amendment deliverable 7 and the Attempt 1 record.
- How I ran it. I did not modify the worktree, the branch or GitHub. `gh issue view 1202`: OPEN,
  and the title matches the spec's H1. The worktree has uncommitted edits from other agents; I
  used only `git show` and `git archive`.
  - I exported `b0c7e29fb` to `/tmp/claude-1002/v1202/src` with my own `CARGO_TARGET_DIR`, and
    built debug with `CARGO_PROFILE_DEV_DEBUG=0`.
  - Mutations and experiments ran in a second export (`.../mut`) with its own target dir.
  - Before writing this, I `diff -r`'d the gate export against a fresh archive: IDENTICAL. I
    deleted all scratch afterwards.
- My proposed guard and tests are in `submix-verdicts/1202-attempt1-verifier-scratch.rs`.

## Verdict: PASS

There is no BLOCKER and no MAJOR.
- D1-D5 and the D7 amendment are present at the sites and in the shapes the spec freezes.
- Every gate I could run passes when I re-run it, and the numbers match Terra's record exactly.
- The re-pin is attributable to field 3 alone (proof below).
- Bus console lanes bank at their level, padded, at the native width. They also bank at a
  simulated native 4-lane width.

There are three MINOR findings and five NITs. MINOR-1 is a defence-in-depth gap that predates the
slice. This slice is the first to put a new console population behind it, so I recommend closing it
inside the K1 batch (about 20 lines, an authorized path, and the fix is proven below). None of the
three needs another attempt.

## Gates (re-run by me on `b0c7e29fb`, x86-64-v3, native W8)

| Gate | Command | Result |
|---|---|---|
| 1-3, 7, 8 | `host-core --test submix_strip` (test-support, realtime-audit) | 11/11 ok |
| 2 | `graph-compiler --test bank_levels every_console_group_binds_when_buses_carry_the_console` | ok. Counts below. |
| 4 | `session --test submix_strip` | ok (inside test-debug-a) |
| 5 | `protocol --test console_session_edits` | ok (inside test-debug-a) |
| 6 | `host-core --test randomized` | ok. `Reach { consoles: 12, refused: 0, armed_collapse_blocks: 54, live_records: 162, bus_console_entries: 11 }` |
| 6 | `bank_levels randomized_consoles_compile_bind_and_render_the_scalar_bits` | ok. Seeds 0..64: foreign refusals `[0, 11, 0]`, rendered 38, bus console lane at level >= 1 `[4 seeds, 15 lanes]` |
| 7 | host-web `live_controlled_boot_of_a_bus_with_an_effect_renders` | ok (inside test-debug-a, `host-web/test-support`) |
| 9 | test-debug-a workspace command (exact excludes and features from `qualification.yml`) | exit 0; 98 binaries, 1133 passed, 0 failed |
| 9 | test-debug-b command, then `conformance_fixtures -- --check` | exit 0 and exit 0; 145 binaries, 787 passed |
| 9 | `check-protocol-wasm-parity.sh` | `issue-005 Wasm golden parity: ok (simd128)` |
| 9 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| 9 | `check-graph-determinism.sh`; `graph_fixture -- --check` | PASS (100/100); exit 0 |
| 9 | `check-console-fixtures.sh target/release/session_validator`; `check-builtins-fixtures.sh . target/release/audit` | ok; ok (50 files) |
| 9 | `cargo fmt --all -- --check`; workspace clippy `--all-targets --all-features -D warnings` | both exit 0 |
| 9 | the five `check-`/`test-` policy pairs; `check-realtime-policy.sh` | all ok |
| 9 | `run-aarch64-tests.sh`'s no-silent-skip scan (its exact `rg --pcre2` pattern and roots) | no match (rg exit 1) |
| 9 | `run-aarch64-tests.sh debug` | **Not runnable here.** The host is x86_64 with no aarch64 runtime. See the simulation below and NIT-5. |
| extra | `session_validator validate --canonical worked-session.json` | All five stages PASS, and the canonical output is byte-identical to the file |

The commit touches only authorized paths. `docs/CONTROL_PROTOCOL_CONFORMANCE.md` gained a reason
clause beside the hash, as #1199 did before it.

## Adversarial checks

**Banking at the bus's level, padded (D3, decision 12).** Gate 2 at native W8, as
`(slot, class, level) -> [lanes, banks]`:

| Sessions | Track groups | Bus groups |
|---|---|---|
| 1 bus | `eq@2`, `comp@3`, `limiter@6`, each `[4,1]` | `@13/14/17`, each `[1,1]` |
| 3 buses | as above | `[3,1]` |
| 5 buses | as above | `[5,1]` |
| 9 buses | as above | `[9,2]` |
| 3 chained buses | as above | eq at levels 13/24/35 (the other slots follow), `[1,1]` each |

- Every row is `ceil(n/8)`, and no bank crosses a level (`cross_level_banks == 0`).
- Bypassed bus lanes are drawn (about 30 %) and are counted as members, so bypass stays in the bank.
- Every width that compiled rendered the scalar plan's bits.

**Simd4 refused on x86: the expected foreign-width behaviour, not a hidden defect.**
- The factories bind only the build's own width:
  - the EQ: `bind_bank(request, Backend::current().width())`;
  - the compressor: `Backend::current().width() != request.width.lanes()` gives `Ok(None)`;
  - the limiter likewise.
- `compiled_at` and `FOREIGN_CONSOLE_REFUSAL` are #1098's rule, already applied to every
  track-console test in `bank_levels.rs` (for example `the_sixty_four_track_console_less_one_eq_binds_at_every_width`).
- **Simulation (E5).** To check the aarch64/wasm path, I patched `Backend::current()` and
  `lane::Native` to `Simd4` under `avx2` in the scratch export. That makes 4 lanes the build's
  native width on x86 (factories, banks and `NATIVE` constants included). Then:
  - Gate 2 passes with `ceil(n/4)` per group: 5 buses `[5,2]`, 9 buses `[9,3]`, tracks `[4,1]`.
  - Simd4 compiles and Simd8 is the width refused with exactly `graph console.slot.unbanked`.
  - Also green at simulated W4: all of `bank_levels` (11 tests, probe `[0, 0, 11]`, bus reach
    `[4, 15]`), `host-core` `submix_strip` (12 tests, including my E3), `randomized`, and
    `collapse_arming`.
  - This is evidence for the logic, not for NEON or simd128 codegen. CI's `aarch64-debug` at the
    batch push is still required (NIT-5).

**D3's mixed group (E4, my test).**
- A fifth track `chu` is not routed to the bus. Its 11 inserts put its `post_insert` limiter at the
  bus limiter's level, 17, in the Stereo class.
- The plan then holds `("limiter","Stereo",17) -> [2,1]`: one padded bank holding a track lane
  and a bus lane.
- It binds and renders the scalar bits at W8 and at simulated W4. D3 holds, but no committed test
  forms such a group (MINOR-2).

**Live controls (E3, my test).**
- A console bank holding a live track lane (`u`) and a channel-less bus lane renders the same bits
  with live controls attached (no record pushed) as the plan prepared without them.
- Checked over 6 seeds; the bank forms at 13 track inserts.
- The bank stage already handles mixed `Some`/`None` lanes (padded lanes are `None`), so no defect.

**PDC (gate 3).**
- One `post_insert` limiter (486 samples at 48 kHz) is on every strip, bypassed.
- `output_latency == 972`, and exactly one inserted delay, on `t0-direct`, of 486.
- The impulse lands at sample 972 on both planes as `0.5` exactly (0.25 + 0.25).
- The numbers match the spec, and the doc's "latency grows with bus depth" paragraph states them.

**Re-pin legitimacy (field 3, canonical order).**
- In the scratch export I cleared the corpus submix's console entries (`submix.console.clear()`)
  and nothing else. The computed corpus hash is then `ca48855fd3a756b7`, the old pin. So the new
  pin `c0f6ecedbf50920a` comes exactly from the two field-3 entries. Moving the submix construction
  below the console changed no other byte.
- Field 3 is pinned structurally. **Mutation E2** (field 3 changed to 7) turns 7 tests red: both
  conformance frozen-corpus tests, `migrated_specs_are_sorted_unique_and_structurally_consistent`,
  the random submix round trip, the deep-fixture depth test, the B1b replay test and gate 5.
- `visit.rs` walks `console` between `builtins` and `inserts` (`[5 + n]` fields), and the
  key-order test pins `id, builtins, console, inserts, fader, pan|matrix`.
- `mandatory=true, repeated=true` is exactly the track's field 11.

**`Submix::unity(id, &console)` (D4).**
- It builds one `{slot, bypass: true, params: []}` per slot, in `slots()` order. That is the
  transparent default the spec freezes.
- The signal is unchanged; latency is still paid (decision 12). `bypass: false` with default
  parameters would run every console slot, which is VERIFY-1 MINOR-5's objection.
- The doc comment and `SESSION_SCHEMA_V1.md` both say the latency is paid. The unity test pins the
  order, the bypass and the empty params, and that it validates with and without a console.

**Bypass stays in the bank on a bus.**
- **Mutation E7** (mine): a bus console slot's lowered bypass never reaches the bank shunt
  (`control: None` for submix console entries in `effect-compiler` `prepare.rs`).
- Gate 1 goes RED: `seed 0: plane 0 sample 1776: bus -0.0038246962 != track-fed-its-sum -0.003822866`.
- Gate 3, the randomized differential and the bank-levels probe stay green. Gate 3's impulse sits
  far under the limiter ceiling, so a processed limiter still passes it exactly. Gate 3 pins
  latency, as its sentence says, not bypass.

**Realtime and portability.**
- No render-path source changed: the graph-compiler `src` diff is doc comments only, plus two
  `unity` call sites in unit tests.
- Bus console lanes use the existing bank stages. Gate 8 reads `allocations == frees ==
  reallocations == 0` after block 0, under both the render audit and the thread-scoped counters.
- There is no target-specific code, and no width returns early.

## Findings

### MINOR-1: the no-fallback guard cannot see a console chain the strip walk never collected

Precise statement, correcting the implementer's framing:
- `unbanked_console_slot` iterates `chains`, the map of every console chain the `strips()` walk
  *collected*, whether or not it became a candidate. It does not look only at candidates.
- **Mutation E1b** drops bus console chains from *candidacy* only (the `'chain` loop). The guard
  catches it: gate 2 is refused at Simd8 with `console.slot.unbanked`.
- **Mutation E1** drops them from *collection* instead (the walk skips submix console racks). The
  guard is then blind:
  - the compile is accepted at W8 and the bus console renders per node;
  - every host-core gate stays green, gates 1, 3, 7 and 8 included (per node and banked are
    bit-identical by design);
  - gate 2 goes red only through its lane-count assertion (`bus_console_lanes 0 != 3`). The
    bank-levels probe's reach assertion would also fire.

Decision 12's guarantee therefore rests, for the bus population this slice adds, on one test and not
on the compile. The structure predates the slice (a walk that skipped a track would be invisible
too), and nothing reaches the hole today. That is why this is MINOR, not MAJOR.

**Fix (recommended inside K1; `crates/graph-compiler/**` is authorized).**
- After the existing `unbanked_console_slot` call, take the console population from preparation:
  every `effects.entries[i]` whose `rack_id(entry.rack)` is a console rack, mapped through `ids[i]`.
- Require every such node to be in `chains`. A missing one returns `console.slot.unbanked` at the
  same path spelling, so no new code is needed. A missing `ids[i]` returns `graph.internal.invariant`.
- The code is Part A of the scratch file. Use `graph.internal.invariant` instead of its `u64::MAX`
  level placeholder.

Verified in the scratch export:
- With E1 plus the fix, the compile is refused with `console.slot.unbanked`, and host-core gates 1,
  3, 7 and 8 and graph-compiler gate 2 go RED.
- Without E1, `cargo test -p graph-compiler -p host-core --all-targets` (test-support features) is
  green: 25 binaries, 288 passed.
- Adopting the fix also makes gate 1's "dropped from its padded bank" clause true (see MINOR-3).

### MINOR-2: no committed test forms a console group holding a track lane and a bus lane

D3 says a bus lane's group "may also hold other buses and any track lane at that level and class".
Neither gate 2's sessions (tracks at 2/3/6, buses at 13+) nor gate 1's (the bus alone at its
level) mix the two.
- **Fix.** Adopt Part B of the scratch file as a `bank_levels.rs` test (`verifier_mixed_track_and_bus_console_group`).
- *Test value:* it turns red if the planner keeps bus lanes out of track groups at a shared
  (slot, class, level). Examples: a key or class split by strip kind, or a bus mis-seeded to
  another pool. Gate 2 cannot see that, because `ceil(n/W)` still holds per split key, and the
  probe compares bits only.
- It is green on `b0c7e29fb` at W8 and at simulated W4.

### MINOR-3: two test-value sentences in the record overclaim

The spec's test-value rule asks which plausible defect turns each test red. Two answers are wrong.
1. **Gate 1** claims "red if a bus lane is dropped from ... its padded bank". E1 leaves gate 1
   green, because banked and per-node renders are bit-identical by design. The clause becomes true
   only with MINOR-1's fix, which turns a dropped lane into a compile refusal.
   - Either adopt MINOR-1, or reword the clause to "red if a bus entry is bound to another lane's
     parameters, or its bypass is lost from the bank shunt" (E7 proves that clause).
2. **Gate 6** claims "red if a bus console lane at level >= 1 diverges from the scalar plan". That
   holds only for the `bank_levels` probe. The host-core differential compares armed, dual and
   serialized arms, and they treat a Stereo bus lane identically (E7 left it green).
   - Say which generator carries the scalar oracle.

### NITs

1. `crates/graph-compiler/src/banks.rs`: the doc line beginning "carries every slot in one order,
   ..." is about 140 characters. rustfmt does not wrap comments; rewrap it at 100.
2. In the automation-target paragraph, `docs/SESSION_SCHEMA_V1.md` now wraps "is / the strip's own
   / fixed section." across three short lines. Reflow it.
3. `session/src/estimate.rs` spells overflow paths `$.tracks.console` and `$.tracks` for every
   strip. This slice puts real submix console entries behind them. The issue is already in the K1
   ledger (#1200 NIT, "$.tracks overflow paths"), and this widens it.
4. `Reach::bus_console_entries` counts the model's entries on fed buses, not lanes the compiled plan
   banks. That is an adequate proxy, but its doc says "rendered". Say "declared on a fed bus".
5. `run-aarch64-tests.sh debug` was not run (x86 host). The simulated native W4 run (E5) is green,
   and no aarch64 known-defect row names a test this slice touches. CI's `aarch64-debug` at the
   batch push must still be green before the issue closes.

## Test value (one sentence each, for new or rewritten tests)

**host-core**
- `a_bus_renders_the_bits_of_a_track_fed_its_sum` (rewritten): red if a bus console entry runs
  with other parameters or loses its bypass from the bank shunt (E7, and Terra's params-cleared and
  bypass-dropped mutations).
- `a_bypassed_bus_console_slot_keeps_its_latency`: red if a bypassed bus slot's latency is dropped
  from PDC on a parallel path. Gate 1 cannot see that: its single-path topology has no second path
  to misalign.
- `no_bus_effect_gets_a_live_channel_or_an_observation_handle` (extended): red if a bus console
  slot gets a live channel or an observation handle during K1 (`track_owned` mutation).
- `a_processed_bus_renders_without_allocating` (now a console session): red if a bus console
  bank's membership or padding allocates on the render thread.

**host-web**
- `live_controlled_boot_of_a_bus_with_an_effect_renders` (extended): red if a live-controlled
  browser boot with a bus console slot is refused or fails to render.

**graph-compiler**
- `every_console_group_binds_when_buses_carry_the_console`: red if bus console lanes go unbanked
  at the native width, are grouped across levels, or are padded wrongly. It is the only test that
  catches E1.

**Generators**
- `bank_levels` probe and host-core randomized: judged by reach. Both assert nonzero bus-console
  reach (probe `[4, 15]`, host-core 11 entries).
- The bank-levels probe is the one that compares the bus lanes with the scalar oracle.

**Session**
- `submix_console_entries_are_refused_with_the_tracks_codes_at_index_paths`: red if submix entries
  escape the track rules or report a track path.
- Key order: red if `console` leaves D1's position or entries leave slot order.
- `compile_session_canonicalizes_submix_parameters`: red if submix entry params keep declared order.
- `unity_submix_is_transparent_and_valid`: red if D4's default goes live, drops a slot or reorders.
- `automation_target_may_name_a_submix`: red if MINOR-2 of #1199 regresses.
- The "missing console" refusal row: red if `console` becomes optional on a submix.

**Protocol**
- `random_submix_strips_round_trip_losslessly`: red if field 3 drops, reorders or truncates
  repeated entries.
- `a_slot_set_change_that_skips_a_submix_refuses_whole`: red if the store commits a slot-set change
  that leaves a submix without an entry. It is also red if `UpsertSubmix`'s field 3 fails to carry
  entries through the wire, because the commit leg would then refuse.

## Mutations (mine; each applied to the scratch export only and reverted)

| Id | Mutation | Result |
|---|---|---|
| E1 | strip walk skips submix console racks | compile accepted at W8; every host-core gate green; only gate 2's lane count red (MINOR-1) |
| E1b | bus console chains collected but never candidates | the guard refuses with `console.slot.unbanked`; gate 2 red |
| E1 + fix | MINOR-1's guard added | compile refused; host-core gates 1, 3, 7 and 8 and gate 2 red |
| E2 | submix `CONSOLE` field 3 changed to 7 | 7 tests red (conformance corpus, spec-order, round trip, gate 5 and others) |
| E7 | bus console bypass never reaches the shunt | gate 1 red; gate 3, randomized and probe green |
| E5 | native width simulated as 4 lanes on x86 | every bus-console suite green; Simd8 refused as the foreign width |
| hash | corpus submix console cleared | hash returns to the old pin `ca48855fd3a756b7` |
