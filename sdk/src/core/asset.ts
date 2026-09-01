import { ABI_LAYOUT } from "../generated/abi.ts";
import { PROVENANCE } from "../generated/provenance.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";

/**
 * A verified, compiled engine module.
 *
 * # One compile per SDK lifetime
 *
 * `WebAssembly.compile` is the expensive call -- it is the only place a browser or a Node process
 * pays for code generation over a two-and-a-half megabyte module -- and the pre-boot-v1 SDK paid
 * it again on every `validate()` because validation went through a whole throwaway engine
 * construction. Boot v1 makes that unnecessary: a module compiles once and instantiates many
 * times, and an instance *is* a session. So the asset is the unit that owns the compile, and
 * `compileCount` is exposed so a test can assert the number rather than trust the sentence
 * (issue #243 eval 1(c)).
 *
 * # Skew is a double exact pin plus provenance
 *
 * `0x0001_0000` is an exact identity, not a negotiated range: there is no feature detection
 * beyond the version word before launch, by design. Three independent detectors compose, and all
 * three refuse typed at `phase: "asset"` rather than booting garbage:
 *
 * 1. **Asset digest.** A caller that has a release manifest passes `sha256`; the bytes are hashed
 *    and compared before they are ever compiled. The digest is the caller's because the browser
 *    module is a build output -- see the note in `src/generated/provenance.ts` for why no hash is
 *    checked into this tree.
 * 2. **Version word.** Every instance is asked `miso_engine_web_v1_abi_version()` before it is
 *    given a single byte to stage. An old artifact under a new SDK, or the reverse, stops here.
 * 3. **Options-block equality.** Written at boot rather than here: the handshake pair names this
 *    exact layout and version, so a struct that changed shape is a typed `refusedOptions` instead
 *    of an accepted misreading. See `writeBootOptions`.
 */
export class MisoEngineAsset {
  readonly #module: WebAssembly.Module;
  readonly #sha256: string | undefined;
  #compiles = 0;

  private constructor(module: WebAssembly.Module, sha256: string | undefined, compiles: number) {
    this.#module = module;
    this.#sha256 = sha256;
    this.#compiles = compiles;
  }

  /**
   * Verify and compile the module bytes exactly once.
   *
   * @param bytes  the `miso-engine-v1-audio-worklet.simd128.wasm` release artifact
   * @param expectedSha256  lowercase hex digest from the caller's release manifest, if it has one
   */
  static async load(
    bytes: Uint8Array<ArrayBuffer> | ArrayBuffer,
    expectedSha256?: string,
  ): Promise<MisoEngineAsset> {
    const view: Uint8Array<ArrayBuffer> = bytes instanceof ArrayBuffer
      ? new Uint8Array(bytes)
      : bytes;
    let digest: string | undefined;
    if (expectedSha256 !== undefined) {
      if (!/^[0-9a-f]{64}$/.test(expectedSha256)) {
        throw new MisoUsageError(
          `expectedSha256 must be 64 lowercase hex characters, got ${expectedSha256}`,
        );
      }
      digest = await sha256Hex(view);
      if (digest !== expectedSha256) {
        throw new MisoEngineError(
          `the engine asset's digest is ${digest}, but the release manifest names ${expectedSha256}`,
          {
            phase: "asset",
            code: "abiMismatch",
            result: constantOf("abiMismatch"),
            diagnostics: [{ code: "sdk.asset.digest", path: "miso-engine-v1-audio-worklet.simd128.wasm" }],
          },
        );
      }
    }
    let module: WebAssembly.Module;
    try {
      module = await WebAssembly.compile(view);
    } catch (error) {
      throw new MisoEngineError(
        `the engine asset did not compile: ${(error as Error).message}`,
        {
          phase: "asset",
          code: "abiMismatch",
          result: constantOf("abiMismatch"),
          diagnostics: [{ code: "sdk.asset.compile", path: "" }],
        },
      );
    }
    return new MisoEngineAsset(module, digest, 1);
  }

  /** How many times `WebAssembly.compile` ran for this asset. Always 1; asserted, not trusted. */
  get compileCount(): number {
    return this.#compiles;
  }

  /** The verified digest, when the caller supplied one. */
  get sha256(): string | undefined {
    return this.#sha256;
  }

  /** What the SDK was generated against: ABI version, schema tags, artifact set. */
  get provenance(): typeof PROVENANCE {
    return PROVENANCE;
  }

  /**
   * Instantiate the compiled module and pin its ABI version word before anything is staged.
   *
   * Instantiation is cheap next to compilation and is *not* counted: an instance is a session, so
   * a mix switch and a `validate()` both make one. What must never happen twice is the compile.
   */
  async instantiate(): Promise<WebAssembly.Instance> {
    const instance = await WebAssembly.instantiate(this.#module, {});
    const exports = instance.exports as Record<string, unknown>;
    const versionExport = exports[ABI_LAYOUT.stagingSequence[0]];
    if (typeof versionExport !== "function") {
      throw new MisoEngineError(
        `the engine asset does not export ${ABI_LAYOUT.stagingSequence[0]}`,
        {
          phase: "asset",
          code: "abiMismatch",
          result: constantOf("abiMismatch"),
          diagnostics: [{ code: "sdk.asset.export", path: ABI_LAYOUT.stagingSequence[0] }],
        },
      );
    }
    const version = Number((versionExport as () => number)());
    if (version !== ABI_LAYOUT.abiVersion) {
      throw new MisoEngineError(
        `the engine asset reports ABI 0x${version.toString(16).padStart(8, "0")}, `
        + `but this SDK was generated against 0x${ABI_LAYOUT.abiVersion.toString(16).padStart(8, "0")}`,
        {
          phase: "asset",
          code: "abiMismatch",
          result: constantOf("abiMismatch"),
          diagnostics: [{ code: "sdk.asset.abi_version", path: String(version) }],
        },
      );
    }
    return instance;
  }
}

function constantOf(name: string): number {
  return ABI_LAYOUT.constants.resultCodes.find((row) => row.name === name)?.value ?? 255;
}

/**
 * SHA-256 through the Web Crypto API.
 *
 * `crypto.subtle` rather than `node:crypto` on purpose: `src/core` is environment-agnostic by the
 * #207 ruling (5448359546), and Web Crypto is the one digest both a browser main realm and a
 * modern Node process have without an import.
 */
export async function sha256Hex(bytes: Uint8Array<ArrayBuffer>): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}
