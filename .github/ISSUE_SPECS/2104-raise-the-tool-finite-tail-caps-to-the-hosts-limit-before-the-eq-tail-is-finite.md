# Raise the tool finite-tail caps to the hosts' limit before the EQ tail is finite

Filed by root from the #1374 verification (2026-10-26, MINOR-2).

## Problem
console-workload's `graph_caps` (tools/console-workload/src/lib.rs ~2483) and the audit tools cap finite tails at `maximum_finite_tail_samples: 10_000_000`. Since #1374 the delay states a finite tail of 35M to 77M samples. Today the console-workload sends fixture's output tail stays Infinite only because the parametric EQ is Infinite, and an Infinite tail wins in composition. When #1372 makes the EQ finite, that fixture is refused with `graph.tail.limit`. Hosts already use `u64::MAX` (host-core).

## Smallest slice
Raise the tool caps to `u64::MAX`, as host-core does, or derive them from the same constant. graph_fixture prepares no effects and is not exposed.

## Gates
- console-workload's tests and the audit tests pass.
- One row composes a finite delay tail (for example a delay followed by a finite-tail effect) through console-workload's caps and gets a Finite tail. This catches the cap coming back.

Order: before #1372 merges.

## Paths
tools/console-workload/src/lib.rs, the audit tool cap sites, their tests, this spec.

## Attempt 1 — implementation and evidence (2026-10-27)

Root approved the bounded slice; Sol implements under the owner's explicit agent choice.
The console-workload graph caps and the three audit graph cap sites now use `u64::MAX`,
matching `host-core`. Other resource ceilings and graph compilation semantics are unchanged.
The standalone graph_fixture cap is outside this slice: it prepares no effects.

The single new regression, `tests::tool_caps_admit_a_composed_finite_delay_tail`, takes the
canonical one-track fixture, replaces its EQ with the real default launch delay, removes EQ
parameters/automation, and prepares the real builtins and graph through console-workload's
`graph_caps()`. The delay's finite tail exceeds the former 10,000,000-sample ceiling. Both
composed output-tail reports must remain finite and at least the delay's own tail.

Test value: restoring the old console-workload cap rejects this finite graph with
`graph.tail.limit`, whereas the existing console fixture tests remain masked by an infinite EQ
tail; graph-compiler tests already own tail-composition arithmetic.

All CPU-intensive checks use the shared `run-serial.sh` runner with two Cargo jobs and one
test thread. Full logs are retained outside the worktree at
`/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/2104/`.

- RED: `cargo test -p console-workload --lib tests::tool_caps_admit_a_composed_finite_delay_tail -- --exact --nocapture`
  compiled and failed as intended with `graph.tail.limit` before changing the caps (`red.log`).
- Formatting: `cargo fmt --package console-workload --package audit` passed (`fmt.log`).
- INTERRUPTED: `cargo test -p console-workload -p audit` compiled both packages and passed eight
  audit tests before root requested SIGINT during repeated debug fixture-corpus generation
  (`audit-debug-interrupted.log`). This is not a package-suite pass. Root assigns the full audit
  suite to the fresh verifier's single integrated release run, matching the standing CI profile.
- GREEN: `cargo test --locked -p console-workload` completed successfully: 56 tests passed
  (library 15, automation 8, chain_shape 22, eq_ramping_scenario 1, paired_spans 9, placement 1;
  zero doc tests), including the new regression (`green-console.log`).
- Independent adversarial review and the integrated release suite: pending with root's fresh
  verifier.

GitHub evidence/state synchronization and merged-tree verification are owned by root at the
CI-conscious batch boundary; this local checkpoint does not claim remote completion.

## Independent adversarial review — attempt 1 PASS (Astra, 2026-10-27)

Fresh review of `08fd9fa7277af1729506dab46762cfdd0904645e` confirms all four tool cap sites
match the host finite-tail limit. This changes admission, not memory allocation or tail
composition arithmetic; existing graph compiler checks still own those contracts.

Test value: `tool_caps_admit_a_composed_finite_delay_tail` fails with `graph.tail.limit` when
the former console cap returns, which existing fixtures with an infinite EQ tail mask. It
uses the real launch delay and finite builtins through the actual console cap; graph compiler
tests already own composition arithmetic. The implementation's retained red log proves the
old cap fails.

Independent integrated command `cargo test --locked --release -p audit -p console-workload`
passed: all 27 audit tests and 56 console-workload tests, plus doc-tests (exit 0). This is the
complete release-profile replacement for the interrupted debug suite above. Evidence:
`/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/fresh-astra/release-tools.log`.
Workspace clippy with all targets/features and warnings denied, formatting, workspace policy,
realtime policy, builtins policy and unfused seal also passed. All heavy commands used the
shared serial wrapper. Full required merged-tree qualification and remote issue synchronization
remain root's delivery gates; this record does not claim upstream closure.
