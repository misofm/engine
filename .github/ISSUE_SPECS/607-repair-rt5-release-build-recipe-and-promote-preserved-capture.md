# Repair the RT5 release build recipe and promote the preserved capture

GitHub: https://github.com/misofm/engine/issues/607

Narrow successor to stopped #606 under audit lane A #559. #606's Astra LOW review accepted its guarded publication lifecycle, validator, runner, stub matrix, and 19-entry qualification manifest, but the exact preflight build recipe applied `-C lto=fat` globally and could not compile Cargo build dependencies. No final preflight, prepared executable, real capture invocation, or timer has run in #602, #603, or #606.

Sol HIGH coordinates documentation, checkpoints, capture authorization, GitHub synchronization, and delivery. Luna XHIGH implements one bounded pass. Every scope, source/harness, seal, capture, and exact-head verification uses Astra LOW. Lane B #605 is the only other active issue and exclusively owns shipped artifact qualification/pinning.

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
6. Integrate current main, recheck #605 disjointness, obtain Astra LOW exact integrated-head PASS, push, run required PR qualification, merge, require successful post-main qualification, synchronize this issue and #559/#560, verify remote states, and remove clean stopped/delivered worktrees. Lane B supplies artifact qualification/pinning applicability.

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
