# Make strip HPF/LPF live and add dedicated cut sections to parametric EQ

## Problem

The Miso mixer exposes per-track HPF and LPF controls, but changing them only updates frontend state and the plotted response. The running engine receives no DSP update.

The app resolves `hpf_on`, `hpf_hz`, `lpf_on`, and `lpf_hz` to `prepared-builtin` targets. `EngineConsoleWriter.setParam()` and the authoritative binding therefore return `prepared-only`; the values are compiled into the session document but have no live command path. The engine catalog confirms `hpf_hz` and `lpf_hz` are `PreparedOnly` / `liveUpdatable: false`, unlike pan, fader, mute, trim, and polarity.

The frontend is removing these controls until the engine exposes an honest live capability. Do not solve this by inserting hidden parametric-EQ instances per track: that duplicates effects and consumes runtime resources to emulate functionality owned by the engine.

## Required product outcome

Deliver both capabilities on the engine side:

1. **Live track-input builtins.** Make the existing builtin HPF and LPF enable/cutoff state safely controllable while a session is running, through the normal console and SDK path, with behavior comparable to the other live strip controls.
2. **Dedicated parametric-EQ cut sections.** Add HPF and LPF sections to `miso.parametric-eq` itself without consuming or repurposing its four general-purpose bands and without requiring extra EQ effect instances. The effect should retain four independently adjustable bell/shelf/notch bands in addition to its dedicated input and output cut filters.

These are related DSP/control-contract changes and should be designed together, as required by `docs/rulings/builtins-input-liveness-d2.md`. That ruling explicitly defers live HPF/LPF until coefficient-ramp design, elision invalidation, and issue #191's variable-slope decision are addressed together.

## Design requirements

### Builtin live control

- Preserve builtin parameter IDs 3 (`hpf_hz`) and 4 (`lpf_hz`) and the existing `0` disabled value. Any ABI additions must be additive; append command kinds rather than renumbering existing kinds.
- Define typed semantic SDK edits (for example on `TrackEdits`) and carry them through browser and headless hosts to the existing input-stage owner. The metadata must report the final capability truthfully.
- Design coefficients off the render thread using the same numerical authority as preparation. Do not run trigonometric/coefficient design or allocate on the render callback.
- Smooth coefficient changes with a stable, explicitly specified ramp. Reuse the parametric EQ's proven coefficient-ramp approach where appropriate; do not step coefficients abruptly or interpolate cutoff Hz while redesigning on the render thread.
- Recompute/invalidate `InputStage::plan` whenever a live cutoff changes. A prepared identity section becoming active must never remain elided.
- Preserve and test channel-symmetry behavior for `Left`, `Right`, and `Both`, including the mono-collapse admission boundary.
- Specify atomic validation for the invariant `0 < hpf < lpf` when both are enabled. A one-parameter command must not leave a track in an invalid filter order; paired changes need an atomic batch story.
- Disabling a filter must ramp safely to the identity response. Define retarget-during-ramp and reset behavior.

### Parametric EQ

- Add dedicated HPF and LPF sections around the four existing general-purpose sections in the single `miso.parametric-eq` instance.
- Expose enable, cutoff, and the adopted slope/Q controls through generated effect metadata and the existing live effect-parameter path. Resolve issue #191's slope requirement explicitly; do not silently freeze a UI-only slope value.
- Preserve stable IDs for every existing EQ parameter. Append new parameter/section IDs and update generated catalogs, response-query section metadata, session builders, SDK types, and fixtures.
- The effect-wide bypass must remain latency-preserving and must bypass the dedicated cut filters together with the four general-purpose bands.
- Recount effect state, scratch, memory, and class-A render floors because the section count changes. Disabled dedicated filters should take the engine's established identity/elision path rather than requiring separate effect instances.

## Acceptance evidence

- A browser and headless console test starts with both filters disabled, enables each filter during continuous audio, sweeps cutoff, disables it again, and proves the rendered PCM and response snapshot change at the acknowledged application boundary.
- Objective response tests cover HPF-only, LPF-only, both enabled, disabled identity, all launch sample rates, boundary cutoffs, channel-asymmetric targets, and valid/invalid HPF–LPF ordering.
- Discontinuity tests cover enable, disable, sweep, mid-ramp retarget, reset, and block-size partition invariance. State and output must remain finite under the existing NaN/denormal policy.
- A regression starts from an elided disabled section, enables it live, and proves the filter processes audio; the inverse returns to the permitted identity/elision state.
- Parametric-EQ tests prove one effect instance contains dedicated HPF + four general-purpose bands + dedicated LPF, all existing band IDs retain their meanings, and the dedicated sections do not consume bands 1–4.
- Metadata/codegen checks prove the SDK exposes only capabilities the running browser and headless engines apply.
- Run the proportional Rust, SDK, Wasm/browser, ABI, metadata, response, realtime-allocation, and package gates required by the repository. Record the exact commands and results in this issue/spec.

