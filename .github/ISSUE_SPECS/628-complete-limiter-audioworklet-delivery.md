# Complete limiter AudioWorklet delivery from preserved evidence

GitHub: https://github.com/misofm/engine/issues/628
Parent: #621 (lane A, FX2). Predecessor: exhausted #627. Coordination: #559/#560.

#627 reached the repository three-attempt hard stop and is closed as superseded without claiming delivery. Attempt 1 failed its exactly-one scratch-builder rule; attempt 2 independently qualified the first frozen-source sequence and promoted the exact three artifact lineage files; final attempt 3 reproduced the exact six-file candidate once but failed when concurrent executors duplicated static/resource/hermetic checks. The accepted limiter source remains frozen at `crates/true-peak-limiter/src/lib.rs` SHA-256 `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`. Candidate Wasm is `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`; promotion commit is `0bb5a820be33a49393386617358ec1db2fe7577d`; preserved predecessor record head is `6e90818e8ff924512752f1df4abbabcd992ea766`.

This successor is a new bounded workflow after #627's hard stop. It does not retry #627. It may use only preserved records to decide whether exact byte identity transfers the accepted scratch qualification to the promoted repository artifact, then execute only delivery checks that never completed in #627. #621 and this issue are the two active slots. Lane B exclusively owns this spec/evidence and the inherited three artifact files. Sol HIGH coordinates documentation, checkpoints, GitHub and delivery; Luna HIGH/XHIGH performs separately authorized evidence work; Astra LOW performs every scope, evidence, exact-head, CI and delivery verification.

## Frozen inheritance and no-repeat boundary

Branch from exact #627 head `6e90818e8ff924512752f1df4abbabcd992ea766`. Preserve #627's complete record and attempt verdicts unchanged. Before any action, Astra LOW must verify the ancestry, clean/pushed state, local/GitHub issue synchronization, two-slot ownership, current main, and these inherited identities:

- first #627 scratch sequence at frozen source `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`, with eleven successful numbered stages, candidate `63ef81c1…`, five delivered non-Wasm hashes, all 26 resource mutations, and Chromium 151.0.7922.34, Firefox 153.0, WebKit 26.5;
- Astra LOW #627 attempt-2 evidence PASS at `40ecd3db63d8b0e6f728217a41a5502287359029`;
- exact three-file promotion `0bb5a820be33a49393386617358ec1db2fe7577d`, byte-identical to the qualified scratch overlay;
- final-attempt builder/preflight/identity records at #627 head `26633c15808bfeff6c76fb09921c22314ed1a84d`, proving one successful ordinary post-pin builder and the same six hashes;
- #627 final FAIL and hard stop at `6e90818e8ff924512752f1df4abbabcd992ea766`.

No artifact builder, Wasm compile, static artifact gate, expected-resource/native-witness gate, hermetic worklet gate, SDK or browser install, browser qualification, benchmark, timing, or compiler capture may run in this successor. The duplicate #627 records receive no new credit and no #627 FAIL is relabeled. Required PR and post-main CI remain delivery events rather than manual retries.

## Smallest closable outcome

After Astra LOW scope PASS, Luna HIGH/XHIGH may create compact documentation-only evidence that mechanically proves:

1. the three repository promotion files equal the first accepted scratch overlay byte-for-byte;
2. the one valid final-attempt post-pin builder output equals the accepted scratch candidate for all six basenames and hashes;
3. every scratch artifact-dependent gate consumed that exact six-file set and the same pin/results/matrix overlay later promoted;
4. the failed duplicate command sets are segregated and unnecessary to this equivalence chain;
5. no source, artifact, result row, version floor, gate vocabulary, resource value, dependency, lock, script, workflow, test, ABI, SDK surface, or #621 file changed after the accepted promotion.

Astra LOW must return evidence PASS before any command. It must reject circular proof, missing source/output linkage, reliance on the non-credit duplicate records, or any claim not established by retained bytes.

After evidence PASS, Luna HIGH/XHIGH may run exactly once only the delivery checks that #627 never completed: SDK package/generated-surface check against the preserved exact six-file output without installation; matrix generator `--check`; `cargo fmt --all --check`; branch diff hygiene; workspace policy; and effect-runtime policy. Preserve compact command/status evidence and stop on the first failure. No other command is authorized.

Astra LOW then performs a no-rerun exact-head/current-main review. Open one PR only after PASS. Require repository `qualification`, verify live main immediately before guarded exact-head merge, verify exact merge parents and post-main qualification, synchronize and close this issue and #621, update #559/#560, and remove every clean delivered/detached #621/#627/successor worktree while retaining branches/history/evidence.

One documentation/evidence attempt and one remaining-check execution are initially authorized only after their preceding Astra LOW PASS. The repository three-attempt rule restarts for this explicitly narrower successor. Do not weaken a gate or disguise any #627 command as new evidence.

## Astra LOW scope review — PASS

Astra LOW passed exact clean pushed brief
`c5d8c553534f841f2c2882508436d055607b4dae` against inherited #627 head
`6e90818e`, live main `30680709`, parent #621, and synchronized tracker
`d31d8c24a2eec5e92729423093a37ef86e6786cf`. Root independently verified live
GitHub #628 title/body/open state, #627 closed-superseded state, and closed
duplicate #629. #621/#628 are the two active slots.

The successor is genuinely narrower and is not a fourth #627 attempt: it
preserves every failure, forbids all repeated artifact work, requires independent
retained-evidence acceptance, and permits only checks never completed afterward.
Luna HIGH/XHIGH may now create the documentation/evidence disposition only. No
build, gate, install, browser, output generation, promotion edit, PR, or merge is
authorized.

## Astra LOW evidence review — PASS; single executor leased

Astra LOW passed exact clean pushed evidence head
`82dbb0d49a3d24cfb8ef057a0c03c8d5c6ba4c94` against live main `30680709`.
The manifest and all five mechanical obligations verify: the 2,596-byte scratch
overlay equals the promotion diff; all three promoted files and six normalized
output identities match; retained commands bind the accepted gates to that
output and overlay; duplicate records remain non-credit; and the post-promotion
path audit preserves every frozen product byte and #627 FAIL.

Root created atomic external execution lease
`/tmp/misofm-engine-issue628-execution.lock/owner.txt` with SHA-256
`086714a9b12f97dde9a9f4818244e7b3bd9b67c8a7fb33a40feed70f9720ecc8`.
It names `/root/issue583_luna_impl`, this accepted evidence head, and only the
SDK, matrix-check, Cargo-format, diff-hygiene, workspace-policy, and effect-
runtime-policy scope. No other executor may run an issue command while that
lease exists. Luna HIGH must verify the lease before each command and stop on
any change or first gate failure.

Exactly one Luna HIGH sequence of those previously unexecuted checks is now
authorized. Builder, static, resource, hermetic, install, browser, artifact-
output, promotion, PR, and merge execution remain forbidden.
