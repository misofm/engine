# Astra #511 artifact integration ruling — approved, bounded

Reviewed clean frozen `0f30be0a76f89994059e9ef435b8ddb6edcc152f` in `/home/bl/misofm/engine-slot-reservation`, the accepted final source review, `/tmp/511-delivery-build.py`, actual builder command/log/status, ordinary builder source and existing #496 integration route. The only change since accepted `8d07e256` is the #511 PASS adoption in its issue spec; all seven accepted source files and the build inputs remain identical.

The builder metadata identifies this exact frozen source, output `/tmp/engine-511-qualified` and outer `CARGO_TARGET_DIR=/tmp/engine-511-artifact-target`. The wrapper checks the accepted HEAD and clean worktree before invocation and checks unchanged HEAD/cleanliness afterward. The ordinary script internally isolates its Wasm compilation in its own temporary target directory and removes it on exit. Its log records successful release compilation, then numeric builder status 1 at the actual artifact comparison:

- Existing expected SHA256: `25c1b72a65ebfd081c74d431614cfba42492e95e490cc4d7c203ee14fe8737e9`.
- Observed SHA256: `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`.

The expected value matches the current pin file. The output directory is empty: the mismatch occurs before publication, and the temporary module has been cleaned up. This review verifies the source/command lineage and authentic observed mismatch; it does not pretend to independently hash a retained or published module that is absent.

Approve exactly the observed SHA256 replacement in `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, the corresponding decision/evidence checkpoint, and the existing seven-step consumer pipeline: ordinary verified rebuild, static/object/ABI checks, resource gate including its 26 rejection controls, hermetic worklet, npm installation, current three-browser qualification with existing self-tests, and generated matrix check. Verify the newly published module's actual bytes/SHA against this exact pin and all current consumer records. Candidate/hash identity records may change as required by that existing route.

This approval does not authorize numerical resource or PCM expectation repins, canonical changes, corpus pins, schemas, gate/script/CI/lint changes, or further runtime implementation. Any actual numerical or additional source discrepancy requires a separate concrete ruling; do not adjust expected numerical outputs to obtain a passing gate. The anticipated artifact identity change does not establish any unobserved numerical outcome.

The immutable workspace/supported-target/native ABI sequence remains pending in the root's active session. Do not mutate its tracked worktree, including pin/spec changes, until that sequence is terminal. This ruling supplies no implied permission to interrupt or contaminate that immutable run and makes no claim that it has passed. Preserve the original mismatch records unchanged.

After all mandatory terminal qualification, the existing final actual-PR exact-head review, required CI and GitHub delivery synchronization remain necessary. #478 remains subject to completed #511 delivery and fresh base review. No timing, benchmark, new matrix or extra fixture framework is authorized. Read-only review; no builds/tests, source/spec/Git/GitHub mutations or timing were performed. Only this requested temporary ruling was written.
