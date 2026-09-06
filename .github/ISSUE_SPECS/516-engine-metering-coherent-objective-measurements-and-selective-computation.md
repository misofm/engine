# Engine metering: coherent objective measurements and selective computation

## Purpose and relationship

Provide accurate, efficient engine-owned measurement data for native, browser, embedded and agent consumers. Consumers choose presentation, ballistics and delivery cadence; the engine owns the meaning of the measurements. This issue is an assignable implementation plan delivered without implementation in this planning task. Execute the separately closable slices below in order; correctness is the first checkpoint, and selective computation is the planned efficiency outcome.

Operationalizes [#203](https://github.com/misofm/engine/issues/203), the historical console metering overhead finding. Its reported 21% / 17.6–17.9 µs overhead is historical evidence, not a current baseline or promised saving. Do not skip audio samples to reduce publication frequency: missed samples can hide peaks and corrupt energy.

The inspected checkout was HEAD `6a08315c`, with unrelated local work and a moving remote. The host poll findings (last-only peak drain, unconditional master reset and effect-dependent timestamps) were also verified against fetched `hosts/host-web/src/lib.rs` at remote `107b9ed1803b8313e434821ec5bf49a178b6bb2f`. Other preparation/accumulator findings remain checkout observations. Before implementation, synchronize an isolated implementation base, revalidate the findings against it, check the matching GitHub/local issue inventory and reconcile #203. Do not overwrite other work. Keep the implementation spec and GitHub issue synchronized under AGENTS.md.

## Verified starting points

- `crates/builtins/src/lib.rs`, `MeterAccumulator::{observe,emit}`, `observe_segment`, `lane_snapshot`: each non-elided sample currently computes sample peak, f64 energy, clip/sanitization counts and held-peak state. RMS square root runs once per emitted window, not per sample. There is already a settled-silence fast path and segment-based window splitting; do not propose them as new optimizations.
- `crates/host-core/src/prepare.rs`: `ConsolePreparation.meter_period_frames` is all-tracks-or-none. When enabled, it requests one configured tap per canonical console track; no requested metric set exists. Hold and decay are zero, but accumulator work still includes energy and held state.
- `hosts/host-web/src/lib.rs`, `poll_meters`: the track adapter retains only `sample_peak`, overwriting each slot for every drained snapshot. It does not fold earlier drained peaks into the reported frame. `windows` is the minimum popped count, which does not establish equal window identities after loss/discontinuity. This is narrower than saying the underlying accumulator loses the data.
- The same poll copies then clears `master_peak` even when zero track windows completed. Render accumulates master output peak only while the lease is held. With per-block polls and longer track windows, the published master peak can cover only the last block, not the track interval.
- Header `first_sample` / `end_sample` come from armed effect observation windows, not track meter snapshots. Without effect observations they can remain zero; effect observations of different ages form an envelope, not proof of a coherent track/master interval. Poll sequence advances even on empty polls.
- `set_meter_lease` clears host frame/header/master state but does not detach/reset prepared track accumulators or drain their queues. Lease-off prevents polling and the master scan; it does not establish that track observation work is off. Preparation with zero meter blocks genuinely binds no observers. Distinguish these two states.
- `hosts/host-web/web/miso-engine-v1-audio-worklet.js` calls `postMeterFrame` each processed block while leased; that calls the Rust poll before deciding whether to publish. Publication is already gated on completed windows. Polling/publication cadence and sample observation cadence are separate costs.
- `crates/engine/src/realtime/observe.rs` provides conflating **effect** observation cells with explicit sample spans/sequences. Their newest-level semantics must not silently replace interval statistics or discard unreported maxima.

## Smallest closable slice: correct existing meter interval publication

Implement this bounded correctness slice first in the existing accumulator/host adapter architecture. Own only the necessary engine data and host-web adaptation; no SDK or application work. Primary paths: `hosts/host-web/src/lib.rs`, its tests and ABI documentation; touch builtin/host-core plumbing only if required for coherent master interval accumulation. If this requires a new generic subscription subsystem or broad transport/schema redesign, split that dependency into a stateless issue before coding.

### Measurement contract

This defines engine measurement semantics, not a requirement to repeat every field in each legacy browser frame. Stable source/tap and sample-rate metadata may be resolved from the prepared configuration; additive interval/generation/validity metadata must be sufficient to interpret a frame honestly. The first slice preserves its existing peak-oriented payload and does not add RMS, loudness, true-peak or a generic measurement protocol. The energy/RMS rules below govern existing accumulator semantics and any later exposure. Freeze the smallest compatible encoding and aggregation policy in the brief before implementation.

1. A complete measurement identifies its source/tap, dual-mono lanes, reset/plan generation, monotonic window identity, explicit sample rate and half-open absolute render interval `[start_sample,end_sample)`. Frame count equals interval length for contiguous data. Track taps retain their actual signal boundary; master means the final output PCM, not a guessed track. PDC does not authorize silently relabelling observation time as source time.
2. Preserve existing raw linear definitions: sample peak is maximum absolute sanitized sample; energy is sum of squared sanitized samples in f64; RMS is `sqrt(energy/N)`. Silence is a valid measured zero; absent, disabled and stale data are not zero. Sample peak is not true peak, RMS is not loudness, and clipped-sample count does not prove converter clipping. Preserve the current `abs(x) >= 1` count threshold and nonfinite/subnormal-to-zero sanitization semantics. Held peak is stateful across windows and distinct from interval maximum.
3. Normal polls publish a complete track/master span with timestamps derived from measurement data even when no effects exist. An empty poll must not clear unfinished master measurement, manufacture a fresh measurement sequence, or mutate the last complete measurement into an apparent zero window.
4. Choose and document one adapter policy for multiple queued windows: bounded per-window delivery, or an explicitly aggregated contiguous interval. For aggregation, take peak maxima, add energy/counts/frames, derive RMS from total energy/frames (never average RMS), and use the final window's held state. Never report the union span with only the final window's interval peak.
5. Do not combine different generations, noncontiguous windows or mismatched track identities into a coherent-looking frame. Dropped telemetry remains permitted, with explicit loss/sequence-gap information and invalid/partial status where needed. A slow consumer may lose intervals; it must be able to discover that fact. Keep the existing bounded queue unless a separately reviewed issue earns replacement. Document drop-newest behavior and the fact that a failed push's counter becomes visible in a later successful snapshot.
6. Define lease release/reacquisition, seek/discontinuity, full reset and plan replacement. Old queued data must not masquerade as a newly activated interval. Bound stale-data rejection by prepared queue capacity; no callback-side unbounded catch-up. An activation mid-window either waits for the next clean boundary or publishes an explicitly partial interval. Freeze the choice in the decision record before coding.
7. Gain reduction keeps its own declared effect-tap folding/unit semantics. Independently aged effect observations must not supply track/master timestamps or imply matching intervals. Preserve existing ABI spellings; document any necessary additive validity metadata and compatibility behavior. If encoding honest validity requires a larger ABI redesign, separate that work before implementation.

### Realtime constraints

No allocations/frees, locks, syscalls, logging, I/O or structural changes in render or callback-reachable polling. Bound work by prepared track/tap count, quantum and queue capacity, including drains and stale-window rejection. A loop whose queue can be concurrently replenished needs an explicit iteration budget, not just `while pop succeeds`. Account memory with existing resource caps; no compiled maximum track count. Measurement must leave rendered PCM bit-identical and preserve numerical/sanitization behavior. No per-sample clock calls or unit conversions.

### Objective gates

- Deterministic impulse fixtures: a master peak early in a multi-block period survives intervening empty polls; unrelated lane/tap peaks remain independent.
- No-effects session still emits accurate nonzero sample spans. Empty polls leave the last completed publication and pending interval intact.
- Multiple queued windows including an early larger peak discriminate true aggregation from last-only drain. Unequal energies/lengths discriminate energy-weighted RMS from averaging RMS if aggregation exposes RMS.
- Queue saturation, asymmetric stream loss, delayed polls, lease off/on mid-window, discontinuity, reset and plan replacement cannot claim stale or mixed data as coherent. Check counter/generation behavior and bounded drain limits.
- Silence, signed zero, NaN/Inf, subnormal and full-scale threshold fixtures preserve the published contract. Arbitrary track counts include a SIMD-width tail.
- Meter enabled/disabled PCM identity; existing realtime allocation/free and shipped Wasm callback gates; focused `builtins`, `host-core` and `host-web` suites as affected. Cover all four supported sample rates for integer interval arithmetic, with representative native and browser execution. No listening study is needed for an observer-only bit-identical change.

## Planned efficiency slices (create separately closable stateless issues before implementation)

Reviewed successor issues are now published: [#519 — selective computation and observer binding](https://github.com/misofm/engine/issues/519), and [#520 — prepared publication readiness](https://github.com/misofm/engine/issues/520). Their briefs were written by Sol medium and adversarially reviewed PASS by Astra medium. They are not implemented by the correctness PR [#521](https://github.com/misofm/engine/pull/521).

1. **Selective engine measurements and observer selection.** Prepare an explicit requested metric set and stable source/tap set, usable from native/embedded/browser adapters without consumer-specific presets as the authority. Separate raw peak, energy/RMS, counts and optional held-peak state. Mark unrequested values absent. Specialize outside sample loops so peak-only does not accumulate energy or evaluate RMS/held state it never requested. Preserve full-statistics defaults for existing callers. Requests, capacities and structural changes are validated off render and adopted through the normal prepared-plan boundary. Decide runtime activation only if a bounded existing mechanism suffices; otherwise keep it a separate lifecycle issue. A dropped UI lease is not proof of no engine consumers. Acceptance: unrequested observers are not bound; peak-only performs no energy accumulation, square root or held-state updates; selected peaks match the full-statistics path under the same sanitization rules; requesting energy/RMS retains its existing numerical results. Verify work avoidance with focused tests or instrumentation outside production render, plus disabled/peak-only/full descriptive timings. This slice is part of the requested implementation plan, not contingent on the historical overhead recurring.
2. **Publication/poll efficiency.** Use explicit completed-window readiness or a prepared cadence to avoid empty scans while still observing every required sample. Decouple measurement window from consumer read frequency. Preserve sequence/loss visibility for slow native and browser readers. No new network protocol or SDK subscription API. Acceptance: frozen fixtures show fewer empty poll scans for long windows without changing interval values, losing early peaks, or hiding queue loss; report poll/drain counts and descriptive callback cost with observation cadence unchanged.
3. **Measured kernel optimization, only if warranted.** Profile existing scans, f64 recurrence, hold logic, queue traffic, master scan and callback polling separately. Consider safe SIMD or scan reuse only with evidence and independent numerical validation. Do not prescribe fusion, reduction reassociation or replacement storage upfront. Extended target matrices and benchmark infrastructure repair/promotion are separate tooling work.

## Descriptive performance evidence and delivery

For each implementation slice, freeze the workload and validator before timing. Use existing console/meter fixtures with active audio and fixed tracks, quantum, rate, window and queue configuration; include disabled, current/full and applicable changed paths. Preflight argument validation, schema, output persistence, shell exit status and overwrite refusal without the timed workload. Run exactly one invocation with one warmup and two measured rounds; no tuning or retries. Report workload, compiler/target, elapsed/cycle measurement, raw output and limitations. If a comparable baseline is needed, include both revisions within that frozen invocation. Historical #203 numbers are context only; do not promise a percentage gain. A runner failure preserves raw output and creates tooling follow-up rather than blocking the product slice.

Sol approves the reconciled brief and gates; Terra implements attempt one; Sol adversarially reviews. Follow the three-attempt stop. Commit coherent exact-path checkpoints; push according to the active delivery mode. Close the smallest slice when its correctness gates pass and evidence is upstream and GitHub synchronized; successor optimization or expanded qualification must not hold it open. Record decisions, actual commands/results, revision and review verdict here. No implementation or performance result is claimed by this plan.

## Implementation attempt 1 — correctness slice

Frozen adapter policy: bounded per-window delivery. Each browser poll publishes at most one
complete track window, leaving later bounded queue entries for later polls. Track snapshots are
matched by expected handle, reset generation, sequence, half-open sample span and frame count;
stale, mismatched, noncontiguous or pre-lease candidates are rejected and counted. This avoids
unioning a last-only peak with a wider timestamp envelope.

The 64-byte `WebMeterHeader` remains ABI-compatible. `reserved[0]` is the host publication
generation, incremented on real lease transitions. In `reserved[1]`, bits 0..3 are complete,
master-aligned, loss-observed and gain-reduction-present; bits 32..63 carry a saturating loss
count. The legacy `f32` peak payload is unchanged. Master output peaks use a fixed-capacity
interval ring sized from the prepared meter queue, split at the same period as track snapshots,
so an early impulse survives delayed polling. Empty polls leave the last frame, header and pending
master interval unchanged. Lease reacquisition drains bounded state and activates at the next
meter boundary; the straddling interval is discarded. Gain reduction retains its own effect-tap
fold and unit conversion and never supplies track/master timestamps.

Changed paths: `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs`,
`hosts/host-web/src/tests.rs`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, and
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`. The fixed pending/master storage is
charged to `WebResourceReport.bridge_metadata_bytes` and `bridge_retained_bytes` before the exact
budget gate. The worklet reads the metadata and refuses to post a frame unless complete and
master-aligned bits are set.

Evidence run from base `5206e187`:

```text
cargo check -p host-web                         # PASS
cargo test -p host-web --lib meter_             # PASS: 6 passed, 61 filtered out
cargo test -p host-web --lib                    # PASS: 63 passed, 1 ignored
```

The focused fixtures cover an early impulse surviving an empty poll, one-window-at-a-time
delivery from multiple queued windows, lease reacquisition at a clean boundary, existing PCM
bit identity, and existing gain-reduction behavior. Native host tests ran on the current host.
Wasm/browser target qualification, allocation-symbol inspection of the rebuilt artifact, and
four-rate target execution remain unavailable in this tranche and are not claimed as performed.
Selective metric requests, poll-scan reduction and measured kernel work remain separate successor
issues as planned.

## Attempt 1 adversarial review

Dedicated Sol review at committed revision `d90bf386` recorded **FAIL**. A poll could consume and
write a coherent track candidate before validating the corresponding master interval, then return
zero with the previous header but changed public peak bytes. The stored meter-producer generation
was never compared with later candidates, so a restarted producer sequence could be published
under the previous host epoch. Public TypeScript documentation also applied the peak interval too
broadly to independently aged gain-reduction folds and promised gapless delivery despite explicit
loss handling. Shipped Wasm callback/browser qualification had stopped honestly at the expected
old artifact digest; it was not claimed as passing. The review also found that the largest bridge
allocation row used the sum of two separately allocated meter buffers rather than their maximum.

## Sol correction attempt 2 — correctness slice

The poll now validates handle/generation/sequence/span/frame identity and the exact master interval
before entering a single public-frame commit section. Every zero-return path leaves all published
frame and header bytes intact. Asymmetric loss consumes only hidden bounded candidate state, records
loss, and later recovers at the next exact master/track span.

The first accepted producer reset generation is retained and compared on every later candidate. An
unexpected change rejects the group under the old public identity, increments the internal delivery
epoch, clears pending/master partial state, and starts at the next clean meter boundary. Producer
queue entries become stale at that boundary and the existing capacity-budgeted poll rejects them;
the new epoch becomes public only with its first successful frame. Lease transitions retain the same
deferred bounded-rejection rule. Source seek is explicitly a content-position
discontinuity on a continuous absolute render clock: it neither resets the meter accumulator nor
advances the meter epoch, so a peak window may straddle it. This web host exposes no in-place plan
replacement or full render reset; those operations create a fresh host and therefore have no queued
or published meter state to inherit.

Gain-reduction documentation now states that each slot is a latest, independently aged effect fold;
its sequence and sample span are not transported and the track/master peak timestamps never apply to
it. Peak-frame documentation treats generation changes and sample gaps as delivery discontinuities
and uses the validity/loss metadata rather than promising gapless windows.

The pending-track and master-ring allocation sizes are still both charged to bridge metadata and
retained totals before the exact budget gate. Largest-allocation reporting now uses the larger
individual allocation, carries that value into `largest_named_allocation_bytes`, and prepares zero
master-ring entries when meters are unattached.

Correction evidence from the dirty candidate based on `d90bf386`:

```text
cargo check -p host-web                         # PASS
cargo test -p host-web --lib meter_             # PASS: 11 passed, 61 filtered out
cargo test -p host-web --lib                    # PASS: 71 passed, 1 ignored
bash scripts/check-realtime-policy.sh            # PASS: 42 marked regions in 12 files
cargo fmt --all -- --check                       # PASS
git diff --check                                 # PASS
```

The added executable probes cover transactional zero-return publication and recovery, asymmetric
master loss, producer reset and fresh epoch recovery, queue saturation with explicit loss, delayed
polling with a partial trailing period, lease reactivation, continuous-clock source seek, all four
launch sample rates, and a nine-track SIMD-width tail. Peak assertions include both master slots.
The existing PCM-identity and gain-reduction tests remain green. Shipped Wasm artifact adoption,
allocation/callgraph inspection, resource-fixture refresh and browser execution remain pending a
separate Astra ruling over the frozen corrected source and are not claimed here.

## Attempt 2 qualification follow-up

The correction was checkpointed at `32db5599`. Dedicated Astra source review returned **PASS** and
approved adoption of the observed corrected Wasm digest
`6cb9675bdd6ccb776f59fa44b48c6b9f870af5d71a79277378fe335167878bdf`; the ordinary artifact builder
then passed and the reviewed pin was adopted at `fb9f0d6c`. Qualification subsequently found two
real integration blockers, so that artifact is preserved as failed evidence rather than treated as
qualified.

First, the unchanged static callgraph gate found callback-reachable `panic_bounds_check` owned by
`ReadyOwnership::reset_meter_delivery`. Replacing public-frame `fill` with an iterator did not alter
the finding. Inspection of rebuilt Wasm function 61 localized the sole panic call at byte `0x00b73a`
inside the helper's inlined `MeterConsumer::try_pop`: it compares the queue read index with capacity
before loading a snapshot slot. The Option-array and public-frame clears compile as direct stores or
`memory.fill` and own no such call. The final bounded correction removes eager producer-queue drains
from lifecycle reset. Reset still clears host pending/master state and sets the clean activation
boundary; the existing capacity-budgeted poll rejects older queued entries before accepting a new
interval. A saturated reacquisition fixture proves the first fresh producer window may be dropped
while stale slots remain full, the bounded rejection poll publishes nothing and preserves every
public byte, and a later exact interval recovers in the new generation with explicit loss. This Rust
change requires a new observed artifact digest and a fresh Astra adoption ruling; no pin, queue API
or gate is changed here.

Second, the hermetic fake header omitted the new complete/master validity bits, but inspection also
found that the production main-realm host's exact-field validator still accepted only the old meter
message shape. A real worklet frame carrying `generation`, `validity` and `lossCount` would therefore
have caused sticky result 255. The validator now requires the exact new field set, a positive BigInt
generation, bounded numeric validity/loss fields, complete+master bits, no unknown validity bits,
and consistency between the loss bit and count. It also enforces the already documented nonempty
half-open peak span. Hermetic fixtures exercise valid metadata, loss transport, missing validity and
malformed metadata domains through both the processor and real host receiver. The generated SDK
declaration mirror is refreshed byte-for-byte from the shipped host declaration.

Follow-up evidence on the dirty candidate after `fb9f0d6c`:

```text
node scripts/test-web-audioworklet.mjs           # PASS
cargo test -p host-web --lib meter_              # PASS: 12 passed, 61 filtered out
bash scripts/check-sdk-generated.sh              # PASS
bash scripts/check-realtime-policy.sh             # PASS: 42 marked regions in 12 files
cargo fmt --all -- --check                        # PASS
git diff --check                                  # PASS
```

The full hermetic wrapper passed its Node tests, allocator mutation, opcode mutations and callgraph
analyser self-tests, then its browser-correctness self-test could not bind a localhost socket in the
restricted environment (`PermissionError: Operation not permitted`). That environment result is not
a browser qualification claim. A rebuilt artifact static gate must verify removal of the named trap
owner before the next review verdict.

## Final correctness-slice qualification

Frozen candidate `7c7001e7885fcbbe176c7bec82cb36b579db527b` produces Wasm SHA-256 `bc73d5205cab8e8e95dde2360198f1614d6ca2e3939afaf0df59d1b50c4def20`. The ordinary builder, unchanged static/object/callback checks, metadata checks and boot memory-budget gate all passed. No callgraph allowlist, realtime gate, PCM expectation or resource-budget gate was weakened. Earlier candidates that failed static qualification remain recorded above; their browser passes do not substitute for this candidate's results.

Final evidence: `cargo test -p host-web --lib` passed 72 tests with 1 ignored; focused `meter_` passed 12; source realtime policy passed 42 marked regions in 12 files; formatting and diff checks passed. The hermetic worklet wrapper, including malformed-metadata and mutation checks, passed with localhost permission needed by its existing self-test. The existing Playwright suite against this exact artifact passed Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, including mutation proofs. `qualification/results.json` and `BROWSER_DEPLOYMENT_MATRIX.md` contain the actual candidate lineage; matrix consistency passed.

This checkpoint delivers the first coherent measurement-publication slice, not selective metric computation or a measured performance gain. No descriptive timing benchmark was executed during this correctness correction; the planned efficiency slices must establish their own frozen baseline and work-avoidance evidence before quoting cost improvements. The unrelated builtin tangent-oracle failure reported earlier was not attributed by an independent baseline comparison and is not claimed fixed.


Final independent Astra medium verdict: PASS for the correctness slice after Sol medium corrections. Final delivery also corrects a stale three-line host-core documentation claim about shared GR/peak spans; no executable source changed after the qualified candidate. Performance follow-ups remain open work.

## Planning review

The original plan was drafted by Astra low and adversarially reviewed by a separate Astra medium agent. That planning review passed after revisions; the implementation verdict and actual qualification results are recorded separately above.
