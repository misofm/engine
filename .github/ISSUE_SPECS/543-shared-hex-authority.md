# 543: Share one lowercase byte-hex encoding authority across Rust packages

One-line summary: Deliver the smallest independently closable Rust slice of audit #349 CP-20 by establishing dependency-free `engine::hex_lower`, delegating every live equivalent Rust encoder to it or recording its separately delivered retirement, and preserving all strings, canonical values, digest pins, public adapters, and hashing ownership.

## Amended scope and completion boundary

Issue #543 is the Rust-package slice of #349 CP-20. It establishes `engine::hex_lower` as the shared lowercase byte-hex authority for Rust packages, delegates every live equivalent Rust encoder, and records the separately delivered #545 retirement of the obsolete `rack_fixture` consumer. Its Rust implementation, dependency edges, pins, APIs, hashing ownership, and full workspace qualification are independently useful and remain accepted evidence.

Issue #543 does not close CP-20. Sol XHIGH attempt 1 correctly found that the original workspace-wide census searched Rust only and omitted three live equivalent encoders:

- `sdk/src/core/asset.ts`: `sha256Hex`;
- `hosts/host-web/qualification/qualification.js`: `bytesToHex`;
- `hosts/host-web/web/stem-store/incremental-sha256.js`: `digestHex`.

