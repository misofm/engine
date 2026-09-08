# Verify the outlined limiter mono kernel and resume artifact qualification

GitHub: https://github.com/misofm/engine/issues/619

Parent: #617. Product parent: #539. Coordination: #559 and #560.

Issue #617 independently reproduced the exact six-file candidate at frozen compiled source
`d63bc437e948d6284b1b6cbe459f0b46c4ed6566`: Wasm SHA-256
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`, with all five non-Wasm
outputs byte-identical to delivered CP8. Its first static gate then failed because the roster still
requires collapsed limiter vector arithmetic inside `PreparedTruePeakLimiterBank::process_bank_mono`.
The candidate keeps that entry but LLVM outlined its arithmetic into the directly called
`LimiterCore::process_block_mono` body.

Astra LOW's read-only disassembly found delivered entry function 768 carried 224 vector operations;
in the candidate entry function 770 carries zero arithmetic and directly calls function 756, whose
`LimiterCore::process_block_mono` carries 440 vector operations and zero scalar operations. The dual
body retains 880 vector and zero scalar operations. This is an outlining change, with no demonstrated
de-vectorization, but the existing required gate correctly blocks delivery until its direct-body
assumption is replaced by an equally discriminating entry-to-kernel rule.

#617 is closed after its single qualification pass. This issue and passive product parent #539 are
the only active slots. Sol HIGH coordinates and owns documentation/checkpoints/GitHub delivery.
Luna HIGH or XHIGH implements one bounded tooling attempt. Astra LOW performs every scope, source,
artifact and delivery verification. Lane B retains artifact qualification and pin authority. No
limiter Rust/test edit, artifact rebuild before conditional post-pin verification, benchmark,
timing/capture or performance claim is allowed.

## Tooling correction

Exact implementation ownership is limited to:

- `scripts/check-web-audioworklet-callgraph.py`, including its synthetic self-tests;
- comments in `scripts/check-web-audioworklet.sh` only if needed to keep the gate contract accurate;
- this issue's spec and bounded evidence.

Represent the collapsed limiter as an entry pattern plus a distinct arithmetic-kernel pattern. The
gate must require exactly one entry and exactly one arithmetic-bearing kernel, require the entry to
directly call that kernel, and require the forwarding entry itself to contain zero counted
`f32x4.{mul,add,sub,div}` and `f32.{mul,add,sub,div}` operations. Apply the existing vector-
dominance, scalar-ratio ceiling and slack to the arithmetic-bearing kernel. All other roster rows
retain their current exact-one behavior and budgets. Do not lower a floor/ceiling, remove a roster
row, accept indirect reachability, or special-case the candidate digest.

Add independent synthetic negative controls proving failure when the entry is absent or ambiguous,
the arithmetic kernel is absent or ambiguous, the entry does not directly call the selected kernel,
the forwarding entry contains scalar arithmetic, and the selected kernel crosses its scalarization
budget. Preserve the existing vanished/ambiguous, partial scalarization, degree-reduction and slack
controls. Before source review run only the checker self-test, Python compile and proportional
format/diff/policy gates. Astra LOW source review must PASS before the preserved candidate is read by
the repaired static gate.

## Resume without rebuilding the candidate

The checksum-verified #617 evidence and external candidate output are preserved. Before use, verify
all 28 committed #617 evidence hashes, the six output hashes, exact frozen source, candidate digest,
five-file identity, and the external output directory. Copy or retain the output in a bounded
external path if needed; do not invoke the builder again.

After source PASS, create an isolated scratch checkout containing only the reviewed tooling change
plus provisional candidate pin, `results.json` lineage (`candidateCommit=d63bc437…`,
`wasmSha256=f80b6392…`) and generated matrix lineage. Prove that overlay. Run exactly one repaired
static-gate invocation against the preserved six-file output. This is the successor's sole candidate
static run, not a retry within #617. If it passes, run only the #617 qualification stages that never
executed: resource/native-witness and all 26 red mutations, hermetic checks, locked SDK,
locked dependency installs, one all-browser Chromium/Firefox/WebKit run with matrix/self-test
mutations, and final matrix/diff checks. Preserve exact commands, streams/status, versions,
identities, overlays and checksum manifest. No browser or gate retry is allowed.

Astra LOW must PASS the complete combined #617/#619 candidate record before repository promotion.

## Conditional promotion and delivery

After candidate PASS, Luna HIGH/XHIGH may change only the already reviewed tooling files plus:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to `f80b6392…` plus LF;
- `hosts/host-web/qualification/results.json` only `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only generated lineage text;
- this issue's spec/evidence.

