# Qualify prepared-effect AudioWorklet artifact applicability

GitHub: https://github.com/misofm/engine/issues/644

Parent/delivery peer: #642. Audit parent: #560 CP1. Coordination: #559. Frozen feature source: `690e05174f57dcea02ddf21ec4b42596c0a88bb1`. Current delivered artifact pin: `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`, originating at `0bb5a820be33a49393386617358ec1db2fe7577d` and qualified/delivered through #627/#628/#630. #587 remains historical supporting provenance. Preserve the delivered #542 and #552/#558 dependency order and do not alter their source, fixtures, pins or evidence.

#642 passed source and release-test qualification, but PR-readiness review found its changed `graph-compiler` production bytes in the shipped `host-web -> host-core -> graph-compiler` dependency closure. This issue owns the bounded artifact applicability decision and any later separately reviewed candidate qualification/pin update. #642 remains the delivery issue and does not create a third slot.

Sol HIGH coordinates scope, checkpoints, GitHub, artifact qualification and pin ownership. Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole executor. Astra LOW reviews scope, evidence, artifact disposition and delivery. No full or compressed compiler streams, `.ll`, `.s`, objects, archives, binaries, `rlib`, `rmeta`, Cargo targets or generated candidates enter Git.

## Stage 1: one identity probe

At exact clean/upstream feature head, record executor/time, cwd/head/upstream/status, toolchain, current pin, literal command/environment and absence of both predeclared paths including dangling symlinks. Create `/tmp/issue644-repin-output` once as an empty non-symlink directory and `/tmp/issue644-repin-evidence` once without overwrite. Then run exactly once:

```text
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue644-repin-output
```

Capture separate complete stdout/stderr and numeric status contemporaneously, postflight tree/output census, current pin, and hashes. Stop on changed preconditions, nonzero status, missing single lowercase 64-hex stdout, nonempty output directory, tree drift or concurrent activity. Do not retry, run the ordinary builder, keep a Cargo target, edit the pin, build a six-file candidate, invoke browser/SDK/static qualification, or delete evidence before Astra review.

If the observed Wasm digest equals the current pin, Astra rules whether the unchanged builder/copied inputs and prior #587 qualification remain applicable. If it differs, stage 1 establishes drift only. Any candidate assembly, static/resource/ABI/native-PCM checks, pin edit, post-pin rebuild, exact six-file comparison, SDK/browser matrix or generated-consumer work requires a pushed scope amendment and fresh Astra PASS. No timing, performance or allocation claim belongs here.

## Objective gates

Stage 1 is attempt 1 and consumes one official builder invocation. Astra must pass the exact clean synchronized brief, source/dependency applicability argument, sole executor and fresh path preconditions before execution. Root commits only compact command/source/toolchain/pin/hash/status/output-census evidence; raw streams and build output stay in `/tmp`. A same-hash PASS permits #642 PR-readiness review after synchronized disposition. Drift requires a bounded stage-2 qualification brief; it does not authorize an automatic repin or weaken prior browser/PCM/ABI gates.


## Stage 1 Astra LOW evidence review — PASS; drift

At clean source/upstream `70899de287c23b70c17b3e41a5b2921801ae8052`, Hypatia ran the
single repin-report command once. Status was 0; stdout was exactly 65 bytes containing candidate
Wasm SHA-256 `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`
plus LF. The output directory remained empty, the repository and delivered pin remained unchanged,
and postflight was clean. The five temporary records have these identities:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `preflight.txt` | 693 | `f76193e8b4b0f4531e8f7c8c21a6ec6204bb3b7f298c6bdf1bdce364ff8bdde8` |
| `stdout.txt` | 65 | `c1f21a0eceb5cb7fb787cf8caec09b159dfeedbde8880d2fcdf91128a9931893` |
| `stderr.txt` | 3,789 | `2007f3f1701b2e9a077f2989563edd4592647cc657b00129c791d4c56b4dddb7` |
| `status.txt` | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| `postflight.txt` | 697 | `4124b56b2dbc8bd716c7c39fd9aa231c50cd0ead108026ae0b32053aece45848` |

The raw `executor=bl` field identifies the OS account; Hypatia's contemporaneous completion supplies
agent attribution. Full streams remain only in `/tmp`. The candidate differs from the delivered
pin, so no prior artifact identity transfers and no PR is authorized.

## Stage 2: scratch candidate qualification

Freeze artifact source `70899de287c23b70c17b3e41a5b2921801ae8052` and expected candidate
digest `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`.
Hypatia is the sole executor. Before work, record exact identities and prove these paths absent,
including dangling symlinks:

- `/tmp/issue644-scratch-source`
- `/tmp/issue644-qualified-output`
- `/tmp/issue644-qualification-evidence`
- `/tmp/issue644-hermetic-target`

Also verify `/tmp/issue627-qualified-output` still contains the delivered six-file authority with
Wasm `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1` and these
non-Wasm hashes: ABI layout `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`,
host declaration `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`, host
JavaScript `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`,
worklet JavaScript `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`,
and parameter metadata `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`.
A mismatch stops without repair.

Create one detached scratch worktree at the frozen source. Change only its artifact pin to the
candidate digest plus LF and record the exact one-file overlay. Create the output and evidence
directories once without overwrite. Run each command once in this order, recording exact argv,
environment, cwd/head, separate streams, numeric status, start/end time and post-command tree state;
stop at the first failure:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN bash scripts/build-web-audioworklet.sh /tmp/issue644-qualified-output
node hosts/host-web/qualification/generate-matrix.mjs
bash scripts/check-web-audioworklet.sh /tmp/issue644-qualified-output
python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue644-qualified-output
CARGO_TARGET_DIR=/tmp/issue644-hermetic-target bash scripts/test-web-audioworklet.sh
npm --prefix sdk ci --ignore-scripts
bash scripts/sdk-package.sh check /tmp/issue644-qualified-output
npm --prefix hosts/host-web/qualification ci --ignore-scripts
npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue644-qualified-output --browser all --check-matrix --self-test-mutations
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Immediately after the builder, require exactly the canonical six filenames, independently hash all
six, require the candidate Wasm digest, and byte-compare all five non-Wasm outputs with the verified
#627 authority. Before the static gate, change only scratch `results.json` fields `candidateCommit`
to the frozen source and `wasmSha256` to the candidate, then regenerate only
`BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Require the scratch diff to contain
exactly the pin, those two JSON lineage values, and matching generated matrix lineage. Browser rows,
Playwright/browser versions, gates, resources, ABI, expected numeric resources, native PCM and all
other inputs remain frozen.

The static gate covers the shipped ABI/export/import/memory/realtime-callgraph/SIMD/metadata and
resource contracts. The resource command must execute its current native witness and mutation set.
The hermetic, SDK, and one all-browser command retain their existing locked behavior. No command may
be retried or replaced. Preserve candidate output, scratch worktree and complete temporary records
until Astra review. Git retains only compact commands, identities, statuses, hashes, six-file census,
non-Wasm comparisons, lineage diff summary, gate totals and browser rows; no full/compressed streams
or compiler payloads.

Stage 2 is not repository promotion. Astra LOW candidate PASS is required before any branch pin,
`results.json`, or matrix edit. Promotion, post-pin rebuild, PR creation and merge remain separately
unauthorized.
