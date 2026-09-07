# Issue 542 bounded attempt 3 evidence

- Candidate: `cbfbc70cd972ffbb2d7c7d7c80a62d7acdb44845`
- Scope: `scripts/test-conformance-boundaries.sh` only
- Verdict: **PASS**
- No product, corpus, Wasm, dependency, benchmark, or unrelated policy files changed.

Each `.meta` capture records UTC, cwd, HEAD, status, source hashes, exact command, and numeric
exit. Each `.raw.gz` is the lossless combined stdout/stderr capture. `verdict.txt` records the
overall result.

| Gate | Exit | Evidence |
|---|---:|---|
| Shell syntax | 0 | `syntax.{meta,raw.gz}` |
| Production boundary checker | 0 | `production-boundary.{meta,raw.gz}` |
| Full hermetic boundary suite | 0 | `hermetic-boundary-suite.{meta,raw.gz}` |
| Protocol default tests | 0 | `protocol-default-focused.{meta,raw.gz}` |
| Protocol `test-support` tests | 0 | `protocol-test-support-focused.{meta,raw.gz}` |
| Default protocol dependency/API absence | 0 | `protocol-default-dependency-api.{meta,raw.gz}` |
| Extracted population and ignored census | 0 | `extracted-test-population.{meta,raw.gz}` |
| Extracted default and `test-support` lists | 0 | `extracted-test-list-{default,test-support}.{meta,raw.gz}` |
| Format and scope/diff checks | 0 | `diff-and-scope.{meta,raw.gz}` |

The hermetic fixture now contains all three exact parent declarations and child `tests.rs` files.
The live source census separately retains 44 controller, 17 message-wire, and 19 session-wire
tests. Focused controls cover missing child population, missing guard, changed guard, and an
outside-path `conformance` use; each resets from the clean fixture. The retained prior
mutation/status suite and fail-open module-consumer mutant also pass.
