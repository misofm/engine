ready to push

# Decision-15 dfg batch verdict: `codex/d15-batch-dfg` at 622aba3b1

Base: main `2a589d360` (branch base). Current `origin/main` is `5e7f9c0b0`; the batch merges onto it
with no conflict (merge tree `21769f04`). Slices: #1326, #1268, #1498, #1471 (verdicts PASS:
1326-attempt1, 1268-attempt1, 1498-attempt1, 1471-attempt3), plus follow-ups and text commits.

**Summary:**
- Every `qualification.yml` step that an x86_64 runner can run passes at `622aba3b1`: **110 steps,
  0 non-zero exits**. This includes the three browser legs.
- Shipped module: `f9e711daba98d87aa81d5a8ebfceafa25a9ea2fd2ff20091bc58e4faf71cda41`. This is the
  value in the #1471 record. The twin from another path and `CARGO_HOME` gives the same bytes.
- C ABI audit: `pcm_digest` `cb10fbface44a3a4`, 0 allocations, 0 deallocations, 0 locks,
  0 syscalls, 0 violations. `check-capi-abi.sh`: ok (shared and static) and its mutations ok.
- The pinned ceilings and digests agree with the batch (see "Pins" below).
- No blockers.

## Method

- Export: `git -C /home/bl/misofm/wt-d15-dfg archive 622aba3b1 | tar -x -C /tmp/claude-1002/vdfg/tree`.
  For the gates that need git metadata, I made the export a throwaway one-commit git repository
  (`git init; git add -A -f; git commit`). Its tree is `a1923518`, which equals the tree of
  `622aba3b1`. I did not edit, build, commit or check out anything in the worktree.
- The router and the artifact-identity `report` ran in a `git clone --shared --no-checkout` of
  `/home/bl/misofm/engine/.git` at `/tmp/claude-1002/vdfg/clone`. Only that clone got the synthetic
  merge commit. The worktree's object store was only read.
- Harness: batch-fp's harness (`/tmp/claude-1002/vbatch-fp`), copied and re-pointed. Each CI job ran
  in its own copy of the export with `CARGO_TARGET_DIR=/tmp/claude-1002/vdfg/target/<job>`,
  `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `GITHUB_EVENT_NAME=pull_request` and
  `GITHUB_SHA=622aba3b1...`. Each step's `run:` text was extracted from this commit's
  `qualification.yml` with a YAML parser and ran as `bash -e`.
- The extracted steps differ from batch-fp's only in the two test-debug-a steps. They now add the
  `control-plane/test-support` feature (the workflow change in this batch).
- Disk: 498 GB free at start; lowest 469 GiB. The 25.5 GiB watchdog never fired.

## Gates at 622aba3b1

| Job | Result |
|---|---|
| route | `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass. Router (`--base 5e7f9c0b0 --head 622aba3b1`, pull_request and push): `route=full`, `math_closure=true`, `release_inputs=true`, `self_tests=[]`. |
| docs-gates | DSP research corpus and listening preregistration validators: pass. |
| gate-self-tests (5 steps) | All pass. CI skips this job here (`self_tests=[]`); I ran it anyway. |
| lint (32 steps) | All pass: fmt, clippy `--all-features -D warnings`, `cargo doc -D warnings`, workspace policy and mutations, session, env vocabulary, bench, test-support CI, scalar-oracle self-test, script reachability, console fixture, builtins fixtures, bench preconditions, host-core, protocol-control, realtime policy (new allow-list entry `crates/capi/tests/design_bound_cache.rs`, which `docs/REALTIME_DEPENDENCY_POLICY.md:206` lists) and mutations, trace validator, lane policy, unfused seal, rack/builtins/graph, effect-runtime, conformance, parametric-EQ seal, release-shape self-test, npm publish modes, stem-store, stem identity, the two sub-v3 refusal probes and the AVX2+FMA probe. |
| test-debug-a (3) | Builtins-compiler compile: pass. Debug tests: **1,507 passed, 0 failed, 10 ignored**; this includes `capi/tests/design_bound_cache.rs` (5 tests) and `host-web/tests/retained_ceilings.rs` (1 test). Workspace doctests: 22 passed. |
| test-debug-b (3) | **914 passed, 0 failed, 26 ignored**. DSP doctests: 7 passed. Research-fixture completeness: pass. |
| test-release (9) | G-gates: 133 passed, 11 ignored. `filter_liveness`: 25 passed. `tail_contract`: 15 passed. Ramp endpoint, designer total, M3 built with FMA, loom SPSC, M1 and F1 exhaustive: pass. |
| artifact | Module `f9e711da...cda41`, named twin `a794dc77...`, closure `cd2288b8...`, rustc 1.97.1. |
| artifact-identity | Self-test: pass. Twin byte-identical. Report against the synthetic PR merge (parents `5e7f9c0b0`, `622aba3b1`; tree `21769f04`): **ARTIFACT CHANGED** `7cbdcff0...` -> `f9e711da...` against main's recorded run 38146658302. This is expected: #1326 and #1471 change host-web. The pin `6c952a2c...` and `npm-publish.yml` are not edited, so this is not a release change, and the report passes. |
| artifact-gates (6) | Digest and named-twin checks: pass. `check-web-audioworklet.sh` and `check-browser-expected-resources.py --artifacts`: pass (32 red mutations). Scalar oracle absent: 2,847 symbols, none of 17. `test-web-audioworklet.sh`: pass; boot contract `callers=6 real-ready=6 real-disposed=6`. V8 spill gate: ok (Node 22.23.2). |
| sdk (3) | "SDK publishable-tarball gate passed". |
| browser: chromium 151.0.7922.34, firefox 153.0, webkit 26.5 | Each leg: "no sdk/dist" before the run (and the step removes it), "sdk bundle: the source ... (CI's mode)", "artifact set: the exact 7-file shipped set is pinned", `npm run qualify -- --artifacts ... --sdk-root ... --browser B --check-matrix --self-test-mutations`, "**all qualification gates passed**". Every render-allocation and SDK render-allocation row is 0 in each browser. |
| audit-native (20) | All pass. C ABI caller audit: 100,000 calls, `pcm_digest` `cb10fbface44a3a4`, 0 allocations, 0 syscalls, 0 violations; record validator: pass. `check-capi-abi.sh`: ok, mutations ok. Release unit tests: 113 passed, 2 ignored. Delay, compressor, EQ, gate/expander, builtins (1M), graph, protocol caller-buffer, realtime syscall trace (1M blocks), probe mutations (7 + 9 + 9), effect (1M): ok. Scalar oracle absent from `libcapi.so`. Graph determinism: 100/100. Builtins fixtures: 50 files. Console fixtures and effect conformance (8 factories): ok. |
| wasm-guests (5) | All pass. wasmtime 47.0.3: 146 cases, 258 comparisons, 0 mismatches. Protocol golden parity: ok. |
| cross-target | "cross-target matrix: PASS". #1018 expected failures unchanged (builtins 4, lane 1). |
| release-shape (2) | Pass. CI runs this job for this batch (`release_inputs=true`). |

