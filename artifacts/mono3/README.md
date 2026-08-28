# mono3 / mono3-baseline — M3's cost, and phase 3's class-A OFF cost

Two arms, two trees, one pair of captures. `mono3-baseline` is `3cc44de7` (`e4691f2b^1`, the merge
of PR #230) with the arm registration and nothing else: **M3 present, phase 3 absent**. `mono3` is
current `main` `565349a6`, plus the same arm registration: **both present**. Read against the sealed
`mono2` capture the baseline arm isolates M3; read against each other the two arms are issue #210's
**C3** clause (c) — *the sealed console fixture measured feature-off vs. pre-change base on the
reserved core pair*.

There is deliberately no third directory. A sealed capture describes *a tree*, and a separate
phase-3 arm would have measured a tree `mono3` already measured — a second authority for one tree,
and an invitation to quote whichever read better.

| | |
|---|---|
| Did any rendered bit move? | **No.** Every `output_sha256` on all 32 native rows and all 96 wasm legs of both arms reproduces `mono2` exactly, and every wasm record still reports `digest_identity: all_legs_identical`. Both changes are class A and both stay class A. |
| Does phase 3's class-A OFF claim hold on cost? | **Yes.** Every row on every leg is inside ±1.5%, with no consistent sign; the two rows that could resolve better than that are under-resolved by their own round-to-round spread. |
| Does M3's "one already-computed bool" cost nothing? | **No.** `sixty_four_track_dispatch_only` **+38.98%**, `sixty_four_track_gain_pan_only` **+42.00%**, `console_mono`'s dual arm **+4.5%**. This is the finding this capture exists to report. |

## Arms

| arm | commit | tree | directory |
|---|---|---|---|
| baseline | native `cc47c8f5`, wasm `f6f63701` | `3cc44de7` plus only the runner-arm commit `f93a95a0` (and its kept refusals) | `artifacts/mono3-baseline/` |
| candidate | native `dc581f34`, wasm `e54c4042` | `565349a6` plus only the runner-arm commit | `artifacts/mono3/` |

The two runner-arm commits are byte-identical in their diffs (4 files, +70/−8, the same hunks); the
baseline's is `dc581f34` cherry-picked onto `3cc44de7` without conflict. The wasm leg's commit is
later than the native leg's on both arms for the reason `artifacts/round2-comp/README.md` records: a
kept refusal is itself a dirty tree, and the runner refuses a dirty tree, so the residue is
committed between attempts.

Both arms `measurement_control: controlled`, `cpu_affinity: 15`, SMT sibling quiet,
`render_errors: 0`, `render_total_forbidden_operations: 0`, `Simd8`, 48 kHz, 128-frame quantum,
1 000 observations, one warmup and two measured rounds, one runner invocation per leg, `raw` and
`accepted` byte-identical on every leg.

## Attempts

| arm | leg | attempts | refusals kept |
|---|---|---|---|
| mono3 | native | 1 | — |
| mono3 | wasm | 2 | `wasm-console-benchmark.attempt-1-refused.*` |
| mono3-baseline | native | 2 | `console-benchmark.attempt-1-refused.*` |
| mono3-baseline | wasm | 2 | `wasm-console-benchmark.attempt-1-refused.*` |

Every refusal is `precondition_loadavg_above_ceiling`, every one launched nothing and timed nothing
(`raw_sha256: null`, `workload_process_launches: 0`), and every one is kept rather than deleted.

# Finding 1 — M3 is not free, and the row it costs most is the row it never touches

`mono3-baseline` against the sealed `mono2` capture. Between those two trees lie exactly two merges,
#229 (track delay) and #230 (M3). Native `Simd8`, p50 ns/block, both rounds:

| row | mono2 r1 / r2 | mono3-baseline r1 / r2 | delta | |
|---|---|---|---|---|
| `sixty_four_track_dispatch_only` | 7 254 / 7 344 | 10 450 / 9 839 | +2 845 ns | **+38.98%** |
| `sixty_four_track_gain_pan_only` | 7 104 / 7 134 | 10 350 / 9 868 | +2 990 ns | **+42.00%** |
| `sixty_four_track_console_mono_dual` | 74 892 / 74 601 | 78 319 / 77 878 | +3 352 ns | **+4.48%** |
| `sixty_four_track_console_half_mono` | 63 641 / 63 401 | 65 484 / 64 923 | +1 682 ns | +2.65% |
| `sixty_four_track_idle` | 20 429 / 20 239 | 20 759 / 20 689 | +390 ns | +1.92% |
| every other row | | | | within ±0.9% |

The `console_mono` record says the same thing from inside one process, arms alternated observation
by observation:

| arm | round | `collapse_eligible` | `collapse_forced_off` | paired delta |
|---|---|---|---|---|
| mono2 | 1 / 2 | 50 627 / 50 336 | 75 845 / 76 555 | 25 017 / 25 969 |
| mono3-baseline | 1 / 2 | 50 155 / 50 566 | **78 700 / 79 190** | 28 345 / 28 373 |
| mono3 | 1 / 2 | 50 646 / 50 736 | **79 842 / 78 990** | 29 005 / 28 193 |

The **collapsed** arm is unmoved (+0.03%). The **dual** arm — the arm the collapse is switched off
on — is up 4.5%. M3's machinery costs nothing where it fires and something where it does not.

And on every leg of the wasm runner, `mono2` → `mono3-baseline`:

| row | native `Simd8` | native `Simd4` | wasm `simd128` |
|---|---|---|---|
| `dispatch_only` | **+35.83%** | **+21.41%** | **+11.98%** |
| `gain_pan_only` | **+35.58%** | **+19.41%** | **+11.41%** |
| `console_mono_dual` | +4.27% | +0.72% | +1.53% |
| every other 64-track row | ≤ +2.5% | ≤ +0.8% | ≤ +1.3% |

## Attribution: it is #230, and #229 is null

Established by unsealed reproduction runs (frozen release profile, pinned cpu 15) at each merge in
between — these consume no authorised measurement and are reported as what they are:

| tree | `dispatch_only` | `gain_pan_only` |
|---|---|---|
| `43432119` — the commit `mono2` was sealed at | 7 294 / 7 204 | 7 314 / 7 244 |
| `51aea688` — merge of #229, track delay | 7 214 / 7 294 | 7 224 / 7 264 |
| `3cc44de7` — merge of #230, **M3** | **9 939 / 9 889** | **9 869 / 9 889** |
| `565349a6` — `main` | 10 180 / 10 280 | 10 069 / 10 069 |

The control run at `43432119` reproduces the sealed `mono2` numbers to under 1%, which is what makes
the other three rows readable. #229 moves nothing. **#230 moves everything that moved.**

## What is not in the numbers, and what the code says

The regression is not the advertised bool. PR #230's body states *"M3 adds one already-computed bool
to the M2 dispatch; no measurable-cost claim is made."* The bool is indeed free. What is not free is
what the bool was conjoined **with**.

`BankChain::run` before #230 (`crates/miso-engine-rack/src/lib.rs:1472` at `51aea688`):

```rust
let collapse = self.collapse_prefix > 0
    && self.collapse_source
    && !self.collapse_forced_off
    && !self.collapse_retired
    && self.all_lanes_symmetric();
```

and after (`crates/miso-engine-rack/src/lib.rs:1572-1587`):

```rust
let witness = self.all_lanes_symmetric();
let armed = self.collapse_prefix > 0 && self.collapse_source && !self.collapse_forced_off;
...
let collapse = armed && witness && self.collapse_channels_agree;
```

`all_lanes_symmetric()` moved **out of the `&&` short-circuit chain** and is now evaluated
unconditionally at the top of every `BankChain::run`, on every chain, every block — including chains
that can never collapse. For an effect bank its answer is a bool cached at bind
(`crates/miso-engine-rack/src/lib.rs:466`, with the reasoning at 1406-1417). The **builtin input
bank has no such cache**: `InputStage::<L>::lane_channel_symmetry`
(`crates/miso-engine-builtins/src/lib.rs:1388-1425`) recomputes a 30-word bitwise channel
comparison, each word through `lane_read` (`:670-674`), which is a SIMD-register-to-stack spill and
scalar reload per word — 30 per lane per block, ~1 920 spill/reload pairs per block at 64 lanes.

That predicts exactly which rows move, and the data matches the prediction row for row:

* `dispatch_only` and `gain_pan_only` are the only two console workloads whose strip is
  `Strip::Identity | Strip::GainPan` (`tools/miso-engine-console-workload/src/lib.rs:588-600`),
  which sets `trim_db = 0.0`, `polarity_invert = false`, `hpf_hz = 0.0`, `lpf_hz = 0.0` on **both**
  channels. Their channel words are bit-equal, so the walk runs to completion on all 64 lanes. They
  are the two rows that move most.
* `console_mono_dual` is the mono fixture with the collapse forced off: symmetric by construction,
  and six slots per chain rather than three. It moves +4.5%, slightly more in absolute terms than
  `dispatch_only`.
* Every row derived from `console-sixty-four-track-intended.toml` keeps the fixture's per-channel
  builtins (ch00 is `trim_db −6.0/−5.5`, `hpf 30/35`, `lpf 19000/18500`), so the walk dies on the
  first word pair of lane 0. `sixty_four_track_console` has the same `[8, 48]` chain shape as
  `console_mono_dual` and does **not** move. That contrast is the load-bearing one.
* `plumbing_only` binds no bank chain at all and calls `BankChain::run` zero times. It does not
  move on the native `Simd8` leg of either pair beyond its own noise.

The irony worth stating: the strip-round job-1 prepared-identity elision is intact and still firing.
What makes these two rows regress is the *same* property that makes them elidable — both channels'
builtin coefficient words being bit-equal — because that is exactly the condition under which the
newly-unconditional symmetry walk cannot short-circuit.

**This is recorded as a finding, not as a rationalisation, and no fix is proposed here.** The
obvious shape of one — cache the builtin bank's designed symmetry term at bind, the way the effect
banks already do, since it is `PreparedOnly` but for the live trim ramp the `LIVE` term already
latches — is noted only so the next reader knows the regression is addressable without touching M3's
semantics. Whether that is the right fix is not this capture's call.

# Finding 2 — phase 3's class-A OFF cost is within noise (C3 clause (c): PASS)

`mono3` against `mono3-baseline`. Native `Simd8`, p50 ns/block, and the per-arm round-to-round
agreement beside each delta:

| row | baseline r1 / r2 | candidate r1 / r2 | delta | baseline agr | candidate agr |
|---|---|---|---|---|---|
| `sixty_four_track_console` | 74 832 / 74 642 | 75 353 / 75 223 | +0.74% | 0.25% | 0.17% |
| `one_twenty_eight_track_stretch` | 152 459 / 151 838 | 153 182 / 153 842 | +0.90% | 0.41% | 0.43% |
| `sixty_four_track_console_mono` | 49 043 / 48 843 | 49 014 / 49 294 | +0.43% | 0.41% | 0.57% |
| `sixty_four_track_console_mono_dual` | 78 319 / 77 878 | 78 460 / 78 560 | +0.53% | 0.56% | 0.13% |
| `sixty_four_track_console_half_mono` | 65 484 / 64 923 | 65 856 / 66 416 | +1.43% | 0.86% | 0.85% |
| `sixty_four_track_dispatch_only` | 10 450 / 9 839 | 10 439 / 10 530 | +3.35% | **6.02%** | 0.87% |
| `sixty_four_track_gain_pan_only` | 10 350 / 9 868 | 10 440 / 10 661 | +4.37% | **4.77%** | 2.09% |
| every other row | | | within ±0.6% | | |

Two rows read above 1% and **both are under-resolved rather than regressed**: the baseline arm's own
round-to-round spread (6.02% and 4.77%) is wider than the delta being read from it. They are the two
rows M3 made expensive and noisy, and they are reported as measured rather than rounded toward the
prediction.

The wasm runner's three legs settle the question the native leg leaves open. Across all 16 rows:

| leg | worst row | band over the other 15 rows |
|---|---|---|
| native `Simd8` | `plumbing_only` −10.21% | −1.18% … +0.42% |
| native `Simd4` | `plumbing_only` +5.54% | −1.09% … +1.91% |
| wasm `simd128` | `plumbing_only` +2.81% | −0.69% … +1.30% |

`plumbing_only` is a ~6 µs row that binds no bank chain, and it swings **both directions** across the
three legs on the same pair of trees — the definition of an unresolved row rather than an effect.
`dispatch_only` and `gain_pan_only`, the two rows that would show a per-bank cost if there were one,
read +0.42% / +0.42% on native `Simd8`, −0.72% / +1.91% on native `Simd4` and −0.69% / +0.97% on
wasm — no consistent sign.

**Verdict: within noise, no consistent sign, class-A OFF cost not resolvable above the machine's own
band.** That is what "delta within noise" was written to mean, and it is a measured null rather than
an absence of measurement.

One honesty note on the pairing. The C3 delta is `#231 + #232`, not `#231` alone: `main` carries the
de-versioning refactor (PR #232) that the pre-change base does not. #232 is rename-only — 146 files,
no control flow touched, and the console subject's own diff across it is nine identifier renames —
and its class-A obligation is discharged by the digest equality above. It is stated rather than
hidden, and a reader who wants #231 in isolation should ask for a capture at `e4691f2b`.

# Boundaries

* **Nothing here is a threshold.** Every record says `descriptive_only: true` and
  `statistical_method: ... descriptive only; no threshold`. The bands quoted above are read off these
  captures, not enforced by anything.
* **The M3 attribution rests on unsealed reproduction runs**, named as such. Only the three sealed
  arms (`mono2`, `mono3-baseline`, `mono3`) are authorities; the bisect table is corroboration.
* **The mechanism is read from the source, not profiled.** No `perf record` was taken against
  `lane_channel_symmetry`. The row-selection prediction matching the data on all three legs is the
  evidence offered; a profile would be stronger.
* **No resource pin moved**, so there is no byte-accounted derivation to carry: this branch changes
  four shell scripts and adds artifact directories, and touches no `size_of` the browser or C-API
  resource gates pin.
* **The `console_mono` numbers are not comparable to the session rows'** `sixty_four_track_console_mono`
  numbers; they are separate measurement surfaces on the same fixture.

# Files

| file | what |
|---|---|
| `console-benchmark.accepted.jsonl` | 46 native records, 2 rounds |
| `console-benchmark.raw.jsonl` | byte-identical to the accepted set (no record was dropped) |
| `console-benchmark.core-clock.csv` | the #184 perf-counter evidence behind the cycle columns |
| `console-benchmark.disposition.json` | PASS, controlled, cpu 15, one runner invocation |
| `wasm-console-benchmark.*` | the same for the three-leg wasm arm, 32 records |
| `wasm-console-benchmark.attempt-1-refused.*` | the kept load-ceiling refusal; launched nothing |
| `../mono3-baseline/` | the paired baseline arm, same file set, two kept refusals |
