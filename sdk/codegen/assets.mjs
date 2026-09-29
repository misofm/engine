#!/usr/bin/env node
/**
 * Refresh `sdk/assets/` from the engine itself.
 *
 * The SDK's type-shaped surface is a transcription chain with exactly one authority at the top:
 *
 *   Rust structures and frozen constants
 *     -> `parameter-metadata` (offset_of!, registry walk)
 *       -> sdk/assets/*.json            <- this script
 *         -> sdk/src/generated/*.ts     <- codegen/generate.mjs
 *           -> the SDK's public types
 *
 * Splitting the chain here, at a checked-in JSON file, is deliberate. The generated TypeScript has
 * to be readable in review and diffable in a pull request, and a published package has to carry
 * the vocabulary it was built against without shipping a Rust toolchain. So the JSON is committed,
 * and `--check` re-runs the engine's generator and compares byte for byte: a hand edit to either
 * the JSON or the TypeScript fails before a consumer can observe the drift, which is issue #243's
 * eval 6 (`codegen drift structural impossibility`).
 *
 * `--check` needs cargo. That is not a hardship in this repository -- it *is* the Rust repository
 * -- and it is the point: the assets are only as good as the engine they were taken from, so the
 * check re-derives rather than re-reads.
 *
 * `--check ENGINE_ARTIFACT_DIRECTORY` (issue #1044) compares with that directory's copies instead
 * of running cargo. `scripts/build-web-audioworklet.sh` writes them with the same generator's
 * `--write`, which emits exactly the `--print` documents, so they are the engine's output at the
 * commit the directory was built from, and the documents the SDK package ships beside the module.
 * CI builds that directory once per run and hands it to the SDK job.
 */

import { execFileSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import process from "node:process";

const here = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(here, "..");
const repoRoot = resolve(sdkRoot, "..");
const assets = resolve(sdkRoot, "assets");

/** Each asset, and the generator mode that renders it. One generator, two documents. */
const documents = [
  ["miso-engine-v1-parameter-metadata.json", "--print"],
  ["miso-engine-v1-abi-layout.json", "--print-abi-layout"],
];

function render(mode) {
  return execFileSync(
    "cargo",
    ["run", "--locked", "--release", "-q", "-p", "parameter-metadata", "--", mode],
    { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 26 },
  );
}

async function run(check, artifacts) {
  let stale = false;
  for (const [name, mode] of documents) {
    const path = resolve(assets, name);
    // The engine's bytes for this document: generated, or an engine artifact directory's copy.
    let expected;
    if (artifacts === undefined) {
      expected = Buffer.from(render(mode), "utf8");
    } else {
      try {
        expected = await readFile(resolve(artifacts, name));
      } catch (error) {
        console.error(`sdk assets: the engine artifact has no ${name}: ${error.message}`);
        stale = true;
        continue;
      }
    }
    if (!check) {
      await writeFile(path, expected);
      console.log(`wrote ${path}`);
      continue;
    }
    let actual;
    try {
      actual = await readFile(path);
    } catch (error) {
      console.error(`sdk assets: missing ${path}: ${error.message}`);
      stale = true;
      continue;
    }
    if (!actual.equals(expected)) {
      const source = artifacts === undefined ? "the generator's output" : resolve(artifacts, name);
      console.error(`sdk assets: ${path} differs from ${source}; run \`npm run assets\` in sdk/`);
      stale = true;
    }
  }
  if (stale) process.exitCode = 1;
  else if (check) console.log("sdk assets are the engine's current output");
}

const args = process.argv.slice(2);
if (args.length > 2 || (args.length >= 1 && args[0] !== "--check")) {
  throw new Error("usage: node sdk/codegen/assets.mjs [--check [ENGINE_ARTIFACT_DIRECTORY]]");
}
await run(args[0] === "--check", args.length === 2 ? resolve(args[1]) : undefined);
