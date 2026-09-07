FAIL — issue 546 attempt 1.

Exact review range:

- Base `origin/main`: `1ce8fd3cb623f8dfe4cf306a01f539a96e8b92ca`
- Head: `a80c94ca16be74ebe204ce7b5931f535ba82af98`
- Merge-base matches base; worktree clean; full committed `git diff --check` passes.

Blocking finding:

- The frozen no-browser-workload boundary is not met. The runner prepares and hashes the full 16 MiB browser-evaluation fixture at module load ([run-stem-store-browser-evals.cjs:32](/home/bl/misofm/engine-operator-live-paths/scripts/operator/run-stem-store-browser-evals.cjs:32)) before `main()` dispatches `--path-self-test` ([run-stem-store-browser-evals.cjs:323](/home/bl/misofm/engine-operator-live-paths/scripts/operator/run-stem-store-browser-evals.cjs:323)). Consequently, every invocation from the existing gate runs browser-workload preparation despite the frozen path-only boundary and the README’s “invokes only” claim. Deferring Playwright loading and browser launch is necessary but insufficient; fixture/digest preparation must also be moved behind the browser-mode branch.

Everything else reviewed correctly:

- Actual HTTP serving checks both the operator HTML and stem-store module against repository bytes.
- The existing gate invokes the self-test.
- Wrong-root and wrong-HTML controls both exited 1 with targeted failures; restoration hashes exactly match the committed four source files.
- Fixture generator and locked `stem-hasher` captures pass; fixture bytes, pins, HTML, module, and workflows are unchanged.
- Current paths/names are repaired; historical sweep/name records remain preserved and classified.
- All 148 compressed captures are uniquely covered by the manifest; packed hashes, decoded hashes, and decoded lengths validate.
- No required qualification-pass capture is present yet; the spec assigns that final delivery gate to Root.