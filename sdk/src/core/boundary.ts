import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BufferKindName, ExportName, ResultCodeName } from "../generated/abi.ts";
import {
  StructView,
  constantValue,
  structBytes,
  writeBootOptions,
} from "./abi.ts";
import type { BootOptions } from "./abi.ts";
import { refuseRetiredObservationTaps } from "../internal/retired-options.ts";
import { MisoEngineAsset } from "./asset.ts";
import { controlCallFailure, encodeRevisionProbe, malformed } from "./control-frame.ts";
import { readControlStripOrderSync } from "./control-list.ts";
import type { ControlStripOrder } from "./control-list.ts";
import { ControlRequestIds } from "./control-queue.ts";
import { MisoEngineError, MisoUsageError, parseDiagnostics, resultName } from "./errors.ts";
import type { MisoDiagnostic } from "./errors.ts";
import { TrackResponseRequester } from "./live-response.ts";
import { watermarkSnapshot } from "./session-edit.ts";
import type { WatermarkSnapshot } from "./session-edit.ts";
import type { TrackResponseQuery, TrackResponseResult } from "./live-response.ts";
import { decodeCapabilitiesReply } from "./protocol-messages.ts";
import { cloneSpectrumCollection, cloneSpectrumQuery } from "./spectrum.ts";
import type {
  SpectrumCollection,
  SpectrumConfigureOptions,
  SpectrumConfigured,
  SpectrumHopFrames,
  SpectrumQuery,
  SpectrumResult,
  SpectrumStreamSelection,
  SpectrumStreamRead,
  SpectrumStreamStart,
} from "./spectrum.ts";
import {
  decodeSpectrumCaptureReply,
  decodeSpectrumConfiguredReply,
  decodeSpectrumStreamReply,
  encodeSpectrumCommand,
} from "./spectrum-frame.ts";
import {
  spectrumCallResult,
  spectrumConfigureRequest,
  spectrumConfiguredOf,
  spectrumMetadataOf,
  spectrumRefusalError,
  spectrumResultOfStream,
  spectrumResultOfWindow,
  spectrumSelection,
} from "./spectrum.ts";
import type { SpectrumFrameReply, SpectrumFrameRequest } from "./spectrum-frame.ts";

/**
 * The wasm boundary: one instance, one staging sequence, one live session.
 *
 * # The guess machine is gone
 *
 * The pre-boot-v1 SDK reached this point through nine operations over a bifurcated plan/raw path.
 * A raw JSON document was funnelled through `sessionHeader`, a regex over the text that could not
 * see a quoted key and would happily match a same-named key in a nested table; whatever it failed
 * to find became `validationFallback`'s silent 48 kHz / 128 frames; a throwaway `SessionPlan` was
 * fabricated solely to invent a ring, defaulting to 1024 -- not a multiple of a 127-frame quantum,
 * so a 96 kHz/127 session was structurally unbootable; and a 29-field configuration table was
 * written from all of it.
 *
 * Every one of those steps existed to *guess* facts the engine already knew. Boot v1 hands them
 * back: stage the bytes, boot, and ask. So the two input paths converge on one verb, and the guess
 * machine is deleted rather than relocated -- there is no `sessionHeader`, no fallback rate, no
 * fabricated plan and no `PrepareLimits` anywhere in this SDK.
 *
 * # Views are recreated, never cached
 *
 * `document_ptr` can grow the module's memory, and growing a `WebAssembly.Memory` **detaches**
 * every `ArrayBuffer` view over the old buffer. A cached `DataView` is therefore a use-after-free
 * waiting for a large document. Every accessor below builds its view at the moment of use.
 */

type ExportTable = Record<ExportName, (...args: (number | bigint)[]) => number | bigint> & {
  readonly memory: WebAssembly.Memory;
};

type ExportFunction = (...args: (number | bigint)[]) => number | bigint;
function exportsOf(instance: WebAssembly.Instance): ExportTable {
  const table = instance.exports as Record<string, unknown>;
  for (const name of ABI_LAYOUT.exports) {
    if (typeof table[name] !== "function") {
      throw new MisoEngineError(`the engine asset does not export ${name}`, {
        phase: "asset",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.asset.export", path: name }],
      });
    }
  }
  if (!(table.memory instanceof WebAssembly.Memory)) {
    throw new MisoEngineError("the engine asset does not export its linear memory", {
      phase: "asset",
      code: "abiMismatch",
      result: constantValue("resultCodes", "abiMismatch"),
      diagnostics: [{ code: "sdk.asset.export", path: "memory" }],
    });
  }
  return table as unknown as ExportTable;
}

function optionalExport(exports: ExportTable, name: string): ExportFunction | undefined {
  try {
    const value = (exports as Record<string, unknown>)[name];
    return typeof value === "function" ? value as ExportFunction : undefined;
  } catch {
    return undefined;
  }
}

/** Capability flag bit 16 of `CAPABILITIES_GET` (#1594 D10): the core serves the spectrum frames. */
const CAPABILITY_SPECTRUM = 1n << 16n;

function spectrumCapabilityRefusal(reason: string): MisoEngineError {
  return new MisoEngineError(`the engine cannot serve the spectrum frames: ${reason}`, {
    phase: "asset",
    code: "unsupported",
    result: constantValue("resultCodes", "unsupported"),
    diagnostics: [{ code: "sdk.asset.capability", path: "CAPABILITIES_GET.flags.spectrum" }],
  });
}

function validateSpectrumHopFrames(value: unknown): SpectrumHopFrames | undefined {
  if (value === undefined) return undefined;
  if (value === 256 || value === 512 || value === 1024 || value === 2048) {
    return value;
  }
  throw new MisoUsageError("spectrumHopFrames must be one of 256, 512, 1024, or 2048");
}

/** The answer of a spectrum call on a disposed engine: there is no session to ask. */
function unsupportedCall(): EngineCallResult {
  return Object.freeze({ ok: false, result: constantValue("resultCodes", "unsupported"), code: "unsupported" });
}

function spectrumKey(target: SpectrumQuery["target"], channels: string): string {
  return JSON.stringify([target.kind, target.kind === "output" ? target.outputId : target.trackId, channels]);
}

/**
 * The boot command of a boot's spectrum taps (#2166): one `SPECTRUM_CONFIGURE` command frame with
 * the policy's hop, staged through `boot_command_ptr` so the engine prepares the first plan with the
 * taps and the boot counts no blocking rebuild. The engine reads no request ID in it: a boot has
 * no reply. `undefined` without taps. @internal
 */
export function bootSpectrumCommand(
  configuration: SpectrumQuery | SpectrumCollection | undefined,
  hopFrames: SpectrumHopFrames | undefined,
): Uint8Array | undefined {
  if (configuration === undefined) return undefined;
  const { request } = spectrumConfigureRequest(configuration, hopFrames === undefined ? undefined : { hopFrames });
  return encodeSpectrumCommand(1n, request);
}

function wantsSpectrum(snapshot: BootSnapshot): boolean {
  return snapshot.spectrumQuery !== undefined || snapshot.spectrumCollection !== undefined;
}

interface BootSnapshot {
  readonly options: BootOptions;
  readonly spectrumHopFrames: SpectrumHopFrames | undefined;
  readonly spectrumQuery: SpectrumQuery | undefined;
  readonly spectrumCollection: SpectrumCollection | undefined;
}

