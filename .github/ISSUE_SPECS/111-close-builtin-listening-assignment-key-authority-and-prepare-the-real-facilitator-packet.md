# 111 Close builtin listening assignment-key authority and prepare the real facilitator packet

## Outcome and status

Close the one missing assignment-key authority edge left by stopped Issue 033, then prepare and
independently verify one no-playback facilitator packet from a fresh successor namespace.

**OPEN / SOL XHIGH FOCUSED PASS / WAITING FOR EXTERNAL FACILITATOR INPUT.** Sol High implemented
one focused pass and its sole bounded correction; Sol XHigh briefed and adversarially verified both.
The focused checkpoint is authorized, but no preparation seal, preflight, render, playback or
listening action or artifact is authorized.

The eventual technical finish line is only agent preparation for a real facilitator; it has not
been reached. No permissioned 48 kHz stereo source, provenance record or private seed was supplied,
so this checkpoint does **not** claim agent preparation complete or readiness for a real
facilitator. Human playback, identities, responses, observations, conditions, reveal, sign-offs,
qualification results and audible-quality claims remain outside agent execution. Issue 111 does
not complete the human listening gate.

Remote Issue 111 was read-only confirmed absent/available on 2026-08-22. Root owns GitHub creation,
body synchronization and eventual state changes after the local docs checkpoint is committed and
upstream. This record claims no GitHub mutation.

## Exact dependencies and predecessor status

- **Complete builtin benchmark host metadata run from a fresh successor namespace** is the accepted
  machine authority through Issue 110.
- **Issue-007 builtin filter and matrix human listening qualification** is stopped Issue 033. Its
  two-pass preparation tranche is technical input only, not PASS or reusable execution authority.

Issue 033 stopped because its linked qualification path did not prove that the supplied private
assignment key was the key committed by preparation. Its self-test permitted an unrelated
preparation key digest and supplied key file to pass together. Historical Issue-033 real
preflight/render/playback/session/trial/response/reveal/result counters are permanently
`0/0/0/0/0/0/0/0`; `target/issue33` is absent and must remain absent.

## Frozen technical input

Preserve the stopped Issue-033 renderer, schemas, facilitator material and scripts byte-for-byte.
They are useful technical input but not accepted authority:

| Path | SHA-256 |
| --- | --- |
| `tools/miso-engine-builtins-fixture/Cargo.toml` | `ee0542febb4b2dbde53f22dab0ae55d73483784fefb5e36b31bcd7b0786726e6` |
| `tools/miso-engine-builtins-fixture/src/listening_main.rs` | `992297bc85b12655d11f090c50d48681ad0ba60b8c33e7c77864089d5227100a` |
| `dsp-research/listening/issue033/FACILITATOR.md` | `5f1029e337c245b36d0535c3c0e9bbc9ff933429d242ec41596fbd69dcbdd2b4` |
| `dsp-research/listening/issue033/README.md` | `b4971c4805086819f6a7393d7932d5ebd2a9c16ab1d331cde3e2c21c189c0704` |
| `dsp-research/listening/issue033/preparation.schema.json` | `1b1d07ffeb00dfc3145848ae241d763e37fff9339aa510e7c3a597217aaf68d9` |
| `dsp-research/listening/issue033/provenance.template.json` | `942d20f743ee7c396101809e31340da56f61120298bdb92ac07f750ae2d688d9` |
| `dsp-research/listening/issue033/qualification.schema.json` | `52c33c96ea4e450294f11270e4af3273971b9e4c7e6d4639a227464b7eeda634` |
| `dsp-research/listening/issue033/response-form.jsonl` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `dsp-research/listening/issue033/response.schema.json` | `6a84c9f469bd705a10947c746e1c9bc9c7dd49d32e04786a79881c9a930fa392` |
| `dsp-research/listening/issue033/reveal.schema.json` | `2a2a3745f4e6734108326eb4ee5ba365d26e6613d1d3f65f44e0b4b8e5f5bfdc` |
| `scripts/check-builtins-listening-033.py` | `3f5c348e3a0e1d93560e347968a8647f65d2ed3672a28e1f79f4bcf8e83eb163` |
| `scripts/check-builtins-listening-033.sh` | `dcabbb2b3e04cd630a6056464002614a30a4ee6888010c74fab822bb6554edfc` |
| `scripts/preflight-builtins-listening-033.sh` | `bf3ba048566343a82f55525730714188ab021cf9859618f454043ea143233f0e` |
| `scripts/prepare-builtins-listening-033.sh` | `57cb7164ec7c7cc8dcefd9e8d56795dcaa1ed140391b094bf306084e2f2e824c` |
| `scripts/test-builtins-listening-033-policy.sh` | `9c997ce9c81e7f3c1858582f5952aa0ac8a1fd987fc787745db869f6410c175b` |
| `scripts/test-builtins-listening-033.sh` | `37cbaff0fe73904fc4d8820c935789b9e7d1bab031b3626dde896813059b903f` |

