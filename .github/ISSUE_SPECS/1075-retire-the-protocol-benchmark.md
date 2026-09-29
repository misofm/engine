# Retire the protocol benchmark (R9: only real host paths are benchmarked)

Owner ruling R9 (`docs/rulings/engine-footprint-2026-09-28.md`): only real host paths are benchmarked, namely the native console `--step` rows and the V8 rows on the shipped artifact. The protocol benchmark is neither. #1062's verifier also found its runner dead: `scripts/run-protocol-benchmark.sh` needs a scalar wasm artifact that #1062 retired, and nothing has built its inputs since before this sprint; CI only runs `scripts/test-protocol-benchmark.sh` against bad arguments.

## Smallest closable slice

Remove the `bench protocol` subject (`tools/bench/src/protocol.rs`, about 1,660 lines), its FlatBuffers comparison schema (`tools/bench/protocol_benchmark.fbs`) and the `flatbuffers` dependency it alone needs, `scripts/run-protocol-benchmark.sh`, `scripts/test-protocol-benchmark.sh`, the two `protocol-benchmark*-validator.jq` files, and the CI step that runs the test script. Update `check-bench-policy.sh`'s `timed_subjects` ratchet honestly, the bench unsafe-owner list if `tools/bench/src/protocol.rs` is on it, and #1027's reachability lint fixtures that name these scripts.

