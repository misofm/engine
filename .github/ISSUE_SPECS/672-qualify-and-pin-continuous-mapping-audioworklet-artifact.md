# Qualify and pin the continuous-mapping AudioWorklet artifact

Parent: #560 (lane B, CP8)

Predecessors: #669 and #670

Coordination: #559

## Problem and inherited result

#670 independently qualified the frozen `continuous_mapping_admissible` extraction and its complete typed and borrowed public fixture matrix. Astra LOW returned SOURCE PASS at `fe6ddb4d1f1aadb254a2cd5e95732652fd457351`. The accepted product hashes are:

- `crates/effect-contract/src/lib.rs`: `be709c2293b108feccfe14b0049c08e32d09ce61188a865dca59fa6cee185f98`
- `crates/effect-package/src/wire.rs`: `9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818`

The shipped `host-web` depends on `effect-contract`. #670 therefore ran one repin-report identity probe. It returned candidate digest `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`, different from delivered pin `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`. Astra confirmed the drift, but rejected the probe as qualification evidence because its retained preflight omitted source hashes, live-main identity, pin byte shape, and explicit fresh-path absence, while its postflight contains unlabeled corrected fields. Preserve that record without reconstruction or rerun. The observed digest is only the expected candidate for this fresh qualification.

#670 closed source-qualified and delivery-incomplete at `2cf84f301d6812d3f915a3ade475828d6f425b54`. This successor owns candidate qualification, pin and lineage promotion, exact delivery, and cleanup. It is the second active issue alongside disjoint #671.

## Ownership and immutable scope

Sol HIGH coordinates the brief, checkpoints, artifact decision, GitHub synchronization, PR, merge, and cleanup. Luna HIGH `/root/issue583_luna_impl` is the sole executor. Per current user routing, Astra LOW performs scope, pre-pin artifact, exact-head, integration, and delivery review.

The qualification source is the accepted product checkpoint `fe6ddb4d1f1aadb254a2cd5e95732652fd457351`; the successor branch begins at #670 disposition `2cf84f301d6812d3f915a3ade475828d6f425b54`. Product source, tests, Cargo manifests/lock, toolchain/config, ABI, JS/TS, metadata, resource and PCM expectations, browser rows, scripts, workflows, and fixtures are frozen. Before pre-pin PASS, repository pins and lineage files are frozen too.

After Astra LOW pre-pin PASS, the only product/config edits authorized are:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, changed to the approved candidate digest plus LF;
- `hosts/host-web/qualification/results.json`, changing only `candidateCommit` to the accepted product checkpoint and `wasmSha256` to the approved digest;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, regenerated from that otherwise unchanged results document so only the corresponding lineage sentence changes;
- this numbered issue record.

Do not change browser results, version floors, gates, resources, PCM identities, source, dependencies, locks, builders, checkers, or expectations. No benchmark, timing, optimization, listening claim, DSP change, new harness, or matrix expansion belongs here.

## Fresh qualification procedure

All work uses fresh absent non-symlink `/tmp/issue672-*` paths and separately records exact argv, cwd, executor/time, head/upstream, live main and merge-base, source/config hashes, Rust/Cargo/Node/npm/browser versions, relevant environment, complete stdout/stderr, numeric status, and before/after identities. Exercise the capture wrapper with harmless status-0 and expected status-1 controls. Stop on a failed prerequisite or gate without correction or retry. Finalize a self-excluding manifest and record its verification output/status/hash outside the evidence directory.

