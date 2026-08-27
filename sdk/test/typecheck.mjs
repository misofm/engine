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
  const fixtures = [
    resolve(sdkRoot, "src", "headless", "node-shims.d.ts"),
    resolve(sdkRoot, "test", "builder-types.ts"),
    resolve(sdkRoot, "test", "headless-types.ts"),
  ];
  const tupleResult = compile([
    "--ignoreConfig", "--noEmit", "--strict", "--target", "ES2022", "--module", "NodeNext", "--moduleResolution", "NodeNext", ...fixtures,
  ]);
  assert.equal(tupleResult.status, 0, `tuple index fixture failed:\n${tupleResult.stdout}${tupleResult.stderr}`);
}

async function selfTest() {
  await checkProject();
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-tsc-"));
  try {
    const red = resolve(directory, "red.ts");
    const sdkIndex = resolve(sdkRoot, "src", "index.js");
    await writeFile(red, [
      `import { effect, type TrackConsole } from ${JSON.stringify(sdkIndex)};`,
      'const effects = [effect("miso.compressor", { threshold: -18 })] as const;',
      'declare const track: TrackConsole<typeof effects>;',
      'track.effect(1);',
      '',
    ].join("\n"), "utf8");
    const result = compile([
      "--ignoreConfig",
      "--noEmit",
      "--strict",
      "--target", "ES2022",
      "--module", "NodeNext",
      "--moduleResolution", "NodeNext",
      red,
    ]);
    assert.notEqual(result.status, 0, "tsc must reject the deliberate unsuppressed tuple-index mutation");
    assert.match(result.stdout + result.stderr, /Argument of type '1' is not assignable/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

if (process.argv.length === 2) {
  await checkProject();
  console.log("SDK TypeScript project check passed");
} else if (process.argv.length === 3 && process.argv[2] === "--self-test") {
  await selfTest();
  console.log("SDK TypeScript self-test passed (deliberate unsuppressed tuple-index red mutation caught)");
} else {
  throw new Error("usage: node sdk/test/typecheck.mjs [--self-test]");
}
