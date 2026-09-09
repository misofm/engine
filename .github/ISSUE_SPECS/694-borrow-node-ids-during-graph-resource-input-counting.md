# Borrow node IDs during graph resource input counting

## Authority and outcome

Parent: CP1 under lane B #560, coordinated through #559 and audit #349. Current baseline is main `6d217d30478226872fb4e5302b98d967c04dd96b`, after #685/#692 delivery. Accounting remains 39 delivered, 1 partial, 81 open, 2 owner dispositions, and 1 historical observation. CP1 is the sole inherited partial; no original open finding may start.

Every production graph compile currently builds temporary `input_counts` scratch in `crates/graph-compiler/src/estimate.rs::resource_estimate` by cloning each `GraphNodeId` into `BTreeMap<GraphNodeId, u64>`. The map only needs typed identities while it borrows the existing node slice. Replace only that scratch ownership with borrowed keys.

Expected production hunk:

```rust
let mut input_counts: BTreeMap<&GraphNodeId, u64> =
    nodes.iter().map(|node| (&node.id, 0_u64)).collect();
```

Sol HIGH coordinates scope, checkpoints, and delivery. Astra XHIGH performs scoping and every verification assignment. Luna HIGH performs this non-delicate mechanical implementation. Any need to redesign counting, admission, ownership transfer, PDC, or audio/DSP behavior stops and requires a new scope; delicate audio/DSP implementation belongs to Astra HIGH.

## Exact ownership

Branch: `codex/borrow-resource-input-counts`

Worktree: `/home/bl/misofm/engine-cp1-borrow-resource-input-counts`

Production ownership is exactly:

- `crates/graph-compiler/src/estimate.rs`, limited to the `input_counts` declaration/initializer inside `resource_estimate`.

Issue-record ownership is this numbered spec plus concise final rows in #559/#560. No public signature, helper, dependency, caller, test source, fixture, PDC, buffer, bank, host, artifact, or pin file is owned. The source child owns no AudioWorklet execution or promotion path. Lane B alone may create a separately numbered applicability/pin peer after source PASS.

Fresh external evidence paths are:

```text
/tmp/issue694-evidence
/tmp/issue694-baseline-target
/tmp/issue694-candidate-target
```

They must be absent including symlinks before attempt 1. Never modify or remove predecessor worktrees, branches, compiler targets, artifacts, or evidence.

## Frozen behavior

Borrow the complete `GraphNodeId`; ordering and equality remain the pointee's derived typed ordering/equality. Do not flatten to text, compare addresses, or change map type beyond key ownership.

Count every edge destination exactly as before, including sidechain destinations and parallel-edge multiplicity. Preserve submix/output reduction counting, maximum input count, empty-map behavior, duplicate-key behavior, and missing-destination `None`. Preserve every checked conversion, addition, multiplication, and evaluation order. Estimator arithmetic failure still produces `graph.resource.arithmetic_overflow` at `$.graph`; cap failures still return both prepared inputs transactionally.

Preserve every semantic and retained estimate field, including `graph_metadata_bytes`, `audio_buffer_samples`, `largest_allocation_bytes`, and session/plan totals. Temporary scratch still allocates B-tree storage. This issue proves only that scratch keys borrow existing IDs; it makes no measured allocation reduction, timing, speedup, throughput, budget, or sound-quality claim. Render code and audio operation order are unchanged.

## Attempt 1 and gates

Astra XHIGH must return SCOPE PASS against the exact clean pushed issue/tracker/main identities, GitHub parity, exact ownership, fresh-path absence, and preserved predecessors before Luna edits production source. The PASS is direct authority for one implementation pass; no intervening scope change is allowed.

Luna HIGH first produces a baseline manifest at the clean issue checkpoint, then makes only the frozen source hunk and produces a candidate manifest. Use separate external Cargo targets and record tool-return evidence with exact command, cwd, head, start/finish, stdout/stderr, and actual status. Do not substitute a file containing `0` for authenticated execution evidence.

