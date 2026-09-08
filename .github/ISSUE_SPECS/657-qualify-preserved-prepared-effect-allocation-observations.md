# Qualify preserved prepared-effect allocation observations

GitHub: https://github.com/misofm/engine/issues/657

Parent: #560 CP1. Failed measurement predecessor: #654. Harness predecessors: #650 and #652. Product predecessor: #633. Coordination: #559. Frozen measurement source: `785838403d02e383ba12553a3464d492e60a5cd5`. Coordinator failure record: `d76f0279`.

#654 achieved source PASS, then produced one candidate run, one matched-counterfactual run and one validator run whose numerical gates passed. #654 nevertheless closed MEASUREMENT FAIL because the executor continued after a self-matching concurrency preflight and corrected a post-run 12-per-variant population assertion to the specified 6+6. No official command was rerun. This evidence-only successor decides whether the unchanged preserved streams and root's contemporaneous pre-delegation observation support limited qualification credit. It never reruns or repairs the workload.

## Frozen inputs and ownership

All #654 temporary source worktrees, targets and records remain preserved at:

```text
/tmp/issue654-measurement-evidence
/tmp/issue654-candidate-source
/tmp/issue654-candidate-target
/tmp/issue654-counterfactual-target
/tmp/issue654-counterfactual-source
```

This issue owns only this spec, compact conclusions in this spec, and #559/#560 coordination text. It owns no Rust, Python, production source, manifest/lock, workflow, artifact directory, benchmark, timing, target, raw stream, JSONL, compiler output or pin. It creates no new evidence path.

Root's immediately pre-delegation read-only command reported all five paths absent and its process scan contained only that shell and `rg`; this observation exists in the coordinator tool transcript, not a retained file. The executor attested that its first shell-level grep self-matched before creating any path or worktree, but did not preserve that failed check. The later `00-preflight.txt` was written after the evidence directory existed and candidly says so. These statements may be weighed but must never be relabeled as independently retained preflight proof.

## One read-only reconciliation tranche

After Astra LOW scope PASS, one named Luna HIGH analyst may read only the five preserved paths and repository/Git histories. It may run read-only filesystem census, `sha256sum`, `git status`, `git rev-parse`, `git diff`, JSON parsing, and comparison commands that do not invoke Cargo, rustc, the audit binary, the validator, or any workload. It must not write, touch timestamps intentionally, reconstruct or replace a missing record, concatenate streams again, clean targets, remove worktrees, edit source, or create an artifact/evidence directory.

The analyst reports to root, without writing files:

- complete preserved file census and hashes, including candidate/counterfactual stdout/stderr, combined JSONL, status records and validator records;
- exact mtimes/order sufficient to check candidate then counterfactual then concatenation/population check then validator chronology;
- detached source heads, candidate cleanliness, exact three-file counterfactual status and the six frozen source hashes;
- exact once-only command strings, environments and numeric statuses from contemporaneous records;
- proof that combined JSONL bytes equal the preserved candidate stdout bytes followed immediately by preserved counterfactual stdout bytes;
- exact 6+6 population, two-round equality, graph/diagnostic identity equality, zero64 equality, and per-corpus counters/deltas;
- the initial unretained concurrency failure and corrected population check as explicit procedural defects;
- whether any record was modified after the final #654 summary or after this successor's scope PASS.

Stop on a missing/changed input, unexpected source/worktree state, unverifiable chronology, a second official command indication, byte mismatch, inconsistent status, numerical gate failure, or any need to reconstruct evidence. Do not correct or rerun.

## Decision and delivery

Astra LOW independently reads the same preserved inputs and the analyst report. PASS may grant only this claim: at the frozen source and exact matched three-file counterfactual, the single preserved observations are deterministic across their two internal rounds, semantically/diagnostically equal, causally unchanged for zero64, and show the exact observed prepared-corpus allocation/requested-byte reductions. PASS does not convert #654's procedure to PASS, prove timing/peak/live memory, generalize beyond the frozen corpora, or authorize a rerun.

If Astra cannot bridge the two candid procedural defects from the immutable streams, chronology and root observation, close this issue without qualification credit. If Astra passes, root records only compact source/toolchain/command/status/hashes, graph/diagnostic identities, exact counters/deltas, defects and limited conclusion in this spec. Full streams, JSONL, targets and compiler/build output stay temporary and never enter Git.

After evidence PASS, ordinary exact-head/current-main review, required CI, guarded merge, successful post-main qualification, GitHub/tracker synchronization and clean delivered-worktree removal are required. Temporary #654 inputs may be removed only after Astra explicitly releases them and the compact evidence is upstream. This closes only the prepared-effect allocation qualification slice; CP1 schedule, PDC, cycle, reduction and buffer identities remain open.

## Astra LOW evidence-scope review — PASS

Astra passed exact clean branch/upstream
`d76ceef28aa42017f659c636a56405eda0d1b560`, live main
`8c6984bc243dc507a137e6f836924b85f1db3658`, synchronized tracker `0c35123a`, and
matching GitHub #657. #654 is closed; #656/#657 are disjoint; all five preserved
paths exist as non-symlink directories.

One root-named Luna HIGH analyst may perform only the listed read-only census,
hashing, Git inspection and JSON comparisons and report without writing. Mtimes are
supporting chronology rather than immutability proof. Compare preserved hashes to
contemporaneous anchors where available, name every unanchored file, and do not
infer once-only execution merely from absent extra logs. Root's transcript and the
executor testimony retain their stated limits. No workload/validator/Cargo run,
record reconstruction, new path, concatenation, cleanup or source edit is
authorized. Any unresolved inconsistency stops; Astra independently reviews the
result before limited credit.
