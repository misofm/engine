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

## Attempt-1 scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean pushed brief `a1a32425`,
tracker `7808e2ec`, and unchanged main/source/product. GitHub parity held, all
six preserved hashes matched ordinary files, and all four issue-688 scratch
paths were absent. The successor owns fresh qualification of fixed existing
bytes without rebuilding or inheriting #687 credit. External targets, the
three-file scratch overlay, ordered commands, and permanent stop rule are
coherent.

One Luna HIGH executor may run the frozen attempt-1 sequence exactly once and
preserve the records for Astra LOW evidence review. No rebuild, artifact
mutation, promotion, pin delivery, PR, merge, or predecessor repair is
authorized.

## Attempt-1 evidence verdict — FAIL

Astra LOW returned **EVIDENCE FAIL**. Attempt 1 is consumed. The preserved
technical records are coherent: all nine gates ran once in order with status 0,
the six candidate hashes match, the source remains exact `c9ccf6ac` plus the
three qualification overlays, no source-local `target/` exists, and only the
expected ignored dependency and SDK output roots appeared. All record sidecars
and the terminal manifest reproduce; the terminal manifest SHA-256 is
`f5f42e259aaf6580977321436f6e5f6a796d83b83c12357d792e61003df7b1b9`.

The evidence does not prove execution provenance. `00-preflight.json` labels its
observations as before creation while recording `/tmp/issue688-evidence` as
already present. No contemporaneous record proves that path's initial absence,
exclusive creation, or the first executor's dispatch authority at `48b2547f`.
A separately dispatched Luna stopped without persistence or a gate after finding
the existing paths; that correct stop does not supply the missing proof. Do not
infer or reconstruct it, alter an existing record, or rerun any gate.

## Attempt 2: read-only evidence disposition

Attempt 2 may decide only whether the frozen attempt-1 technical outputs qualify
the candidate for a later promotion amendment while permanently retaining the
freshness/executor limitation. It does not retroactively pass attempt 1 or grant
#687 credit. It owns exactly one new record:

```text
/tmp/issue688-attempt2-disposition.json
```

Astra LOW must first return SCOPE PASS against the exact clean pushed amendment,
matching GitHub body, current tracker heads, unchanged main/source/product, and
the absent record path, including as a dangling symlink. It may then perform one
read-only adjudication. No Luna execution is needed because this is a verification
disposition, not implementation or a gate run.

The adjudicator may read the #688 spec/body, Git history and refs, existing
`/tmp/issue688-*` records and scratch source, and the preserved
`/tmp/issue687-stage2-artifact`. It must verify every record/sidecar and terminal
manifest hash; ordered command identities, statuses, and complete streams; exact
source head and three-file overlay; absence of a source-local `target/`; ignored
output roots; final candidate census; and that no builder or repin command is
recorded. It must preserve and state the unproved initial-absence/exclusive-
executor facts.

After all in-memory checks pass and the record path is still absent, create that
single file with exclusive non-overwriting creation, flush and synchronize it,
read it back, make it read-only, and report its SHA-256. The record must identify
the exact #688 scope and tracker heads, main/source/product/artifact identities,
the reviewed record hashes and statuses, the supported technical conclusion,
and every provenance limitation. No other path may be created or changed. A
mismatch or need to reconstruct evidence returns FAIL without a record.

An attempt-2 PASS qualifies only the preserved candidate's technical
applicability for a separately pushed promotion scope. It cannot itself edit a
pin, rebuild, open a PR, merge, close #685/#688, or claim delivery or performance.

## Concurrent verdict reconciliation

A concurrent review also reported technical EVIDENCE PASS while retaining the
unproved pre-creation/exclusive-executor limitation. The stricter adversarial
verdict above controls: attempt 1 remains failed and the promotion brief that was
appended with the favorable verdict is withdrawn. Do not rerun a gate or perform
promotion. Attempt 2 is only the read-only disposition already specified.

## Attempt-2 scope review — PASS

Astra LOW returned **SCOPE PASS** at exact clean pushed issue head `f0253618`,
tracker `0d5368b2`, and unchanged main/source/product. GitHub parity held and the
sole disposition path was absent, including as a dangling symlink. Attempt 2 may
perform only the specified read-only checks, then exclusively create the one
disposition record with durable readback and retained provenance limits.

The record must also state that the preserved command sequences do not prove an
exact historical total invocation count. No gate execution, predecessor
mutation, #687 credit, pin edit, rebuild, PR, merge, or delivery is authorized.

