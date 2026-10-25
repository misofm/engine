import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BufferKindName, ExportName, ResultCodeName } from "../generated/abi.ts";
import type { CommandReasonName } from "../generated/catalog.ts";
import {
  StructView,
  constantValue,
  defaultSourceRingFrames,
  structBytes,
  writeBootOptions,
} from "./abi.ts";
import type { BootOptions } from "./abi.ts";
import { MisoEngineAsset } from "./asset.ts";
import { controlCallFailure } from "./control-frame.ts";
import { readControlStripOrderSync } from "./control-list.ts";
import type { ControlStripOrder } from "./control-list.ts";
import { ControlRequestIds } from "./control-queue.ts";
import { MisoEngineError, MisoUsageError, parseDiagnostics, resultName } from "./errors.ts";
import { TrackResponseRequester } from "./live-response.ts";
import { watermarkSnapshot } from "./session-edit.ts";
import type { WatermarkSnapshot } from "./session-edit.ts";
import type { TrackResponseQuery, TrackResponseResult } from "./live-response.ts";
import {
  SpectrumModule,
  cloneSpectrumCollection,
  cloneSpectrumQuery,
  spectrumChannelsRaw,
  spectrumTargetId,
  spectrumTargetRaw,
  stageSpectrumCollectionRequest,
  stageSpectrumRequest,
} from "./spectrum.ts";
import type {
  SpectrumCollection,
  SpectrumQuery,
  SpectrumResult,
  SpectrumStreamSelection,
  SpectrumStreamRead,
  SpectrumStreamStart,
} from "./spectrum.ts";
import {
  decodeObservationRows,
  enrichObservationMap,
  resolveObservationAddressesWithTracks,
  validateObservationSelections,
} from "./observation.ts";
import type {
  ObservationMap,
  ObservationReadResult,
  ObservationSelection,
  RawObservationBinding,
  RawObservationRow,
} from "./observation.ts";

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

const SPECTRUM_HOP_CAPABILITY = "miso_engine_web_v1_spectrum_hop_capability";
const SPECTRUM_HOP_BOOT = "miso_engine_web_v1_boot_with_spectrum_hop";
const OPTIONAL_ADDITIVE_EXPORTS: ReadonlySet<string> = new Set([
  SPECTRUM_HOP_CAPABILITY,
  SPECTRUM_HOP_BOOT,
]);
type SpectrumHopFrames = 256 | 512 | 1024 | 2048;
type ExportFunction = (...args: (number | bigint)[]) => number | bigint;

