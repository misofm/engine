# Issue 615 post-pin review

Exact delivery head 5cf6f9f690924115e4d5be7a9b93b553841382c7. One invocation of each command in commands.json; no builder/browser rerun. Full streams and statuses retained. Fresh external CARGO_TARGET_DIR created for this review and reused by its gates. Exclude target/ when preserving evidence.

Initial manual checksum reads for the two issue615 manifests used repository-root cwd and reported missing files; these were non-credit invocation mistakes. Correct directory-relative verification passed all 55 and 11 entries; issue614 root-relative verification passed 13. No evidence bytes changed.

Verdict: FAIL delivery hygiene. git diff --check origin/main...HEAD exited 2 for lines 8 and 11 of both retained final-overlay.diff and second-overlay.diff. Stopped without retries; workspace/effect-runtime policy commands were not run. Commands 00 through 06 all exited zero, including 26 resource mutations and 11 SDK tests. Required correction: losslessly gzip the two captures and refresh their manifest, then review the evidence-only checkpoint; do not rerun successful artifact gates.

## Corrected packaging continuation — PASS

Reviewed clean pushed 911f82a828d767b7e3bd1675dd791aeb85f377f9 against live main/merge-base 9e113be98cf31c1eaf4297b0a031518244b71c33. Both gzip captures decode exactly to original committed bytes (mtime zero); 55-entry manifest passes. GitHub 615/560 bodies synchronize with feature and tracker d7e60beba32ec20bef5b5dfd3120180ea3b5d73a. Only diff-check and previously unexecuted workspace/effect-runtime policies ran; all exited zero. Prior successful qualification remains applicable. Root may preserve evidence and open PR after final checkpoint/current-main confirmation; merge is not authorized.

The initial failed outer-diff stdout is committed as deterministic `07.stdout.gz` because it
contains the literal single-space evidence lines that caused the failure. Compression preserves
those raw bytes while keeping the repository's own outer diff clean.