Keep: `crates/protocol` (the C ABI's command path), its fuzz targets and `run-protocol-fuzz.sh`, `check-protocol-wasm-parity.sh`, the protocol allocation audit, and `check-protocol-control-policy.sh`.

## Gates

1. `cargo check --workspace --all-targets --all-features`; clippy and fmt; `bench` tests; console digests unchanged.
2. `check-bench-policy.sh` and its suite, the reachability lint and its suite, the routing tests, every policy script.
3. `cargo tree` shows `flatbuffers` gone from the workspace, or names what still needs it.

## Attempt 1 evidence

Terra, attempt 1, on `codex/1075-retire-protocol-benchmark` from `e35318d7`; implementation commit
`9b5c3c34`.

**Proof it was unused before deleting.** Workflows, scripts, docs, the console `--step` vocabulary
(`scripts/operator/{preflight,run}-console-benchmark.sh`, the console validators),
`check-bench-policy.sh` and #1043's `SELF_TEST_INPUTS` were searched for `protocol-benchmark`,
`protocol_benchmark`, `bench protocol`, `issue005_btlv`, `flatbuffers` and `tools/bench/src/protocol.rs`.
The only live callers were the audit-native CI step (the negative-argument test), the runner and the
test themselves, and gates or fixtures naming the file. No trigger list names these scripts. Nothing
builds `bench` for wasm32: `check-protocol-wasm-parity.sh` builds `conformance`, not `bench`. The
four `MISO_ENGINE_BENCH_WASM_*` names were read only by `protocol.rs` and its runner.

**Removed** (77 lines added, 2,018 removed, net -1,941): `tools/bench/src/protocol.rs` (1,661),
`protocol_benchmark.fbs`, `tools/bench/CORPUS_MANIFEST.md` (the manifest of that corpus alone),
`run-protocol-benchmark.sh`, `test-protocol-benchmark.sh`, both `protocol-benchmark*-validator.jq`
files, `scripts/fixtures/protocol-benchmark-validator-record.json`, and the audit-native step.
`bench` drops `flatbuffers`, `protocol` and `session`. Its wasm32 `main` ran only this subject, so
`bench` is now a native tool and its target-specific dependency table is folded into
`[dependencies]`.

**Gates changed with the removal:**
- `check-bench-policy.sh`: the unsafe owners go from four to three. `timed_subjects` is unchanged
  because the protocol subject timed with its own `Instant::now` and never joined it; the comment
  says so. The suite's cases write into `effect_contract.rs`, and its counts go from 4 to 3. This
  also fixes the stale comment #1039 left.
- `check-realtime-policy.sh` drops the file from its unsafe exclusions. The fixture loses it, and
  a new row, `unsafe-in-deleted-bench-protocol`, refuses unsafe code at the old path. The row goes
  red with the old exclusion restored.
- `check-conformance-boundaries.sh`: the bench dependency union loses `flatbuffers`, `protocol`
  and `session`, and its fixture does too.
- `docs/ENGINE_ENV_VOCABULARY.md` loses the four `MISO_ENGINE_BENCH_WASM_*` rows, which
  `check-env-vocabulary.sh` would otherwise refuse as unused.
- `test-script-reachability.py`: the jq `include` case wrote nothing and relied on
  `protocol-benchmark-record-validator.jq`, the tree's last library reached only by include. It now
  writes its own library. Three cases cover it: an include from a reached `.jq` passes, removing
  that include is refused, and an include in a jq program inline in a reached shell script passes.
  With jq module handling disabled in the checker, the first case goes red. With it limited to
  `.jq` files, the inline case goes red.
- `docs/REALTIME_DEPENDENCY_POLICY.md` and `docs/CONTROL_PROTOCOL_CONFORMANCE.md` record the
  retirement. Historical audits, handoffs and closed specs are untouched.

**Gate 3: `flatbuffers` leaves the tree.** `cargo tree --workspace -i flatbuffers` and
`--target all -i flatbuffers` both answer `package ID specification 'flatbuffers' did not match any
packages`. On the base, the only path was `flatbuffers v25.12.19 <- bench`. `Cargo.lock` drops
`flatbuffers` and its only-user dependency `rustc_version`; `bitflags` and `semver` stay for other
users.

**Gate 1.**
- `cargo check --locked --workspace --all-targets --all-features`, `cargo clippy` (same flags,
  `-D warnings`) and `cargo fmt --all -- --check` pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` passes, as does the release check
  under `CARGO_PROFILE_RELEASE_PANIC=unwind`.
- `cargo test -p bench` passes 13 of 13. The audit-native `cargo test --release -p audit -p bench
  -p console-workload` passes 112 with 0 failed. These ran in a fresh target dir, because a target
  shared with the base tree reuses binaries whose baked `CARGO_MANIFEST_DIR` points at the base.
- `bench protocol --rounds 2` now exits 2 with `usage: bench <console|effect-contract>`.
  `check-effect-contract.sh`, `check-builtins-fixtures.sh` and `check-console-fixtures.sh` pass on
  the release binaries.
- `check-protocol-wasm-parity.sh` still passes (simd128).
- The console `digests` test's 17 digests are byte-identical to the base.

**`cargo test --workspace --all-targets --all-features -- --list` diff (base -> change).** 2,212
tests become 2,207. The only differences are these five removals, all in `bench`:
`protocol::tests::{actual_flatbuffer_builder_and_bounded_verifier_agree,
btlv_sources_encode_and_decode_without_schema_escapes,
frozen_corpus_has_required_cardinality_and_checksum,
jsonl_schema_record_carries_frozen_corpus_and_environment_fields,
metadata_command_projection_maps_empty_and_failures_to_unknown}`.

**Gate 2** is green, running every Python script with `python3 -B`:
- workspace, session, bench, host-core, protocol-control, realtime, realtime-audit-leak,
  artifact-evidence-leak, lane, rack, builtins, graph, effect-runtime (policy and fixtures),
  conformance-boundaries, env-vocabulary and dsp-research, each gate with its suite;
- `test-realtime-trace-validator.sh`, `test-builtins-fixtures.sh`, `check-bench-preconditions.sh`,
  `check-console-benchmark-fixture.sh` and `test-console-benchmark.sh`;
- the unfused seal and its self-test, the parametric-EQ render-contract seal,
  `check-builtins-listening.sh` and both listening validators' `--self-test`, and
  `test-gate-lib.sh`;
- the test-support CI check and its suite, script reachability and its suite (20 cases), CI path
  routing and its suite, and `check-session-map-shape.py`;
- the release shape, run for real and with `--self-test`;
- the SDK deletions, the command-kind and command-reason vocabularies, the parameter metadata, the
  ABI layout, the AudioWorklet call graph and the scalar-oracle-absent gate, run with `--self-test`
  (the deletions and both vocabularies also run for real);
- `test-npm-publish-modes.py`, and actionlint 1.7.7 on `qualification.yml`, `nightly.yml` and
  `fuzz.yml`.

The router gives this diff the `full` route. Because `qualification.yml` changed, it selects every
self-test suite (env-vocabulary, conformance-boundaries, console-benchmark, sdk-deletions,
dsp-research), and all of them are among the green runs above.

**Not run.** `check-cross-targets.sh`, and the browser, SDK and AudioWorklet artifact gates: no
input to them changed, and `bench` is in none of them. The full `cargo test --workspace` was not
run either; only `tools/bench` changed among Rust sources, its tests and the audit-native set ran,
and every other test list is identical.

**Found, not changed (outside the slice).** Two `bench-support` helpers now have no caller outside
their own unit tests: `Metadata::nonempty_or_unknown` (#590) and `sysinfo::parse_cpu_model` (#593).
Both were added for the session, conformance and protocol subjects that #1039 and this issue
retired. They are candidates for a small follow-up. The provenance note in
`tools/bench-support/src/alloc.rs:10` ("the protocol audit/bench thread-local counter") is
historical and was left as it is.

## Sol verdict, attempt 1

**PASS.** Nothing live was retired, no product claim was lost, the ratchets shrank honestly, and
every gate is green on the merge.

**Merge.** `3c4f1827` merges cleanly into `codex/batch-slim-4` at `3a2782f1` (scratch merge
`a08c6779`). #1050, #1047 and #1044 were already in this branch's base `e35318d7`. The only batch
commit since then is #1046 (and a docs commit), and it touches nothing here. The one semantic
difference is that `bench` has 12 unit tests on the merge, not 13, because #1046 removed one.

**Gates on the merge, all green.**
- `cargo check --locked --workspace --all-targets --all-features`, clippy with `-D warnings`, and
  fmt.
- `RUSTDOCFLAGS='-D warnings' cargo doc`, and the release check under
  `CARGO_PROFILE_RELEASE_PANIC=unwind`.
- `cargo test -p bench -p protocol`: bench 12, protocol 126 + 1 + 3.
- The audit-native release tests for `audit`, `bench` and `console-workload`: 107 passed, 0 failed.
- `check-cross-targets.sh`, including the AArch64 rows, the #1018 expected failures, and the wasm
  simd128 and refusal rows.
- `check-protocol-wasm-parity.sh`, `run-protocol-allocation-audit.sh` (64 edits and 10,000 records
  in 40 batches) and `check-effect-contract.sh`, on release binaries.
- 57 lint-job, self-test and suite commands, every Python one with `python3 -B`. They include the
  bench, realtime, protocol-control and conformance-boundary gates with their suites, the
  reachability lint (20 cases), CI routing and its suite, the env vocabulary and
  `test-env-vocabulary.sh`, the release shape, and `test-gate-lib.sh`.
- The 17 console `digests` are byte-identical to `3a2782f1`.
- The full `cargo test --workspace --all-targets --all-features -- --list` goes from 2,171 to
  2,166. Exactly the five `bench` `protocol::tests::*` are gone.
- `flatbuffers` is absent from `cargo tree --target all`, and `Cargo.lock` drops only
  `flatbuffers` and `rustc_version`.

**Checked adversarially.**
- **Nothing live was retired.** No file under `crates/`, `hosts/` or `fuzz/` changed. `fuzz.yml`
  (`run-protocol-fuzz.sh`), the nightly deep fuzz, `check-protocol-wasm-parity.sh`, the allocation
  audit and the protocol-control gate with its suite are all still wired.
- **Every removed name was searched.** The removed script, subject, schema, validator and env
  names were grepped across workflows, scripts, the router's `SELF_TEST_INPUTS`,
  `check-bench-policy.sh`, the operator scripts and docs. Every remaining hit is a historical spec
  or handoff, or a comment recording the retirement.
- **Nothing still builds `bench` for wasm32**, so folding its dependency table is safe.
- **The ratchets are honest.** The base `protocol.rs` timed with its own `Instant::now` and never
  called `timing::timed`, so `timed_subjects` is rightly unchanged. It opened with
  `#![allow(unsafe_code)]`, so the unsafe owners go from 4 to 3, and the suite's count cases go
  from 4 to 3 to match.
- **The new guards discriminate.** Reintroducing an unsafe `tools/bench/src/protocol.rs` fails
  both `check-bench-policy.sh` and `check-realtime-policy.sh`. Restoring the old realtime
  exclusion turns `unsafe-in-deleted-bench-protocol` red. Disabling the checker's jq `include`
  handling, or limiting it to `.jq` files, turns the reachability suite red.
- **The five removed tests guarded no surviving product claim.**
  - The corpus checksum, the FlatBuffers builder/verifier and the JSONL record tests pinned only
    the retired benchmark's own corpus and output.
  - The BTLV encode/decode test survives as:
    - `typed_frame.rs` `full_{command,success_response,non_ok,event}_frames_*`;
    - the `message_wire/tests.rs` b2a/b2b/b3/b4 goldens;
    - `queue.rs::ten_thousand_events_fit_as_exactly_forty_atomic_batches`;
    - the CI-run allocation audit.
  - The metadata-command test survives as
    `bench-support::sysinfo::tests::command_output_distinguishes_success_and_unavailable_results`.

**Findings, ranked (none blocks).**
1. **Low: a false statement in `docs/CONTROL_PROTOCOL_CONFORMANCE.md`.** It now says the
   comparison runner "never ran its descriptive comparison". Issue 005's record says the final
   authorized workload did run: its raw JSONL, SHA-256 `630fab07…`, was accepted as the final
   descriptive evidence. The earlier text had the same error ("has not run"). This change restated
   it rather than fixing it.
2. **Low: a contradiction in `docs/REALTIME_DEPENDENCY_POLICY.md`.** It still says the checker
   "currently accepts unsafe syntax in exactly four source files", one of them now "(retired by
   #1075)". It also still calls "the latter two" the Issue-005 exceptions. The next paragraph
   already disowns that sentence.
3. **Info: two orphaned helpers.** `Metadata::nonempty_or_unknown` and `sysinfo::parse_cpu_model`
   are confirmed to have no caller outside their own tests. This is the follow-up the implementer
   already named.
