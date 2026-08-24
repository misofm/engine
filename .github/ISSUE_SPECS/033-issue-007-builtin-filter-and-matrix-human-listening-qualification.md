# 033 Issue-007 builtin filter and matrix human listening qualification

## Outcome and readiness status

Complete the real blinded listening evidence for the exact machine-qualified Issue-007 builtin
candidate before any launch or audible-quality claim.

**TERMINAL STOP / NO SOL XHIGH PASS / RESCOPED TO ISSUE 111 / REAL LISTENING NOT AUTHORIZED.** The
smallest honest agent-executable tranche was preparation only: freeze the accepted
candidate and source identities, render and seal the exact stimuli without playback, preregister a
concealed balanced schedule, validate the response/reveal/statistics formats, and produce a
no-clobber facilitator packet. It must stop at **AGENT PREPARATION COMPLETE / READY FOR REAL
FACILITATOR** with Issue 033 still open. No coding agent may supply a listener identity, answer,
observation, sign-off, playback condition or completed listening result.

Sol High is the implementer and Sol XHigh is the briefer/adversarial verifier. The implementation
budget is at most two total passes: one focused Sol High pass and, only after a concrete Sol XHigh
HOLD, one bounded correction. A second HOLD is terminal STOP/rescope. Human facilitation is not an
implementation pass and cannot cure a defective or unsealed preparation packet.

At this briefing checkpoint all preparation, playback, human-session, trial, response, reveal and
listening-result counters are zero. No audio was rendered or played and no human listening command
ran.

## Context and accepted authority

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. Sound quality and DSP correctness are release criteria.
Objective fixtures and machine conformance do not replace documented listening, and an agent
cannot fabricate human listeners, acoustic conditions or responses.

Issue 007 stopped after three attempts and retained only its explicitly accepted scalar DSP/runtime
slice and listening methodology. The two historical preregistrations remain immutable methodology
inputs:

- `dsp-research/listening/issue007-filter-abx-preregistration.md`, SHA-256
  `be3a9c192f2e73783af6f16e6268f065fc5862b1688cf02e791faf2ba666060c`;
- `dsp-research/listening/issue007-matrix-ramp-preregistration.md`, SHA-256
  `66316d8fd6a2a25fb904373e8a88c6802fbc1c60fa2f95ed02f549b423d014a9`;
- `dsp-research/listening/TEMPLATE.md`, SHA-256
  `18866f68bcb416732fb43b9a6e27cf9d042e12f68e30d9169b6cc60f1f9d18c2`; and
- `scripts/check-builtins-listening.sh`, SHA-256
  `6a3628687f31007b402e706fc09b0bb4d5eb85303fb9abd3b0a1921e859c8cd8`.

The final machine authority is completed Issue 110, **Complete builtin benchmark host metadata run
from a fresh successor namespace**, not stopped Issues 072 or 109. Its clean candidate is commit
`47daeda00683acb6e0fd29bafd3ee6d6403cd782`, tree
`1f51a7bba86bbe34afb18567272faa2dc86bc397`. Its exact accepted benchmark artifacts are:

| `target/issue110` artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `completion.seal.json` | 2,988 | `3ce39b2653d6b912b6ede083fe8479e46bcbce665095190bd94d15fe82ca238d` |
| `miso_engine_builtins_bench` | 3,200,296 | `a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912` |
| `builtins-benchmark.preflight.json` | 1,893 | `9a7a78748b32d8a7cdee1bf7e886e38e6a358f6dfd093d93bbd51bdac2eddaa0` |
| `builtins-benchmark.raw.jsonl` | 38,477 | `8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc` |
| `builtins-benchmark.jsonl` | 38,477 | `8a2d3f2f9f6d5a6f2edb4513fd304b121c934f6dcc1f5379b96f4256b54aa2dc` |
| `builtins-benchmark.validator.stderr` | 211 | `7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396` |
| `builtins-benchmark.disposition.json` | 1,075 | `361f3a4f612e88dcc8a6dcb9f810528b175a64fbf3eea07122024df7971f274f` |

Those seven files are regular nlink-1 files; raw and accepted bytes are identical but their inodes
are distinct; the Issue-110 prelaunch disposition is absent. Issue-110 counters are permanently
`1/1/1/1` for preflight/runner/workload/timed, and no further Issue-110 execution is authorized.
Issue 033 must only read and hash this evidence; it must never write beneath `target/issue110`.

