# #509 Luna attempt 1 report

## Verdict

PASS for the authorized five-file source tranche on branch `codex/509-audit-hash-authority`, base `8f1c483fd844d1e3b1e50c4c409d96c745a87f0e`.

## Changes

Replaced the one-shot SHA-256 helper in each authorized audit module with the existing `bench_support::digest::sha256_hex` import, preserving the existing call names (`sha256` aliases where required). The source fixture module imports the helper under its existing `sha256_hex` name. Raw/incremental `Sha256` tree hashing remains in `fixture_builtins.rs` and `builtins_fixture_check.rs`. Added one independent `shared_sha256_alias_matches_published_literals` test to each module, checking the fixed empty and `abc` lowercase SHA-256 strings.

Only these paths changed:

- `tools/audit/src/vectorization.rs`
- `tools/audit/src/source_fixture.rs`
- `tools/audit/src/fixture_builtins_listening.rs`
- `tools/audit/src/builtins_fixture_check.rs`
- `tools/audit/src/fixture_builtins.rs`

No Cargo, shared helper/API, runtime, schema, fixture, manifest, listening record, validator, or historical seal changes.

## Focused tests

Successful direct subprocess captures for all five named tests in debug and release are in `/tmp/509-luna1-1788677618-617204/`. `status.tsv` records numeric exit status and argv; `identity.txt` records cwd, HEAD, each actual source SHA-256/blob, and the whitelisted effective environment. Each command has independent `.stdout`, `.stderr`, and `.meta` files.

All ten invocations exited 0. The five-test filter reported exactly five new tests. The vectorization and builtins checker filters passed their existing focused populations plus the new alias test; the two exact filters passed their named existing tests plus the new alias test where applicable.

The first capture attempt is preserved in `/tmp/509-luna1-1788677597-616854/`; all ten commands exited 101 because the inherited `CARGO_TARGET_DIR` was explicitly passed as an empty value. This was an environment-capture setup failure, not a source/test failure. The corrected run omitted unset optional variables while retaining `PATH=/home/bl/.cargo/bin:$PATH`.

## Static checks

Successful direct subprocess captures are in `/tmp/509-luna1-static-1788677701-622851/`; all five commands exited 0:

- `cargo clippy --locked -p audit --bin audit --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `git diff --check`
- `bash scripts/check-bench-policy.sh`
- `bash scripts/check-realtime-audit-leak.sh`

No full audit CLI, fixture/listening generation, repin, benchmark/preflight/capture/timing, or source-fixture generator matrix was run.
