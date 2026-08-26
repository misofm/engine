# Eight-lane wasm banks: the width-guard survey, and what a `Backend::current()` switch would meet

**Candidate.** Issue #183, step 1. The compressor round-1 research found that the wasm compressor
gap is lane width and not codegen — wasmtime at W4 runs about 0.92x of native-at-W4 on this host,
and the native W4 -> W8 width probe cost +9 % wall for 2x lanes. Switching `Backend::current()` on
`wasm32` to `Simd8`, so that `wide::f32x8` lowers to two `v128` values, was projected to move the
wasm compressor increment 81 -> ~50-60 us and to generalise to every banked effect.

**Status.** Survey complete; the switch is **not** cleared by this document. Two findings are
blockers and one is a design gap, all cited below. The decision evidence is the paired W4/W8
measurement of step 2, recorded in `artifacts/issue183/` and read in the last section.

**Scope.** Every `bind_homogeneous_bank` in the workspace, the builtins bank width, the
`Lane::SVF_CASCADE_DEPTH` tuning at the new width, and register-pressure exposure under Cranelift's
sixteen host `xmm` registers. Nothing here changes a shipped artifact: the only production edit in
this branch is one build-time `cfg` in `crates/miso-engine-lane/src/backend.rs`, and the default
`wasm32` guest module is byte-identical with and without it (evidence below).

---

## The switch, stated precisely

`Backend::current()` (`crates/miso-engine-lane/src/backend.rs`) is a compile-time constant with one
arm per target. The wasm arm returns `Simd4`. Changing it to `Simd8` changes exactly one thing
directly — `Backend::current().width()` becomes 8 — and everything else follows from two rules
already in the tree:

* `BankWidth::for_backend` (`crates/miso-engine-effect-contract/src/lib.rs`) maps `Simd8` to
  `BankWidth::Eight`. It is the workspace's single backend-to-width law (#84 phase A), so every
  planner that groups tracks into cohorts starts asking for eight-lane banks;
* `impl Lane for wide::f32x8` (`crates/miso-engine-lane/src/simd8.rs`) is **not** `cfg`-gated. It
  is compiled on every target, and its own module comment already says what happens off `x86`:
  "`wide` lowers `f32x8` to two four-lane values; that is correct but is not a production width".

So the question the survey has to answer per effect is not "does a `Simd8` instantiation exist" —
it does, everywhere, and it compiles for `wasm32` (verified: the whole guest dependency graph
builds clean for `wasm32-unknown-unknown` with the width override, no warnings). The question is
whether each factory's **availability guard** would accept the eight-lane request or decline it
with `Ok(None)` and drop the cohort to scalar instances.

---

## The guards, effect by effect

`Ok(None)` from `bind_homogeneous_bank` is not an error. It is the contract's frozen "this artifact
cannot bank this cohort" answer, and the compiler renders every member as a scalar instance
instead — correct output, one lane at a time. That is why a guard that silently refuses is the
dangerous failure mode and not a loud one.

| effect | guard, and where | at `Simd8` on `wasm32` |
|---|---|---|
| parametric EQ | `lanes != Backend::current().width()` — `src/lib.rs:1852` | **banks at 8.** `BankWidth::Eight => prepare_width::<Simd8, 8>` |
| compressor | `Backend::current().width() != lanes`, then `match Backend::current()` — `src/lib.rs:720`, `:728` | **banks at 8.** `Backend::Simd8 => PreparedCompressorBank::<Simd8>` |
| true-peak limiter | `Backend::current().width() < request.width.lanes()` — `src/lib.rs:1977` | **banks at 8.** `BankWidth::Eight => PreparedTruePeakLimiterBank::<Simd8>` |
| gate/expander | `match request.width { Eight => Backend::current() == Backend::Simd8 }` — `src/lib.rs:1226` | **banks at 8.** |
| transient shaper | `request.width.lanes() != Backend::current().width()` — `src/lib.rs:856` | **banks at 8.** |
| multiband compressor | `has_matching_backend_width()` only; `prepare_bank` declines on program key alone — `src/lib.rs:1774` | **banks at 8.** |
| builtins | not a factory: `BankWidth::for_backend(dispatch)`, `builtins-compiler/src/lib.rs:265`, `:994`, `:1017`, with `dispatch = Backend::current()` from `host-core/src/prepare.rs:697` | **banks at 8**, `InputStageKernel::Simd8` |
| delay | returns `Ok(None)` unconditionally — `src/lib.rs:573` | unchanged: no banked kernel at any width |
| **soft clip** | **`width_is_native(BankWidth::Eight) = cfg!(any(target_arch = "x86", target_arch = "x86_64"))` — `src/lib.rs:900`** | **BLOCKER: declines, and every soft-clip cohort falls to scalar instances** |

