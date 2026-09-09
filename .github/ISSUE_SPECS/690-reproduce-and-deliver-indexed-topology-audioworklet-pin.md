# Reproduce and deliver the indexed-topology AudioWorklet pin

## Authority and outcome

Parent implementation: #685. Exhausted predecessors: #687 and #688.
Coordination: #559 and #560. Audit: #349 CP1.

The owner has explicitly superseded #688's no-successor boundary and directed
the lane to rescope and continue through all audit fixes. This successor owns
only the missing ordinary post-pin reproduction, final review, and delivery of
the already source-qualified indexed-topology change.

Use successor branch `codex/reproduce-indexed-topology-artifact`, based on
preserved delivery lineage `f4703fe6330ca9453ec31540ab25ebb26314dbda`. It contains product commit
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`, exact three-file pin checkpoint
`2cc6fff5cc21bcca96ebeb106fa3827b47f043fd`, and the preserved decision
records. The pin names candidate digest
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`
and qualification results name the full product commit.

The qualified six-file byte authority remains
`/tmp/issue687-stage2-artifact`:

```text
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919 miso-engine-v1-abi-layout.json
445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf miso-engine-v1-audio-worklet-host.d.ts
21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb miso-engine-v1-audio-worklet.js
31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b miso-engine-v1-audio-worklet.simd128.wasm
6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d miso-engine-v1-parameter-metadata.json
```

Do not mutate any predecessor scratch path or record. They supply byte authority
and candid history only; they receive no retroactive attempt credit.

## Attempt 1 scope

After Astra LOW passes this exact pushed scope, one Luna HIGH executor owns these
fresh paths, which must first be absent including symlinks:

```text
/tmp/issue690-artifact
/tmp/issue690-target
/tmp/issue690-evidence
```

At exact clean successor head, run the ordinary builder once:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN CARGO_TARGET_DIR=/tmp/issue690-target bash scripts/build-web-audioworklet.sh /tmp/issue690-artifact
```

The executor must launch this as a persistent unified command session. If the
initial call yields a session ID, poll that same handle with `write_stdin` until
it reports an exit code. Do not interrupt, restart, or declare failure because a
poll times out, output is quiet, or more than 60/120 seconds elapse. No shell
`timeout` is allowed. Failure means the persistent command itself exits
nonzero, its session disappears without a durable exit result, branch identity
changes, or an explicit gate mismatch occurs.

Before dispatch, observe all three paths absent in memory, exclusively create
the evidence directory, and durably record the preceding absence observation
and successful exclusive creation. Every record must be non-overwriting,
flushed, synchronized, read back, and covered by one terminal self-excluding
manifest. Keep start time, final exit status, complete stdout, and complete
stderr only under `/tmp/issue690-evidence`. On status zero, require exactly six
ordinary output files and byte-for-byte equality with the authority above. Then
run these gates once in order, using persistent sessions for any command that
yields:

```text
CARGO_TARGET_DIR=/tmp/issue690-target bash scripts/check-web-audioworklet.sh /tmp/issue690-artifact
CARGO_TARGET_DIR=/tmp/issue690-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue690-artifact
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

The worktree must remain clean and source-local `target/` absent. Persist
concise statuses and the final six-file census under the evidence path. Commit
no binary, generated SDK/Wasm artifact, compiler stream, target, browser
dependency, or evidence capture. No `.ll` or `.s` capture is requested or
permitted.

Astra LOW adversarially reviews the exact branch, persistent-session result,
six-file byte equality, ordered gates, worktree cleanliness, and evidence
limitations. PASS authorizes a PR from the successor branch. The PR must receive
required `qualification`; after merge, verify post-main `qualification`,
synchronize and close #685 and this issue, update #349/#559/#560 accounting, and
remove only clean delivered worktrees. Preserve failed predecessor worktrees and
scratch evidence.

If three attempts fail, follow AGENTS.md: preserve evidence, rebrief the bounded
failure cause, and restart the workflow under the owner's instruction to finish
the audit. Do not weaken a gate or count an unmerged checkpoint as delivery.

## Attempt-1 verdict — FAIL

