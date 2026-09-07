**FAIL — consolidated attempt-1 verdict for #539.** One bounded evidence gap remains: complete state is not compared at mono disengagement, as explicitly required by the accepted baseline decision. I found no additional blocking DSP or dispatch defect in the reviewed change.

**Luna high attempt 2 is authorized only for the correction below.** Attempt 1 remains counted; maximum three total attempts.

Verified identity and provenance:

- Clean, pushed HEAD `37bf253f1f6b6f0676cb81878ae1720c9fd43d10`; live main remained `32a4c2058f2c01dea78af32affab660dbc17cae0` at the final check. No #540 base advance was observed.
- [#539](https://github.com/misofm/engine/issues/539) is OPEN with matching title and byte-exact local/GitHub body.
- Frozen `lib.rs` SHA is `08ea2bfa…7be09d90`; allocation SHA is `7825d307…ca0ea920`, both matching the supplied full hashes and captured final gates.
- Only `lib.rs` and `tests/allocation.rs` changed under the limiter crate. `mono_collapse.rs` is unchanged.
- Both original native outputs, linked Wasm modules and decoder outputs match recorded identities. All **32 native intervals and eight Wasm intervals** match their originals and hashes. **95 attempt capture files** match their retained raw counterparts.

The blocking finding is **missing full-state equality at disengagement**.

The accepted baseline decision requires “dual equivalence and full-state comparison at the existing disengagement/desymmetrization boundary.” The new [private witness](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/src/lib.rs:4742) compares candidate mono against runtime-oracle mono and checks unchanged right-channel runtime words, but never disengages either instance. The existing [desymmetrization test](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/tests/mono_collapse.rs:246) calls `desymmetrize_channels()` and then compares output samples only. It contains no snapshot/state comparison.

Those tests establish useful, distinct properties, but neither proves complete state equality immediately after copying the stale right channel back into service. Subsequent PCM agreement does not substitute for that explicit gate.

**Bounded attempt-2 correction:** extend the existing mono transition test, or the existing private witness, to compare populated complete state against a never-collapsed reference immediately after disengagement, before another render can obscure a difference, and after the first resumed dual block. Retain the existing PCM comparisons and prove meaningful populated history/rings. Use existing snapshot/state helpers; no new framework, corpus, production change or allocator work is needed.

The rest of attempt 1 withstands review:

- **Source semantics:** [block dispatch](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/src/lib.rs:1692) retains the exact stationary predicate and uniform-shape/phase selection. Const specialization shares the DSP source. The ramping instantiation advances for the whole block, including samples after an internal endpoint; stationary selection resumes only at the next block. Coefficient design, detector arithmetic, gain law, quantization, state layout, cursors, resources, API, silent-rest handling and nonfinite recovery are unchanged. No FX2 rewrite, cached eligibility or new realtime operation appears.
- **Old-behavior oracle:** `DISPATCH_RUNTIME` preserves the original runtime predicate rather than forcing `advance` at rest. Sharing unchanged DSP arithmetic is appropriate for this dispatch comparison. The witness exercises all four W8 routes and scalar dual/uniform over a 64-update ramp inside a 512-frame block and its following stationary block. It compares PCM, both channels’ runtime arrays, cursors and all ramp words. The observation lives inside the selected body and uses thread-local storage.
- **Negative control:** pristine SHA `5b25b033…defce28d` reconstructs to mutant `768f1aba…513a1465` through exactly four stationary-callsite const substitutions. The retained failure is the same W8 dual-per-lane block-1 specialization assertion, observing Runtime instead of Stationary, after PCM/state/silent-state equality. Later mutant cases are not credited. The first restored invocation reused the mutant binary; its stderr shows no compilation. The subsequent restored invocation compiled and passed. Final source differs from pristine only by the recorded test-only enum annotation and private-oracle lint allowance.
- **Allocation:** the existing thread-local allocator now demonstrates positive allocate/free liveness and measures mono rendering in addition to the original paths. W8 availability is required by the allocation fixture; the passing result is not an early-return skip.

Candidate lowering supports the intended mechanism. References below use selected-file lines:

| Path | Candidate evidence |
|---|---|
| W8 dual | `process-core-w8.ll:335` defines the stationary predicate; its dispatch uses are at 470/478, outside frame loops. Assembly tests `%dl` at 443/5895. Stationary per-lane backedge 2824 returns to `.LBB34_264`; stationary uniform backedge 7921 returns to `.LBB34_513`, without revisiting stationary classification. |
| W8 mono | The wrapper reaches `outlined-07`, the actual `process_bank_inner<true>` body. Admission tests at assembly 605/3320 select separate bodies. Stationary per-lane backedge 1884 returns to `.LBB1_81`; stationary uniform backedge 5923 returns to `.LBB1_293`. |
| Scalar dual/uniform | The predicate’s IR uses are confined to block dispatch. The stationary uniform loop returns to `.LBB36_538` at assembly 7974; ramping has its separate recurrence path. No scalar mono or valid scalar per-lane claim is made. |

The ramping loops retain decrement, endpoint selection and step clearing; their backedges remain within that selected body. The former per-frame stationary tests are removed, rather than merely hidden in the newly outlined mono wrapper.

Linked Wasm independently supports this:

- Actual wrappers reach scalar core **716**, and SIMD-module cores **755/756/757** through wrappers **769/770/767**.
- SIMD4 dual core 755 selects on its stationary local at `0xae278` and `0xb0c21`, before the respective bodies. Its stationary uniform frame loop is `0xb3e72…0xb4299`; the separate ramping loop at `0xb1baf` retains the decrement/select operations.
- SIMD4 mono core 756 selects uniform dispatch at `0xb830a` and per-lane dispatch at `0xb9ecf`. Its stationary uniform loop is `0xb9913…0xb9b5d`; ramp updates remain in the separate loop at `0xb8b7c`. These structured loop backedges do not re-enter the stationary admission.

This is crate-emitted native evidence, not final post-LTO native proof. The Wasm modules are linked inspection outputs, not published artifact identities.

**The code-size tradeoff is substantial and unmeasured.** As a descriptive proxy, emitted assembly instruction-line counts grow from 3,860 to 7,302 for W8 dual core and 3,777 to 6,759 for scalar core; mono grows from 2,336 to 4,090 in the outlined body, plus its wrapper. These are not executable byte sizes or timing measurements. They establish code expansion, not a net performance benefit.

Recorded final gates are consistent with the captures: full suite **42 passed, zero failed, one descriptive benchmark ignored**; release private selector **1 passed**; release allocation and mono binaries **2 each**; strict Clippy and four corrected policy invocations passed. Formatting was applied successfully; that capture is not a `fmt --check` invocation. The wrong environment-script command returned retained status 127; the existing vocabulary script subsequently passed. Immutable corpus legs each reported **139 cases / 349 comparisons / zero mismatches**. No benchmark ran.

For attempt 2, preserve all attempt-1 evidence and the successful mechanism control. A test-only disengagement correction does not require another mutation campaign or baseline capture. Run the changed focused checks in debug/release and proportional final checks, checkpoint the coherent result, then obtain one consolidated attempt-2 verdict.

Ordinary native ABI, published-artifact/browser qualification, exact PR-head/current-base review, required CI, merge, synchronization and eventual worktree cleanup remain delivery work. No issue closure or audit-finding status change is approved here.

Review was read-only; no files, builds, tests, mutations, captures, Git/GitHub state or agents were created or changed.