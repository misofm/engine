# Prune `artifacts/` to the live step records and the ruling-cited evidence

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 6. No code ruling
is needed. `artifacts/` is history, and git already keeps it. This draft proposes a retention rule
you can accept or change: step 1 below.

## Context

- **Size.** `artifacts/` holds 275 folders and one loose file: 62.4 MB and 10,685 files. That is 84% of the
  repository's tracked files and 61% of its bytes.
  - The largest folders are `issue537-candidate-lowering` (11.2 MB), `steps` (4.0 MB),
    `issue539-candidate-lowering` (3.6 MB) and `issue534-production-wasm` (2.4 MB).
  - By type: `.txt` 18.4 MB, `.jsonl` 15.2 MB, `.log` 11.2 MB, assembly dumps 4.2 MB, and 1,772
    `.gz` files.
- **Nothing reads it at test or CI time.** There is no `include_str!`, `include_bytes!` or file
  read of any `artifacts/` path in `crates/`, `hosts/`, `tools/`, `scripts/`, `sdk/`, `fuzz/` or the
  workflows.
  - The remaining mentions are doc comments: `crates/lane/src/fpenv.rs:75`,
    `crates/lane/src/lib.rs:185`, `crates/lane/tests/b2_interleave.rs:7`,
    `crates/capi/src/ffi.rs:743` and `tools/console-workload/src/lib.rs:218`.
  - Also the output paths of the used-up runner arms, which `04a-…`, `04b-…`, `04c-…` and `R9-…`
    remove.
- **Links into it.**
  - 28 links in 12 rulings, citing 17 folders totalling 2.93 MB:
    `compressor-round1{,-baseline}`, `issue149{,-phase2,-phase3}`,
    `issue163-phase{0,1,2,4}`, `issue163-phase2-wasm-baseline`, `issue175`,
    `issue183{,-post-round2}`, `issue184`, `issue-loop-eq-r1`, `round1-composed`, `strip4`.
  - 23 links in 9 live docs. 19 of them are in `docs/issue880-*.md`, which `08-…` removes.
  - `docs/C_ABI_V1_QUALIFICATION.md` has 2, which go with `R2-…`.
- **CI routing.** Any change under `artifacts/` routes CI to `full`, because
  `scripts/ci-path-router.py:17` treats only `docs/` and `.github/ISSUE_SPECS/` as evidence.
- **Clone size does not shrink.** The git pack is 82 MiB, and AGENTS.md forbids rewriting
  history. The gain is working-tree size, grep noise and review load.

## Smallest closable slice

1. **Retention rule.** Keep:
   - `artifacts/steps/`, the live `--step` records of the console and V8 benchmarks;
   - the 17 ruling-cited folders (2.93 MB).

   Delete everything else. If you prefer, re-point the ruling links to commit permalinks
   (`https://github.com/misofm/engine/tree/a9414c0c/artifacts/…`) and delete those folders too.
2. **Delete the other 257 folders and the loose file** `artifacts/issue470-wasm-resource-derivation.md`.
   - Some names look alike but are unrelated, so leave them alone: the `/artifacts/…` URL routes
     in host-web's qualification JavaScript (a served build-output directory), and
     `check-workspace-policy.sh:247`, which scans the tracked-path list for `artifacts/*.ll`.
3. **Re-point or drop** every remaining link from a live doc (outside `docs/handoffs/`, and not
   removed by `08-…`) that names a deleted folder. Use a commit permalink, because the target no
   longer exists in the tree.
4. **Edit the five doc comments** above to cite the issue number instead of the folder. These are
   comment-only edits in `lane` and `capi`. `lane` is in the shipped closure, so see gate 3.
5. **State the rule in `scripts/operator/README.md`:** records live in `artifacts/steps/` while an
   optimisation batch is open, and older records live in git history.

## Objective gates

1. **Build:** `cargo check --locked --workspace --all-targets --all-features` passes, and
   `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
   passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change.
3. **Shipped artifact:** build base and change on one machine with
   `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`.
   - The comment edits in `crates/lane` can shift panic line numbers. Prove with a
     function-by-function `wasm-objdump -d` comparison that nothing else changes, and re-pin with
     that reason.
   - Or do step 4 without changing any line count, for example by editing the comment text in
     place.
4. **CI routing:**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-artifact-evidence-leak.sh`, `test-artifact-evidence-leak.sh`, `check-dsp-research.sh`,
     `check-builtins-listening.sh` and `check-bench-preconditions.sh` pass.
   - `rg -n 'artifacts/' crates hosts tools scripts sdk fuzz .github/workflows` names only `steps/`,
     kept folders, or build-output directories under `target/`.
5. **No live claim lost:** no test or check reads a deleted file. This is shown by the `rg` in
   gate 4 and by a green run of the full CI route.

## Dependencies

`04a-…`, `04b-…`, `04c-…` and `05-…` first, so no runner still names a deleted folder. Batch this
with `08-…`, so the full CI route runs once.

## Standing rules for the implementer

- Delete only. Do not rewrite history, do not move records to another path in the same change, and
  do not touch `artifacts/steps/`.
- Commit on `codex/<issue>-prune-artifacts`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F15. The central claim holds.

1. **Reproduced:** 275 folders plus one loose file, 10,685 of 12,789 tracked files, 62,445,918 of
   102,503,598 bytes; 28 ruling links in 12 rulings naming 17 folders (2,931,608 bytes); 23 live-doc
   links in 9 files, 19 in `docs/issue880-*`. No script, test, jq validator,
   `check-step-vocabulary.py`, `run-console-benchmark.sh`, its preflight or
   `run-web-mixing-automation-benchmark.sh` reads a sealed record outside `artifacts/steps/`; the
   only readers of real folders are three test scripts deleted together with their folders by
   04b/04c. On a scratch copy with this draft, 06 and 08 applied, `check-dsp-research.sh`,
   `test-dsp-research.sh`, `check-builtins-listening.sh`, `check-session-policy.sh`,
   `test-session-policy.sh`, `check-artifact-evidence-leak.sh`, `check-bench-preconditions.sh`,
   `check-step-vocabulary.py`, `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
2. **Missed: citations that live only on GitHub.** Open #560 (no local spec) points readers at
   `artifacts/issue470-delivery-qualification`, the loose `artifacts/issue470-wasm-resource-derivation.md`,
   and the `issue555-*`, `issue557-qualification`, `issue558-*`, `issue563-*` and `issue570-*`
   folders as "durable raw evidence". Gate 4's `rg` cannot see issue bodies. Add a step: grep the
   bodies of all open issues (`gh issue list --state open --json number,body`) for `artifacts/`
   and either keep those folders or post a comment on each issue with the commit permalink.
3. **CI cost of step 4:** the doc-comment edits in `crates/lane` also turn on the router's
   `math_closure` (the M1 and F1 exhaustive sweeps in `test-release`), not only the `full` route.
   Prefer the in-place comment edit that keeps line counts, and batch it with 08 as drafted.
4. **Keep `artifacts/compressor-round1/`** (already in the 17): the wasm floor rule of
   `docs/rulings/effect-floor-accounting.md:815-840` derives its residual from it (R9 amendment).
5. Mobile scope: no effect.