Run once at the coherent candidate checkpoint:

1. Exact diff review: one production hunk, `BTreeMap<&GraphNodeId, u64>`, `(&node.id, 0_u64)`, no clone in this initializer, unchanged consumers/arithmetic.
2. `cargo test --locked -p graph-compiler --lib` in debug.
3. `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --lib`.
4. Generate baseline and candidate manifests with the corresponding external targets using `cargo run --quiet --locked -p graph-compiler --bin graph_fixture -- --manifest`; require byte equality. This is representative canonical/resource identity evidence, not a performance or comprehensive PDC claim. Do not run the known-stale checked-in `graph_fixture --check`, regenerate fixtures, or claim that unrelated defect is repaired.
5. `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`.
6. `cargo fmt --all --check`, `bash scripts/check-graph-policy.sh`, `bash scripts/check-workspace-policy.sh`, and `git diff --check`.
7. Release library checks for `wasm32-unknown-unknown` with `RUSTFLAGS='-C target-feature=-simd128'` and `RUSTFLAGS='-C target-feature=+simd128'`, using separate external targets. Do not reopen native AArch64 qualification.

Existing library suites supply the transactional caps, sidechain, scalar dispatch, resource boundaries, owner return, and 100-repeat in-process determinism coverage. No new test or harness is required for this type-and-borrow substitution.

Astra XHIGH adversarially verifies source/evidence. Reject any scope growth, escaping borrow, textual/address identity, changed counting/arithmetic/diagnostics/caps, failed gate, identity drift, or unsupported allocation/performance claim. After source PASS, Sol may create one separately numbered lane-B AudioWorklet applicability/pin peer in the second slot. Source closure requires that peer's decision, exact reviewed PR qualification, guarded merge, post-main aggregate qualification, GitHub synchronization, and eligible clean-worktree cleanup. Three failed implementation attempts hard-stop without weakened gates or a disguised fourth attempt.

Closing this child advances but does not complete CP1. All other owned-string families remain future bounded CP1 work.

## Attempt 1 source checkpoint

- Baseline/reviewed scope head: `c38da118f83ebcb97f17b45bc21a9bd4c88cd879`.
- Pushed source checkpoint: `60ec5cb314e20e018e515927041f6dbe5deb10cf`.
- Exact one-hunk borrowed-key substitution only: `BTreeMap<&GraphNodeId, u64>` with `(&node.id, 0_u64)`.
- Baseline and candidate manifest SHA-256: `aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`; manifests were byte-equal.
- All frozen local gates returned actual status `0`; evidence is retained under `/tmp/issue694-evidence` with separate external targets.
- No test, fixture, artifact, pin, or product-semantics changes were made, and no timing, allocation, or improvement claim is made.
- Astra XHIGH source/evidence review is pending; no artifact or merge authority exists.

## Astra XHIGH source/evidence verdict

SOURCE/EVIDENCE PASS at exact clean pushed evidence head `6ed4e7e7bddf1cd1c3433d5d0d373f1fb2a349f0`, with source checkpoint `60ec5cb314e20e018e515927041f6dbe5deb10cf` and unchanged main `6d217d30478226872fb4e5302b98d967c04dd96b`. The exact frozen borrowed-key hunk and graph resource-counting semantics are preserved. Baseline and candidate manifests are byte-identical with SHA-256 `aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`. Independent 66/66 graph-compiler library tests returned actual exit `0` in session `94679`; all recorded local gates are accepted through the original Luna session attribution. Saved command labels omitted target environment details and abbreviated comparison paths, and zero-valued metadata alone is insufficient, so that evidence limitation is preserved. No artifact, pin, performance, timing, allocation, or improvement claim is made. A separate numbered lane-B AudioWorklet peer is now permitted; source closure and delivery remain pending.
