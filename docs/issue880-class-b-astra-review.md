# Issue #880 Class B and #902 — Astra adversarial review

Reviewed 2026-09-25 by Astra xhigh. This is the first coherent adversarial verdict for
each task below. The reviewed source is `b2b5337a60b32493ebdc29444d7786ef93cd6e19`
on `codex/batch-880-class-b`, compared with merged Class A baseline
`a5cb5d8e066ed9a7f01ab3a96cba0d18f14a1b41`. Unchanged Class A implementation was
not reopened. No production source was edited and no timed workload was run.

| Task | Attempt | Verdict | Blocking findings |
|---|---:|---|---|
| MB-1: approved L3 logarithm | 1 | **PASS** | None |
| MB-2: approved X7/X8 shaper, including delegated single-row R2 amendment | 1 | **PASS** | None |
| MC-2: approved coefficient-domain compressor ramps | 1 | **PASS** | None |
| #902: MQ-1 extraction and offline baseline recovery | 1 | **PASS** | None |

These are source/evidence verdicts, not delivery or browser-artifact qualification.
The current AudioWorklet pin still identifies Class A. #905 must qualify the accepted
Class B source before delivery; exact-head CI, merge, GitHub synchronization and
worktree cleanup remain root's responsibility. No issue closure is claimed here.

## MB-1

The production body matches R1's exact coefficient words, mantissa fold, division,
Horner order and unfused final summation. The division denominator stays away from
zero, and clamp/anchor semantics are retained. No target-specific implementation,
table lookup, fused operation or render allocation was introduced.

The M1 ceiling remains 2 ulp. Only the logarithm's measured-worst point and its
per-row lower tripwire floor change. The revised sweep also joins monotonicity
across worker boundaries and reduces each maximum together with its witness bits,
avoiding the old independently updated maximum/witness pair. The all-pattern M2
test actually evaluates scalar, Simd4 and Simd8 for all `2^32` inputs, in disjoint
eight-aligned ranges. Its clamps make the compared results finite even for NaN
inputs; this does not create a general NaN-payload determinism promise.

The implementation's exhaustive evidence reports 1.298297 ulp, zero reversals and
all-width identity; both fresh L3 mutations exceed the unchanged M1 limit. The
implementer confirmed that those raw exhaustive outputs exist in its tool
transcript, with committed numerical summaries in `issue880-mb1.md`, rather than
separate retained raw files. I inspected the gate code and summaries; I did not
independently rerun or claim filesystem inspection of those exhaustive outputs.
An independent cheap logarithm subsample passed: 519,811 inputs, maximum 1.045646
ulp and zero reversals. Required delivery CI still runs exhaustive M1.

The diff moves only the approved logarithm M2, D1 level and Wasm logarithm pins for
MB-1. Exp2 is unchanged, consistent with declined R4. The combined full F1 log
reports exact level error `1.538161e-5` dB against fast `2.810286e-5` dB, ratio
1.827, inside the unchanged two-times gate. MB-1 receives no shaper timing credit:
the integrated shaper no longer consumes this logarithm.

## MB-2 and the R2 amendment

X7/X8 replace only the two approved conversions. Existing contrast/shape clamps,
dual-mono state, gain mixing and the `shape == 0` dry select remain intact. Clippy
exceptions are attached to the named call sites. Default F1 includes both crossing
checks and the eight-crossing count; the separate exhaustive tests cover their
full domains, out-of-domain clamp rails and the four-corner pipeline. The three
shaper pins move once in the final diff; scalar pin provenance and separate width
and Wasm comparisons are documented.

The original 96-sample oracle row exceeded `< 2.0e-5`; that failure is preserved,
not relabelled a pass. The owner's explicit delegation authorizes root's amended
`< 2.5e-5` bound for that row alone. The implementation changes exactly that
assertion. The observed maximum `2.098083496e-5`, programme null of
`7.629394531e-6` (`-102.350199` dBFS), and maximum nonzero sample-magnitude change
`0.000013073` dB support the stated bounded engineering acceptance. They are not
a full parameter-domain audibility proof. The programme dump was temporary; I
reviewed the recorded procedure, fixture/source/harness hashes and measurements,
not retained PCM. No blinded listening evidence is claimed or credited.

The oracle header correctly limits MA-5's gain measurements to its four amount
corners. The unchanged `0.01` dB impulse/step/decay limits remain in force, as do
F1/M1 and identity gates. The combined full F1 log passes all eight sweeps. Its
`1.654020e-5` dB fast/exact gain difference uses L3 as comparator; the historical
`1.654115e-5` figure is MA-5 provenance, not a new integrated measurement.

The single integrated MQ-1 record preserves the frozen programme hash, one warmup
and two observations per arm: bank `4.276925/4.305019`, scalar
`30.301938/30.328344` ns/lane-sample. I checked the recorded benchmark and runner
hashes against its candidate commit. These observations remain descriptive; I do
not promote them into a release budget or an isolated causal estimate.

## MC-2

