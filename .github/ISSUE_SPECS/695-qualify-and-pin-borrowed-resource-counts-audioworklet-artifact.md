# Qualify and pin the borrowed-resource-counts AudioWorklet artifact

## Authority and outcome

Parent: #560/#559/#349 CP1. Source peer: #694. This is lane B's second and
final active slot; CP1 remains partial. The source review descendant is
`7ba374eb21abd364fdf8c51edd7baebcb4beb08d`, with product source checkpoint
`60ec5cb314e20e018e515927041f6dbe5deb10cf`, main
`6d217d30478226872fb4e5302b98d967c04dd96b`, and tracker
`c63edd89934571d04d10caf796ecc535aaf26c9d`.

Sol HIGH coordinates. Astra XHIGH owns scoping and every verification assignment.
One later explicitly named Luna HIGH executor performs the mechanical artifact
and pin work. Astra HIGH is reserved for delicate audio/DSP work; none is
authorized here. Lane B alone owns AudioWorklet qualification and pinning.

This issue qualifies the artifact generated from #694's borrowed-resource-counts
source and promotes its identity only after the evidence gates below. It does
not alter product behavior, fixtures, tests, scripts, workflows, dependencies,
lockfiles, browser expectations, or any predecessor path.

## Source and path ownership

The branch is `codex/qualify-resource-input-counts-artifact` and the worktree is
`/home/bl/misofm/engine-cp1-resource-input-counts-artifact`. The only allowed tracked
paths are this numbered spec, the pin file
`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, only the
`candidateCommit` and `wasmSha256` fields in
`hosts/host-web/qualification/results.json`, and regenerated lineage in
`hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`.

No other source, fixture, test, script, workflow, dependency, lockfile, browser
row, expectation, generated payload, or artifact path may change. The source
estimate identity is SHA-256
`1396b98604641441626ccc5b85d7db0eb7adea23fbab5339fadc127b2367e50d`.

Preserve every predecessor worktree, branch, target, artifact, and evidence
path, including #694's `/tmp/issue694-evidence`,
`/tmp/issue694-baseline-target`, `/tmp/issue694-candidate-target`, and
`/tmp/issue694-astra-review-target`.

Fresh roots for this issue must be absent, including dangling symlinks, before
execution:

```text
/tmp/cp1-resource-input-counts-a1-evidence
/tmp/cp1-resource-input-counts-a1-probe-output
/tmp/cp1-resource-input-counts-a1-artifact
/tmp/cp1-resource-input-counts-a1-target
/tmp/cp1-resource-input-counts-a1-hermetic-target
/tmp/cp1-resource-input-counts-a1-tmp
```

## Phase 1: repin identity probe

After fresh Astra XHIGH exact-head SCOPE PASS against the pushed issue, tracker,
main, source identity, ownership, preserved predecessors, and all absent roots,
one named Luna HIGH executor may run exactly one repin probe from the peer source
worktree. The probe is identity evidence only and pauses for Astra review.

Before any creation, the named executor must observe all six execution roots
above absent, including dangling symlinks. Exclusively create the evidence
directory and durably record the preceding absence observations. Then create
the designated `...-a1-tmp` path as an ordinary directory and the probe-output
path as an empty ordinary directory, keeping evidence, tmp, and probe creation
distinct. Leave the artifact directory, ordinary Cargo target, and hermetic
Cargo target absent for their commands to create. No target or artifact
creation beyond the probe command's own target behavior is authorized. Retain
the complete lifecycle record, then dispatch exactly one probe:

```text
(cd /home/bl/misofm/engine-cp1-resource-input-counts-artifact && TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/cp1-resource-input-counts-a1-probe-output)
```

Require actual exit status 0, exactly one lowercase 64-hex digest followed by
LF on stdout, an empty output directory, and unchanged tracked bytes. The
current delivered pin expected by the probe is
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b` plus LF.
Do not infer qualification, source equivalence, or promotion from a matching
probe. Stop for Astra XHIGH probe evidence review before Phase 2.

### Phase 1 evidence record

Astra PROBE EVIDENCE PASS reviewed the Phase 1 probe at clean head
`a0099d2b97995b7978c6ff7614cef46548595f26`. The probe ran once from that head,
finished before the later docs-only clean/pushed head
`6533ac339884c84d9001d86822c7b18aa49c0a0b`, exited zero, produced an empty
output directory, and observed
`6745de399c56e322303e2da55d69a5cbd538e075f0fd480c6620b1896d566645`. This
record authorizes the bounded Phase 2 overlay only; it does not qualify an
ordinary artifact or authorize a performance or sound-quality claim.