function exportsOf(instance: WebAssembly.Instance): ExportTable {
  const table = instance.exports as Record<string, unknown>;
  for (const name of ABI_LAYOUT.exports) {
    // These are additive spectrum-hop exports. An older asset remains valid for the legacy boot
    // path and is checked only when a caller explicitly requests the prepared spectrum hop.
    if (OPTIONAL_ADDITIVE_EXPORTS.has(name)) continue;
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

function spectrumHopRefusal(name: string, reason: string): MisoEngineError {
  return new MisoEngineError(
    `the engine asset cannot honor an explicit spectrum hop: ${reason}`,
    {
      phase: "asset",
      code: "abiMismatch",
      result: constantValue("resultCodes", "abiMismatch"),
      diagnostics: [{ code: "sdk.asset.capability", path: name }],
    },
  );
}

function requireSpectrumHopCapability(
  exports: ExportTable,
  spectrumHopFrames: SpectrumHopFrames | undefined,
): ExportFunction | undefined {
  if (spectrumHopFrames === undefined) return undefined;

  const capability = optionalExport(exports, SPECTRUM_HOP_CAPABILITY);
  if (capability === undefined) {
    throw spectrumHopRefusal(SPECTRUM_HOP_CAPABILITY, "the capability export is missing");
  }
  let value: number | bigint;
  try {
    value = capability();
  } catch (error) {
    throw spectrumHopRefusal(
      SPECTRUM_HOP_CAPABILITY,
      `the capability export could not be queried: ${(error as Error).message}`,
    );
  }
  if (value !== 1) {
    throw spectrumHopRefusal(
      SPECTRUM_HOP_CAPABILITY,
      `the capability export returned ${String(value)} instead of 1`,
    );
  }
  const spectrumHopBoot = optionalExport(exports, SPECTRUM_HOP_BOOT);
  if (spectrumHopBoot === undefined) {
    throw spectrumHopRefusal(SPECTRUM_HOP_BOOT, "the spectrum-hop boot export is missing");
  }
  return spectrumHopBoot;
}

function validateSpectrumHopFrames(value: unknown): SpectrumHopFrames | undefined {
  if (value === undefined) return undefined;
  if (value === 256 || value === 512 || value === 1024 || value === 2048) {
    return value;
  }
  throw new MisoUsageError("spectrumHopFrames must be one of 256, 512, 1024, or 2048");
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
    const { meterBlocks, observationTaps } = options.liveControls!;
    return Object.freeze({
      ...(meterBlocks === undefined ? {} : { meterBlocks }),
      ...(observationTaps === undefined ? {} : { observationTaps }),
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
  /** The ring the engine derived, from the published rule applied to the reported shape. */
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

interface ObservationLayoutField {
  readonly name: string;
  readonly offset: number;
  readonly type: string;
}

function observationField(structure: "observationSelection" | "observationResult", name: string): ObservationLayoutField {
  const field = ABI_LAYOUT.structures[structure].fields.find((candidate) => candidate.name === name);
  if (field === undefined) {
    throw new MisoUsageError(`the generated ABI layout has no ${structure}.${name}`);
  }
  return field;
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
  #spectrum: SpectrumModule | undefined;
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
    const exports = exportsOf(await asset.instantiate());
    const staged = stage(exports, document, snapshot.options, snapshot.spectrumHopFrames);
    return new WasmBoundary(
      exports,
      staged.handle,
      staged.optionBytes,
      (snapshot.options.liveControls?.meterBlocks ?? 0) > 0,
      snapshot.spectrumQuery,
      snapshot.spectrumCollection,
    );
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
    // Capability refusal must happen while the current handle is still live. The validated
    // additive export is passed into `stage`, so this preflight also avoids a second probe after
    // the current session has been disposed.
    const spectrumHopBoot = requireSpectrumHopCapability(this.#exports, snapshot.spectrumHopFrames);
    this.dispose();
    this.#stripOrder = undefined;
    this.#stripOrders.clear();
    const staged = stage(
      this.#exports,
      document,
      snapshot.options,
      snapshot.spectrumHopFrames,
      spectrumHopBoot,
    );
    this.#handle = staged.handle;
    this.#optionBytes = staged.optionBytes;
    this.#metersAttached = (snapshot.options.liveControls?.meterBlocks ?? 0) > 0;
    this.#spectrumQuery = snapshot.spectrumQuery;
    this.#spectrumCollection = snapshot.spectrumCollection;
    this.#spectrumActiveQuery = snapshot.spectrumQuery;
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
      // The ring is an input word with no readback export, so it is *derived* from the rule the
      // generated document publishes, applied to the shape the boot just reported.
      sourceRingFrames: defaultSourceRingFrames(sampleRateHz, quantumFrames),
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
    return new TextDecoder().decode(new Uint8Array(this.#exports.memory.buffer, pointer, length));
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

  /**
   * The strip order of the plan that renders (#1562b): the list of the structure epoch in that
   * plan's meter header, which is the newest list's except between a rebuild's commit and the block
   * that adopts its replacement. The observation bindings and reads address the rendering plan, so
   * they name their strips by this order and never by the newest list.
   */
  #renderingStripOrder(): ControlStripOrder {
    const newest = this.stripOrder();
    const header = new StructView(
      this.#exports.memory,
      "meterHeader",
      Number(this.#exports.miso_engine_web_v1_meter_header_ptr(this.#live())),
    );
    const epoch = header.u64("structureEpoch");
    if (epoch === 0n || epoch === newest.structureEpoch) return newest;
    const held = this.#stripOrders.get(epoch);
    if (held === undefined) {
      // The plan that renders was prepared from a list this instance never read, and a newer one
      // replaced it: the strips of its bindings cannot be named. A render adopts the newer plan.
      throw new MisoEngineError("the rendering plan belongs to a control list that is gone", {
        phase: "lifecycle",
        code: "control.list.stale",
        diagnostics: [{ code: "control.list.stale", path: "structureEpoch" }],
      });
    }
    return held;
  }

  /**
   * Read the current prepared owner's stable resident-observation bindings: the plan that renders,
   * named by the strip order of its own structure epoch.
   */
  observationMap(): ObservationMap {
    return this.#observationMapOf(this.#renderingStripOrder());
  }

  #observationMapOf(order: ControlStripOrder): ObservationMap {
    const handle = this.#live();
    const count = Number(this.#exports.miso_engine_web_v1_observation_count(handle));
    if (!Number.isSafeInteger(count) || count < 0 || count > 0xffff_ffff) {
      throw new MisoEngineError("the engine returned an invalid observation binding count", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.observation.count", path: "$" }],
      });
    }
    const idPointer = Number(this.#exports.miso_engine_web_v1_observation_id_ptr());
    const idCapacity = Number(this.#exports.miso_engine_web_v1_observation_id_capacity());
    if (idPointer === 0 || !Number.isSafeInteger(idCapacity) || idCapacity <= 0) {
      throw new MisoEngineError("the engine returned invalid observation ID staging", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.observation.id_staging", path: "$" }],
      });
    }
    // Issue #1210 D5: a binding's index is a strip index, the tracks then the submixes.
    const strips = [...order.tracks, ...order.submixes];
    const raw: RawObservationBinding[] = [];
    const readId = (length: number, path: string): string => {
      if (!Number.isSafeInteger(length) || length <= 0 || length > idCapacity) {
        throw new MisoEngineError("the engine returned an invalid observation ID length", {
          phase: "output",
          code: "abiMismatch",
          result: constantValue("resultCodes", "abiMismatch"),
          diagnostics: [{ code: "sdk.observation.id", path }],
        });
      }
      return new TextDecoder().decode(new Uint8Array(this.#exports.memory.buffer, idPointer, length));
    };
    for (let index = 0; index < count; index += 1) {
      const effectSlotId = readId(
        Number(this.#exports.miso_engine_web_v1_observation_effect_slot_id(handle, index)),
        `${index}.effectSlotId`,
      );
      const nativeEffectId = readId(
        Number(this.#exports.miso_engine_web_v1_observation_native_effect_id(handle, index)),
        `${index}.nativeEffectId`,
      );
      const tapCount = Number(this.#exports.miso_engine_web_v1_observation_tap_count(handle, index));
      if (!Number.isSafeInteger(tapCount) || tapCount <= 0 || tapCount > 32) {
        throw new MisoEngineError("the engine returned an invalid observation tap count", {
          phase: "output",
          code: "abiMismatch",
          result: constantValue("resultCodes", "abiMismatch"),
          diagnostics: [{ code: "sdk.observation.taps", path: String(index) }],
        });
      }
      const tapIds: number[] = [];
      for (let tapIndex = 0; tapIndex < tapCount; tapIndex += 1) {
        const tapId = Number(this.#exports.miso_engine_web_v1_observation_tap_id(
          handle,
          index,
          tapIndex,
        ));
        if (!Number.isSafeInteger(tapId) || tapId <= 0) {
          throw new MisoEngineError("the engine returned an invalid observation tap ID", {
            phase: "output",
            code: "abiMismatch",
            result: constantValue("resultCodes", "abiMismatch"),
            diagnostics: [{ code: "sdk.observation.tap", path: `${index}/${tapIndex}` }],
          });
        }
        tapIds.push(tapId);
      }
      raw.push({
        trackIndex: Number(this.#exports.miso_engine_web_v1_observation_track_index(handle, index)),
        rack: Number(this.#exports.miso_engine_web_v1_observation_rack(handle, index)),
        effectIndex: Number(this.#exports.miso_engine_web_v1_observation_effect_index(handle, index)),
        effectSlotId,
        nativeEffectId,
        tapIds,
      });
    }
    return enrichObservationMap(strips, raw);
  }

  /** Internal managed-subscription view of the optional prepared spectrum query. */
  preparedSpectrumQuery(): SpectrumQuery | undefined {
    return this.#spectrumQuery;
  }

  /** Internal managed-subscription view of the optional prepared spectrum collection. */
  preparedSpectrumCollection(): SpectrumCollection | undefined {
    return this.#spectrumCollection;
  }

  /** Read one bounded non-consuming batch from the current prepared owner. */
  readObservations(selections: readonly ObservationSelection[]): readonly ObservationReadResult[] {
    validateObservationSelections(selections);
    const handle = this.#live();
    // Issue #1210 D5: selections resolve against the strips, the tracks then the submixes. The
    // order is read once and shared with the map: each read probes the structure epoch.
    const order = this.#renderingStripOrder();
    const map = this.#observationMapOf(order);
    const strips = [...order.tracks, ...order.submixes];
    const addresses = resolveObservationAddressesWithTracks(map, strips, selections);
    const selectionPointer = Number(this.#exports.miso_engine_web_v1_observation_selection_ptr());
    const selectionCapacity = Number(this.#exports.miso_engine_web_v1_observation_selection_capacity());
    const selectionBytes = Number(this.#exports.miso_engine_web_v1_observation_selection_bytes());
    if (selectionPointer === 0 || selectionBytes !== structBytes("observationSelection")
        || selectionCapacity < selections.length) {
      throw new MisoEngineError("the engine returned insufficient observation selection staging", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.observation.selection_staging", path: "$" }],
      });
    }
    const memory = new DataView(
      this.#exports.memory.buffer,
      selectionPointer,
      selectionCapacity * selectionBytes,
    );
    const writeU32 = (field: string, value: number, index: number): void => {
      memory.setUint32(
        index * selectionBytes + observationField("observationSelection", field).offset,
        value,
        true,
      );
    };
    addresses.forEach((address, index) => {
      writeU32("structSize", selectionBytes, index);
      writeU32("abiVersion", ABI_LAYOUT.abiVersion, index);
      writeU32("trackIndex", address.trackIndex, index);
      writeU32("rack", address.rack, index);
      writeU32("effectIndex", address.effectIndex, index);
      writeU32("tapId", address.tapId, index);
      writeU32("channels", address.channels, index);
      writeU32("reserved", 0, index);
    });
    const result = Number(this.#exports.miso_engine_web_v1_observation_read(
      handle,
      selections.length,
    ));
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused the selected observation read", {
        phase: result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
        code: resultName(result, "call"),
        result,
      });
    }
    const resultPointer = Number(this.#exports.miso_engine_web_v1_observation_result_ptr());
    const resultBytes = Number(this.#exports.miso_engine_web_v1_observation_result_bytes());
    const rowBytes = structBytes("observationResult");
    if (resultPointer === 0 || resultBytes !== selections.length * rowBytes) {
      throw new MisoEngineError("the engine returned invalid observation result staging", {
        phase: "output",
        code: "abiMismatch",
        result: constantValue("resultCodes", "abiMismatch"),
        diagnostics: [{ code: "sdk.observation.result_staging", path: "$" }],
      });
    }
    const output = new DataView(this.#exports.memory.buffer, resultPointer, resultBytes);
    const readU32 = (field: string, index: number): number => output.getUint32(
      index * rowBytes + observationField("observationResult", field).offset,
      true,
    );
    const readU64 = (field: string, index: number): bigint => output.getBigUint64(
      index * rowBytes + observationField("observationResult", field).offset,
      true,
    );
    const readF32 = (field: string, index: number): number => output.getFloat32(
      index * rowBytes + observationField("observationResult", field).offset,
      true,
    );
    const rows: RawObservationRow[] = selections.map((_selection, index) => ({
      trackIndex: readU32("trackIndex", index),
      rack: readU32("rack", index),
      effectIndex: readU32("effectIndex", index),
      tapId: readU32("tapId", index),
      channels: readU32("channels", index),
      status: readU32("status", index),
      sampleRateHz: readU32("sampleRateHz", index),
      firstSample: readU64("firstSample", index),
      endSample: readU64("endSample", index),
      sequence: readU64("sequence", index),
      blocks: readU32("blocks", index),
      leftPresent: readU32("leftPresent", index),
      rightPresent: readU32("rightPresent", index),
      left: readF32("left", index),
      right: readF32("right", index),
    }));
    return decodeObservationRows(
      map,
      strips,
      selections,
      addresses,
      rows,
      this.#status().u32("sampleRateHz"),
    );
  }

  /**
   * The response of one track, as a C ABI host reads it (#1579a D1, D2): a strip `RESPONSE_GET`
   * through the control entry, then the `RESPONSE_READY` with its tag from the event lane. The core
   * captures in its render step, so the promise settles after the next `render()`, at the latest
   * when the request deadline passes. Queries run one at a time, in call order.
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

  /** Arm the one prepared spectrum boundary for its next complete window. */
  armSpectrum(): EngineCallResult {
    const query = this.#spectrumActiveQuery;
    if (query === undefined) {
      return Object.freeze({ ok: false, result: constantValue("resultCodes", "unsupported"), code: "unsupported" });
    }
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_arm(this.#live()));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Atomically select one exact target and channel mask from the prepared collection. */
  selectSpectrum(query: SpectrumQuery): EngineCallResult {
    const collection = this.#spectrumCollection;
    if (collection === undefined) {
      return Object.freeze({ ok: false, result: constantValue("resultCodes", "unsupported"), code: "unsupported" });
    }
    const selected = cloneSpectrumQuery(query);
    const channels = spectrumChannelsRaw(selected.channels);
    const targetKey = JSON.stringify(selected.target);
    const requestedChannels = selected.channels ?? "both";
    const prepared = collection.entries.some((entry) =>
      JSON.stringify(entry.target) === targetKey
      && (entry.channels ?? "both") === requestedChannels);
    if (!prepared) {
      return Object.freeze({ ok: false, result: constantValue("resultCodes", "invalidArgument"), code: "invalidArgument" });
    }
    const id = new TextEncoder().encode(spectrumTargetId(selected.target));
    const pointer = Number(this.#exports.miso_engine_web_v1_spectrum_target_id_ptr());
    const capacity = Number(this.#exports.miso_engine_web_v1_spectrum_target_id_capacity());
    if (pointer <= 0 || id.byteLength > capacity) {
      throw new MisoEngineError("the engine returned insufficient spectrum selection staging", {
        phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
      });
    }
    new Uint8Array(this.#exports.memory.buffer, pointer, id.byteLength).set(id);
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_select(
      this.#live(), spectrumTargetRaw(selected.target), channels, id.byteLength,
    ));
    if (result === constantValue("resultCodes", "ok")) this.#spectrumActiveQuery = selected;
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Atomically update the active collection entry and smoothing profile. */
  selectSpectrumStream(query: SpectrumQuery, smoothingMs: number): SpectrumStreamSelection {
    const collection = this.#spectrumCollection;
    if (collection === undefined) {
      return Object.freeze({
        ok: false,
        result: constantValue("resultCodes", "unsupported"),
        code: "unsupported",
      });
    }
    const selected = cloneSpectrumQuery(query);
    const channels = spectrumChannelsRaw(selected.channels);
    const targetKey = JSON.stringify(selected.target);
    const requestedChannels = selected.channels ?? "both";
    const prepared = collection.entries.some((entry) =>
      JSON.stringify(entry.target) === targetKey
      && (entry.channels ?? "both") === requestedChannels);
    if (!prepared) {
      return Object.freeze({
        ok: false,
        result: constantValue("resultCodes", "invalidArgument"),
        code: "invalidArgument",
      });
    }
    const id = new TextEncoder().encode(spectrumTargetId(selected.target));
    const pointer = Number(this.#exports.miso_engine_web_v1_spectrum_target_id_ptr());
    const capacity = Number(this.#exports.miso_engine_web_v1_spectrum_target_id_capacity());
    if (pointer <= 0 || id.byteLength > capacity) {
      throw new MisoEngineError("the engine returned insufficient spectrum selection staging", {
        phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
      });
    }
    new Uint8Array(this.#exports.memory.buffer, pointer, id.byteLength).set(id);
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_stream_select(
      this.#live(), spectrumTargetRaw(selected.target), channels, id.byteLength, smoothingMs,
    ));
    if (result !== constantValue("resultCodes", "ok")) {
      return Object.freeze({ ok: false, result, code: resultName(result, "call") });
    }
    const metadata = this.#spectrumModule().streamMetadata(selected);
    this.#spectrumActiveQuery = selected;
    return Object.freeze({
      ok: true,
      result,
      code: resultName(result, "call"),
      metadata,
    });
  }

  /** Read and analyze a completed spectrum window; `undefined` means the fixed window is pending. */
  readSpectrum(): SpectrumResult | undefined {
    const query = this.#spectrumActiveQuery;
    if (query === undefined) throw new MisoUsageError("this engine has no prepared spectrum boundary");
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_read(
      this.#live(),
      spectrumChannelsRaw(query.channels),
    ));
    if (result === constantValue("resultCodes", "backpressure")) return undefined;
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused the spectrum capture read", {
        phase: result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
        code: resultName(result, "call"),
        result,
      });
    }
    this.#spectrum ??= new SpectrumModule(this.#exports);
    return this.#spectrum.analyzeCurrent(query);
  }

  /** Start the managed continuous spectrum stream for the prepared boundary. */
  startSpectrumStream(smoothingMs = 100, _query?: SpectrumQuery): SpectrumStreamStart {
    const query = _query ?? this.#spectrumActiveQuery;
    if (query === undefined) {
      throw new MisoUsageError("this engine has no prepared spectrum boundary");
    }
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_stream_start(
      this.#live(), smoothingMs,
    ));
    const metadata = this.#spectrumModule().streamMetadata(query);
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
      metadata,
    });
  }

  /** Read and analyze one managed stream window, or return its explicit availability state. */
  readSpectrumStream(queryOverride?: SpectrumQuery): SpectrumStreamRead {
    const query = queryOverride ?? this.#spectrumActiveQuery;
    if (query === undefined) {
      throw new MisoUsageError("this engine has no prepared spectrum boundary");
    }
    const prepared = this.#spectrumQuery;
    const collection = this.#spectrumCollection;
    if (prepared !== undefined) {
      const preparedTarget = JSON.stringify(prepared.target);
      if (preparedTarget !== JSON.stringify(query.target)
          || (prepared.channels ?? "both") !== (query.channels ?? "both")) {
        throw new MisoUsageError("the spectrum stream query differs from the prepared boundary");
      }
      const preparedMaximum = prepared.spectrumLimits?.maximumCaptureBytes
        ?? ABI_LAYOUT.constants.spectrumCaptureBytes;
      const requestedMaximum = query.spectrumLimits?.maximumCaptureBytes ?? preparedMaximum;
      if (!Number.isSafeInteger(requestedMaximum) || requestedMaximum < 1 || requestedMaximum > preparedMaximum) {
        throw new MisoUsageError("the spectrum stream capture limit exceeds the prepared bound");
      }
    } else if (collection !== undefined) {
      const requestedChannels = query.channels ?? "both";
      const preparedEntry = collection.entries.some((entry) =>
        JSON.stringify(entry.target) === JSON.stringify(query.target)
        && (entry.channels ?? "both") === requestedChannels);
      if (!preparedEntry) throw new MisoUsageError("the spectrum stream query differs from the prepared collection");
    }
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_stream_read(this.#live()));
    const metadata = this.#spectrumModule().streamMetadata(query);
    const backpressure = constantValue("resultCodes", "backpressure");
    const renderRejected = constantValue("resultCodes", "renderRejected");
    const ok = constantValue("resultCodes", "ok");
    if (result === backpressure || result === renderRejected) {
      return Object.freeze({ metadata });
    }
    if (result !== ok) {
      throw new MisoEngineError("the engine refused the spectrum stream read", {
        phase: result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
        code: resultName(result, "call"),
        result,
      });
    }
    if (metadata.status !== "ready") return Object.freeze({ metadata });
    return this.#spectrumModule().analyzeStreamCurrent(query);
  }

  /** Stop the managed continuous spectrum stream. */
  stopSpectrumStream(): EngineCallResult {
    if (this.#spectrumQuery === undefined && this.#spectrumCollection === undefined) {
      return Object.freeze({ ok: false, result: constantValue("resultCodes", "unsupported"), code: "unsupported" });
    }
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_stream_stop(this.#live()));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  /** Cancel a pending spectrum capture. */
  cancelSpectrum(): EngineCallResult {
    if (this.#spectrumQuery === undefined && this.#spectrumCollection === undefined) {
      return Object.freeze({ ok: false, result: constantValue("resultCodes", "unsupported"), code: "unsupported" });
    }
    const result = Number(this.#exports.miso_engine_web_v1_spectrum_cancel(this.#live()));
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
    });
  }

  #spectrumModule(): SpectrumModule {
    this.#spectrum ??= new SpectrumModule(this.#exports);
    return this.#spectrum;
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
      throw new MisoEngineError("the engine staged no buffer for this replacement document", {
        phase: "lifecycle",
        code: "bufferTooSmall",
        result: constantValue("resultCodes", "bufferTooSmall"),
        diagnostics: [{ code: "sdk.document.bytes", path: `${document.length}` }],
      });
    }
    new Uint8Array(memory.buffer, pointer, document.length).set(document);
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
   * Stage `count` already-encoded command records (`ABI_LAYOUT.commandRecord.bytes` each) and submit them as one transaction.
   *
   * Only the observation records (kinds 7 and 8) are command records any more; a value edit is a
   * `SET_CONTROL` transaction (`controlFrame`).
   */
  submitCommands(records: Uint8Array, count: number): CommandReport {
    const handle = this.#live();
    const recordBytes = ABI_LAYOUT.commandRecord.bytes;
    const staging = this.#buffer("command");
    if (staging.pointer === 0 || staging.capacity === 0) {
      throw new MisoUsageError(
        "this engine booted with no observation taps attached; set liveControls.observationTaps",
      );
    }
    if (records.byteLength !== count * recordBytes) {
      throw new MisoUsageError(
        `${count} records is ${count * recordBytes} bytes, got ${records.byteLength}`,
      );
    }
    if (records.byteLength > staging.capacity) {
      throw new MisoUsageError(
        `${count} records exceed the ${staging.capacity / recordBytes}-record staging buffer`,
      );
    }
    new Uint8Array(this.#exports.memory.buffer, staging.pointer, records.byteLength).set(records);
    const result = Number(this.#exports.miso_engine_web_v1_command_submit(handle, count));
    return this.#readCommandReport(handle, result);
  }

  #readCommandReport(handle: number, result: number): CommandReport {
    const report = new StructView(
      this.#exports.memory,
      "commandReport",
      Number(this.#exports.miso_engine_web_v1_command_report_ptr(handle)),
    );
    const reason = report.u32("reason");
    return Object.freeze({
      ok: result === constantValue("resultCodes", "ok"),
      result,
      code: resultName(result, "call"),
      reason,
      reasonName: commandReasonName(reason),
      rejectedIndex: report.u32("rejectedIndex"),
      admitted: report.u32("admitted"),
      appliedAtSample: report.u64("appliedAtSample"),
    });
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
    this.#spectrum?.close();
    this.#spectrum = undefined;
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

export interface CommandReport {
  /** Whole-batch admission. A refusal admits nothing and names the first offending record. */
  readonly ok: boolean;
  readonly result: number;
  readonly code: ResultCodeName;
  readonly reason: number;
  readonly reasonName: CommandReasonName;
  readonly rejectedIndex: number;
  readonly admitted: number;
  readonly appliedAtSample: bigint;
}

export function commandReasonName(value: number): CommandReasonName {
  const name = ABI_LAYOUT.constants.commandReasons.find((row) => row.value === value)?.name;
  if (name === undefined) {
    throw new MisoUsageError(`the engine reported an unknown command reason ${value}`);
  }
  return name;
}

/**
 * The four-call staging sequence, in the order the generated document names it.
 *
 * Extracted so a first boot and a mix switch are literally the same code: a reboot that drifted
 * from a boot is exactly the class of bug that makes "it works the first time" a support ticket.
 */
function stage(
  exports: ExportTable,
  document: Uint8Array,
  options: BootOptions,
  spectrumHopFrames: SpectrumHopFrames | undefined,
  preflightedSpectrumHopBoot?: ExportFunction,
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

  // The additive capability is checked after the ABI guard but before any options or document
  // staging. Reboot supplies its pre-disposal result here so a failed explicit probe cannot
  // destroy the current live session.
  const spectrumHopBoot = preflightedSpectrumHopBoot
    ?? requireSpectrumHopCapability(exports, spectrumHopFrames);

  // 2. `boot_options_ptr` -- module-owned, zero-default, stable for the module's lifetime.
  const optionsPointer = Number(exports.miso_engine_web_v1_boot_options_ptr());
  const optionBytes = writeBootOptions(exports.memory, optionsPointer, options);
  if (options.spectrumCollection !== undefined) {
    stageSpectrumCollectionRequest(exports, options.spectrumCollection);
  } else {
    stageSpectrumRequest(exports, options.spectrum);
  }

  // 3. `document_ptr(len)` -- the admission gate. A document over the engine's maximum is refused
  //    *here*, before a byte is copied and before any staging is allocated, which is what makes
  //    oversize admission cost no memory (issue #243 eval 3).
  const documentPointer = Number(exports.miso_engine_web_v1_document_ptr(document.byteLength));
  if (documentPointer === 0) {
    throw bootFailure(exports, 0, "the engine refused to stage the document");
  }
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);

  // 4. `boot(len)`.
  const handle = spectrumHopFrames === undefined
    ? Number(exports.miso_engine_web_v1_boot(document.byteLength))
    : Number((spectrumHopBoot as ExportFunction)(
      document.byteLength,
      spectrumHopFrames,
    ));
  if (handle === 0) {
    throw bootFailure(exports, documentPointer, "the engine refused to boot the document");
  }
  return { handle, optionBytes };
}

/**
 * Build the typed refusal for a failed staging or boot.
 *
 * The diagnostic is written *over the staged document buffer*, so it is read back from the pointer
 * `document_ptr` returned -- and when staging itself refused there is no buffer, which is exactly
 * why `boot_diagnostic_bytes` is zero on that path and the code alone carries the answer.
 */
function bootFailure(
  exports: ExportTable,
  documentPointer: number,
  message: string,
): MisoEngineError {
  const result = Number(exports.miso_engine_web_v1_boot_result());
  const length = Number(exports.miso_engine_web_v1_boot_diagnostic_bytes());
  const text = documentPointer !== 0 && length > 0
    ? new TextDecoder().decode(new Uint8Array(exports.memory.buffer, documentPointer, length))
    : "";
  const code = resultName(result, "boot");
  return new MisoEngineError(message, {
    phase: code === "refusedLifecycle" ? "lifecycle" : "boot",
    code,
    result,
    diagnostics: parseDiagnostics(text),
  });
}

/** The engine's maximum staged document length, from the generated document. */
export const MAXIMUM_DOCUMENT_BYTES: number = ABI_LAYOUT.constants.maximumDocumentBytes;

/** Byte size of the boot options block, from the generated document. */
export const BOOT_OPTIONS_BYTES: number = structBytes("bootOptions");