function snapshotBootOptions(options: BootOptions): BootSnapshot {
  // Read and validate the plain preparation value before any caller-visible suspension. The
  // spectrum objects retain their existing deep copies, while the surrounding options object and
  // live-control words become private snapshots for the later instantiate/stage sequence.
  const spectrumHopFrames = validateSpectrumHopFrames(options.spectrumHopFrames);
  const spectrumQuery = options.spectrum === undefined ? undefined : cloneSpectrumQuery(options.spectrum);
  const spectrumCollection = options.spectrumCollection === undefined
    ? undefined : cloneSpectrumCollection(options.spectrumCollection);
  if (spectrumQuery !== undefined && spectrumCollection !== undefined) {
    throw new MisoUsageError("spectrum and spectrumCollection are mutually exclusive");
  }
  const liveControlOptions = options.liveControls === undefined ? undefined : (() => {
    // Issue #1689: the retired master boot word is refused, never ignored, for an untyped caller.
    if ((options.liveControls as { masterTrackPlusOne?: unknown }).masterTrackPlusOne !== undefined) {
      throw new MisoUsageError(
        "liveControls.masterTrackPlusOne was retired (issue #1689): designate the master after boot with strip.master(true)",
      );
    }
    refuseRetiredObservationTaps(options.liveControls);
    const { meterBlocks } = options.liveControls!;
    return Object.freeze({
      ...(meterBlocks === undefined ? {} : { meterBlocks }),
    });
  })();
  const bootOptions: BootOptions = Object.freeze({
    ...options,
    ...(liveControlOptions === undefined ? {} : { liveControls: liveControlOptions }),
    ...(spectrumHopFrames === undefined ? {} : { spectrumHopFrames }),
    ...(spectrumQuery === undefined ? {} : { spectrum: spectrumQuery }),
    ...(spectrumCollection === undefined ? {} : { spectrumCollection }),
  });
  return { options: bootOptions, spectrumHopFrames, spectrumQuery, spectrumCollection };
}

/** The shape the boot itself reported. Nothing here was read from the document's text. */
export interface SessionShape {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  /** The ring the engine resolved at boot, read back through its export (#1406b). */
  readonly sourceRingFrames: number;
  readonly backend: "scalar" | "simd128";
  readonly sources: readonly SourceShape[];
  readonly tracks: readonly string[];
  /** The submix IDs in canonical order (issue #1210 D3): submix `j` is strip `tracks.length + j`. */
  readonly submixes: readonly string[];
}

export interface SourceShape {
  readonly id: string;
  readonly channels: number;
  readonly frames: bigint;
}

/** One non-throwing engine call result. Refusals are answers, not exceptions. */
export interface EngineCallResult {
  readonly ok: boolean;
  readonly result: number;
  readonly code: ResultCodeName;
}

/** The compiled addressing map shared by the direct and MessagePort hosts. */
export interface SessionMap {
  readonly tracks: readonly string[];
  readonly sources: readonly SourceShape[];
  readonly metersAttached: boolean;
  /**
   * The submix IDs in canonical order (issue #1210 D3), the order of the meter frame's submix
   * sections. Submix `j` is strip `tracks.length + j`.
   */
  readonly submixes: readonly string[];
}

/** A direct snapshot of the frozen status structure. */
export interface EngineStatus {
  readonly state: number;
  readonly stateName: "ready" | "failed" | "disposed";
  readonly lastResult: number;
  readonly lastResultName: ResultCodeName;
  readonly backend: number;
  readonly backendName: "scalar" | "simd128";
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  /**
   * The session's count of live values a later write replaced before render read them
   * (`live_values_superseded`, decision 15 D15-2). It only grows, and one session counts across
   * its plans. A value on a strip that a plan swap starts at rest is not counted here: a
   * `COUNTERS_GET` sent with `controlFrame()` reports each such strip, once per adopted plan, as
   * `STRIPS_RESTARTED` (protocol counter 18, issue 1768).
   */
  readonly liveValuesSuperseded: bigint;
  readonly memoryBytes: number;
}

/**
 * The output latency and tail of the plan that renders now (#2116): the words of the C ABI's plan
 * resource report (`latency_samples`, `tail_kind`, `tail_samples`), the same on both hosts.
 */
export interface PlanLatency {
  /** The plan's output latency in samples at the session rate. */
  readonly samples: number;
  /** The plan's output tail in samples (it includes the latency); `null` for an unbounded tail. */
  readonly tailSamples: number | null;
}

/** The record words of a plan latency read as a `PlanLatency`; both engines decode through this. */
export function planLatencyOf(samples: bigint, tailKind: bigint, tailSamples: bigint): PlanLatency {
  const count = (name: string, value: bigint): number => {
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
      throw new MisoEngineError(`the plan's ${name} ${value} is above Number.MAX_SAFE_INTEGER`, {
        phase: "lifecycle",
        code: "internal",
        result: constantValue("resultCodes", "internal"),
      });
    }
    return Number(value);
  };
  if (tailKind !== 0n && tailKind !== 1n) {
    throw new MisoEngineError(`the plan latency record has tail kind ${tailKind}`, {
      phase: "lifecycle",
      code: "internal",
      result: constantValue("resultCodes", "internal"),
    });
  }
  return Object.freeze({
    samples: count("latency", samples),
    tailSamples: tailKind === 1n ? null : count("tail", tailSamples),
  });
}

/** One copied decimated meter window. No field retains a view onto mutable Wasm memory. */
export interface MeterFrame {
  readonly tag: "miso.meter.v1";
  readonly sequence: bigint;
  readonly windows: number;
  readonly trackCount: number;
  /**
   * The structure epoch of the control list of the plan that rendered this frame (#1562b). The
   * frame names its own strips (`trackIds`, `submixIds`) by the order of that epoch: the newest
   * list's, except between a rebuild's commit and the block that adopts its replacement, when the
   * plan that still renders keeps the previous list.
   */
  readonly structureEpoch: bigint;
  /**
   * Meter windows this instance dropped before this frame because no list of their structure epoch
   * was held or could be read (the frame could not be named), counted over the instance's life. It
   * only grows; a gap in `sequence` that this count explains is not a host loss.
   */
  readonly unnamedWindows: bigint;
  /** The tracks of the frame, in the order of its arrays: the strip order of `structureEpoch`. */
  readonly trackIds: readonly string[];
  /** The submix strips of the frame, in the order of its submix arrays. */
  readonly submixIds: readonly string[];
  /** `[track0 L, track0 R, .., master L, master R]`. */
  readonly peaks: Float32Array;
  readonly trackGrDb: Float32Array;
  readonly masterGrDb: number | null;
  readonly firstSample: bigint;
  readonly endSample: bigint;
  /** Submix strips (buses) the frame carries, in canonical submix order (issue #1209). */
  readonly submixCount: number;
  /** `[bus0 L, bus0 R, ..]`, `2 * submixCount` long. */
  readonly submixPeaks: Float32Array;
  /** One non-negative decibel magnitude per bus. */
  readonly submixGrDb: Float32Array;
}

/** The error of a call on a disposed engine, and of a track response query it left pending. */
const DISPOSED = "this engine was disposed; boot a new one";

/** A staged-and-booted engine instance. */
export class WasmBoundary {
  readonly #exports: ExportTable;
  #handle: number;
  readonly #controlListeners = new Set<(event: ControlEvent) => void>();
  #discardedControlEvents = 0;
  #controlListenerFailures = 0;
  #optionBytes: Uint8Array;
  #metersAttached: boolean;
  /** The one request-ID allocator of this instance's control endpoint (the core refuses a reused ID). */
  readonly #controlRequestIds = new ControlRequestIds();
  /** The strip order read last, with the structure epoch it belongs to. */
  #stripOrder: ControlStripOrder | undefined;
  /**
   * The orders read, by structure epoch, newest last. The plan that renders keeps naming its strips
   * by the list it was prepared from until the block that adopts a rebuild's replacement, so a meter
   * frame is named by the order of the epoch it carries (#1562b).
   */
  readonly #stripOrders = new Map<bigint, ControlStripOrder>();
  /** Meter windows dropped because no list named their structure epoch. */
  #unnamedMeterWindows = 0n;
  #trackResponse: TrackResponseRequester | undefined;
  #spectrumQuery: SpectrumQuery | undefined;
  #spectrumCollection: SpectrumCollection | undefined;
  #spectrumActiveQuery: SpectrumQuery | undefined;