Also preserve `Cargo.lock` SHA-256
`4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a`
and the exact seven regular nlink-1 Issue-110 artifacts, including distinct raw/accepted inodes and
absent prelaunch disposition. No predecessor file or namespace may be modified.

## Smallest closable correction

Create successor-named validator and lifecycle routes. The successor validator may reuse the
frozen Issue-033 algorithms and record definitions, but it must add the missing closed edge before
linked qualification can pass:

1. load and strictly validate the preparation record and its closed 15-member non-self digest map;
2. require the supplied assignment-key file to be the regular nlink-1 mode-0600 private packet
   member and recompute its SHA-256;
3. require that digest to equal both `preparation.assignment_key_sha256` and
   `preparation.packet_member_sha256["private/assignment-key.json"]`;
4. validate reveal against that same key and the immutable response bytes;
5. require exact qualification authority keys and recomputed preparation/response/reveal hashes;
   and
6. derive ABX correctness, matrix preference, attempt counts and all frozen statistics from those
   linked bytes.

The validator must reject preparation/key mismatch in both directions, alternate same-shaped key
files, wrong packet location/type/link/mode, packet member drift, reveal/key mismatch, response
drift, authority substitution and derived count/statistic mutation. Its valid fixture must use one
key digest consistently across preparation, packet member map, reveal and qualification.

Do not change renderer, WAVE, filter, matrix, level matching, SplitMix64, schema, response or
statistic semantics. Do not edit product/compiler code, Cargo manifests/lock, accepted fixtures,
preregistrations, Issue-110 artifacts or any benchmark path.

## Fresh no-clobber lifecycle

Use only fresh `target/issue111`; never create or reuse `target/issue33`. If persistent artifacts
are needed, successor scripts and seals use `issue=111`, successor names and exact clean candidate
identity. The expected workflow is:

1. focused implementation and fake/static gates with `target/issue111` absent and all real counters
   zero;
2. clean exact-path checkpoint and a successor preparation seal plus permissioned private inbox;
3. independent Sol XHigh seal review before exactly one zero-render preflight;
4. independent Sol XHigh preflight review before exactly one machine-only preparation render; and
5. terminal independent packet review before **READY FOR REAL FACILITATOR**.

Preflight may build and seal but must execute no renderer or audio path. Preparation may run the
frozen renderer exactly once only after explicit authorization and complete permissioned source,
provenance and private seed are present. It must never play audio. Every persistent artifact is
regular, nlink-1, exact-membership and no-clobber; seals, binary, packet and dispositions publish
atomically, partial render/stderr evidence is preserved, and either prelaunch or final disposition
consumes authority. No retry, alternate/direct invocation or second namespace is allowed.

Initial Issue-111 preflight/preparation/playback/session/trial/response/reveal/result counters are
`0/0/0/0/0/0/0/0`. A successful machine preparation may change only preflight and preparation to
`1/1`; playback/session/trial/response/reveal/result remain zero.

## Allowed tracked paths

- this spec and its tracked brief;
- `scripts/check-builtins-listening-111.py` and `.sh`;
- `scripts/test-builtins-listening-111.sh` and `scripts/test-builtins-listening-111-policy.sh`;
- `scripts/preflight-builtins-listening-111.sh`;
- `scripts/prepare-builtins-listening-111.sh`; and
- minimal exact Issue-111 README/implementation-plan routing.

Any Issue-033 implementation byte, renderer/product/schema/preregistration/fixture, Cargo file,
Issue-110 artifact, playback/player, human-answer or unrelated path edit is STOP.

## Focused gates and attempt boundary

