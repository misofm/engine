# Remove multiband-compressor lookahead while preserving its crossover

## Scope status and delivery boundary

Final stateless scope, revalidated by Astra XHIGH against clean primary checkout and
`origin/main` at `17d755e90217f12fb3b8324d0b77bf560d8b35f7`, after #738 merged through
PR #747. The multiband crate, independent LR4 reference, shared envelope, Lane and
effect-contract sources are unchanged from the prior read-only base `d2aeb62d`.
The handoff-branch #739 body matched the proposed scope; this final scope supersedes it.

#739 implementation starts only after root verifies #738 closure and required delivery,
reconciles local/remote issue state, installs this numbered scope and records activation.
Luna XHIGH implements; Sol XHIGH supplies one adversarial verdict per coherent attempt,
maximum five attempts. Root owns commits, pushes, pins, GitHub, CI and cleanup. A changed
DSP/contract base before activation requires a focused delta revalidation, not a restart
of unchanged research or qualification.

The owner has authorized removing utility-effect lookahead. The smallest closable slice is
the existing two-band multiband compressor with current-sample detection and gain, exact
zero added sample delay, unchanged crossover/gain arithmetic and retained banking, plus
its necessary state, compatibility, graph and representative quality evidence. Historical
issues #018/#051/#094/#149/#537 describe earlier contracts; their immutable evidence stays
historical. This issue supersedes their lookahead/ring requirements only.

## Algorithm ruling

**PASS: a direct causal expression exists without changing the crossover.** Current
`lr4_step` already produces the two required band samples from the current input. Per
channel and track, preserve this exact order:

1. `(v1, lp1) = svf_step(x[n])` using the first existing Butterworth-Q TPT section.
2. `ap = nk2.fma(v1, x[n])`, with existing `nk2 = -2 * rounded_f32(sqrt(2))`.
3. `(_, low) = svf_step(lp1)` using the second identical-design TPT section.
4. `high = ap.sub(low)`.
5. Link the current corresponding low-band L/R samples, and separately the current
   corresponding high-band L/R samples; compute each band's current gain.
6. `y[n] = low.mul(gain_low[n]).add(high.mul(gain_high[n]))` in that order.

Remove the four ring writes, detector gathers and delayed output loads. Do not replace
`lr4_step`, change coefficient design, reassociate arithmetic, invert a band or insert a
new dry/sum switch. The old `Fs/50` audio delay and per-channel lookahead tap are the
removed behavior; no audio/detector ring, cursor, tap-offset array, uniform/ragged
classification or replacement quantum buffer remains.

The existing identity is `LP2^2 + HP2^2 = D(-s)/D(s)` for
`D(s) = s^2 + sqrt(2)*s + 1`, preserved by the bilinear transform. Thus the enabled
unity-band output is the current LR4 all-pass sum, within the established f32 error bound.
It is not a dry-bit identity and has frequency-dependent phase/group delay. Exact reported
zero means zero added integer-sample processing delay. The product has no mix/dry-wet
parameter. Its prepare-time bypass returns current dry bits, including signed zero, while
advancing parameter ramps and skipping crossover/dynamics exactly as presently intended.

This ruling does not authorize a different crossover, new phase/reconstruction guarantee,
changed antialiasing, limiter/clipper work or a new gain law. If any becomes necessary,
stop that shape for an owner algorithm ruling; do not hide it as ring removal.

## Frozen public contract and controls

Keep `miso.multiband-compressor`, contract pair `1.1`, live state-layout label `1`, current
V1 wire/ABI identities, main-in/main-out dual-mono ports, no external sidechain,
DualMono/Maximum/Average links, and Normal quality at exactly 44.1/48/88.2/96 kHz.
Every scalar/bank quality reports `LatencySamples(0)` enabled or bypassed, with the existing
conservative `TailSamples::Infinite`. Tail and causal IIR/envelope memory are not audio delay.

Retire stable parameter ID 2 permanently; do not reuse it. Keep all other IDs, names, units,
domains, defaults, mappings, channel policy and lattice descriptors. The compact descriptor
indices become 0..10, so preparation requires exactly 22 ordered L/R initial records.
Never mistake these compact internal indices for stable wire IDs.

