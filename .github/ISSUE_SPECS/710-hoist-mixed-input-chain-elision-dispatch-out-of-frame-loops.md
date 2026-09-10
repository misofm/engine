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

## Attempt 1 — FAIL

Astra XHIGH supplied an uncommitted implementation from clean pushed brief
`413767be8f3a54d47b33296bdf767772aca7da60`; base remains
`898bdc94b0143288049397629f3afeded384f8c2`. External evidence is
`/tmp/issue710-attempt1-xwq9EPTa`. The original product source and extracted
mixed bodies are preserved there; original `builtins.rs` SHA-256 is
`365fe127ef70d579bc35eb3817d57b62bf2d9a303908d37d952aef3c0a4e90fb`.

Each mixed wrapper selects each processed channel through one four-arm match,
then invokes a const-generic frame body with no runtime plan parameter. Its
three const conditions retain HPF→LPF order and exactly one `add(+0.0)` at the
original identity-run position. Dual channels have separate buffers,
coefficients, local integrators and report accumulators; the existing scalar,
W4 and W8 lane primitives and recurrence have no cross-channel dependency or
observable side effect. Caller state is still published once after both
channels complete. Mono publishes only channel 0 and duplicates its report.
No public contract, prepared-plan boundary or arithmetic primitive changed.

The integration evidence retains the actual pre-change mixed dual and mono
bodies, renamed only, and reuses the existing designs, signal corpus and
output/state/report packer. Supplemental cases cover all 16 channel-plan
pairings, two section orders and 0/1/17 frames at scalar/W4/W8, with different
channel trim/data/state, signed zero, nonfinite inputs and forced elided-state
sentinels. Mono explicitly checks channel-1 and elided-section state remains
untouched. A private counter checks one selection per processed channel
independent of frame count. One production structural validator checks the
wrappers, four-arm selector, const frame conditions and the existing
`no_lanes`/`svf_step`/`flush` helper closure; no frozen reference participates
in that structural assertion.

The attempt stopped at its first failed gate without a retry. Commands ran
from `/home/bl/misofm/engine-rt7-hoist-elision-plan`, with Cargo output under
`CARGO_TARGET_DIR=/tmp/issue710-attempt1-xwq9EPTa/target`:

| Gate | Result | Log |
| --- | --- | --- |
| `cargo fmt --check` | Exit 0 | `01-fmt.log` |
| Compare both frozen bodies with the actual pre-change bodies, restoring only their names | Exact equality | `02-frozen-reference.log` |
| `cargo test --locked -p lane --test input_chain_elision -- --nocapture` | Exit 0; all 9 tests passed, including scalar/W4/W8 bitwise and production structural checks | `03-input-chain-elision.log` |
| `cargo test --locked -p lane --lib mixed_elision -- --nocapture` | Exit 101; unqualified `thread_local!` and both `vec!` calls are unavailable in the `no_std` lib-test context; unresolved `MIXED_PLAN_SELECTIONS` errors follow | `04-selection-count.log` |

Selector instrumentation did not compile. Full lane tests, release-unwind,
Clippy, allocation, external structural mutation, policy and portability
qualification were not run. The failed uncommitted state is preserved as
`failed-attempt.diff` and `failed-state-sha256.json` under the Attempt 1 evidence
directory. SHA-256 values are:

| Preserved input | SHA-256 |
| --- | --- |
| Product `builtins.rs` | `402c3b38bb7e610c5921a22cffa53c40801eb24a9b2dcc32d5e667a6b7660f8d` |
| Integration `input_chain_elision.rs` | `b30e02d8171ae994ee53a5f09d6088d7f141745e3a5dd9a33c1a7008d805023b` |
| Issue spec before this failure record | `ec9b55b8a73a5f8a0175c6e4537fdf2f0b5a6831f407904cf22e8cfcdee13353` |
| `failed-attempt.diff` | `a60a60281da7f723a94b0519f0c980cfe71f4e97312d9cfc7c068757e3ec8b67` |

Astra LOW (`/root/issue709_astra_low_scope`) recorded **Attempt 1 FAIL** on that
inspected state: the lib-test compile failure is blocking, despite nine passing
integration tests. No additional blocking production finding emerged. The
review found the const shapes and independent channel execution preserve the
specified order, mono leaves channel 1 untouched, and the approximately 374
integration-test lines are proportional. Selector instrumentation and broader
qualification remain incomplete; no SOURCE PASS was recorded.

