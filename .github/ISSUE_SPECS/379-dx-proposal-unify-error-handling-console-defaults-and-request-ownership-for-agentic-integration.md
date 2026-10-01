Companion to #376 (browser host lacks the headless "no console attached" guard) and misofm/engine-web-adapter#9 (adapter boots with no console unless `policy.console` is passed). Those report symptoms. This is the design pass they asked for: it re-verifies every finding against the published `@misofm/engine@0.1.0` bytes and current `main`, corrects one of them, maps the full client boundary, and proposes a concrete API shape plus a short list of engine-side changes so the fourth integration of this engine does not rediscover any of it.

Three integrations exist today: `misofm/app` (hand-rolled, ~256 KB of non-test engine glue plus ~151 KB of stem store), `misofm/website` (277 lines over the adapter), and the SDK's own headless boundary. Getting the website's mixer to drive fader/mute/solo/metering cost a full headless-Chrome debugging session, and every hour of it went to integration footguns rather than the feature. The owner's framing: a developer using the SDK should feel like it is magic; this engine is meant to be driven by agents, so the SDK has to give an agent maximum signal with zero noise.

Everything below is anchored to file:line in the published package (`dist/...`, 0.1.0) or the repos at current `main`. Nothing here changes code. misofm/engine-web-adapter#8 (dense-FLAC rewrite) is in flight; the adapter half of this proposal is filed separately as misofm/engine-web-adapter#11, an input to it, not a competitor.

---

## 1. Verified findings

### 1.1 The console is opt-in, silently, all the way down — confirmed

- `dist/core/abi.d.ts:33`: `/** The four console words. Absent attaches no console at all. */`, all four optional.
- The same interface's own rule for every *other* word (`abi.d.ts:15-22`): "absent means zero, and zero means 'the engine's own default', never a number the SDK invented." The console words are the one exception where zero means "off". #239's `WebBootOptions` repeats it: "all-zero = defaults" for the struct, `0` = no console for those four.
- Adapter forwards `policy` only if supplied (`session.ts:99`); `createEngine` defaults to `{}`; `hosts/host-web/src/lib.rs` `console_request` maps `0` to `None`; the worklet then answers every batch with a **resolved** `{ result: 7, reason: 7 }` = `unsupported` / `unsupportedKind` (`miso-engine-v1-audio-worklet.js:648-663`, comment: "a typed refusal of a well-formed request"). Headless, the identical misconfiguration **throws** `MisoUsageError("this engine booted with no console attached; set console.commandQueueRecords")` (`sdk/src/core/boundary.ts:512`). Same bug, two shapes.
- `host.meters({ enabled: true })` with `meterBlocks = 0` resolves `unsupported` (`lib.rs:1016-1022`).
- The adapter README quickstart never shows `policy`, so the documented path is the broken one.

### 1.2 "The four console words are all-or-nothing" — **not confirmed; the observation was finding 1.3**

The published mapping is `word = (name, value) => BigInt(nonNegative(name, value ?? 0))` (`dist/browser/host-mirror.js:35`) and `writeBootOptions` does `BigInt(options.console?.x ?? 0)` (`sdk/src/core/abi.ts:181-184`). A partial `{ commandQueueRecords, meterBlocks }` encodes byte-identically to the same object with two explicit zeros. Rust `validate_options` (`lib.rs:2463-2500`) has no "all nonzero" rule; its only cross-field rules are `observationTaps != 0 ⟹ commandQueueRecords != 0` and `masterTrackPlusOne != 0 ⟹ observationTaps != 0`, and a violation is refused **at boot** as `refusedOptions` (2, diagnostic `web.options.console`), never as a per-command `invalidArgument`. The "every command fails with result 1 unless all four are set" report in engine-web-adapter#9's follow-up is the request-id collision below, observed on the first submit; I am correcting it there.

What *is* true and still untyped: the two cross-field rules exist in three places with three shapes — SDK `toWebBootOptions` throws `MisoUsageError` (`host-mirror.js:43-50`), the shipped host factory throws a bare `webError(1)` (`host.js:204-218`, `:837`), and Rust refuses with `refusedOptions`. The type says nothing about either rule.

### 1.3 The first host request after boot fails once — root cause found, and it is not Rust

One strictly monotonic request-id ledger, three independent allocators:

- Main-thread host: `host.js:582-585` rejects `requestId <= #lastRequestId` with `webError(1)` before `postMessage`. Its own `status()` (`:655`), `sessionMap()` (`:763`) and `dispose()` (`:813`) allocate `#lastRequestId + 1` internally, and there is no accessor for the counter.
- Worklet: `worklet.js:536-540` re-checks and on failure calls `sticky(RESULT_INVALID_ARGUMENT, ...)`, which sets `ready = false` permanently. Only the main-thread pre-check stands between a stale id and session death.
- SDK console: `dist/browser/console.js:31-35` seeds a closure-local counter from the `sessionMap()` ack and increments privately; its comment names the hazard ("restarting at one would be locally well-typed and rejected by the host's monotonic ledger").
- Public API: `meters()`, `telemetry()`, `observe()`, `command()`, `submitSource()`, `seekSource()` all take `requestId` from the caller (`host.d.ts:638-644`). The SDK wraps none of them except `command()`.

Sequence: `engine.console()` → `sessionMap()` consumes 1. Caller: `host.meters({ requestId: 1 })` → `1 <= 1` → result 1. Retry with 2 → ok, ledger = 2. First `console.submit()` uses `remoteMap.requestId + 1 = 2` → `2 <= 2` → result 1. Retry → 3 → ok. That is exactly "whichever call is first fails once," and why the website's workaround needed a fresh id per attempt. `misofm/app` carries a *third* allocator, a shadow counter with `#consumeRequestId()` and a documented cascade failure (`app/src/lib/mixer/engine/index.ts:1013-1024`); the engine's own qualification harness dodges the ledger with magic ids `10001`/`10002`/`20001` (`hosts/host-web/qualification/qualification.js:311-326`). No wrapper can fix this: every wrapper shares the ledger with the host's internal allocations, so the only correct allocator is the host.

### 1.4 The canonical-PCM conversion trap — quantified, and worse than reported

The adapter wants int16/int24 LE PCM and verifies SHA-256 identity. A caller decoding via `decodeAudioData` gets float32 and converts. The website's shipped fix, `Math.round(s < 0 ? s * 32768 : s * 32767)`, against Chromium's `1/32768` int16→float scale mismatches **16,383 of 65,536 codes** — every positive sample at or above code 16385 (−6.02 dBFS) lands one LSB low. It passes today only because the demo stems peak at −46 dBFS or are silent (measured). Symmetric `Math.round(s * 32768)` has zero mismatches; the app's WebCodecs path refuses any non-integral `value * 2^(depth-1)` outright (`app/src/lib/mixer/stem-store/webcodecs-pcm.ts:65-77`), which is the right rule. The error a caller sees is `stem.corrupt`, three frames from the bug. #8 moves the conversion into the adapter with an exactness gate; the adapter issue names this trap so the rewrite's gate 6 covers it.

### 1.5 Seven "did it work?" shapes, not four — confirmed

