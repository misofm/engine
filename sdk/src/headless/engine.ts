import type { BootOptions } from "../core/abi.ts";
import { MisoEngineAsset } from "../core/asset.ts";
import { MAXIMUM_DOCUMENT_BYTES, WasmBoundary } from "../core/boundary.ts";
import type { CommandReport, SessionShape } from "../core/boundary.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";
import type { ErrorPhase, MisoDiagnostic, MisoErrorCode } from "../core/errors.ts";

/**
 * Faster-than-realtime rendering over the browser engine, in Node or Bun.
 *
 * # One verb, two input paths
 *
 * The pre-boot-v1 SDK had a plan path and a raw-TOML path that reached the engine differently, and
 * the raw path is where every one of #207's red probes failed: a regex header sniff that could not
 * read a quoted key, a silent 48 kHz/128 fallback when it failed, and a fabricated ring that was
 * not a multiple of a 127-frame quantum. Both paths now do exactly one thing -- hand the engine
 * bytes and ask it what it compiled -- so a `SessionPlan` and a raw document are the same call with
 * a different `toString`.
 *
 * # The headless engine needs no scratch instance
 *
 * A browser has a physical rate and quantum it must satisfy, so it boots a scratch instance in a
 * Worker to discover a document's shape before committing an `AudioContext`. Headless has no
 * physical anything: its one instance *is* the boot, with both `require_*` words at zero, and it
 * accepts whatever the document declares. That is why there is no scratch path in this file.
 */

/** Anything that can be handed to the engine as a Session V1 document. */
export type SessionDocument = string | Uint8Array | { toToml(): string };

function documentBytes(document: SessionDocument): Uint8Array<ArrayBuffer> {
  if (typeof document === "string") return new TextEncoder().encode(document);
  if (document instanceof Uint8Array) {
    return document.buffer instanceof ArrayBuffer
      ? (document as Uint8Array<ArrayBuffer>)
      : new Uint8Array(document);
  }
  if (typeof document.toToml === "function") {
    return new TextEncoder().encode(document.toToml());
  }
  throw new MisoUsageError(
    "a session document must be a string, a Uint8Array, or an object with toToml()",
  );
}

export interface OfflineEngineOptions extends BootOptions {
  /** The verified, compiled module. One asset serves any number of engines. */
  readonly asset: MisoEngineAsset;
}

/** A booted headless session. */
export class OfflineEngine {
  readonly #boundary: WasmBoundary;
  readonly #asset: MisoEngineAsset;

  private constructor(asset: MisoEngineAsset, boundary: WasmBoundary) {
    this.#asset = asset;
    this.#boundary = boundary;
  }

  static async create(
    document: SessionDocument,
    options: OfflineEngineOptions,
  ): Promise<OfflineEngine> {
    const { asset, ...boot } = options;
    return new OfflineEngine(asset, await WasmBoundary.boot(asset, documentBytes(document), boot));
  }

  /** The asset this engine booted from, including its compile count and provenance. */
  get asset(): MisoEngineAsset {
    return this.#asset;
  }

  /**
   * What the engine compiled: rate, quantum, derived ring, backend, sources, tracks.
   *
   * Every field is the engine's answer. Nothing here was parsed out of the document's text, which
   * is the whole point -- a consumer that reads its rate from this object cannot be told 48000 by
   * a fallback that never looked.
   */
  shape(): SessionShape {
    return this.#boundary.shape();
  }

  state(): "ready" | "failed" | "disposed" {
    return this.#boundary.state();
  }

  nextAbsoluteSample(): bigint {
    return this.#boundary.nextAbsoluteSample();
  }

  renderedQuanta(): bigint {
    return this.#boundary.renderedQuanta();
  }

  resources(): Readonly<Record<string, number | bigint | readonly number[]>> {
    return this.#boundary.resources();
  }

  /** The exact bytes this engine wrote to the boot options block. */
  get optionBytes(): Uint8Array {
    return this.#boundary.optionBytes;
  }

  submitSource(request: {
    readonly sourceId: string;
    readonly generation: bigint;
    readonly startFrame: bigint;
    readonly planes: readonly Float32Array[];
    readonly endOfRegion: boolean;
  }): { readonly ok: boolean; readonly result: number; readonly code: string } {
    return this.#boundary.submitSource(request);
  }

  render(actualFrames?: number): { readonly left: Float32Array; readonly right: Float32Array } {
    return this.#boundary.render(actualFrames ?? this.shape().quantumFrames);
  }

  submitCommands(records: Uint8Array, count: number): CommandReport {
    return this.#boundary.submitCommands(records, count);
  }

  /**
   * The supported headless mix switch: dispose this session and boot another on the same instance.
   *
   * `dispose()` + `OfflineEngine.create()` is *not* the mix switch and is not documented as one --
   * it makes a fresh instance with a fresh linear memory. Issue #243 S2(a) asks for one exposed
   * reboot verb; this is it.
   */
  loadSession(document: SessionDocument, options: BootOptions = {}): void {
    this.#boundary.reboot(documentBytes(document), options);
  }

  dispose(): void {
    this.#boundary.dispose();
  }
}

/** Boot a headless engine. */
export async function createOfflineEngine(
  document: SessionDocument,
  options: OfflineEngineOptions,
): Promise<OfflineEngine> {
  return OfflineEngine.create(document, options);
}

export type ValidationResult =
  | { readonly ok: true; readonly shape: SessionShape }
  | {
    readonly ok: false;
    readonly phase: ErrorPhase;
    readonly code: MisoErrorCode;
    readonly result: number;
    readonly diagnostics: readonly MisoDiagnostic[];
  };

/**
 * Validate a document by booting it and throwing the result away.
 *
 * # Why this is the same verb rather than a parser
 *
 * There is no TOML parser in this SDK and there never will be (ruling 5438024085): a second
 * implementation of the grammar would answer a question the engine did not ask. `validate()` runs
 * the real engine, so its diagnostics *are* the engine's diagnostics and its budget checks are the
 * real ones, under the real physics gate.
 *
 * # The unbounded-admission bug this fixes
 *
 * The pre-boot-v1 `validateSession` sized its staging from the input -- `max(1 MiB, input.length)`
 * -- so handing it a 64 MiB document made it allocate 64 MiB to discover the document was too
 * large. Boot v1's `document_ptr` refuses an oversize length *before any allocation*, and this
 * function is a thin wrapper over that, so admission of an over-maximum document now costs no
 * staging at all (issue #243 eval 3, and S1's "standing unbounded-admission bug").
 *
 * A refusal is returned, not thrown: validation's whole job is to produce an answer about a
 * document, and a refusal is the most useful answer it has.
 */
export async function validate(
  document: SessionDocument,
  options: OfflineEngineOptions,
): Promise<ValidationResult> {
  let engine: OfflineEngine | undefined;
  try {
    engine = await OfflineEngine.create(document, options);
    return Object.freeze({ ok: true as const, shape: engine.shape() });
  } catch (error) {
    if (error instanceof MisoEngineError) {
      return Object.freeze({
        ok: false as const,
        phase: error.phase,
        code: error.code,
        result: error.result,
        diagnostics: error.diagnostics,
      });
    }
    throw error;
  } finally {
    engine?.dispose();
  }
}

export { MAXIMUM_DOCUMENT_BYTES };
