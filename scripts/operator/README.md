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
