# Retire the unused sidecars directory and obsolete disjointness task

## Mission and authorization

The user requested: “Remove 307 and also the sidecars directory. Proceed with other issues: astra medium -> luna max -> astra medium.” Retire the unused directory and its mandatory scan-root assumptions, then close #307 with its history preserved. This is not deletion of the GitHub issue or implementation of its obsolete disjointness proposal.

Baseline inspected: origin/main `c2c1368dcf4b8984fa7bdf8add5807e4336492ae`. The only tracked file under sidecars is README.md. It says delivery codecs, resolution, transport, decode, verification and platform publishing belong in external packages or misofm/cli. #307's existing body concerns a former FLAC decoder under sidecars and former package names; its proposed bidirectional sidecar disjointness gate has no remaining production population. Delivery codecs remain outside this engine. Preserve the original issue text in a clearly marked historical section of this amended issue before replacing its live scope; root owns that archival amendment.

## Smallest closable slice

Delete sidecars/README.md and remove sidecars from existing policy scan populations and fixtures. Preserve every currently covered real root (crates, hosts, tools, and any other gate-specific roots) and all current forbidden patterns, exceptions, semantic checks and error propagation. Required real roots must still fail closed when missing. Existing valid fixtures should pass without creating sidecars at all. No new scan-root abstraction or optional-root discovery.

There is no Rust product, session, wire, DSP, realtime, target, artifact, dependency, or runtime behavior change. No benchmarks or listening qualification are applicable. Qualification/tooling expansion beyond adapting the existing tests is a successor task, not a prerequisite for this retirement.

## Exact allowed implementation paths

Deletion:
- sidecars/README.md

Scan population/configuration and directly corresponding comments:
- scripts/check-bench-policy.sh
- scripts/check-conformance-boundaries.sh
- scripts/check-effect-runtime-policy.sh
- scripts/check-lane-policy.sh
- scripts/check-native-pcm-runner.sh
- scripts/check-realtime-audit-leak.sh
- scripts/check-realtime-policy.sh
- scripts/check-stem-store-v1.mjs
- scripts/check-unfused-seal.sh
- scripts/check-workspace-policy.sh
- scripts/policies/lane-source.toml
- scripts/lib/gate-rules.py

Existing mutation fixtures and fault-injection matchers:
- scripts/test-bench-policy.sh
- scripts/test-conformance-boundaries.sh
- scripts/test-effect-runtime-policy.sh
- scripts/test-lane-policy.sh
- scripts/test-native-pcm-runner-v1-policy.sh
- scripts/test-realtime-audit-leak.sh
- scripts/test-realtime-policy.sh
- scripts/test-workspace-policy.sh

Comment/live policy description consistency only:
- AGENTS.md (package-root list only)
- clippy.toml (scan-move explanatory comment only)
- crates/graph/tests/MUTATIONS.md (current realtime marker scan-root description only)
- docs/REALTIME_DEPENDENCY_POLICY.md (current audit-leak population and “a sidecar ships” assertion only)
- scripts/check-builtins-policy.sh (root-set comment only)
- scripts/check-graph-policy.sh (root-set comment only)
- scripts/check-rack-policy.sh (root-set comment only)

Root-owned issue/evidence bookkeeping:
- .github/ISSUE_SPECS/307-retire-the-unused-sidecars-directory-and-obsolete-disjointness-task.md
- .github/ISSUE_SPECS/README.md only if an index entry needs adding/updating; no unrelated prose changes.

Any additional implementation path requires an amended scope before editing it. No Cargo manifest, Cargo.lock, Rust source, CI workflow, browser artifact/pin or SDK changes are authorized.

## Concrete implementation decisions

