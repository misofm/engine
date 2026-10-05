# Decision 15, stream A: verdicts

This directory holds byte-for-byte copies of every stream-A verifier verdict. The work is on
branch `codex/d15-stream-a`, base `8be19c86e`. Each issue's spec in `.github/ISSUE_SPECS/` holds
its Attempt record.

| Issue | Title | Commits on this branch | Attempts | Verdict |
| --- | --- | --- | --- | --- |
| #1300 | Let soft-clip restore its own non-finite history | `c288358b4`; merge `13302fceb`; follow-up (the commit that adds this file) | 1 | PASS ([1300-attempt1.md](1300-attempt1.md)) |

## Open for root/S0

- **#1300 worklet digest.** The merged module (`13302fceb`) is `9275bcce...` (2896157 B). The
  batch head (`43ba3f7c0`) builds `d4cf86ea0fe3523a560a4e2bc0eefd738435c5c7f977b0ae4b47d37cc02cb9fa`
  (2896157 B, reproduced from a second checkout): the comment-only follow-up shifts 11 data-section
  bytes, very probably panic line numbers. It is ARTIFACT CHANGED and not re-pinned
  (`docs/RELEASE.md`).
- **Batch verdict.** [stream-a-batch-verdict.md](stream-a-batch-verdict.md): ready to push.
- **#1300 non-goals still open.** Soft-clip's in-flight ramp current validated by
  `ramp_path_within` (#1278 attempt-1 NIT-2), and the `X`/`e` tightening to `0 or |x| >= FLUSH_EPS`
  (#1071 attempt-1 NIT-1), each need their own issue.
