# Complete compiler and reference source discovery before policy checks

Parent: #401 (kept open until this child and #406 both land); grandparents #306/#349 TOOL-11. Depends on merged #400 and merged #406, so it extends the actual shared helper shape rather than predicting it. This is queued issue #407; implementation waits for its merged prerequisites.

## Smallest closable outcome and exact roster

Three existing gates reject failed/incomplete source discovery while retaining graph ownership, the sole Session JSON format, and the independent conformance/reference boundary:

- `scripts/check-graph-policy.sh`
- `scripts/check-session-policy.sh`
- `scripts/check-conformance-boundaries.sh`

No other gate, runtime/Rust source, benchmark, artifact, or manifest changes belong here. If the concrete three fixtures cannot fit one coherent half-day pass, split session alone before implementation; do not expand during a revision.

## Allowed paths

- the three gates above;
- three small direct hermetic suites: `scripts/test-graph-policy.sh`, `scripts/test-session-policy.sh`, and `scripts/test-conformance-boundaries.sh` (use an existing exact filename if one is found after merge, never a generic mutation framework);
- `scripts/lib/gate.sh` and `scripts/test-gate-lib.sh`, only for minimal backward-compatible checked producer and dependency declaration modes required here;
- this child's numbered issue spec/evidence.

Do not use or edit `scripts/test-builtins-benchmark.sh`. Session currently has no fixture-root argument; either copy the script/helper intact into a disposable minimal repository or explicitly brief a backward-compatible root argument before coding. Do not silently change its CLI.

## Frozen dependency declaration modes and outputs

These modes differ from child A and #400 and must remain deliberate:

| gate / manifest | mode | frozen sorted output |
|---|---|---|
| graph / `crates/graph/Cargo.toml` | exact `[dependencies]` only; current grammar accepts only workspace-suffixed keys and returns the full `$1` key with `.workspace` retained | `effect-contract.workspace`, `engine.workspace`, `lane.workspace`, `rack.workspace` |
| conformance / `crates/conformance/Cargo.toml` | exact `[dependencies]` plus every `[target.*.dependencies]`; full-line key extraction before `=` accepts compact declarations; strip `.workspace`; ignore dev/build/features | `dsp-reference`, `effect-contract`, `engine`, `lane` |
| conformance / `tools/bench/Cargo.toml` | same plain-plus-target mode; target dependencies are mandatory inputs to the union | `bench-support`, `builtins`, `builtins-compiler`, `conformance`, `console-workload`, `effect-compiler`, `effect-contract`, `effect-package`, `engine`, `flatbuffers`, `graph`, `graph-compiler`, `lane`, `protocol`, `rack`, `session`, `sha2` |

Directed fixtures cover compact-key acceptance for conformance, and preserve graph's original `$1` behavior including rejection by its exact-output gate of compact `name.workspace=true`; graph output keeps `.workspace`. Do not normalize graph keys under the other modes. Cover conformance bare/`.workspace` keys, target inclusion, dev/build exclusion, and independent extraction/sort failures including partial output. Do not replace these with “all dependency-like tables.”

## Per-gate frozen semantics

### Graph

Both graph manifests are required. Preserve the exact dependency output above, required compiler `sha2.workspace = true` positive query, and render-graph control-plane ban over `crates/graph/src` plus its manifest. The first discovery producer at lines 32–34 must successfully and non-vacuously enumerate Rust files from both `crates/graph/src` and `crates/graph-compiler/src` before comment-stripped concatenation and the publication/I/O/threading ban. The second producer at lines 44–52 must successfully and non-vacuously enumerate workspace Rust candidates before per-file comment stripping; its exact final production implementation set remains only `crates/graph/src/lib.rs`. Preserve regexes, test-module truncation, roots, CLI fixture root, trap behavior, and `graph policy failure:` diagnostics.

### Session