Before a checkpoint, pass only successor validator self-tests, exact mismatch/link/type/mode rows,
the hermetic zero-real-execution preflight/preparation lifecycle, static checker and mutations,
shell/Python syntax, frozen Issue-033/110 identity checks and docs/title/dependency/diff sanity.
Compile-only use is allowed only if needed to prove the frozen renderer remains callable; no real
preflight, renderer, playback, listening, benchmark, timing or workload runs during implementation.

Sol High pauses after one coherent pass. Sol XHigh returns focused PASS or one bounded HOLD. A
focused PASS authorizes a checkpoint only. Root separately owns each later seal/preflight/prepare
authorization. Issue 111 is complete only when Sol XHigh verifies the final packet and records
**AGENT PREPARATION COMPLETE / READY FOR REAL FACILITATOR**. It never claims human listening PASS.

## Readiness evidence (2026-08-22)

Sol XHigh independently identified the exact missing comparison, confirmed Issue 033 exhausted its
two-pass budget with no real execution and `target/issue33` absent, and read-only confirmed remote
Issue 111 is absent/available. This docs checkpoint performs no Cargo/build, render, playback,
listening, benchmark, timing, workload, Git mutation or GitHub mutation.

## Focused implementation evidence (2026-08-22)

The successor-script checkpoint is commit `ac5cd0d` with tree `c4781fdc`. Its exact six-path
authority is:

| Path | SHA-256 |
| --- | --- |
| `scripts/check-builtins-listening-111.py` | `07e22c32daf33aece1d26fe69c1e3a70d1f3b6a60c9b7f751db1817b87d28b7a` |
| `scripts/check-builtins-listening-111.sh` | `350eb43361caa38e5a24c8489049bb00aab11813335b0821221f307edb08c137` |
| `scripts/test-builtins-listening-111-policy.sh` | `5b43417da6db58ea31b0d09b64842e3b5f7a381631340d4be8a162a558147179` |
| `scripts/test-builtins-listening-111.sh` | `13d7e7457bf5054c024ca1a77d9b3fb683adb67869c0c2285aa03761c5b9d23c` |
| `scripts/preflight-builtins-listening-111.sh` | `541e4821ac7eae07440591f1579bc088c5943bae97a6625acf452c943f9dcef3` |
| `scripts/prepare-builtins-listening-111.sh` | `57eb87fd4ed23f3cf4476350cc9d123a95391761eaaba77e828bba2ffd2ae165` |

Pass 1 received a bounded Sol XHigh HOLD because persistent `preparation.stderr` used an ordinary
redirection after a prior absence check, leaving a follow-or-truncate race for regular files,
hardlinks and symlinks. The bounded pass-2 correction creates it once under mode-0600 authority,
holds and verifies the descriptor/path identity across rendering, and adds hermetic collision rows
that preserve sentinels, launch no renderer and publish zero-counter prelaunch evidence. Sol XHigh
returned **STRICT SOL XHIGH FOCUSED PASS**. This verdict authorizes only the clean exact-path
checkpoint.

Reported focused gates passed: shell syntax for the five successor shell scripts; the successor
validator self-test with the inherited 21 count rows and 14 linked-authority rejection classes;
the static checker with all eight real counters at zero; 12 policy mutations; the hermetic
lifecycle with all eight real counters at zero; and conflict-marker, trailing-whitespace, Python
cache and target-namespace scans. `target/issue33` and `target/issue111` remained absent. The real
preflight/preparation/playback/session/trial/response/reveal/result counters remain exactly
`0/0/0/0/0/0/0/0`.

Issue 111 therefore remains **OPEN / WAITING FOR EXTERNAL FACILITATOR INPUT**. No permissioned
48 kHz stereo source, provenance record or private seed exists; no Issue-111 preparation seal,
preflight, render, playback, listening authorization or persistent artifact exists. Root must not
advance the later authority sequence until those external inputs are supplied and independently
reviewed.

## Retirement note (#104 phase A, 2026-08-24)

#104 phase A (#83 W4-D2): the whole Issue-111 script family (`{check,preflight,prepare,test}-builtins-listening-111.sh`, `test-builtins-listening-111-policy.sh`) is retired. It pinned `Cargo.lock`, sixteen frozen paths and seven `target/issue110/` artifacts, and `prepare-builtins-listening-111.sh` additionally refused to run off the branch `codex/listening-111`. `scripts/check-builtins-listening-111.py` survives (its `--packet`/`--linked` modes are the live validator); `scripts/prepare-builtins-listening.sh` replaces the retired wrapper. The pins are recorded here.
