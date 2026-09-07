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
separate. The candidate's in-suite counter-mutant mutates only the scratch `gate_sort_lines`
branch from `else rc=$?` to `else rc=0` and requires the causal wrong-diagnostic failure. Astra LOW
independently reproduced that direct mutation and exit under `artifacts/issue567-review/`; that
review capture is the exact-command evidence for the external counter-mutant gate.

The earlier `sort-status-countermutant.{meta,raw.gz}` and
`sort-status-countermutant-corrected.{meta,raw.gz}` probes are retained as non-credit diagnostics:
nested shell quoting left the mutation ineffective and the suite exited 0. They do not contribute
to the verdict. The later `sort-status-countermutant-direct` and
`sort-status-countermutant-verified` pairs are also non-credit for exact command provenance: their
raw output ordering and missing final diagnostic line do not match their recorded command. They
remain preserved without alteration. The internal candidate control and Astra's independent review
proof supply acceptance.
