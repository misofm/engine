# Qualify the resident-input AudioWorklet and deliver #713

## Authority and outcome

This is the lane-B artifact successor to #713. It owns qualification and
delivery of the already accepted resident-input source; it does not reopen
#705, change product source, or repeat #716. #713 remains OPEN at SOURCE PASS
and retains its complete implementation history. #716 is a closed failed
delivery qualification with its branch, run, and evidence preserved.

Sol HIGH coordinates. Astra XHIGH scopes and executes the low-level
qualification commands. Astra LOW independently reviews the scope, lease,
exact checkout, evidence, exact-head diff, merge, and post-main state. Luna
XHIGH may perform only the mechanical provisional overlay after scope PASS and
before the execution lease. No agent owns the issue.

The adopted source and product/test identities are:

- source and PR #716 head:
  d6a04d93e4296e6c1d6eaefc9547b60bcf8462cc;
- #713 product/test PASS identity:
  7576d1b6c794f01df4abeb7256a8309d45879b21;
- current main:
  0dc066337d990a368801138d1c1f8b63830cd57e;
- #716 test merge:
  a4072c7097e0983a71b5dab1c6a3db37ace06828;
- #716 test tree, equal to the PR head tree:
  ced9b39475ddc2c9e05c441efa0db0f18f716dd2.

Run 34448821511 passed every non-artifact required leaf, but shipped-artifact
job 102779640808 failed the aggregate: expected
5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574 was
observed instead of
c4312e05d4f7e8117d9cfba8fc5a07b5f294fb6473db75f4804353730a302569.
The provisional observed digest
5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574 has zero
qualification or delivery credit. #716 is closed. The failure is retained as
the reason for this bounded successor, with no timing, performance, or full
RT9 claim.

## Bounded scope and owned paths

The issue qualifies one ordinary generated six-file AudioWorklet artifact from
the adopted #713 source and delivers it through the normal guarded PR path.
The build runs once with MISO_ENGINE_WEB_AUDIOWORKLET_REPIN unset in a fresh
detached checkout after the mechanical provisional overlay. A digest mismatch,
unexpected file, failed gate, or receipt failure stops the attempt. There is
no retry or automatic repin; a further attempt requires a new reviewed brief.

After Astra LOW scope PASS and before the execution lease, Luna XHIGH may make
the mechanical overlay only. The overlay sets candidateCommit to
d6a04d93e4296e6c1d6eaefc9547b60bcf8462cc and wasmSha256 to the provisional
observed digest above. It changes the pin only when the generated digest
differs from the existing pin, updates only the corresponding generated
browser matrix lineage, and changes no product/test source. If qualification
observes a different digest, preserve the failure and stop for rebrief; do not
repin in the same attempt.

After authorization, tracked edits are limited to this numbered spec, the
#713 spec for its final delivery row, the AudioWorklet pin
hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256, only
candidateCommit and wasmSha256 in hosts/host-web/qualification/results.json,
and the corresponding generated lineage in
hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md. The inherited #713 product/test
paths remain byte-identical:

- crates/rack/src/lib.rs;
- crates/graph/src/runtime.rs;
- crates/graph/src/lib.rs; and
- crates/graph/tests/rt9_resident_bank_input_alloc.rs.

No other product source, test, fixture, script, workflow, dependency,
lockfile, host expectation, DSP behavior, artifact authority, or AudioWorklet
pin input may change. Do not reopen #705 or edit #714. #714 remains queued
behind this lane-B delivery.

## Frozen artifact authority

The artifact must contain exactly six ordinary files. The Wasm file must equal
the authenticated provisional digest above after the ordinary build. The five
non-Wasm hashes are frozen from the delivered #705 authority:

    40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919  miso-engine-v1-abi-layout.json
    445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf  miso-engine-v1-audio-worklet-host.d.ts
    21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a  miso-engine-v1-audio-worklet-host.js
    225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb  miso-engine-v1-audio-worklet.js
    6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d  miso-engine-v1-parameter-metadata.json

The five hashes and the Wasm digest must agree with the pin, results, and
matrix lineage. Unexpected non-Wasm, resource, PCM, SDK, browser, mutation,
or generated drift fails. No size, timing, allocation, sound-quality,
performance, or universal PCM-equivalence claim is authorized.

## Immutable checkout and fresh roots

This preparation worktree is branched from clean #713 head
32d75d36492095e4914251dcbcb89a7aa262d13d, which retains origin/main
0dc066337d990a368801138d1c1f8b63830cd57e as an ancestor:

    preparation branch: codex/qualify-resident-input-artifact-717
    preparation worktree: /home/bl/misofm/engine-rt9-resident-artifact-717

