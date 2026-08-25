# 146 Canonical FP environment at every render entry: clear FTZ/DAZ, render, restore (fixes the G6 divergence)

Successor to #144 item 1, escalated to a correctness fix by the merged G6 reproducer.

**Authority: GitHub issue #146.** Its body is the stateless brief. This file is the local decision
record and evidence log; it never replaces the issue body.

## The defect

Under hardware FTZ+DAZ, 69-70 of the 331 cross-target corpus comparisons render off-pin: the
recursive SVF, the feed-forward lane, scalar math, and the effect/builtin chains all produce
transient intra-block denormals that the master-plan D7 flush law -- a *state* law -- cannot reach.
Browser Wasm is unaffected: the core specification mandates round-to-nearest-even and full subnormal
arithmetic, confirmed by the three-browser digest parity. The exposure is every native host with an
FTZ audio thread, which is every DAW callback.

## The decision, as implemented

Pin the floating-point environment at the boundary, not in the kernels.

* `crates/miso-engine-lane/src/fpenv.rs` owns `CanonicalFpEnv`: `enter()` saves the caller's control
  word and installs the canonical one; `Drop` writes the caller's exact word back. It is neither
  `Send` nor `Sync`.
* The canonical word is the architectural default: MXCSR `0x1F80` on x86 (all six SIMD exceptions
  masked, round-to-nearest-even, FTZ clear, DAZ clear) and FPCR `0` on AArch64 (`RMode`
  round-to-nearest, `FZ`/`FZ16` clear, no traps, no FEAT_AFP alternate handling). Installing the
  whole word also removes a caller's directed rounding mode and any unmasked exception that would
  trap inside a render -- both break determinism exactly as FTZ does.
* Render entries that pin: `miso_engine_v2_render_f32_planar` (C ABI) and
  `miso_engine_host_core::StartedRenderSessionV1` (every embedding host). Browser Wasm is
  deliberately **not** an entry: on a target with no control word `CanonicalFpEnv` is a zero-sized
  value with no `Drop` implementation, so no code and no drop glue is emitted.
* Re-attestation on the render thread, per engine-v2-old's `MountedSession::start`:
  `StartedRenderSessionV1::start` calls `miso_engine_lane::attest_fp_environment` on the thread that
  will render and returns the plan unchanged on refusal (the render thread frees nothing). The C ABI
  has no "the render thread starts now" call, so a plan's *first block* is its session start there:
  it verifies the canonical word took and returns `RESULT_RENDER_REJECTED` with
  `render.fp_environment.invalid` if it did not.
* Attestation compares *control* bits; restoration is bit-exact over the whole word. Bits 0-5 of
  MXCSR are sticky exception status flags that any arithmetic sets and that say nothing about how
  the next operation rounds; a caller's sticky flags are the caller's and come back untouched.

## AArch64: implemented, not runtime-proven

`read_fp_control_word`/`write_fp_control_word` issue `mrs`/`msr FPCR` through `core::arch::asm!`,
because the standard library exposes no stable FPCR intrinsic. This is the one new unsafe site of
the issue and the workspace's only inline assembly; `scripts/check-realtime-policy.sh` and
`scripts/check-lane-policy.sh` both name `fpenv.rs` explicitly, each with a mutation test proving a
third lane file does not inherit the exemption.

`cargo check --target aarch64-linux-android -p miso-engine-lane` passes, and the emitted assembly
carries the instructions (`mrs x9, FPCR` / `msr FPCR, x10`). **It has never been executed**: this
delivery host is x86-64 and has no AArch64 runner or emulator. The x86-independent tests
(`attestation_passes_on_this_thread`, `the_canonical_word_is_the_word_inside_the_guard`,
`the_target_declares_whether_it_pins`) are written against `FP_ENV_CONTROLLED` rather than
`target_arch`, so they exercise the FPCR path the first time this suite runs on AArch64 hardware.
Until then the AArch64 half is compile-proven only. A runner is out of this issue's scope.

## Evidence

| eval | gate | result |
|---|---|---|
| E1 | `tools/miso-engine-wasm-gates/tests/g6_full_corpus_ftz.rs` | GREEN, 331 comparisons, 0 mismatches under caller FTZ+DAZ; the unguarded control arm still shows 70 |
| E2 | `miso-engine-lane --test fp_env`, `miso-engine-capi --lib fp_environment`, `miso-engine-host-core --test fp_environment` | GREEN; bit-exact restore proven on the success path, two rejection paths and an unwind |
| E3 | `g6_the_guard_is_an_identity_for_a_caller_who_never_set_ftz`; every frozen pin; the shipped browser artifact | GREEN; the stripped `simd128` module is byte-identical to `94e8702` (`sha256:579bb210…`) |
| E4 | `scripts/check-web-audioworklet.sh` call-graph gate; `check-realtime-policy.sh`; `check-lane-policy.sh` and both mutation suites | GREEN |
| E5 | `artifacts/issue146/fp-environment-benchmark.raw.jsonl` | 0.93-2.89 ns per 128-frame block |

Red mutations are recorded in `crates/miso-engine-lane/tests/MUTATIONS.md` (rows 16-17),
`crates/miso-engine-host-core/tests/MUTATIONS.md` (M-146-1, M-146-2, M-146-2b, M-146-3) and
`tools/miso-engine-wasm-gates/MUTATIONS.md`. The E1 mutation the brief names -- remove the guard at
one entry -- reproduces the #144 figure to the row: *70 of 331*.

### E3, exactly

The browser artifact was built from the same worktree at `94e8702` and at this branch's head. The
two `.wasm` files are the same length (2,371,545 bytes) and differ in 1,826 bytes, all of them
inside the `name` custom section (offsets 2,060,805-2,098,083; the Code section ends at 1,974,232
and the Data section at 2,060,575). The differing bytes are crate-disambiguator hashes inside
mangled symbol names: `miso-engine-host-core` gained a module and a dependency, so its `-C metadata`
changed and every downstream symbol name's hash moved with it. `wasm-strip` on both produces
`sha256:579bb21018a36817afad26bfdcd0d6b8364aed6e0832474f481c92a33ab4ee6a` for each, and the
code-plus-data prefix hashes identically without stripping. The browser executes identical bytes.

## Non-goals

* An AArch64 execution runner or emulator leg.
* Any change to the D7 flush law, `FLUSH_EPS`, or a kernel's arithmetic. The flush law is still the
  law that keeps recursive state out of the subnormal range; it was never the whole story and is not
  asked to become it.
* A new C ABI symbol. The re-attestation rides the existing render call.
