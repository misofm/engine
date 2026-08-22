# 079 Prepared effect state envelope and transactional current-layout restore

## Outcome and status

Ship one canonical, bounded current-layout state envelope for an accepted native effect and restore
it only into an unpublished destination. **COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Accepted
Issues 082, 011 and 004 supply descriptor identity, runtime factory and transactional compiler
seams. Per the user override, Sol High implemented and Sol XHigh briefed and adversarially verified.
Workload, benchmark, timed, audit, browser and general-target invocation counts remain zero. Remote
closure is intentionally reserved for root after the single eventual bulk `main` merge/push/CI and
GitHub evidence synchronization; this local record does not claim either action.

Existing `effect-package/src/state.rs`, `tests/state_vectors.rs` and `fuzz/effect_state.rs` are
provisional technical input and may be replaced. `effect-package/src/compile.rs` is obsolete to this
vertical and remains read-only.

## Smallest launch product

Effect-package owns canonical wire encode/borrowed verification. Effect-compiler orchestration must
consume the complete retained `EffectBankPreparationV1`; metadata cannot recover initial values or
preparation caps. Add only a direct effect-package dependency to effect-compiler and one narrow
Issue-082 binding helper described below.

To avoid a package/compiler dependency cycle, effect-package defines a borrowed
`EffectStateReplayViewV1<'a>` over effect-contract types: native effect type ID, rate, quantum,
quality, bypass, link, ports, ordered initial values and `PrepareEffectLimits`. Compiler adapts its
owned `EffectBankPreparationV1` to that view and owns every reconstructed replay. The serialized
`effect_id` is `metadata.descriptor.id`, the native effect **type** ID and registry key; it is never
the session's `EffectPreparedEntry.effect_id` instance ID. V1 omits instance identity: an envelope
is portable between instances only when type descriptor, complete replay and current layout match.

Issue 082 adds an opaque, private-construction `BoundEffectDescriptorWireV1` (exact name may vary
mechanically). Its single constructor accepts a static `EffectDescriptorV1` and caller wire,
validates both through one coherent Issue-082 operation, compares every semantic table/field in
frozen wire order, and returns the canonical wire plus its domain-separated identity only when they
match. It reuses Issue 082's private borrowed semantic view and identity computation: it does not
encode into scratch, invoke a second public verifier, trust the caller, leak storage or extend a
lifetime. Malformed wire keeps the existing exact diagnostic; a static-versus-wire mismatch maps to
the earliest corresponding wire field/record by Issue 082's frozen order. The token's provenance
fields are private and cannot be forged.

Effect-compiler wraps that token with the exact injected `Arc<dyn NativeEffectFactory>` whose
`factory.descriptor()` was bound. Scalar restore consumes this wire-bound factory capability,
rebuilds the exact request, prepares a new processor, checks complete metadata, restores current
payload and returns ownership only on success. It never receives the live processor and never
re-resolves an ID to an unbound factory.

A bank envelope remains per member, but program-key equality is not provenance. Compiler-owned
preparation produces an opaque unpublished-bank capability containing the prepared bank, the exact
wire-bound factory capability and complete owned sibling `EffectBankPreparationV1` replays.
Bank-member restore consumes that capability by value and returns it only after the selected member
has restored successfully. Any validation, index, metadata or hook failure drops the capability and
its possibly partially mutated bank, so publishable bank ownership cannot escape. Repeated member
restores consume and return the capability serially. Backend, width, siblings and cohort membership
are still not persisted in one member envelope; no whole-bank reconstruction API is added. Snapshot
may borrow the same provenance-bound unpublished capability. Snapshot and restore require quiescent
control ownership and are never render-callable.

The wire intentionally has no scalar/bank/backend/width discriminator. Current-layout payloads are
scalar↔bank-member interchangeable for an identical bound descriptor, complete replay and program
key; the effect payload hooks own that compatibility contract. Portable mock gates exercise both
directions. Backend/cohort choice and sibling replays remain destination preparation, not persisted
state.