Astra LOW's controlling review records a procedural and evidence failure. The
branch advanced to the execution-identity correction `5114dd2c`, and the first
brief did not explicitly require creation of the builder's empty artifact
directory. A concurrent Luna execution then started after that scope failure.
The coordinator stopped the unauthorized process group. Its persistent session
returned actual status 143, consistent with SIGTERM; the sender and cause are
unproved.

Complete stderr records successful host-web Wasm compilation without a compiler
diagnostic failure. Four matching files exist; ABI-layout and parameter-metadata
JSON are absent. No later gate ran. Attempt 1 is consumed and its three paths
remain preserved. Key SHA-256 identities are:

```text
bd64cf95e7162df92bb31ad28d266ad6a97dfc26dda10a3af28b735c68ac5aa8  00-preflight.json
1448b99c2340b09004f3b6cb792f30da6d8d1268cdff6406fc3ec12480c34eeb  01-path-creation.json
b35ffd0ab935fe5bcd67b29bdb92729ce9776f975a8bc3182fd01798bdfaa03d  02-builder.json
0db705ac70725a1282d58a3eeb0668f247bdc5977ce6837d635cd11baaa24e1e  02-builder.stderr
c54a61a885866aecf16609c17fb1232a768c0bcf3b297c8fb904ec336de02164  99-terminal-manifest.json
```

## Attempt 2: Luna-owned persistent reproduction

One named Luna HIGH executor owns artifact qualification and the long-lived
commands. The root coordinator owns checkpoints, synchronization, and review
handoffs but performs no attempt-2 execution.
Attempt 2 owns only these new paths, initially absent including dangling
symlinks:

```text
/tmp/issue690-attempt2-artifact
/tmp/issue690-attempt2-target
/tmp/issue690-attempt2-evidence
```

Fresh Astra LOW SCOPE PASS is required against the exact clean pushed rebrief,
matching GitHub/tracker bodies, current main and branch identities, preserved
attempt-1 records, six authority hashes, and the three absent paths. No
attempt-2 path may be created before that PASS is recorded and pushed.

The named Luna executor must then recheck the same identities and absence of
relevant processes, exclusively create the evidence directory, and durably
record the preceding three-path absence plus successful creation. Luna next
exclusively creates the artifact path as an empty ordinary directory, verifies
it is not a symlink and has no entries, and records that result. Leave the target
absent for Cargo.

Run this frozen sequence once:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN CARGO_TARGET_DIR=/tmp/issue690-attempt2-target bash scripts/build-web-audioworklet.sh /tmp/issue690-attempt2-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt2-target bash scripts/check-web-audioworklet.sh /tmp/issue690-attempt2-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt2-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue690-attempt2-artifact
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

The named Luna executor launches each potentially long command directly with
`exec_command`. When it returns a session ID, Luna records the ID and polls that
same handle with `write_stdin` until an exit code is returned. A poll timeout
triggers another poll of the same handle. Do not transfer or interrupt the
handle, use a shell timeout, start a monitor process, retry a command, or run
another command while the handle is live.

Redirect complete stdout/stderr to the attempt-2 evidence path and record actual
start, finish, exit status, exact argv/environment/cwd/head, worktree porcelain,
and artifact census. Require status zero, exactly six ordinary files
byte-identical to the frozen authority, four later status-zero gates, clean Git,
and absent source-local `target/`. Preserve non-overwriting synchronized records
and one terminal self-excluding manifest for Astra LOW review. No predecessor
path, tracked file, compiler capture, generated artifact, or pin may change.

## Attempt-2 scope review — superseded without execution

Astra LOW returned **SCOPE PASS** at exact clean pushed issue `0453b71b`,
tracker `e7053db4`, and unchanged main for a root-owned execution brief. That
executor assignment conflicts with the owner's required Luna HIGH routing. No
fresh path was created and no command ran, so attempt 2 remains unconsumed. This
amendment supersedes that authorization. A fresh Astra LOW SCOPE PASS against
the exact clean pushed Luna-owned brief is required and must be recorded and
pushed before any attempt-2 path is created.

