# Housekeeping: engine

## Authorized scope and smallest closable slice

Review the complete `crates/engine` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/engine` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — GPT-6.1 Sol xhigh worker A, 2026-10-01

Read all nine production Rust modules, the integration test, Cargo manifest and mutation record
(4,866 package source/test/documentation lines at base `72e8d0ba`). Reviewed the relevant open
#897 reserve/commit, #887 partial-bank arena and #559/#560 audit handoffs before editing their
subjects. Product checkpoint `f80d6dfc` changes three files, +37/-67 lines: **net -30**. No public
API, feature, arithmetic, ordering, allocation or ISA contract changes; no timed benchmark or
performance percentage is claimed.

#### All five review axes

| Axis | Evidence and decision |
|---|---|
| Repetition/LoC | `spsc.rs::try_push` now uses the existing `try_reserve`/`PushPermit::commit`, removing its second implementation of slot initialization, Release publication, local-cursor advance and saturating success counting. Full admission still increments the same full counter and returns the original value/generation. Typed plan/executor forwarding methods and their distinct audit diagnostics remain clearer than another abstraction. |
| Test value | Removed one snapshot test whose central claim was that a copied local `usize` does not change after a later publication. The retained wrapped-cursor test already checks snapshot values 0/1/full across pushes/pops. Removed literal ring-header footprint pins (256/192 bytes), keeping physical slot count and cache-line alignment. Renamed the move-only test to describe its actual ownership/full-return assertions; it never measured drops. Full family assessment is below. |
| Copies | The concurrent arena probe borrows each stable buffer-ID list through scoped threads instead of cloning it for each of 6 writers in each of 200 rounds: 1,200 Vec clones removed. It still moves each lease to its writer and joins every writer before inspection. Preparation-owned shape/write/read lists, Arc endpoint ownership, queued move-only values and response snapshots earn their copies/lifetimes. #897's large host spectrum queue-copy reduction remains its own issue. |
| Micro SIMD | This crate has ownership/storage/publication primitives and a zero-fill reference renderer; bulk fill/copy operations already use slice primitives. Atomic cursor/observation fields need their existing publication ordering, and small fixed ID checks contain no f32 kernel to vectorize. Native optimized base/head probes show the 4,096-byte record wrapper still performs one memcpy into the queue slot; no extra stack record or copy appears. No new ISA code or dispatch is justified. |
| Data structures | SPSC uses exact caller capacity plus one sentinel, cached peer cursors, compare-wrap and separate cache lines. Observation cells conflate levels with bounded reads. Flat PCM storage and one-byte-per-buffer lease access maps preserve contiguous data and O(1) write membership; duplicate checks scan at most 8 IDs. No demonstrated structure bottleneck warrants a representation/API change. The unused-by-graph generic BufferArena question remains #559 RT13. |

The consolidation retains the same cached-full Acquire reload and slot-before-Release ordering.
The internal permit exclusively borrows the producer; commit cannot race another producer
operation or publish twice. A rejected push retains `T`; successful pop moves it once. No Arc
clone/drop or allocation was added to either operation.

#### Retained load-bearing test families

| Family (names identify homogeneous groups) | Plausible defect defended |
|---|---|
| `hex_lower_encodes_fixed_literal_cases`; launch-rate predicate/prepare/render/refusal | Wrong nibble/order or accepting a removed rate; preparation must actually reject it, and every accepted rate renders. The set-membership unit gate alone cannot prove plan admission. |
| Audit forbidden hooks and armed panic | Hooks armed outside render, missing any forbidden-operation refusal, or escaped unwind not counted. |
| BufferArena fixed/disjoint/zero-frame; strided stereo/padding and non-stereo refusal | Wrong plane offsets, zero capacity admitted, padding overwritten, or stereo accessor accepting another shape. |
| Disjoint builder overlap/same-wave/silence/unproduced-read refusals | Each distinct structural ownership/dependency violation admitted before publication. |
| Later-wave audio, own-buffer reads, release read-ID bounds | Lost plane data/dependency transfer, own-read rejection, or missing release bounds guard. The exact arena-byte formula checks its public accounting query, rather than freezing a resource budget. |
| `write_read_many` slice/address/word differential and unsound-borrow refusals | Wrong plane/buffer offsets or missing output-alias, write-set, read-ID or plane rejection. Repeated reads/silence and supported min/max counts remain. |
| Stereo-many invalid shapes/no partial writes | Mono/unsupported width, duplicate/unwritable/silence/out-of-range IDs or excess frames forming references/writes before refusal. |
| Concurrent arena foreign-word probe (rewritten) | Actual unsafe storage offset/ownership divergence writes another lease's words under overlapping writers; sequential builder checks cannot detect that. Same 6 leases × 5 buffers × 2 planes × 17 frames × 200 rounds and silence checks; scoped joins retain the happens-before boundary. |
| Native full/FIFO/wrap/generation/counters; move-only public constructor; endpoint destruction | Dropped/reforged refused value, wrong FIFO or count, Copy requirement accidentally introduced, or queued owned payload leaked/dropped twice when consumer dies first. |
| Million-item concurrent SPSC and real-ring Loom | Missing cursor Acquire/Release or unsound cached-full/empty refresh under native stress and explored wrap interleavings. |
| Compare-wrap/remainder; wrapped consumer availability; producer capacity/counter stability | Wrong ring wrap/count formula, availability larger than capacity, or snapshot changing producer counters. |
| Ring slot-count/cache-line alignment (trimmed) and plan-exchange resource projection/overflow | Wrong physical sentinel count/alignment report or queue budget arithmetic overflowing before allocation. No other surviving test checks the alignment report; literal compiled footprint pins add no product budget claim. |
| Legacy exchange retirement-full defer/reclaim; reserved credit/epoch admission | Active plan dropped on render or complete replacement admitted without guaranteed retirement storage. |
| Cancel/drop, failure precedence/FIFO, queued and pending legacy predecessor reservations | Credits/epochs leaked on cancellation, candidate ownership lost, replacements reordered, or an acked reserved successor strands an already admitted predecessor. The queued and pending states are distinct. |
| Concurrent plan publication/off-render retirement | Partial plan visible, epochs regress, or displaced plan Drop runs on the render owner. |
| Contiguous clock and response capture after successful/failed render | Stale block admitted, clock advanced wrongly, or snapshot copied from a refused render boundary. |
| Four observation transport cases: concurrent whole/order/gap, stalled latest/monotonic ack, repeated reads/final gap, exact words | Torn atomic window, backlog instead of conflation, backwards acknowledgment, missed-window double counting, or signed-zero/subnormal/u64 field corruption. |
| PreparedRenderPlan compile-fail doctest | Exclusive render owner accidentally becomes Sync. |

No new test was added. The scoped concurrency rewrite and trimmed alignment test retain the
unique purposes above. Unit count falls 39→38 under all features; the four integration tests and
one compile-fail doctest stay. Existing FIFO, move/drop, retirement, numeric-word, target and RT
gates cover the changed ownership path; no DSP oracle or fault/allocation gate was removed.

#### Actual checks and one-time comparisons

All Cargo commands used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a`.

