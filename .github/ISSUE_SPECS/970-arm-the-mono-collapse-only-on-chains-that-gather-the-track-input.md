# Arm the mono collapse only on chains that gather the track input


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A correctness fix. Base `6ca203f8`.

## Problem

A mono-mapped track (`left_source_channel == right_source_channel`) can render **wrong audio**: its
right channel is replaced by its left one. It happens when the track's strip is split into more
than one bank chain and a stage *before* the later chain makes L and R differ.

The collapse's structural term `SOURCE` means "the track input carries identical planes". It is
decided per track from the session (`session_structural_symmetry`,
`crates/builtins-compiler/src/lib.rs:3936`) and joined onto chains by
`Runtime::arm_mono_collapse` (`crates/graph/src/runtime.rs:2304-2316`), which arms **every** chain
whose lanes' tracks have it. That premise is true only for the chain that gathers the track input.
A later chain reads planes produced by an earlier chain or a per-node op of the same track, which
are in neither its own witness (`BankChain::lane_symmetry_bank`, `crates/rack/src/lib.rs:2003`)
nor the join. `crates/rack/src/lib.rs:2342-2350` assumes `SOURCE` "cannot be an episode", which is
only true of the first chain.

A strip splits when a stage meter or a send taps an internal boundary (`PostInputBuiltins`,
`PostSimd1`, `PostDynamic`), or when a per-node op (delay, sidechained effect, effect in a partial
cohort) sits between banked racks.

Reproduced by two tests (file `crates/host-core/tests/dualmono_probe.rs` in
`docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch`; eight tracks of
`fixtures/session/v1/parametric-eq-bank-console.json`, all mono-mapped; collapse armed versus
`force_mono_collapse_off(true)`, output bits compared):

* asymmetric EQ in `simd1` (band gain +6 dB left, -6 dB right), symmetric EQ in `simd2`, console
  meter at `MeterTap::PostSimd1`: right channel differs from sample 0;
* asymmetric per-node `miso.delay` in `dynamic` (5 ms / 7 ms) between two banked EQs: right channel
  differs from sample 240.

## Outcome

Only a chain whose every lane's **first slot is that track's input-builtins stage**
(`GraphNodeId::TrackStage { stage: TrackStage::PostInputBuiltins, .. }`) may be armed on the
structural witness. Every later chain stays unarmed, so it renders dual: correct, and slower only
on split strips. No standing row changes shape, bits or collapse counts (all standing strips are
one chain per cohort).

## Read first

* `crates/graph/src/runtime.rs:5661-5710`: where each unit's `UnitIdentity` is built from its ops;
  `node_of(ops[lane])` is the first slot's node for lane `lane`.
* `crates/graph/src/runtime.rs:1961-1980, 2047-2062`: `UnitIdentity` and the compile-time
  assertion that its flags fit the padding; on wasm32 there is room for exactly four flag bytes,
  and all four are used.
* `crates/graph/src/runtime.rs:2304-2316`: `arm_mono_collapse`.

## Authorized paths

`crates/graph/src/runtime.rs`, `crates/host-core/tests/` (one new test file), and this issue's spec.

## Design constraint

Do not grow `UnitIdentity` (the wasm32 assertion must hold). Carry the "reads the track input" fact
some other way, for example a per-unit `Box<[bool]>` beside `identity` computed in the same loop,
or by deciding the arming there and storing only the chain's existing `collapse_source`. Bind-time
only; nothing on the render path changes.

## Gates

* The two probe tests above, ported as a new `crates/host-core/tests/` file: **red on the base,
  green after**. Keep the two controls (symmetric per-node delay: bit-identical and still
  collapsing its first chain; asymmetric EQ in one chain: bit-identical, no collapse).
* A count assertion: in the per-node-delay probe, `bank_collapse_counters()[1]` is `1` (the first
  chain only), not `2`.
* Unchanged: `cargo test -p console-workload --test chain_shape` (every collapse counter, e.g. 8 of
  8 cohorts on `_mono`, 4 of 8 on `half_mono`), `-p host-core --test symmetry_witness --test
  track_delay --test input_liveness_console`, `-p rack --test mono_reengage`, `-p graph --test
  rt9_resident_bank_input_alloc`.
* `cargo check -p graph --target wasm32-unknown-unknown` (the identity-size assertion).
* `cargo fmt --check`, `cargo clippy -p graph -p host-core --all-targets -- -D warnings`.

## Non-goals

Letting a later chain collapse when its predecessor collapsed in the same block (a separate
design issue); anything about pooling or detection.


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27. Branch `codex/970-arm-collapse-on-input-chains`, base `28964870`
(the local optimisation batch); fix commit `954790d1`. Host x86_64 (`x86-64-v3`), debug profile,
`CARGO_INCREMENTAL=0`. The amendment comment overrides the body; this record follows it.

