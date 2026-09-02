import { createHash } from "node:crypto";
import { open, readFile, readdir } from "node:fs/promises";
import { constants } from "node:fs";
import type { BigIntStats, Dirent } from "node:fs";
import { basename, resolve, sep } from "node:path";

import { BUNDLED_ENGINE_ASSETS, BUNDLED_ENGINE_FILES } from "../assets.ts";
import { session } from "../core/session.ts";
import type { SessionBuilder } from "../core/session.ts";
import type { SessionSampleRateHz } from "../core/types.ts";
import { readBundledPackageManifest } from "../headless/assets.ts";

const MAXIMUM_AUTHORING_BYTES = 4 * 1024 * 1024;
const MAXIMUM_DECODER_U32 = 0xffff_ffff;
const STABLE_ID_MAXIMUM_BYTES = 127;
const DECODER_LOADER_SHA256 = "0151436982c986b5747933a718d8378cf23a1fb75a4b70ddccbbb6a734caca11";
const LAUNCH_RATES: ReadonlySet<number> = new Set([44_100, 48_000, 88_200, 96_000]);

export class StemsImportError extends Error {
  readonly code: string;
  readonly internal: boolean;
  readonly extra: Readonly<Record<string, unknown>>;

  constructor(
    code: string,
    message: string,
    options: { readonly internal?: boolean; readonly extra?: Readonly<Record<string, unknown>> } = {},
  ) {
    super(message);
    this.name = "StemsImportError";
    this.code = code;
    this.internal = options.internal ?? false;
    this.extra = options.extra ?? {};
  }
}

interface FlacStreamInfo {
  readonly sampleRateHz: number;
  readonly channels: number;
  readonly bitDepth: number;
  readonly frames: bigint;
}

interface PinnedFlacDecoder {
  decodeBlocks(
    bytes: Uint8Array,
    options: { readonly maximumCanonicalBytes: number },
  ): AsyncGenerator<{ readonly stream: FlacStreamInfo; readonly pcm: Uint8Array }, void, void>;
}

interface DecoderModule {
  readonly MISO_ENGINE_FLAC_DECODER_SHA256: string;
  instantiatePinnedFlacDecoder(bytes: Uint8Array): Promise<PinnedFlacDecoder>;
}

interface DiscoveredStem {
  readonly nameBytes: Buffer;
  readonly name: string;
  readonly path: Buffer;
  readonly digest: string;
  readonly sourceId: string;
  readonly trackId: string;
  readonly routeId: string;
}

export interface StemMapping {
  readonly filename: string;
  readonly sourceId: string;
  readonly trackId: string;
  readonly content: string;
  readonly channels: 1 | 2;
  readonly bitDepth: 16 | 24;
  readonly frames: number;
}

export interface StemsBuild {
  readonly builder: SessionBuilder;
  readonly directory: string;
  readonly sessionId: string;
  readonly sampleRateHz: SessionSampleRateHz;
  readonly quantumFrames: number;
  readonly mappings: readonly StemMapping[];
}

function compareBytes(left: Uint8Array, right: Uint8Array): number {
  return Buffer.compare(left, right);
}

function displayName(bytes: Uint8Array): string {
  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    throw new StemsImportError(
      "stems.filename.utf8",
      `stem filename bytes are not valid UTF-8: ${Buffer.from(bytes).toString("hex")}`,
    );
  }
}

function childPath(directory: string, name: Buffer): Buffer {
  const prefix = directory.endsWith(sep) ? directory : `${directory}${sep}`;
  return Buffer.concat([Buffer.from(prefix), name]);
}

function hasFlacSuffix(name: Uint8Array): boolean {
  if (name.byteLength < 5) return false;
  return Buffer.from(name.subarray(name.byteLength - 5)).toString("ascii").toLowerCase() === ".flac";
}

function normalizedBase(name: Uint8Array, stripFlac: boolean): string {
  const end = stripFlac ? name.byteLength - 5 : name.byteLength;
  let value = "";
  let replacement = false;
  for (const byte of name.subarray(0, end)) {
    const lower = byte >= 0x41 && byte <= 0x5a ? byte + 0x20 : byte;
    const legal = (lower >= 0x61 && lower <= 0x7a)
      || (lower >= 0x30 && lower <= 0x39)
      || lower === 0x5f || lower === 0x2e || lower === 0x2d;
    if (legal) {
      value += String.fromCharCode(lower);
      replacement = false;
    } else if (!replacement) {
      value += "-";
      replacement = true;
    }
  }
  if (value.length === 0) value = "stem";
  if (!/^[a-z]/.test(value)) value = `stem-${value}`;
  return value;
}