All browser rows, version floors, gates, resources, ABI/metadata/JS, dependencies, lock/toolchain,
workflows, source and #539 limiter files remain frozen. Root checkpoints the exact promotion. Run
one ordinary no-bypass post-pin build and require exact identity with the preserved qualified six-
file candidate, followed by proportional static/resource/hermetic/SDK/format/diff/workspace/effect-
runtime gates. Do not repeat browsers without a specific failed gate.

Astra LOW then reviews the exact clean pushed head/current main. Open one PR only after PASS. Require
routed qualification/fuzz success, a live checked head/base before merge, exact merge parents and
post-main qualification. Close #539 and #619 only after upstream evidence and GitHub synchronization;
update #559/#560 and remove clean delivered worktrees while retaining branches/history.

One tooling implementation attempt and one resumed qualification pass are authorized after scope
PASS. A substantive failure stops for reviewed rescope. This issue does not add a #539 product
implementation attempt or weaken the static SIMD gate.

## Initial Astra LOW scope review — FAIL

Astra LOW returned **FAIL** at exact clean pushed head
`e0597993a4381d815d9d36d102a6d58f3100c06b`, main `77368243`. The bounded correction and all
preserved #617 identities were sound, but #559/#560 coordination was stale, the proposed kernel-only
budget allowed scalar arithmetic to escape into the forwarding wrapper, and the brief ambiguously
requested two repaired candidate static runs.

The corrected scope requires zero counted vector or scalar arithmetic in the forwarding entry, adds
an independent scalar-wrapper negative, and permits no candidate static run before source PASS. The
sole repaired candidate static invocation occurs afterward and is distinct from stopped #617. #560
now records #617 stopped/#619 active; #559 receives the same handoff in its next tracker checkpoint.
No builder, static gate, browser, implementation or pin/lineage edit ran. Corrected Astra LOW scope
review is pending.

## Corrected Astra LOW scope review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`f712caef0b6da4310a86b1bad1aab4bab3588acb`, live main `77368243` and tracker
`b33a5692`. The corrected contract requires exactly one forwarding entry and arithmetic kernel, a
direct call between them, zero counted vector/scalar arithmetic in the entry, and the unchanged
kernel scalarization budget. Its independent scalar-wrapper negative closes the initial scope gap.

All 28 retained #617 evidence hashes and the six preserved output hashes verify. Independent
disassembly confirms wrapper 770 is 0/0 and directly calls collapsed core 756 at 440/0, distinct
from dual core 755 at 880/0. #559/#560/#619 and the two-slot/path holds are synchronized. Luna HIGH
or XHIGH may perform the single bounded tooling implementation attempt. No candidate static run,
qualification continuation, builder, pin/lineage edit, PR or merge is authorized before Astra LOW
source PASS.

## Tooling attempt 1 source review — PASS

Luna HIGH changed only the two authorized checker/comment scripts at exact clean pushed head
`fc77a5c6218b84b6ffa73b6ca335e15d1d4dc531`. Astra LOW returned **PASS** there against live main
`77368243`. The checker requires exact-one entry and arithmetic kernel, zero counted wrapper
arithmetic and a direct call, while retaining existing dominance, ceiling, slack and all other
roster rules. Its seven new independent red controls and all prior controls pass; Python/shell
syntax, diff hygiene, workspace and realtime policies also pass.

