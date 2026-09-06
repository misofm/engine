# #512 — Use the existing hex authority for DSP cross-target digest diagnostics

Read-only inspection in /home/bl/misofm/engine-slot-reservation against the supplied delivered0b8cf178 base, compared with /tmp/astra-cp20-remaining-hex-inventory.md. No Git/GitHub, builds, tests, runner, timing or repository mutations performed. This is an unnumbered proposed scope for root to number and synchronize before actual-base approval and Luna1.

The delivered bench/audit slices remove the previously listed duplicate encoders in those packages. The remaining inventory is still materially accurate. Existing dependency boundaries permit further consolidation without inventing a utility crate: wasm-gates and parameter-metadata have normal bench-support dependencies; compressor, parametric-eq, transient-shaper and true-peak-limiter already have dev-only bench-support dependencies. The latter four form one coherent cross-target test-diagnostic outcome and are the recommended next slice. Do not make one PR per helper or combine unrelated tooling/production migrations into this issue.

## Proposed title and minimum product contract

Use the existing hex authority for DSP cross-target digest diagnostics.

Exactly these four existing integration-test helpers are in scope:
- crates/compressor/tests/cross_target.rs::hex, currently borrowed &[u8; 32].
- crates/parametric-eq/tests/determinism.rs::hex, currently owned [u8; 32].
- crates/transient-shaper/tests/cross_target.rs::hex, currently borrowed &[u8; 32].
- crates/true-peak-limiter/tests/determinism.rs::hex, currently borrowed &[u8; 32].

Replace the four formatting bodies with delegation to bench_support::digest::hex. Preserve each private helper's exact existing signature and every existing call site; use hex(&digest) inside the owned parametric-eq adapter and hex(bytes) inside the borrowed adapters. These are compatibility adapters, not independent encoders. Use the fully qualified shared name if that avoids local-name collisions. No shared API, Cargo/lock or production source change is needed. This is ordinary finalized-byte encoding: do not use sha256_hex or introduce a second hash.

The contract is byte-for-byte diagnostic/pin-comparison text equivalence. Preserve all raw SHA256 builders, little-endian PCM-word encoding, cases, partitions, widths, comparison/assertion behavior and expected corpus constants. Preserve separate 0x-prefixed Rust-source repin emitters and their environment controls; they serialize a different language and are not duplicated contiguous hex encoders. Do not launch repin paths or change fixtures/pins. No DSP algorithm, sound-quality or performance improvement is claimed.

## Finite independent gates

Add one top-level test named shared_hex_adapter_matches_literal_bytes in each affected integration test. Invoke that file's actual adapter with [0;32] and with a literal32-byte input consisting of 00,11,22,33,44,55,66,77,88,99,aa,bb,cc,dd,ee,ff repeated twice. Compare directly against literal64-zero text and literal "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff". Pass by value only for the existing owned adapter. Expected text must not be produced by another encoder, format expression or digest operation. This checks width, leading zeros, lowercase and every nibble through the actual consumers without running a DSP corpus.

Run exactly these four named gates in debug and release, each with -- --exact and exactly one successful nonignored test:
- cargo test --locked -p compressor --test cross_target shared_hex_adapter_matches_literal_bytes -- --exact
- cargo test --locked -p parametric-eq --test determinism shared_hex_adapter_matches_literal_bytes -- --exact
- cargo test --locked -p transient-shaper --test cross_target shared_hex_adapter_matches_literal_bytes -- --exact
- cargo test --locked -p true-peak-limiter --test determinism shared_hex_adapter_matches_literal_bytes -- --exact

For release insert --release before the -- separator. Run strict Clippy for each same package and same --test target, ending -- -D warnings; cargo fmt --all -- --check; source/spec git diff --check; existing check-bench-policy.sh and check-realtime-audit-leak.sh. Preserve independent argv/cwd/source identities/stdout/stderr/numeric status, including failures; no zero-test success. Required CI remains enabled and may run its existing broader corpus. There is no reason for local fixture generation, a full corpus replay, browser rebuild, listening, timing, runner preflight or benchmark harness to qualify this formatting-only slice.

A targeted script/doc search did not reveal a source checksum over these four files. That is not permission to bypass a newly discovered concrete seal. Root must compare current inputs and check applicable static constraints at numbered activation. Preserve historical validators and source identities; do not repin historical artifacts to claim they represent a newly built tool.

## Remainder and architecture boundary

CP20 stays PARTIAL. Other existing-edge opportunities remain outside this issue: wasm-gates' public fixed32-byte hex wrapper can delegate without widening its API; parameter-metadata's lattice digest can delegate only its already-finalized bytes while preserving its incremental index/tab/canonical/tab/intrinsic/newline stream. wasm-console's eight guest u32 words remain a separate endian contract, not an interchangeable byte helper.

The production graph-compiler/stem identity owners and the other listed host/protocol/math/effect-runtime/package/compiler-test owners lack the appropriate existing direct bench-support edge at their relevant dependency section. They must not acquire tooling/audit dependencies merely for text formatting. For a future repository-wide ruling the concrete choices are: retain production-local encoders while consolidating existing-edge test/tool owners (recommended now); adopt a production-neutral codec authority through an explicit dependency/API decision; or expand an existing production crate's public helper surface and accept that semantic coupling explicitly. No such architectural decision is necessary for this four-test successor, and no new utility crate is proposed.

Root retains issue numbering/synchronization, fresh actual-base scope approval, Luna attempt1 followed by consolidated Astra review, and Sol2/3 only after FAIL. This brief does not spend implementation authority and does not interfere with #511 runtime accounting.

## Numbered activation boundary

