# Qualify prepared-effect AudioWorklet artifact applicability

GitHub: https://github.com/misofm/engine/issues/644

Parent/delivery peer: #642. Audit parent: #560 CP1. Coordination: #559. Frozen feature source: `690e05174f57dcea02ddf21ec4b42596c0a88bb1`. Current delivered artifact authority: #587. Preserve the delivered #542 and #552/#558 dependency order and do not alter their source, fixtures, pins or evidence.

#642 passed source and release-test qualification, but PR-readiness review found its changed `graph-compiler` production bytes in the shipped `host-web -> host-core -> graph-compiler` dependency closure. This issue owns the bounded artifact applicability decision and any later separately reviewed candidate qualification/pin update. #642 remains the delivery issue and does not create a third slot.

Sol HIGH coordinates scope, checkpoints, GitHub, artifact qualification and pin ownership. Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole executor. Astra LOW reviews scope, evidence, artifact disposition and delivery. No full or compressed compiler streams, `.ll`, `.s`, objects, archives, binaries, `rlib`, `rmeta`, Cargo targets or generated candidates enter Git.

## Stage 1: one identity probe

At exact clean/upstream feature head, record executor/time, cwd/head/upstream/status, toolchain, current pin, literal command/environment and absence of both predeclared paths including dangling symlinks. Create `/tmp/issue644-repin-output` once as an empty non-symlink directory and `/tmp/issue644-repin-evidence` once without overwrite. Then run exactly once:

```text
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue644-repin-output
```

Capture separate complete stdout/stderr and numeric status contemporaneously, postflight tree/output census, current pin, and hashes. Stop on changed preconditions, nonzero status, missing single lowercase 64-hex stdout, nonempty output directory, tree drift or concurrent activity. Do not retry, run the ordinary builder, keep a Cargo target, edit the pin, build a six-file candidate, invoke browser/SDK/static qualification, or delete evidence before Astra review.

If the observed Wasm digest equals the current pin, Astra rules whether the unchanged builder/copied inputs and prior #587 qualification remain applicable. If it differs, stage 1 establishes drift only. Any candidate assembly, static/resource/ABI/native-PCM checks, pin edit, post-pin rebuild, exact six-file comparison, SDK/browser matrix or generated-consumer work requires a pushed scope amendment and fresh Astra PASS. No timing, performance or allocation claim belongs here.

## Objective gates

Stage 1 is attempt 1 and consumes one official builder invocation. Astra must pass the exact clean synchronized brief, source/dependency applicability argument, sole executor and fresh path preconditions before execution. Root commits only compact command/source/toolchain/pin/hash/status/output-census evidence; raw streams and build output stay in `/tmp`. A same-hash PASS permits #642 PR-readiness review after synchronized disposition. Drift requires a bounded stage-2 qualification brief; it does not authorize an automatic repin or weaken prior browser/PCM/ABI gates.
