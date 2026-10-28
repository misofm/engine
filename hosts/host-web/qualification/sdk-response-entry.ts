import { WasmBoundary } from "../../../sdk/src/core/boundary.ts";
import { EngineLiveControls } from "../../../sdk/src/core/live-controls.ts";
import { ControlTransactionQueue } from "../../../sdk/src/core/control-queue.ts";
import { createBrowserControlListTransport, createBrowserControlQueue, createBrowserLiveControls, createBrowserStripOrder } from "../../../sdk/src/browser/live-controls.ts";
import {
  MESSAGE_SESSION_TRANSACTION_APPLY, WIRE_BYTES, WIRE_U32, WIRE_U8, commandFrame, concat, controlCallFailure,
  decodeControlReply, decodeObservationReadReply, decodeRefusalBody, encodeObservationRead,
  encodeObserveTransaction, decodeResponseEnvelope, field, scalar, u32,
} from "../../../sdk/src/core/control-frame.ts";
import type { ObservationFrameSelection, ObserveEdit } from "../../../sdk/src/core/control-frame.ts";
import { ProtocolClient } from "../../../sdk/src/core/protocol-messages.ts";
import { decodeResponseReady, decodeResponseTagReply, encodeStripResponseRequest } from "../../../sdk/src/core/response-frame.ts";
import type { MisoAudioWorkletHost, MisoStatus } from "../../../sdk/src/browser/shipped-host.d.ts";
import { MisoEngineAsset } from "../../../sdk/src/core/asset.ts";
import { CATALOG } from "../../../sdk/src/generated/catalog.ts";
import { effect, session } from "../../../sdk/src/core/session.ts";
import { MisoEngineError } from "../../../sdk/src/core/errors.ts";
import type { ObservationReadResult } from "../../../sdk/src/core/observation.ts";
import type { TrackResponseResult } from "../../../sdk/src/core/live-response.ts";
import { createResponsePreview } from "../../../sdk/src/browser/response.ts";
import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { createDefaultHost } from "../../../sdk/src/browser/default-host.ts";
import { decodeSpectrumCaptureReply, decodeSpectrumStreamReply } from "../../../sdk/src/core/spectrum-frame.ts";
import { spectrumMetadataOf } from "../../../sdk/src/core/spectrum.ts";
import { PROTOCOL_BLOCKS, protocolCalls } from "./protocol-calls.mjs";

// The core's largest spectrum capture budget (`host_core::SPECTRUM_MAXIMUM_CAPTURE_BYTES`).
const SPECTRUM_MAXIMUM_CAPTURE_BYTES = 1 << 20;

const EQ_CONFIGURATION_ID = 9_007_199_254_740_993n;
/**
 * The sample a transaction submitted before any render applies from. A `SET_CONTROL` transaction
 * takes effect in the first block that begins after its submit returns, so an edit sent before the
 * first quantum is in force from sample 0.
 */
const APPLIED_BEFORE_FIRST_SAMPLE = 0;
const OBSERVATION_FRAMES = 2_048;
const SPECTRUM_FRAMES = 2_048;
const CONTINUOUS_FRAMES = 6_144;
const PEAK_FREQUENCY_HZ = 750;
const PEAK_LEFT_AMPLITUDE = 0.1;
const PEAK_RIGHT_AMPLITUDE = 0.05;
const PEAK_EQ_GAIN_DB = 6;

const SPECTRUM_QUERIES = [
  {
    target: { kind: "trackPostInput" as const, trackId: "track" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  },
  {
    target: { kind: "trackPostPan" as const, trackId: "track" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  },
  {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  },
];

const SPECTRUM_COLLECTION_ENTRIES = [
  {
    target: { kind: "trackPostPan" as const, trackId: "track-a" },
    channels: "both" as const,
  },
  {
    target: { kind: "trackPostPan" as const, trackId: "track-b" },
    channels: "left" as const,
  },
];

const SPECTRUM_COLLECTION = {
  entries: SPECTRUM_COLLECTION_ENTRIES,
  maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES * 2,
};

/**
 * Issue #1476 D1-D3: every SDK engine this entry creates, and its render-locked allocator count.
 *
 * `createSdkEngine` is the only place this file calls `createEngine`, and `closeSdkEngine` the
 * only way it closes an engine: it reads `browser.host.renderAllocationCount()` and then closes.
 * The rows are in read order; `instances` counts every engine that booted, so an instance closed
 * without its read leaves the two lengths unequal.
 */
type SdkEngine = Awaited<ReturnType<typeof createEngine>>;
type SdkEngineOptions = Parameters<typeof createEngine>[0];
const sdkRenderAllocations = {
  instances: 0,
  rows: [] as { readonly workload: string; readonly count: number }[],
  /** Issue #1332 S9: each booted engine's `instanceMode`, in boot order. */
  instanceModes: [] as string[],
  open: new Map<SdkEngine, { readonly workload: string; failure?: object }>(),
  /** The first error each instance's body threw, kept after the instance leaves `open`. */
  failures: new WeakMap<SdkEngine, object>(),
};

/**
 * Create, record and connect one SDK engine (D1). A failed connect still reads and closes the
 * instance before its error propagates (D3).
 */
async function createSdkEngine(workload: string, options: SdkEngineOptions): Promise<SdkEngine> {
  const browser = await createEngine(options);
  sdkRenderAllocations.instances += 1;
  sdkRenderAllocations.instanceModes.push(browser.instanceMode);
  sdkRenderAllocations.open.set(browser, { workload });
  try {
    browser.host.node.connect(browser.context.destination);
  } catch (error) {
    const failure = sdkEngineFailed(browser, error);
    await closeSdkEngine(browser);
    throw failure;
  }
  return browser;
}

/** A thrown value as an object, so a later read or cleanup failure can be attached to it (D3). */
function sdkFailureObject(error: unknown): object {
  return error !== null && (typeof error === "object" || typeof error === "function")
    ? error
    : new Error(`non-object error: ${String(error)}`, { cause: error });
}

type SdkFailureKey = "renderAllocationReadFailures" | "sdkCleanupFailures";

/**
 * The read and cleanup failures that followed a thrown value. A thrown value is never written to:
 * the SDK's own errors are frozen, and a write to one throws and hides the failure being reported.
 */
const sdkFailureAttachments = new WeakMap<object, Record<SdkFailureKey, string[]>>();

function attachSdkFailure(failure: object, key: SdkFailureKey, message: string) {
  let attached = sdkFailureAttachments.get(failure);
  if (attached === undefined) {
    attached = { renderAllocationReadFailures: [], sdkCleanupFailures: [] };
    sdkFailureAttachments.set(failure, attached);
  }
  attached[key].push(message);
}

/**
 * The value to throw for `failure`: itself when nothing followed it, else a new `Error` that keeps
 * its message and name, carries it as `cause` (and its stack as `causeStack`, since the report
 * spreads own properties) and lists what followed. Call it once, after every instance has closed.
 */
function reportSdkFailure(failure: object): object {
  const attached = sdkFailureAttachments.get(failure);
  if (attached === undefined) return failure;
  const wrapper = new Error(sdkErrorMessage(failure), { cause: failure });
  const name = (failure as { name?: unknown }).name;
  wrapper.name = name === undefined ? "Error" : String(name);
  const stack = (failure as { stack?: unknown }).stack;
  return Object.assign(wrapper, {
    causeStack: typeof stack === "string" ? stack : undefined,
    renderAllocationReadFailures: attached.renderAllocationReadFailures,
    sdkCleanupFailures: attached.sdkCleanupFailures,
  });
}

function sdkErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  const message = (error as { message?: unknown } | null)?.message;
  if (typeof message === "string") return message;
  if (error !== null && typeof error === "object") {
    try {
      return JSON.stringify(error, (_key, value) => typeof value === "bigint" ? `${value}n` : value) ?? String(error);
    } catch {
      return Object.prototype.toString.call(error);
    }
  }
  return String(error);
}

/**
 * Record the first error an instance's body threw, so its close cannot mask it (D3), and return
 * the value to throw: the error itself, or an `Error` that wraps a non-object throw.
 */
function sdkEngineFailed(browser: SdkEngine, error: unknown): object {
  const failure = sdkFailureObject(error);
  if (!sdkRenderAllocations.failures.has(browser)) sdkRenderAllocations.failures.set(browser, failure);
  const instance = sdkRenderAllocations.open.get(browser);
  if (instance !== undefined && instance.failure === undefined) instance.failure = failure;
  return failure;
}

/** The bound on the `suspend()` before a read, the same 10 s as this file's `resume()` bounds. */
const sdkSuspendTimeoutMs = 10_000;

/**
 * Read the instance's count after its last render and before its close, then close it (D2).
 * A running `AudioContext` is suspended first, so no render follows the read; an offline
 * context has finished rendering. A second call for an instance this helper already closed
 * does nothing. After a recorded body error, a failed read or close is attached to that error
 * instead of thrown, so the body's error stays the one that propagates. A `suspend()` that does
 * not settle within `sdkSuspendTimeoutMs` is a failed read that names the workload: no read, no
 * row, no retry.
 */
async function closeSdkEngine(browser: SdkEngine): Promise<void> {
  const instance = sdkRenderAllocations.open.get(browser);
  if (instance === undefined) return;
  sdkRenderAllocations.open.delete(browser);
  let readFailure: Error | undefined;
  try {
    const context = browser.context as unknown as BaseAudioContext;
    if (typeof AudioContext !== "undefined" && context instanceof AudioContext && context.state === "running") {
      let suspendTimer: ReturnType<typeof setTimeout> | undefined;
      try {
        await new Promise<void>((resolve, reject) => {
          suspendTimer = setTimeout(() => reject(new Error(
            `audio suspend timed out after ${sdkSuspendTimeoutMs} ms (state=${context.state})`,
          )), sdkSuspendTimeoutMs);
          context.suspend().then(resolve, reject);
        });
      } finally {
        if (suspendTimer !== undefined) clearTimeout(suspendTimer);
      }
    }
    const reply = await browser.host.renderAllocationCount();
    if (reply.result !== 0) {
      readFailure = new Error(`${instance.workload}: render allocation count refused: ${reply.result}`);
    } else {
      sdkRenderAllocations.rows.push({ workload: instance.workload, count: reply.count });
    }
  } catch (error) {
    readFailure = new Error(`${instance.workload}: render allocation count failed: ${sdkErrorMessage(error)}`,
      { cause: error });
  }
  let closeFailure: Error | undefined;
  try {
    await browser.close();
  } catch (error) {
    closeFailure = new Error(`${instance.workload}: close failed: ${sdkErrorMessage(error)}`, { cause: error });
  }
  if (instance.failure !== undefined) {
    if (readFailure !== undefined) attachSdkFailure(instance.failure, "renderAllocationReadFailures", readFailure.message);
    if (closeFailure !== undefined) attachSdkFailure(instance.failure, "sdkCleanupFailures", closeFailure.message);
    return;
  }
  if (readFailure !== undefined) {
    if (closeFailure !== undefined) attachSdkFailure(readFailure, "sdkCleanupFailures", closeFailure.message);
    throw readFailure;
  }
  if (closeFailure !== undefined) throw closeFailure;
}

/**
 * The `finally` of an instance's body: run its other cleanups (subscription and stream closes),
 * then read and close the instance whatever they did (D3). The first cleanup error becomes the
 * instance's failure when its body did not throw, and is thrown after the close. When the body
 * threw, every cleanup error is attached to the body's error, also when the instance has already
 * left `open`, so a cleanup error never replaces the body's error.
 */
async function finishSdkEngine(browser: SdkEngine, ...cleanups: Array<() => unknown>): Promise<void> {
  const recordedFailure = () =>
    sdkRenderAllocations.open.get(browser)?.failure ?? sdkRenderAllocations.failures.get(browser);
  const bodyFailed = recordedFailure() !== undefined;
  let cleanupFailure: object | undefined;
  for (const cleanup of cleanups) {
    try {
      await cleanup();
    } catch (error) {
      const failure = recordedFailure();
      if (failure === undefined) {
        cleanupFailure = sdkEngineFailed(browser, error);
      } else {
        attachSdkFailure(failure, "sdkCleanupFailures", sdkErrorMessage(error));
      }
    }
  }
  await closeSdkEngine(browser);
  if (!bodyFailed && cleanupFailure !== undefined) throw cleanupFailure;
}

/**
 * The run's backstop (D3): read and close every instance still open when the run ends, whatever
 * path it took. On the error path, `failure` is the run's error and every read or close failure
 * is attached to it; on the success path an instance left open here has no row in the returned
 * rows, so the gate's length check is red.
 */
async function closeOpenSdkEngines(failure: object | undefined): Promise<void> {
  let firstError: unknown;
  for (const [browser, instance] of [...sdkRenderAllocations.open]) {
    if (failure !== undefined && instance.failure === undefined) instance.failure = failure;
    try {
      await closeSdkEngine(browser);
    } catch (error) {
      firstError ??= error;
    }
  }
  if (firstError !== undefined) throw firstError;
}

function observationSelection(effectSlotId: string, channels: "left" | "right" | "both" = "both") {
  return { trackId: "track", rack: "inserts" as const, effectSlotId, tapId: 1, channels };
}

function observationPlanes(block: number): Float32Array[] {
  const left = new Float32Array(128);
  const right = new Float32Array(128);
  left.fill(0.5 + block * 0.001);
  right.fill(0.5 + block * 0.001);
  return [left, right];
}

function observationRowKey(row: ObservationReadResult): string {
  const window = row.window;
  return JSON.stringify([
    row.effectSlotId,
    row.channels,
    row.status,
    row.left,
    row.right,
    window?.firstSample.toString(),
    window?.endSample.toString(),
    window?.sequence.toString(),
    window?.blocks,
  ]);
}

function observationRowsEqual(
  left: readonly ObservationReadResult[],
  right: readonly ObservationReadResult[],
): boolean {
  return left.length === right.length && left.every((row, index) => {
    const other = right[index];
    return other !== undefined && observationRowKey(row) === observationRowKey(other);
  });
}

function serializeObservationRows(rows: readonly ObservationReadResult[]) {
  return rows.map((row) => ({
    effectSlotId: row.effectSlotId,
    nativeEffectId: row.nativeEffectId,
    channels: row.channels,
    status: row.status,
    left: row.left,
    right: row.right,
    window: row.window === undefined ? null : {
      firstSample: row.window.firstSample.toString(),
      endSample: row.window.endSample.toString(),
      sequence: row.window.sequence.toString(),
      blocks: row.window.blocks,
    },
  }));
}

/** Keep the browser probe on the same compressor/gate fixture as the headless evals. */
function observationDocumentWithGate(raw: string): Uint8Array {
  const document = JSON.parse(raw);
  const track = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  const gate = CATALOG.effects.find((candidate) => candidate.id === "miso.gate-expander");
  if (track === undefined || gate === undefined) throw new Error("observation gate fixture is unavailable");
  const gateEntry = {
    id: "gate",
    identity: { kind: "native", effect_id: gate.id },
    quality: "normal",
    bypass: false,
    link_mode: "dual_mono",
    params: gate.parameters.map((parameter) => ({
      parameter_id: parameter.id,
      channel: "both",
      unit: parameter.unitName,
      value: parameter.default,
    })),
    sidechain: { kind: "none" },
  };
  track.inserts.effects = [
    ...track.inserts.effects.filter((entry: { id?: string }) => entry.id !== "gate"),
    gateEntry,
  ];
  return new TextEncoder().encode(JSON.stringify(document));
}

function responseArrayEqual(left: ArrayLike<number> | undefined, right: ArrayLike<number> | undefined): boolean {
  if (left === undefined || right === undefined) return left === undefined && right === undefined;
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index += 1) {
    if (left[index] !== right[index]) return false;
  }
  return true;
}

