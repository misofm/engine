# Update the environment vocabulary mutation count and deliver RT5 evidence

GitHub: https://github.com/misofm/engine/issues/611

Final test-only delivery successor to stopped #610 under audit lane A #559. #610 correctly added the 20 missing RT5 capture vocabulary rows, bringing the checked population from 114 to 134. Its unchanged mutation suite fails only because the `COUNT` and `COUNT_TR` full-output fault assertions still expect 114. #610's exact ownership excluded that script and allowed no correction, so it is closed with the documentation and failure evidence preserved.

This issue inherits the accepted issue-607 capture, release recipe, vocabulary documentation, source/seal/capture reviews, and lane-B artifact PASS/N/A unchanged. Sol HIGH coordinates checkpoints, verification, GitHub synchronization, and delivery. Luna XHIGH implements one bounded test correction. Every scope, implementation, exact-head, and delivery verification uses Astra LOW. Lane-B #608 is the only other active issue and owns disjoint scalar endpoint test/evidence plus shipped artifact qualification/pinning.

## Exact ownership and outcome

Change only `scripts/test-env-vocabulary.sh` so the two existing full-output fault payloads for `COUNT` and `COUNT_TR` expect the current checked population `134` instead of `114`. Preserve the same fault injection, diagnostic strings, subprocess status assertions, partial-output checks, counter-mutants, and all other expectations byte-for-byte.

Implementation may additionally add focused issue evidence. This numbered spec, focused review records, and #559/#560 handoff status are root-owned.

No vocabulary row, checker, capture script, validator, runner, Rust source, fixture, Cargo file, policy/workflow, runtime/browser/SDK/ABI source, shipped artifact, pin, accepted/raw/seal/disposition/capture manifest, or predecessor evidence may change.

## Gates and delivery

1. Push and synchronize this brief, #559/#560, #608 coordination, and GitHub. Astra LOW must return exact clean scope PASS.
2. Luna XHIGH makes the two literal expectation changes and runs the checker plus complete mutation suite. Root commits and pushes the exact tranche.
3. Astra LOW verifies the two-line delta, causal full-output failure controls, unchanged diagnostics/status assertions, and no scope drift. Run `bash scripts/check-env-vocabulary.sh`, `bash scripts/test-env-vocabulary.sh`, docs/research gates, format/diff, and both inherited capture checksum manifests.
4. Integrate current main if needed, recheck #608 disjointness, obtain Astra LOW exact integrated-head PASS, and retain lane B's inherited artifact PASS/N/A.
5. Open one new PR; keep failed PR #609 closed. Require successful qualification, guarded merge, successful post-main qualification, GitHub issue/handoff synchronization, and clean delivered-worktree removal.

The accepted capture stays anchored to source commit `3b632cdb68105e2023b08ac90b4e2baa99bafa53`, raw/accepted SHA-256 `59257eb092f197b616cbaa20ec713ed8b4e10446c29941e8a1d7d23c96db89ca`, and descriptive results 7,226/7,219 ns per plan render. No build, preflight, prepared executable, runner, capture, retry, resume, or timing is authorized.

One Luna XHIGH implementation pass and no correction are authorized. Any substantive defect stops and splits again. No original open #559 finding may start until this successor and lane B's remaining partials are delivered.

## Astra LOW scope review

Astra LOW returned **PASS** at exact clean pushed brief
`9f3898e24319e4e7e1704d16687d32ac0b10ab09`, with synchronized #559/#560/#608 and disjoint active
#608/#611 ownership. The reviewer confirmed 134 unique vocabulary rows and that changing only the
two `COUNT`/`COUNT_TR` payloads preserves fault injection, diagnostics, statuses, partial-output
checks, and counter-mutants. Luna XHIGH may perform the sole two-literal correction. All inherited
capture bytes and identities remain frozen; no workload execution is authorized.