The docs baseline is clean `main` commit
`5ee46fcbb68aed854f7fb2ead772335dbc116bce`, tree
`169cb4008ace661e7856a1bd2d449b6bc7ca16e7`. Current authorities are:

- `Cargo.lock` SHA-256
  `4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`;
- `crates/miso-engine-builtins/src/lib.rs` SHA-256
  `9095eb93a0c04b1eabf95d95a94c4490dd20887ed602f2ce4ed3d90724980f79`;
- `crates/miso-engine-builtins-compiler/src/lib.rs` SHA-256
  `1b7fad8a72fc76ffcb97e31c11bfa386168b9c9c688083069c24623f3e2afc75`;
- `fixtures/builtins/v1/MANIFEST.tsv` SHA-256
  `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`; and
- `fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm` SHA-256
  `69968e143ce9b3920825d3c3a308eb7f6d042ebd44c5cbeea836118184241795`.

`target/issue33` is absent. These baseline hashes identify the starting point; the later preparation
seal must bind the clean post-implementation commit/tree and prove that product sources, accepted
fixtures, historical preregistrations and Issue-110 evidence did not drift.

## Dependencies by exact issue title

- Complete builtin benchmark host metadata run from a fresh successor namespace
- DSP research corpus and conformance harness

Stopped Issues 072 and 109 are transitive historical evidence through Issue 110, not direct PASS
dependencies or current authorities.

## Scope: smallest agent-executable tranche

Add one off-render listening-preparation binary beside the existing builtin fixture tool, strict
stdlib validators/statistics, one zero-playback preflight, one no-clobber preparation wrapper,
hermetic lifecycle/mutation tests, schemas and a facilitator guide. The tranche may render exact
PCM/WAVE files and validate synthetic/fake records, but it may not open an audio device, invoke a
player, collect a human response, reveal a live assignment or calculate a real listening result.

No engine runtime interface is added. The production builtin and compiler sources, benchmark tool,
benchmark scripts, accepted fixtures and Issue-110 artifacts are read-only. A new listening binary
may call the accepted public `miso-engine-builtins` control/offline rendering API; inability to do
so without a product change is a product/API blocker and stops Issue 033 for rescope.

## Frozen source and render contract

The facilitator must supply one exact, permissioned source before the real preparation invocation:

- strict `.mepcm` v1, stereo planar finite `f32`, exactly 48,000 Hz and 480,000 frames;
- no implicit SRC, channel conversion, crop, normalization or decode in Issue 033;
- peak at or below `0.5`, with the first and last 480 frames of both channels exactly zero; and
- a canonical provenance record naming the source SHA-256, conversion tool/version and command,
  rights holder, license/permission, redistribution status and retention location. If the audio
  cannot be redistributed, the private source stays outside Git and the packet records its exact
  reproducible private-artifact reference. Missing or ambiguous permission is a pre-render STOP.

The repository-owned deterministic probe is the accepted 48-kHz dual-mono PRNG fixture named
above. It is rendered and checked as machine evidence but does not substitute for the licensed
excerpt or count as a human response.

Render the source in 128-frame blocks from fresh state. Emit canonical stereo interleaved IEEE-f32
little-endian WAVE files with a 44-byte RIFF/WAVE header, format tag 3, two channels, 48,000 Hz,
block align 8, 32 bits/sample, one `data` chunk and no optional chunks or trailing bytes. The exact
four listening roles are:

1. filter comparator: identity builtins;
2. filter candidate: both lanes use a 100-Hz HPF followed by a 1-kHz LPF, with all other controls
   identity;
3. matrix comparator: the frozen target sequence below with `smoothing_samples=0`; and
4. matrix candidate: the same sequence with `smoothing_samples=64`.

The matrix begins at `[ll,lr,rl,rr]=[0.7,0.3,-0.2,0.8]`. At exact sample indices
`48000,96000,...,432000`, set the target before processing that sample, alternating
`[0.6,0.4,-0.4,0.6]` then `[0.9,-0.1,0.2,0.8]`. These values and event locations never depend on
answers.

For each candidate/comparator pair, compute combined-channel RMS in `f64`; attenuate only the
louder render to the quieter RMS; then apply one common gain to both only if needed to cap the
larger peak at `10^(-1/20)` (-1 dBFS). Multiplication is performed once per sample and cast to
`f32`; there is no limiter, dither or hidden processing. Recompute and record exact `f64` RMS,
RMS-difference dB, peak and SHA-256 values after matching. Preparation rejects non-finite samples,
RMS difference greater than `0.1 dB`, peak not strictly below `1.0`, unequal lengths, product/
source drift or nondeterministic repeat output.

