# Rewrite canonical session sources for exact mono preparation

Status: Astra xhigh scoped and Astra medium scope-approved. Matching GitHub issue: misofm/engine#744. Adapter #54 received PASS and is closed with upstream evidence `17048e7`; this issue is the next implementation slice. Baseline engine `b4e4e11e2ed8578d30930f994169887552feef5d`. User requested Luna xhigh implementation and independent Astra medium verification; that workflow overrides the guide's Sol/Terra labels. Maximum five coherent attempts. Additional fresh-context reviewer creation currently hits the session thread limit; evidence must name the actual independent reviewer without claiming a fresh context.

## Problem and outcome

Preparation can prove a stereo source has identical integer samples in both channels, but replacing its bytes with canonical mono requires a new PCM identity and correct source-channel references in the session. The existing renderer and SDK already accept mono sources and map channel 0 into independent track lanes. Deliver a narrow native control tool that applies verified producer mappings transactionally while preserving every other session value. It does not decode or certify audio equality, introduce a runtime/SDK ABI, or change DSP. CLI encoding and app adoption are separately required successors.

## Scope and smallest closable capability

Own only `tools/session-validator/src/lib.rs`, one focused module under that tool if useful, its existing CLI entry dispatch, focused tests, and its numbered issue. A focused extension to `sdk/test/render-evals.mjs` may be included as consumer evidence; do not change SDK runtime or host DSP. No new Cargo package or production dependency is necessary: typed model mutation can use the already present `session` APIs directly. Reuse existing `host-web` dev dependency for native render equivalence tests.

### Command/API

Add:

    session_validator fold-mono --map <folds.tsv> <canonical-session.json>

Successful stdout is only the complete canonical `S_out`, with its existing final LF. Diagnostics go to stderr. Failure before output emits no document; any process failure makes the caller discard staging. This command reads inputs and writes stdout only, preserving the tool's no-output-file behavior. Existing `validate` behavior remains compatible.

`folds.tsv` is a narrow bounded protocol, not a general migration manifest: each LF-terminated line is exactly `old_sha256_identity<TAB>new_sha256_identity`, with each identity `sha256:` plus 64 lowercase hex characters. No header, blank lines, comment syntax, extra columns, or duplicate old identities. Permit the zero-byte file as the no-fold operation. Bound session input/output to 8 MiB, map input to 256 KiB, and entries to the sparse format's 1024-source ceiling. These are control-tool resource limits, not engine track limits. Read through bounded handles/limits, not unchecked whole-file reads beyond those caps.

A pure internal helper accepting document text and parsed replacement pairs keeps command parsing separate from the typed transformation. It does not certify audio equality; the CLI supplies mappings only after complete source verification. The command's help and result must not claim it decoded audio or proved L/R equality.

### Transformation contract

1. Parse/validate the complete original session with the engine's own APIs. Require source text to equal the engine's canonical output before proceeding; reject noncanonical input rather than normalize unrelated content during a source migration. A valid empty map returns the exact original bytes.
2. Every mapped old identity must occur in the session. Every corresponding source must be PCM16 or PCM24 and have exactly two channels. Multiple source IDs sharing the old identity must have identical source shape, as required by the delivery contract. Reject unknown/duplicate/conflicting mappings, old==new, malformed identities, unsupported source shape, or resulting identity/shape conflicts. New identities can intentionally coincide with an already mono source or another replacement when complete shapes agree; preserve source IDs and coalesce delivery assets later, not source declarations.
3. Clone the typed model off render. For each affected source, set `content` to the supplied new identity and `channels` to 1. Keep source ID, native frame count, bit depth, and root sample rate unchanged.
4. Enumerate every track whose `source_id` refers to any affected source ID. Both formerly valid channel indices 0/1 become 0. This covers original 0/1, 1/0, 0/0, 1/1, and multiple tracks sharing one source. Never infer routing from array position or assume one track per source.
5. Preserve every other typed value: all IDs; input builtins/delays; racks/order/effect identities/parameters; bypass/link modes; both faders/mutes; pan/matrix/smoothing; routes/sends/sidechains/submixes; automation and render/output profile. Do not add editor effects here. Do not convert through a flat mix or app display projection.
6. If at least one source changed, increment the session revision exactly once with checked u64 arithmetic; refuse exhaustion. All sources change in one transaction, not one revision per source. No-op leaves revision and every byte untouched.
7. Compile/validate the final candidate and use the canonical writer. Refuse if any transformed model is invalid. Input file is never modified and no partial accepted model escapes. Unknown session keys remain subject to the existing strict engine schema; “preservation” is not permission to admit unknown fields.

