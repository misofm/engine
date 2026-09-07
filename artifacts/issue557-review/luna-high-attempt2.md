# Issue #557 Luna HIGH attempt-two correction evidence

This tranche addresses only the three blockers in `artifacts/issue557-review/sol-high-attempt1.md`.

## Lineage and changed paths

- Base `main`: `30f658ee1c0c7d86002f5f2fea075a5dfa8a7c2c`.
- Attempt-one parent/current pushed head: `6946339057fb449492467ce245b2367dc8f65845`.
- Candidate status: uncommitted working tree descended directly from the parent; root owns the
  checkpoint and will record the final implementation head after committing.
- Parent-to-candidate changed paths:

  ```text
  tools/bench-support/src/sysinfo.rs
  tools/bench/src/conformance.rs
  tools/bench/src/session.rs
  ```

No policy script, manifest, lockfile, fixture, runner, schema, or other subject changed in this
correction tranche.

## Correction evidence

`sysinfo::SourceValue` now has independent private test-seam states `Value`, `Absent`, and
`NonUnicode`. `from_snapshot` maps the memoized `Metadata::var` errors to the corresponding
states. The injected test `absent_non_unicode_and_empty_environment_values_are_independently_injectable`
passes absent, non-Unicode, empty, and valid values through the same fallback function and through
`HostToolchainFacts::from_sources`; it does not mutate the process environment or `PATH`.

The two exact projection tests derive only `env::consts::ARCH` and `env::consts::OS`. Their
ordered contract-owned metadata bytes remain literal and exact, including field names, values,
missing-list order, and the fixed synthetic fixture values.

## Source hashes

The following are raw `sha256sum` results from the candidate and base source files:

```text
candidate 9c23f823e545870234729eb965752cab8bd1455a942dd5d8bd395df41aaf4f39  tools/bench-support/src/sysinfo.rs
candidate bdbc81871e10bba8ce45ab8f84092c0acdec63f29ab01413fc55b56e2a572293  tools/bench/src/session.rs
candidate 38cdcde82ce866efe9ab894eade25142d67bb9db6149a322202cc43197ea3bbd  tools/bench/src/conformance.rs
base      33e5ebd2d9e3d6fd5d61ffe59b4321c2fc78f5e8ca891943aba6176d2d07f6bb  tools/bench-support/src/sysinfo.rs
base      dd5bff1b2ac41808302821ef9664a527fc8ec371def51631b76eadf3ec286c5e  tools/bench/src/session.rs
base      5e2094408de10f80e9f3106bd72d953c9f985b911fcc33bdcaf5fee5ae33d8ff  tools/bench/src/conformance.rs
```

## Finite gate results

All commands below exited `0`; no benchmark subject, runner, timed workload, or timing command
was invoked.

```text
cargo test --locked -p bench-support sysinfo::tests                         11 passed, 0 failed
cargo test --locked -p bench --bin bench session::tests::shared_host_toolchain_facts_preserve_session_projection -- --exact
                                                                           1 passed, 0 failed
cargo test --locked -p bench --bin bench conformance::tests::shared_host_toolchain_facts_preserve_conformance_projection -- --exact
                                                                           1 passed, 0 failed
cargo test --locked -p bench --bin bench                                      37 passed, 0 failed
bash scripts/check-bench-policy.sh                                          ok (1 allocator, 1 escaper, 1 percentile, 1 digest sink, 6 unsafe owners, 3 subjects)
bash scripts/test-bench-policy.sh                                           ok (all directed mutations, including grep/parser/sort faults)
cargo clippy --locked -p bench-support -p bench --all-targets -- -D warnings   finished, 0 warnings
cargo fmt --all -- --check                                                  exited 0
bash scripts/check-workspace-policy.sh                                       workspace policy: ok
cargo check --locked -p bench-support                                        finished
cargo check --locked -p bench --bin bench                                      finished
cargo check --locked --target wasm32-unknown-unknown -p bench-support        finished
cargo check --locked --target wasm32-unknown-unknown -p bench --bin bench    finished
git diff --check                                                              clean
```

## Gate-nine post-implementation diff audit

The record-format extraction command compared the `json_record` `concat!` blocks against base
`30f658ee`; both hashes matched byte-for-byte:

```text
tools/bench/src/session.rs      base=candidate 6d824942eeab0ad2c60b386e283ae404b59f7f46773ddbfdd53d036e87398934  cmp_status=0
tools/bench/src/conformance.rs  base=candidate 82e40ae3d67395a9cad7f8f290a58d3b2347215c889886e1e741358437b18ddc  cmp_status=0
```

The same audit compared each parent-to-candidate production section (through the `cfg(test)`
boundary). The marker scan had a clean no-match (`status=1`) in all three files for:
`schema_version`, `fixture_path`, `fixture_sha256`, `fixture_size_bytes`, `fixture_crc32c`,
`percentile_method`, `warmup_batches`, `measured_batches`, `operations_per_batch`,
`batch_samples`, `timer`, `missing_metadata`, `metadata_incomplete`, `architecture`, and `os`.
The test-only assertion changes are the intended platform portability correction.

The explicit category audit is:

| Gate-nine category | Result and evidence |
| --- | --- |
| Record-format strings | Unchanged; both production `json_record` format hashes match base and parent. |
| Key order | Unchanged; the extracted production format blocks are byte-identical. |
| Schemas | Unchanged; no production schema marker changed. |
| Fixture/hash identities | Unchanged; fixture path, SHA/CRC, and fixture-size markers did not change in production. |
| Percentiles | Unchanged; percentile method and p50/p95/p99/p99.9 markers did not change in production. |
| Workloads | Unchanged; warmup/measured batch and operation/sample markers did not change in production. |
| Timers | Unchanged; the record timer marker did not change in production. |
| Environment names | Unchanged; common names remain owned by `sysinfo`; the correction changes only private source-state tests and derives platform constants in exact tests. |

The platform-literal scan over both exact projection tests returned no hard-coded projection
`architecture`/`os` value (`literal_scan_status=1`). The correction audit also recorded
`timed-workload-scan=NOT RUN` per the issue prohibition.
