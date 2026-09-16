# Prepare configurable overlapping spectrum capture in the native host

Parent: #763. Browser/headless successor: #852.

## Product slice

The protected native observation owner accepts one preparation-time spectrum hop and produces exact overlapping 2048-frame spectrum windows without changing PCM, allocating on render, or weakening bounded loss reporting. Browser transport, SDK configuration, artifact promotion, deployment defaults, package release, app adoption, larger queues, alternative smoothing, and simultaneous spectrum jobs are outside this issue.

Configuration is host observation policy, not canonical musical session JSON and not an environment read inside the engine. One prepared host uses one immutable effective cadence shared by its prepared spectrum targets. Omitted configuration retains the existing default `SpectrumCadence::new(Fs,Q)` profile. Explicit hops are exactly 256, 512, 1024, or 2048 frames; reject every other explicit value. Do not require hop divisibility by render quantum and do not silently round.

For activation sample `S`, explicit overlapping windows are exactly `[S+kH, S+kH+2048)`. The first result appears only after 2048 valid samples. A render block may cross multiple completion boundaries. Preserve the existing one-slot completed-record queue and owned `[f32; 2048]` record representation: private circular history never becomes queued borrowed storage. Every scheduled completion advances sequence; a full queue drops the new completed record, increments the cumulative loss count, and never changes the retained queued record.

Current source assumptions that must be removed deliberately:

- `project_spectrum_work` assumes at most one publication per block and derives traffic without effective `H`.
- continuous capture resets `filled` after a non-overlapping window and waits for a future block-aligned start.
- `try_read_continuous_record` can report newly changed drops before every pop, so continuing loss can starve a valid queued record.
- one scalar underrun flag cannot describe every overlapping window that contains an affected sample.

## N1: cadence and checked work projection

Own `crates/host-core/src/spectrum.rs` cadence/configuration definitions, `crates/host-core/src/observation_demand.rs` projection, focused unit tests, and only mechanical exhaustive error matches needed to compile.

- Add a validated explicit-hop representation/constructor while keeping the old default constructor behavior, including default hops above 2048 at higher rates.
- Use effective `H` in checked work projection. Bound completion/publication attempts by `ceil(Q/H)` and payload rate conservatively from `Fs/H`.
- Account for selected input validation, history writes, chronological owned-record construction, queue copies/attempts even when full, and checked arithmetic. Reconcile exact copy/storage terms with N2 before exposure.

Gates cover all four launch rates, all four explicit hops, default regression, invalid values, checked overflow, `Q={128,192,768,4096}`, nondividing boundaries, and increased projected work as H falls. This checkpoint does not expose active overlap.

## N2: circular-history producer

Depends on N1. Own continuous capture state/producer paths in `crates/host-core/src/spectrum.rs`, their unit tests, and narrowly necessary projection/resource corrections resulting from the actual storage/copy design.

- Preallocate private dual-plane 2048-frame history and process the entire observed block for planar and resident AoSoA paths, including wrap and several completion boundaries.
- Construct every publication as an immutable chronological owned record before attempting the existing queue.
- Preserve existing default-profile spacing when `H >= 2048`.
- Carry sample validity so every overlapping window containing an underrun is marked, then clears when affected samples leave the span.
- Discontinuity, source-generation change, nonfinite selected input, failed render, and checked timestamp/sequence overflow invalidate partial history and fence the epoch; no completed record from a failed block escapes.

Gates use independent ramp/impulse sample oracles across wrap for every H; Q192 nonaligned completions; Q768/H256 and Q4096/H256 multiple completions; left/right/stereo and planar/resident equality; a held queued record remaining byte-identical during later wrap/drop; validity propagation/clear; failure after multiple in-block completions; existing one-shot/default behavior; and render allocation/operation counters.

## N3: bounded loss recovery and smoothing

Depends on N2. Own consumer bookkeeping/read methods and analyzer tests in `crates/host-core/src/spectrum.rs`, plus directly relevant protected-owner read tests.

