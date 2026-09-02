/** Issue #330: the shell gate preserves every accepted artifact-path byte across its sdk/ cd. */

import assert from "node:assert/strict";
import { chmod, mkdir, mkdtemp, realpath, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import process from "node:process";
import { after, describe, test } from "node:test";
import { fileURLToPath } from "node:url";

const sdkRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(sdkRoot, "..");
const defaultScript = join(repoRoot, "scripts", "check-sdk-headless.sh");
const scriptUnderTest = process.env.MISO_ENGINE_HEADLESS_SCRIPT_UNDER_TEST ?? defaultScript;
const scriptAbsolute = isAbsolute(scriptUnderTest) ? scriptUnderTest : resolve(repoRoot, scriptUnderTest);
const expectedSdkRoot = resolve(dirname(scriptAbsolute), "..", "sdk");
const wasmName = "miso-engine-v1-audio-worklet.simd128.wasm";
const minimalWasm = Uint8Array.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);

const scratchRoot = await mkdtemp(join(tmpdir(), "sdk-headless-path-evals-"));
const fakeBin = join(scratchRoot, "bin");
const fakeNodeModule = join(scratchRoot, "fake-node.mjs");
const fakeNode = join(fakeBin, "node");
let unsearchableDirectory;

await mkdir(fakeBin);
await writeFile(fakeNodeModule, `
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import process from "node:process";

assert.equal(process.cwd(), process.env.SDK_PATH_EXPECTED_CWD);
assert.equal(process.env.MISO_ENGINE_SDK_ARTIFACTS, process.env.SDK_PATH_EXPECTED_ARTIFACTS);
const wasm = readFileSync(join(process.env.MISO_ENGINE_SDK_ARTIFACTS, ${JSON.stringify(wasmName)}));
assert.equal(WebAssembly.validate(wasm), true, "fixture must be a valid minimal Wasm module");
assert.deepEqual(process.argv.slice(2), ["--test", "test/*-evals.mjs"]);
process.exit(Number(process.env.SDK_PATH_NODE_STATUS ?? "0"));
`, "utf8");
await writeFile(fakeNode, `#!/usr/bin/env bash
exec "$SDK_PATH_REAL_NODE" "$SDK_PATH_FAKE_NODE" "$@"
`, { encoding: "utf8", mode: 0o755 });

after(async () => {
  if (unsearchableDirectory) {
    await chmod(unsearchableDirectory, 0o700).catch(() => {});
  }
  await rm(scratchRoot, { recursive: true, force: true });
});

async function makeArtifacts(name) {
  const directory = join(scratchRoot, name);
  await mkdir(directory);
  await writeFile(join(directory, wasmName), minimalWasm);
  return directory;
}

function invoke(args, { expectedArtifacts, nodeStatus } = {}) {
  const env = {
    ...process.env,
    PATH: `${fakeBin}:${process.env.PATH ?? ""}`,
    SDK_PATH_REAL_NODE: process.execPath,
    SDK_PATH_FAKE_NODE: fakeNodeModule,
    SDK_PATH_EXPECTED_CWD: expectedSdkRoot,
  };
  if (expectedArtifacts !== undefined) {
    env.SDK_PATH_EXPECTED_ARTIFACTS = expectedArtifacts;
  }
  if (nodeStatus !== undefined) {
    env.SDK_PATH_NODE_STATUS = String(nodeStatus);
  }
  const result = spawnSync("bash", [scriptUnderTest, ...args], {
    cwd: repoRoot,
    env,
    encoding: "utf8",
  });
  assert.equal(result.signal, null, `script was signalled: ${result.signal}`);
  return result;
}

function diagnostic(result) {
  return `stdout=${JSON.stringify(result.stdout)} stderr=${JSON.stringify(result.stderr)}`;
}

async function expectAccepted(name, { absolute = false } = {}) {
  const directory = await makeArtifacts(name);
  const oracle = await realpath(directory); // Node/libc oracle; never production's shell capture.
  const argument = absolute ? directory : relative(repoRoot, directory);
  const result = invoke([argument], { expectedArtifacts: oracle });
  assert.equal(result.status, 0, diagnostic(result));
}

describe("check-sdk-headless artifact path bytes", { concurrency: false }, () => {
  const accepted = [
    ["ordinary relative", "ordinary"],
    ["absolute", "absolute", { absolute: true }],
    ["spaces", "space path"],
    ["tabs and metacharacters", "tab\t$;[]{}*?"],
    ["embedded newline", "embedded\nnewline"],
    ["one terminal newline", "terminal-newline\n"],
    ["repeated terminal newlines", "repeated-newline\n\n"],
    ["sentinel-like final byte", "sentinel-like-x"],
  ];
  for (const [label, name, options] of accepted) {
    test(label, async () => expectAccepted(name, options));
  }

  test("alternate script probe is exact", () => {
    assert.equal(scriptUnderTest, process.env.MISO_ENGINE_HEADLESS_SCRIPT_UNDER_TEST ?? defaultScript);
    assert.equal(isAbsolute(defaultScript), true);
  });

  test("usage and missing or non-directory inputs return 2", async () => {
    const nonDirectory = join(scratchRoot, "ordinary-file");
    await writeFile(nonDirectory, "not a directory", "utf8");
    for (const args of [["one", "two"], [join(scratchRoot, "missing")], [nonDirectory]]) {
      const result = invoke(args);
      assert.equal(result.status, 2, diagnostic(result));
    }
  });

  test("missing module and direct symlink return 2", async () => {
    const missingModule = join(scratchRoot, "missing-module");
    await mkdir(missingModule);
    const valid = await makeArtifacts("symlink-target");
    const link = join(scratchRoot, "artifact-link");
    await symlink(valid, link, "dir");
    for (const argument of [missingModule, link]) {
      const result = invoke([argument]);
      assert.equal(result.status, 2, diagnostic(result));
    }
  });

  test("unsearchable directory returns 2", async () => {
    unsearchableDirectory = await makeArtifacts("unsearchable");
    await chmod(unsearchableDirectory, 0o000);
    const result = invoke([unsearchableDirectory]);
    await chmod(unsearchableDirectory, 0o700);
    unsearchableDirectory = undefined;
    assert.equal(result.status, 2, diagnostic(result));
  });

  test("validated Node failure propagates unchanged", async () => {
    const directory = await makeArtifacts("node-failure");
    const oracle = await realpath(directory);
    const result = invoke([relative(repoRoot, directory)], {
      expectedArtifacts: oracle,
      nodeStatus: 37,
    });
    assert.equal(result.status, 37, diagnostic(result));
  });
});
