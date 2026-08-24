# 078 Canonical effect package, CID, and artifact selection

## Outcome

Accept one bounded, deterministic, non-archive package stream that binds an Issue-082 descriptor to
opaque source, core-Wasm and target-native artifact bytes; identify the exact package with CIDv1;
and select one already-verified compatible artifact by a frozen precedence rule.

## Status and attempt budget

**SOL-BRIEFED / READY FOR TERRA ATTEMPT 1.** This is one launch-sized product vertical, not a
qualification campaign. Permit one Terra implementation attempt and one bounded Sol correction; a
second failure stops and rescopes. Workload, benchmark and timed invocation counts are zero and
remain zero.

The current `package.rs`, `cid.rs` and `package_vectors.rs` are provisional technical input only.
They are neither accepted bytes nor compatibility promises. Issue 082 is the accepted descriptor
wire/C-inspection dependency merged at `fb054bae41777585d12a48e71c99a2cfa9c3e3e4` and must remain
unchanged. The other exact dependencies below are already accepted; no stopped issue is treated as
a PASS dependency.

## Canonical package V1 bytes

All integers are unsigned little-endian. The package is exactly:

`96-byte header || descriptor bytes || artifact table || artifact contents`.

There is no archive metadata, filename normalization, timestamp, compression, signature or trailing
data. Header layout is:

| Offset | Bytes | Field | Canonical value |
| ---: | ---: | --- | --- |
| 0 | 8 | magic | ASCII `MISOEPKG` |
| 8 | 2 | version | `1` |
| 10 | 2 | header bytes | `96` |
| 12 | 4 | flags | `0` |
| 16 | 8 | total bytes | exact complete package length |
| 24 | 8 | descriptor bytes | exact descriptor length |
| 32 | 8 | table bytes | exact padded table length |
| 40 | 8 | content bytes | exact concatenated content length |
| 48 | 4 | artifact count | exact record count |
| 52 | 4 | reserved | `0` |
| 56 | 32 | descriptor identity | exact Issue-082 domain-separated identity |
| 88 | 8 | reserved | all zero |

The descriptor begins at byte 96 and must pass
`verify_effect_descriptor_wire_v1(descriptor, limits.maximum_descriptor_bytes)` before any package
is accepted. `effect_descriptor_identity_v1` must equal the header field.

Each variable-size table record has a 72-byte fixed prefix followed immediately by `path`, `target`
and `features` bytes, then the minimum zero padding needed to reach the next multiple of eight:

| Record offset | Bytes | Field |
| ---: | ---: | --- |
| 0 | 4 | artifact kind: Source=`1`, CoreWasm=`2`, TargetNative=`3` |
| 4 | 4 | reserved=`0` |
| 8 | 4 | path byte length |
| 12 | 4 | target byte length |
| 16 | 4 | feature-string byte length |
| 20 | 4 | reserved=`0` |
| 24 | 8 | content offset relative to content section |
| 32 | 8 | content byte length |
| 40 | 32 | SHA-256 of the exact content bytes |

Content offsets start at zero and are contiguous in record order; content is nonempty. Records are
strictly sorted by `(kind numeric, target bytes, features bytes, path bytes)` and exact duplicate
keys reject. Contents are concatenated in that same order without padding. Reordered authoring
input therefore encodes byte-identically. Any gap, overlap, out-of-order offset, nonzero padding or
reserved byte, unconsumed table/content byte, truncation or trailing byte rejects.

At least one Source record is required. CoreWasm and TargetNative are optional package capabilities;
absence is reported only when that kind is requested. The package layer treats artifact content as
opaque and never claims ABI, executable, import or trust validation.

## Canonical strings and authoring model

- `path` is 1..=255 ASCII bytes, relative and lowercase. `/` separates nonempty segments; leading
  or trailing `/`, `//`, `.` and `..` segments reject. Segment bytes are only `[a-z0-9._-]`.
- Source has empty `target` and empty `features`.
- CoreWasm target is exactly `wasm32-unknown-unknown`.
- TargetNative target is 3 or 4 nonempty `-`-separated components, 1..=127 total ASCII bytes; each
  component uses only `[a-z0-9_]`. A target beginning `wasm32-` rejects for TargetNative.
