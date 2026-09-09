# Borrow graph node IDs in buffer-position scratch lookup

## Authority and outcome

Parent: CP1 under lane B #560, coordinated through #559 and audit #349. Exact baseline is main `a703f7574d88ea74841c9dbc6001e5e31eb50300`; tracker is `a171adb7dee662ee0bddbdac842a261057b8d787`. Accounting remains 39 delivered, 1 partial, 81 open, 2 owner dispositions, and 1 historical observation. CP1 is the sole inherited partial; no original open finding starts.

`crates/graph-compiler/src/schedule.rs::buffer_assignments` clones every scheduled `GraphNodeId` into its temporary `positions` map even though the caller retains the schedule for the entire lookup. Replace only that scratch ownership with borrowed typed keys.

Expected production hunk:

```rust
let positions: BTreeMap<&GraphNodeId, usize> = schedule
    .iter()
    .enumerate()
    .map(|(position, node)| (node, position))
    .collect();
```

Sol HIGH coordinates. Astra XHIGH scopes and verifies. Luna HIGH performs this non-delicate mechanical implementation. Astra HIGH performs delicate audio/DSP implementation. Any change to graph semantics, scheduling, PDC, coloring, aliasing, validation, ownership transfer, or audio/DSP stops and requires rescope.

The dependency order is satisfied and frozen: #543/#555 merge `b20b27d5` precedes #558/#552 merge `b95c9b7b`, which precedes #542/#567 delivery `3ac24f7f`; all are ancestors of current main and #552/#558/#542 are closed.

## Exact ownership

Branch: `codex/borrow-buffer-position-node-ids`

Worktree: `/home/bl/misofm/engine-cp1-buffer-position`

Production ownership is exactly the `positions` declaration/initializer in `crates/graph-compiler/src/schedule.rs::buffer_assignments`, baseline SHA-256 `8aedb8eaae1edbaba55093befbe6525a972f4b1abe622b4ae62be5cc57120e13`.

No helper, signature, consumer, test, fixture, dependency, graph construction, SCC/cycle scratch, PDC map, retained output, public identity, artifact, pin, SDK, browser, workflow, or other source is owned. Retain the required owned output clone later in `buffer_assignments`. The issue record owns only this numbered spec and concise #559/#560 state.

Fresh external paths, absent including dangling symlinks before attempt 1, are:

```text
/tmp/cp1-buffer-position-a1-evidence
/tmp/cp1-buffer-position-a1-baseline-target
/tmp/cp1-buffer-position-a1-candidate-target
/tmp/cp1-buffer-position-a1-wasm-scalar-target
/tmp/cp1-buffer-position-a1-wasm-simd-target
```

Never modify or remove predecessor worktrees, branches, evidence, targets, or artifacts. Commit no `.ll`, `.s`, compiler stream, binary, Wasm/SDK artifact, target directory, or raw evidence.

## Frozen behavior

Borrow complete `GraphNodeId` values. Reference-key ordering and equality remain the pointee's derived typed ordering/equality, including independently allocated equal IDs; do not use address or flattened text identity.

Preserve schedule positions and duplicate-key last-position behavior; empty-input and malformed-endpoint behavior; every edge consumer including parallel and sidechain edges; last-consumer positions and output-buffer retention; identity alias eligibility; deterministic smallest-free-buffer reuse; assignment and reduction order; `MainOutput` ports; buffer indices; canonical bytes and SHA; resource fields; diagnostics; transactional owner return; and rendered PCM.

This removes one owned-ID scratch population. B-tree storage and string comparison remain. No allocation reduction, timing, speedup, budget, throughput, or sound-quality claim is authorized.

## Attempt 1 and objective gates

Astra XHIGH must return SCOPE PASS against the exact clean pushed issue/tracker/main identities, GitHub parity, ownership, fresh-path absence, preserved predecessors, and no relevant process before Luna edits source.

Luna HIGH captures the baseline manifest before the one frozen hunk, then candidate evidence using separate external targets. Every record preserves exact command/environment, cwd, head, dirty state, timestamps, complete stdout/stderr, actual tool-return status and persistent session receipts. A status file containing `0` is not execution authority. Stop on first failure; do not retry or continue. Three failed attempts hard-stop without weakened gates or a disguised fourth attempt.

Run once at the coherent candidate checkpoint:

1. Exact changed-path/hunk review: only the initializer above, no clone, unchanged consumers and required owned output clone.
2. `cargo test --locked -p graph-compiler --lib`.
3. `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --lib`.
4. Baseline and candidate `cargo run --quiet --locked -p graph-compiler --bin graph_fixture -- --manifest`; require byte equality. Do not run the known-stale checked-in `graph_fixture --check` or regenerate fixtures.
5. `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`.
6. `cargo fmt --all --check`, `bash scripts/check-graph-policy.sh`, `bash scripts/check-workspace-policy.sh`, and `git diff --check`.
7. `cargo check --locked --release -p graph-compiler --lib --target wasm32-unknown-unknown` with command-local `RUSTFLAGS='-C target-feature=-simd128'` and `RUSTFLAGS='-C target-feature=+simd128'` in the separate scalar/SIMD targets above.

