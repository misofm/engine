# 002 DSP research corpus and conformance harness

## Outcome

Create the in-repo evidence base and reusable numerical/audio test harness before effects ship.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are retained only as byte-frozen extended compatibility corpus evidence. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Add concise Markdown notes for filters, dynamics, loudness, oversampling, true peak, delay, nonlinear antialiasing, multirate/crossover design, SIMD numerical rules, and console/DAW architecture; create an independent offline `f64` reference path, fixture loader, tolerance comparison, spectrum/impulse tests, benchmark harness and listening-test record format.

## Required public interfaces/contracts

`dsp-research/` note format requires algorithm/equations, coefficient/update rules, numerical/stability limits, latency/tail, units/smoothing, denormal/NaN handling, citations, fixtures, objective tests, benchmarks, and listening evidence. Harness API accepts deterministic `f32` blocks and reports peak/RMS/error.

## Deliverables

Research notes, citation policy, golden-fixture format, independent `f64` references, test/benchmark crate, deterministic seed policy, and blinded listening-evidence template. The architecture note compares official documentation for at least three current large-format hardware console families and two production DAWs, covering channel structure, inserts, send taps, buses, latency, automation and remote control.

## Explicit non-goals

Choosing effect UX, writing production effects, subjective claims without recorded evidence, or comparing/copying V1.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix

## Hazards/decisions

Use primary/official sources: RBJ https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html; Giannoulis/Massberg/Reiss https://eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf; BS.1770 https://www.itu.int/rec/R-REC-BS.1770-5-202311-I; EBU R128 https://tech.ebu.ch/publications/r128; Orfanidis, Vaidyanathan and Julius O. Smith for filter/multirate analysis; AES17 for measurement; and peer-reviewed antiderivative/oversampled antialiasing work for nonlinear processors. Official manufacturer/DAW manuals are evidence of workflow patterns, not academic proof of DSP quality.

## Acceptance gates with objective measurements

Each topic note has all required fields and at least two primary/official sources where available; the console/DAW matrix records common patterns, disagreements, and a reason for every adopted V2 pattern; fixture corruption is detected; the independent `f64` path cannot call the production kernel under test; scalar repeat render is bit-identical; benchmark emits machine-readable median/p95/p99/p99.9 data plus the complete machine/runtime metadata required by this issue.

## Target matrix

All build targets for harness logic; native benchmark baseline mandatory; browser/mobile may run reduced fixtures.

## Required evidence

Rendered fixtures, checksum manifest, test log, benchmark JSON, and completed listening-evidence sample.

## Decision and evidence record — Terra attempt 1

Implementation added `miso-engine-dsp-reference` (zero dependencies, independent offline `f64`
identity/reference/spectrum path) and `miso-engine-conformance` (validated planar blocks, tolerance
metrics, exact-repeat checks, SplitMix64, strict `.mepcm` v1/CRC-32C and manifest parsing). Production
crates and hosts do not depend on either harness crate; `scripts/check-conformance-boundaries.sh` enforces
that direction. The f64 oracle accepts no production closure or kernel.

Fixture corpus: `fixtures/conformance/MANIFEST.tsv` is sorted and lists eleven files under `v1/`: eight
required-rate asymmetric dual-mono impulses, 48 kHz dual-mono PRNG noise, 48 kHz mono sine, and 96 kHz
dual-mono near-Nyquist multitone. The on-disk corpus is 13,354 bytes including manifest/readme, well below
1 MiB. The generator's `--check` verifies exact generated bytes and rejects missing/unlisted files.
“Eight required-rate” is the historical issue-002 corpus label. Issue 032 preserves its bytes and
measurements, but classifies the four higher-rate impulse files as extended compatibility evidence only.

Correction made during this attempt: initial fixture encoding omitted the required `u16 encoding=1` field at
offset 22; parser/generator validation caught it as `InvalidField` before the corpus was accepted. The
encoder was corrected, all generated files were regenerated under frozen names, and `--check` then passed.
No V1/legacy source was inspected.

Executed gates (all passed):

- `cargo fmt --all`
- `cargo test --locked --workspace --all-targets` (including CRC vector, every-bit mutation, header/limit/
  truncation/trailing checks, manifest invalid classes, PRNG vectors/range, comparison/signed-zero checks,
  independent f64 identity/DFT checks, and checked-in fixture integration tests)
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `bash scripts/check-workspace-policy.sh`, `bash scripts/test-workspace-policy.sh`,
  `bash scripts/check-conformance-boundaries.sh`, `bash scripts/check-dsp-research.sh`