## Canonical wire V1

All integers are little-endian. Bytes are:

`224-byte header || effect_id || sidechain_port_id || minimum zero pad to 8 || initial table || common || left || right`

| Offset | Bytes | Field |
|---:|---:|---|
| 0 | 8 | ASCII `MISOEFST` |
| 8 | 2 | wire version `1` |
| 10 | 2 | header bytes `224` |
| 12 | 4 | flags, zero |
| 16 | 8 | exact total bytes |
| 24 | 32 | accepted Issue-082 descriptor identity |
| 56 | 32 | state-envelope digest |
| 88 | 2 | contract major |
| 90 | 2 | contract minor |
| 92 | 4 | current nonzero state-layout version |
| 96 | 4 | sample rate |
| 100 | 4 | quantum |
| 104 | 4 | `EffectQuality` raw value |
| 108 | 4 | bypass, canonical `0` or `1` |
| 112 | 4 | `LinkMode` raw value |
| 116 | 4 | sidechain kind: `0=None`, `1=Unconnected`, `2=Connected` |
| 120 | 4 | sidechain required, canonical `0` or `1` |
| 124 | 4 | effect-ID bytes |
| 128 | 4 | sidechain-port-ID bytes |
| 132 | 4 | initial-value count |
| 136 | 8 | latency samples |
| 144 | 4 | tail kind: `1=Finite`, `2=Infinite` |
| 148 | 4 | reserved, zero |
| 152 | 8 | finite tail samples; zero for Infinite |
| 160 | 4 | common payload bytes |
| 164 | 4 | left payload bytes |
| 168 | 4 | right payload bytes |
| 172 | 4 | reserved, zero |
| 176 | 8 | exact prepared scratch bytes |
| 184 | 4 | exact automation capacity |
| 188 | 4 | initial-table bytes, exactly `count * 16` |
| 192 | 8 | request maximum total state bytes |
| 200 | 8 | request maximum scratch bytes |
| 208 | 4 | request maximum automation spans per block |
| 212 | 4 | reserved, zero |
| 216 | 8 | payload bytes, exactly `common + left + right` |

IDs use the accepted 1..=127-byte ASCII grammar. None requires empty port ID and `required=0`;
Unconnected requires `required=0`; Connected preserves the descriptor bit. There is only minimum
string padding and no trailing pad. Each initial record is exactly 16 bytes: `u32 parameter_index`,
`u32 ParameterChannel`, raw `f32::to_bits()` in `u32`, zero `u32 reserved`. Records are complete and
ordered exactly as `validate_initial_values`: parameter index order; Shared has Both; PerLane has
Left then Right. Values are preparation-valid and negative zero rejects. Common/left/right counts
are independent and equal exact metadata; no equality/nonempty rule is invented.

Digest:

`SHA-256("miso.engine.effect-state.current-layout.v1\\0" || u64_le(total_bytes) || bytes[0..56] || 32 zero bytes || bytes[88..total_bytes])`.

It binds every envelope byte but is not authentication. Descriptor identity is Issue 082's exact
domain-separated identity, never a digest of effect-ID text.

## Limits, APIs and atomicity

`EffectStateLimitsV1::default()` is descriptor 4,194,304 bytes; envelope 268,435,456 bytes; payload
134,217,728 bytes; initial values 4,096. Caps are inclusive. Every sum/multiply and `u64`/`u32`,
`usize` and `isize::MAX` conversion is checked before slicing/writing.

Freeze these semantic Rust operations; names/lifetimes may vary only mechanically:

- Issue 082 constructs the private-field descriptor binding token from one static descriptor and
  canonical caller wire, and exposes only the bound wire/identity needed by state orchestration;
- effect-compiler constructs a wire-bound factory capability only by binding
  `factory.descriptor()` through that helper;
- requirements returns exact envelope bytes, payload snapshot-scratch bytes and initial-value
  scratch slots from a bound descriptor token + package-owned borrowed replay view;