The minor summary text still prints `entry=0/0` after an already reported wrapper-arithmetic
failure, but the specific diagnostic and nonzero verdict are correct; no correction is required.
The single tooling attempt is accepted. Astra LOW may now run exactly one repaired static invocation
against the preserved #617 six-file output and, only after it passes, the previously unexecuted
qualification stages. No builder, repository pin/lineage edit, PR or merge is authorized.

## Resumed candidate qualification checkpoint

At source-accepted head `794d0dddf17370098f832607766819f73fc77d26`, Astra LOW used the
preserved #617 six-file output without invoking the builder. Its identity remained candidate
`f80b6392…` plus the five delivered non-Wasm hashes, and the detached scratch diff remained exactly
the provisional pin, two results lineage fields and generated matrix lineage.

The repaired static invocation passed once. It found exactly one 0/0 collapsed limiter forwarding
entry directly calling the unique 440-vector/0-scalar mono core, retained dual core 880/0 and kept
all other roster rows within their existing budgets. The previously unexecuted resource/native-
witness stage and all 26 red mutations, hermetic gate, locked SDK install and 11 SDK tests, locked
browser install, one all-browser qualification and final matrix check each passed once. Chromium
151.0.7922.34, Firefox 153.0 and WebKit 26.5 passed. No command was retried.

The 39-entry checksum manifest and bounded command/context/stream/status/overlay evidence are under
`artifacts/issue619-resumed-qualification/`; build targets and the preserved generated candidate stay
outside Git. No builder, repository pin/lineage edit, source change, benchmark or timing ran. Astra
LOW must review the complete combined #617/#619 candidate record before Luna may promote the three
repository lineage surfaces.

## Candidate evidence review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`ffb19e9be887d4feca09ad9dd3d6fdaef5ec7655`, live main `77368243`. All 28 inherited #617 and
39 new #619 manifest entries verify. The record proves one repaired static invocation plus seven
previously unexecuted commands, all status 0, with no builder or retry. Candidate `f80b6392…`, the
five delivered non-Wasm identities, exact scratch overlay, browser versions and frozen rows/resources
remain supported; no cache or generated output was committed.

Luna HIGH/XHIGH may now copy exactly the three qualified pin/lineage surfaces into the feature
worktree while leaving both reviewed scripts unchanged. Root must checkpoint that promotion before
the one ordinary post-pin build and proportional gates. PR and merge remain unauthorized.

## Astra LOW combined candidate review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`ffb19e9be887d4feca09ad9dd3d6fdaef5ec7655`, live main/merge-base `77368243`. All 28 #617 and 39
#619 evidence hashes, preserved candidate bytes and five-file non-Wasm identity verify. Eight unique
resumed commands match their raw streams; no builder repeated.

The sole repaired static run proves the 0/0 wrapper's direct edge to the 440/0 kernel with unchanged
budgets. Resource/native-witness and 26 mutations, hermetic, 11 SDK tests, Chromium
151.0.7922.34, Firefox 153.0, WebKit 26.5 and matrix checks pass. Scratch changes are exactly the
pin, two results lineage fields and generated matrix lineage; browser rows/resources are frozen.
Luna XHIGH may now promote only those three repository surfaces. PR/merge remain unauthorized.

## Conditional promotion and ordinary post-pin build

The qualified pin and lineage surfaces are checkpointed at `8577f1aa4e2f723929137a978703581ea25da483`.
Their diff is exactly the pin, `candidateCommit`/`wasmSha256`, and generated matrix lineage; every
browser row, version, gate and resource remains byte-stable, and the two reviewed checker scripts
are unchanged.

Root ran the single authorized ordinary no-bypass post-pin builder at that exact clean pushed head.
It continued under one process after the initial output yield, exited zero and emitted exactly six
files. Every output byte-matches the preserved qualified candidate: Wasm is `f80b6392…`, and the
five non-Wasm files retain their delivered CP8 hashes. The 13-entry checksum-verified record is under
`artifacts/issue619-postpin/`; generated outputs and build targets remain outside Git. No retry,
browser, benchmark or timing ran. Astra LOW post-pin static/resource/hermetic/SDK and exact-head/
current-main review remain required before PR delivery.