- Preserve explicit cumulative `Gap` reporting while guaranteeing that continuing new drops cannot starve a retained valid current-generation record. Freeze the queue population at read entry and never chase refills. After one Gap, the next read gets one bounded recovery opportunity before another drop-only Gap; intervening failure/epoch invalidation remains higher priority.
- Preserve truthful record metadata: an old immutable record does not acquire later loss metadata.
- Use actual H in existing `exp(-H/(Fs*tau))` power smoothing. Preserve reset on sequence loss, epoch/configuration change, and invalid history.

Gates deterministically add drops between Gap and recovery reads and prove bounded delivery, cumulative loss, no duplicate loss, bounded stale-generation rejection, independent smoothing recurrence for every explicit H, zero-smoothing equivalence, and reset semantics.

## N4: protected preparation and native closure

Depends on N1-N3. Own host-core preparation/accessor/export glue and focused host-core integration tests. Use additive native configuration/wrappers rather than breaking existing constructors.

- Thread requested H through protected preparation and retain one immutable effective cadence per prepared owner. Start, replacement, selection, and restart use it.
- Ordinary single/collection wrappers keep default semantics.
- Reconcile retained/largest-allocation accounting for circular history, bookkeeping, paired controlled slots, and preparation metadata. Budget refusal is transactional.

Gates cover protected prepare/admit/apply/render/read spans and metadata, replacement/restart identity fences, exact and one-below work/storage limits, unchanged state on refusal, observed/unobserved PCM bit identity, default/one-shot/owner lifecycle regression, realtime allocation/policy gates, and native/scalar/wasm32-simd compilation applicable to host-core.

## Execution and delivery

Assign N1-N4 sequentially to one fresh Luna MAX agent each. An agent gets only this contract, its task, exact prior checkpoint, and focused commands. Root audits and commits/pushes each coherent checkpoint before the next task starts. Each Luna task has at most two implementation/revision rounds; if still unsatisfactory, escalate to Sol high for one round, then Astra xhigh for one round, then stop and rescope. Record one verdict per round and never weaken gates.

After N4, a fresh Astra medium agent adversarially verifies the exact integrated native candidate. Push PASS evidence, synchronize this body, close the issue, and verify remote closure before browser/SDK implementation begins. Keep #763 open.

## Checkpoint evidence

**N1 cadence and work projection — complete.** Fresh Luna MAX added the validated four-hop
vocabulary and one explicit cadence constructor while preserving the default profile, including
high-rate/default hops above 2048 and nondividing quantums. The work projection now uses effective
H for checked `ceil(Q/H)` completion attempts and `ceil(Fs/H)+1` payload traffic, and charges
selected history writes plus conservative owned-record/publication copies. No active overlap path
is exposed. Root's first strict all-target Clippy run found five `manual_div_ceil` violations in the
new tests; Luna corrected them in its second and final round and removed duplicate constructor/
accessor spellings. Root reran the five focused projection tests, explicit-hop validation, and
strict host-core all-feature/all-target Clippy successfully. Implementer also reports the complete
host-core all-feature suite/doctests, host-web all-feature check, formatting and diff checks PASS.
Checkpoint source follows the pushed planning baseline `2313a93d`; N2 remains unstarted.

**N2 circular-history producer — complete.** Fresh Luna MAX replaced the non-overlapping fill/reset
state with private preallocated dual-plane circular history and per-sample validity. It emits exact
owned `[S+kH,S+kH+2048)` records at arbitrary block boundaries, handles several completions per
block, advances sequence/drop facts without overwriting the one queued record, and preserves the
existing default spaced profile. Planar ramp/impulse oracles cover every explicit hop and channel
mask across wrap; Q192, Q768/H256 and Q4096/H256 cover nonaligned and multiple completions.
Underrun membership/clear, discontinuity/nonfinite/failure fencing, held-record immutability,
allocation freedom and operation attempts are exercised. A real resident-bank integration compares
every continuous sample against an independent one-shot target oracle at Q192 and Q4096 and proves
PCM identity. Luna round 1 left that integration's old `2*N` storage-write expectation failing;
round 2 corrected it to the actual `2*blocks*Q` history work and strengthened the payload oracle.
Root reran the complete locked host-core all-feature suite/doctests and strict all-target Clippy:
PASS. Implementer also reports release spectrum unit 38/38, spectrum integration 9/9, formatting
and diff checks PASS. N3 remains unstarted.
