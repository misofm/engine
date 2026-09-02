import { readFile } from "node:fs/promises";

import {
  BUNDLED_ENGINE_ASSETS,
  BUNDLED_ENGINE_FILES,
  PACKAGE_ASSET_MANIFEST_SCHEMA,
} from "../assets.ts";
import type { PackageAssetManifest, PackageAssetRecord } from "../assets.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import { PROVENANCE } from "../generated/provenance.ts";
import { MisoEngineAsset } from "../core/asset.ts";
import { constantValue } from "../core/abi.ts";
import { MisoEngineError } from "../core/errors.ts";

function assetFailure(message: string, path: string): MisoEngineError {
  return new MisoEngineError(message, {
    phase: "asset",
    code: "abiMismatch",
    result: constantValue("resultCodes", "abiMismatch"),
    diagnostics: [{ code: "sdk.asset.package_manifest", path }],
  });
}

function record(value: unknown, name: string): PackageAssetRecord {
  if (typeof value !== "object" || value === null) {
    throw assetFailure(`the bundled manifest has no record for ${name}`, name);
  }
  const candidate = value as Record<string, unknown>;
  if (!Number.isSafeInteger(candidate.bytes) || Number(candidate.bytes) <= 0
    || typeof candidate.sha256 !== "string" || !/^[0-9a-f]{64}$/.test(candidate.sha256)) {
    throw assetFailure(`the bundled manifest has an invalid record for ${name}`, name);
  }
  return Object.freeze({ bytes: Number(candidate.bytes), sha256: candidate.sha256 });
}

function parseManifest(text: string): PackageAssetManifest {
  let value: unknown;
  try {
    value = JSON.parse(text);
  } catch (error) {
    throw assetFailure(`the bundled manifest is not JSON: ${(error as Error).message}`, "");
  }
  if (typeof value !== "object" || value === null) {
    throw assetFailure("the bundled manifest is not an object", "");
  }
  const candidate = value as Record<string, unknown>;
  if (candidate.schema !== PACKAGE_ASSET_MANIFEST_SCHEMA
    || candidate.abiVersion !== ABI_LAYOUT.abiVersion
    || candidate.catalogSchema !== PROVENANCE.schemas.catalog
    || candidate.abiLayoutSchema !== PROVENANCE.schemas.abiLayout
    || typeof candidate.artifacts !== "object" || candidate.artifacts === null) {
    throw assetFailure("the bundled manifest does not match this SDK's generated contract", "");
  }
  const artifacts: Record<string, PackageAssetRecord> = {};
  for (const name of PROVENANCE.artifacts) {
    artifacts[name] = record((candidate.artifacts as Record<string, unknown>)[name], name);
  }
  return Object.freeze({
    schema: PACKAGE_ASSET_MANIFEST_SCHEMA,
    abiVersion: ABI_LAYOUT.abiVersion,
    catalogSchema: PROVENANCE.schemas.catalog,
    abiLayoutSchema: PROVENANCE.schemas.abiLayout,
    artifacts: Object.freeze(artifacts),
  });
}

/** Read and validate the manifest installed in this exact package. */
export async function readBundledPackageManifest(): Promise<PackageAssetManifest> {
  try {
    return parseManifest(await readFile(BUNDLED_ENGINE_ASSETS.manifest, "utf8"));
  } catch (error) {
    if (error instanceof MisoEngineError) throw error;
    throw assetFailure(`the bundled manifest could not be read: ${(error as Error).message}`, "");
  }
}

/**
 * Load, hash-verify, compile and ABI-pin the Wasm installed in this package.
 *
 * Call this once and pass the returned asset to several engines when compilation sharing matters.
 * `createOfflineEngine` calls it automatically when no explicit asset is supplied.
 */
export async function loadBundledEngineAsset(): Promise<MisoEngineAsset> {
  const manifest = await readBundledPackageManifest();
  const expected = manifest.artifacts[BUNDLED_ENGINE_FILES.wasm];
  if (expected === undefined) {
    throw assetFailure("the bundled manifest does not name the engine Wasm", BUNDLED_ENGINE_FILES.wasm);
  }
  let bytes: Uint8Array<ArrayBuffer>;
  try {
    bytes = Uint8Array.from(await readFile(BUNDLED_ENGINE_ASSETS.wasm));
  } catch (error) {
    throw assetFailure(
      `the bundled engine Wasm could not be read: ${(error as Error).message}`,
      BUNDLED_ENGINE_FILES.wasm,
    );
  }
  if (bytes.byteLength !== expected.bytes) {
    throw assetFailure(
      `the bundled engine Wasm is ${bytes.byteLength} bytes; the manifest records ${expected.bytes}`,
      BUNDLED_ENGINE_FILES.wasm,
    );
  }
  return MisoEngineAsset.load(bytes, expected.sha256);
}
