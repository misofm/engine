# Root cause: bind refuses a compiled plan whose effect bank spans dependency levels (#966)

Date: 2026-09-27. Research and prototype only. Nothing was pushed, and no GitHub issue was
created or edited.

**Tree.** Branch `bug-966-research`, created from `codex/batch-plumbing-floor-2` at `c04027b0`.
Every `path:line` anchor below is on `c04027b0`. The prototype is `prototype.patch` in this
directory, and the scratch evidence harness is `evidence-harness.patch`. Both apply cleanly to
`c04027b0`; the harness patch goes on top of the prototype patch.

---

## Verdict in plain terms

1. **The layout check is right, and the bank binder is wrong.** A cohort pads a short effect chain
   with "identity" slots so that it can share banks with longer chains. An identity slot is only a
   planning idea: nothing exists in the graph for it. So when a track is missing an effect that its
   bank-mates have (say, the EQ in front of the compressor), its compressor runs one dependency
   level earlier than theirs. The binder still puts all those compressors into one bank.
   A bank runs as a single step, at the position of its first member. If the misaligned track
   sorts first, the bank runs before its bank-mates' EQs have produced their audio, so the bank
   would read stale data. Bind refuses the plan to prevent that.
2. **The browser is hit harder than native.** The "binds at `Simd4`" observation is an artifact of
   testing on x86. An `x86-64-v3` build refuses to bank most effects at four lanes, because it has
   no runtime SIMD dispatch (D4). The real four-lane builds (browser wasm, iOS and Android) bank
   every launch effect except the delay. In the wasm build, 324 of 1,000 generated consoles fail to
   load, against 194 of 1,000 natively. iOS and Android were not run; they share the wasm build's
   `Backend::current()` of `Simd4`, so the same factories bank there.
3. **Realistic trigger.** A shared channel strip where some track lacks a non-final effect, or
   carries one extra effect in front of a shared one. Uniform strips never trigger it, and neither
   does removing the *last* effect.
4. **Recommended fix: option (a).** The binder leaves a slot unbound when its members sit at
   different levels, and those members render per node.
   - About ten lines of code in one function.
   - Byte-identical for every plan that binds today.
   - Costs about 1.8 of roughly 8.5 planned banks in each session it rescues. Those sessions do not
     load at all today.

---

## 1. The exact mechanism

**Step 1: pooling.** `bind_rack_banks_indexed` (`crates/graph-compiler/src/banks.rs:67`) builds one
candidate per `(track, rack)` chain. It buckets each chain by the level of its **first** slot
(`banks.rs:148-204`) and asserts that chain slot `k` sits at `level + k`.
`plan_bank_groups` (`crates/rack-compiler/src/lib.rs:253`) then pools the candidates by
`(level, rack, class)`, picks the longest program as the leader, and admits every candidate whose
program is a subsequence of the leader's (`rack-compiler/src/lib.rs:323`, via
`RackProgram::subsequence_mask`, `crates/rack/src/lib.rs:117`). A skipped leader slot becomes a
`false` in that lane's `active_slots`: an identity slot. `order_members`
(`rack-compiler/src/lib.rs:220`) sorts members by `(active_count desc, id)` and chunks them into
groups of W lanes, so short programs sort last and share the last full group with full programs.

**Step 2: binding.** For each full group and each leader slot `s` that every lane runs
(`banks.rs:219-229`), the binder takes each lane's member at chain position
`rank = count(active_slots[lane][..s])` (`banks.rs:233`). It then binds one bank over those members.
Nothing checked that the ranks agree, and a lane's member sits at `group.level + rank`, not
`group.level + s`.

**Step 3: the result.** Any lane that skipped an earlier leader slot that another lane runs
contributes a member one or more levels early. On the 64-track console without `ch00`'s EQ, the
last `Simd8` group is `[ch00 (FT), ch57..ch63 (TT)]`:

- The EQ slot is correctly unbound, because `ch00` does not run it.
- The compressor slot is bound across levels `[2, 3, 3, 3, 3, 3, 3, 3]`.
- `PreparedGraphPlan::has_valid_structural_layout` (`crates/graph/src/lib.rs:1158`, level clause at
  `:1235`) refuses it at bind (`:1712`) with `graph.scheduler.layout`.

