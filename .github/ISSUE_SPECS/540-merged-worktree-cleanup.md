# Remove completed local worktrees after merged delivery

Status: OPEN #540; documentation-only brief.

The user requires merged local worktrees to be removed to reclaim space. Add the rule to AGENTS.md delivery control, then apply it to completed worktrees. This is documentation/housekeeping only and independent of active limiter #539.

Scope: AGENTS.md plus this issue and concise cleanup evidence. After a merge, finish required evidence/issue synchronization and verify every local checkpoint is pushed. Remove the clean completed worktree with git worktree remove, preserving branches/history and any evidence needed outside it. Account for work integrated through another PR and completed detached baselines. Keep active worktrees and the primary checkout. Do not discard uncommitted/unpushed work or unique evidence; preserve it first and report any concrete blocker. Record the removed paths and reclaimed space without claiming timing/engine performance.

Luna high implements the concise guide addition; Sol high verifies this noncritical documentation change. Root owns Git/GitHub and removal. Nine completed clean/upstream-preserved worktrees have already been removed, increasing available space by approximately31.8GiB; ignored evidence logs were preserved under /tmp/worktree-cleanup/preserved-ignored. Main/audit-handoff/active limiter remain.

Gates: inspect exact guide diff and cleanup records, confirm actual merged PRs/ancestor reachability and clean/pushed checkpoints, required CI for the documentation PR, then merge/synchronize closure and remove this worktree under the new rule. No audio-path changes, new cleanup framework, benchmark, or history rewrite.
