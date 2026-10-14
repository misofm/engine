import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import { MisoEngineError, MisoUsageError, resultName } from "./errors.ts";
import type { MisoDiagnostic } from "./errors.ts";
import {
  decodeResponseReply,
  encodeResponseRequest,
  responseRefusalResult,
} from "./response-frame.ts";
import type { ResponseFrameCurves, ResponseFrameOverride, ResponseFrameRequest } from "./response-frame.ts";
import { parameterRows } from "./session.ts";
import type { Channel, EffectDecl } from "./types.ts";

/** An explicit stopped input-filter configuration. */
export interface InputFilterPreviewConfiguration {
  readonly kind: "inputFilters";
  readonly left: Readonly<{ readonly hpfHz?: number; readonly lpfHz?: number }>;
  readonly right: Readonly<{ readonly hpfHz?: number; readonly lpfHz?: number }>;
}

/** A configuration accepted by the first response-preview slice. */
export type ResponsePreviewConfiguration = EffectDecl | InputFilterPreviewConfiguration;

/** A generated inclusive frequency axis. */
export interface ResponsePreviewGrid {
  readonly kind: "linear" | "logarithmic";
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
}

/** Caller metadata echoed as correlation; it is never treated as an active plan binding. */
export interface ResponseTargetCorrelation {
  readonly trackId?: string;
  readonly rack?: "console" | "inserts";
  readonly slotId?: string;
}

/** Bounded control-plane resources for one preview instance. */
export interface ResponsePreviewLimits {
  readonly maximumPreparedBytes?: number | bigint;
  readonly maximumTotalStateBytes?: number | bigint;
  readonly maximumScratchBytes?: number | bigint;
  readonly maximumAutomationSpansPerBlock?: number;
  readonly maximumResultBytes?: number;
  readonly requestDeadlineMs?: number;
}

/** The explicit response request accepted by both SDK entry points. */
export interface ResponsePreviewQuery {
  readonly configurationId: bigint;
  readonly sampleRateHz: 44_100 | 48_000 | 88_200 | 96_000;
  readonly quantumFrames: number;
  readonly configuration: ResponsePreviewConfiguration;
  readonly targetCorrelation?: ResponseTargetCorrelation;
  readonly grid: ResponsePreviewGrid;
  readonly channels?: Channel;
  readonly fields?: "total" | "totalAndSections";
}

/** One returned owner section with independently owned channel arrays. */
export interface ResponsePreviewSection {
  readonly id: number;
  readonly name: string;
  readonly leftDb?: Float32Array;
  readonly rightDb?: Float32Array;
}

/** Generated owner declaration exposed through `capabilities`. */
export interface ResponsePreviewCapability {
  readonly owner: "effect" | "builtins";
  readonly target: string;
  readonly id: number;
  readonly name: string;
  readonly axisUnit: string;
  readonly unit: string;
  readonly floorDb: number;
  readonly sections: readonly Readonly<{ readonly id: number; readonly name: string }>[];
}

/** A response query's owned result. */
export interface ResponsePreviewResult {
  readonly requestedConfiguration: ResponsePreviewConfiguration;
  readonly configurationId: bigint;
  readonly sampleRateHz: number;
  readonly mode: "requestedConfiguration";
  readonly targetCorrelation?: ResponseTargetCorrelation;
  readonly frequenciesHz: Float32Array;
  readonly totalLeftDb?: Float32Array;
  readonly totalRightDb?: Float32Array;
  readonly sections: readonly ResponsePreviewSection[];
  readonly floorDb: number;
  readonly bypass: boolean;
  readonly enabledLeft: readonly boolean[];
  readonly enabledRight: readonly boolean[];
  readonly retainedBytes: bigint;
  readonly resultBytes: bigint;
}

type RawExports = Record<string, unknown> & { readonly memory: WebAssembly.Memory };

