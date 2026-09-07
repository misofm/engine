# Exercise the native PCM runner executable boundary

Parent #349 TOOL14, based on Sol high census at e5b86cf315487fcc602db420dc1a6121f1ac4837. Queued successor; no implementation until assigned an active slot. Root owns Git/GitHub, checkpoints, qualification, exact-head/base review, merge/closure and completed-worktree cleanup. Sol high coordinates Luna high; Sol xhigh verifies nonaudio work. Maximum three counted attempts. Capture actual command/source/exit evidence, preserve failures, no benchmark or new fixture pin. TOOL14 remains partial until all children complete.

The documented native-pcm-runner CLI remains intentionally shipped. Existing library tests exercise its real C ABI/frozen corpus, but do not spawn the executable. Add one integration test under tools/native-pcm-runner/tests/ that uses Cargo's actual binary path, preserving current runtime and library implementation. Exercise one accepted 48-kHz frozen fixture and require successful exit, empty success diagnostics, exact 8192-byte output and no retained partial file. Exercise one existing closed-CLI rejection and require exit 2 and the existing typed diagnostic. Reuse fixture truth; do not create a correlated or new output digest pin.

Scope: the integration test and only necessary existing test support/development dependencies. No CLI/runtime/audio/fixture bytes or parser/error-contract changes. If output expectations contradict existing documented truth, stop and rebrief with evidence; do not change production to make the new test pass. Audio-path changes require Sol medium coordination and Astra medium verification.

Gates: actual process execution for both cases, existing native-pcm-runner tests, affected strict Clippy, fmt/diff, workspace policy and required qualification. Capture process stdout/stderr/exit and output metadata; keep timing descriptive and do not invoke benchmarks. Audit completion requires this actual boundary witness, not only in-process run_cli tests. The orphan rack retirement and live operator path corrections are separate TOOL14 children.

## Numbered TOOL14 disposition

This issue is #547. The complete residual is tracked by #545 orphan rack retirement, #546 current operator/path corrections and #547 native-runner process coverage. Only #545 is currently active; #546/#547 await a slot. FLAC retirement was delivered by #356. Closing one child does not close TOOL14.

## Active slot

#545 is merged through PR549 at86d5b4bd and CLOSED, with its completed worktree removed. #547 now occupies that freed slot alongside #542, #543 and #546. Root activates this frozen smallest process-boundary slice from synchronized main86d5b4bd. Sol high coordinates Luna high implementation; Sol xhigh verifies. Production/audio changes remain out of scope.

## Attempt1 focused-green source checkpoint

Luna high under Sol high coordination added one process_boundary integration test using Cargo actual binary path. Accepted frozen riff-48000 case returns0 with empty diagnostics and8192 output bytes, without a retained partial file. Existing frames.zero rejection returns2 with exact typed stderr and no output/partial. The full runner suite passed19 library plus1 integration tests; strict Clippy, final format, workspace policy and diff checks passed. Direct executable captures independently record both cases. Initial focused compile failed101 for missing crate-level docs and initial formatting failed1; both are preserved and corrected before final gates. Expected rejected-process exit2 is success evidence for the rejection contract.

Only the new integration test changes source; production/CLI/audio/fixtures/pins/manifests/lockfile remain byte-identical. Raw captures are preserved losslessly under artifacts/issue547-attempt1 with original/packed hashes. Sol xhigh review and required remote qualification remain; this checkpoint is not delivery.
