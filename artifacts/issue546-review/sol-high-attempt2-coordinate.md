Attempt 2 is green and checkpointed by Root.

- Source revised: [run-stem-store-browser-evals.cjs](/home/bl/misofm/engine-operator-live-paths/scripts/operator/run-stem-store-browser-evals.cjs)
- Actual delegated model: `gpt-5.6-luna`, reasoning `high`, confirmed in [luna-high.command.json](/tmp/issue546/attempt2/luna-high.command.json).
- Fixture generation and hashing now occur only after `--path-self-test` returns. The 16 MiB size, 64 KiB chunks, byte formula, SHA-256 operation, and digest passed to the browser probe remain unchanged.
- Temporary path-mode witness: `createHash_calls=0 playwright_loads=0`, exit 0.
- `node --check`: exit 0.
- Direct path self-test: exit 0.
- Existing stem-store gate: exit 0.
- Wrong-root control: exit 1 with the expected `scripts/scripts/operator/...` failure.
- Wrong-HTML control: exit 1 with the expected HTTP 404.
- Both controls restored byte-exactly; both `cmp` checks exited 0.
- Final `git diff --check`: exit 0.
- Source SHA-256 changed from `2639f487…072a869` to `990953c8…cceb7a6`; HTML, stem-store module, and checker identities remained unchanged.

Complete raw evidence is under [/tmp/issue546/attempt2](/tmp/issue546/attempt2). Each captured command has `.command.json`, `.stdout`, `.stderr`, and `.status` companions. The post-checkpoint audit passed in [post-checkpoint-evidence-audit.stdout](/tmp/issue546/attempt2/post-checkpoint-evidence-audit.stdout).

Root checkpointed the revision as `e3b41577`, then integrated current main at `bb6f6175`; the worktree is clean. One intermediate evidence audit failure was preserved: Root advanced HEAD while it still expected `85549518`. No verifier, browser, package installation, benchmark, fixture/Rust gate, Git write, or GitHub write was launched by this coordinator.