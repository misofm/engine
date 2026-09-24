# Capture compiler scratch measurements and prepare fresh Astra review

GitHub: https://github.com/misofm/engine/issues/876

## Outcome and dependencies

Depends on [#875](https://github.com/misofm/engine/issues/875) and all predecessor gates. Run the frozen benchmark once, retain honest evidence and provide the user a complete independent-review packet. No product or harness implementation is owned by this issue.

## Execution contract

Before timing, root records the clean implementation SHA, source baseline, exact compiler/lockfile/profile/target features, workload/config/validator identities, host/CPU/OS/load/governor information, the successful zero-workload preflight, and fresh output paths. Freeze the matched timing and byte-instrumented executables. No unrelated build, test or other agent workload runs concurrently with capture.

Run `scripts/run-compiler-scratch-benchmark.sh` once under its frozen arguments. It owns one complete-sequence warmup and two measured rounds per declared policy/workload/pass; there are no manual trial runs. Preserve raw results and actual exit status even on failure. If persistence or validation fails after execution, retain the raw data and create a tooling successor after at most one bounded tooling correction; never repeat a timed workload merely to obtain favorable or promotable results.

Report preparation time, allocation/reallocation/deallocation counts, cumulative requested bytes, baseline/live/peak requested heap bytes, retained std capacity or pool backing/occupancy, and fresh-process RSS where available. Separate cold, steady reuse, shape changes and late rejection. Show active-plus-candidate overlap and post-retirement retention explicitly. Explain instrumented versus timing builds and the unavoidable limits of two rounds. A numerical null result is an acceptable completed experiment.

## Decision record and delivery boundary

Require unchanged graph semantics, diagnostics/owner returns, representative PCM, render allocation/free behavior and every retention cap before discussing speed. State whether (a) ordinary reuse is useful, (b) pooling adds a demonstrated benefit beyond ordinary reuse, or (c) neither has a supported benefit under these workloads. Do not set a speedup target after looking at results, tune to the numbers, project realtime track-count gains, or automatically enable a pool in production.

If evidence supports adoption, scope a successor for persistent CAPI `SessionState` ownership, active/queued/candidate scratch admission, and its ABI/resource-accounting implications. Browser preparation ownership/artifact/package adoption is a separate successor if required. The current series delivers the native Rust opt-in API and experimental evidence, not automatic improvements to existing CABI/browser clients. Native timing cannot be presented as Wasm/iOS performance.

## Fresh review handoff

The user requested GPT-6 Luna MAX implementation and a fresh GPT-6 Astra XHIGH verifier after implementation. Do not consume that final review with a planning agent or claim it ran. Prepare a concise packet containing the exact source/base SHAs, issue map, changed paths, test commands/results, raw benchmark locations and validator command, unresolved limitations, and the proposed adoption decision. The reviewer starts with no implementation-agent history and independently inspects ownership/reset safety, ordering/PDC/PCM invariance, post-failure reuse, caps, experimental-feature isolation, measurement attribution/controls and reproducibility of the reported interpretation. The user initiates this final verification; implementation delivery remains review-pending until its verdict.

Merge production only after that review and required qualification. Synchronize issue evidence/states after upstream delivery, keep failed evidence, and remove only eligible clean completed worktrees. No npm release, SDK artifact rebuild, or CABI adoption is silently included in this measurement issue.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