After this spec is pushed and synchronized, Sol prepares the mechanical
overlay branch and then the exact detached qualification checkout:

    overlay branch: codex/qualify-resident-input-artifact-overlay-717
    overlay worktree: /home/bl/engine-rt9-resident-artifact-overlay-717
    E2: /home/bl/engine-rt9-resident-artifact-717-qualify

The final detached checkout is clean, detached, and has literal
upstream=not-applicable-detached. No executor creates a checkout or branch.
Every external lease names the exact spec, overlay, E2 head, current main,
#713 source/test identity, tracker, issue bodies, roots, commands, and marker
paths. Astra LOW reviews that lease before any root or command exists.

Fresh roots are all absent as ordinary paths and dangling symlinks before
overlay preparation and again before the lease:

    /tmp/cp1-resident-input-artifact-717-qualify-evidence
    /tmp/cp1-resident-input-artifact-717-qualify-tmp
    /tmp/cp1-resident-input-artifact-717-artifact
    /tmp/cp1-resident-input-artifact-717-qualify-target
    /tmp/cp1-resident-input-artifact-717-hermetic-target

Preserve every #713 and #716 branch, worktree, target, artifact, lease,
receipt, failure, temporary compiler/evidence directory, and retained
provisional payload. No execution root is created while drafting this spec.

## Qualification sequence and gates

All calls use the absolute E2 checkout, login=true, direct real exit
propagation, and a printed cwd, exact HEAD, detached state, literal
upstream=not-applicable-detached, and clean status before assertions. A live
lease and its release/revocation markers are checked before every call. No
custom runner, retry, fabricated status, or polling wrapper is allowed.

After the exact-head Astra LOW lease review, create the evidence, temporary,
and ordinary artifact directories separately and in that order. Keep both
Cargo target roots absent until their commands own them. Run one ordinary
builder invocation, with REPIN unset:

    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target bash scripts/build-web-audioworklet.sh /tmp/cp1-resident-input-artifact-717-artifact

Require the builder's actual exit zero, exactly six ordinary artifact files,
the provisional Wasm digest, the five non-Wasm hashes, unchanged E2 HEAD and
clean status, and no supplied target or hermetic target before their commands.
Builder stdout, stderr, actual exit, environment, cwd, head, status, and
directory traversal are retained in the evidence receipt. A builder or
validation failure consumes the attempt before later gates.

Then run each command exactly once, in order, with REPIN unset, the explicit
qualification TMPDIR and target, and the supplied artifact:

    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target bash scripts/check-web-audioworklet.sh /tmp/cp1-resident-input-artifact-717-artifact
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/cp1-resident-input-artifact-717-artifact
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-hermetic-target bash scripts/test-web-audioworklet.sh
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target npm_config_cache=/tmp/cp1-resident-input-artifact-717-qualify-tmp/npm-cache npm --prefix sdk ci --ignore-scripts
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target bash scripts/sdk-package.sh check /tmp/cp1-resident-input-artifact-717-artifact
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target npm_config_cache=/tmp/cp1-resident-input-artifact-717-qualify-tmp/npm-cache npm --prefix hosts/host-web/qualification ci --ignore-scripts
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target npm_config_cache=/tmp/cp1-resident-input-artifact-717-qualify-tmp/npm-cache npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/cp1-resident-input-artifact-717-artifact --browser all --check-matrix --self-test-mutations
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target node hosts/host-web/qualification/generate-matrix.mjs --check
    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target git diff --check 32d75d36492095e4914251dcbcb89a7aa262d13d <exact-reviewed-E2-head>

The committed review before execution includes name-status and unified diffs
from origin/main to E2 and from #713 head to E2. It permits only the #717
spec, the #713 delivery row, conditional pin, candidateCommit/wasmSha256
fields, and generated matrix lineage. The four inherited #713 product/test
paths, source identity, and main ancestry must remain byte-identical.

Astra LOW reviews the ordinary build evidence, six-file manifest, digest and
hash parity, browser/resource/SDK/mutation gates, current-base exact path
diff, and clean delivered worktree. A first unexpected failure stops the
attempt; preserve all evidence and do not weaken any gate.

## Delivery

After Astra LOW records PASS and the evidence commit is upstream, deliver the
qualified overlay through one PR that supersedes #716. Require the ordinary
PR qualification, exact reviewed-head/current-base inspection, guarded
live-head merge, post-main qualification, exact GitHub body/state
synchronization, issue census, and clean delivered-worktree removal. On
successful synchronization, close #713 and #717 together and verify both
closed states. Until then #713 and #717 remain OPEN and no delivery claim is
valid. Preserve all failed attempts and evidence. A later attempt requires a
fresh numbered root set, a new lease, and a newly reviewed rebrief.
