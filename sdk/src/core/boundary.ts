import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BufferKindName, ExportName } from "../generated/abi.ts";
import {
  StructView,
  constantValue,
  defaultSourceRingFrames,
  structBytes,
  writeBootOptions,
} from "./abi.ts";
import type { BootOptions } from "./abi.ts";
import { MisoEngineAsset } from "./asset.ts";
import { MisoEngineError, MisoUsageError, parseDiagnostics, resultName } from "./errors.ts";

/**
 * The wasm boundary: one instance, one staging sequence, one live session.
 *
 * # The guess machine is gone
 *
 * The pre-boot-v1 SDK reached this point through nine operations over a bifurcated plan/raw path.
 * A raw TOML document was funnelled through `sessionHeader`, a regex over the text that could not
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

/** The shape the boot itself reported. Nothing here was read from the document's text. */
export interface SessionShape {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  /** The ring the engine derived, from the published rule applied to the reported shape. */
  readonly sourceRingFrames: number;
  readonly backend: "scalar" | "simd128";
  readonly sources: readonly SourceShape[];
  readonly tracks: readonly string[];
}

export interface SourceShape {
  readonly id: string;
  readonly channels: number;
  readonly frames: bigint;
}

/** A staged-and-booted engine instance. */
export class WasmBoundary {
  readonly #exports: ExportTable;
  #handle: number;
  #optionBytes: Uint8Array;

  private constructor(exports: ExportTable, handle: number, optionBytes: Uint8Array) {
    this.#exports = exports;
    this.#handle = handle;
    this.#optionBytes = optionBytes;
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
    const exports = exportsOf(await asset.instantiate());
    const staged = stage(exports, document, options);
    return new WasmBoundary(exports, staged.handle, staged.optionBytes);
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
    this.dispose();
    const staged = stage(this.#exports, document, options);
    this.#handle = staged.handle;
    this.#optionBytes = staged.optionBytes;
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
      throw new MisoUsageError("this engine was disposed; boot a new one");
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
    const trackCount = Number(this.#exports.miso_engine_web_v1_console_track_count(handle));
    const tracks: string[] = [];
    for (let index = 0; index < trackCount; index += 1) {
      const idBytes = Number(this.#exports.miso_engine_web_v1_console_track_id(handle, index));
      tracks.push(this.#readIdBuffer(idBytes));
    }
    return Object.freeze({
      sampleRateHz,
      quantumFrames,
      // The ring is an input word with no readback export, so it is *derived* from the rule the
      // generated document publishes, applied to the shape the boot just reported.
      sourceRingFrames: defaultSourceRingFrames(sampleRateHz, quantumFrames),
      backend: backendValue === constantValue("backends", "simd128") ? "simd128" : "scalar",
      sources: Object.freeze(sources),
      tracks: Object.freeze(tracks),
    });
  }

  /** The engine's state word, named through the generated vocabulary. */
  state(): "ready" | "failed" | "disposed" {
    const value = this.#status().u32("state");
    for (const row of ABI_LAYOUT.constants.states) {
      if (row.value === value) return row.name;
    }
    throw new MisoUsageError(`the engine reported an unknown state word ${value}`);
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
  }): { readonly ok: boolean; readonly result: number; readonly code: string } {
    const handle = this.#live();
    const id = new TextEncoder().encode(request.sourceId);
    const idBuffer = this.#buffer("sourceId");
    if (id.byteLength > idBuffer.capacity) {
      throw new MisoUsageError(
        `source id ${request.sourceId} is ${id.byteLength} bytes; staging holds ${idBuffer.capacity}`,
      );
    }
    new Uint8Array(this.#exports.memory.buffer, idBuffer.pointer, id.byteLength).set(id);

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
      id.byteLength,
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
    return Object.freeze({
      left: contiguous.slice(0, actualFrames),
      right: contiguous.slice(quantum, quantum + actualFrames),
    });
  }

  /** Stage `count` already-encoded 48-byte command records and submit them as one transaction. */
  submitCommands(records: Uint8Array, count: number): CommandReport {
    const handle = this.#live();
    const recordBytes = ABI_LAYOUT.commandRecord.bytes;
    const staging = this.#buffer("command");
    if (staging.pointer === 0 || staging.capacity === 0) {
      throw new MisoUsageError(
        "this engine booted with no console attached; set console.commandQueueRecords",
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
      reasonName: reasonNameOf(reason),
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
    const result = Number(this.#exports.miso_engine_web_v1_dispose(this.#handle));
    this.#handle = 0;
    if (result !== constantValue("resultCodes", "ok")) {
      throw new MisoEngineError("the engine refused to dispose this handle", {
        phase: "lifecycle",
        code: resultName(result, "call"),
        result,
      });
    }
  }
}

export interface CommandReport {
  /** Whole-batch admission. A refusal admits nothing and names the first offending record. */
  readonly ok: boolean;
  readonly result: number;
  readonly code: string;
  readonly reason: number;
  readonly reasonName: string;
  readonly rejectedIndex: number;
  readonly admitted: number;
  readonly appliedAtSample: bigint;
}

function reasonNameOf(value: number): string {
  return ABI_LAYOUT.constants.commandReasons.find((row) => row.value === value)?.name ?? "unknown";
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

  // 3. `document_ptr(len)` -- the admission gate. A document over the engine's maximum is refused
  //    *here*, before a byte is copied and before any staging is allocated, which is what makes
  //    oversize admission cost no memory (issue #243 eval 3).
  const documentPointer = Number(exports.miso_engine_web_v1_document_ptr(document.byteLength));
  if (documentPointer === 0) {
    throw bootFailure(exports, 0, "the engine refused to stage the document");
  }
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);

  // 4. `boot(len)`.
  const handle = Number(exports.miso_engine_web_v1_boot(document.byteLength));
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
