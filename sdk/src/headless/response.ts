import { MisoEngineAsset } from "../core/asset.ts";
import { ResponsePreviewModule } from "../core/response.ts";
import type {
  ResponsePreviewCapability,
  ResponsePreviewLimits,
  ResponsePreviewQuery,
  ResponsePreviewResult,
} from "../core/response.ts";
import { loadBundledEngineAsset } from "./assets.ts";

export interface HeadlessResponsePreviewOptions {
  /** A verified asset supplied by the host. Absent loads the package's bundled asset. */
  readonly asset?: MisoEngineAsset;
  readonly responseLimits?: ResponsePreviewLimits;
}

/** A stopped, synchronous response preview over one analysis-only Wasm instance. */
export class HeadlessResponsePreview {
  readonly #asset: MisoEngineAsset;
  readonly #module: ResponsePreviewModule;

  private constructor(asset: MisoEngineAsset, module: ResponsePreviewModule) {
    this.#asset = asset;
    this.#module = module;
  }

  static async create(options: HeadlessResponsePreviewOptions = {}): Promise<HeadlessResponsePreview> {
    const asset = options.asset ?? await loadBundledEngineAsset();
    const instance = await asset.instantiate();
    return new HeadlessResponsePreview(asset, new ResponsePreviewModule(instance, options.responseLimits));
  }

  get asset(): MisoEngineAsset { return this.#asset; }

  get capabilities(): readonly ResponsePreviewCapability[] { return this.#module.capabilities; }

  query(request: ResponsePreviewQuery): ResponsePreviewResult {
    return this.#module.query(request);
  }

  close(): void { this.#module.close(); }
}

/** Create a response preview without booting a session or audio engine. */
export async function createResponsePreview(
  options: HeadlessResponsePreviewOptions = {},
): Promise<HeadlessResponsePreview> {
  return HeadlessResponsePreview.create(options);
}
