# Bind refuses a compiled plan whose effect bank spans dependency levels

Root-cause study: `docs/handoffs/bug-966-2026-09-27/ROOT-CAUSE.md`. Prototype:
`docs/handoffs/bug-966-2026-09-27/prototype.patch`. Both are on `codex/batch-plumbing-floor-2` at
`c04027b0`.

## Product outcome

The compiler accepts sessions that bind then refuses with `graph.scheduler.layout`, so the host
fails prepare. This happens on every host:

- native: `prepare_host_session`;
- the browser and mobile, which are four-lane builds and are hit more often than native.

The ordinary trigger is a shared channel strip where one track lacks an effect its neighbours have
in front of another one. Remove `ch00`'s `eq` from
`fixtures/session/v1/console-sixty-four-track-intended.json` and the result is refused:

- at `Simd8` natively;
- at `Simd4` in the wasm build.

A randomized console probe refuses 194 of 1,000 sessions at native `Simd8` and 324 of 1,000 in the
wasm `Simd4` build. After this issue, every plan compile accepts binds.

## Root cause

`bind_rack_banks_indexed` (`crates/graph-compiler/src/banks.rs:67`) binds leader slot `s` of a full
cohort group whenever every lane runs it. It takes each lane's member at that lane's own chain
position `rank`, which is the count of its active slots before `s`. A lane whose program skipped an
earlier leader slot has no graph node for that identity slot, so its member sits at
`group.level + rank`, one or more levels before its bank-mates at `group.level + s`.

Bind runs a bank as one unit at its first member's position (`crates/graph/src/runtime.rs:4888`),
so it correctly refuses such a bank (`crates/graph/src/lib.rs:1235`). Relaxing that check
mis-renders 2,100 of 3,072 output samples on the reproducer. **The check is right, and the binder
is wrong.**

The "binds at `Simd4`" report is an x86 artifact: an `x86-64-v3` build refuses to bank most
effects at four lanes (D4).

## The fix (decided: study option (a))

In `bind_rack_banks_indexed`, after a slot's members are collected, `continue` (leave the slot
unbound) unless every member has the same dependency level. Read the level from the
`level_by_node` map the function already builds. Unbound members render per node, exactly as a
slot that some lane skips does today.

Nothing else changes:

- no graph-crate change;
- no planner or lane-order change;
- no change to levels, the schedule or the canonical SHA.

Every plan that binds today is byte-identical, because the only output that changes is a bank bind
refuses today.

**Rejected** (the reasons are in the study, §5):

- relaxing `has_valid_structural_layout`;
- keeping misaligned members out of the cohort in `rack-compiler`;
- level-aligned layering. It is a possible performance successor that needs an owner ruling; it is
  not part of this issue.

## Authorized paths

- `crates/graph-compiler/src/banks.rs`: the check, plus one doc paragraph on the
  `bind_rack_banks_indexed` "Level bucketing" comment.
- `crates/rack-compiler/src/lib.rs`: **doc comment on `CohortLevel` only**. It currently claims
  that the level partition keeps every bank on one level, which is true only for slot 0.
- `crates/graph-compiler/tests/bank_levels.rs`: new. Start from the prototype's file.
- `crates/graph-compiler/tests/MUTATIONS.md`: record M1 and M2.
- `.github/ISSUE_SPECS/966-bind-refuses-a-compiled-plan-whose-effect-bank-spans-dependency-levels.md`:
  evidence.

Anything else, and in particular `crates/graph/**`, `rack-compiler` code, `builtins-compiler`,
fixtures and console workloads, is out of scope. Stop and report instead.

## Smallest closable slice

1. Apply the check in `banks.rs` and the two doc corrections.
2. Commit `crates/graph-compiler/tests/bank_levels.rs`. It holds public-API compile, bind and render
   helpers, the console generator, and the four tests below.

## Objective gates

1. **Reproducer 1: the 64-track console without `ch00`'s EQ**
   (`the_sixty_four_track_console_less_one_eq_binds_at_every_width`). It compiles, binds and
   renders 8 blocks at `Scalar`, `Simd4` and `Simd8`, and asserts:
   - no bound effect bank spans levels;
   - PCM is bit-identical to `Scalar`, and the `Scalar` render is not silent;
   - exactly one misaligned planned slot at each SIMD width;
   - bound effect banks at `Backend::current()`: **21 at `Simd8`, 45 at `Simd4`**.