/** The most request frame bytes the engine's session-less response entry takes. */
const MAXIMUM_REQUEST_FRAME_BYTES = 1 << 20;
const FRAME_EXPORTS = [
  "miso_engine_web_v1_response_frame_ptr",
  "miso_engine_web_v1_response_frame_submit",
  "miso_engine_web_v1_response_frame_reply_ptr",
  "miso_engine_web_v1_response_frame_reply_bytes",
  "miso_engine_web_v1_response_frame_close",
] as const;
const RESULT_VALUES: ReadonlyMap<string, number> = new Map(
  ABI_LAYOUT.constants.resultCodes.map((row) => [row.name, row.value] as const),
);
const HPF_CONTROL = CATALOG.builtins.parameters.find((row) => row.name === "hpf_hz")?.id;
const LPF_CONTROL = CATALOG.builtins.parameters.find((row) => row.name === "lpf_hz")?.id;
/** The control ID of the effect-wide bypass in a response request: lane `both`, value 0 or 1. */
const BYPASS_CONTROL = 0;

const capabilityRows: ResponsePreviewCapability[] = [];
for (const descriptor of CATALOG.effects) {
  if (descriptor.response !== null) {
    capabilityRows.push(Object.freeze({
      owner: "effect",
      target: descriptor.id,
      id: descriptor.response.id,
      name: descriptor.response.name,
      axisUnit: descriptor.response.axisUnit,
      unit: descriptor.response.unit,
      floorDb: descriptor.response.floorDb,
      sections: Object.freeze(descriptor.response.sections.map((section) => Object.freeze({ ...section }))),
    }));
  }
}
const builtinResponse = CATALOG.builtins.response;
capabilityRows.push(Object.freeze({
  owner: "builtins",
  target: "inputFilters",
  id: builtinResponse.id,
  name: builtinResponse.name,
  axisUnit: builtinResponse.axisUnit,
  unit: builtinResponse.unit,
  floorDb: builtinResponse.floorDb,
  sections: Object.freeze(builtinResponse.sections.map((section) => Object.freeze({ ...section }))),
}));

/** Generated native response declarations. */
export const RESPONSE_CAPABILITIES: readonly ResponsePreviewCapability[] = Object.freeze(capabilityRows);

function rawExports(instance: WebAssembly.Instance): RawExports {
  const exports = instance.exports as unknown as RawExports;
  if (!(exports.memory instanceof WebAssembly.Memory)) {
    throw new MisoUsageError("the response asset does not export linear memory");
  }
  return exports;
}

function callable(exports: RawExports, name: string): (...args: number[]) => number {
  const value = exports[name];
  if (typeof value !== "function") throw new MisoUsageError(`the response asset does not export ${name}`);
  return value as (...args: number[]) => number;
}

