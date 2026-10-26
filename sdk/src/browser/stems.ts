/**
 * Local stems into the engine (issue #2102): read a WAV header, hash the canonical PCM to the
 * engine's `blake3:` identity, and pump it into MSB1 rings.
 *
 * The pump and the hash are the engine's own `hosts/host-web/web/stem-store` modules, staged
 * verbatim into the package; the WAV header reader is the SDK's. A WAV is not canonical PCM, but
 * for a 16- or 24-bit integer WAV its `data` chunk is exactly the canonical bytes
 * (docs/STEM_IDENTITY_V1.md), so `blob.slice(dataOffset, dataOffset + canonicalBytes)` is both
 * the hash preimage and what the pump reads.
 */
import type { SessionSampleRateHz } from "../core/types.ts";

export { CanonicalPcmPump, CanonicalPcmPumpError, deinterleaveCanonicalPcm } from "./stems/pcm-pump.js";
export type {
  CanonicalPcmLease,
  CanonicalPcmPumpOptions,
  CanonicalPcmPumpSource,
  CanonicalPcmRing,
  CanonicalStemIdentity,
} from "./stems/pcm-pump.js";
export { IncrementalBlake3, blake3Stream } from "./stems/incremental-blake3.js";

/** The canonical PCM a WAV's `data` chunk carries, and where it starts. */
export interface WavInfo {
  readonly sampleRateHz: SessionSampleRateHz;
  readonly channels: 1 | 2;
  readonly bitDepth: 16 | 24;
  /** Whole frames in the `data` chunk, clamped to the file's end. */
  readonly frames: number;
  /** The byte offset of the first sample in the file. */
  readonly dataOffset: number;
  /** `frames * channels * bitDepth / 8`: the canonical PCM length from `dataOffset`. */
  readonly canonicalBytes: number;
}

/** Why `readWavHeader` refused a file. */
export type WavFormatErrorCode =
  | "wav.not_riff"
  | "wav.ds64_missing"
  | "wav.ds64_short"
  | "wav.fmt_short"
  | "wav.fmt_duplicate"
  | "wav.float_unsupported"
  | "wav.format_unsupported"
  | "wav.channels"
  | "wav.bit_depth"
  | "wav.block_align"
  | "wav.sample_rate"
  | "wav.data_before_fmt"
  | "wav.no_frames"
  | "wav.no_data";

/** A typed WAV header refusal; the message names the fix. */
export class WavFormatError extends Error {
  readonly code: WavFormatErrorCode;
  constructor(code: WavFormatErrorCode, message: string) {
    super(message);
    this.name = "WavFormatError";
    this.code = code;
  }
}

const WAVE_FORMAT_PCM = 0x0001;
const WAVE_FORMAT_IEEE_FLOAT = 0x0003;
const WAVE_FORMAT_EXTENSIBLE = 0xfffe;
// `WAVEFORMATEXTENSIBLE` is the 16-byte `WAVEFORMAT` plus cbSize, valid bits, channel mask and the
// 16-byte sub-format GUID, whose first two bytes are the format tag.
const FMT_BYTES = 16;
const EXTENSIBLE_FMT_BYTES = 40;
const SUBFORMAT_TAG_AT = 24;
// The KSDATAFORMAT_SUBTYPE GUID tail every WAVE_FORMAT_* sub-format shares:
// xxxx0000-0000-0010-8000-00AA00389B71.
const SUBFORMAT_GUID_TAIL = [0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71];
const LAUNCH_RATES: readonly number[] = [44_100, 48_000, 88_200, 96_000];

// RF64 and BW64 (EBU Tech 3306, ITU-R BS.2088) carry the 64-bit sizes in a `ds64` chunk, which
// comes before `data`: RIFF size, data size and sample count (three u64), then a table.
const DS64_BYTES = 28;
const DS64_DATA_SIZE_AT = 8;
const SIZE_IN_DS64 = 0xffff_ffff;

/**
 * Read a 16- or 24-bit integer PCM WAV's header without reading its samples. It reads RIFF, and
 * RF64/BW64 with the 64-bit data size from their `ds64` chunk. It walks the chunk headers with
 * one 8-byte read each (and one read of the `fmt ` and `ds64` bodies), steps over each odd-sized
 * chunk's pad byte, accepts `WAVE_FORMAT_EXTENSIBLE`, and reads a `data` size larger than the
 * file (an unpatched or `0xFFFFFFFF` RIFF size) to the file's end. It refuses float, more than
 * two channels, and a rate other than 44.1, 48, 88.2 or 96 kHz with a `WavFormatError`.
 */
