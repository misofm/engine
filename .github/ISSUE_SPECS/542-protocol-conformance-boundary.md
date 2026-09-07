# Move protocol conformance fixtures and inline tests out of production modules

Issue #542; parent audit #349, IO-1. Local and GitHub identities must remain synchronized. Read-only brief base:
`e5b86cf315487fcc602db420dc1a6121f1ac4837` on 2026-09-07. Root owns the issue, worktree, Git,
GitHub, checkpoints and delivery. This issue is non-audio work: Sol high coordinates, Luna high
implements attempt 1, and Sol xhigh performs the consolidated adversarial verification. Later
attempts require the ordinary counted verdict/rescope workflow; no implementation starts until
root assigns the numbered issue and its isolated worktree.

## Problem and original obligation

The original IO-1 finding had three independently visible parts:

1. `protocol` ships the 487-line complete-schema corpus, its decoder selector and digest pin, plus
   `complete_all_opcode_fixture`, even though these values exist only to drive conformance tests,
   mutation seeds and the scalar/SIMD Wasm golden runner.
2. `MockProvider` and `MockProviderConfig` shipped on the default production surface.
3. The complete test bodies for `controller.rs`, `message_wire.rs` and `session_wire.rs` remain
   inline in three already large implementation files.

Part 2 is delivered by #369/PR375: the production C ABI uses `SessionControlProvider`, and the mock
exports and definitions are gated by `cfg(any(test, feature = "test-support"))`. Do not reopen or
reimplement it. The current audit reconciliation therefore marks IO-1 partial: `protocol/src/lib.rs`
still declares and publicly exports `COMPLETE_SCHEMA_HASH`, `ConformanceDecoder`,
`ConformanceFrame` and `complete_schema_corpus`; `session_wire.rs` still defines and `lib.rs`
exports `complete_all_opcode_fixture`; the three named implementation files still contain their
large inline test bodies. The audit-era 33.8 percent and line-reduction estimates are historical and
are not acceptance claims.

Close the full remaining IO-1 outcome. Do not declare IO-1 delivered after relocating only the
corpus or only one test module.

## Smallest closable implementation

Make `crates/conformance` the sole owner of the protocol fixture corpus. Add a
`protocol_corpus` module that owns the exact current 46-frame builder, `ConformanceFrame`,
`ConformanceDecoder`, the single `COMPLETE_SCHEMA_HASH` pin and the 39-edit all-opcode fixture.
The module may depend on the public typed `protocol` and `session` surfaces; it must not require a
new production escape hatch. Remove `protocol/src/conformance.rs`, the fixture definition in
`protocol/src/session_wire.rs`, their declarations and public re-exports from `protocol/src/lib.rs`,
and the protocol package's golden runner.

Move the Wasm returned-verdict runner to `crates/conformance/src/main.rs`. It is the package's
same-named `conformance` binary, so its target identity complies with the package/bin naming rule.
Update `scripts/check-protocol-wasm-parity.sh` to build `-p conformance --bin conformance`, locate
that artifact, and mutate/restore the new sole pin and guest files. Preserve its exact contract:
both scalar and simd128 guests execute the exported `main`; success is the one expected returned
zero result; silence, interpreter error/trap, digest mismatch and corpus-count panic are red; the
existing unmutated control and three rebuilt red mutations remain causal. Do not turn a failed
decoder/build/interpreter status into a clean result.

Relocate the existing native corpus and deterministic million-mutation consumers to the
conformance owner, or otherwise make them conformance-package tests without adding a normal
`conformance` dependency to `protocol`. Preserve every existing corpus/mutation assertion and test
name unless a path-only rename is necessary. Protocol unit tests that need the all-opcode fixture
may use the conformance dev dependency only from test-only code; they must not cause fixture code or
an external harness dependency to enter the default protocol library.

Extract the complete inline `#[cfg(test)] mod tests { ... }` bodies from
`protocol/src/controller.rs`, `message_wire.rs` and `session_wire.rs`. Put public-surface tests in
`crates/protocol/tests/` where that requires no API change. Put tests that legitimately inspect
private implementation state in dedicated test-only child files such as
`src/controller/tests.rs`, `src/message_wire/tests.rs` and `src/session_wire/tests.rs`, included by
small `cfg(test)` module declarations. Do not add public or `test-support` exports merely to make an
integration test compile. Preserve all existing tests and assertions; this is organization, not a
test-pruning or coverage-redefinition issue.

