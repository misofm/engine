Filed by root from the #2111 verification (2026-10-26).

## Problem
#2111 ships `miso-engine-v1-blake3.simd128.wasm` in the delivery closure. Two gaps remain:
1. Its Wasm path is tested only in node V8. No committed test runs it in a real browser (Chromium or WebKit), in a Worker. The engine's web tree resolves the default URL to a path that does not exist, so the browser qualification legs take the JS fallback.
2. The release fingerprints only the AudioWorklet module. The BLAKE3 module has no release pin.

## Smallest slice
- One qualification row in Chromium and WebKit that loads the module in a Worker, resolves `"simd128"`, and hashes the corpus with the expected identities.
- A release pin for the BLAKE3 module beside the worklet pin, checked by the identity report and by npm-publish's qualify step, with a test-npm-publish-modes mutation.


## Approved brief and decisions (2026-10-27)
Root approved the packaging/qualification slice above. The owner requested Sol 6.1 high implementation, replacing the standard Terra implementation stage; fresh Astra review follows the independent issue implementations. No runtime DSP, hash algorithm, worklet pin, or store ownership change is authorized.

- The raw host module lives at `/web/stem-store/incremental-blake3.js`, whose existing static default resolves to `/assets/miso-engine-v1-blake3.simd128.wasm`. The qualification server now serves that layout, which also matches the SDK's `dist/browser/stems` to `dist/assets` relationship. The row uses no URL override.
- One Worker row per browser runs in the normal qualification flow; `--blake3-only` runs this bounded packaging gate without rebuilding the SDK or exercising unrelated audio rows. JavaScript fallback is an immediate failure, before corpus hashes.
- The BLAKE3 release pin and `EXPECTED_BLAKE3_SHA256` have independent release lineage, approved by root so adding the BLAKE3 pin does not force an unrelated worklet repin. A package-version change checks both identities; a browser toolchain change requires both qualifications. `results.json.blake3Qualification` records its own candidate, built hash, and both browsers' SIMD completion. `--record-blake3 --blake3-only --browser all` updates only that lineage.
- The identity job reads BLAKE3 from the artifact job's uploaded closure. npm qualification verifies the source pin/environment agreement and the built BLAKE3 bytes; publish/verify retain the existing qualified-archive path. No release is dispatched or package published here.

## Attempt 1 implementation evidence (Sol)
Initial focused verification: identity-report self-test PASS; `scripts/test-npm-publish-modes.py` PASS, including executing the real workflow source-pin and build-digest checks against changed/empty BLAKE3 bytes, stale/missing pin, and stale environment fingerprint. Chromium 151.0.7922.34 and WebKit 26.5 each returned `simd128` and all six existing `fixtures/stem-identity/v1/VECTORS.tsv` identities.

Initial browser artifact provenance: `/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/fresh-astra/browser-artifacts`, reviewed source `08fd9fa7277af1729506dab46762cfdd0904645e`; `git diff` from that source to this worktree's baseline is empty for `hosts/blake3-web`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `scripts/build-web-audioworklet.sh`, and `scripts/strip-wasm-names.py`. This reused closure has an older worklet; these scoped rows exercise only its unchanged BLAKE3 module. Fresh Astra will build the integrated closure once and verify it. BLAKE3 SHA256: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1`.

Test value for review: the Worker parity row catches an absent/misrouted default asset or browser-specific load/instantiation failure that Node-only Wasm coverage and JavaScript-fallback browser ingest allow to pass. The extended npm workflow test catches a qualify step accepting altered BLAKE3 bytes or a stale/missing source fingerprint while the independently pinned worklet still matches. The extended identity-report self-test catches BLAKE3 release or toolchain checks accepting stale bytes, missing SIMD browser qualification, or malformed candidate lineage independently of the worklet.

Checkpoint status: local implementation and focused gates green; lineage recording and explicit browser red mutation still pending; fresh independent verdict and remote delivery remain root-owned. All verification uses the batch's shared `run-serial.sh`; no Rust build or full browser matrix was run in this worktree.

## Attempt 1 continuation evidence
Implementation checkpoint: `8890792627e9eb1abf49f8626403d9016fad8cc8` (verified as an existing commit). The recorded `blake3Qualification.candidateCommit` is that checkpoint; its two-browser record hashes the unchanged BLAKE3 bytes described above. The worklet's existing `candidateCommit`, `wasmSha256`, browser rows, matrix, and release pin remain unchanged.

- Chromium and WebKit route-revert proof: temporarily removing only the `/assets/` server route makes each scoped row fail with `expected simd128, got javascript`; restoring the route makes both pass the six identities. The reversible proof restored the server before later checks. External logs: `2117/chromium-default-asset-revert-red.log`, `2117/webkit-default-asset-revert-red.log`, `2117/browser-green.log`.
- `scripts/web-audioworklet-identity.py --self-test`: PASS, including independent BLAKE3 pin/environment, stale qualification, JavaScript completion, malformed candidate, and npm toolchain mutations; a BLAKE3-only pin update passes while the worklet pin remains unchanged. Log: `2117/identity-self-test.log`.
- `scripts/test-npm-publish-modes.py`: PASS, including the behavioral BLAKE3 source pin and built-byte mutations. Log: `2117/npm-mode-tests.log`.
- The existing CI routing policy seals the identity job's shape; adding the artifact download therefore required synchronizing its existing step contract and checkout-mutation anchor. The downloaded closure is verified against the artifact job's digest before the report reads BLAKE3. `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py`: PASS. Logs: `2117/ci-routing-check.log`, `2117/ci-routing-tests.log`.
- Test value of the rewritten checkout mutation: losing the identity job's full-history checkout still turns its owning routing test red after the new download step is inserted; without this check, a shallow checkout could make every report unable to find its base.

Evidence directory for all paths above: `/home/bl/misofm/engine-work-evidence/easy2-20261027/`. `2117/blake3-lineage-record.log` records the successful lineage run. No full audio matrix, Rust build, release dispatch, npm publication, or upstream push was performed in this worktree. Fresh independent Astra review and integration verification remain required, followed by root-owned GitHub synchronization and delivery. Local gates are green; this implementation author does not claim the independent verdict.
