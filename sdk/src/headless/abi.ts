import { ABI_LAYOUT } from "../generated/abi.js";
import { PROVENANCE } from "../generated/provenance.js";
import { session, type PrepareLimits, type SessionPlan, type SessionShape } from "../core/session.js";
import { MisoAssetHashError, MisoOfflineError, type SessionDiagnostic } from "./errors.js";
import { readBytes, sha256 } from "./io.js";

export interface WasmAssetOptions {
  readonly bytes?: Uint8Array;
  readonly url?: string | URL;
  readonly expectedSha256?: string;
}

export interface SessionDiagnostics {
  readonly ok: boolean;
  readonly diagnostics: readonly SessionDiagnostic[];
  readonly resources?: EngineResources;
}

export interface EngineStatus {
  readonly state: string;
  readonly lastResult: string;
  readonly backend: string;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  readonly memoryBytes: number;
}

export type EngineResources = Readonly<Record<string, number | bigint>>;

export interface WebExports {
  readonly memory: WebAssembly.Memory;
  miso_engine_web_v1_abi_version(): number;
  miso_engine_web_v1_config_bytes(): number;
  miso_engine_web_v1_config_new(): number;
  miso_engine_web_v1_config_ptr(handle: number): number;
  miso_engine_web_v1_prepare(handle: number): number;
  miso_engine_web_v1_buffer_ptr(handle: number, kind: number): number;
  miso_engine_web_v1_buffer_capacity(handle: number, kind: number): number;
  miso_engine_web_v1_compile(handle: number, tomlBytes: number): number;
  miso_engine_web_v1_source_submit(handle: number, idBytes: number, generation: bigint, startFrame: bigint, channels: number, frames: number, end: number): number;
  miso_engine_web_v1_source_seek(handle: number, idBytes: number, generation: bigint, sourceFrame: bigint): number;
  miso_engine_web_v1_render(handle: number, frames: number): number;
  miso_engine_web_v1_resource_ptr(handle: number): number;
  miso_engine_web_v1_command_submit(handle: number, count: number): number;
  miso_engine_web_v1_command_report_ptr(handle: number): number;
  miso_engine_web_v1_meter_lease(handle: number, enabled: number): number;
  miso_engine_web_v1_meter_poll(handle: number): number;
  miso_engine_web_v1_meter_header_ptr(handle: number): number;
  miso_engine_web_v1_console_track_count(handle: number): number;
  miso_engine_web_v1_console_track_id(handle: number, index: number): number;
  miso_engine_web_v1_status_ptr(handle: number): number;
  miso_engine_web_v1_dispose(handle: number): number;
}

export const WASM_ASSET_NAME = "miso-engine-v2-audio-worklet.simd128.wasm";
const defaultWasmUrl = new URL(`../../assets/${WASM_ASSET_NAME}`, import.meta.url);
const expectedDefaultHash = PROVENANCE.assets[WASM_ASSET_NAME].sha256;
const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

const resultNames: ReadonlyMap<number, string> = new Map(ABI_LAYOUT.constants.resultCodes.map((row) => [row.value, row.name]));
const stateNames: ReadonlyMap<number, string> = new Map(ABI_LAYOUT.constants.states.map((row) => [row.value, row.name]));
const backendNames: ReadonlyMap<number, string> = new Map(ABI_LAYOUT.constants.backends.map((row) => [row.value, row.name]));
const bufferKinds = Object.freeze(Object.fromEntries(ABI_LAYOUT.constants.bufferKinds.map((row) => [row.name, row.value]))) as Readonly<Record<string, number>>;

function fieldMap(structure: keyof typeof ABI_LAYOUT.structures): Readonly<Record<string, number>> {
  return Object.freeze(Object.fromEntries(ABI_LAYOUT.structures[structure].fields.map((field) => [field.name, field.offset])));
}

const statusOffsets = fieldMap("status");
const resourceOffsets = fieldMap("resourceReport");

function resultName(value: number): string {
  return resultNames.get(value) ?? `unknown(${value})`;
}

function positiveU32(value: number, path: string): number {
  if (!Number.isSafeInteger(value) || value <= 0 || value > 0xffff_ffff) throw new MisoOfflineError(`Expected positive u32 at ${path}`, "prepare");
  return value;
}

function u32(value: number, path: string): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffff_ffff) throw new MisoOfflineError(`Expected u32 at ${path}`, "prepare");
  return value;
}

function u64(value: bigint, path: string, positive: boolean): bigint {
  if (typeof value !== "bigint" || value < (positive ? 1n : 0n) || value > 0xffff_ffff_ffff_ffffn) {
    throw new MisoOfflineError(`Expected ${positive ? "positive " : ""}u64 at ${path}`, "prepare");
  }
  return value;
}

