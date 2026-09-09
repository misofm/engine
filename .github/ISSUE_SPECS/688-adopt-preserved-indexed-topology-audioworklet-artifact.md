# Adopt preserved indexed-topology AudioWorklet artifact

## Parent and outcome

Parent implementation: #685. Exhausted predecessor: #687. Coordination:
#559 and #560. Audit: #349 CP1.

Adopt and qualify the exact six-file AudioWorklet candidate already built from
source-qualified indexed-topology product commit
`276ffb6097a84088e3b5f4a16892a33bca9e26fb` at source
`c9ccf6acf4030a0b567b5c18cbb0f52126aa16f5`. This successor exists because
#687 exhausted all three attempts after contradictory execution ownership. It
does not retroactively pass #687 or reuse its attempt count.

The candidate authority is the preserved ordinary directory
`/tmp/issue687-stage2-artifact`. It must contain exactly these ordinary files:

```text
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919 miso-engine-v1-abi-layout.json
445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf miso-engine-v1-audio-worklet-host.d.ts
21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb miso-engine-v1-audio-worklet.js
31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b miso-engine-v1-audio-worklet.simd128.wasm
6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d miso-engine-v1-parameter-metadata.json
```

Do not rebuild, replace, chmod, copy over, or otherwise mutate those bytes.
Preserve all #687 paths and contradictory records unchanged. The #687 builder
and post-stop static gate have observed status-zero records but no qualification
credit.

## Attempt 1: bounded candidate qualification

One Luna HIGH executor may act only after Astra LOW passes the exact pushed
scope and matching GitHub bodies. Create fresh, initially absent paths:

```text
/tmp/issue688-source
/tmp/issue688-evidence
/tmp/issue688-target
/tmp/issue688-hermetic-target
```

Create a detached source worktree at exact `c9ccf6ac`. In that scratch
worktree, change only the artifact pin to the candidate digest, set
`candidateCommit` in `hosts/host-web/qualification/results.json` to the full
product commit and `wasmSha256` to the candidate digest, then regenerate the
tracked browser matrix once. Require exactly those three tracked overlays.

Run each gate once, in order, using the preserved artifact directly:

```text
CARGO_TARGET_DIR=/tmp/issue688-target bash scripts/check-web-audioworklet.sh /tmp/issue687-stage2-artifact
CARGO_TARGET_DIR=/tmp/issue688-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue687-stage2-artifact
CARGO_TARGET_DIR=/tmp/issue688-hermetic-target bash scripts/test-web-audioworklet.sh
npm --prefix sdk ci --ignore-scripts
CARGO_TARGET_DIR=/tmp/issue688-target bash scripts/sdk-package.sh check /tmp/issue687-stage2-artifact
npm --prefix hosts/host-web/qualification ci --ignore-scripts
CARGO_TARGET_DIR=/tmp/issue688-target npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue687-stage2-artifact --browser all --check-matrix --self-test-mutations
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Stop at the first failure and do not retry a command. Keep all Cargo output under
the named `/tmp` targets. Persist concise command, status, and stream records
under `/tmp/issue688-evidence`; no compiler stream, target, binary, generated
SDK/Wasm artifact, browser dependency, or evidence capture may enter Git.
Preserve every scratch path through Astra LOW adversarial review.

Astra reviews source identity, the exact six candidate hashes, ordered statuses,
the three-file scratch overlay, matrix consistency, and absence of a source-local
`target/`. PASS qualifies the preserved bytes for promotion only.

## Promotion boundary

No pin edit, repository promotion, PR, merge, or delivery is authorized by this
initial scope. After attempt-1 evidence PASS, amend this issue once with the
smallest promotion and post-pin ordinary-build check, push that amendment, and
obtain fresh Astra LOW scope PASS. The post-pin build must use a new external
`/tmp` target/output and reproduce the candidate digest without committing the
artifact. Only then may the exact source, pin, qualification record, and specs be
reviewed and delivered in one PR.

Failure after three attempts hard-stops this successor. Do not weaken gates or
create another continuation. CP1 stays partial until the source and artifact pin
are merged, post-main qualification passes, #685 and this issue are synchronized
and closed, and clean delivered worktrees are removed.
