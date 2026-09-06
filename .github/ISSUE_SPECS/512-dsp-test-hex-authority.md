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
