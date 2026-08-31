// Provenance-pinned FLAC delivery adapter. This module is Worker-safe and never touches audio APIs.

export const MISO_ENGINE_FLAC_DECODER_SHA256 =
  "7c6ef5c612e5f68ccf485c38456ebbd70b7440a02d1b42ffdf6f6359b4691eec";

const ABI_VERSION = 0x0001_0000;
const RESULT_OK = 0;
const RESULT_END = 1;
const RESULT_CODES = new Map([
  [2, "miso.flac.decoder.invalid_argument"],
  [3, "miso.flac.decoder.decode_refused"],
  [4, "miso.flac.decoder.bit_depth_unsupported"],
  [5, "miso.flac.decoder.shape_mismatch"],
  [6, "miso.flac.decoder.resource_limit"],
  [7, "miso.flac.decoder.output_write"],
  [255, "miso.flac.decoder.internal"],
]);
const REQUIRED_EXPORTS = Object.freeze([
  "memory",
  "miso_flac_decoder_v1_abi_version",
  "miso_flac_decoder_v1_begin",
  "miso_flac_decoder_v1_bit_depth",
  "miso_flac_decoder_v1_channels",
  "miso_flac_decoder_v1_create",
  "miso_flac_decoder_v1_decode_next",
  "miso_flac_decoder_v1_dispose",
  "miso_flac_decoder_v1_frames_high",
  "miso_flac_decoder_v1_frames_low",
  "miso_flac_decoder_v1_input_pointer",
  "miso_flac_decoder_v1_pcm_length",
  "miso_flac_decoder_v1_pcm_pointer",
  "miso_flac_decoder_v1_sample_rate_hz",
]);

export class MisoFlacDecoderError extends Error {
  constructor(code, result = null) {
    super(code);
    this.name = "MisoFlacDecoderError";
    this.code = code;
    this.result = result;
  }
}

function bytesToHex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function asBytes(value) {
  if (value instanceof Uint8Array) return value;
  if (value instanceof ArrayBuffer) return new Uint8Array(value);
  throw new MisoFlacDecoderError("miso.flac.decoder.invalid_bytes");
}

function exactU32(value, code) {
  if (!Number.isInteger(value) || value <= 0 || value > 0xffff_ffff) {
    throw new MisoFlacDecoderError(code);
  }
  return value;
}

function refuse(result) {
  throw new MisoFlacDecoderError(
    RESULT_CODES.get(result) ?? "miso.flac.decoder.unknown_result",
    result,
  );
}

class PinnedFlacDecoder {
  constructor(instance) {
    this.exports = instance.exports;
  }

  async *decodeBlocks(flacBytes, { maximumCanonicalBytes }) {
    const flac = asBytes(flacBytes);
    exactU32(flac.byteLength, "miso.flac.decoder.input_size_invalid");
    exactU32(maximumCanonicalBytes, "miso.flac.decoder.maximum_size_invalid");
    const handle = this.exports.miso_flac_decoder_v1_create(
      flac.byteLength,
      maximumCanonicalBytes,
    );
    if (handle === 0) refuse(2);
    try {
      const pointer = this.exports.miso_flac_decoder_v1_input_pointer(handle);
      if (pointer === 0 || pointer + flac.byteLength > this.exports.memory.buffer.byteLength) {
        refuse(255);
      }
      new Uint8Array(this.exports.memory.buffer, pointer, flac.byteLength).set(flac);
      const begun = this.exports.miso_flac_decoder_v1_begin(handle);
      if (begun !== RESULT_OK) refuse(begun);
      const stream = Object.freeze({
        sampleRateHz: this.exports.miso_flac_decoder_v1_sample_rate_hz(handle),
        channels: this.exports.miso_flac_decoder_v1_channels(handle),
        bitDepth: this.exports.miso_flac_decoder_v1_bit_depth(handle),
        frames: BigInt(this.exports.miso_flac_decoder_v1_frames_low(handle))
          | (BigInt(this.exports.miso_flac_decoder_v1_frames_high(handle)) << 32n),
      });
      for (;;) {
        const result = this.exports.miso_flac_decoder_v1_decode_next(handle);
        if (result === RESULT_END) break;
        if (result !== RESULT_OK) refuse(result);
        const pcmPointer = this.exports.miso_flac_decoder_v1_pcm_pointer(handle);
        const pcmLength = this.exports.miso_flac_decoder_v1_pcm_length(handle);
        if (pcmLength === 0
          || pcmPointer === 0
          || pcmPointer + pcmLength > this.exports.memory.buffer.byteLength) {
          refuse(255);
        }
        const pcm = new Uint8Array(pcmLength);
        pcm.set(new Uint8Array(this.exports.memory.buffer, pcmPointer, pcmLength));
        yield Object.freeze({ stream, pcm });
      }
    } finally {
      const disposed = this.exports.miso_flac_decoder_v1_dispose(handle);
      if (disposed !== RESULT_OK) refuse(disposed);
    }
  }
}