  private constructor(
    exports: ExportTable,
    handle: number,
    optionBytes: Uint8Array,
    metersAttached: boolean,
    spectrumQuery: SpectrumQuery | undefined,
    spectrumCollection: SpectrumCollection | undefined,
  ) {
    this.#exports = exports;
    this.#handle = handle;
    this.#optionBytes = optionBytes;
    this.#metersAttached = metersAttached;
    this.#spectrumQuery = spectrumQuery;
    this.#spectrumCollection = spectrumCollection;
    this.#spectrumActiveQuery = spectrumQuery;
  }

  /**
   * The four-call staging sequence, in the order the generated document names it:
   * `abi_version` -> `boot_options_ptr` -> `document_ptr` -> `boot`.
   *
   * Not three: the options block must be *addressed* before it can be written, and a shorthand
   * that omits the address is what let five hand-written copies of the configuration table drift.
   */
  static async boot(
    asset: MisoEngineAsset,
    document: Uint8Array,
    options: BootOptions = {},
  ): Promise<WasmBoundary> {
    const snapshot = snapshotBootOptions(options);
    return WasmBoundary.#boot(asset, document, snapshot, bootCommandOf(snapshot));
  }

  /**
   * Boot with `bootCommand` staged as given (#2166), as the worklet stages the command the browser
   * engine hands its host. The browser engine's test host boots its core through this. @internal
   */
  static async bootWithCommand(
    asset: MisoEngineAsset,
    document: Uint8Array,
    options: BootOptions,
    bootCommand: Uint8Array | undefined,
  ): Promise<WasmBoundary> {
    return WasmBoundary.#boot(asset, document, snapshotBootOptions(options), bootCommand);
  }

