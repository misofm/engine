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

## Rulings (coordinator, 2026-09-27)

- **The cost is accepted.** Only sessions that fail to bind today lose banks (about 79% of the banks
  the compiler formed survive), and failing to load is worse than running some slots per node.
- **Aligning dependency levels** to recover those banks changes graph identity; it is a separate
  performance follow-up, not part of this fix.
- **A wasm run in CI for 4-lane effect banks** is flagged to the owner (x86 `Simd4` tests barely form
  4-lane effect banks, which hid the browser case).


## Attempt 1 evidence

Implementer: attempt 1, on `codex/966-unbank-cross-level-slots` (the brief's
`codex/966-unbind-misaligned-bank-slots` was created under this name), cut from `584dbc16`, which
carries #970. Host: `x86_64`, `x86-64-v3` (`Backend::current()` = `Simd8`), rustc 1.97.1,
`CARGO_INCREMENTAL=0`, debug unless stated. Nothing was timed.

### The fix

`bind_rack_banks_indexed` (`crates/graph-compiler/src/banks.rs`): after a slot's members are
collected, the slot is left unbound (`continue`) unless every member has the same dependency level,
read from the `level_by_node` map the function already builds. It is the prototype's check,
unchanged: 13 lines of code, plus a comment and one doc paragraph on the "Level bucketing" comment.
`CohortLevel`'s doc (`crates/rack-compiler/src/lib.rs`) now says the level partition aligns only
each candidate's first slot. No graph, planner, schedule, fixture or digest change; nothing on the
render path changes, so render stays allocation-, lock- and syscall-free, and there is no new
`unsafe`.

### Committed tests (`crates/graph-compiler/tests/bank_levels.rs`)

Every reproducer compiles, binds and renders 16 blocks at `Scalar`, `Simd4` and `Simd8` with the
mono collapse unarmed, and at `Simd4` and `Simd8` armed exactly as `prepare_host_session` arms it
(`session_structural_symmetry` -> `arm_mono_collapse`). Every render must equal the `Scalar`
render bit for bit, the `Scalar` render must be audible, and no bound effect bank may span levels.
The misaligned planned slots are asserted at both SIMD widths, and the bank count at
`Backend::current()`.

| test | shape | misaligned `[Simd4, Simd8]` | ragged lanes `[Simd4, Simd8]` | banks `Simd8` (x86) | banks `Simd4` (wasm guest) |
|---|---|---|---|---|---|
| `the_sixty_four_track_console_less_one_eq_binds_at_every_width` | gate 1, `ch00` | 1, 1 | 0, 0 | 21 | 45 |
| `the_console_less_a_middle_lanes_eq_binds_at_every_width` | gate 1, `ch60` (amendment 3) | 1, 1 | 0, 4 | 21 | 45 |
| `the_console_less_a_last_lanes_eq_binds_at_every_width` | gate 1, `ch63` (amendment 3) | 1, 1 | 3, 7 | 21 | 45 |
| `the_mono_console_less_one_eq_binds_and_collapses_at_every_width` | gate 1 on the mono desk; asserts the armed collapse fires | 1, 1 | | 21 | 45 |
| `a_cohort_lane_behind_an_extra_dynamic_eq_binds_at_every_width` | seed-412 shape | 1, 1 | | 2 | 6 |
| `the_reduced_mono_console_from_the_970_probe_binds_at_every_width` | `reduced-nobus-from-970-verify.json` | 1, 1 | | 0 | 1 |
| `lanes_that_skip_the_same_slot_still_bank_it` | over-reach guard, nine EQs | 1, 1 | | 20 | 43 |
| `a_slot_after_a_misaligned_one_realigns_and_still_banks` | realignment guard (amendment 2) | 2, 1 | | 9 | 18 |
| `randomized_consoles_compile_bind_and_render_the_scalar_bits` | probe, seeds `0..64` | | | | |

The `Simd4` pins were read in the wasm guest (below), never off an x86 run.

