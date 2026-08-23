<!--
Local mirror of GitHub issue #83. The issue body below is reproduced verbatim; the AUTHORITY for
everything this workstream decides is the set of comments on that issue, not this file.
-->

# 083 Audit: workspace numeric contract, kernel API shape, and cross-crate duplication

**Authority: GitHub issue #83 and its comments.** In reading order:

1. `gh issue view 83 --json comments -q '.comments[0].body'` -- the **master plan, revision 4**. It
   decides the numeric contract (D1-D12), the `Lane` trait and its per-operation semantics (§3), the
   block-kernel contract (§4), `miso-engine-math` (§5), `miso-engine-effect-runtime` (§6), the
   profile and policy scripts (§7), the fixture re-pin policy (§8), the workstream waves (§9), the
   evals (§10) and the hazards (§11). Nothing it decides may be re-decided by a per-crate job.
2. `gh issue view 83 --json comments -q '.comments[1].body'` -- the 83a-83d execution plan, at step
   level.
3. The comment titled "Amendment to the 83a execution plan -- `wide` adoption (master plan revision
   4)", which overrides the execution plan's hand-intrinsic steps: `wide = "=1.6.1"` backends, the
   `.cargo/config.toml` x86-64-v3 pin, the `compile_error!` guard, boot attestation, no
   `#[target_feature]` wrappers, raw intrinsics only in `softfma.rs`, the G1 signed-zero cases, and
   the new G6 FTZ-inertness gate.
4. Issue #125 -- standing instructions for the agent executing the workstream.

Where this file and those comments disagree, they win and this file is corrected in the same
checkpoint (AGENTS.md: no cross-cutting change without an amended issue).

## Wave-1 job status

| job | scope | state |
|---|---|---|
| 83a | `crates/miso-engine-lane`: the `Lane` trait over `wide`, the ISA pin and boot attestation, the block kernels of §4.2, gates G1-G4/G6/P1, the lane policy scripts, this ISSUE_SPECS sync | merged |
| 83b | `crates/miso-engine-math`: vendored scalar functions and the lane-wide `exp2`/`log2`, gates M1-M3 | merged |
| 83c | `crates/miso-engine-effect-runtime`: the scaffolding of master plan §6 and the block boundary check | merged |
| 83d | release profiles, CI, the boot-attestation wiring, and the `wasmtime` cross-target gate crates (G5 and the wasm replay of M3 and D1) | delivered on branch `audit-083d-ci` |

## Evidence -- 83a

Delivered: the crate, `.cargo/config.toml`, `scripts/check-lane-policy.sh` and
`scripts/test-lane-policy.sh`, the `check-realtime-policy.sh` and `check-workspace-policy.sh`
amendments, the CI steps, and `crates/miso-engine-lane/tests/MUTATIONS.md`.

Gates, on `x86_64` with the pin applied (`vfmadd231ps %ymm` present in the release disassembly):
G1 operation identity, G2 kernel identity, G3 software FMA against hardware FMA, G4 the flush law,
G6 FTZ inertness with a non-vacuity control arm, and P1 partition invariance over blocks of 1, 7,
64, 128 and 512 frames -- all green, each with its red mutation recorded in `tests/MUTATIONS.md`.
B1 (descriptive, not a gate): `svf_block` at `Simd8` is 4.51 ns per frame of eight lanes against
11.43 ns for a per-sample function-pointer path, a ratio of 2.51x.