## Pins (resource ceilings and digests)

- `hosts/host-web/tests/retained_ceilings.rs`: #1326 re-pinned `builtin_retained_bytes` from 2,176 to
  3,776 (both lane widths). The native test passes at this head.
- `hosts/host-web/tests/browser-v1/expected.json`: `builtinRetainedBytes` 2,048 -> 3,584. The built
  module measures 3,219 of 3,584 (`check-browser-expected-resources.py --artifacts`), and the
  expected.json digests and exact rows agree with the built module. Every other retained row is
  within its budget.
- The committed artifact pin `6c952a2c...` is unchanged and is not the built module, which is
  correct for a change that is not a release (`docs/RELEASE.md`).
- C ABI: the frozen symbol set check passes; `pcm_digest` is unchanged at `cb10fbface44a3a4`.

## Supplementary checks (not CI steps on this commit)

- **Merge with current main.** `622aba3b1` merges onto `origin/main` `5e7f9c0b0` with no
  conflict. On that merge tree (`21769f04`, exported the same way), these steps pass: route 02,
  docs-gates 03, fmt, clippy (`--all-features -D warnings`, a fresh build), and lint steps 08-32.
  `qualification.yml` is the same on both sides; main changed only `nightly.yml`.
- **Two later commits on the branch.** The worktree head is now `daf47d2d0`, two commits after
  `622aba3b1`: `233ee9071` and `daf47d2d0`. They change only `.github/ISSUE_SPECS` (#1268 and
  #1471 text, new #1515 and #1516 specs). The router classifies `622aba3b1..daf47d2d0` as
  `route=evidence`. At `daf47d2d0` these pass: route 02, docs-gates 03, workspace policy and its
  mutations, test-support CI, script reachability. No code, workflow or pin changed, so this
  verdict carries to `daf47d2d0`.
- **GitHub (read only).** #1326, #1268, #1498 and #1471 are OPEN, and their titles equal the local
  H1s. #1515 and #1516 exist, are OPEN, and match their H1s.

## Not run, and why

- `aarch64-debug` and `aarch64-release`: they need arm64 runners. I did not emulate them, as
  instructed. So the four-lane `builtin_retained_bytes` ceiling (3,776, inferred and not measured,
  as the #1326 note says) is checked only by CI. The cross-target job's AArch64 iOS and Android
  `cargo check` and clippy rows ran and pass.
- `artifact-record`: runs only on a push to main and writes a commit status.
- `verdict`: aggregates GitHub job results. On its table, `gate-self-tests` would be skipped and
  every other x86 job, `release-shape` included, would succeed.
- Install, cache, upload and download steps: the tools are installed on this machine. The artifact
  job's files were copied instead of downloaded. "Install pinned browser" ran `npm ci` and
  `npx playwright install B` without its `sudo apt` lines.
- The artifact-identity twin: CI uses `git worktree add`, which I was told not to use. I built the
  twin from a second `git archive` of the same commit at another path with another `CARGO_HOME`.
  The claim it tests is the same.

## Files

- Logs: `/tmp/claude-1002/vdfg/logs/` (one script and log per step, `results.tsv` with 110 rows,
  `progress.log`, `merge/` for the merge tree, `daf47/` for the later head).
- Harness: `/tmp/claude-1002/vdfg/{lib.sh,run-all.sh,job-*.sh,extract.py,steps/}`.
- Deleted: all targets, the exports, the per-job copies, runner temp and the shared clone.