1. General workspace scans continue over crates/hosts/tools; bench and realtime-audit-leak scans continue over crates/hosts, preserving their existing tools exemption. Effect-runtime retains fuzz. Stem-store retains crates and both existing named native/mobile host roots. Do not broaden unrelated populations.
2. Conformance's host/sidecar harness ban becomes the existing host-only ban; its required-host check and diagnostic are adjusted together. Workspace manifest/name discovery retains crates/hosts/tools.
3. The lane TOML loader currently requires exactly four root fields. Change that narrowly to exactly three, with the Bash decoder consuming three fields and exactly ten separators instead of eleven. Retain exactly four rule records, all rule identities/order, nonempty-root checks, malformed/short/extra output rejection, loader status rejection, and scanning error behavior. Do not make the loader variadic or generalize its schema.
4. Remove fixture creation of sidecars. Update injected rg/find argument matchers to the actual new root lists, preserving verification that each injected fault reached its intended operation and was rejected for the intended reason.
5. Replace the lane fusion-in-sidecar mutation with fusion under a host package, and replace its missing-sidecars case with a missing real root (hosts). In the unfused seal's embedded self-test, remove hosts for the required-root-missing case and place the empty nested directory control under hosts.
6. Move workspace valid-package, package-prefix and nested-directory-mismatch fixtures from sidecars to hosts, renaming their helpers/case labels accordingly. Move its retired flac-decoder manifest mutation to crates/flac-decoder: rejection of retired package identities is still mandatory, independent of the retired location. Remove the redundant dedicated missing-sidecars helper/case; retain the existing missing-root loop over crates/hosts/tools. Do not remove retired-codec dependency, lockfile, nested-manifest or directory prohibition checks.
7. Existing bench/audit/native-runner missing-root loops drop only sidecars; native-runner's separate missing-tools/tool-surface check remains. Conformance's missing-sidecars mutation becomes missing-hosts and its diagnostic expectation changes with the production check. Other real-root/error controls remain intact.
8. The three-root lane loader contract should be exercised by the existing malformed-field/status tests and a narrowly added wrong-root-count case if those tests do not explicitly reject a four-root policy. This is coupled schema validation, not a new test framework.
9. AGENTS.md's hypothetical “a local sidecar may use local IPC” and CONTROL_PROVIDER_BOUNDARY.md's adapter example remain valid conceptual transport possibilities, not claims that this directory ships code. Leave them unchanged. Preserve historical issue bodies, rulings, migration inventories, snapshots and evidence, including past sidecar paths. Do not perform a repository-wide prose replacement or pursue a zero-occurrence sidecar gate.

## Objective gates

Run each once on the coherent implementation; rerun only checks affected by a correction or unresolved failure:

- bash scripts/check-workspace-policy.sh and bash scripts/test-workspace-policy.sh
- bash scripts/check-bench-policy.sh and bash scripts/test-bench-policy.sh
- bash scripts/check-conformance-boundaries.sh and bash scripts/test-conformance-boundaries.sh
- bash scripts/check-effect-runtime-policy.sh and bash scripts/test-effect-runtime-policy.sh
- bash scripts/check-lane-policy.sh and bash scripts/test-lane-policy.sh
- bash scripts/check-native-pcm-runner.sh and bash scripts/test-native-pcm-runner-v1-policy.sh
- bash scripts/check-realtime-audit-leak.sh and bash scripts/test-realtime-audit-leak.sh
- bash scripts/check-realtime-policy.sh and bash scripts/test-realtime-policy.sh
- bash scripts/check-unfused-seal.sh and bash scripts/check-unfused-seal.sh --self-test
- node scripts/check-stem-store-v1.mjs (normal hermetic mode, no --budgets/browser timing)
- git diff --check; shell syntax checks for changed .sh files; node --check scripts/check-stem-store-v1.mjs; Python AST parsing of scripts/lib/gate-rules.py without generated bytecode.

Review the full diff and tracked-root inventory: no tracked sidecars paths and no physical sidecars directory remain; no active scan/config/fixture still requires it; actual covered roots and exclusions match the contract above. Existing tests must demonstrate normal no-sidecars fixtures pass while required real-root absence and rg/find/loader faults fail. Keep evidence concise: exact checkpoint, commands/statuses, substantive mutation outcomes, changed-path inventory and verdict. Do not pin prose bytes or create a new evidence framework. A full Rust/target/artifact build is unnecessary for this scripts/docs-only slice; normal required delivery CI remains binding.

## Workflow, attempts and delivery

User-specified workflow overrides the guide's default model names: Astra medium scope, Luna max implementation, Astra medium adversarial verification. Maximum five total implementation attempts, each with one coherent implementation pass and one consolidated adversarial verdict. No disguised sixth retry; stop/rescope if exhausted.