## Attempt 2 — required implementation gates complete; review pending

Astra LOW and Sol authorized only qualifying the test macros as
`std::thread_local!` and `std::vec!`, updating this record and rerunning the
complete sequence. No suppression, production change or additional path is
authorized. Astra XHIGH applied those three qualifications without changing
the integration tests. Attempt 2 evidence is separate at
`/tmp/issue710-attempt2-IYCU7pK6`; Attempt 1 evidence remains preserved.

The correction's formatting, frozen-reference provenance, nine integration
tests and one selector-count test passed (`01`–`04` logs and
`focused-results.jsonl`). Astra XHIGH paused; Sol audited, committed and pushed
the exact source checkpoint `c5b08616454d7367fc3e758cd541a0873c503ba5`.
Sol then authorized a restart of the full sequence on that frozen source.
All commands below passed without a failed gate or retry. This record is an
implementation evidence report, pending the separate Astra LOW verdict.

Commands ran from `/home/bl/misofm/engine-rt7-hoist-elision-plan` with Rust/Cargo
1.97.1. Let `E=/tmp/issue710-attempt2-IYCU7pK6`; the default Cargo target was
`CARGO_TARGET_DIR=$E/target`. Native commands inherited the repository's
approved `-C target-feature=+avx2,+fma` configuration. Policy commands also
used `TMPDIR=$E/policy-tmp`; scripts with explicit `/tmp` scratch paths kept
their existing behavior. `toolchain-and-environment.log` records toolchain,
installed targets and flag environment. `full-results.jsonl` records literal
argv, environment overrides, source commit and exit for every row. Log names
below are relative to `E` and have the suffix `.log`.

| Exact command (environment overrides shown where different) | Result | Log |
| --- | --- | --- |
| `cargo fmt --check` | Exit 0 | `full-01-fmt` |
| `python3 "$E/verify-frozen-references.py"` | Exit 0; both actual pre-change bodies equal their frozen copies after restoring only function names | `full-02-frozen-reference` |
| `cargo test --locked -p lane --test input_chain_elision -- --nocapture` | Exit 0; 9 passed | `full-03-input-chain-elision` |
| `cargo test --locked -p lane --lib mixed_elision -- --nocapture` | Exit 0; 1 passed | `full-04-selection-count` |
| `CARGO_TARGET_DIR="$E/mutation-target" cargo test --offline --manifest-path "$E/inner-plan-mutation/Cargo.toml" --test input_chain_elision mixed_elision_production_structure_selects_before_frames_and_audits_helpers -- --exact --nocapture` | Compiled; expected test exit 101 at the intended structural assertion; validator exit 0 | `full-05-inner-plan-mutation` |
| `cargo test --locked -p lane --test sanitise_counter` | Exit 0; 2 passed | `full-06-sanitise` |
| `cargo test --locked -p builtins --test mono_collapse --test input_liveness_mono` | Exit 0; 10 passed | `full-07-builtin-mono` |
| `CARGO_TARGET_DIR="$E/allocation-target" cargo run --offline --release --manifest-path "$E/allocation-harness/Cargo.toml"` | Exit 0; detector liveness and 288 audited calls passed | `full-08-allocation` |
| `cargo test --locked -p lane` | Exit 0; 48 passed, 2 descriptive timing tests ignored | `full-09-debug` |
| `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked -p lane --release` | Exit 0; 48 passed, same 2 ignored | `full-10-release` |
| `cargo clippy --locked -p lane --all-targets -- -D warnings` | Exit 0 | `full-11-clippy` |
| `bash scripts/check-workspace-policy.sh` | Exit 0 | `full-12-workspace-policy` |
| `bash scripts/check-lane-policy.sh` | Exit 0 | `full-13-lane-policy` |
| `bash scripts/check-realtime-policy.sh` | Exit 0; 41 regions in 12 files | `full-14-realtime-policy` |
| `bash scripts/check-unfused-seal.sh` | Exit 0 | `full-15-unfused-seal` |
| `bash scripts/check-realtime-audit-leak.sh` | Exit 0; production dependency graphs exclude audit instrumentation | `full-16-realtime-audit-leak` |
| `bash scripts/test-workspace-policy.sh` | Exit 0; expected counter-mutation diagnostics retained | `full-17-workspace-mutations` |
| `bash scripts/test-lane-policy.sh` | Exit 0 | `full-18-lane-mutations` |
| `bash scripts/test-realtime-policy.sh` | Exit 0 | `full-19-realtime-mutations` |
| `bash scripts/test-realtime-audit-leak.sh` | Exit 0 | `full-20-audit-leak-mutations` |
| `cargo build --locked --release -p lane --target x86_64-unknown-linux-gnu` | Exit 0; approved native ISA flags | `full-21-native-build` |
| `CARGO_TARGET_DIR="$E/wasm-scalar" cargo build --locked --release -p lane --target wasm32-unknown-unknown` | Exit 0 | `full-22-wasm-scalar` |
| `CARGO_TARGET_DIR="$E/wasm-simd128" CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C target-feature=+simd128" cargo build --locked --release -p lane --target wasm32-unknown-unknown` | Exit 0 | `full-23-wasm-simd128` |
| `python3 "$E/verify-final-source.py"` | Exit 0; exact HEAD/upstream, clean tree, base/brief diffs confined to the three owned paths, unchanged product/test hashes, actual brief provenance and prior gate statuses verified | `full-24-source-review` |