| ID | Index | Control | Unit | Inclusive domain | Default | Mapping | Automation/smoothing |
|---:|---:|---|---|---:|---:|---|---|
| 1 | 0 | crossover | Hz | 80..8000 | 1000 | logarithmic | None/None |
| 3 | 1 | low_threshold | dB | -80..0 | -18 | linear | Block Point/Linear 64 |
| 4 | 2 | low_ratio | ratio | 1..20 | 4 | logarithmic | Block Point/Linear 64 |
| 5 | 3 | low_attack | ms | 0.1..200 | 10 | logarithmic | Block Point/Linear 64 |
| 6 | 4 | low_release | ms | 5..5000 | 100 | logarithmic | Block Point/Linear 64 |
| 7 | 5 | low_makeup | dB | -24..24 | 0 | linear | Block Point/Linear 64 |
| 8 | 6 | high_threshold | dB | -80..0 | -18 | linear | Block Point/Linear 64 |
| 9 | 7 | high_ratio | ratio | 1..20 | 4 | logarithmic | Block Point/Linear 64 |
| 10 | 8 | high_attack | ms | 0.1..200 | 10 | logarithmic | Block Point/Linear 64 |
| 11 | 9 | high_release | ms | 5..5000 | 100 | logarithmic | Block Point/Linear 64 |
| 12 | 10 | high_makeup | dB | -24..24 | 0 | linear | Block Point/Linear 64 |

All controls remain readable continuous PerLane controls; only the ten band controls are
automatable. Keep the one resident per-block Gain Reduction observation, ID 1, exposing
the deeper of the two negative-dB smoother words per channel, with existing descriptor
fold/range/unit semantics. Preserve W1/W4/W8 one-body arithmetic and all rack eligibility.
The current bank does not implement mono collapse: preserve that opt-out and ordinary
equal-input dual-mono behavior. No new mono specialization or generic copy framework.

Keep strict bank request shape and all-member validation before fallback. A backend/width
mismatch or wrong member count remains `effect.bank.requests`; malformed member data
remains an error even when another member makes the cohort unbankable. Preserve valid
mixed-program fallback. Current multiband admits matching backend/width pairs directly,
unlike gate's additional current-backend check: do not import gate's admission policy or
loosen admission to make tests pass. Native engine dispatch remains compile-time W8 on
x86-64-v3; W4 internal/property or explicitly bound fixture coverage is not evidence of
production native W4 dispatch. Supported Wasm simd128 uses W4.

## Retained numerical and update law

Crossover design stays in f64 through the existing `math::tan`: `g=tan(pi*fc/Fs)`,
`k=sqrt(2)`, `t=g*(g+k)`, `c1=t/(1+t)`, `a2=g*(1-c1)`, `a3=g*a2`, rounded once
as currently implemented. Preserve the domain, normal/zero checks, positive a2/a3,
`0<=c1<1`, recovered-g consistency and 0.005 dB half-power self-check. Preserve all
four recursive filter words and current `lane::kernels::svf_step` arithmetic/flush.

Links operate on absolute current band samples: DualMono keeps separate magnitudes;
Maximum shares the existing ordered-select max; Average uses `0.5*l + 0.5*r` in that
order. Independent L/R crossover frequencies remain legal; do not silently link filters.

Per band retain `fast_level_db(max(magnitude,1e-8))` clamped to [-160,24] dB, the
Giannoulis/Massberg/Reiss soft-knee curve with fixed 6 dB knee, and target clamp [-100,0]
dB. With `d=level-threshold`, mathematical reduction is zero below the knee,
`(1/ratio-1)*d` above it, and `(1/ratio-1)*(d+3)^2/12` inside. Keep existing
`gain_delta_db` and fast-dB operation order and sealed annotations.

The multiband smoother specifically uses the existing f32
`retention_coefficient(time_ms,Fs) = expf(-1/(time_ms*0.001*Fs))` clamped [0,1].
This is not #737's f64-designed incremental coefficient. `BandCache::refresh` retains
its current ratio/attack/release bit keys and segment-start refresh timing. Select attack
strictly when target < prior reduction; equality uses release. Preserve
`G=flush(c.fma(Gprev-target,target))` and `fast_gain_from_db(G+makeup)`.
The crate's current Lane fma contract is unfused; do not take stale "one rounding" prose
as permission to change it. Keep ten four-word 64-update ramps, positive-zero
normalization, exact endpoint snap and flat/ramping segmentation. This issue does not
add a stronger arbitrary time-constant-ramp partition guarantee than the current engine.

