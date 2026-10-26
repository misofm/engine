#!/usr/bin/env node
/** Stage the exact gated browser artifact closure under the emitted package tree. */

import { build } from "esbuild";
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
  throw new Error("usage: node sdk/codegen/stage-package.mjs ENGINE_ARTIFACT_DIRECTORY");
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
await copyFile(
  resolve(sdkRoot, "src", "browser-assets", "miso-engine-v1-pcm-feed-worklet.js"),
  resolve(destination, "miso-engine-v1-pcm-feed-worklet.js"),
);
await copyFile(resolve(repoRoot, "NOTICE"), resolve(sdkRoot, "dist", "NOTICE"));
// Issue #2102: the browser entry re-exports the engine's canonical PCM pump and incremental
// BLAKE3 from `./stems/`. In the source tree those two files redirect to
// `hosts/host-web/web/stem-store`; the package carries verbatim copies (with the hash's provenance
// record and its `../hex-lower.js` import), and the hand-written declarations beside them.
const stemStore = resolve(repoRoot, "hosts", "host-web", "web", "stem-store");
const stems = resolve(sdkRoot, "dist", "browser", "stems");
await mkdir(stems, { recursive: true });
for (const name of ["pcm-pump.js", "incremental-blake3.js", "incremental-blake3.provenance.json"]) {
  await copyFile(resolve(stemStore, name), resolve(stems, name));
}
await copyFile(resolve(stemStore, "..", "hex-lower.js"), resolve(sdkRoot, "dist", "browser", "hex-lower.js"));
for (const name of ["pcm-pump.d.ts", "incremental-blake3.d.ts"]) {
  await copyFile(resolve(sdkRoot, "src", "browser", "stems", name), resolve(stems, name));
}
console.log(`staged ${expected.length} Engine V1 artifacts and package manifest`);

// A consumer bundler may copy an exported new-URL asset without following its imports. Emit an
// import-complete Worker so both that URL and the default literal Worker constructor are valid.
await build({
  entryPoints: [resolve(sdkRoot, "dist", "browser", "scratch-worker.js")],
  outfile: resolve(sdkRoot, "dist", "browser", "scratch-worker.js"),
  allowOverwrite: true,
  bundle: true,
  format: "esm",
  platform: "browser",
  target: "es2022",
  legalComments: "inline",
});
await build({
  entryPoints: [resolve(sdkRoot, "dist", "browser", "response-worker.js")],
  outfile: resolve(sdkRoot, "dist", "browser", "response-worker.js"),
  allowOverwrite: true,
  bundle: true,
  format: "esm",
  platform: "browser",
  target: "es2022",
  legalComments: "inline",
});