- encode accepts supplied common/left/right slices and atomically writes caller output;
- verify returns a private-construction borrowed envelope, borrowed payload sections and initial
  iterator;
- scalar/bank-track snapshot takes the wire-bound factory/unpublished-bank capability, exact retained
  replay, processor/bank+track, caller payload scratch and output;
- scalar restore takes the wire-bound factory capability, current admission policy and exact caller
  `&mut [InitialParameterValue]` scratch, returning owned unpublished processor, metadata, factory
  and reconstructed replay only after success;
- bank preparation validates every request before binding and returns an opaque capability owning
  the bank, exact factory provenance and every sibling replay;
- bank-track restore consumes that opaque capability by value, validates the verified envelope,
  selected replay and complete metadata, and returns the capability only on success.

Restore also takes current caller admission policy
`EffectStateRestoreAdmissionV1 { sample_rate, quantum, maximum_total_state_bytes,
maximum_scratch_bytes, maximum_automation_spans_per_block }` (name may vary mechanically).
`sample_rate` must be one of the four launch rates and nonzero `quantum` is the exact current
preparation quantum. For scalar reconstruction, saved rate/quantum must equal that policy, each
saved request cap must not exceed its current ceiling, derived payload state must fit the current
state ceiling, and saved prepared scratch/automation must fit their current ceilings before
`factory.prepare`. The exact saved request values are replayed only after admission. For banks, the
opaque capability is prepared solely from caller-current requests already admitted against this
policy; saved envelope values never drive bank allocation, and restore checks equality before its
payload hook. Saved caps are historical request inputs, never authority to allocate. Each one-below
current ceiling and extreme saved rate/quantum/cap row rejects with an exact Limit diagnostic; a
call-counted scalar factory proves prepare was not entered.

Each descriptor-binding-token construction performs exactly one coherent Issue-082
static-validation/wire-verification/semantic-comparison/identity pass. State
requirements/encode/verify/snapshot/restore accept that capability and perform no second descriptor
validation or identity pass; one token may be reused. The helper's bounded temporary heap dies
before return. Wire work retains no allocation. Accepted off-render factory/bank preparation may
allocate declared processor resources; only fully successful unpublished ownership and owned replay
may be returned. Allocator layout is Issue 081 scope.

Encode preflights before output writes. Snapshot preflights exact output/scratch, snapshots only into
scratch, then publishes; any failure leaves output bit-identical and scratch disposable. Restore
verifies before initial scratch, prepares/restores only unpublished state and returns no ownership
on failure. A by-value bank wrapper is consumed and dropped on failure. Live processor and envelope
bytes remain unchanged.

Output and scratch slices may be oversized. Only the exact required prefix is used and every suffix
byte remains untouched on success or failure. Any one-short output or scratch rejects before a hook
and leaves the complete output unchanged. Snapshot hooks may partially overwrite only the
disposable required scratch prefix before returning failure; public output remains bit-identical.
Every encode cap one below the derived requirement rejects before writing any output byte.

## Diagnostics

`EffectStateDiagnosticV1` is 32-byte `repr(C)`: `u32 code`, `u32 detail`, `u32 item_index`, zero
`u32 reserved`, `u64 byte_offset`, `u64 required_bytes`. Unavailable index/offset are maxima. Codes:

`Ok=0, Limit=1, BufferTooSmall=2, Header=3, Length=4, Reserved=5, Enum=6, Order=7, Text=8, Descriptor=9, Digest=10, Metadata=11, InitialValues=12, Payload=13, Factory=14, Restore=15, Overflow=16`.

