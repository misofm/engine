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

## Preserved-observation evidence — PASS (limited)

Hypatia completed the authorized read-only reconciliation without writing. Astra
LOW independently reviewed exact clean branch/upstream
`03f4f9703138e385c978e746ca8916ffb0af3d10` and returned **EVIDENCE PASS** for the
limited frozen-corpus claim below. #654's measurement procedure remains FAIL.

The detached candidate and counterfactual sources are both exact
`785838403d02e383ba12553a3464d492e60a5cd5`. The candidate is clean. The
counterfactual differs only in the three specified graph-compiler files. Toolchain
identity is rustc 1.97.1 commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`,
LLVM 22.1.6, host `x86_64-unknown-linux-gnu`, and cargo 1.97.1 commit
`c980f4866`. The candidate, counterfactual, and validator commands and environments
are exactly the literal commands in #654's frozen amendment; recorded statuses are
0, 0, and 0.

Exact source SHA-256 identities:

| Path | Candidate | Counterfactual |
|---|---|---|
| `compile.rs` | `109949399f4da4fc44078eb3121dbc6939bc5f635ba34c7789883d7a377f6c53` | `ecfe271b944f63d921a1ec64d6e74512d4e259435bb02104997491122fcabb26` |
| `ids.rs` | `5c6ade8f0887bec4a0c39d4413929bb526cf1d822d7e66c6835961751d0ede58` | `08a5a2265f31061acbc33737438bf1d27c08c9f5c7010bca82825e61eaf1d115` |
| `banks.rs` | `981b0fa269401ef1f985e4eeffc4580b1bc66c2e0bd87f4333e8486444ef0f4b` | `997ccfda181b6ffa01472d380e1e4c777eaceaceeefb158f9ddbb58af9216475` |

Anchored stream SHA-256 identities are candidate stdout
`3a410c69d08ca61ccf347a5a615be7b14794f4611c00ee47cd62be4d1b885b48`, candidate
stderr `6c44fbd2c2a93e6d8f0b064f7660975b1ddd10479f027b4abc265e6e8e2bd389`,
counterfactual stdout
`f6afaabd1b277a78ca4b6f52e94acdf07e909fb90839a2070f1e017ce9e19fad`,
counterfactual stderr
`f13c885206d06eef4a892b05d01541b131581c1ec736759105b7f9117a6057d2`, and combined
JSONL `15029acf4832042852bd4d8bd64a7297478436951e398e4b0c122470aa1d171f`. Read-only
comparison proved the combined bytes equal candidate stdout immediately followed by
counterfactual stdout, with six records from each variant.

Both internal rounds match exactly. All diagnostic identities are
`47bbd81f757f46e0e8108fd525f4deacfb71a2182c3465ab7975390a924957c2`.
Exact counters and counterfactual-minus-candidate deltas are:

| Corpus / variant | Allocations | Deallocations | Reallocations | Requested bytes | Graph SHA-256 |
|---|---:|---:|---:|---:|---|
| zero64 candidate | 20,527 | 15,913 | 233 | 3,001,967 | `e585f5f04deb5e8e520327279e07219a5c5be4c873cdb97530b1169f18bd553e` |
| zero64 counterfactual | 20,527 | 15,913 | 233 | 3,001,967 | `e585f5f04deb5e8e520327279e07219a5c5be4c873cdb97530b1169f18bd553e` |
| zero64 delta | 0 | 0 | 0 | 0 | equal |
| crossed-small candidate | 2,257 | 1,752 | 50 | 276,599 | `d1e23bbacbc343691ee1c86d31b1847edc2af2a86e081990e01f26110a844ff1` |
| crossed-small counterfactual | 2,371 | 1,866 | 50 | 279,687 | `d1e23bbacbc343691ee1c86d31b1847edc2af2a86e081990e01f26110a844ff1` |
| crossed-small delta | 114 | 114 | 0 | 3,088 | equal |
| banks64 candidate | 35,738 | 27,916 | 355 | 5,075,495 | `cebd8ba7421bd00b6f93640ac0837b3812af6353557e141563c84fc542de18b9` |
| banks64 counterfactual | 37,564 | 29,742 | 355 | 5,132,635 | `cebd8ba7421bd00b6f93640ac0837b3812af6353557e141563c84fc542de18b9` |
| banks64 delta | 1,826 | 1,826 | 0 | 57,140 | equal |

This proves only that the frozen candidate observations preserve semantic and
diagnostic identities, leave all four zero64 counters unchanged, and reduce
allocation calls/requested bytes by 114/3,088 for crossed-small and 1,826/57,140
for banks64 relative to the exact three-file counterfactual.

The initial self-matching concurrency failure, continued execution, corrected
population check and missing retained failed-check status remain explicit. Root's
transcript observation and executor testimony retain their limited authority.
Preflight/setup/status/population/summary/validator metadata without prior hash
anchors is supported by current contents and mtime chronology, not proven
immutable. Absent extra logs do not independently prove once-only execution. This
record makes no timing, peak/live-memory, percentage, historical-count, broader
corpus or procedural-PASS claim.