## Attempt-2 Luna scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean pushed issue
`23fbe24e1bb975719624b39a59bf0cbc813f4038`, tracker
`c325ae774dd95765617c437f05bbaf27be364039`, and unchanged main
`e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`. GitHub parity, lineage, the
six authority hashes, the preserved attempt-1 record hashes, the three
promotion-file identities, clean worktrees, two-slot boundary, and absence of
the three attempt-2 paths and relevant processes all passed. The explicitly
named Luna HIGH executor is `/root/issue583_luna_impl`. After this PASS record is
pushed and synchronized, that executor alone may perform the frozen attempt-2
sequence once. Stop at the first failure and preserve every record. Root retains
coordination only; this grants no retroactive credit, promotion, or merge.

## Attempt-2 Luna-owned scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean pushed issue `23fbe24e`,
tracker `c325ae77`, and unchanged main. GitHub parity held; all three fresh paths
were absent, no relevant process was active, and predecessor/authority hashes
matched. One root-named Luna HIGH executor may create the evidence and empty
artifact directories, leave target creation to Cargo, and run the five frozen
commands through its own directly polled persistent handles. No transfer,
monitor, timeout, interruption, retry, tracked output, or root execution is
authorized. Final Astra LOW evidence review remains required.

## Attempt-2 evidence verdict — FAIL

Astra LOW returned **EVIDENCE FAIL** at exact clean pushed feature `4ac1fd01`,
tracker `cfa01b52`, and unchanged main `e4dfe353`. Initial identity and hash
checks passed, and Luna created the evidence directory followed by the empty
artifact directory. The sole builder session request then used nonexistent cwd
`/home/bl/engine-cp1-indexed-topo-reproduction`; process creation failed before
the builder ran. Recorded `127` is a failure sentinel, not a builder exit status.
No later gate ran, the artifact directory is empty, and external plus
source-local targets are absent.

Preserve `/tmp/issue690-attempt2-{evidence,artifact}` unchanged. Its evidence
directory is mode `0555`; 13 ordinary records are mode `0444`; all 12 manifest
rows match. `SHA256SUMS` is 1,058 bytes with SHA-256
`60068527e834c654fb26060688180167145e7db445b325db117c3cf2742112a7`.
The three false `absence_before_creation` values do not independently prove
prior absence without the producer transcript, and continuous exclusivity or
predecessor immutability is not independently proved. Attempt 2 is consumed and
earns no qualification credit.

## Attempt 3: canonical-cwd Luna reproduction

Attempt 3 is the final attempt and owns only these fresh paths, initially absent
including dangling symlinks:

```text
/tmp/issue690-attempt3-artifact
/tmp/issue690-attempt3-target
/tmp/issue690-attempt3-evidence
```

One explicitly named Luna HIGH executor, `/root/issue583_luna_impl`, owns
execution. Fresh Astra LOW SCOPE PASS is required against the exact clean pushed
rebrief, matching GitHub/tracker bodies, current main and branch identities,
both preserved failed-attempt namespaces, the six authority hashes, and absence
of the three attempt-3 paths and relevant processes. Astra's PASS against this
exact pushed head is execution authority; do not append another authorization
commit before Luna's preflight.

Before creating any path, Luna must record and require both `pwd -P` and
`git rev-parse --show-toplevel` to equal exactly
`/home/bl/misofm/engine-cp1-indexed-topo-reproduction`; recheck clean exact
HEAD/upstream, tracker identity, all three paths absent in memory, and no
relevant process. Every tool call must use that literal absolute path as its
`workdir`. Luna then exclusively creates the evidence directory first and
durably records the preceding validation, absence, and creation. Luna next
exclusively creates the artifact path as an empty ordinary non-symlink directory
and records that fact. Leave target absent for Cargo.

Run these five commands exactly once, in order:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN CARGO_TARGET_DIR=/tmp/issue690-attempt3-target bash scripts/build-web-audioworklet.sh /tmp/issue690-attempt3-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt3-target bash scripts/check-web-audioworklet.sh /tmp/issue690-attempt3-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt3-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue690-attempt3-artifact
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Launch each potentially long command directly through a unified persistent
session, record its session ID, and poll only that handle until it returns an
actual exit code. A poll timeout triggers another poll of the same handle. Stop
on the first failure. Do not transfer or interrupt a handle, use a shell timeout,
start a monitor, retry, or run another command while a handle is live.

