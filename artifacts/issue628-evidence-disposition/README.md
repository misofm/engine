# Issue 628 evidence disposition

This is a documentation-only disposition. It uses retained #627 bytes and Git
metadata; it runs no build, gate, install, browser, output, promotion, or source
operation. The mechanical results are in `disposition.json`.

The accepted scratch overlay is byte-identical to the three-file promotion diff:
the decompressed `artifacts/issue627-scratch-qualification/final-overlay.diff.gz`
and `git diff 0bb5a820^ 0bb5a820` have the same 2,596 bytes and SHA-256.
The accepted scratch six-file record and final-attempt `02-output.sha256` have
the same six basenames and hashes. The numbered scratch `commands.json` records
all eleven successful stages from the frozen scratch checkout; artifact-facing
commands use `/tmp/issue627-qualified-output`, and `final-proof.json` records
the exact promoted overlay.

The attempt-2 disposition explicitly marks the duplicate builder records
non-credit and proves they use distinct checkout/output paths. The final
attempt-3 failure record separately marks duplicate static/resource/hermetic
executions as procedural FAIL evidence. Neither duplicate set supplies credit.

The post-promotion Git path audit finds only issue-spec and artifact-evidence
paths after `0bb5a820`; no source, product artifact, result/resource, dependency,
lock, script, workflow, test, ABI, SDK, or #621 path changed. #627's attempt
verdicts remain preserved and are not relabeled.