- `features` is empty or a comma-separated list of strictly increasing, duplicate-free tokens.
  Each token is 1..=32 ASCII bytes, begins `[a-z]`, continues `[a-z0-9-]`, and the whole string is
  at most 255 bytes. No whitespace, `+`, empty token or implicit feature relation is accepted.

The public authoring structs borrow descriptor, record strings and content slices. They do not own
or mutate caller data. Required-size and encoding scan for canonical record order without changing
the authoring slice. All length/count arithmetic uses checked `u64`, checked `usize` conversion and
`isize::MAX` fit before slicing or writing.

## Exact limits and resource behavior

`EffectPackageLimitsV1::default()` is frozen as:

- `maximum_descriptor_bytes: u64 = 4_194_304` (4 MiB);
- `maximum_manifest_bytes: u64 = 16_777_216` (16 MiB, counting header + descriptor + table);
- `maximum_package_bytes: u64 = 268_435_456` (256 MiB);
- maximum artifacts: 4,096;
- `maximum_artifact_bytes: u64 = 134_217_728` (128 MiB).

Every configured limit is an inclusive cap and a zero limit rejects any nonempty corresponding
input. Exact-cap and one-byte/count-below tests are mandatory. Each public required-size, encode,
borrowed-verify or package-CID operation performs exactly one accepted Issue-082
validation-and-identity pass, using `effect_descriptor_identity_v1` or an internal call path with
identical one-pass behavior, before it publishes success or output. That nested pass may use the
accepted Issue-082 verifier's temporary heap under the exact 4,194,304-byte descriptor cap; all of
those temporaries die before the public package operation returns. The package must not call the
descriptor verifier and identity helper separately or otherwise validate the same descriptor twice.

Everything native to the package layer—layout arithmetic, canonical ordering through one
fixed-size stack index of at most 4,096 `u16` slots, table/content parsing, borrowed artifact
iteration/selection and CID binary/text coding—performs no heap allocation. There is no retained
allocation, package-sized copy or hidden artifact-sort `Vec`; the encoder uses only caller output
and that bounded stack index, and the verified result borrows the original immutable package bytes.
The index is sorted in place with a total comparator over `(canonical key, caller index)`, so the
emitted permutation and the duplicate-key diagnostic are pure functions of the authoring input and
independent of the sort algorithm; an allocating stable sort is forbidden. `maximum_artifacts`
above 4,096 is clamped to 4,096 exactly as `maximum_descriptor_bytes` is clamped to the descriptor
cap, matching the independent reference's `MAX_ARTIFACTS`, on both encode and borrowed verify. Exact allocator-dependent byte counts for the
accepted nested Issue-082 temporaries are explicitly deferred to **Canonical effect interchange
qualification, fuzzing, and benchmark**; Issue 078 freezes their input cap and lifetime, not an
unprovable allocator layout. These APIs are control/offline only and must not become
render-call-graph reachable.

## Frozen Rust API and atomicity

Replace the provisional API with these semantic operations (exact Rust spelling may add lifetimes
but not ownership or behavior):

- `EffectArtifactKindV1::{Source, CoreWasm, TargetNative}` with raw values `1,2,3`;
- borrowed `EffectArtifactAuthoringV1 { kind, path: &str, target: &str, features: &str,
  content: &[u8] }` and `EffectPackageAuthoringV1 { descriptor: &[u8], artifacts: &[...] }`;
- `EffectPackageLimitsV1` with the exact five fields and types above, including
  `maximum_artifacts: u32 = 4_096`;
- borrowed `ArtifactSelectionRequestV1 { kind, target: &str, capabilities: &[&str] }` and
  `VerifiedArtifactV1 { artifact_index, kind, path, target, features, content, sha2_256 }`;

- `effect_package_v1_required_size(&EffectPackageAuthoringV1, EffectPackageLimitsV1) ->
  Result<u64, EffectPackageDiagnosticV1>`;
- `encode_effect_package_v1(&EffectPackageAuthoringV1, EffectPackageLimitsV1, &mut [u8]) ->
  Result<usize, EffectPackageDiagnosticV1>`;
