# 509: Use the existing SHA256 text authority throughout the audit package

Ready-to-number queued CP20 maintenance after #506 delivery. Inspected engine-bench-hex-authority4b29296d. This is one coherent five-helper package outcome, not five PRs and not complete resolution of repository-wide CP20. Root must integrate delivered #506, verify unchanged audit inputs and synchronize the numbered issue before fresh Luna1. No implementation or execution is authorized by this draft.

## Exact implementation

Allowed source paths, and no others:
- tools/audit/src/vectorization.rs
- tools/audit/src/source_fixture.rs
- tools/audit/src/fixture_builtins_listening.rs
- tools/audit/src/builtins_fixture_check.rs
- tools/audit/src/fixture_builtins.rs

Each contains one one-shot SHA256-to-lowercase-text helper: sha256 at336/578/502/5155 respectively, and source_fixture::sha256_hex at1161. All hash precisely their supplied byte slice once, with no domain prefix or extra update. Replace the five bodies with imports of existing bench_support::digest::sha256_hex, aliased as sha256 where necessary; delete the redundant local definitions. Keep every call site and argument unchanged. tools/audit already depends on bench-support, so no Cargo/lock/API/shared-helper change is needed.

Preserve raw/incremental tree hashing, length/path separators, float/word encoding, manifest/JSON record bytes, error ordering and ownership. Remove sha2 or fmt::Write imports only when that particular module has no remaining use: fixture_builtins and builtins_fixture_check retain raw tree-hash Sha256 work and other formatting. Source fixture seek transcripts and other raw hash consumers must not be accidentally removed. No parser/validator change, new encoder, abstraction, benchmark framework, production dependency or allocation-performance claim.

## Actual static and seal constraints

check-bench-policy.sh scans audit under the existing shared allocator/escaper/percentile/digest rules; its private hash prohibition targets hand-written SHA tables, not these one-shot delegations. Keep the shared timing-subject roster unchanged. The targeted source-path/seal search found no frozen checksum over these five Rust files requiring a repin for an import change. This does not authorize overriding a concrete newly discovered source gate.

fixture_builtins' existing source-shape check scans from fn check_fixture_root to its cfg(test) module and forbids authoring reachability. Deleting the unrelated sha256 body must preserve those anchors and the existing author/checker separation. builtins_fixture_check has an explicit source inspection rejecting author entrypoint reachability; retain it. Fixture manifests/checksums under fixtures/builtins/v1, tools/audit/fixtures/builtins-audit-v1, fixtures/sources/v1, listening records and all historical benchmark scripts/validators/pins remain byte-identical. The normal check-builtins-fixtures.sh invokes a real audit checker and broader corpus; do not run it merely to verify this five-alias change or refresh fixtures. Any required CI checker remains enabled and authoritative.

## Finite independent proof and commands

Add `shared_sha256_alias_matches_published_literals` to each of the five existing cfg(test) modules. Each calls that module's actual imported name, comparing b"" and b"abc" directly to fixed independent published SHA256 lowercase strings:
empty e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855;
abc ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.
Do not assemble expected output with a formatter or call another helper for expected values. The module aliases preserve all existing consumers by source inspection; no new mutation campaign or corpus is needed.

Run these five commands in debug and with --release, retaining independent argv/cwd/source/stdout/stderr/numeric status and the indicated nonempty populations:
1. `cargo test --locked -p audit --bin audit shared_sha256_alias_matches_published_literals` — exactly five new tests.
2. `cargo test --locked -p audit --bin audit vectorization::tests::` — five existing parser/kernel-shape rejection tests plus the new alias test; no external disassembler/vectorizer launch.
3. `cargo test --locked -p audit --bin audit builtins_fixture_check::tests::` — two existing checks plus new alias test. The existing checker reads accepted small audit fixtures, checks no modification, copies a bounded scratch payload and rejects its corruption; it does not author or repin the checked-in corpus.
4. `cargo test --locked -p audit --bin audit fixture_builtins::tests::owned_jsonl_parsers_reject_duplicate_extra_reordered_and_wrong_variants -- --exact` — one literal parser contract test, no generation.
5. `cargo test --locked -p audit --bin audit fixture_builtins_listening::tests::level_match_attenuates_only_and_caps_peak -- --exact` — one small in-memory level-match test, not a listening-packet generator or listening evidence claim.