`check-conformance-boundaries.sh` currently pins the exact conformance dependency set and treats
protocol's same-named local conformance module specially. Update it for the intentional direction:
the harness may depend on `protocol` (and `session` if the moved all-opcode builder needs its public
model), while production `protocol` may not normally depend on `conformance`. Retain checked
producer/status behavior. Add only a small structural boundary assertion to the existing checker:
the default protocol library no longer declares/exports the corpus or all-opcode fixture, the old
production fixture source/bin paths are absent, and the three named implementation files no longer
contain inline test bodies. This assertion protects ownership and organization; do not build a new
source corpus, byte pin, or implementation-mirror test.

## Frozen behavior

- The corpus remains exactly 46 stable labels and canonical byte frames: 11 commands including
  the all-opcode transaction, 11 successful responses, 18 registered non-OK responses and six
  events.
- The label-and-byte FNV-1a-64 pin remains `0xbdeb_b0f8_1c38_ec42`. There is one authoritative
  literal shared by native and Wasm verdicts. Relocation alone does not authorize a repin.
- Every frame still succeeds through its current typed command, response, event or session
  transaction decoder. The deep transaction remains a typed command-dispatch case.
- The all-opcode fixture remains the current 39 edits, in the same order and with the same strict
  canonical session-derived values.
- The one-million deterministic mutation run keeps the same seed scheduling, mutation generation,
  decoder selection, limit classification and repeatability assertions.
- The protocol wire bytes, public production codec/controller/session types, message IDs, status
  and error precedence, replay behavior, queue/admission semantics and test-support MockProvider
  behavior do not change.
- Default `protocol` consumers no longer compile or expose the conformance corpus/decoder/frame,
  digest pin or all-opcode fixture. Test-only consumers retain them from `conformance`.
- All existing tests from the three extracted modules remain live with their original semantic
  assertions. Moving a test must not turn it into ignored or feature-only coverage beyond the
  feature condition it already had.

## Exact path authority

Allowed implementation paths:

- `crates/protocol/Cargo.toml`
- `crates/protocol/src/lib.rs`
- `crates/protocol/src/controller.rs`
- `crates/protocol/src/message_wire.rs`
- `crates/protocol/src/session_wire.rs`
- removal of `crates/protocol/src/conformance.rs`
- removal of `crates/protocol/src/bin/protocol_wasm_golden.rs` and the empty directory if applicable
- existing/new test-only files under `crates/protocol/tests/`,
  `crates/protocol/src/controller/`, `crates/protocol/src/message_wire/` and
  `crates/protocol/src/session_wire/`
