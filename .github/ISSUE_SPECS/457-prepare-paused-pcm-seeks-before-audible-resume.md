# Prepare paused PCM seeks before audible resume

## Outcome

Provide the narrow SDK-owned PCM consumer preparation needed for a browser host to seek while its AudioContext is suspended, refill current-generation PCM, and resume with correct target PCM on the first quantum. The SDK owns ring layout, consumer indices, epoch acknowledgement and worklet processing. The web adapter owns session readiness; applications must not manipulate ring internals.

## Concrete defect and baseline

App issue misofm/app#101 integrates SDK445 source 11c52271e1f686d15eeee6fce90107fcdec50b5e and adapter34 source 6d448ea8a2507fcabe062e0e52acb833bdd78588. During paused resume the producer publishes a new generation while old ring slots remain full. App occupancy incorrectly accepts those slots as fresh prefill. After unmuting, the worklet discards them before replacement PCM arrives. Actual eight-stem Ghost evidence records 512 stale slots discarded and 16 underruns between pause and resume. Independent Astra review confirms this is a correctness gap; initial attachment-ready is not post-seek readiness.

Preserve the original dirty engine checkout. Work from the exact reviewed SDK445 checkpoint in an isolated branch. Do not include unrelated later engine work.

## Smallest slice and first decision gate

Before freezing a new public API, use existing actual-Wasm PCM parity fixtures and the existing browser PCM evaluator to prove the proposed bounded, consumer-owned suspended preparation mechanism. Fill both the old internal engine queue and all old shared-ring slots, publish a new seek, prepare on the owning worklet control port while the context stays suspended, refill, then require exact target PCM on the first resumed quantum without an underrun. Confirm the control-port handler actually executes while suspended.

A source-seek admission ACK is insufficient: Rust PcmSourceProducer.try_seek queues work consumed by render and may return backpressure. The proof must discriminate old internal queue state and engine-applied seek semantics. Stop and report if the message-only mechanism fails; do not silently render/discard a quantum, move the target frame, hide underruns with mute, or weaken the first-quantum assertion.

If that gate passes, implement one bounded awaited public feed preparation operation using the existing control port and shared seek/stale handling. It releases stale slots only through their owning consumer, retains current-generation work, and reports typed failure/backpressure with explicit close, timeout and supersession behavior. No allocation, lock, I/O, logging or unbounded work is added to process/render. A Rust change, if the decisive proof requires one, needs an amended decision record and review before implementation.

## Expected boundary

SDK browser feed, SDK-owned PCM worklet prelude, public types/exports if needed, existing SDK PCM tests/browser evaluator and this spec. Adapter consumption and app archive adoption are bounded successors under app#101. No new storage backend, progressive playback, codec, generic transport framework, benchmark framework, or unrelated architecture work.

## Acceptance

- Existing real-Wasm first-quantum discriminator is RED on the old behavior and GREEN on the final behavior with old queues full.
- Suspended-context control delivery and preserved target frame are demonstrated in the existing browser gate.
- Current-generation PCM proof is distinct from ring occupancy and initial attachment; stale or superseded preparation cannot grant readiness.
- Close and timeout reject outstanding preparation; typed queue refusal is preserved, never acknowledged before a drop.
- Proportional existing SDK type/headless/package/browser checks pass; no unnecessary full engine rebuild if Rust/engine assets are unchanged. Modified SDK-owned feed bytes receive accurate package provenance.
- Dedicated Astra medium review records PASS before dependent adapter implementation. Root checkpoints exact paths, pushes and synchronizes this GitHub issue; completion requires upstream evidence.

## Execution

User-selected Astra medium implements and a separate Astra medium agent reviews. Root approves this bounded contract; mechanism remains conditional on the first decisive proof. Detailed read-only diagnosis: /private/tmp/dx101-paused-resume-readiness-brief.md. Actual failure evidence: /private/tmp/dx101-ghost-32db3f2.json. No implementation has started.

## First decision gate: message-only preparation blocked

Astra medium ran a bounded actual-Wasm probe using the existing SDK sessionDocument fixture and WasmBoundary, with the reviewed SDK445 artifact from the writer checkout. The bytes compare exactly with the app's installed reviewed module; SHA256 `22e4c25cba7f97b66db720ad8ac8cf653de0afcabe84101693f4fa166b90d4e6`. No Rust rebuild or production modification.

The one-source 48 kHz/128-frame fixture uses a 512-frame internal PCM ring. Four old-generation quanta are accepted; the fifth returns typed backpressure (6), establishing a full internal queue. All 64 shared-ring slots are also filled with old-generation work, then the producer publishes generation 2 at target frame 10000. Actual engine seek admission returns OK, but direct submission of the new target quantum still returns backpressure (6). The first actual render returns zeros; a fresh same-document engine with the same seek and target PCM returns the exact nonzero ramp (left starts 0.00390625, 0.0078125, 0.01171875). Exact first-quantum equality is RED.

Probe `/private/tmp/dx457-first-quantum-probe.mjs`, output `/private/tmp/dx457-first-quantum-probe.log`; command `node /private/tmp/dx457-first-quantum-probe.mjs` exits 1 at the first-target assertion. This narrower ABI probe does not claim worklet control-port delivery or a complete browser gate. It establishes a prerequisite failure even if stale shared slots were ideally released: the full internal queue cannot accept target PCM until render consumes its pending seek. Consequently no public preparation API or worklet implementation was frozen, no output was silently rendered/discarded, and no counters or target frame were changed. Execution stops at the brief's decision gate. A reviewed amendment is required before any Rust/internal-consumer change or alternative mechanism.