function responseMemberKey(member: TrackResponseResult["members"][number]): string {
  return JSON.stringify([
    member.trackId,
    member.nativeId,
    member.stableId,
    member.rack,
    member.rackValue,
    member.slot,
    member.kind,
    member.kindValue,
    member.bypassed,
    member.available,
    member.excludedReason,
    member.enabledLeft,
    member.enabledRight,
  ]);
}

function responseValuesEqual(left: TrackResponseResult, right: TrackResponseResult): boolean {
  return left.trackId === right.trackId
    && left.mode === right.mode
    && left.meaning === right.meaning
    && left.sampleRateHz === right.sampleRateHz
    && left.floorDb === right.floorDb
    && responseArrayEqual(left.frequenciesHz, right.frequenciesHz)
    && responseArrayEqual(left.leftDb, right.leftDb)
    && responseArrayEqual(left.rightDb, right.rightDb)
    && left.excludedMemberCount === right.excludedMemberCount
    && left.members.length === right.members.length
    && left.members.every((member, index) => responseMemberKey(member) === responseMemberKey(right.members[index]!));
}

function serializeTrackResponse(result: TrackResponseResult) {
  return {
    trackId: result.trackId,
    mode: result.mode,
    meaning: result.meaning,
    sampleRateHz: result.sampleRateHz,
    points: result.frequenciesHz.length,
    frequencies: Array.from(result.frequenciesHz),
    left: result.leftDb === undefined ? undefined : Array.from(result.leftDb),
    right: result.rightDb === undefined ? undefined : Array.from(result.rightDb),
    members: result.members.map((member) => ({
      nativeId: member.nativeId,
      stableId: member.stableId,
      rack: member.rack,
      slot: member.slot,
      kind: member.kind,
      available: member.available,
      bypassed: member.bypassed,
      excludedReason: member.excludedReason,
      enabledLeft: Array.from(member.enabledLeft),
      enabledRight: Array.from(member.enabledRight),
    })),
    capturedSample: result.capturedSample.toString(),
    snapshotToken: result.snapshotToken.toString(),
    resultBytes: result.resultBytes.toString(),
  };
}

/**
 * The spectrum operation a request frame names (#1596): its message (`0x0011` capture, `0x0012`
 * stream) and its first field's one-byte value, the operation code, at byte 56 (the 48-byte header,
 * then the field's 8-byte prefix), as `spectrum-frame.ts` encodes them. `undefined` for another frame.
 */
function spectrumOperation(frame: Uint8Array): { readonly message: number; readonly code: number } | undefined {
  const message = (frame[16] ?? 0) | ((frame[17] ?? 0) << 8);
  if (message !== 0x0011 && message !== 0x0012) return undefined;
  return { message, code: frame[56] ?? 0 };
}

const CAPTURE_ARM = 1;
const STREAM_START = 1;
const STREAM_READ = 3;

/**
 * Watch the spectrum frames the SDK posts through `host.control()` (#1596): `before` may hold a
 * frame before it is posted, and `after` sees each served reply. The SDK reads spectrum through
 * these frames alone, so this is where the native stream's metadata is seen.
 */
function watchSpectrumFrames(host, hooks: {
  readonly before?: (operation: { readonly message: number; readonly code: number }) => Promise<void> | void;
  readonly after?: (
    operation: { readonly message: number; readonly code: number },
    response: Uint8Array | undefined,
  ) => void;
}) {
  const control = host.control.bind(host);
  host.control = async (frame: Uint8Array, document?: Uint8Array) => {
    const operation = spectrumOperation(frame);
    if (operation !== undefined) await hooks.before?.(operation);
    const reply = await control(frame, document);
    if (operation !== undefined) hooks.after?.(operation, reply.result === 0 ? reply.response : undefined);
    return reply;
  };
}

/** The native metadata of a served `SPECTRUM_STREAM` reply, or `undefined` for a refusal. */
function streamReplyMetadata(response: Uint8Array | undefined) {
  if (response === undefined) return undefined;
  const reply = decodeSpectrumStreamReply(response);
  return reply.ok ? spectrumMetadataOf(reply.value) : undefined;
}

function spectrumPlanes(block: number): Float32Array[] {
  const left = new Float32Array(128);
  const right = new Float32Array(128);
  left.fill(0.25 + block * 0.002);
  right.fill(-0.5 - block * 0.001);
  return [left, right];
}

function spectrumTargetKey(target: typeof SPECTRUM_QUERIES[number]["target"]): string {
  return target.kind === "output"
    ? `${target.kind}:${target.outputId}`
    : `${target.kind}:${target.trackId}`;
}