Preserve the physical script-root behavior and required allowlist/session manifest/source. Check every direct negative and positive rg operation separately (engine reverse edge, session engine/json-syntax presence, parser baggage, publication APIs, allocation vocabulary). Each of the five compile-order anchors must have at least one match from a successful complete scan; use its first numeric line as before, then enforce the existing strict order. Multiple matches retain the existing first-match behavior. Capture the successful scan before selecting its first result; missing/error values must not coerce to zero.

The grouped producer at lines 38–43 contains four independent `find` invocations over six required repository populations: `fixtures/session`, `fixtures/native-pcm-runner`, `hosts/host-web/qualification`, `hosts/host-web/tests/browser-v1`, `sdk`, and `fuzz`. Capture every producer and sort status before looping. Zero TOML matches after a complete traversal is valid because the policy forbids non-allowlisted live Session TOML. Preserve historical allowlist parsing and exact patterns. The retired-spelling rg producer at line 51 may validly filter to empty; its search, glob exclusions, self exclusions, and allowlist handling must complete successfully first.

### Conformance

`crates/dsp-reference/Cargo.toml` and its zero-production-dependency rule are required. Workspace library discovery must successfully traverse all four required roots `crates hosts tools sidecars`; an individual root and an individual manifest may contribute no `[lib]` row (bin-only packages are legitimate), but the aggregate sorted unique library-name set must be nonempty.

Every named production crate (`engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math`) must resolve to one exact directory with a required manifest and readable source root; remove the current silent `[[ -f ]] || continue`. Preserve manifest harness bans, comment exemptions, and the local same-named `mod conformance`/`mod dsp_reference` exemption only after a successful checked module probe. An empty module match is valid; unreadable/error is not. The filtered harness-use scan may validly be empty only after successful source scanning and comment filtering.

Hosts and sidecars are both required roots, but each may cleanly contain no harness match. Their combined producer must distinguish no match from traversal error and partial matches plus error. The reference-use ban requires a successfully derived nonempty production-library pattern and readable `crates/dsp-reference/src`. Preserve the exact dependency modes/outputs above, production roster, allowlists, roots, and `conformance boundary failure:` prefix.

## Tests

Create three direct disposable fixture suites; there is no suitable current named mutation suite. Each has a clean positive control, existing policy violations, intended required-root/surface deletion, injected producer error with valid metadata, and partial-output-then-failure. Graph covers both discovery producers and compiler SHA presence. Session covers all six roots, each of five ordering anchors, expected-empty TOML/retired-spelling results, allowlist read failure, and sort/find/rg failures without touching the real repository. Conformance covers bin-only manifest success, nonempty aggregate library discovery, all mandatory named crate manifests/source roots, local-module exemption, empty hosts/sidecars success, target dependencies, and source/extractor partial failures. Assertions discriminate the named error class and counter-test helper negatives. No Cargo/build or benchmark invocation is needed.

## Common acceptance

For each changed gate: real-tree positive check, all existing relevant violations, explicit required-root/required-surface deletion, clean optional-empty positive, injected producer error with otherwise-valid metadata, and failure AFTER valid partial output. Check producer status before filtering, counting or looping. Test direct/no-match/positive queries separately; filters may validly leave nothing. Error assertions require the intended class, explicit rejection of unexpected success and one counter-mutation per new helper failure mechanism. Preserve physical-script library sourcing, CLI defaults, diagnostics, caller shell state, exact roots/globs/allowlists and no runtime/source changes.

Final gates are affected shell suites, bash syntax, real policy scripts, existing workspace unchanged-count comparison and required CI. No benchmark, artifact regeneration or publication. Root checkpoints one coherent pass; Luna first attempt, Sol only following Astra FAIL (three total maximum), Astra actual PR review before merge. #401 closes only after both children and all seven gate/extractor obligations are upstream/closed; broad #306/TOOL-11 remain open for the rest of their program.

## Numbered program and approved scope

Astra approved this exact brief on 2026-09-05 after the recorded declaration/first-match corrections. #401 remains OPEN until #406 and #407 are upstream and CLOSED; #306 retains its full program obligations. #406 depends on merged #400; #407 depends on merged #406. The actual base and shared API are frozen at assignment. No implementation has started.
