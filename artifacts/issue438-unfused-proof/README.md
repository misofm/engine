# Unfused seal registered-status proof

#438 replaces the rejected registry-count fallback from #411 with one disposable production-local occurrence-status mutation. The failed producer's real `mul_add(` payload reaches the ordinary count operation. Astra independently verified original exit 9, mutant exit 0, and the same focused unexpected-success assertion exit 97. The real production checker is unchanged by #438; all 62 self-test cases remain. The three original failed #411 reviews and their decisive phase/counter evidence are preserved as history.

The integrated candidate `67ac8993e087ab936b5d314b5d0aa68744095be8` completed `cargo test --locked --workspace`, including doctests, with exit 0: 274 result blocks, 1,569 passed, zero failed, 24 ignored. This matches the retained RT-3 baseline at `artifacts/issue420-qualification/engine-420-candidate-workspace.log`; no Rust or Cargo test population changed. Relevant actual policies and syntax/diff checks also passed. The manifest preserves original log paths, byte lengths and SHA-256 hashes; historical verdicts do not describe the final candidate as failed.

This evidence does not replace final exact-head PR review or required CI. No benchmark, target rebuild, artifact publication or runtime arithmetic change belongs to this shell-only delivery.
