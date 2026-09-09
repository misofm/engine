# Complete continuous-mapping AudioWorklet artifact delivery

Parent: #560 (lane B, CP8)

Predecessors: #669, #670, and exhausted #672

Coordination: #559 and disjoint #677

## Problem and accepted inherited evidence

#670 source-qualified the `continuous_mapping_admissible` extraction. #672 then
exhausted three evidence attempts without repository promotion. Its final attempt
did reproduce the expected candidate, prove all five non-Wasm files unchanged,
and pass Wasm validation plus the static artifact gate. It failed before the
remaining gates because a Cargo-backed metadata command created `target/` inside
the scratch source export and invalidated the exact-overlay assertion.

This is a genuinely rescoped evidence-disposition and remaining-qualification
issue. It does not repeat #672's builds or relabel #672 successful. It owns only:

1. classification and immutability proof of the generated scratch `target/`;
2. tracked-source and exact three-overlay proof on the preserved export;
3. the previously unexecuted dependency, resource/PCM, SDK, and three-browser
   gates with an external Cargo target fixed before every remaining Cargo user;
4. independent pre-pin review, an exact later promotion amendment, delivery,
   synchronization, and cleanup.

The accepted product source checkpoint remains
`8708c9b998a484d49ccb17a803e79540ca13fcd6`. Its accepted source hashes are:

- `crates/effect-contract/src/lib.rs`:
  `be709c2293b108feccfe14b0049c08e32d09ce61188a865dca59fa6cee185f98`;
- `crates/effect-package/src/wire.rs`:
  `9a4e833512ab8f70bf4804fc149bfe21e2cb568eb6f53a707c212529e7e66818`.

The preserved candidate is `/tmp/issue672-attempt3-candidate-artifact`, with
exact six-file census and hashes:

- `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`
  `miso-engine-v1-abi-layout.json`;
- `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`
  `miso-engine-v1-audio-worklet-host.d.ts`;
- `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`
  `miso-engine-v1-audio-worklet-host.js`;
- `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`
  `miso-engine-v1-audio-worklet.js`;
- `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`
  `miso-engine-v1-audio-worklet.simd128.wasm`;
- `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`
  `miso-engine-v1-parameter-metadata.json`.

The preserved pristine and overlaid sources are respectively
`/tmp/issue672-attempt2-candidate-pristine` and
`/tmp/issue672-attempt2-candidate-source`. The pristine export verified all
12,195 tracked paths against the source commit. The overlaid source contains the
three intended lineage files plus the generated `target/` subtree. Preserve all
#672 paths and evidence unchanged; do not clean, rebuild, or reconstruct them.

## Ownership and frozen scope

Sol HIGH coordinates the brief, Git checkpoints, artifact decision, GitHub
synchronization, promotion, PR, merge, and cleanup. Luna HIGH
`/root/issue583_luna_impl` is the sole executor. Astra LOW performs scope,
pre-pin, exact-head/current-main, integration, and delivery review.

Product source, tests, Cargo manifests/lock, toolchain/config, ABI, JS/TS,
metadata, resource and PCM expectations, browser rows, scripts, workflows, and
fixtures are frozen. Repository pins and lineage are frozen until PRE-PIN PASS
and a separately reviewed promotion amendment. No benchmark, timing, compiler
dump, optimization, DSP change, new product harness, or matrix expansion belongs
here. Git must contain no Wasm, Cargo target, `.ll`, `.s`, object/archive, raw
compiler stream, browser payload, or temporary qualification record.

## Attempt 1 qualification scope

Fresh paths are:

- `/tmp/issue678-candidate-source`;
- `/tmp/issue678-evidence`;
- `/tmp/issue678-target`;
- `/tmp/issue678-verifier-control`.

Require all four absent including dangling symlinks, the feature clean at its
pushed authorization head, live `origin/main` exactly
`df0b9b93636de36a7143da15b83444f280b65e6b`, and no process owning the named
source, target, artifact, dependency, browser, or evidence resources. Unrelated
repository activity does not compete. Record head/upstream/main/merge-base,
tool versions, relevant environment values, the two accepted source hashes,
candidate six-file hashes, and complete pre-run censuses of all preserved #672
paths. Exercise the capture wrapper with harmless status-0 and status-1 controls.

Before any dependency install or gate, require the retained frozen verifier
`/tmp/issue672-attempt2-export-verifier.py` to have SHA-256
`2bd12b45cc916faacbb75cda3ae7df48d228022e121da5288f81e46b566f049c`.
Run its already-reviewed self-test with exactly
`TMPDIR=/tmp/issue678-verifier-control python3 -B /tmp/issue672-attempt2-export-verifier.py --self-test`,
then verify the preserved pristine export in exact mode against `8708c9b9`.

Require the preserved candidate source's `target` to be an ordinary non-symlink
directory. Record its deterministic content identity twice: a sorted
path/type/mode/size census and the SHA-256 of a streamed tar made with sorted
names, normalized mtime/owner/group, and numeric ownership. Retain only the
census and digest, never the tar stream or target payload. Use exactly
`LC_ALL=C find target -printf '%y %m %s %p %l\n' | LC_ALL=C sort` from the
preserved candidate source for the census, and exactly
`bash -o pipefail -c 'tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -C "$1" -cf - target | sha256sum' target-census /tmp/issue672-attempt2-candidate-source`
for the streamed identity; require both pipeline statuses 0. Create the fresh
candidate source by running exactly once:

`rsync -a --exclude=/target/ /tmp/issue672-attempt2-candidate-source/ /tmp/issue678-candidate-source/`

This copy deliberately excludes only the classified generated target. Any other
extra path is copied and must make verification fail. Create empty ordinary
`sdk/node_modules` and `hosts/host-web/qualification/node_modules` directories,
run the frozen verifier in overlay mode against `8708c9b9`, and require PASS for
all 12,195 tracked paths with only its three frozen overlay files and two named
dependency roots allowed. Then remove those two still-empty directories and
require them absent before install.

Independently require the pin to equal the expected digest plus LF. Parse both
results documents and require that replacing only pristine `candidateCommit`
with `8708c9b9...` and pristine `wasmSha256` with `93108e94...` produces exact
candidate semantic equality, including every browser row. Require the candidate
matrix bytes to equal the pristine bytes after replacing only that one lineage
sentence, and run exactly
`node hosts/host-web/qualification/generate-matrix.mjs --check` with status 0.
Recompute the preserved target census/digest and require exact equality.

After those checks pass, export
`CARGO_TARGET_DIR=/tmp/issue678-target` before the first remaining command and
keep it fixed for the whole attempt. Install locked dependencies once from the
fresh candidate source:

1. `npm --prefix sdk ci --no-audit --no-fund --prefer-offline`;
2. `npm --prefix hosts/host-web/qualification ci --ignore-scripts --no-audit --no-fund`;
3. `(cd hosts/host-web/qualification && npx playwright install chromium firefox webkit)`.

Require package manifests/locks unchanged, Playwright package version `1.62.1`,
all three browser executables present, and the preserved target census unchanged.
Then run each previously unexecuted command exactly once, stopping on the first
failure without correction or retry:

1. `python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue672-attempt3-candidate-artifact`;
2. `bash scripts/test-web-audioworklet.sh`;
3. `python3 -B scripts/check-sdk-deletions.py`;
4. `python3 -B scripts/check-sdk-deletions.py --self-test`;
5. `bash scripts/check-sdk-types.sh`;
6. `bash scripts/check-sdk-headless.sh /tmp/issue672-attempt3-candidate-artifact`;
7. `bash scripts/sdk-package.sh check /tmp/issue672-attempt3-candidate-artifact`;
8. `npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue672-attempt3-candidate-artifact --browser all --check-matrix --self-test-mutations`.

`scripts/check-web-audioworklet.sh` is inherited from #672's successful static
gate and is not rerun. `sdk-package.sh check` owns its generated SDK check.
Chromium, Firefox, and WebKit must each execute and pass every existing gate and
mutation; `--record-matrix` is forbidden. After each command require the feature
repository clean, all tracked/overlay checks unchanged, candidate six-file hashes
unchanged, and the preserved target census unchanged. At the end, require the
fresh external Cargo target exists only outside the source export and retain a
compact census/hash conclusion; do not retain or commit its payload.

Final evidence is a compact command/status/hash record in `/tmp/issue678-evidence`.
Do not create a recursive evidence ledger or copy compiler/browser/generated
payloads into Git. The frozen verifier must pass one final time on the fresh
source with only the three tracked overlays and two dependency roots; require the
source-local `target/` absent, the external target outside the source, and the
preserved #672 target census unchanged. Astra LOW then performs PRE-PIN review.
A failure stops the attempt and earns no missing-gate credit.

## Promotion and delivery boundary

PRE-PIN PASS alone authorizes no repository edit. Root must append and push an
exact promotion/post-pin amendment, synchronize #678/#559/#560, and obtain fresh
Astra LOW scope PASS. That amendment may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`;
- `hosts/host-web/qualification/results.json`, only `candidateCommit` and
  `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, only regenerated lineage;
- this issue record and concise tracker rows.

The post-pin stage runs one ordinary no-bypass build in fresh external paths and
requires all six files byte-identical to the qualified candidate. It does not
repeat browser qualification. Astra LOW then reviews the exact feature head and
current main. Root opens one PR, waits for required `qualification` success,
performs guarded head/base review, merges, verifies post-main `qualification`,
synchronizes #669/#670/#678/#559/#560, and removes only clean delivered
worktrees. Preserve failed #672 evidence until delivery is complete.

## Acceptance

- Preserved candidate and source identities are independently reverified without
  another build.
- The generated source-local target subtree is fully classified and unchanged;
  every remaining Cargo consumer uses the fresh external target.
- Resource/PCM, SDK/package, and all three browser gates pass once.
- Astra LOW returns PRE-PIN PASS before any repository promotion.
- The post-pin ordinary build byte-matches all six qualified candidate files.
- Required PR and post-main qualification succeed on immutable reviewed heads.
- Git contains only compact decisions and the three intended pin/lineage edits,
  with no compiler or generated artifact payload.