### Objective gates and stop boundary

- Canonical no-op bytes/revision, malformed/noncanonical input, unknown source, invalid map grammar, repeated old identity, mono/float input refusal, shape conflict, revision overflow, and no document on failure.
- Two source IDs sharing one identity and multiple tracks over those sources; all four original stereo map pairs remap correctly. A new mono digest that already exists with matching shape shares an asset without merging or renaming source/track nodes.
- Compare the parsed before/after models with only the permitted source/map/revision fields removed, and also restore those fields in a test clone and require canonical byte equality with `S_in`. Include negative-zero f32 values: ordinary numeric equality alone does not prove identity preservation.
- Nonzero render A/B proof using existing valid launch session fixtures and `AudioWorkletEngineHost`: original stereo supplies `(x,x)`; transformed mono supplies one `x` plane. Both outputs must be nonzero, and at least one asymmetric case must have unequal L/R output. Their complete PCM bits must match between original and transformed sessions. Exercise unequal lane trim/filter/delay, unequal faders, a nonidentity matrix, and a valid native effect fixture; preserve declared route/automation fields.
- Add one bounded SDK Wasm A/B consumer proof in the existing render-evals harness with explicit identical stereo input planes versus mono. Do not use the existing distinct-per-channel signal generator unchanged: that would not represent duplicate stereo. Existing native-vs-Wasm mono evidence remains; no second oracle framework is needed.
- Reuse existing symmetry-witness/mono-collapse gates for the first block after a left-only control change and state divergence, plus one migrated-session live asymmetry case if the existing fixture makes it small. If this reveals a render defect, stop and brief the concrete fix separately; the transformer issue cannot acquire a DSP redesign.
- Run focused tool tests, `cargo test --locked -p session-validator`, relevant formatting/lint checks, and the focused SDK eval through the existing `check-sdk-headless.sh` artifact environment. No benchmark or target-matrix expansion. AArch64/iOS/browser delivery qualification continues in existing package/host gates, not a new mono hardware program.

The engine issue closes when the bounded command and representative semantic tests pass with upstream evidence. Produce a retained build of `session_validator` for the CLI consumer, record source revision, target, executable hash, and build command. This is a control-plane executable dependency; no npm SDK release or AudioWorklet artifact rebuild is implied by tool-only code and SDK test changes.


## Decision and delivery record

Astra medium approves this scope after the Astra xhigh mono plan. Mappings apply simultaneously to original identities: a mapping A->B plus B->C must not cascade A to C. Resolve final identity/shape conflicts after the full transformation, including matching-shape convergence without merging source or track nodes. Native PCM16/24 scope, exact no-op bytes, source/map/revision-only edits, and bounded stdout are acceptance gates.

Start implementation only after adapter #54 is accepted with upstream evidence and synchronized GitHub state. One active implementation issue; root checkpoints exact paths once focused-green and publishes evidence before more implementation. Record commands/results and bounded-memory behavior; independent adversarial review receives one PASS/FAIL per attempt, with the actual reviewer/context limitation disclosed. Close the GitHub issue when PASS evidence is upstream and verify its state. Preserve a source-attested native executable for the CLI consumer; do not commit generated binaries or private audio. No timing or broad target-matrix work is required.

Issue-boundary audit: all 372 local numbered engine specs have matching GitHub issue numbers; prior owned research issue #741 is closed. Historical heading decoration/brief labels do not change those issue identities and are preserved. This new spec title/number and GitHub body are synchronized before implementation. No unrelated issue state or history is changed.