export async function readWavHeader(file: Blob): Promise<WavInfo> {
  const riff = await readView(file, 0, 12);
  const magic = riff.byteLength < 12 ? "" : fourcc(riff, 0);
  const wide = magic === "RF64" || magic === "BW64";
  if ((magic !== "RIFF" && !wide) || fourcc(riff, 8) !== "WAVE") {
    throw new WavFormatError("wav.not_riff", "not a RIFF, RF64 or BW64 WAVE file");
  }
  let ds64DataSize: number | null = null;
  let format: Omit<WavInfo, "frames" | "dataOffset" | "canonicalBytes"> | null = null;
  let offset = 12;
  while (offset + 8 <= file.size) {
    const header = await readView(file, offset, 8);
    const id = fourcc(header, 0);
    const size = header.getUint32(4, true);
    const body = offset + 8;
    if (id === "fmt ") {
      if (format !== null) throw new WavFormatError("wav.fmt_duplicate", "the file has two fmt chunks");
      format = await readFormat(file, body, size);
    } else if (id === "ds64" && wide) {
      if (size < DS64_BYTES) throw new WavFormatError("wav.ds64_short", "the ds64 chunk is too short");
      const ds64 = await readView(file, body, DS64_BYTES);
      if (ds64.byteLength < DS64_BYTES) throw new WavFormatError("wav.ds64_short", "the ds64 chunk is truncated");
      // A size beyond 2^53 is far beyond any file, and the clamp to the file's end below wins.
      ds64DataSize = Number(ds64.getBigUint64(DS64_DATA_SIZE_AT, true));
    } else if (id === "data") {
      if (format === null) throw new WavFormatError("wav.data_before_fmt", "the data chunk comes before the fmt chunk");
      const frameBytes = format.channels * (format.bitDepth / 8);
      let declared = size;
      if (wide && size === SIZE_IN_DS64) {
        if (ds64DataSize === null) {
          throw new WavFormatError("wav.ds64_missing", `a ${magic} data chunk needs a ds64 chunk before it`);
        }
        declared = ds64DataSize;
      }
      const available = Math.min(declared, file.size - body);
      const frames = Math.floor(available / frameBytes);
      if (frames <= 0) throw new WavFormatError("wav.no_frames", "the data chunk has no audio frames");
      return { ...format, frames, dataOffset: body, canonicalBytes: frames * frameBytes };
    }
    // RIFF chunks are word aligned: an odd-sized chunk has one pad byte after it.
    offset = body + size + (size & 1);
  }
  throw new WavFormatError("wav.no_data", "the file has no data chunk");
}

async function readFormat(
  file: Blob,
  body: number,
  size: number,
): Promise<Omit<WavInfo, "frames" | "dataOffset" | "canonicalBytes">> {
  if (size < FMT_BYTES) throw new WavFormatError("wav.fmt_short", "the fmt chunk is too short");
  const fmt = await readView(file, body, Math.min(size, EXTENSIBLE_FMT_BYTES));
  if (fmt.byteLength < FMT_BYTES) throw new WavFormatError("wav.fmt_short", "the fmt chunk is truncated");
  let tag = fmt.getUint16(0, true);
  const channels = fmt.getUint16(2, true);
  const sampleRateHz = fmt.getUint32(4, true);
  const blockAlign = fmt.getUint16(12, true);
  const bits = fmt.getUint16(14, true);
  if (tag === WAVE_FORMAT_EXTENSIBLE) {
    if (fmt.byteLength < EXTENSIBLE_FMT_BYTES) {
      throw new WavFormatError("wav.fmt_short", "the extensible fmt chunk is too short");
    }
    tag = fmt.getUint16(SUBFORMAT_TAG_AT, true);
    if (SUBFORMAT_GUID_TAIL.some((byte, index) => fmt.getUint8(SUBFORMAT_TAG_AT + 2 + index) !== byte)) {
      throw new WavFormatError("wav.format_unsupported", "the extensible sub-format is not a WAVE format; export 16- or 24-bit PCM");
    }
  }
  if (tag === WAVE_FORMAT_IEEE_FLOAT) {
    throw new WavFormatError("wav.float_unsupported", `${bits}-bit float WAV is not supported; export 16- or 24-bit PCM`);
  }
  if (tag !== WAVE_FORMAT_PCM) {
    throw new WavFormatError("wav.format_unsupported", `WAV format ${tag} is not integer PCM; export 16- or 24-bit PCM`);
  }
  if (channels !== 1 && channels !== 2) {
    throw new WavFormatError("wav.channels", `${channels} channels: a stem must be mono or stereo`);
  }
  if (bits !== 16 && bits !== 24) {
    throw new WavFormatError("wav.bit_depth", `${bits}-bit PCM is not supported; export 16- or 24-bit`);
  }
  if (blockAlign !== channels * (bits / 8)) {
    throw new WavFormatError("wav.block_align", `block align ${blockAlign} is not ${channels} x ${bits / 8} bytes`);
  }
  if (!LAUNCH_RATES.includes(sampleRateHz)) {
    throw new WavFormatError("wav.sample_rate", `${sampleRateHz} Hz: use 44.1, 48, 88.2 or 96 kHz`);
  }
  return { sampleRateHz: sampleRateHz as SessionSampleRateHz, channels, bitDepth: bits };
}

async function readView(file: Blob, offset: number, length: number): Promise<DataView> {
  return new DataView(await file.slice(offset, offset + length).arrayBuffer());
}

function fourcc(view: DataView, at: number): string {
  return String.fromCharCode(view.getUint8(at), view.getUint8(at + 1), view.getUint8(at + 2), view.getUint8(at + 3));
}