The source_fixture module's existing generated_fixtures_match_manifest_oracles_and_mutation_policy invokes generated_fixtures, decode and frozen seek schedules. Preserve its source but do not expand this local qualification into that generator matrix; its new alias literal is the focused hash-consumer gate. Likewise do not invoke fixture/listening author commands, --write, repin paths, response sweeps, timed workloads or full audit CLI. Only named test entrypoints above execute locally.

Run `cargo clippy --locked -p audit --bin audit --all-targets -- -D warnings`, `cargo fmt --all -- --check`, source/spec `git diff --check`, `bash scripts/check-bench-policy.sh` and `bash scripts/check-realtime-audit-leak.sh`, retaining individual statuses. These existing static checks may use offline metadata but must not launch the audit runner/timed workloads. Shared bench-support implementation and its accepted #506 proof remain unchanged; no redundant new support matrix is needed.

## Delivery and failure boundary

Root owns exact-path checkpoints; fresh Luna1 receives one coherent source verdict, Sol2/3 only after consolidated failure, then hardstop/rescope. A concrete static seal or unrelated inherited lint failure must be recorded and ruled on, not repaired by broadening these five files or repinning history. No benchmark/preflight/capture authority is spent. Future tool-source/binary identities naturally change and do not inherit a historical prepared authority.

After source acceptance, compare production/worklet input identity; this tooling-only slice does not demand an unconditional artifact/browser rebuild. Required CI and actual-PR exact-head Astra review remain necessary before closure. Existing #496 runtime and queued #478 priorities are unaffected. CP20 stays PARTIAL for encoders outside this audit package.

## Numbered current-base checkpoint

GitHub #509 matches this title/body. Base is delivered main0fc4e959 after #506/PR507 with its closure record; audit package inputs are unchanged from Astra scoped4b29296d. Root retains Git/GitHub/checkpoints. Fresh Luna attempt1 remains pending numbered actual-base Astra approval. No timing or generation authority is granted.

## Astra numbered PASS and current activation

# Astra #509 numbered actual-base review — PASS

Reviewed clean exact head f8b37be672abd8e0a27b4fc42af62d0aaca626fd in engine-audit-hash-authority, based on delivered main0fc4e959 after PR507/#506. Only #506 closure documentation and #509 spec differ from main. Audit/bench-support/Cargo/config match inspected4b29296d. Independently verified live GitHub509 OPEN, matching title and exact local/remote body; the entire approved five-helper brief is preserved beneath its numbered title.

Approve fresh Luna1 for this single package outcome. Preserve the exact five audit module paths, unchanged one-shot input boundaries and all raw/incremental hashes, decoders, record/manifest bytes and error ordering. Use existing bench_support::digest::sha256_hex aliases and delete only duplicate helper bodies plus actually unused imports. No Cargo, shared encoder/API, runtime, schema, fixture, historical seal or validator change is authorized.

The numbered gate roster remains finite and executable: five independent empty/abc alias tests, existing vectorization parser checks, small read-only audit fixture checker tests, exact literal JSON parser and small in-memory level-match tests, debug/release; strict affected Clippy/fmt/diff and existing static policies. No source-fixture generator matrix, listening packet authoring, fixture regeneration/repin, timed runner/preflight/capture or new qualification framework. Preserve historical tests not selected for proportional local execution; required CI remains enabled.

CP20 remains partial for other encoder owners. Root retains checkpoints, remote synchronization, Luna1/Sol2/3 workflow and actual-PR/required-CI delivery gates. This independent tooling work does not change #496/#478 runtime sequencing. No builds/tests/timing or source/spec/Git/GitHub mutations performed; only /tmp review/readback files written.

Root integrated delivered main1543c4c2 after PR508 and its closure record. All five audit files remain byte-identical to the reviewed numbered base; #496 only changes builtins runtime/test code and current artifact identities. Root activates fresh Luna attempt1 within the unchanged five-file maintenance scope. No generation/timing authority is granted.

## Luna attempt 1 source and evidence

Source5368843f replaces all five one-shot text helper bodies with existing shared imports. The frozen final debug/release filters execute5,6,3,1,1 tests with exit0; strict affected Clippy/fmt/diff and both static policies pass. Raw records in artifacts/issue509-audit-hash-authority preserve the initial empty-target-directory setup failures, final actual source hashes and corrected execution. Root verified hashes; no fixture generation, repin, runner or timing was invoked. Consolidated Astra review pending.