- `verify_effect_package_v1(&[u8], EffectPackageLimitsV1) ->
  Result<VerifiedEffectPackageV1<'_>, EffectPackageDiagnosticV1>`;
- `VerifiedEffectPackageV1::artifacts()` returning a deterministic borrowed iterator;
- `select_effect_package_artifact_v1(&VerifiedEffectPackageV1,
  ArtifactSelectionRequestV1) -> Result<VerifiedArtifactV1<'_>, EffectPackageDiagnosticV1>`;
- `effect_package_cid_v1(&[u8], EffectPackageLimitsV1) ->
  Result<EffectCid, EffectPackageDiagnosticV1>`.

Required-size validates the complete authoring model before returning. Encode performs the same full
preflight before touching output. Every invalid/overflow/short call leaves the entire output slice
bit-identical; short output reports the exact required package bytes. Borrowed verify and selection
return slices only into the immutable verified input. No package C ABI or header is added here; the
accepted Issue-082 C API remains descriptor inspection only.

For encode, the one descriptor validation/identity pass is part of the preflight and therefore
precedes every output write. `effect_package_cid_v1` consumes the result of its one package verify;
it must not trigger a second descriptor pass while hashing the already-verified exact package bytes.

`EffectPackageDiagnosticV1` is a 32-byte `repr(C)` value with `u32 code`, `u32 detail`, `u32
artifact_index`, zero `u32 reserved`, `u64 byte_offset` and `u64 required_bytes`. Unavailable index
is `u32::MAX`; unavailable offset is `u64::MAX`; unused fields are zero/unavailable. Codes are:

`Ok=0, Limit=1, BufferTooSmall=2, Header=3, Length=4, Reserved=5, Enum=6, Offset=7,
Order=8, Path=9, Target=10, Features=11, Descriptor=12, Hash=13, Unavailable=14, Cid=15,
Overflow=16`.

Descriptor failure uses code `Descriptor`, preserves the Issue-082 diagnostic code in `detail`, and
maps its available offset to `96 + descriptor_offset`. Within each verification phase the earliest
record index, then lowest field offset wins. Cross-phase order is: limits/overflow; fixed header;
reserved; declared lengths/section arithmetic; descriptor; table structure/offset/padding; kind and
string grammar; record order; hashes; required Source invariant. Authoring validation follows field
order descriptor, artifact count, then caller record index and path/target/features/content. Rust
and independent Python freeze the same package diagnostic identity. The accepted Issue-082 C smoke
continues to cover the embedded descriptor boundary; no new package C symbol is exported.

## CIDv1 binary and lowercase base32

`EffectCid` is exactly 36 bytes:

`0x01 || 0x55 || 0x12 || 0x20 || SHA-256(exact complete package bytes)`

for CIDv1, raw codec and SHA2-256 multihash. Canonical text is exactly 59 ASCII bytes: multibase
prefix `b` followed by 58 lowercase RFC-4648 base32 digits using `a-z2-7`, without padding.
Parsing rejects any other length/prefix/alphabet, uppercase, `=`, nonzero unused trailing bits,
noncanonical varints, codec/hash/length mismatch or non-round-tripping spelling.

Retain `EffectCid::as_binary`, strict `from_binary`, `Display` and `FromStr`; add a 59-byte
caller-buffer text writer whose short/error path leaves output untouched. A raw-byte CID primitive
may remain for the official `hello` vector, but package CID creation must call the package verifier
first. CID verification compares the digest of the exact verified package bytes.

## Deterministic artifact compatibility and precedence

Selection validates the request before scanning. Kind and target match exactly; there is no target,
architecture, ABI or kind fallback. Request capabilities use the same token grammar and must be
strictly sorted and unique. An artifact is compatible iff every artifact feature token is present
in the request capabilities; no feature implies another.

Choose the compatible candidate with the greatest feature-token count. Break equal-count ties by
lexicographically smaller feature string, then lexicographically smaller path. Return `Unavailable`
if none match. Before returning, recompute the selected content SHA-256 and compare it with the table
record even though package verification already checked every record. Tests cover empty baseline,
strict supersets, equal-cardinality ties, exact target/kind rejection and reordered request/artifact
inputs.

