# Expose prepared spectrum hop through browser and headless SDKs

Parent: #763. Depends on delivered native overlap issue #851.

## Product slice

Browser and headless SDK callers may explicitly prepare a native continuous-spectrum hop of 256, 512, 1024, or 2048 frames. The AudioWorklet capture, response Worker analysis, and public stream metadata all honor the same admitted value. Omission preserves the existing engine-derived default profile. Explicit configuration against an old or mismatched required asset fails clearly and never silently runs at the default.

Configuration is a preparation option outside canonical musical session JSON. Environment or deployment configuration may choose an application-side default, but Rust/Wasm receives an ordinary validated number. Runtime subscription cadence remains separate from native capture hop. Runtime hop changes, deployment wiring, application adoption, package release, larger queues, alternative smoothing, and simultaneous jobs are outside this issue.

Preserve all existing C-ABI record sizes, offsets, tags, reserved-zero contracts, exported spellings, and old boot behavior. Add capability/configuration surfaces rather than changing packed records gratuitously. Keep FFT analysis off render. Respect #844's synchronous plain-data boot snapshot: copy and validate the option before any await.

Current source assumptions that must be replaced:

- host-web derives protected cadence independently from only sample rate/quantum;
- Worker-side native analysis reconstructs the default cadence and rejects metadata carrying a different hop;
- existing browser/headless boot types have no explicit preparation hop or old-asset capability rule.

## B1: additive Rust host-web/ABI path

Own `hosts/host-web/src/{lib,ffi}.rs`, corresponding Rust tests, ABI layout generation/validation sources and focused tests, export inventories, and normal generated ABI metadata required by this additive boundary.

- Freeze an additive V1 configuration/boot mechanism carrying requested H while preserving existing boot exports and packed layouts.
- Thread explicit H through ordinary supported capture preparation and protected observation preparation so both use one validated effective cadence.
- Change native Worker analysis validation to accept either the existing derived default or an explicitly supported effective H from trusted stream metadata. Continue rejecting unsupported/forged values and all existing timestamp/channel/snapshot violations.
- Expose an unambiguous capability that SDKs can test before honoring an explicit request.

Gates cover all explicit/default values, invalid refusal before publication, actual metadata H, overlapping-window analyzer acceptance, forged metadata rejection, byte-for-byte old ABI layouts/constants, exact new exports/layout metadata, generator/validator negative controls, and render-callgraph exclusion.

## B2: shared SDK, headless, and Worker support

Depends on B1. Own the shared spectrum/boundary types and staging code, headless engine, response Worker plumbing required for capability/configuration, public type exports, and focused SDK tests.

- Add one preparation-level optional `spectrumHopFrames` copied as plain numeric data and validated to the supported set.
- Carry it through headless boot and Worker analysis; consume the native effective metadata rather than deriving a competing default.
- Before successful explicit boot/stream use, verify that every required supplied asset supports the additive capability. Missing capability returns a typed refusal; never retry through a default-only path.
- Omitted configuration keeps the existing compatibility/default behavior.

Gates cover headless exact starts/spans/sequences/metadata/smoothing for explicit H, omitted default, invalid local values, fake/old assets missing capability, delayed Worker/loss progress, owned arrays, SDK type checks, and focused headless/Worker suites.

## B3: browser factory and AudioWorklet forwarding

Depends on B2. Own the public browser engine factory, authoritative Worklet host/processor scripts and declaration mirrors, narrowly required host adapter code, and focused fake-host/actual-module browser tests.

- Snapshot and validate the option synchronously before any asynchronous module/context/worklet work.
- Carry it through preparation/init transport and invoke the additive native path. Worklet `process()` continues only bounded render/copy/capture work; no FFT is introduced there.
- Preserve acknowledgement, transfer ownership, cleanup, and one-in-flight contracts.

Gates prove public option to native metadata, caller mutation after invocation cannot alter it, invalid shape/value refusal, unsupported asset refusal without stripping the option, omitted legacy initialization, fake-host forwarding, and actual-module behavior.

## B4: integrated artifact qualification and documentation

Depends on B1-B3. Own only normally generated artifact metadata/pins/manifests, minimal additions to existing headless/browser fixtures, `sdk/README.md`, and this issue's evidence. Do not create a new benchmark framework or change deployment/application/release state.

Build one candidate with the normal repository builder and use those exact bytes for headless, Worklet, Worker, export/layout/resource, package, and supported-browser gates. Preserve raw evidence. Update resource fixtures only with explained actual-layout/allocation evidence; PCM changes may not be repinned away.

Required integrated evidence:

- actual headless and browser H256 and H1024, plus omitted-default regression;
- fast capture with slow reads showing truthful drops and bounded recovery;
- effective H preserved Worklet → SDK → Worker → result;
- smoothing agrees with native evidence;
- PCM identity, no tested render-time memory growth, realtime/callgraph gates;
- existing supported-browser harness, generated declarations/assets, package checks, and resource negative controls.

## Execution and delivery

Assign B1-B4 sequentially to one fresh Luna MAX agent each after #851 is delivered and closed. An agent gets only this contract, its task, exact prior checkpoint, and focused commands. Root audits and commits/pushes each coherent checkpoint before the next task starts. Each Luna task has at most two implementation/revision rounds; if still unsatisfactory, escalate to Sol high for one round, then Astra xhigh for one round, then stop and rescope. Record one verdict per round and never weaken gates.