function finiteInteger(value: number, name: string, minimum: number, maximum: number): number {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) {
    throw new MisoUsageError(`${name} must be an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

function u64(value: number | bigint, name: string): bigint {
  const normalized = typeof value === "bigint" ? value : BigInt(finiteInteger(value, name, 0, Number.MAX_SAFE_INTEGER));
  if (normalized < 0n || normalized > 0xffff_ffff_ffff_ffffn) throw new MisoUsageError(`${name} must fit u64`);
  return normalized;
}

function channelName(channel: Channel | undefined): Channel {
  if (channel === undefined) return "both";
  if (channel === "left" || channel === "right" || channel === "both") return channel;
  throw new MisoUsageError("channels must be left, right, or both");
}

function wantsSections(fields: ResponsePreviewQuery["fields"]): boolean {
  if (fields === undefined || fields === "totalAndSections") return true;
  if (fields === "total") return false;
  throw new MisoUsageError("fields must be total or totalAndSections");
}

function gridName(kind: unknown): "linear" | "logarithmic" {
  if (kind === "linear" || kind === "logarithmic") return kind;
  throw new MisoUsageError("grid.kind must be linear or logarithmic");
}

function qualityName(value: EffectDecl["options"]["quality"]): "normal" {
  if (value !== "normal") throw new MisoUsageError("response previews support only normal native quality");
  return "normal";
}

function linkModeName(value: EffectDecl["options"]["linkMode"]): "dual_mono" | "maximum" | "average" {
  if (value === "dual_mono") return "dual_mono";
  if (value === "maximum") return "maximum";
  return "average";
}

function asF32(value: number | undefined, name: string): number {
  const normalized = value ?? 0;
  if (!Number.isFinite(normalized)) throw new MisoUsageError(`${name} must be finite`);
  return Math.fround(normalized);
}

function isInputFilters(
  configuration: ResponsePreviewConfiguration,
): configuration is InputFilterPreviewConfiguration {
  return "kind" in configuration && configuration.kind === "inputFilters";
}

function resultValue(name: string): number {
  return RESULT_VALUES.get(name) ?? (RESULT_VALUES.get("internal") as number);
}

/** A refusal of the module itself (the frame never reached the core's response code). */
function responseError(result: number): MisoEngineError {
  return new MisoEngineError("the response query was refused", {
    phase: "output",
    code: resultName(result, "call"),
    result,
  });
}

function normalizedConfiguration(configuration: ResponsePreviewConfiguration): ResponsePreviewConfiguration {
  if (isInputFilters(configuration)) {
    return Object.freeze({
      kind: "inputFilters",
      left: Object.freeze({ hpfHz: asF32(configuration.left.hpfHz, "left.hpfHz"), lpfHz: asF32(configuration.left.lpfHz, "left.lpfHz") }),
      right: Object.freeze({ hpfHz: asF32(configuration.right.hpfHz, "right.hpfHz"), lpfHz: asF32(configuration.right.lpfHz, "right.lpfHz") }),
    });
  }
  const descriptor = CATALOG.effects.find((candidate) => candidate.id === configuration.effectId);
  if (descriptor === undefined) throw new MisoUsageError(`unknown generated effect ${configuration.effectId}`);
  if (descriptor.response === null) {
    throw new MisoUsageError(`effect ${configuration.effectId} does not publish a response capability`);
  }
  const parameters = Object.fromEntries(descriptor.parameters.map((row) => {
    const supplied = configuration.parameters[row.name];
    if (supplied !== undefined) return [row.name, supplied];
    if (row.domainName === "boolean") return [row.name, row.default !== 0];
    if (row.domainName === "enumeration") {
      return [row.name, row.enumChoices.find((choice) => choice.value === row.default)?.label ?? row.default];
    }
    return [row.name, row.default];
  }));
  return Object.freeze({
    ...configuration,
    parameters: Object.freeze(parameters),
  }) as ResponsePreviewConfiguration;
}

/** A synchronous query client around one analysis-only Wasm instance. */
export class ResponsePreviewModule {
  readonly #exports: RawExports;
  readonly #responseLimits: {
    readonly maximumPreparedBytes: bigint;
    readonly maximumTotalStateBytes: bigint;
    readonly maximumScratchBytes: bigint;
    readonly maximumAutomationSpansPerBlock: number;
    readonly maximumResultBytes: number;
    readonly requestDeadlineMs: number;
  };
  #closed = false;
  #busy = false;
  #nextRequestId = 1n;

  constructor(instance: WebAssembly.Instance, responseLimits: ResponsePreviewLimits = {}) {
    this.#exports = rawExports(instance);
    this.#responseLimits = {
      maximumPreparedBytes: u64(responseLimits.maximumPreparedBytes ?? (1 << 20), "maximumPreparedBytes"),
      maximumTotalStateBytes: u64(responseLimits.maximumTotalStateBytes ?? (1 << 30), "maximumTotalStateBytes"),
      maximumScratchBytes: u64(responseLimits.maximumScratchBytes ?? (1 << 30), "maximumScratchBytes"),
      maximumAutomationSpansPerBlock: finiteInteger(responseLimits.maximumAutomationSpansPerBlock ?? 4096, "maximumAutomationSpansPerBlock", 1, 0xffff_ffff),
      maximumResultBytes: finiteInteger(responseLimits.maximumResultBytes ?? (16 << 20), "maximumResultBytes", 1, 16 << 20),
      requestDeadlineMs: finiteInteger(responseLimits.requestDeadlineMs ?? 5000, "requestDeadlineMs", 1, 2_147_483_647),
    };
    for (const name of FRAME_EXPORTS) callable(this.#exports, name);
  }

  get capabilities(): readonly ResponsePreviewCapability[] { return RESPONSE_CAPABILITIES; }

  query(request: ResponsePreviewQuery): ResponsePreviewResult {
    if (this.#closed) throw new MisoUsageError("the response preview is closed");
    if (this.#busy) throw new MisoUsageError("a response query is already in flight");
    this.#busy = true;
    try {
      return this.#query(request);
    } finally {
      this.#busy = false;
    }
  }

  close(): void {
    if (this.#closed) return;
    callable(this.#exports, "miso_engine_web_v1_response_frame_close")();
    this.#closed = true;
  }

  #query(request: ResponsePreviewQuery): ResponsePreviewResult {
    if (request.configurationId < 0n || request.configurationId > 0xffff_ffff_ffff_ffffn) throw new MisoUsageError("configurationId must fit u64");
    finiteInteger(request.sampleRateHz, "sampleRateHz", 1, 0xffff_ffff);
    finiteInteger(request.quantumFrames, "quantumFrames", 1, 0xffff_ffff);
    finiteInteger(request.grid.points, "grid.points", 2, 0xffff_ffff);
    const grid = gridName(request.grid.kind);
    if (!Number.isFinite(request.grid.minimumHz) || !Number.isFinite(request.grid.maximumHz)) throw new MisoUsageError("grid endpoints must be finite");
    if (grid === "logarithmic" && request.grid.minimumHz <= 0) throw new MisoUsageError("logarithmic grid minimumHz must be positive");
    const channels = channelName(request.channels);
    const sections = wantsSections(request.fields);
    const configuration = normalizedConfiguration(request.configuration);
    const requestId = this.#nextRequestId;
    this.#nextRequestId += 1n;
    const frame = encodeResponseRequest({
      requestId,
      ...this.#target(configuration),
      grid,
      points: request.grid.points,
      minimumHz: asF32(request.grid.minimumHz, "grid.minimumHz"),
      maximumHz: asF32(request.grid.maximumHz, "grid.maximumHz"),
      channels,
      sections,
      configurationId: request.configurationId,
      sampleRateHz: request.sampleRateHz,
      quantumFrames: request.quantumFrames,
      maximumPreparedBytes: this.#responseLimits.maximumPreparedBytes,
      maximumStateBytes: this.#responseLimits.maximumTotalStateBytes,
      maximumScratchBytes: this.#responseLimits.maximumScratchBytes,
      maximumAutomationSpans: this.#responseLimits.maximumAutomationSpansPerBlock,
      maximumReplyBytes: this.#responseLimits.maximumResultBytes,
    });
    if (frame.length > MAXIMUM_REQUEST_FRAME_BYTES) throw new MisoUsageError("the response request exceeds the frame bound");
    const memory = this.#exports.memory;
    const requestPtr = callable(this.#exports, "miso_engine_web_v1_response_frame_ptr")(frame.length);
    if (requestPtr === 0) throw responseError(resultValue("refusedBudget"));
    new Uint8Array(memory.buffer, requestPtr, frame.length).set(frame);
    const result = callable(this.#exports, "miso_engine_web_v1_response_frame_submit")(frame.length);
    if (result !== 0) throw responseError(result);
    const replyPtr = callable(this.#exports, "miso_engine_web_v1_response_frame_reply_ptr")();
    const replyBytes = callable(this.#exports, "miso_engine_web_v1_response_frame_reply_bytes")();
    const reply = new Uint8Array(memory.buffer.slice(replyPtr, replyPtr + replyBytes));
    const decoded = decodeResponseReply(reply);
    if (decoded.requestId !== requestId) {
      throw new MisoEngineError("the response reply answers another request", {
        phase: "output", code: "internal", result: resultValue("internal"),
      });
    }
    if (!decoded.ok) {
      const resultName = responseRefusalResult(decoded.status);
      const diagnostics: readonly MisoDiagnostic[] = decoded.diagnostics.map((row) => ({ code: row.code, path: "" }));
      throw new MisoEngineError("the response query was refused", {
        phase: "output",
        code: (diagnostics[0]?.code ?? resultName) as MisoEngineError["code"],
        result: resultValue(resultName),
        diagnostics,
      });
    }
    return this.#result(request, configuration, decoded.curves, sections, BigInt(reply.length));
  }

  /** The target and override rows of a configuration, as the frame names them. */
  #target(
    configuration: ResponsePreviewConfiguration,
  ): Pick<ResponseFrameRequest, "target" | "overrides"> {
    if (isInputFilters(configuration)) {
      if (HPF_CONTROL === undefined || LPF_CONTROL === undefined) {
        throw new MisoUsageError("the generated catalog has no input-filter rows");
      }
      const row = (control: number, lane: "left" | "right", value: number | undefined): ResponseFrameOverride => ({
        control, lane, value: asF32(value, `${lane}.cutoff`),
      });
      return {
        target: { kind: "inputFilters" },
        overrides: [
          row(HPF_CONTROL, "left", configuration.left.hpfHz),
          row(LPF_CONTROL, "left", configuration.left.lpfHz),
          row(HPF_CONTROL, "right", configuration.right.hpfHz),
          row(LPF_CONTROL, "right", configuration.right.lpfHz),
        ],
      };
    }
    const descriptor = CATALOG.effects.find((candidate) => candidate.id === configuration.effectId);
    if (descriptor === undefined) throw new MisoUsageError(`unknown generated effect ${configuration.effectId}`);
    const overrides: ResponseFrameOverride[] = Object.entries(configuration.parameters)
      .flatMap(([name, value]) => parameterRows(
        descriptor,
        name,
        value,
        configuration.options.channel,
        `response.${name}`,
      ))
      .map((row) => ({ control: Number(row.parameter_id), lane: row.channel as Channel, value: Number(row.value) }));
    if (configuration.options.bypass) overrides.push({ control: BYPASS_CONTROL, lane: "both", value: 1 });
    return {
      target: {
        kind: "effect",
        effect: configuration.effectId,
        quality: qualityName(configuration.options.quality),
        linkMode: linkModeName(configuration.options.linkMode),
      },
      overrides,
    };
  }

  #result(
    request: ResponsePreviewQuery,
    configuration: ResponsePreviewConfiguration,
    curves: ResponseFrameCurves,
    sections: boolean,
    resultBytes: bigint,
  ): ResponsePreviewResult {
    const points = curves.frequenciesHz.length;
    const sectionCount = curves.sectionCount;
    const capability = isInputFilters(configuration)
      ? RESPONSE_CAPABILITIES.find((candidate) => candidate.owner === "builtins")
      : RESPONSE_CAPABILITIES.find((candidate) => candidate.owner === "effect" && candidate.target === configuration.effectId);
    const returned: ResponsePreviewSection[] = [];
    for (let index = 0; sections && index < sectionCount; index += 1) {
      const declaration = capability?.sections[index];
      if (declaration === undefined) continue;
      const leftDb = curves.sectionsLeftDb?.slice(index * points, (index + 1) * points);
      const rightDb = curves.sectionsRightDb?.slice(index * points, (index + 1) * points);
      returned.push(Object.freeze({
        id: declaration.id,
        name: declaration.name,
        ...(leftDb === undefined ? {} : { leftDb }),
        ...(rightDb === undefined ? {} : { rightDb }),
      }));
    }
    const enabledLeft = Object.freeze(Array.from({ length: sectionCount }, (_, index) => (curves.enabledLeft & (1 << index)) !== 0));
    const enabledRight = Object.freeze(Array.from({ length: sectionCount }, (_, index) => (curves.enabledRight & (1 << index)) !== 0));
    return Object.freeze({
      requestedConfiguration: configuration,
      configurationId: curves.configurationId,
      sampleRateHz: curves.sampleRateHz,
      mode: "requestedConfiguration" as const,
      ...(request.targetCorrelation === undefined ? {} : { targetCorrelation: Object.freeze({ ...request.targetCorrelation }) }),
      frequenciesHz: curves.frequenciesHz,
      ...(curves.leftDb === undefined ? {} : { totalLeftDb: curves.leftDb }),
      ...(curves.rightDb === undefined ? {} : { totalRightDb: curves.rightDb }),
      sections: Object.freeze(returned),
      floorDb: curves.floorDb,
      bypass: curves.bypass,
      enabledLeft,
      enabledRight,
      retainedBytes: curves.retainedBytes,
      resultBytes,
    });
  }
}