## Scope boundary

This issue owns the engine, ABI, SDK, generated metadata, and objective qualification needed for the two capabilities. It does not own the app UI reintroduction. After an immutable engine package is published and verified, the app can remove its temporary frontend suppression and bind to the generated live targets.

Do not add hidden EQ instances in the app or host, rebuild the session on every drag, perform coefficient design on the render thread, or weaken existing acknowledgement/backpressure guarantees.


## Owner-directed delivery — 2026-09-14

This is the parent coordination issue. Split the two product capabilities into
small, separately closable stateless children before implementation. Astra XHIGH
scopes, Luna XHIGH implements bounded tranches, and a fresh Astra MEDIUM
adversarially verifies each attempt. Maximum five attempts per child. Root
checkpoints every focused-green tranche and synchronizes GitHub evidence.

Include a bounded agent-facing structured key/unit ABI and SDK slice: preserve
numeric IDs, use semantic parameter keys and explicit unit metadata, and avoid
a wholesale suffix rename. The prior #147 mandatory unit-in-name proposal is
not the selected design. Existing builtin names may remain.

After engine and SDK acceptance, coordinate immutable package publication,
any necessary adapter adoption, and app release. In the app, expose the new
HPF/LPF within the existing parametric EQ instance; hide builtin filter controls
and prepare builtin HPF/LPF disabled. Prove bypass/elision and account honestly
for any retained preparation state; do not claim zero resources from a UI flag.
No hidden extra EQ instance. This extends the parent's original app-UI non-goal
through separately scoped app/adapter issues, as explicitly requested by the owner.

## Current scope checkpoint

Baseline engine main: `551f6d7e`. Planning only; no source implementation or
qualification is claimed. Existing unrelated #774 work is preserved. The dirty
primary app checkout is outside this work; integrations use isolated worktrees
from fetched remote main. Child contracts and sequencing follow Astra scope.

## Approved first slices and slope decision

Astra XHIGH approved the bounded design at `/tmp/804-astra-dsp-plan.md`;
its durable first child is #805. Order: #147 structured parameter/unit surface,
#805 prepared EQ cuts, a separately scoped off-render live EQ control child,
then a separately scoped live builtin-filter child. Fresh Astra MEDIUM reviews
one coherent attempt per child. Do not give one implementer the whole program.

Dedicated cuts use the current second-order TPT SVF authority: 12 dB/oct with
independent Q. Higher slopes and first-order variants remain #191 post-launch
product decisions; no inert slope parameter or UI is shipped. Existing four
bands and IDs remain unchanged. App-authored builtin filter cutoff values are
zero; claim identity-elided filter DSP, not zero retained metadata/state.

#147 is amended to preserve the existing metadata `name` as machine key, derive
builtin units from Rust authority, and add object-shaped edits over the existing
live console. No duplicate `key` metadata column or suffix rename is introduced.

## Approved live implementation children

Astra XHIGH briefs are frozen as #807 (off-audio-thread live EQ targets) and
#808 (builtin live filter pairs using the same preparation/transport owner).
Both use bounded Luna XHIGH tranches and fresh Astra MEDIUM acceptance.
The original 48-byte semantic command ABI remains narrow; engine-owned
prepared coefficients cross an internal trusted-host companion boundary.
#808 also requires settled-disabled filters to skip SVF work during trim
ramps, preserving signed-zero and sanitization behavior. Retained input-stage
state and actual trim work are not claimed to disappear.

Sequence: #147 merged and closed (PR #806, merge39288df4; required CI34811007238 and main CI34811451810 passed), #805
prepared EQ, #807 live EQ, #808 live builtins, then the coordinated immutable
SDK/adapter releases and app adoption/deployment. Parent stays open throughout.

The bounded engine release is tracked as #809. It requires accepted #147,
#805, #807 and #808, a new immutable version, actual browser qualification
and exact-archive publish/verify before downstream registry adoption.

