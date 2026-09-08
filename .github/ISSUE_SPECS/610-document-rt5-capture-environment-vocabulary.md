# Document RT5 capture environment vocabulary and deliver preserved evidence

GitHub: https://github.com/misofm/engine/issues/610

Narrow delivery successor to stopped #607 under audit lane A #559. #607 repaired and independently verified the release build recipe, completed one reviewed preflight, and produced one Astra LOW accepted descriptive capture. PR #609 then failed required qualification because 20 identifiers used by the inherited capture scripts are absent from `docs/ENGINE_ENV_VOCABULARY.md`. #607 allowed no correction and is closed. This successor inherits its pushed stop record, exact accepted capture, and all prior source/seal/capture/artifact-applicability reviews unchanged.

Sol HIGH coordinates documentation, checkpoints, GitHub synchronization, and delivery. Luna XHIGH implements one bounded documentation pass. Every scope, source/documentation, exact-head, and delivery verification uses Astra LOW. Lane-B #608 is the only other active issue and owns disjoint scalar endpoint test/evidence paths plus shipped artifact qualification/pinning.

## Smallest closable outcome and exact ownership

Add exactly the 20 missing entries to `docs/ENGINE_ENV_VOCABULARY.md`:

- `MISO_ENGINE_606_CHILD_PREFIX`
- `MISO_ENGINE_606_EXPECTED_HEAD`
- `MISO_ENGINE_606_MARKER_MODE`
- `MISO_ENGINE_606_PROBE_ROOT`
- `MISO_ENGINE_606_ROOT`
- `MISO_ENGINE_606_SELF_TEST`
- `MISO_ENGINE_606_SELF_TEST_FAULT`
- `MISO_ENGINE_606_STUB_LEDGER`
- `MISO_ENGINE_606_VALID_RECORDS`
- `MISO_ENGINE_CAPTURE_ARGV`
- `MISO_ENGINE_CAPTURE_BINARY_SHA256`
- `MISO_ENGINE_CAPTURE_COMMIT`
- `MISO_ENGINE_CAPTURE_COMPILER`
- `MISO_ENGINE_CAPTURE_CWD`
- `MISO_ENGINE_CAPTURE_FIXTURE_SHA256`
- `MISO_ENGINE_CAPTURE_FLAGS`
- `MISO_ENGINE_CAPTURE_PHASE`
- `MISO_ENGINE_CAPTURE_SOURCE_SHA256`
- `MISO_ENGINE_CAPTURE_TARGET`
- `MISO_ENGINE_CAPTURE_TREE`

Each row must state its real producer/consumer direction and bounded meaning. Distinguish inherited/stub control variables, preflight-to-subprocess capture metadata, and `MISO_ENGINE_CAPTURE_PHASE` as child stdout/stderr marker vocabulary rather than an inherited environment control.

Implementation may change only that vocabulary document and focused issue evidence. This numbered spec, focused review records, and #559/#560 handoff status are root-owned.

No capture script, validator, runner, Rust source, fixture, Cargo file, policy, workflow, runtime/browser/SDK/ABI source, shipped artifact, pin, accepted/raw/seal/disposition/capture manifest, or predecessor evidence may change.

## Gates and delivery

1. Push and synchronize this brief, #559/#560, and GitHub. Astra LOW must return exact clean scope PASS before implementation.
2. Luna XHIGH adds the 20 exact rows in the document's existing schema and style, with no duplicate or broader invented vocabulary. Root commits and pushes the exact tranche.
3. Astra LOW reviews every description against actual source use and confirms no existing row changed semantically. Run `bash scripts/check-env-vocabulary.sh`, `bash scripts/test-env-vocabulary.sh`, the docs/research gates, formatting/diff, and focused checksum verification of the inherited capture evidence. No capture executable or timing path receives credit.
4. Rebase or merge current main if needed, recheck #608 disjointness, and obtain Astra LOW exact integrated-head PASS. Lane B confirms the inherited PASS/N/A artifact disposition remains applicable.
5. Push the branch and open one new PR; PR #609 remains closed with its failed run and old branch identity. Require successful qualification, guarded merge, successful post-main qualification, issue/handoff synchronization and clean delivered-worktree removal.

The accepted capture remains anchored to source commit `3b632cdb68105e2023b08ac90b4e2baa99bafa53`, raw/accepted SHA-256 `59257eb092f197b616cbaa20ec713ed8b4e10446c29941e8a1d7d23c96db89ca`, and results 7,226/7,219 ns per plan render. Those values are descriptive only. No preflight, prepared build, runner, capture, retry, resume, or timing is authorized.

One Luna XHIGH implementation pass and no correction are authorized. A substantive documentation or policy defect stops and splits again. No original open #559 finding may start until this successor and lane B's remaining partials are delivered.

## Astra LOW scope review

Astra LOW returned **PASS** at exact clean pushed brief
`94143e96728ac4bb09ab7cb2c3c35b31d4737e25`, with live #559/#560 synchronization and disjoint
#608 ownership. The reviewer accepted the exact 20-row documentation-only scope and capture
immutability. The implementation must state that `MISO_ENGINE_CAPTURE_PHASE` is emitted on stderr and
parsed by the runner, and that `MISO_ENGINE_CAPTURE_TARGET` is exported by the runner but currently
has no reader. Luna XHIGH may perform the sole documentation pass. No preflight, build, runner,
capture, retry, resume, or timing is authorized.

## Attempt 1 Astra LOW verdict

Luna XHIGH added exactly 20 accurate rows and the checker reports 134 unique names. Docs/research,
diff, and inherited capture checksum gates pass. The unchanged mutation suite fails because
`scripts/test-env-vocabulary.sh:222-223` hard-code the previous count 114 for the `COUNT` and
`COUNT_TR` full-output fault assertions. Astra LOW returned **PASS to stop / FAIL to correct within
#610**: the rows are correct, but the test script is outside this issue's ownership and the sole pass
has no correction allowance. Preserve the documentation and candid failure; a two-expectation
successor inherits all accepted source/capture evidence and authorizes no workload execution.
