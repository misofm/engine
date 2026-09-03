# Retarget the SDK deletion gate to the internal Session JSON authority

## Objective

Restore the SDK deletion gate and its mutation proof after the canonical Session JSON writer moved
from `sdk/src/core/session.ts` to its package-internal module. Preserve every existing deletion rule
and keep the writer package-internal; this issue changes the checker's source-of-truth path, not SDK
product behavior.

## Baseline and cause

The exact baseline is checkpoint `2cf8aa84` on PR #339, based on `origin/main` `51468d5d`. The
terminal #338 correction correctly moved arbitrary-model serialization into
`sdk/src/internal/session-json.ts`, where `ROOT_KEYS` now lives, while public
`SessionBuilder.toJson()` remains the only authoring path. `scripts/check-sdk-deletions.py` still
looks for `ROOT_KEYS` in `sdk/src/core/session.ts` and fails:

```text
FAIL sdk deletions: sdk/src/core/session.ts no longer declares ROOT_KEYS
```

Its `limits` root-key self-test mutation is anchored to the same retired location. The SDK source is
correct; the policy gate's authority path and mutation anchor are stale. This invalidates #338's
terminal qualification claim but does not authorize a fourth #338 attempt.

## Smallest closable slice

Teach the existing checker the internal writer path, read the canonical root-key list there, and
inject the existing `limits` mutation there. Leave all other rules and mutation targets attached to
their current authorities.

### Allowed paths

- `.github/ISSUE_SPECS/341-retarget-the-sdk-deletion-gate-to-the-internal-session-json-authority.md`
- `scripts/check-sdk-deletions.py`

No SDK source, generated file, workflow, package artifact, or other tracked path may change.

### Required checker change

- Declare one path constant for `sdk/src/internal/session-json.ts` alongside the existing SDK source
  constants.
- In the positive root-key assertion, search that file for `ROOT_KEYS` and name that file in any
  missing-authority diagnostic.
- In the existing `a limits root key returns` self-test row, insert `"limits"` after the
  `ROOT_KEYS` anchor in that same internal file.
- Keep emitted-source-row checks and every other `CORE_SESSION`, ABI, boundary, type, error and
  generated-artifact mutation exactly where they are.

### Forbidden scope

- moving, exporting, renaming or changing the internal serializer, `ROOT_KEYS`,
  `SessionBuilder.toJson()`, public barrels, declarations, package exports or runtime code;
- weakening/removing the positive root-key assertion, the `limits` prohibition, comment stripping,
  deleted-name scans, mutation rows, or source-file coverage;
- changing SDK build/package scripts, CI workflows/routing, fuzz files, Session V1 schema/parser,
  generated/browser/Wasm artifacts, or #338 evidence except a later cross-reference; and
- discretionary package/artifact regeneration, benchmark execution or browser requalification.
  The unchanged remote SDK workflow may rebuild and smoke-test its temporary package closure as its
  normal required job; that does not authorize tracked package or product changes in this issue.

## Objective gates

1. `python3 -B scripts/check-sdk-deletions.py` passes on the unchanged SDK source tree and still
   derives `ROOT_KEYS` from the actual serializer authority rather than duplicating the list.
2. `python3 -B scripts/check-sdk-deletions.py --self-test` catches every existing mutation. In
   particular, adding `"limits"` to the internal writer's `ROOT_KEYS` makes the production validator
   fail for the established reason.
3. The captured baseline failure above is the negative proof for the retired `CORE_SESSION`
   lookup. Fresh adversarial inspection confirms that both the positive assertion and the existing
   mutation anchor use the internal authority; no new mutation framework is required.
4. `bash scripts/check-session-policy.sh` passes, so the checker's retained historical/retired-name
   literals do not weaken the sole-session-format boundary.
5. `git diff --check` passes and the checkpoint changes only the two allowed paths.
6. The existing SDK qualification job passes `python3 -B scripts/check-sdk-deletions.py` without a
   source or generated-package change.
7. No discretionary local package/artifact rebuild, benchmark, or browser qualification is run.
   The unchanged remote SDK job may perform its ordinary temporary package build and smoke test.

## Review and delivery

This issue gets one implementation attempt and one fresh Sol-high adversarial review. HOLD and open
a newly bounded issue if the checker exposes another independent defect; do not broaden #341 into a
general SDK or CI cleanup and do not weaken a mutation to obtain green.

Keep the work on `codex/batch-338-canonical-json` and deliver it in PR #339 as a distinct
`fix(#341)` checkpoint. Although #340 and #341 may share the CI-conscious batch/PR, they remain
separate issues because the standalone Rust fuzz workspace and the SDK policy checker are unrelated
tooling boundaries with independently useful outcomes. Commit this exact two-path tranche locally,
run its proportional gates once, and include it in the same coherent PR update as other approved
successors. Do not force-push or manufacture CI commits.

Before implementation, create the matching GitHub issue with this exact title, verify it receives
number 341, synchronize its body with this file, and commit the brief checkpoint. After Sol PASS and
remote green evidence, synchronize and close #341. #338 may cite #341 as resolution of its terminal
SDK-qualification blocker but must not claim a fourth attempt or PASS from this issue.

## Evidence

Sol-high briefing reproduced the stale authority assumption at the checker's positive `ROOT_KEYS`
lookup and at its corresponding mutation anchor. It ruled that retargeting only those two uses is the
smallest correction and that combining this independently failing SDK gate with #340's fuzz lock
would violate the repository's split rules.

## Implementation and Sol-high review evidence

Checkpoint `d22ed0fa` changes only `scripts/check-sdk-deletions.py` (six insertions and three
deletions). It adds `INTERNAL_SESSION_JSON` for `sdk/src/internal/session-json.ts`, reads
`ROOT_KEYS` from that authority with a matching diagnostic, and retargets only the existing
`a limits root key returns` mutation to the same file. The emitted source-row check and every other
`CORE_SESSION`, ABI, boundary, type, error and generated-artifact rule or mutation remain attached
to their prior authorities.

`python3 -B scripts/check-sdk-deletions.py` passes across 46 checked files,
`python3 -B scripts/check-sdk-deletions.py --self-test` catches all 37 existing mutations, and
`bash scripts/check-session-policy.sh` plus `git diff --check` pass. No SDK product source,
workflow, package artifact, fuzz file, benchmark or browser qualification changed or ran.

Fresh Sol-high adversarial review returned PASS for `d22ed0fa`: it independently inspected the
exact commit scope, both moved authority uses, every retained `CORE_SESSION` use and the full
mutation result. Remote closure still requires the unchanged SDK qualification workflow to pass on
the pushed candidate; until then this is local accepted evidence only.