## Objective acceptance gates

Check in two Python-standard-library-authored vectors under `fixtures/effect-package/v1/`. Each binds
one accepted Issue-082 descriptor and distinct Source/CoreWasm/TargetNative bytes; one includes
baseline and multi-feature selection alternatives. Freeze package byte count/SHA-256, 36-byte CID,
59-byte CID text, descriptor identity, every content hash and a sorted manifest before Rust golden
tests consume them. The reference must independently encode, decode/re-encode and select; it may not
shell to Rust or copy candidate output.

Executed tests must prove:

- authoring permutation gives identical package/CID and every canonical semantic change changes CID;
- descriptor changes without matching identity reject; a legal descriptor plus matching identity
  yields a different CID;
- every header/length/reserved/table/padding/offset/order/path/target/feature/hash/source mutation
  has the exact diagnostic, including truncation/trailing and checked-overflow cases;
- content mutation without its hash rejects, while changing content plus its hash accepts with a
  different CID;
- required-size, exact output, one-short and every configured one-below limit preserve canaries and
  all-or-none publication;
- CID official `hello` raw vector, binary/text round trip and exhaustive prefix/alphabet/padding/
  trailing-bit mutations;
- the frozen selection matrix, selected-content rehash and borrowed slice identity;
- current production descriptor packages verify; allocation instrumentation distinguishes exactly
  one permitted nested Issue-082 pass from zero package-native allocations and proves no allocation
  survives return; and no package/CID/selector path is render reachable.

Pass focused package tests/check/Clippy/rustdoc, independent-reference/manifest read-only checks,
native tests, scalar `wasm32-unknown-unknown` compile/object checks, format, workspace/realtime policy
and mutations, static dependency/unsafe/artifact scans, then one proportional locked nonbenchmark
workspace seal. No benchmark or timed command is authorized.

## Final Sol evidence — 2026-08-22

**PASS / COMPLETE / READY TO CLOSE.** The allocation-boundary briefing correction at `f295734`
permitted exactly one accepted Issue-082 validation-and-identity pass, bounded by the 4,194,304-byte
descriptor cap, while retaining the allocation-free package-native contract. Terra completed the
product in coherent checkpoints `6af336c` (canonical package wire), `d375db3` (CID), `05cfabd`
(verified selection) and `ae02d2a` (qualification). Sol's single bounded correction produced clean
candidate `40fbcff97f82210b81db112c42dc162cf156a6b1` on `codex/batch-feature-078`; it changed only
`src/package.rs`, `src/cid.rs` and `scripts/effect-package-v1-reference.py` (241 insertions, 107
deletions). The correction made partial-table prefix limit/reserved priority executable, proved
encode canaries for every one-below configured limit, froze complete independent diagnostic
identities including overflow and the 128 MiB artifact cap, and exhaustively rejected every invalid
ASCII CID prefix/alphabet value and position.

The two independent vectors remain exact. Comprehensive A is 2,547 package bytes with package
SHA-256 `af7b5d38afd3191c33d9d40d95d933ff9b83fe949cb95c3d80bd7bbf916daa52` and CID text
`bafkreifppnotrl6tdeodhwoubwk5sm77tob75fe4xfod3af5po7zc3nkki`. Comprehensive B is 1,327
package bytes with package SHA-256
`6a5934e1222a8601c0aca2194da10f00cc5357596b6355f6c5d64baf748f532c` and CID text
`bafkreidkle2ocirkqya4blfcdfg2cdyazrjvowllmnk7nrowjoxxjd2tfq`. The sorted fixture manifest
SHA-256 is `74cb06877960c1675e24742b65df373254ce71341522e2196038053c9d571bf3`.