## Attempt 1 implementation evidence (Luna)

The native `session_validator fold-mono --map <folds.tsv> <canonical-session.json>` command now
reads both inputs through bounded handles (8 MiB session, 256 KiB map), accepts only the exact
LF-terminated two-column SHA-256 map grammar, rejects noncanonical input, and transforms one
transactional typed model. Mappings are simultaneous; all mapped sources must be PCM16/PCM24 with
two channels and matching shape, resulting identity/shape conflicts are refused, source IDs and
all unrelated model fields are preserved, every affected track channel reference becomes zero,
and a successful non-empty transform increments revision exactly once with checked arithmetic.
Empty maps return canonical input bytes exactly. Final candidates pass `compile_session` and the
canonical writer; failures emit no document. A bounded SDK render-eval covers duplicate stereo
planes versus the prepared mono source, and native tests cover asymmetric builtins/matrix and a
valid native EQ fixture with exact PCM bit equality and nonzero output.

Evidence from the implementation worktree before root checkpoint `e6ccbd3f`:

- `cargo test --locked -p session-validator`: 8 focused fold-mono tests, 1 skill test, 9 existing validator tests, and doc-tests passed.
- `cargo clippy --locked -p session-validator --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- `bash scripts/check-sdk-headless.sh`: 193 SDK tests passed, including the new duplicate-stereo/mono case.
- `git diff --check`: passed.

No session runtime, SDK runtime/ABI, DSP, codec, package, release, or dependency behavior was
changed. No generated Wasm artifact or executable is committed. The first executable was a
dirty-worktree build based on parent `4651563a949cf81165890b99ea9fd99754061825`, not an executable
attested to that parent alone. Its metadata was
`target=x86_64-unknown-linux-gnu`, `sha256=f3f34b73f3904dd70c4fd2807caf882a05e60f6b4e8114a23e0bb0ae1feb1f42`,
from `cargo build --locked -p session-validator --release`.

Root checkpointed and pushed attempt 1 as `e6ccbd3f`, then merged independently delivered main
`8b1f0cb3` in clean pushed `f4ff06cfed68fd17db20bcc7bf1a232211b43d3d`. After that merge, the
tool's 18 tests, clippy, formatting, 193 SDK headless tests and release build all passed again.
The retained committed-source executable has target `x86_64-unknown-linux-gnu`, SHA-256
`a916ce9c080256bc72f0de460d5e8d1c8850e32ecea3136a4e20c531fb481103`, and build command
`cargo build --locked -p session-validator --release`. Its provenance identifies `f4ff06cf`
and Rust 1.97.1; it remains an unaccepted attempt-1 artifact pending the review correction.

## Attempt 1 adversarial verdict and bounded revision

Astra medium independently reviewed clean pushed `f4ff06cf` and recorded **FAIL** for one
test/evidence finding, with no production-code defect identified. The existing A/B fixture
exercises unequal trim/delay and a nonidentity matrix, but both filters are disabled and its
fader values -0.0/+0.0 have equal gain. The native EQ fixture also has equal filters and faders.
Attempt 2 must exercise valid unequal lane filters and unequal fader gains in an existing
bounded A/B fixture, preserving the separate negative-zero canonical witness, full bit equality
and nonzero/unequal-output assertions. Run and record the existing focused symmetry-witness
and applicable mono-collapse state-divergence/first-left-control-block gates. This is a small
test/evidence revision, not permission to change DSP or introduce another harness.

The reviewer independently passed 8 focused tests and boundary probes for malformed/oversized
maps and source-shape conflicts, all failures producing empty stdout. The reviewer is the
non-implementing Astra medium planning thread; it is independent of Luna implementation but
not a fresh context. The user paused implementation to settle delivery packaging, then approved
resumption with one indexed blob per stem and a 16-byte header. That transport decision does
not alter this tool's mono/session contract. Root authorizes the bounded attempt-2 revision
after this evidence checkpoint is upstream; no new transport code belongs to this issue.