1. Export exact live main and the frozen product checkpoint into separate scratch trees without `.git`, targets, dependencies, or prior output. Prove every tracked path, executable bit, symlink target, and byte identity against its source commit. The frozen product export may additionally carry this issue record; no build input may differ.
2. In the live-main scratch, create one fresh empty output directory and run the ordinary unchanged `scripts/build-web-audioworklet.sh` exactly once. Require the exact six-file set and delivered Wasm digest `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`.
3. In the candidate scratch, overlay only the artifact pin with the expected candidate digest plus LF. Prove the one-line overlay and run the ordinary builder exactly once into a fresh empty output directory. Require the exact six-file set and independently computed Wasm digest `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`. A different digest stops; it is no authority to select another pin.
4. Produce complete manifests for both six-file sets and classify every delta. Run `wasm-validate`, `wasm-objdump` section/import/export summaries, and the repository's existing Wasm structural gates. Full dumps, `.ll`, `.s`, binaries, objects, archives, Cargo targets, and raw compiler streams remain temporary; Git receives only compact identities, counts, commands, statuses, and reviewed delta conclusions.
5. From the candidate scratch, run exactly once in order: `bash scripts/check-web-audioworklet.sh <candidate>`; `python3 -B scripts/check-browser-expected-resources.py --artifacts <candidate>`; `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-sdk-generated.sh`; `python3 -B scripts/check-sdk-deletions.py`; `python3 -B scripts/check-sdk-deletions.py --self-test`; `bash scripts/check-sdk-types.sh`; `bash scripts/check-sdk-headless.sh <candidate>`; and `bash scripts/sdk-package.sh check <candidate>`. Use the already pinned dependency graph; installs may create ignored scratch-only dependency directories but must not alter manifests or locks.
6. For browser qualification, apply scratch-only lineage overlays to `results.json` and the generated deployment matrix: change only the candidate commit/digest fields described above and prove every browser row and other parsed value byte/semantically unchanged. Run the existing all-browser command once from the scratch source: `npm --prefix hosts/host-web/qualification run qualify -- --artifacts <candidate> --browser all --check-matrix --self-test-mutations`. Chromium, Firefox, and WebKit must each execute and pass every existing gate and mutation; `--record-matrix` is forbidden.
7. Run the already accepted #670 source gates only as focused frozen-source/current-environment confirmation: affected debug/release tests, strict affected Clippy, formatting, effect runtime/package/descriptor/workspace policies, and diff hygiene. This supplies no new implementation attempt and cannot repair source.

Any source/config drift, second candidate identity, artifact-census error, unexplained delta, structural/resource/PCM/SDK/browser failure, zero-selected browser run, or evidence-capture defect stops for root disposition. Do not rerun or repin.

## Review, promotion, and delivery

Astra LOW must review the exact frozen source, live-main baseline, retained candidate, six-file and Wasm delta, every gate record, scratch overlays, incomplete #670 probe disposition, and verified manifest. Candidate novelty or green compilation alone is insufficient. No repository pin or lineage edit occurs before explicit PRE-PIN PASS.

After PRE-PIN PASS, Luna changes only the three authorized pin/lineage paths and pauses for root checkpoint. Then Luna runs the ordinary no-bypass builder once from the clean pushed feature head into a new empty directory. The six output files must be byte-identical to the prequalified candidate. Run the focused pin/lineage generators and static/resource/SDK preflights without repeating browser execution. Root checkpoints and pushes compact evidence; no generated artifact or compiler capture enters Git.

Astra LOW then performs exact feature-head/current-main PR-readiness review. Root opens one PR, waits for required `qualification` SUCCESS on its immutable head/current base, performs guarded live-head/base review, merges, and verifies post-main `qualification` SUCCESS. Synchronize #559/#560 and this issue before closure. The source-qualified #669/#670 and this successor receive product-delivery credit only after merge and post-main success. Remove their clean delivered worktrees only after all checkpoints are pushed and required evidence remains available outside them.

## Acceptance

- The accepted #670 source bytes remain unchanged and integrate on current main.
- Fresh baseline and candidate ordinary builds reproduce the delivered and expected digests respectively.
- Every artifact delta is explained by the accepted mapping extraction; ABI, static realtime properties, resource/PCM identities, SDK/package behavior, and all three browsers remain green.
- Astra LOW returns PRE-PIN PASS before the exact pin/lineage promotion.
- The post-pin ordinary build reproduces all six prequalified files byte-for-byte.
- Required PR and post-main `qualification` runs succeed on their stated immutable commits.
- Git contains no generated Wasm, Cargo target, `.ll`, `.s`, full compiler stream, or raw qualification payload.