Output filenames are opaque random tokens. A private mode-0600 assignment key is the only artifact
that maps tokens to the four roles. Public manifests contain token, length, format and hash but no
candidate/comparator words, source path, assignment seed or reconstructable mapping.

## Frozen preregistration and concealment

The preparation packet owns two procedures and no others:

- `issue007-filter-abx-v1`: 20 valid logical ABX trials; exactly ten have X=candidate and ten
  X=comparator.
- `issue007-matrix-ramp-v1`: 20 valid logical randomized A/B trials; exactly ten present the
  candidate first and ten second.

Use the accepted SplitMix64-v1 algorithm. The facilitator supplies one private 64-bit seed before
preparation through a regular nlink-1 mode-0600 file, never an argument, environment variable,
stdout or public record. Independently Fisher-Yates shuffle each balanced ten/ten vector and the opaque token
names. The private assignment key stores the seed and mappings; the public preregistration stores
only SHA-256 commitments to the canonical key and schedule. The listener must never receive or
access the private directory. Assignments remain concealed until all response bytes are closed,
made regular nlink-1, hashed and signed off by the listener and facilitator.

Each logical trial permits at most two attempts, so each procedure has at most 40 retained attempt
rows. Hardware interruption, accidental unblinding or failure to complete a presentation is an
invalid attempt with a required reason; it never becomes an answer and does not change that
logical trial's frozen assignment. Stop after exactly 20 valid logical trials or declare the record
incomplete after the bounded attempts. Training is non-answer-bearing and is logged separately.

## Schemas, validation and statistics

The tracked schema freezes four canonical UTF-8/LF, sorted-key JSON/JSONL records:

- `Issue007ListeningPreparationV1`: clean product/tool authority, Issue-110 table, source and
  provenance, exact render parameters, opaque stimulus hashes, level/peak results, assignment-key
  commitment, packet membership/modes and zero human counters;
- `Issue007ListeningResponseV1`: procedure, logical trial, attempt, validity/reason, opaque answer,
  confidence on the fixed `0..100` integer scale, bounded observation text and monotonic sequence;
- `Issue007ListeningRevealV1`: pre-reveal response hash, assignment-key hash, exact token/role and
  trial mappings, reveal UTC time and three pseudonymous sign-offs; and
- `Issue007ListeningQualificationV1`: all authority hashes, counts, computed statistics, adverse
  observations, deviations, bounded conclusion, corrective links and `PASS|FAIL` disposition.

Unknown keys, wrong types/versions/IDs, absolute paths in distributable records, duplicate or
missing members/trials, invalid UTF-8/control text, fabricated/synthetic/agent listener markers,
post-reveal response drift, uncommitted mappings, noncanonical order, wrong counters/hashes/modes,
out-of-range confidence and any pending placeholder in a completed record reject.

For ABX let `k` be correct answers among `n=20`. Report the exact two-sided binomial p-value
`min(1, 2 * sum(C(20,i), i=max(k,20-k)..20) / 2^20)`. For both ABX accuracy and matrix candidate
preference, report the two-sided 95% Wilson score interval with
`z=1.959963984540054`. Emit numerator/denominator for the exact p-value and 17-significant-digit
decimal statistics. The stdlib implementation and an arithmetically independent test oracle must
agree on all `k=0..20`; statistics are descriptive and define no preference or audibility
threshold.

## No-clobber lifecycle and facilitator packet

All persistent Issue-033 artifacts live under fresh `target/issue33`; never reuse a predecessor
namespace. The preparation seal, sealed binary, preflight seal, private key, public packet,
prelaunch disposition and final preparation disposition are regular nlink-1 files/directories with
exact closed membership. Seals, key, public manifests and dispositions publish by no-replace rename;
render files are created once so partial failure evidence remains. Symlinks, hardlinks, devices,
extra members or an existing prelaunch/final disposition reject before rendering. Either
disposition consumes preparation authority; there is no retry, overwrite, alternate/direct
invocation or second namespace inside Issue 033.

Preflight may build/hash the binary, validate schemas and fake records, and publish only the binary
and preflight seal. It must not execute the renderer, open audio, create stimuli or key material,
or increment preparation/listening counters. After a focused Sol XHigh PASS on the clean committed
tranche, root may separately authorize at most one machine-only preparation invocation. That
invocation renders but never plays audio. Success records
`preflight_invocations=1`, `preparation_invocations=1`, and exact zeros for
`audio_playback_invocations`, `human_listening_sessions`, `human_trial_attempts`,
`valid_human_responses`, `reveal_invocations` and `completed_listening_records`.