### The fix

* `crates/graph/src/runtime.rs`: `UnitIdentity::banked: bool` is re-encoded as the one-byte
  three-state `UnitBanking` (`Plain` / `Bank` / `BankGatheringTrackInput`), per amendment 3. The
  third state is decided at bind in `build_sequential`, in the loop that already builds the row:
  every lane's first slot (`node_of(ops[lane])`) is `GraphNodeId::TrackStage { stage:
  PostInputBuiltins, .. }`. `Runtime::arm_mono_collapse` adds `identity.banking.gathers_track_input()`
  to its conjunction; every other chain stays unarmed and renders dual.
* `crates/graph/src/lib.rs:2876` (authorized by amendment 3): `banked: identity.banking.banked()`,
  so `PlanUnitEligibility::banked` is unchanged.
* The layout mirror `UnitIdentityWithoutFlags` keeps `banked: bool`, so the compile-time assertion
  also refuses a re-encoding wider than one byte. No `Runtime` field is added, so neither `Runtime`
  mirror (`RuntimeWithoutSplitPairTable`, `RuntimeWithoutObservationActivation`) changes and the
  resource accounting is untouched. Nothing on the render path changes; no new `unsafe`.
* `crates/host-core/tests/collapse_arming.rs` (new): the reproducers, the count gate and the controls.

### Reproducers: armed against `force_mono_collapse_off(true)`, output bits compared

Eight mono-mapped tracks of `parametric-eq-bank-console.json` (sixteen for the live writes), 16
blocks of 128. Counters are the armed arm's `bank_collapse_counters()`; "R from n" is the first
differing right-plane sample. Four lanes = the prototype's `--cfg miso_native_simd4` lane hunk,
applied only for the measurement (own target dir `target/w4`) and reverted.

| test | base, 8 lanes | base, 4 lanes | fix, 8 lanes | fix, 4 lanes |
|---|---|---|---|---|
| asym simd1 EQ + `PostSimd1` meter | [16,2], **R from 0** | [32,4], **R from 0** | [0,1], identical | [0,2], identical |
| asym per-node delay 5/7 ms | [32,2], **R from 240** | [64,4], **R from 240** | [16,1], identical | [32,2], identical |
| send at `post_simd1`, asym simd1 EQ | [16,2], **R from 0** | [32,4], **R from 0** | [0,1], identical | [0,2], identical |
| send at `post_input_builtins`, asym trim | [16,2], **R from 0** | [32,4], **R from 0** | [0,1], identical | [0,2], identical |
| live left-only polarity, misaligned cohorts (16 tracks, odd ones without effects) | [36,3], **R from 512** | [84,6], **R from 512** | [20,2], identical | [52,4], identical |
| live left-only trim -6 dB, same session | [36,3], **R from 512** | [84,6], **R from 512** | [20,2], identical | [52,4], identical |
| count: delay probe `counters[1]` = half the shipped value | **2 (want 1)** | **4 (want 2)** | 1 | 2 |
| control: symmetric delay (identical, first chain collapses every block) | ok | ok | ok [16,1] | ok [32,2] |
| control: asym EQ in one chain (identical, no collapse) | ok | ok | ok [0,1] | ok [0,2] |

The count gate is written width-free: `shipped = 2 * tracks / width` (two chains per cohort around
the delay; the test also asserts the plan has exactly `shipped` banked units with an upstream
stage), and `counters[1] == shipped / 2`. Command: `cargo test --locked -p host-core --test
collapse_arming`: base 2 passed / 7 failed at both widths, fix 9 passed at both widths.

### Standing console rows

All 17 `native_session_rows()` rows, 32 blocks each, base `28964870` against fix `954790d1`, at 8
lanes and at 4 lanes (scratch example over `SessionRuntime::new`, not committed): `bank_shape`,
`bank_collapse_counters`, `bank_collapse_transitions`, `bank_route_folds`,
`bank_scatter_redirects`, both symmetry censuses, unit and bank counts and the output SHA-256 are
**identical row for row at both widths** (e.g. `_mono` collapse [256,8] / [512,16], `half_mono`
[128,4] / [256,8]).

### Gates

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| `cargo check --locked -p graph --target wasm32-unknown-unknown` (identity-size assertion) | pass |
| `cargo test --locked -p graph` | 110 passed (incl. `rt9_resident_bank_input_alloc`) |
| `cargo test --locked -p graph --features test-support` | 117 passed |
| `cargo test --locked -p rack` | 54 passed (incl. `mono_reengage` 4/4) |
| `cargo test --locked -p graph-compiler` | 101 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 79 passed |
| `cargo test --locked -p host-core --all-features` | 234 passed, 2 ignored (release-mode budgets, nightly); `collapse_arming` 9, `symmetry_witness` 11, `track_delay` 8+2+7, `input_liveness_console` 10 |
| `cargo test --locked -p capi` | 36 passed (incl. `resource_lifecycle` 4/4) |
| `cargo test --locked -p console-workload` | 39 passed, 2 ignored (measurement harnesses); `chain_shape` 23/23; lib resource test `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables` ok |
| `scripts/check-graph-policy.sh`, `check-realtime-policy.sh`, `check-rack-policy.sh`, `check-host-core-policy.sh`, `check-workspace-policy.sh` | all pass |

### Red mutations (applied to the fix, run, reverted)

* **M1, the shipped rule**: drop `identity.banking.gathers_track_input() &&` from
  `arm_mono_collapse`. `collapse_arming`: all seven reproducers and the count gate red, with the
  base's first differences (R from 0, 240, 512) and `left: 2, right: 1`; both controls green.
* **M2, over-disarm**: test `TrackStage::Input` instead of `PostInputBuiltins`. `collapse_arming`:
  6 red (count 0 vs 1, symmetric control, one-chain control, delay and both live-write probes on
  `counters[0] > 0`); `console-workload --test chain_shape`: 9 red (e.g.
  `the_collapse_fires_on_every_mono_cohort_and_no_other`).
* **M3, lost third state**: `UnitBanking::banked()` returns `matches!(self, Self::Bank)`.
  `collapse_arming`: 3 red (count gate `collapsible` 1 vs 2, both live-write split assertions).
* **M4, layout**: `#[repr(u16)]` on `UnitBanking`. `cargo check -p graph --target
  wasm32-unknown-unknown`: `a UnitIdentity flag no longer fits the row's padding`.
