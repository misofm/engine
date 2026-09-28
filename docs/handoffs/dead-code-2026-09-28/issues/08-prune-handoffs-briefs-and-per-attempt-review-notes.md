# Prune finished handoffs, old briefs and per-attempt review notes

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
- `docs/issue880-*.md`: 14 notes, 70 KB, for closed #880. They hold 17 of the 23 live-doc links
  into `artifacts/`.
- `docs/audits/`: 44 per-attempt review and evidence notes for #539-#822, 117 KB. Many cover the
  endpoints that `02-…` deletes and the capture that `04c-…` deletes.
  - `docs/audits/test-usefulness-2026-09-04/` (228 KB) is a separate ledger that is still a live
    proposal. It stays.
- `docs/research/legacy-v2old/`: 7 files, 32 KB, copied from the legacy engine's research docs for
  #144. AGENTS.md: "never … inherit an architecture from a legacy engine source".

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