Root first synchronizes the checkout, preserves the old issue body, creates this numbered local spec, and amends GitHub #307 to the exact title/body with number/title checked before implementation. The existing issue is reused, never duplicated. Root owns exact-path checkpoint commits, status/upstream audits, PR/merge/required CI, remote evidence synchronization, issue closure and clean completed-worktree removal under the active delivery mode. Implementation stops at a coherent green tranche for root's checkpoint. Closure occurs only after Astra PASS and evidence is upstream; verify GitHub state and retain the original historical rationale. Do not implement #25, create a sidecar, enforce speculative future disjointness, or begin another feature within this tranche.

## Scope decision record

Astra medium scope PASS against the stated origin/main baseline: the unused directory can be retired in this bounded policy/config/test adaptation without changing engine behavior. This scope review inspected sources only and did not run tests or edit the repository. Implementation and its gates remain pending.

## Historical issue body — superseded by the authorized retirement

Found while verifying #305 (the FLAC decoder's move into `sidecars/`).

## The claim, and the measurement

`AGENTS.md` now states that a sidecar is disjoint from the render engine's dependency graph in both directions. That property is the entire justification for the sidecar category — and it was tested directly during verification:

Adding **both** `miso-engine-graph.workspace = true` and `miso-engine-core.workspace = true` to `sidecars/flac-decoder/Cargo.toml`, refreshing the lockfile, and running all 15 fast gates: **every one passed.**

Sidecar disjointness is a convention, not an invariant. Nothing mechanical stops the next sidecar from depending on the render graph.

## Consequence, and why #305 did not fix it

Two gates were deliberately left unextended in #305 on the reasoning that *"a sidecar cannot violate these without first taking a dependency that would break its disjointness."* Verification showed the premise is false, because nothing enforces disjointness. Concrete escapes confirmed:

- `impl PreparedPlanExecutor for` in `sidecars/flac-decoder/src/lib.rs` → `check-graph-policy.sh` **PASSES** (identical mutation in `crates/` reds).
- `impl NativeEffectFactory for` in the sidecar → invisible to `check-effect-contract.sh`'s scan.

**This is not a regression from #305.** Both gates are rooted at `crates` only and never scanned `hosts/` either — the same mutations escape from `hosts/` identically, and always have. #305's only genuine narrowing is that the FLAC crate itself left the `find crates` set, and its `MAX_TRACKS` arm is separately covered by `check-workspace-policy.sh`. That is why it was judged non-blocking.

## What to fix

1. **Enforce the disjointness AGENTS.md asserts.** A gate over each `sidecars/*/Cargo.toml`: no dependency edge to any `crates/` or `hosts/` package, in either direction. This is the load-bearing one — it is what makes the category mean anything, and it would restore the reasoning that justified leaving the two gates alone.
2. Decide whether `check-graph-policy.sh` and `check-effect-contract.sh` should be repo-wide rather than `crates`-only. If disjointness is enforced per (1), they can stay scoped and the argument becomes sound.

## A trap for whoever does extend `check-effect-contract.sh`

`packages+=(-p "${crate_dir#crates/}")` assumes directory basename equals package name. The sidecar directory-naming exemption added in #305 breaks that assumption:

- `sidecars/flac-decoder` → `-p sidecars/flac-decoder` → cargo: *"looks like a file path"*
- the obvious basename fix → `-p flac-decoder` → cargo: *"did not match any packages"*
- the real name is `miso-engine-flac-decoder`

It fails **loudly** under `set -e`, so it is a trip-wire rather than a silent false pass — but it will catch the next person. `check-realtime-audit-leak.sh` shows the correct pattern: derive the package from the manifest's `name =` field, not from the directory.

## Related

- #306 — gate scripts reading `rg` exit 2 as "no violation". Same family: gates that pass because they looked at nothing.
- Two smaller residuals from the same review, not worth their own issues: eleven of the twelve gates extended in #305 degrade *silently* on a missing scan root rather than loudly like `check-workspace-policy.sh`'s `scan_forbidden` does; and `check-flac-decoder.sh:16` is the one place #305 added a `sidecars` root to a bare `if rg …; then`, verified unreachable in practice because the gate dies earlier at rc=101 when the workspace member is absent.

