# Sol implementation brief — issue 033 sealed listening preparation and human handoff

## Decision and role boundary

**TERMINAL STOP / NO SOL XHIGH PASS / RESCOPED TO ISSUE 111 / REAL LISTENING NOT AUTHORIZED.** Sol
High completed the machine-preparation vertical and its sole bounded correction; Sol XHigh's second
review found the remaining assignment-key commitment gap described below. The two-pass budget is
exhausted and Issue 033 must not be retried.

The technical finish line is **AGENT PREPARATION COMPLETE / READY FOR REAL FACILITATOR**, not Issue
033 completion. Human identities, conditions, playback, answers, observations, reveal and sign-offs
must remain absent. Initial preparation/playback/session/trial/response/reveal/result counters are
all zero.

## Exact authorities

Start from clean `main` `5ee46fcbb68aed854f7fb2ead772335dbc116bce` / tree
`169cb4008ace661e7856a1bd2d449b6bc7ca16e7`. Bind the exact baseline hashes in the spec. The
accepted machine authority is completed Issue 110, **Complete builtin benchmark host metadata run
from a fresh successor namespace**, candidate `47daeda00683acb6e0fd29bafd3ee6d6403cd782` /
tree `1f51a7bba86bbe34afb18567272faa2dc86bc397`, its exact seven-artifact table and counters
`1/1/1/1`. Do not write beneath `target/issue110` or treat stopped Issues 072/109 as PASS.

Preserve the two accepted Markdown preregistrations, template, Issue-002 synthetic format example,
existing checker, product/compiler sources, accepted manifests and all benchmark bytes. The new
canonical records reference those historical hashes; they do not rewrite history.

## Literal implementation

Add a second binary to `miso-engine-builtins-fixture` at
`tools/miso-engine-builtins-fixture/src/listening_main.rs`; use only existing workspace
dependencies. It accepts one strict, permissioned 48-kHz/two-channel/480,000-frame `.mepcm` source
plus its complete provenance, and renders in 128-frame blocks from fresh state. It emits the four
canonical f32 WAVE roles and deterministic probe evidence specified by the issue:

- identity versus 100-Hz-HPF + 1-kHz-LPF for filter ABX; and
- smoothing 0 versus smoothing 64 for the exact nine-event matrix sequence.

Implement the exact attenuation-only RMS match/common -1-dBFS peak cap. Assert deterministic repeat
bytes, finite samples, equal lengths, RMS delta at most 0.1 dB and peak below 0 dBFS. Use opaque
tokens publicly and keep the sole role mapping in a mode-0600 private key.

Under `dsp-research/listening/issue033/`, freeze preparation, response, reveal and qualification
schemas plus the facilitator guide. Under the named `scripts/*-033` paths, implement a strict
stdlib validator/statistics command, zero-playback preflight, single no-clobber preparation wrapper,
and hermetic/static mutation gates. Do not add a player, UI, network service or external package.

## Randomization, records and arithmetic

Use SplitMix64-v1 with a facilitator-owned private seed read only from a regular nlink-1 mode-0600
file, never from arguments, environment or public output. Fisher-Yates shuffle exact balanced ten/
ten vectors independently for 20 filter ABX X assignments and 20 matrix candidate-position
assignments, plus opaque filenames. Commit the canonical private key/schedule hash publicly; do not
expose seed or mapping before sealed responses.

Each of 20 logical trials allows at most two retained attempts. Invalid attempts require a reason,
never contain an answer and reuse the logical trial's frozen assignment. The response validator
accepts exactly 20 valid logical responses, rejects duplicate/missing/pending/synthetic/agent rows,
and binds the immutable pre-reveal hash before reveal.

Compute the spec's exact two-sided ABX binomial p-value and 95% Wilson intervals. Freeze all 21
possible count results against an independent oracle. Statistics have no preference/audibility
threshold and are not run on real data during implementation.

## Lifecycle and no-clobber

`target/issue33` starts absent. Close the target namespace exactly as the spec requires: regular
nlink-1 artifacts, mode-0600 key, closed membership, atomic no-replace publication for authorities,
create-once partial renders, prelaunch/final disposition consumption and no retry/direct/alternate
invocation. Recheck clean HEAD/tree and every current/predecessor authority immediately before the
first render output is created.

Fake lifecycle coverage must prove zero renderer launch on argument/tool/input/provenance,
candidate/hash, Issue-110 membership/inode, mode/link/type, extra-member, dirty-tree and authority-
drift failures. Prove partial failure preservation, public/private separation, no mapping leakage,
second-call refusal and exact phase-derived counters. Fakes may contain synthetic answers only when
prominently typed as format-only and must never be presented as listening evidence.