export function sessionHeader(toml: string, fallback = false): Readonly<{ sampleRateHz: number; quantumFrames: number }> {
  const scalar = (name: string): number | undefined => {
    const match = toml.match(new RegExp(`^\\s*${name}\\s*=\\s*\\+?([0-9](?:_?[0-9])*)\\s*(?:#.*)?$`, "m"));
    return match ? Number(match[1].replaceAll("_", "")) : undefined;
  };
  const sampleRateHz = scalar("sample_rate_hz");
  const quantumFrames = scalar("quantum_frames");
  if (sampleRateHz === undefined || quantumFrames === undefined) {
    if (fallback) return Object.freeze({ sampleRateHz: 48_000, quantumFrames: 128 });
    throw new MisoOfflineError("Session TOML must expose root sample_rate_hz and quantum_frames integer scalars", "prepare");
  }
  if (![44_100, 48_000, 88_200, 96_000].includes(sampleRateHz)) throw new MisoOfflineError("Unsupported launch sample rate", "prepare");
  return Object.freeze({ sampleRateHz, quantumFrames: positiveU32(quantumFrames, "quantum_frames") });
}

export function isSessionPlan(value: SessionPlan<SessionShape> | { readonly toml: string }): value is SessionPlan<SessionShape> {
  return "limits" in value && typeof value.limits === "function" && "json" in value;
}

export async function verifiedWasm(options: WasmAssetOptions = {}): Promise<Readonly<{ bytes: Uint8Array; sha256: string; asset: string }>> {
  if (options.bytes !== undefined && options.url !== undefined) throw new MisoOfflineError("Choose wasm.bytes or wasm.url, not both", "asset");
  const source = options.bytes ?? await readBytes(options.url ?? defaultWasmUrl);
  const bytes = new Uint8Array(source);
  const actual = await sha256(bytes);
  const expected = options.expectedSha256 ?? expectedDefaultHash;
  const asset = options.url === undefined ? WASM_ASSET_NAME : String(options.url);
  if (!/^[0-9a-f]{64}$/.test(expected)) throw new MisoOfflineError("expectedSha256 must be 64 lowercase hexadecimal digits", "asset");
  if (actual !== expected) throw new MisoAssetHashError(asset, expected, actual);
  return Object.freeze({ bytes, sha256: actual, asset });
}

function diagnostics(exports: WebExports, handle: number): readonly SessionDiagnostic[] {
  const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, bufferKinds.diagnostic);
  const capacity = exports.miso_engine_web_v1_buffer_capacity(handle, bufferKinds.diagnostic);
  if (pointer === 0 || capacity === 0) return Object.freeze([]);
  const bytes = new Uint8Array(exports.memory.buffer, pointer, capacity);
  const end = bytes.indexOf(0);
  const prefix = bytes.subarray(0, end < 0 ? capacity : end);
  let body: string;
  try { body = decoder.decode(prefix); } catch (_error) { return Object.freeze([{ code: "diagnostic.utf8", path: "$" }]); }
  return Object.freeze(body.split("\n").filter(Boolean).map((line) => {
    const tab = line.indexOf("\t");
    return Object.freeze({ code: tab < 0 ? line : line.slice(0, tab), path: tab < 0 ? "$" : line.slice(tab + 1) });
  }));
}

function prepareLimits<S extends SessionShape>(plan: SessionPlan<S> | { readonly toml: string }, header: Readonly<{ sampleRateHz: number; quantumFrames: number }>, overrides: Partial<PrepareLimits>): PrepareLimits {
  const candidate = plan as SessionPlan<SessionShape> | { readonly toml: string };
  if (isSessionPlan(candidate)) return candidate.limits(overrides);
  return session({ id: "sdk-headless-defaults", sampleRateHz: header.sampleRateHz as 44_100 | 48_000 | 88_200 | 96_000, quantumFrames: header.quantumFrames }).build().limits(overrides);
}

