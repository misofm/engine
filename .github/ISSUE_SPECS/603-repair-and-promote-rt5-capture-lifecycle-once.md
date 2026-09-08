# Repair and promote the RT5 capture lifecycle once

GitHub: https://github.com/misofm/engine/issues/603

Bounded tooling successor to stopped #602 under audit lane A #559. Issue #600 delivered the entirely
untimed sustained W8 input-trim workload. Issue #602 produced a reusable narrow timing seam at
`68950adf1a1f2458431f64d9a5fc84766e182d45`, but Astra LOW rejected its runner after the sole bounded
correction. No #602 final preflight, real capture entry, or timer ran. This issue repairs only the
rejected lifecycle, seal, and validator boundary, then owns the sole retained descriptive capture.

Sol HIGH coordinates documentation, exact-path checkpoints, capture authorization, GitHub sync, and
delivery. Luna HIGH/XHIGH implements. Every scope, source/harness, seal, capture, and exact-head
verification uses Astra LOW. Lane B #598 is the only other active issue and retains exclusive shipped
artifact qualification/pinning authority. This native tools-only issue must not alter or pin the
AudioWorklet artifact.

## Smallest closable outcome

Repair the five blockers recorded in `docs/audits/602-attempt1-review.md`, prove them through isolated
stub-only tests, and promote the frozen #602 timed entry through one protected preflight and exactly one
workload-process invocation. Preserve #600's `input-symmetry` command as entirely untimed and preserve
the #602 Rust timing seam unchanged unless Astra finds a source correctness defect that requires an
explicit scope amendment.

The capture contract remains two independent owners, 512 untimed preparation renders per owner, then
two measured rounds of 4,096 renders per owner: 8,192 timed plan renders per round and 16,384 total.
Only `PreparedRenderPlan::render` is inside each timing observation. Publication, buffer construction,
PCM hashing, bookkeeping, and record assembly remain outside. A valid capture passes regardless of
magnitude and makes no baseline, speedup, isolated-cycle, floor, budget, or 64-track claim.

## Exact ownership

Implementation may change only:

- `scripts/input-symmetry-capture-validator.py`;
- `scripts/preflight-input-symmetry-capture.sh`;
- `scripts/run-input-symmetry-capture.sh`;
- `scripts/test-input-symmetry-capture.sh`;
- this spec, focused `docs/audits/` records, and
  `artifacts/issue-603-input-symmetry-capture/` qualification/seal/raw/accepted evidence;
- concise #559/#560 handoff status under root ownership.

The successor may read the #602 scripts and evidence but publishes all new protected artifacts below
the issue-603 root. It may not change runtime crates, Rust benchmark sources, fixtures, manifests,
`Cargo.lock`, existing captures, policies, workflows, browser/SDK/ABI sources, shipped artifacts, or
artifact pins. If source review proves a Rust correction necessary, stop and amend this issue before
editing it.

## Repair contract

1. The validator retains exact JSON key/type/identity checks and adds finite u64 bounds for elapsed
   fields, `0 < nonzero_samples <= output_words`, unique sorted `missing_metadata`, and record/seal
   agreement. The seal freezes the exact target pair, smoothing sample count, and owner count in
   addition to all #602 identities and counts.
2. Preflight rejects every inherited build-affecting Cargo/Rust setting, including target, target-dir,
   release-profile, compiler, and compiler-wrapper overrides. The fixed release profile and target
   features must be both effective and sealed. Candidate commit/tree, Cargo inputs, toolchain, binary,
   fixture, scripts, and source hashes remain exact.
3. Every no-clobber publication operation explicitly checks failure and exits nonzero. A failed seal,
   accepted-record, or disposition publication may never fall through to READY or PASS.
4. Runner PASS requires exactly one `capture_started`, then exactly one `round_1_complete`, then exactly
   one `round_2_complete`, in that order with no duplicate lifecycle marker. It also requires child and
   validator success, accepted publication, two strict JSON records, and matching sealed identities.
   Marker validation occurs before accepted publication. Timed-call accounting reports 0, 8,192, or
   16,384 only from the longest exact ordered completed prefix and PASS requires 16,384.
5. Prelaunch failure reports zero process launches. Once the prepared binary is launched, capture
   authority is consumed even if the child, validation, publication, or disposition step fails. Raw
   stdout/stderr and all available validator/recovery bytes are retained. The runner never retries or
   resumes.

## Stub-only qualification

The self-test uses only temporary executable stubs and a temporary artifact root. It must assert the
exact child path and argv in a connected invocation ledger, prove the repository's prepared/final
namespace stays absent, and exercise:

- bad validator, preflight, and runner arguments;
- valid seal and two records plus duplicate JSON keys, unknown/missing keys, boolean/integer/float and
  range errors, nonfinite values, missing/duplicate/extra/reordered rounds, duplicate missing metadata,
  impossible nonzero count, wrong backend, and every record/seal identity mismatch;
- each build-affecting environment override refusing before any build or seal publication;
- each sealed file/candidate/argv/cwd/toolchain/build identity refusing before child launch, with the
  fixture otherwise valid so the named mismatch is the discriminator;
