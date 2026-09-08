# Issue 539 unauthorized combined-source artifact observation

Status: **INVALID FOR QUALIFICATION CREDIT — retained candidly; do not rerun.**

One ordinary no-bypass builder invocation was launched in the background from clean pushed head
`d63bc437e948d6284b1b6cbe459f0b46c4ed6566`, after verifying current main
`773682433ef451b89e5359fa8f722e1016c64fb3`, but it ran without root authorization after Astra LOW
had blocked artifact work on evidence hygiene. It therefore does not satisfy #539's reviewed
artifact-decision step and does not authorize a successor or pin update. Compilation succeeded,
then the unchanged builder exited 1 on pin mismatch:

- Delivered pin: `e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`.
- Combined-source candidate: `f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`.

The fresh external output directory is empty because pin verification precedes output copying;
the builder removed its temporary compiled module on exit. Six-file identity cannot be granted.
The earlier standalone limiter candidate and delivered CP8 candidate do not qualify this observed
combined-source digest. The observation may be considered only by a later Astra ruling; it must not
be presented as qualified candidate evidence.

Exact argv, cwd, source/main identities, toolchain, input hashes, full build streams/status and
output census are adjacent. No builder retry, static/resource/browser qualification, pin/lineage/
consumer/source edit, benchmark, timing/capture, commit, push or GitHub mutation occurred.
The background helper restored the clean committed worktree while root's evidence-packaging
correction was in progress. Root detected that race, waited for the helper to finish, and reapplied
the lossless packaging correction. No committed source or product byte was changed. This directory
preserves the unauthorized invocation exactly so the workflow violation is not hidden.
