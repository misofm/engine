# Finish RT5 capture publication qualification once

GitHub: https://github.com/misofm/engine/issues/606

Final bounded tooling successor to stopped #603 under audit lane A #559, with current delivered main
`6fe8676e1537bc2c952ac87ee2fe31c545438474` after lane-B #598 / PR #604. #600 delivered the entirely
untimed W8 workload; #602 delivered the still-unexecuted narrow Rust timing seam; #603 corrected its
main seal/lifecycle boundary but exhausted its runner correction with five qualification defects.
No predecessor final preflight, real capture entry, or timer ran. This issue fixes exactly those five
defects and owns the sole retained RT5 descriptive capture.

Sol HIGH coordinates documentation, checkpoints, capture authorization, GitHub sync, and delivery.
Luna XHIGH implements one coherent pass. Every scope, source/harness, seal, capture, and exact-head
verification uses Astra LOW. Lane B #605 is the only other active issue and exclusively owns shipped
artifact qualification/pinning. This native tools-only issue cannot change or pin that artifact.

## Smallest closable outcome and ownership

Reuse the frozen Rust `input-symmetry-capture` entry unchanged. Tooling seals/dispositions use issue
606 while the unchanged Rust records retain issue 602. Repair, qualify, preflight, and run that entry
exactly once. Implementation may change only:

- `scripts/input-symmetry-capture-validator.py`;
- `scripts/preflight-input-symmetry-capture.sh`;
- `scripts/run-input-symmetry-capture.sh`;
- `scripts/test-input-symmetry-capture.sh`; and
- focused `artifacts/issue-606-input-symmetry-capture/` evidence.

This spec, focused `docs/audits/` records, and concise #559/#560 handoff status are root-owned.

No Rust source, fixture, manifest, lockfile, runtime crate, policy, workflow, existing issue-602/603
artifact, browser/SDK/ABI source, shipped artifact, or pin may change. One implementation pass and no
runner correction are authorized. A substantive defect in that pass stops this issue and requires a
new successor.

## Exact repair gates

1. **Build environment:** preflight rejects before any build every present variable whose name is
   `RUSTFLAGS`, `RUSTC`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `RUSTDOC`, `RUSTDOCFLAGS`,
   `CARGO_ENCODED_RUSTFLAGS`, or `CARGO_INCREMENTAL`, or begins `CARGO_BUILD_`, `CARGO_TARGET_`, or
   `CARGO_PROFILE_RELEASE_`.
   This covers target-specific rustflags, package/build-override profile inputs, split debuginfo,
   debug assertions, overflow checks, compiler wrappers, target dir, and all previously named inputs.
   A stub-only loop proves representative exact and wildcard-family variables receive their specific
   refusal diagnostic before any build/seal action.
2. **Actual publication faults:** runner accepted-output fault injection creates the collision and then
   executes the same production hard-link operation and status handling as normal publication. The
   disposition fault already follows this rule. Preflight factors its seal hard-link publication into
   a guarded stub-only probe that executes the production operation without building; a collision must
   return nonzero and retain the validated scratch file at the reported recovery path. Production
   success removes scratch only after the hard link exists. No failure may reach READY/PASS.
3. **Exit status:** every child, validator, marker, accepted-publication, persistence, disposition,
   identity, overwrite, and dangling-path failure test captures the runner's process status and asserts
   it is nonzero in addition to checking the exact disposition and diagnostic. No `|| true` may hide a
   required status assertion.
4. **Strict record matrix:** add explicit duplicate rounds `[1,1]` and `[2,2]`, missing round, extra
   round, true `[2,1]` reorder, and literal `NaN`, `Infinity`, and `-Infinity` record mutations. For
   every validator-enforced record/seal identity pair, create an otherwise valid record, mutate only
   that field to another validly typed value, and require the field-specific mismatch diagnostic.
   Cover source commit/tree, binary/fixture/source hashes, argv, cwd, compiler, target, flags, metadata
   commit/Rust version, all paired workload counts, target pair, smoothing, and owners.
