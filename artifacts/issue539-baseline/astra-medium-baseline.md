**PASS — baseline sufficient; a reachable FX1 residual survives in all four W8 bodies. Luna high/xhigh attempt 1 is authorized within the bounded scope below.** This establishes retained dispatch work, not a measured or projected speedup.

Verified clean, pushed HEAD `a57c8a7f7f89ef51c9d188aa6e4f1f711360cd3d` against live main `32a4c2058f2c01dea78af32affab660dbc17cae0`. [#539](https://github.com/misofm/engine/issues/539) remains OPEN with exact title/body synchronization. Source and captured configuration hashes match; no production changes separate this checkpoint from the base. Post-main qualification `34089134766` completed SUCCESS.

I independently verified both full original compiler-output hashes and all ten selected intervals, sizes and hashes. The successful capture is at `c97c1944`, status 0. The interrupted worker report supplies no mapping evidence; the findings below come from the retained compiler output.

**Provenance is sufficient for this bounded decision.** Captured Cargo argv, Rust `1.97.1`/LLVM `22.1.6`, configuration and source identities are consistent. The retained Cargo fingerprint records `["-C","target-feature=+avx2,+fma"]`; actual function attributes include AVX2/FMA, and the module identifies the pinned compiler and optimized compilation. Its profile fingerprint matches the retained #537 baseline/candidate captures under the unchanged release configuration.

The environment record is selective and there is no verbose rustc invocation. Fat LTO/codegen-units=1 are recorded configuration, not proof that these crate-emitted excerpts are final linked, post-LTO native code. That limitation does not invalidate this previously authorized crate-production baseline. No extracted object is credited and no replacement capture is needed.

The production caller chain is explicit: scalar `process` calls scalar `LimiterCore::process_block`; W8 `process_bank` calls W8 `LimiterCore::process_block`; W8 `process_bank_mono` contains the mono bodies.

All line numbers below are **selected-file lines** in [the retained package](/home/bl/misofm/engine-limiter-stationary-dispatch/artifacts/issue539-baseline/selected/manifest.json).

| Supported W8 body | Stationary test and taken target | Frame-loop backedge |
|---|---|---|
| Dual per-lane, source 1701 | `process-core-w8.s:1888`: `cmpb $0, 256(%rsp)`; `jne .LBB32_98` | Line 3224 returns to `.LBB32_96`, before the test |
| Dual uniform, source 1890 | `process-core-w8.s:5125`: `cmpb $0, 2304(%rsp)`; `jne .LBB32_315` | Line 5720 returns to `.LBB32_313`, before the test |
| Mono per-lane, source 3086 | `process-bank-mono-w8.s:1429`: `cmpb $0, 48(%rsp)`; `jne .LBB60_88` | Line 2077 returns to `.LBB60_86`, before the test |
| Mono uniform, source 3197 | `process-bank-mono-w8.s:3037`: `cmpb $0, 8(%rsp)`; `jne .LBB60_178` | Line 3335 returns to `.LBB60_176`, before the test |

These are stationary flags, not attribution based solely on debug locations:

- Their definitions test each ramp’s `remaining` word at offset 12 against zero and compare current/target as integer words at offsets 0/4. Dual combines four ramp sets; mono combines two.
- Successful admission writes the true flag before entering the frame walk. IR carries those invariant predicates to the corresponding branches at W8-core lines **1158/3310** and W8-mono lines **786/1996**.
- In each W8 case, the stationary jump bypasses decrement/max/compare/add/select ramp updates and rejoins the shared DSP frame body. That body’s backedge revisits the same flag test.
- Uniform admission checks lane shapes and phases, on both channels for dual and the live channel for mono. Valid mixed lookahead or restored phase differences reach per-lane fallback while coefficients can remain stationary.
- Positive signal with stationary coefficients reaches these loops without taking silent-rest bypass. Silent-rest has separate input/state admission and cursor/phase advancement.

Consequently, compiler unswitching has **not removed these four per-frame stationary tests** in this capture. Existing stationary handling already avoids ramp arithmetic; #539 must claim only removal of the repeated dispatch test, not rediscover that earlier saving.

Scalar dual/uniform also retains the test: `process-core-scalar.s:5183` compares `992(%rsp)`; `je .LBB34_287` enters ramp handling, and line 5783 returns to `.LBB34_283`. The emitted scalar per-lane body is not credited as supported one-lane execution. There is no scalar mono claim.

**Bounded attempt-1 authority**

After root records this decision, Luna may specialize the four existing common bodies using the existing block-entry stationary decision. Preserve uniform/per-lane selection, ramping behavior for the entire block, and all frozen DSP/state contracts. No mid-block transition, cached eligibility, per-lane dispatch, hand-maintained divergent DSP copies, FX2 access rewrite, or new public surface.

Required mechanism/state gates:

1. **Old-behavior oracle:** retain the original runtime-predicate behavior in a bounded private test path. Compare populated PCM and complete state for stationary blocks, a ramp ending inside a block, and the following stationary block across all four W8 routes and scalar dual/uniform. Preserve ramp `current`, `target`, `step` and `remaining` exactly. An always-advance oracle is not automatically equivalent to the original stationary path.
2. **Actual-path control:** observe execution inside the selected specialization, on the witness’s test thread—not classification before dispatch. Freeze one fallback-only control that routes through the old unspecialized behavior. Identity assertions must pass before the same positive specialization assertion fails. Retain that failure, restore byte-exactly, verify focused green, then pause for root checkpoint.
3. **Mono comparison:** compare candidate mono state against old mono behavior, including right-plane non-access. Retain dual equivalence and full-state comparison at the existing disengagement/desymmetrization boundary.
4. **Candidate lowering:** under comparable settings, demonstrate block-entry specialization and removal of these repeated stationary tests from the admitted frame loops. Preserve the ramping path and arithmetic order. Test instrumentation alone cannot establish the machine-code mechanism.
5. Retain the numbered brief’s finite existing correctness, realtime, native/Wasm and delivery gates. Explicitly identify supported W8 execution; skips are not evidence.

Resolve the allocation-path ambiguity narrowly: `crates/true-peak-limiter/tests/allocation.rs` may receive only existing-allocator positive liveness controls and focused coverage needed for the changed paths. Reuse its thread-local allocator; no new allocator or framework.

No implementation attempt has started; this authorizes **attempt 1 of at most three**. No timing, projected gain, FX2 work, finding-status change, delivery acceptance or issue closure follows from this decision.

Review was read-only: no edits, builds, tests, benchmarks, captures, Git/GitHub writes, agents or report-file writes.