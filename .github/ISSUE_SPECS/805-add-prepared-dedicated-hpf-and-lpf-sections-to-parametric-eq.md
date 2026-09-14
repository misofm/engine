# Add prepared dedicated HPF and LPF sections to parametric EQ

## Parent and authorization

Delivery child of #804. Astra XHIGH scope approved for implementation; the user's explicit workflow is bounded Luna XHIGH implementation and Astra MEDIUM adversarial verification. Root records the GitHub issue number and matching local spec filename/index before implementation. At most five coherent implementation attempts, each receiving one adversarial verdict. This spec owns one independently usable capability: prepared session/native/offline EQ cut sections. The next child owns live off-render prepared updates, and the app release waits for that child. No prepared-only control is advertised as live.

## Product contract

A single `miso.parametric-eq` instance renders dedicated HPF -> original four general bands -> dedicated LPF. Both dedicated cuts default disabled. They consume neither an existing band nor another effect instance. Existing band parameter IDs, descriptor order and meaning remain stable. Existing general bands continue to support every existing kind. Effect-wide bypass covers all six sections with unchanged zero latency.

Adopt second-order **12 dB/oct** TPT SVF cut filters in this release, with independent Q. This explicitly resolves the #804/#191 slope decision for the product slice: variable 24/36/48 dB/oct and first-order variants are deferred to separate post-launch scope. No selectable slope parameter or inert UI slope exists. Reuse the current EQ HPF/LPF design equations and numerical authority, already documented in `crates/parametric-eq/src/lib.rs` and `docs/rulings/`; no new filter algorithm.

## Frozen metadata and session schema

Append the following six parameters after the existing 24 rows. The existing `name` field is the stable machine key; the separate authoritative unit/domain metadata provides semantics. There is no renamed alias or additional competing key field.

| Numeric ID | name | domain | unit/mapping | minimum | maximum | default |
|---|---|---|---|---|---|---|
|65|hpf-enabled|boolean|linear/stepped|none|none|false|
|66|hpf-frequency|continuous|Hz/logarithmic|10|20000|80|
|67|hpf-q|continuous|ratio/logarithmic|0.1|18|1/sqrt(2)|
|81|lpf-enabled|boolean|linear/stepped|none|none|false|
|82|lpf-frequency|continuous|Hz/logarithmic|10|20000|18000|
|83|lpf-q|continuous|ratio/logarithmic|0.1|18|1/sqrt(2)|

All six use PerLane channel policy, readable=true, automation rate=None, smoothing=None/0, automatable=false, liveUpdatable=false in this child. The live successor changes capability only when both running hosts can apply every admitted edit. Values use the existing lattice rules associated with their units/domains. All launch sample rates 44.1/48/88.2/96kHz use the same range 10..20000 Hz, below each Nyquist. Dedicated EQ cutoffs may overlap; no cross-cutoff ordering restriction is invented here. Builtin live control's separate hpf<lpf invariant remains in its child.

Session sparse values preserve default behavior: old sessions with no cut parameters render with both disabled. New explicit values round-trip through the existing typed session parameter representation. Existing SDK prepared `effect()` objects acquire these rows through ordinary generated metadata; no handwritten parameter map.

## Implementation decisions

