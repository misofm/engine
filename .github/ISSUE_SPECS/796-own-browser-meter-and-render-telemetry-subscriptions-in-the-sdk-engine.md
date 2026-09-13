# Own browser meter and render-telemetry subscriptions in the SDK engine

Parent: misofm/engine#763. Release: misofm/engine#794. Downstream:
misofm/engine-web-adapter#95 and misofm/app#210.

## Outcome and source facts

A consumer of `@misofm/engine/browser` can obtain truthful track/master meters
and render telemetry from its `BrowserEngine`, without any source adapter.
The same engine owns the semantic console, managed observations and measurement
leases, and its close terminates them. This is one browser SDK ownership slice.

Inspection baseline: engine main `69c268f240bf30b2a43b43dd521120cd89dcc0b9`
(accepted #793/PR #795); adapter main
`f833303f146de7cbe1705fe88ae68a6d6e0d4e45`. This is source evidence, not a
reported runtime incident or a claim that new tests passed. Adapter
`src/console.ts` currently owns `HostFeed`, maps meters and constructs
`createBrowserConsole(host)`. Its projection drops generation/validity/loss and
changes unavailable master GR to zero. SDK `sdk/src/browser/engine.ts` already
owns `ObservationSubscriptionOwner` and its `console()` installs the
`beforeConsoleSubmit` hook which the adapter-created console omits.

## Frozen contract

1. Add `BrowserEngine.subscribeMeters(listener): Promise<() => void>` and
   `BrowserEngine.subscribeTelemetry(listener): Promise<() => void>`; export their
   callback result types from `@misofm/engine/browser`. Admission resolves only
   after the corresponding host lease succeeds. Unsubscribe is idempotent.
   No console attached means a typed SDK refusal. Existing `console()`, resident
   observations, response and spectrum APIs remain the engine-owned paths.
2. SDK `TrackMeter` has `peakLeft: number`, `peakRight: number` and
   `gainReductionDb: number`. `MasterMeter` has the same peak fields and
   `gainReductionDb: number | null`. SDK `MeterUpdate` has `sequence: bigint`,
   `generation: bigint`, `validity: number`, `lossCount: number`, `windows: number`,
   `firstSample: bigint`, `endSample: bigint`,
   `tracks: ReadonlyMap<string, TrackMeter>` and `master: MasterMeter`.
   Resolve IDs using the SDK's compiled map, never consumer order. Preserve
   original lane values and metadata. Do not add the adapter's presentation
   `peak = max(L,R)` field to the canonical model or convert master `null` to 0.
3. SDK `TelemetryUpdate` contains `sequence: bigint`, `blocks`, `cpuPercent`,
   `peakBlockMs`, `meanBlockMs`, `budgetMs`, `deadlineMisses`, `resolutionMs`
   (all numbers), and `belowResolution: boolean`, retaining the existing host
   meanings. No invented generation/sample span where the host provides none.
   Reuse host decoding/validation and canonical result codes; no duplicate ABI.
4. Document peak amplitudes as linear magnitudes, GR as non-negative dB,
   `[firstSample,endSample)` as the peak window only, and generation/loss/validity
   exactly as `sdk/src/browser/shipped-host.d.ts::MisoMeterFrame` defines them.
   Per-track positional GR still conflates unobserved with zero and folds
   independently aged effects; do not claim per-effect GR timing or an exact
   peak/GR join. Nullable master GR remains distinguishable from measured zero.
5. Move/adapt the small `HostFeed` lease reconciler into the SDK with explicit
   provenance to the adapter SHA above. One meter and one telemetry host lease
   serve all engine subscribers. Keep this beside the current SDK owner; do not
   rewrite `ObservationSubscriptionOwner` into a generic event framework.
   One listener throwing cannot prevent another listener's delivery or break
   ownership. Returned results remain stable across subsequent host frames;
   never expose borrowed host buffers as owned values. Retained state is bounded
   by the current subscribers/results, independent of duration.
6. Integrate disposal with existing `BrowserEngine.close()`: reject new
   subscriptions once closing starts, clear callbacks, settle pending admission
   against close, reconcile a late successful lease, and finish host disposal
   without a late owner being revived. Reuse current host request deadlines and
   close serialization; add no indefinite close wait, polling timer or Worker.
   A lease refusal is a typed SDK error and leaves no subscribed callback or
   owner reservation. Preserve existing engine console conflict coordination.

## Locations and two checkpoints

SDK `sdk/src/browser/engine.ts`, one small browser measurement module,
`sdk/src/browser/index.ts`, `sdk/README.md`; extend the existing browser/console
and observation subscription tests plus `sdk/test/package-tarball-smoke.mjs`.
Use adapter `tests/console.test.ts` as explicitly cited source for its useful
lease-race cases. Do not edit adapter production code under this issue.

1. Implement the public types, projection and shared lease lifecycle; focused
   SDK tests and types pass, then root commits those exact paths.
2. Integrate engine close and the existing console/managed-observation seam;
   add the narrow package-consumer proof and documentation, stop for review.

## Discriminating gates

- Two subscriptions of each kind arm once; first unsubscribe keeps delivery,
  last releases once; repeated unsubscribe/engine close is harmless. Delay the
  arm acknowledgement across close and show no surviving callback/lease. Refuse
  admission and show a later valid subscriber can proceed. Cover one throwing
  listener without suppressing another. Reuse one existing fake host seam.
- Feed a meter frame with reordered stable IDs, asymmetric L/R, nonzero
  generation/loss, explicit validity bits and `masterGrDb: null`; assert every
  field survives and an older result stays unchanged after the next frame.
  Telemetry keeps every host measurement and its units.
- Through the SDK's actual `console()` and managed observation owner, prove an
  existing conflicting manual observation edit is refused/coordinated by the
  current hook. This proves the shared route, not a newly invented runtime bug.
- In the existing fresh packed consumer, import these public types and obtain
  one real SDK meter using the existing direct-PCM/browser fixture, without
  installing the FLAC adapter. Exercise close. Reuse accepted #793 browser
  artifacts/DSP evidence if unchanged; no new multi-browser or DSP matrix.
- Run focused existing SDK tests, TypeScript/generated-surface checks and the
  existing package check against the accepted artifact. Required changed-source
  CI remains binding. No benchmark campaign or forced Wasm rebuild loop.

## Boundaries, workflow and completion

No Rust DSP/Worklet transport rewrite, headless/native parity expansion, codecs,
storage, source policy changes, feed extraction, UI or new plugin abstraction.
Existing #789/#791/#793 acceptance is preserved. Parent #763's remaining
multi-target/discovery/identity/transport promises stay open.

User workflow overrides repository model defaults: Luna XHIGH implements;
a fresh Astra MEDIUM independently verifies and may fix concrete bugs within
this frozen scope only. The Astra MEDIUM coordinator is a different fresh agent
from the verifier. Maximum five coherent attempts, one adversarial verdict per
attempt; after five failures preserve evidence and rebrief, never weaken gates.
Root checkpoints focused-green tranches before layering, integrates current
main, records commands/results and source SHAs, and synchronizes local/GitHub
evidence. Close this source issue only after independent PASS, merged delivery
and required CI with evidence upstream; verify GitHub CLOSED. Registry
publication belongs to #794 and is not implied by source acceptance.

## Decision record

Scope-only at creation. No implementation or fresh runtime result is claimed.
The SDK result and method names above are frozen for downstream implementation.
