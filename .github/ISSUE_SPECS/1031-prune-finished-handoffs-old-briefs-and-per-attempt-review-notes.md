# Prune finished handoffs, old briefs and per-attempt review notes

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 6. No code ruling
is needed. These are history that git keeps. One light call is yours: whether `BRIEFS/` goes.

## Context

**`docs/handoffs/` is 3.1 MB in 12 folders, mostly `.patch` files and raw timings.**

| folder | bytes | status |
|---|---:|---|
| `effects-2026-09-27` | 1,972,687 | all 24 draft issues filed (#976-980, #981-986, #988-992, #1003-1007, #1013, #1014) |
| `804-account-switch-2026-09-14` | 208,162 | session history |
| `silence-2026-09-27` | 188,203 | **keep:** none of its 13 drafts (A0, S1-S12) is filed yet |
| `plumbing-floor-2026-09-26` | 168,365 | history |
| `dual-mono-2026-09-27` | 139,954 | **keep `DUAL-MONO.md`:** its "Owner ruling" is in force. Drafts 01-06 are filed as #970-975; drafts 07-08 are ruling notes |
| `meters-2026-09-26` | 134,833 | history |
| `bug-966-2026-09-27` | 127,831 | history (#966 closed) |
| `builtins-less-removal-2026-09-27` | 58,330 | **keep `SCOPE.md`:** specs #956-#964 cite it |
| `gain-pan-2026-09-26` | 54,276 | history |
| `copy-removal-cycle-2026-09-25` | 11,495 | history |
| `overnight-2026-09-27` | 5,572 | the latest owner answers; keep until superseded |
| `dead-code-2026-09-28` | — | this audit |

**The other items:**

- `.github/ISSUE_SPECS/BRIEFS/`: 77 files, 622 KB of old design briefs. The only reader is one line
  of `scripts/session-policy-historical-allowlist.txt`. `ISSUE_SPECS/README.md` is still titled
  "Engine V2".
- `docs/issue880-*.md`: 12 notes, 61 KB, for closed #880. They hold 19 of the 23 live-doc links
  into `artifacts/`.
- `docs/audits/`: 44 per-attempt review and evidence notes for #539-#822, 109 KB. Many cover the
  endpoints that `02-…` deletes and the capture that `04c-…` deletes.
  `docs/audits/issue-triage-2026-09-14.md` is a separate note; it stays unless you say otherwise.
  - `docs/audits/test-usefulness-2026-09-04/` (228 KB) is a separate ledger that is still a live
    proposal. It stays.
- `docs/research/legacy-v2old/`: 13 files (11 numbered notes, `PROVENANCE.md` and `sources.json`),
  32 KB, copied from the legacy engine's research docs for #144. AGENTS.md: "never … inherit an architecture from a legacy engine source".

**Links that must be re-pointed:** three test notes link into `docs/handoffs/`
(`crates/graph/tests/MUTATIONS.md`, `crates/graph-compiler/tests/MUTATIONS.md` and
`crates/true-peak-limiter/tests/MUTATIONS.md`).

## Smallest closable slice

1. Delete the handoff folders marked "history" and "all filed". Keep `silence-2026-09-27/`,
   `dual-mono-2026-09-27/DUAL-MONO.md`, `builtins-less-removal-2026-09-27/SCOPE.md`,
   `overnight-2026-09-27/` and this audit's folder.
2. Delete `docs/issue880-*.md`, the `docs/audits/<issue>-*.md` per-attempt notes, and
   `docs/research/legacy-v2old/`.
3. If you agree, delete `.github/ISSUE_SPECS/BRIEFS/` and its allowlist line. Retitle
   `ISSUE_SPECS/README.md`.
4. Re-point the three `MUTATIONS.md` links, and any other live-doc link to a deleted file, to a
   commit permalink at the base commit.

## Objective gates

1. **Docs gates:** `bash scripts/check-dsp-research.sh`, `bash scripts/check-session-policy.sh`,
   `bash scripts/test-session-policy.sh` and `python3 -B scripts/check-step-vocabulary.py` pass.
2. **Links:** a grep of live docs (`docs/` outside `docs/handoffs/`, `AGENTS.md`, every
   `README.md` and `MUTATIONS.md`) for the deleted paths finds nothing.
3. **Build, console digests and artifact:** unaffected, because only `.md` and patch files change.
   Show `git diff --stat`.
4. **CI routing:** the change stays on the `evidence` route if it touches only `docs/` and
   `.github/ISSUE_SPECS/`. Editing `MUTATIONS.md` under `crates/` routes to `full`, so batch this
   with `07-…`. `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
5. **No live claim lost:** no test, check or workflow reads a deleted file (same grep, over
   `crates hosts tools scripts sdk fuzz .github/workflows`).

## Dependencies

`02-…` and `04c-…`, so the notes about deleted code go after the code. Batch it with `07-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-prune-handoffs`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F13.

1. **Do not delete `.github/ISSUE_SPECS/BRIEFS/` wholesale. Some briefs are normative DSP
   sources, not history.** "The only reader is one allowlist line" counts machine readers only:
   - `BRIEFS/019-antialiased-saturator-clipper.md` is the frozen source of the soft-clip graph and
     the 63-tap half-band coefficients, copied "character for character"
     (`crates/soft-clip/src/lib.rs:3`, `src/kernel.rs:3`, `tests/polyphase_identity.rs:5`, `:20`;
     `crates/lane/src/kernels/halfband.rs:3`, `:58`);
   - `BRIEFS/016-true-peak-limiter.md` is the source of `crates/dsp-reference/src/true_peak_limiter.rs:57`;
   - `BRIEFS/013-compressor.md` is named the compressor's authority in `docs/README.md:19` and is
     cited throughout `crates/compressor/tests/MUTATIONS.md`.

   Failure scenario: step 3 lands, and the citation chain AGENTS.md requires for every effect
   ("primary/official citations", frozen coefficients) points at deleted files; a reviewer checking a
   coefficient table has no in-tree source. Step 3 becomes: keep every brief cited from `crates/`,
   `docs/README.md` or a `MUTATIONS.md` (at least 013, 016 and 019); delete the rest only if the
   owner agrees; retitle `ISSUE_SPECS/README.md`.
2. **Gate 2's link grep must include `crates/**` source comments**, not only live docs, because
   the BRIEFS citations above are in `.rs` files.
3. Everything else in this draft is unaffected by the mobile scope correction.
4. **Several "history" files are cited by open issues or live code** (verified against issue bodies
   and the tree):
   - `effects-2026-09-27/`: open #988, #989, #991 and #992 cite `LIMITER-DIAGNOSIS.md` and
     `limiter-diagnosis-prototypes.patch`;
   - `plumbing-floor-2026-09-26/DIAGNOSIS-2.md` is cited by open #938;
   - `builtins-less-removal-2026-09-27/VERIFY.md` is cited by open #965 (the draft keeps only
     `SCOPE.md` there);
   - `docs/issue880-mb1.md` is the recorded provenance of committed coefficients
     (`crates/math/src/lane_math.rs:42`, `:85`), and `docs/issue880-mb2.md` records the tolerance
     amendment behind `crates/transient-shaper/tests/oracle.rs:30`;
   - `docs/research/legacy-v2old/02-numerics-determinism.md` is cited by `crates/lane/src/fpenv.rs:30`;
   - `docs/issue905-astra-review.md:7` links `docs/issue880-class-b-astra-review.md`.

   Keep each of these until its citing issue closes, or re-point the citation to a commit permalink
   in the same change. Re-pointing the `lane` and `math` comments routes CI to `full` and turns on
   `math_closure`.
5. **Widen gate 2 and gate 5** to `crates`, `hosts`, `tools` source comments and to the bodies of
   open GitHub issues.

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1031-prune-handoffs` from `c69736c1` (`codex/batch-slim-2`).
Checkpoint `441722c2`. Nothing pushed; no issue edited.

**Owner's light call.** `.github/ISSUE_SPECS/BRIEFS/` is kept whole (R10: "keep `BRIEFS/`"), with
its `session-policy-historical-allowlist.txt` line. `ISSUE_SPECS/README.md` is retitled "Engine issue
specifications", and its "Shared definition" no longer calls the engine "V2".

**Keep rule.** A file stays if an open issue (local spec or GitHub body), a ruling, a test, a
script or source code cites it by path or by name, or if a live proposal uses it as evidence. Each
research family (diagnosis, verification, patches, timings, drafts) is kept or removed as a unit.
A citation that is only history is re-pointed to a permalink.

### Removed: 114 files, 1,569,237 bytes

| what | files | bytes | why nothing is lost |
|---|---:|---:|---|
| `docs/handoffs/804-account-switch-2026-09-14/` | 18 | 208,162 | only closed specs 804 and 807 cite it |
| `docs/handoffs/bug-966-2026-09-27/` | 7 | 127,831 | #966 closed; three citations re-pointed |
| `docs/handoffs/copy-removal-cycle-2026-09-25/` | 1 | 11,495 | nothing cites it |
| `docs/handoffs/gain-pan-2026-09-26/` | 3 | 54,276 | closed specs only; two citations re-pointed |
| `docs/handoffs/meters-2026-09-26/` | 6 | 134,833 | closed specs 943 and 950 only |
| `effects-2026-09-27/`: the limiter-2 family (`LIMITER-DIAGNOSIS-2.md`, `VERIFY-LIMITER-2.md`, the `limiter-diagnosis-2-*` and `verify-limiter-2-*` files, `issues/limiter-2-{1,2,3}-*.md`) | 10 | 829,798 | only closed #1013 and #1014 cite it, and #1014 absorbed the drafts |
| `docs/issue880-*.md` | 12 | 61,447 | four citations re-pointed |
| `docs/audits/<issue>-*.md` (#539-#822) | 44 | 109,091 | only closed specs, and 580's note (also removed), cite them |
| `docs/research/legacy-v2old/` | 13 | 32,304 | one citation re-pointed |

### Kept, and why

- **`BRIEFS/`**: owner ruling. 013, 016 and 019 are normative DSP sources cited from `crates/`
  and `docs/README.md`.
- **Handoff folders kept whole:**
  - the current sprint's `dead-code-2026-09-28`, `test-value-2026-09-28`,
    `live-control-2026-09-28`, `control-smoothing-defaults`, `dual-mono-2026-09-27` and
    `overnight-2026-09-27`;
  - `silence-2026-09-27` (its drafts are unfiled);
  - `builtins-less-removal-2026-09-27`: open #961 cites `SCOPE.md` and open #965 cites `VERIFY.md`;
  - `plumbing-floor-2026-09-26`: open #938 cites `DIAGNOSIS-2.md` and `DIAGNOSIS-2-VERIFY.md`, and
    unfiled silence draft S10 cites `SILENCE-MASKS-VERIFY.md`. `DIAGNOSIS-2` corrects `PLAN.md`, and
    each note cites its own patch, so the folder is one unit.
- **`effects-2026-09-27`, four of five families kept.** **New finding:** the Amendments' grep
  counted path citations only. About 30 name-only citations in code, tests and scripts reach three
  more families.
  - **limiter-1:** open #988, #989, #991 and #992 cite `LIMITER-DIAGNOSIS.md` and its patch. Open
    #1039 cites `limiter-diagnosis-wasm-console.mjs.txt`. `VERIFY-LIMITER.md` is the verification of
    #988-#992, and `true-peak-limiter/tests/MUTATIONS.md:383` cites it.
  - **EQ:** "VERIFY-EQ finding 1/2" is cited in `parametric-eq/src/lib.rs` (3) and in
    `parametric-eq/tests/bank.rs` (4).
  - **Automation:** "VERIFY-AUTOMATION A1-A6/F1-F6" is cited in:
    - `tools/console-workload` (`src/mixing_automation.rs` and 4 tests);
    - `scripts/console-benchmark-record-lib.jq` and `scripts/test-console-benchmark.sh`;
    - `parametric-eq/src/lib.rs`, `effect-contract/tests/paired_spans.rs` and
      `compressor/tests/MUTATIONS.md:340`.
  - **Compressor:** `compressor/src/kernel.rs:2000` cites VERIFY-COMPRESSOR, and `LIMITER-DIAGNOSIS.md:374`
    cites `COMPRESSOR-DIAGNOSIS.md`.
- **Other kept files:**
  - `docs/audits/issue-triage-2026-09-14.md` and `docs/audits/test-usefulness-2026-09-04/`, as the
    body says;
  - `docs/rulings/` and every doc that AGENTS.md names;
  - `docs/issue904-*` and `docs/issue905-*`, which are outside this scope.

### Re-pointed citations (12 edits in 10 files)

Every citation now points at `https://github.com/misofm/engine/blob/5379e46ca3b349b9d277d642c008bb7a9643fb76/<path>`.
That commit is `origin/main`, the last upstream commit with these files. They are byte-identical
there and at `c69736c1`, which is not upstream yet. Each citation is marked "removed by #1031".

| citing file | removed target |
|---|---|
| `crates/graph/tests/MUTATIONS.md:473` | `gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch` |
| `crates/graph-compiler/tests/MUTATIONS.md:288` | `bug-966-2026-09-27/VERIFY-966.md` |
| `crates/math/src/lane_math.rs:42`, `:85` | `docs/issue880-mb1.md` |
| `crates/transient-shaper/tests/oracle.rs:30` | `docs/issue880-mb2.md` |
| `crates/lane/src/fpenv.rs:30` | `docs/research/legacy-v2old/02-numerics-determinism.md` |
| `docs/issue905-astra-review.md:7` | `docs/issue880-class-b-astra-review.md` |
| `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md:382` | `gain-pan-2026-09-26/GAIN-PAN-VERIFY.md` |
| `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md:428`, `:431` | `bug-966-2026-09-27/README.md#L13-L19`, `wasm-pins-harness.patch` |
| `docs/handoffs/test-value-2026-09-28/issues/12-…md:28`, `:65` | the same two |

`true-peak-limiter/tests/MUTATIONS.md` needs no change, because `VERIFY-LIMITER.md` stays.

### Gates

1. **Docs gates: PASS.**
   - The spec's own gates: `check-dsp-research.sh`, `check-session-policy.sh`,
     `test-session-policy.sh` and `check-step-vocabulary.py`, plus `--self-test`.
   - CI's "docs and research evidence gates": `test-dsp-research.sh` and
     `check-builtins-listening.sh`.
   - `check-script-reachability.py` and `test-script-reachability.py`.
   - Every policy script in the lint job, each **PASS**:
     - workspace, env-vocabulary, bench, test-support-ci, host-core, protocol-control, realtime,
       realtime-audit-leak, artifact-evidence-leak and lane (check and test);
     - `check-unfused-seal.sh` and its `--self-test`;
     - rack, builtins, graph, effect-runtime policy and effect-runtime fixtures (check and test);
     - native-pcm-runner and its two policy tests;
     - conformance-boundaries (check and test);
     - `check-parametric-eq-render-contract.sh`, `check-release-shape.py --self-test` and
       `check-sdk-deletions.py`.
2. **Links: PASS.** A scratch gate listed 236 tokens: every removed path, every distinctive
   basename and stem (for example `VERIFY-966`), and the removed folder names. It searched:
   - `docs/` outside `docs/handoffs/`, `AGENTS.md`, every `README.md` and every `MUTATIONS.md`;
   - `crates`, `hosts`, `tools`, `scripts`, `sdk`, `fuzz` and `.github/workflows`;
   - the 107 open GitHub issue bodies.

   Markdown links to a commit permalink are stripped first. The only hits are #1031's own body.
   #1027 and #1030 name `docs/issue880-*` only as this issue's scope.
3. **Build, digests and artifact: PASS.** `git diff --stat c69736c1 HEAD` gives 124 files changed,
   31 insertions and 25,802 deletions. Outside the deletions, only `.md` files and comment lines in
   `lane_math.rs`, `fpenv.rs` and `oracle.rs` change.
   - `rustfmt --check` passes.
   - `RUSTDOCFLAGS='-D warnings' cargo doc -p math -p lane` passes, and so does `math` with
     `--document-private-items`.
   - `cargo clippy -p math -p lane -p transient-shaper --all-targets --all-features -D warnings`
     passes.
   - `cargo test -p transient-shaper --test oracle` passes 2 tests, and
     `cargo test -p session --test json_contract_artifacts` passes 3. The session test is the one
     that reads `docs/`.
   - `build-web-audioworklet.sh` builds a byte-identical module at `c69736c1` and at this commit
     (`6c952a2c…`). Both fail the committed pin (`f7bd75ca…`) the same way, so the pin mismatch was
     already there at the batch base and is not caused by this change. The batch repins at its
     boundary, or #1061 does.
4. **CI routing:** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   `ci-path-router.py --base c69736c1 --head HEAD` gives `route=full` and `math_closure=true`
   (from the `crates/` comments). The batch already routes the same way (`5379e46c..c69736c1`:
   `full`, `math_closure=true`), so this issue adds no CI.
5. **No live claim lost: PASS.** No test, check or workflow reads a removed file. Gate 2 covers
   `crates hosts tools scripts sdk fuzz .github/workflows` and the open issue bodies.

**Found outside scope, not changed:**
- Open #881-#899 (16 issues) cite `docs/audits/render-path-cost-audit-2026-09-24.md`. It exists
  only on the unmerged branch `codex/render-path-cost-audit` (`54804bc1`).
- Open #560 cites `docs/audits/524-merge-base-addendum.md`, which was removed before this issue.
