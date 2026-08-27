#!/usr/bin/env node
/** Phase 1 compiler gate, including a deliberate E5-style red type mutation. */

import assert from "node:assert/strict";
import { access, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const here = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(here, "..");
const compiler = resolve(sdkRoot, "node_modules", ".bin", "tsc");

async function requireCompiler() {
  try {
    await access(compiler);
  } catch {
    throw new Error("TypeScript compiler is absent; run `npm ci --prefix sdk --ignore-scripts`");
  }
}

function compile(args) {
  return spawnSync(compiler, args, { cwd: sdkRoot, encoding: "utf8" });
}

async function checkProject() {
  await requireCompiler();
  const result = compile(["--project", "tsconfig.json", "--noEmit"]);
  assert.equal(result.status, 0, `tsc --noEmit failed:\n${result.stdout}${result.stderr}`);
}

async function selfTest() {
  await checkProject();
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-tsc-"));
  try {
    const red = resolve(directory, "red.ts");
    await writeFile(red, "const impossible: string = 1;\n", "utf8");
    const result = compile([
      "--ignoreConfig",
      "--noEmit",
      "--strict",
      "--target", "ES2022",
      "--module", "NodeNext",
      "--moduleResolution", "NodeNext",
      red,
    ]);
    assert.notEqual(result.status, 0, "tsc must reject the deliberate wrong-type mutation");
    assert.match(result.stdout + result.stderr, /Type 'number' is not assignable to type 'string'/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

if (process.argv.length === 2) {
  await checkProject();
  console.log("SDK TypeScript project check passed");
} else if (process.argv.length === 3 && process.argv[2] === "--self-test") {
  await selfTest();
  console.log("SDK TypeScript self-test passed (wrong-type red mutation caught)");
} else {
  throw new Error("usage: node sdk/test/typecheck.mjs [--self-test]");
}
