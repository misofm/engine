# Issue 495 Luna attempt 1 evidence

Production/test source: `0143434b`, isolated graph-binding branch. Only graph/src/lib.rs changes production/test behavior. The combined owned set is replaced by a borrowed ordered union equality at the same evaluation point; all other validation remains.

Luna raw logs are retained unchanged, including incorrect CLI invocations and the failed RUSTFLAGS override. The separately attributed execution inventory was supplied afterward from agent tool history; it is not contemporaneous log metadata. Earlier full suites precede the final empty-coverage fixture correction. The final focused debug/release logs each execute one exact test at the final candidate.

Root independently ran final committed-source graph library debug/release (57 tests each), strict Clippy using ordinary workspace flags, fmt, diff check, and graph policy. Each root command JSON records exact argv/cwd/source/environment, with raw output and actual numeric process status; all six exits are zero. Root initially inferred from Clippy warnings that Luna's command was nonstrict; the recovered exact command corrects that inference. Existing invalid-path configuration warnings can appear despite `-D warnings`; no lint suppression or configuration edit was made.

This package does not claim source-review PASS, immutable delivery qualification, artifact identity, or timing. Borrowed comparison removes the redundant combined collection only; binding retains other allocations. No allocation-free or measurable-speedup claim.

## Qualified delivery

Astra accepted source at2208961f. Actual ordinary builder atbc9a0d36 completed compilation then exited1 for the old pin mismatch. The explicit artifact ruling authorizes only the observed hash; pin source3e17caf3 then passed verified rebuild, static checks, resources including26 negative controls, hermetic tests, npm install, current Chromium151.0.7922.34/Firefox153.0/WebKit26.5 with self-test mutations, and generated matrix check. Each command has actual argv/cwd/source/log/numeric exit (all7 zero). Final published module hash and size are independently retained. Generated records change only candidate/hash identity atf87c9680; numeric expectations are unchanged. Actual PR review and required CI remain pending.