No real preflight or preparation invocation occurs during implementation. After focused gates,
Sol High pauses with `target/issue33` absent and all counters zero. Sol XHigh reviews the exact
committed tranche. Only a later explicit root authorization may permit one zero-playback preflight,
then a separate review may permit one machine-only preparation run after the licensed source and
provenance are present. That success still leaves all playback/human/response/reveal/result counters
at zero and ends in facilitator handoff.

## Exact path boundary

Allowed implementation paths are only:

- the two Issue-033 docs;
- `tools/miso-engine-builtins-fixture/Cargo.toml` and
  `tools/miso-engine-builtins-fixture/src/listening_main.rs`;
- `dsp-research/listening/issue033/**`;
- `scripts/check-builtins-listening-033.*`, `scripts/test-builtins-listening-033.*`,
  `scripts/preflight-builtins-listening-033.sh`, and
  `scripts/prepare-builtins-listening-033.sh`;
- a minimal exact-path policy/mutation change if required; and
- minimal Issue-033 README/implementation-plan routing.

Any product, accepted fixture/preregistration, Issue-110, benchmark, player, host, network, V1 or
unrelated path is STOP. `Cargo.lock` may move only as a mechanical existing-dependency workspace
registration; a new third-party dependency is STOP.

## Focused gates and handoff

Run only the non-listening gates enumerated in the spec: renderer unit/repeat/rejection tests;
schema/statistics self-tests; hermetic no-clobber lifecycle and mutations; focused locked check/
test and warning-denied Clippy/rustdoc; formatting; applicable policy/mutations; shell/Python syntax;
and docs/title/dependency/conflict/whitespace/diff scans. Static scans must prove no audio-player or
device launch, benchmark/timer/workload, fabricated live row or product mutation.

Report exact changed paths/hashes, clean candidate, preserved Issue-110 evidence, target-033
absence and counters zero. A focused PASS authorizes a checkpoint commit only. It does not authorize
preflight, rendering, playback, human listening, reveal, issue closure or a sound-quality claim.

After the later sealed packet passes Sol XHigh review, hand it to a real facilitator and listener.
Issue 033 remains OPEN until the exact two completed real records, pre-reveal hashes, matching
reveal, statistics, conditions, three sign-offs and adverse-observation disposition pass the full
spec. Root alone owns commits, remote body synchronization and closure.

## Pass-1 implementation handoff (2026-08-22)

Sol High completed the allowed agent-only implementation on local base
`7d2f99f31694a78914632d015efcbcfd31955a68` / tree
`88434df3b55678f4a80564e8347a2dbf0ec8b601`. The focused renderer, schema/statistics, static
mutation, hermetic lifecycle, locked package check/test, warning-denied Clippy/rustdoc, format and
shell-syntax gates are green. The lifecycle's real
preflight/render/playback/session/trial/response/reveal/result counters are exactly
`0/0/0/0/0/0/0/0`; `target/issue33` remains absent. No source audio, private assignment, human
identity, response, observation, sign-off or completed result was created. This handoff requests
Sol XHigh review of a commit-ready checkpoint only and does not authorize preflight, preparation,
rendering or listening.

Sol XHigh's one bounded protocol-closure HOLD was addressed without expanding paths or execution:
the schemas and validator now close every record, the packet carries recomputed non-self member
digests, both seals bind every copied tracked input, response retries terminate at the sole valid
row, and qualification is linked to exact preparation/response/reveal bytes and derived counts.
The expanded hermetic/static rows reject those drifts with all real counters still zero.

## Terminal verifier decision

Pass 2 is a second HOLD and therefore terminal STOP. `validate_linked_qualification` validates the
preparation record and separately binds reveal to the supplied assignment-key file, but never
requires that key SHA-256 to equal the preparation's `assignment_key_sha256` and
`packet_member_sha256["private/assignment-key.json"]`. Preparation/key A may therefore be mixed
with response/reveal/key B and still produce internally correct authority hashes, counts and
statistics. The self-test's unrelated base-preparation key digest demonstrates the defect.

Preserve the exact 18-path tranche as technical input only. `target/issue33` remains absent and
real preflight/render/playback/session/trial/response/reveal/result counters remain
`0/0/0/0/0/0/0/0`. No preflight, render or human activity is authorized. Issue 111, **Close builtin
listening assignment-key authority and prepare the real facilitator packet**, owns the stateless
correction and fresh `target/issue111` lifecycle.