function fitId(base: string, digest: string, suffix: boolean): string {
  if (!suffix && Buffer.byteLength(base) <= STABLE_ID_MAXIMUM_BYTES) return base;
  const tail = `-${digest.slice(0, 16)}`;
  const maximumBase = STABLE_ID_MAXIMUM_BYTES - tail.length;
  return `${base.slice(0, maximumBase)}${tail}`;
}

function deriveIds(entries: readonly { readonly nameBytes: Buffer }[]): readonly DiscoveredStem[] {
  const bases = entries.map(({ nameBytes }) => normalizedBase(nameBytes, true));
  const counts = new Map<string, number>();
  for (const base of bases) counts.set(base, (counts.get(base) ?? 0) + 1);
  const discovered = entries.map(({ nameBytes }, index) => {
    const digest = createHash("sha256").update(nameBytes).digest("hex");
    const base = bases[index] as string;
    const sourceId = fitId(base, digest, (counts.get(base) ?? 0) > 1);
    const trackId = fitId(base, digest, (counts.get(base) ?? 0) > 1 || base === "main");
    const routeId = fitId(`route-${trackId}`, digest, Buffer.byteLength(`route-${trackId}`) > 127);
    return {
      nameBytes,
      name: displayName(nameBytes),
      path: Buffer.alloc(0),
      digest,
      sourceId,
      trackId,
      routeId,
    };
  });
  for (const field of ["sourceId", "trackId", "routeId"] as const) {
    const values = new Set<string>();
    for (const entry of discovered) {
      if (values.has(entry[field])) {
        throw new StemsImportError("stems.id.collision", `could not derive unique ${field} values`);
      }
      values.add(entry[field]);
    }
  }
  return discovered;
}

async function discover(directoryArgument: string): Promise<readonly DiscoveredStem[]> {
  const directory = resolve(directoryArgument);
  let entries: Dirent<Buffer>[];
  try {
    entries = await readdir(directory, { encoding: "buffer", withFileTypes: true });
  } catch (error) {
    throw new StemsImportError("stems.read", `could not read stems directory '${directoryArgument}': ${String(error)}`);
  }
  entries.sort((left, right) => compareBytes(left.name, right.name));
  const groups = entries.filter((entry) => entry.isDirectory()).map((entry) => displayName(entry.name));
  if (groups.length > 0) {
    throw new StemsImportError(
      "stems.collection",
      "stems directory contains child directories; build each leaf as its own session",
      { extra: { groups } },
    );
  }
  if (entries.length === 0) {
    throw new StemsImportError("stems.empty", "stems directory contains no files");
  }
  for (const entry of entries) {
    const name = displayName(entry.name);
    if (entry.isSymbolicLink()) {
      throw new StemsImportError("stems.symlink", `stem entry '${name}' is a symbolic link`);
    }
    if (!entry.isFile()) {
      throw new StemsImportError("stems.entry", `stem entry '${name}' is not a regular file`);
    }
    if (!hasFlacSuffix(entry.name)) {
      throw new StemsImportError("stems.extension", `stem entry '${name}' is not a FLAC file`);
    }
  }
  const ids = deriveIds(entries.map((entry) => ({ nameBytes: entry.name })));
  let authoringBytes = Buffer.byteLength(JSON.stringify({
    schemaVersion: 1,
    session: { id: "s", sampleRateHz: 96_000, revision: 0, quantumFrames: MAXIMUM_DECODER_U32 },
    outputs: ["main"],
  }));
  const result = ids.map((entry) => {
    // Charge the complete request-equivalent record, including the fixed shape that is not present
    // in the filename. This remains a byte budget rather than a compiled track-count maximum.
    authoringBytes += Buffer.byteLength(JSON.stringify({
      filename: entry.name,
      source: {
        id: entry.sourceId,
        spec: {
          channels: 2,
          bitDepth: 24,
          frames: Number.MAX_SAFE_INTEGER,
          content: `sha256:${"0".repeat(64)}`,
        },
      },
      track: { id: entry.trackId, spec: { source: entry.sourceId } },
      route: {
        id: entry.routeId,
        source: { kind: "track", trackId: entry.trackId, tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "main" },
        gainDb: 0,
      },
    }));
    return { ...entry, path: childPath(directory, entry.nameBytes) };
  });
  if (authoringBytes > MAXIMUM_AUTHORING_BYTES) {
    throw new StemsImportError(
      "stems.metadata.too_large",
      `stem authoring metadata exceeds the ${MAXIMUM_AUTHORING_BYTES}-byte limit`,
    );
  }
  return result;
}

