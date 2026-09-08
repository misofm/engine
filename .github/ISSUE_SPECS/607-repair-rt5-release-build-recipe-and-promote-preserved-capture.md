# Repair the RT5 release build recipe and promote the preserved capture

GitHub: https://github.com/misofm/engine/issues/607

Narrow successor to stopped #606 under audit lane A #559. #606's Astra LOW review accepted its guarded publication lifecycle, validator, runner, stub matrix, and 19-entry qualification manifest, but the exact preflight build recipe applied `-C lto=fat` globally and could not compile Cargo build dependencies. No final preflight, prepared executable, real capture invocation, or timer has run in #602, #603, or #606.

Sol HIGH coordinates documentation, checkpoints, capture authorization, GitHub synchronization, and delivery. Luna XHIGH implements one bounded pass. Every scope, source/harness, seal, capture, and exact-head verification uses Astra LOW. Lane B #608 is the only other active issue and exclusively owns shipped artifact qualification/pinning. It replaced stopped #605 after the capture was accepted.

## Smallest closable outcome and ownership

Preserve the reviewed issue-606 tooling identity and `artifacts/issue-606-input-symmetry-capture/` namespace, and the frozen issue-602 Rust record identity. Change only:

- `scripts/preflight-input-symmetry-capture.sh`; and
- focused successor qualification/review evidence under the existing issue-606 artifact root, this numbered spec, and `docs/audits/`.

The validator, runner, lifecycle self-test, Rust sources, fixture, Cargo.toml, Cargo.lock, runtime crates, policies, workflows, shipped artifacts, and pins remain unchanged.

The repaired build must obtain `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and `debug = 1` from the committed `[profile.release]`, whose default optimization level is 3. It may inject only the required x86-64-v3 target features `+avx2,+fma`; it must not inject LTO, codegen-unit, panic, or debug flags globally. The seal continues to state the effective combined release configuration and binds the exact candidate tree, preflight hash, binary hash, toolchain, target, sources, fixture, lockfile, cwd, argv, and workload contract.

## Workflow and gates

1. Push and synchronize this brief and #559/#560. Astra LOW must return exact-head scope PASS.
2. Luna XHIGH makes one coherent implementation pass. Before committing, compile-check the exact repaired Cargo recipe outside protected final artifact paths and record the command/status truthfully. The implementation must also rerun the unchanged guarded stub matrix and all 19 repository-root qualification checksums.
3. Root commits and pushes the exact tranche. Astra LOW reviews the preflight source, the Cargo profile derivation, the compile-only evidence, the unchanged #606 lifecycle paths, and exact-head cleanliness. No final preflight or timer may run before PASS.
4. Root then runs final preflight exactly once at the approved clean head. Astra LOW verifies the published issue-606 seal, prepared binary, environment contract, profile-derived effective flags, and candidate identity.
5. Only after seal PASS, root invokes `bash scripts/run-input-symmetry-capture.sh` exactly once. No retry or resume is permitted. Astra LOW validates raw/accepted bytes, schemas, lifecycle, disposition, hashes, output digests, two finite timing records, and honest accounting. Timing magnitude is descriptive.
6. Integrate current main, recheck #608 disjointness, obtain Astra LOW exact integrated-head PASS, push, run required PR qualification, merge, require successful post-main qualification, synchronize this issue and #559/#560, verify remote states, and remove clean stopped/delivered worktrees. Lane B supplies artifact qualification/pinning applicability.

One Luna XHIGH implementation pass and no correction are authorized. Any substantive defect stops this issue and requires a new successor. No original open #559 finding may start until this successor and lane B's remaining partials are delivered.

## Astra LOW corrected scope review

Astra LOW returned **PASS** at exact clean brief/upstream
`4bfb3a25865bfb72257975b33ae4f615b00be905`, with synchronized handoff head
`69117bf82e02bcfc5c87b0f9a840279349686d49` and #605 coordination head
`6f5005d37af9bc8d51484b8e26b464127dcc4337`. Live #559/#560/#605 identify #605/#607 as the
two disjoint active issues and #606 as stopped. The reviewer accepted the single-script profile-
derived build repair, preserved issue-606 tooling/artifact identity, frozen issue-602 record identity,
compile-only proof, and successive source/seal/capture review gates. Luna XHIGH may begin the sole
implementation pass. Final preflight and timing remain unauthorized.

## Astra LOW source and harness review

Astra LOW returned **PASS** on exact pushed implementation head
`d2170d5617cf7e720b58722f0b78d5fcb1d3525c`. The exact-path delta, unchanged lifecycle tooling,
and 26-entry manifest passed. An independent verbose compile-only build exited zero and confirmed
the committed release profile supplies opt-level 3, fat LTO, one codegen unit, abort panic and debug
info while only `+avx2,+fma` is injected globally. The raw compiler log is preserved with this review
record. One final preflight is authorized after an evidence-only exact-head check; capture remains
locked pending separate seal PASS. No protected preflight, runner, capture entry, or timer ran.

The evidence-only follow-up returned **PASS** at exact clean head
`3b632cdb68105e2023b08ac90b4e2baa99bafa53`; all 27 checksums verified directly from Git blobs and
the preserved raw compile log matched the reviewer's original bytes. Root then ran the final
preflight exactly once. Astra LOW passed the strict READY seal, exact candidate/tree/hash/profile/
workload identity, and prepared executable SHA-256
`74da9ae5c249fb94fd9c9c6306cbcbf30eded280513e793c369e99bb8ebb3a89` before authorizing capture.

Root invoked the runner exactly once. Astra LOW returned final capture **PASS**. Raw and accepted
JSONL are byte-identical, 4,244 bytes, SHA-256
`59257eb092f197b616cbaa20ec713ed8b4e10446c29941e8a1d7d23c96db89ca`. Both ordered rounds completed
8,192 successful renders with zero errors and matching reviewed owner digests. The descriptive
results are 7,226 and 7,219 ns per plan render. One start plus two completion markers prove one
workload process and 16,384 timed calls. No retry or resume occurred, and no magnitude gate or
performance-improvement claim applies.

After capture acceptance, lane-B #605 stopped at its third attempt and #608 inherited its disjoint
IO5 test-only qualification. Live delivery coordination therefore uses #607/#608 as the two active
issue slots. Historical #605 scope-review statements above remain accurate for those checkpoints.

Lane-B Astra LOW returned artifact-applicability **PASS / N/A** at exact pushed head
`8bb2a2e763103030af258a69eecae5bd2374850b`. The native benchmark/capture paths have no dependency
edge into the six-file AudioWorklet builder or consumers, and the delivery delta changes no runtime,
browser/SDK, manifest/lockfile, build input, generated consumer, artifact, pin, or matrix. Retain pin
`39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55` with its existing
qualification attribution. No artifact build or pin change ran; #608's later artifact gate is
separate.

## Pull-request qualification verdict

PR #609 required qualification run `34195043020` failed the environment/marker vocabulary gate at
exact reviewed head `20a9265cd705fe33d76657a37ce44c43fc1e7997`. The gate listed 20 identifiers used
by the inherited capture scripts but absent from `docs/ENGINE_ENV_VOCABULARY.md`. Astra LOW
independently confirmed the source/table mismatch and returned **PASS to stop / FAIL to correct within
#607**. The missing central vocabulary rows are outside this issue's exact ownership, and its sole
implementation pass has no correction allowance. PR #609 is closed without merge. The accepted
capture remains preserved and must not be rerun; a vocabulary-only successor inherits it unchanged.