- Base `cargo test --locked -p engine --all-features`: 39 unit +4 integration +1 doctest passed.
- Head same command, debug and `--release`: 38+4+1 passed each. Default-feature debug:
  `cargo test --locked -p engine`, 36+4+1 passed.
- CI recipe `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine
  --lib spsc_loom`: the real-ring model passed (one test).
- `cargo clippy --locked -p engine --all-targets --all-features -- -D warnings`,
  `cargo fmt --all --check`, `git diff --check`, `scripts/check-realtime-policy.sh` and
  `scripts/check-realtime-audit-leak.sh`: passed.
- `cargo check --locked -p engine --lib --all-features --target ...`: passed for
  `wasm32-unknown-unknown` without additional ISA flags and with `-C target-feature=+simd128`,
  `aarch64-apple-ios`, and `aarch64-linux-android`. These are compile checks, not target execution.
- Temporary optimized native base/head move-only trace: capacities 1..9, 240 push/pop actions
  each, both endpoint-drop orders. All 4,329 lines match, including returned IDs/generations,
  full/empty/success counters, queue availability and drop totals. This is one-time evidence,
  not a permanent old-implementation oracle or digest test.
- Temporary native opt-level=3 AVX2/FMA wrappers: 4,096-byte record function instructions are
  text-identical. The u64 Result wrapper differs in register allocation and placement of its
  shared-ring pointer reload across the cached-full branches; each executed branch retains one
  reload, the same slot write/Release publication and saturating counters, with no extra copy or
  atomic. Whole assembly text comparison is **not** identical (also differing panic line data).
  The first probe invocation needed its rlib renamed to Rust's required `lib*.rlib` spelling;
  it had not produced assembly. Scratch files are under `/tmp/engine-housekeeping-1115/` and
  are not committed or claimed as a cross-target corpus.

#### Deferred decision: confirmed arena safety issue #1154

The public safe legacy accessors depend on debug-only write-set/alias checks (`checked_write`,
`write`/`write_stereo`, `write_read`/`write_read2`/`write_read_stereo`) and caller-supplied E1
ordering across leases. `read`/`offset` also need plane/overflow review. Root confirmed this and
created the separate stateless issue #1154; its API direction requires a bounded safety brief,
not implementation inside behavior-preserving #1115. Existing valid production rendering is
unchanged; these housekeeping gates do not establish soundness for arbitrary safe callers.

Safe-only constructions for that investigation: build a 2-plane, 3-frame arena, reserve `a=1`,
and declare a lease with writes/reads `[a]`. Release `lease.write_read(0,a,a)` can form overlapping
mutable/shared slices. Two properly wave-ordered leases can hold producer `write(0,a)` and
consumer `read(0,a)` simultaneously through separate lease objects; safe Rust does not enforce
E1. Out-of-write-set legacy mutation can similarly address another lease's buffer in release.
The same 2-plane/2-buffer/3-frame arena has 12 cells: `read(usize::MAX/6 + 2,a)` wraps its release
offset to 11 and forms a three-word slice from the final cell. **No UB reproducer was executed.**
The final owner question is the checked-capability versus narrowed/unsafe multi-lease API choice
recorded in #1154. #897/#887 and #559 RT13 remain separate pre-existing outcomes.

### Root adversarial review — attempt 1

Root adversarial verdict: PASS for the frozen housekeeping scope. Reviewed the exact reserve/commit path, preserved full-value/generation ownership and saturating counters, and retained slot-before-Release publication and cached peer Acquire refresh. Loom, native stress, focused debug/release/feature checks and the one-time move/drop trace support the consolidation. The optimized record wrapper adds no copy; the changed register allocation in the u64 wrapper is recorded without a timing claim.

Test value: scoped arena writers still reject real cross-lease storage corruption under concurrent writes, which bind-time structural checks cannot detect; borrowing their immutable ID lists preserves every previous writer/word assertion. The trimmed alignment test rejects a wrong physical sentinel count or reported cache-line alignment, without fixing a compiled header footprint. The renamed move-only test rejects lost/reforged refused payloads and a Copy-only queue contract. The deleted local-usize snapshot assertion adds no behavior beyond the retained wrapped-cursor availability gate. The separately confirmed safe-API defects are tracked in #1154 and collected for owner review; this PASS does not assert arbitrary arena-call soundness.