The listener-visible facilitator packet contains the immutable preregistrations, opaque stimuli/manifest,
response form/schema, calibration/playback checklist, deviation/adverse-observation form and
completion/reveal instructions. The private packet contains only the mode-0600 key and source
provenance needed by the facilitator. “Listener-visible” does not grant redistribution rights;
the source permission controls every derived render. The packet does not bundle a player, automate
a DAC, claim acoustic concealment, or contain a listener answer.

## Allowed implementation paths

- `tools/miso-engine-builtins-fixture/Cargo.toml` only to register the new binary;
- `tools/miso-engine-builtins-fixture/src/listening_main.rs`;
- `dsp-research/listening/issue033/**` for schemas, facilitator material and answer-free fixtures;
- `scripts/check-builtins-listening-033.*`;
- `scripts/test-builtins-listening-033.*`;
- `scripts/preflight-builtins-listening-033.sh`;
- `scripts/prepare-builtins-listening-033.sh`;
- this spec and its tracked brief; and
- a minimal exact-path policy/README/implementation-plan route if required.

`Cargo.lock` may change only if registering the existing-dependency binary mechanically requires
it; any third-party dependency or product/reference/fixture/benchmark/predecessor edit is STOP.

## Focused gates and evidence

Before any real preflight or preparation invocation, pass:

1. renderer unit tests for exact input parsing, block/event boundaries, role outputs, WAVE bytes,
   deterministic repeat, level/peak rules and every rejection class;
2. validator/statistics self-tests for all counts `0..20`, full valid synthetic *format-only*
   records and missing/extra/duplicate/order/hash/type/status/concealment/counter/adverse rows;
3. hermetic fake preflight/preparation lifecycle covering arguments, missing tools/input/
   provenance, source/product/Issue-110 drift, regular/nlink/mode/extra-member checks, partial
   failure preservation, no-clobber, authority recheck immediately before render, and second-call
   refusal with render/playback/human counters zero;
4. focused locked check/test, warning-denied Clippy/rustdoc, format, applicable policy and policy
   mutations, shell/Python syntax, docs/title/dependency/conflict/whitespace/diff sanity; and
5. static proof that no audio-device/player command, benchmark/timer, network, V1, product edit or
   fabricated answer path is reachable.

The focused checkpoint records exact paths and hashes, clean candidate identity, preserved
Issue-110 evidence, `target/issue33` absence and every real counter at zero. Sol High then pauses.
Sol XHigh either returns focused PASS or one bounded HOLD. No broad qualification matrix,
benchmark, timing, playback or listening is part of this tranche.

## Human handoff and overall pass/fail

After a separately reviewed successful machine preparation, Sol XHigh may mark only **AGENT
PREPARATION COMPLETE / READY FOR REAL FACILITATOR**. The facilitator must independently verify the
packet, record pseudonymous facilitator/listener/reveal-verifier IDs and conflicts, playback
hardware/driver/mode, transducer, calibration level/method, room/headphone conditions and
environmental notes, then conduct both procedures without exposing the key. An agent may explain
the packet but may not act as the listener or signatory.

Overall Issue-033 PASS still requires two completed real-human records with exactly 20 valid
responses each, immutable pre-reveal response hashes, matching reveal, reproducible statistics,
three distinct pseudonymous sign-offs, no material unblinding/stopping-rule deviation and no unresolved candidate-
attributable click, pop, image jump, gain discontinuity, instability or other adverse observation.
Preference and nonsignificant ABX are not required. A significant ABX result without an adverse
defect receives a bounded descriptive explanation; it is never relabeled better/worse.

Incomplete evidence, candidate/source drift, a protocol defect, fabricated response, or unresolved
candidate-attributable adverse observation is FAIL and blocks **End-to-end release, performance,
and listening qualification**. Preserve the bytes and open a stateless corrective issue; never
edit answers, rerender, tune, repeat or weaken the gate inside Issue 033.

## Readiness evidence (2026-08-22)

Sol XHigh read local and remote Issue 033, completed Issue 110, the Issue-007 accepted record and
final methodology, Issue-002 template/example, listening checker/resources, issue index and
implementation plan. Remote Issue 033 is OPEN and its number/title match the local spec, but its
body still names stopped Issue 072 and requires synchronization by root after this docs checkpoint
is committed/upstream. No product/API blocker was found. The only external pre-execution input is
the exact permissioned excerpt and provenance record defined above; its absence forbids a real
preparation invocation but does not block implementing and adversarially testing the preparation
tooling.