After B4, a fresh Astra medium agent adversarially verifies both issue contracts and the exact integrated artifact candidate. Push PASS evidence, synchronize this body, close this issue, and verify remote closure. Keep #763 open unless its independent parent criteria are all complete.

## Checkpoint evidence

**B1a0 ordinary-capture prerequisite — complete in fresh Luna MAX round 1.** The initial combined
B1 handoff was interrupted after inventory with no edits and does not count as an implementation
attempt. Root reduced the work before any tranche accumulated. The replacement found that
host-core's protected owner accepted a prepared hop, but the ordinary single/collection capture
APIs exposed only the derived default. Its first coherent tranche adds typed
`start_continuous_with_hop` methods to `SpectrumCapture` and `SpectrumCaptureCollection`; both take
an already validated `SpectrumHop`, preserve the old start methods unchanged, and reuse the same
continuous activation path. Focused tests cover all four supported hops on both shapes and pin the
legacy derived cadence. Root reran both focused tests, strict host-core library Clippy, wasm32
no-default compilation, formatting, and diff checks: PASS. B1's host-web ABI/configuration work
will begin from this checkpoint with a fresh agent; Worker analysis remains a separate checkpoint.

**B1a1 host-web internal configuration seam — complete after escalation.** Two fresh Luna MAX
rounds spent their bounded inventory windows without producing an edit, so root stopped them and
escalated this two-file slice exactly as directed. Sol high preserved every existing boot method as
a default wrapper and added internal configured paths carrying `Option<SpectrumHop>` through boot,
resource projection, protected preparation, and ordinary stream start. Protected preparation uses
`HostObservationPreparationConfig`; ordinary single/collection capture uses the typed host-core
start methods; omitted configuration retains the derived cadence. No FFI export, packed record,
SDK, Worker, artifact, or deployment surface changed. Tests cover ordinary H256, collection H1024,
protected H256, and omitted default. Root reran the complete host-web library suite (195 passed,
2 existing ignored), strict host-web library Clippy, wasm32 compilation, formatting, and diff
checks: PASS. The additive FFI/capability boundary remains a fresh bounded task.

**B1a2 additive Wasm boundary — complete in two Luna MAX rounds.** Round 1 added three explicit
exports: capability value 1, ordinary spectrum-hop boot, and protected observation spectrum-hop
boot. Raw values are accepted only through `SpectrumHop::new`; 0 and every unsupported value
refuse before capture preparation, publish no host, return handle 0, carry a bounded invalid-argument
diagnostic, and permit a restaged valid retry. Existing boot exports remain default wrappers and no
packed structure changed. Root review found that the borrow-conflict branch of the invalid-hop
helper could return `RESULT_INTERNAL` as a fake nonzero handle, and that parameter-metadata's
authoritative export inventory remained stale. Luna round 2 fixed the return path, added a focused
borrow-conflict/no-publication test, and updated the canonical inventory from 131 to 134 exports.
Root reran all 201 host-web library tests (2 existing ignored), parameter-metadata's 18 ABI-layout
and 5 round-trip tests, strict host-web library Clippy, wasm32 compilation, the 27-mutation ABI
self-test, formatting, and diff checks: PASS. The agent additionally reports exact 134/134 Wasm
export inspection and render-callgraph PASS. The artifact digest is expected to change and remains
deferred to B4. Worker-side explicit-metadata validation is the remaining B1 subtask.

**B1b Worker-side imported-cadence validation — complete in two Luna MAX rounds.** The native
analysis instance now reconstructs the cadence from the already validated stream metadata with
`SpectrumCadence::with_hop`, so H256, H512, H1024, and H2048 are the only accepted effective
values and no competing default is derived. Existing structure, ABI, status, target, channel,
reserved-zero, epoch, span, capture-header, snapshot-token, and capture-length checks still run
before configuration is committed. Focused tests exercise H256 and H1024 through configuration
and analysis, retain the existing H2048/default path, and prove that zero and H300 refuse without
partially changing stream analysis state. Round 1's implementation and behavior tests passed, but
root's independent audit caught one rustfmt discrepancy; round 2 corrected formatting without a
semantic change. Root reran the focused tests, strict host-web library Clippy, wasm32 compilation,
formatting, and diff checks: PASS. This closes the Rust/ABI B1 slice; SDK capability enforcement
and transport remain B2.

**B2a shared/headless boot contract — complete in two Luna MAX rounds.** `BootOptions` now accepts
the literal preparation value `spectrumHopFrames` for H256, H512, H1024, or H2048. The SDK copies
and validates it synchronously before instantiation, leaves it outside the packed boot-options
record, and selects the additive boot export only after the existing ABI-version guard and an
exact capability value of 1. Omission still uses the legacy export without probing the optional
capability; an old, wrong-capability, or incomplete explicit asset raises a typed asset refusal
and never falls back. Reboot preflights capability before disposing the live session. Round 1
implemented the seam and focused fake/proxy asset coverage. Root review found that its initial
capability probe preceded the documented ABI-version step; round 2 restored ABI-first ordering
and added a regression proving the ABI diagnostic wins without probing capability. Root reran all
eight focused cases, the SDK type/mirror gate, Node syntax checks, and diff checks: PASS. The full
boot eval also passed against the prior qualified artifact in the agent run. Generated ABI and
actual new-artifact qualification remain deferred to B4; Worker preservation is the next bounded
B2 slice.