- `cargo run --locked -p miso-engine-conformance --example miso_engine_conformance_fixtures -- --check`
- scalar and `+simd128` Wasm, Android AArch64, and iOS AArch64 `cargo check --locked` for both harness
  libraries.

One and only one benchmark invocation was run with two internal rounds. Evidence is
`target/issue2/conformance-benchmark.jsonl`. At 48 kHz, 4096 frames × 2 channels: decode/CRC median was
5.8534 and 5.8546 ns/sample (rounds 1/2); f32-vs-f64 comparison median was 6.5690 and 6.5713 ns/sample.
The JSONL contains nearest-rank p50/p95/p99/p99.9/min/max plus machine/toolchain metadata. Host governor
was `powersave`, and unavailable git/power metadata is explicitly `unknown` with `metadata_incomplete=true`;
these are descriptive measurements only and no retry/optimization was performed.

## Sol adversarial review and correction attempt 2 — PASS

Attempt 1 did not pass the frozen brief despite its green local tests. The benchmark used abbreviated,
non-contract field names and its validator did not check the complete schema or percentile ordering; the
benchmark also depended directly on core/reference crates. The dependency guard's `awk` `END` action
overrode its failure exit. The listening format example described an unrelated TOML/SHA fixture proposal,
the note template disagreed with the topic-note headings, the manifest header/checksum convention differed
from the brief, and several claimed corruption/metric cases were not implemented.

Sol correction attempt 2 stayed inside issue 002. It restored the exact crate boundary, fixed the guard,
made the comparator rate-aware with scaled RMS accumulation and explicit finite/+infinity/-infinity/
undefined SNR states, made DFT/signal domains typed and bounded, made `.mepcm` encoding arithmetic checked,
and froze the canonical stored CRC-32C manifest convention. The fixture generator regenerated the manifest
mechanically under the unchanged eleven fixture names; `--check` proves byte identity and rejects unlisted
files. The complete fixture corpus is 15,330 bytes. Tests now cover every-bit corruption, every truncation,
all header/limit/EOF classes, 4,096 deterministic mutations without panic, strict manifest text/path classes,
SplitMix vectors, tolerance/tie/non-finite/SNR semantics, signed-zero repeatability, and known impulse/sine
DFT behavior. The research checker verifies nonempty required headings, at least two resolved bibliography
keys per note, the five-family console/DAW comparison, and explicit synthetic-listening disclosure.

Independent attempt-2 gates passed: format; workspace policy plus mutation tests; conformance dependency
boundary; research structure; locked all-target/all-feature check; all-target tests; Clippy with warnings
denied; rustdoc with warnings denied; fixture generator `--check`; separate Wasm scalar and SIMD128 checks;
and Android/iOS AArch64 checks. `miso-engine-dsp-reference` has zero dependencies, production packages do
not depend on either harness crate, and the benchmark depends only on `miso-engine-conformance`. Cargo.lock
has no registry or Git sources. No production effect/DSP kernel was added and no V1/legacy source was read.

Because the attempt-1 artifact was structurally invalid, Sol ran exactly one corrected two-round invocation
and replaced `target/issue2/conformance-benchmark.jsonl`. It contains one record per case/round with every
required exact field, a named missing-metadata array, 4,096 batch observations, nearest-rank ordered
p50/p95/p99/p99.9, and no timing threshold. Decode/CRC p50 was 5.8522/5.8558 ns/sample; comparison p50 was
3.6495/3.6581 ns/sample. The host was an AMD Ryzen 7 9700X with 8 physical/16 logical cores under the
`powersave` governor; Git/worktree and power-source metadata were unavailable and are explicitly named.
The tail variance was accepted as descriptive machine noise and was not retried or optimized.

## Oracle inventory (#105 phase 1, 2026-08)

`crates/miso-engine-dsp-reference` is the only oracle source master plan §8.3 allows for re-pinning
implementation-bit fixtures. Every module it still compiles, what it models, what its derivation was
checked against, whether it shares code with production, and who consumes it:

| module | models | derivation checked against | shares code with production? | consumers |
| --- | --- | --- | --- | --- |
| `svf.rs` (new) | Simper/Zavalishin TPT SVF, all seven types, master plan §4.2 A1 `c1` storage; the crate's single `H(z) = D + C(zI - A)^-1 B` | RBJ cookbook closed forms via `parametric_eq.rs` (E1, 102,696 probes) and the realized recurrence via the state-space iteration (E2) | no | `lr4.rs`, `tpt.rs`, wave-2 re-pins |
| `lr4.rs` (was `multiband_compressor.rs`) | LR4 crossover as two cascaded `Q = 1/sqrt(2)` SVF sections per band | the analytic LR4 identity `LP^2 + HP^2 = (s^2 - sqrt(2)s + 1)/(s^2 + sqrt(2)s + 1)` (E3) and the cascaded state space (E4) | no | `miso-engine-multiband-compressor` tests |
| `biquad.rs` | RBJ Butterworth LP/HP, transposed DF-II, plus `rbj_butterworth_magnitude_db` (moved here from `tpt.rs`) | cookbook LP/HP with `alpha = sin(w0)/(2*(1/sqrt(2)))`; strict Jury | no (`check-conformance-boundaries.sh` rule 3) | `miso-engine-builtins` tests, `miso-engine-builtins-fixture` |
| `parametric_eq.rs` | six RBJ families, direct-form I | cookbook including `alpha_S` and `beta = 2*sqrt(A)*alpha_S`; exact identity shortcut at 0 dB | no, but the **same closed forms** as `miso-engine-parametric-eq/src/lib.rs` — its independence comes from the E1 cross-derivation against `svf.rs` | `miso-engine-parametric-eq` tests, `svf.rs` E1 |
| `tpt.rs` | `ReferenceRetainedTptF32` conditioned `f32` twin; `ReferenceTptStateSpace` transfer adapter | the adapter is a thin wrapper over `svf.rs` and is `to_bits`-identical to the model it replaced (E5) | **yes** — `ReferenceRetainedTptF32::process` is the `core/arch` scalar graph transcribed; it is a twin, not an oracle, and now says so | `miso-engine-builtins` tests, `miso-engine-builtins-fixture`, `miso-engine-builtins-audit` |
| `compressor.rs` | GMR 2012 eq. 4 soft knee, one-pole dB smoother | GMR eq. 4; `exp(-1/(tau*fs))` | no | `miso-engine-compressor` tests |
| `gate_expander.rs` | static downward-expansion curve with range cap | contract text (tautological today; a real oracle comparison is #89) | no | `miso-engine-gate-expander` tests |
| `transient_shaper.rs` | two-follower contrast | contract text | no | `miso-engine-transient-shaper` tests |
| `soft_clip.rs` | 2x zero-stuff, 63-tap Blackman halfband, cubic | Vaidyanathan halfband constraints (even-index zeros, centre 1/2, off-centre sum 1/2) | no; the table is bit-compared by `miso-engine-soft-clip` after an `f32` cast | `miso-engine-soft-clip` tests |
| `delay.rs` | integer delay, 128-sample tap crossfade, feedback matrix | contract text | no | `miso-engine-delay` tests |
| `spectrum.rs` | direct DFT, bounded to 4,096 frames | the DFT definition | no | crate tests, effect tests |
| `signals.rs` | impulse / sine / SplitMix64 noise | — | no | crate tests, effect tests |
| `block.rs`, `processor.rs` | planar `f64` buffer and offline processor plumbing | — | no | crate tests |

§8.3 re-pin mapping for wave-2 jobs: EQ -> `ReferenceParametricEqCoefficients` cross-checked by
`ReferenceSvfStateSpace`; builtins HPF/LPF -> `ReferenceSvfStateSpace` + `ReferenceBiquad`;
multiband -> `ReferenceLr4Crossover`; compressor -> `ReferencePeakCompressor`; gate ->
`reference_gate_expander_gain_reduction_db`; transient -> `ReferenceTransientShaper`; soft clip ->
`ReferenceSoftClip`; delay -> `ReferenceDelayPair`; true-peak limiter -> none today (gap owned by
#90).

Gap and caveat: `ReferenceRetainedTptF32` is a bit-identity twin of production, so fixtures
regenerated from it prove reproducibility, not correctness; the correctness oracle for that
topology is `ReferenceSvfStateSpace`. Four stopped-issue research harnesses (031/042/044/045) are
archived under `dsp-research/archive/` and are not compiled.
