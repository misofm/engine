# Housekeeping: target-smoke

## Authorized scope and smallest closable slice

Review the complete `crates/target-smoke` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/target-smoke` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker A no-change completion, root verdict recorded below

Read the complete two-file package (`Cargo.toml`, `src/lib.rs`), default-only feature configuration, current lane width/guard definitions, target matrix and relevant #83/#1041/#1062/#1112 scope/decision records. Closed specs were read from git history; no retired scope was reopened. Product/test/manifest changes: **0 lines, 0 tests added or removed**. Only this evidence record changes.

#### Five review axes

- **Repetition/LoC:** the public bootstrap is one small struct initializer. Three literal assertions are keyed to AVX2, NEON and Wasm simd128 and identify the selected platform width independently. Sharing them with the production selector would erase that independence; consolidating two short four-lane diagnostics earns no practical benefit. No cleanup is warranted.
- **Test purpose:** retain the sole `smoke_values_are_canonical` test. It catches a wrong canonical 48-kHz/128-frame bootstrap or a wrong width returned by `Backend::current()` on an enabled target feature. It compares against literal `Simd8`/`Simd4`, not another call to the producer. #1112's existing mutation record specifically reports this test red when the AVX2 selector returns Simd4. The AArch64 debug CI package set includes target-smoke, and the existing Wasm simd128 compile probe includes it; no gate or refusal probe was removed. No additional test is needed for this housekeeping slice.
- **Copies:** the return value contains only small Copy values and the compile-time backend; there is no heap buffer, cloned ownership or repeated preparation work to remove. Returning this public value by ownership is appropriate.
- **Micro SIMD:** there is no PCM, numeric loop or bank kernel in the package. Width selection is a compile-time constant; the actual SIMD implementation is owned by lane. No additional micro SIMD, ISA dispatch or generated-code build is justified here.
- **Data structures:** one fixed-size value, no collection/queue/index/capacity or data-dependent work. An alternative structure would add complexity without removing work.

#### Frozen target boundaries

#83's AVX2/FMA compile-time pin and separate control-plane CPU attestation remain unchanged. `target_smoke()` reports bootstrap data; it neither starts audio nor performs that CPU attestation. #1041's lane guard refuses unsupported/32-bit native targets, #1062 refuses Wasm without simd128, and #1112 keys width/item availability on target features rather than architecture names. This package preserves every literal width assertion, public field/function and CI/refusal owner; no scalar-Wasm replay, architecture change or broad matrix was attempted.

#### Actual gates and limits

- `cargo test --locked -p target-smoke`: **1 PASS**, zero ignored/doc-tests.
- `cargo test --locked --release -p target-smoke`: **1 PASS**, zero ignored/doc-tests.
- `cargo clippy --locked -p target-smoke --all-targets -- -D warnings`: PASS, no warnings.
- `cargo check --locked --tests -p target-smoke` for Wasm `+simd128`, `aarch64-apple-ios` and `aarch64-linux-android`: all PASS. `--tests` also type-checks the corresponding literal width assertions. These are compile checks; the local native test executes AVX2, while NEON execution remains the existing AArch64 CI owner. No mobile-device or Wasm-runtime execution is claimed.
- `cargo fmt --all --check` and `git diff --check`: PASS. Package dependencies/features and all refusal scripts remain unchanged, so their existing qualification evidence applies; no unsupported-target probe was repeated. No allocation measurement, timed benchmark, fixture, harness or listening claim was manufactured for this constant-value function.

All builds used target directory `/home/bl/misofm/engine/target/housekeeping-a`. Raw logs: `/tmp/housekeeping-a-target-smoke-{debug,release,clippy,wasm,ios,android}.log`; exit statuses were checked before the daemon restart and completed logs were reread afterward without repeating the gates. No owner question or product finding remains. Work is paused for the single root attempt-1 verdict.


## Root adversarial verdict: PASS — attempt 1, no product change

Root Sol read the complete two-file package, its five-axis record and actual native/lint/foreign compile logs. The fixed Copy bootstrap has no avoidable allocation, collection, repeated kernel or sample arithmetic. The existing test has an independent purpose: it catches incorrect canonical bootstrap values or a wrong compile-time width using literal expectations rather than the producer itself. Target-feature selection/refusal and CPU attestation remain in their existing owners; foreign builds are explicitly compile-only evidence. No implementation/test/dependency edit, new gate or owner decision is warranted. This crate's housekeeping review is complete.
