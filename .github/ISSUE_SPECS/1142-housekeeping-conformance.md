# Housekeeping: conformance

## Authorized scope and smallest closable slice

Review the complete `crates/conformance` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/conformance` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — implementation and verification complete; root verdict recorded below

Worker A (GPT-6.1 Sol xhigh) read all 19 package files: `Cargo.toml`; source `block`, `compare`, `determinism`, `effect`, `fixture`, `lib`, `main`, `manifest`, `prng`, `protocol_corpus`, `randomized`; tests `api_metrics`, `conformance_corpus`, `effect_contract`, `fixture_corruption`, `fixtures`, `mutation_million`; and example `conformance_fixtures`. Baseline: 26 active tests passed, one existing nightly test ignored. The eight-path focused-green product checkpoint is `21a3a6b6`, integrated/pushed by root as `e693a1a7`. Product work is paused.

#### Five-axis findings

1. **Repetition/LoC:** reuse the unchanged CRC body through a private ordered-byte helper; public `crc32c(&[u8])` stays unchanged. Each affected panic/refusal assertion now inspects one parser/validator result. Retire three subset test functions after relocating their unique observations. Correct the enabled impulse-probe comment (D7 bounds, not the position of first nonzero output) and the feature comment naming `engine`. Rust: 60 additions, 95 removals, **−35 lines**. Package including manifest: 61 additions, 96 removals, also −35. No public API, dependency, algorithm, arithmetic or feature change.
2. **Test value:** every retained family and retired-function survivor is mapped below. No new test, generator, historical implementation oracle, output pin or harness. Keep raw NaN/signed-zero checks, numeric extremes, current faulty-mock controls and the intentional repeated protocol decode classifications. The sole native/Wasm protocol corpus hash and all frozen fixture bytes remain unchanged.
3. **Copies:** parser CRC validation streams bytes before offset 40, four zero bytes, then bytes from offset 44, removing one input-sized allocation/copy. Header/shape/limit/length refusals still precede these indexed ranges; polynomial, init/final XOR and byte order are identical. Borrow active report slices and the original automation slice on an unchunked scalar render; real chunks retain their filtered owned spans and the same processing order. Mutable PCM copies for independent arms, snapshots/restore payloads, decoded owned fixture samples, manifest-owned paths and static-lifetime descriptor fixtures remain. The manifest's decimal formatting check is unchanged; no new lexical rule is introduced.
4. **Micro SIMD/codegen:** inspect the existing linked release `fixture_corruption-fd0cd446c123ee11` artifact. PCM word decoding already uses packed YMM `vmovups` loads/stores at `0x31420` onward and a `vmovss` tail at `0x31474`; the decoded fixture owns its samples, so this copy is necessary. CRC at `0x31250` remains sequential byte work (compiler-folded scalar lookup, segment branches), without a new CRC instruction or source algorithm. Ordered scaled RMS/error reductions, first-mismatch diagnostics, PRNG draws and mock state recurrences remain unchanged. The LTO rlib is not directly disassemblable by objdump; the already-linked test supplied the evidence. No added SIMD, timing, instruction-count comparison or performance percentage is claimed.
5. **Data structures:** remove temporary report Vecs and the unchunked span Vec. Keep ordered fixture/corpus/parameter collections and short failure lists, the manifest's borrowed previous-path validation, constant-space metrics and fixed mock delay/state arrays. No demonstrated need for a hash/tree replacement; preserve deterministic ordering, coverage counters and bounded request/resource behavior.

#### Retired functions and surviving defects

| Retired function | Surviving owner and preserved observations |
| --- | --- |
| `fixture_header_limits_and_exact_length_reject` | `every_header_field_limit_overflow_truncation_and_eof_is_rejected`: retain precise invalid-flags `InvalidField` and short-header `TruncatedHeader` results; the existing every-byte truncation loop now checks short headers and partial payload `LengthMismatch` explicitly. Existing exact trailing-data refusal, frame/channel/payload limits and overflow checks supersede the remaining assertions. These failures occur before PCM decoding, so the removed fixture's NaN payload adds no unique header coverage. |
| `manifest_rejects_invalid_classes` | `manifest_rejects_all_noncanonical_text_and_path_classes`: move the wrong-header and three-digit CRC inputs plus valid positive length-one row unchanged. Existing CRLF, unsafe parent path and unsorted rows cover the other classes, with additional duplicate/uppercase-CRC/leading-zero/absolute-path/extension/text refusals. |
| `fixture_trailing_and_truncated_bytes_fail_before_decode` | Complete header/EOF matrix checks every truncation and exact trailing-data refusal; `checked_in_manifest_lists_only_valid_exact_fixtures` retains the actual checked-in corpus's exact lengths/checksums and successful decoding. The retired appended-byte/final-byte-truncation calls exercised the same length branch. |

#### Complete retained test-family purposes

| Current owner | Plausible load-bearing defect |
| --- | --- |
| Library `crc32c_known_vector` | Wrong Castagnoli polynomial, initialization or final XOR independent of encode/parse agreeing with each other. |
| Library `fixture_preserves_bits_and_detects_every_bit_flip` | Normalized signed zero/NaN payload, accepted corruption or panic on any of the original 512 bit flips. Inspect the single caught result; every input and raw-word assertion remains. |
| Library `prng_vectors_and_range_are_frozen` | Wrong current SplitMix64 draws or bipolar range; the four generator vectors and 10,000 range draws stay fixed. |
| Library `block_metrics_and_repeat_bits_cover_boundaries` | Nonzero error on an exact numeric reference or bit-repeat logic that folds signed zero/NaN payloads. This is a validator contract check, not an effect comparing its output with itself. |
| Private `scaled_sum_squares_handles_extreme_finite_values` | Overflow/underflow in scaled RMS on extreme finite values; normal-sized public examples do not reach that branch. |
| `api_metrics` (two tests) | Wrong tolerance boundary/tie ordering or explicit finite/silent/exact SNR classes; incorrect typed rate/tolerance/nonfinite refusal. |
| `conformance_corpus` (three tests) | Drift of the sole 46-frame cross-target wire corpus or a typed decoder; deep transaction generic-envelope refusal (separate API from typed dispatch); missing literal mandatory descriptor-handle flag. Both native and Wasm runners remain independent executions of the same single pin. |
| `correct_factory_binds_distinguishable_four_lane_bank` | Wrong channel/lane gain or three-sample history/addressing in the reference bank; every lane has independent expected words and report width. |
| `normalized_mapping_endpoints_ties_and_round_trips_are_stable` | Incorrect mapping endpoints/inverse, stepped tie selection or NaN refusal. |
| `smoothers_and_segments_finish_on_the_exact_update_or_endpoint` | Wrong linear/one-pole completion sample or segment endpoint/interior law. |
| `correct_mock_passes_every_enabled_conformance_gate` | Harness rejects a conforming factory or prepares/renders nothing: eight rate/bypass configurations and process-call nonvacuity stay. |
| `every_faulty_mock_is_detected` | Detector misses one of 22 current fault controls: actual heap allocation plus audit allocation/free/lock/I/O/network/log/syscall hooks, lane sharing, metadata/tail/bypass latency/resources, malformed spans, nonfinite output, snapshot/restore, panic, partition dependence, sticky reset or bypass delay. The installed audited allocator and harness positive control remain. |
| `ten_thousand_descriptor_and_span_mutations_reject_without_panic` | Invalid descriptor metadata/parameter or malformed automation admission escapes or panics. Preserve both original 10,000-iteration five-class cycles; one caught validator result now proves panic freedom and refusal. No claim of 10,000 distinct mutations. |
| `descriptor_requires_launch_rows_and_refuses_extended_rows` | Missing launch row, accepted extended/unsupported rate, duplicate/unordered rows or incomplete quality; positive launch/multiple-quality controls distinguish an always-refusing validator. |
| `fixture_corruption` (three tests) | Header/EOF/limit/overflow precedence; accepted corruption or panic on every bit and the same 4,096 seeded mutations; noncanonical manifest text/path/ordering. Relocated cases preserve the retired owners' unique distinctions. |
| `fixtures` (two tests) | Corrupt/missing checked-in corpus entry, changed on-disk length/CRC/nonfinite PCM; unsupported rate accepted by block/encode/parse or supported launch rate refused. |
| `mutation_million` active 10,000 plus existing ignored million extension | Complete-schema typed decoding panics or changes error class on the same mutated input. Retain corpus round-robin, length/trailing-byte and byte-mutation draws, double classification and limit assertion. No million-case/nightly expansion was run. |

Shared harness families also remain intact: declared quality/resource/metadata checks; enabled dual-channel bounded impulse; control-vs-impulse dual-mono state isolation; malformed-span admission; exact delayed bypass; real 1/quantum-minus-one/quantum partitions and 100 fresh repeats; hostile main/connected-sidechain bounds; reset and snapshot sentinel/determinism/restore/continuation probes. Randomized bank/scalar, whole/chunked, collapsed/dual/desymmetrization, own/cross-lane/crafted/hostile restore, eligibility/refusal, legal-input nonrecovery, bounded-output and allocation/fault coverage observers keep every original draw, schedule, comparison and census. Fixed D7 report observations remain unchanged.

#### Actual gates, logs and limits

- `cargo test --locked -p conformance --all-features`: baseline **26 PASS / 1 ignored** (`/tmp/housekeeping-a-conformance-baseline.log`); focused final **23 PASS / 1 ignored** (`/tmp/housekeeping-a-conformance-focused.log`). Full `--release --all-features`: **23 PASS / 1 ignored** (`/tmp/housekeeping-a-conformance-release.log`); no doctests or compiler warnings.
- Default `cargo check --locked -p conformance --no-default-features --lib` PASS (`/tmp/housekeeping-a-conformance-default.log`). Frozen fixture generator `cargo run --locked -p conformance --example conformance_fixtures -- --check` PASS (`/tmp/housekeeping-a-conformance-fixtures-check.log`); no fixtures/manifest were written.
- Existing representative `cargo test --locked -p compressor -p delay --test randomized -- --nocapture`: **2 PASS**, unchanged seed budgets, audit true for both (`/tmp/housekeeping-a-conformance-downstream.log`). Compressor: 32 seeds, 770 blocks, 164 chunked, 73 collapsed, 33 disengages, 4,127 state comparisons, 23 crafted restores/witness checks. Delay: six seeds, 211 blocks, 54 chunked, six scalar scenarios, two restores. This covers both borrowed and actual-chunk paths plus current bank/mono reports without a new harness or sweep.
- `cargo clippy --locked -p conformance --all-targets --all-features -- -D warnings` exits 0 (`/tmp/housekeeping-a-conformance-clippy.log`). Existing configuration warnings remain: real `math::fast_db::{fast_level_db,fast_gain_from_db}` paths are unreachable in this graph because those functions require math's `lane` feature; warnings repeat across targets. No source lint failure or workspace config edit; this is not a warning-free invocation.
- `cargo check --locked -p conformance --all-features --target …` PASS for Wasm with `RUSTFLAGS='-C target-feature=+simd128'`, `aarch64-apple-ios` and `aarch64-linux-android` (`/tmp/housekeeping-a-conformance-{wasm,ios,android}.log`). Compile evidence only for iOS/Android, no device execution or bare-Wasm replay. Existing `scripts/check-protocol-wasm-parity.sh` PASS: exported `main` returned the required success, unchanged corpus pin (`/tmp/housekeeping-a-conformance-wasm-corpus.log`). No unchanged red-mutation self-test expansion.
- `cargo fmt -p conformance -- --check`, `git diff --check`, workspace policy, conformance boundaries and realtime-audit feature-leak policies PASS (policy logs `/tmp/housekeeping-a-conformance-{workspace-policy,boundaries,audit-leak}.log`). Native codegen evidence: `/tmp/housekeeping-a-conformance-codegen.log` and `/tmp/housekeeping-a-conformance-pcm-codegen.log`.

#### Deferred boundaries and owner questions

Read current #559 FX16/FX17/FX19 and #1069/#1070/#1071/#1073. The move-only mock split, sibling allocator/digest consolidation, ramp-cut bits, bind-before-validation, subnormal restore and D7 report defects remain with their existing owners. `Known` narrowing/reproducers and generators are unchanged; these gates do not claim those open defects fixed. No new safety/product decision or owner question was discovered. No benchmark, new algorithm/ISA dispatch, listening claim, dependency/lock edit or additional implementation tranche.


## Root adversarial verdict: PASS — attempt 1

Root Sol inspected all eight changed package diffs, the original CRC/parser and retired/surviving matrices, complete family evidence and actual release/fixture/randomized/lint/target/corpus logs. The byte iterator supplies exactly the former copied input with offsets40..44 replaced by zeros after unchanged header/shape/length guards; public CRC API, Castagnoli body and on-disk format stay fixed. Report slices and unchunked automation borrow immutable live owners, while actual chunks retain their filtered storage and call order. All unique retired refusal inputs were relocated. Supported foreign compiles, actual Wasm corpus execution and clippy configuration warnings are stated accurately; no benchmark or broad missing-defect claim is made.

Rewritten-test purposes:

- `fixture_preserves_bits_and_detects_every_bit_flip` catches changed raw signed-zero/NaN words and corruption acceptance, including CRC NaN-payload folding that the finite-input corruption fixture cannot reach; every original flip now uses one caught parse result.
- `ten_thousand_descriptor_and_span_mutations_reject_without_panic` catches invalid descriptor/automation admission or panic across both unchanged five-class cycles, with one caught result preserving both observations.
- `every_header_field_limit_overflow_truncation_and_eof_is_rejected` catches typed header/flags/length/limit/overflow refusal drift, including the relocated exact error distinctions in the complete truncation matrix.
- `manifest_rejects_all_noncanonical_text_and_path_classes` catches malformed CRC/header, path/order or text acceptance while preserving the relocated wrong-header/short-CRC and minimal positive-length cases.

The three retired strict subsets now have stronger named owners; the unique numeric extremes, raw-word cases, genuine fault/allocation observers, intentional repeated decode classifications and single cross-target corpus remain. All five requested axes are complete. No new test, owner decision or product-algorithm change is introduced.
