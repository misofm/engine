# interchange source-path scan prunes only ./target — npm trees red it

## Approved current scope — 2026-09-11

Fix only the generated-artifact traversal in
scripts/check-effect-interchange-qualification.sh to prune node_modules directories
at any depth, preserving the existing root target exclusion and fail-closed traversal
errors. Extend scripts/test-effect-interchange-policy.sh with root and nested npm
Wasm acceptance and genuine source-path Wasm refusal. Do not exempt arbitrary source
paths or files merely named node_modules. Reuse the existing mutation harness; verify
normal baseline and all existing policy controls. No generic scan framework.

Astra LOW implements; Astra XHIGH independently verifies. Five attempts maximum.
Root commits exact paths at each coherent compiling/focused-green tranche and pushes
promptly before more edits. At most two active issues (#285/#176); isolated worktrees
and no overlapping paths. No timed benchmark, fixture regeneration, compiler captures,
DSP/runtime change, or performance claim. Preserve actual commands/environment/source/
exit/output evidence externally; ordinary compiler feedback is corrected within the
unfinished pass, while substantive failed gates receive bounded adversarial review.
Required exact-head PR and main qualification, upstream evidence and verified GitHub
closure precede clean delivered-worktree removal. Root owns delivery and any artifact
qualification; these tooling-only slices should not require a new shipped pin.

## Historical issue body

check-effect-interchange-qualification.sh's 'generated artifact exists under a source path' scan prunes only ./target, so any node_modules containing a .wasm (playwright-core's webp_codec.wasm) turns it red. Cost two agents a sweep run each (#272, #278). Fix: prune node_modules/ (any depth) in the scan; self-test row proving a planted wasm under src/ still reds.

🤖 Generated with [Claude Code](https://claude.com/claude-code)