function sameFile(left: BigIntStats, right: BigIntStats): boolean {
  return left.dev === right.dev && left.ino === right.ino && left.size === right.size
    && left.mtimeNs === right.mtimeNs && left.ctimeNs === right.ctimeNs;
}

async function readStem(entry: DiscoveredStem): Promise<Uint8Array> {
  let handle;
  try {
    handle = await open(entry.path, constants.O_RDONLY | constants.O_NOFOLLOW);
    const before = await handle.stat({ bigint: true });
    if (!before.isFile()) throw new StemsImportError("stems.entry", `stem '${entry.name}' is not a regular file`);
    if (before.size <= 0n || before.size > BigInt(MAXIMUM_DECODER_U32)) {
      throw new StemsImportError("flac.encoded_bytes.limit", `stem '${entry.name}' exceeds the decoder input limit`);
    }
    const bytes = await handle.readFile();
    const after = await handle.stat({ bigint: true });
    if (!sameFile(before, after) || BigInt(bytes.byteLength) !== before.size) {
      throw new StemsImportError("stems.changed", `stem '${entry.name}' changed while it was read`);
    }
    return bytes;
  } catch (error) {
    if (error instanceof StemsImportError) throw error;
    throw new StemsImportError("stems.read", `could not read stem '${entry.name}': ${String(error)}`);
  } finally {
    await handle?.close().catch(() => undefined);
  }
}

async function loadDecoder(): Promise<PinnedFlacDecoder> {
  try {
    const manifest = await readBundledPackageManifest();
    const loader = await readFile(BUNDLED_ENGINE_ASSETS.flacDecoderModule);
    const loaderDigest = createHash("sha256").update(loader).digest("hex");
    const loaderRecord = manifest.artifacts[BUNDLED_ENGINE_FILES.flacDecoderModule];
    if (loaderDigest !== DECODER_LOADER_SHA256 || loaderRecord?.bytes !== loader.byteLength
      || loaderRecord.sha256 !== loaderDigest) {
      throw new StemsImportError("internal.packaged_decoder", "packaged FLAC decoder loader digest differs", { internal: true });
    }
    const source = `data:text/javascript;base64,${loader.toString("base64")}`;
    const module = await import(source) as DecoderModule;
    const pinText = await readFile(BUNDLED_ENGINE_ASSETS.flacDecoderDigest, "utf8");
    const pin = pinText.trim();
    const pinRecord = manifest.artifacts[BUNDLED_ENGINE_FILES.flacDecoderDigest];
    if (!/^[0-9a-f]{64}$/.test(pin) || module.MISO_ENGINE_FLAC_DECODER_SHA256 !== pin
      || pinRecord?.bytes !== Buffer.byteLength(pinText)
      || pinRecord.sha256 !== createHash("sha256").update(pinText).digest("hex")) {
      throw new StemsImportError("internal.packaged_decoder", "packaged FLAC decoder digest pin differs", { internal: true });
    }
    const wasm = await readFile(BUNDLED_ENGINE_ASSETS.flacDecoderWasm);
    const wasmRecord = manifest.artifacts[BUNDLED_ENGINE_FILES.flacDecoderWasm];
    if (wasmRecord?.bytes !== wasm.byteLength || wasmRecord.sha256 !== pin) {
      throw new StemsImportError("internal.packaged_decoder", "packaged FLAC decoder byte length differs", { internal: true });
    }
    return await module.instantiatePinnedFlacDecoder(wasm);
  } catch (error) {
    if (error instanceof StemsImportError) throw error;
    throw new StemsImportError(
      "internal.packaged_decoder",
      `packaged FLAC decoder could not be loaded: ${error instanceof Error ? error.message : String(error)}`,
      { internal: true },
    );
  }
}

function safeFrames(frames: bigint, filename: string): number {
  if (frames <= 0n || frames > BigInt(Number.MAX_SAFE_INTEGER)) {
    throw new StemsImportError("flac.frames.unsupported", `stem '${filename}' has an unsupported frame count`);
  }
  return Number(frames);
}

export function normalizeSessionId(directory: string): string {
  const bytes = Buffer.from(basename(resolve(directory)));
  const digest = createHash("sha256").update(bytes).digest("hex");
  return fitId(normalizedBase(bytes, false), digest, normalizedBase(bytes, false).length > 127);
}

