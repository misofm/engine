# Qualify indexed-topology AudioWorklet artifact applicability

GitHub: https://github.com/misofm/engine/issues/687

Parent/delivery peer: #685. Audit parent: #560 CP1. Coordination: #559.
Frozen source predecessor: `c9ccf6acf4030a0b567b5c18cbb0f52126aa16f5`
(product commit `276ffb6097a84088e3b5f4a16892a33bca9e26fb`).
Current delivered AudioWorklet pin:
`93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`,
delivered through #680/PR #682 at main `8999def5`.

## Problem

#685 received hard-final Astra LOW SOURCE/EVIDENCE PASS for its bounded
topological-scheduling implementation. Attempt 1 failed on a pre-existing stale
graph fixture manifest. Attempt 2's technical gates passed, but a concurrent
spec edit failed its final clean-tree assertion. Read-only attempt 3 verified the
documentation-only drift, frozen non-spec hashes, and all 102 preserved status-0
command records without rerunning a gate. `graph-compiler` is in the shipped
`host-web -> host-core -> graph-compiler` dependency closure, so source PASS does
not establish that the currently pinned AudioWorklet artifact represents the
accepted source. Lane B alone must decide applicability and own any later
qualification or pin change before #685 can deliver.

This issue begins with one identity probe. It does not assume drift and does not
reuse, repair, or rerun #644/#680 artifact evidence. All prior failed and retained
state remains preserved.

The earlier artifact brief inherited premature concurrent PASS rows and granted
no execution. This corrected brief relies only on #685's controlling hard-final
PASS at `c9ccf6ac`, with product frozen at `276ffb60`. No stage-1 path exists and
no builder has run.

## Stage 1: one repin-report identity probe

One designated Luna HIGH or XHIGH executor may run the builder once in repin-
report mode from a fresh detached worktree at exact source `c9ccf6ac` after Astra
LOW passes the corrected numbered brief. The Astra PASS record must state literal
reviewed #687 brief and tracker checkpoints. Execution preflight must compare
against those literal checkpoints, exact main
`e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`, source
`c9ccf6acf4030a0b567b5c18cbb0f52126aa16f5`, and product
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`; recording whatever identities happen
to be live is insufficient.

Before creating anything, observe in memory and verify:

- executor identity and UTC start;
- coordinator cwd, exact #687 HEAD/upstream equality, tracker identity, main,
  source/product ancestry and empty coordinator porcelain including untracked
  files;
- `rustc -Vv`, `cargo -V`, literal command and complete relevant environment;
- current delivered pin; and
- absence, including dangling symlinks, of all three predeclared paths:
  `/tmp/issue687-repin-source`, `/tmp/issue687-repin-output`, and
  `/tmp/issue687-repin-evidence`.

Stop before creation on any mismatch or concurrent Cargo/rustc builder activity.
Then perform this persistence order once:

1. Exclusively create `/tmp/issue687-repin-evidence`.
2. Persist the pre-creation observations to `00-preflight.json` using a flushed
   and file-synchronized write, synchronize the evidence directory entry, read
   the file back, and verify its SHA-256 before any source/output path creation.
3. Create `/tmp/issue687-repin-source` once as a detached worktree at exact
   `c9ccf6ac`; require exact HEAD, clean porcelain, and no untracked files.
4. Create `/tmp/issue687-repin-output` once as an empty non-symlink directory,
   persist/read back its creation record, then dispatch from the detached source.

Run exactly once:

```text
(cd /tmp/issue687-repin-source && MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue687-repin-output)
```

Capture the exact argv/environment/cwd/head, start/finish, complete stdout and
stderr, numeric status, detached-source and coordinator postflight porcelain,
output census, delivered pin, and SHA-256/size/mode of every record. Each record
must be written without overwrite, synchronized, read back, and included in a
terminal self-excluding manifest. Stop on nonzero status, anything other than
one lowercase 64-hex digest plus LF on stdout, a nonempty output directory, tree
drift, or changed source/authority. Do not retry, run the ordinary builder,
inspect compiler payloads, retain a Cargo target intentionally, edit a pin or
lineage file, assemble a six-file candidate, run static/resource/hermetic/SDK/
browser gates, or remove evidence before Astra review.

## Decision rule and ownership

If the observed digest equals the delivered pin, Astra LOW decides whether the
existing artifact qualification transfers to #685. If it differs, stage 1 proves
only drift. Candidate assembly, qualification, browser execution, repository pin
or lineage edits, post-pin rebuild, PR, merge, and delivery require a pushed stage
2 amendment and fresh Astra LOW SCOPE PASS.

This issue owns only its numbered spec, compact decision records, concise #559/
#560 coordination rows, and any later explicitly amended lane-B artifact/pin
paths. Stage 1 owns no tracked product, artifact, pin, result, matrix, SDK, host,
compiler, manifest, lockfile, dependency, policy, or workflow edit. Full streams,
Wasm, generated files, Cargo targets, binaries, objects, libraries, compiler
outputs, and raw evidence remain temporary and never enter Git.

No timing, allocation, throughput, percentage, improvement, regression, budget,
or performance claim is authorized. #685 remains the sole implementation issue;
this qualification peer uses the second shared slot. Lane A owns no artifact or
pin action.

## Review and delivery

Astra LOW must pass the exact clean synchronized brief and fresh-path/executor
preconditions before Luna executes stage 1. Astra then adversarially reviews the
preserved output and compact record. The normal three-attempt ceiling applies,
but the single repin-report invocation is consumed once it starts and may not be
rerun under another attempt. A procedural or builder failure is recorded without
repair unless a separately reviewed next stage can use immutable outputs without
rerunning.

After an artifact decision passes, #685 still requires exact-head/current-main PR
readiness, required `qualification`, guarded live head/base review, exact-merge
post-main qualification, GitHub/spec synchronization, closure, and clean removal
only of delivered worktrees. Preserve all failed worktrees, branches, histories,
targets, and temporary evidence named by #559/#560, including #668 and the soft-
clip chain. Do not inspect, rewrite, repair, rerun, or remove them here.

## Controlling stage 1 verdict — EVIDENCE FAIL

The stage-1 invocation is consumed and must never be repeated. Preflight observed
reviewed head `c07dee0e`, but the branch advanced to `5153ce43` at 09:46:13 UTC,
before the recorded invocation began at 09:46:45 UTC. Both invocation identity
records correctly name `5153ce43`; the later drift record incorrectly calls
`c07dee0e` the invocation head. This violated the reviewed stop rule. The
preceding fresh-detached-probe rebrief was committed after the invocation and
cannot reset or authorize another run.

The preserved result is technically coherent but unqualified: status 0, exact
65-byte lowercase digest stdout, empty output directory, and reproducible record
manifest. Observed digest
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`
differs from delivered pin
`93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`.
No stage-2 authority attaches.

