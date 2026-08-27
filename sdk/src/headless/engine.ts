import type { SessionPlan, SessionShape } from "../core/session.js";
import { createHash } from "node:crypto";
import { closeSync, openSync, unlinkSync, writeSync } from "node:fs";
import { isSessionPlan, WasmBoundary, type EngineStatus } from "./abi.js";
import { OfflineConsole } from "./console.js";
import { MisoIntrospectionUnavailableError, MisoOfflineError, MisoSourceError } from "./errors.js";
import type {
  MeterFrame,
  OfflineEngine,
  OfflineEngineOptions,
  OfflineSource,
  RenderReport,
  RenderedAudio,
} from "./types.js";
import {
  decodeWave,
  f32lePlanarBytes,
  openWaveFile,
  parseWave,
  wav32fHeader,
  wav32fInterleavedBytes,
  type WaveData,
  type WaveFile,
} from "./wav.js";
import { ABI_LAYOUT } from "../generated/abi.js";

interface SourceDeclaration {
  readonly id: string;
  readonly sampleRateHz: number;
  readonly channels: number;
  readonly startFrame: number;
  readonly frames: number;
}

interface PreparedSource {
  readonly declaration: SourceDeclaration;
  readonly read: (absoluteStart: number, frames: number) => readonly Float32Array[];
  readonly close?: () => void;
  consumed: number;
}

interface SessionIntrospection {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly tracks: readonly string[];
  readonly sources: readonly SourceDeclaration[];
}

const meterOffsets = Object.freeze(Object.fromEntries(ABI_LAYOUT.structures.meterHeader.fields.map((field) => [field.name, field.offset]))) as Readonly<Record<string, number>>;
const encoder = new TextEncoder();

function numeric(value: unknown, sourceId: string, path: string): number {
  const number = typeof value === "string" ? Number(value) : value;
  if (typeof number !== "number" || !Number.isSafeInteger(number) || number < 0) throw new MisoSourceError("Session source declaration is not a non-negative safe integer", sourceId, path);
  return number;
}

function sessionIntrospection<S extends SessionShape>(plan: SessionPlan<S> | { readonly toml: string }): SessionIntrospection {
  const candidate = plan as SessionPlan<SessionShape> | { readonly toml: string };
  if (isSessionPlan(candidate)) {
    const sources = Object.freeze(candidate.json.sources.map((row) => {
      const id = String(row.id);
      const mapping = row.mapping as Readonly<Record<string, unknown>>;
      const region = mapping.region as Readonly<Record<string, unknown>>;
      return Object.freeze({
        id,
        sampleRateHz: numeric(row.sample_rate_hz, id, "sample_rate_hz"),
        channels: numeric(mapping.channel_count, id, "mapping.channel_count"),
        startFrame: numeric(region.start_sample, id, "mapping.region.start_sample"),
        frames: numeric(region.length_samples, id, "mapping.region.length_samples"),
      });
    }));
    return Object.freeze({
      sampleRateHz: numeric(candidate.json.sample_rate_hz, "$", "sample_rate_hz"),
      quantumFrames: numeric(candidate.json.quantum_frames, "$", "quantum_frames"),
      tracks: Object.freeze(candidate.tracks.map((track) => track.id)),
      sources,
    });
  }
  // Coordinator ruling 5438024085: do not grow a second TOML parser or guess zero-origin regions.
  // The additive engine-owned query will replace this typed temporary refusal when its ABI lands.
  throw new MisoIntrospectionUnavailableError("sources");
}

function byteOrder(left: string, right: string): number {
  const a = encoder.encode(left), b = encoder.encode(right);
  for (let index = 0; index < Math.min(a.length, b.length); index += 1) if (a[index] !== b[index]) return a[index] - b[index];
  return a.length - b.length;
}

function memorySource(id: string, input: readonly Float32Array[], declaration: SourceDeclaration): PreparedSource {
  if (input.length === 0 || input.length > 8 || input.some((plane) => !(plane instanceof Float32Array) || plane.length !== input[0].length)) {
    throw new MisoSourceError("In-memory source requires 1..8 equal-length Float32Array planes", id, "sources");
  }
  if (declaration.channels === 0) declaration = Object.freeze({ ...declaration, channels: input.length, frames: input[0].length });
  if (input.length !== declaration.channels) throw new MisoSourceError("In-memory source channel count differs from Session V1", id, "mapping.channel_count");
  if (declaration.startFrame + declaration.frames > input[0].length) throw new MisoSourceError("In-memory source is shorter than the declared region", id, "mapping.region");
  return {
    declaration,
    consumed: 0,
    read: (absoluteStart, frames) => input.map((plane) => plane.slice(absoluteStart, absoluteStart + frames)),
  };
}