Every fixed-size scratch array the banked kernels index by lane is already dimensioned for eight
(`MAX_WIDTH`/`MAX_LANES`/`MAXIMUM_WIDTH`/`MAX_BANK_LANES` are all `8`, in the compressor's
`design.rs:76`, the EQ's `lib.rs:89`, the gate's `kernel.rs:39`, the limiter's `lib.rs:78` and the
builtins' `lib.rs:641`), so no effect would index out of bounds at the new width.

### Blocker 1 — soft clip's target table, not its backend

```rust
// crates/miso-engine-soft-clip/src/lib.rs
const fn width_is_native(width: BankWidth) -> bool {
    match width {
        BankWidth::Four => cfg!(any(
            target_arch = "aarch64",
            all(target_arch = "wasm32", target_feature = "simd128")
        )),
        BankWidth::Eight => cfg!(any(target_arch = "x86", target_arch = "x86_64")),
    }
}
```

This is the one factory in the workspace that answers the availability question from a **table of
target architectures** rather than from `Backend::current()`. Its `Eight` arm names `x86` and
`x86_64` and nothing else, so on `wasm32` at `Simd8` the request passes
`has_matching_backend_width()` (backend and width agree: both say eight), reaches `prepare_bank`,
and is refused by `!width_is_native(request.width)` — the same `Ok(None)` the contract uses for a
genuinely absent capability. Nothing errors, nothing logs, and the whole soft-clip population
renders one lane at a time.

The failure is invisible to the standing measurement, because soft clip is not in any #175 strip
row. That is precisely what makes it a blocker: the arm that would catch it is not the arm that
would be run. The correction is one expression — the table has to become the backend question the
other eight factories already ask — and it belongs in step 3, with a bank-membership assertion on
`wasm32` behind it so the table cannot drift out of the backend law a second time.

### Blocker 2 — the harnesses assert the wasm backend by name, and would refuse the switch

`tools/miso-engine-wasm-gate-guest/src/lib.rs:81` exports the guest's backend as `0`/`1`/`2`, and
`tools/miso-engine-wasm-gates/src/lib.rs:183` computes the same code for the native process. Both
sides derive it from `Backend::current()`, so G5's own comparison moves with the switch and is
safe. What does **not** move is every place that has written the expectation down:

* `scripts/run-wasm-gates.sh:46` — `run_guest simd128 +simd128 simd4`. The third argument is
  `--expect-backend`, and `miso-engine-wasm-gates` exits with "the artifact was built with the
  wrong feature set" (`src/lib.rs:300`) when the guest disagrees. **The G5 wasm gate fails on the
  first build after the switch**, and the fix is `simd4 -> simd8` on that line;
* `scripts/run-wasm-kernel-timing.sh:183-184` — both wasm legs pass `--expect-backend simd4`;
* `tools/miso-engine-wasm-console/src/main.rs` refused any guest whose backend code was not `1`
  until this branch made the expectation a parameter of the leg being timed;
* `scripts/wasm-console-benchmark-validator.jq:81` pins the `wasm_simd128` leg's `backend` field to
  the string `"Simd4"`, and every sealed record in `artifacts/` carries that string. A switch turns
  that pin into a claim about the *history* of the arm rather than about the current build, so the
  leg naming needs a deliberate decision rather than an edit in passing.

None of this is deep. It is listed as a blocker because it is the class of thing that turns a
one-line revert into a half-day of red gates, and because the first item is a hard failure of a
named gate rather than a warning.

---

## `SVF_CASCADE_DEPTH` at the new width

`Lane::SVF_CASCADE_DEPTH` is `4` for the scalar oracle and **`2` for both `Simd4` and `Simd8`**
(`crates/miso-engine-lane/src/wide_impl.rs:32`, the fourth macro argument, instantiated in
`simd4.rs` and `simd8.rs`). It was fixed by the B2 sweep (`crates/miso-engine-lane/tests/
b2_interleave.rs`) on the native host, and it is a constant of the **vector type**, not of the
target.

That distinction is the design gap. `wide::f32x8` means one 256-bit register on `x86-64-v3` and two
128-bit registers on `wasm32`. The depth that is right for the first is not derived from anything
that is also true of the second, and the B2 sweep cannot be re-run to find out: it is a native test
that builds a `std::time::Instant`, which `wasm32-unknown-unknown` cannot construct — the same
blocker `docs/rulings/wasm-kernel-timing-interim.md` recorded for the whole kernel-timing family.

Concretely, at the standing fixture the EQ keeps two of four sections
(`docs/rulings/effect-floor-accounting.md`, EQ inventory: `live.div_ceil(depth) * depth = 2`), so
the kernel runs one pass of `svf_cascade_interleaved::<L, S = 2, D = 2>`. Its loop-carried live set
is `S * D * 2 = 8` integrator vectors plus `S * D = 4` hoisted `nc1` vectors — **twelve** vectors,
which is twelve `v128` at W4 and **twenty-four** at W8, before `svf_step`'s five temporaries and
before the twenty coefficient vectors the frame body reads. If the depth wants to be `1` on wasm at
eight lanes, the constant cannot express that today without becoming per-target, and a second
`DEPTH` instantiation in the wasm artifact is itself a reportable change: the elision gate's own
comment says a second arithmetic-carrying EQ kernel "reads to `KERNEL_ROSTER` as a kernel that
moved" (`crates/miso-engine-parametric-eq/src/lib.rs`, `cascade_sections`).

**The survey's finding is therefore: the depth is `2` at W8 on wasm, unchanged and untuned, and
nothing in the tree can tell you whether that is the right number.** The step-2 measurement can:
the `sixty_four_track_eq_only` row is the EQ alone, and a W8/W4 ratio at or above 1 on that row is
the depth asking to be re-tuned.

---

## Register pressure under Cranelift

Cranelift's `x86_64` backend has sixteen `xmm` registers, and a `v128` value occupies one. A
`wide::f32x8` on `wasm32` is two `v128` values, so **every count below doubles at W8**.

The native evidence that sixteen registers is already the binding constraint is in
`docs/rulings/effect-floor-accounting.md`: `objdump` of the compressor's `Simd8` idle body finds
**29 `vbroadcastss` per channel-frame** — "constants re-splatted inside the loop; sixteen `ymm`
registers is not enough to hold them". That is the same file size, with the same shortage, on the
target that has the *narrower* pressure of the two.

| kernel | vectors the hot loop wants live | `v128` at W4 | `v128` at W8 |
|---|---:|---:|---:|
| EQ `svf_cascade_interleaved<S=2, D=2>` — 8 integrators + 4 `nc1`, plus 5 `svf_step` temporaries | 17 | 17 | 34 |
| EQ frame body, coefficients re-read per frame (`a2, a3, m0, m1, m2` x 4 chains) | +20 | +20 | +40 |
| compressor `frames_loop` idle body — 6 splatted constants, 5 link/bypass masks, two `Coef` sets of 12, two recursive `gain_reduction_db` | 37 | 37 | 74 |
| limiter `detector_peak` history, one channel | 12 | 12 | 24 |
| limiter, both channels' histories (the shape the chunking exists to avoid) | 24 | 24 | 48 |

The limiter's own source states the rule this table is measuring against: "Both channels' twelve
history words together are twenty-four vector registers, which is more than any of the three
backends has; splitting the detector into two passes over a short chunk ... removes the spill from
the inner loop" (`crates/miso-engine-true-peak-limiter/src/lib.rs`, `DETECTOR_CHUNK = 32`). At W8
on wasm **one** channel's history is twenty-four `v128`, so the mitigation that made the W4 inner
loop spill-free is no longer sufficient at W8 — the chunk split would have to become a second split
along the FIR, or the spill returns.

Read together: none of these kernels fits the register file at W4 either, and the compressor is
2.3x over it before the doubling. So W8 on wasm is not a step from "fits" to "spills"; it is a step
from one spill rate to roughly twice it, against twice the lane-work per instruction pair. **Which
of those two wins is a throughput question and not a static one, and that is exactly why step 2 is
a measurement and not an argument.** The prediction the survey will stand behind is only the
ordering: the more a kernel's hot loop is dominated by loop-invariant coefficient vectors it cannot
hold (compressor, EQ), the less of the 2x it should keep.

---

## The measurement build, and why the default artifact is untouched

Step 2 needs two `wasm32` guests that differ in the backend constant and in nothing else. The
override is one build-time `cfg`, `miso_wasm_simd8`, read in exactly one place
(`crates/miso-engine-lane/src/backend.rs`) and set only by
`scripts/run-wasm-console-benchmark.sh --issue183` through
`RUSTFLAGS="-C target-feature=+simd128 --cfg miso_wasm_simd8"`.

It is a `cfg` rather than a Cargo feature deliberately: Cargo unifies features across every package
one invocation selects, so a feature would let one crate's opt-in silently rewidth another's. It is
declared in the workspace's `unexpected_cfgs` check-cfg list, so a typo in the flag is a warning
rather than a W4 run reported as a W8 number.

**Class A evidence.** The default `wasm32-unknown-unknown` console guest module was built from this
branch and from the same tree with the change stashed, into one target directory, and hashed:

```text
with the override present : c5f4e95f4df4ae10295354cfd91531a2ff58e0e4bd68e4612ff958ff5dfce0a6
with the change stashed   : c5f4e95f4df4ae10295354cfd91531a2ff58e0e4bd68e4612ff958ff5dfce0a6
```

Byte-identical. The eight-lane guest hashes differently
(`80978c7ad6e0c84ce740d341d566692dd7665c1688e8a0a103718ab277c9b70a`), and both the runner and the
preflight refuse the run if the two modules hash alike — a paired width measurement taken from one
module twice would report a ratio of 1.0 by construction.

The host refuses a mislabelled module in either direction: the four-lane guest in the eight-lane
slot and the eight-lane guest in the four-lane slot are both preflight failures
(`scripts/preflight-wasm-console-benchmark.sh --issue183`), because both modules export the same
names and only the backend constant they report tells them apart.

---

## What the survey does not settle

* **Whether the EQ wants a different `SVF_CASCADE_DEPTH` on wasm at W8.** The B2 sweep cannot run
  on the target. The `eq_only` row of the step-2 record is the only instrument this tree has.
* **Soft clip's actual W8 wasm throughput.** It is not in any measured row, and blocker 1 means it
  would not even be banked. Its number is unknown at both widths on this target.
* **The browser.** Every number in this family is wasmtime under Cranelift, ahead-of-time, on a
  desktop. `docs/rulings/wasm-kernel-timing-interim.md` owns that boundary and it is unchanged: a
  browser JIT tiers and deoptimises, and the shipped artifact's width decision is a field question
  the owner's browser pass answers, not this one.

---

# Step 2: the paired W4/W8 measurement

**Record.** `artifacts/issue183/wasm-console-benchmark.accepted.jsonl`, 22 rows (the eleven #175
strip workloads x two measured rounds), `status: PASS`, `measurement_control: controlled`,
candidate `b628524`, CPU 15, one untimed warmup launch and exactly two measured rounds. Two guest
modules, `f3128a27...` (W4) and `f19c35b7...` (W8), rendered inside the same observation on every
one of the 1,000 observations per row, alongside the two native legs.

**Digest identity.** `all_legs_identical` on all 22 rows: `native_simd8`, `native_simd4`,
`wasm_simd128` and `wasm_simd128_w8` produced one output digest per row, and **all 44 wasm digests
reproduce `artifacts/round1-composed/` exactly** — both the W4 leg (which must, class A) and the
W8 leg (which need not have, and does). Eight-lane wasm banks are bit-identical to four-lane ones
on every row of the standing strip. That is the lane-identity claim of master plan §1 holding
across a width change on the target, not merely on the host.

**The W4 leg reproduces the standing baseline.** 275.3 us/block against the sealed 274.9
(+0.15 %), `eq_only` 79.1 against 78.7, `compressor_only` 128.7 against 128.9,
`builtins_only` 47.4 against 47.1. The denominator of every ratio below is the number the
scoreboard already carries.

## The rows

`p50 us/block`, both rounds. `W8/W4` is the record's own `wasm_simd128_w8 / wasm_simd128` ratio,
recomputed by the validator from the two legs under it; the paired median of the per-observation
quotient agrees with it to within 0.004 on every row.

| workload | W4 r1 | W4 r2 | W8 r1 | W8 r2 | W8/W4 r1 | W8/W4 r2 |
|---|---:|---:|---:|---:|---:|---:|
| `sixty_four_track_console` | 275.032 | 275.594 | 245.958 | 245.517 | **0.894** | **0.891** |
| `one_twenty_eight_track_stretch` | 554.684 | 556.367 | 494.009 | 495.241 | 0.891 | 0.890 |
| `nine_track_ragged_strip` | 48.031 | 47.731 | 45.677 | 45.376 | 0.951 | 0.951 |
| `sixty_four_track_compressor_only` | 128.264 | 129.135 | 124.717 | 124.626 | 0.972 | 0.965 |
| `sixty_four_track_console_legacy` | 159.122 | 159.433 | 155.175 | 155.034 | 0.975 | 0.972 |
| `sixty_four_track_idle` | 78.049 | 77.647 | 76.054 | 75.984 | 0.974 | 0.979 |
| `sixty_four_track_eq_comp_simd1` | 150.366 | 150.907 | 147.841 | 147.279 | 0.983 | 0.976 |
| `sixty_four_track_dispatch_only` | 47.290 | 47.299 | 47.330 | 47.400 | 1.001 | 1.002 |
| `sixty_four_track_builtins_only` | 47.150 | 47.540 | 47.320 | 47.831 | 1.004 | 1.006 |
| `sixty_four_track_eq_only` | 79.090 | 79.181 | 79.551 | 79.290 | 1.006 | 1.001 |
| `nine_track_baseline` | 11.071 | 11.111 | 13.446 | 13.465 | **1.215** | **1.212** |

**The null exit does not fire.** The ruling condition was `W8 >= 1.8x W4`; the worst row is 1.215
and the strip rows are all below 1.01. Round-to-round agreement is within 0.008 on every row, so
none of this is noise.

## What the win is made of, and what it is not

Reading the decomposition rows as increments (round means):

| increment | W4 | W8 | change | share of the available 2x converted |
|---|---:|---:|---:|---:|
| composed console (`sixty_four_track_console`) | 275.31 | 245.74 | **-29.6 us** | 22 % |
| limiter + its rack (console minus `eq_comp_simd1`) | 124.67 | 98.18 | **-26.5 us** | **43 %** |
| compressor (`compressor_only` minus `builtins_only`) | 81.35 | 77.09 | -4.3 us | 10 % |
| EQ (`eq_only` minus `builtins_only`) | 31.79 | 31.84 | +0.05 us | **0 %** |
| builtins + graph dispatch (`builtins_only`) | 47.35 | 47.58 | +0.2 us | -1 % |

("Share converted" is `2 * (1 - ratio)`: a kernel that turned the doubled width into half the time
scores 100 %, one that gained nothing scores 0 %.)

**The issue's mechanism is refuted and the win is real anyway.** #183 projected the wasm compressor
increment 81 -> ~50-60 us from lane width. Measured, it goes **81.35 -> 77.09**: the compressor
converts a tenth of the doubled width, not the four-fifths the native W4->W8 probe suggested. The
EQ converts **nothing at all** — 31.79 to 31.84, inside round-to-round noise. Almost the entire
-29.6 us on the console row is the **limiter**, which #183 did not name, and which converts 43 %.

That ordering is the one the survey's register-pressure section predicted and it is worth stating
as a confirmed prediction rather than a coincidence: the compressor's hot loop wants 37 vectors
live and the EQ's wants 37 including its re-read coefficients, both far past sixteen at W4 already,
so doubling the vector size doubles spill traffic against the doubled lane-work and nets out near
zero. The limiter is the one kernel that hoists its live set out of the inner loop by construction
(`DETECTOR_CHUNK`), and it is the one kernel that keeps a real fraction of the width.

**The nine-track row regresses 21 %.** `nine_track_baseline` is EQ-only at nine tracks: at W4 it is
two full banks plus a one-track scalar tail, at W8 one full bank plus a one-track tail. It pays the
wider vector on the effect that converts none of the width, and it is the one row where the switch
is a clear loss. Small sessions are a real product shape, and this is a 21 % regression on one.

## Against the floor

Using `docs/rulings/effect-floor-accounting.md` (native cycles/lane-sample: compressor 3.176, EQ
1.723, limiter 4.662, builtins 2.331) at 16,384 lane-samples per 64-track block and the pinned
core's measured 5.45 GHz, the native `Simd8` arithmetic floor for the composed strip is
**35.75 us/block**. The wasm floor is that figure times the width factor: **x2 at W4** (four lanes
against the floor's eight) and **x1 at W8**.

| build | measured | its own width floor | multiple of floor |
|---|---:|---:|---:|
| wasm W4 | 275.31 | 71.50 | 3.85x |
| wasm W8 | 245.74 | 35.75 | **6.87x** |
| native `Simd8` | 124.73 | 35.75 | 3.49x |

**The switch halves the floor and moves the measurement by a ninth, so the W8 build stands twice as
far from its own bottom as the W4 build does.** That is the honest floor statement, and it is the
argument against reading -10.8 % as convergence: the arithmetic the strip requires did not get
cheaper, the machine simply retired a little more of it per instruction pair before the spill
traffic ate the rest.

## Recommendation

**Not a null exit** (the 1.8x condition is nowhere near met) and **not an unconditional switch**.
The recommendation to the coordinator is a **conditional switch, with the issue's mechanism claim
corrected**, in this order:

1. **Correct the record on #183.** The projection "compressor 81 -> ~50-60 us, and it generalises
   to every banked effect" is measured false in both halves. It does not generalise: the EQ gains
   nothing and the builtins gain nothing. The compressor itself gains 5 %. Whatever is decided
   next should be decided about the limiter, which is where the 29.6 us lives.
2. **Clear the two blockers before the one-line change**, or the switch lands red: soft clip's
   `width_is_native` table (`crates/miso-engine-soft-clip/src/lib.rs:900`) would silently drop
   every soft-clip cohort to scalar, and `scripts/run-wasm-gates.sh:46` fails G5 on the first
   build. Neither is deep; both are invisible to the arm that was just run.
3. **Then the switch is defensible on the standing strip**: the composed wasm console becomes
   **245.7 us/block** from 275.3, measured rather than projected, with every rendered bit
   unchanged. Its cost is a **21 % regression on the nine-track EQ-only row**, and that trade has
   to be accepted explicitly rather than discovered later.
4. **The design's real answer is per-effect width, and this tree cannot express it.**
   `Backend::current()` is one constant and `BankWidth::for_backend` is one law (#84 phase A), so
   "limiter at eight, EQ at four" is not a `cfg` — it is a contract change to make bank width a
   per-factory capability. The measurement says that is where the remaining value is (43 % against
   0 %), and it is issue-sized work, not a step-3 edit.
5. **Before the switch ships, re-tune `SVF_CASCADE_DEPTH` for wasm at eight lanes.** The `eq_only`
   row at 1.006/1.001 is the depth asking to be looked at, and the constant cannot express a
   per-target value today (see the depth section above). A W8 wasm EQ at depth 1 is untested and
   is the cheapest remaining hypothesis in this phase.

**Revert remains one line** (`crates/miso-engine-lane/src/backend.rs`), and nothing in this branch
has switched anything: the default `wasm32` guest module is byte-identical to `origin/main`'s.

## What an adversarial verifier should probe

* **That the W4 leg is genuinely the shipped build.** Compare `guest_module_sha256`
  (`f3128a27...`) against a fresh default `wasm32` guest build; it must match, and the W8 digest
  (`f19c35b7...`) must not.
* **That the two legs were not the same module.** The runner and the preflight both refuse equal
  digests, and the validator refuses a paired record whose two guest digests agree — but the
  cheapest independent check is that the 44 W8 digests equal the 44 W4 digests *while the timings
  differ by 21 % on one row and 0.1 % on another*. A single module timed twice cannot do that.
* **The nine-track regression.** It is the row that most resists the thesis; re-run it alone, and
  check whether the loss is the EQ's width or the bank/tail split at nine tracks by measuring a
  sixteen-track EQ-only row, which is two full banks at both widths.
* **The limiter attribution.** It is a subtraction of two rows (`console` minus `eq_comp_simd1`),
  not a direct measurement, and it carries the second rack's dispatch with it. A `limiter_only`
  decomposition row would settle it and does not exist.
* **The floor multiple.** Every figure in that table is inversely proportional to the 3.7 ops/cycle
  of `docs/rulings/effect-floor-accounting.md`, which is a measurement of one host. The *ratio*
  between the W4 and W8 multiples is not: it is exactly the width factor, and it is 2x by
  construction.
* **`descriptive_only` and the family flags.** Every row carries
  `comparable_with_console_records: false` and `browser_field_measurement: false`. None of these
  numbers is a browser number, and none of them may be quoted beside the 124.0 us native console
  figure as if it were the same measurement.
