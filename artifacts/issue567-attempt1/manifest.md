# Issue 567 attempt 1 evidence

- Candidate: `c0a00b4296ced4e8709ff882e57ead5ec9b2d3b0`
- Scope: `scripts/test-conformance-boundaries.sh` only
- Verdict: **PASS**
- No product, production checker, helper, dependency, lockfile, Wasm, benchmark, or prior
  qualification evidence was changed.

Each final `.meta` capture records contemporaneous UTC/cwd/HEAD/status/source hashes, the exact
command, and numeric exit. Each final `.raw.gz` is lossless combined output. `verdict.txt` records
the overall result.

| Gate | Exit | Evidence |
|---|---:|---|
| Shell syntax | 0 | `syntax-verified.{meta,raw.gz}` |
| Production boundary checker | 0 | `production-boundary-verified.{meta,raw.gz}` |
| Complete hermetic fixture suite | 0 | `hermetic-boundary-suite-verified.{meta,raw.gz}` |
| Protocol default and `test-support` tests | 0 | `protocol-{default,test-support}-focused-verified.{meta,raw.gz}` |
| Default dependency/API absence | 0 | `protocol-default-dependency-api-verified.{meta,raw.gz}` |
| Extracted population and zero ignored | 0 | `extracted-test-population-verified.{meta,raw.gz}` |
| Extracted test lists | 0 | `extracted-test-list-{default,test-support}-verified.{meta,raw.gz}` |
| Format and scope/diff checks | 0 | `diff-and-scope-verified.{meta,raw.gz}` |

The workspace `sort` discriminator targets manifest-path input and fails after all earlier TOML
dependency sorts delegate successfully. The existing TOML dependency-sort controls remain
separate. The direct counter-mutant proof in `sort-status-countermutant-verified.{meta,raw.gz}`
mutates only the scratch `gate_sort_lines` branch from `else rc=$?` to `else rc=0`; the candidate
suite exits 1 with the causal `wrong conformance diagnostic ... no workspace library names found`.

The earlier `sort-status-countermutant.{meta,raw.gz}` and
`sort-status-countermutant-corrected.{meta,raw.gz}` probes are retained as non-credit diagnostics:
nested shell quoting left the mutation ineffective and the suite exited 0. They do not contribute
to the verdict. The internal candidate control and the direct proof both use the exact mutation.