Retain the shared once-per-block output bound/nonfinite policy, counters and whole-bank
recovery; remove only ring-specific clearing. The new current-sample fault must be observed
in its actual block. Preserve the six recursive filter/gain flushes, normal/zero state
validation, and current parameter-domain rejection/normalization. Do not insert per-sample
finite checks or redesign fault attribution.

## State, resources and compatibility

Empty common section. Each channel has exactly 47 little-endian words (188 bytes):

- word 0: crossover Hz;
- words 1,2: low/high gain reduction;
- words `3+4*i` through `6+4*i`, `i=0..9`: ramp current, target, step, remaining,
  in low-then-high control order;
- words 43..46: first section ic1/ic2, second section ic1/ic2.

Total payload is 376 bytes at every launch rate; scratch remains exactly zero. Remove
lookahead values, tap offsets, rings and cursor from runtime, staged restore and snapshot.
Keep crossover design/cache data as existing bounded in-memory derived state; it is not
part of the saved payload. Remaining storage is O(width), independent of Fs, quantum and
source duration apart from established outer control/bank allocations. Distinguish payload
bytes from total prepared-object/host-buffer memory; do not call 376 total heap usage.

Retain validate-both-channels-before-commit restore, gain/filter/ramp invariants, exact
current snapshot continuation, scalar/bank-track interchange, default reset and
discontinuity target-snap behavior. An old raw payload rejects by exact lengths in scalar
and bank hooks; the old per-channel lengths were 7256/7880/14312/15560 at the four rates.
Do not truncate/reinterpret it. Old descriptor-bound Effect State V1 envelopes reject
transactionally under the changed descriptor digest. Keeping V1 labels does not make this
prelaunch descriptor/payload change compatible. No migration edge is legal under the
current identical-parameters/latency migration rule; do not weaken that rule.

Session/control requests explicitly naming stable ID 2 reject with existing typed
unknown-parameter/invalid-address behavior, without aliasing another compact index,
silently dropping data, acknowledging it, or advancing revision. Current valid sessions
still round-trip canonically. Old sessions omitting ID 2 cannot be distinguished from new
default sessions by the current effect-ID-only selector; document that they resolve to
the amended causal product, not that all historical sessions can be detected/rejected.

**Root-approved bounded reset amendment.** Because this issue rewrites the same saved-state
and restore boundary while retaining reset semantics, add one discriminator: restore state
prepared at a different crossover, call `FullToDefaults`, and compare PCM/state with a
fresh instance using the receiver's original prepared defaults. If that proves the inherited
coefficient/default mismatch identified below, repair it by retaining or rederiving the
prepared-default coefficient set within the existing local control/reset design. Record
the inherited failure and correction explicitly. No generic reset framework or other
effect's reset changes; keep reset allocation-free and maintain its existing bounded work.
Prefer retaining the prepared-default coefficient set so reset preserves the current
no-redesign implementation rule; this derived in-memory cache does not enlarge the payload.

## Allowed edits and identified stale assumptions

Luna owns `crates/multiband-compressor/src/{lib,split,shim,corpus}.rs` and its existing
tests/support. `shim`/`corpus` changes should be explanatory only; the six existing
primitive corpus cases should retain their digest bytes. Reuse
`crates/dsp-reference/src/lr4.rs` and the existing independent mapping/response tests;
neither reference filter redesign nor new production reference API is needed. A small
test-local f64 current-band dynamics composition is permitted for the mandatory oracle.

Necessary descendants are multiband-related portions of
`crates/graph-compiler/src/lib.rs`, `crates/effect-compiler/tests/native_session.rs`,
`crates/effect-package/tests/descriptor_v1_qualification.rs`, and affected current
descriptor/metadata assertions. `effect-compiler/tests/observation_identity.rs` already
contains misleading general historical-compatibility prose; scope only a necessary
clarification, no observation redesign. Root owns generated
`sdk/assets/miso-engine-v1-parameter-metadata.json`, its generated
`sdk/src/generated/catalog.ts`, the minimal corresponding negative/positive compile
assertions in `sdk/test/console-types.ts`, current Wasm/browser artifacts and their
legitimate derived pins. The current-base JSON inventory finds no multiband session
fixture requiring parameter removal.