export async function buildFromStems(options: {
  readonly directory: string;
  readonly sessionId: string;
  readonly quantumFrames: number;
}): Promise<StemsBuild> {
  const entries = await discover(options.directory);
  const decoder = await loadDecoder();
  const mappings: StemMapping[] = [];
  let sampleRateHz: SessionSampleRateHz | undefined;
  for (const entry of entries) {
    const encoded = await readStem(entry);
    const hash = createHash("sha256");
    let stream: FlacStreamInfo | undefined;
    let decodedBytes = 0;
    try {
      for await (const block of decoder.decodeBlocks(encoded, { maximumCanonicalBytes: MAXIMUM_DECODER_U32 })) {
        stream ??= block.stream;
        if (stream.sampleRateHz !== block.stream.sampleRateHz || stream.channels !== block.stream.channels
          || stream.bitDepth !== block.stream.bitDepth || stream.frames !== block.stream.frames) {
          throw new StemsImportError("flac.shape.changed", `stem '${entry.name}' changed shape between decoder blocks`);
        }
        decodedBytes += block.pcm.byteLength;
        hash.update(block.pcm);
      }
    } catch (error) {
      if (error instanceof StemsImportError) throw error;
      const code = typeof error === "object" && error !== null && "code" in error
        ? String((error as { readonly code: unknown }).code)
        : "miso.flac.decoder.refused";
      throw new StemsImportError("flac.refused", `stem '${entry.name}' was refused: ${code}`);
    }
    if (stream === undefined) throw new StemsImportError("flac.empty", `stem '${entry.name}' decoded no PCM blocks`);
    if (!LAUNCH_RATES.has(stream.sampleRateHz)) {
      throw new StemsImportError("flac.sample_rate.unsupported", `stem '${entry.name}' uses unsupported ${stream.sampleRateHz} Hz`);
    }
    if (stream.channels !== 1 && stream.channels !== 2) {
      throw new StemsImportError("flac.channels.unsupported", `stem '${entry.name}' has ${stream.channels} channels`);
    }
    if (stream.bitDepth !== 16 && stream.bitDepth !== 24) {
      throw new StemsImportError("flac.bit_depth.unsupported", `stem '${entry.name}' has ${stream.bitDepth}-bit PCM`);
    }
    const expectedBytes = stream.frames * BigInt(stream.channels * (stream.bitDepth / 8));
    if (expectedBytes > BigInt(MAXIMUM_DECODER_U32) || BigInt(decodedBytes) !== expectedBytes) {
      throw new StemsImportError("flac.byte_length.mismatch", `stem '${entry.name}' decoded byte length disagrees with its declared shape`);
    }
    if (sampleRateHz !== undefined && sampleRateHz !== stream.sampleRateHz) {
      throw new StemsImportError("stems.sample_rate.mixed", `stem '${entry.name}' does not match the directory sample rate`);
    }
    sampleRateHz = stream.sampleRateHz as SessionSampleRateHz;
    mappings.push(Object.freeze({
      filename: entry.name,
      sourceId: entry.sourceId,
      trackId: entry.trackId,
      content: `sha256:${hash.digest("hex")}`,
      channels: stream.channels,
      bitDepth: stream.bitDepth,
      frames: safeFrames(stream.frames, entry.name),
    }));
  }
  if (sampleRateHz === undefined) throw new StemsImportError("stems.empty", "stems directory contains no files");
  let builder = session({
    id: options.sessionId,
    sampleRateHz,
    revision: 0,
    quantumFrames: options.quantumFrames,
  });
  for (const mapping of mappings) {
    builder = builder.source(mapping.sourceId, {
      channels: mapping.channels,
      bitDepth: mapping.bitDepth,
      frames: mapping.frames,
      content: mapping.content,
    });
  }
  builder = builder.output("main");
  for (const [index, mapping] of mappings.entries()) {
    const entry = entries[index] as DiscoveredStem;
    builder = builder
      .track(mapping.trackId, { source: mapping.sourceId })
      .route({
        id: entry.routeId,
        source: { kind: "track", trackId: mapping.trackId, tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "main" },
        gainDb: 0,
      });
  }
  return Object.freeze({
    builder,
    directory: resolve(options.directory),
    sessionId: options.sessionId,
    sampleRateHz,
    quantumFrames: options.quantumFrames,
    mappings: Object.freeze(mappings),
  });
}