function waveSource(id: string, wave: WaveData | WaveFile, declaration: SourceDeclaration, sessionRate: number): PreparedSource {
  const inferred = declaration.channels === 0 ? Object.freeze({ ...declaration, channels: wave.channels, frames: wave.frames }) : declaration;
  if (wave.sampleRateHz !== sessionRate || inferred.sampleRateHz !== sessionRate) throw new MisoSourceError("Source/session sample-rate mismatch; no implicit SRC exists", id, "sample_rate_hz");
  if (wave.channels !== inferred.channels) throw new MisoSourceError("WAV channel count differs from Session V1", id, "mapping.channel_count");
  if (inferred.startFrame + inferred.frames > wave.frames) throw new MisoSourceError("WAV is shorter than the declared source region", id, "mapping.region");
  return {
    declaration: inferred,
    consumed: 0,
    read: "decode" in wave
      ? (absoluteStart, frames) => wave.decode(absoluteStart, frames)
      : (absoluteStart, frames) => decodeWave(wave, absoluteStart, frames, id),
    close: "close" in wave ? () => wave.close() : undefined,
  };
}

async function prepareSources(introspection: SessionIntrospection, inputs: Readonly<Record<string, OfflineSource>>): Promise<readonly PreparedSource[]> {
  const rows = introspection.sources;
  const expected = new Set(rows.map((row) => row.id));
  for (const id of Object.keys(inputs)) if (!expected.has(id)) throw new MisoSourceError("Source input is not declared by the session", id, "sources");
  const prepared: PreparedSource[] = [];
  try {
    for (const row of rows) {
      const input = inputs[row.id];
      if (!input) throw new MisoSourceError("Session source has no headless input", row.id, "sources");
      if (Array.isArray(input)) {
        prepared.push(memorySource(row.id, input, row));
      } else {
        const wav = (input as Readonly<{ wav: Uint8Array | string }>).wav;
        if (wav instanceof Uint8Array) {
          prepared.push(waveSource(row.id, parseWave(new Uint8Array(wav), row.id), row, introspection.sampleRateHz));
        } else {
          const file = openWaveFile(wav, row.id);
          try { prepared.push(waveSource(row.id, file, row, introspection.sampleRateHz)); }
          catch (error) { try { file.close(); } catch (_closeError) { /* primary typed error wins */ } throw error; }
        }
      }
    }
    return Object.freeze(prepared.sort((left, right) => byteOrder(left.declaration.id, right.declaration.id)));
  } catch (error) {
    closeSources(prepared);
    throw error;
  }
}

function writeExact(fd: number, bytes: Uint8Array): void {
  let written = 0;
  while (written < bytes.byteLength) {
    const count = writeSync(fd, bytes, written, bytes.byteLength - written, null);
    if (count <= 0) throw new Error("output write made no progress");
    written += count;
  }
}

function closeSources(sources: readonly PreparedSource[]): void {
  for (const source of sources) {
    try { source.close?.(); } catch (_error) { /* disposal remains idempotent and best effort */ }
  }
}

class Engine<S extends SessionShape> implements OfflineEngine<S> {
  readonly console: OfflineConsole<S>;
  readonly resources;
  private pendingLeft: Float32Array = new Float32Array(0);
  private pendingRight: Float32Array = new Float32Array(0);
  private pendingOffset = 0;
  private deliveredFrames = 0;
  private disposed = false;
  private readonly totalFrames: number;

  constructor(private readonly boundary: WasmBoundary, private readonly sources: readonly PreparedSource[], plan?: SessionPlan<S>, meterLease = false) {
    this.console = new OfflineConsole(boundary, plan);
    this.resources = boundary.resources;
    this.totalFrames = sources.reduce((maximum, source) => Math.max(maximum, source.declaration.frames), 0);
    if (meterLease) {
      const result = boundary.exports.miso_engine_web_v1_meter_lease(boundary.handle, 1);
      if (result !== 0) throw new MisoOfflineError("Wasm refused the headless meter lease", "prepare", result, boundary.readDiagnostics());
    }
  }

  private assertLive(): void {
    if (this.disposed) throw new MisoOfflineError("Offline engine is disposed", "lifecycle");
    this.boundary.assertLive("lifecycle");
  }