Executed evidence proves the 96-byte header and 72-byte artifact prefix; exact canonical grammar,
ordering, zero padding, contiguous content and per-content hashes; checked limits/host fit and
32-byte diagnostic phases; borrowed private-construction verified values; 36-byte CIDv1 binary and
59-byte lowercase base32; deterministic feature-subset precedence and selected-content rehash; and
atomic required-size/encode failure behavior. Allocation instrumentation observed exactly one
nested Issue-082 pass for each publication operation, zero package-native allocations, and no
surviving allocation. Current production descriptors, independent Python encode/verify/re-encode,
native execution and scalar-Wasm compile/object/no-SIMD evidence all passed.

The clean candidate passed `cargo fmt --all -- --check`; locked workspace all-target/all-feature
check and tests; warning-denied locked workspace all-target/all-feature Clippy and rustdoc; the
effect-package reference/native/scalar-Wasm check; workspace, realtime and effect-runtime policies
plus applicable mutation suites; and dependency, allocation/unsafe, render-reachability,
generated-artifact and diff scans. Final counters are `workload_invocations=0`,
`benchmark_invocations=0`, `timed_invocations=0`, `audit_invocations=0`,
`browser_invocations=0`, and `general_target_invocations=0`.

Issue 078 is complete and ready for evidence commit/push, GitHub synchronization and closure. It
directly unblocks **Third-party WASM package and effect ABI conformance kit** (Issue 027) and
**Canonical effect interchange qualification, fuzzing, and benchmark** (Issue 081).

## Amendment — audit #97, 2026-08-24

The "bounded repeated scans" wording above was amended to "one fixed-size stack index of at most
4,096 `u16` slots" after audit #97 measured the repeated-scan encoder at the frozen artifact cap:
`effect_package_v1_required_size` 1.37 s and `encode_effect_package_v1` 5.55 s for a 366 KB package
that `verify_effect_package_v1` accepts in 0.33 ms (release, 4,096 one-byte source artifacts). The
replacement sorts an `[u16; 4096]` stack index in place with `sort_unstable_by` over
`(canonical key, caller index)`: 3.44 ms for the same `required_size` + `encode` pair, still zero
package-native heap allocation under the tracking allocator (exactly the 8 nested Issue-082
descriptor-pass allocations at n = 4,096), and byte-identical output, CIDs and diagnostics — both
frozen vectors and the independent Python reference corpus are unchanged. The clamp of
`maximum_artifacts` to 4,096 removes a Rust-only divergence from the reference's `MAX_ARTIFACTS`,
which already hard-rejected larger counts on both encode and verify. Gate:
`encode_at_the_frozen_artifact_cap_has_one_nested_descriptor_pass_and_no_native_allocation` in
`crates/miso-engine-effect-package/tests/package_allocation.rs`; the package check script now also
rejects the allocating stable sorts and `cargo check --locked`s the `fuzz/` crate.

## Allowed files and non-goals

Allowed implementation surface is `crates/miso-engine-effect-package/src/{package,cid,diagnostic,
lib}.rs`, its direct `Cargo.toml`/mechanical root `Cargo.lock` entry if required, package-only tests,
`fixtures/effect-package/v1/`, one independent Python reference, one package check script and minimal
direct workspace/realtime policy mutations. `compile.rs` may receive only a mechanical type-name
adaptation if compilation requires it. Descriptor wire/FFI/header/vectors, effect-contract,
compiler, state and migration modules are read-only.

Non-goals: package C ABI; state envelope or migration; signatures, trust, licensing, repository or
network resolution; installation; artifact ABI or executable validation; third-party execution;
broad fuzz/100-process/multitarget qualification; benchmark, timing or listening.

## Dependencies by exact issue title

- Close canonical effect descriptor wire, identity, and C inspection ABI
- DSP research corpus and conformance harness
- Native effect runtime contract and conformance

## Downstream boundary

No dependency rewire is needed. Accepted Issue 078 directly unblocks **Third-party WASM package and
effect ABI conformance kit**. **Canonical effect interchange qualification, fuzzing, and benchmark**
continues to depend on accepted Issues 078 and 080; it owns broad parser fuzzing, 100-process,
multitarget, allocation audit and the sole later descriptive benchmark.

## References

- [CIDv1 specification](https://specs.ipfs.tech/cid/)
- [FIPS 180-4, Secure Hash Standard](https://csrc.nist.gov/pubs/fips/180-4/upd1/final)
