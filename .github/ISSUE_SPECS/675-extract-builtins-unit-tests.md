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

## Attempt 1 result

Luna HIGH stopped at gate 1 (exact transform proof), status **1**, on 2026-09-09.
The proof's test inventory matcher expected four-space indentation, while the
deindented module correctly has `#[test]` and test functions at column zero.
The pinned formatter command passed with status 0. Gates 2–8 were not run and
no correction or rerun occurred. Evidence is preserved at
`/tmp/issue-builtins-tests-attempt1-evidence`. The two source paths contain the
exact extraction: `lib.rs` SHA-256
`75198efb88ad2c52f31fff37a22135780393a838551933abe697e7646252dfe4` and
`tests.rs` SHA-256
`5c2be93ef98b1d890d34e0900407d30f68dd2bf262afca617c235ae3a0139609`.

## Attempt-1 verdict and attempt-2 brief

Astra LOW returned **ATTEMPT-1 FAIL / SOURCE ASSESSMENT PASS** at exact clean
pushed checkpoint `0742bc172369d40d8a3a5a514bca05d1cbb73053`, authoritative
`origin/main` `acd625d72a57f83f50f26279717464744504b4c4`, and synchronized
tracker `576cded546d5d6dc438f769e6b37fe97ab1b5884`. GitHub parity holds.
The frozen source/prefix/formatted identities, exact extraction, path fence, and
nine ordered tests agree; formatter status is 0. Gate 1 failed because its
new-file matcher requires four leading spaces although correctly extracted tests
begin at column zero. Gates 2-8 are absent. The attempt-1 identities file
correctly distinguishes authoritative `origin/main` from the primary checkout's
stale local `main`. Attempt 1 supplies no gate credit.

Attempt 2 is qualification-only. Preserve every attempt-1 evidence byte and
freeze repository source at:

- `crates/builtins/src/lib.rs` SHA-256
  `75198efb88ad2c52f31fff37a22135780393a838551933abe697e7646252dfe4`;
- `crates/builtins/src/tests.rs` SHA-256
  `5c2be93ef98b1d890d34e0900407d30f68dd2bf262afca617c235ae3a0139609`.

Only this spec may change in Git. Require fresh literal path
`/tmp/issue-builtins-tests-attempt2-evidence` absent including symlinks before
creation. Persist clean HEAD/upstream/authoritative `origin/main`, exact Git
status, frozen source identities, and the attempt-1 directory's deterministic
path/size/SHA-256 snapshot before qualification. Read back and hash each literal
script before execution.

Correct the transform proof so its test-name expression permits any leading
whitespace in both the persisted inline body and formatted extracted file,
requires exactly nine names, and compares their ordered lists. Preserve the
exact original reconstruction, prefix/declaration, deindent, formatted-byte, and
source-hash checks. Format only a fresh reconstructed copy with the pinned
formatter; do not format repository source again.

Forbidden-construct searches must capture `rg` status explicitly: status 1 is
the required no-match result, status 0 is a prohibited match, and status 2 or
higher is a search failure. `! rg` is forbidden because it masks search errors.
Run all eight original gates fresh once in order with direct literal command,
separate-stream, and numeric-status records. Stop without correction or rerun on
any failure. Gate 8 explicitly requires the two frozen source hashes, exact
three-path branch diff, and unchanged attempt-1 snapshot. No prior gate or review
credit carries and no recursive evidence ledger is required.

A clean pushed amendment and fresh Astra LOW attempt-2 scope PASS are required
before sole Luna HIGH `issue675_luna_impl` resumes. Only the spec may change in
Git. No source formatter, production, artifact, AudioWorklet, pin, timing, or
performance work is authorized.

## Attempt 2 authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed feature
`f1c75ad23c8257fdc075fc86d1197c2b868f5bb6`, authoritative `origin/main`
`acd625d72a57f83f50f26279717464744504b4c4`, and synchronized tracker
`6ac76fa2b5efd70caae8e367d5cefb173a0c8fbd`. GitHub parity, both frozen
source hashes, preserved attempt-1 evidence, and fresh attempt-2 path absence
verify. The corrected whitespace-tolerant inventory, explicit `rg` statuses,
exact reconstruction, eight fresh gates, and final source/path/prior-evidence
checks adequately address attempt 1. #672 remains disjoint.

Only Luna HIGH `issue675_luna_impl` may run qualification-only attempt 2.
Preserve source and attempt-1 evidence, format only the reconstructed copy, run
all eight gates once with direct command/stream/status records, and stop on any
failure. Only this spec may change in Git; root owns commits and trackers. No
credit, recursive ledger, source formatter, artifact, AudioWorklet, pin, timing,
or performance work is authorized.

## Attempt 2 result

Luna HIGH qualification passed on 2026-09-09. All eight gates ran once in order
with status **0**. Debug and release `builtins` tests each ran 9 tests with 9
passed and 0 failed; clippy, format check, diff check, workspace policy, and
final census passed. The explicit forbidden searches returned `rg` status 1
(required no-match) for both `lib.rs` and `tests.rs`. Gate 8 confirmed the
frozen source hashes, exact three-path branch census, clean Git status, and
unchanged attempt-1 snapshot. Evidence is preserved at
`/tmp/issue-builtins-tests-attempt2-evidence`; the transform proof hash is
`ada45be34b30cb7ed538565f7f2ac8d567df082c70d8f2fdd911654d723c407f` and the
gate-8 checker hash is
`0a64a67654798fcbb8e4723cd9f156fc7d6f30e7fe2b14fb3ad6b61b8a46d74b`.

## Source verdict and artifact applicability

Astra LOW returned **SOURCE PASS** at exact clean pushed record
`5fccfa8714b93e32f9b04f8299b1dad9dfb7542f`, authoritative `origin/main`
`acd625d72a57f83f50f26279717464744504b4c4`, and synchronized tracker
`104bfdfd0879493c8f67c3bf18ff9341099dec48`. Exact extraction/prefix,
frozen hashes, nine ordered tests, corrected matching, reconstructed-copy-only
formatting, explicit `rg` status handling, and all eight successful direct
command records independently verify. Debug and release each passed 9 tests;
Clippy passed with existing configuration warnings. Final checks enforce both
source hashes, the exact three-path branch scope, and unchanged attempt-1
evidence. Attempt 1 remains failed without carried credit.

Root records **no artifact applicability**. Every relocated implementation byte
remains behind `cfg(test)` and the production input is unchanged, so no native or
Wasm artifact identity receives changed input. No AudioWorklet qualification or
pin action is required or authorized. Exact-head/current-main PR-readiness review
is next.

## PR-readiness verdict

Astra LOW returned **PR-READINESS PASS** at exact clean pushed head
`2bdd8ba51c032387c530fa2e3cec92a2d674c9b1`, live main and merge-base
`acd625d72a57f83f50f26279717464744504b4c4`, and synchronized tracker
`0500846c87c04f1a366e17f3861f28a810237993`. Only this spec changed after
SOURCE PASS. Exactly the three authorized paths differ; branch-wide whitespace,
GitHub parity, attempt accounting, no-artifact applicability, and #672
disjointness pass. One PR may close #675 only; broader RT17 remains open.
Required qualification and a fresh guarded head/current-main review precede
merge.