  private feedSources(): void {
    const idBuffer = this.boundary.buffer("sourceId");
    const pcmBuffer = this.boundary.buffer("sourcePcm");
    const staging = new Float32Array(this.boundary.exports.memory.buffer, pcmBuffer.pointer, pcmBuffer.capacity / 4);
    const quantum = this.boundary.quantumFrames;
    for (const source of this.sources) {
      if (source.consumed === source.declaration.frames) continue;
      const frames = Math.min(quantum, source.declaration.frames - source.consumed);
      const absolute = source.declaration.startFrame + source.consumed;
      const planes = source.read(absolute, frames);
      const id = encoder.encode(source.declaration.id);
      if (id.byteLength > idBuffer.capacity || planes.length * quantum * 4 > pcmBuffer.capacity) throw new MisoSourceError("Source exceeds prepared Wasm staging capacity", source.declaration.id, "limits");
      new Uint8Array(this.boundary.exports.memory.buffer, idBuffer.pointer, id.byteLength).set(id);
      for (let channel = 0; channel < planes.length; channel += 1) staging.set(planes[channel], channel * quantum);
      const end = source.consumed + frames === source.declaration.frames;
      const result = this.boundary.exports.miso_engine_web_v1_source_submit(this.boundary.handle, id.byteLength, 1n, BigInt(absolute), planes.length, frames, end ? 1 : 0);
      if (result !== 0) throw new MisoOfflineError(`Source submission refused for ${source.declaration.id}`, "source", result, this.boundary.readDiagnostics());
      source.consumed += frames;
    }
  }

  private renderQuantum(): RenderedAudio {
    this.assertLive();
    this.feedSources();
    let result: number;
    try { result = this.boundary.exports.miso_engine_web_v1_render(this.boundary.handle, this.boundary.quantumFrames); }
    catch (_error) { throw new MisoOfflineError("Wasm render trapped; dispose and recreate the engine", "render", 255, this.boundary.readDiagnostics()); }
    if (result !== 0) throw new MisoOfflineError("Wasm render refused; dispose and recreate the engine", "render", result, this.boundary.readDiagnostics());
    const output = this.boundary.buffer("outputPcm");
    if (output.capacity !== this.boundary.quantumFrames * 8) throw new MisoOfflineError("Invalid Wasm output staging capacity", "render", 255);
    const left = new Float32Array(this.boundary.quantumFrames);
    const right = new Float32Array(this.boundary.quantumFrames);
    left.set(new Float32Array(this.boundary.exports.memory.buffer, output.pointer, this.boundary.quantumFrames));
    right.set(new Float32Array(this.boundary.exports.memory.buffer, output.pointer + this.boundary.quantumFrames * 4, this.boundary.quantumFrames));
    return Object.freeze({ left, right });
  }

  render(frames: number): RenderedAudio {
    this.assertLive();
    if (!Number.isSafeInteger(frames) || frames < 0) throw new MisoOfflineError("render(frames) requires a non-negative safe integer", "render");
    const left = new Float32Array(frames), right = new Float32Array(frames);
    let written = 0;
    while (written < frames) {
      if (this.pendingOffset === this.pendingLeft.length) {
        const block = this.renderQuantum();
        this.pendingLeft = block.left; this.pendingRight = block.right; this.pendingOffset = 0;
      }
      const count = Math.min(frames - written, this.pendingLeft.length - this.pendingOffset);
      left.set(this.pendingLeft.subarray(this.pendingOffset, this.pendingOffset + count), written);
      right.set(this.pendingRight.subarray(this.pendingOffset, this.pendingOffset + count), written);
      written += count; this.pendingOffset += count;
    }
    this.deliveredFrames += frames;
    return Object.freeze({ left, right });
  }

  renderAll(): RenderedAudio {
    return this.render(Math.max(0, this.totalFrames - this.deliveredFrames));
  }