## Read-only reconciliation attempt

One Astra LOW review may exclusively create the single new compact record
`/tmp/issue687-repin-evidence/attempt2-reconciliation.json` after proving that
path absent, including as a dangling symlink. It may inspect existing records,
reflogs, commit/tree/path hashes, and diffs only. It must establish the exact
`c07dee0e..5153ce43` chronology and whether every builder/product input—product
source, builder script, manifests, lockfile, toolchain/environment record, and
delivered pin—was identical despite the documentation-only merge. It must state
that the retained records do not independently prove historical immutability or
the exact total invocation count.

Do not modify an existing record, rerun a builder, run Cargo/rustc, assemble a
candidate, inspect compiler payloads, qualify browsers/resources/SDKs, or edit a
pin. A reconciliation PASS may qualify only the limited drift observation and
authorize a separately amended stage-2 scope; any need to reconstruct evidence
or rerun the probe hard-stops this issue.

## Stage 1 concurrent execution — PROCEDURAL FAIL; read-only attribution only

After Astra LOW passed the original `c07dee0e` scope, the branch advanced without
review to `5153ce43`; an actor then invoked the repin-report builder once from
that changed artifact worktree. Preserve every existing record and both
directories. `/tmp/issue687-repin-source` remains absent. No replacement repin
invocation, overwrite, repair, reconstruction of existing evidence, timestamp
change, cleanup, or pin action is authorized. Only the one new reconciliation
record named above may be added.

The preserved records report one builder status 0, empty output directory, one
65-byte lowercase digest line
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`,
and delivered pin `93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531`.
They violate the original reviewed stop rule because authority changed before
dispatch. The preflight names head `c07dee0e`, while the invocation-head/upstream
records name later merge `5153ce43`; later records observe another spec-only
drift. The stricter detached-source protocol was committed afterward and is not
applied retroactively. Stage 1 remains consumed as a procedural failure and the
digest receives no artifact-qualification or pin credit yet.

Attempt 2 owns one read-only attribution review of the existing records and Git
history. Astra LOW may verify hashes, modes, sizes, mtimes, command/status/stream
consistency, the exact ancestry and diff among `c07dee0e`, `5153ce43`,
`c9ccf6ac`, and frozen product `276ffb60`, the absence of non-documentation
source drift, output emptiness, digest format, current pin, and whether one
builder invocation is supported. It may write the named reconciliation file
once with exclusive creation, flush and synchronize it, read it back, record its
hash in the verdict, and make it read-only. It may not execute Cargo, rustc, a
builder, any artifact/SDK/browser gate, or any generated binary; create another
path; modify an existing record; inspect compiler payloads; reconstruct
freshness; or infer sole-executor provenance that the records do not prove.

An attribution PASS may preserve only the observed candidate digest and exact
limitations for a separately amended stage 2 candidate qualification. It cannot
make stage 1 pass, transfer prior artifact qualification, or authorize a pin
edit. A missing/inconsistent record, non-doc source drift, unresolvable head
identity, or evidence of another invocation closes #687 without candidate use.

## Attempt 2 read-only attribution verdict — PASS (limited)

Astra LOW returned **ATTRIBUTION PASS** at exact clean pushed #687 brief
`266317b6e2c46d7ab9100895c561c6fc234b1d02`, tracker
`e9f130a00dc7f54fb110fbfb0c34cc381170a034`, and unchanged main. The
`c07dee0e..5153ce43` merge completed at 09:46:13 UTC before the recorded
09:46:45–09:47:02 invocation. Both invocation identity records name
`5153ce43`; the later drift record's `c07dee0e` invocation claim is wrong. The
merge changed only #685's spec.

All non-spec tracked inputs, including product source, builder, manifests,
`Cargo.lock`, and the delivered pin, are byte-identical across the two heads,
source-qualified `c9ccf6ac`, and product `276ffb60`. All 17 manifest-covered
records match their current hashes, sizes, and modes. The records support one
observed status-0 invocation, one valid 65-byte digest line, and empty output.
They do not prove historical immutability, sole-executor provenance, or the exact
total invocation count. Stage 1 remains FAIL and consumed.

The limited observation establishes candidate digest
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`
for a separately reviewed hard-final candidate qualification only. It grants no
artifact, pin, PR, merge, delivery, or performance credit.