1. Keep the count of general bands 4 distinct from physical cascade sections 6. Original band parameter/descriptors index 0..3 remains stable; their actual DSP cascade order is 1..4, preceded by dedicated HPF at 0 and followed by LPF at 5. Use explicit mappings where descriptor order differs from DSP order; do not force new cuts into the old six-fields-per-band arithmetic.
2. Each cut is the existing fixed `EqBandKind::HighPass`/`LowPass` design, no kind/gain/shelf parameter exposed. Disabled uses the exact existing identity words. Reuse existing f64 preparation and rounded-f32 coefficient authority, NaN/domain rejection, denormal handling, reset, state, and effect-wide bypass semantics.
3. The stationary renderer must support 6 sections safely. Today backend depth-4 dispatch plus integer `length/DEPTH` assumes divisibility and can drop tail sections in release. Adopt an effective depth 2 for six-section mono/dual interleave selection, padding and dispatch, or use an explicitly reviewed correct remainder implementation. Tests must make the last LPF audible under every backend. Identity elision preserves established sign-of-zero/state rules; disabled cuts must leave original-band rendered bits unchanged.
4. Public response descriptor IDs 1..4 remain the original bands. Append response section 5 `HPF` and 6 `LPF`. Public output order remains ascending IDs 1..6 even though DSP order is 5,1,2,3,4,6. Both requested-configuration and retained-coefficient response snapshots use a single explicit mapping. The response descriptor validator requires strictly increasing IDs.
5. State payload is 6 × 19 = 114 32-bit words = 456 bytes per channel, plus the existing 8-byte common header. Keep prelaunch layout tag 1 and reject old 304-byte channel payloads explicitly by length/header shape. No reinterpretation or fabricated migration. Recount prepared/runtime state and memory ceilings as well as serialized state. Scratch remains 0; latency 0; tail infinite.
6. Recount the EQ's class-A floor by active section count and actual backend cascade padding. This is accounting, not a new optimization loop. No projected or measured performance claim without evidence.
7. This child does not modify the live `EffectControlRecord` path. Current `automate()` designs coefficients inside process for old numeric bands; no new cut row enters that path. The off-render live successor must address it before claiming live cuts; copying this existing function is not evidence of off-render design.

## Bounded implementation tranches

Root owns commits, GitHub synchronization, issue/spec evidence, branch changes and release. Each Luna stops immediately after its coherent compiling tranche and focused tests; root commits exact paths before the next tranche. No overlapping uncommitted edits.

**Tranche 1 — EQ representation and prepared render.** Primary owner paths: `crates/parametric-eq/src/lib.rs`, `crates/parametric-eq/src/response.rs` only for the minimum compile-required response array/mapping adaptation, and `crates/parametric-eq/tests/{contract,conformance,bank,mono_collapse,analytic,time_domain,determinism}.rs` plus `tests/support/mod.rs` when helpers need to derive descriptor counts. Implement descriptor/default-target storage, six-stage ordering, correct mono/dual stationary dispatch, prepared/reset/payload updates, bypass and focused tests. No host/SDK/ABI-control edits. Use existing fixtures or tiny deterministic signals. Checkpoint gate: `cargo test -p parametric-eq` (bounded failures in older fixtures may be recorded only if root explicitly approves a useful buildable checkpoint; prefer repair in the owned test paths), and `cargo check -p parametric-eq --target wasm32-unknown-unknown` with repository target flags. Stop with paths, exact commands/results and remaining failures.

**Tranche 2 — response, catalog and downstream compatibility.** Primary owner paths: `crates/parametric-eq/src/response.rs`, `crates/parametric-eq/tests/response.rs`, `tools/parameter-metadata/tests/round_trip.rs`, `sdk/assets/miso-engine-v1-parameter-metadata.json`, `sdk/src/generated/catalog.ts`, SDK prepared-session tests, existing focused response/bank integration fixtures that fail solely because section/parameter counts changed. Generated files are refreshed by normal generators. Add exact response-ID/DSP mapping tests, 6-section total/section curves, old/default session bit regression, catalog truthful prepared-only rows and old payload refusal. Audit direct EQ_SECTION_COUNT consumers with rg and adapt only concrete necessary source/test consumers; report any expansion to root before new cross-crate design. The parallel #147 ABI child owns metadata schema structure; coordinate regeneration after root checkpoint, do not overwrite its work.

**Root evidence tranche.** Root may delegate a separate bounded read-only audit of state/floor accounting and reference updates. Owned documentation: numbered local child spec, #804 tracker, `docs/rulings/effect-floor-accounting.md` relevant EQ inventory, and a concise existing EQ implementation note or child decision record. No whole-corpus byte repinning merely to make gates green. Any new unrelated benchmark or audit harness is a successor.

## Mandatory acceptance evidence

