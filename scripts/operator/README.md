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
`artifacts/` (see "Where records live" below), and the procedures that invoke
them are documented in `docs/`.

One tool here is not a benchmark or a browser workload: `sync-spec-bodies.sh` is a GitHub operator
script that edits issue bodies and titles (it needs a token with issue write access, which CI does not
hold), and a person runs it by hand after the specs it reads are on `origin/main`.

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

## Syncing issue specs to GitHub (#1445)

`sync-spec-bodies.sh` copies the committed specs under `.github/ISSUE_SPECS/` to their GitHub issue
bodies and titles, with checks. It reads each spec only from the `refs/remotes/origin/main` blob,
after proving that ref equals `git ls-remote origin refs/heads/main`. `--check` (the default) is
read-only and prints one class per spec; `--apply --backup-dir DIR` writes only the specs whose GitHub
body and title are an earlier committed state of the spec, saving each issue's old raw JSON first.
Every `gh` call carries `--repo OWNER/REPO` derived from `git remote get-url origin` (github.com URLs
only; the script refuses otherwise, and `GH_REPO` cannot redirect it), and a relative `--backup-dir` or
`--reconcile-dir` is resolved against the directory you run it from. A body that is an earlier committed
spec with its title line and the blank line after it removed counts as a match for `fast-forward`
(never `in-sync`). `--reviewed DIR` takes the directory of an earlier `--reconcile-dir` run: an
`unmatched` issue named on the command line becomes `fast-forward reviewed` while its GitHub body and
title still equal the reviewed files byte for byte (never through `--all` or `--range`). Its header lists the classes and
exit codes, and `test-sync-spec-bodies.sh` is its self-test (a stub
`gh`, a local bare `origin`, no network). Editing this README selects the `dsp-research` suite on the
run that carries it (`scripts/ci-path-router.py` lists this file as a `dsp-research` input); that is
harmless.

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

## Where records live (#1030)

Step records live in `artifacts/steps/` while an optimisation batch is open;
older records live in git history, which keeps every deleted file.

Anything else under `artifacts/` stays only while something live reads or cites
it:

* a workflow or script, including a runner arm that names the folder as its
  output (its overwrite refusal depends on the record being there);
* a test or tool, including a doc comment in a crate;
* a ruling, a surviving doc outside `docs/handoffs/`, a handoff note that an
  open issue cites, an open issue spec, or the body of an open GitHub issue.

Remove the rest. When a surviving doc cites a removed record only as history,
cite the commit that removed it instead (`git show <commit>^:artifacts/...`
reads it). When an open GitHub issue cites one, comment on the issue with that
commit. When a runner arm is retired, delete the folders only that arm named
in the same change.