### Reproducers before and after

- **Native, before** (base binder; also mutation M1 below): all eight reproducer and guard tests
  refuse at `Simd8` with `graph.scheduler.layout`.
- **Native, after:** 9 of 9 pass.
- **Wasm `Simd4`, before:** all eight shapes are refused, each with one cross-level bank (two for the
  realignment guard). Planned banks: 46, 46, 46, 46, 7, 2, 44 and 20.
- **Wasm `Simd4`, after:** all eight bind, with banks 45, 45, 45, 45, 6, 1, 43 and 18. Unarmed and
  armed renders equal the `Scalar` render bit for bit over 16 blocks (4,096 samples each), none
  silent. On the mono desk the armed collapse fired (256 collapsed blocks, 16 collapsible
  cohorts).

The wasm leg is a scratch `wasm32-unknown-unknown` + `simd128` guest that includes the committed
`bank_levels.rs` by `#[path]`, driven by a scratch host test in `tools/wasm-gates/tests/` under the
pinned wasmtime 47.0.3 (relaxed SIMD rejected). The guest reports `Backend::current()` = `Simd4`.
It needs a workspace member, so `Cargo.toml` and `Cargo.lock` were edited for the run and
restored; none of it is committed. Commands: `CARGO_TARGET_DIR=target/zz966 RUSTFLAGS='-C
target-feature=+simd128' cargo build --release --target wasm32-unknown-unknown -p zz-966-guest`,
then `ZZ966_GUEST=<the .wasm> ZZ966_REPROS=1 ZZ966_BLOCKS=16 ZZ966_START=0 ZZ966_COUNT=1000 cargo
test --release -p wasm-gates --test zz_966_host -- --nocapture` (`ZZ966_RENDER_ALL=1` renders every
seed). The native manual sweeps are the committed probe with `PROBE_966_START`, `PROBE_966_COUNT`
and `PROBE_966_RENDER_ALL`.

### Probe: unarmed and armed (amendment 1)

The collapse is armed exactly as the host arms it. #970 is on the base, and **both legs show zero
divergence everywhere**.

| run | seeds | refused before | refused after | rendered | moved, unarmed / armed | armed collapse fired |
|---|---|---|---|---|---|---|
| native, committed | `0..64` | 29 lines, 23 seeds (16 `Simd4`, 13 `Simd8`) | 0 | 30 | 0 / 0 | 12 seeds |
| native, manual | `0..1000` | | 0 | 338 | 0 / 0 | 86 |
| native, every seed rendered | `20000..20300` | | 0 | 300 | 0 / 0 | 79 |
| wasm `Simd4` | `0..64` | 28 seeds | 0 | 30 | 0 / 0 | 12 |
| wasm `Simd4` | `0..1000` | 323 seeds | 0 | 336 | 0 / 0 | 85 |
| wasm `Simd4`, every seed rendered | `20000..21000` | | 0 | 1,000 | 0 / 0 | 244 |

- Seeds with a misaligned planned slot: `[0, 30, 16]` at `[Scalar, Simd4, Simd8]` over `0..64`, and
  `[0, 336, 191]` over `0..1000`.
- No rendered seed was silent, except one in the wasm render-everything sweep (`20000..21000`); that
  mode does not assert audibility.
- The "every seed rendered" rows cover the plans that bind today (`PROBE_966_RENDER_ALL=1`),
  including the verifier's pre-#970 divergence range `20000..20300`. **Corrected in attempt 2:**
  with attempt 1's generator this range could not see a #970 revert (Sol, finding 1). Attempt 2's
  generator does: 22 armed lines over 16 seeds, including 20194.
- Committed probe runtime: 15-16 s wall and about 95 s CPU in debug on this 32-core host. It uses
  up to 8 worker threads; on a 4-core CI runner I estimate 25-30 s (not measured). The whole
  `bank_levels` binary finishes in the same time.

### Class A: every plan that binds today is unchanged