- `crates/conformance/Cargo.toml`
- `crates/conformance/src/lib.rs`
- new `crates/conformance/src/protocol_corpus.rs`
- new `crates/conformance/src/main.rs`
- existing/new focused tests under `crates/conformance/tests/`
- `scripts/check-protocol-wasm-parity.sh`
- `scripts/check-conformance-boundaries.sh`
- only directly stale location/provenance text in `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
  `docs/derivations/241-schema-repins.md` and `docs/derivations/274-parity-repin.md`
- this numbered decision/evidence record, owned by root

No schema, wire-format, decoder, encoder, controller, queue, delivery, host, C ABI, session model,
render, DSP, fixture byte, fuzz corpus, Cargo workspace membership, CI workflow or unrelated
conformance-harness behavior change is authorized. No new generic harness, test-pruning campaign,
benchmark, timing run or artifact publication belongs here.

## Coordination and overlap

CP-20 has a direct mechanical overlap: its planned shared lowercase-hex authority replaces the
three test-only `hex` bodies currently in `controller.rs`, `message_wire.rs` and `session_wire.rs`.
CP-20 must deliver first. Rebase and freeze this issue on that delivered source, then relocate the
already-delegating helpers unchanged; IO-1 owns no hex behavior or authority change.

IO-2 and IO-3 also touch `message_wire.rs`, `controller.rs`, `schema.rs` and their tests. Serialize
them after this relocation, or rebrief them on the delivered paths; do not mix their codec/registry
rewrites into this issue. #369 remains delivered. #140/IO-5 automation, protocol response IO-16,
conformance semantics and all host/audio work remain separate.

## Objective gates and evidence

Run from the exact assigned candidate with commands, numeric exits and selected test counts
retained:

1. Focused native corpus tests prove all 46 names/bytes, the unchanged single hash, all typed
   decoder successes, the deep transaction command dispatch and the exact 39-edit fixture.
2. The existing deterministic million-mutation test passes unchanged in debug and release. Do not
   create a second mutation generator.
3. `cargo test --locked -p protocol` and
   `cargo test --locked -p protocol --features test-support` pass. Record the extracted test names
   and show that none became ignored or silently unselected.
4. `cargo test --locked -p conformance` passes in debug and release for the moved focused protocol
   corpus tests and the crate's existing suite.
5. `bash scripts/check-protocol-wasm-parity.sh --self-test` passes both scalar and simd128 actual
   guests, the unmutated scratch control, the historical inert-invocation rejection, both digest
   mismatch rebuilds and the corpus-count panic rebuild. Record compiler/interpreter statuses and
   actual artifact identities.
6. `bash scripts/check-conformance-boundaries.sh`,
   `bash scripts/check-protocol-control-policy.sh` and
   `bash scripts/check-workspace-policy.sh` pass. The boundary checker must reject a focused
   temporary counterexample that restores one forbidden default protocol fixture export/source;
   restore it byte-exactly. This single structural control is sufficient; do not add a mutation
   framework.
7. A default production protocol build and dependency inspection demonstrate no normal
   `conformance` dependency and no fixture exports. Supported scalar and simd128 Wasm compilation
   remains green through the parity gate. Do not claim that source relocation changes runtime DSP,
   PCM, allocation or speed.
8. Affected strict Clippy and rustdoc, `cargo fmt --all --check`, `git diff --check`, and the ordinary
   repository policy/diff review pass. The complete workspace test population must retain the same
   semantic named tests and zero failures relative to the frozen baseline; changed test-binary
   grouping from extraction is not itself a failure.
9. After source PASS, root performs the ordinary actual-PR-head/current-base Sol xhigh review,
   required `qualification` success, merge, issue/body/evidence synchronization, #349 IO-1 delivery
   accounting, post-main check and completed-worktree cleanup. A local green move is not delivery.

The evidence record must distinguish the baseline, implementation source, structural control,
native results, Wasm artifacts and final reviewed/delivered heads. File movement is credited only
with the production-boundary and maintained-test evidence above. No audio path is touched, and no
sound-quality, realtime-cost or performance gain is claimed.

## Root concurrency decision

CP20 owns a first coherent checkpoint containing the new authority and the three protocol adapters. IO1 may begin nonoverlapping corpus/runner work immediately after its numbered brief is pushed; delay extraction or edits of those three adapters until root integrates the focused-green CP20 checkpoint into this worktree. CP20 must merge before IO1 final delivery. This permits four issues to progress without conflicting source edits. Cargo.lock changes induced solely by approved dependency edges are authorized. All implementation/revision attempts use Luna high under Sol high coordination.

## Implementation dependency clarification

Inspection found tools/bench/src/protocol.rs imports the removed ConformanceDecoder only for an unused From<ConformanceDecoder> for FrameDecoder conversion. Root verified the occurrences and authorizes deletion of exactly that stale import and unused conversion as part of the corpus ownership move. No benchmark workload, corpus, timing, pin or local FrameDecoder behavior changes are authorized. Include a compile check for the affected bench consumer.

Root integrated CP20's focused-green shared authority and three protocol adapters (8ba9c5dc, evidence clarification acd57011) before any IO1 source writes. The first Luna inspection session was interrupted cleanly for integration; it made no implementation edits and is not a failed implementation verdict. Full attempt 1 may now proceed on the integrated source, preserving the three delegating adapters while extracting tests. CP20 delivery remains a prerequisite for IO1 merge.
