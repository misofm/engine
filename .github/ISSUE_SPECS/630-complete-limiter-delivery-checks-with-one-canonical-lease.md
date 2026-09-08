# Complete limiter delivery checks with one canonical lease

GitHub: https://github.com/misofm/engine/issues/630

Parent: #621 (lane A, FX2). Exhausted predecessors: #627 and #628. Coordination: #559/#560.

#627 qualified candidate Wasm `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1` from frozen limiter source SHA-256 `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`, promoted exactly three pin/lineage files at commit `0bb5a820be33a49393386617358ec1db2fe7577d`, and then exhausted its attempts through executor duplication. #628 proved the retained byte-equivalence chain at `82dbb0d49a3d24cfb8ef057a0c03c8d5c6ba4c94`, ran one successful `npm ci --ignore-scripts` in its isolated successor worktree, and exhausted its attempts before any delivery check ran. Its hard-stop record is `feb8063e`; GitHub #627 and #628 remain closed as superseded without delivery. Their failures receive no new credit or relabeling.

This successor owns only agreement on one canonical execution lease, read-only installed-state/output preflight, and the six checks that neither predecessor completed. #621 and this issue are the two active slots. Lane B retains exclusive artifact/pin qualification ownership. Sol HIGH coordinates documentation, checkpoints, GitHub and delivery; Luna HIGH/XHIGH executes the separately authorized sequence; Astra LOW performs every scope, evidence, exact-head, CI and delivery verification.

## Frozen inheritance and prohibitions

Branch from exact #628 hard-stop head `feb8063e`. Reuse the installed `sdk/node_modules` already present in that isolated worktree, the preserved exact six-file output `/tmp/issue627-postpin-attempt3-output-20260908`, #627 qualification records, and #628 evidence. Do not copy or reconstruct a fictitious pre-install record.

No `npm install`, `npm ci`, dependency mutation, artifact builder, Wasm compile, static artifact gate, expected-resource/native-witness gate, hermetic worklet gate, browser install/qualification, benchmark, timing, compiler capture, product edit, promotion edit, or predecessor command retry is permitted. Required PR and post-main qualification are delivery events.

## Smallest closable outcome

Before execution, Astra LOW must verify exact clean pushed ancestry, current main, local/GitHub synchronization, two-slot ownership, predecessor hard stops, inherited identities, retained installed dependency presence, and this boundary. It must return scope PASS.

After PASS, root creates one atomic external lease with exactly these five newline-terminated fields and no alternate representation:

```
issue=ISSUE_NUMBER
attempt=1
owner=EXECUTOR_CANONICAL_NAME
authorized_head=ASTRA_REVIEWED_HEAD
scope=installed-state-preflight,sdk-check,matrix-check,cargo-fmt,diff-hygiene,workspace-policy,effect-runtime-policy
```

The decision record records its path and SHA-256 before execution. Luna reads that file as the sole authority and validates the five parsed fields against this template; it must not compare against another spelling or add/remove fields. No other executor may run while the lease exists.

Luna then performs one read-only repository-root preflight. It verifies exact HEAD/lease; `sdk/package.json` and `sdk/package-lock.json` identities at install-authorized #628 commit `522c614d` and current HEAD; installed `.package-lock.json` consistency while allowing platform-inapplicable optional packages; installed package name/version agreement and required SDK executables; clean tracked state; and the preserved six-file hashes. A preflight failure stops the attempt without repair or installation.

Only after preflight PASS, Luna runs exactly once and in order, stopping on first failure:

1. `bash scripts/sdk-package.sh check /tmp/issue627-postpin-attempt3-output-20260908`;
2. `node hosts/host-web/qualification/generate-matrix.mjs --check`;
3. `cargo fmt --all --check`;
4. branch diff hygiene against current main;
5. `bash scripts/check-workspace-policy.sh`;
6. `bash scripts/check-effect-runtime-policy.sh`.

Preserve compact commands, actual statuses, streams, identities and stop state. Root checkpoints the exact-path evidence before further work. Astra LOW performs a no-rerun final evidence and exact-head/current-main review. Only after PASS may root open one PR, require repository `qualification`, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize and close this issue and #621, update #559/#560, and remove clean delivered predecessor/successor worktrees while preserving branches and upstream evidence.

Attempt 1 alone is initially authorized after Astra LOW scope PASS. A failure receives no retry until Astra LOW records its evidence and explicitly briefs any lawful next attempt. No gate may be weakened.

## Astra LOW scope review — PASS

Astra LOW passed exact clean pushed head
`6b59637a70340105f5c11b9a6a335e4aed1bd129` against live main `30680709`.
Local and GitHub #621/#627/#628/#630 records synchronize; #627/#628 are closed
without delivery and #621/#630 are the two active slots. The predecessor records,
accepted source/promotion/output and retained install remain frozen.

Root created the canonical five-field sole-executor lease at
`/tmp/misofm-engine-issue630-attempt1-execution.lock/owner.txt`, SHA-256
`dc84cbe3e36e8200643671d77cae6d0191b92da4fd5790f4515c0d6dfb661cc6`.
It names only `/root/issue610_luna_xhigh`, this reviewed head and the exact scope
spelling in the brief. Luna reads and parses this file as the sole authority.

Luna XHIGH may now perform the read-only installed-state/output preflight and,
only after PASS, the six named delivery checks once in order. It stops on first
failure. No installation, predecessor qualification work, product edit, PR or
merge is authorized before Astra LOW reviews the resulting evidence.

## Attempt 1 evidence review — procedural FAIL

The sole executor completed the preflight and all six checks once; their retained
status records are zero, the SDK suite reports 11/11, and the evidence checkpoint
is `ddfcf86d75b365df05a4e58e2a8985a4dcd74784`. Astra LOW verified every retained
manifest and summary hash without rerunning a command. The attempt nevertheless
fails because the lease authorized reviewed head `6b59637a` while the executor
ran from its documentation-only child `4c729144`. The preflight proved that
parent relationship instead of enforcing `HEAD == authorized_head`.

Preserve this procedural FAIL and the earlier registry-based no-execution
observation in chronological order. Do not relabel the attempt, edit its records,
or rerun any preflight, SDK, matrix, formatting, diff, workspace, or effect-runtime
check. Attempt 2 owns only a compact documentation/evidence disposition from
existing Git objects and attempt-1 records. It must identify the exact paths and
bytes changed from `6b59637a` through execution head `4c729144`, prove that no
product, dependency, lock, script, workflow, promotion, result, resource, ABI,
SDK surface, source, or retained six-file output input changed, and state the
limited technical applicability of the already-retained zero-status results.
It must not claim attempt-1 PASS or execute a product gate.

Luna HIGH may prepare that disposition as one exact-path evidence tranche. Root
must checkpoint and push it immediately. Astra LOW then performs a no-rerun
adversarial review and decides whether the unchanged-input proof is sufficient
for delivery. No PR, merge, issue closure, or further command is authorized
before that review.