| Shape | Kind | Where | Inspectable how |
|---|---|---|---|
| `EngineWebAdapterError { code, details, cause }` | thrown `Error` | adapter | `code` string union |
| `MisoUsageError` | thrown `Error`, **no `code` field** (`errors.ts:118-124`) | SDK core: 60+ sites incl. `submit()` empty batch, `ConsoleWriter` escalation, headless no-console | message string only |
| `MisoEngineError { phase, code, result, diagnostics }` | thrown `Error` | SDK boot/asset/quantum paths | typed |
| `{ tag: "miso.error.v1", requestId, result }` | **rejected frozen plain object, not an `Error`** (`host.js:177-181`); untyped and unexported by the SDK | every host verb; passes through `console.submit()` and `engine.console()` unwrapped (`browser/console.js` has no catch) | bare number; `resultName(result, "call")` exists but is never applied on this path |
| `{ tag: "miso.unsupported.v1", capability }` | rejected plain object | `createMisoAudioWorkletHost` | `capability` |
| `CommandReport { ok: false, ... }`, `MisoAck { result != 0 }`, `EngineCallResult` | **resolved** | `console.submit()`, `host.meters()`, headless `submitSource()` | must check `ok`/`result` |
| `ValidationResult` `{ ok: false, phase, code, diagnostics }` | resolved union | headless `validate()` | typed (this one is right: validation's job is an answer) |

`console.submit()` alone can throw `MisoUsageError`, reject with a plain object, or resolve `ok: false`. The resolved-refusal arm caused a real bug in the website (a refused edit checked for a throw but not `report.ok`). `misofm/app` shows the production cost of the bare-number shape: its normalizer renders `"Mixer engine error (result 6)"` and folds every engine refusal to product code `"decode"` (`app/src/lib/mixer/engine/host.ts:81-104`, `player.ts:350-372`).

The SDK already states the rule it wants, twice, and then breaks it in `submit()`: `core/writer.d.ts` — "a flow-control refusal is never an error… a refusal that is *not* flow control throws… retrying it silently would be an infinite loop wearing the costume of resilience"; `core/agent.d.ts` — "Only *usage* errors — naming a parameter that does not exist — throw." `EngineConsole.submit()` resolves `unknownTrack` as `ok: false`.

### 1.6 Two more engine-level footguns both integrations work around

- The shipped host refuses a non-suspended `AudioContext` with bare `webError(1)` (`host.js:830-837`). A freshly constructed context is not reliably suspended, so the adapter (`session.ts:68`) and the app (`boot.ts:262-279`) both carry `if (context.state === "running") await context.suspend()` inside `createHost`.
- A track id longer than the source-id staging buffer fails boot with `RESULT_INTERNAL` 255 and no diagnostic, because the worklet reads track identities back through that buffer. The app enforces it by hand (`assertTrackIdsFitSourceIdBuffer`, `session-document.ts:175-225`) and names its sources `source-000` to stay under it.

### 1.7 `sources` must be restated by every caller

`SessionShape.sources` (`core/boundary.d.ts:15-19`) carries `id`, `channels`, `frames` but not `content` identity or `bitDepth`, so the adapter cannot derive `DeclaredStemSource[]` from the scratch boot; every caller re-declares what the document says and gets `session.declaration_mismatch` when they drift.

---

## 2. The boundary a client crosses today

| Step | Knowledge the types do not encode | Failure shape if wrong | Proposed default |
|---|---|---|---|
| Capability check | COOP/COEP, OPFS, Web Locks, SIMD all required on the SAB path | `EngineWebAdapterError("capability.*")` — good | keep; add `remedy` naming the header/flag |
| Session document | rate/quantum come from the engine (good, #239); track ids must fit the source-id buffer | `RESULT_INTERNAL` 255, no diagnostic | typed boot diagnostic (3.7) |
| `sources` declaration | must match the document, incl. identity and bit depth | `session.declaration_mismatch` | derive from shape (1.7) |
| `leaseId` | required, non-empty | `TypeError` | random UUID |
| Stem decode | int16/24 LE; scale and rounding on the caller | `stem.corrupt`, unrelated to the bug | adapter-owned (#8), symmetric scale, refuse non-integral |
| `openEngineWebSession` | `policy.console` or nothing works; taps ⟹ queue, master ⟹ taps | resolved `unsupported/unsupportedKind`, or `refusedOptions` at boot | console + meters attached by default; `ConsolePolicy \| false` |
| AudioContext handoff | host demands `state === "suspended"` | bare `webError(1)` | host suspends internally (3.7) |
| Console attach | the console shares a request ledger with the host and the caller | first call result 1 once | host-owned ids (3.3) |
| Command submission | three outcome shapes; one batch in flight else `backpressure`; `ConsoleWriter` exists but is unreachable from `EngineConsole` | silent `ok: false` | one thrown `MisoError`; console built on the writer |
| Metering | separate `meterBlocks` word; caller `requestId`; refusal is a resolved `MisoAck` | resolved `result: 7` | `mix.meters.subscribe(cb)` |
| Teardown | `close()` idempotent — good | — | keep |

Eleven steps; eight need knowledge that only exists in someone else's source.

---

## 3. Proposal

### 3.1 One error envelope, one rule

**The rule** (the SDK's own, from `writer.ts` and `agent.ts`, applied everywhere): a refusal that cannot succeed on retry **throws**; an answer that is the engine working as designed is a **typed resolved value whose type cannot express failure**. Flow control (`backpressure`) is absorbed inside the SDK's writer, so the semantic console never resolves a refusal and never rejects with anything but `MisoError`. `validate()` keeps its result union — validation's product *is* the refusal.

```ts
export type MisoErrorCode =
  | "capability.cross_origin_isolation" | "capability.shared_array_buffer" | "capability.audio_worklet"
  | "capability.opfs" | "capability.web_locks" | "capability.simd128" | "capability.webcodecs_flac"
  | "boot.refused_document" | "boot.refused_options" | "boot.abi_mismatch" | "boot.memory_budget"
  | "boot.context_not_suspended" | "boot.track_id_too_long"
  | "console.not_attached" | "console.unknown_track" | "console.unknown_parameter" | "console.domain"
  | "console.malformed" | "console.unsupported_kind" | "console.observation_unbound" | "console.meters_not_attached"
  | "host.backpressure" | "host.invalid_request" | "host.disposed" | "host.sticky" | "host.internal"
  | "stem.not_found" | "stem.corrupt" | "stem.invalid_declaration" | "stem.quota" | "stem.read_deadline"
  | "usage.empty_batch" | "usage.batch_too_large" | "usage.closed";

export class MisoError extends Error {
  readonly code: MisoErrorCode;
  /** "capability" | "asset" | "boot" | "stem" | "host" | "console" | "usage" */
  readonly phase: ErrorPhase;
  /** One sentence naming the exact fix, composed from generated vocabulary. Never empty. */
  readonly remedy: string;
  /** Engine facts when there are any: result, reason, rejectedIndex, diagnostics, requestId. */
  readonly details: Readonly<Record<string, unknown>>;
  /** `true` when the same call can succeed later (backpressure, read deadline). */
  readonly transient: boolean;
  readonly cause?: unknown;
  toJSON(): { code: MisoErrorCode; phase: ErrorPhase; message: string; remedy: string; details: object; transient: boolean };
}
```

1. Every public function in `.`, `./browser`, `./headless` and the adapter throws `MisoError` or returns a success value. `MisoUsageError`, `MisoEngineError`, `EngineWebAdapterError` become `MisoError` with a `phase` (keep the old names as deprecated subclasses for one release; `MisoUsageError` finally gets a `code`).
2. The shipped host's `miso.error.v1` / `miso.unsupported.v1` objects are wrapped at `browser/host-mirror.ts` and never escape. `host-core/src/lib.rs:14-17` already says this is the SDK's job: "The C ABI and the browser ABI number their results differently, so the facade returns typed values and each host maps them onto its own frozen numbering."
3. `EngineConsole` is built on `ConsoleWriter`. Today the writer is exported from the root but unreachable through `EngineConsole` or the adapter, which is why the website and the app each re-implemented latest-wins coalescing. `submit()` keeps its transactional, sample-exact semantics and throws `MisoError` for every non-`ok` report (`host.backpressure` with `transient: true` for a caller who wants raw transactions; the writer-backed `track.*`/`set()` verbs never surface it). The resolved type loses its failure arm:

```ts
export interface CommandReport { readonly ok: true; readonly admitted: number; readonly appliedAtSample: bigint; }
```

This revises #207's "resolved, never thrown — a refusal is an answer". That remains right for the transport (`MisoAck` inside the host, the render-side admission value) and for `validate()`. It is wrong at the semantic console, where a resolved `unknownTrack` is a silently dropped caller bug — the website shipped exactly that.

4. Messages name the fix. What the SDK should literally produce:
   - `console.not_attached`: "This session booted with `console: false`. Omit `policy.console` to attach the default console (64 command records, 12 meter blocks), or pass a `ConsolePolicy`."
   - `console.unknown_track`: "No track 'bas' in this session. Tracks: bass, drums, keyboard, percussion, vocals."
   - `capability.cross_origin_isolation`: "`crossOriginIsolated` is false. Serve with `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`."
   - `boot.track_id_too_long`: "Track id 'lead-vocal-double-take-2' is 24 bytes; the source-id buffer holds 10. Shorten the id."

### 3.2 Console defaults and the four words

```ts
export interface ConsolePolicy {
  /** Default ABI_LAYOUT.constants.defaultCommandQueueRecords (64). */
  readonly commandQueueRecords?: number;
  /** Default ABI_LAYOUT.constants.defaultMeterBlocks (12). */
  readonly meterBlocks?: number;
  /** Default 0. Nonzero requires commandQueueRecords (typed at the SDK, not discovered at boot). */
  readonly observationTaps?: number;
  /** Default 0. Nonzero requires observationTaps. */
  readonly masterTrackPlusOne?: number;
}
export interface BootOptions {
  readonly requireSampleRateHz?: number;
  readonly requireQuantumFrames?: number;
  readonly sourceRingFrames?: number;
  readonly maximumMemoryBytes?: bigint;
  /**
   * Absent: attach a console with the engine defaults. `false`: none (playback-only).
   * An object: override words; every word not named takes its default, never zero.
   */
  readonly console?: ConsolePolicy | false;
}
```

The SDK normalizes to the fully-specified fixed-size struct before `writeBootOptions`; the struct stays fixed-size (it sizes the per-track control queue, the meter fold and the tap lanes — real preallocation, `lib.rs:129-141`, `prepare.rs:275-286`). "Off" becomes a visible `false` in code review and in the type. The two cross-field rules are checked once, in the SDK, with `MisoError("boot.refused_options")` and a remedy — not three times in three shapes.

ABI (fold into #239's `WebBootOptions`): make the console words obey the struct's own "all-zero = defaults" principle — `0` = engine default, opt-out is an explicit `console_flags` word (bit 0 `CONSOLE_DISABLED`) in the slot #239 already reserves. That deletes the one exception in the struct's semantics and makes ABI, SDK and adapter agree that the default session has a console.

Reason code (#376 item 2): `COMMAND_REASON_NO_CONSOLE`, so no-console never reads as `unsupportedKind`. Same for the meter lease: `RESULT_UNSUPPORTED` with no reason becomes distinguishable as `console.meters_not_attached` at the SDK.

### 3.3 Request ids: the host owns the ledger — engine-side, `hosts/host-web/web`

- `MisoAudioWorkletHost` allocates every request id. `command()`, `meters()`, `telemetry()`, `observe()`, `submitSource()`, `seekSource()` drop `requestId` from their request types; acks keep it for correlation.
- Worklet: a stale id is a per-request `RESULT_INVALID_ARGUMENT`, not `sticky()`. A misnumbered message must not kill a live session; the render side is unaffected because the check is JS before any export call.
- SDK `browser/console.ts` deletes its counter; the app deletes its shadow counter; the qualification harness deletes its magic numbers.

The gates that pin `host.js` are additive here — no wire tag changes; request records lose a field on the way in.

### 3.4 Zero-config entry, escape hatches intact

`openEngineWebSession(options)` keeps its name and full option surface (`policy`, `assets`, `createHost`, `createContext`, `store`, `createPump`, `createOutput`, `scratchBoot`); only defaults and result types change.

```ts
import { openEngineWebSession } from "@misofm/engine-web-adapter";

const mix = await openEngineWebSession({
  document,                                                              // string | Uint8Array | { toJson() }
  flac: { locate: (identity) => `${cdn}/${identity.slice(7)}.flac` },    // per #8; `resolver` stays as the PCM escape hatch
});
await mix.play();                                                        // inside the user gesture
await mix.console.track("bass").fader(-6);                               // throws MisoError on refusal
mix.console.set({ tracks: { bass: { mute: true } } });                   // coalesced desired state (3.5)
const stop = mix.meters.subscribe((frame) => draw(frame.peaks));         // no requestId, no retry
await mix.close();
```

Defaults: console with engine defaults; meters attached, lease taken on first `subscribe` and released on last unsubscribe; `leaseId` random; `sources` derived from the scratch boot once 3.7 item 5 lands (until then the adapter reads them from the canonical-JSON document it already holds as text — the adapter is not bound by the SDK's no-parser ruling, and #338 makes the document canonical JSON); every capability failure carries `remedy`.

```ts
export interface EngineWebSession {
  readonly shape: SessionShape;
  readonly state: "ready" | "playing" | "paused" | "closed";
  readonly console: EngineConsole;
  readonly meters: MeterFeed;        // { subscribe(cb): () => void; latest(): MeterFrame | null }
  readonly telemetry: TelemetryFeed;
  play(): Promise<void>; pause(): Promise<void>; seekFrames(frame: number | bigint): Promise<void>;
  close(): Promise<void>;
  /** Escape hatches, typed and documented as such. */
  readonly host: MisoAudioWorkletHost; readonly context: EngineAudioContext; readonly output: AudioNode;
}
```

### 3.5 Declarative surface for the builtin strip — recommended, narrowly, on top of the writer

Add `console.set(desired)` as a **complement** to the imperative console, scoped to the builtin strip (fader, mute, solo, pan; per track and master). Do not build a declarative graph or effect-parameter reconciler.

Why it earns its place: UI-driven strip control is exactly last-write-wins with stale intermediates dropped. The SDK already ships that policy as `ConsoleWriter` (`core/writer.ts`: latest-wins keyed by address, halving on backpressure, escalation on caller bugs), and both non-headless integrations re-implemented it because it is not wired into `EngineConsole` — website `wireMixEvents`, app `console-writer.ts` (16 KB). `set()` is a `MixState → LaneEdit[]` diff (under 200 lines) staged into the existing writer; it removes the most-duplicated code in the three integrations and makes `backpressure` disappear from the caller's vocabulary.

Why it stays narrow: the imperative path is what a scripting agent needs — `appliedAtSample` is sample-exact, a transaction can mix heterogeneous edits, effect parameters are typed by the catalog and the lattice (`core/agent.ts`). A declarative layer over effect parameters would re-express the whole catalog as state and give up the transaction boundary; that would be ceremony around a fundamentally imperative, timing-sensitive channel.

```ts
export interface StripState { readonly faderDb?: number; readonly mute?: boolean; readonly solo?: boolean; readonly pan?: { left: number; right: number } }
export interface MixState { readonly tracks?: Readonly<Record<string, StripState>>; readonly master?: Pick<StripState, "faderDb" | "mute"> }
export interface EngineConsole {
  /** Imperative, transactional, sample-exact. Throws MisoError on any refusal. */
  submit(...edits: LaneEdit[]): Promise<CommandReport>;
  track(id: string): TrackConsole;            // fader(), mute(), solo(), pan(), effect(i).set(); writer-backed
  /** Declarative, coalesced, last-write-wins. Resolves when the newest desired state is applied. */
  set(desired: MixState): Promise<void>;
  /** Last engine-acknowledged strip state; what a UI binds to. */
  readonly applied: MixState;
  settle(): Promise<void>;
}
```

`set()` on an unknown track throws `console.unknown_track` before anything is staged.

### 3.6 Effect: not in `@misofm/engine`; internal-only in the adapter, and only because #8 already pays for it

Measured against the app's installed `effect@4.0.0-rc.112` with `bun build --minify --target=browser`: the smallest lifecycle usage (`acquireRelease` + `scoped` + `runPromise`) is **86.6 KB minified / 29.6 KB gzipped**; with `HttpClient`/`FetchHttpClient`, **112 KB / 37.8 KB**. The adapter's entire dist is 27.6 KB unminified; the SDK's browser+core JS is 148 KB unminified. #324 removed an Effect entry before first publication: "reintroducing an official adapter requires a concrete product consumer."

- `@misofm/engine` (core, browser, headless): **no**. The lifecycle is five ordered steps with reverse cleanup that already exist and are tested (#324 kept those tests). None of the bugs in this issue are lifecycle bugs; they are contract bugs (a default, a type, a shared counter, a rounding rule) that structured concurrency does not catch. Zero runtime dependencies is a product contract.
- `@misofm/engine-web-adapter`: **internal-only**, and only if #8 lands Effect for the FLAC HTTP pipeline as the owner required. Once paid for, `Scope`/`acquireRelease` for the open sequence (scratch worker → store lease → engine → feed → pump → output → console) is a fair replacement for the hand-maintained `cleanup[]` array. Public API stays plain Promise; `EngineWebSessionOptions`, `EngineWebSession` and `MisoError` are the whole contract.
- Effect types at the public boundary: **no**. It doubles the model an agent must learn; #324's reasoning holds.

### 3.7 Engine-side changes, explicitly

Things no wrapper can or should paper over, in priority order:

1. **Host-owned request ids** (`hosts/host-web/web/*-host.js`, `.d.ts`) — 3.3. Root cause of 1.3.
2. **Stale id is per-request, not sticky** (`*-worklet.js`) — 3.3.
3. **`COMMAND_REASON_NO_CONSOLE`** (`hosts/host-web/src/lib.rs`) — #376 item 2.
4. **Console words obey zero-means-default** with an explicit disable flag in #239's `WebBootOptions` — 3.2.
5. **`SourceShape` gains `content` and `bitDepth`** via the #219 introspection exports — 1.7; lets the adapter drop `sources` and delete `session.declaration_mismatch` as a caller-reachable state.
6. **Host suspends the context itself** (or accepts a running one and suspends before `addModule`) — 1.6; deletes the same workaround from two repos.
7. **Track-id length is a boot diagnostic** (`track.id.exceeds_source_id_buffer` at `$.tracks[i].id`), not `RESULT_INTERNAL` — 1.6.
8. **Browser host gets the headless guard** (#376 item 1) — superseded by defaults, but keep it for `console: false` so `session.console` throws `console.not_attached` at first touch.
9. **`enginectl describe`** — today the CLI has exactly one command (`session build`). A `describe` that prints `describe()`'s JSON (catalog, ABI constants, error codes with remedies, supported rates/depths) is the enumerable-capabilities affordance an agent reaches for first.

Not proposed: any change to the render plane, `PreparedRenderPlan`, the fixed-size console struct, or the wire tags. The three result shapes (`MisoAck`, `miso.error.v1`, `CommandReport`) are historical accretion at the JS boundary, not a realtime constraint: all three are produced on the main thread after the worklet posts, and `process()` never sees them. Unifying them at the SDK boundary is sufficient; a wire-level unification is not worth a tag bump.

### 3.8 Agentic affordances

- `describe()` (#207) plus `describeSession(mix)`: tracks, sources, attached budgets, and every `MisoErrorCode` with its remedy. An agent should never grep a repo to learn what a session can do.
- `MisoError.toJSON()` is the whole recovery contract: `transient` says whether to retry, `remedy` says what to change, `details.diagnostics` carries the engine's dotted codes and paths.
- `preflight()` (#207) reports `{ code, severity, message, remedy }` per finding; COI is a blocker only for the SAB path.
- Both READMEs open with the zero-config path in 3.4 and nothing else; escape hatches get their own section. An `llms.txt` generated from `describe()` ships in the SDK package.

### 3.9 Collapse the three integrations

The stem store exists in three copies: `engine/hosts/host-web/web/stem-store` (97 KB JS), `app/src/lib/mixer/stem-store` (151 KB non-test TS, including the FLAC ingest worker/pool that #8 lifts into the adapter), and the adapter's `src/stems` (copied from engine `bd7f330a` per its spec 001). The boot/feed/console glue exists twice (app `engine/`, 256 KB; adapter). Position:

- The adapter is the single owner of browser stem delivery, verification, pumping and session lifecycle. After #8 plus this proposal, `app` migrates `engine/{boot,sab-feed,sab-ring,console-writer,real-engine-realm}.ts` and `stem-store/*` onto `openEngineWebSession({ flac })`. What stays app-owned is product policy: `locate` (URL/auth), UI state, attestation display.
- `engine/hosts/host-web/web/stem-store` is deleted once the adapter is the consumer of record; the engine ships worklet, host, wasm and generated metadata — nothing that fetches or stores.
- No new package, no new layer. Every change here is a default, a type, or an ownership move inside packages that already exist.

---

## 4. Sequencing and gates

1. **SDK**: `MisoError` + boundary wrapping; `ConsolePolicy | false` with defaults; `EngineConsole` on `ConsoleWriter`. Gates: a default-booted browser session's first `submit()` succeeds with no `policy`; `console: false` throws `console.not_attached` at first touch; every rejection reaching a caller is `instanceof MisoError` (red mutation: reintroduce one raw `webError` pass-through).
2. **Engine host**: host-owned ids, non-sticky stale id. Gates: a fresh session's first `meters()` and first `command()` succeed in either order with no retry (red mutation: restore caller ids); a bypassing caller's duplicate id refuses one request and the next succeeds.
3. **Engine**: reason code, `SourceShape` fields, `console_flags`, context suspend, track-id diagnostic, `enginectl describe`.
4. **Adapter** (input to #8, filed separately): defaults, `MixState`, `meters.subscribe`, error normalization, README.
5. **App migration** (3.9).

Agent blind test (mirrors #207 E17): a fresh agent given only the two packed tarballs and their READMEs must open a session from a document and a FLAC locator, move a fader, read a meter frame, and correctly explain one provoked `console.unknown_track` using only the thrown error. Any failure is an SDK defect, not an agent failure.

---

## 5. Cross-references

- #376 — subsumed by 3.1, 3.2, 3.7 (items 3, 8)
- misofm/engine-web-adapter#9 — subsumed by 3.2, 3.4; its follow-up's "all four words required" claim is corrected in 1.2
- misofm/engine-web-adapter#8 — the adapter half of this proposal is filed as an input to it: misofm/engine-web-adapter#11
- #207 — adopts `describe()`/`preflight()`; 3.1 revises "resolved, never thrown" for the semantic console only
- #239 — 3.2 asks the console words to follow the struct's own zero-means-default rule
- #324 — upheld by 3.6
- #219 — 3.7 item 5 extends the introspection exports