export async function instantiatePinnedFlacDecoder(bytes) {
  const artifact = asBytes(bytes);
  const observed = bytesToHex(new Uint8Array(await crypto.subtle.digest("SHA-256", artifact)));
  if (observed !== MISO_ENGINE_FLAC_DECODER_SHA256) {
    throw new MisoFlacDecoderError("miso.flac.decoder.artifact_mismatch");
  }
  let module;
  try {
    module = await WebAssembly.compile(artifact);
  } catch (_error) {
    throw new MisoFlacDecoderError("miso.flac.decoder.artifact_invalid");
  }
  if (WebAssembly.Module.imports(module).length !== 0) {
    throw new MisoFlacDecoderError("miso.flac.decoder.imports_forbidden");
  }
  const exportNames = WebAssembly.Module.exports(module).map(({ name }) => name).sort();
  if (JSON.stringify(exportNames) !== JSON.stringify(REQUIRED_EXPORTS)) {
    throw new MisoFlacDecoderError("miso.flac.decoder.exports_mismatch");
  }
  const instance = await WebAssembly.instantiate(module);
  if (instance.exports.miso_flac_decoder_v1_abi_version() !== ABI_VERSION) {
    throw new MisoFlacDecoderError("miso.flac.decoder.abi_mismatch");
  }
  return Object.freeze(new PinnedFlacDecoder(instance));
}

export async function loadPinnedFlacDecoder(url) {
  let response;
  try {
    response = await fetch(url);
  } catch (_error) {
    throw new MisoFlacDecoderError("miso.flac.decoder.fetch_refused");
  }
  if (!response.ok) {
    throw new MisoFlacDecoderError("miso.flac.decoder.fetch_refused");
  }
  return instantiatePinnedFlacDecoder(await response.arrayBuffer());
}

export function canonicalPcmBlockToPlanarF32(pcmBytes, channels, bitDepth) {
  const pcm = asBytes(pcmBytes);
  if (!Number.isInteger(channels) || channels <= 0 || (bitDepth !== 16 && bitDepth !== 24)) {
    throw new MisoFlacDecoderError("miso.flac.decoder.pump_shape_invalid");
  }
  const sampleBytes = bitDepth / 8;
  const frameBytes = channels * sampleBytes;
  if (pcm.byteLength === 0 || pcm.byteLength % frameBytes !== 0) {
    throw new MisoFlacDecoderError("miso.flac.decoder.pump_length_mismatch");
  }
  const frames = pcm.byteLength / frameBytes;
  const planes = Array.from({ length: channels }, () => new Float32Array(frames));
  const view = new DataView(pcm.buffer, pcm.byteOffset, pcm.byteLength);
  const denominator = 2 ** (bitDepth - 1);
  let offset = 0;
  for (let frame = 0; frame < frames; frame += 1) {
    for (let channel = 0; channel < channels; channel += 1) {
      let sample;
      if (bitDepth === 16) {
        sample = view.getInt16(offset, true);
      } else {
        const unsigned = view.getUint8(offset)
          | (view.getUint8(offset + 1) << 8)
          | (view.getUint8(offset + 2) << 16);
        sample = (unsigned & 0x80_0000) === 0 ? unsigned : unsigned - 0x1_000000;
      }
      planes[channel][frame] = sample / denominator;
      offset += sampleBytes;
    }
  }
  return planes;
}