**Exclude `scripts/fixtures/parameter-metadata-v1-self-test.json` from blanket refresh.**
It is a synthetic positive document for the schema validator, not a live registry mirror;
its allowed `stateLayoutVersion>=1` and old control examples are deliberate independent
inputs. Removing live lookahead does not invalidate it. Only a demonstrated
validator-specific failure would warrant a separately explained narrow edit. Shipped
SDK metadata/catalog must always be regenerated from the current descriptor.

Preserve existing test bodies/assertions and make surgical index, payload and time-index
adaptations. Do not replace whole product/identity/fault suites with a smaller new suite.
Remove only obsolete tap-access mechanism tests and duplicate offset-profile permutations;
carry their still-relevant heterogeneous-lane, populated-state and restore assertions into
the existing general tests. Add only the compact causal oracle/reset discriminators and
thin descendant compatibility/PDC checks below. Reuse the now-existing causal gate and
compressor descendant patterns locally; do not factor a new shared harness.

Address these source/test assumptions explicitly, not through blanket numeric replacement:

- `lib.rs` module/render/codec/resource comments claiming Fs/50, ring chronology or
  lookahead; delete `detector_access_tests` and private #537 instrumentation once their
  removed mechanism has no caller. Retain public DSP/identity tests.
- `tests/product.rs`: 12 parameters, 24 initial records, old resource rows/offsets,
  delayed-unity reference indexing and signed-zero bypass at sample 960.
- `tests/nonfinite.rs`: its claim that bad input arrives 960 samples later is false
  after this change; assert immediate block recovery instead of merely waiting long enough.
- `tests/{identity,no_alloc_render}.rs` and `tests/support/mod.rs`: remove lookahead
  offset profiles and ring-populated assertions; retain heterogeneous band/crossover
  parameters, meaningful nonzero gain/filter state, all links and continuation/reset gates.
- `split.rs`: remove cursor/ring fingerprint words and lookahead fixtures, update ramp
  descriptor-index +2 to +1, and remove latency-padding rationale. Preserve actual
  split-versus-forced-ramping PCM/state identity, exact snap and active-right-channel probes.
- `corpus.rs`: composition no longer contains rings/taps. The existing corpus checks
  arithmetic primitives, not full effect time-index causality; do not repin or relabel it.
- Graph fixture: replace 960-sample arrivals/silence/probe times with causal arrivals and
  current active probes, preserving ten-track banks/tails and transactional resource gates.
- When a test name/comment changes, inspect `scripts/check-step-vocabulary.py`'s exact
  `(path, needle)` allowlist and run its normal check plus `--self-test`. Its self-test
  fails on unused rows. There is no current multiband allowlist row and no current
  multiband retired-spelling hit, so no script edit is presently expected. Permit only
  removal/update of an exact row made stale by an authorized edit; do not broaden name
  admission or mechanically rewrite unrelated test prose.

No shared DSP/graph/state/control framework edits, new dependencies, compiler-IR campaign,
`.ll` evidence, historical resealing, full qualification matrix or optimization loop.

## Minimum closable evidence

Existing unaffected assertions remain mandatory regression checks. The following are
the focused claims to adapt/prove, not seven new frameworks or a larger test matrix:

1. All four launch rates: actual scalar and supported native-bank preparation/render,
   zero reported latency enabled/bypass, exact 188+188 state/zero scratch, exact cap
   accepted and one byte below rejected; frozen retained table and retired-ID rejection.
2. Current-sample first impulse and irregular nonzero signal: bypass preserves dry bits
   including -0; enabled unity output agrees with the current independent LR4 sum, never
   a 960-sample-shifted oracle. Exercise quantum 1 and 128/partition boundaries so a pad
   cannot pass. Preserve the established 2e-5 crossover/reference tolerance.
3. Keep independent four-section f64 and analytic all-pass mapping gates unchanged:
   launch rates, crossover 80/1000/8000 Hz, current 0.02 dB crossover, 0.01 dB sum
   flatness, and existing f64 phase/magnitude bounds. Keep no-step unity transition test.
