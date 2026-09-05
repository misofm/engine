# Make public and runtime boundary gates reject incomplete scans

Parent: #401 (which remains open until both approved children land); grandparents #306/#349 TOOL-11. Depends on merged #400 and its frozen shared helper API. This is queued issue #406; implementation waits for its merged prerequisites.

## Smallest closable outcome and exact roster

Four existing gates reject failed or partial searches while preserving their public/runtime boundary rules:

- `scripts/check-protocol-control-policy.sh`
- `scripts/check-effect-runtime-policy.sh`
- `scripts/check-host-core-policy.sh`
- `scripts/check-builtins-policy.sh`

No graph, session, conformance, Rust, manifest, artifact, benchmark, or other gate change belongs here.

## Allowed paths

- the four gates above;
- `scripts/test-protocol-control-policy.sh`, `scripts/test-effect-runtime-policy.sh`, `scripts/test-host-core-policy.sh`, `scripts/test-builtins-policy.sh`;
- `scripts/lib/gate.sh` and `scripts/test-gate-lib.sh`, only for the smallest backward-compatible checked-result and plain-dependency declaration-mode extension needed here;
- this child's numbered issue spec/evidence.

Do not invoke or edit `scripts/test-builtins-benchmark.sh`; its graph/builtins references do not substitute for focused policy fixtures and it owns unrelated runner lifecycle behavior.

## Frozen dependency extraction contract

Effect-runtime and builtins share the same current plain-section grammar: enter only exact `[dependencies]`; stop at the next section; accept a key matching `[A-Za-z0-9_-]+` with optional `.workspace` followed by optional whitespace and `=`; derive the key from the complete line before `=`, so compact `name="value"` and `name.workspace=true` remain accepted; strip `.workspace`; sort names. Ignore dev/build/target dependency sections. #400 rack's `$1` parsing is not silently equivalent. Extend semantics narrowly; do not invent a universal TOML parser.

| gate / manifest | frozen sorted output |
|---|---|
| effect-runtime / `crates/effect-contract/Cargo.toml` | `engine`, `lane`, `math` |
| effect-runtime / `crates/effect-compiler/Cargo.toml` | `compressor`, `delay`, `effect-contract`, `effect-package`, `engine`, `gate-expander`, `lane`, `multiband-compressor`, `parametric-eq`, `session`, `soft-clip`, `transient-shaper`, `true-peak-limiter` |
| builtins / `crates/builtins/Cargo.toml` | `effect-contract`, `engine`, `lane`, `math` |
| builtins / `crates/builtins-compiler/Cargo.toml` | `builtins`, `effect-contract`, `engine`, `graph`, `lane`, `rack`, `rack-compiler`, `session`, `sha2` |

Directed helper tests must preserve spaced and compact forms, bare and `.workspace` keys, sorted output, comment tolerance already inherent in the line grammar, and dev/target exclusion. Extraction and sorting errors, including valid partial output followed by failure, must remain errors with caller `pipefail` both on and off and in conditional calls.

## Per-gate frozen semantics

### Protocol control

`crates/protocol/src/controller.rs` and the named `pub trait ControlProvider` body are required. Its awk extraction may not return empty because the trait vanished, could not be read, or awk failed. Scan that body for the existing raw-byte pattern. `MockProvider` is test support: its public-field population may legitimately be empty, but a successful extraction must precede the existing vector/raw-byte filter. `message_wire.rs` and `session_wire.rs` are optional when absent; if present they join `controller.rs`, and unreadable/search failure is red. Preserve the exact payload regex, exclusions, CLI fixture-root argument, and `protocol control policy failure:` prefix.

### Effect runtime

Preserve both dependency boundaries and every current direct ban/presence rule: reverse dependencies (line 13); the effect-package source producer and its three staged allowlist filters (14–23), where a successful filtered empty result is valid; wire/hash vocabulary (28); orphan-header/directory absence (33–34); `repr(C)` (35); state migration in five runtime roots (36); and all 17 required diagnostic presence checks (37–39).

The omitted producer is mandatory scope: `helper_definitions` at lines 101–103 searches `crates/*/src`, then excludes effect-package/dsp-reference; every manifest row at 69–91 is counted exactly at 104–114. A failed or partial source scan must never satisfy a pinned count, including zero-count rows. Preserve `MISO_ENGINE_PRINT_HELPER_MANIFEST=1` as existing fixture support after applicable setup, not a general policy bypass; retain every row/count/owner and exemption.

### Host core

Preserve required capi source, hosts root, at least one host directory, and a `src` directory for every discovered host. Migrate all four forbidden scans: compile pipeline, identity processor, hand-decoded control wire, and forbidden C export from host-core. Preserve all exact-count/presence queries: exactly one `default = []`, one feature declaration, one optional protocol edge, one capi feature edge, exactly two tree-wide `control-provider` occurrences, exactly one facade protocol dependency, and rlib-only crate type. The two `rg | wc` producers at lines 101 and 104 must capture source status before counting; partial output plus error is red. Protocol remains optional/non-default; no new host exemption.

### Builtins

Preserve both dependency boundaries, reverse-dependency ban, and unsafe scan followed by the exact allocation-tracker exclusion. A clean filtered-empty unsafe result is valid only after successful source traversal. Preserve both positive builtins queries at lines 28–29 as required matches with search errors distinct from absence. Retain `MISO_ENGINE_BUILTINS_SKIP_METADATA` fixture behavior and the default real-tree metadata check; do not broaden into metadata/build tooling.

## Tests

Retain every mutation in the four named existing suites. Add focused cases for required roots/surfaces, optional-empty protocol files/MockProvider fields, present-unreadable optional source, every newly checked producer error, and a producer that emits valid partial output before failing. Host tests cover the fourth C-export scan and both exact-count producers; effect tests cover helper definitions/counts as well as the package producer; builtins tests separately cover unsafe source scan and both positive queries. Assertions must name the intended diagnostic class, reject unexpected success, and counter-test each new helper failure mechanism. Foreign-CWD cases must source the repository helper by the physical script path.

## Common acceptance

For each changed gate: real-tree positive check, all existing relevant violations, explicit required-root/required-surface deletion, clean optional-empty positive, injected producer error with otherwise-valid metadata, and failure AFTER valid partial output. Check producer status before filtering, counting or looping. Test direct/no-match/positive queries separately; filters may validly leave nothing. Error assertions require the intended class, explicit rejection of unexpected success and one counter-mutation per new helper failure mechanism. Preserve physical-script library sourcing, CLI defaults, diagnostics, caller shell state, exact roots/globs/allowlists and no runtime/source changes.

Final gates are affected shell suites, bash syntax, real policy scripts, existing workspace unchanged-count comparison and required CI. No benchmark, artifact regeneration or publication. Root checkpoints one coherent pass; Luna first attempt, Sol only following Astra FAIL (three total maximum), Astra actual PR review before merge. #401 closes only after both children and all seven gate/extractor obligations are upstream/closed; broad #306/TOOL-11 remain open for the rest of their program.

## Numbered program and approved scope

Astra approved this exact brief on 2026-09-05 after the recorded declaration/first-match corrections. #401 remains OPEN until #406 and #407 are upstream and CLOSED; #306 retains its full program obligations. #406 depends on merged #400; #407 depends on merged #406. The actual base and shared API are frozen at assignment. No implementation has started.
