## Sol XHIGH verdict: FAIL

Issue #542 attempt 1 is not accepted and must not deliver.

### Reviewed identities

- Exact HEAD: `aecc86d9ca5a4185bebda5c4f7038ce03614e5a7`
- #542 comparison base / merge-base: `3a996760d6aa9826d5b896d2718d0d5a9a2f5ebd`
- Integrated main: `86d5b4bd97999a376123f3be3d6c04d27b8733e1`
- Original #542 source checkpoint: `33eed6752f3a4324f37146e65bb9feff26244481`
- Worktree is clean.
- `git diff --check 3a996760...HEAD` and `86d5b4bd...HEAD` both return 0.
- #542 source files are byte-identical from `33eed675` through HEAD; only the merged lockfile changed afterward.

### Blocking finding

The boundary checker has a real false-green path. [check-conformance-boundaries.sh:100](/home/bl/misofm/engine-protocol-conformance-boundary/scripts/check-conformance-boundaries.sh:100) removes every scan result whose pathname ends in `tests.rs`, for every production crate:

```sh
filtered_uses="$(gate_filter_exclude ... '/tests\.rs:' ...)"
```

It never establishes that the file is reachable only through `#[cfg(test)]`.

The retained reproduction independently supports the defect: removing the guard before `controller`’s `mod tests;` and adding an external `conformance` import to `controller/tests.rs` still produced `conformance boundaries: ok`, checker status 0. The capture correctly marks the reproduction itself failed.

The checked-in modules are currently guarded—e.g. [controller.rs:3934](/home/bl/misofm/engine-protocol-conformance-boundary/crates/protocol/src/controller.rs:3934) and [session_wire.rs:1713](/home/bl/misofm/engine-protocol-conformance-boundary/crates/protocol/src/session_wire.rs:1713)—so this is not a claim that the current production library imports `conformance`. It is a blocking acceptance-gate weakness violating the required structural boundary and “no correlated weakening” condition.

### Otherwise verified

- The 46-frame corpus generator is source-identical to the base after the expected crate import and provenance-path relocation. The exact 11 command, 11 success, 18 non-OK and six event labels/bytes are preserved.
- The sole authoritative Rust FNV pin remains `0xbdeb_b0f8_1c38_ec42` at [protocol_corpus.rs:248](/home/bl/misofm/engine-protocol-conformance-boundary/crates/conformance/src/protocol_corpus.rs:248).
- The 39-edit builder is source-identical to the base and contains exactly 39 ordered `SessionEdit` entries.
- The million-mutation test body is byte-identical from `MUTATION_RUNS` onward: one million iterations, identical seed, scheduling, generator, decoder selection, limit classification and repeatability checks.
- All 80 extracted inline tests remain named and selected: controller 44, message wire 17, session wire 19. Normalized comparisons show only relocation paths, conformance-owner references and rustfmt changes.
- Captures report protocol default 150 and test-support 151 tests; conformance debug/release each total 27, including corpus 3 and mutation 1, with zero ignored or failed.
- Production exports and old fixture/runner paths are absent. Protocol’s normal dependency tree remains only `engine` and `session`; `conformance` is dev-only. The `3a996760...HEAD` lock delta contains only #542’s conformance→protocol/session and flattened protocol dev-dependency record. Separately owned #543 lock edges are already in the base.
- All 28 attempt-1 compressed captures match their raw and packed SHA-256 manifests. Preserved failures include lock status 101, initial protocol compile status 101, initial fmt failure, and three boundary-check corrections before green.
- The Wasm capture causally proves scalar/SIMD builds, exported `main`, exact returned-verdict acceptance, green control, inert-invocation rejection, two digest rejections and corpus-count panic rejection. Still-present correct artifacts were observed read-only as:
  - scalar: 2,734,565 bytes, SHA-256 `c881d6288a6d130c25643a0198ad6e5c0086a70f95a98f5c6fa21365109e5240`
  - simd128: 2,728,061 bytes, SHA-256 `765b42cc97612d79e9da775771485e1cb7b08e1d1b3c425c9109c2fef8bf98b5`

The committed Wasm log does not itself retain those module hashes or print each successful interpreter status/output. Preserve explicit artifact identities in the corrected evidence record; a rebuild is unnecessary if the existing outputs are accepted and retained.

The prior purported review is correctly identified as interrupted Luna XHIGH in [incomplete-luna-evidence.md](/home/bl/misofm/engine-protocol-conformance-boundary/artifacts/issue542-review-routing/incomplete-luna-evidence.md:1). It receives no Sol authority.

### Smallest bounded revision

Change only the boundary checker so test-child exemptions:

- apply only to the exact protocol child files that need the dev dependency; and
- are conditional on verified literal `#[cfg(test)] mod tests;` parent declarations, failing if the guard is absent.

Revalidate with:

1. Normal boundary checker positive run and shell syntax.
2. The reproduced unguarded-controller mutation must return nonzero.
3. The equivalent session-wire guard removal must return nonzero.
4. Existing forbidden production-export counterexample must remain red.
5. Byte-exact restoration followed by a green checker.
6. Normal protocol dependency metadata/tree assertion.
7. `git diff --check` on the complete committed base-to-revised-head diff.

No million-mutation, native suite, or Wasm rebuild is needed for a checker-only correction. Final delivery still requires #543 to complete qualification and deliver first, followed by #542’s current-head qualification and remote synchronization.

No files, Git state, GitHub state, builds, browsers, benchmarks, or agents were mutated or launched during this review.