Buffer details: `1=EnvelopeOutput`, `2=PayloadScratch`, `3=InitialValueScratch`. Descriptor detail is
`(kind << 16) | issue082_code`: kind `1` is malformed external wire, `2` is
static-descriptor/wire semantic mismatch and `3` is envelope identity/token mismatch; the nested
code is zero when none applies. For kinds 1/2, `byte_offset` refers to the external descriptor wire,
not the state envelope; kind 3 has unavailable offset. Metadata details in order: effect ID,
contract, layout, rate, quantum, quality, bypass, link, ports, latency, tail, state sizes, scratch,
automation, request limits. Payload hook details are scalar snapshot, bank snapshot, scalar restore,
bank restore = 1..4. Factory details are bound factory unavailable, request invalid, prepare
failed/`Ok(None)`, returned metadata mismatch = 1..4. Restore details are bank track index,
replay/config mismatch, program-key mismatch, provenance mismatch = 1..4; `Restore=15` is therefore
used. Free-form hook/factory strings never become canonical diagnostics.

First-error order uses a lexicographic internal ordinal, never emitted unavailable
`item_index=u32::MAX`: (0) caller limits/checked arithmetic in API-argument order; (1) fixed header
fields in increasing byte offset; (2) reserved fields in increasing byte offset; (3) structural
length/host-fit/padding fields in table/header traversal order; (4) enums, text and initial records
in table order then record index then increasing field offset; (5) digest; (6) external descriptor
wire, static binding, then identity; (7) admission, replay/request and metadata in their listed field
order; (8) payload hook. Within a table subphase use record index then field offset. Authoring order
is descriptor binding, API limits, metadata/replay, initial rows, payload lengths.

The exactly-one Issue-082 pass rule applies only once an operation reaches descriptor binding or
uses an already bound token. Early limits/header/reserved/length/digest rejection performs zero
Issue-082 passes. No path performs two.

## Product-closing gates

1. One independent standard-library vector freezes exact bytes/digest/identity/manifest; Rust
   decode/re-encode is byte-identical.
2. Exact, one-short and every one-below cap preserve canaries. Representative mutation coverage
   spans every header field/class, padding, enum/text/order, digest, metadata and payload plus
   truncation/trailing/overflow with exact diagnostics. Oversized output/scratch suffixes remain
   exact; a deliberately partial-writing failing snapshot hook cannot publish output.
3. A production scalar delay with active common and independent lane history restores through its
   wire-bound factory capability and matches uninterrupted PCM/report/next snapshot across uneven
   partitions.
4. A portable unequal-section mock proves three-section independence. A host-legal production W8
   soft-clip/compressor member plus portable mock bank prove track snapshot/by-value unpublished
   restore, continuation/isolation, bad index/config/program key/provenance and hook-failure
   disposal. A failed restore returns no bank token; sequential successful member restores consume
   and return it. Portable mock payloads restore scalar→bank member and bank member→scalar exactly.
   Legal `Ok(None)` remains fallback, not bank success.
5. Wrong descriptor/effect/layout/request/initial/metadata/size/digest never returns a destination;
   every representable descriptor semantic-class mismatch rejects token construction; hook failure
   publishes nothing and a sentinel live processor stays exact.
6. Production replays cover four launch rates and representative link/sidechain/tail/bypass/zero
   and nonzero-common rows without an extended-rate claim. Current-admission one-below rows and
   extreme saved quantum/automation prove exact Limit diagnostics and zero factory prepare calls.
7. Replace state tests and update only the obsolete state fuzz target to compile; fuzz execution is
   Issue 081. Pass focused package/compiler/effect/reference/scalar-Wasm tests, Clippy/rustdoc/fmt,
   policies+mutations/static scans, then one locked nonbenchmark workspace seal.

## Allowed files and non-goals

Allowed: `effect-package/src/{state,diagnostic,wire,lib}.rs`, state/wire binding tests and direct
manifest+lock;
`effect-compiler/src/prepare.rs`, direct package dependency/state tests; `fixtures/effect-state/v1/`,
one state reference/check script, `docs/EFFECT_STATE_V1.md`, `fuzz/fuzz_targets/effect_state.rs`,
minimal direct policy mutations. No effect production change except an essential test-only mock.

