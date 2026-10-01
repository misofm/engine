# Housekeeping: source

## Authorized scope and smallest closable slice

Review the complete `crates/source` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/source` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — GPT-6.1 Sol xhigh worker A, 2026-10-01

Read the complete manifest, production module and both integration tests (4,429 Rust lines at
`e376d134`), plus related #560 IO12/IO13 and #938 scope. Root product checkpoint `654c7c62`
changes four paths: package +14/-138 (**net -124 LoC**), plus one deleted Cargo.lock dependency
entry. The pre-edit scope amendment below authorizes exactly that lockfile row; no versions,
checksums or other dependency lists moved.

#### Five-axis findings and implemented cleanup

| Axis | Finding/decision |
|---|---|
| Repetition/LoC | Folded single-caller `submit_planes` into `submit`: complete metadata/plane validation occurs once before recycled storage is acquired, keeping error precedence and publication-before-ack. Its only caller always supplied zero native-decoder replacements; that block metadata is still cleared to zero, and every public telemetry field remains. `flush_deferred_recycle` now calls the existing ownership-preserving `recycle_block`. |
| Test value | Deleted the zero-claim test that mirrored the retained-report sum and asserted producer refill: #917's extra retained block permits that refill even without zero-claim release. The surviving `graph_driver_played_planes_map_claims_until_the_next_begin_or_seek` explicitly asserts no played block after zero-claim begin. Removed the SHA/byte serialization of generated schedule actions, keeping all 256 independent model schedules and all behavioral assertions; that hash was neither a wire format nor a shared cross-target corpus owner. Two historical oracles remain and require the bounded revision recorded below. |
| Copies | `HoldingDriver` takes its necessary pre-copy snapshot directly as bits, dropping the intermediate float Vec and subsequent second conversion: two Vec allocations/copies fewer per played quantum in this test helper. Production submission's one planar copy transfers caller-borrowed PCM into owned prepared storage; it cannot be removed without a reservation/lifetime API. Played planes already permit zero-copy fanout. Mapping/claim stable-ID clones have separate ownership and are charged. |
| Micro SIMD | Reviewed per-plane submission/copy, short-tail fill, bounded stale-block drain and graph channel mapping. Bulk PCM operations already use slice copy/fill; queue/generation logic moves owned blocks and contains no arithmetic kernel to bank. No additional ISA code/dispatch or arithmetic change is justified, and no SIMD speedup is claimed. Native AVX2/FMA and Wasm/NEON support stay in lane. |
| Data structures | Fixed transfer-block pool, data/recycle SPSC rings and one-slot seek admission preserve exact configured depth plus one consumer-retained block. Queue moves transfer Box ownership, rather than copying PCM records; the graph driver uses direct indexed mappings and borrowed played planes. These choices bound memory by configured capacity and frame/channel shape, not stem duration. Changing depth, pooling layout or ownership is unnecessary. |

Host-provided sample words remain exact, including NaNs/subnormals/signed zero; this host-fed ring
does not sanitize them. Removed native decoding already left host sanitizer counters at zero.
Generation, contiguous-frame/EOF admission, underrun positive zero/counts, stale-block discard,
and retained-block ownership are unchanged. The only render-side cleanup delegates to the same
SPSC push/failure ownership path; it introduces no allocation/free, Arc operation, lock or syscall.
No new test, algorithm or timed benchmark was added.

#### Complete test-purpose assessment

| Test or homogeneous family | Plausible load-bearing defect / disposition |
|---|---|
| Resource separation report, fixed-shape rejection, one-quantum shape/readback, host-region preparation | Double-charging retained PCM or omitting metadata/queues; invalid channels/quantum/capacity admitted; producer/consumer shape diverges; nonzero initial absolute source origin lost. Exact charge formulas are public accounting claims, not incidental compiled-byte budget pins. |
| FIFO/full/no-prefix and rejected-submission immutability | Partial chunk publication, wrong FIFO across wrap, stale/short-invalid submission mutating producer telemetry or leaving rejected audio ahead of fresh PCM. |
| Registry/path and host-shape errors | Stable diagnostic identity changes or wrong channel/rate accepted. Other invalid chunk classes and precedence are reached by the randomized model. |
| Underrun/EOF, copy-channel full/short/None poison checks, played-plane poisoned-tail/absence | Missing positive-zero writes or PCM prefix, EOF counted as underrun, out-of-range plane exposed, or reused block tail retaining old hostile words. The old-copy oracle itself adds no distinct behavior beyond explicit supplied-prefix/zero-tail expectations and must be replaced. |
| Repeat fanout/auto recycle and configured-depth borrowed-plane integrity | Repeat channel copy consumes frames; invalid copy releases the block; producer overwrites a live played borrow or admits a count other than configured depth. Counts 1..3 and both planes remain. |
| Paused seek/full queues and boundary seek/stale discard | Preparation consumes the target frame, drops current-generation PCM, fails to reclaim queued/retained stale storage, applies seek at the wrong boundary or accepts a non-increasing generation. |
| Graph underrun/generation facts | Lost source invalidity or pending between-block seek fact when graph begin runs. |
| Driver retain-through complete/incomplete/failed claim; mapped borrowed planes through begin/seek/zero claims | Premature recycle on last/missing/error claim, wrong source-channel mapping, stale plane after next begin/seek, or played storage retained despite zero claims. This last explicit absence check supersedes the deleted zero-claim smoke/sum test. |
| Four-channel/three-input sequential graph fixture and transactional binding refusals | Wrong repeated/reversed channel fanout or graph reduction; missing, extra, duplicate or doubly-owned source claims accepted instead of returning the rejected source set. |
| `played_block_retention_keeps_the_pre_change_admission_sequence` | Historical hold/end/driver replay checks credit transitions and counters, but permanently stores old implementation output. Normal paths are covered by the gates above and independent models; only preparation-specific admission-depth behavior merits a small replacement. Delete the script, literal transcript and wrappers in attempt 2. |
| Fixed 256 schedules / independent seek model | FIFO/depth/wrap/seek/late-block/short-or-empty EOF and modeled output/report/stale-count mismatch over capacities 1/2/3/8. Every actual outcome still compares against an independent model after SHA removal. |
| Randomized independent ring model | Edge-biased quantum 1..16, 1..3 channels, capacities 1/2/3/8, all 9 submit and 4 seek verdicts, future/late/stale data, hostile sample words, zero/short terminal blocks and one-slot seek backpressure disagree with the model. Reach counters require every verdict, underruns and EOF. Models use `read_block`; direct seek/driver tests separately check generation-change facts and preparation. |

Unit count falls 20→19; both integration tests remain. No independent model or ownership/fault
gate was deleted. The historical oracle debt below prevents an attempt-1 PASS claim.

#### Actual checks

All Cargo commands used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a`.

