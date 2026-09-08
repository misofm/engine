## Verdict

TOOL14 remains partially open, but it is closable with one bounded non-audio issue.

The original row is under-specified because it records counts without naming every target. Correlating its FLAC-generator anchor with current lineage gives this complete disposition:

| Original obligation | Current disposition |
|---|---|
| `stem-publisher` binary | Retired with the FLAC stack by #356. |
| `catalog-migrate` binary | Retired with the FLAC stack by #356. |
| Broken prefixed invocations of those binaries | Retired with their generator; do not repair or restore them. |
| `native-pcm-runner` binary | Retained intentionally, but still not exercised through its actual process boundary. |
| Orphan `rack_fixture` validator and `fixtures/rack/v1` | Still present and only validate each other; justified deletion. |
| Deleted `sweep.sh` references | Historical references should remain; one live operator instruction remains stale. |
| Other live stale references | Stem-identity README has an obsolete prefixed package command; the moved browser-eval tool contains several broken pre-move paths. |

No old count such as “3” or “24” should be reused as a current acceptance claim.

## Exact evidence

- Original compressed finding: [original-349.md](/tmp/issue349-partials/original-349.md:325).
- Handoff correctly leaves the row partial: [349-handoff-findings.json](/home/bl/misofm/engine-audit-handoff/docs/audits/349-handoff-findings.json:852).
- #356 explicitly removed `stem-publisher`, `catalog-migrate`, and the FLAC corpus while preserving historical records: [356-remove-in-repository-flac-delivery-stack.md](/home/bl/misofm/engine/.github/ISSUE_SPECS/356-remove-in-repository-flac-delivery-stack.md:16).
- Their resurrection is now prohibited by workspace policy: [check-workspace-policy.sh](/home/bl/misofm/engine/scripts/check-workspace-policy.sh:162).

The surviving native runner:

- Its executable is only a thin adapter to `run_cli`: [main.rs](/home/bl/misofm/engine/tools/native-pcm-runner/src/main.rs:1).
- The library exercises the real C ABI and frozen corpus directly, but not by spawning the binary: [lib.rs](/home/bl/misofm/engine/tools/native-pcm-runner/src/lib.rs:1502).
- Its current checker is structural and fixture-oriented; it does not execute the binary: [check-native-pcm-runner.sh](/home/bl/misofm/engine/scripts/check-native-pcm-runner.sh:5).
- It remains a documented user-facing command, so deletion is not justified: [NATIVE_PCM_REFERENCE_RUNNER_V1.md](/home/bl/misofm/engine/docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md:9).

The orphan rack validator:

- `rack_fixture` generates, hashes, and validates its own corpus: [rack_fixture.rs](/home/bl/misofm/engine/crates/graph-compiler/src/bin/rack_fixture.rs:22), [rack_fixture.rs](/home/bl/misofm/engine/crates/graph-compiler/src/bin/rack_fixture.rs:129).
- Its only test checks corruption of that self-generated corpus: [rack_fixture.rs](/home/bl/misofm/engine/crates/graph-compiler/src/bin/rack_fixture.rs:179).
- The later exhaustive test audit confirms that nothing else consumes `fixtures/rack/v1` and recommends deletion: [03-compilers-hosts-tools.md](/home/bl/misofm/engine/docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:155), [04-non-rust.md](/home/bl/misofm/engine/docs/audits/test-usefulness-2026-09-04/04-non-rust.md:116).
- This is distinct from the live Issue-038 benchmark corpus and checker at [check-rack-benchmark-fixture.sh](/home/bl/misofm/engine/scripts/check-rack-benchmark-fixture.sh:1).

Live stale references:

- Obsolete crate command: [fixtures/stem-identity/v1/README.md](/home/bl/misofm/engine/fixtures/stem-identity/v1/README.md:18).
- Moved operator tool still documents its old invocation and deleted sweep: [run-stem-store-browser-evals.cjs](/home/bl/misofm/engine/scripts/operator/run-stem-store-browser-evals.cjs:7).
- Its repository root resolves to `scripts/`, not the repository root, and its default page still names the pre-move location: [run-stem-store-browser-evals.cjs](/home/bl/misofm/engine/scripts/operator/run-stem-store-browser-evals.cjs:30), [run-stem-store-browser-evals.cjs](/home/bl/misofm/engine/scripts/operator/run-stem-store-browser-evals.cjs:49).

The dated sweep references in `docs/derivations/`, `docs/rulings/`, historical issue specs, and the explicitly historical paragraph in [scripts/operator/README.md](/home/bl/misofm/engine/scripts/operator/README.md:16) are evidence, not live instructions. Preserve them.

## Proposed stateless issue

**Title:** Complete TOOL14 executable reachability and retire the orphan rack corpus

**Body:**