Deferred with owners: G5 cross-target digests and the wasm leg of G6 (83d, they need the `wasmtime`
gate crate); wiring `attest_host` into the C ABI and the hosts (#106); the `KernelTable` of function
pointers, which has no consumer until wave 2; deleting `crates/miso-engine-core/src/arch/` and the
`Prepared*KernelV1` tokens (#84).

## Evidence -- 83d

Delivered: the D12 release profile in the root `Cargo.toml`; three new dev/tooling crates
(`tools/miso-engine-wasm-gate-corpus`, `-guest`, `miso-engine-wasm-gates`);
`scripts/run-wasm-gates.sh`; the `wasm-gates` CI job and the `miso-engine-lane` additions to the
wasm/Android/iOS package lists; the D4 boot-attestation call in
`hosts/miso-engine-host-native`, `hosts/miso-engine-host-mobile` and
`miso_engine_v2_engine_create`; the `check-conformance-boundaries.sh` extension to `lane` and
`math`; and `tools/miso-engine-wasm-gates/MUTATIONS.md`.

**G5 cross-target digests, and the wasm replay of M3 and D1.** One frozen corpus
(`tools/miso-engine-wasm-gate-corpus`) compiled twice: linked natively, and built for
`wasm32-unknown-unknown` with and without `simd128` and executed under wasmtime 47.0.3
(Apache-2.0 WITH LLVM-exception, pinned exactly, a dependency of the host runner alone). 92 cases:
twelve block kernels x four signals, `lane_fma`, `exp2_lane`, `log2_lane`, the 32
`miso-engine-math` M3 cases, and the 9 `miso-engine-effect-runtime` D1 cases. The last two groups
are compared against those crates' own `M3_DIGESTS` and `D1_DIGESTS` rather than a second pin, so
the wasm run replays gates M3 and D1 instead of a transcription of them that could drift. Every
case with a lane instantiation -- which is all of them except the scalar math functions -- is
digested at `f32`, `Simd4` and `Simd8` on both legs; the kernel cases are read back lane-major so
the digest describes the arithmetic and not the AoSoA layout. The 51 lane pins were generated from
the scalar `Lane` oracle (master plan §8), never from a vector or wasm run.

| leg | backend | cases | comparisons | mismatches |
|---|---|---|---|---|
| native (`x86-64-v3`) | `Simd8` | 92 | 212 | 0 |
| wasm, `-simd128` | `Scalar` | 92 | 212 | 0 |
| wasm, `+simd128` | `Simd4` | 92 | 212 | 0 |

The `+simd128` leg is the first execution anywhere of the lane crate's `v128` software FMA
(master plan §3.5); 83a could only compile-check it.

**Red mutations** (recorded in `tools/miso-engine-wasm-gates/MUTATIONS.md`): `f32x4_relaxed_madd`
built with `+relaxed-simd` -- rejected twice, by `check-lane-policy.sh` on the source and by the
runner on the artifact (`relaxed SIMD support is not enabled`, from `wasm_relaxed_simd(false)`);
an unconditional `| 1` round-to-odd in the `v128` body -- `lane_fma` moves at `simd4` and `simd8`
on the wasm leg only; one pin byte flipped -- both legs red.

Two things the mutations found rather than confirmed. First, with the runner temporarily allowing
relaxed SIMD the digests still matched, because wasmtime 47 lowers `f32x4.relaxed_madd` to a
hardware `vfmadd` on this host: rejecting the opcode is the load-bearing check, and a digest
comparison alone would have passed that mutation. Second, the round-to-odd mutation was *green*
against the corpus as first written, whose FMA operands all produced an exact `f64` sum so the
adjustment never fired; the corpus now carries a midpoint family and the pins were regenerated
from the oracle after the correction. A distinctness assertion separately caught
`ramp_block/impulse` digesting 8,192 zeros, which would have agreed on every target while proving
nothing.

**D12 profiles.** `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `debug = 1`, with
`[profile.bench]` inheriting release. Verified from the `rustc` command lines that a release
*binary* is built with `-C panic=abort` while a release *test harness* is not, so the release
gates, the Loom race model and `#[should_panic]` still unwind. Hazard H6's rack benchmark is an
exactly-once qualification that refuses to overwrite its committed artifact, so it cannot be
re-run; that artifact records `render_panic_unwinds = 0`, so its `catch_unwind` never fired, and
the benchmark is not in CI -- `panic = "abort"` removes a diagnostic there that has never had
anything to diagnose. What `panic = "abort"` changes is
documented for embedders in `docs/REALTIME_DEPENDENCY_POLICY.md` ("Panic behaviour by profile"):
a release `libmiso_engine_capi` no longer converts a panic into `RESULT_INTERNAL`, and the
AudioWorklet artifacts trap instead. One consequence shaped the design: a lib target that emits a
`cdylib` is compiled with the profile's `panic = "abort"` and an unwinding test harness then cannot
link it, which is why the corpus is a separate `rlib` from the guest `cdylib`.

**What D12 cost, measured.** Fat LTO makes rustc write LLVM bitcode into the `.o` files three
qualification scripts disassemble, so `check-builtins-target-instructions.sh` (068),
`check-parametric-eq-targets.sh` (042) and `check-rack-instructions.sh` (008) failed with
"file format not recognized". Each now exports `CARGO_PROFILE_RELEASE_LTO=false` for its own
single-crate `--emit=obj` probe, which is the setting they were written against; they never link,
and LTO has no part in instruction selection within one crate. The profile decision was not
weakened -- cross-crate inlining of the lane kernels is exactly what finding F8 asked for.

`debug = 1` grew the browser artifacts sevenfold, and that was worth fixing rather than filing:

| scalar AudioWorklet artifact | bytes |
|---|---|
| `main`, before D12 | 2,153,061 |
| D12 as written | 16,661,225 |
| D12, debug information stripped | 1,940,472 |

Fat LTO alone makes the module about ten percent *smaller* than it was; the whole of the growth was
DWARF. `scripts/build-web-audioworklet.sh` now strips it -- in the delivery script, not in
`[profile.release]` (hazard H7), so native artifacts keep their line tables.

**Boot attestation (D4).** `miso_engine_lane::attest_host` is called at
`hosts/miso-engine-host-native::main` (diagnostic and `ExitCode::FAILURE`),
`hosts/miso-engine-host-mobile::mobile_target_smoke` (`Err(HostAttestation)`) and
`miso_engine_v2_engine_create` (`MISO_ENGINE_V2_UNSUPPORTED`). The C header previously said
`UNSUPPORTED` was reserved and never returned; it now documents the one entry point that returns
it. `host-web` is `wasm32`, where the instruction set is a build flag rather than a CPU property,
so the attestation is a compile-time no-operation and no call is added.

Three policy scripts are red on `main` and were red before this branch; they are named here so a
verifier does not attribute them to 83d. `check-capi-qualification-v1.sh` reports "accepted
authority drifted" for seven pinned files, six of which already drifted on `main` (#103's wave-0
capi work); this branch adds a seventh, `crates/miso-engine-capi/Cargo.toml`, by giving capi its
`miso-engine-lane` dependency. The pin is a record of what was qualified, so it is not refreshed
here: re-pinning it would claim a qualification nobody ran. `check-builtins-fixtures.sh` reports an
unsorted manifest and `check-web-audioworklet.sh` an unexpected `miso_engine_effect_descriptor_v1_inspect`
export, both unchanged from `main`.

Deferred with an owner: the `--lane-kernels` flag of `tools/miso-engine-realtime-audit` (eval A1)
is not delivered; the existing realtime allocation and syscall trace still runs unchanged, so the
lane kernels are not yet inside an audited allocator window. That is a bounded successor, not a
gate this job weakened.

---

*The remainder of this file is the GitHub issue body, verbatim.*

---

Audit issue (label `audit`). Cross-cutting findings that every per-crate audit issue (084–1xx)
references. Nothing here is a feature request; every item is a defect, inefficiency, or structural
choice in code that exists today, with `path:line` evidence and a concrete recommendation.

## Outcome

Decide and freeze one workspace-wide numeric/kernel contract so that the effect crates stop
re-deriving it one issue at a time, then make the core kernel API block-shaped so the contract is
cheap to honour. The four owner priorities this serves: correct/state-of-the-art math, minimal
cycles and zero render allocation, maximal FMA/SIMD, and native↔wasm bit-determinism.

## Findings

### F1. No FMA anywhere, and the one FMA backend breaks bit-identity  [critical] [determinism, simd]

- Where: `grep -rn mul_add crates hosts` → zero hits. `crates/miso-engine-core/src/arch/x86.rs:589-617`
  (`process_tpt_x86_avx2_fma_inner` uses `_mm256_fmsub_ps/_mm256_fmadd_ps/_mm256_fnmadd_ps`) vs
  `x86.rs:547-578` (non-FMA graph). `x86.rs:406-410`, `502-506`, `539-543`, `733-736`: every other
  `*_avx2_fma` function is an alias of the non-FMA graph. `crates/miso-engine-core/src/lib.rs:135-146`
  `KernelBackendV1` carries five variants including `X86Avx2` (AVX2 without FMA3), a CPU
  configuration that does not ship (every AVX2 CPU from Haswell/Piledriver on has FMA3).
- Measured (scratch experiment, Zen 5, 524,288 lane-samples through the TPT kernel with identical
  inputs): `X86Avx2` vs `X86Avx2Fma` outputs differ in **304,255 / 524,288 samples**. So a session
  rendered on a machine that selects `X86Avx2Fma` is not bit-identical to the same session on
  NEON, wasm, scalar or AVX2 — the current design side-steps this by calling the backend a
  "program key", i.e. by declaring the non-determinism acceptable.
- Why it matters: the owner's goal is maximal FMA **and** cross-target bit identity. The present
  state is the worst quadrant: FMA is forfeited everywhere except one path, and that one path is
  the one that breaks identity. Mul+add also costs two rounding sites per product-sum instead of one,
  which is exactly the precision that the f32 recurrences (F2) are short of.
- Recommendation (decision for the owner, see "Decision record"): adopt **explicit-FMA-everywhere**
  with an **exact software FMA on wasm**. For `f32` lanes the exact emulation is cheap: the
  product of two f32 is exact in f64 (48 ≤ 53 bits); compute `s = p + c` in f64 and apply the
  Boldo–Melquiond round-to-odd fix before demoting (TwoSum error ≠ 0 ⇒ set lsb of `s`), then
  `f64 → f32` once. Scratch experiment: naive `(a*b+c in f64) as f32` mismatches hardware
  `fmaf` on 32 of 20 M random triples (double rounding); the round-to-odd form mismatches **0** of
  20 M random + 2 M adversarial midpoint triples. On wasm `f64x2` this is ~10 ops per 2 lanes
  (mul, add, 6-op TwoSum, cmp, or, demote), i.e. roughly 5× a plain `f32x4` mul+add per FMA site —
  only recurrence sites need it; feed-forward products can stay mul/add if the contract says so.
  Natively, `f32::mul_add` / `_mm256_fmadd_ps` / NEON `vfmaq_f32` are the same single rounding,
  so **all native targets become bit-identical to each other and to wasm by construction**.
  Delete the `X86Avx2` (no-FMA) backend and the aliasing `*_avx2_fma` shims; the backend enum
  becomes {Scalar, Simd4 (NEON/wasm), Simd8 (AVX2+FMA)} with identical numerics.
  Rust never contracts `a*b+c` implicitly, so "fusion only where `fma` is written" is enforceable
  with a grep/policy script, as the repo already does for allocation.

### F2. Recursive-filter topology, not f32 per se, is what sank 012/042/044/045  [critical] [dsp-math]

- Where: `.github/ISSUE_SPECS/045-*.md:38-39` ("f64 production state ... forbidden");
  012/042/044/045 record STOP on retained-f32 recurrence gates; `031` closed NO ADOPTION.
  Production parametric EQ: "endpoint-conditioned delta" recurrence
  (`crates/miso-engine-parametric-eq/src/lib.rs:1882-1895`, core `scalar.rs:870-908`) — this is
  direct-form I in f32 with a re-labelled denominator, with `scale`/`q2` rounded in f32 on the
  render path. Builtins use a TPT SVF in f32 (`scalar.rs:842-866`) and pass.
- Measured (parametric-eq audit, scratch harness over the frozen 1,488-row grid + 48 impulse
  cases): the shipped graph fails 483/1,488 rows (worst 12.49 dB; a 10 Hz +24 dB bell renders
  +7.9 dB) — exactly the numbers 045 recorded. A correctly mapped **f32 TPT SVF** (Zavalishin /
  Simper; 6 coefficient words, 2 states, no division) passes **every** frozen gate: 0/1,488
  analytic failures (worst 0.00068 dB), 48/48 impulses within 0.0091 dB, 1 M-sample noise bounded.
  With f64 state the impulse error drops to 0.00042 dB — margin, not a requirement. Separately,
  the "recovery" gate rejects subnormal state, which every correctly decaying impulse crosses;
  that gate is unsatisfiable by any correct filter without a flush (see the EQ issue).
- Why it matters: four issues were spent inventing bespoke direct-form recurrences and arguing
  about f32 vs f64, when the standard answer (the same TPT SVF the builtins already use) passes.
  The policy ban on f64 state is a blunt instrument; the real rule is "no direct forms for
  recursive audio-rate state".
- Recommendation: freeze the contract as: **f32 lanes; recursive filters are TPT/SVF (or
  equivalent zero-delay-feedback / coupled forms); direct-form I/II recurrences are forbidden at
  audio rate; f64 state is a per-kernel opt-in when a frozen gate needs the margin** (e.g. very
  low f0 × high Q at 44.1 kHz). Realise the parametric EQ as cascaded TPT SVF sections with the
  Simper/Zavalishin shelf/peak mapping; Orfanidis Nyquist-matching only if a gate demands it.
  Flush state with one threshold+select per block, and rewrite the subnormal "fault" gate.

### F3. Per-sample kernel API: one sample per function-pointer call with full re-validation  [critical] [perf, arch]

- Where: `crates/miso-engine-core/src/arch/mod.rs:396-440` (`process_tpt`: 8 length checks + mask
  scan + indirect call per sample), `:480-539` (`process_delta`: 13 checks), `:576-619`,
  `:656-681`, `:722-758`; every kernel is `#[inline(never)]` (`x86.rs:361,400,480,508,545,587,626`).
  Caller shape: `crates/miso-engine-builtins/src/lib.rs:1115-1140` copies 8 lanes into a local
  array, calls `self.left.process(...)?` then `self.right.process(...)?` per sample, copies back.
  Coefficients are re-loaded from memory each sample (`x86.rs:551-557`); state round-trips memory
  each sample (`x86.rs:575-577`).
- Measured (scratch, Zen 5, AVX2, same arithmetic, bit-identical output): per-sample API
  4.42 ns/frame-of-8 vs block loop with register-resident state 3.05 ns (1.45×) vs 4 independent
  banks interleaved 2.56 ns (1.73×). Zen 5's OOO hides most of the call overhead; expect a larger
  ratio on mobile cores and under wasm where each slice check is a bounds-checked load.
  The structural cost is larger than the measured one: the API makes it impossible to (a) keep
  state in registers, (b) interleave L/R or multiple cohorts to fill the recurrence latency
  (2 sample-dependent rounding sites ≈ 8–10 cycles of dependent latency per sample, during which
  a single chain leaves the FMA ports ~70 % idle), (c) amortise coefficient loads and mask
  selects across a block, (d) let the compiler fuse adjacent slots.
- Recommendation: replace the five `process_*` per-sample tokens with a **block kernel contract**:
  `fn process_block(&mut State, &Coeffs, io: &mut [f32] /* frames*lanes AoSoA */, frames)` per
  backend, validated **once per block** (or once at prepare time since shapes are plan-fixed), with
  state loaded to registers before the frame loop and stored after. Keep one scalar reference
  body written in the same op order; instantiate it over a small `Lane` trait
  (`splat/load/store/add/sub/mul/fma/select/cmp/and/andnot`, ISA-pinned semantics for min/max/NaN
  as in prior art) so the scalar, NEON, wasm and AVX2 bodies are one generic function, not five
  hand-copied graphs (today `scalar.rs`, `x86.rs`, `aarch64.rs`, `wasm32.rs` each hand-transcribe
  every kernel — any op-order slip silently breaks identity).

### F4. Per-intermediate finite checks and integer modulo on the soft-clip hot path  [high] [perf]

- Where: `crates/miso-engine-core/src/arch/x86.rs:469-478` `soft_clip_checked_x86` stores the
  vector, runs scalar `is_finite/is_subnormal` on 8 lanes, reloads — called after **every**
  mul/add (`:432-436`, `:457-460`), i.e. ~70 store/check/reload round-trips per sample per phase;
  `:424-430` gathers 31 taps × 8 lanes with `(cursor + 63 - tap) % 63` — 248 integer divisions
  per phase per sample; `scalar.rs:774-792` same in scalar. `x86.rs:459` divides by 3.0 per sample.
- Recommendation: NaN/Inf propagate, so test once per block at the output with one vector compare
  (`cmp_ps(x,x)` / `abs(x) < HUGE`); flush subnormals once per block at the state boundary with a
  threshold-select; use a power-of-two history (64) with an AND mask, or a linear double-buffered
  history so the FIR is a straight dot product; exploit the half-band structure (every other tap
  zero + symmetric ⇒ 16 distinct coefficients, polyphase decimation/interpolation halves the work
  again); multiply by a precomputed `1/3` if the cubic is retained, or replace 2× oversampling of
  a cubic by first-order ADAA (Parker, Zavalishin, Le Bivic, DAFx-16) which needs no FIR at all.
  See the soft-clip crate issue for the full op count.

### F5. Coefficient-only arithmetic and a division inside the parametric-EQ per-sample kernel  [high] [dsp-math, perf]

- Where: `crates/miso-engine-core/src/arch/x86.rs:654-661` — `scale = (d0 - a*d1) + d2`,
  `q2 = (d1 - a*d2) - a*d2`, then `y = (num - history) / scale` per sample; same in
  `scalar.rs:887-894`, and the NEON/wasm twins. `a, d0, d1, d2` are coefficients.
- Why: 4 multiplies, 4 add/subs and one `vdivps` (≈11-cycle latency on the recurrence) per sample
  that are constants of the coefficient set. The division sits on the feedback path.
- Recommendation: fold `scale` into the numerator/denominator coefficients at coefficient-update
  time (normalise so the leading denominator coefficient is 1, as every textbook biquad does), and
  drop the structure altogether per F2.

### F6. Every effect crate re-implements the same runtime scaffolding  [high] [duplication, arch]

- Where (function names × crates, `crates/miso-engine-*/src/lib.rs`): `write_u32/write_f32/
  read_u32/read_f32` ×7, `snapshot_state_payload/restore_state_payload` ×8,
  `snapshot_track_state_payload/restore_track_state_payload` ×7, `validate_state_lengths` ×7,
  `normal_or_zero` ×7 (e.g. compressor `:1212`, gate-expander `:1113` — byte-identical),
  `normalize_zero/negative_zero` ×7, `sanitize` ×9, `recover` ×8, `bind_homogeneous_bank/
  prepare_homogeneous_bank/process_bank` ×7, `apply_automation` ×8, `discontinuity_reset` ×8,
  `parameter_value_valid` ×7. Envelope followers, dB↔linear conversions and attack/release
  coefficient maths are also private to each crate (see compressor, gate, transient-shaper,
  multiband issues).
- Why: the prior engine's most expensive bug class was one law with two homes. Here each law has
  seven. A fix to state serialisation, sanitisation, or the bank binding must be applied seven
  times, and each copy is free to drift (the audits found several already differ subtly).
- Recommendation: one `miso-engine-effect-runtime` (or a module in `effect-contract`) owning
  state-payload codec, lane read/write, sanitise/recover, homogeneous-bank binding, parameter
  validation/automation application, and shared DSP primitives (one-pole envelope, dB/linear
  via a **self-contained, target-deterministic** polynomial `exp2/log2` — see F7). Effects keep
  only their own equations.

### F7. `f32::exp/ln/powf/tan/sin/cos` from libm on the control and (in places) render path  [high] [determinism]

- Where: counts in `src/*.rs`: builtins ln/log 12, powf 4, sin/cos 27, tan 1; parametric-eq
  exp 3, ln 13, powf 4, sin/cos 6; compressor exp 2, ln 1, powf 1; gate exp 2, powf 3;
  true-peak exp 1, powf 2; soft-clip sin/cos 9; transient-shaper ln 5; multiband exp 2, ln 4, tan 1.
  (Per-crate issues cite the exact lines and which are render-reachable.)
- Why: Rust's `f32::exp/ln/powf/sin/cos` resolve to the platform libm (glibc, Apple libm, Android
  bionic, wasm's compiler-rt/musl port). They are **not** correctly rounded and **not** identical
  across those libraries, so any coefficient derived from them differs by ulps between native and
  wasm, and a session will not render bit-identically even if every kernel does. Trig/tan on the
  coefficient path is expected, but it must come from one engine-owned implementation.
- Recommendation: an engine-owned `miso-engine-math` module with deterministic, documented
  polynomial/rational `exp2`, `log2`, `tan`/`sin`/`cos` (range reduction + minimax, plain
  mul/add or explicit fma), used by every coefficient update and every dB↔linear conversion. Ban
  `std` transcendental calls in production crates via the existing policy-script mechanism.

### F8. No release profile; kernels cannot be cross-crate inlined  [medium] [perf]

- Where: root `Cargo.toml` has no `[profile.*]` section; the hosts and capi link many crates.
- Why: defaults (codegen-units=16, no LTO, `panic=unwind`) prevent the per-sample `process_*`
  calls from being inlined or the block loop from being specialised across crate boundaries, and
  `panic=unwind` adds landing pads on every render-path call in the C ABI/wasm hosts.
- Recommendation: `[profile.release] lto = "fat", codegen-units = 1, panic = "abort"`, plus a
  `[profile.bench]` inheriting it; keep `debug = 1` for symbolised traces if the audit tooling
  needs it.

### F9. Evidence/qualification machinery dominates the codebase  [medium] [arch, test-scaffold]

- Where: `tools/` 19 crates, `scripts/` 100+ shell/jq validators, 82 issue specs; several
  production crates carry `sha2` (`builtins-compiler`, `graph-compiler`) for fixture hashing.
- Why: not a render-path cost, but it is the dominant maintenance cost and it makes every
  structural fix (F1–F3) expensive because dozens of frozen hashes/fixtures pin the current
  bit-pattern, including bit-patterns the audit shows are wrong (per-sample order, no-FMA graph).
  Pins captured from the current kernels pin the defects.
- Recommendation: before landing F1–F3, classify every fixture hash as "pins a contract" vs "pins
  the current implementation"; the latter must be regenerated from an independent f64 oracle, not
  from production output. Move `sha2` out of production crates (it is only needed by fixtures and
  the package CID).

## Decision record (owner decisions requested)

1. Numeric contract: (a) f32 lanes + no FMA (status quo, deterministic, least precise),
   (b) f32 lanes + explicit FMA everywhere with exact soft-FMA on wasm (one rounding per site,
   native targets identical by construction, ~5× per-site cost on wasm f32 sites — only the
   sites that need it), (c) f64 state for selected recursive kernels (mul+add or FMA; ~20 ops
   per f64x2 FMA site on wasm). Recommendation: fix topology first (F2: TPT/SVF, no direct forms)
   which the data shows is sufficient in f32; then adopt (b) as the workspace contract so FMA is
   no longer forfeited; keep (c) as a per-kernel opt-in with a measured gate.
2. Backend matrix: drop `X86Avx2` (no-FMA) and the aliasing FMA shims; three backends with one
   generic body.
3. Block kernel API (F3) is a prerequisite for every per-crate perf item; schedule it first.

## Gates for the follow-up implementation issues

- `N lanes == N scalars` bit-identity gate per kernel, all backends, incl. NaN/±0/subnormal edges.
- Cross-target gate: native AVX2, NEON and wasm (wasmtime) render the same fixture to the same
  SHA-256, for every effect, with the chosen contract.
- Partition invariance: block sizes {1, 7, 64, 128, 512} produce bit-identical output.
- Render allocation counter = 0 and syscall trace clean (existing tooling).
- Cycle/sample per kernel recorded before/after; no gate on the number during feature work.

## Index of per-crate audit issues (same audit, 2026-08-22)

| # | Crate(s) | Headline |
|---|---|---|
| #84 | core | per-sample fn-pointer kernel API (2.9×), soft-clip kernel 37× off, FMA backend non-identical, SPSC false sharing |
| #85 | builtins | AVX2 bank measured **slower than scalar**; 11 classify ops per section per sample |
| #86 | builtins-compiler | partial cohorts never banked (≤7 tracks on AVX2 = zero SIMD); banked vs tail tracks use different arithmetic |
| #87 | parametric-eq | shipped recurrence is DF-I with f32-rounded denominator (483/1,488 rows fail); f32 TPT SVF passes every gate |
| #88 | compressor | libm `exp/powf/log10` per lane-sample; W8 bank slower than scalar; per-sample coefficient recompute + `%` |
| #89 | gate-expander | libm per sample (1–2 ulp native/wasm drift measured); W8 slower than scalar; duplicated detector ring |
| #90 | true-peak-limiter | libm per sample; step-attack gain law; FIR scalar inside bank |
| #91 | soft-clip | 258 store/check/reload wrappers per sample (95 % of time); polyphase half-band is bit-identical and 73× faster |
| #92 | transient-shaper | `log10`×2 + `powf` per lane-sample; <2 % SIMD coverage; 7-crate scaffolding duplication |
| #93 | delay | per-tap 64-bit `div` from `%` on non-pow2 ring; no block path for delay ≥ block; 19 classifies/frame |
| #94 | multiband-compressor | identity/active path 2× discontinuity at crossover; W8 = scalar speed; 80 % diverged copy of compressor |
| #95 | effect-contract / effect-compiler | per-value classification contract; dead automation runtime; smoothing rule copied into every effect |
| #96 | rack / rack-compiler | cohort compiler output is report-only — rendered banks come from a second algorithm; N transposes per chain |
| #97 | effect-package | O(n²) canonical scan = 5.6 s at cap; dead `compile.rs`; state verifier doesn't check its own hash |
| #98 | graph | **correctness bug**: `GraphExecutor` renders a bank at its first member under an ID-ordered schedule → stale inputs; per-edge copies + scalar pairwise reduction |
| #99 | graph-compiler | **correctness bug**: levels emitted in Kahn pop order, native blueprint requires ascending → valid sessions rejected; no lowering pass |
| #100 | native-scheduler | serial coordinator data movement (1.74× on 7 workers); unbounded spin-wait on a dead worker; 100 % spin between blocks |
| #101 | source | **bug**: second seek between blocks kills the worker silently; idle worker spins a core; per-sample decode |
| #102 | protocol | automation queue never reaches render; 2^depth re-encoding; ~9.8k lines of hand codec with no schema |
| #103 | capi | 400-line verbatim copy of host-web; `RefCell` error slot raced between threads via `const` API; missing `isize::MAX`/alignment checks |
| #104 | tools/, scripts/ | SHA-256 inside timed interval; runner exports none of the env vars it records; 14 allocator wrappers |
| #105 | conformance / dsp-reference / target-smoke | conformance harness only ever runs against its own mock; `realtime-audit` feature leaks into host builds |
| #106 | hosts | web host is a diverging copy of capi runtime; plan drop (free) on the audio thread on failure; ≤1 quantum per message source feed |
| #107 | session | canonical float round-trip not bit-exact (2 values + `-0.0`); O(routes×tracks) validation run twice; ~505 allocs/track parse |

## Suggested order for follow-up agents

1. Correctness first, all small: #98 F1, #99 F1, #101 F1, #94 F1, #103 F2/F3, #107 F1.
2. Contract decisions in this issue (F1 numeric contract, F2 topology rule, F3 block kernel API, F7 deterministic math library, F8 profile) — these gate everything below; do not fix libm/FMA/kernel-shape findings crate-by-crate.
3. Shared runtime crate for effect scaffolding (F6) and the engine-owned `exp2/log2/tan` (F7), then re-land each effect on it: #87, #88, #89, #90, #91, #92, #93, #94.
4. Plan lowering + bank/cohort unification: #96, #99 F2/F3, #86 F3, #98 F2/F3.
5. Scheduler/source/host/protocol structural items: #100, #101, #106, #102, #103.
6. Tooling consolidation and fixture re-pinning from independent oracles: #104, #105, and F9 here.

## Project status and execution order (reviewed 2026-08-23)

Closed as superseded on 2026-08-23: 046, 047, 049, 051, 052, 054, 055 (per-effect qualification of the pre-audit implementations), 077 (AudioWorklet identity binding). Their surviving content is in the #85–#94 plan evals, #106, and #26. Wave-0 fixes already landed through Sol's flow: #99 F1 (#122), #101 F1 (`dfdefff`, #112); #98 F1 is in flight as #123.

Execute in this order; items in the same row run in parallel on separate branches. Every job reads this comment (revision 3) and its own issue's plan comment first.

| Step | Jobs | Gate to move on |
|---|---|---|
| 0 | #123 (finish), #94 F1, #103 F2/F3, #107 F1 | each plan's wave-0 regression test green; CI green |
| 1 | 83a `miso-engine-lane` ∥ 83b `miso-engine-math` ∥ #105 phase 1 (f64 SVF/LR4 oracles) | G1–G5, M1–M3 green incl. wasmtime job |
| 1b | 83c `miso-engine-effect-runtime`, then 83d profiles/policy/CI | §10 table green; policy mutation tests red→green |
| 2 | #85, #87, #88, #89, #90, #91, #92, #93, #94 (rest) — all parallel; then #86 phase A; then #84 phase A (delete `core/arch`) | per-plan evals: lane identity, partition invariance, cross-target digest, oracle bound, allocation audit |
| 3 | #96 → #99 (rest) → #98 (rest); #86 phase B; #95 | bank membership unchanged on the 100 layouts; bits unchanged across cohort boundaries |
| 4 | #102 → #103 (rest) → #106; #100; #124; #97; #107 (rest); #84 phases B–D | each plan's evals; #26 deadline-miss gate dry run |
| 5 | #104, #105 phase 2, #84 phase D leak gate | one descriptive benchmark per effect, run once |
| then | #15, #17 (new effects on the foundation), #25, #74, #26 (single release qualification), #111 listening (after #85) | — |

Process notes for the executing agents: work on the audit issues directly with checkpoint commits and plan-comment updates — do not spawn a new issue per attempt; a plan step that cannot be met is reported on the issue as a bounded fallback (each plan §10), never by loosening a gate; the `.github/ISSUE_SPECS` mirror for 083–124 is created by the first wave-1 job (83 execution plan step 0).