Downstream delivery is now tracked in misofm/engine-web-adapter#111 and misofm/app#222, with matching committed local specs. No new package publication or app deployment is claimed yet. #805 retained a real signed-zero regression and is correcting it through the narrowly approved masked-output helper; its gate is not weakened.

### Prepared-cut delivery boundary

#805 is merged through PR #810 at 80f2918b5aba5b2428c5f5cc76c24f46b4e0edde; required qualification34820834881 passed and #805 is CLOSED. Together with merged #147 this completes the prepared EQ/structured-edit foundation. #807 live EQ implementation follows the approved bounded assignments; #808 builtin live controls, #809 package release, adapter #111 and app #222 remain open. No app integration or release is claimed yet.


### Account-switch pause — 2026-09-14

Implementation is paused at the user's request for another Codex account to resume. All source checkpoints are pushed on `codex/807-live-eq`, through `e81df55b61f563fa26952608e7383d3c060cb956`. #147 and #805 are merged/closed; #807 assignments 1–3 are complete, assignments 4–10 remain. #808, #809, adapter #111 and app #222 have not entered implementation/release. No public live-cut activation, package publication or app deployment is claimed.

The self-contained [resume handoff](https://github.com/misofm/engine/blob/codex/807-live-eq/docs/handoffs/804-account-switch-2026-09-14/README.md) records branches, exact commits, accepted decisions, test evidence, preserved plans, downstream requirements and the next bounded task. All agents are stopped. Before assignment 4, freeze its staging layout and amend the brief using the linked handoff's Astra XHIGH queue/accounting recommendation; no assignment-4 source has been written. #807 remains attempt 1 and still requires its final fresh Astra MEDIUM review. Keep this issue open.


### Resumed delivery — 2026-09-14

Implementation resumed from the account-switch handoff on codex/807-live-eq.
The assignment-4 staging/accounting amendment is frozen and pushed at 388e3c4d.
A compiling runtime-foundation recovery checkpoint follows; its new delivery/budget
acceptance fixtures and host preflight are still pending. #807 remains attempt 1,
with no public live-cut activation or final PASS. #808, #809, adapter #111 and app
#222 remain subsequent delivery work. The active app/adapter worktrees remain clean
at their preserved preparation checkpoints. Parent #804 remains open.


The next #807 checkpoint completes original-command host preflight and focused queue/
refusal/staging gates, including actual counter, observation and solo rollback checks.
Real EQ runtime application and exact budget tests remain before assignment 4 acceptance.
No live-cut activation or release is claimed; the complete evidence is in #807.


#807 now has real EQ queued PCM evidence: scalar/bank parity, mono-collapse exit with
an asymmetric retarget, and FIFO overlap against an independently prepared final state.
A reordered negative control fails as required; the restored fixture and complete host
suite pass. Exact assignment-4 resource fixtures remain, followed by owner/host/SDK
integration. No public live-cut activation, package publication or app deployment yet.


The next #807 checkpoint proves graph queue/target/lane storage with independent
arithmetic and exact/one-byte-below cap tests, including a queue that determines the
largest allocation. Native focused tests, strict Clippy and Wasm compilation pass.
Host decoded-storage evidence remains before assignment 5; no release claim.


#807 assignment 4 is complete: host decoded-buffer charges and console-off/on exact
aggregate budget acceptance/refusal now pass, alongside queue/runtime/PCM gates.
The compiler-owned candidate/publication transaction is the next bounded assignment;
its separate resource-report and checked publication design is frozen in #807.
#807 remains open, attempt 1, with assignments 5–10 and final review pending. No
live-cut activation, SDK release or app deployment has occurred.


#807 assignment5 now has a checked owner/publication component: exact candidate
validation and complete-prefix queue room precede publication, and commit follows
publication once. Root strengthened real-EQ refusal/state tests; native focused gates,
strict Clippy, Wasm and realtime policy pass. Actual owner resource propagation and
remaining transaction fixtures are still pending before assignment5 acceptance.
Production live-cut activation and downstream releases remain unperformed.


A follow-up #807 caller audit fixed five remaining host-core test accesses to the
private endpoint. The three affected suites pass (23 tests, one existing ignored),
and full workspace all-target compilation passes. Resource implementation resumes;
this is caller correction evidence, not another completed product capability.


#807 assignment5 resource propagation now charges actual native producer/owner/factory
storage and the browser's replacement table. Independent arithmetic and exact-budget
refusal tests pass, alongside strict Clippy and Wasm checks. Remaining assignment5
transaction and allocation fixtures precede the preparation ABI work; #807 stays open.


#807 assignment5 is complete: actual authoritative owner seeding, successive queued
transactions, revision overflow and prewarmed zero-allocation/free gates pass.
Preparation ABI and host/browser integration are next. Per the user's direction,
reuse existing critical evidence and prioritize the working end-to-end route over
optional verification and optimization. No production activation or release yet.


#807 now includes the stateless preparation interface, fixed browser workspace and
generated additive ABI. Focused real-EQ and malformed-header checks, native/Wasm,
Clippy and generated ABI gates pass. The next step connects prepared targets to
atomic host admission, then the shared browser/headless helper. Production activation
and all releases/deployment remain pending.


#807 host admission and accepted-shadow copy are implemented with fixed storage and
original command indexes. The corrected checkpoint passes 98 host tests (2 existing
ignored), native/Wasm, Clippy and ABI generation gates. Shared browser/headless helper
wiring follows. Opted-in transaction integration gates are consolidated into the real
production cutover rather than an intermediate fake registry setup; they remain
required before release. No public activation, package release or deployment yet.


### Assignment8 recoverable draft, 2026-09-14

Checkpoint the bounded Luna browser transport/helper draft without claiming assignment8
completion. The worklet calls real config-copy/prepared admission exports; helper asset
inclusion and private host correlator branches are present. Root restored ordinary
`command()` routing: production activation remains assignment10.

Syntax checks, helper construction against generated ABI, generated SDK check, source
policy and diff check passed. Browser defaults reported 30 passes and 11 failures
without the required built artifact; this is not a passing browser integration result.
Package smoke did not start because its TypeScript dependency was unavailable. No
benchmark or final artifact build was performed.

Known helper corrections remain before use: separate transferred/ACK comparison bytes,
immutable error construction, synchronous existing-instance/raw-byte entry point,
post-await lifecycle guards, known-ordinary bypass of the prepared pending slot, and
one authoritative encoded semantic snapshot. Preserve this draft and finish the same
component; no new attempt/verdict or completion claim.


### Assignment8 shared helper checkpoint, 2026-09-14

The corrected browser component now accepts one raw semantic snapshot, shares Rust
preparation and ACK commit logic across async and sync entry points, owns separate
transfer/comparison bytes, and reserves the prepared pending slot before awaits.
Known ordinary-only batches bypass that slot. Refusals preserve original indexes and
normal ACK/report backpressure; malformed replies and lifecycle failures invalidate
cached seeds. The selected verified module is retained; preparation instantiation is
lazy, with no boot/render. ABI JSON is loaded once before host exposure and injected
into the helper. The temporary source JSON symlink is removed. Production command
routing and factory capability remain unchanged until assignment10.

Root corrected remaining busy/revision exhaustion, missing-export, non-edit refusal
and sync-error handling. A compact committed transport fixture is now part of the
existing worklet suite: async/sync payload parity, detached transfer/ACK, next accepted
seed/revision, indexed nonfinite refusal, known-ordinary submission while busy,
disposal during config-copy, malformed ACK and fresh-seed recovery. This fixture is
transport evidence, not an alternate EQ designer or actual production PCM proof.

PASS: root compact helper plus existing worklet Node suite, implementer full existing
worklet shell suite/source checks, root focused Rust control-target tests (2), ABI
validator and ordinary generated assets/codegen. No benchmark or final artifact build.
Proceed to assignment9 headless callbacks/types, then assignment10 production cutover
and the consolidated real-host/audio gates. #807 remains open, attempt1.


### Assignment9 headless boundary checkpoint, 2026-09-14

The SDK's private prepared submission now uses its existing Wasm exports and the
shared synchronous helper with generated ABI layouts. Config-copy, companion staging,
real prepared admission, command-report decoding and dispose/reboot invalidation are
connected. Root factored ordinary submission through its existing staging/validation
function; numeric and semantic commands still converge at submitCommands. A narrow
internal declaration describes the already staged shared JS asset. Public submission
remains on the ordinary route until assignment10.

PASS: TypeScript noEmit (including root's ordinary-path factoring), canonical shared
helper fixture, generated code check and diff check. No benchmark or artifact build.
Assignment10 now owns the factory/callsite/metadata switch plus actual production
admission, DSP and packed browser/headless qualification. #807 is open, attempt1.


### Assignment10 source cutover checkpoint — qualification pending

The real EQ factory now advertises prepared targets, the six cut rows are live with
fixed64-sample linear coefficient smoothing, and browser/headless public commands
call their shared prepared lowering. Raw scalar/bank EQ spans are counted/refused;
coefficient design was removed from processing. Direct EQ fixtures and the existing
console-workload/bench callers prepare targets off render; no benchmark was run.
Production host classification now uses the native descriptor ID, correcting a
previous comparison against the session effect-slot ID.

PASS: all parametric-eq tests, console-workload/bench compilation, metadata roundtrip,
production preparation-workspace test, TypeScript and source browser bundle. The
full host run recorded93 passes/5 failures/2 existing ignored; root then corrected
the exact-budget owner accounting fixture and its focused test passes. Four old
raw-EQ callers remain to migrate: native command-timeline parity, effect parameter
application sample, bypass setup, and the maximum decoded-staging fixture (use an
ordinary effect there so coalescing does not weaken its capacity assertion).

The existing browser response episode now prepares all six cut rows plus old numeric
EQ, checks fresh target capture after ACK A and compares post-A+64 PCM with the same
SDK headless boundary. It has not yet run against a new artifact. Qualification can
resolve runtime imports from an unpacked SDK distribution; existing source-only CI
remains supported. The helper is included in exact artifact sets and has one source
symlink for native Node type-stripping/source bundling, with no duplicated algorithm.

This is a recoverable compiling checkpoint, NOT #807 PASS or release readiness.
Remaining: migrate the four host callers/direct JS oracle, close actual production
transaction/diagnostic/allocation/designer proofs, adapt affected hermetic host
stubs, then build/repin the artifact and run proportional SDK/packed browser gates.
Fresh Astra MEDIUM adversarial review and merged/GitHub delivery remain required.
Keep issue open, attempt1; do not treat pending qualification as completed evidence.


### Assignment10 caller and realtime checkpoint, 2026-09-14

The four stale host callers now use actual prepared admission; the staging-capacity
fixture uses an ordinary compressor and retains its 510-span assertion. The shared
JS direct oracle is migrated, and ordinary worklet fakes provide a real minimal
Unsupported preparation export without weakening missing-export handling.

PASS: full native host library suite (98 passed, two existing ignored), existing
worklet shell suite, and the production scalar/bank test-support episode. Root also
ran that feature-enabled episode: off-thread preparation calls the real designer;
prepared admission/render make zero designer calls, allocations or frees. Successive
scalar ACKs advance owner revisions before rendering. Existing contract declarations
now describe prepared EQ targets and exact A/A+64 timing.

Still pending: production mixed-owner refusal variants and the built artifact,
SDK/packed browser qualification, then fresh adversarial review and merged delivery.
These are source checkpoints; #807 remains open, attempt1. No benchmark was run.


### Assignment10 production artifact checkpoint, 2026-09-14

Production-owner refusal/recovery now covers missing/extra and unsafe targets,
stale generation/revision, invalid original edits before overwrite, original wire
indexes, and a full unrelated queue. The same mixed EQ/matrix batch succeeds after
that queue drains. Root's focused refusal gate passes; the earlier scalar/bank
allocation/designer gate remains green. Strict all-target/all-feature clippy passes
for the touched Rust packages, with small test/benchmark caller lint corrections.

The reproducible shipped Wasm pin is now
`1e80588ceafce0777fd0e098b3ceb966b74352c4cd9c7b4111fada2ba301aa52`.
Artifact static/object, ABI, actual boot high-water and SDK publishable-tarball gates
pass. The migrated direct oracle preserves every native/Wasm PCM digest; expected
resource totals reflect actual larger bridge storage, and a main-realm preparation
refusal has application sample0 because no audio admission occurred.

Headless qualification found three stale fixture expectations (artifact count and
old EQ row/queue costs); those test updates are in progress. Packed-browser runtime
qualification and final adversarial verdict are still pending. No benchmark,
release, merge or issue completion is claimed.


### Assignment10 packed browser/headless qualification, 2026-09-14

The existing qualification runner passed Chromium151.0.7922.34, Firefox153.0 and
WebKit26.5 against the unpacked npm archive, including its compiled SDK modules and
workers. The live EQ episode edits all six cut controls plus an existing band gain,
requires ACK/fresh target capture, and compares finite post-A+64 browser/headless PCM
within1e-6. Existing fault mutations pass. The checked deployment matrix identifies
source candidate695b38bb25f7d6b548fd04fa20474d87cc3ebd87 and the new Wasm pin.

Final headless eval suite:254 passed,0 failed. The writer fixture retains ordinary
compressor two-slot saturation and exercises prepared EQ coalescing/backpressure
with its22 semantic rows and a deliberately small two-target queue. Package tarball,
static/object, direct native/Wasm PCM, strict Rust lint and the focused production
transaction/realtime gates are green. No benchmark or extra framework was added.

Implementation attempt1 is ready for fresh Astra MEDIUM adversarial review. No final
verdict, merge, release or closure is claimed yet; #807 and #804 remain open.


### Final adversarial verdict — PASS, attempt1

Fresh Astra MEDIUM review accepted source6d8c1f75 with no material blockers. It checked
whole-batch validation/preflight before publication/owner commit/ACK, target coverage
and semantic association without redesign, frozen queue drains, bank application
after desymmetrization, exact A+64 timing, and helper semantic-byte/ACK/lifecycle
handling. It inspected the production realtime, refusal/recovery, artifact,254-test
headless and three-browser packed qualification logs. No additional harness was
requested or added.

#807 implementation and local qualification are accepted. PR#811 required CI and
merged delivery remain pending; close/synchronize #807 in the delivery workflow once
this evidence is upstream. Parent#804 continues with #808, #809, adapter#111 and app#222.


### Required-CI compatibility correction — attempt2 PASS

Required run34861401024 exposed three assumptions missed by local qualification:
the graph accounting fixture still treated production EQ as target-unsupported;
the SDK deletion gate interpreted a private owner label as a retired engine state;
and the conformance discovery scan classified cfg(test)-only compiler/host wrappers
as product factories. #807 was reopened for this bounded correction.

The accounting fixture now explicitly constructs its ordinary lane case, retaining
exact independent byte arithmetic and cap assertions. The helper consistently uses
private owner label `targets`. Existing mock-directory exclusions now include the
verified effect-compiler/host-core test wrappers; all eight production effect
factories still run the shared conformance harness. No DSP, ABI or Wasm changes.

PASS: focused graph resource test, SDK deletion gate plus37 mutation checks, shared
sync/async helper tests, formatting, and the existing effect-contract conformance
run (eight production factories, zero failed gates). Fresh Astra MEDIUM reviewed the
four-file correction and recorded attempt2 PASS with no blockers. All other jobs in
the prior required run passed, including DSP, math, Wasm guests, cross-target and
all three browsers. Restart required CI on this correction before merging PR#811.


### Native integration compatibility correction — attempt3 PASS

Required run34862552366 passed every leaf except workspace debug tests, which
progressed past the corrected graph fixture and reached two native bank fixtures
still submitting raw EQ parameter records. The existing symmetry fixture had the
same caller pattern, and the host resource fixture still counted strings alone.
#807 was reopened for this bounded test-only correction.

The two fixtures now share a small helper using real off-render owner preparation,
preflight, publication and commit; their existing bank routing and symmetry
assertions are unchanged. The host accounting fixture measures owner/slice layouts
and asserts all nine owners share the factory before charging its actual Arc layout
once. Exact table/payload/largest-allocation assertions remain intact.

PASS: full host-core integration/doc suite (156 passed,two existing ignored), strict
host-core Clippy, and the exact previously failing workspace CI command locally
(1211 passed,nine existing ignored), plus native host smoke. Fresh Astra MEDIUM
reviewed the four test-file delta and recorded attempt3 PASS with no blockers.
No runtime,DSP,ABI,SDK or Wasm changes. Push this correction and require green CI
before merged delivery; no new test framework or benchmark was introduced.


### #807 merged delivery, 2026-09-14

Required qualification34863942121 passed all jobs for4333a227. PR#811 merged as
4cdc878fc4d38c2d3b3e60de6a6ff616359c1ddb; remote #807 is verified CLOSED and the
primary checkout is synchronized to that main commit. Dedicated EQ cuts and the
shared live prepared-control path are delivered to engine main. No npm publication
or app deployment is claimed: #808 builtin filters, #809 SDK release, adapter#111
and app#222 remain. The completed clean/pushed #807 worktree is eligible for removal;
checked evidence is upstream and local artifact/log copies remain outside it.
