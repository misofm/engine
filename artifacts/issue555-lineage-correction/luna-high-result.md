# Luna HIGH lineage correction result

Starting at clean pushed head `cb7a6b94e4e367486890a93a472df2983a8f883a`, Luna HIGH changed only `hosts/host-web/qualification/results.json` and the generated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`.

The result changed only `candidateCommit` to frozen artifact source `e4f46fa808e413507d204e81b6a4ebc27254869c` and `wasmSha256` to `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`. Every other parsed field and every browser row stayed identical. Regeneration with the unchanged generator changed exactly the corresponding matrix lineage sentence.

The retained six-file artifact and Wasm digest passed the non-browser static lineage preflight. Generator consistency, `git diff --check`, and the two-path scope audit passed. No browser was launched and `--record-matrix` was not used. Root independently repeated the semantic and path-scope checks, then committed the exact tranche at `8afb380ebd1338202a296a8b1e9835c11914982f`.
