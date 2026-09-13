import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import { MisoEngineError, MisoUsageError, resultName } from "./errors.ts";
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
  readonly rack?: "simd1" | "dynamic" | "simd2";
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
type ResponseField = Readonly<{ readonly name: string; readonly offset: number }>;

const responseRequestFields = ABI_LAYOUT.structures.responseRequest.fields as readonly ResponseField[];
const responseParameterFields = ABI_LAYOUT.structures.responseParameter.fields as readonly ResponseField[];
const responseResultFields = ABI_LAYOUT.structures.responseResult.fields as readonly ResponseField[];

function offset(fields: readonly ResponseField[], name: string): number {
  const field = fields.find((candidate) => candidate.name === name);
  if (field === undefined) throw new Error(`generated ABI is missing response field ${name}`);
  return field.offset;
}

const requestOffset = (name: string) => offset(responseRequestFields, name);
const parameterOffset = (name: string) => offset(responseParameterFields, name);
const resultOffset = (name: string) => offset(responseResultFields, name);

const TARGET_EFFECT = ABI_LAYOUT.constants.responseTargets.find((row) => row.name === "effect")?.value ?? 1;
const TARGET_INPUT_FILTERS = ABI_LAYOUT.constants.responseTargets.find((row) => row.name === "inputFilters")?.value ?? 2;
const GRID_LINEAR = ABI_LAYOUT.constants.responseGrids.find((row) => row.name === "linear")?.value ?? 1;
const GRID_LOGARITHMIC = ABI_LAYOUT.constants.responseGrids.find((row) => row.name === "logarithmic")?.value ?? 2;
const CHANNEL_LEFT = ABI_LAYOUT.constants.responseChannels.find((row) => row.name === "left")?.value ?? 1;
const CHANNEL_RIGHT = ABI_LAYOUT.constants.responseChannels.find((row) => row.name === "right")?.value ?? 2;
const CHANNEL_BOTH = ABI_LAYOUT.constants.responseChannels.find((row) => row.name === "both")?.value ?? 3;
const FIELD_TOTAL = ABI_LAYOUT.constants.responseFields.find((row) => row.name === "total")?.value ?? 1;
const FIELD_SECTIONS = ABI_LAYOUT.constants.responseFields.find((row) => row.name === "sections")?.value ?? 2;

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

function channelBits(channel: Channel): number {
  if (channel === "left") return CHANNEL_LEFT;
  if (channel === "right") return CHANNEL_RIGHT;
  return CHANNEL_BOTH;
}

function qualityValue(value: EffectDecl["options"]["quality"]): number {
  if (value !== "normal") throw new MisoUsageError("response previews support only normal native quality");
  return 2;
}

