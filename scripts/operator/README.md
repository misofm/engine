# Operator tools

These are **not gates**. Nothing in CI runs them, and that is deliberate.

Each produces evidence on demand during work a human initiates: benchmark
preflights and runners, listening-test preparation, a browser-correctness seal,
and the stem store's browser evals (which need Playwright and downloaded
browsers, so they cannot be hermetic CI rows). Their output is the sealed records under `artifacts/`, and the procedures
that invoke them are documented in `docs/`.

They live here, separately from `scripts/`, because of a rule that now holds
for everything above this directory:

> **Every script under `scripts/` is reachable from a GitHub workflow.**

That rule is mechanically checkable, and it exists because it was previously
false in a way nobody could see. `scripts/sweep.sh` ran 102 gate rows and was
invoked by no workflow and by no human — so a crate move silently blinded five
gates while the suite printed 101/101 PASS, and a rename left a C-ABI evidence
ledger verifying 0 of its 26 rows. Both were invisible for days.

Keeping operator tools inside `scripts/` would make that rule unenforceable,
because every future audit would have to re-derive which unreachable scripts
are fine and which are dead. Here, the answer is the directory.

**If you add a script under `scripts/`, wire it into a workflow.** If it is a
tool a person runs deliberately, it belongs here instead — and say in its
header what invokes it and what it produces.