- **Console rows:** `console-workload` probe (the harness's `zz_probe_966.rs` only, release). This
  covers every console row at `Scalar`, `Simd4` and `Simd8`, baseline and meters/control/
  observation: digest, bank shape, transposes, folds, redirects, symmetry, collapse counters, unit
  census and meters drained. **99 of 99 rows are identical**; the output file's SHA-256 is
  `754f4ed8…681976c` before and after.
- **By construction:** the only output that changes is a bank whose members span levels, and bind
  refuses every plan that contains one.
- `graph_fixture --check`: **PASS**.
- `scripts/check-graph-determinism.sh`: **PASS (100/100)**.

### Mutations (`crates/graph-compiler/tests/MUTATIONS.md`, section "Issue #966")

| # | mutation | result |
|---|---|---|
| M1 | delete the check | RED 9 of 9, `graph.scheduler.layout`. The probe refuses 29 lines over 23 seeds |
| M2 | over-strict (`rank == slot`) | RED 2 of 9, the two over-reach guards only (19 ≠ 20, 8 ≠ 9) |
| M7 | `continue` becomes `break` | RED 1 of 9: the realignment guard (8 ≠ 9) |
| M3 | first versus last member only | RED 3 of 9: the middle-lane gate 1, the seed-412 shape and the probe |
| M8 | skip lane 1 | RED 1 of 9: the probe (seeds 31, 58) |
| M9 | the test source feeds a mono-mapped track two different sides | RED 2 of 9, armed legs only: the mono desk and the probe |

### Gates

All run on the tree committed as `2c6562d3`, with `CARGO_INCREMENTAL=0`, the worktree's own
`target/`, and `set -o pipefail` wherever output was piped.

| gate | command | result |
|---|---|---|
| fmt | `cargo fmt --all --check` | PASS |
| clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS, no warnings |
| docs | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| graph | `cargo test --locked -p graph` | 110 passed, 0 failed |
| graph, test support | `cargo test --locked -p graph --features test-support` | 117 passed, 0 failed |
| graph-compiler | `cargo test --locked -p graph-compiler` | 110 passed, 0 failed (`bank_levels` 9 in 14.9 s; `scale` 59.5 s) |
| rack-compiler | `cargo test --locked -p rack-compiler` | 13 passed, 0 failed |
| builtins-compiler | `cargo test --locked -p builtins-compiler --features test-support` | 79 passed, 0 failed |
| host-core | `cargo test --locked -p host-core --all-features` | 234 passed, 0 failed, 2 ignored (pre-existing release-budget tests) |
| capi | `cargo test --locked -p capi` | 36 passed, 0 failed |
| console-workload | `cargo test --locked -p console-workload` | 39 passed, 0 failed, 2 ignored (pre-existing measurement harnesses) |
| wasm-gates | `cargo test --locked -p wasm-gates` | 9 passed, 0 failed |
| graph policy | `bash scripts/check-graph-policy.sh` | `graph policy: PASS` |
| rack policy | `bash scripts/check-rack-policy.sh` | `rack policy: PASS` |
| realtime policy | `bash scripts/check-realtime-policy.sh` | `realtime policy: ok (57 marked regions in 16 files)` |
| graph determinism | `bash scripts/check-graph-determinism.sh` | `PASS (100/100)` |
| fixtures | `cargo run -p graph-compiler --bin graph_fixture -- --check` | PASS |
| console class A | the harness's `zz_probe_966.rs` in `console-workload`, release, before and after | 99/99 rows identical |

No timed benchmark was run. The scratch harness (`zz_probe_966.rs`, the wasm guest and its host
test) and the temporary `Cargo.toml`/`Cargo.lock` edit were removed before these gates ran.

### Deviations from the brief

1. **Generator.** An all-mono desk (`mono == 1000`) now draws its strip from
   `console-sixty-four-track-mono.json`, which is symmetric.
   - ~~With the intended fixture's asymmetric strip, the armed collapse fired on 0 of 29 rendered
     seeds, so an armed leg compared nothing.~~ **Wrong; corrected in attempt 2.** The collapse did
     not fire there because the arming correctly declined every chain that does not gather the
     track input (#970). That decline is exactly what the armed leg must check, and only that strip
     can see it fail. Attempt 2 keeps both strips.
   - Parsing draws nothing from the RNG, so every other draw is unchanged. A symmetric strip can
     still pool its tracks differently, and the prototype's counts moved: misaligned seeds are 30
     and 16 (were 29 and 15), and M1 refuses 29 lines over 23 seeds (was 28 over 22).
2. **Input.** The test's input reads each side from the session's mapped source channel, so a
   mono-mapped track is symmetric at its input as a host's source ring keeps it. The prototype
   gave each side its own signal, which would make the armed collapse diverge in a way no host
   can produce (M9 shows exactly that).
3. **Probe cost.**
   - Seeds are spread over up to 8 threads. Serial with the armed leg, the probe took 83 s.
   - Armed renders are skipped on desks with no mono-mapped track, because arming is then a no-op.
   - The probe also asserts that the armed collapse fired on at least one rendered seed.
4. **Blocks.** Reproducers render 16 blocks, not 8 (amendment 5).
5. **Extra tests.**
   - From the amendments and comments: `ch60` and `ch63`, the realignment guard and the reduced
     `#970` reproducer.
   - My own addition: the mono-desk reproducer, the one reproducer where the armed collapse fires on
     a rescued plan.
   - `early_lanes` pins where the ragged lane sits.
6. **Reduced reproducer location.** It is `include_str!`-ed from
   `docs/handoffs/bug-966-2026-09-27/`, where the issue records it, rather than copied into
   `fixtures/` (out of scope).
7. **Scope wording.** The claim is scoped to the effect-bank layout refusal (amendment 5). The probe
   still asserts that every seed binds.
8. **Not done** (optional amendment 4): the facility-armed reproducer (meters, controls,
   observation). The verification covered it with scratch tooling.

## Sol attempt 1 verdict: FAIL

Reviewed `584dbc16..7e3d75e8` (`2c6562d3`, `7e3d75e8`). Host: `x86_64`, `x86-64-v3`, rustc 1.97.1,
`CARGO_INCREMENTAL=0`, debug unless stated. Nothing was timed. The base and mutation builds ran in
a separate scratch worktree; nothing from them is committed.

**The code fix is correct, minimal and in the right place. Attempt 1 fails on test coverage and
evidence accuracy only.** The generator change removed the one probe shape that could see #970's
defect on a rescued plan, and the evidence says the opposite. The `Simd4` pins also cannot be
reproduced from the repository. Both are cheap to correct, and `banks.rs` does not need to change.

### Findings, by severity

1. **Medium: the armed leg is now blind to #970, and deviation 1 and the `20000..20300` row
   misdescribe it.** The fixture choice is at `crates/graph-compiler/tests/bank_levels.rs:579`.
   - The mutation used here is M970, the functional revert of `954790d1`. It sets
     `let armed = !tracks.is_empty()` at `crates/graph/src/runtime.rs:2375`, so chains that do not
     gather the track input are armed again.
   - Under M970, **all 9 `bank_levels` tests pass**. The committed probe also stays green on
     `20000..20300` with every seed rendered: 300 rendered, collapse fired on 79, 0 moved.
   - The same probe fails under M970 when all-mono desks keep the intended strip (the prototype's
     choice):
     - on `0..64`, 9 lines over 7 seeds (12, 19, 31, 48, 50, 54 and 59);
     - on `20000..20300`, 32 lines over 24 seeds, including the verification's seed 20194.
   - The reason is that a symmetric all-mono strip leaves nothing asymmetric upstream of a later
     chain. Wrongly arming that chain is then bit-exact, so the armed leg cannot fail.
   - The old armed leg did not "compare nothing" (spec deviation 1). The collapse never fired
     there because the arming correctly declined, and that decline is exactly what #970 made
     correct.
   - Amendment 1 exists to cover rescued mono-mapped sessions under the host's arming. The
     evidence row that runs `20000..20300` "including the verifier's pre-#970 divergence range"
     suggests coverage the committed generator no longer has.
   - **Required:**
     - Keep both all-mono shapes. For example, pick `MONO` or `INTENDED` for a `mono == 1000`
       desk by seed parity or by one draw inside that branch, so the armed leg must both fire and
       decline.
     - Keep the "collapse fired" assertion.
     - Record M970 as red in `MUTATIONS.md`.
     - Correct deviation 1 and that evidence row.
   - Without the change, the M970 counts are unchanged, and the prototype-strip probe is green
     unmutated apart from the "collapse fired" assertion.
2. **Medium: the `Simd4` pins cannot be reproduced from the repository.**
   - Where the pins are asserted and reported:
     - `native_bank_count` (`bank_levels.rs:855`) pins 45, 45, 45, 45, 6, 1, 43 and 18 at `Simd4`;
     - the spec reports them at lines 229-237.
   - Why nothing checks them now:
     - They came from an uncommitted guest, host test and workspace-member edit.
     - No CI job builds `bank_levels` for `wasm32` or `aarch64`, so the `Simd4` arm never runs.
     - `docs/handoffs/bug-966-2026-09-27/evidence-harness.patch` holds only the study's guest,
       which covers three shapes and not the five new ones.
   - I reproduced all 8 pins with that scratch guest (below), so the numbers are right.
   - **Required**, once the coordinator authorizes the path: commit the guest crate, the host test,
     the member line and the two commands as `docs/handoffs/bug-966-2026-09-27/wasm-pins-harness.patch`.
     Otherwise, file the brief's "wasm leg for the probe" successor and cite it next to the pins.
   - Reasoning from the code: the pins should also hold on `aarch64`. There `Backend::current()`
     is `Simd4`, and each factory banks only at `Backend::current().width()`. That is unverified
     on hardware.
3. **Low: the test compiles against a handoff directory.** `bank_levels.rs:57-58` `include_str!`s
   `docs/handoffs/.../reduced-nobus-from-970-verify.json`.
   - It is the only Rust source in the workspace that does. Every other cross-crate
     `include_str!` reads `fixtures/`.
   - Pruning or renaming the handoff breaks the `graph-compiler` test build.
   - Move it, when authorized, to a test data directory (for example
     `crates/graph-compiler/tests/data/`) or into `fixtures/`.
   - Not blocking on its own.
4. **Info: an unreachable case unbinds silently.** `banks.rs:263` unbinds silently when
   `first_level` is `None`.
   - That case is unreachable: every member comes from a candidate chain whose slot levels
     `banks.rs:151-190` already resolved, returning `graph.internal.invariant` if one is missing.
   - The line is identical to the prototype. If a future touch reaches it, make it the invariant
     error.
5. **Info: doc wording overstates.** `banks.rs:67-70` says the bucket "aligns every lane's slot 0
   and nothing after it". Lanes with identical programs align at every slot, so "guarantees
   alignment only at slot 0" would be exact.
6. **Info: the CI cost is acceptable.**
   - The whole `bank_levels` binary uses about 114 s of CPU.
   - Wall time is 18 s on this host, and 37 s pinned to 4 cores (`taskset -c 0-3`) with the host
     loaded.
   - `test-debug-a` runs on public 4-vCPU runners with a 15-minute limit.
   - It also passes with that job's features (`graph/test-support`, `engine/realtime-audit`,
     `builtins-compiler/test-support`).

### Confirmed

- **The fix.**
  - The `banks.rs` hunk is byte-identical to the prototype (blob `8f1dc9cc`).
  - The check runs before `bind_homogeneous_bank`, whose only call site is this one.
  - The `rack-compiler` change is doc-only, and the diff stays within the authorized paths.
  - The branch merges cleanly with `codex/batch-plumbing-floor-2` at `1d8c4851`.
- **Byte identity, native, fresh seeds `70000..72000`.**
  - The probe used a different input signal from the committed test's.
  - Every width was rendered for 16 blocks, unarmed and armed as the host arms it.
  - It compared, base (`584dbc16`) against the fix:
    - the SHA and levels;
    - effect banks, builtin banks and bound slots;
    - the estimate and the unit census;
    - shape, transposes, folds and redirects;
    - the unarmed and armed PCM and the collapse counters.
  - **5,227 of 5,227 lines that bind on the base are identical.**
- **Refused lines are rescued, native.**
  - The base refuses 773 lines (363 `Simd4`, 410 `Simd8`) over 551 seeds, all with
    `graph.scheduler.layout`.
  - All 773 bind with the fix, and they render the `Scalar` bits, unarmed and armed.
  - Levels are unchanged, and the collapse fired on 140 of these lines.
- **`prepare_host_session`.**
  - Base: the `ch00`, `ch60` and `ch63` consoles less one EQ, and the mono desk less `ch00`'s EQ,
    are refused with `graph.scheduler.layout`.
  - Fix: all four prepare.
- **Wasm `Simd4` guest.** It runs under the pinned wasmtime 47.0.3, and the guest reports 4 lanes.
  - **Reproducers:**
    - the base refuses all 8, with planned banks 46, 46, 46, 7, 2, 44, 20 and 46;
    - the fix binds them with 45, 45, 45, 6, 1, 43, 18 and 45 banks;
    - over 16 blocks, 0 renders moved, unarmed or armed;
    - on the mono desk the collapse fired: 256 collapsed blocks, 16 cohorts.
  - **Fresh seeds `70000..71000`:**
    - the base refuses 344 seeds; the fix binds all 1,000;
    - 351 misaligned seeds were rendered for 16 blocks, with 0 moved unarmed or armed;
    - the collapse fired on 69 seeds, and 2 renders were silent.
  - **Seeds `72000..72300`, every seed rendered:** 300 of 300 bind, 0 moved, 0 silent, and the
    collapse fired on 80.
- **Class A.**
  - Console probe (the harness's `zz_probe_966.rs`, debug): 99 of 99 rows are identical. The
    SHA-256 is `754f4ed8…681976c` for both base and fix.
  - `graph_fixture --check` passes, and so does the determinism script (100/100).
- **Mutations.** Every recorded mutation reproduces exactly:
  - M1: 9 of 9 red, 29 lines over 23 seeds (16 at `Simd4`, 13 at `Simd8`);
  - M2: 2 of 9 (19 ≠ 20, 8 ≠ 9);
  - M7: 1 of 9 (8 ≠ 9);
  - M3: 3 of 9;
  - M8: 1 of 9, only the probe (seeds 31 and 58);
  - M9: 2 of 9, armed legs only.
  - My own extra, M10 (compare only lanes that ran every earlier slot), is red on 4 of 9.
- **Generator counts (30 and 16, were 29 and 15).**
  - Every RNG-driven field matches the prototype-strip generator on `0..64`: track count, source
    mapping, delays, and effect ids, bypass and sidechains.
  - Only seeds 19, 31, 35 and 62 differ, all of them all-mono desks with asymmetric-delay tracks.
  - On those desks the symmetric strip splits pool classes, which moves grouping. That matches the
    evidence's explanation.
  - The input change (deviation 2) is host-realistic and correct.
- **Gates**, on the worktree with `-j 12`:
  - fmt passes;
  - clippy `--workspace --all-targets --all-features -D warnings` passes;
  - doc `-D warnings` passes;
  - test suites:

    | Suite | Passed | Ignored |
    |---|---|---|
    | `graph` | 110 | |
    | `graph --features test-support` | 117 | |
    | `graph-compiler` | 110 | |
    | `rack-compiler` | 13 | |
    | `builtins-compiler --features test-support` | 79 | |
    | `host-core --all-features` | 234 | 2 |
    | `capi` | 36 | |
    | `console-workload` | 39 | 2 |
    | `wasm-gates` | 9 | |

  - scripts and checks: graph, rack and realtime policy pass, and `graph_fixture --check` passes.

### Attempt 2 scope

- Test and evidence only, with no `banks.rs` change:
  - finding 1: the generator keeps both all-mono shapes, M970 is recorded, and the prose is
    corrected;
  - finding 2: the wasm harness patch, or the named successor.
- Re-pin the probe's misaligned counts, and re-run M1-M10 and M970.
- Findings 3-5 are optional.

## Attempt 2 evidence

Implementer: attempt 2, on the same branch, at `008d0ef3` (test and evidence changes) on top of
Sol's attempt-1 verdict `cd316992`. Same host and settings as attempt 1. **`banks.rs` is
unchanged** from `2c6562d3`, and so is every other production file. Nothing was timed.

### Finding 1: the armed leg sees #970 again

- **Generator.** An all-mono desk takes the symmetric mono fixture on an odd seed, and keeps the
  intended fixture's asymmetric strip on an even one; every other desk is unchanged.
  - Parity draws nothing from the RNG, so every other field matches the prototype generator.
  - On the symmetric strip the armed collapse fires, so a wrong collapse moves a bit (M9).
  - On the asymmetric strip the arming must decline every chain that does not gather the track
    input. That is the only shape that sees a revert of #970's check (M970).
  - The "collapse fired" assertion stays.
- **Corrected prose.**
  - Attempt 1's deviation 1 and its `20000..20300` bullet are corrected in place, above.
  - The doc comments on `MONO` and on the probe say the same.
- **M970**: `Runtime::arm_mono_collapse` without `identity.banking.gathers_track_input()`.
  - Committed probe (`0..64`): **RED**, 5 armed lines over seeds 12, 48, 50 and 54. The other 8
    tests stay green.
  - Manual sweep `20000..20300`, every seed rendered: **RED**, 22 armed lines over 16 seeds,
    including 20194.
  - Wasm `Simd4`, `0..1000`: **RED**, 24 armed renders moved.
  - Unmutated, all three are green.

### Finding 2: the `Simd4` pins are reproducible from the repository

I chose the patch. A real `tools/wasm-gates` case would need a new guest crate and workspace member,
which is new infrastructure: `wasm-gate-guest` runs a kernel corpus, not sessions.

- `docs/handoffs/bug-966-2026-09-27/wasm-pins-harness.patch` holds:
  - the guest `tools/wasm-pins-966-guest`, which `#[path]`-includes the committed `bank_levels.rs`;
  - the host test `tools/wasm-gates/tests/wasm_pins_966.rs`;
  - the workspace-member line.
- `docs/handoffs/bug-966-2026-09-27/README.md` gives the apply, build and run commands.
- The host test **asserts** the 8 `Simd4` pins: effect banks, misaligned slots, no refusal, no
  cross-level bank, and `Scalar` bits unarmed and armed over 16 blocks. It no longer only prints
  them.
- **Checked from the commit.** In a fresh detached worktree at `008d0ef3`, the patch applied
  cleanly and the README's two commands passed. The guest reports 4 lanes, all 8 pins hold, and the
  probe over `0..64` is clean. The worktree was then removed.

| reproducer | `Simd4` banks, fix | `Simd4` banks, base (M1) | misaligned `Simd4` |
|---|---|---|---|
| console less `ch00`'s EQ | 45 | 46, refused | 1 |
| console less `ch60`'s EQ | 45 | 46, refused | 1 |
| console less `ch63`'s EQ | 45 | 46, refused | 1 |
| mono console less `ch00`'s EQ (collapse fired: 256 blocks, 16 cohorts) | 45 | 46, refused | 1 |
| seed-412 shape | 6 | 7, refused | 1 |
| reduced #970 console | 1 | 2, refused | 1 |
| console less nine EQs | 43 | 44, refused | 1 |
| realignment guard | 18 | 20, refused | 2 |

Under M1 the harness fails on all 8 pins, and its probe (`0..1000`) refuses 324 seeds.

### Finding 3: the reduced reproducer is test data

`git mv` moved it to `crates/graph-compiler/tests/data/reduced-nobus-from-970-verify.json`. The
bytes are unchanged, `bank_levels.rs` reads it with `include_str!("data/...")`, and the handoff
README points to it. Findings 4 and 5 are in `banks.rs`, which this attempt must not change, so
they are left as they are.

### Probe, re-pinned

| run | seeds | misaligned `[Scalar, Simd4, Simd8]` | rendered | moved, unarmed / armed | collapse fired | bind refused |
|---|---|---|---|---|---|---|
| native, committed | `0..64` | `[0, 29, 15]` | 29 | 0 / 0 | 6 seeds | 0 |
| native, manual | `0..1000` | `[0, 335, 196]` | 336 | 0 / 0 | 34 | 0 |
| native, every seed rendered | `20000..20300` | `[0, 116, 64]` | 300 | 0 / 0 | 33 | 0 |
| wasm `Simd4` | `0..1000` | 335 at `Simd4` | 335 | 0 / 0 | 34 | 0 |

The committed probe's counts, 29 and 15, match the prototype generator's again.

### Mutations (`crates/graph-compiler/tests/MUTATIONS.md`, re-run on this test file)

| # | mutation | result |
|---|---|---|
| M1 | delete the check | RED 9 of 9; the probe refuses 28 lines over 22 seeds (15 `Simd4`, 13 `Simd8`) |
| M2 | over-strict (`rank == slot`) | RED 2 of 9, the two over-reach guards (19 ≠ 20, 8 ≠ 9) |
| M3 | first versus last member only | RED 3 of 9: middle-lane gate 1, seed-412 shape, probe (11 lines over 10 seeds) |
| M7 | `continue` becomes `break` | RED 1 of 9, the realignment guard (8 ≠ 9) |
| M8 | skip lane 1 | RED 1 of 9, the probe (seeds 31, 58) |
| M9 | the test source feeds a mono-mapped track two different sides | RED 2 of 9, armed legs only: the mono desk, and the probe (11 lines over 6 seeds) |
| M10 | compare only lanes that ran every earlier slot | RED 9 of 9. This is my formulation: Sol reported 4 of 9 for M10, and Sol's code is not recorded |
| M970 | revert #970's arming check | RED 1 of 9: the probe (seeds 12, 48, 50, 54) |

### Gates

Run on the attempt-2 tree with `CARGO_INCREMENTAL=0` and the worktree's own `target/`:

| gate | result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| `cargo test --locked -p graph` | 110 passed |
| `cargo test --locked -p graph --features test-support` | 117 passed |
| `cargo test --locked -p graph-compiler` | 110 passed (`bank_levels` 9) |
| `cargo test --locked -p rack-compiler` | 13 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| `cargo test --locked -p host-core --all-features` | 234 passed, 2 ignored (pre-existing) |
| `cargo test --locked -p capi` | 36 passed |
| `cargo test --locked -p console-workload` | 39 passed, 2 ignored (pre-existing) |
| `cargo test --locked -p wasm-gates` | 9 passed |
| graph, rack and realtime policy scripts | PASS; PASS; ok (57 regions) |
| `scripts/check-graph-determinism.sh` | PASS (100/100) |
| `graph_fixture --check` | PASS |
| console class A (the harness's `zz_probe_966.rs`, release) | 99/99 rows identical, SHA-256 `754f4ed8…` as on the base |

After the runs, the scratch console probe, the applied harness, the `Cargo.toml` and `Cargo.lock`
edits, and the scratch worktree were all removed. Nothing from them is committed except the patch
file.