function linkModeValue(value: EffectDecl["options"]["linkMode"]): number {
  if (value === "dual_mono") return 1;
  if (value === "maximum") return 2;
  return 3;
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

function responseError(result: number): MisoEngineError {
  return new MisoEngineError("the response query was refused", {
    phase: "output",
    code: resultName(result, "call"),
    result,
  });
}

function copyF32(memory: WebAssembly.Memory, pointer: number, count: number): Float32Array {
  if (!Number.isSafeInteger(pointer) || pointer < 0 || count < 0) throw new MisoUsageError("invalid response vector bounds");
  const bytes = count * 4;
  if (!Number.isSafeInteger(bytes) || pointer + bytes > memory.buffer.byteLength) {
    throw new MisoEngineError("the response result exceeded Wasm memory", {
      phase: "output", code: "bufferTooSmall", result: 4,
    });
  }
  return new Float32Array(memory.buffer.slice(pointer, pointer + bytes));
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
  readonly #limits: {
    readonly maximumPreparedBytes: bigint;
    readonly maximumTotalStateBytes: bigint;
    readonly maximumScratchBytes: bigint;
    readonly maximumAutomationSpansPerBlock: number;
    readonly maximumResultBytes: number;
    readonly requestDeadlineMs: number;
  };
  #closed = false;
  #busy = false;

  constructor(instance: WebAssembly.Instance, limits: ResponsePreviewLimits = {}) {
    this.#exports = rawExports(instance);
    this.#limits = {
      maximumPreparedBytes: u64(limits.maximumPreparedBytes ?? (1 << 20), "maximumPreparedBytes"),
      maximumTotalStateBytes: u64(limits.maximumTotalStateBytes ?? (1 << 30), "maximumTotalStateBytes"),
      maximumScratchBytes: u64(limits.maximumScratchBytes ?? (1 << 30), "maximumScratchBytes"),
      maximumAutomationSpansPerBlock: finiteInteger(limits.maximumAutomationSpansPerBlock ?? 4096, "maximumAutomationSpansPerBlock", 1, 0xffff_ffff),
      maximumResultBytes: finiteInteger(limits.maximumResultBytes ?? (16 << 20), "maximumResultBytes", 1, 16 << 20),
      requestDeadlineMs: finiteInteger(limits.requestDeadlineMs ?? 5000, "requestDeadlineMs", 1, 2_147_483_647),
    };
    for (const name of [
      "miso_engine_web_v1_response_request_ptr", "miso_engine_web_v1_response_effect_id_ptr",
      "miso_engine_web_v1_response_parameter_ptr", "miso_engine_web_v1_response_query",
      "miso_engine_web_v1_response_result_ptr", "miso_engine_web_v1_response_result_bytes",
      "miso_engine_web_v1_response_close",
    ]) callable(this.#exports, name);
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
    callable(this.#exports, "miso_engine_web_v1_response_close")();
    this.#closed = true;
  }

  #query(request: ResponsePreviewQuery): ResponsePreviewResult {
    if (request.configurationId < 0n || request.configurationId > 0xffff_ffff_ffff_ffffn) throw new MisoUsageError("configurationId must fit u64");
    finiteInteger(request.sampleRateHz, "sampleRateHz", 1, 0xffff_ffff);
    finiteInteger(request.quantumFrames, "quantumFrames", 1, 0xffff_ffff);
    finiteInteger(request.grid.points, "grid.points", 2, 0xffff_ffff);
    if (!Number.isFinite(request.grid.minimumHz) || !Number.isFinite(request.grid.maximumHz)) throw new MisoUsageError("grid endpoints must be finite");
    if (request.grid.kind === "logarithmic" && request.grid.minimumHz <= 0) throw new MisoUsageError("logarithmic grid minimumHz must be positive");
    const channels = channelBits(request.channels ?? "both");
    const fields = request.fields === "total" ? FIELD_TOTAL : FIELD_TOTAL | FIELD_SECTIONS;
    const configuration = normalizedConfiguration(request.configuration);
    const inputFilters = isInputFilters(configuration);
    const requestPtr = callable(this.#exports, "miso_engine_web_v1_response_request_ptr")();
    const memory = this.#exports.memory;
    const view = new DataView(memory.buffer);
    const setU32 = (name: string, value: number) => view.setUint32(requestPtr + requestOffset(name), value >>> 0, true);
    const setF32 = (name: string, value: number) => view.setFloat32(requestPtr + requestOffset(name), value, true);
    const setU64 = (name: string, value: bigint) => view.setBigUint64(requestPtr + requestOffset(name), value, true);
    setU32("structSize", ABI_LAYOUT.structures.responseRequest.bytes);
    setU32("abiVersion", ABI_LAYOUT.abiVersion);
    setU32("target", inputFilters ? TARGET_INPUT_FILTERS : TARGET_EFFECT);
    setU32("grid", request.grid.kind === "linear" ? GRID_LINEAR : GRID_LOGARITHMIC);
    setU32("channels", channels);
    setU32("fields", fields);
    setU32("points", request.grid.points);
    setU32("sampleRateHz", request.sampleRateHz);
    setU32("quantumFrames", request.quantumFrames);
    setF32("minimumHz", asF32(request.grid.minimumHz, "grid.minimumHz"));
    setF32("maximumHz", asF32(request.grid.maximumHz, "grid.maximumHz"));
    setU64("configurationId", request.configurationId);
    setU64("maximumPreparedBytes", this.#limits.maximumPreparedBytes);
    setU64("maximumTotalStateBytes", this.#limits.maximumTotalStateBytes);
    setU64("maximumScratchBytes", this.#limits.maximumScratchBytes);
    setU32("maximumAutomationSpansPerBlock", this.#limits.maximumAutomationSpansPerBlock);
    setU32("maximumResultBytes", this.#limits.maximumResultBytes);
    const idPtr = callable(this.#exports, "miso_engine_web_v1_response_effect_id_ptr")();
    const parameterPtr = callable(this.#exports, "miso_engine_web_v1_response_parameter_ptr")();
    const bytes = new Uint8Array(memory.buffer);
    let parameterCount = 0;
    if (inputFilters) {
      setU32("effectIdBytes", 0);
      setU32("parameterCount", 0);
      setF32("leftHpfHz", asF32(configuration.left.hpfHz, "left.hpfHz"));
      setF32("leftLpfHz", asF32(configuration.left.lpfHz, "left.lpfHz"));
      setF32("rightHpfHz", asF32(configuration.right.hpfHz, "right.hpfHz"));
      setF32("rightLpfHz", asF32(configuration.right.lpfHz, "right.lpfHz"));
    } else {
      const effectId = new TextEncoder().encode(configuration.effectId);
      const maxId = Number(ABI_LAYOUT.constants.maximumResponseEffectIdBytes);
      if (effectId.length === 0 || effectId.length > maxId) throw new MisoUsageError("effect ID exceeds the response bound");
      bytes.set(effectId, idPtr);
      setU32("effectIdBytes", effectId.length);
      const descriptor = CATALOG.effects.find((candidate) => candidate.id === configuration.effectId);
      if (descriptor === undefined) throw new MisoUsageError(`unknown generated effect ${configuration.effectId}`);
      const rows = Object.entries(configuration.parameters)
        .flatMap(([name, value]) => parameterRows(descriptor, name, value, configuration.options.channel, `response.${name}`));
      const parameterBytes = Number(ABI_LAYOUT.structures.responseParameter.bytes);
      if (rows.length > Number(ABI_LAYOUT.constants.maximumResponseParameterOverrides)) throw new MisoUsageError("too many response parameter overrides");
      for (const row of rows) {
        const at = parameterPtr + parameterCount * parameterBytes;
        const parameterView = new DataView(memory.buffer);
        parameterView.setUint32(at + parameterOffset("parameterId"), Number(row.parameter_id), true);
        parameterView.setUint32(at + parameterOffset("channel"), row.channel === "left" ? CHANNEL_LEFT : row.channel === "right" ? CHANNEL_RIGHT : CHANNEL_BOTH, true);
        parameterView.setFloat32(at + parameterOffset("value"), Number(row.value), true);
        parameterView.setUint32(at + parameterOffset("reserved"), 0, true);
        parameterCount += 1;
      }
      setU32("parameterCount", parameterCount);
      setU32("quality", qualityValue(configuration.options.quality));
      setU32("linkMode", linkModeValue(configuration.options.linkMode));
      setU32("bypass", configuration.options.bypass ? 1 : 0);
    }
    const result = callable(this.#exports, "miso_engine_web_v1_response_query")();
    if (result !== 0) throw responseError(result);
    const resultPtr = callable(this.#exports, "miso_engine_web_v1_response_result_ptr")();
    const resultBytes = callable(this.#exports, "miso_engine_web_v1_response_result_bytes")();
    const resultView = new DataView(this.#exports.memory.buffer, resultPtr, resultBytes);
    const resultHeaderBytes = ABI_LAYOUT.structures.responseResult.bytes;
    if (resultBytes < resultHeaderBytes) throw new MisoUsageError("response result header is truncated");
    const points = resultView.getUint32(resultOffset("points"), true);
    const sectionCount = resultView.getUint32(resultOffset("sectionCount"), true);
    const copyAt = (field: string, count: number): Float32Array | undefined => {
      const pointer = resultView.getUint32(resultOffset(field), true);
      return pointer === 0 ? undefined : copyF32(this.#exports.memory, resultPtr + pointer, count);
    };
    const frequenciesHz = copyAt("frequenciesOffset", points) ?? new Float32Array();
    const left = (channels & CHANNEL_LEFT) !== 0 ? copyAt("totalLeftOffset", points) : undefined;
    const right = (channels & CHANNEL_RIGHT) !== 0 ? copyAt("totalRightOffset", points) : undefined;
    const sections: ResponsePreviewSection[] = [];
    const capability = inputFilters
      ? RESPONSE_CAPABILITIES.find((candidate) => candidate.owner === "builtins")
      : RESPONSE_CAPABILITIES.find((candidate) => candidate.owner === "effect" && candidate.target === configuration.effectId);
    for (let index = 0; (fields & FIELD_SECTIONS) !== 0 && index < sectionCount; index += 1) {
      const declaration = capability?.sections[index];
      if (declaration === undefined) continue;
      const leftDb = (channels & CHANNEL_LEFT) !== 0
        ? copyAt("sectionsLeftOffset", points * sectionCount)?.slice(index * points, (index + 1) * points)
        : undefined;
      const rightDb = (channels & CHANNEL_RIGHT) !== 0
        ? copyAt("sectionsRightOffset", points * sectionCount)?.slice(index * points, (index + 1) * points)
        : undefined;
      sections.push(Object.freeze({
        id: declaration.id,
        name: declaration.name,
        ...(leftDb === undefined ? {} : { leftDb }),
        ...(rightDb === undefined ? {} : { rightDb }),
      }));
    }
    const enabledLeftMask = resultView.getUint32(resultOffset("enabledLeft"), true);
    const enabledRightMask = resultView.getUint32(resultOffset("enabledRight"), true);
    const enabledLeft = Object.freeze(Array.from({ length: sectionCount }, (_, index) => (enabledLeftMask & (1 << index)) !== 0));
    const enabledRight = Object.freeze(Array.from({ length: sectionCount }, (_, index) => (enabledRightMask & (1 << index)) !== 0));
    return Object.freeze({
      requestedConfiguration: configuration,
      configurationId: resultView.getBigUint64(resultOffset("configurationId"), true),
      sampleRateHz: resultView.getUint32(resultOffset("sampleRateHz"), true),
      mode: "requestedConfiguration" as const,
      ...(request.targetCorrelation === undefined ? {} : { targetCorrelation: Object.freeze({ ...request.targetCorrelation }) }),
      frequenciesHz,
      ...(left === undefined ? {} : { totalLeftDb: left }),
      ...(right === undefined ? {} : { totalRightDb: right }),
      sections: Object.freeze(sections),
      floorDb: resultView.getFloat32(resultOffset("floorDb"), true),
      bypass: resultView.getUint32(resultOffset("bypass"), true) !== 0,
      enabledLeft,
      enabledRight,
      retainedBytes: resultView.getBigUint64(resultOffset("retainedBytes"), true),
      resultBytes: resultView.getBigUint64(resultOffset("resultBytes"), true),
    });
  }
}
