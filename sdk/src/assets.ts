/** The package-relative Engine V1 artifact closure embedded in every published SDK tarball. */

export const PACKAGE_ASSET_MANIFEST_SCHEMA = "miso.sdk.package-assets.v1" as const;

export const BUNDLED_ENGINE_FILES = Object.freeze({
  wasm: "miso-engine-v1-audio-worklet.simd128.wasm",
  workletModule: "miso-engine-v1-audio-worklet.js",
  hostModule: "miso-engine-v1-audio-worklet-host.js",
  hostDeclaration: "miso-engine-v1-audio-worklet-host.d.ts",
  parameterMetadata: "miso-engine-v1-parameter-metadata.json",
  abiLayout: "miso-engine-v1-abi-layout.json",
  manifest: "miso-engine-v1-sdk-manifest.json",
} as const);

/**
 * Stable URLs to the artifacts installed beside the emitted SDK modules.
 *
 * `new URL(..., import.meta.url)` deliberately leaves asset delivery to the consumer's runtime or
 * bundler while binding every URL to this exact package version. A browser no longer needs a CDN
 * URL from a potentially different engine release, and a Node/Bun consumer can read the same URL
 * directly from the installed package.
 */
export const BUNDLED_ENGINE_ASSETS = Object.freeze({
  wasm: new URL("./assets/miso-engine-v1-audio-worklet.simd128.wasm", import.meta.url),
  workletModule: new URL("./assets/miso-engine-v1-audio-worklet.js", import.meta.url),
  hostModule: new URL("./assets/miso-engine-v1-audio-worklet-host.js", import.meta.url),
  hostDeclaration: new URL("./assets/miso-engine-v1-audio-worklet-host.d.ts", import.meta.url),
  parameterMetadata: new URL("./assets/miso-engine-v1-parameter-metadata.json", import.meta.url),
  abiLayout: new URL("./assets/miso-engine-v1-abi-layout.json", import.meta.url),
  manifest: new URL("./assets/miso-engine-v1-sdk-manifest.json", import.meta.url),
});

export interface PackageAssetRecord {
  readonly bytes: number;
  readonly sha256: string;
}

export interface PackageAssetManifest {
  readonly schema: typeof PACKAGE_ASSET_MANIFEST_SCHEMA;
  readonly abiVersion: number;
  readonly catalogSchema: string;
  readonly abiLayoutSchema: string;
  readonly artifacts: Readonly<Record<string, PackageAssetRecord>>;
}
