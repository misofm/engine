## Authority and smallest closable outcome

Parent: #559 RT7. Companion tracker: #560. Base and current main are `898bdc94b0143288049397629f3afeded384f8c2`.

Sol HIGH coordinates. Astra XHIGH implements because this is low-level render-time SIMD kernel work. A separate Astra LOW reviewer performs scope review and each adversarial verdict. Agents are bounded assignees and do not own the issue.

#709 is passive at SOURCE PASS and consumes no implementation slot. Lane B independently owns #705 and every AudioWorklet qualification/pin decision. This issue may occupy lane A's one active implementation slot without waiting on disjoint lane-B work.

The smallest closable outcome selects each dual or mono input channel's two-section elision shape once before its frame loop, then runs a bounded specialized body whose loop neither reads `InputChainPlan` nor branches on plan elision. It preserves the exact per-channel arithmetic order, identity-run `add(+0.0)` placement, state writes, reports, and output bits for every one of the four two-section shapes at scalar, W4, and W8 widths. No DSP equation, coefficient, state format, public API/ABI, session schema, benchmark, artifact, pin, or performance budget changes.

## Current fact pattern

`input_chain_block_elided` already dispatches the all-unelided and all-elided four-section cases once per block. Its mixed dual body, `mixed_chain_block`, still reads `plan.elided[channel][section]` and branches inside the frame/channel/section loop. The newer mono-collapse sibling `mixed_chain_block_mono` repeats the same plan read inside its frame/section loop. Both plans are invariant for the call: HPF/LPF coefficients are prepared-only, and reset/state restoration recomputes the plan.

The existing elision corpus proves bit identity across widths and section patterns, including signed-zero placement and retained state. It does not by itself prove plan selection moved outside the frame loops. The audit's location and cost are structural hypotheses, not current timing evidence. No timing or speedup claim is authorized.

## Exact-path ownership

Implementation and private/unit evidence:

```text
crates/lane/src/kernels/builtins.rs
```

Existing integration evidence may be extended only at:

```text
crates/lane/tests/input_chain_elision.rs
```

Decision record:

```text
.github/ISSUE_SPECS/710-hoist-mixed-input-chain-elision-dispatch-out-of-frame-loops.md
```

Coordinator-only concise rows in #559/#560/#349 are synchronized outside this checkout. No other path is owned. If the correct change requires another crate, public contract, benchmark framework, generated artifact, policy change, or timing claim, stop and split it into a new stateless issue.

## Product contract

1. Mixed dual and mono input-chain execution each select a channel's exact `[elide_hpf, elide_lpf]` shape once outside that channel's frame loop. A private match plus const-generic or equivalent bounded specialization is acceptable. No frame-loop body may read `InputChainPlan`, index `plan.elided`, or branch on a runtime elision flag.
2. All four per-channel shapes retain the current order:
   - neither elided: HPF then LPF;
   - HPF elided: one `add(+0.0)` before LPF;
   - LPF elided: HPF then one `add(+0.0)`;
   - both elided: exactly one `add(+0.0)` and no recurrence/state write.
   Sanitization, trim, nonfinite accumulation, store, and report order remain unchanged within each channel.
3. Dual left/right channels remain independent and use their own plan, coefficient, state, report, and buffer. Mono collapse continues to use channel 0 only. Processing one channel's complete block before the other is permitted only if the implementation and adversarial review establish that no shared mutable state, fallible step, observable side effect, or cross-channel arithmetic dependency changes behavior.
4. Output PCM, `InputChainReport`, and retained state are bit-identical to the current implementation for scalar, W4, and W8; all section patterns, signed-zero cases, nonfinite inputs, zero and short blocks, and representative multi-frame blocks are covered. Elided sections remain unwritten.
5. The plan remains prepared/recomputed at the existing boundaries. No live parameter, reset, restore, collapse eligibility, or symmetry rule changes.
6. Render stays allocation/free, lock, I/O, logging, syscall, panic-edge, and structural-mutation free. Work remains statically bounded by width and frames.
7. No public API or ABI expansion is required. Existing callers and `InputChainPlan` layout/meaning remain unchanged.

## Attempt and checkpoint discipline

The repository's five-attempt maximum applies. Each attempt is one coherent Astra XHIGH implementation pass followed by a separate Astra LOW adversarial verdict. A failed prerequisite, compile, bit-identity, state/report, structural, policy, portability, allocation, or evidence gate stops that attempt. No retry within an attempt and no weakened gate. After five failures, preserve evidence and hard-stop; no disguised sixth attempt.

Astra XHIGH edits only the owned paths. It stops at each coherent compiling/focused-test checkpoint for Sol to audit, commit, and push before more implementation or qualification is layered on. Raw targets, compiler streams, mutation copies, binaries, and temporary evidence remain outside Git.

## Objective gates

Before implementation, Astra LOW must verify exact main/branch/spec/GitHub identity, clean exact-path ownership, slot independence from #705/#709, the current dual and mono inner-loop plan-read shape, and that the proposed specialization can preserve the four operation orders without a public contract change.

Each implementation attempt must provide:

1. Focused scalar/W4/W8 tests over every dual left/right plan pairing and every mono plan shape, comparing output words, report words, and state words bit-for-bit with a frozen reference copy of the pre-change mixed bodies. Include signed zero, nonfinite sanitization, zero/one/multi-frame blocks, and elided-state non-write assertions.
2. A discriminating structural gate proving plan selection occurs once per processed channel per call and no runtime plan read/branch remains inside either mixed frame loop. An external mutation restoring an inner-loop plan lookup must be rejected for the intended reason. Compiler/lowering evidence may supplement, but not replace, a gate tied to this source claim.
3. Existing lane input-chain elision, sanitization, mono-collapse, and relevant builtins integration tests.
4. A render allocation gate covering mixed dual and mono shapes.
5. `cargo test --locked -p lane` in debug and release-unwind, strict lane Clippy, formatting, and exact diff/path review.
6. Workspace and realtime policy/mutation gates plus supported native x86-64-v3 and Wasm scalar/simd128 compilation appropriate to `lane`.

Astra LOW independently reviews the exact pushed checkpoint and reruns the claim-discriminating focused/structural gates before recording SOURCE PASS or FAIL. SOURCE PASS grants no timing, artifact, pin, PR, merge, or delivery claim.

## Delivery boundary

After SOURCE PASS, Sol performs exact-head/current-main review. `lane` is in the browser artifact dependency closure, so lane B alone owns any separately numbered AudioWorklet applicability/qualification and pin decision. Lane A does not run a builder, qualify an artifact, or change a pin, and a passive source issue consumes no implementation slot.

Required PR qualification, guarded live head/base merge, post-main qualification, exact GitHub synchronization, and clean delivered-worktree removal follow the repository workflow. All failed branches, worktrees, commits, mutation copies, and evidence directories are preserved.