* Not discriminable, recorded: "any lane" instead of "every lane" in the first-slot test. A bank's
  slot 0 is one stage across all its lanes, so no session builds a mixed chain.

### Deviations and notes for the verifier

* Test file is `crates/host-core/tests/collapse_arming.rs`, not the prototype's
  `dualmono_probe.rs`; the probes are rewritten (shared oracle helper, send and live-write probes
  added, meters drained every block).
* Not edited (outside the authorized paths): the rack M3 comment ("`SOURCE` ... cannot be an
  episode", `crates/rack/src/lib.rs` about 2521-2527) is true again, since only input-gathering
  chains are armed, and the obligation is now stated on `Runtime::arm_mono_collapse`.
  `PlanUnitEligibility` still has no armed bit, so a later chain can report every lane eligible
  while it can never collapse (the verification's minor note).
* **Expected performance cost, not measured here** (timed benchmark not run, per the brief): the
  verification measured +42% at 8 lanes and +48% at 4 on 64 mono tracks with half lacking effects,
  and +25% / +24% with one track in eight lacking the limiter. Recovery is #987; not attempted here.

## Sol attempt 1 verdict: PASS

Adversarial review of `28964870..31c78b5a` (fix `954790d1`), 2026-09-27. x86_64 (`x86-64-v3`),
debug, `CARGO_INCREMENTAL=0`; no timed run. Four lanes = the prototype's `miso_native_simd4`
`Backend::current()` hunk in a scratch copy of the branch (own target dirs, never in this tree):
`Backend::current()` is the only width selector, and the four-lane kernels it selects already build
and run natively under the graph tests; no wasm test runner is installed.

### What was checked

* **Reproducers.** `collapse_arming` on base: 2 passed / 7 failed at 8 lanes and at 4, with the
  recorded first differences (R from 0, 240, 512) and counters ([32,4], [64,4] at 4 lanes; count
  gate 2 vs 1 and 4 vs 2). On the fix: 9/9 at both widths.
* **Randomized probe** (scratch `verify970_probe.rs`): mono-heavy sessions of 1-20 tracks (some
  stereo, mono on channel 0 or 1), random strips over EQ, compressor (dual/max/avg link), delay,
  limiter, soft-clip, gate, transient shaper in all three racks, bypass, symmetric/asymmetric
  trim, polarity, HPF/LPF and input delay, random pan/fader, sends at all seven taps, routed
  sidechains, random meter tap, observation taps, live writes on the input, fader, matrix and
  effect queues (left/right/both), and optionally left/right automation of trim and EQ gain.
  Armed against `force_mono_collapse_off(true)`, output bits compared. **Fix: 0 differences in
  2,245 rendered sessions at 8 lanes and 1,954 at 4** (60-75% collapsing, about 20% (8) / 50% (4)
  holding an all-mono later chain left unarmed). **Base, same generator: 19 of 748 differ at 8
  lanes, 56 of 441 at 4**, so the probe discriminates. Other paths (plan swaps, rebinds,
  observation activation): `UnitIdentity` is built only in `build_sequential` and the only
  production arm is `host-core/src/prepare.rs:1646`, which every prepare entry reaches.
* **Standing rows.** All 17 `native_session_rows()`, 32 blocks, base against fix: shape,
  collapse counters, transitions, route folds, scatter redirects, both censuses, unit and bank
  counts, a digest of the whole `unit_eligibility()` rows and the output SHA-256 are identical at
  8 and 4 lanes (`_mono` [256,8]/[512,16], `half_mono` [128,4]/[256,8]).
* **Layout.** One read site of the old bool (`graph/src/lib.rs:2876`), behaviour-identical
  (`banked()` is `!Plain`, and `of` yields `Plain` exactly when membership is empty). No `Runtime`
  field. `cargo check -p graph` and `-p host-web --target wasm32-unknown-unknown` pass; capi
  `resource_lifecycle` 4/4 and the console-workload lib resource test pass.
* **Mutations.** M1-M4 reproduced exactly (M1: 7 red; M2: 6 red + 9 `chain_shape`; M3: 3 red +
  2 `chain_shape`; M4: wasm32 const assertion, native still builds).
* **Gates.** fmt; clippy `--workspace --all-targets --all-features -D warnings`; doc `-D warnings`;
  `-p graph` 110, `--features test-support` 117, `-p rack` 54, `-p graph-compiler` 101,
  `-p builtins-compiler --features test-support` 79, `-p host-core --all-features` 234 (+2
  ignored), `-p capi` 36, `-p console-workload` 39 (+2 ignored); graph, realtime and rack policy
  scripts pass (the first two are mode 644 in the tree and need `bash`).

### Findings (none blocking)

1. **Low, docs outside the authorized paths: follow-up.** `crates/rack/src/lib.rs:2748-2749`
   defines `BankChain::arm_mono_collapse`'s `structural` as "every active lane of this chain
   renders a track whose two channels read one source channel", the per-track premise #970
   refutes; `:1828` and `crates/host-core/src/lib.rs:211-213` say the same. The M3 comment at
   `crates/rack/src/lib.rs:2521-2527` is true again and needs no change. Follow-up (doc-only):
   restate `structural` as "this chain gathers its tracks' input (every active lane's first slot
   is its track's `PostInputBuiltins` stage) and each of those tracks reads one source channel;
   a chain fed by an earlier unit must be passed `false` (#970)", and add "on a chain that gathers
   the track input" at the other two sites.
2. **Low, plan surface: follow-up.** `crates/engine/src/realtime/plan.rs:185-228` tells a
   caller to join `lane_eligible` with `SOURCE` through `lane_tracks`. That now over-reports every
   later chain of a split strip, and neither `console-workload`'s `bank_symmetry_counters`
   (`tools/console-workload/src/lib.rs:1518`) nor this issue's own count gate (proxy
   `banked && upstream_of_seam_stages > 0`) can see the armed set. Follow-up: add
   `pub gathers_track_input: bool` to `PlanUnitEligibility`, filled at
   `crates/graph/src/lib.rs:2876` from `identity.banking.gathers_track_input()`; limit the doc's
   join to rows where it holds; have `collapse_arming.rs` assert that the second chain of each
   cohort reports `false`.
3. **Low, one important mutation survives.** Changing `UnitBanking::of`'s
   `(false, _) => Plain` (`crates/graph/src/runtime.rs:2016`) to map a plain op whose node is
   `PostInputBuiltins` onto `BankGatheringTrackInput` passes `collapse_arming`, `chain_shape` and
   `-p graph`. No audio changes (`arm_mono_collapse` skips non-bank units), but
   `PlanUnitEligibility::banked` would be wrong for an unbanked input stage. A row assertion on a
   one-track session, or a unit test of `of`, kills it. Also surviving, and equivalent: dropping
   `lanes > 0` (redundant with the empty-lanes guard) and `any` for `all` (a graph bank slot has
   one stage and one lane set, `runtime.rs:4031`), as recorded.
4. **Nit.** The `954790d1` message says "seven reproducers" and lists six. M1's "all seven
   reproducers and the count gate red" is 7 red in total: 6 reproducers and the count gate.
5. **Info, pre-existing, unrelated.** Some valid sessions are refused at prepare with
   `graph.scheduler.layout` on the base as well as the fix (2 of 1,500 at 8 lanes, about 1.5% at
   4), against `runtime.rs:368-369`'s "the graph compiler never emits either". Reduced reproducer:
   8 mono tracks all routed to `main-out`, each with a `simd1` compressor (link `average`), mixed
   EQ/soft-clip programs and per-track asymmetries (7 tracks or no effects pass). It fails safe,
   not wrong audio, and deserves its own issue. The scratch `reduced-nobus.json` holds it.
6. **Info.** Four-lane coverage exists only by the scratch hunk; CI runs these tests at 8 lanes.
   The tests are written width-free, which is the right mitigation. Cost is unmeasured by design;
   recovery is #987.