The one external mutation passed the runtime plan into the specialized body
and restored all three per-frame conditions to `plan.elided` lookups. The
production structural test rejected it with
`runtime plan/control in frame body or helper: mixed_channel_block: InputChainPlan`.
It compiled and reached that assertion: 0 passed, 1 failed, 8 filtered out.
The isolated manifest emitted two existing `miso_wasm_simd8` check-cfg warnings
because it does not inherit the workspace lint declarations; these did not
cause the rejection. The input copy, manifest, source, test and exact
`mutation.diff` remain under `E/inner-plan-mutation`; their hashes are in
`mutation-input-sha256.json`. Mutation-diff SHA-256 is
`63f7ceeac7d6053148dbe5679edc0ddb408bd07d1ea18a9de6b10b028a35bdb5`.
The authoritative checkout was never mutated for this gate.

The external allocation harness used the public dual/mono elided entries,
actual prepared plans for all 16 pairings, different channel data/trim,
nonfinite and signed-zero inputs, and 0/1/17 frames at scalar/W4/W8. Its
288 scoped calls include every mixed shape and the existing fast paths;
each had zero allocations and frees. Mono channel-1 sentinels stayed intact.
The System-forwarding detector first observed its expected 3 allocations and
3 frees across alloc, zeroed alloc, realloc and dealloc. All buffers were
prepared outside the measured intervals; this checks kernel allocation, not
whole-host rendering or syscalls. Harness source, manifest, generated lockfile,
binary and target remain external. `allocation-input-sha256.json` records
manifest SHA-256 `330d8145d95b0653aba6b26e4a2abc1505b52d5686c4fd27e85297aaa634c393`
and source SHA-256 `0fa81daf3d655a7f265af1b972d3e70b502bd0ecad62ba8be88573e6347c7810`.

`qualified-source.diff` preserves the reviewed base-to-source diff.
`frozen-checkpoint-sha256.json` and the final review confirm product SHA-256
`217815ba5ddddf85a3a3ed7d522103a8a8b20730e69f395fcdc2a29ce42e08f1`
and unchanged integration SHA-256
`b30e02d8171ae994ee53a5f09d6088d7f141745e3a5dd9a33c1a7008d805023b`.
No source or test edit followed the checkpoint; only this decision record
changed after qualification.

Bitwise and allocation executions are native scalar/W4/W8 evidence. Wasm
results establish compilation only; no Wasm runtime or AArch64 execution was
performed. The structural gate checks the production source and its named
free-helper closure, not an assembly or cycle claim. No timing, speedup,
AudioWorklet, artifact, pin, PR, merge, delivery or SOURCE PASS claim is made.
Astra XHIGH pauses here for Sol's exact-path checkpoint audit and Astra LOW's
independent adversarial review.