## Attempt-2 disposition — PASS

Astra LOW returned **DISPOSITION PASS** after revalidating the exact issue
`f0253618`, tracker `0d5368b2`, all record sidecars, nine ordered status-zero
gates, source and overlay identities, external targets, terminal manifest, and
six candidate hashes. It created only
`/tmp/issue688-attempt2-disposition.json`, mode `0444`, size 13,768 bytes, at
SHA-256 `fe263bf681dca977b924c63683e403ee2d0f48f3d960d902a3e5aa429379dfc5`.
Attempt 1 remains failed. Initial path absence, exclusive creation, first-
executor authority, historical immutability, and exact invocation count remain
unproved. The technical applicability result permits only the separately
reviewed final promotion attempt below.

## Attempt 3: exact promotion and ordinary rebuild

Attempt 3 is final. After Astra LOW passes this exact pushed scope, one Luna
HIGH executor may copy the qualified contents of exactly these files from
`/tmp/issue688-source` into this clean worktree:

```text
hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256
hosts/host-web/qualification/results.json
hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md
```

Require all three destination files to match their qualified source bytes and
the Git diff to touch only those paths. The executor must stop and hand the
worktree to root. Root commits and pushes that exact tranche before any post-pin
command. No other tracked content or generated artifact may be copied.

At the clean pushed promotion checkpoint, Astra LOW must verify the three-file
diff byte-for-byte against `/tmp/issue688-source`, candidate digest
`31c882af32959c0164ae069b5ba63a5d5e7890b024d07c04bb75afc06e66cd4b`,
full product identity `276ffb6097a84088e3b5f4a16892a33bca9e26fb`, and initial absence including
symlinks of:

```text
/tmp/issue688-attempt3-artifact
/tmp/issue688-attempt3-target
/tmp/issue688-attempt3-evidence
```

Only checkpoint PASS authorizes one Luna HIGH executor to create those paths and
run these commands once in order, stopping permanently at the first failure:

```text
CARGO_TARGET_DIR=/tmp/issue688-attempt3-target bash scripts/build-web-audioworklet.sh /tmp/issue688-attempt3-artifact
CARGO_TARGET_DIR=/tmp/issue688-attempt3-target bash scripts/check-web-audioworklet.sh /tmp/issue688-attempt3-artifact
CARGO_TARGET_DIR=/tmp/issue688-attempt3-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue688-attempt3-artifact
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

The ordinary build must contain exactly six ordinary files and match the
qualified `/tmp/issue687-stage2-artifact` byte for byte. The promotion worktree
must remain clean and source-local `target/` absent. Persist concise complete
records in the named evidence path and preserve all outputs through Astra LOW
review. Commit no artifact, binary, compiler stream, target, SDK output, browser
dependency, or evidence capture. PR/merge remains blocked until final Astra LOW
evidence PASS. Any failure hard-stops #688 without retry or successor.

## Final attempt-3 verdict — FAIL; hard stop

Astra LOW independently reproduced the attempt-2 disposition at exact SHA-256
`fe263bf681dca977b924c63683e403ee2d0f48f3d960d902a3e5aa429379dfc5`
and accepted only its limited technical-applicability conclusion. Attempt 1
remains failed; every freshness, dispatch, exclusivity, historical-immutability,
and exact-invocation-count limitation remains controlling.

The first attempt-3 scope review returned **SCOPE FAIL** because all three
promotion files were already modified before review. It expressly withheld
retroactive authority and required preservation. A competing actor then committed
and pushed those exact bytes as `2cc6fff5cc21bcca96ebeb106fa3827b47f043fd`
after the failure. The commit is clean, changes only the pin, results, and matrix,
and matches `/tmp/issue688-source`, but it receives no qualification, promotion,
PR, merge, or delivery credit.

The competing execution continued after the scope failure by creating all three
attempt-3 paths and starting the ordinary builder. Astra LOW ruled that the
unauthorized copy/commit/build sequence consumed final attempt 3. The coordinator
stopped its process group while the builder was still running. Preserve the
partial artifact, target, evidence, and commit exactly as found. The evidence
directory contains only `00-preflight.json` and its sidecar; no builder result,
static/resource result, terminal manifest, or complete six-file output exists.

#688 is hard-stopped after three attempts. Do not repair, rerun, adopt, merge,
rewrite, reset, clean, or create a successor. The explicit no-successor boundary
requires new owner instruction to change. #685 remains an undelivered partial;
no original open finding may begin while that inherited partial remains.
