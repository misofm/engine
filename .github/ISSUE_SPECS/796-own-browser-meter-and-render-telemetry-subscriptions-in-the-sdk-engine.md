# Own browser meter and render-telemetry subscriptions in the SDK engine

Parent: misofm/engine#763. Release: misofm/engine#794. Downstream:
misofm/engine-web-adapter#95 and misofm/app#210.

## Outcome and source facts

A consumer of `@misofm/engine/browser` can obtain truthful track/master meters
and render telemetry from its `BrowserEngine`, without any source adapter.
The same engine owns the semantic console, managed observations and measurement
leases, and its close terminates them. This is one browser SDK ownership slice.

Inspection baseline: engine main `69c268f240bf30b2a43b43dd521120cd89dcc0b9`
(accepted misofm/engine#793/PR #795); adapter main
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
  installing the FLAC adapter. Exercise close. Reuse accepted misofm/engine#793 browser
  artifacts/DSP evidence if unchanged; no new multi-browser or DSP matrix.
- Run focused existing SDK tests, TypeScript/generated-surface checks and the
  existing package check against the accepted artifact. Required changed-source
  CI remains binding. No benchmark campaign or forced Wasm rebuild loop.

## Boundaries, workflow and completion

No Rust DSP/Worklet transport rewrite, headless/native parity expansion, codecs,
storage, source policy changes, feed extraction, UI or new plugin abstraction.
Existing misofm/engine#789/misofm/engine#791/misofm/engine#793 acceptance is preserved. Parent misofm/engine#763's remaining
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
publication belongs to misofm/engine#794 and is not implied by source acceptance.

## Decision record

Scope-only at creation. No implementation or fresh runtime result is claimed.
The SDK result and method names above are frozen for downstream implementation.

## Fresh coordinator entry point and complete issue map

This issue is the entry point for the user's requested cross-repository handoff.
Deliver the seven scoped issues below, with the independent PCM/storage work
ordered after the plotting path unless it is ready without delaying that path.
Do not implement the previously proposed three adapter analysis wrappers.

| Work | Authoritative issue | Dependency |
| --- | --- | --- |
| SDK measurement/console ownership | https://github.com/misofm/engine/issues/796 | Accepted engine misofm/engine#793 |
| Exact SDK release | https://github.com/misofm/engine/issues/794 | misofm/engine#796 PASS, merged source and required CI |
| Adapter uses/exposes the same SDK engine | https://github.com/misofm/engine-web-adapter/issues/95 | misofm/engine#796 API; misofm/engine#794 registry for final publication |
| App direct SDK measurements and EQ visualizer | https://github.com/misofm/app/issues/210 | Published SDK and adapter pair |
| SDK generic PCM readiness | https://github.com/misofm/engine/issues/797 | Existing SDK ring/feed; independent of misofm/engine#796 |
| Adapter adopts SDK readiness | https://github.com/misofm/engine-web-adapter/issues/101 | misofm/engine#797 and its published SDK before final acceptance |
| Backend-specific capability checks | https://github.com/misofm/engine-web-adapter/issues/102 | Independent of measurement/plotting work |

Primary order: misofm/engine#796 → misofm/engine#794 → adapter misofm/engine-web-adapter#95 → app misofm/app#210. Accepted exact tarballs
may support downstream source preparation after independent API PASS; final
pins, publication and deployment use verified registry packages. Do not publish
the old 0.2.5 first and require another release solely for misofm/engine#796: 0.2.5 was still
unused at scoping, so reuse it if a fresh registry check confirms that. Already
accepted siblings may share a frozen release cut, but plotting never waits for
the PCM/backend issues. If misofm/engine#797 needs a later package, create its ordinary
bounded release issue before claiming availability; source PASS is not npm
publication. No extra codec/backend or new release framework is authorized.

The requested coordinator is a fresh Astra MEDIUM agent. It delegates each
implementation to Luna XHIGH and each independent verification to another,
fresh Astra MEDIUM agent; the coordinator does not substitute for the verifier.
Verification fixes concrete bugs within the brief and does not expand scope.
Keep one active launch-critical implementation tranche, commit focused-green
exact paths before layering, preserve current main, use required/proportional
gates and synchronize remote evidence/closure. Deploying the final app main and
necessary package publication were already authorized by the user.

Source state at this scoping checkpoint: engine main
`69c268f240bf30b2a43b43dd521120cd89dcc0b9`, adapter main
`f833303f146de7cbe1705fe88ae68a6d6e0d4e45`, app main
`7d876e2569b3f40c416663e4bf780fe41e2ee328`; registry latest SDK 0.2.4,
adapter 0.5.4. These are baselines to refresh, not pins against future main.
Engine misofm/engine#793 is delivered and CLOSED: PR #795 and merged-main required CI
34788089349 passed, including the existing three-browser qualification. Engine
misofm/engine#763 remains OPEN for its larger stated scope; do not silently absorb or close
that program as part of this handoff.

Scoping changed documentation only. In the shared workspace, engine specs are
on `codex/sdk-adapter-boundary-spec` at `/tmp/miso-engine-sdk-boundary-spec`;
adapter specs are on `codex/engine-analysis-forwarding` at
`/tmp/miso-adapter-engine-analysis-integration`, incorporating current main;
app specs are on local `codex/engine-analysis-integration` at
`/tmp/miso-app-engine-analysis-integration`, incorporating current main. The app
scope checkpoint `0cd7eaf` has not been pushed: its AGENTS requires lint/typecheck/test/
build before any push, and no app implementation/testing is claimed by scoping.
The GitHub issue body is synchronized and sufficient in a fresh environment.
Audit/preserve these branches and any unpushed work before starting. Leave the
unrelated dirty primary app untouched. The completed spectrum feature worktree
has been removed after all commits were merged/pushed and evidence preserved;
no old temporary worktree or audio daemon is a prerequisite. Each repo's current
AGENTS and issue bodies remain the implementation authority.


## Execution evidence — first checkpoint

Fresh coordinator audit on 2026-09-13: all seven local issue bodies and titles
match their OPEN GitHub issues; every numbered local spec has a remote issue.
Refreshed main remains engine `69c268f240bf30b2a43b43dd521120cd89dcc0b9`,
adapter `f833303f146de7cbe1705fe88ae68a6d6e0d4e45`, app
`7d876e2569b3f40c416663e4bf780fe41e2ee328`. Registry latest is SDK 0.2.4 and
adapter 0.5.4; SDK 0.2.5 is unused. Handoff branches include current main;
eight unpushed app scope commits and unrelated dirty work are preserved.
Audit inventory is retained outside the worktrees at `/tmp/miso-796-audit`.

Luna XHIGH implemented the first browser-only checkpoint: canonical owned meter
and telemetry projections, public exports/methods and shared lease reconciliation.
Root reran `node --experimental-strip-types --test sdk/test/measurement-evals.mjs`
(5/5), `bash scripts/check-sdk-types.sh` (PASS, locked SDK dependencies), and
`git diff --check` (PASS). Logs: `/tmp/miso-796-audit/measurement-checkpoint1.log`
and `/tmp/miso-796-audit/types-checkpoint1.log`. Headless boundary changes were
removed as outside this issue. This checkpoint is intentionally incomplete:
engine-close integration, console-owner regression, packed browser proof, final
independent Astra MEDIUM verdict and merged CI remain required.


Second Luna XHIGH checkpoint adds engine-close invalidation, independent
reservations for duplicate callback functions, pending-admission close refusal,
typed host-result error translation, public documentation and packed consumer
coverage. Root reran the current measurement suite (10/10), the locked SDK
typecheck and diff whitespace check (all PASS); logs are
`/tmp/miso-796-audit/measurement-checkpoint2.log` and
`/tmp/miso-796-audit/types-checkpoint2.log`. Fresh Astra MEDIUM read-only feedback
identified the duplicate-callback and close/admission edges during implementation.
Root paused implementation at this focused-green recovery boundary before more
work. Packed-browser/generated/full-SDK results and complete independent verdict
are not yet claimed; Astra now verifies the stable candidate and may fix only
concrete bugs within this frozen scope. This remains coherent attempt 1.


Astra MEDIUM verification checkpoint (attempt 1, no final verdict): independent
full SDK 241/241 and types PASS. Review fixed a late-arm fixture that incorrectly
fed two tracks to a one-track engine, added complete span/window assertions and
telemetry/close-delivery cases; focused measurements now pass 12/12. The packed
browser meter proof still times out after a bounded sample-zero prequeue
correction; that unsuccessful fixture diagnosis is preserved, not accepted as
a passing browser proof. Root checkpoints both test changes as useful, buildable
evidence. Logs: `/tmp/miso-796-audit/verify-sdk-full.log`, `verify-types.log`,
`verify-measurements.log`, and `verify-package1.log`. No runtime correction or
registry availability is claimed by this checkpoint.


## Independent attempt 1 verdict — PASS

Fresh Astra MEDIUM verifier (separate from coordinator and Luna XHIGH implementer)
records **PASS** on `bc771b9ad1c94af6df6d93a28baf10cadf78d778` plus the final README
metadata-semantics clarification. Full independent record and preserved failure
logs: `/tmp/miso-796-audit/verify-796-attempt1.md`.

Independent `check-sdk-headless.sh /tmp/issue793-candidate1-artifact` passed 241/241
on the final runtime; the final focused measurement suite passed 12/12 after two
additional regressions. Typecheck passed. The final
`MISO_ENGINE_SDK_BROWSER_TOOLS=/tmp/miso-app-engine-analysis-integration/node_modules
bash scripts/sdk-package.sh check /tmp/issue793-candidate1-artifact` passed its
artifact preflight, generated surface, package build, enginectl, fresh strict typed
consumer and Vite/Chromium checks (`verify-package-final.log`). `git diff --check`
passed. The real packed SDK meter had sequence/generation 1, validity 3, loss 0,
windows 1, peak span [0,128), track/master L/R .25/.5, track GR 0, master GR null,
and awaited close completion. No FLAC adapter was installed.

Verifier fixed the late-arm test's invalid frame shape and added missing metadata
and telemetry/close assertions. The packed fixture timeout was independently
explained by its default centered pan mixing lanes: first-frame actual peaks were
both 0.1767766923, at sample 0 with a suspended context. The meter-only fixture now
sets an explicit identity matrix and queues PCM before resume; no DSP/product
change or weakened assertion was needed. Earlier failed fixture runs are preserved
alongside the successful result. Approved adapter `tests/console.test.ts` at
`f833303f146de7cbe1705fe88ae68a6d6e0d4e45` supplied the explicitly reviewed lease-race
source cases. The actual engine console/managed-observation hook regression passes.

Accepted #793 Wasm reused without rebuild, independently verified SHA-256
`c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
Required merged-main CI, evidence commit delivery and GitHub closure remain root
responsibilities; this source PASS is not #794 registry publication or #763 closure.


## Combined source cut

The independently accepted #797 helper was ready before release qualification
and merged without conflicts into the accepted #796 candidate at
`f78d599e0a51b49daa0d5def7c01e9b8a8aaeb41`. Refreshed main remains
`69c268f240bf30b2a43b43dd521120cd89dcc0b9` and is already an ancestor. Root's
combined SDK gate passes 248/248; types, generated/package and real packed
Vite/Chromium meter/boot/seek checks pass using the unchanged accepted artifact.
Logs: `/tmp/miso-796-audit/combined-types.log`, `combined-sdk.log`,
`combined-package.log`. The 0.2.5 release input set is accepted #789/#791/#793
plus #796 and #797; no further sibling work is needed for this cut. Required
PR/merged-main CI and #794 immutable registry publication remain pending.


## Source delivery — CLOSED

PR #798 merged at `1646a6a1bd0011cc2b5480283bf498b27be460e5`. Final PR
qualification run 34791858639 and merged-main required qualification run
34792194652 both PASS. Independent attempt-1 evidence is upstream in the
merged source. This source issue is complete; exact 0.2.5 registry availability
remains separately gated by #794. The larger #763 program remains OPEN.
