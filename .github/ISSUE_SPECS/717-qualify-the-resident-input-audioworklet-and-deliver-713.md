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
c4312e05d4f7e8117d9cfba8fc5a07b5f294fb6473db75f4804353730a302569 was
observed as
5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574.
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
    overlay worktree: /home/bl/misofm/engine-rt9-resident-artifact-overlay-717
    E2: /home/bl/misofm/engine-rt9-resident-artifact-717-qualify

The final detached checkout is clean, detached, and has literal
upstream=not-applicable-detached. No executor creates a checkout or branch.
Freeze the command text before lease review. Every external lease names the
exact spec, overlay, E2 head, current main, #713 source/test identity, tracker,
issue bodies, roots, commands, and marker paths. Astra LOW reviews that lease
after command text is frozen and before any execution root is created or
command executes.

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
lease and its release/revocation markers are checked before every call. Only
an empty-stdin continuation on an already-started persistent session is
permitted; it is not a second invocation. No other continuation, retry,
fabricated status, or polling wrapper is allowed. The later Astra LOW lease
review freezes per-call receipts, a data-only terminal result, an independent
acknowledgement marker, a release marker, absent revocation, and exact stop
semantics.

After the exact-head Astra LOW lease review, create the evidence, temporary,
and ordinary artifact directories separately and in that order. Keep both
Cargo target roots absent until their commands own them. Run one ordinary
builder invocation, with REPIN unset:

    env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-resident-input-artifact-717-qualify-tmp CARGO_TARGET_DIR=/tmp/cp1-resident-input-artifact-717-qualify-target bash scripts/build-web-audioworklet.sh /tmp/cp1-resident-input-artifact-717-artifact

Require the builder's actual exit zero, exactly six ordinary artifact files,
the provisional Wasm digest, the five non-Wasm hashes, unchanged E2 HEAD and
clean status. The builder owns and creates qualify-target; only hermetic-target
must remain absent until the hermetic command owns it.
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

The committed review before execution has separate allowlists. The
`origin/main→E2` name-status and unified diff permits the inherited #713 spec,
the four inherited #713 product/test paths above, and the #717/overlay paths:
this #717 spec, the conditional pin, the candidateCommit/wasmSha256 fields,
and the generated matrix lineage. The updated #713 head `32d75d36→E2`
name-status and
unified diff permits only this #717 spec, a #713 delivery/qualification row if
later added, the pin, the two results fields, and the generated matrix lineage.
The inherited product/test bytes must equal the #713 PASS identity
7576d1b6c794f01df4abeb7256a8309d45879b21.

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

## Preparation checkpoint — Astra LOW SCOPE PASS and mechanical overlay

Astra LOW recorded SCOPE PASS at preparation head
`f84e75477852a9be94963fc1216eeb271a87eca2`. Luna XHIGH then performed only
the authorized mechanical overlay. The overlay diff is limited to this spec,
the pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`,
`hosts/host-web/qualification/results.json` (only `candidateCommit` and
`wasmSha256`), and the generated
`hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`. The pin is exactly
`5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574` plus LF;
results identify candidate `d6a04d93e4296e6c1d6eaefc9547b60bcf8462cc` and that
same Wasm digest. Resulting file SHA-256 values are pin
`2eeea48cb64576f15fe7c9d28bc40d26fe7d9aacf9671e2089cefdbf415652aa`, results
`17fe2f7d7026c0f930c8f980eca3e73d50f72fdb42068fdf2aee07b31c27270a`, and
matrix `ed8c79252c3638c4b09785613752505fb77be7298b6c7caadb9f5b5703aa5296`.
The existing matrix generator ran once and its `--check` passed. This overlay
earns no qualification or delivery credit, created no execution roots, and
did not create E2 or a lease. The next gate is the detached E2 at
`/home/bl/misofm/engine-rt9-resident-artifact-717-qualify` with a reviewed
lease before any execution root or command.

## Astra LOW ATTEMPT 1 PASS — qualification only

Astra LOW returned ATTEMPT 1 PASS at overlay/E2 head
`8b8fc4b41cda90a02a8ec78280a75523b0611f73` using the immutable lease
`/tmp/cp1-resident-input-artifact-717.lease` (SHA-256
`bc6d08313c92b0c52afa53604e152fa6c0685f36efdc434acd3d36b33dbb0080`). The
rejected lease drafts remain preserved at
`/tmp/cp1-resident-input-artifact-717.lease.invalid-marker-guard` and
`/tmp/cp1-resident-input-artifact-717.lease.invalid-hermetic-guard`. Named
executor `/root/issue717_astra_xhigh_exec` recorded transcript
`/home/bl/.codex2/sessions/2026/09/10/rollout-2026-09-10T08-00-02-01a08a54-b541-7c90-93c0-f9682c13b146.jsonl`
with SHA-256
`298e154d8e14a11b2ac7d78df30da6b508bc7474d5badfc41ef5f836f4c711f3`.

All 18 commands ran once, in order, with exit 0, including one builder;
12 permitted empty continuations occurred across six already-started
persistent sessions. The executor reported the three qualified browser
versions `chromium 151.0.7922.34`, `firefox 153.0`, and `webkit 26.5`.
The data-only terminal result SHA-256 is
`470fb962a994d3f58f4e5e7ac3f1e29395501644e142e53bfe0138ea2b42c9e6`, the
independent acknowledgement marker is
`da4769119aa609d7631dc3331f1ff2ac9c59e1f0a6e282a44c8964523b271e69`, and
the release marker is
`fdc935e6a3f33abdcfb4f5d7a335d408b2b988e7a5f8411d9f73349d1fab39be`;
revocation is absent. The six artifact hashes remain exactly:

    40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919  miso-engine-v1-abi-layout.json
    445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf  miso-engine-v1-audio-worklet-host.d.ts
    21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a  miso-engine-v1-audio-worklet-host.js
    225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb  miso-engine-v1-audio-worklet.js
    6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d  miso-engine-v1-parameter-metadata.json
    5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574  miso-engine-v1-audio-worklet.simd128.wasm

E2 remained clean and detached. This is qualification PASS only; required PR
CI, guarded merge, post-main qualification, GitHub synchronization, and
clean-worktree removal remain pending. No timing, performance, full-RT9, or
product-change claim is made.