> Parent: #349 TOOL14.
>
> The original audit combined executable reachability, a broken FLAC generator, an orphan validator, and stale references. The FLAC generator and its `stem-publisher`/`catalog-migrate` targets were retired by #356 and must not be restored. Current source still has three concrete gaps:
>
> 1. `native-pcm-runner` is a retained, documented CLI whose library path is thoroughly tested, but no test invokes the actual executable boundary.
> 2. `crates/graph-compiler/src/bin/rack_fixture.rs` and `fixtures/rack/v1/**` only consume and validate each other. They prove no current graph, rack, benchmark, or host behavior.
> 3. Active instructions contain stale package and post-move paths. The stem-identity README names `miso-engine-stem-hasher`; the operator browser-eval tool names its old location, computes the wrong repository root, serves the old page location, and instructs users to run deleted `scripts/sweep.sh`.
>
> ### Smallest closable implementation
>
> - Add one integration test under `tools/native-pcm-runner/tests/` that spawns `CARGO_BIN_EXE_native-pcm-runner`.
>   - Exercise one accepted 48-kHz frozen fixture through the executable.
>   - Require successful exit, empty success diagnostics, exact 8,192-byte output, and no retained partial file.
>   - Exercise one closed-CLI failure and require exit 2 plus the existing typed diagnostic.
>   - Reuse existing fixture truth; do not introduce another output digest pin.
> - Delete `crates/graph-compiler/src/bin/rack_fixture.rs` and exactly `fixtures/rack/v1/**`.
> - Preserve `fixtures/rack/issue038-v1/**`, `scripts/check-rack-benchmark-fixture.sh`, `graph_fixture`, and all production graph/rack tests.
> - Correct the stem-hasher command in `fixtures/stem-identity/v1/README.md`.
> - Repair `scripts/operator/run-stem-store-browser-evals.cjs`:
>   - document `scripts/operator/run-stem-store-browser-evals.cjs`;
>   - resolve the repository with `../..`;
>   - serve `scripts/operator/stem-store-eval.html`;
>   - replace the deleted-sweep instruction with the current qualification/checker boundary;
>   - add a browser-free path self-test that proves the page and imported stem-store module are served from the real repository.
> - Invoke that hermetic path self-test from the existing stem-store gate. Do not launch Playwright or a browser in this issue.
> - Update the operator README only as needed to distinguish CI’s hermetic self-test from human-triggered browser workloads; preserve its historical account of the retired sweep.
> - Record a present-day executable-target classification. Do not claim closure from the audit’s old numeric counts.
>
> ### Non-goals
>
> - No `tools/audit/**` or TOOL3 dispatcher changes.
> - No DSP, render, graph, rack, session, ABI, fixture PCM, or canonical-byte changes.
> - No restoration of FLAC delivery tooling.
> - No benchmark, timing, browser launch, listening run, or artifact regeneration.
> - No edits to historical issue specs, derivations, rulings, or mutation records merely to erase old names.
> - Do not absorb the later independent orphan recommendations for `fixtures/capi-qualification/v1` or `fixtures/effects/v1`.
>
> ### Objective gates
>
> 1. Cargo metadata contains no `rack_fixture` binary target, while all other current executable targets have an explicit product, workflow, test, or operator consumer classification.
> 2. `crates/graph-compiler/src/bin/rack_fixture.rs` and `fixtures/rack/v1` are absent; `fixtures/rack/issue038-v1` and its checker remain byte-unchanged.
> 3. The new native-runner integration test reaches the real executable and passes its accepted and rejected cases. Existing `native-pcm-runner` tests remain green.
> 4. `python3 fixtures/stem-identity/v1/generate.py --check` and `cargo test --locked -p stem-hasher` pass using the corrected documented package name.
> 5. The operator script passes `node --check`; its browser-free path self-test successfully serves the operator HTML and `hosts/host-web/web/stem-store/index.js`.
> 6. The required stem-store gate invokes that self-test. Mutating either the repository-root depth or the operator HTML path makes the same gate fail without launching a browser.
> 7. Active-source scans contain neither the obsolete stem-hasher package spelling nor a live instruction to execute `scripts/sweep.sh`. Explicitly historical occurrences remain classified and untouched; zero repository-wide matches is not the gate.
> 8. Focused formatting and package tests, proportional workspace tests, and required qualification pass. No timing or browser workload is run.

## Audio impact and coordination

This is non-audio implementation: it changes a CLI-boundary test, deletes an unconsumed historical PCM corpus, and repairs documentation/operator paths. It does not alter render code or PCM semantics. The native-runner invocation is a regression witness over existing frozen output, not authorization to repin it.

Use Luna high for implementation and Sol xhigh for verification. If implementation discovers that any current product test consumes `fixtures/rack/v1`, or requires changing output bytes, DSP, runtime code, or fixture pins, stop and rescope through Sol medium audio coordination and Astra medium verification.

## Overlap

- **CP20:** direct file overlap at [rack_fixture.rs](/home/bl/misofm/engine/crates/graph-compiler/src/bin/rack_fixture.rs:171), whose local SHA-256 hex encoder is part of CP20’s remaining inventory. TOOL14 should delete this file; CP20 must record that disposition and avoid refactoring it concurrently. Integrate TOOL14 first or drop CP20’s hunk for this file.
- **IO1:** no path or semantic overlap.
- **TOOL3:** no path overlap; leave `tools/audit/**` and its dispatcher entirely to that coordinator.
- **Delivered #456:** similarly named but separate. It owns `fixtures/rack/issue038-v1` and `scripts/check-rack-benchmark-fixture.sh`; both are protected from TOOL14 edits.
- **Delivered #356/#319/#454:** use their current boundaries as inputs; do not reopen the FLAC stack, broad script reachability program, or native-runner static checker.