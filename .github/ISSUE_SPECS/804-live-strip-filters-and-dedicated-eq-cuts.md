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