Matching GitHub issue #512 created. Based on delivered main0b8cf178 plus #509 delivery record. Await numbered actual-base Astra scope approval before fresh Luna1. Only the four test files and numbered spec/evidence are authorized; #511 runs independently in its own worktree.

## Numbered scope approval

# Astra #512 numbered actual-base scope review — PASS

Approve fresh Luna attempt1 for the numbered four-test-file scope in /home/bl/misofm/engine-dsp-test-hex, root-reported clean pushed8779bcb6 based on delivered0b8cf178 plus #509 closure documentation. Git/GitHub inspection was explicitly excluded from this task; root's head/clean/upstream and matching GitHub512 title/body attestations remain root-owned, not independently reverified here.

Independently compared current filesystem bytes with the previously scoped engine-slot-reservation inputs: all four target integration-test files, all four package manifests, bench-support's digest authority, workspace Cargo.toml/Cargo.lock and .cargo/config.toml match exactly. Verified the numbered spec preserves the complete proposed scope verbatim beneath its numbered heading and appends the activation boundary. Its preserved historical wording about an unnumbered proposal is superseded by that boundary and this approval.

All four packages already have bench-support in dev-dependencies. Their integration tests can use its existing hex function without new dependency edges, features, APIs or production linkage. Preserve the existing owned [u8;32] parametric-eq adapter and the other three borrowed &[u8;32] adapters; replace only their encoder bodies with fully qualified delegation. Do not accidentally recurse via the local hex name or rehash finalized bytes with sha256_hex. Preserve raw hashes, word endian handling, all existing assertions and the separate source-literal repin emitters.

Targeted source-path search across scripts, workflows/specs, documentation and tools found no concrete checksum/shape seal over the four test files beyond this issue's own path roster. The realtime-leak policy explicitly excludes dev edges from production resolution; no manifest change is needed or allowed. Existing bench policy and realtime-leak scripts remain required finite static gates, and any newly discovered concrete seal must be recorded rather than repinned.

The authorized local proof remains four actual-adapter literal tests, each exactly one successful nonignored test in debug and release; strict Clippy for each exact test target, fmt/diff and the two existing policies with independent raw statuses/source identities. Existing corpora remain in source and required CI stays enabled. No full local corpus replay, fixture/listening generation, pin changes, artifact/browser rebuild, benchmark/preflight/capture, timing or utility crate is authorized. CP20 remains partial outside this coherent diagnostic-encoding slice.

No blocking scope findings. Root retains exact-path checkpoints and synchronization, consolidated Astra attempt review and actual-PR/required-CI delivery gates. Sol2/3 follow only a consolidated FAIL. #511 remains independent. No builds/tests/timing, Git/GitHub commands or repository mutations performed; only this /tmp review was written.

Root verified clean pushed8779bcb6 and matching GitHub512 body/title; fresh Luna attempt1 is activated at this checkpoint.

## Luna attempt 1 source and evidence

Source ab4627ae replaces the four private encoder bodies with delegation while preserving signatures and call sites. Eight exact debug/release literal tests each pass one test; four strict same-target Clippy and four static commands pass. Raw records and source identities are preserved in artifacts/issue512-dsp-test-hex-authority. Consolidated Astra review is pending.

## Astra Luna attempt 1 PASS

# #512 Luna attempt 1 — PASS

Reviewed clean f6d9eb6439541537df9e8b1608f99b3afb6a99fe in engine-dsp-test-hex, source ab4627ae5bad60e836d8afa5cf85a3c08c3b5937, against the complete numbered spec and scope approval. No blocking findings.

The cumulative source delta is exactly the four authorized integration-test files. Each old encoder body now calls fully qualified bench_support::digest::hex. Parametric EQ retains its owned [u8;32] signature and borrows inside the adapter; the other three retain &[u8;32]. Existing callers, finalized/raw hash streams, little-endian PCM word handling, pins, corpus assertions and separate repin emitters are unchanged. No production/Cargo/feature/shared-helper changes. The four new tests invoke the actual adapter with the two exact frozen literal inputs and compare against independent literal strings. This is finalized-byte encoding, not rehashing or recursion.

Independently verified all 66 manifest payloads for unique paths, actual SHA256 and sizes, and exact tracked coverage of 66 payloads plus manifest =67 files. The 64 raw command records comprise16 metadata/stdout/stderr/status sets; every numeric status is0. Eight exact debug/release test commands each report1 passed/0 failed/0 ignored, not zero-match success. Four Clippy commands target precisely their integration tests with `-- -D warnings` and have normal compiler completion output. Fmt/diff and bench/realtime-leak policies also have independent status0; both policies report success.

The precommit metadata honestly identifies HEAD793a6b18 plus the changed test blobs, rather than pretending execution occurred at the later evidence head. All four recorded blobs independently match the source commit and current HEAD bytes; all four supplemental SHA256 values also match. README accurately distinguishes root's later source-identity supplement from original execution provenance and describes the inherited PATH suffix limitation. No full corpus, fixture/repin, browser, listening or timed execution is claimed or required for this slice.

Source contract and finite gates are complete. Root may carry the separately accepted509 post-main documentation, then request actual-PR exact-head review and wait for required CI success. Test-only delegation does not justify a runtime artifact rebuild or pin change. CP20 remains partial beyond these four adapters. No builds/tests/timing or repository/GitHub mutations were performed by this review; only this /tmp report was written.

## Delivered

PR #513 merged as a6a5903082692bca58810f44a40b9717d49254a0 after Astra exact-head PASS at f76da15d6e08dbd8fc0bee5775fc7dff75276b37 and required qualification run34019289183 SUCCESS. GitHub #512 is closed; the four DSP test adapters share the existing encoding authority. This is the48th merged audit PR. CP20 remains partial and no runtime timing claim is made. Post-main qualification pending.