These are transferred obligations owned by [#552](https://github.com/misofm/engine/issues/552). They are not exclusions and are not evidence that the original finding is complete. The original workspace-wide completion language is superseded by this amended Rust boundary.

Issue #543 may close only as the reviewed and delivered Rust slice. Audit #349 CP-20 must remain `PARTIAL` until [#552](https://github.com/misofm/engine/issues/552) is reviewed, delivered, merged, and synchronized. CP-20 delivery must cite both #543 and that successor.

## Preserved acceptance

- Exactly one dependency-free Rust implementation remains: `engine::hex_lower`.
- All live equivalent Rust encoders delegate to it or were removed by separately reviewed #545.
- Existing adapters, public APIs, SHA-256 ownership, call placement, canonical strings, fixtures, and pins remain unchanged.
- Existing parser, decorated diagnostic/repin, fixed-width integer, uppercase, and word-oriented formats retain the concrete semantic exclusions already recorded by #543.
- The independent fixed-literal engine test and the recorded focused and full Rust workspace gates remain acceptance evidence.
- No claim of cross-language or whole-CP20 completion is made by #543.


## Governing rebrief and retained record

Sol high approved this split because the remaining JS/TS work crosses independently shipped package boundaries. #552 has a matching numbered spec and OPEN GitHub issue, pushed0e147aa2. Its complete contract owns all three discovered non-Rust sites and final cross-language census; no obligation is discarded. Submit this amended claim as attempt2 for actual Sol xhigh review. No Rust implementation or gate changed. The detailed Rust site roster, invariants and gates in the original record below remain applicable; original whole-workspace completion claims are historical and superseded by the governing scope above.

## Historical original brief and implementation record

# 543: Share one lowercase byte-hex encoding authority workspace-wide

One-line summary: Finish audit #349 CP-20 by making `engine::hex_lower` the single implementation of whole-byte-stream lowercase hexadecimal encoding, delegating every current equivalent workspace encoder to it while preserving all strings, canonical SHA-256 values, digest pins, public adapters, and test ownership.

## Authority and current baseline

- Stateless successor to audit tracker #349 finding CP-20 at synchronized `main` `e5b86cf315487fcc602db420dc1a6121f1ac4837`.
- CP-20 originally found at least fourteen lowercase byte encoders in three styles and separately found hand-maintained graph identity text/length twins. The original numeric count is historical, not an acceptance oracle; current source is the scope oracle.
- Delivered #500 / PR #505 already completed the graph node/edge identity text/length half with shared borrowed visitors and allocation-free UTF-8 byte-length accumulation.
- Delivered #506 / PR #507 moved the three bench-package encoders onto `bench_support::digest::hex`.
- Delivered #509 / PR #510 moved five audit one-shot SHA-256 helpers onto that bench-support authority.
- Delivered #512 / PR #513 moved the compressor, parametric-EQ, true-peak-limiter and transient-shaper digest adapters onto that authority.
- Those slices deliberately left production, protocol, fixture, and independently owned test encoders. This issue completes the remaining lowercase byte-encoding obligation; it does not reopen the delivered identity visitor design.

## Frozen authority and dependency decision

Add exactly one dependency-free implementation:

```rust
// crates/engine/src/lib.rs (or one small private module re-exported here)
#[must_use]
pub fn hex_lower(bytes: &[u8]) -> String
```

It returns exactly two lowercase ASCII characters per input byte, preserves leading zeroes, returns an empty `String` for empty input, and allocates only the returned string with exact `bytes.len() * 2` capacity. Use direct nibble lookup/push or an equally bounded implementation; it does not hash, parse, emit prefixes/separators, or perform I/O. Keep the internal name unversioned.

`engine` has no Cargo dependencies, so this creates no cycle and does not pull `sha2` or tooling into the foundation. `graph-compiler` and `protocol` already depend on `engine`. `bench-support` already depends on `engine` (with `realtime-audit`) and retains its existing public `digest::hex` signature as a thin adapter, so its current bench/tool dependents need no new edge. Test-only consumers in `math` and `effect-runtime` may add `engine.workspace = true` only under `[dev-dependencies]`; `native-pcm-runner` may do the same because its remaining adapter is inside tests. `stem-hasher` owns a production `lowercase_hex`, so it may add the direct non-cyclic `engine.workspace = true` dependency and retain its private adapter/call sites. Existing consumers that already name `engine` use it directly. Do not add a new crate, make production crates depend on `bench-support`, or move SHA-256 ownership into `engine`.

This is an off-render formatting helper. Do not call it from a realtime policy region or claim an allocation/performance improvement.

## Current complete in-scope census

At the baseline above, the following are equivalent whole-byte-stream lowercase hex encoders and must delegate to the one implementation. Retain private/public wrapper signatures where removal would widen the diff or alter API ownership; delete only bodies that duplicate byte-to-hex arithmetic.

1. Shared/current adapters and canonical owner:
   - `tools/bench-support/src/digest.rs`: `pub fn hex(&[u8]) -> String` delegates to `engine::hex_lower`; `sha256_hex`, `Sha256Sink::snapshot_hex`, and `finish_hex` keep hashing/counting ownership here.
   - `crates/graph-compiler/src/canonical.rs`: `hex_digest(&[u8])`; `hex_sha256` still owns hashing and canonical evidence plumbing.
2. Graph fixture binaries in the already-engine-dependent graph-compiler package:
   - `crates/graph-compiler/src/bin/graph_fixture.rs`: `sha256_hex` encoding body.
   - `crates/graph-compiler/src/bin/rack_fixture.rs`: `sha256_hex` encoding body.
3. Remaining original test/digest owners:
   - `crates/math/tests/m2_lane_identity.rs`: `hex`.
   - `crates/math/tests/m3_determinism.rs`: `hex`.
   - `crates/effect-runtime/tests/determinism.rs`: `hex`.
   - `tools/native-pcm-runner/src/lib.rs`: test-only `hex_digest`.
   - `crates/protocol/src/controller.rs`: test-only `hex`.
   - `crates/protocol/src/message_wire.rs`: test-only `hex`.
   - `crates/protocol/src/session_wire.rs`: test-only `hex`.
4. Current equivalent encoders elsewhere in the live tree, included so completion does not merely freeze the audit's stale count:
   - `tools/wasm-gates/src/lib.rs`: public `hex(&[u8; 32])` becomes a thin adapter; keep its API and callers.
   - `hosts/host-web/examples/sdk_render_oracle.rs`: finalized digest `.map(format!("{byte:02x}"))`.
   - `hosts/host-web/src/tests.rs`: the three finalized digest byte loops currently near lines 1280, 1555, and 4066.
   - `crates/effect-compiler/tests/migration_terminal.rs`: finalized digest `.map(format!("{byte:02x}"))`.
   - `tools/parameter-metadata/src/bin/lattice_oracle.rs`: finalized digest `.map(format!("{byte:02x}"))`; this package already reaches the retained bench-support adapter.
   - `crates/effect-package/tests/package_v1_qualification.rs`: `hex_string(&[u8])`; the adjacent `hex(&str) -> Vec<u8>` parser is excluded.
   - `tools/stem-hasher/src/lib.rs`: production `lowercase_hex([u8; 32])`.
   - `tools/stem-hasher/tests/conformance.rs`: independent lowercase byte loop. Retain an independent fixed expected literal test even if this helper delegates; do not make an implementation-parity assertion the only oracle.

Before editing, repeat the exact current census with `rg` for `byte:02x`, `b:02x`, nibble tables, and `char::from_digit`. Any additional current function whose semantics are exactly `&[u8]`/`[u8; N]` to unprefixed lowercase two-digits-per-byte text belongs in this issue and must be listed in the evidence record. This closes the semantic duplication rather than an obsolete count.

## Deliberate exclusions

Do not broaden this into a general formatting rewrite. Exclude:

- all hex parsers and fixture decoders (`hex_bytes`, `hex_fixture`, `hex_nibble`, `hex_after`, and equivalents);
- `0x..` comma-separated repin/debug emitters in effect determinism tests;
- uppercase hex output, because case/prefix are part of those evidence formats;
- integer-field formatting such as `{bits:08x}`, `{value:016x}`, JSON numeric strings, random filename tokens, and JSON `\\uXXXX` escaping;
- `tools/wasm-console`'s word-oriented `{word:08x}` digest protocol unless inspection proves that replacing it is byte-for-byte and endian-independent without changing the protocol representation;
- SHA-256 algorithms, incremental update counters, hash call placement, digest word order, fixture contents, canonical graph text, and any pinned expected value;
- the already delivered node/edge visitor and allocation-free length path from #500.

The exclusion boundary is semantic: a full raw byte sequence rendered as two lowercase hex digits per byte is in scope; a parser, decorated debug record, fixed-width integer field, uppercase format, or word protocol is not the same function.

## Smallest implementation and invariants

1. Add `engine::hex_lower` and one independent engine unit test covering empty input, leading zero, all sixteen nibbles, and a non-32-byte input. Expected text must be a fixed literal, not generated with another formatter.
2. Make `bench_support::digest::hex` delegate to it and retain the existing public API and timing/hash-counter behavior.
3. Replace every in-scope body/inline loop with a direct call or a thin existing-signature adapter. Do not rehash finalized digest bytes and do not change any caller's ownership, error, or visibility contract.
4. Add only the dependency edges justified above. Production manifests must not gain `bench-support`; test-only use must remain in dev-dependencies.
5. Preserve exact lowercase strings, leading zeroes, canonical graph SHA-256, package/session/protocol vectors, browser/native parity digests, stem hashes, and all pinned corpus values. No pin update is accepted as part of this issue.
6. Record the final current-source census and each retained exclusion in the issue evidence. CP-20 becomes delivered only when no equivalent independent encoder remains.

## Independent finite gates

Run and record exact argv/cwd/status for:

- `cargo test -p engine hex_lower`
- `cargo test -p bench-support digest::tests`
- `cargo test -p graph-compiler` (includes canonical SHA/fixture behavior)
- `cargo test -p protocol`
- `cargo test -p math --features lane --test m2_lane_identity --test m3_determinism`
- `cargo test -p effect-runtime --test determinism`
- `cargo test -p native-pcm-runner`
- `cargo test -p wasm-gates`
- `cargo test -p effect-package --test package_v1_qualification`
- `cargo test -p effect-compiler --test migration_terminal`
- `cargo test -p stem-hasher`
- the existing host-web native digest/parity tests and the `sdk_render_oracle` example build or its owning existing gate;
- the parameter-metadata lattice-oracle owning test/gate or, if no direct test exists, its exact build plus existing fixed digest consumer comparison;
- `cargo test --workspace` as the original Wave-3 acceptance gate;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo fmt --all -- --check`, `git diff --check`, and `scripts/check-workspace-policy.sh`.

Static completion gate: inspect the final `rg` census. Any surviving unprefixed lowercase byte-stream encoder must either delegate to `engine::hex_lower`/the delegating bench-support adapter or appear in the evidence record with a concrete semantic exclusion above. The independent fixed literal engine test and retained consumer pins are both required; wrapper-to-authority parity alone is correlated evidence.

No benchmark, browser artifact repin, listening test, PCM timing, allocation counter, or new fixture corpus is required. Any existing pin or canonical digest change is a failure to investigate, not authorization to update expected output.

## Overlap, ordering, and delivery

- IO-1's active successor will relocate the complete inline protocol test modules from `controller.rs`, `message_wire.rs`, and `session_wire.rs`. This issue edits the three local `fn hex` bodies inside those modules. Serialize this issue first; IO-1 must rebase/freeze the delivered protocol source and mechanically carry the already-delegating helpers without changing hex authority. The IO-1 owner has acknowledged this prerequisite.
- The current #349 continuation and unrelated audit issues do not authorize edits to these paths. Root owns the numbered local spec/GitHub issue, isolated worktree, exact-path checkpoints, synchronization, integration, and delivery.
- User routing: Luna high implements attempt 1; Sol high coordinates this nonaudio issue; Sol xhigh performs adversarial verification. One coherent attempt, then the standing Sol revision/hard-stop workflow applies.
- This issue makes no runtime, DSP, canonical-format, protocol-format, ABI, hash, or measured-performance claim. PASS means the shared authority and every current equivalent encoding site are complete with unchanged values.

## Decision record

- Chosen authority: dependency-free `engine::hex_lower`, matching the original CP-20 proposal and avoiding a production dependency on a tool crate.
- Retained adapter: `bench_support::digest::hex`, because existing benchmark/tool ownership and SHA update accounting remain valuable; delegation makes it an API adapter rather than an independent implementation.
- Rejected: make `bench-support` the universal authority. It would invert dependencies by pulling a tooling crate into production/control packages.
- Rejected: scope only the seven audit-era paths still named after #512. Live source contains additional equivalent byte encoders; leaving them would preserve the exact semantic duplication and redefine success around a stale count.
- Rejected: absorb decorated repin/integer/word formats. They have different output contracts and would turn a bounded byte encoder consolidation into unrelated representation changes.

## Root concurrency and checkpoint decision

First Luna high tranche: engine authority plus independent literal test and three protocol adapters, then focused engine/protocol tests and pause for root commit/push. Root integrates that coherent checkpoint into #542 before its test extraction. Remaining consumers follow only after checkpoint. IO1 may concurrently work on corpus ownership outside these adapters; CP20 merges before IO1 final delivery. TOOL14 is investigating retirement of rack_fixture.rs and fixtures/rack/v1; do not independently delete that owner. If retirement is approved and delivered, record it as retired rather than introducing a dead adapter. Coordinate this exact overlap, contrary to the initial no-overlap assumption. All implementation and revisions use Luna high, Sol high coordination, Sol xhigh nonaudio verification. Use --locked for commands except the necessary explicit lockfile update induced by the approved new dependency edges; record it.

## Attempt 1 first coherent tranche — Luna high, Sol high coordination

The new engine::hex_lower uses one lowercase nibble table and exact two-character-per-byte output allocation. An independent literal test covers empty input, leading zero, every nibble and a nine-byte input. The three protocol test-local hex adapters now delegate to this authority; no manifests, test ownership or pins changed. Focused engine test passed; protocol suite passed 154 tests across its existing binaries, with zero failures/ignored tests. The initial formatting check returned 1 for the new formatting delta; the subsequent final formatting check returned 0. Both raw results are retained. Exact command/stdout/stderr/status evidence and post-test source identities are preserved in artifacts/issue543-tranche1. The capture helper recorded argv/cwd and output hashes; source identities are explicitly a post-test checkpoint record, not a pre-command attestation.

This is a recoverable implementation checkpoint, not whole-attempt verification or delivery. Remaining consumer census/adapters, affected/workspace gates and Sol xhigh review remain. Root will integrate this checkpoint into #542 before protocol test extraction. No runtime performance claim.

Root corrected the first checkpoint prose immediately after comparing the raw formatting statuses: the initial check was 1, not 0. This correction preserves the failure record and does not rerun or change source.

## Attempt 1 remaining-consumer checkpoint

Luna high delegated the remaining current in-scope byte encoders, except rack_fixture owned by pending #545 retirement. The exact tranche touches20 files (25 insertions/83 deletions), including four approved engine dependency entries in Cargo.lock. Protocol and rack_fixture sources are unchanged. Existing hashing ownership, word-oriented formats, decorated/uppercase/integer emitters and parsers remain distinct; pins are unchanged.

All19 captured records returned0: before/after census; formatting/diff; explicit non-locked bench-support digest test for the approved lock update; locked graph/compiler, math lane identity/determinism, runtime determinism, native runner, wasm-gates, effect-package, effect-compiler, stem-hasher, selected host-web native tests/example build, parameter-metadata tests/lattice build; final checkpoint inspection. Exact commands/environment/pre-command source hashes and raw results are under artifacts/issue543-tranche2. Source implementation is paused.

This focused-green checkpoint precedes #545 integration and final workspace gates. It is not final census acceptance or CP20 closure; Sol xhigh review and remote qualification/merge remain. No benchmark or repinning occurred.

The terminal Luna report is now retained. The additional nineteenth record repeats only final read-only inspection after a harmless printf label warning; no tests or source work were repeated. The worker was paused before root committed7c0d9586; its acd57011 identity is the test-time base. The graph-compiler focused suite passed84 here because it still contains the pending #545 orphan-validator test; #545 removes that one target/test and records83. No final whole-finding count or delivery is inferred from either number.

## Lossless evidence packaging before final review

Luna high replaced14 whitespace-bearing stdout files with deterministic gzip captures and a manifest of original/packed paths, byte counts and hashes. Root independently decoded each and compared it byte-for-byte with its original committed blob at b5317fe2. No source, dependency or test execution changed. The working-tree diff check passed; root will additionally check the full committed range before integration/final review. This is still attempt1 and is not a formal review retry.

## Integrated final qualification candidate

Root verified the full original-base committed diff after lossless packaging e724a02b, then integrated accepted #545 checkpoint c6fa0c0e at f8688ee0 and pushed it. The orphan rack_fixture encoder is absent through that separately reviewed retirement. Sol high now coordinates Luna high final locked workspace tests, strict workspace Clippy, formatting, policy and semantic census on this integrated candidate. #545 must merge before this issue delivery; neither integration nor started gates supplies completion credit.

## Final workspace qualification

All nine final records returned0 on source3a996760: locked full workspace tests; locked workspace Clippy all-targets/all-features with warnings denied; formatting; workspace policy; full original/current-base committed diff checks; semantic and authority censuses; final clean status. Captures include precommand identities and are preserved losslessly under artifacts/issue543-final-qualification. Remaining lowercase-byte arithmetic census consists only of the shared engine authority and explicitly decorated repin emitters. No source, pins or dependencies changed during final qualification. Root will integrate delivered main86d5b4bd, whose source is already present through accepted #545, before actual Sol xhigh whole-attempt verification. This is not delivery or CP20 closure.

The terminal Sol high coordination report confirms actual Luna high launcher provenance and the single successful frozen qualification candidate. An earlier mis-prefixed invocation was interrupted130 before any gate status; no result is credited. Exact untracked generated captures were removed, clean source verified, and the successful run used fixed absolute capture prefixes. The interrupted transcript and successful terminal report are now retained losslessly. Aggregate captured suite results are1709 passed,0 failed,25 ignored,18 filtered across280 result summaries; individual command/output records remain the acceptance evidence.

## Attempt1 Sol xhigh FAIL: cross-language census gap

Actual Sol xhigh reviewed55eb15e0 against86d5b4bd and accepted all enumerated Rust implementation/dependency/pin/qualification evidence, but returned FAIL because three equivalent JS/TS encoders remain in sdk/src/core/asset.ts, hosts/host-web/qualification/qualification.js and hosts/host-web/web/stem-store/incremental-sha256.js. The Rust-only census cannot prove the frozen workspace-wide semantic claim. Original claims are superseded by this explicit incomplete finding.

Root requested Sol high rebrief under the standing split rule for work crossing original crates/dependency boundaries: preserve a smallest Rust product slice and separately numbered concrete JS/TS successor, without introducing a new Wasm ABI merely to call Rust formatting from JS. No narrowed acceptance is adopted yet; CP20 remains partial until the entire residual, including JS/TS, is addressed. No new implementation is authorized before that stateless scope is approved and numbered.

Root integrated current main a3b4ed76 at0c37a68c. Final committed diff check caught terminal blank lines in two copied rebrief evidence files; both are now stored losslessly as gzip with original/packed identities, without changing their content or Rust source. The failed check is acknowledged; final committed-range validation follows the packaging checkpoint.

## Attempt2 Sol xhigh PASS: Rust slice only

Actual Sol xhigh accepted77c85d5b againsta3b4ed76. The amended claim preserves all original obligations through explicit #552 ownership, exact remote issue synchronization and unchanged qualified Rust source. No additional local gate is required for scope/evidence-only attempt2; required PR qualification and exact-head/base final review remain. CP20 must stay PARTIAL until #552 delivers.

## Counted attempt 3: preserve the native runner ABI boundary

Required qualification34108626421 failed on97915c6d: shipped artifact job101699476520 reported the retained pin mismatch, and policy job101699476598 rejected the direct engine dev-dependency. Sol HIGH approved this bounded final correction. Native-pcm-runner's remaining encoder is test-only, but issue073 forbids a direct engine manifest dependency in every section. Route its test-local hex_digest through the existing bench_support::digest::hex adapter under dev-dependencies. Preserve exactly four normal runner dependencies, add no encoder or cycle, and leave engine::hex_lower as the sole Rust implementation. Do not change the checker.

Only tools/native-pcm-runner/Cargo.toml (engine dev-dependency to bench-support), tools/native-pcm-runner/src/lib.rs (one test-local delegate), and the corresponding Cargo.lock edge may change. Luna HIGH implements; Sol XHIGH supplies the counted attempt3 adversarial verdict. A substantive failure stops this attempt sequence. Root owns Git and evidence records.

Frozen focused gates: inspect locked normal and normal/dev dependency trees; bench-support digest tests; all native-pcm-runner tests; strict all-target native-pcm-runner clippy; existing native runner check and both current native runner policy mutation suites; workspace formatting/policy; Rust authority census; exact-path and committed-range diff checks. No benchmark, policy edit, production runner change, or pin edit is authorized. Artifact qualification freezes only after this corrected source checkpoint and remains a separate delivery obligation.

## Attempt3 focused checkpoint

Actual Luna HIGH correction is pushed at e4f46fa808e413507d204e81b6a4ebc27254869c. Exactly three one-line substitutions preserve the four normal runner dependencies and unchanged policy scripts. All12 frozen focused gates returned0, including both policy mutation suites, complete runner tests and strict clippy. Raw commands/streams/statuses, actual model argv, source identities, and original failed CI job logs are retained losslessly with a manifest in artifacts/issue543-attempt3. Sol XHIGH attempt3 review is pending. This exact corrected source is the artifact qualification source freeze; no pin has changed and delivery remains incomplete.

## Counted attempt3 adversarial PASS

Actual Sol XHIGH returned PASS at f2e87ab0b010b5f706038c5eaa8461a1e26fa7a4 against maina3b4ed763c47658e10fc111e2cfcbd141c77f064. It verified exactly three one-line changes, unchanged four normal dependencies and policy checkers, test-only bench-support reachability, sole Rust hex implementation, source identities and all117 lossless records. Gate11 contained an invalid rg -E subscan despite its aggregate zero status: that subscan is not credited. The reviewer independently ran a correct read-only scan and inspected the exact source diff; no substantive blocker remains. Raw evidence is retained unchanged and the full verdict/provenance are in artifacts/issue543-attempt3-review.

Artifact qualification555 remains pending against sourcee4f46fa8. No pin changed, required deliveryCI is not satisfied, and CP20 remains partial pending delivered552. This PASS is implementation acceptance only.
