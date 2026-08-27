import { MisoSourceError } from "./errors.js";

type WaveEncoding = "pcm16" | "pcm24" | "float32";

export interface WaveData {
  readonly bytes: Uint8Array;
  readonly sampleRateHz: number;
  readonly channels: number;
  readonly frames: number;
  readonly dataOffset: number;
  readonly blockAlign: number;
  readonly encoding: WaveEncoding;
}

const text = new TextDecoder("ascii");
const PCM_GUID = "0100000000001000800000aa00389b71";
const FLOAT_GUID = "0300000000001000800000aa00389b71";

function tag(bytes: Uint8Array, offset: number): string {
  return text.decode(bytes.subarray(offset, offset + 4));
}

function hex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function sourceError(sourceId: string, message: string, path = "wav"): never {
  throw new MisoSourceError(message, sourceId, path);
}

function safeNumber(value: bigint, sourceId: string, path: string): number {
  if (value < 0n || value > BigInt(Number.MAX_SAFE_INTEGER)) sourceError(sourceId, "WAV size exceeds the JavaScript addressable range", path);
  return Number(value);
}

function encoding(tagValue: number, bits: number, guid: string | undefined, sourceId: string): WaveEncoding {
  const format = tagValue === 0xfffe ? (guid === PCM_GUID ? 1 : guid === FLOAT_GUID ? 3 : 0) : tagValue;
  if (format === 1) {
    if (bits === 16) return "pcm16";
    if (bits === 24) return "pcm24";
  }
  if (format === 3 && bits === 32) return "float32";
  return sourceError(sourceId, "Unsupported WAV scalar format", "wav.fmt");
}

/** Parse bounded RIFF/WAVE or RF64/WAVE metadata without decoding the stem. */
export function parseWave(bytes: Uint8Array, sourceId = "wav"): WaveData {
  if (bytes.byteLength < 12) sourceError(sourceId, "Truncated WAV header");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const container = tag(bytes, 0);
  if ((container !== "RIFF" && container !== "RF64") || tag(bytes, 8) !== "WAVE") sourceError(sourceId, "Expected RIFF/WAVE or RF64/WAVE");
  if (container === "RIFF" && view.getUint32(4, true) + 8 !== bytes.byteLength) sourceError(sourceId, "RIFF size does not match the byte sequence", "wav.riff");
  let rf64DataBytes: bigint | undefined;
  let rf64RiffBytes: bigint | undefined;
  let rf64SampleCount: bigint | undefined;
  let format: { sampleRateHz: number; channels: number; blockAlign: number; encoding: WaveEncoding } | undefined;
  let dataOffset = -1;
  let dataBytes = -1;
  let offset = 12;
  let chunks = 0;
  while (offset + 8 <= bytes.byteLength) {
    chunks += 1;
    if (chunks > 4_096) sourceError(sourceId, "WAV chunk count exceeds 4096", "wav.chunks");
    const id = tag(bytes, offset);
    const size32 = view.getUint32(offset + 4, true);
    const payload = offset + 8;
    let size = size32;
    if (container === "RF64" && id === "data" && size32 === 0xffff_ffff) {
      if (rf64DataBytes === undefined) sourceError(sourceId, "RF64 data precedes ds64", "wav.ds64");
      size = safeNumber(rf64DataBytes, sourceId, "wav.data");
    }
    if (payload + size > bytes.byteLength) sourceError(sourceId, "WAV chunk exceeds the byte sequence", `wav.${id.trim() || "chunk"}`);
    if (id === "ds64") {
      if (container !== "RF64" || size < 28 || rf64DataBytes !== undefined) sourceError(sourceId, "Invalid or duplicate RF64 ds64 chunk", "wav.ds64");
      rf64RiffBytes = view.getBigUint64(payload, true);
      rf64DataBytes = view.getBigUint64(payload + 8, true);
      rf64SampleCount = view.getBigUint64(payload + 16, true);
    } else if (id === "fmt ") {
      if (format) sourceError(sourceId, "Duplicate WAV fmt chunk", "wav.fmt");
      if (size !== 16 && size !== 40) sourceError(sourceId, "Unsupported WAV fmt chunk size", "wav.fmt");
      const tagValue = view.getUint16(payload, true);
      const channels = view.getUint16(payload + 2, true);
      const sampleRateHz = view.getUint32(payload + 4, true);
      const byteRate = view.getUint32(payload + 8, true);
      const blockAlign = view.getUint16(payload + 12, true);
      const bits = view.getUint16(payload + 14, true);
      if (size === 40 && (view.getUint16(payload + 16, true) !== 22 || view.getUint16(payload + 18, true) !== bits)) sourceError(sourceId, "Invalid extensible WAV fmt fields", "wav.fmt");
      const guid = size === 40 ? hex(bytes.subarray(payload + 24, payload + 40)) : undefined;
      const scalar = encoding(tagValue, bits, guid, sourceId);
      const bytesPerSample = scalar === "pcm16" ? 2 : scalar === "pcm24" ? 3 : 4;
      if (channels === 0 || sampleRateHz === 0 || blockAlign !== channels * bytesPerSample || byteRate !== sampleRateHz * blockAlign) {
        sourceError(sourceId, "Inconsistent WAV fmt fields", "wav.fmt");
      }
      format = { sampleRateHz, channels, blockAlign, encoding: scalar };
    } else if (id === "data") {
      if (dataOffset >= 0) sourceError(sourceId, "Duplicate WAV data chunk", "wav.data");
      dataOffset = payload;
      dataBytes = size;
    }
    const next = payload + size + (size & 1);
    if (next > bytes.byteLength) sourceError(sourceId, "WAV chunk padding exceeds the byte sequence", "wav.padding");
    offset = next;
  }
  if (!format || dataOffset < 0 || dataBytes < 0 || dataBytes % format.blockAlign !== 0) sourceError(sourceId, "WAV requires one valid fmt and data chunk");
  if (container === "RF64") {
    if (rf64RiffBytes === undefined || rf64DataBytes === undefined || rf64SampleCount === undefined
        || safeNumber(rf64RiffBytes + 8n, sourceId, "wav.ds64") !== bytes.byteLength
        || safeNumber(rf64DataBytes, sourceId, "wav.ds64") !== dataBytes
        || safeNumber(rf64SampleCount, sourceId, "wav.ds64") !== dataBytes / format.blockAlign) sourceError(sourceId, "RF64 ds64 sizes do not match the container", "wav.ds64");
  }
  return Object.freeze({ bytes, ...format, frames: dataBytes / format.blockAlign, dataOffset });
}

