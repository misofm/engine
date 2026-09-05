# Complete compiler and reference source discovery before policy checks

Parent: #401 (kept open until #406, this child and Session successor #417 all land); grandparents #306/#349 TOOL-11. Depends on merged #400 and merged #406, so it extends the actual shared helper shape rather than predicting it. This is queued issue #407; implementation waits for its merged prerequisites.

## Smallest closable outcome and exact roster

Two existing gates reject failed/incomplete source discovery while retaining graph ownership and the independent conformance/reference boundary:

- `scripts/check-graph-policy.sh`
- `scripts/check-conformance-boundaries.sh`

No other gate, runtime/Rust source, benchmark, artifact, or manifest changes belong here. Astra invoked the pre-implementation half-day split: Session policy and its entire fixture contract now belong to #417, after this issue.

## Allowed paths

- the two gates above;
- two small direct hermetic suites: `scripts/test-graph-policy.sh` and `scripts/test-conformance-boundaries.sh` (use an existing exact filename if one is found after merge, never a generic mutation framework);
- `scripts/lib/gate.sh` and `scripts/test-gate-lib.sh`, only for minimal backward-compatible checked producer and dependency declaration modes required here;
- `.github/workflows/qualification.yml` only to add each new suite immediately after its existing checker (convert the scalar conformance run to a two-line block); preserve all job/router/trigger/expectation behavior and helper wiring;
- this child's numbered issue spec/evidence.

Do not use or edit `scripts/test-builtins-benchmark.sh` or the Session checker/suite. Session CLI/fixture behavior is wholly assigned to #417.

## Frozen dependency declaration modes and outputs

These modes differ from child A and #400 and must remain deliberate:

| gate / manifest | mode | frozen sorted output |
|---|---|---|
| graph / `crates/graph/Cargo.toml` | exact `[dependencies]` only; preserve actual selection regex `^[a-zA-Z0-9_-]+[.]workspace` (no equals requirement); return full `$1` with `.workspace` retained | `effect-contract.workspace`, `engine.workspace`, `lane.workspace`, `rack.workspace` |
| conformance / `crates/conformance/Cargo.toml` | exact `[dependencies]` plus every `[target.*.dependencies]`; full-line key extraction before `=` accepts compact declarations; strip `.workspace`; ignore dev/build/features | `dsp-reference`, `effect-contract`, `engine`, `lane` |
| conformance / `tools/bench/Cargo.toml` | same plain-plus-target mode; target dependencies are mandatory inputs to the union | `bench-support`, `builtins`, `builtins-compiler`, `conformance`, `console-workload`, `effect-compiler`, `effect-contract`, `effect-package`, `engine`, `flatbuffers`, `graph`, `graph-compiler`, `lane`, `protocol`, `rack`, `session`, `sha2` |

Directed fixtures cover compact-key acceptance for conformance, and preserve graph's original `$1` behavior including rejection by its exact-output gate of compact `name.workspace=true`; graph output keeps `.workspace`. Do not normalize graph keys under the other modes. Cover conformance bare/`.workspace` keys, target inclusion, dev/build exclusion, and independent extraction/sort failures including partial output. Do not replace these with “all dependency-like tables.”

## Per-gate frozen semantics

### Graph

Both graph manifests are required. Preserve the exact dependency output above, required compiler `sha2.workspace = true` positive query, and render-graph control-plane ban over `crates/graph/src` plus its manifest. The first discovery producer at lines 32–34 must successfully and non-vacuously enumerate Rust files from both `crates/graph/src` and `crates/graph-compiler/src` before comment-stripped concatenation and the publication/I/O/threading ban. The second producer at lines 44–52 must successfully and non-vacuously enumerate workspace Rust candidates before per-file comment stripping; its exact final production implementation set remains only `crates/graph/src/lib.rs`. Preserve regexes, test-module truncation, roots, CLI fixture root, trap behavior, and `graph policy failure:` diagnostics.

### Conformance

`crates/dsp-reference/Cargo.toml` is required and its exact existing `[dependencies]`-heading ban stays intact, including an otherwise-empty heading; do not reinterpret it as dependency counting. Workspace library discovery must successfully traverse all four required roots `crates hosts tools sidecars`; an individual root and an individual manifest may contribute no `[lib]` row (bin-only packages are legitimate), but the aggregate sorted unique library-name set must be nonempty.

Every named production crate (`engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math`) must resolve to one exact directory with a required manifest and readable source root; remove the current silent `[[ -f ]] || continue`. Preserve manifest harness bans, comment exemptions, and the local same-named `mod conformance`/`mod dsp_reference` exemption only after a successful checked module probe. An empty module match is valid; unreadable/error is not. The filtered harness-use scan may validly be empty only after successful source scanning and comment filtering.

Hosts and sidecars are both required roots, but each may cleanly contain no harness match. Their combined producer must distinguish no match from traversal error and partial matches plus error. The reference-use ban requires a successfully derived nonempty production-library pattern and readable `crates/dsp-reference/src`. Preserve the exact dependency modes/outputs above, production roster, allowlists, roots, and `conformance boundary failure:` prefix.

## Tests

Create two direct disposable fixture suites. Each has a clean positive control, existing policy violations, intended required-root/surface deletion, injected producer error with valid metadata, and partial-output-then-failure. Graph covers both discovery producers, per-file sed reads, exact parser quirk, compiler SHA presence and failed sort after useful output. Conformance covers bin-only manifest success, nonempty aggregate library discovery, all mandatory named crate manifests/source roots, local-module exemptions at their original probe scope, empty hosts/sidecars success, target dependencies, and source/extractor/filter/paste partial failures. Assertions discriminate the named error class. Counter-mutations must run actual acceptance assertions against faulty implementations and demonstrate rejection; constructing bad controls alone is insufficient. No Cargo/build or benchmark invocation is needed.

## Common acceptance

For each changed gate: real-tree positive check, all existing relevant violations, explicit required-root/required-surface deletion, clean optional-empty positive, injected producer error with otherwise-valid metadata, and failure AFTER valid partial output. Check producer status before filtering, counting or looping. Test direct/no-match/positive queries separately; filters may validly leave nothing. Error assertions require the intended class, explicit rejection of unexpected success and one counter-mutation per new helper failure mechanism. Preserve physical-script library sourcing, CLI defaults, diagnostics, caller shell state, exact roots/globs/allowlists and no runtime/source changes.

Final gates are affected shell suites, bash syntax, real policy scripts, existing workspace unchanged-count comparison and required CI. No benchmark, artifact regeneration or publication. Root checkpoints one coherent pass; Luna first attempt, Sol only following Astra FAIL (three total maximum), Astra actual PR review before merge. #401 closes only after #406, #407 and #417 and all seven gate/extractor obligations are upstream/closed; broad #306/TOOL-11 remain open for the rest of their program.

## Numbered program and approved assignment

Astra approved the exact graph/conformance scope after the pre-code Session split on 2026-09-05. Base is merged #406 commit `882277b65ff64780f57c4df33dee127abc6a33e2`, with its actual shared-helper API. #406 is verified CLOSED; #401 remains OPEN until this issue and #417 close, retaining all seven original gates and extractor obligations. Luna gets one coherent implementation pass, then Astra supplies a verdict before any Sol retry. Root owns checkpoints and synchronization.