- Exactly one prepared EQ instance includes HPF + 4 general bands + LPF; every old ID keeps its meaning; old sessions default to cuts disabled.
- Deterministic filtered PCM and existing independent response oracle cover HPF-only, LPF-only, both, disabled identity, Q/cutoff boundaries and all 4 launch rates. All output/state stays finite under existing policy. Reuse existing independent analytic/impulse authority, not a test that only repeats the implementation.
- Original-band rendered bits with cuts disabled remain identical, including scalar/SIMD/mono collapse and tail-track coverage. A 6-section regression makes the final LPF detectably affect output under scalar, Simd4, Simd8; this specifically catches remainder-dropping dispatch.
- Public response IDs and original-band meanings remain stable; HPF/LPF appear as 5/6 and totals reflect physical cascade. Effect-wide bypass returns unity for the full effect.
- Payload sizes reflect 456 bytes per channel, old shape is refused atomically, and valid snapshot/restore/reset round trips include all 6 sections. Relevant prepared memory accounting follows actual allocations.
- Metadata/SDK generator checks show all six controls and accurately keep them prepared-only. Native and Wasm builds pass. Existing realtime allocation gate passes for rendering prepared cuts; no coefficient design is added to callback paths by this child.
- Proportional commands include `cargo test -p parametric-eq`, `cargo test -p parameter-metadata`, the repo metadata generator/check, SDK `check:generated` and relevant prepared-session/type tests, plus target build. Root chooses integration checks from actual affected consumers. Run broad checks once at delivery; repeat only for new changes/failures.

Existing SVF listening provenance remains applicable to the reused second-order design. Record any new listening actually performed candidly; do not claim unheard fixtures or block an additive prepared slice on scheduling a new broad listening corpus. No descriptive benchmark is mandatory solely to close this correct product slice. If run, freeze/preflight once and use exactly one warmup/two measured rounds without tuning.

## Non-goals and successor contract

No live cut control, new generic queue ABI, variable slope, new filter family, higher general-band count, timeline/UI, hidden EQ instances, new response protocol, or dependency on unfinished #774. A separate live-EQ issue owns off-render prepared coefficient updates, enable/disable ramps, retarget/ack/backpressure tests and browser/headless SDK parity. A separate builtin issue owns live strip cuts and paired cutoff invariant. Parent #804 stays open until both live outcomes are delivered. The app uses dedicated EQ cuts only after an immutable qualified live-capable release.

## Decision/evidence record

Astra XHIGH brief PASS: frozen above from current engine 551f6d7e and live issue 804, the D2 builtin ruling, and current EQ/response source. Implementation attempt 1: pending. Astra MEDIUM adversarial verdict: pending. Root must record checkpoint hashes, focused gates, upstream issue synchronization and later final verdict here; no issue completion claim before upstream evidence.

## Attempt 1 — prepared representation checkpoint

Luna XHIGH completed the six-section representation, original-band mapping,
prepared cut descriptors, depth-two dispatch, payload codec and minimum
response/fixture adaptations. Root approved the necessary existing corpus and
response-test count adaptations. Exact changed implementation paths are
`src/{lib,corpus,response}.rs` and `tests/{contract,response}.rs`,
`tests/support/mod.rs`, all under `crates/parametric-eq/`.

Passed: package cargo check, fmt/diff hygiene, clippy over all targets/features
with `-D warnings`, Wasm target check, and the complete parametric-eq test run.
All active suites passed; existing ignored descriptive cases were not changed.
The agent unnecessarily started a duplicate final test run; it also completed
without failures, and no additional repeat is required. Evidence:
`/tmp/805-tranche1-evidence.md`.

The six-section serialized payload is 456 bytes per lane, 920 bytes total with
the common header; this is not the resident Rust object size. New HPF/LPF
stationary regressions prove both sections execute at widths1/4/8. The current
disabled-cut test compares two six-section paths; it does not independently
prove old-four-section bit identity. Tranche2 must add that direct oracle,
complete response/catalog/prepared-SDK coverage and confirm resident accounting
before the coherent attempt receives its fresh Astra MEDIUM verdict. No live
cut capability or completed #805 claim is made by this checkpoint.
