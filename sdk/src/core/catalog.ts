import { ABI_LAYOUT } from "../generated/abi.js";
import { CATALOG } from "../generated/catalog.js";
import { PROVENANCE } from "../generated/provenance.js";

/** The generated catalog is the sole SDK authority for effects and parameters. */
export { CATALOG } from "../generated/catalog.js";

const assetHashes = Object.freeze(Object.fromEntries(
  Object.entries(PROVENANCE.assets).map(([name, asset]) => [name, asset.sha256]),
)) as Readonly<Record<keyof typeof PROVENANCE.assets, string>>;

/** A stable, JSON-serializable description suitable for a human or an agent. */
export const ENGINE_DESCRIPTION = Object.freeze({
  schema: "miso.sdk.describe.v1",
  engine: Object.freeze({
    revision: PROVENANCE.sourceRevision,
    abiVersion: ABI_LAYOUT.abiVersion,
    sampleRates: Object.freeze([44_100, 48_000, 88_200, 96_000] as const),
    quantumFrames: 128,
    wasmBytes: PROVENANCE.assets["miso-engine-v2-audio-worklet.simd128.wasm"].bytes,
    assetHashes,
    backend: PROVENANCE.backend,
  }),
  catalog: CATALOG,
});

export type EngineDescription = typeof ENGINE_DESCRIPTION;

/** Return the frozen generated description; callers must treat it as read-only. */
export function describe(): EngineDescription {
  return ENGINE_DESCRIPTION;
}