The original receipts prove two actual `write_stdin` polls; the saved `polls=3`
label is imprecise because it counts the initial launch result plus those two
polls. The
six-path absence/creation record was written retrospectively, with independent
Astra scope authority for the pre-setup absence observation. The probe's
actual environment authority is the captured `pre-probe-environment` record
plus the three explicit assignments in the actual command; the later
`probe-environment` file is reconstructed and is not authority. The post-probe
documentation-only head drift is recorded in the transcript: the probe stayed
at `a0099d2b97995b7978c6ff7614cef46548595f26`, while the later docs checkpoint
advanced to `6533ac339884c84d9001d86822c7b18aa49c0a0b`. Complete transcript and
receipt are preserved outside Git under
`/tmp/cp1-resource-input-counts-a1-evidence/`; its finite self-excluding
manifest has SHA-256
`e3bca0c919a8c2222beab5463ab998d34f96f909123dd685570e685436bdadae`. No failed attempt was
consumed.

## Phase 2: provisional lineage and ordinary qualification

After Astra accepts the probe record, make only the bounded provisional overlay.
Set `candidateCommit` to the full product
`60ec5cb314e20e018e515927041f6dbe5deb10cf` and `wasmSha256` to the observed
probe digest in `results.json`; change the pin only when that digest differs
from the current pin; regenerate only the matrix lineage. Checkpoint the
qualification-pending overlay candidly, synchronize it, and obtain fresh Astra
XHIGH exact-head SCOPE PASS before execution.

Run one ordinary builder followed by these gates exactly once and in order from
the artifact worktree. Every command explicitly unsets
`MISO_ENGINE_WEB_AUDIOWORKLET_REPIN` and carries `TMPDIR` and
`CARGO_TARGET_DIR`; the hermetic gate has its separate target, and all sessions
are persistent unified sessions polled to an actual result:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target bash scripts/build-web-audioworklet.sh /tmp/cp1-resource-input-counts-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target bash scripts/check-web-audioworklet.sh /tmp/cp1-resource-input-counts-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/cp1-resource-input-counts-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-hermetic-target bash scripts/test-web-audioworklet.sh
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target npm_config_cache=/tmp/cp1-resource-input-counts-a1-tmp/npm-cache npm --prefix sdk ci --ignore-scripts
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target bash scripts/sdk-package.sh check /tmp/cp1-resource-input-counts-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target npm_config_cache=/tmp/cp1-resource-input-counts-a1-tmp/npm-cache npm --prefix hosts/host-web/qualification ci --ignore-scripts
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target npm_config_cache=/tmp/cp1-resource-input-counts-a1-tmp/npm-cache npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/cp1-resource-input-counts-a1-artifact --browser all --check-matrix --self-test-mutations
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target node hosts/host-web/qualification/generate-matrix.mjs --check
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target git diff --check
```

Only newly created ignored `sdk/node_modules`, `sdk/dist`, and qualification
`node_modules` may appear in the peer worktree. Missing pinned browser or system
prerequisites stop the attempt without changing floors or installing unrelated
system tooling.

The matrix lineage regeneration itself is the only permitted matrix write and
must occur before this sequence with the same explicit environment:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resource-input-counts-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-resource-input-counts-a1-target node hosts/host-web/qualification/generate-matrix.mjs
```

Immediately after the builder, require exactly six ordinary artifact files. The
Wasm digest must equal the probe and resulting pin. The five non-Wasm authority
hashes are:

```text
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919  miso-engine-v1-abi-layout.json
445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf  miso-engine-v1-audio-worklet-host.d.ts
21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a  miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb  miso-engine-v1-audio-worklet.js
6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d  miso-engine-v1-parameter-metadata.json
```

Any unexpected non-Wasm, resource, PCM, SDK, matrix, browser, or mutation drift
fails the attempt. If Wasm is unchanged, require full six-file equality with the
authority; if Wasm changes, qualify the new six-file set without any size,
timing, allocation, performance, or sound-quality claim.

## Evidence, attempts, and delivery

Evidence must record complete actual argv, environment, cwd, head, dirty state,
start/finish times, stdout, stderr, actual result, and persistent session receipt
for every command. Labels cannot abbreviate these fields. Record path absence and
path creation as separate facts. Use one finite self-excluding checksum manifest;
a status file containing `0` is not authenticated execution evidence.

Phase 1 and Phase 2 are one attempt. A phase pause for Astra review does not
consume an attempt. A failure stops immediately; a later attempt requires fresh
named roots and fresh review. Three failed attempts hard-stop the issue without
weakened gates or a disguised fourth attempt. #687/#688/#690 supply byte
authorities and historical provenance only; they grant no execution credit.

After final Astra XHIGH evidence PASS, deliver one PR with #694, required
aggregate PR qualification, guarded live-head/base merge, post-main aggregate
qualification, GitHub synchronization and closure, #559/#560/#349 accounting,
and eligible clean delivered-worktree cleanup. No artifact, pin, or source claim
is delivered before those remote steps. No performance or sound-quality claim is
authorized.
