# 029 Canonical effect interchange, state migration, and CID package identity

## Outcome

Define deterministic external bytes for effect descriptors, packages, artifacts, content identity
and persisted state without expanding or blocking the launch native processing contract.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark,
or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only
through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O,
logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are
retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono
L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix
declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000,
352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

Issue 011 deliberately owns only semantic native descriptors, preparation, processing, immutable
prepared metadata and current-layout common/left/right payload hooks. This issue owns every
external byte representation that was removed during the issue-011 rescope. It is independently
implementable only after its exact dependencies are complete. Its change must follow the
Sol-approved brief -> Terra attempt 1 with evidence -> Sol adversarial review workflow; Sol may
make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening
gates.

## Scope

Specify a canonical versioned descriptor wire and matching fixed-width C descriptor records;
domain-separated descriptor identity; a canonical non-archive package stream; source, core-Wasm
and target-native artifact records and exact-content hashes; strict CIDv1 raw/SHA2-256 binary and
text identity; bounded encode/decode/verify and artifact selection; a persisted state envelope over
the issue-011 common/left/right payload sections; effect-ID/configuration binding; transactional
current-version restore; explicit registered `N -> N+1` migration chains with bounded caller
scratch; golden vectors and an independent reference implementation. Publisher, transport,
signature, trust, license, cache and repository metadata are sidecars outside content identity.

## Required public interfaces/contracts

`EffectDescriptorWireV1` round-trips every semantic `EffectDescriptorV1` field, including supported
link modes, through one canonical byte sequence and exposes fixed-width C records without callable
plugin entrypoints. `EffectPackageV1` binds canonical descriptor bytes to sorted artifact metadata
and exact content; `EffectCid` is CIDv1 over the complete canonical package bytes.
`EffectStateEnvelopeV1` binds an effect ID, contract/layout versions and the complete prepared
configuration to exact common/left/right payload bytes. A `StateMigrationRegistry` registers one
unique bounded step per `(effect_id, from_version)`; migration and restore operate on an unpublished
prepared temporary so any failure leaves the live processor unchanged. Bounded encoders accept
caller storage and return required size without partial writes.

## Deliverables

Control-plane package(s) named with the `miso-engine-` prefix and matching `miso_engine_` crate
identifier; descriptor wire and C header specifications; canonical package/CID encoder, verifier
and artifact selector; state envelope and migration dispatcher; strict diagnostic registry;
checked golden and malformed fixtures; independent non-Rust reference CLI; fuzz targets; ABI
layout reports; producer/distribution identity documentation; conformance integration; and a
bounded benchmark tool. If the existing provisional `miso-engine-effect-package` code is retained,
it must be reviewed against the new Sol brief rather than treated as accepted issue-011 work.

## Explicit non-goals

Changing the issue-011 runtime traits or descriptor meaning; graph compilation/PDC; production DSP;
executing or dynamically loading native or third-party artifacts; defining the third-party Wasm
process ABI; repository service, network resolution, installation, signing, trust-store, malware
policy, licensing enforcement or marketplace UI; claiming a CID proves trust, safety, quality or
cross-CPU/backend bit-identical audio; noncanonical tar/zip hashing; or any render-thread parsing,
hashing, migration, allocation or I/O.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Versioned TOML schema and transactional session compiler
- Native effect runtime contract and conformance

## Hazards/decisions

CID is exact-byte identity only; use the CIDv1 specification at https://specs.ipfs.tech/cid/.
Descriptor identity must cover every semantic field and use a frozen domain-separated preimage.
The Sol brief must freeze all header/record bytes, enum numbers, canonical ordering, string rules,
hash primitives, multicodecs, migration keys and scratch-size rules before implementation. Rust/C
layout is never transmuted as wire. State bytes must preserve dual-mono lane separation and reject
wrong prepared configuration. All parsing, hashing, artifact selection and migration are bounded
control-plane operations.

## Acceptance gates with objective measurements

Rust and an independent non-Rust implementation produce byte-identical descriptor, package, CID
and state vectors in 100 fresh processes. Decode/re-encode is identical; truncation, trailing data,
unknown enums/flags, nonzero reserved bytes, noncanonical order/offset/string/path/padding and
overflow reject with exact codes. Every semantic descriptor field mutation either rejects or
changes the frozen descriptor identity; every manifest/artifact/content mutation rejects or yields
the frozen changed CID. Every artifact hash is recomputed before selection. Caller-buffer
output-too-small reports the exact required size and writes no byte. C11 and Rust reports agree on
every fixed record size/alignment/offset across x86_64 Linux, AArch64 Android/iOS and wasm32.
Current state restores exactly; wrong effect/configuration/digest/version rejects; every registered
migration path succeeds within declared scratch, every missing/duplicate/failing step rejects, and
failed migration/restore leaves the live destination unchanged. At least 10,000 deterministic
mutations per parser complete without panic or unbounded allocation.

## Target matrix

Control-plane native, mobile and browser/Wasm validation builds; independent CLI on native hosts.
No package, descriptor or state parser is render-reachable. Third-party ABI and future repository
work consume these bytes but do not execute in this issue.

## Required evidence

Normative byte tables and diagnostic registry; sorted fixture manifest with exact sizes and
SHA-256; Rust/independent-reference 100-process transcript; content-mutation/CID matrix; C/Rust
multi-target ABI reports; output-buffer canary results; state round-trip, migration-chain and
transactional-failure reports; fuzz corpus/checksums and run summaries; dependency/realtime-boundary
audit; target build report; and bounded benchmark JSONL produced only after all nonbenchmark gates
pass.