function writeConfig(exports: WebExports, handle: number, header: Readonly<{ sampleRateHz: number; quantumFrames: number }>, tomlBytes: number, limits: PrepareLimits): void {
  const pointer = exports.miso_engine_web_v1_config_ptr(handle);
  if (pointer === 0) throw new MisoOfflineError("Wasm returned no prepare-config pointer", "prepare");
  const view = new DataView(exports.memory.buffer, pointer, ABI_LAYOUT.structures.prepareConfig.bytes);
  const headers: Readonly<Record<string, number>> = Object.freeze({
    structSize: ABI_LAYOUT.structures.prepareConfig.bytes,
    abiVersion: ABI_LAYOUT.abiVersion,
    sampleRateHz: header.sampleRateHz,
    quantumFrames: header.quantumFrames,
  });
  for (const field of ABI_LAYOUT.structures.prepareConfig.fields) {
    const value = field.name in headers ? headers[field.name] : limits[field.name as keyof PrepareLimits];
    if (field.type === "u32") {
      const checked = u32(value as number, `limits.${field.name}`);
      view.setUint32(field.offset, checked, true);
    } else {
      view.setBigUint64(field.offset, u64(value as bigint, `limits.${field.name}`, false), true);
    }
  }
  if (tomlBytes > limits.sessionTomlBytes) throw new MisoOfflineError("Session TOML exceeds sessionTomlBytes", "prepare");
  if ((limits.consoleObservationTaps !== 0n && limits.consoleCommandQueueRecords === 0n)
      || (limits.consoleMasterTrackPlusOne !== 0n && limits.consoleObservationTaps === 0n)) {
    throw new MisoOfflineError("Invalid console observation prepare limits", "prepare");
  }
}

export class WasmBoundary {
  readonly exports: WebExports;
  readonly handle: number;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly resources: EngineResources;
  readonly assetSha256: string;
  private readonly memoryBuffer: ArrayBuffer;
  private disposed = false;

  private constructor(exports: WebExports, handle: number, header: Readonly<{ sampleRateHz: number; quantumFrames: number }>, assetSha256: string) {
    this.exports = exports;
    this.handle = handle;
    this.sampleRateHz = header.sampleRateHz;
    this.quantumFrames = header.quantumFrames;
    this.assetSha256 = assetSha256;
    this.memoryBuffer = exports.memory.buffer as ArrayBuffer;
    this.resources = this.readResources();
  }

  static async create<S extends SessionShape>(plan: SessionPlan<S> | { readonly toml: string }, overrides: Partial<PrepareLimits> = {}, wasm: WasmAssetOptions = {}, validationFallback = false): Promise<WasmBoundary> {
    const toml = plan.toml;
    const header = sessionHeader(toml, validationFallback);
    const asset = await verifiedWasm(wasm);
    let instance: WebAssembly.Instance;
    try {
      const moduleBytes = new Uint8Array(asset.bytes.byteLength);
      moduleBytes.set(asset.bytes);
      const module = await WebAssembly.compile(moduleBytes.buffer);
      instance = await WebAssembly.instantiate(module, {});
    }
    catch (_error) { throw new MisoOfflineError("Verified Wasm failed instantiation", "asset"); }
    const exports = instance.exports as unknown as WebExports;
    if (exports.miso_engine_web_v1_abi_version() !== ABI_LAYOUT.abiVersion
        || exports.miso_engine_web_v1_config_bytes() !== ABI_LAYOUT.structures.prepareConfig.bytes) {
      throw new MisoOfflineError("Wasm ABI version or prepare-config size mismatch", "prepare", 2);
    }
    const handle = exports.miso_engine_web_v1_config_new();
    if (handle === 0) throw new MisoOfflineError("Wasm refused a configuration handle", "prepare", 255);
    try {
      const bytes = encoder.encode(toml);
      writeConfig(exports, handle, header, bytes.byteLength, prepareLimits(plan, header, overrides));
      let result = exports.miso_engine_web_v1_prepare(handle);
      if (result !== 0) throw new MisoOfflineError(`Wasm prepare refused: ${resultName(result)}`, "prepare", result, diagnostics(exports, handle));
      const pointer = exports.miso_engine_web_v1_buffer_ptr(handle, bufferKinds.sessionToml);
      const capacity = exports.miso_engine_web_v1_buffer_capacity(handle, bufferKinds.sessionToml);
      if (pointer === 0 || bytes.byteLength > capacity) throw new MisoOfflineError("Wasm Session TOML staging is too small", "prepare", 4);
      new Uint8Array(exports.memory.buffer, pointer, bytes.byteLength).set(bytes);
      result = exports.miso_engine_web_v1_compile(handle, bytes.byteLength);
      if (result !== 0) throw new MisoOfflineError(`Wasm compile refused: ${resultName(result)}`, "compile", result, diagnostics(exports, handle));
      return new WasmBoundary(exports, handle, header, asset.sha256);
    } catch (error) {
      exports.miso_engine_web_v1_dispose(handle);
      throw error;
    }
  }