Redirect complete streams into the fresh evidence directory and durably record
exact argv/environment/cwd/head/start/finish/actual status. Require status zero,
exactly six ordinary artifact files with the frozen SHA-256 values and byte
equality to `/tmp/issue687-stage2-artifact`, four later status-zero gates, clean
Git, and absent source-local `target/`. Preserve non-overwriting synchronized
read-back records and one terminal self-excluding manifest for Astra LOW review.
Do not mutate any predecessor path, tracked file, compiler capture, generated
predecessor artifact, or pin. No timing, improvement, or budget claim is in
scope. A third failure hard-stops this issue under the three-attempt rule.

## Attempt-3 evidence verdict — FAIL and hard stop

Astra LOW returned **EVIDENCE/PROCEDURAL FAIL** at exact clean pushed feature
`dd8a93ca`, tracker `c43b36b8`, and unchanged main `e4dfe353`. The named Luna
reported a correct read-only preflight followed by a failed evidence-creation
tool request whose workdir mistakenly contained `indexed-topology` rather than
`indexed-topo`; it reported no created namespace and no retry. Subsequent
read-only review contradicted that account: the fresh evidence, artifact, and
Cargo target directories existed, with records naming the same executor and
correct worktree, a builder command/start, and compiler stderr. No terminal
status or manifest existed and no relevant process was observed. The record
cannot establish who launched the builder, how many invocations occurred, or
its terminal result.

At the final read-only snapshot, the artifact directory contained all six
ordinary files byte-identical to `/tmp/issue687-stage2-artifact`, but no later
gate or complete synchronized evidence record existed. That technical snapshot
does not cure the procedural or evidence failure and earns no artifact,
qualification, promotion, performance, or delivery credit. Preserve all
`/tmp/issue690-attempt3-*` paths and every executor transcript unchanged. The
checked predecessor identities remained unchanged, without a claim of
continuous exclusivity.

Attempts 1, 2, and 3 are consumed. Issue #690 is hard-stopped without retry,
continuation gate, promotion, merge, or disguised fourth attempt. #685 remains
the sole inherited partial.

The proposed fresh canonical task `/root/issue690_luna_attempt3` completed only
a read-only readiness check and created no path. The available preserved Luna
HIGH executor above is bound to final execution; this identity choice changes
no command, path, gate, or attempt state.

## Owner delivery reconciliation

The earlier #690 hard-stop snapshot was incomplete. The five preserved status
files each contain `0`; these are recorded status values, not authenticated exit
results or success evidence. The preserved intervals are builder
11:33:18–11:34:53 UTC, gate 1 11:35:02–11:36:07, gate 2
11:36:14–11:37:08, gate 3 11:37:17–11:37:31, and gate 4 11:37:38–11:38:15.

Hard-stop commit `ceafeda5` was recorded at 11:35:24 and GitHub close at
11:35:54. Gate 1 therefore finished, and gates 2–4 started and finished, after
the hard stop and closure. Later records omit the cwd and head for those gates,
and the branch advanced during gate 1. No manifest, persistent session IDs or
tool receipts, or attributable invocation proof exists. Preserve these
limitations and every evidence path unchanged.

The six current attempt-3 artifact files are byte-equal to the authority at
`/tmp/issue687-stage2-artifact`, with the authority hashes listed above in this
spec. This is current identity only. The source files equal product
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`; the three pin/promotion files
equal `2cc6fff5cc21bcca96ebeb106fa3827b47f043fd`.

#688's independently verified limited disposition is
`fe263bf681dca977b924c63683e403ee2d0f48f3d960d902a3e5aa429379dfc5`, with
terminal manifest `f5f42e259aaf6580977321436f6e5f6a796d83b83c12357d792e61003df7b1b9`;
all 24 referenced records were verified. This supports technical applicability
with provenance limits and does not declare the predecessor protocol satisfied.

Attempts 1, 2, and 3 remain failed, consumed, and hard-stopped with no
retroactive credit. Owner-directed #692 now owns documentation/evidence
reconciliation and delivery with NO rerun. It authorizes no product or pin
change, terminal-manifest fabrication, timing, allocation, performance, or
sound-quality claim.