This briefing changed documentation only. Cargo/build, render, audio playback, listening, preflight,
benchmark, timing, workload, Git mutation and GitHub mutation counts are all zero.

## Agent-preparation implementation evidence (2026-08-22)

Sol High implemented the bounded, nonexecuting preparation tranche from local base
`7d2f99f31694a78914632d015efcbcfd31955a68` / tree
`88434df3b55678f4a80564e8347a2dbf0ec8b601`. The tranche registers the offline renderer, adds the
four canonical schemas and answer-free facilitator resources, and adds the stdlib validator,
static mutation gate, zero-render preflight, no-playback preparation wrapper and hermetic fake
lifecycle at only the allowed paths.

Focused evidence is green: five renderer unit tests; the independent statistics/record self-test
over all 21 count rows; the static checker; 26 rejected policy mutations; the expanded hermetic
lifecycle including missing-tool/input/provenance, authority drift, link/type/mode/namespace,
partial preservation and no-clobber rows; locked package check/test; warning-denied package Clippy
and rustdoc; formatting; and shell syntax. The lifecycle reports exact real counters
`0/0/0/0/0/0/0/0` for preflight/render/playback/session/trial/response/reveal/result.

No permissioned source was supplied or read. `target/issue33` remains absent. No real preflight,
renderer, playback, listening session, response, reveal, result, benchmark, timing, workload, Git
mutation or GitHub mutation ran. This is a commit-ready implementation checkpoint pending Sol
XHigh adversarial review; it does not claim agent preparation complete and grants no authority for
the real preflight or preparation invocation.

Sol XHigh's sole bounded HOLD was closed before checkpoint handoff: all four schemas now freeze the
validator's closed fields and expressible cross-field rules; the preparation record commits every
non-self packet member and the validator recomputes every digest; all copied tracked inputs are
bound into both seals and checked again after render; response attempts allow only an invalid
prefix followed by one terminal valid row; and qualification requires and recomputes the exact
preparation/response/reveal authorities, mappings, counts and statistics. New fake/static rows
reject post-preflight schema drift, post-render packet-input drift, copied-packet mutation,
post-valid retries and linked qualification/order faults. No real counter changed.

## Terminal second HOLD and successor routing (2026-08-22)

Sol XHigh's pass-2 review found one remaining end-to-end authority defect. The linked qualification
validator hashes the named preparation, responses and reveal and validates the reveal against the
supplied assignment-key file, but it never compares that key file's SHA-256 with either
`Issue007ListeningPreparationV1.assignment_key_sha256` or
`packet_member_sha256["private/assignment-key.json"]`. A preparation and concealed schedule from
key A can therefore be combined with responses, reveal and key B while the three qualification
authority hashes, derived counts and statistics still validate. The format-only self-test exposes
the same mismatch by retaining an unrelated preparation key digest while its independently hashed
key file passes linked qualification.

This was the second HOLD under the frozen two-pass budget, so Issue 033 is **TERMINAL STOP / NO
OVERALL PASS**. The current 18-path tranche is useful technical input only; it is not accepted
preparation authority and authorizes no preflight, render, playback, listening, response, reveal or
qualification action. No permissioned source was supplied or read, `target/issue33` remains absent,
and exact real preflight/render/playback/session/trial/response/reveal/result counters remain
`0/0/0/0/0/0/0/0`.

Stateless Issue 111, **Close builtin listening assignment-key authority and prepare the real
facilitator packet**, owns only the missing commitment link, fresh successor authority and eventual
machine-only packet preparation. It must preserve this failed tranche and use `target/issue111`;
Issue 033 is never retried and `target/issue33` is never created.

## Retirement note (#104 phase A, 2026-08-24)

`scripts/{check,preflight,prepare,test}-builtins-listening-033.sh` and `scripts/test-builtins-listening-033-policy.sh` are retired by #104 phase A (#83 W4-D2). They sealed `Cargo.lock`, `crates/miso-engine-builtins{,-compiler}/src/lib.rs` and seven `target/issue110/` build artifacts; the lane waves rewrote the sources and the artifacts have never existed in a fresh checkout, so the checker could not pass again and could not be honestly re-sealed. The sealed hashes stay in this file. The live half -- the two packet validators' `--self-test` and the public-packet canonicality/answer-free assertions -- moved into `scripts/check-builtins-listening.sh`, and `scripts/prepare-builtins-listening.sh` renders a packet without the seal machinery.