- exact successful marker sequence and missing, duplicate, extra, and reordered start/round markers;
- child failure at each observable lifecycle prefix, raw stdout/stderr retention, validator failure,
  accepted publication failure, disposition persistence failure, and overwrite/dangling-path refusal;
- explicit nonzero status for every failure, exact honest disposition fields, and one successful
  transactional publication case.

Tests may compile the timed entry and test injected arithmetic without invoking its command. They may
not run final preflight, a real timing helper, the real `input-symmetry-capture` entry, or any timed
workload. Preserve a direct restored mutation showing a central new gate fails with a specific
diagnostic, and a repository-root checksum manifest for the focused evidence.

## One-shot qualification and capture

After Astra LOW source/harness PASS, root runs final preflight once with the exact approved clean head.
Preflight builds one release binary in the protected issue-603 prepared directory, fully validates the
prospective seal, and publishes the only final seal without overwrite. It does not launch the workload.
Astra LOW then verifies the published seal, binary, environment contract, and exact candidate identity.

Only after that seal PASS may root run `bash scripts/run-input-symmetry-capture.sh` exactly once. The
runner launches the sealed prepared binary once with sealed cwd/argv and no retry. Any post-launch
failure preserves evidence, consumes authority, blocks capture acceptance, and moves repair/promotion
to another numbered issue. Astra LOW validates raw and accepted bytes, strict schemas, lifecycle,
disposition, hashes, frozen output digests, two finite timing records, and honest accounting. Capture
PASS depends on validity rather than magnitude.

## Workflow and gates

1. Push this stateless brief, synchronize GitHub #603, and obtain Astra LOW exact-head scope PASS.
2. Luna HIGH/XHIGH implements one coherent pass. Root commits and pushes each green exact-path
   checkpoint. Astra LOW adversarially reviews the exact pushed source/harness. One bounded runner
   correction and three total implementation attempts are the hard limits.
3. Before preflight, pass the existing #600 debug/release qualification unchanged; capture arithmetic
   unit tests without clock observation; the complete stub lifecycle matrix; the direct mutation;
   strict bench Clippy/rustdoc; fmt/diff; workspace/realtime/builtins/graph/lane/bench policies; and the
   unchanged #238/#496 debug/release tests. `Cargo.lock` must remain unchanged.
4. Root performs the protected preflight only after source PASS; Astra LOW verifies the seal; root
   performs the sole capture only after seal PASS; Astra LOW verifies the capture.
5. Integrate current main, recheck #598 exact-path disjointness, obtain Astra LOW exact integrated-head
   PASS, push, run required PR qualification, merge, require successful post-main qualification,
   synchronize and close #603, mark RT5 delivered in #559/#560, verify remote states, and remove clean
   delivered/stopped worktrees. Lane B supplies artifact qualification/pinning applicability.

No original open #559 finding may start until this final lane-A partial and lane B's remaining partials
are delivered.

## Astra LOW scope review

Astra LOW returned **PASS** on exact pushed brief
`ea23e9e547d4c00fc89d8c057a2dfb37a9e1b40d` after the required coordination correction. Pushed
handoff head `55a197e981f2180f38af15b8f3bb5b6759e49d83` and the matching GitHub #559/#560 bodies record
#602 stopped, #598/#603 as the active slots, RT5 partial, lane-B artifact authority, and
completed-prefix semantics for partial marker-derived counts. The worktree was clean, #603 was open
and synchronized, and its paths were disjoint from #598. Luna HIGH/XHIGH implementation may begin.
Final preflight and capture remain unauthorized pending their separate Astra LOW reviews.

## Attempt 1 adversarial verdict

Astra LOW returned **FAIL** on exact pushed attempt-1 head
`475ac5c249a85ab7e72074adf4010f5b5be9c399`. The Rust timing seam remains frozen and unexecuted,
but the validator rejects its issue-602 records; preflight rejects committed qualification evidence
and retains a seal-publication fallthrough; build isolation/tests are incomplete; copied identity
and marker fixtures are masked by unrelated failures; and missing/reordered record plus 8,192-call
prefix cases are absent. `docs/audits/603-attempt1-review.md` records the exact evidence.

One bounded correction is authorized solely within the four scripts and focused issue-603
qualification evidence. Final preflight, the real capture entry, and timing remain unauthorized. If
a substantive runner defect remains after this correction, stop and split repair/promotion again.

## Bounded-correction verdict

The sole correction is preserved at `3a7fa0c6edb516b722949b0238de56b5a45ecaf9`. Astra LOW returned
**FAIL**: build isolation still omits build-affecting variables; the accepted-publication fault
bypasses the production link; failure exit statuses are not asserted; required duplicate-round,
nonfinite, and record/seal identity mutations remain absent; and the claimed failed-publication
scratch retention is contradicted by the EXIT trap. The detailed review is
`docs/audits/603-correction-review.md`.

#603 stops without preflight or capture. Its reusable corrections remain pushed, while remaining
repair and promotion move to another numbered successor.