  static async #boot(
    asset: MisoEngineAsset,
    document: Uint8Array,
    snapshot: BootSnapshot,
    bootCommand: Uint8Array | undefined,
  ): Promise<WasmBoundary> {
    const exports = exportsOf(await asset.instantiate());
    const staged = stage(exports, document, snapshot.options, bootCommand);
    const boundary = new WasmBoundary(
      exports,
      staged.handle,
      staged.optionBytes,
      (snapshot.options.liveControls?.meterBlocks ?? 0) > 0,
      snapshot.spectrumQuery,
      snapshot.spectrumCollection,
    );
    try {
      if (wantsSpectrum(snapshot)) boundary.#requireSpectrumCapability();
    } catch (error) {
      boundary.dispose();
      throw error;
    }
    return boundary;
  }

  /**
   * The headless mix switch: dispose this session and boot another on the **same instance**.
   *
   * An instance is single-live-handle, and `dispose` resets the staging block to its zero
   * defaults, so restaging is exactly what the ABI supports (issue #243 S2(a)). This is the one
   * exposed reboot verb. A caller can of course call `createOfflineEngine` again instead, but that
   * is a *different* operation -- a fresh instance with a fresh linear memory, which pays another
   * instantiation and throws away the module's warmed allocator -- and it is deliberately not
   * dressed up as the mix switch. There is no `reprepare()` and no in-place session replacement:
   * boot v1 has no verb that mutates a live session's document.
   */
  reboot(document: Uint8Array, options: BootOptions = {}): void {
    const snapshot = snapshotBootOptions(options);
    // A capability refusal happens while the current handle is still live: the module, and so its
    // answer, is the one the next session boots on.
    if (wantsSpectrum(snapshot) && this.#handle !== 0) this.#requireSpectrumCapability();
    this.dispose();
    this.#stripOrder = undefined;
    this.#stripOrders.clear();
    const staged = stage(this.#exports, document, snapshot.options, bootCommandOf(snapshot));
    this.#handle = staged.handle;
    this.#optionBytes = staged.optionBytes;
    this.#metersAttached = (snapshot.options.liveControls?.meterBlocks ?? 0) > 0;
    this.#spectrumQuery = snapshot.spectrumQuery;
    this.#spectrumCollection = snapshot.spectrumCollection;
    this.#spectrumActiveQuery = snapshot.spectrumQuery;
    try {
      if (wantsSpectrum(snapshot)) this.#requireSpectrumCapability();
    } catch (error) {
      this.dispose();
      throw error;
    }
  }

  /** Refuse, with `sdk.asset.capability`, an engine whose `CAPABILITIES_GET` lacks flag bit 16. */
  #requireSpectrumCapability(): void {
    const requestId = this.#controlRequestIds.next();
    const reply = decodeCapabilitiesReply(this.controlFrame(encodeRevisionProbe(requestId)));
    if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
    if (!reply.ok) throw spectrumCapabilityRefusal(`CAPABILITIES_GET was refused with ${reply.status}`);
    if ((reply.value.flags & CAPABILITY_SPECTRUM) === 0n) {
      throw spectrumCapabilityRefusal("capability flag bit 16 is clear");
    }
  }

  /** Send one spectrum command through the control entry and decode its reply. */
  #spectrumCommand<T>(
    request: SpectrumFrameRequest,
    decode: (bytes: Uint8Array) => SpectrumFrameReply<T>,
  ): SpectrumFrameReply<T> {
    const requestId = this.#controlRequestIds.next();
    const reply = decode(this.controlFrame(encodeSpectrumCommand(requestId, request)));
    if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
    return reply;
  }

  /** The exact bytes written to the options block, for the scratch/worklet equality rule. */
  get optionBytes(): Uint8Array {
    return this.#optionBytes.slice();
  }

  get disposed(): boolean {
    return this.#handle === 0;
  }

  #live(): number {
    if (this.#handle === 0) {
      throw new MisoUsageError(DISPOSED);
    }
    return this.#handle;
  }

  #status(): StructView {
    return new StructView(
      this.#exports.memory,
      "status",
      Number(this.#exports.miso_engine_web_v1_status_ptr(this.#live())),
    );
  }

  /** The whole status structure, with numeric ABI words and their generated names together. */
  status(): EngineStatus {
    const status = this.#status();
    const state = status.u32("state");
    const backend = status.u32("backend");
    const stateName = ABI_LAYOUT.constants.states.find((row) => row.value === state)?.name;
    if (stateName === undefined) {
      throw new MisoUsageError(`the engine reported an unknown state word ${state}`);
    }
    const backendName = ABI_LAYOUT.constants.backends.find((row) => row.value === backend)?.name;
    if (backendName === undefined) {
      throw new MisoUsageError(`the engine reported an unknown backend word ${backend}`);
    }
    const lastResult = status.u32("lastResult");
    return Object.freeze({
      state,
      stateName,
      lastResult,
      lastResultName: resultName(lastResult, "call"),
      backend,
      backendName,
      sampleRateHz: status.u32("sampleRateHz"),
      quantumFrames: status.u32("quantumFrames"),
      nextAbsoluteSample: status.u64("nextAbsoluteSample"),
      renderedQuanta: status.u64("renderedQuanta"),
      liveValuesSuperseded: status.u64("liveValuesSuperseded"),
      memoryBytes: this.#exports.memory.buffer.byteLength,
    });
  }

  /** The engine's own answer about what it compiled. No number here came from the document text. */
  shape(): SessionShape {
    const handle = this.#live();
    const status = this.#status();
    const sampleRateHz = status.u32("sampleRateHz");
    const quantumFrames = status.u32("quantumFrames");
    const backendValue = status.u32("backend");
    const sourceCount = Number(this.#exports.miso_engine_web_v1_source_count(handle));
    const sources: SourceShape[] = [];
    for (let index = 0; index < sourceCount; index += 1) {
      const idBytes = Number(this.#exports.miso_engine_web_v1_source_id(handle, index));
      sources.push(Object.freeze({
        id: this.#readIdBuffer(idBytes),
        channels: Number(this.#exports.miso_engine_web_v1_source_channels(handle, index)),
        frames: BigInt(this.#exports.miso_engine_web_v1_source_frames(handle, index)),
      }));
    }
    const { tracks, submixes } = this.stripOrder();
    return Object.freeze({
      sampleRateHz,
      quantumFrames,
      // The ring the boot resolved, read back from the engine (#1406b): an override as given, or on
      // a zero override the engine's default ring.
      sourceRingFrames: Number(this.#exports.miso_engine_web_v1_resolved_source_ring_frames(handle)),
      backend: backendValue === constantValue("backends", "simd128") ? "simd128" : "scalar",
      sources: Object.freeze(sources),
      tracks,
      submixes,
    });
  }

  /**
   * The one allocator of request IDs for this instance's control endpoint. Every frame sent through
   * {@link controlFrame} draws its ID from it: the core takes IDs in ascending order and refuses a
   * reused one, so two allocators on one core collide.
   */
  get controlRequestIds(): ControlRequestIds {
    return this.#controlRequestIds;
  }

  /**
   * The strips of the running session in the core's own order: the owner headers of its control list
   * (decision 17, C4; #1564 D5), tracks then submixes. A probe reads the structure epoch; the list is
   * read again only when the epoch is not the one the cached order belongs to.
   */
  stripOrder(): ControlStripOrder {
    this.#live();
    this.#stripOrder = readControlStripOrderSync(
      { send: (frame) => this.controlFrame(frame), requestIds: this.#controlRequestIds },
      this.#stripOrder,
    );
    this.#holdStripOrder(this.#stripOrder);
    return this.#stripOrder;
  }

  /** The order of the control list whose structure epoch is `epoch`, if this boundary read it. */
  stripOrderOf(epoch: bigint): ControlStripOrder | undefined {
    return this.#stripOrders.get(epoch);
  }

  #holdStripOrder(order: ControlStripOrder): void {
    this.#stripOrders.delete(order.structureEpoch);
    this.#stripOrders.set(order.structureEpoch, order);
    while (this.#stripOrders.size > HELD_STRIP_ORDERS) {
      const oldest = this.#stripOrders.keys().next();
      if (oldest.done) break;
      this.#stripOrders.delete(oldest.value);
    }
  }

  /** The engine's state word, named through the generated vocabulary. */
  state(): "ready" | "failed" | "disposed" {
    // Disposal clears the handle, so there is deliberately no engine status pointer left to ask.
    // The public union has always included `disposed`; answer it from boundary ownership before
    // applying the live-handle guard used by every operation that actually reaches Wasm.
    if (this.disposed) return "disposed";
    return this.status().stateName;
  }

  /** Absolute sample the next render begins at. */
  nextAbsoluteSample(): bigint {
    return this.#status().u64("nextAbsoluteSample");
  }

  renderedQuanta(): bigint {
    return this.#status().u64("renderedQuanta");
  }

  /** The retained-resource projection, every field named. */
  resources(): Readonly<Record<string, number | bigint | readonly number[]>> {
    return new StructView(
      this.#exports.memory,
      "resourceReport",
      Number(this.#exports.miso_engine_web_v1_resource_ptr(this.#live())),
    ).snapshot();
  }

  #buffer(kind: BufferKindName): { pointer: number; capacity: number } {
    const value = constantValue("bufferKinds", kind);
    return {
      pointer: Number(this.#exports.miso_engine_web_v1_buffer_ptr(this.#live(), value)),
      capacity: Number(this.#exports.miso_engine_web_v1_buffer_capacity(this.#live(), value)),
    };
  }

  #readIdBuffer(length: number): string {
    if (length === 0) return "";
    const { pointer } = this.#buffer("sourceId");
    // `.slice()`: the module's memory is shared (#1380), and a decoder is not required to read a
    // view of a `SharedArrayBuffer`; the copy is the id's own bytes.
    return new TextDecoder().decode(new Uint8Array(this.#exports.memory.buffer, pointer, length).slice());
  }

  #stageSourceId(sourceId: string): number {
    if (typeof sourceId !== "string" || sourceId.length === 0) {
      throw new MisoUsageError("sourceId must be a nonempty string");
    }
    const id = new TextEncoder().encode(sourceId);
    const buffer = this.#buffer("sourceId");
    if (id.byteLength > buffer.capacity) {
      throw new MisoUsageError(
        `source id ${sourceId} is ${id.byteLength} bytes; staging holds ${buffer.capacity}`,
      );
    }
    new Uint8Array(this.#exports.memory.buffer, buffer.pointer, id.byteLength).set(id);
    return id.byteLength;
  }

  /**
   * The canonical track/source/submix order in the engine's own order, and whether meter
   * observers were prepared.
   */
  sessionMap(): SessionMap {
    const shape = this.shape();
    return Object.freeze({
      tracks: shape.tracks,
      sources: shape.sources,
      metersAttached: this.#metersAttached,
      submixes: shape.submixes,
    });
  }

  /** Internal managed-subscription view of the optional prepared spectrum query. */
  preparedSpectrumQuery(): SpectrumQuery | undefined {
    return this.#spectrumQuery;
  }

  /** Internal managed-subscription view of the optional prepared spectrum collection. */
  preparedSpectrumCollection(): SpectrumCollection | undefined {
    return this.#spectrumCollection;
  }

  /**
   * The response of one track, as a C ABI host reads it (#1579a D1, D2): a strip `RESPONSE_GET`
   * through the control entry, then the `RESPONSE_READY` with its tag from the event lane. The core
   * captures in its render step, so the promise settles after the next `render()`, at the latest
   * when the request deadline passes. Queries run one at a time, in call order.
   * A query made from inside an event callback sends its frame there, which runs a nested event
   * drain: the events still queued reach the other listeners before the callback returns.
   */
  queryTrackResponse(request: TrackResponseQuery): Promise<TrackResponseResult> {
    try {
      this.#live();
    } catch (error) {
      return Promise.reject(error);
    }
    this.#trackResponse ??= new TrackResponseRequester({
      send: (frame) => this.controlFrame(frame),
      onControlEvent: (listener) => this.onControlEvent(listener),
      nextRequestId: () => this.#controlRequestIds.next(),
    });
    return this.#trackResponse.query(request);
  }

  /**
   * Set, change or remove the spectrum taps of the running session (`SPECTRUM_CONFIGURE`, #1596
   * D5). A change of taps prepares a replacement plan, adopted at a later render block: the reply
   * reports `rebuild` and the epochs, and the taps are in effect once a spectrum reply reports
   * `adoptedEpoch === configuredEpoch`. A core refusal throws `MisoEngineError` whose
   * `diagnosticCode` is the core's code (`spectrum.target.unknown`, `spectrum.budget`, ...).
   */
  configureSpectrum(
    configuration: SpectrumQuery | SpectrumCollection,
    options?: SpectrumConfigureOptions,
  ): SpectrumConfigured {
    this.#live();
    const { request, query, collection } = spectrumConfigureRequest(configuration, options);
    const reply = this.#spectrumCommand(request, decodeSpectrumConfiguredReply);
    if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum configuration", reply);
    this.#spectrumQuery = query;
    this.#spectrumCollection = collection;
    const active = this.#spectrumActiveQuery;
    // A collection keeps the selection of an entry it still holds (#1594 D6); a single form selects
    // its one entry.
    this.#spectrumActiveQuery = query ?? (active !== undefined && collection !== undefined
      && collection.entries.some((entry) =>
        spectrumKey(entry.target, entry.channels ?? "both") === spectrumKey(active.target, active.channels ?? "both"))
      ? active : undefined);
    return spectrumConfiguredOf(reply.value);
  }

  /** Arm the selected spectrum capture for its next complete window (`SPECTRUM_CAPTURE` arm). */
  armSpectrum(): EngineCallResult {
    if (this.#handle === 0) return unsupportedCall();
    return spectrumCallResult(this.#spectrumCommand({ op: "capture", capture: "arm" }, decodeSpectrumCaptureReply));
  }

  /** Select one exact entry of the configured collection (`SPECTRUM_CAPTURE` select). */
  selectSpectrum(query: SpectrumQuery): EngineCallResult {
    if (this.#handle === 0) return unsupportedCall();
    const selected = spectrumSelection(query);
    const reply = this.#spectrumCommand({
      op: "capture",
      capture: "select",
      target: selected.target,
      channels: selected.channels ?? "both",
    }, decodeSpectrumCaptureReply);
    if (reply.ok) this.#spectrumActiveQuery = selected;
    return spectrumCallResult(reply);
  }

  /**
   * Select one exact entry and set the running stream's smoothing (`SPECTRUM_STREAM` select): the
   * core restarts the stream on the entry and reports its metadata.
   */
  selectSpectrumStream(query: SpectrumQuery, smoothingMs: number): SpectrumStreamSelection {
    if (this.#handle === 0) return unsupportedCall();
    const selected = spectrumSelection(query);
    const reply = this.#spectrumCommand({
      op: "stream",
      stream: "select",
      target: selected.target,
      channels: selected.channels ?? "both",
      smoothingMs,
    }, decodeSpectrumStreamReply);
    if (!reply.ok) return spectrumCallResult(reply);
    this.#spectrumActiveQuery = selected;
    return Object.freeze({ ...spectrumCallResult(reply), metadata: spectrumMetadataOf(reply.value) });
  }

  /** Read the completed one-shot window, analyzed; `undefined` means it is still pending. */
  readSpectrum(): SpectrumResult | undefined {
    if (this.#handle === 0) throw new MisoUsageError("this engine has no prepared spectrum boundary: it was disposed");
    const reply = this.#spectrumCommand({
      op: "capture",
      capture: "read",
      channels: this.#spectrumActiveQuery?.channels ?? "both",
    }, decodeSpectrumCaptureReply);
    if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum capture read", reply, "lifecycle");
    if (reply.value.status === "pending") return undefined;
    if (reply.value.window === undefined) throw malformed("a spectrum read answered done");
    return spectrumResultOfWindow(reply.value.window);
  }

  /** Start the continuous stream on the selected entry (`SPECTRUM_STREAM` start). */
  startSpectrumStream(smoothingMs = 100, _query?: SpectrumQuery): SpectrumStreamStart {
    this.#live();
    const reply = this.#spectrumCommand({ op: "stream", stream: "start", smoothingMs }, decodeSpectrumStreamReply);
    if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum stream start", reply);
    return Object.freeze({ ...spectrumCallResult(reply), metadata: spectrumMetadataOf(reply.value) });
  }

  /** Read the stream once: its metadata, and with `ready` the smoothed window (`SPECTRUM_STREAM` read). */
  readSpectrumStream(_query?: SpectrumQuery): SpectrumStreamRead {
    this.#live();
    const reply = this.#spectrumCommand({ op: "stream", stream: "read" }, decodeSpectrumStreamReply);
    if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum stream read", reply);
    const metadata = spectrumMetadataOf(reply.value);
    const result = spectrumResultOfStream(reply.value);
    return Object.freeze(result === undefined ? { metadata } : { metadata, result });
  }

  /** Stop the continuous stream (`SPECTRUM_STREAM` stop). */
  stopSpectrumStream(): EngineCallResult {
    if (this.#handle === 0) return unsupportedCall();
    return spectrumCallResult(this.#spectrumCommand({ op: "stream", stream: "stop" }, decodeSpectrumStreamReply));
  }

  /** Cancel a pending one-shot capture (`SPECTRUM_CAPTURE` cancel). */
  cancelSpectrum(): EngineCallResult {
    if (this.#handle === 0) return unsupportedCall();
    return spectrumCallResult(this.#spectrumCommand({ op: "capture", capture: "cancel" }, decodeSpectrumCaptureReply));
  }

  /**
   * Submit one quantum-sized block of planar source PCM.
   *
   * The engine owns the staging buffer; the caller's planes are copied into it and never retained.
   */
  submitSource(request: {
    readonly sourceId: string;
    readonly generation: bigint;
    readonly startFrame: bigint;
    readonly planes: readonly Float32Array[];
    readonly endOfRegion: boolean;
  }): EngineCallResult {
    const handle = this.#live();
    const idBytes = this.#stageSourceId(request.sourceId);

    const frames = request.planes[0]?.length ?? 0;
    for (const plane of request.planes) {
      if (plane.length !== frames) {
        throw new MisoUsageError("every source plane must carry the same frame count");
      }
    }
    const pcm = this.#buffer("sourcePcm");
    const needed = frames * request.planes.length * 4;
    if (needed > pcm.capacity) {
      throw new MisoUsageError(
        `${request.planes.length} planes of ${frames} frames need ${needed} bytes; `
        + `staging holds ${pcm.capacity}`,
      );
    }
    const staging = new Float32Array(this.#exports.memory.buffer, pcm.pointer, frames * request.planes.length);
    request.planes.forEach((plane, index) => {
      staging.set(plane, index * frames);
    });

    const result = Number(this.#exports.miso_engine_web_v1_source_submit(
      handle,
      idBytes,
      request.generation,
      request.startFrame,
      request.planes.length,
      frames,
      request.endOfRegion ? 1 : 0,
    ));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Seek one streamed source. The new generation invalidates every earlier queued chunk. */
  seekSource(request: {
    readonly sourceId: string;
    readonly generation: bigint;
    readonly sourceFrame: bigint;
  }): EngineCallResult {
    const handle = this.#live();
    if (typeof request.generation !== "bigint" || request.generation <= 0n) {
      throw new MisoUsageError("source seek generation must be a positive bigint");
    }
    if (typeof request.sourceFrame !== "bigint" || request.sourceFrame < 0n) {
      throw new MisoUsageError("sourceFrame must be a non-negative bigint");
    }
    const idBytes = this.#stageSourceId(request.sourceId);
    const result = Number(this.#exports.miso_engine_web_v1_source_seek(
      handle,
      idBytes,
      request.generation,
      request.sourceFrame,
    ));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /**
   * Declare a discontinuity: the next block need not continue the previous output, so a latency
   * floor a structural edit raised is reset as it is on the C ABI. Control work: call it from the
   * message handler, never from `process()`.
   */
  declareDiscontinuity(): EngineCallResult {
    const result = Number(this.#exports.miso_engine_web_v1_declare_discontinuity(this.#live()));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /**
   * Seek the timeline and every source to `frame` under one new `generation`, all or nothing.
   * A plain session seek is a declared discontinuity: the engine declares it in the same call, and
   * a refused seek declares nothing, so this makes no separate declaration. The seek does not
   * prepare the consumers: each applies it at the next block, and the caller then submits every
   * source's PCM from its clamped frame under the same generation. Control work, never
   * `process()`.
   */
  sessionSeek(request: {
    readonly generation: bigint;
    readonly frame: bigint;
  }): EngineCallResult {
    const handle = this.#live();
    if (typeof request.generation !== "bigint" || request.generation <= 0n) {
      throw new MisoUsageError("session seek generation must be a positive bigint");
    }
    if (typeof request.frame !== "bigint" || request.frame < 0n) {
      throw new MisoUsageError("session seek frame must be a non-negative bigint");
    }
    const result = Number(this.#exports.miso_engine_web_v1_session_seek(
      handle,
      request.generation,
      request.frame,
    ));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Take or release the decimated meter lease. */
  meterLease(enabled: boolean): EngineCallResult {
    if (typeof enabled !== "boolean") {
      throw new MisoUsageError("meter lease enabled must be boolean");
    }
    const result = Number(this.#exports.miso_engine_web_v1_meter_lease(
      this.#live(),
      enabled ? 1 : 0,
    ));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Drain completed meter windows, copying their values out of mutable Wasm memory. */
  pollMeters(): MeterFrame | undefined {
    const handle = this.#live();
    // The strip order the last read of the core's list gave; read once, then only when a frame's
    // header names another structure epoch (a poll is a per-block call and must not probe the core
    // each time).
    const known = this.#stripOrder ?? this.stripOrder();
    const windows = Number(this.#exports.miso_engine_web_v1_meter_poll(handle));
    if (windows === 0) return undefined;

    const readHeader = (): StructView => new StructView(
      this.#exports.memory,
      "meterHeader",
      Number(this.#exports.miso_engine_web_v1_meter_header_ptr(handle)),
    );
    let header = readHeader();
    // The frame names its strips by the list of the plan that rendered it (#1562b): the epoch in its
    // header, which is the newest list's except while a rebuild's replacement waits for its block.
    const structureEpoch = header.u64("structureEpoch");
    let order = known.structureEpoch === structureEpoch ? known : this.#stripOrders.get(structureEpoch);
    if (order === undefined) {
      // The structure may have changed since the order was read. A control call can grow the
      // module's memory, so the header view is taken again after it.
      const fresh = this.stripOrder();
      header = readHeader();
      order = fresh.structureEpoch === structureEpoch ? fresh : this.#stripOrders.get(structureEpoch);
    }
    // The list of that epoch was never read and is gone: the frame cannot be named, and a frame
    // named by another list would key readings by the wrong strips. The browser drops it the same
    // way (`createMeasurementFeeds`). The windows are counted, never lost without a trace: the next
    // frame reports the count.
    if (order === undefined) {
      this.#unnamedMeterWindows += BigInt(windows);
      return undefined;
    }
    const trackCount = order.tracks.length;
    const reportedWindows = header.u32("windows");
    const masterGrPresent = header.u32("masterGrPresent");
    const submixCount = header.u32("submixCount");
    if (header.u32("structSize") !== structBytes("meterHeader")
      || header.u32("abiVersion") !== ABI_LAYOUT.abiVersion
      || header.u32("trackCount") !== trackCount
      || header.u32("submixCount") !== order.submixes.length
      || reportedWindows !== windows
      || masterGrPresent > 1) {
      throw new MisoEngineError("the engine returned a malformed meter frame header", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.meter.header", path: "meterHeader" }],
      });
    }

    const buffer = this.#buffer("meterFrame");
    // Issue #1209 D1: `3(T + S) + 3` words -- a peak pair per strip (tracks, then submixes), the
    // master pair, one gain-reduction word per strip in the same order, the master's.
    const strips = trackCount + submixCount;
    const words = strips * 3 + 3;
    if (buffer.pointer === 0 || buffer.capacity !== words * Float32Array.BYTES_PER_ELEMENT) {
      throw new MisoEngineError("the engine returned a malformed meter frame buffer", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.meter.frame", path: "meterFrame" }],
      });
    }
    const frame = new Float32Array(this.#exports.memory.buffer, buffer.pointer, words);
    const trackPeakWords = trackCount * 2;
    const masterPeak = strips * 2;
    const gainBase = masterPeak + 2;
    // `peaks` keeps its `2T + 2` shape: the track words, then the master pair, which sits after
    // the submix peaks in the frame.
    const peaks = new Float32Array(trackPeakWords + 2);
    peaks.set(frame.subarray(0, trackPeakWords));
    peaks.set(frame.subarray(masterPeak, gainBase), trackPeakWords);
    return Object.freeze({
      tag: "miso.meter.v1" as const,
      sequence: header.u64("sequence"),
      windows,
      trackCount,
      structureEpoch,
      unnamedWindows: this.#unnamedMeterWindows,
      trackIds: order.tracks,
      submixIds: order.submixes,
      peaks,
      trackGrDb: frame.slice(gainBase, gainBase + trackCount),
      masterGrDb: masterGrPresent === 1 ? frame[words - 1]! : null,
      firstSample: header.u64("firstSample"),
      endSample: header.u64("endSample"),
      submixCount,
      submixPeaks: frame.slice(trackPeakWords, masterPeak),
      submixGrDb: frame.slice(gainBase + trackCount, gainBase + strips),
    });
  }

  /** Render one quantum. Returns the two output planes, copied out of engine memory. */
  render(actualFrames: number): { readonly left: Float32Array; readonly right: Float32Array } {
    const handle = this.#live();
    const result = Number(this.#exports.miso_engine_web_v1_render(handle, actualFrames));
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError(`the engine refused to render ${actualFrames} frames`, {
        phase: "render",
        code: resultName(result, "call"),
        result,
      });
    }
    const { pointer } = this.#buffer("outputPcm");
    const quantum = this.#status().u32("quantumFrames");
    const contiguous = new Float32Array(this.#exports.memory.buffer, pointer, quantum * 2);
    const output = Object.freeze({
      left: contiguous.slice(0, actualFrames),
      right: contiguous.slice(quantum, quantum + actualFrames),
    });
    // A block produces events (telemetry, render diagnostics, a transport change it applied), and
    // the lossy lane holds one: they are read here, after each block, or a later block's would
    // replace them before the next control frame read them (#1571 D4). The read runs the core's
    // service step first, so the headless engine needs no other service call.
    this.#drainControlEvents(handle);
    return output;
  }

  /**
   * Run one command frame through the core's control entry and return the reply frame (#1562 D1).
   *
   * The entry is the same `SessionState::command` the C ABI calls: the frame is a complete BTLV
   * command and the reply a complete BTLV response, refusals included. The four exports are looked
   * up when called, not at load, because a module built before #1562 does not export them; such a
   * module refuses here with the typed `unsupported`.
   *
   * A malformed reply after the core committed rejects although the edit was applied; values are
   * absolute, so a resend is safe.
   *
   * `document`, when present, is staged with `replace_document_ptr` before the frame and goes
   * beside it (#1697, #1386 D7): the document a `SESSION_REPLACE` frame names by length and digest.
   * The core frees the staging after the command, whatever it was, so a document serves one frame.
   */
  controlFrame(frame: Uint8Array, document?: Uint8Array): Uint8Array {
    const handle = this.#live();
    if (document !== undefined) this.#stageReplaceDocument(handle, document);
    const request = optionalExport(this.#exports, "miso_engine_web_v1_control_request_ptr");
    const command = optionalExport(this.#exports, "miso_engine_web_v1_control_command");
    const responsePointer = optionalExport(this.#exports, "miso_engine_web_v1_control_response_ptr");
    const responseBytes = optionalExport(this.#exports, "miso_engine_web_v1_control_response_bytes");
    if (
      request === undefined || command === undefined
      || responsePointer === undefined || responseBytes === undefined
    ) {
      throw new MisoEngineError("the engine module has no control entry", {
        phase: "asset",
        code: "unsupported",
        result: constantValue("resultCodes", "unsupported"),
        diagnostics: [{ code: "sdk.asset.export", path: "miso_engine_web_v1_control_command" }],
      });
    }
    const memory = this.#exports.memory;
    const staging = Number(request(handle, frame.length));
    if (staging === 0 || staging + frame.length > memory.buffer.byteLength) {
      throw new MisoEngineError("the engine staged no buffer for this control frame", {
        phase: "lifecycle",
        code: "bufferTooSmall",
        result: constantValue("resultCodes", "bufferTooSmall"),
      });
    }
    new Uint8Array(memory.buffer, staging, frame.length).set(frame);
    const result = Number(command(handle, frame.length));
    const pointer = Number(responsePointer(handle));
    const length = Number(responseBytes(handle));
    const inBounds = pointer !== 0 && pointer + length <= memory.buffer.byteLength;
    // The reply is copied out before anything else touches the staging: the event drain below
    // reuses it.
    const reply = inBounds ? new Uint8Array(memory.buffer, pointer, length).slice() : undefined;
    // A commit queues a reliable event, and the queue holds two: the next commit is refused with
    // backpressure until the host reads them. A control call is therefore followed by the drain, as
    // the browser worklet's message handler does (#1570b D4).
    this.#drainControlEvents(handle);
    if (result !== constantValue("resultCodes", "ok")) {
      // A failed call leaves the refusal's diagnostic text where a success leaves the response.
      throw controlCallFailure(result, reply ?? new Uint8Array(0));
    }
    if (reply === undefined) {
      throw new MisoEngineError("the engine returned a malformed control reply buffer", {
        phase: "lifecycle",
        code: "internal",
        result: constantValue("resultCodes", "internal"),
      });
    }
    return reply;
  }

  #stageReplaceDocument(handle: number, document: Uint8Array): void {
    const pointer = document.length === 0
      ? 0
      : Number(this.#exports.miso_engine_web_v1_replace_document_ptr(handle, document.length));
    const memory = this.#exports.memory;
    if (pointer === 0 || pointer + document.length > memory.buffer.byteLength) {
      // Issue #2194: a document whose parse the budget cannot hold beside the live session is
      // refused with the host's diagnostic as the control response, where a refused control
      // call leaves its own; every other refusal leaves the response empty.
      const diagnostics = this.#controlResponseDiagnostics(handle);
      if (diagnostics[0]?.code === "host.budget.parse_projection") {
        throw new MisoEngineError("the engine refused this replacement document's parse beside the live session", {
          phase: "lifecycle",
          code: "refusedBudget",
          result: constantValue("resultCodes", "refusedBudget"),
          diagnostics,
        });
      }
      throw new MisoEngineError("the engine staged no buffer for this replacement document", {
        phase: "lifecycle",
        code: "bufferTooSmall",
        result: constantValue("resultCodes", "bufferTooSmall"),
        diagnostics: [{ code: "sdk.document.bytes", path: `${document.length}` }],
      });
    }
    new Uint8Array(memory.buffer, pointer, document.length).set(document);
  }

  /** The diagnostics the control response holds now, or none. */
  #controlResponseDiagnostics(handle: number): readonly MisoDiagnostic[] {
    const responsePointer = optionalExport(this.#exports, "miso_engine_web_v1_control_response_ptr");
    const responseBytes = optionalExport(this.#exports, "miso_engine_web_v1_control_response_bytes");
    if (responsePointer === undefined || responseBytes === undefined) return [];
    const pointer = Number(responsePointer(handle));
    const length = Number(responseBytes(handle));
    const memory = this.#exports.memory;
    if (pointer === 0 || length === 0 || pointer + length > memory.buffer.byteLength) return [];
    // `.slice()`: a decoder is not required to read a view of a `SharedArrayBuffer` (#1380).
    return parseDiagnostics(new TextDecoder().decode(new Uint8Array(memory.buffer, pointer, length).slice()));
  }

  /**
   * The plan's applied-revision watermark (#1381a D5, D15-17): `watermark_read` copies it into its
   * fixed record, read here at `watermark_ptr`. The record's counters stay in the engine; the
   * snapshot carries the revision, its first sample and the outcome flags of the last advance.
   * Render is the only writer and runs on this thread, so a read never meets a torn record; a
   * `backpressure` result would still be the core's to report, and is thrown as such.
   */
  readWatermark(): WatermarkSnapshot {
    const handle = this.#live();
    const result = Number(this.#exports.miso_engine_web_v1_watermark_read(handle));
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused to read its watermark", {
        phase: "lifecycle",
        code: resultName(result, "call"),
        result,
      });
    }
    const record = new StructView(
      this.#exports.memory,
      "planWatermark",
      Number(this.#exports.miso_engine_web_v1_watermark_ptr(handle)),
    );
    return watermarkSnapshot(record.u64("revision"), record.u64("firstSample"), record.u64("outcomeFlags"));
  }

  /**
   * The output latency and tail of the plan that renders now (#2116): `plan_latency_read` copies
   * the control plane's row into its fixed record, read here at `plan_latency_ptr`. A rebuild's
   * successor shows from the first read after the `render()` that adopts it.
   */
  readPlanLatency(): PlanLatency {
    const handle = this.#live();
    const result = Number(this.#exports.miso_engine_web_v1_plan_latency_read(handle));
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused to read its plan latency", {
        phase: "lifecycle",
        code: resultName(result, "call"),
        result,
      });
    }
    const record = new StructView(
      this.#exports.memory,
      "planLatency",
      Number(this.#exports.miso_engine_web_v1_plan_latency_ptr(handle)),
    );
    return planLatencyOf(record.u64("latencySamples"), record.u64("tailKind"), record.u64("tailSamples"));
  }

  /**
   * Subscribe to the protocol events of the engine's control plane (#1570b): every listener gets
   * `{ lane, frame }` for each event the engine queued, in arrival order, where `lane` is `0` for the
   * reliable lane and `1` for the lossy lane and `frame` is the event frame the engine encoded,
   * unchanged. Events are read after each control frame and after each `render()`; one that arrives
   * while no listener is subscribed is dropped and counted in `discardedControlEvents` as it is read.
   * With no listener, each block's lossy telemetry is therefore dropped after that block, and a
   * listener subscribed later does not receive the last block's lossy event. An event is not the ack of a
   * command (its reply is), so a drop changes no engine state. Returns the unsubscribe function.
   */
  onControlEvent(listener: (event: ControlEvent) => void): () => void {
    if (typeof listener !== "function") {
      throw new MisoUsageError("a control event listener must be a function");
    }
    this.#controlListeners.add(listener);
    return () => {
      this.#controlListeners.delete(listener);
    };
  }

  /** The number of times a control event listener threw. The throw never reaches the caller of a control frame. */
  get controlListenerFailures(): number {
    return this.#controlListenerFailures;
  }

  /** The number of control events dropped because no listener was subscribed. */
  get discardedControlEvents(): number {
    return this.#discardedControlEvents;
  }

  #drainControlEvents(handle: number): void {
    const event = optionalExport(this.#exports, "miso_engine_web_v1_control_event");
    const responsePointer = optionalExport(this.#exports, "miso_engine_web_v1_control_response_ptr");
    const responseBytes = optionalExport(this.#exports, "miso_engine_web_v1_control_response_bytes");
    if (event === undefined || responsePointer === undefined || responseBytes === undefined) return;
    const memory = this.#exports.memory;
    for (const lane of [0, 1]) {
      // The queues hold two reliable events, two render diagnostics and one telemetry event, so
      // the bound is never the limit.
      for (let calls = 0; calls < MAXIMUM_CONTROL_EVENTS_PER_LANE; calls += 1) {
        const result = Number(event(handle, lane));
        if (result !== constantValue("resultCodes", "ok")) {
          throw new MisoEngineError("the engine refused to read a control event", {
            phase: "lifecycle",
            code: resultName(result, "call"),
            result,
          });
        }
        const length = Number(responseBytes(handle));
        if (length === 0) break;
        const pointer = Number(responsePointer(handle));
        if (pointer === 0 || pointer + length > memory.buffer.byteLength) {
          throw new MisoEngineError("the engine returned a malformed control event buffer", {
            phase: "lifecycle",
            code: "internal",
            result: constantValue("resultCodes", "internal"),
          });
        }
        if (this.#controlListeners.size === 0) {
          this.#discardedControlEvents += 1;
          continue;
        }
        const frame = new Uint8Array(memory.buffer, pointer, length).slice();
        const delivered = Object.freeze({ lane: lane as 0 | 1, frame });
        // A listener that throws loses nothing: the reply the core committed is still returned. The
        // failure is counted and reported outside this call.
        for (const listener of [...this.#controlListeners]) {
          try {
            listener(delivered);
          } catch (error: unknown) {
            this.#controlListenerFailures += 1;
            reportAsynchronously(error);
          }
        }
      }
    }
  }

  /**
   * The committed session revision of the core's control plane, as the next `SET_CONTROL`
   * transaction must expect it (#1562a). A transaction queue seeds from it after every successful
   * boot, so it never assumes a session starts at `0`.
   */
  controlRevision(): bigint {
    const handle = this.#live();
    const revision = optionalExport(this.#exports, "miso_engine_web_v1_control_revision");
    if (revision === undefined) {
      throw new MisoEngineError("the engine module has no control entry", {
        phase: "asset",
        code: "unsupported",
        result: constantValue("resultCodes", "unsupported"),
        diagnostics: [{ code: "sdk.asset.export", path: "miso_engine_web_v1_control_revision" }],
      });
    }
    return BigInt(revision(handle));
  }

  /**
   * Release the session.
   *
   * The module is re-bootable afterwards: dispose resets the staging block to its zero defaults,
   * which is what makes `dispose()` -> restage -> `boot` the supported headless mix switch
   * (issue #243 S2(a)).
   */
  dispose(): void {
    if (this.#handle === 0) return;
    // A pending query rejects now, with the error a call on a disposed engine gets.
    this.#trackResponse?.close(new MisoUsageError(DISPOSED));
    this.#trackResponse = undefined;
    this.#spectrumQuery = undefined;
    this.#spectrumCollection = undefined;
    this.#spectrumActiveQuery = undefined;
    const result = Number(this.#exports.miso_engine_web_v1_dispose(this.#handle));
    this.#handle = 0;
    this.#stripOrder = undefined;
    this.#stripOrders.clear();
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused to dispose this handle", {
        phase: "lifecycle",
        code: resultName(result, "call"),
        result,
      });
    }
  }
}

/** One protocol event of the engine's control plane, as the engine encoded it. */
export interface ControlEvent {
  /** `0` the reliable lane, `1` the lossy lane. */
  readonly lane: 0 | 1;
  readonly frame: Uint8Array;
}

/** The strip orders a boundary remembers: a structural commit and its adoption are the two in flight. */
const HELD_STRIP_ORDERS = 8;

/** The most `control_event` calls one drain makes on a lane (#1570b D4). */
const MAXIMUM_CONTROL_EVENTS_PER_LANE = 16;

/**
 * Report an error outside the current call, as an event target does for a listener that throws:
 * `reportError` where the global has one (browsers and workers), otherwise an uncaught throw from a
 * microtask, which Node surfaces as `uncaughtException`.
 */
function reportAsynchronously(error: unknown): void {
  const report = (globalThis as { reportError?: (error: unknown) => void }).reportError;
  if (typeof report === "function") {
    queueMicrotask(() => report.call(globalThis, error));
  } else {
    queueMicrotask(() => {
      throw error;
    });
  }
}

/**
 * The four-call staging sequence, in the order the generated document names it.
 *
 * Extracted so a first boot and a mix switch are literally the same code: a reboot that drifted
 * from a boot is exactly the class of bug that makes "it works the first time" a support ticket.
 */
/** The boot command of `snapshot`'s spectrum taps (#2166), or `undefined`. */
function bootCommandOf(snapshot: BootSnapshot): Uint8Array | undefined {
  return bootSpectrumCommand(snapshot.spectrumCollection ?? snapshot.spectrumQuery, snapshot.spectrumHopFrames);
}

function stage(
  exports: ExportTable,
  document: Uint8Array,
  options: BootOptions,
  bootCommand: Uint8Array | undefined,
): { handle: number; optionBytes: Uint8Array } {
  // 1. `abi_version` -- already pinned by `asset.instantiate()`, re-read here so this function is
  //    the published sequence rather than an abbreviation of it.
  const version = Number(exports.miso_engine_web_v1_abi_version());
  if (version !== ABI_LAYOUT.abiVersion) {
    throw new MisoEngineError(`the instance reports ABI 0x${version.toString(16)}`, {
      phase: "asset",
      code: "abiMismatch",
      result: constantValue("resultCodes", "abiMismatch"),
      diagnostics: [{ code: "sdk.asset.abi_version", path: String(version) }],
    });
  }

  // 2. `boot_options_ptr` -- module-owned, zero-default, stable for the module's lifetime.
  const optionsPointer = Number(exports.miso_engine_web_v1_boot_options_ptr());
  const optionBytes = writeBootOptions(exports.memory, optionsPointer, options);

  // 3. `document_ptr(len)` -- the admission gate. A document over the options' limit (the
  //    budget's parse projection, or the `maximumDocumentBytes` word, issue #2191) is refused
  //    *here*, before a byte is copied and before any staging is allocated, which is what makes
  //    oversize admission cost no memory (issue #243 eval 3).
  const documentPointer = Number(exports.miso_engine_web_v1_document_ptr(document.byteLength));
  if (documentPointer === 0) {
    throw bootFailure(exports, "the engine refused to stage the document");
  }
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);

  // #2166: the boot's spectrum taps, staged beside the document, are the first plan's. Optional,
  // so not a step of the published sequence.
  if (bootCommand !== undefined) {
    const commandPointer = Number(exports.miso_engine_web_v1_boot_command_ptr(bootCommand.byteLength));
    if (commandPointer === 0) {
      throw new MisoEngineError("the engine refused to stage the boot command", {
        phase: "boot",
        code: "invalidArgument",
        result: constantValue("resultCodes", "invalidArgument"),
        diagnostics: [{ code: "sdk.boot.command", path: String(bootCommand.byteLength) }],
      });
    }
    new Uint8Array(exports.memory.buffer, commandPointer, bootCommand.byteLength).set(bootCommand);
  }

  // 4. `boot(len)`.
  const handle = Number(exports.miso_engine_web_v1_boot(document.byteLength));
  if (handle === 0) {
    throw bootFailure(exports, "the engine refused to boot the document");
  }
  return { handle, optionBytes };
}

/**
 * Build the typed refusal for a failed staging or boot.
 *
 * The diagnostic is read where `boot_diagnostic_ptr` says (issue #2191): over the staged document
 * after a refused boot, and in a buffer of its own after a refused `document_ptr`, which names the
 * document's bytes and the limit it is over.
 */
function bootFailure(
  exports: ExportTable,
  message: string,
): MisoEngineError {
  const result = Number(exports.miso_engine_web_v1_boot_result());
  const length = Number(exports.miso_engine_web_v1_boot_diagnostic_bytes());
  const pointer = Number(exports.miso_engine_web_v1_boot_diagnostic_ptr());
  const text = pointer !== 0 && length > 0
    ? new TextDecoder().decode(new Uint8Array(exports.memory.buffer, pointer, length).slice())
    : "";
  const code = resultName(result, "boot");
  return new MisoEngineError(message, {
    phase: code === "refusedLifecycle" ? "lifecycle" : "boot",
    code,
    result,
    diagnostics: parseDiagnostics(text),
  });
}

/** Byte size of the boot options block, from the generated document. */
export const BOOT_OPTIONS_BYTES: number = structBytes("bootOptions");
