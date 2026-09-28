# Operator tools

The browser workloads here are **not gates**. The one CI-facing exception is
the runner's browser-free path self-test, which does not run a browser or a
timed workload.

Each produces evidence on demand during work a human initiates: benchmark
preflights and runners, listening-test preparation, and the stem store's
browser evals (which need Playwright and downloaded
browsers). The hermetic stem-store gate invokes only the browser eval runner's
browser-free `--path-self-test`; it does not install Playwright, launch a
browser, or run a timed workload. Their output is the sealed records under
`artifacts/`, and the procedures that invoke them are documented in `docs/`.

They live here, separately from `scripts/`, because of a rule that now holds
for everything above this directory:

> **Every script under `scripts/` is reachable from a GitHub workflow.**

That rule is mechanically checked: `scripts/check-script-reachability.py` (run
by the lint job) fails on any file above this directory that no workflow
reaches. The rule exists because it was previously false in a way nobody could
see. Historical note: the retired `scripts/sweep.sh` ran 102 gate
rows and was invoked by no workflow and by no human — so a crate move
silently blinded five gates while the suite printed 101/101 PASS, and a
rename left a C-ABI evidence ledger verifying 0 of its 26 rows. Both were
invisible for days. Every one of those rows now runs from CI instead.

Keeping operator tools inside `scripts/` would make that rule unenforceable,
because every future audit would have to re-derive which unreachable scripts
are fine and which are dead. Here, the answer is the directory.

## Which benchmarks exist (#1039)

Only real host paths are benchmarked (owner ruling R9,
`docs/rulings/engine-footprint-2026-09-28.md`):

* the native console `--step` rows, `run-console-benchmark.sh` here (the
  host-core compile path at the native lane width), and
* the V8 rows on the shipped `host_web.wasm`,
  `scripts/run-web-mixing-automation-benchmark.sh run WORKDIR --step N`.

#1039 retired the wasmtime console benchmark (`run-wasm-console-benchmark.sh`
and its preflight, which timed Cranelift rather than a browser engine) and the
nightly descriptive native benchmarks (conformance, realtime, session,
effect-contract and FP-environment). Their sealed records that a ruling or a
live note cites stay under `artifacts/`; git history keeps the rest.

**If you add a script under `scripts/`, wire it into a workflow.** If it is a
tool a person runs deliberately, it belongs here instead — and say in its
header what invokes it and what it produces.