async function createSpectrumBrowser(
  query: typeof SPECTRUM_QUERIES[number],
  workload = `spectrum-query:${spectrumTargetKey(query.target)}`,
) {
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createSdkEngine(workload, {
    document,
    spectrum: query,
    policy: { sourceRingFrames: SPECTRUM_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: SPECTRUM_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(SPECTRUM_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, SPECTRUM_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  return browser;
}

function spectrumDocument(raw: string, frames: number, peak = false): Uint8Array {
  const document = JSON.parse(raw);
  const source = document.sources?.find((candidate: { id?: string }) => candidate.id === "live-control-source");
  const track = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  if (source === undefined || track === undefined) throw new Error("spectrum fixture is unavailable");
  source.frames = String(frames);
  if (peak) {
    for (const lane of [track.builtins?.left, track.builtins?.right]) {
      if (lane === undefined) throw new Error("spectrum peak fixture has no builtins");
      lane.hpf_hz = 0;
      lane.lpf_hz = 0;
      lane.delay_samples = 0;
      lane.trim_db = 0;
      lane.polarity_invert = false;
    }
    // The first console slot is the EQ; its knobs are this track's entry for that slot.
    const slot = document.console?.pre_insert?.find((entry: { identity?: { effect_id?: string } }) =>
      entry.identity?.effect_id === "miso.parametric-eq");
    const eq = track.console?.find((entry: { slot?: string }) => entry.slot === slot?.slot);
    const definition = CATALOG.effects.find((candidate) => candidate.id === "miso.parametric-eq");
    if (eq === undefined || definition === undefined) throw new Error("spectrum peak EQ fixture is unavailable");
    const names = new Map([
      ["band-1-enabled", 1],
      ["band-1-kind", 1],
      ["band-1-frequency", PEAK_FREQUENCY_HZ],
      ["band-1-gain", PEAK_EQ_GAIN_DB],
    ]);
    eq.params = definition.parameters
      .filter((parameter) => names.has(parameter.name) || parameter.name === "band-1-q")
      .map((parameter) => ({
        parameter_id: parameter.id,
        channel: "both",
        unit: parameter.unitName,
        value: names.get(parameter.name) ?? parameter.default,
      }));
    // Only that EQ stays: no inserts, and no post_insert slot (nor its entry).
    track.inserts.effects = [];
    document.console.pre_insert = [slot];
    document.console.post_insert = [];
    track.console = [eq];
  }
  return new TextEncoder().encode(JSON.stringify(document));
}

/** Build the same known-signal fixture with two prepared, differently tuned track boundaries. */
function spectrumCollectionDocument(raw: string, frames: number): Uint8Array {
  const document = JSON.parse(new TextDecoder().decode(spectrumDocument(raw, frames, true)));
  const source = document.sources?.find((candidate: { id?: string }) => candidate.id === "live-control-source");
  const original = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  const route = document.routes?.[0];
  if (source === undefined || original === undefined || route === undefined) {
    throw new Error("spectrum collection fixture is unavailable");
  }
  const trackA = structuredClone(original);
  const trackB = structuredClone(original);
  trackA.id = "track-a";
  trackB.id = "track-b";
  // The input is the same 750 Hz signal for both tracks; this trim makes a selected B
  // window distinguishable without changing the capture boundary or adding a DSP fixture.
  for (const lane of [trackB.builtins?.left, trackB.builtins?.right]) {
    if (lane === undefined) throw new Error("spectrum collection track has no builtins");
    lane.trim_db = -6;
  }
  document.tracks = [trackA, trackB];
  document.routes = [
    { ...structuredClone(route), id: "track-a-main", source: {
      ...structuredClone(route.source), track_id: "track-a",
    } },
    { ...structuredClone(route), id: "track-b-main", source: {
      ...structuredClone(route.source), track_id: "track-b",
    } },
  ];
  return new TextEncoder().encode(JSON.stringify(document));
}

async function createContinuousSpectrumBrowser(
  query,
  frames = CONTINUOUS_FRAMES,
  peak = false,
  spectrumHopFrames?: 256 | 512 | 1024 | 2048,
  live = false,
) {
  const raw = await (await fetch("/qualification/observation-session.json")).text();
  const document = spectrumDocument(raw, frames, peak);
  const workload = `spectrum-continuous-hop-${spectrumHopFrames ?? "default"}`;
  const browser = await createSdkEngine(workload, {
    document,
    spectrum: query,
    policy: {
      sourceRingFrames: frames,
      liveControls: { meterBlocks: 16 },
      ...(spectrumHopFrames === undefined ? {} : { spectrumHopFrames }),
    },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: frames,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(frames) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = live
        ? new AudioContext({ sampleRate: 48_000, latencyHint: "interactive" })
        : new OfflineAudioContext(2, frames, 48_000);
      // An engine may create a later context already running after this page's first trusted
      // resume (Firefox did; it is not a supported browser since #2092). Keep a suspension pending
      // across addModule so the shipped host observes its required suspended preparation state,
      // matching the collection qualification below.
      if (live) void (context as AudioContext).suspend();
      else Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  return browser;
}

/**
 * Wait until `read()` reports `required` items, bounded by `deadlineMs` of wall-clock time and
 * never by a poll count (#1106). `pump()` can return at once, so each poll is followed by a
 * `pollMs` timer: the loop yields to the event loop and the audio clock instead of spinning, and
 * a slow audio start only costs time. Returns what it saw so a failure can name it.
 */
async function waitForCount(options: {
  readonly read: () => number;
  readonly pump: () => Promise<unknown>;
  readonly required: number;
  readonly deadlineMs: number;
  readonly pollMs: number;
}) {
  const began = performance.now();
  let polls = 0;
  while (options.read() < options.required && performance.now() - began < options.deadlineMs) {
    await options.pump();
    polls += 1;
    await new Promise((resolve) => globalThis.setTimeout(resolve, options.pollMs));
  }
  return { count: options.read(), polls, elapsedMs: performance.now() - began };
}

function streamMetadataFields(metadata) {
  if (metadata === undefined) return undefined;
  return {
    status: metadata.status,
    hopFrames: metadata.hopFrames,
    sampleRateHz: metadata.sampleRateHz,
    quantumFrames: metadata.quantumFrames,
    smoothingMs: metadata.smoothingMs,
    capturedSample: metadata.capturedSample.toString(),
    endSample: metadata.endSample.toString(),
    sequence: metadata.sequence.toString(),
    windows: metadata.windows.toString(),
    droppedCaptures: metadata.droppedCaptures.toString(),
  };
}

async function createSpectrumCollectionBrowser() {
  const raw = await (await fetch("/qualification/observation-session.json")).text();
  const frames = 16 * SPECTRUM_FRAMES;
  const document = spectrumCollectionDocument(raw, frames);
  const browser = await createSdkEngine("spectrum-collection", {
    document,
    spectrumCollection: SPECTRUM_COLLECTION,
    policy: { sourceRingFrames: frames },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: frames,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(frames) }],
      tracks: ["track-a", "track-b"],
      submixes: [],
    }),
    createContext: () => {
      const context = new AudioContext({ sampleRate: 48_000, latencyHint: "interactive" });
      // Keep the known first capture at sample zero even when autoplay is permitted.
      void context.suspend();
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  return browser;
}

async function submitSpectrumSource(browser, frames = SPECTRUM_FRAMES, planeForBlock = spectrumPlanes) {
  for (let block = 0; block < frames / 128; block += 1) {
    const acknowledgement = await browser.host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * 128),
      sampleRateHz: 48_000,
      planes: planeForBlock(block, block * 128),
      frames: 128,
      endOfRegion: block === frames / 128 - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("SDK spectrum source submission refused");
  }
}

function toneSpectrumPlanes(block: number, startFrame: number): Float32Array[] {
  const scale = block < 2 * SPECTRUM_FRAMES / 128 ? 1 : 0.5;
  return [PEAK_LEFT_AMPLITUDE * scale, PEAK_RIGHT_AMPLITUDE * scale].map((amplitude) =>
    Float32Array.from({ length: 128 }, (_, index) => amplitude * Math.sin(
      2 * Math.PI * PEAK_FREQUENCY_HZ * (startFrame + index) / 48_000,
    )));
}

function collectionSpectrumPlanes(_block: number, startFrame: number): Float32Array[] {
  return [PEAK_LEFT_AMPLITUDE, PEAK_RIGHT_AMPLITUDE].map((amplitude) =>
    Float32Array.from({ length: 128 }, (_, index) => amplitude * Math.sin(
      2 * Math.PI * PEAK_FREQUENCY_HZ * (startFrame + index) / 48_000,
    )));
}

function spectrumPeak(values: Float32Array): { readonly index: number; readonly value: number } {
  let index = 0;
  for (let candidate = 1; candidate < values.length; candidate += 1) {
    if (values[candidate]! > values[index]!) index = candidate;
  }
  return { index, value: values[index]! };
}

async function runContinuousSpectrumQualification(): Promise<Record<string, unknown>> {
  const query = {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  };
  const browser = await createContinuousSpectrumBrowser(query, CONTINUOUS_FRAMES, true, 256);
  const nativeStarts = [];
  const nativeReads = [];
  // Issue #1480 D2: the capture loss is made certain by construction, not by a race. While the
  // offline context renders, no native read runs: a read the SDK starts then waits on
  // `readsHeld` (a promise, not a sleep), and every read already dispatched has settled before
  // `startRendering` is called. The render therefore completes all
  // (CONTINUOUS_FRAMES - 2,048) / 256 + 1 = 17 hop-256 windows into the native stream's one-record
  // queue with no pop between them: the first is queued and the other 16 are dropped. The first
  // read after the render must report that gap (`renderLoss`), and the SDK's first ready
  // notification must carry it. A read that popped mid-render (the race #1480 D1 found) leaves
  // fewer than 16 drops and the `renderLoss` predicate red.
  const inFlightReads = new Set<Promise<unknown>>();
  let readsHeld: Promise<void> | undefined;
  let releaseReads = () => {};
  let settleRead: (() => void) | undefined;
  watchSpectrumFrames(browser.host, {
    before: async ({ message, code }) => {
      if (message !== 0x0012 || code !== STREAM_READ) return;
      if (readsHeld !== undefined) await readsHeld;
      // In flight from its post until its reply is seen (`after`); the owner sends one at a time.
      inFlightReads.add(new Promise<void>((resolve) => { settleRead = resolve; }));
    },
    after: ({ message, code }, response) => {
      if (message !== 0x0012) return;
      const metadata = streamReplyMetadata(response);
      if (code === STREAM_START && metadata !== undefined) nativeStarts.push(metadata);
      if (code === STREAM_READ) {
        if (metadata !== undefined) nativeReads.push(metadata);
        settleRead?.();
        inFlightReads.clear();
      }
    },
  });
  const meterFrames: Array<{ readonly peaks: Float32Array; readonly firstSample: bigint; readonly endSample: bigint }> = [];
  const automaticNotifications = [];
  let callbackCount = 0;
  let callbackGap = false;
  let recoveryDelivery: Record<string, unknown> | undefined;
  let callbackResolve: (() => void) | undefined;
  const callbackSeen = new Promise<void>((resolve) => { callbackResolve = resolve; });
  let subscription;
  let shared;
  try {
    const meterLease = await browser.host.meters({
      enabled: true,
      onFrame: (frame) => meterFrames.push(frame),
    });
    if (meterLease.result !== 0) throw new Error("continuous spectrum meter lease refused");
    // A track response is captured in a rendered block (#1579b): armed now, read after the render.
    const responseQuery = await armTrackResponse(browser, {
      trackId: "track",
      grid: { kind: "linear", points: 1_025, minimumHz: 0, maximumHz: 24_000 },
      channels: "both",
      responseLimits: { maximumResultBytes: 1 << 20, requestDeadlineMs: 10_000 },
    });
    subscription = await browser.subscribeSpectrum({
      ...query,
      smoothingMs: 0,
      cadenceMs: 1,
      onUpdate: (notification) => {
        automaticNotifications.push(notification);
        callbackCount += 1;
        callbackGap ||= notification.nativeMissedWindows > 0n || notification.skippedPublications > 0n;
        if (recoveryDelivery === undefined && notification.status === "ready" && notification.available) {
          const result = notification.handle.readLatest();
          recoveryDelivery = {
            status: notification.status,
            available: notification.available,
            nativeMissedWindows: notification.nativeMissedWindows.toString(),
            skippedPublications: notification.skippedPublications.toString(),
            notificationCapturedSample: notification.metadata.capturedSample.toString(),
            notificationEndSample: notification.metadata.endSample.toString(),
            resultCapturedSample: result?.capturedSample.toString() ?? null,
            resultEndSample: result?.endSample.toString() ?? null,
          };
        }
        callbackResolve?.();
      },
    });
    shared = await browser.subscribeSpectrum({ ...query, smoothingMs: 0, cadenceMs: 100 });
    const pendingBeforeRender = subscription.readLatest() === undefined;
    const sharedJob = shared.job === subscription.job
      && shared.owner === subscription.owner && shared.epoch === subscription.epoch;
    await submitSpectrumSource(browser, CONTINUOUS_FRAMES, toneSpectrumPlanes);
    readsHeld = new Promise<void>((resolve) => { releaseReads = resolve; });
    await Promise.allSettled([...inFlightReads]);
    const readsBeforeRender = nativeReads.length;
    let rendered: AudioBuffer;
    try {
      rendered = await browser.context.startRendering();
    } finally {
      releaseReads();
    }
    const response = await responseQuery.query;
    const firstPcmPeak = Math.max(
      ...[rendered.getChannelData(0), rendered.getChannelData(1)].map((plane) => {
        let peak = 0;
        for (const value of plane) peak = Math.max(peak, Math.abs(value));
        return peak;
      }),
    );
    // Both waits share one wall-clock deadline (the hop probe's 10 s), so a slow runner that
    // delivers late still passes while a subscription that never delivers fails in bounded time.
    const deadlineMs = 10_000;
    const deadline = performance.now() + deadlineMs;
    let automaticTimer: ReturnType<typeof setTimeout> | undefined;
    const automaticDelivery = await Promise.race([
      callbackSeen.then(() => true),
      new Promise<boolean>((resolve) => { automaticTimer = setTimeout(() => resolve(false), deadlineMs); }),
    ]);
    clearTimeout(automaticTimer);
    // Both automatic reads and explicit pumps invoke onUpdate. Keep its live list while waiting
    // for a ready/available publication with a result present through readLatest(). pump() can
    // return at once, so yield to the event loop between pumps and stop only at the deadline.
    const notifications = automaticNotifications;
    for (;;) {
      if (recoveryDelivery !== undefined && subscription.readLatest() !== undefined) break;
      const notification = await subscription.pump();
      if (notification?.status === "ready" && notification.available
        && subscription?.readLatest() !== undefined) break;
      if (performance.now() >= deadline) break;
      await new Promise((resolve) => setTimeout(resolve, 5));
    }
    const first = subscription.readLatest();
    if (first === undefined) throw new Error("continuous spectrum did not publish a window");
    const firstLeft = first.leftDb;
    const firstRight = first.rightDb;
    if (firstLeft === undefined || firstRight === undefined) throw new Error("continuous spectrum omitted a channel");
    const effectiveHopFrames = subscription.bounds.hopFrames;
    const leftPeak = spectrumPeak(firstLeft);
    const rightPeak = spectrumPeak(firstRight);
    const sharedResult = shared.readLatest();
    if (sharedResult === undefined) throw new Error("shared continuous spectrum had no result");
    const original = sharedResult.leftDb?.[0];
    if (original === undefined || subscription.readLatest()?.leftDb?.[0] !== original) {
      throw new Error("continuous spectrum did not preserve owned arrays");
    }
    sharedResult.leftDb![0] = original + 100;
    const ownedArrays = subscription.readLatest()?.leftDb?.[0] === original;
    const secondNotification = notifications.at(-1);
    const gap = secondNotification?.status === "gap"
      || secondNotification?.nativeMissedWindows > 0n
      || secondNotification?.skippedPublications > 0n
      || callbackGap;
    const readyNotification = notifications.find((notification) => notification.available);
    const firstReadAfterRender = nativeReads[readsBeforeRender];
    const sharedAfterFirstClose = (await subscription.close(), shared.readLatest() !== undefined);
    await shared.close();
    let staleReadRefused = false;
    try { shared.readLatest(); } catch { staleReadRefused = true; }
    const meter = meterFrames.find((frame) => frame.peaks.length >= 4);
    const expectedLeft = PEAK_LEFT_AMPLITUDE * 10 ** (PEAK_EQ_GAIN_DB / 20);
    const expectedRight = PEAK_RIGHT_AMPLITUDE * 10 ** (PEAK_EQ_GAIN_DB / 20);
    const expectedLeftDb = 20 * Math.log10(PEAK_LEFT_AMPLITUDE) + PEAK_EQ_GAIN_DB;
    const expectedRightDb = 20 * Math.log10(PEAK_RIGHT_AMPLITUDE) + PEAK_EQ_GAIN_DB;
    return {
      pendingBeforeRender,
      sharedJob,
      callbackCount,
      automaticDelivery,
      windows: notifications.length,
      statuses: notifications.map((notification) => notification.status),
      sequences: notifications.map((notification) => notification.metadata.sequence.toString()),
      hopFrames: effectiveHopFrames,
      nativeStart: streamMetadataFields(nativeStarts[0]),
      nativeReadHopFrames: nativeReads.map((metadata) => metadata.hopFrames),
      publicationHopFrames: notifications.map((notification) => notification.metadata.hopFrames),
      publicationSmoothingMs: notifications.map((notification) => notification.metadata.smoothingMs),
      gap,
      renderLoss: firstReadAfterRender === undefined ? undefined : {
        status: firstReadAfterRender.status,
        droppedCaptures: firstReadAfterRender.droppedCaptures.toString(),
      },
      recoveryDelivery,
      ownedArrays,
      sharedAfterFirstClose,
      staleReadRefused,
      first: {
        capturedSample: first.capturedSample.toString(),
        endSample: first.endSample.toString(),
        sequence: readyNotification?.metadata.sequence.toString() ?? "",
        snapshotToken: first.snapshotToken.toString(),
        hopFrames: notifications.find((notification) => notification.available
          && notification.metadata.capturedSample === first.capturedSample)?.metadata.hopFrames,
        smoothingMs: notifications.find((notification) => notification.available
          && notification.metadata.capturedSample === first.capturedSample)?.metadata.smoothingMs,
        peakBins: [leftPeak.index, rightPeak.index],
        peakHz: [first.frequenciesHz[leftPeak.index], first.frequenciesHz[rightPeak.index]],
        peakDbfs: [leftPeak.value, rightPeak.value],
        expectedPeakDbfs: [expectedLeftDb, expectedRightDb],
        responseHz: response.frequenciesHz[32],
        responseGainDb: [response.leftDb[32], response.rightDb[32]],
        sampleRateHz: first.sampleRateHz,
        windowFrames: first.windowFrames,
        binCount: first.binCount,
        sourceUnderrun: first.graphSourceUnderrun,
        firstPcmPeak,
        expectedLinearPeaks: [expectedLeft, expectedRight],
        meterPeaks: meter === undefined ? [] : Array.from(meter.peaks),
        meterSpan: meter === undefined ? [] : [meter.firstSample.toString(), meter.endSample.toString()],
      },
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await finishSdkEngine(browser, () => shared?.close(), () => subscription?.close());
  }
}

/** A short live probe drains two overlapping H1024 windows through the real Worklet queue. */
/** The configured hop probe's waits (#1106): a planted slow audio start, and the bounds it must outlast. */
const HOP_PROBE_START_DELAY_MS = 750;
const HOP_PROBE_DEADLINE_MS = 10_000;
const HOP_PROBE_NEVER_DEADLINE_MS = 250;
const HOP_PROBE_POLL_MS = 5;
// #1522: the bound on each clock read (one worklet status round trip).
const HOP_PROBE_CLOCK_TIMEOUT_MS = 2_000;

async function runConfiguredSpectrumHopQualification(): Promise<Record<string, unknown>> {
  const query = {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  };
  const hopFrames = 1_024;
  const smoothingMs = 37.5;
  const frames = 8 * SPECTRUM_FRAMES;
  const browser = await createContinuousSpectrumBrowser(query, frames, false, hopFrames, true);
  const nativeStarts = [];
  const nativeReads = [];
  // Plant a slow audio start (#1106): the native stream starts, and so its first window can publish,
  // only when `streamStartGate` opens, `HOP_PROBE_START_DELAY_MS` after the click that resumed the
  // context. Until then the context runs and every `pump()` returns at once with nothing published.
  let openStreamStart!: () => void;
  const streamStartGate = new Promise<void>((resolve) => { openStreamStart = resolve; });
  watchSpectrumFrames(browser.host, {
    before: async ({ message, code }) => {
      if (message === 0x0012 && code === STREAM_START) await streamStartGate;
    },
    after: ({ message, code }, response) => {
      if (message !== 0x0012) return;
      const metadata = streamReplyMetadata(response);
      if (metadata === undefined) return;
      if (code === STREAM_START) nativeStarts.push(metadata);
      if (code === STREAM_READ) nativeReads.push(metadata);
    },
  });
  let subscription;
  const notifications = [];
  const publications = [];
  const publicationTimes: number[] = [];
  const publicationKeys = new Set();
  const remember = (notification) => {
    notifications.push(notification);
    if (!notification.available || subscription === undefined) return;
    const result = subscription.readLatest();
    if (result === undefined || result.capturedSample !== notification.metadata.capturedSample) return;
    const key = `${notification.metadata.captureEpoch}:${notification.metadata.sequence}`;
    if (publicationKeys.has(key)) return;
    publicationKeys.add(key);
    publications.push({ metadata: notification.metadata, result });
    publicationTimes.push(performance.now());
  };
  try {
    const subscribing = browser.subscribeSpectrum({
      ...query,
      smoothingMs,
      cadenceMs: 1,
      onUpdate: remember,
    }).then((subscribed) => { subscription = subscribed.handle; });
    // Settled by the await after the wait; a failure before it must not become an unhandled rejection.
    subscribing.catch(() => undefined);
    await submitSpectrumSource(browser, frames);
    const context = browser.context as AudioContext;
    const resumeButton = document.createElement("button");
    resumeButton.id = "spectrum-hop-resume";
    resumeButton.textContent = "Start configured spectrum qualification audio";
    document.body.append(resumeButton);
    let resumeTimer: ReturnType<typeof setTimeout> | undefined;
    let resumeGesture = false;
    let resumeCall!: Promise<void>;
    let clickedAt = 0;
    // #1522 Amendment 1: the context's clock and the worklet's status at the resume and at the end
    // of the wait, so a failing record shows whether the context stalled (the stall signature).
    // Not sampled at each `pump()`: a status round trip per poll would change the wait it records.
    // Each read is bounded, so a stalled `status()` yields a record instead of hanging the probe.
    const bounded = async <T>(read: Promise<T>, timedOut: T): Promise<T> => {
      let timer: ReturnType<typeof setTimeout> | undefined;
      try {
        return await Promise.race([read, new Promise<T>((resolve) => {
          timer = setTimeout(() => resolve(timedOut), HOP_PROBE_CLOCK_TIMEOUT_MS);
        })]);
      } finally {
        if (timer !== undefined) clearTimeout(timer);
      }
    };
    const sampleClock = async (): Promise<Record<string, unknown>> => {
      const contextState = context.state;
      const currentTime = context.currentTime;
      try {
        const status = await bounded<MisoStatus | undefined>(browser.host.status(), undefined);
        if (status === undefined) return { contextState, currentTime, status: "timeout" };
        return { contextState, currentTime, requestId: status.requestId,
          renderedQuanta: String(status.renderedQuanta), lastResult: status.lastResult };
      } catch {
        return { contextState, currentTime, status: null };
      }
    };
    let clockAtResume: Promise<Record<string, unknown>> = Promise.resolve({ resumed: false });
    try {
      await new Promise<void>((resolve, reject) => {
        resumeTimer = setTimeout(() => reject(new Error(
          `configured spectrum audio resume timed out (gesture=${resumeGesture}, state=${context.state})`,
        )), 10_000);
        resumeButton.addEventListener("click", () => {
          resumeGesture = true;
          clickedAt = performance.now();
          globalThis.setTimeout(openStreamStart, HOP_PROBE_START_DELAY_MS);
          resumeCall = context.resume();
          resumeCall.catch(reject);
          clockAtResume = resumeCall.then(sampleClock, () => ({ resumed: false }));
          resolve();
        }, { once: true });
      });
    } finally {
      resumeButton.remove();
    }
    // The wait starts at the click, not when `resume()` resolves: a browser may resolve it before or
    // after its first render. No window can publish for `HOP_PROBE_START_DELAY_MS`, and the wait
    // must outlast that by its deadline, never by a poll count. It ends on two windows one hop
    // apart: a late start can leave the first reads behind the audio and drop windows, and the hop
    // is the distance between two consecutive publications.
    const consecutive = () => {
      for (let index = 0; index + 1 < publications.length; index += 1) {
        const a = publications[index].metadata;
        const b = publications[index + 1].metadata;
        if (a.captureEpoch === b.captureEpoch && b.capturedSample - a.capturedSample === BigInt(hopFrames)) {
          return index;
        }
      }
      return -1;
    };
    let waited;
    try {
      waited = await waitForCount({
        read: () => (consecutive() >= 0 ? 2 : Math.min(1, publications.length)),
        pump: async () => { await subscription?.pump(); },
        required: 2,
        deadlineMs: HOP_PROBE_DEADLINE_MS,
        pollMs: HOP_PROBE_POLL_MS,
      });
      await resumeCall;
    } finally {
      if (resumeTimer !== undefined) clearTimeout(resumeTimer);
    }
    await subscribing;
    const clock = {
      atResume: await bounded(clockAtResume, { status: "timeout" }),
      atEnd: await sampleClock(),
    };
    const pairIndex = consecutive();
    if (pairIndex < 0) {
      throw new Error(`H1024 browser probe published ${publications.length} windows `
        + `[${publications.map(({ metadata }) => metadata.capturedSample).join(", ")}], none one hop apart, in ${waited.elapsedMs.toFixed(0)} ms `
        + `(deadline ${HOP_PROBE_DEADLINE_MS} ms, ${waited.polls} polls, audio start delayed ${HOP_PROBE_START_DELAY_MS} ms, `
        + `${notifications.length} notifications, clock ${JSON.stringify(clock)})`);
    }
    const neverPublishes = await waitForCount({
      read: () => 0,
      pump: async () => undefined,
      required: 1,
      deadlineMs: HOP_PROBE_NEVER_DEADLINE_MS,
      pollMs: HOP_PROBE_POLL_MS,
    });
    const first = publications[pairIndex];
    const second = publications[pairIndex + 1];
    const firstResult = first.result;
    const secondResult = second.result;
    const firstFrequencies = firstResult.frequenciesHz.slice();
    const firstLeft = firstResult.leftDb?.slice();
    const firstRight = firstResult.rightDb?.slice();
    secondResult.leftDb?.set([secondResult.leftDb[0]! + 100], 0);
    const ownedArrays = firstResult.frequenciesHz !== secondResult.frequenciesHz
      && firstResult.leftDb !== secondResult.leftDb
      && firstResult.rightDb !== secondResult.rightDb
      && firstResult.frequenciesHz.every((value, index) => value === firstFrequencies[index])
      && (firstLeft === undefined || firstResult.leftDb?.every((value, index) => value === firstLeft[index]))
      && (firstRight === undefined || firstResult.rightDb?.every((value, index) => value === firstRight[index]));
    return {
      requestHopFrames: hopFrames,
      smoothingMs,
      boundsHopFrames: subscription.bounds.hopFrames,
      nativeStart: streamMetadataFields(nativeStarts[0]),
      nativeRead: nativeReads.slice(0, 4).map(streamMetadataFields),
      publicationCount: publications.length,
      wait: {
        startDelayMs: HOP_PROBE_START_DELAY_MS,
        deadlineMs: HOP_PROBE_DEADLINE_MS,
        pollMs: HOP_PROBE_POLL_MS,
        firstPublicationAfterMs: publicationTimes[0]! - clickedAt,
        secondPublicationAfterMs: publicationTimes[pairIndex + 1]! - clickedAt,
        elapsedMs: waited.elapsedMs,
        polls: waited.polls,
        // A wait that can never be satisfied ends at its own deadline and spins no faster than
        // its poll timer.
        neverPublishes: { ...neverPublishes, deadlineMs: HOP_PROBE_NEVER_DEADLINE_MS },
      },
      notificationHops: notifications.map((notification) => notification.metadata.hopFrames),
      notificationSmoothingMs: notifications.map((notification) => notification.metadata.smoothingMs),
      publications: [first, second].map(({ metadata, result }) => ({
        metadata: streamMetadataFields(metadata),
        result: {
          capturedSample: result.capturedSample.toString(),
          endSample: result.endSample.toString(),
          windowFrames: result.windowFrames,
          binCount: result.binCount,
          finite: Array.from(result.frequenciesHz).every(Number.isFinite)
            && Array.from(result.leftDb ?? []).every(Number.isFinite)
            && Array.from(result.rightDb ?? []).every(Number.isFinite),
          ownedArrays: result.frequenciesHz !== result.leftDb
            && result.frequenciesHz !== result.rightDb
            && result.leftDb !== result.rightDb,
        },
      })),
      startDelta: (secondResult.capturedSample - firstResult.capturedSample).toString(),
      firstSpan: (firstResult.endSample - firstResult.capturedSample).toString(),
      secondSpan: (secondResult.endSample - secondResult.capturedSample).toString(),
      resultHopFrames: first.metadata.hopFrames,
      resultSmoothingMs: first.metadata.smoothingMs,
      ownedArrays,
      clock,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    openStreamStart();
    await finishSdkEngine(browser, () => subscription?.close());
  }
}

async function querySpectrumAfterArm(browser, query) {
  let acknowledgeArm;
  let rejectArm;
  const armed = new Promise((resolve, reject) => {
    acknowledgeArm = resolve;
    rejectArm = reject;
  });
  watchSpectrumFrames(browser.host, {
    after: ({ message, code }, response) => {
      if (message !== 0x0011 || code !== CAPTURE_ARM) return;
      // A refusal is the SDK's to report: the query rejects with the core's code.
      if (response !== undefined && decodeSpectrumCaptureReply(response).ok) acknowledgeArm();
      else rejectArm(new Error("the core refused the spectrum arm"));
    },
  });
  const pending = browser.querySpectrum(query);
  await armed;
  await browser.context.startRendering();
  return pending;
}

function serializeSpectrum(result, query) {
  const frequencies = Array.from(result.frequenciesHz);
  const left = result.leftDb === undefined ? [] : Array.from(result.leftDb);
  const right = result.rightDb === undefined ? [] : Array.from(result.rightDb);
  const finite = frequencies.every(Number.isFinite)
    && left.every(Number.isFinite) && right.every(Number.isFinite);
  return {
    target: result.target,
    targetKey: spectrumTargetKey(query.target),
    channels: result.channels,
    sampleRateHz: result.sampleRateHz,
    windowFrames: result.windowFrames,
    binCount: result.binCount,
    floorDb: result.floorDb,
    frequencies,
    left,
    right,
    capturedSample: result.capturedSample.toString(),
    endSample: result.endSample.toString(),
    snapshotToken: result.snapshotToken.toString(),
    sourceUnderrun: result.graphSourceUnderrun,
    resultBytes: result.resultBytes.toString(),
    finite,
    ownedArrays: result.frequenciesHz !== result.leftDb
      && result.frequenciesHz !== result.rightDb
      && result.leftDb !== result.rightDb,
  };
}

async function runSdkSpectrumQualification(): Promise<Record<string, unknown>> {
  const rows = [];
  let busyRefused = false;
  for (const query of SPECTRUM_QUERIES) {
    const browser = await createSpectrumBrowser(query);
    try {
      await submitSpectrumSource(browser);
      // #1596 gate 5: on this page (no cross-origin isolation) the core serves the spectrum frames
      // in the worklet's message handler, so its allocations are counted as single-mode control work.
      const allocationsBefore = (await browser.host.status()).singleModeControlAllocations;
      const pending = querySpectrumAfterArm(browser, query);
      if (rows.length === 0) {
        try {
          await browser.querySpectrum(query);
        } catch {
          busyRefused = true;
        }
      }
      const result = await pending;
      const allocationsAfter = (await browser.host.status()).singleModeControlAllocations;
      const frequencies = result.frequenciesHz.slice();
      const left = result.leftDb?.slice();
      const right = result.rightDb?.slice();
      const row = {
        ...serializeSpectrum(result, query),
        controlAllocations: [String(allocationsBefore), String(allocationsAfter)],
        ownedAfterClose: false,
      };
      await closeSdkEngine(browser);
      row.ownedAfterClose = frequencies.every(
        (value, index) => value === row.frequencies[index],
      ) && (left === undefined || left.every(
        (value, index) => value === row.left[index],
      )) && (right === undefined || right.every(
        (value, index) => value === row.right[index],
      ));
      rows.push(row);
    } catch (error) {
      throw sdkEngineFailed(browser, error);
    } finally {
      await closeSdkEngine(browser);
    }
  }

  const closedBrowser = await createSpectrumBrowser(SPECTRUM_QUERIES[0], "spectrum-query-closed");
  let closedRefused = false;
  try {
    await submitSpectrumSource(closedBrowser);
    let acknowledgeArm;
    const armed = new Promise((resolve) => { acknowledgeArm = resolve; });
    watchSpectrumFrames(closedBrowser.host, {
      after: ({ message, code }, response) => {
        if (message === 0x0011 && code === CAPTURE_ARM && response !== undefined
          && decodeSpectrumCaptureReply(response).ok) acknowledgeArm();
      },
    });
    const pending = closedBrowser.querySpectrum(SPECTRUM_QUERIES[0]);
    // Settle `pending` before the close rejects it: a rejection with no handler yet is an unhandled
    // rejection for a turn (a pageerror in both supported browsers). The handler is attached first,
    // so the refusal this check proves is the only outcome observed (root, 2026-10-12, named #1345
    // exception).
    const settled = pending.then(() => false, () => true);
    await armed;
    await closeSdkEngine(closedBrowser);
    closedRefused = await settled;
  } catch (error) {
    throw sdkEngineFailed(closedBrowser, error);
  } finally {
    await closeSdkEngine(closedBrowser);
  }

  return {
    targets: rows,
    targetKeys: rows.map((row) => row.targetKey),
    allTargetsDistinct: new Set(rows.map((row) => row.targetKey)).size === 3,
    postChainDiffers: rows[0]?.left.some((value, index) => value !== rows[1]?.left[index]) === true,
    windows: rows.map((row) => [row.capturedSample, row.endSample, row.windowFrames]),
    allFinite: rows.every((row) => row.finite),
    ownedArrays: rows.every((row) => row.ownedArrays && row.ownedAfterClose),
    busyRefused,
    closedRefused,
    continuous: await runContinuousSpectrumQualification(),
    collection: await runSpectrumCollectionQualification(),
    configuredHop: await runConfiguredSpectrumHopQualification(),
  };
}

async function runSpectrumCollectionQualification(): Promise<Record<string, unknown>> {
  const [entryA, entryB] = SPECTRUM_COLLECTION_ENTRIES;
  if (entryA === undefined || entryB === undefined) throw new Error("spectrum collection fixture is incomplete");
  const browser = await createSpectrumCollectionBrowser();
  const query = (entry: typeof entryA) => ({
    ...entry,
    spectrumLimits: { maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES },
  });
  const firstQuery = query(entryA);
  const secondQuery = query(entryB);
  const readinessTimeoutMs = 10_000;
  const totalFrames = 16 * SPECTRUM_FRAMES;
  const totalBlocks = totalFrames / 128;
  const submitBlocks = async (firstBlock: number, lastBlock: number, endOfRegion: boolean) => {
    for (let block = firstBlock; block < lastBlock; block += 1) {
      let acknowledgement;
      const submitDeadline = performance.now() + readinessTimeoutMs;
      for (;;) {
        acknowledgement = await browser.host.submitSource({
          sourceId: "live-control-source",
          generation: 1n,
          startFrame: BigInt(block * 128),
          sampleRateHz: 48_000,
          planes: collectionSpectrumPlanes(block, block * 128),
          frames: 128,
          endOfRegion: endOfRegion && block === lastBlock - 1,
        });
        if (acknowledgement.result === 0) break;
        if (acknowledgement.result !== 6) {
          throw new Error(`SDK spectrum collection source submission refused (${acknowledgement.result}) at block ${block}`);
        }
        if (performance.now() >= submitDeadline) break;
        await new Promise((resolve) => globalThis.setTimeout(resolve, 5));
      }
      if (acknowledgement?.result !== 0) {
        throw new Error(`SDK spectrum collection source submission remained backpressured at block ${block}`);
      }
    }
  };
  const waitForResult = async (subscription, entry) => {
    const targetKey = spectrumTargetKey(entry.target);
    let lastNotification;
    const startedAt = performance.now();
    // A running live context can still be starting its output backend. Poll count
    // is not rendered progress: use the same bounded readiness budget as resume.
    while (performance.now() - startedAt < readinessTimeoutMs) {
      const current = subscription.readLatest();
      if (current !== undefined
          && spectrumTargetKey(current.target) === targetKey
          && current.channels === (entry.channels ?? "both")) {
        return current;
      }
      lastNotification = await subscription.pump() ?? lastNotification;
      await new Promise((resolve) => globalThis.setTimeout(resolve, 10));
    }
    const status = await browser.host.status();
    throw new Error(`SDK spectrum collection did not publish ${targetKey}: ${JSON.stringify({
      elapsedMs: performance.now() - startedAt,
      contextState: browser.context.state,
      contextTime: (browser.context as AudioContext).currentTime,
      nativeStatus: status,
      lastStatus: lastNotification?.status,
      lastMetadata: lastNotification?.metadata,
    }, (_, value) => typeof value === "bigint" ? value.toString() : value)}`);
  };
  try {
    const subscription = (await browser.subscribeSpectrum({
      ...firstQuery,
      smoothingMs: 0,
      cadenceMs: 1,
    })).handle;
    const context = browser.context as AudioContext;
    // Feed the fixed short signal before playback; this gate measures selection continuity,
    // not per-message producer throughput on the browser's main thread.
    await submitBlocks(0, totalBlocks, true);
    const resumeButton = document.createElement("button");
    resumeButton.id = "spectrum-collection-resume";
    resumeButton.textContent = "Start spectrum qualification audio";
    document.body.append(resumeButton);
    let resumeTimer: ReturnType<typeof setTimeout> | undefined;
    let resumeGesture = false;
    try {
      await new Promise<void>((resolve, reject) => {
        resumeTimer = setTimeout(() => reject(new Error(
          `spectrum collection audio resume timed out (gesture=${resumeGesture}, state=${context.state})`,
        )), readinessTimeoutMs);
        resumeButton.addEventListener("click", () => {
          resumeGesture = true;
          void context.resume().then(resolve, reject);
        }, { once: true });
      });
    } finally {
      if (resumeTimer !== undefined) clearTimeout(resumeTimer);
      resumeButton.remove();
    }
    const first = await waitForResult(subscription, entryA);
    const firstClock = context.currentTime;
    const firstLeft = first.leftDb;
    if (firstLeft === undefined) throw new Error("spectrum collection omitted the first A channel");
    const firstPeak = spectrumPeak(firstLeft);

    const toB = await subscription.update({ ...secondQuery, smoothingMs: 0, cadenceMs: 1 });
    const bCleared = subscription.readLatest() === undefined;
    const second = await waitForResult(subscription, entryB);
    const secondClock = context.currentTime;
    const secondLeft = second.leftDb;
    if (secondLeft === undefined) throw new Error("spectrum collection omitted the selected B channel");
    const secondPeak = spectrumPeak(secondLeft);

    const toA = await subscription.update({ ...firstQuery, smoothingMs: 0, cadenceMs: 1 });
    const aCleared = subscription.readLatest() === undefined;
    const result = await waitForResult(subscription, entryA);
    const finalClock = context.currentTime;
    const left = result.leftDb;
    if (left === undefined) throw new Error("spectrum collection omitted the final A channel");
    const peak = spectrumPeak(left);
    const owned = result.frequenciesHz !== result.leftDb
      && result.frequenciesHz !== result.rightDb
      && result.leftDb !== result.rightDb;
    const resultTarget = spectrumTargetKey(result.target);
    await subscription.close();
    const audioContinued = context.state === "running" && secondClock >= firstClock && finalClock >= secondClock;
    return {
      selections: [firstQuery, secondQuery, firstQuery].map((entry) => spectrumTargetKey(entry.target)),
      masks: [firstQuery.channels, secondQuery.channels],
      toBTarget: spectrumTargetKey(toB.configuration.target),
      toBChannels: toB.configuration.channels,
      toBOk: true,
      toAOks: true,
      bCleared,
      aCleared,
      firstTarget: spectrumTargetKey(first.target),
      firstChannels: first.channels,
      firstCapturedSample: first.capturedSample.toString(),
      firstEndSample: first.endSample.toString(),
      secondTarget: spectrumTargetKey(second.target),
      secondChannels: second.channels,
      secondCapturedSample: second.capturedSample.toString(),
      secondEndSample: second.endSample.toString(),
      resultTarget,
      resultChannels: result.channels,
      capturedSample: result.capturedSample.toString(),
      endSample: result.endSample.toString(),
      peakBin: peak.index,
      peakHz: result.frequenciesHz[peak.index],
      firstPeakDb: firstPeak.value,
      secondPeakDb: secondPeak.value,
      finalPeakDb: peak.value,
      firstPeakHz: first.frequenciesHz[firstPeak.index],
      secondPeakHz: second.frequenciesHz[secondPeak.index],
      distinctSelectedSignal: Math.abs(firstPeak.value - secondPeak.value) > 3,
      finite: Array.from(result.frequenciesHz).every(Number.isFinite)
        && Array.from(result.leftDb ?? []).every(Number.isFinite)
        && Array.from(result.rightDb ?? []).every(Number.isFinite),
      owned,
      audioContinued,
      switchedWithoutRestart: true,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

/**
 * Start a track response query on an engine whose offline context has not rendered yet, and
 * resolve, with the pending query, once the core has armed its capture: the strip request's tag reply has
 * come back, so the render that follows answers it (#1579b).
 */
async function armTrackResponse(
  browser: SdkEngine,
  request: Parameters<SdkEngine["queryTrackResponse"]>[0],
): Promise<{ readonly query: Promise<TrackResponseResult> }> {
  const host = browser.host;
  const control = host.control;
  let armed!: () => void;
  const tagged = new Promise<void>((resolve) => { armed = resolve; });
  host.control = async (...args: Parameters<MisoAudioWorkletHost["control"]>) => {
    const reply = await control.apply(host, args);
    try {
      if (reply.result === 0 && decodeResponseTagReply(reply.response).ok) armed();
    } catch {
      // Another command's reply.
    }
    return reply;
  };
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    const query = browser.queryTrackResponse(request);
    query.catch(() => undefined);
    await Promise.race([tagged, query, new Promise<never>((_resolve, reject) => {
      timer = setTimeout(() => reject(new Error("the track response capture was not armed")), 10_000);
    })]);
    // Wrapped: an async function that returned the promise itself would wait for it to settle.
    return { query };
  } finally {
    if (timer !== undefined) clearTimeout(timer);
    host.control = control;
  }
}

/** The workload label of the live track-response engine (#1579b). */
const TRACK_RESPONSE_LIVE = "track-response-live";
/** The bound on every wait of the live track-response row that depends on the audio clock. */
const TRACK_RESPONSE_WAIT_MS = 10_000;
/** The rendered blocks, and the least wall time, a "no new publication" window spans. */
const TRACK_RESPONSE_QUIET_BLOCKS = 32n;
const TRACK_RESPONSE_QUIET_MS = 150;

/** The track's live EQ and input-filter edits the response rows and the PCM parity row submit. */
function trackResponseEdits(owner: EngineLiveControls) {
  const builtinPair = { hpfHz: 80, lpfHz: 12_000 } as const;
  // A value edit names the console slot by its stable ID, which needs no builder.
  const eq = owner.edit.track("track").console("eq-simd1", "miso.parametric-eq");
  return [eq.parameter("band-1-gain", -3), eq.parameter("hpf-frequency", 400),
    eq.parameter("hpf-q", 0.8), eq.parameter("hpf-enabled", true),
    eq.parameter("lpf-frequency", 6_000), eq.parameter("lpf-q", 0.9),
    eq.parameter("lpf-enabled", true), owner.edit.track("track").inputFilters(builtinPair, { channel: "left" })];
}

/**
 * The live track-response engine (#1579b): a track response is captured in a rendered block, so
 * its queries run on a real `AudioContext` that renders on its own clock after one declared
 * trusted click. `stats` counts the response Workers it creates (none: the query is the core's
 * frames) and every frame it sends through `host.control`.
 */
async function createTrackResponseLiveBrowser(stats: { workers: number; controlFrames: number }) {
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  return createSdkEngine(TRACK_RESPONSE_LIVE, {
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new AudioContext({ sampleRate: 48_000, latencyHint: "interactive" });
      // An engine may create a later context already running; the click below starts it.
      void context.suspend();
      return context;
    },
    createHost: async (request) => {
      const host = await createDefaultHost({
        ...request,
        hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
      });
      return new Proxy(host, {
        get(target, key) {
          const value = Reflect.get(target, key, target);
          if (key === "control") {
            return (...args: Parameters<MisoAudioWorkletHost["control"]>) => {
              stats.controlFrames += 1;
              return target.control(...args);
            };
          }
          return typeof value === "function" ? value.bind(target) : value;
        },
      });
    },
    createResponseWorker: (url, options) => {
      stats.workers += 1;
      return new Worker(url, options);
    },
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
}

/** Wait, bounded, until the live engine has rendered `blocks` more quanta and `ms` have passed. */
async function renderLiveBlocks(browser: SdkEngine, blocks: bigint, ms: number): Promise<bigint> {
  const began = performance.now();
  const from = BigInt((await browser.host.status()).renderedQuanta);
  for (;;) {
    const now = BigInt((await browser.host.status()).renderedQuanta);
    if (now >= from + blocks && performance.now() - began >= ms) return now - from;
    if (performance.now() - began > TRACK_RESPONSE_WAIT_MS) {
      throw new Error(`the live track-response context rendered ${now - from} of ${blocks} blocks (state=${browser.context.state})`);
    }
    await new Promise((resolve) => globalThis.setTimeout(resolve, 5));
  }
}

/**
 * The browser's track responses on a live context (#1579b, gates 1, 3 and 4): one-shot queries
 * (two at once settle one after the other) and the managed subscription (no publication while
 * nothing changes, one publication after an edit).
 */
async function runTrackResponseLiveQualification(): Promise<Record<string, unknown>> {
  const stats = { workers: 0, controlFrames: 0 };
  const browser = await createTrackResponseLiveBrowser(stats);
  const request = {
    trackId: "track",
    grid: { kind: "logarithmic" as const, points: 5, minimumHz: 20, maximumHz: 20_000 },
    channels: "both" as const,
    responseLimits: { maximumResultBytes: 1 << 20, requestDeadlineMs: 5_000 },
  };
  let callbackCount = 0;
  let callbackOwner = "";
  let callbackEpoch = "";
  let callbackJob = "";
  let callbackRevision = "";
  let callbackResult: TrackResponseResult | undefined;
  let resolveCallback: (() => void) | undefined;
  const callbackDelivered = new Promise<void>((resolve) => { resolveCallback = resolve; });
  try {
    const context = browser.context as AudioContext;
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const acknowledgement = await browser.host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes: observationPlanes(block),
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("live track-response source submission refused");
    }
    const resumeButton = document.createElement("button");
    resumeButton.id = "track-response-resume";
    resumeButton.textContent = "Start track response qualification audio";
    document.body.append(resumeButton);
    let resumeTimer: ReturnType<typeof setTimeout> | undefined;
    let resumeGesture = false;
    try {
      await new Promise<void>((resolve, reject) => {
        resumeTimer = setTimeout(() => reject(new Error(
          `track response audio resume timed out (gesture=${resumeGesture}, state=${context.state})`,
        )), TRACK_RESPONSE_WAIT_MS);
        resumeButton.addEventListener("click", () => {
          resumeGesture = true;
          void context.resume().then(resolve, reject);
        }, { once: true });
      });
    } finally {
      if (resumeTimer !== undefined) clearTimeout(resumeTimer);
      resumeButton.remove();
    }
    await renderLiveBlocks(browser, 4n, 0);

    // One-shot queries. Two at once settle in turn: the second is captured in a later block.
    const live = await browser.queryTrackResponse(request);
    const [first, second] = await Promise.all([
      browser.queryTrackResponse(request),
      browser.queryTrackResponse(request),
    ]);
    const serialized = second.snapshotToken > first.snapshotToken
      && second.capturedSample > first.capturedSample
      && first.capturedSample > live.capturedSample
      && responseValuesEqual(first, live) && responseValuesEqual(second, live);
    const liveAgain = await browser.queryTrackResponse(request);
    const liveLeft = live.leftDb === undefined ? [] : Array.from(live.leftDb);
    const liveRight = live.rightDb === undefined ? [] : Array.from(live.rightDb);
    const liveFinite = live.frequenciesHz.length === request.grid.points
      && liveLeft.length === request.grid.points
      && liveRight.length === request.grid.points
      && Array.from(live.frequenciesHz).every(Number.isFinite)
      && liveLeft.every(Number.isFinite)
      && liveRight.every(Number.isFinite);
    const liveOwnedAfterSecondQuery = liveFinite && responseValuesEqual(live, liveAgain)
      && live.leftDb !== liveAgain.leftDb;

    // The managed subscription.
    const subscription = await browser.subscribeTrackResponse({
      ...request,
      cadenceMs: 10,
      onUpdate: (notification) => {
        callbackCount += 1;
        callbackOwner = notification.owner.toString();
        callbackEpoch = notification.epoch.toString();
        callbackJob = notification.job.toString();
        callbackRevision = notification.revision.toString();
        callbackResult = notification.handle.readLatest();
        resolveCallback?.();
      },
    });
    const initial = subscription.readLatest();
    if (initial === undefined) throw new Error("track response subscription returned no initial result");
    // No edit: renders publish nothing new.
    await renderLiveBlocks(browser, TRACK_RESPONSE_QUIET_BLOCKS, TRACK_RESPONSE_QUIET_MS);
    await subscription.pump();
    const unchangedOverRenders = subscription.revision === 1n
      && responseValuesEqual(subscription.readLatest()!, initial);
    const unchangedNotificationCount = callbackCount;

    const liveControls = await browser.liveControls();
    const command = await liveControls.submit(...trackResponseEdits(liveControls));
    // Seven EQ parameter edits and the two input-filter cutoffs: nine edits in one transaction.
    if (!command.ok || command.appliedOperations !== 9) {
      throw new Error("live track-response EQ and builtin-filter edits were refused or split");
    }
    const appliedAtSample = (await browser.applied(command.revision)).sample;
    const timerDelivered = await Promise.race([
      callbackDelivered.then(() => true),
      new Promise<boolean>((resolve) => globalThis.setTimeout(() => resolve(false), TRACK_RESPONSE_WAIT_MS)),
    ]);
    let pumped = false;
    if (!timerDelivered) pumped = (await subscription.pump())?.available === true;
    const afterRender = subscription.readLatest();
    if (afterRender === undefined) throw new Error("track response subscription lost its result after the edit");
    const changedPublication = !responseValuesEqual(initial, afterRender)
      && subscription.revision === 2n
      // A capture is stamped with its block's exclusive end, so the block that applied the edit
      // reports appliedAt + 128: equality would be a read of the committed model, not the audio.
      && afterRender.capturedSample > appliedAtSample;
    const callbackMatches = callbackResult !== undefined && responseValuesEqual(callbackResult, afterRender);
    // After the edit: one publication, however many blocks render.
    await renderLiveBlocks(browser, TRACK_RESPONSE_QUIET_BLOCKS, TRACK_RESPONSE_QUIET_MS);
    await subscription.pump();
    const publishedOnce = subscription.revision === 2n && callbackCount === 1
      && responseValuesEqual(subscription.readLatest()!, afterRender);

    const shared = await browser.subscribeTrackResponse({ ...request, cadenceMs: 25 });
    const sharedJob = shared.job === subscription.job
      && shared.owner === subscription.owner
      && responseValuesEqual(shared.readLatest()!, afterRender);
    const sharedView = shared.readLatest();
    const originalSharedLeft = sharedView?.leftDb?.[0];
    if (sharedView?.leftDb !== undefined && originalSharedLeft !== undefined) {
      sharedView.leftDb[0] = originalSharedLeft + 100;
    }
    const arraysIsolated = originalSharedLeft !== undefined
      && subscription.readLatest()?.leftDb?.[0] === originalSharedLeft
      && shared.readLatest()?.leftDb?.[0] === originalSharedLeft;
    await subscription.close();
    const firstCloseKeepsJob = responseValuesEqual(shared.readLatest()!, afterRender);

    const oldJob = shared.job;
    const oldResult = shared.readLatest();
    let invalidUpdateRefused = false;
    try {
      await shared.update({ ...request, cadenceMs: 10, grid: { ...request.grid, points: 1 } });
    } catch {
      invalidUpdateRefused = true;
    }
    const invalidUpdatePreserved = shared.job === oldJob
      && oldResult !== undefined && responseValuesEqual(shared.readLatest()!, oldResult);
    const updatedReceipt = await shared.update({ ...request, cadenceMs: 10, grid: { ...request.grid, points: 7 } });
    const updated = shared.readLatest();
    const gridUpdated = updated !== undefined
      && updatedReceipt.job === shared.job
      && shared.configuration.grid.points === 7
      && updated.frequenciesHz.length === 7;
    const leftOnly = await browser.subscribeTrackResponse({
      ...request,
      cadenceMs: 10,
      grid: { ...request.grid, points: 7 },
      channels: "left" as const,
    });
    const leftOnlyResult = leftOnly.readLatest();
    const independentChannelJob = leftOnly.job !== shared.job
      && leftOnlyResult?.leftDb !== undefined && leftOnlyResult.rightDb === undefined;
    await shared.close();
    await leftOnly.close();
    // A poll that began before the last close may still send; after that, nothing does.
    await new Promise<void>((resolve) => globalThis.setTimeout(resolve, 20));
    const framesAtLastClose = stats.controlFrames;
    await new Promise<void>((resolve) => globalThis.setTimeout(resolve, 60));
    const lastCloseStoppedCapture = stats.controlFrames === framesAtLastClose;
    let staleReadRefused = false;
    try {
      shared.readLatest();
    } catch {
      staleReadRefused = true;
    }
    await closeSdkEngine(browser);
    let liveClosedRefused = false;
    try {
      await browser.queryTrackResponse(request);
    } catch {
      liveClosedRefused = true;
    }
    return {
      liveResponse: {
        trackId: live.trackId,
        mode: live.mode,
        meaning: live.meaning,
        sampleRateHz: live.sampleRateHz,
        points: live.frequenciesHz.length,
        firstFrequency: live.frequenciesHz[0],
        lastFrequency: live.frequenciesHz[live.frequenciesHz.length - 1],
        left: liveLeft,
        right: liveRight,
        finite: liveFinite,
        members: live.members.map((member) => ({
          nativeId: member.nativeId,
          stableId: member.stableId,
          rack: member.rack,
          rackValue: member.rackValue,
          slot: member.slot,
          kind: member.kind,
          available: member.available,
          bypassed: member.bypassed,
          enabledLeft: member.enabledLeft.length,
          enabledRight: member.enabledRight.length,
        })),
        excludedMemberCount: live.excludedMemberCount,
        capturedSample: live.capturedSample.toString(),
        snapshotToken: live.snapshotToken.toString(),
        resultBytes: live.resultBytes.toString(),
        ownedAfterSecondQuery: liveOwnedAfterSecondQuery,
        serialized,
        closedRefused: liveClosedRefused,
      },
      managedResponse: {
        initial: serializeTrackResponse(initial),
        afterRender: serializeTrackResponse(afterRender),
        updated: updated === undefined ? null : serializeTrackResponse(updated),
        leftOnly: leftOnlyResult === undefined ? null : serializeTrackResponse(leftOnlyResult),
        responseWorkersCreated: stats.workers,
        appliedAtSample: appliedAtSample.toString(),
        unchangedOverRenders,
        unchangedNotificationCount,
        changedPublication,
        publishedOnce,
        callbackCount,
        callbackOwner,
        callbackEpoch,
        callbackJob,
        callbackRevision,
        callbackMatches,
        automaticDelivery: timerDelivered,
        pumpDelivered: pumped,
        sharedJob,
        arraysIsolated,
        firstCloseKeepsJob,
        invalidUpdateRefused,
        invalidUpdatePreserved,
        gridUpdated,
        independentChannelJob,
        lastCloseStoppedCapture,
        staleReadRefused,
        parityWithOneShot: responseValuesEqual(initial, live),
        owner: subscription.owner.toString(),
        epoch: subscription.epoch.toString(),
        job: subscription.job.toString(),
        revision: subscription.revision.toString(),
        snapshotToken: initial.snapshotToken.toString(),
        bounds: subscription.bounds,
      },
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

/**
 * The live EQ cuts on an offline context (`sdk-live-eq-cuts`): the edits are committed before the
 * render, and the browser's PCM equals the headless engine's after the parameter ramp.
 */
async function runLiveEqCutsQualification(): Promise<Record<string, unknown>> {
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createSdkEngine("live-eq-cuts", {
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  const asset = await MisoEngineAsset.load(await (await fetch(
    "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
  )).arrayBuffer());
  const headless = await WasmBoundary.boot(asset, document, {
    sourceRingFrames: OBSERVATION_FRAMES,
  });
  const headlessLiveControls = new EngineLiveControls(headless.sessionMap(), {
    queue: new ControlTransactionQueue((frame) => headless.controlFrame(frame), {
      initialRevision: headless.controlRevision(),
      // The boundary's one allocator: its own strip-order reads at boot drew IDs from the same
      // endpoint, and the core refuses an ID at or below the highest it has seen.
      requestIds: headless.controlRequestIds,
    }),
  });
  try {
    const liveControls = await browser.liveControls();
    const command = await liveControls.submit(...trackResponseEdits(liveControls));
    const headlessCommand = await headlessLiveControls.submit(...trackResponseEdits(headlessLiveControls));
    if (!headlessCommand.ok || command.path !== headlessCommand.path
        || command.revision !== headlessCommand.revision) {
      throw new Error("browser/headless live EQ and builtin-filter acknowledgements disagree");
    }
    // Seven EQ parameter edits and the two input-filter cutoffs: nine edits in one transaction.
    if (!command.ok || command.appliedOperations !== 9 || headlessCommand.appliedOperations !== 9) {
      throw new Error("queued live EQ and builtin-filter edits were refused or split");
    }
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const headlessSource = headless.submitSource({ sourceId: "live-control-source", generation: 1n,
        startFrame: BigInt(block * 128), planes,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1 });
      if (!headlessSource.ok) throw new Error("headless live EQ source refused");
      const acknowledgement = await browser.host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK live EQ source submission refused");
    }
    const rendered = await (browser.context as unknown as OfflineAudioContext).startRendering();
    let postRampMaximumDifference = 0;
    let postRampComparedFrames = 0;
    // Both transactions were committed before the first render, so every block is post-edit; the
    // comparison starts after the 64-sample parameter ramp.
    const settledAt = APPLIED_BEFORE_FIRST_SAMPLE + 64;
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const expected = headless.render(128);
      for (let frame = 0; frame < 128; frame += 1) {
        const absolute = block * 128 + frame;
        if (absolute < settledAt) continue;
        postRampComparedFrames += 1;
        for (const [channel, plane] of [expected.left, expected.right].entries()) {
          const difference = Math.abs(rendered.getChannelData(channel)[absolute]! - plane[frame]!);
          if (!Number.isFinite(difference)) throw new Error("live EQ produced nonfinite PCM");
          postRampMaximumDifference = Math.max(postRampMaximumDifference, difference);
        }
      }
    }
    return {
      appliedAtSample: APPLIED_BEFORE_FIRST_SAMPLE.toString(),
      postRampComparedFrames,
      postRampMaximumDifference,
      liveInputFilters: {
        hpfHz: 80,
        lpfHz: 12_000,
        channel: "left",
        mixedBatchAdmitted: command.appliedOperations === 9 && headlessCommand.appliedOperations === 9,
        browserHeadlessAckParity: command.path === headlessCommand.path
          && command.revision === headlessCommand.revision,
      },
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await finishSdkEngine(browser, () => headless.dispose());
  }
}

/**
 * One `Observe` transaction of the observation wait instances (#1530 D4, on the core's frames since
 * #1589b): the `host.control` call's result, the reply's status (`null` for a failed call, which
 * carries no reply frame) and the frame's edits (its payload, without the header's request ID and
 * revision), as hex.
 */
interface ObservationWaitCommand {
  readonly result: number;
  readonly status: string | null;
  readonly records: string;
}

/** Issue #1530 D2: the three arguments the wait instances add; the resident call passes none. */
interface ResidentObservationOptions {
  readonly workload?: string;
  readonly requestDeadlineMs?: number;
  readonly commandLog?: ObservationWaitCommand[];
}

async function createResidentObservationBrowser(
  residentOptions: ResidentObservationOptions = {},
): Promise<SdkEngine> {
  const document = observationDocumentWithGate(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const commandLog = residentOptions.commandLog;
  const browser = await createSdkEngine(residentOptions.workload ?? "resident-observation", {
    document,
    ...(residentOptions.requestDeadlineMs === undefined
      ? {} : { requestDeadlineMs: residentOptions.requestDeadlineMs }),
    policy: { sourceRingFrames: OBSERVATION_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: async (request) => {
      const host = await createDefaultHost({
        ...request,
        hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
      });
      if (commandLog !== undefined) {
        // D4: log each `Observe` transaction and its answer; nothing else in the host is wrapped.
        const control = Object.getPrototypeOf(host).control as (this: typeof host, ...args: never[]) => Promise<{
          readonly result: number;
          readonly response: Uint8Array;
        }>;
        Object.defineProperty(host, "control", {
          configurable: true,
          writable: true,
          value: async (frame: Uint8Array, ...rest: unknown[]) => {
            const answer = await control.call(host, frame as never, ...rest as never[]);
            if (frame.length >= 48 && (frame[16]! | (frame[17]! << 8)) === MESSAGE_SESSION_TRANSACTION_APPLY) {
              commandLog.push({
                result: answer.result,
                status: answer.result === 0 ? decodeControlReply(answer.response).status : null,
                records: Array.from(frame.subarray(48), (byte) => byte.toString(16).padStart(2, "0")).join(""),
              });
            }
            return answer;
          },
        });
      }
      return host;
    },
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  return browser;
}

/** The 16 source blocks the resident flow submits, sent before any render (#1530 D3). */
async function submitObservationSourceBlocks(browser: SdkEngine, label: string): Promise<void> {
  for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
    const acknowledgement = await browser.host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * 128),
      sampleRateHz: 48_000,
      planes: observationPlanes(block),
      frames: 128,
      endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
    });
    if (acknowledgement.result !== 0) throw new Error(`${label}: source submission refused`);
  }
}

async function renderedQuantaOf(browser: SdkEngine): Promise<string> {
  return String((await browser.host.status()).renderedQuanta);
}

/**
 * Settle one `close()` within a guard (#1530 D5): a guard that fires records `settled: false`, so
 * a defect is a named red gate and never a page timeout. `began` is `performance.now()` taken just
 * before the `close()` call.
 */
async function settleWithinGuard(closing: Promise<void>, began: number, guardMs: number) {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const outcome = await Promise.race([
    closing.then(
      () => ({ settled: true, error: undefined as unknown }),
      (error: unknown) => ({ settled: true, error }),
    ),
    new Promise<{ settled: false; error: undefined }>((resolve) => {
      timer = globalThis.setTimeout(() => resolve({ settled: false, error: undefined }), guardMs);
    }),
  ]);
  if (timer !== undefined) globalThis.clearTimeout(timer);
  const elapsedMs = performance.now() - began;
  const error = outcome.error;
  return {
    settled: outcome.settled,
    resolved: outcome.settled && error === undefined,
    typed: error instanceof MisoEngineError,
    errorCode: error instanceof MisoEngineError ? error.code : error === undefined ? null : String(error),
    elapsedMs,
  };
}

/**
 * Issue #1530: the browser SDK's wait on observation backpressure against the shipped worklet.
 * Each instance owns an `OfflineAudioContext`, which renders nothing until `startRendering()`
 * (D1) and renders once, so each case has its own instance.
 */
async function runObservationWaitQualification(): Promise<Record<string, unknown>> {
  return { deadline: await runObservationWaitDeadline(), render: await runObservationWaitRender() };
}

async function subscribeWaitTap(browser: SdkEngine) {
  // D3: one selection, the bound's maximum cadence so the timer never reads during the case.
  const subscription = await browser.subscribeObservations({
    selections: [observationSelection("comp")],
    windowBlocks: 2,
    cadenceMs: 60_000,
  });
  await submitObservationSourceBlocks(browser, "observation wait");
  return subscription;
}

async function runObservationWaitDeadline(): Promise<Record<string, unknown>> {
  const commandLog: ObservationWaitCommand[] = [];
  const browser = await createResidentObservationBrowser({
    workload: "observation-wait-deadline",
    requestDeadlineMs: 250,
    commandLog,
  });
  try {
    const subscription = await subscribeWaitTap(browser);
    const renderedQuantaBefore = await renderedQuantaOf(browser);
    const began = performance.now();
    const refusal = await settleWithinGuard(subscription.close(), began, 10_000);
    const renderedQuantaAfterRefusal = await renderedQuantaOf(browser);
    let handleOpenAfterRefusal = true;
    try { subscription.readLatest(); } catch { handleOpenAfterRefusal = false; }
    const commandsBeforeRender = commandLog.map((entry) => ({ ...entry }));
    await browser.context.startRendering();
    const secondBegan = performance.now();
    const second = await settleWithinGuard(subscription.close(), secondBegan, 10_000);
    const renderedQuantaAfterRender = await renderedQuantaOf(browser);
    return {
      requestDeadlineMs: 250,
      settled: refusal.settled,
      typed: refusal.typed,
      errorCode: refusal.errorCode,
      elapsedMs: refusal.elapsedMs,
      renderedQuantaBefore,
      renderedQuantaAfterRefusal,
      handleOpenAfterRefusal,
      commandsBeforeRender,
      closeAfterRender: { resolved: second.resolved, elapsedMs: second.elapsedMs },
      renderedQuantaAfterRender,
      commands: commandLog.map((entry) => ({ ...entry })),
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

async function runObservationWaitRender(): Promise<Record<string, unknown>> {
  const commandLog: ObservationWaitCommand[] = [];
  const browser = await createResidentObservationBrowser({
    workload: "observation-wait-render",
    requestDeadlineMs: 10_000,
    commandLog,
  });
  try {
    const subscription = await subscribeWaitTap(browser);
    const renderedQuantaBefore = await renderedQuantaOf(browser);
    const began = performance.now();
    const closing = settleWithinGuard(subscription.close(), began, 15_000);
    // The arm is log entry 0; the close's first send is entry 1, and it is the refusal.
    const seenBy = performance.now() + 5_000;
    while (commandLog.length < 2 && performance.now() < seenBy) {
      await new Promise<void>((resolve) => globalThis.setTimeout(resolve, 1));
    }
    const refusalSeenBeforeRender = commandLog.length >= 2 && commandLog[1]?.result !== 0;
    const rendering = browser.context.startRendering();
    rendering.catch(() => undefined);
    const close = await closing;
    await rendering;
    const renderedQuantaAfterRender = await renderedQuantaOf(browser);
    const commands = commandLog.map((entry) => ({ ...entry }));
    return {
      requestDeadlineMs: 10_000,
      refusalSeenBeforeRender,
      settled: close.settled,
      resolved: close.resolved,
      elapsedMs: close.elapsedMs,
      renderedQuantaBefore,
      renderedQuantaAfterRender,
      commands,
      resendIdentical: commands.length === 3 && commands[1]?.records === commands[2]?.records,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

async function runResidentObservationQualification(): Promise<Record<string, unknown>> {
  const browser = await createResidentObservationBrowser();
  const selections = [observationSelection("comp"), observationSelection("gate", "left")];
  const callbackRows: ReturnType<typeof serializeObservationRows>[] = [];
  let callbackCount = 0;
  let callbackAvailable = false;
  let callbackOwner = "";
  let callbackEpoch = "";
  let resolveCallback: (() => void) | undefined;
  const callbackDelivered = new Promise<void>((resolve) => { resolveCallback = resolve; });
  try {
    const map = await browser.observationMap();
    const subscription = await browser.subscribeObservations({
      selections,
      windowBlocks: 2,
      cadenceMs: 10,
      onUpdate: (notification) => {
        callbackCount += 1;
        callbackAvailable ||= notification.available;
        callbackOwner = notification.owner.toString();
        callbackEpoch = notification.epoch.toString();
        callbackRows.push(serializeObservationRows(notification.handle.readLatest()));
        resolveCallback?.();
      },
    });
    const pending = subscription.readLatest();
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const acknowledgement = await browser.host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK resident observation source submission refused");
    }
    await browser.context.startRendering();
    const timerDelivered = await Promise.race([
      callbackDelivered.then(() => true),
      new Promise<boolean>((resolve) => globalThis.setTimeout(() => resolve(false), 500)),
    ]);
    let pumped = false;
    if (!timerDelivered) pumped = (await subscription.pump())?.available === true;
    const ready = subscription.readLatest();
    const repeated = subscription.readLatest();
    const firstSnapshot = ready.map(observationRowKey);
    // #1589 D4: the reads above saw both taps armed, so `armed()` has settled; the guard turns a
    // pending one into a named red gate, never a page timeout.
    const armedSample = await Promise.race([
      subscription.armed().then((sample) => sample.toString(), (error: unknown) => `rejected: ${String(error)}`),
      new Promise<null>((resolve) => globalThis.setTimeout(() => resolve(null), 1_000)),
    ]);

    const shared = await browser.subscribeObservations({
      selections,
      windowBlocks: 2,
      cadenceMs: 10,
    });
    const sharedBinding = shared.owner === subscription.owner
      && shared.epoch === subscription.epoch
      && observationRowsEqual(shared.readLatest(), ready);
    await subscription.close();
    const firstCloseKeepsLive = observationRowsEqual(shared.readLatest(), ready);

    let invalidUpdateRefused = false;
    try {
      await shared.update({
        selections: [observationSelection("missing")],
        windowBlocks: 2,
        cadenceMs: 10,
      });
    } catch {
      invalidUpdateRefused = true;
    }
    const invalidUpdatePreserved = observationRowsEqual(shared.readLatest(), ready);
    const updatedReceipt = await shared.update({
      selections: [observationSelection("comp", "right")],
      windowBlocks: 2,
      cadenceMs: 10,
    });
    const updated = shared.readLatest();
    const firstSnapshotRetained = JSON.stringify(firstSnapshot) === JSON.stringify(ready.map(observationRowKey));
    const updatedRightOnly = updated.length === 1
      && updated[0]?.channels === "right"
      && updated[0]?.left === undefined
      && updated[0]?.right !== undefined
      && Number.isFinite(updated[0].right);
    await shared.close();
    let staleReadRefused = false;
    try {
      shared.readLatest();
    } catch {
      staleReadRefused = true;
    }
    return {
      mapBindings: map.bindings.map((binding) => binding.effectSlotId),
      selectionEffectSlots: selections.map((selection) => selection.effectSlotId),
      pendingStatuses: pending.map((row) => row.status),
      readyStatuses: ready.map((row) => row.status),
      readyChannels: ready.map((row) => row.channels),
      readyRows: serializeObservationRows(ready),
      readyValuesFinite: ready.length === 2 && ready.every((row) =>
        (row.left === undefined || Number.isFinite(row.left))
        && (row.right === undefined || Number.isFinite(row.right))),
      windows: ready.map((row) => row.window === undefined ? null : {
        firstSample: row.window.firstSample.toString(),
        endSample: row.window.endSample.toString(),
        sequence: row.window.sequence.toString(),
        blocks: row.window.blocks,
      }),
      ownedReadStable: observationRowsEqual(ready, repeated),
      callbackCount,
      callbackAvailable,
      callbackOwner,
      callbackEpoch,
      callbackRows,
      automaticDelivery: timerDelivered,
      pumpDelivered: pumped,
      subscriptionId: subscription.id.toString(),
      owner: subscription.owner.toString(),
      epoch: subscription.epoch.toString(),
      armedSample,
      bounds: subscription.bounds,
      sharedBinding,
      firstCloseKeepsLive,
      invalidUpdateRefused,
      invalidUpdatePreserved,
      updatedSelections: updatedReceipt.configuration.selections.map((selection) => ({
        effectSlotId: selection.effectSlotId,
        channels: selection.channels,
      })),
      updatedRightOnly,
      firstSnapshotRetained,
      staleReadRefused,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

async function runSdkObservationQualification(): Promise<Record<string, unknown>> {
  const resident = await runResidentObservationQualification();
  const wait = await runObservationWaitQualification();
  // Issue #1579b: the browser's track responses run on a live context after this file's first
  // declared click; the live EQ PCM parity keeps its offline render.
  const trackResponse = await runTrackResponseLiveQualification();
  const liveEq = await runLiveEqCutsQualification();
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createSdkEngine("sdk-observation", {
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      // OfflineAudioContext has no close() lifecycle method; BrowserEngine's injected-context
      // seam still requires one so the host can use the same cleanup path as a live context.
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  try {
    const selections = [observationSelection("comp")];
    const map = await browser.observationMap();
    const pendingBeforeArm = await browser.readObservations(selections);
    // #1589c: the arm is the core's `Observe` edit through the host's `control` message.
    const subscriptions = await observeThroughSdk(browser.host, {
      stripId: "track", rack: "inserts", effectId: "comp", tapId: 1, windowBlocks: 2, armed: true,
    });
    const pendingAfterArm = await browser.readObservations(selections);
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const acknowledgement = await browser.host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK observation source submission refused");
    }
    await browser.context.startRendering();
    const ready = await browser.readObservations(selections);
    const projected = await browser.readObservations([observationSelection("comp", "left")]);
    const repeated = await browser.readObservations(selections);
    await closeSdkEngine(browser);
    const first = ready[0];
    const projectedRow = projected[0];
    return {
      mapBindings: map.bindings.map((binding) => binding.effectSlotId),
      subscribeResult: subscriptions.result,
      subscribeStatus: subscriptions.status,
      pendingBeforeArm: pendingBeforeArm.map((row) => row.status),
      pendingAfterArm: pendingAfterArm.map((row) => row.status),
      readyStatuses: ready.map((row) => row.status),
      readyChannels: ready.map((row) => row.channels),
      readyValuesFinite: ready.every((row) =>
        (row.left === undefined || Number.isFinite(row.left))
        && (row.right === undefined || Number.isFinite(row.right))),
      windows: ready.map((row) => row.window === undefined ? null : {
        firstSample: row.window.firstSample.toString(),
        endSample: row.window.endSample.toString(),
        sequence: row.window.sequence.toString(),
        blocks: row.window.blocks,
      }),
      ownedAfterSecondQuery: ready.length === repeated.length && ready.every((row, index) => {
        const other = repeated[index];
        return row.left === other?.left && row.right === other?.right
          && row.window?.firstSample === other?.window?.firstSample
          && row.window?.endSample === other?.window?.endSample
          && row.window?.sequence === other?.window?.sequence
          && row.window?.blocks === other?.window?.blocks;
      }),
      projectedChannels: projected.map((row) => row.channels),
      distinctChannelProjection: first?.left !== undefined && first?.right !== undefined
        && projectedRow?.left !== undefined && projectedRow?.right === undefined,
      resident,
      wait,
      ...trackResponse,
      liveEq,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

const LIVE_BYPASS_QUANTUM = 128;
/**
 * The fixture's mute ramp, written out: a live bypass crossfades over the session's `mute_ms`
 * (#1341a1, #1341a2), so the row below measures the ramp it declares. The other two lengths are
 * unused by this fixture.
 */
const LIVE_BYPASS_MUTE_MS = 10;
const LIVE_BYPASS_SAMPLE_RATE_HZ = 48_000;
/** The ramp's length in samples at the fixture's rate, as the core derives it (`rate * ms / 1000`). */
const LIVE_BYPASS_RAMP_SAMPLES = Math.round(LIVE_BYPASS_SAMPLE_RATE_HZ * LIVE_BYPASS_MUTE_MS / 1_000);
const LIVE_BYPASS_FRAMES = 16 * LIVE_BYPASS_QUANTUM;

/** Deterministic broadband PCM, so each EQ band below is audible in the output. */
function liveBypassPlanes(block: number): Float32Array[] {
  return [0, 1].map((channel) => {
    let state = (0x9e37_79b9 ^ (block * 1_024 + channel * 16 + 7)) >>> 0;
    return Float32Array.from({ length: 128 }, () => {
      state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
      return ((state / 0x1_0000_0000) * 2 - 1) * 0.25;
    });
  });
}

/**
 * The console-strip bypass fixture (#1097 gate 4), built with the SDK so live controls resolve IDs:
 * a `pre_insert` EQ, a `post_insert` EQ and an insert EQ, each tuned to a different band.
 */
function liveBypassSession(bypassed?: "desk-hi" | "ins-mid") {
  const band = (gain: number, frequency: number) => ({
    "band-1-enabled": true, "band-1-gain": gain, "band-1-frequency": frequency,
  });
  return session({
    id: "qualification.live-bypass",
    sampleRateHz: LIVE_BYPASS_SAMPLE_RATE_HZ,
    revision: 1,
    controlSmoothing: { muteMs: LIVE_BYPASS_MUTE_MS, faderMs: 10, panMs: 10 },
  })
    .source("live-control-source", {
      channels: 2, bitDepth: 24, frames: LIVE_BYPASS_FRAMES, content: `blake3:${"0".repeat(64)}`,
    })
    .console({
      preInsert: [{ slot: "desk-lo", effectId: "miso.parametric-eq" }],
      postInsert: [{ slot: "desk-hi", effectId: "miso.parametric-eq" }],
    })
    .track("track", {
      source: "live-control-source",
      console: [
        { slot: "desk-lo", parameters: band(9, 200) },
        { slot: "desk-hi", bypass: bypassed === "desk-hi", parameters: band(-12, 6_000) },
      ],
      inserts: [effect("miso.parametric-eq", band(12, 1_500), { slotId: "ins-mid", bypass: bypassed === "ins-mid" })],
    })
    .output("main-out")
    .route({
      id: "main",
      source: { kind: "track", trackId: "track", tap: "post_pan" },
      destination: { kind: "output_input", outputId: "main-out" },
    });
}

async function sha256Hex(planes: readonly Float32Array[]): Promise<string> {
  const bytes = new Uint8Array(planes.reduce((total, plane) => total + plane.byteLength, 0));
  let at = 0;
  for (const plane of planes) {
    bytes.set(new Uint8Array(plane.buffer, plane.byteOffset, plane.byteLength), at);
    at += plane.byteLength;
  }
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** This render's stable workload label: its authored bypass and, when present, its live edit. */
function liveBypassWorkload(
  bypassed: "desk-hi" | "ins-mid" | undefined,
  live?: { readonly target: "desk-hi" | "ins-mid"; readonly bypass: boolean },
): string {
  const authored = `live-bypass:authored-${bypassed ?? "none"}`;
  return live === undefined
    ? authored
    : `${authored}:live-${live.target}-${live.bypass ? "bypass" : "lift"}`;
}

/**
 * Render the fixture once in an OfflineAudioContext, after an optional live bypass edit.
 *
 * The engine boots from the builder itself, so `console("desk-hi")` and `insert("ins-mid")`
 * resolve through the SDK, and the edit is admitted before the first quantum: its render must be
 * the render of the same bypass authored in the document.
 */
async function renderLiveBypass(
  bypassed: "desk-hi" | "ins-mid" | undefined,
  live?: { readonly target: "desk-hi" | "ins-mid"; readonly bypass: boolean },
): Promise<{ digest: string; blocks: string[]; energy: number; section?: string; path?: string }> {
  const browser = await createSdkEngine(liveBypassWorkload(bypassed, live), {
    document: liveBypassSession(bypassed),
    policy: { sourceRingFrames: LIVE_BYPASS_FRAMES, liveControls: {} },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: LIVE_BYPASS_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(LIVE_BYPASS_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, LIVE_BYPASS_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  try {
    let section: string | undefined;
    let path: string | undefined;
    if (live !== undefined) {
      const controls = await browser.liveControls();
      const track = controls.edit.track("track");
      const edit = live.target === "desk-hi"
        ? track.console("desk-hi", "miso.parametric-eq").bypass(live.bypass)
        : track.insert("ins-mid", "miso.parametric-eq").bypass(live.bypass);
      const report = await controls.submit(edit);
      if (!report.ok || report.appliedOperations !== 1) {
        throw new Error(`live bypass ${live.target} was not applied`);
      }
      const address = edit.edits[0]!.address;
      section = `${address.section.kind}:${"slot" in address.section ? address.section.slot : "effect" in address.section ? address.section.effect : ""}:${address.control}`;
      path = report.path;
    }
    for (let block = 0; block < LIVE_BYPASS_FRAMES / 128; block += 1) {
      const acknowledgement = await browser.host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes: liveBypassPlanes(block),
        frames: 128,
        endOfRegion: block === LIVE_BYPASS_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("live bypass source submission refused");
    }
    const rendered = await (browser.context as OfflineAudioContext).startRendering();
    const planes = [rendered.getChannelData(0), rendered.getChannelData(1)];
    const energy = planes.reduce((total, plane) =>
      total + plane.reduce((sum, value) => sum + value * value, 0), 0);
    const blocks: string[] = [];
    for (let at = 0; at < LIVE_BYPASS_FRAMES; at += LIVE_BYPASS_QUANTUM) {
      blocks.push(await sha256Hex(planes.map((plane) => plane.subarray(at, at + LIVE_BYPASS_QUANTUM))));
    }
    return {
      digest: await sha256Hex(planes),
      blocks,
      energy,
      ...(section === undefined ? {} : { section }),
      ...(path === undefined ? {} : { path }),
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

/**
 * #1097 gate 4: a live bypass toggle on a console slot and on an insert, through the SDK.
 *
 * Each direction is checked: bypassing an enabled instance renders the document that authors it
 * bypassed, and lifting an authored bypass renders the document with none. The console slot is the
 * `post_insert` one, so its live address (console slot 1) crosses the section split.
 */
async function runSdkLiveBypassQualification(): Promise<Record<string, unknown>> {
  const base = await renderLiveBypass(undefined);
  const consoleOff = await renderLiveBypass("desk-hi");
  const insertOff = await renderLiveBypass("ins-mid");
  const consoleOffLive = await renderLiveBypass(undefined, { target: "desk-hi", bypass: true });
  const insertOffLive = await renderLiveBypass(undefined, { target: "ins-mid", bypass: true });
  const consoleOnLive = await renderLiveBypass("desk-hi", { target: "desk-hi", bypass: false });
  const insertOnLive = await renderLiveBypass("ins-mid", { target: "ins-mid", bypass: false });
  return {
    base: base.digest,
    consoleOff: consoleOff.digest,
    insertOff: insertOff.digest,
    consoleAddress: consoleOffLive.section,
    insertAddress: insertOffLive.section,
    liftAddresses: [consoleOnLive.section, insertOnLive.section],
    paths: [consoleOffLive, insertOffLive, consoleOnLive, insertOnLive].map((render) => render.path),
    baseEnergy: base.energy,
    quantumFrames: LIVE_BYPASS_QUANTUM,
    muteRampSamples: LIVE_BYPASS_RAMP_SAMPLES,
    blocks: {
      base: base.blocks,
      consoleOff: consoleOff.blocks,
      insertOff: insertOff.blocks,
      consoleOffLive: consoleOffLive.blocks,
      insertOffLive: insertOffLive.blocks,
      consoleOnLive: consoleOnLive.blocks,
      insertOnLive: insertOnLive.blocks,
    },
  };
}

/**
 * #1571 gates 2, 3 and 5, the browser half: `protocol-calls.mjs`'s sequence on the browser engine,
 * rendering its quanta in an OfflineAudioContext. `run.mjs` runs the same sequence on the headless
 * engine, booted in Node from the `document` returned here and the same module, and holds the two
 * equal.
 */
async function runSdkProtocolQualification(): Promise<Record<string, unknown>> {
  const frames = PROTOCOL_BLOCKS * LIVE_BYPASS_QUANTUM;
  const document = liveBypassSession();
  const browser = await createSdkEngine("protocol-messages", {
    document,
    policy: { sourceRingFrames: LIVE_BYPASS_FRAMES, liveControls: {} },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: LIVE_BYPASS_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "live-control-source", channels: 2, frames: BigInt(LIVE_BYPASS_FRAMES) }],
      tracks: ["track"],
      submixes: [],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, frames, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  try {
    const browserResult = await protocolCalls(
      browser,
      async () => {
        const controls = await browser.liveControls();
        return controls.submit(controls.edit.track("track").faderDb(-3));
      },
      async () => {
        await (browser.context as OfflineAudioContext).startRendering();
      },
    );
    return {
      document: document.toJson(),
      sourceRingFrames: LIVE_BYPASS_FRAMES,
      blocks: PROTOCOL_BLOCKS,
      browser: browserResult,
    };
  } catch (error) {
    throw sdkEngineFailed(browser, error);
  } finally {
    await closeSdkEngine(browser);
  }
}

/**
 * The raw qualification rows' live edit (#1563B): one matrix retarget of `trackId` as a `SET_CONTROL`
 * transaction, built and sent by the SDK through `host.control()` with the explicit ramp `0`, and
 * the core's report. `qualification.js` imports this from the served bundle so that it carries no
 * encoder of its own.
 */
export async function applyMatrixThroughSdk(
  host: MisoAudioWorkletHost,
  trackId: string,
  matrix: { readonly ll: number; readonly lr: number; readonly rl: number; readonly rr: number },
): Promise<{ ok: boolean; path: string; appliedOperations: number }> {
  const controls = await createBrowserLiveControls(host);
  const report = await controls.submit(controls.edit.track(trackId).matrix(matrix, { smoothingSamples: 0 }));
  return { ok: report.ok, path: report.path, appliedOperations: report.appliedOperations };
}

/**
 * Designate `stripId` the master with its `master` monitor row (#1689 D6): `strip.master(true)` as a
 * `SET_CONTROL` transaction through `host.control()`, a monitor write that spends no revision. The
 * observation row's master reading (`masterGrDb`) follows it.
 */
export async function designateMasterThroughSdk(
  host: MisoAudioWorkletHost,
  stripId: string,
): Promise<{ ok: boolean; path: string }> {
  const controls = await createBrowserLiveControls(host);
  const report = await controls.submit(controls.edit.track(stripId).master(true));
  return { ok: report.ok, path: report.path };
}

/**
 * One `delay_samples` edit of a track as a `SET_CONTROL` transaction (#1562b): the plan needs a new
 * latency, so the core classifies it as a rebuild, prepares a replacement plan and publishes it for
 * the render to adopt at a block boundary. Returns the core's report, with the committed revision.
 */
export async function applyDelayThroughSdk(
  host: MisoAudioWorkletHost,
  trackId: string,
  samples: number,
): Promise<{ ok: boolean; path: string; appliedOperations: number; revision: string }> {
  const controls = await createBrowserLiveControls(host);
  const report = await controls.submit(controls.edit.track(trackId).delaySamples(samples));
  return {
    ok: report.ok,
    path: report.path,
    appliedOperations: report.appliedOperations,
    revision: String(report.revision),
  };
}

/**
 * Send one structural command frame the protocol codec wrote (#1682: the frame that adds the first
 * stem) through the SDK's endpoint for the host. The SDK has no encoder for a structural edit, and
 * the engine's replay cache refuses a request ID twice, so the frame takes the next ID of the
 * host's one allocator and the committed revision, the two header words that depend on the engine.
 * Returns the core's verdict: its status, the edit path and the committed revision.
 */
export async function sendStructuralFrameThroughSdk(
  host: MisoAudioWorkletHost,
  hex: string,
  revision: string,
): Promise<{ status: string; path: string | undefined; revision: string; replyBytes: number }> {
  const transport = await createBrowserControlListTransport(host);
  const frame = Uint8Array.from(hex.match(/../g) ?? [], (byte) => Number.parseInt(byte, 16));
  const view = new DataView(frame.buffer);
  view.setBigUint64(24, transport.requestIds.next(), true);
  view.setBigUint64(32, BigInt(revision), true);
  const response = await transport.send(frame);
  const reply = decodeControlReply(response);
  return {
    status: reply.status,
    path: reply.applied?.editPath,
    revision: String(reply.revision),
    replyBytes: response.length,
  };
}

/** `SESSION_REPLACE`'s message ID (#1386, `crates/protocol/src/wire.rs`). */
const MESSAGE_SESSION_REPLACE = 0x0013;
/** The reply's edit-path codes (`crates/protocol`'s `EditPath`), as the SDK's reply decoder names them. */
const EDIT_PATH_NAMES = new Map([[1, "live"], [2, "model_only"], [3, "rebuild"], [4, "monitor"]]);

/**
 * Replace the host's running session with `document` (#1295, #1697): one `SESSION_REPLACE` frame,
 * its two fields the document's length and SHA-256, built with the SDK's frame encoder, its request
 * ID from the host's one allocator and its expected revision the queue's committed one, sent with
 * the document beside it through `host.control(frame, document)`. Returns the core's verdict and the
 * frame's bytes, which `run.mjs` sends to its Node reference unchanged.
 */
export async function replaceSessionThroughSdk(
  host: MisoAudioWorkletHost,
  document: Uint8Array,
): Promise<{
  status: string; path: string | undefined; appliedOperations: number | undefined; revision: string;
  expectedRevision: string; frameHex: string; replyBytes: number; diagnostics: readonly string[];
}> {
  const queue = await createBrowserControlQueue(host);
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", document));
  const expectedRevision = queue.revision;
  const frame = commandFrame(
    MESSAGE_SESSION_REPLACE,
    0,
    queue.requestIds.next(),
    expectedRevision,
    2,
    concat([field(1, WIRE_U32, true, u32(document.byteLength)), field(2, WIRE_BYTES, true, digest)]),
  );
  const frameHex = Array.from(frame, (byte) => byte.toString(16).padStart(2, "0")).join("");
  const reply = await host.control(frame, document);
  if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
  const { status, revision, fields } = decodeResponseEnvelope(reply.response, MESSAGE_SESSION_REPLACE);
  const ok = status === "ok";
  return {
    status,
    path: ok ? EDIT_PATH_NAMES.get(scalar(fields, 2, WIRE_U8, 1)?.getUint8(0) ?? 0) : undefined,
    appliedOperations: ok ? scalar(fields, 1, WIRE_U32, 4)?.getUint32(0, true) : undefined,
    revision: String(revision),
    expectedRevision: String(expectedRevision),
    frameHex,
    replyBytes: reply.response.length,
    diagnostics: ok ? [] : decodeRefusalBody(fields).diagnostics.map((row) => row.code),
  };
}

/**
 * The host's `sourceUnderrunBlocks` protocol counter (`COUNTERS_GET`, counter 20), through the
 * SDK's protocol client on the host's one request-ID allocator.
 */
export async function sourceUnderrunBlocksThroughSdk(host: MisoAudioWorkletHost): Promise<string> {
  const queue = await createBrowserControlQueue(host);
  const client = new ProtocolClient({
    send: async (frame) => {
      const reply = await host.control(frame);
      if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
      return reply.response;
    },
    requestIds: queue.requestIds,
    committedRevision: () => queue.revision,
  });
  const snapshot = await client.counters(["sourceUnderrunBlocks"]);
  const row = snapshot.values.find((value) => value.id === "sourceUnderrunBlocks");
  if (row === undefined) throw new Error("the counter snapshot holds no sourceUnderrunBlocks");
  return String(row.value);
}

/**
 * One `Observe` edit as a `SESSION_TRANSACTION_APPLY` (#1589b), posted as the host's `control`
 * message with the host's one request-ID allocator: the #1535 run arms and disarms its tap with it.
 * A failed call (a full tap queue) reports its result with no status; an answered one reports the
 * reply's status and, when applied, its edit path and the edit's arm tag.
 */
export async function observeThroughSdk(
  host: MisoAudioWorkletHost,
  edit: ObserveEdit,
): Promise<{ result: number; status: string | null; path: string | null; armTag: string | null }> {
  const queue = await createBrowserControlQueue(host);
  const frame = encodeObserveTransaction({
    requestId: queue.requestIds.next(),
    expectedRevision: queue.revision,
    edits: [edit],
  });
  const reply = await host.control(frame);
  queue.noteCommit(reply.revision);
  if (reply.result !== 0) return { result: reply.result, status: null, path: null, armTag: null };
  const decoded = decodeControlReply(reply.response);
  return {
    result: 0,
    status: decoded.status,
    path: decoded.applied?.editPath ?? null,
    armTag: decoded.applied?.observeArmTags[0]?.toString() ?? null,
  };
}

/**
 * One `OBSERVATION_READ` of one tap (#1590: the browser's own selected read is gone), built with
 * the SDK's frame encoder and posted as the host's `control` message with the host's one
 * request-ID allocator. Returns the call's result and the page's row count.
 */
export async function readObservationThroughSdk(
  host: MisoAudioWorkletHost,
  selection: ObservationFrameSelection,
): Promise<{ result: number; rows: number }> {
  const queue = await createBrowserControlQueue(host);
  const reply = await host.control(encodeObservationRead(queue.requestIds.next(), [selection]));
  queue.noteCommit(reply.revision);
  if (reply.result !== 0) return { result: reply.result, rows: 0 };
  const decoded = decodeObservationReadReply(reply.response);
  return { result: decoded.ok ? 0 : 1, rows: decoded.ok ? decoded.value.rows.length : 0 };
}

/**
 * The track IDs of the host's running session, in the core's own order (the owner headers of its
 * control list, #1564 D5). The worklet's session map no longer carries them (#1565B), and the
 * raw qualification rows read them here, through the SDK, so that they hold no reader of their own.
 */
export async function readTracksThroughSdk(host: MisoAudioWorkletHost): Promise<readonly string[]> {
  return (await createBrowserStripOrder(host)()).tracks;
}

/**
 * The host-level track response (#1580 D2): one strip `RESPONSE_GET` for `stripId`, built with the
 * SDK's frame encoder and posted as the host's `control` message with the host's one request-ID
 * allocator. The capture runs in a render step, so the caller renders and then calls `ready`, which
 * reads the reliable lane's `RESPONSE_READY` events until one carries the reply's tag or
 * `deadlineMs` passes, then drains once more, so a duplicate answer is counted too. Each wait step
 * sends a `status` message, after whose reply the worklet posts the events it drained. Returns the
 * reply's status and tag; `ready` returns every `RESPONSE_READY` read, the ones with the tag and
 * the first one's status. The event listener lives until `stop`, which the caller calls in a
 * `finally` (`ready` calls it too); a throw before this function returns stops it here.
 */
export async function requestTrackResponseThroughSdk(
  host: MisoAudioWorkletHost,
  stripId: string,
): Promise<{
  readonly result: number;
  readonly status: string | null;
  readonly ready: (deadlineMs: number) => Promise<{
    readonly events: number;
    readonly tagged: number;
    readonly status: string | null;
  }>;
  readonly stop: () => void;
}> {
  const events: { readonly tag: bigint; readonly status: string }[] = [];
  const stop = host.onControlEvent((event) => {
    if (event.lane !== 0) return;
    const ready = decodeResponseReady(event.frame);
    if (ready !== undefined) events.push({ tag: ready.requestTag, status: ready.status });
  });
  try {
    const queue = await createBrowserControlQueue(host);
    const frame = encodeStripResponseRequest({
      requestId: queue.requestIds.next(),
      stripId,
      grid: "linear",
      points: 5,
      minimumHz: 20,
      maximumHz: 20_000,
      channels: "both",
    });
    const reply = await host.control(frame);
    queue.noteCommit(reply.revision);
    const tag = reply.result === 0 ? decodeResponseTagReply(reply.response) : undefined;
    const tagged = () => tag?.ok === true ? events.filter((event) => event.tag === tag.tag) : [];
    return {
      result: reply.result,
      status: tag === undefined ? null : tag.ok ? "ok" : tag.status,
      ready: async (deadlineMs) => {
        try {
          const end = performance.now() + deadlineMs;
          while (tagged().length === 0 && performance.now() < end) {
            await host.status();
            await new Promise<void>((resolve) => { setTimeout(resolve, 5); });
          }
          // One more round trip: an event the worklet queued with the first is posted before this
          // reply, so a second answer to the same tag is counted rather than missed.
          await host.status();
        } finally {
          stop();
        }
        const matching = tagged();
        return { events: events.length, tagged: matching.length, status: matching[0]?.status ?? null };
      },
      stop,
    };
  } catch (error) {
    stop();
    throw error;
  }
}

export async function runSdkResponseQualification(): Promise<Record<string, unknown>> {
  sdkRenderAllocations.instances = 0;
  sdkRenderAllocations.rows.length = 0;
  sdkRenderAllocations.instanceModes.length = 0;
  sdkRenderAllocations.open.clear();
  const bytes = new Uint8Array(await (await fetch("/artifacts/miso-engine-v1-audio-worklet.simd128.wasm")).arrayBuffer());
  const asset = await MisoEngineAsset.load(bytes);
  const preview = await createResponsePreview({
    asset,
    responseWorkerModuleUrl: "/sdk/response-worker.js",
    responseLimits: { requestDeadlineMs: 5_000 },
  });
  let runFailure: object | undefined;
  try {
    const eq = await preview.query({
      configurationId: EQ_CONFIGURATION_ID,
      sampleRateHz: 48_000,
      quantumFrames: 128,
      configuration: effect("miso.parametric-eq", {
        "band-1-enabled": true,
        "band-1-kind": "bell",
        "band-1-frequency": 1_000,
        "band-1-gain": 3,
        "band-1-q": 1,
      }),
      grid: { kind: "logarithmic", points: 32, minimumHz: 20, maximumHz: 20_000 },
      channels: "both",
      fields: "totalAndSections",
    });
    const eqLeft = Array.from(eq.totalLeftDb ?? []);
    const filters = await preview.query({
      configurationId: EQ_CONFIGURATION_ID + 1n,
      sampleRateHz: 48_000,
      quantumFrames: 128,
      configuration: { kind: "inputFilters", left: { hpfHz: 80 }, right: { lpfHz: 12_000 } },
      grid: { kind: "linear", points: 16, minimumHz: 0, maximumHz: 24_000 },
      channels: "both",
      fields: "totalAndSections",
    });
    return {
      capabilities: preview.capabilities.map((row) => ({ owner: row.owner, target: row.target })),
      eq: {
        configurationId: eq.configurationId.toString(),
        mode: eq.mode,
        points: eq.frequenciesHz.length,
        sections: eq.sections.length,
        left: eqLeft,
        right: Array.from(eq.totalRightDb ?? []),
        frequencies: Array.from(eq.frequenciesHz),
        enabledLeft: Array.from(eq.enabledLeft),
        enabledRight: Array.from(eq.enabledRight),
      },
      filters: {
        configurationId: filters.configurationId.toString(),
        points: filters.frequenciesHz.length,
        sections: filters.sections.length,
        firstFrequency: filters.frequenciesHz[0],
        lastFrequency: filters.frequenciesHz[filters.frequenciesHz.length - 1],
        left: Array.from(filters.totalLeftDb ?? []),
        right: Array.from(filters.totalRightDb ?? []),
      },
      ownedAfterSecondQuery: eqLeft.every((value, index) => value === eq.totalLeftDb?.[index]),
      observations: await runSdkObservationQualification(),
      spectrum: await runSdkSpectrumQualification(),
      liveBypass: await runSdkLiveBypassQualification(),
      protocol: await runSdkProtocolQualification(),
      // Issue #1476 D2: read last, after every instance above has closed through closeSdkEngine.
      renderAllocations: sdkRenderAllocations.rows.slice(),
      renderAllocationInstances: sdkRenderAllocations.instances,
      instanceModes: sdkRenderAllocations.instanceModes.slice(),
    };
  } catch (error) {
    runFailure = sdkFailureObject(error);
    try {
      await closeOpenSdkEngines(runFailure);
    } finally {
      await preview.close();
    }
    throw reportSdkFailure(runFailure);
  } finally {
    // The success path; a failed run closed its instances above, once its error was recorded.
    if (runFailure === undefined) {
      try {
        await closeOpenSdkEngines(undefined);
      } catch (error) {
        throw reportSdkFailure(sdkFailureObject(error));
      } finally {
        await preview.close();
      }
    }
  }
}
