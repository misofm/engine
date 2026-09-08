# Issue 539 combined-source artifact decision

Astra LOW lane-B decision: **DRIFT — separately numbered artifact successor required.**

Root explicitly authorized exactly one ordinary no-bypass builder invocation after Astra LOW's
integrated-source PASS. The first worker turn stopped before builder launch when its clean-tree
assertion detected concurrent evidence-only changes. Root restored the exact clean pushed head and
then explicitly authorized the still-unused invocation. The resumed worker rechecked clean HEAD and
upstream at `d63bc437e948d6284b1b6cbe459f0b46c4ed6566`, with current main
`773682433ef451b89e5359fa8f722e1016c64fb3`, before launching the builder once.

Compilation succeeded, then the unchanged builder exited 1 on pin mismatch:

- Delivered pin: `e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`.
- Combined-source candidate: `f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`.

The fresh external output directory is empty because pin verification precedes output copying;
the builder removed its temporary compiled module on exit. Six-file identity cannot be granted.
The earlier standalone limiter candidate and delivered CP8 candidate do not qualify this combined-
source digest.

Exact argv, cwd, source/main identities, toolchain, input hashes, full build streams/status and
output census are adjacent. No builder retry, static/resource/browser qualification, pin/lineage/
consumer/source edit, benchmark, timing/capture or GitHub mutation occurred. The later evidence-
packaging correction changes no compiled input and does not affect this decision.