The event path designs one exact f64 target coefficient per changed lane/channel
and time parameter. It uses the live coefficient as the start, so a mid-ramp
retarget is continuous. The sample loop excludes attack/release from
`design_lane` and advances preallocated coefficient ramps instead. The 64th sample
snaps to the exact target. Other coefficient updates retain their previous law.
Validated positive time constants and bounded interpolation keep the ballistic
coefficients finite and positive; no new render I/O, allocation, lock, syscall or
unbounded loop appears.

The cancellation case is handled: returning to the current milliseconds can leave
a coefficient still needing to reach that time's exact design. A zero-step
parameter ramp keeps the existing ramp-prefix scheduler live until that return
finishes. The public current/target values remain in milliseconds. Reset paths
seed both representations, and mono symmetry and state copying include all four
fields of the auxiliary ramps. The focused tests exercise those transitions,
including retargeting after the mono path reopens.

The 22-word payload and layout identity remain unchanged. Restore explicitly
reconstructs coefficients from current/target/remaining, and the new tests check
that reconstruction, endpoint arrival and partition invariance. This acceptance
does **not** claim that an active restore continues the unsaved coefficient
trajectory bit-for-bit; idle restore retains its existing exactness gate. The
issue and implementation notes make that limitation explicit.

Only `dual_mono_ramping` moves in C1; the three static rows remain fixed. The
combined package log includes the new coefficient tests, payload/mono/partition
coverage, and the live allocation-counter conformance tests. Native and both Wasm
legs agree with the scalar corpus.

MQ-2's source diff changes coefficient-call accounting and its record schema,
without changing the timed audio/event workload. Counts 128/256/0 follow from
eight banks × eight lanes × two channels × changed parameters at event time.
I verified the candidate tree, runner/benchmark hashes and raw measurements
against `mc2-record.json`, including its per-round means/maxima through the
validator. Baseline records remain byte-for-byte unchanged. No timing was rerun.

## #902 and the MB-2 runner integration

Extraction accepts the actual libtest-prefixed line and requires one complete
result object. It rejects missing/duplicate markers, malformed JSON, nonfinite or
nonpositive observations and incomplete arrays. The selected MB-2 source check
requires ancestry and equality of the cited render sources, and the record keeps
the selected source/revision separately from the candidate commit.

Offline recovery validates the archived raw hash and the original failure
disposition, compares extracted values with the recorded observations, and writes
a separate no-clobber result with explicit recovery provenance and zero workload
invocations. I verified that every original metadata/measurement field survives
promotion and that all three original MQ-1 artifacts are identical to baseline.
The recovered values remain attributable to `2a8977f5`; they are not new timings.

Independent checks passed:

- Shell syntax for both runners and the MQ-1 helper/self-test.
- MQ-1 self-test with source `e00d4c2d` and revision `mb2-fast-db`, including full
  fixture promotion, provenance rejection, failure propagation and overwrite
  refusal; zero timed workload launches.
- Additional scratch parser probes: Infinity, negative Infinity, numeric overflow,
  negative timing, a second JSON object, trailing text, two same-line markers and
  a marker glued to its prefix all rejected.
- All recovered/new MQ-1 and new MQ-2 records passed their validators, with source
  hashes and original-artifact preservation independently checked.

`native.log` ends at the intentionally rejected old E1 selection. I do not count
that file as a complete green self-test run. The corrected selected-source test
above independently supplies the missing result. Other retained combined logs
show default/focused release tests, strict workspace Clippy, policy gates, full F1
and native/scalar-Wasm/SIMD-Wasm passing; each Wasm leg reports 141 cases and 355
comparisons with zero mismatches. Review-only logs are under
`/tmp/issue880-astra-class-b-review/`; combined logs are under
`/tmp/issue880-class-b-gates/`.

## Nonblocking documentation notes

- `lane_math.rs`'s module prose calls `P3` degree 3; the three coefficients form a
  degree-2 polynomial, with `r = z * P3(z)` degree 3. The function-level operation
  description and implemented polynomial are correct. Prefer “three-coefficient
  polynomial” in the module prose.
- `fast-db-tier-boundaries.md` now says eight adopted crossings beside the
  historical 22.64% console figure. That figure belongs to the earlier six-crossing
  experiment. Preserve that attribution explicitly; this review credits only the
  new MQ-1 observations for the shaper, not a remeasurement of that console claim.

Neither note changes an algorithm or numerical gate. There are no blocking
findings and no second implementation attempt is required by this review.

## Editorial confirmation — 2026-09-25

Astra xhigh inspected Luna's editorial checkpoint
`bb9efd095bd8d70be8b8503cba6c3617fd51b74e` against its parent `053d427c`.
Both nonblocking notes above are resolved: the module correctly distinguishes the
degree-2, three-coefficient `P3` from degree-3 `r`, and the historical console
performance prose/table explicitly attributes 22.64% to X1–X6 rather than X7/X8.
The diff touches only those two prose paths. A byte comparison after excluding
Rust module-documentation lines confirms all remaining Rust content is identical;
no gates, constants, operation order or numeric pins changed. No tests or timings
were rerun for this editorial confirmation.

The existing attempt-1 PASS verdicts stand. Exact source
`bb9efd095bd8d70be8b8503cba6c3617fd51b74e` is accepted for #905 qualification.
This confirmation is not a new implementation attempt or an artifact PASS.