- Base `cargo test --locked -p source --all-features`: 20 unit +2 integration passed.
- Head same command, debug and `--release`: 19+2 passed each; zero doctests. The package has no
  declared features, so its default/all-feature surfaces coincide.
- `cargo clippy --locked -p source --all-targets --all-features -- -D warnings`: **exit 0**, with
  configuration warnings that `clippy.toml` paths `math::fast_db::{fast_level_db,
  fast_gain_from_db}` are unreachable in this selected feature graph. The warnings propagate
  through dependencies and source/test targets despite `-D warnings`; this is not a warning-free
  result. Global configuration repair is outside this package's approved ownership.
- `cargo fmt --all --check`, `git diff --check`, `scripts/check-realtime-policy.sh`, and
  `scripts/check-realtime-audit-leak.sh`: passed. No source-specific policy script exists.
- `cargo check --locked -p source --all-targets --all-features --target ...`: passed for Wasm
  with `RUSTFLAGS='-C target-feature=+simd128'`, AArch64 iOS and Android. These compile checks do
  not execute target binaries. An extra bare-Wasm check failed at lane's deliberate unsupported
  target refusal (plus the unreachable Backend return); Wasm simd128 is the supported contract,
  and the passing check uses it. No source regression or scalar-tail removal is claimed.

#### Root review disposition and frozen bounded revision

Root identified retained test debt after the compiling checkpoint: `copy_channel_oracle` is the
verbatim former production body, and `PRE_RETENTION_ADMISSION_ORACLE` permanently pins a former
ring's transcript. Attempt 1 awaits root **REQUEST CHANGES**; the worker has paused product edits.
Attempt 2 is scoped to borrowed explicit expected slices in the four-poison helper, deletion of
duplicate direct assertions and the old script/transcript/wrappers, and one small public-contract
preparation-depth gate. That gate uniquely catches the idle retained block mistakenly recycled
during `prepare_seek`: after empty-generation preparation admit exactly configured depth, repeat
preparation to prefetch without consuming PCM, and refuse an extra admission. Existing models do
not invoke preparation, and existing paused-seek cases do not fill to this refusal boundary.
Root records/commits its verdict before authorizing that single revision. No owner API/feature
decision or extra work on deferred #1154 is requested by this package.

## Scope amendment — dependency cleanup

Root Sol authorizes removal of source's test-only SHA transcript pin and unused sha2 dev dependency while preserving every independent model schedule/assertion. `Cargo.lock` may change only the `source` package dependency list to reflect that removal; no package version, checksum or other dependency list may change. Approved before implementation on 2026-10-01.

### Root adversarial review — attempt 1

Root adversarial verdict: REQUEST CHANGES. Submission validation still precedes storage acquisition/publication; the removed second validation received the same metadata and a constant-zero sanitizer count, and the deferred-recycle helper preserves failed-push ownership. The source-only Cargo.lock removal matches the amended brief. Focused debug/release and supported-target checks are sufficient for those product changes, with clippy configuration warnings recorded honestly. However, the old-copy body and pre-retention literal transcript remain permanent historical equivalence gates. They must be removed before the test-value claim passes.

Attempt 2 is authorized exactly as the frozen revision above: explicit borrowed expected PCM/zero tails for the poison helper, delete redundant assertions and historical admission scaffolding, and retain a small preparation-depth contract gate. Test value: the rewritten poison checks catch stale destination words and incorrect source prefixes across full/short/absent blocks without recomputing expectations from private production state; the preparation-depth gate catches accidental recycling of the consumer's reserved block or consuming prefetched PCM during repeated preparation, neither reached by the read_block models. No product changes or additional target matrix are needed.