2. **Reproducer 2: the #962 seed-412 shape** (`a_cohort_lane_behind_an_extra_dynamic_eq_binds_at_every_width`).
   Nine intended-strip tracks: seven with dynamic `[soft-clip]`, `ch03` with `[eq, soft-clip]`,
   and `ch08` with an empty dynamic rack. Same assertions, with banks **2 at `Simd8`, 6 at `Simd4`**.
   The original seed cannot be replayed; this rebuilds the described shape.
3. **Over-reach guard** (`lanes_that_skip_the_same_slot_still_bank_it`). The 64-track console
   without the EQs of `ch00` and `ch56..=ch63`. Same assertions, with banks **20 at `Simd8`, 43 at
   `Simd4`**: the all-skip group `ch56..=ch63` must keep its compressor bank.
4. **Randomized probe** (`randomized_consoles_compile_bind_and_render_the_scalar_bits`), seeds
   `0..64`, fixed.
   - Every seed compiles and binds at all three widths.
   - Every seed with a misaligned planned slot is rendered for 16 blocks at all three widths. The
     render is bit-identical to `Scalar`, and the `Scalar` render is not silent.
   - Seeds with a misaligned slot are nonzero at both `Simd4` and `Simd8`. The prototype measured
     29 and 15.
   - Debug runtime is at most about 60 s on the reference host (the prototype takes 54 s).
5. **Class A.** Every plan that binds today is unchanged:
   - Every console workload digest, unit census, bank shape and fold/redirect count is unchanged at
     `Scalar`, `Simd4` and `Simd8`. Compare before and after with the harness's
     `console-workload` probe; the prototype found 99 of 99 rows identical.
   - `graph_fixture --check` passes.
   - The graph determinism script passes.
6. **Red mutations**, recorded in `MUTATIONS.md`:
   - **M1: delete the check.** All four tests fail with `graph.scheduler.layout`. The probe refuses
     28 lines (22 seeds) on x86.
   - **M2: over-strict check.** Require that no lane skipped any earlier slot (`rank == slot`).
     Only gate 3 fails, with 19 ≠ 20 banks at `Simd8`.
7. **Hygiene:**
   - fmt;
   - clippy `-D warnings` for `graph-compiler` and `rack-compiler` with `--all-targets`;
   - `cargo test -p graph-compiler -p rack-compiler -p graph -p console-workload -p host-core -p builtins-compiler`;
   - the graph and rack policy scripts;
   - `scripts/check-graph-determinism.sh`.

## Hazards

- **x86 cannot show four-lane effect banks.** On `x86-64-v3` only the limiter and multiband
  factories bank at `Simd4`, so an x86 `Simd4` compile hides most of the defect.
  - This is why the tests assert the plan-level misaligned-slot count at both widths, and pin bank
    counts only at `Backend::current()`.
  - Do not "fix" a `Simd4` pin by reading it off an x86 run.
  - The prototype's `Simd4` pins were verified in a `wasm32` + `simd128` guest under the pinned
    wasmtime (harness patch).
- **Silent renders compare nothing.** PDC delays a limiter or multiband strip by more than a
  thousand samples: at 2 blocks only 2 of 64 scalar renders were audible. Keep 16 blocks and the
  audibility assertion.
- **Keep the fix exactly where it is:**
  - Do not fix it in `has_valid_structural_layout`. Relaxing it mis-renders, or makes loading
    depend on track names.
  - Do not return `graph.internal.invariant` for a misaligned slot, which would turn a load failure
    into a compile failure.
  - Do not change `plan_bank_groups` or `order_members`, which moves lane order and digests in
    plans that bind today.
- **`cargo test --release -p graph-compiler` can fail spuriously** with "can't find crate for
  `effect_compiler`", because `effect-package`'s rlib and cdylib outputs collide. Rerun; debug is
  unaffected.
- **Rendered-bits claim.** The fix changes which lanes bank and never per-lane arithmetic, so every
  width must render the `Scalar` plan's bits. A mismatch is a real defect, not a tolerance
  question.

## Out of scope (possible successors, not filed)

- **Level-aligned cohort layering (study option (b)).** It would recover the misaligned banks (349
  of 1,649 in the 194 affected native sessions) and the ragged track's downstream limiter and
  builtin banks. It needs an owner ruling, because it changes the dispatch-independence of the
  semantic graph (#99 F6) or the graph identity of ragged sessions that bind today.
- **A wasm leg for the probe.** Qualification tooling on the `wasm-console-guest` pattern.

## Standing rules for the implementer

- No product behaviour changes for any plan that binds today. A moved console digest or unit census
  is a hard stop.
- Commit on `codex/966-unbind-misaligned-bank-slots`. Do not run timed benchmarks.