**The stated invariant never held past slot 0.** `CohortLevel`'s doc
(`rack-compiler/src/lib.rs:110`) says "a bank never crosses a level" (#96 F12). The level partition
guarantees that only for slot 0. #206's end-to-end test
(`a_subsequence_program_track_binds_instead_of_panicking`, `graph-compiler/src/lib.rs:8471`)
removed the *last* slot. That makes a prefix, whose ranks always agree, so the misaligned case was
never exercised.

**Builtin banks cannot do this.** Their planner pools single-slot candidates by each node's own
level (`builtins-compiler/src/lib.rs:1304-1338`), and `with_builtin_banks` checks it again
(`graph/src/lib.rs:1361`). The only source of cross-level banks is the rack binder's use of
identity slots. In every probe line that bind refused, the refusal was exactly a cross-level effect
bank: 733 of 733 native lines over 2,000 scratch-generator seeds (325 at `Simd4`, 408 at `Simd8`),
and 324 of 324 in wasm.

## 2. Is the check right? Can a cross-level bank ever render correctly?

A bank's ops become one scheduling unit at the **first member's position** in the level-major,
id-sorted schedule (`crates/graph/src/runtime.rs:4888-4892`, #98 F1). That is correct only if:

- every member's producers precede that position; and
- every member's consumers follow it.

The layout check has two clauses:

- **The level clause** requires one level, which is sufficient and independent of track names.
- **The producer clause** (`graph/src/lib.rs:1240-1257`) requires the precise precondition.

**Experiment.** An environment switch relaxed the clauses (harness patch, `zz_relax_966.rs`). Each
case rendered 12 blocks at `Simd8` and was compared with the same session compiled at `Scalar`.

| Relaxed | Ragged track `ch00` (sorts first) | Ragged track `ch63` (sorts last) |
|---|---|---|
| nothing (today) | refused | refused |
| level clause only | refused (producer clause) | **binds, 0 of 3,072 samples differ** |
| both clauses | **binds, 2,100 of 3,072 samples differ** | binds, 0 differ |

So a cross-level bank is renderable only when the misaligned lane happens to sort after its
bank-mates' producers. Whether a session loads would then depend on track names. The level clause
is the correct, name-independent rule. Sidechains, sends and PDC do not change this:

- **Sidechains.** A chain with a connected sidechain is never a bank candidate
  (`RackProgram::is_bankable`, `crates/rack/src/lib.rs:106`; `blocks_banking`, `:72`). A sidechain
  only shifts that track's downstream, which splits pools but misaligns nothing.
- **Sends.** A send reads the same node outputs whether a slot is banked or not.
- **PDC.** Compensation is computed from latency, not from levels. All lanes of a group share one
  program key, and so one latency. The fix leaves the semantic SHA and the levels unchanged, even
  for the rescued sessions: 362 of 362 unchanged.

## 3. Why `Simd4` "binds" and `Simd8` does not

D4 pins one lane width per build. Most factories decline any other width with `Ok(None)`:

| Factory | Four-lane bank on an `x86-64-v3` build | Anchor |
|---|---|---|
| parametric EQ | declines (`lanes != Backend::current().width()`) | `parametric-eq/src/lib.rs:2400` |
| compressor | declines | `compressor/src/lib.rs:800` |
| gate/expander | declines | `gate-expander/src/lib.rs:1075` |
| soft-clip | declines (`!width_is_native`) | `soft-clip/src/lib.rs:938` |
| transient shaper | declines | `transient-shaper/src/lib.rs:865` |
| delay | never banks | `delay/src/lib.rs:609` |
| true-peak limiter | **binds** (width ≤ native) | `true-peak-limiter/src/lib.rs:2893` |
| multiband compressor | **binds** (no native-width check) | `multiband-compressor/src/lib.rs:1534` |

So a `Simd4` compile on x86 refuses only when the misaligned slot holds a limiter or a multiband
compressor. The planner and the layout check do not depend on width.

**On a real four-lane build.** A `wasm32` + `simd128` guest ran the same code under the pinned
wasmtime 47.0.3, with `Backend::current()` = `Simd4` (harness patch):

- both reported reproducers were refused, and so was the over-reach guard's shape (§6);
- 324 of 1,000 probe seeds were refused, against 194 of 1,000 at native `Simd8`.

Four-lane groups fill more readily, so a misaligned lane lands in a full group more often. The
browser (`hosts/host-web`) and mobile builds are therefore more exposed than native, not less.
Seed 412 of #962 cannot be replayed exactly, because that probe was scratch and was not kept.
`ragged_dynamic_soft_clip()` rebuilds the described shape: it fails at native `Simd8`, passes at
x86 `Simd4` (soft-clip declines four lanes), and fails in wasm.

## 4. Every shape that triggers it

**Rule.** A full cohort group with a slot that every lane runs, where the lanes reach that slot at
different chain positions. That is, some lane skipped an earlier leader slot that another lane
runs. The effect factory must also bank at that width.

**Triggers.** All three racks bank, so each of these applies in `simd1`, `dynamic` and `simd2`:

- A track missing a **non-final** effect of the shared strip: `[comp]` among `[eq, comp]`. This is
  the 64-track reproducer.
- A track with an **extra** effect in front of a shared one: `[eq, soft-clip]` among `[soft-clip]`.
  That track becomes the leader and misaligns everyone else. This is the seed-412 shape.
- Tracks missing *different* earlier slots of a strip of three or more slots.

**Non-triggers:**

- uniform strips;
- a track missing only trailing effects (a prefix, the #206 case);
- ragged tracks that fill whole groups by themselves (all lanes skip the same slot, so the ranks
  agree; `lanes_that_skip_the_same_slot_still_bank_it` pins this);
- ragged tracks confined to the padded last group, since effect banks bind only full groups;
- sidechained chains, mono/stereo pool splits, sends, submixes, input delays and PDC;
- builtin banks.

**Randomized probe.** The generator is in `crates/graph-compiler/tests/bank_levels.rs::generate`,
and its size and content are:

- 1 to 40 tracks, or 64;
- a shared strip per rack, with per-slot drops and inserts, or free per-track racks;
- all eight launch effects, with bypass;
- mono tracks and input delays;
- compressor and gate sidechains at every tap;
- sends into submixes.

Native x86 results over 2,000 seeds. The "misaligned slot" column is width-independent:

| Shape | Seeds | Seeds with a misaligned slot at `Simd4` or `Simd8` |
|---|---|---|
| uniform strip | 309 | 0 |
| strip, 3% per-slot variation | 318 | 100 (31%) |
| strip, 8% | 292 | 143 (49%) |
| strip, 20% | 294 | 188 (64%) |
| strip, 40% | 287 | 168 (59%) |
| free per-track racks | 500 | 95 (19%) |

Refused binds on the base tree, per 1,000 seeds of the committed generator:

- native `Simd8`: 194;
- wasm `Simd4`: 324;
- x86 `Simd4`: 166, which undercounts for the reason in §3.

Scratch-generator runs. The before/after fingerprint comparison in §5 and the cost figures in §5(a)
came from the harness's `zz_fingerprint_966.rs`. That is the same generator except for the order in
which it draws a bus route's tap and submix. Per 1,000 seeds it gives 194 refusals at `Simd8` and
168 at `Simd4`; over 2,000 seeds, 113 of its 500 free-rack seeds have a misaligned slot.

The #962 probe used a different generator (1-2 random effects per rack) and found fewer: 22
seed-width pairs in 800 seeds.

## 5. Fix options

**(a) The binder leaves misaligned slots unbound (recommended, prototyped).** After it has
collected a slot's members, `bind_rack_banks_indexed` checks that they share one dependency level,
and `continue`s if they do not. The level check is the exact invariant bind enforces. Equal levels
and equal ranks are the same condition, given the path arithmetic the function already asserts.

- **Bit identity.** Proved by construction: the only output that changes is a bank whose members
  span levels, and bind refuses every plan that contains one today. Also measured:
  - 2,638 of 2,638 probe lines that bind on the base are byte-identical after the fix. The
    compared fields are:
    - semantic SHA and level count;
    - every effect-bank and builtin-bank member list;
    - the estimate and the unit census;
    - bank shape, transposes, folds, redirects and PCM.
  - All 99 console-row fingerprints are identical: 17 rows, 3 widths, baseline plus
    meters/control/observation. The fingerprint covers digest, shape, transposes, folds,
    redirects, symmetry, collapse counters and unit census.
  - `graph_fixture --check` passes.
  - The semantic SHA stays width-independent: 1,000 of 1,000 seeds.
- **Performance.** Only in sessions that fail today.
  - Native: the 194 rescued `Simd8` sessions bind 1,300 of the 1,649 banks the planner formed, and
    349 misaligned banks (2,792 lanes, about 1.8 banks per session) render per node.
  - 64-track console without `ch00`'s EQ: 21 banks rather than 22, with eight compressors per node.
  - Wasm: 45 rather than 46 four-lane banks. Across the 1,000 wasm seeds, 10,315 of 10,971 planned
    banks bind.
- **Complexity.** About ten lines of code in one function, plus comments and two doc paragraphs. No
  API change, no graph, planner, fixture or digest change.

**(b) Level-aligned layering: pad shorter chains so that levels align.** Give a skipped slot a
level: either a real pass-through node, or a level floor in `schedule::topo` so that a ragged lane's
later slots sit at the leader's slot level.

- **Performance.** Better than (a). It recovers the 349 misaligned banks. If applied without regard
  to width, it would also realign the ragged track's whole downstream. For example, today `ch00`'s
  limiter lands alone at a different level, and `ch57..ch63`'s limiters form a 7-lane remainder,
  so 8 limiters render per node. The same happens to its fader and matrix builtin banks. That
  downstream cost exists today in every ragged plan that binds, and (a) does not change it. By
  hand count, the 64-track case would bind 23 effect banks rather than (a)'s 21 (not measured).
- **Bit identity.** Lost in one of two ways:
  - Aligning only bound slots makes levels, schedule and canonical SHA depend on the dispatch.
    That breaks #99 F6: "the semantic graph ... is deliberately independent of" dispatch.
  - Aligning every cohort member without regard to width changes the levels, schedule, buffer
    colouring and semantic SHA of ragged sessions that bind today. PCM bits do not change.
- **Complexity.** High. Pooling needs levels and alignment changes levels, so the per-rack planning
  has to iterate. The change also affects canonical bytes and possibly the `dependency_levels` cap.

Needs its own issue and an owner ruling. It should not be this bug's fix.

**(c) Relax the layout check.** Unsound as the schedule stands (§2):

- relaxing both clauses mis-renders 2,100 of 3,072 samples;
- relaxing the level clause alone still refuses the reproducer, and admits only sessions whose
  ragged track happens to sort last.

A correct version needs a new unit-placement rule and a new schedule model (#98 F1, #99 F1).
Rejected.

**(a′) Planner-side: keep misaligned members out of the cohort.** Rejected:

- It changes group membership and lane order for plans that bind today, for example a ragged
  member in a padded group, and so moves digests.
- It banks nothing more than (a), because the ragged lane still needs a group.

**(d) Per-lane bypass masks in the effect contract (#96 F7).** This would let the EQ slot bind with
`ch00` as an identity lane, but it still needs (b)'s alignment. Out of scope.

## 6. Prototype and evidence

`prototype.patch` contains the following.

- **`crates/graph-compiler/src/banks.rs`:** the level check and a doc paragraph.
- **`crates/rack-compiler/src/lib.rs`:** a doc-only correction to `CohortLevel`.
- **`crates/graph-compiler/tests/bank_levels.rs`:** a new file with four tests:
  - `the_sixty_four_track_console_less_one_eq_binds_at_every_width`;
  - `a_cohort_lane_behind_an_extra_dynamic_eq_binds_at_every_width`, the seed-412 shape;
  - `lanes_that_skip_the_same_slot_still_bank_it`, the over-reach guard;
  - `randomized_consoles_compile_bind_and_render_the_scalar_bits`, seeds `0..64`.

  Each reproducer compiles, binds and renders 8 blocks at Scalar, `Simd4` and `Simd8`. It asserts:
  - no cross-level bank;
  - PCM bit-identical to Scalar, and audible;
  - exactly one misaligned planned slot at each SIMD width;
  - the bank count at `Backend::current()`: 21/2/20 at `Simd8`, and 45/6/43 at `Simd4`. The
    `Simd4` pins were verified in the wasm guest.

  The probe compiles and binds every seed at all three widths. For seeds whose plan has a
  misaligned slot (29 of 64), it also renders 16 blocks at all three widths and compares bits.
  Sixteen blocks are needed because PDC delays the output of a limiter or multiband strip: at
  2 blocks only 2 of 64 scalar renders were audible, and at 16 all were.

**Results on the x86-64-v3 host, with the fix:**

- 4 of 4 pass in 54 s (debug).
- fmt and clippy `-D warnings` are clean.
- The `graph-compiler`, `rack-compiler`, `graph`, `console-workload`, `host-core` and
  `builtins-compiler` suites pass.
- Graph policy PASS, rack policy PASS, graph determinism 100/100, `graph_fixture --check` PASS.
- `prepare_host_session` on the 64-track console without `ch00`'s EQ:
  - base: `graph.scheduler.layout`;
  - fix: OK.

**Red mutations:**

- **M1: remove the level check.** All 4 tests fail. The probe refuses 28 lines (15 at `Simd4`, 13 at
  `Simd8`, 22 seeds), and each reproducer refuses with `graph.scheduler.layout`.
- **M2: over-strict variant.** Unbinds any slot behind an identity slot, that is, requires
  `rank == slot`. Only `lanes_that_skip_the_same_slot_still_bank_it` fails (19 banks, not 20).
  This is why that test exists: the probe and the other reproducers cannot see a pure
  banking-count loss.

**Wasm (`simd128`, `Simd4`):**

| Tree | Reproducers | Probe, 1,000 seeds | PCM moved |
|---|---|---|---|
| base | 3 of 3 refused | 324 refused | 0 |
| fix | 3 of 3 bind | 0 refused | 0 |

## 7. What the owner must rule on

1. **Accept (a)'s cost.** A session with ragged strips loads, and renders its misaligned slots per
   node: about 21% of the banks the planner formed in those sessions. Loading it is strictly
   better than refusing it.
2. **Whether to open (b) as a performance issue.** Level-aligned layering recovers those banks and
   the ragged track's downstream banks. It changes either the dispatch-independence of the semantic
   graph (#99 F6) or the graph identity of ragged sessions that bind today. That is an architecture
   choice, not a bug fix.
3. **Four-lane coverage on x86.** A `Simd4` compile on the x86 CI host binds only limiter and
   multiband banks, so every x86 `Simd4` test is weak evidence for the browser. The committed probe
   works around this with a plan-level count. A wasm leg for the probe, reusing the
   `wasm-console-guest` pattern, would be a separate tooling issue.

## 8. Reproducing

```sh
git checkout -B bug-966-research codex/batch-plumbing-floor-2     # c04027b0
git apply docs/handoffs/bug-966-2026-09-27/prototype.patch
CARGO_INCREMENTAL=0 cargo test -p graph-compiler --test bank_levels
# Red: revert crates/graph-compiler/src/banks.rs and rerun.
git apply docs/handoffs/bug-966-2026-09-27/evidence-harness.patch  # scratch; never commit
PROBE_COUNT=1000 PROBE_BLOCKS=3 PROBE_OUT=/tmp/probe.txt \
  cargo test -p graph-compiler --release --test zz_fingerprint_966 probe -- --nocapture
PROBE_966_OUT=/tmp/console.txt cargo test -p console-workload --release --test zz_probe_966
X966_RELAX=level cargo test -p graph-compiler --test zz_relax_966 -- --nocapture   # or =both
CARGO_TARGET_DIR=target/zz966 RUSTFLAGS="-C target-feature=+simd128" \
  cargo build --release --target wasm32-unknown-unknown -p zz-probe-966-guest
ZZ966_GUEST=$PWD/target/zz966/wasm32-unknown-unknown/release/zz_probe_966_guest.wasm \
  ZZ966_COUNT=1000 cargo test --release -p wasm-gates --test zz_probe_966_host -- --nocapture
```

`cargo test --release -p graph-compiler` sometimes fails with "can't find crate for
`effect_compiler`", because `effect-package`'s rlib and cdylib outputs collide in `target/release`.
Rerunning succeeds. The debug profile is unaffected.