4. A compact active current-sample f64 composition uses independent LR4 low/high and
   the frozen mathematical curve/retention smoother, not production `band_amplitude`.
   Include distinguishable low/high bursts (e.g. 120 Hz/4 kHz at Fc=1 kHz), independent
   gain states and attack/release ordering; use existing 2e-5 absolute PCM oracle bound
   on bounded fixtures and 0.005 dB envelope bound. Explicitly assert both band envelopes
   engage and release so a dry/identity implementation fails. One first-sample high-band
   reduction witness distinguishes current from one-sample-old detection; equal prefixes
   with different future suffixes prove no anticipation. Prefix equality alone is inadequate.
5. Preserve actual W1/W4/W8 PCM/state/report equality, heterogeneous channels/tracks,
   all links, representative ramp/settled partition equivalence, both resets, nonzero
   current-state scalar/bank interchange and transactional malformed/old payload rejection.
   Include the root-approved different-crossover restore/full-default-reset discriminator.
   Keep strict malformed request/member rejection, legitimate program fallback, signed-zero
   bypass and current-block fault/recovery with allocator positive control. Preserve actual
   W4 generic/property execution without representing it as native engine dispatch.
6. Extend the existing graph vertical to prove a zero multiband contribution beside an
   unchanged delayed route, using a still-delayed processor such as limiter, not #738's
   now-causal gate. Assert exact PDC arrivals/inserted delays and enabled/bypass agreement.
   Do not assert phase cancellation between LR4-processed and dry parallel audio.
7. Run real prepared scalar/bank render under the existing allocation/free audit with
   sustained nonzero input and automation; preserve zero allocations/frees. Focused tests,
   affected strict Clippy/fmt/policies, native corpus and supported scalar-Wasm/simd128
   evidence are required. Reuse existing root current-artifact/browser gates; do not change
   budgets or pins merely to pass. The existing primitive corpus should stay unchanged.

Run each proportional full affected suite once after the focused tranches are green;
repeat only for a new edit/failure. Do not rerun long unaffected crossover or artifact
matrices at every local checkpoint. Changing a tolerance, gate or required assertion needs
an explicit finding and scope ruling; self-comparison is not independent DSP evidence.

## First implementation checkpoint

After root activation, Luna's first tranche is `src/lib.rs` plus the necessary surgical
`src/split.rs` adaptation: remove ID 2 and ring/tap/cursor machinery, implement current
band detection/output, install the frozen codec/resources, preserve all unchanged DSP,
and retain the split/endpoint/channel properties. Add the bounded different-crossover
restore/default-reset discriminator in the crate's existing test area, capture its inherited
failure before fixing it, then repair only the prepared-default coefficient ownership.
Do not add a production API or a new benchmark/reference framework.

As soon as the crate builds and its focused library/split tests pass, pause for root's
exact-path checkpoint. It may candidly identify external tests/fixtures still carrying
the old parameter/latency contract; that checkpoint is not product PASS. Next adapt the
existing public tests/support in place, then the thin descendant compatibility/PDC cases.
Root performs SDK/artifact/delivery work after source freezes. No second implementation
tranche may be layered onto an uncommitted coherent checkpoint.

## CPU and listening boundary

No new performance budget or speedup claim. The inspected standing console workload
contains no multiband. `tests/descriptive_frame_cost.rs` feeds processed output back as
the next input and retains a best-of result: it is not valid active-workload evidence.
The ignored `split.rs` benchmark refreshes real stimulus but measures split on/off,
lacks activity/persistence/preflight qualification, and is not causal-before/after evidence.

**Root-approved CPU disposition: no timing in #739.** Create one stateless bounded
measurement successor at the implementation boundary; root assigns and synchronizes its
actual number. Record numerical CPU qualification pending that successor. It must reuse
one existing active multiband workload, freeze arguments, activity checks, schema, counts,
units, output persistence, exit propagation and overwrite refusal, and run exactly one
invocation with one warmup and two measured rounds. Any necessary existing-harness
correction is scoped there. No new framework, broad console mutation, measurement redo
or historical baseline claim belongs in #739. No unmeasured low-CPU/floor claim.

The causal effect no longer anticipates transients; finite attack can pass initial peaks.
It is not a brickwall/true-peak limiter, and no superiority over old default lookahead is
claimed. Changed-sound matched-level blinded listening remains pending in #26 unless an
actual new record exists. Deterministic renders/oracle agreement are not human listening.

