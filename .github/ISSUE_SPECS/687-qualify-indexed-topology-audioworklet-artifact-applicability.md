# Qualify indexed-topology AudioWorklet artifact applicability

GitHub: https://github.com/misofm/engine/issues/687

Parent/delivery peer: #685. Audit parent: #560 CP1. Coordination: #559.
Frozen source predecessor: `f3f70e546d5f96b372be828fb02ea7c643ddd788`.
Current delivered AudioWorklet pin:
`93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`,
delivered through #680/PR #682 at main `8999def5`.

## Problem

#685 received Astra LOW SOURCE/EVIDENCE PASS for its bounded topological-
scheduling implementation. `graph-compiler` is in the shipped
`host-web -> host-core -> graph-compiler` dependency closure, so source PASS does
not establish that the currently pinned AudioWorklet artifact represents the
accepted source. Lane B alone must decide applicability and own any later
qualification or pin change before #685 can deliver.

This issue begins with one identity probe. It does not assume drift and does not
reuse, repair, or rerun #644/#680 artifact evidence. All prior failed and retained
state remains preserved.

## Stage 1: one repin-report identity probe

One designated Luna HIGH or XHIGH executor may run the builder once in repin-
report mode from an exact clean pushed branch after Astra LOW passes the numbered
brief. Before creating anything, record and read back:

- executor identity and UTC start;
- cwd, exact HEAD/upstream equality, merge base and frozen #685 product commit
  `276ffb6097a84088e3b5f4a16892a33bca9e26fb`;
- empty repository porcelain including untracked files;
- `rustc -Vv`, `cargo -V`, literal command and complete relevant environment;
- current delivered pin; and
- absence, including dangling symlinks, of both predeclared paths:
  `/tmp/issue687-repin-output` and `/tmp/issue687-repin-evidence`.

Stop before creation on any mismatch or concurrent Cargo/rustc builder activity.
Create the output as an empty non-symlink directory and the evidence directory
once without overwrite. Run exactly once:

```text
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue687-repin-output
```

Capture the exact argv/environment/cwd/head, start/finish, complete stdout and
stderr, numeric status, postflight porcelain, output census, delivered pin, and
SHA-256/size/mode of every record. Stop on nonzero status, anything other than
one lowercase 64-hex digest plus LF on stdout, a nonempty output directory, tree
drift, or changed source/authority. Do not retry, run the ordinary builder,
inspect compiler payloads, retain a Cargo target intentionally, edit a pin or
lineage file, assemble a six-file candidate, run static/resource/hermetic/SDK/
browser gates, or remove evidence before Astra review.

## Decision rule and ownership

If the observed digest equals the delivered pin, Astra LOW decides whether the
existing artifact qualification transfers to #685. If it differs, stage 1 proves
only drift. Candidate assembly, qualification, browser execution, repository pin
or lineage edits, post-pin rebuild, PR, merge, and delivery require a pushed stage
2 amendment and fresh Astra LOW SCOPE PASS.

This issue owns only its numbered spec, compact decision records, concise #559/
#560 coordination rows, and any later explicitly amended lane-B artifact/pin
paths. Stage 1 owns no tracked product, artifact, pin, result, matrix, SDK, host,
compiler, manifest, lockfile, dependency, policy, or workflow edit. Full streams,
Wasm, generated files, Cargo targets, binaries, objects, libraries, compiler
outputs, and raw evidence remain temporary and never enter Git.

No timing, allocation, throughput, percentage, improvement, regression, budget,
or performance claim is authorized. #685 remains the sole implementation issue;
this qualification peer uses the second shared slot. Lane A owns no artifact or
pin action.

## Review and delivery

Astra LOW must pass the exact clean synchronized brief and fresh-path/executor
preconditions before Luna executes stage 1. Astra then adversarially reviews the
preserved output and compact record. The normal three-attempt ceiling applies,
but the single repin-report invocation is consumed once it starts and may not be
rerun under another attempt. A procedural or builder failure is recorded without
repair unless a separately reviewed next stage can use immutable outputs without
rerunning.

After an artifact decision passes, #685 still requires exact-head/current-main PR
readiness, required `qualification`, guarded live head/base review, exact-merge
post-main qualification, GitHub/spec synchronization, closure, and clean removal
only of delivered worktrees. Preserve all failed worktrees, branches, histories,
targets, and temporary evidence named by #559/#560, including #668 and the soft-
clip chain. Do not inspect, rewrite, repair, rerun, or remove them here.
