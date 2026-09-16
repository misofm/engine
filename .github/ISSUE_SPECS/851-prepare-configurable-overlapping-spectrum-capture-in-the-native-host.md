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

**N3 bounded loss recovery and smoothing — complete in Luna round 1.** The consumer now freezes
the one-slot population at read entry and arms exactly one bounded recovery opportunity after
reporting a Gap. New drops arriving before that recovery cannot starve the retained valid record;
the following read reports the newer cumulative loss, while the immutable record keeps its
original loss metadata. Failure/lifecycle reset clears the recovery state, bounded stale-record
rules remain intact, and no refill is chased. A deterministic repeated-drop fixture proves three
successive Gap→intervening-drop→record recoveries and exact cumulative counts. An independent
power recurrence proves `exp(-H/(Fs*tau))` smoothing for H=256/512/1024/2048; existing load-bearing
tests retain zero-smoothing equivalence and reset-on-loss/epoch/invalid/configuration coverage.
Root reran both new focused tests, strict all-target Clippy, formatting and diff checks: PASS.
Implementer reports spectrum unit 45, spectrum integration 9, protected-owner 23 and the complete
host-core all-feature suite PASS. N4 remains unstarted.

**N4 protected native preparation — candidate complete.** Fresh Luna MAX added one additive
`HostObservationPreparationConfig` wrapper whose sole optional setting is a validated `SpectrumHop`;
the two existing execution modes each have one config entry point, while all old constructors keep
the derived default. One immutable effective cadence is stored by the protected owner and reused by
start, replacement and restart; its checked H drives admission work. Resource accounting continues
to derive actual capture/owner layouts, with shared constants replacing the prior paired-slot and
five-selection magic numbers. Integration gates exercise every hop through prepare→admit→apply→
render→read, exact spans and identities, target replacement/restart, exact and one-below work and
paired-storage budgets with unchanged refused state, and PCM bit identity. Root review rejected
duplicate convenience spellings after round 1; Luna round 2 reduced the API to one wrapper builder,
two required execution-mode functions, one cadence accessor and the `SpectrumHop` re-export.
Focused explicit-hop tests and strict Clippy pass under root. Implementer reports locked complete
host-core all-feature/no-default suites, doctests, wasm32 host-core check, host-web all-feature check,
formatting and diff checks PASS. Fresh Astra medium native review remains required before delivery.

**Astra medium integrated native verdict at `f11e710a`: FAIL, one blocker.** Cadence, window
scheduling, queue ownership/loss, validity/failure fencing, smoothing, lifecycle, retained storage,
PCM, realtime policy and portability passed. Copy-work admission did not: the projection charged
`C*Q + attempts*4*N`, while each completion reconstructs selected history into persistent buffers
(`C*N`), copies both buffers into the owned record (`2*N`), then moves both planes into queue
storage (`2*N`). Stereo Q128/H256 can therefore perform 12,544 logical sample copies under an
8,448 admitted bound. Existing probes omitted reconstruction writes and repeated the low formula.
N1 and N2 exhausted their two Luna rounds, so the bounded correction escalates per user direction
to one Sol high round: charge the actual conservative term (or eliminate a proved copy), add an
independent reconstruction/queue discriminator, and update exact/one-below admission gates. A
fresh Astra medium rereview is required afterward; browser issue #852 remains unstarted.

**Sol high copy-accounting correction — candidate complete.** The checked projection now charges
`C*Q + ceil(Q/H)*(C*N + 4*N)`: circular-history writes, selected-plane chronological
reconstruction, construction of the dual-plane owned record, and transfer of that record into the
one-slot queue, including a rejected full-queue attempt. Runtime-only probes independently count
the three completion copy stages rather than deriving them from the projection. The stereo
Q128/H256 discriminator observes exactly 12,544 sample-copy operations; protected admission
accepts that exact bound and transactionally refuses 12,543 with the public copy-work limit name.
Root reran the complete host-core all-feature suite and doctests, strict all-feature/all-target
Clippy, the realtime policy gate, formatting, and the wasm32 no-default host-core check: PASS. A
fresh Astra medium rereview of this exact checkpoint remains required before native delivery.

**Fresh Astra medium integrated native verdict at `e6a60c02`: PASS.** The reviewer found no
remaining blockers. It independently verified that
`C*Q + ceil(Q/H)*(C*N + 4*N)` conservatively covers every selected-plane history/reconstruction,
dual-plane owned-record, and queue-transfer copy, including repeated completions and rejected
full-queue attempts. The Q128/H256 stereo probe measures 12,544 operations and exact/one-below
admission accepts 12,544 and transactionally refuses 12,543. Cadence, exact windows, immutable
ownership, bounded Gap recovery, validity/failure fencing, smoothing, preparation/lifecycle,
retained resources, PCM identity, realtime rules, additive compatibility, host-core default and
no-default suites, wasm32 scalar/SIMD checks, and host-web compilation all passed. The only notes
were two existing nightly-only ignored budget tests and an existing no-default internal-helper
warning; neither affects delivery.

**Delivery artifact reconciliation.** PR #853's first qualification run reached the intended
shipped-artifact guard and refused the stale AudioWorklet digest: the linked host-core/host-web
Wasm necessarily changed with this native implementation. A pinned Rust 1.97.1 reproducible build
produced `082e04b609278e016bfa194358952600321552e3322e6a549ef51cf974062034` twice, once through the
ordinary refusing path and once through the explicit print-only repin path. The checked-in digest
now names those exact source bytes; no ABI record, JavaScript asset, package, or deployment state
changed. Qualification must rerun and pass before merge.