function signed24(view: DataView, offset: number): number {
  const raw = view.getUint8(offset) | (view.getUint8(offset + 1) << 8) | (view.getUint8(offset + 2) << 16);
  return (raw & 0x80_0000) === 0 ? raw : raw | ~0xff_ffff;
}

function finiteNormalF32(view: DataView, offset: number): number {
  const bits = view.getUint32(offset, true);
  const magnitude = bits & 0x7fff_ffff;
  const exponent = bits & 0x7f80_0000;
  return ((exponent !== 0 && exponent !== 0x7f80_0000) || magnitude === 0) ? view.getFloat32(offset, true) : 0;
}

/** Decode one bounded source region into fresh planar f32 arrays. */
export function decodeWave(wave: WaveData, startFrame: number, frames: number, sourceId = "wav"): readonly Float32Array[] {
  if (!Number.isSafeInteger(startFrame) || !Number.isSafeInteger(frames) || startFrame < 0 || frames < 0 || startFrame + frames > wave.frames) {
    sourceError(sourceId, "WAV decode region is out of bounds", "wav.region");
  }
  const output = Array.from({ length: wave.channels }, () => new Float32Array(frames));
  const view = new DataView(wave.bytes.buffer, wave.bytes.byteOffset, wave.bytes.byteLength);
  for (let frame = 0; frame < frames; frame += 1) {
    const base = wave.dataOffset + (startFrame + frame) * wave.blockAlign;
    for (let channel = 0; channel < wave.channels; channel += 1) {
      let value: number;
      switch (wave.encoding) {
        case "pcm16": value = view.getInt16(base + channel * 2, true) * (1 / 32_768); break;
        case "pcm24": value = Math.fround(Math.fround(signed24(view, base + channel * 3)) * Math.fround(1 / 8_388_608)); break;
        case "float32": value = finiteNormalF32(view, base + channel * 4); break;
      }
      output[channel][frame] = value;
    }
  }
  return output;
}

/** Native-runner record bytes: one left plane then one right plane for every logical block. */
export function f32lePlanarBytes(left: Float32Array, right: Float32Array, quantumFrames: number): Uint8Array {
  if (left.length !== right.length || !Number.isSafeInteger(quantumFrames) || quantumFrames <= 0) throw new RangeError("Invalid planar output shape");
  const bytes = new Uint8Array(left.length * 8);
  const view = new DataView(bytes.buffer);
  let offset = 0;
  for (let block = 0; block < left.length; block += quantumFrames) {
    const end = Math.min(left.length, block + quantumFrames);
    for (let index = block; index < end; index += 1) { view.setFloat32(offset, left[index], true); offset += 4; }
    for (let index = block; index < end; index += 1) { view.setFloat32(offset, right[index], true); offset += 4; }
  }
  return bytes;
}

/** Conventional interleaved stereo RIFF/WAVE IEEE-f32 output. */
export function wav32fBytes(left: Float32Array, right: Float32Array, sampleRateHz: number): Uint8Array {
  if (left.length !== right.length) throw new RangeError("WAV channels must have equal frame counts");
  const dataBytes = left.length * 8;
  if (dataBytes > 0xffff_ffff - 36) throw new RangeError("RIFF/WAVE output exceeds the 32-bit container limit");
  const bytes = new Uint8Array(44 + dataBytes);
  const view = new DataView(bytes.buffer);
  const put = (offset: number, value: string) => bytes.set(new TextEncoder().encode(value), offset);
  put(0, "RIFF"); view.setUint32(4, 36 + dataBytes, true); put(8, "WAVE"); put(12, "fmt ");
  view.setUint32(16, 16, true); view.setUint16(20, 3, true); view.setUint16(22, 2, true);
  view.setUint32(24, sampleRateHz, true); view.setUint32(28, sampleRateHz * 8, true);
  view.setUint16(32, 8, true); view.setUint16(34, 32, true); put(36, "data"); view.setUint32(40, dataBytes, true);
  for (let frame = 0; frame < left.length; frame += 1) {
    view.setFloat32(44 + frame * 8, left[frame], true);
    view.setFloat32(48 + frame * 8, right[frame], true);
  }
  return bytes;
}