Non-goals: migration/registry (Issue 080); package/CID; runtime trait, descriptor wire format or
session schema changes beyond the narrow Issue-082 static-descriptor binding helper; whole-bank
reconstruction; live render/plan mutation; trust/signatures; broad fuzz, target/process matrices;
benchmark, timing or listening.

## Implementation evidence and Sol decision

The clean product/reference candidate was local commit
`7bb0e0ea8d57674f6e5cbb5cdb6a74470fe9ecfa` (tree
`15074afdf345b8c3b302bda459de1b5083852d95`). Its coherent local checkpoints are:

- `c91fdb0`: package-owned V1 wire, bounded verification, identity binding and diagnostics;
- `77e9bc1`: scalar snapshot/transactional restore and admission;
- `d4377cd`: by-value unpublished bank capability, member snapshot/restore and compatibility; and
- `7bb0e0e`: independent reference vector, exact documentation, compile-only fuzz update and
  product qualification.

The independent Python-standard-library vector is exactly 309 state bytes: a 224-byte header,
three 16-byte initial records and payload beginning at byte 296. Its accepted descriptor wire is
653 bytes with SHA-256 `cff6a313be6b04a8932343928a0ab69c296fadcdca88922032b217c20a81aea9`;
the descriptor identity is
`752552864ed6796526d1859f83795bb3facea6cc91bc1fa5e3e796ee67284ff1`; the envelope digest is
`858e6db10df1b69626736bf2d5f29634866269599d0428710c95e2103dfc837f`; the complete state file
SHA-256 is `b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48`; and
`MANIFEST.sha256` has SHA-256
`3cee39d0fd213989d81f92675085b2d90d11bf6ff1f4b4a4323b158fce8b7220`. The manifest covers all
eight listed fixture members (nine fixture files including the manifest). Rust binds the independent
descriptor and re-encodes all 309 bytes exactly; the malformed-vector oracle is nonvacuous and
covers every frozen header/diagnostic class.

One locked broad nonbenchmark seal ran once against that exact candidate. It registered 541
workspace tests: 535 passed, six pre-existing ignored/manual tests remained ignored and zero failed;
all eight doctests passed. The remaining ordered formatting, checks, strict Clippy, rustdoc,
reference/manifest, fuzz compile-only, scalar-Wasm, static scan and policy/mutation gates passed.
The seal counted 11 policy commands and syntax-checked all 76 repository shell scripts. Pre/post
branch, HEAD, tree, worktree and index observations were identical, as were these four
seal-sensitive files:

| File | SHA-256 |
|---|---|
| `Cargo.lock` | `8db695d722dc2055faaac82ffebb8741bf948117fc733834de9e157ff4e31e6c` |
| `fuzz/Cargo.lock` | `af4547d5bae367e4249c6fcf482b249ff8af0ae29b9a933957d34b36ec36e5d5` |
| `fixtures/effect-state/v1/MANIFEST.sha256` | `3cee39d0fd213989d81f92675085b2d90d11bf6ff1f4b4a4323b158fce8b7220` |
| `scripts/effect-state-v1-reference.py` | `9dc95018daa5c993c16fc10ca5185ec17193bd69d6b71a199737b110ba7f0c0e` |

Invocation accounting for the seal is explicit: benchmark 0, timed 0, workload 0, audit-main 0 and
fuzz execution 0. The fuzz target was checked and linted only. No Issue 080 migration work or Issue
081 broad fuzzing, allocation accounting, multitarget qualification or benchmark work entered this
issue.

**Sol XHigh verdict: PASS. Issue 079 is complete and ready to close after root performs the one
eventual batch delivery, remote evidence synchronization and GitHub closure.**

## Dependencies by exact issue title

- Close canonical effect descriptor wire, identity, and C inspection ABI
- Native effect runtime contract and conformance
- Versioned TOML schema and transactional session compiler

## Downstream boundary

Acceptance unblocks **Effect state migration registry and bounded chains**. Broad state fuzz,
allocation and multitarget evidence remains in **Canonical effect interchange qualification,
fuzzing, and benchmark** after Issues 078 and 080.