  async renderToFile(path: string, options: Readonly<{ format?: "f32le-planar" | "wav32f" }> = {}): Promise<RenderReport> {
    this.assertLive();
    const format = options.format ?? "wav32f";
    if (format !== "f32le-planar" && format !== "wav32f") throw new MisoOfflineError(`Unsupported output format: ${String(format)}`, "output");
    const frames = Math.max(0, this.totalFrames - this.deliveredFrames);
    const digest = createHash("sha256");
    let fd = -1, byteCount = 0, created = false;
    try {
      fd = openSync(path, "wx"); created = true;
      if (format === "wav32f") {
        const header = wav32fHeader(frames, this.boundary.sampleRateHz);
        writeExact(fd, header); digest.update(header); byteCount += header.byteLength;
      }
      let remaining = frames;
      while (remaining > 0) {
        const audio = this.render(Math.min(this.boundary.quantumFrames, remaining));
        const bytes = format === "f32le-planar"
          ? f32lePlanarBytes(audio.left, audio.right, this.boundary.quantumFrames)
          : wav32fInterleavedBytes(audio.left, audio.right);
        writeExact(fd, bytes); digest.update(bytes); byteCount += bytes.byteLength;
        remaining -= audio.left.length;
      }
      closeSync(fd); fd = -1;
      return Object.freeze({ path, format, frames, bytes: byteCount, sha256: digest.digest("hex") });
    } catch (error) {
      if (fd >= 0) { try { closeSync(fd); } catch (_closeError) { /* output failure wins */ } }
      if (created) { try { unlinkSync(path); } catch (_unlinkError) { /* partial output cleanup is best effort */ } }
      if (error instanceof MisoOfflineError || error instanceof MisoSourceError) throw error;
      throw new MisoOfflineError(`Output write failed: ${path}`, "output");
    }
  }

  pollMeters(): MeterFrame | null {
    this.assertLive();
    const windows = this.boundary.exports.miso_engine_web_v1_meter_poll(this.boundary.handle);
    if (windows === 0) return null;
    const pointer = this.boundary.exports.miso_engine_web_v1_meter_header_ptr(this.boundary.handle);
    const header = new DataView(this.boundary.exports.memory.buffer, pointer, ABI_LAYOUT.structures.meterHeader.bytes);
    if (header.getUint32(meterOffsets.structSize, true) !== ABI_LAYOUT.structures.meterHeader.bytes
        || header.getUint32(meterOffsets.abiVersion, true) !== ABI_LAYOUT.abiVersion
        || header.getBigUint64(meterOffsets.reserved, true) !== 0n
        || header.getBigUint64(meterOffsets.reserved + 8, true) !== 0n) throw new MisoOfflineError("Invalid Wasm meter header", "lifecycle", 255);
    const trackCount = header.getUint32(meterOffsets.trackCount, true);
    const frame = this.boundary.buffer("meterFrame");
    if (frame.capacity !== (trackCount * 3 + 3) * 4) throw new MisoOfflineError("Invalid Wasm meter frame", "lifecycle", 255);
    const all = new Float32Array(this.boundary.exports.memory.buffer, frame.pointer, frame.capacity / 4);
    const peaks = new Float32Array(all.subarray(0, trackCount * 2 + 2));
    const trackGrDb = new Float32Array(all.subarray(trackCount * 2 + 2, trackCount * 3 + 2));
    const masterGrDb = header.getUint32(meterOffsets.masterGrPresent, true) === 1 ? all[trackCount * 3 + 2] : null;
    return Object.freeze({
      sequence: header.getBigUint64(meterOffsets.sequence, true), windows,
      trackCount, peaks, trackGrDb, masterGrDb,
      firstSample: header.getBigUint64(meterOffsets.firstSample, true),
      endSample: header.getBigUint64(meterOffsets.endSample, true),
    });
  }

  status(): EngineStatus { this.assertLive(); return this.boundary.status(); }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    closeSources(this.sources);
    this.boundary.dispose();
  }
}

export async function createOfflineEngine<S extends SessionShape>(options: OfflineEngineOptions<S>): Promise<OfflineEngine<S>> {
  const introspection = sessionIntrospection(options.session);
  const sources = await prepareSources(introspection, options.sources);
  let boundary: WasmBoundary | undefined;
  try {
    boundary = await WasmBoundary.create(options.session, options.limits, options.wasm);
    if (boundary.sampleRateHz !== introspection.sampleRateHz || boundary.quantumFrames !== introspection.quantumFrames) {
      throw new MisoOfflineError("Engine preparation shape differs from SessionPlan introspection", "prepare");
    }
    const plan = isSessionPlan(options.session as SessionPlan<SessionShape> | { readonly toml: string }) ? options.session as SessionPlan<S> : undefined;
    return new Engine(boundary, sources, plan, (options.limits?.consoleMeterBlocks ?? 0n) !== 0n);
  } catch (error) {
    closeSources(sources);
    boundary?.dispose();
    throw error;
  }
}