## Hard-final stage 2: scratch candidate qualification

Stage 2 is attempt 3 and the last attempt in #687. Freeze source
`c9ccf6acf4030a0b567b5c18cbb0f52126aa16f5`, product
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`, candidate digest
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`,
and current qualified six-file authority
`/tmp/issue672-attempt3-candidate-artifact`. Preserve the authority read-only and
require its exact hashes:

```text
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919 miso-engine-v1-abi-layout.json
445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf miso-engine-v1-audio-worklet-host.d.ts
21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb miso-engine-v1-audio-worklet.js
93108e9407f4cd343b644e9e821cfd3ca80c3667983a35db7f2e5c3228934531 miso-engine-v1-audio-worklet.simd128.wasm
6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d miso-engine-v1-parameter-metadata.json
```

One designated Luna HIGH or XHIGH executor may act only after Astra LOW passes
the exact pushed amendment. Require these fresh paths absent, including dangling
symlinks, and stop on any mismatch or concurrent Cargo/rustc/builder/browser
qualification process:

```text
/tmp/issue687-stage2-source
/tmp/issue687-stage2-artifact
/tmp/issue687-stage2-evidence
/tmp/issue687-stage2-target
/tmp/issue687-stage2-hermetic-target
```

Observe all preconditions in memory, exclusively create the evidence directory,
durably persist/read back/hash a pre-creation record, then create one detached
source worktree at exact `c9ccf6ac`. Require exact HEAD and clean porcelain.
Within that scratch worktree only:

1. replace the artifact pin with the candidate digest plus LF;
2. replace only `candidateCommit` in
   `hosts/host-web/qualification/results.json` with full product `276ffb60` and
   `wasmSha256` with the candidate digest; and
3. run `node hosts/host-web/qualification/generate-matrix.mjs` once, requiring
   the tracked overlay to contain exactly the pin, those two JSON fields, and
   the matching generated matrix lineage.

Create the artifact directory once, leave both target paths absent for their
commands to create, and run these commands once in order. Persist exact argv,
environment, cwd/head, start/finish, complete streams, numeric status, source
porcelain, and output census after each; stop permanently at the first failure:

```text
CARGO_TARGET_DIR=/tmp/issue687-stage2-target bash scripts/build-web-audioworklet.sh /tmp/issue687-stage2-artifact
bash scripts/check-web-audioworklet.sh /tmp/issue687-stage2-artifact
python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue687-stage2-artifact
CARGO_TARGET_DIR=/tmp/issue687-stage2-hermetic-target bash scripts/test-web-audioworklet.sh
npm --prefix sdk ci --ignore-scripts
bash scripts/sdk-package.sh check /tmp/issue687-stage2-artifact
npm --prefix hosts/host-web/qualification ci --ignore-scripts
npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue687-stage2-artifact --browser all --check-matrix --self-test-mutations
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Immediately after the builder, require exactly the six canonical ordinary files.
The Wasm must have the candidate digest; the other five must be byte-identical to
the frozen authority and retain the hashes above. The static, expected-resource/
native-witness, hermetic, SDK-package, all-browser, matrix, and mutation gates
must pass without retry. Require the final tracked diff to remain exactly the
three scratch lineage overlays and the source-local `target/` to remain absent.

Every new record is non-overwriting, synchronized, read back, and covered by one
terminal self-excluding manifest. Preserve scratch source, artifact, targets,
and records through Astra review. Commit no scratch overlay, artifact, target,
stream, node_modules, binary, or generated candidate. A PASS qualifies the
candidate bytes only; repository promotion and a post-pin ordinary rebuild need
a separately pushed scope amendment and Astra PASS. Failure hard-stops #687
without retry or a disguised fourth attempt.
