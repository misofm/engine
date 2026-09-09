# Extract builtins unit tests into their module file

Parent: #559 (lane A, RT17). Coordination: #560. Base:
`acd625d72a57f83f50f26279717464744504b4c4`.

## Problem and slice

`crates/builtins/src/lib.rs` ends with one large inline
`#[cfg(test)] mod tests` containing 9 existing tests. This issue moves only
that module body into `crates/builtins/src/tests.rs` and replaces the wrapper
with `#[cfg(test)] mod tests;`. Preserve `builtins::tests`, `super::*`
access, imports, helpers, test names/order, assertions, literals, and behavior.
The earlier test-only helper items, `corpus.rs`, production implementation,
the combined builtins implementation-module concern, the exhausted #674
graph-root extraction, and broader RT17 remain outside this slice.

## Exact ownership

This issue owns only:

- this numbered spec;
- `crates/builtins/src/lib.rs`;
- new `crates/builtins/src/tests.rs`;
- concise #559/#560 rows written serially by root.

It owns no production behavior, public API, dependency, allocation, arithmetic,
scheduling, policy, workflow, lockfile, benchmark, artifact, AudioWorklet, pin,
timing, or performance change. It must not touch active #672, failed #674, or any
other retained worktree, branch, history, or evidence.

## Frozen transform

At base:

- `lib.rs`: 208,611 bytes, SHA-256
  `fc5cee40fd082b4b7526697af18f5d57e2bf43acd0a9f0945f176e04fe03ad15`;
- unique final marker offset: 181,906;
- byte-identical prefix SHA-256:
  `e589814c8152e5c1c590ba68bb1d7e4cfacc07fca92feeae4df84639dec4b082`;
- exact wrapper deindent: 23,958 bytes, SHA-256
  `c29ff01c7ad23d92d7831e39b7e2f7914e45d6792b4b8dc7dc8ab817c8908e0b`;
- Rust 1.97.1/rustfmt 1.9.0 with the repository Rust 2024 config: 23,942
  bytes, SHA-256
  `5c2be93ef98b1d890d34e0900407d30f68dd2bf262afca617c235ae3a0139609`;
- ordered explicit test count: 9, from
  `meter_metric_subsets_match_full_and_peak_only_omits_work` through
  `eligibility_sequence_uses_whole_call_fallback_then_fuses_the_next_call`.

Before editing, require the literal evidence directory
`/tmp/issue-builtins-tests-attempt1-evidence` absent including symlinks.
Persist clean HEAD/upstream/main, exact Git status, original source, hashes,
marker, and ordered test inventory there before changing source. Freeze and read
back/hash the transform proof script before execution.

Perform one exact wrapper removal/deindent, then run
`cargo +1.97.1 fmt --all` once as the declared transformation. Capture its
separate streams and numeric status and stop on failure. Reject any resulting
change outside the two owned source paths before gates.

The transform proof reconstructs the unformatted deindent from the persisted
original, requires its frozen hash, formats a separate reconstruction once with
`rustup run 1.97.1 rustfmt --edition 2024 --config-path
/home/bl/misofm/engine-builtins-tests/rustfmt.toml`, and compares that result to
the new file and frozen formatted hash. It independently requires current
`lib.rs` to equal the frozen prefix plus exactly
`#[cfg(test)]\nmod tests;\n`, preserves the ordered 9 tests, and rejects
`include_str!`, `include_bytes!`, `file!`, `line!`, and `#[path` in
both changed source files.

## Gates and evidence

After Astra LOW scope PASS, one Luna HIGH executor performs the source tranche
and runs these gates once in order, stopping on the first failure:

1. exact transform proof;
2. `cargo test --locked -p builtins --lib`;
3. `cargo test --locked --release -p builtins --lib`;
4. `cargo clippy --locked -p builtins --all-targets -- -D warnings`;
5. `cargo fmt --all -- --check`;
6. `git diff --check`;
7. `bash scripts/check-workspace-policy.sh`;
8. exact changed-path census and frozen final source hashes.

Persist each literal command, separate stdout/stderr, and numeric status in the
evidence directory. Every wrapper is fail-closed with explicit nonzero exits;
no failed assertion may be masked. Do not build a recursive evidence ledger:
the source commit, per-gate records, and Astra's independent reproduction are
the evidence appropriate to this test-only file move. No correction or rerun
occurs within an attempt. Root checkpoints immediately; three failed attempts
hard-stop without a renamed retry.

## Review and delivery

Astra LOW reviews scope before implementation and independently checks the
checkpoint, transform, test identity, commands, statuses, and exact paths.
Root records no artifact applicability because all relocated code remains behind
`cfg(test)` and non-test inputs are unchanged. Lane B alone owns #672 and all
AudioWorklet qualification/pins.

Delivery requires exact-head/current-main PR-readiness review, required PR
qualification, fresh guarded merge review, ordered-parent verification,
post-main qualification, GitHub/tracker synchronization, and removal only of a
clean delivered worktree. Failed/stopped worktrees and evidence stay preserved.

## Attempt 1 authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed feature
`e02bbb729c1f8cb80506129ef71e6335d26df20c`, current main
`acd625d72a57f83f50f26279717464744504b4c4`, and synchronized tracker
`6c4578629ba165b86ab4f3756dea136e41bcaafd`. GitHub parity, #672/#675
disjointness, the unique marker/EOF, base/prefix/deindent identities, nine ordered
tests, forbidden-construct absence, formatted hash, and fresh evidence path all
independently reproduce. This is a separate builtins slice, not a #674 retry.

Only Luna HIGH `issue675_luna_impl` may perform attempt 1: persist clean pre-edit
evidence, make the exact extraction, capture the pinned formatting transformation,
enforce the two-source-path fence, and run all eight gates once with explicit
failure propagation. The direct per-command records are the proportionate
evidence; no recursive ledger is required. Stop on any failure and return the
coherent tranche to root for checkpoint. Root owns commits and trackers. No
production, artifact, AudioWorklet, pin, timing, performance, or unrelated path
change is authorized.
