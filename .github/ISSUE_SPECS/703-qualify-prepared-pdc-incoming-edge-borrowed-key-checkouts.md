# Qualify prepared PDC incoming-edge borrowed-key checkouts

## Authority and outcome

Parent: closed, hard-stopped #702 under CP1, lane B #560, coordinated through
#559/#349. Product main and the isolated baseline are exactly
`ebf404d73d58b72acad8d67ca70759940e034f36`; tracker authority is
`ba04ef090fae57ad1f8629d1b43a35e36b023400`. #701 and #702 remain closed
procedural hard stops and supply no source, qualification, artifact, delivery,
or accounting credit.

This successor separates checkout preparation from verification. Root prepared
both checkouts before scope, then adopted only the already Luna-authored,
preserved incoming-edge borrowed-key hunk from #702 source checkpoint
`9e8c37477bcba15b6fe556d30d55eb48159cac82`. No new implementation or runner
framework is owned. The smallest closable outcome is fresh source qualification
of those prepared immutable inputs.

Sol HIGH coordinates, prepares documentation/checkouts, and performs a read-only
evidence audit. Astra XHIGH scopes and runs verification, then returns in a
separate adversarial review turn for the verdict. Luna HIGH remains the author
of the carried non-delicate mechanical implementation. Astra HIGH remains
reserved for delicate audio/DSP implementation; none is authorized here.

## Prepared checkouts and exact ownership

Candidate branch: `codex/qualify-prepared-pdc-incoming`

Candidate worktree: `/home/bl/misofm/engine-cp1-pdc-incoming-final`

Baseline worktree: `/home/bl/misofm/engine-cp1-pdc-incoming-final-baseline`

The baseline worktree is detached, clean, and pinned to main `ebf404d7`. The
candidate begins at that same commit and contains only this numbered spec plus
the complete canonical transform in `crates/graph-compiler/src/pdc.rs::timings`:

```rust
// baseline
let mut incoming_by_node: BTreeMap<_, Vec<_>> = schedule
    .iter()
    .cloned()
    .map(|node| (node, Vec::new()))
    .collect();

// candidate
let mut incoming_by_node: BTreeMap<&GraphNodeId, Vec<&GraphEdge>> =
    schedule.iter().map(|node| (node, Vec::new())).collect();
```

Baseline `pdc.rs` SHA-256 is
`b33b90388248c137baf18210fd0dbfb98339568f562b80844c1825e83ac0ed10`.
Candidate `pdc.rs` SHA-256 is
`2fc6a8e84dd47dd86f177aae6a206a0caef297829c4f25192382764ff0877e2e`.

Keep `.get_mut(&edge.destination.node)`, `incoming_by_node[node]`, every other
consumer, owned arrival/extent maps, and both `node.clone()` insertions
unchanged. Preserve signatures, typed value ordering/equality, lifetimes,
schedule processing, edge order/multiplicity, main and sidechain participation,
duplicate-ID behavior, diagnostics, default arrivals/tails, integer arithmetic,
overflow/cap precedence, compensation samples/counts, route/delay sorting,
output latency, canonical bytes, resource fields, transactional owner return,
and rendered PCM. No allocation, performance, timing, throughput, budget,
sound-quality, or additional functional claim is authorized.

Do not edit source, tests, fixtures, dependencies, builders, artifacts, pins,
SDK/browser files, workflows, or either prepared checkout during qualification.
Never commit `.ll`, `.s`, compiler streams, binaries, target directories,
generated Wasm/SDK payloads, or raw evidence.

## Preserved failures and fresh evidence

Preserve every #701/#702 branch, worktree, detached checkout, target, lease, and
a1–a3 evidence path unchanged. Their observations explain provenance but grant
no qualification credit here. #702 Attempt 3's corrected 14-record evidence is
sealed by self-excluding manifest SHA-256
`b9b49238838a3cc7d9a55f23258cf2efa9ba38582f014702096eb1b440eb5381`.

Attempt 1 uses one exclusive Astra XHIGH verification executor and these fresh
roots, absent including dangling symlinks before authorization:

```text
/tmp/cp1-pdc-incoming-final-a1-evidence
/tmp/cp1-pdc-incoming-final-a1-baseline-target
/tmp/cp1-pdc-incoming-final-a1-candidate-target
/tmp/cp1-pdc-incoming-final-a1-wasm-scalar-target
/tmp/cp1-pdc-incoming-final-a1-wasm-simd-target
```

Attempts 2 and 3, if needed, replace `a1` with their attempt number. Three failed
attempts hard-stop this issue; no fourth attempt or weakened gate is permitted.

## Qualification

Before execution, Astra XHIGH verifies both prepared directories exist, resolve
to the recorded paths, have the exact heads/source bytes and clean state, and
have no relevant process. It also verifies exact #703 title/body parity,
synchronized #559/#560 trackers, preserved predecessor evidence, the five fresh
roots, dependency order, sole-active issue status, and deferred artifact peer.

