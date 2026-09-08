# Issue 607 pull-request qualification review

Reviewer: Astra LOW

Reviewed head: `20a9265cd705fe33d76657a37ce44c43fc1e7997`

Pull request: #609

Qualification run: `34195043020`

Verdict: **PASS to stop; FAIL to correct within issue 607**.

The required `fmt, clippy, doc, and hermetic policy gates` job failed its environment/marker
vocabulary step. `check-env-vocabulary.sh` found 20 identifiers used by the inherited capture scripts
but absent from `docs/ENGINE_ENV_VOCABULARY.md`: nine `MISO_ENGINE_606_*` test/control variables, ten
`MISO_ENGINE_CAPTURE_*` subprocess metadata variables, and the `MISO_ENGINE_CAPTURE_PHASE` output
marker. Static source/table comparison independently confirms the complete list preserved in focused
qualification evidence.

This is a substantive required-policy failure. Issue 607 permits only its preflight script and
focused evidence, authorizes one implementation pass and no correction, and requires a successor for
any substantive defect. Editing the central vocabulary document would exceed that scope. PR #609 is
closed without merge, and issue 607 stops with its accepted capture preserved but undelivered.

The smallest successor may own only the 20 exact vocabulary rows, its numbered spec, and bounded
review/qualification evidence. It inherits the implementation and capture unchanged and authorizes no
preflight, runner, capture, retry, resume, or timing.