## Research basis and known independent finding

Start with `dsp-research/dynamics.md`, `dsp-research/BIBLIOGRAPHY.md`, the existing
`ReferenceLr4Crossover`, and `lr4_two_section_mapping_f64.rs`. The current coefficient
derivation is independently supported by [Simper's author technical note](https://cytomic.com/files/dsp/SvfLinearTrapezoidalSin.pdf).
The repo records Zavalishin's *The Art of VA Filter Design*, chapter 4, for the squared
Butterworth/LR4 mapping and Giannoulis, Massberg and Reiss, JAES 60(6), 2012, for the
gain-curve/envelope law. This scope read the repository derivations; it does not claim a
newly fetched full reading of those two publications (the author/publisher endpoints
did not return their full documents during this scope). Explicit sample latency and
separate tail reporting follow [Steinberg's processor contract](https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html).
Causal product choice is the owner's ruling, not a requirement attributed to these papers.

Read-only inspection also found a pre-existing reset hazard: `commit_side` replaces
`side.designed` with a restored crossover, while `full_reset` reloads that design but
sets `crossover_hz` to prepared defaults. A restore at a different crossover can leave
actual coefficients inconsistent with the advertised default frequency. Existing offset
profile tests do not discriminate that case. Root explicitly approved the bounded local
discriminator/fix amendment above during scope; it is part of #739 if the test reproduces
the mismatch. Do not silently claim it previously qualified. It is not evidence that causal
ring removal needs a crossover redesign.

Scope approval: **Astra XHIGH PASS at actual base
`17d755e90217f12fb3b8324d0b77bf560d8b35f7` for the direct causal product boundary,
exact contract, bounded reset amendment and measurement-successor disposition above.**
Root's #738 closure/issue synchronization/activation is still required before implementation.
No repository file was edited and no build, test, benchmark or implementation PASS is
claimed by this read-only scoping turn.


## Activation — 2026-09-11

Root verified #738 CLOSED/COMPLETED after PR #747, exact PR and main qualification
PASS, and clean delivered-worktree removal. The current base remains
`17d755e90217f12fb3b8324d0b77bf560d8b35f7`; Astra XHIGH final scope above applies.
Issue/spec reconciliation found no missing remote issue. Implementation is active
in isolated branch `codex/causal-multiband-739`, one launch feature at a time.
CPU qualification successor is #748, “Qualify causal multiband-compressor CPU
cost on an active-input workload”; it is queued and authorizes no timing here.
Luna XHIGH owns the first bounded core/reset tranche, Sol XHIGH verifies.


## User-requested immediate usage handoff — 2026-09-11

Implementation is paused mid-tranche at the owner's request to transfer instances.
The preserved `src/lib.rs` WIP is deliberately **not compiling** and must not be
merged or counted as completed. Root's `cargo check --locked -p multiband-compressor`
returns101 with18 errors and1 warning: removed cursor/ring fields remain referenced
in state adapter wrappers and one reset/render location; codec helper call arities
still need adjustment. The raw diagnostic is `/tmp/issue739-gates/handoff-check.stderr`.
This is a recovery checkpoint, not a failed final attempt verdict or qualified product.

The inherited different-crossover restore/full-reset discriminator was added and
failed before the fix at exact test
`reset_tests::full_reset_restores_prepared_crossover_coefficients`. Raw baseline
and RED logs are in `/tmp/issue739-luna-tranche1/`; Sol independently confirmed
that the discriminator exercises the mismatch. No post-change test PASS exists.

Resume the existing Luna XHIGH core/reset tranche, finish remaining lib.rs codec/
reset callers and surgical split.rs changes, then run the build and focused library
tests and stop for root's next exact-path checkpoint. Public tests/support, active
f64 oracle, compatibility/admission/PDC descendants, SDK/artifacts and final
Sol XHIGH verification/PR/main CI all remain. Preserve the existing frozen gates.
The thin existing-pattern host-web atomic ID2 rejection test is scope-consistent;
Sol accepted this clarification, alongside lattice63rows/189checks. No production
control framework change is authorized. Tracked JSON inventory found only shipped
SDK metadata plus the synthetic validator fixture, and no session JSON to update.
CPU #748 remains queued with zero timing; listening #26 remains pending.