After exact-head SCOPE PASS and an exclusive lease, the same named Astra XHIGH
executor captures one fresh baseline manifest from the prepared baseline using
the baseline target, then runs these candidate gates once in order and stops on
the first setup, launch, assertion, or gate failure:

1. `cargo fmt --all --check`.
2. Ordinary committed path/hunk/hash review from explicit main `ebf404d7` to the
   exact reviewed head; require only this spec and `pdc.rs`, the two source
   hashes above, and the displayed canonical transform. Do not inspect an empty
   working-tree diff or use a custom substitution validator.
3. `cargo test --locked -p graph-compiler --lib`.
4. `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --lib`.
5. `cargo test --locked -p graph-compiler --test track_delay a_track_delay_moves_no_pdc_row -- --exact`.
6. Candidate `cargo run --quiet --locked -p graph-compiler --bin graph_fixture -- --manifest`
   using the candidate target; require byte equality with the fresh baseline,
   699 bytes, and SHA-256
   `aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`.
7. `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`.
8. `bash scripts/check-graph-policy.sh`.
9. `bash scripts/check-workspace-policy.sh`.
10. `git diff --check ebf404d73d58b72acad8d67ca70759940e034f36 <exact-reviewed-head>`.
11. `cargo check --locked --release -p graph-compiler --lib --target wasm32-unknown-unknown`
    with command-local `RUSTFLAGS='-C target-feature=-simd128'` and scalar target.
12. The same Wasm check with command-local
    `RUSTFLAGS='-C target-feature=+simd128'` and SIMD target.

Use each prepared directory's exact absolute path as recorded; no checkout
creation occurs during the attempt. Preserve every direct structured setup,
launch, completion and poll result, exact argv/effective environment, cwd,
head/upstream/clean identity, UTC timestamps, and complete streams. A status
file is not execution authority. Do not retry. Root and other coordinators make
no checkout, worktree, or branch mutation until lease release.

After execution, Sol performs a read-only receipt/inventory audit without
altering evidence. Astra XHIGH then adversarially reviews the exact source and
evidence in a separate turn. Reject any directory/head/path/source drift,
inherited-credit claim, manifest mismatch, failed or missing gate, changed
behavior, unsupported claim, or receipt ambiguity that prevents authentication.

## Delivery boundary

After source PASS, Sol may open one separately numbered lane-B artifact
applicability/pin peer in slot two, starting from delivered AudioWorklet pin
`3a9de0b07c8242922ff773114ce44306b8bae7c3444b3785cbbe8bd18028cc16`.
#703 owns no artifact generation, pin, SDK/browser lineage, PR, merge, or issue
closure before that peer's decision.

Closure requires the artifact peer decision, exact reviewed PR qualification,
guarded live-head/base merge, post-main qualification, GitHub synchronization,
and eligible clean-worktree cleanup. CP1 remains partial; no original open
finding starts before this source qualification and its artifact peer finish.

## Attempt 1 FAIL and attempt 2

Astra XHIGH records **Attempt 1 FAIL — procedural preflight failure; one attempt
consumed**. Its original rollout is
`/home/bl/.codex1/sessions/2026/09/09/rollout-2026-09-09T14-49-19-01a086a5-0f2a-7c13-8290-187cb9a4d7ed.jsonl`.
At line 1414 the preflight accessed lease key `source_head`, while the published
lease contained `candidate_head`. Line 1417 records chunk `c833f4`, actual exit
1, and `KeyError: 'source_head'`.

The failure preceded checkout validation and evidence-root creation. No
baseline, candidate gate, session, poll, retry, repository mutation, or later
operation ran. Both prepared checkouts remain clean at their exact heads and
source hashes; all five attempt-1 roots remain absent. Preserve the original
rollout and released lease. Do not manufacture an empty evidence package.

Attempt 2 uses the same prepared checkouts and unchanged twelve gates. Its
exclusive lease must contain both `source_head` and `candidate_head`, with each
equal to the exact candidate head selected by fresh SCOPE. It also contains the
exact `candidate_dir`, `baseline_dir`, and `baseline_head`. The executor first
verifies those literal fields without a custom parser or wrapper, then invokes
direct commands with the recorded absolute paths and heads.

Fresh roots, absent including dangling symlinks before Attempt 2, are:

```text
/tmp/cp1-pdc-incoming-final-a2-evidence
/tmp/cp1-pdc-incoming-final-a2-baseline-target
/tmp/cp1-pdc-incoming-final-a2-candidate-target
/tmp/cp1-pdc-incoming-final-a2-wasm-scalar-target
/tmp/cp1-pdc-incoming-final-a2-wasm-simd-target
```

