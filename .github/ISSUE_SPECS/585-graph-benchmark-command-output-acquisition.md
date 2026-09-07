# Reuse shared command-output acquisition in graph benchmark

Status: proposed numbered TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `4ef2ee1cae8553847851e0f5a886be8929be388d` after #583 / PR #584 and successful post-main qualification `34166341583`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH implements. Astra LOW performs all scope, source and exact-head/current-base verification under the user's latest routing. No audio or artifact work is indicated.

## Smallest closable outcome

Move the graph benchmark's remaining private program/argument command-output acquisition onto delivered `bench_support::sysinfo::command_output`. Preserve graph's existing projection: trimmed successful UTF-8 is retained, including `""` for empty or whitespace-only success; spawn failure, nonzero exit and invalid UTF-8 become `"unknown"`.

Use the shared authority for graph's existing rustc, CPU and uname metadata calls. Preserve first compiler-line extraction, LLVM and host-field extraction, CPU and OS composition, environment sentinel/missing-field policy, metadata schema and synthetic record bytes. This is one consumer migration; TOOL9 remains partial afterward.

## Exact ownership

Allowed paths are `tools/bench/src/graph.rs`, this numbered spec and bounded issue evidence. Exclude `bench-support`, every other benchmark consumer, schemas, timing/workloads/corpora, manifests/lockfiles, policies/workflows, runtime/host/DSP/session/control code, SDK/browser/generated artifacts and pins. Active lane-A #580 retains its endpoint, PCM and artifact-qualification paths; no ownership overlaps.

## Objective gates

1. Prove exact program/argument forwarding and trimming for successful text.
2. Prove empty and whitespace-only success remain `""`; spawn failure, nonzero exit with plausible stdout and invalid UTF-8 map to exactly `"unknown"`.
3. Preserve graph metadata's first compiler line, LLVM/host extraction, CPU value, OS composition, sentinel/missing-field policy and synthetic JSON record bytes.
4. Remove graph's private `Command::new(...).output()` body and reuse the delivered authority without another wrapper or shared-helper change.
5. Focused graph tests pass in debug/release, complete affected bench debug and proportional release tests pass, and strict affected Clippy/rustdoc, format/diff and workspace/bench policies pass. No timed benchmark, browser or artifact run receives credit.

## Stop and split triggers

Stop before changing metadata semantics, the shared helper, another consumer, schema/formatting, timing, workload, a manifest/lock, policy/workflow or any runtime/audio/artifact path. Preserve the checkpoint and brief the independent outcome. One Luna HIGH attempt receives one Astra LOW adversarial verdict; stop after three failed attempts without weakening gates. The implementer pauses at the first coherent focused-green tranche for root checkpointing.

## Preliminary residual audit

Astra LOW reviewed main `4ef2ee1cae8553847851e0f5a886be8929be388d` after #583 and found one remaining acquisition copy in `tools/bench/src/graph.rs`. Its behavior matches the delivered shared authority except for graph's local `None` → `"unknown"` projection, which remains local. Current statistics helpers are already shared; remaining metadata and formatting differences include intentional schema and character-policy distinctions. TOOL11 and IO5 residuals require broader contracts.

This one-file extraction is disjoint from active #580 source `2b1a7d5a46a2caaaba599f30e12b1c6e36d119ff` and qualification `370ef7cd6f0f1d07094af2a08ad1545b53a676f9`. Astra LOW returned conditional PASS and is sufficient for all verification. Activation requires matching numbered local/GitHub identity, exact pushed brief, current-base and ownership checks, and Astra LOW scope PASS. No implementation is authorized by this preliminary audit alone. Historical #543/#555 → #558/#552 and #542 → #567 delivery order and verdict provenance remain unchanged.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact pushed brief `7353a61c54a209ea3ae797675a2fe1717f858c36` on base and merge-base `4ef2ee1cae8553847851e0f5a886be8929be388d`. The clean exact upstream contains only this spec, diff checks pass, GitHub #585 has exact number/title/body identity, and base qualification `34166341583` succeeded. The shared helper plus graph's local unavailable projection preserve successful-empty behavior. Narrow fixed metadata seams can cover compiler/LLVM/host/CPU/OS/missing-field projections without schema or timing changes.

#580 remains disjoint at source `2b1a7d5a46a2caaaba599f30e12b1c6e36d119ff` and qualification `370ef7cd6f0f1d07094af2a08ad1545b53a676f9`. No correction, helper/manifest/lock/policy/artifact change or broader migration is authorized. Astra LOW is sufficient. Luna HIGH attempt 1 may begin and must pause at its first coherent focused-green tranche. TOOL9 remains partial.

## Attempt 1 implementation checkpoint

Luna HIGH delivered the one-file source checkpoint `d49cd93feaf3965686f951f778ce8358b32cc69b`. Graph's private `Command::output` body is removed; its local `command` projection delegates to `bench_support::sysinfo::command_output` and retains `None` → `"unknown"` while successful empty stays empty. Unix fixtures cover trimmed text, empty/whitespace success, nonzero plausible stdout, spawn failure and invalid UTF-8. A fixed metadata/record fixture pins all graph metadata fields, Unicode escaping, incomplete status and missing-field output.

Focused graph debug/release each pass 5 tests, complete bench debug passes 42 tests, and strict bench Clippy/rustdoc, formatting and diff checks pass. Only `tools/bench/src/graph.rs` changed; no benchmark, helper, manifest, lock, policy, artifact or #580 path changed. Astra LOW adversarial source review remains pending.

## Attempt 1 source verdict

Astra LOW returned **FAIL** at exact pushed head `00bfaf031a383db8b654ee8a1f180c89e8e63309`, source `d49cd93feaf3965686f951f778ce8358b32cc69b`, merge-base `4ef2ee1cae8553847851e0f5a886be8929be388d`. The acquisition change is correct, all proportional gates pass, issue/upstream identity is exact, and no scope or #580 overlap exists.

Attempt 1 fails because the fixed record test constructs already-projected `Metadata`; it cannot detect broken compiler-line/LLVM/host extraction, CPU/OS composition or environment sentinel/missing-field handling. Attempt 2 may add only a narrow production-used projection seam with deterministic raw command and environment inputs, assert those outputs including absent and sentinel cases, and retain the record oracle. No other change is authorized.

During review, main advanced disjointly through #580 / PR #582 to `defa979cbf0bf86b4ebba2f52b0647eb01b9ff29`. That delta changes neither bench nor bench-support. Final delivery still requires integration and exact current-base review after source PASS.