Existing library tests cover fixed buffer assignments and fanout liveness with independently cloned IDs, independent interval checks and a wrong-schedule control, direct-route assignment/resource/binding/PCM behavior, canonical determinism, mutations, and transactional return. No new test or harness is required for this ownership-only substitution. Manifest equality is representative identity evidence, not comprehensive PDC evidence.

Astra XHIGH adversarially reviews source/evidence. Reject scope growth, escaping borrow, address/text identity, changed topology/liveness/PDC/coloring/aliasing/diagnostics, failed gates, manifest drift, or unsupported claims.

## Artifact peer and delivery

Production reachability is `host-web -> host-core -> graph-compiler`; `host-core/src/prepare.rs` invokes the compiler. The preceding ownership-only change altered shipped Wasm bytes, so artifact identity cannot be presumed. After source PASS, Sol may open one separately numbered lane-B artifact applicability/pin peer in slot two. The source child owns no builder, artifact, pin, or browser-lineage work. The peer starts from delivered pin `6745de399c56e322303e2da55d69a5cbd538e075f0fd480c6620b1896d566645`.

Closure requires the artifact peer decision, exact reviewed PR qualification, guarded live-head/base merge, post-main qualification, GitHub synchronization, and eligible clean-worktree cleanup. This closes only the buffer-position ownership child; SCC/cycle scratch, PDC identity maps, graph construction, and retained public identities remain separate CP1 residuals.

## Attempt 1 source checkpoint and Astra XHIGH SOURCE/EVIDENCE PASS

The source checkpoint is `13ce1c1f841d3369b59a3f4c0f2b0a127b3a4166`; `crates/graph-compiler/src/schedule.rs` hashes to `559abff738fe8fe9155065561d4c47fdc13f426c0b8f4a02414d8e15b0d0dfd1`. The exact production hunk changes only the temporary `positions` map from cloned owned keys to `BTreeMap<&GraphNodeId, usize>`, populated from `schedule.iter().enumerate().map(|(position, node)| (node, position)).collect()`. The required later owned output clone remains. Typed reference-key ordering/equality, schedule positions, duplicate-key behavior, all edge consumers, liveness, aliasing, buffer assignment/reduction order, diagnostics, transactional ownership, canonical output, and PCM semantics are preserved.

Astra XHIGH records SOURCE/EVIDENCE PASS. The 110-file external evidence manifest is SHA-256 `11ca5932db5133752ff3475465cb3ff8680583772fd9546b63e4726009c2193b`; baseline and candidate graph-fixture manifests are byte-equal with SHA-256 `aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`. Debug and release-unwind graph-compiler libraries passed 66/66 tests; clippy, fmt, graph policy, workspace policy, diff check, and scalar/SIMD Wasm release checks passed. The original rollout path was `/home/bl/.codex1/sessions/2026/09/09/rollout-2026-09-09T15-00-29-01a086af-476c-72e3-b3e4-91cc52346125.jsonl`.

The review records these limitations. The wrong unchecked 2/3 line assertion gives no credit, and the exact production hunk was independently verified. The preliminary invalid GitHub field and the rejected unexecuted summary command make the phrase “No failure occurred” overbroad without rerunning gates. The evidence linkage has no continuous write monitoring or per-command source hashes. Tracker bodies have content parity but extra terminal line feeds.

This source PASS authorizes only a separately numbered lane-B artifact peer. It authorizes no artifact execution, pinning, PR, merge, performance claim, or CP1 completion.

## Artifact peer PASS

#699 received Astra XHIGH FINAL SOURCE/EVIDENCE PASS at `63404d509107242ddaca1c001739a8973bae6ab6`. The one probe and ten ordered qualification commands establish the six-file shipped artifact at Wasm pin `3a9de0b07c8242922ff773114ce44306b8bae7c3444b3785cbbe8bd18028cc16`; all five non-Wasm authorities are unchanged. Chromium, Firefox, WebKit, resource, hermetic, SDK, matrix, mutation, and diff gates passed. The 96-file Phase 2 evidence manifest hashes to `35fa5ecd27c6f90931b8dbb06e0070207edaa5ea87fed284e93f4c0ae5b8b09f` and preserves the accepted 27-file Phase 1 record.

The review gives no assertion-level credit to non-propagating setup and six-file-validation wrappers; independent observations and later gates establish their required facts. Three metadata `end_epoch_ns` labels contain elapsed time, a saved matrix receipt abbreviates original chunk `27467e`, saved matrix/diff durations differ from original receipts, and continuous monitoring is not claimed. Original tool receipts govern. This authorizes a combined #698/#699 PR only; merge, delivery, closure, cleanup, and CP1 completion remain gated.