5. **Truthful retention:** failed seal publication retains scratch exactly as reported. Failed accepted
   or disposition publication retains raw/validator/temporary recovery bytes. The disposition records
   issue 606, one workload process after launch, exact marker state, and 0/8,192/16,384 completed-prefix
   evidence; a mid-round crash makes no exact total-call claim.

## Frozen capture contract

The workload remains two owners, 512 untimed preparation renders per owner, and two rounds of 4,096
renders per owner: 8,192 timed plan renders per round and 16,384 total. Only
`PreparedRenderPlan::render` is timed. Buffer construction, publication, PCM hashing, bookkeeping, and
record assembly are outside. Both owner digests equal the reviewed #600 digest. A valid result passes
regardless of magnitude and makes no baseline, speedup, floor, budget, or 64-track claim.

The strict issue-606 seal freezes the candidate, binary, all source/script/fixture/lock identities,
toolchain/target/effective flags, exact cwd/argv, target pair, smoothing, owners, counts, and one
workload process. The strict records retain issue 602 because that is frozen in the reviewed Rust
entry. Validator record/seal agreement and exact key/type/range rules remain mandatory.

## Workflow

1. Push and synchronize this brief and #559/#560. Astra LOW must return exact-head scope PASS.
2. Luna XHIGH implements the one pass and produces truthful stub-only qualification plus a restored
   direct central mutation and repository-root checksum manifest. Root commits and pushes the exact
   tranche; Astra LOW returns a decisive source/harness verdict. No final preflight or real timed entry
   may run before PASS.
3. Before preflight, pass the full stub matrix; #600 debug/release qualification unchanged; capture
   arithmetic tests without clock observation; bench check/strict Clippy/rustdoc; fmt/diff;
   workspace/realtime/builtins/graph/lane/bench policies; unchanged #238/#496 debug/release tests; and
   unchanged `Cargo.lock`.
4. Root runs final preflight once at the exact clean approved head. Astra LOW verifies the published
   issue-606 seal, prepared binary, environment contract, and candidate identity. Only then may root
   invoke `bash scripts/run-input-symmetry-capture.sh` exactly once. No retry or resume is permitted.
5. Astra LOW validates raw/accepted bytes, schemas, lifecycle, disposition, hashes, output digests,
   two finite timing records, and honest accounting. Capture PASS depends on validity, not magnitude.
6. Integrate current main, recheck #605 disjointness, obtain Astra LOW exact integrated-head PASS,
   push, run required PR qualification, merge, require successful post-main qualification, synchronize
   and close #606, mark RT5 delivered in #559/#560, verify remote states, and remove clean stopped and
   delivered worktrees. Lane B supplies artifact qualification/pinning applicability.

No original open #559 finding may start until #606 and lane B's remaining partials are delivered.

## Astra LOW scope review

Astra LOW returned **PASS** on exact pushed corrected brief
`e550b290ea21b6264b3b97a88b153e174919af65`. The local/upstream tree is clean, current main is
contained, GitHub #606 is open and synchronized, and #605/#606 are the two disjoint active slots.
The review accepted tooling identity 606 versus frozen record identity 602, all four exact script
paths, complete build-environment rejection including `CARGO_ENCODED_RUSTFLAGS`, actual publication
fault paths, explicit failure statuses, full record/seal mutations, truthful recovery, and the guarded
publication-only probe. Luna XHIGH may begin the sole implementation pass. No runner correction,
final preflight, or capture is authorized by this scope approval.

## Attempt 1 Astra LOW verdict

Astra LOW returned **FAIL** on exact pushed implementation head
`b148ffc079ce0ed51e205fc188e6dd0648fa3aa3`. The exact preflight
`CARGO_ENCODED_RUSTFLAGS` apply `-C lto=fat` to Cargo build dependencies and build scripts; a
compile-only review exited 101 because Cargo's `-C embed-bitcode=no` is incompatible with `-C lto`.
The stub publication lifecycle and all 19 checksum-manifest entries passed, and the reviewer found no
acceptance hole in the redundant record/seal comparisons. No prepared executable, final preflight,
capture entry, or timer ran. Because this issue authorized no runner correction, issue 606 stops and
splits to a narrow build-recipe successor.
