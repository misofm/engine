#!/usr/bin/env node
/** Stage the exact gated browser artifact closure under the emitted package tree. */

import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import process from "node:process";

// Read emitted JavaScript, not TypeScript source: package preparation works on every supported
// Node version and never depends on Node's evolving type-stripping loader.
import { PACKAGE_ASSET_MANIFEST_SCHEMA } from "../dist/assets.js";
import { PROVENANCE } from "../dist/generated/provenance.js";

const here = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(here, "..");
const repoRoot = resolve(sdkRoot, "..");

if (process.argv.length !== 3) {
  throw new Error("usage: node sdk/codegen/stage-package.mjs ARTIFACT_DIRECTORY");
}
const source = resolve(process.argv[2]);
const destination = resolve(sdkRoot, "dist", "assets");
const expected = [...PROVENANCE.artifacts];
const actual = (await readdir(source)).sort();
if (JSON.stringify(actual) !== JSON.stringify([...expected].sort())) {
  throw new Error(
    `artifact closure differs from generated provenance\nexpected=${expected.join(",")}\nactual=${actual.join(",")}`,
  );
}

await rm(destination, { recursive: true, force: true });
await mkdir(destination, { recursive: true });
const artifacts = {};
for (const name of expected) {
  const bytes = await readFile(resolve(source, name));
  artifacts[name] = {
    bytes: bytes.byteLength,
    sha256: createHash("sha256").update(bytes).digest("hex"),
  };
  await copyFile(resolve(source, name), resolve(destination, name));
}

const manifest = {
  schema: PACKAGE_ASSET_MANIFEST_SCHEMA,
  abiVersion: PROVENANCE.abiVersion,
  catalogSchema: PROVENANCE.schemas.catalog,
  abiLayoutSchema: PROVENANCE.schemas.abiLayout,
  artifacts,
};
await writeFile(
  resolve(destination, "miso-engine-v1-sdk-manifest.json"),
  `${JSON.stringify(manifest, null, 2)}\n`,
  "utf8",
);
// TypeScript consumes the checked source mirror but does not copy declaration-only inputs to
// `outDir`. The emitted browser declarations refer to it, so stage the same gated declaration
// beside them as part of the package closure.
await copyFile(
  resolve(source, "miso-engine-v1-audio-worklet-host.d.ts"),
  resolve(sdkRoot, "dist", "browser", "shipped-host.d.ts"),
);
await copyFile(resolve(repoRoot, "LICENSE"), resolve(sdkRoot, "dist", "LICENSE"));
console.log(`staged ${expected.length} Engine V1 artifacts and package manifest`);
