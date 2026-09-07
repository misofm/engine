# Remove completed local worktrees after merged delivery

Status: OPEN #540; documentation-only brief.

The user requires merged local worktrees to be removed to reclaim space. Add the rule to AGENTS.md delivery control, then apply it to completed worktrees. This is documentation/housekeeping only and independent of active limiter #539.

Scope: AGENTS.md plus this issue and concise cleanup evidence. After a merge, finish required evidence/issue synchronization and verify every local checkpoint is pushed. Remove the clean completed worktree with git worktree remove, preserving branches/history and any evidence needed outside it. Account for work integrated through another PR and completed detached baselines. Keep active worktrees and the primary checkout. Do not discard uncommitted/unpushed work or unique evidence; preserve it first and report any concrete blocker. Record the removed paths and reclaimed space without claiming timing/engine performance.

Luna high implements the concise guide addition; Sol high verifies this noncritical documentation change. Root owns Git/GitHub and removal. Nine completed clean/upstream-preserved worktrees have already been removed, increasing available space by approximately31.8GiB; ignored evidence logs were preserved under /tmp/worktree-cleanup/preserved-ignored. Main/audit-handoff/active limiter remain.

Gates: inspect exact guide diff and cleanup records, confirm actual merged PRs/ancestor reachability and clean/pushed checkpoints, required CI for the documentation PR, then merge/synchronize closure and remove this worktree under the new rule. No audio-path changes, new cleanup framework, benchmark, or history rewrite.

## Sol high brief approval

COMPLETE — Sol HIGH brief approval: **PASS**.

- Remote #540 identity/body matches the local spec and remains OPEN.
- Clean, pushed baseline verified: `ae666af7`; base/current remote `main`: `32a4c205`.
- Evidence adequately records nine removals, preserved ignored evidence, active checkout retention, and a qualified 31.788 GiB available-space delta.
- Scope authorizes Luna HIGH to add exactly one concise `AGENTS.md` delivery-control bullet covering the requested safeguards.
- No new framework, permission ceremony, tests, builds, or unrelated review is warranted.
- No blocker found. Final Sol diff review, required CI, merge/closure synchronization, and removal of this completed worktree remain required.

## Luna high documentation checkpoint

Added one AGENTS.md delivery-control bullet requiring completed merged worktree removal after required synchronization, clean/pushed checkpoints and evidence preservation; active/primary checkouts and branches/history remain. It also covers integrated-through-another-PR work and completed detached baselines. git diff --check passed. No product code, tests or builds changed. Final Sol high review and required CI/merge/closure/self-removal remain pending.