After a fresh Astra XHIGH exact-head SCOPE PASS and root-published lease, one
Astra XHIGH executor captures one fresh baseline and runs all twelve gates once
in the frozen order. Stop on every preflight, setup, launch, assertion, or gate
failure. No checkout creation, source edit, inherited credit, retry, artifact,
PR, or merge is authorized.

## SOURCE PASS

Astra XHIGH records **SOURCE/EVIDENCE PASS** for Attempt 3 at exact source head
`6a61bf14fd6bad10d76c62a06a3ae3845c70f097`. Attempts 1 and 2 remain
procedural failures with zero qualification credit. The candidate stays
clean/pushed with `pdc.rs` SHA-256
`2fc6a8e84dd47dd86f177aae6a206a0caef297829c4f25192382764ff0877e2e`;
the prepared baseline stays clean/detached at
`ebf404d73d58b72acad8d67ca70759940e034f36` with source SHA-256
`b33b90388248c137baf18210fd0dbfb98339568f562b80844c1825e83ac0ed10`.

The fresh baseline and all twelve candidate gates ran once in order with no
retry and final structured exit 0. Debug and release-unwind library suites each
passed 66/66; the exact track-delay test passed 1/1; baseline and candidate
manifests are byte-identical at 699 bytes and SHA-256
`aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`.
Formatting, committed main-to-head path/hunk/hash review, strict Clippy, graph
policy, workspace policy, explicit committed diff check, scalar Wasm, and SIMD
Wasm all passed. All 15 clean-state records are empty; no relevant process
remains and the lease is released.

External evidence is `/tmp/cp1-pdc-incoming-final-a3-evidence`. Its finite
self-excluding manifest covers 130 unchanged files in sorted relative-name
order, is 11,907 bytes, and has SHA-256
`b2bb2e8c19be27d50a76d976ee9d691862fabd6fd94695b82e2e51320254fd30`.
The original execution authority is
`/home/bl/.codex1/sessions/2026/09/09/rollout-2026-09-09T14-49-19-01a086a5-0f2a-7c13-8290-187cb9a4d7ed.jsonl`.

Evidence supports representative native tests, exact fixture identity, and
scalar/SIMD Wasm compilation. It supports no performance, allocation,
listening, universal PCM-equivalence, browser-artifact, or sound-quality claim.
#703 remains open pending its separately numbered artifact applicability/pin
peer and final delivery workflow. CP1 remains partial.

## Attempt 2 FAIL and final attempt 3

Astra XHIGH records **Attempt 2 FAIL — command-environment failure; second
attempt consumed**. The original invocation and result in
`/home/bl/.codex1/sessions/2026/09/09/rollout-2026-09-09T14-49-19-01a086a5-0f2a-7c13-8290-187cb9a4d7ed.jsonl`
at line 1518 authenticate chunk `1a03f6`, actual exit 127, with no session or
poll. Its non-login `PATH` omitted `/home/bl/.cargo/bin`; stderr is
`env: ‘cargo’: No such file or directory`. The shell ran, but Cargo did not.

Lease read, preflight, evidence creation, and baseline identity setup passed.
No baseline output, candidate gate, retry, checkout/source/branch mutation, or
later operation ran. Both prepared checkouts remain clean and every target root
remains absent. The lease records release. Preserve the 11 data files plus their
finite self-excluding manifest, whose SHA-256 is
`4c8db9c28a7845e1dd0b08912f3e2e32d12f4b1dee63bba7e80ea91c4a6ebc6d`.
The JSON `end` UTC is the later receipt-recording time; the original tool result
returned at `18:18:51.718Z`. It supports no runtime-duration claim.

Attempt 3 is final. It keeps both prepared checkouts and uses these fresh roots,
absent including dangling symlinks before authorization:

```text
/tmp/cp1-pdc-incoming-final-a3-evidence
/tmp/cp1-pdc-incoming-final-a3-baseline-target
/tmp/cp1-pdc-incoming-final-a3-candidate-target
/tmp/cp1-pdc-incoming-final-a3-wasm-scalar-target
/tmp/cp1-pdc-incoming-final-a3-wasm-simd-target
```

After fresh Astra XHIGH exact-head SCOPE PASS and an exclusive lease, every
command invocation uses `login:true`. Preflight directly requires
`command -v cargo` to return `/home/bl/.cargo/bin/cargo` in both prepared
directories. Every Cargo stage invokes the literal binary
`/home/bl/.cargo/bin/cargo` with the unchanged flags and designated target.
Capture one fresh baseline, then run all twelve gates once in the frozen order.

Preserve complete structured setup, launch, completion, and poll receipts plus
exact identities, environment, timestamps, and streams. Stop immediately on
every preflight, setup, launch, assertion, or gate failure. There is no retry or
fourth attempt. No checkout creation, source edit, inherited credit, artifact,
PR, or merge is authorized.
