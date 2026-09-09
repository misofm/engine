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

## Attempt-1 verdict — PROCEDURAL FAIL

Astra LOW returned SCOPE FAIL before execution because the branch had advanced
to the sound execution-identity correction `5114dd2c` and the brief did not
explicitly create the builder's required empty artifact directory. All three
fresh paths were absent at that review.

A concurrent Luna then created the attempt-1 evidence, artifact, and target
paths and started the ordinary builder after SCOPE FAIL. The coordinator stopped
the unauthorized process group. Its persistent session terminated with actual
status 143, produced four of six candidate files, and ran no later gate. Attempt
1 is consumed with no qualification or delivery credit. Preserve all three
paths and records. Key SHA-256 identities are:

```text
bd64cf95e7162df92bb31ad28d266ad6a97dfc26dda10a3af28b735c68ac5aa8  00-preflight.json
1448b99c2340b09004f3b6cb792f30da6d8d1268cdff6406fc3ec12480c34eeb  01-path-creation.json
b35ffd0ab935fe5bcd67b29bdb92729ce9776f975a8bc3182fd01798bdfaa03d  02-builder.json
c54a61a885866aecf16609c17fb1232a768c0bcf3b297c8fb904ec336de02164  99-terminal-manifest.json
```

## Attempt 2: corrected fresh reproduction

Attempt 2 owns only these new paths, initially absent including dangling
symlinks:

```text
/tmp/issue690-attempt2-artifact
/tmp/issue690-attempt2-target
/tmp/issue690-attempt2-evidence
```

Fresh Astra LOW SCOPE PASS is required against the exact clean pushed rebrief,
matching GitHub/tracker bodies, current main and branch identities, all preserved
attempt-1 records, and the three absent attempt-2 paths. No attempt-2 path may be
created before that PASS is itself recorded and pushed.

One named Luna HIGH executor may then observe all paths absent in memory,
exclusively create the evidence directory, and durably record the preceding
absence plus successful exclusive creation. It must next exclusively create
`/tmp/issue690-attempt2-artifact` as an empty ordinary directory, verify it is
not a symlink and has no entries, and durably record that result. Leave the
target absent for Cargo to create. Recheck the exact clean upstream-equal branch
and absence of relevant processes immediately before dispatch.

Run the same frozen sequence once with only the namespace changed:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN CARGO_TARGET_DIR=/tmp/issue690-attempt2-target bash scripts/build-web-audioworklet.sh /tmp/issue690-attempt2-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt2-target bash scripts/check-web-audioworklet.sh /tmp/issue690-attempt2-artifact
CARGO_TARGET_DIR=/tmp/issue690-attempt2-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue690-attempt2-artifact
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Every potentially long command uses one persistent unified session and the same
handle is polled until its actual exit result. No fixed timeout, restart, or
retry is allowed. Keep exclusive, complete, synchronized, read-back records and
one terminal self-excluding manifest. Stop permanently at the first nonzero
status or identity/output mismatch. The original six hashes, byte-equality,
clean-worktree, no-source-local-target, no-generated-Git-output, Astra evidence
review, PR/CI/merge/post-main, synchronization, and cleanup requirements remain
unchanged.
