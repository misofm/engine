# #514 browser resource integration ruling — approved, exactly two rows

Reviewed clean `f01ad542fc15893d1669d0c1ba43fdb8cdbf2f43` in `/home/bl/misofm/engine-idle-admission-clear`, the actual terminal resource failure, both explicit print-mode oracle captures, published modules and host ownership formulas. The derivation is sufficient for the two observed resource rows. No additional layout probe or production attempt is required.

## Actual evidence

The ordinary builder and static checks have status 0. `/tmp/514-qualified-resource.{command.json,log,status}` identifies this source and the current artifact directory, records status 1, names exactly `bridgeMetadataBytes 4235 → 4243` and `bridgeRetainedBytes 24744 → 24752`, and reports all 26 red controls passed. Preserve that genuine failure.

Independently hashed both retained published modules:

- Current `/tmp/engine-514-qualified/miso-engine-v1-audio-worklet.simd128.wasm`: 2,693,780 bytes, SHA-256 `c06f9517763890a081f9ee60c70fc8531fdcafbd9630495e4de5782bcf0dc79a`, matching the separately approved current pin.
- Delivered #511 `/tmp/engine-511-qualified/miso-engine-v1-audio-worklet.simd128.wasm`: 2,693,746 bytes, SHA-256 `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`. This is the previously qualified module, reused without a new baseline build.

Both `/tmp/514-integration-browser-{direct-oracle,baseline-oracle}.{command.json,stdout,stderr,status}` captures run the unchanged direct oracle with `MISO_ENGINE_WEB_ORACLE_PRINT=1` at clean `f01ad542`, against the respective published directory. Both have status 0 and empty stderr. Independently verified all four recorded source SHA-256/blob identities against current host production, tests, expected document and artifact-pin bytes.

Recursive comparison establishes that the complete baseline output equals `expected.json.directOracle` exactly. Current versus baseline has exactly two differences, at `/simd128/resources/bridgeMetadataBytes` and `/simd128/resources/bridgeRetainedBytes`. Every other value, key and array element is equal, including all PCM digests, command acknowledgements/application samples, observation values, status/memory, source/builtin/effect/graph rows and largest allocations. The oracle asserts identity, command and observation PCM equality to their unchanged native pins before printing; print mode bypasses only the final full-document comparison. These are current Wasm comparisons to existing native pins, not a claim that the print invocation performed a new native render.

## Target-specific accounting derivation

In `hosts/host-web/src/lib.rs:2423`, `project_buffers` computes:

`bridge_metadata_initial = H - BOOT_OPTIONS_BYTES - STATUS_BYTES + P`

where `H = size_of::<AudioWorkletEngineHost>()` for the actual build and `P` is the configured plane-reference table. `compile_ready` adds the same ready metadata `U = control_retained_bytes + session_model_bytes` to both bridge rows (`lib.rs:2676`). Thus the reported metadata is:

`M = H - 64 - 80 + P + U`.

Compared with delivered #511, production/build inputs differ only by the accepted #514 host change; the other source difference is its test, and the pin records the resulting module identity. The only added host-owned storage is the private boolean inline in ReadyOwnership, retained inside the host. It creates no heap owner. Configuration, plane-reference shape, source controls, compiled session and their accounting are unchanged. The `P`, `U`, options and status terms are therefore unchanged. The actual Wasm observations give `ΔM = 4243 - 4235 = 8`; through this unchanged formula they establish `ΔH = 8` for these two built modules. This is a target-observed host-shell charge, not an extrapolation from native padding. No unobserved absolute Wasm struct size, field offset or alignment is asserted.

For this exact oracle fixture, the retained total includes metadata once plus seven unchanged public rows. Their sum is:

`64 + 80 + 1919 + 16384 + 14 + 1024 + 1024 = 20509`.

Therefore baseline retained is `20509 + 4235 = 24744`, and current retained is `20509 + 4243 = 24752`. The two observations reconcile exactly with the existing ownership formula, without adding the host delta twice. Aggregate host budget totals that include bridge retained once consequently inherit the same delta; no additional expected aggregate row is requested.

Both largest bridge/named rows remain 16384 in the actual module. The changed shell/metadata charges do not supersede that existing diagnostic allocation. Queue payloads and the inherited #511 graph reservation remain untouched; its three graph rows retain their delivered values. The actual resource comparison and the uniquely changed term establish precisely the required delta. A separate temporary layout probe would only be needed to claim absolute Wasm struct sizes/offsets, which this bounded ruling does not require or claim.

## Exact authorized correction and remaining gates

Approve only these two numeric substitutions in `hosts/host-web/tests/browser-v1/expected.json`:

- `directOracle.simd128.resources.bridgeMetadataBytes`: `"4235"` → `"4243"`.
- `directOracle.simd128.resources.bridgeRetainedBytes`: `"24744"` → `"24752"`.

Record this derivation in the matching decision/evidence checkpoint. Preserve every other expected value, PCM/native digest, canonical fixture, resource row, largest allocation, memory/status/timeline result, schema, production file and gate. No new artifact digest is authorized.

On the checkpointed corrected document, run the existing direct oracle without print mode against the same current published module and require complete equality. Run the ordinary resource gate including native-row witness and all 26 controls. Then finish the already approved hermetic worklet, pinned npm installation, all three current browsers with existing mutation self-tests, and generated matrix checks. Reuse the accepted module because neither its production inputs nor its pin changes. Attribute the corrected expected-document hash and all terminal statuses explicitly. Preserve both pre-edit oracle captures and original failure unchanged.

Any further observed discrepancy requires a separate concrete ruling. Final integrated review, actual-PR/required CI and GitHub synchronization remain root's obligations. This is a bounded integration expectation correction, not a new production attempt or final delivery PASS.

No builds, tests, timing, source/spec edits, Git mutations or GitHub operations were performed. Read-only inspection, hashing and JSON comparisons were used; the only file written is this requested `/tmp` ruling.