  assertLive(phase: "source" | "render" | "lifecycle"): void {
    if (this.disposed) throw new MisoOfflineError("Offline engine is disposed", "lifecycle");
    if (this.exports.memory.buffer !== this.memoryBuffer) throw new MisoOfflineError("Wasm memory changed; dispose and recreate the engine", phase, 9, diagnostics(this.exports, this.handle));
  }

  buffer(name: string): Readonly<{ pointer: number; capacity: number }> {
    this.assertLive("lifecycle");
    const kind = bufferKinds[name];
    const pointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, kind);
    const capacity = this.exports.miso_engine_web_v1_buffer_capacity(this.handle, kind);
    return Object.freeze({ pointer, capacity });
  }

  readDiagnostics(): readonly SessionDiagnostic[] { return diagnostics(this.exports, this.handle); }

  status(): EngineStatus {
    this.assertLive("lifecycle");
    const pointer = this.exports.miso_engine_web_v1_status_ptr(this.handle);
    const view = new DataView(this.exports.memory.buffer, pointer, ABI_LAYOUT.structures.status.bytes);
    if (view.getUint32(statusOffsets.structSize, true) !== ABI_LAYOUT.structures.status.bytes
        || view.getUint32(statusOffsets.abiVersion, true) !== ABI_LAYOUT.abiVersion
        || view.getUint32(statusOffsets.reserved0, true) !== 0
        || [0, 1, 2, 3].some((index) => view.getBigUint64(statusOffsets.reserved + index * 8, true) !== 0n)) throw new MisoOfflineError("Invalid Wasm status structure", "lifecycle", 255);
    return Object.freeze({
      state: stateNames.get(view.getUint32(statusOffsets.state, true)) ?? "unknown",
      lastResult: resultName(view.getUint32(statusOffsets.lastResult, true)),
      backend: backendNames.get(view.getUint32(statusOffsets.backend, true)) ?? "unknown",
      sampleRateHz: view.getUint32(statusOffsets.sampleRateHz, true),
      quantumFrames: view.getUint32(statusOffsets.quantumFrames, true),
      nextAbsoluteSample: view.getBigUint64(statusOffsets.nextAbsoluteSample, true),
      renderedQuanta: view.getBigUint64(statusOffsets.renderedQuanta, true),
      memoryBytes: this.exports.memory.buffer.byteLength,
    });
  }

  private readResources(): EngineResources {
    const pointer = this.exports.miso_engine_web_v1_resource_ptr(this.handle);
    const view = new DataView(this.exports.memory.buffer, pointer, ABI_LAYOUT.structures.resourceReport.bytes);
    if (view.getUint32(resourceOffsets.structSize, true) !== ABI_LAYOUT.structures.resourceReport.bytes
        || view.getUint32(resourceOffsets.abiVersion, true) !== ABI_LAYOUT.abiVersion
        || [0, 1, 2].some((index) => view.getUint32(resourceOffsets.reserved0 + index * 4, true) !== 0)
        || [0, 1, 2].some((index) => view.getBigUint64(resourceOffsets.reserved + index * 8, true) !== 0n)) throw new MisoOfflineError("Invalid Wasm resource structure", "prepare", 255);
    const resources: Record<string, number | bigint> = {};
    for (const field of ABI_LAYOUT.structures.resourceReport.fields) {
      if (field.name.startsWith("reserved") || field.name === "structSize" || field.name === "abiVersion") continue;
      resources[field.name] = field.type === "u32" ? view.getUint32(field.offset, true) : view.getBigUint64(field.offset, true);
    }
    return Object.freeze(resources);
  }

  dispose(): void {
    if (this.disposed) return;
    const result = this.exports.miso_engine_web_v1_dispose(this.handle);
    this.disposed = true;
    if (result !== 0) throw new MisoOfflineError(`Wasm dispose refused: ${resultName(result)}`, "lifecycle", result);
  }
}

/** Fresh-instance, capacity/NUL-bounded engine-true Session V1 validation. */
export async function validateSession(toml: string, wasm: WasmAssetOptions = {}): Promise<SessionDiagnostics> {
  let boundary: WasmBoundary | undefined;
  try {
    boundary = await WasmBoundary.create({ toml }, { sessionTomlBytes: Math.max(1 << 20, encoder.encode(toml).byteLength) }, wasm, true);
    return Object.freeze({ ok: true, diagnostics: Object.freeze([]), resources: boundary.resources });
  } catch (error) {
    if (error instanceof MisoOfflineError && (error.phase === "compile" || error.phase === "prepare")) {
      return Object.freeze({ ok: false, diagnostics: error.diagnostics });
    }
    throw error;
  } finally {
    boundary?.dispose();
  }
}
