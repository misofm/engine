#!/usr/bin/env node
/** E1: generated catalog values are a transcription of the shipped metadata JSON. */

import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const here = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(here, "..");
const metadataPath = resolve(sdkRoot, "assets", "miso-engine-v2-parameter-metadata.json");
const catalogPath = resolve(sdkRoot, "src", "generated", "catalog.ts");
const generatorPath = resolve(sdkRoot, "codegen", "generate.mjs");
const start = "export const CATALOG = deepFreeze(\n";
const end = " as const,\n);";

function catalogValue(source) {
  const from = source.indexOf(start);
  assert.notEqual(from, -1, "generated catalog start marker");
  const jsonStart = from + start.length;
  const to = source.indexOf(end, jsonStart);
  assert.notEqual(to, -1, "generated catalog end marker");
  return JSON.parse(source.slice(jsonStart, to));
}

async function parity(path = catalogPath) {
  const [metadata, catalog] = await Promise.all([
    readFile(metadataPath, "utf8").then(JSON.parse),
    readFile(path, "utf8").then(catalogValue),
  ]);
  assert.deepEqual(catalog, metadata, "generated CATALOG must deep-equal engine metadata");
}

async function selfTest() {
  await parity();
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-e1-"));
  try {
    const copied = resolve(directory, "catalog.ts");
    const source = await readFile(catalogPath, "utf8");
    assert.match(source, /"observationUnbound"/, "red mutation anchor exists");
    await writeFile(copied, source.replace("observationUnbound", "observationDetached"), "utf8");
    await assert.rejects(parity(copied), assert.AssertionError);

    const generatedCopy = await readFile(catalogPath, "utf8");
    await writeFile(catalogPath, generatedCopy.replace("observationUnbound", "observationDetached"), "utf8");
    try {
      const result = spawnSync(process.execPath, [generatorPath, "--check"], {
        cwd: sdkRoot,
        encoding: "utf8",
      });
      assert.notEqual(result.status, 0, "codegen check catches a generated-value mutation");
    } finally {
      await writeFile(catalogPath, generatedCopy, "utf8");
    }
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

if (process.argv.length === 2) {
  await parity();
  console.log("E1 generated catalog parity passed");
} else if (process.argv.length === 3 && process.argv[2] === "--self-test") {
  await selfTest();
  console.log("E1 generated catalog parity self-test passed (generated-value red mutation caught)");
} else {
  throw new Error("usage: node sdk/test/generated-parity.mjs [--self-test]");
}
