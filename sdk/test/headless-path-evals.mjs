/** Issue #330: the shell gate preserves every accepted artifact-path byte across its sdk/ cd. */

import assert from "node:assert/strict";
import { chmod, copyFile, mkdir, mkdtemp, realpath, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import process from "node:process";
import { after, describe, test } from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

const sdkRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(sdkRoot, "..");
const defaultScript = join(repoRoot, "scripts", "check-sdk-headless.sh");
const scriptUnderTest = process.env.MISO_ENGINE_HEADLESS_SCRIPT_UNDER_TEST ?? defaultScript;
const scriptAbsolute = isAbsolute(scriptUnderTest) ? scriptUnderTest : resolve(repoRoot, scriptUnderTest);
const scriptRepoRoot = resolve(dirname(scriptAbsolute), "..");
const workflowScript = relative(scriptRepoRoot, scriptAbsolute);
const expectedSdkRoot = resolve(dirname(scriptAbsolute), "..", "sdk");
const supportModuleUrl = pathToFileURL(join(sdkRoot, "test", "support.mjs")).href;
const wasmName = "miso-engine-v1-audio-worklet.simd128.wasm";
const minimalWasm = Uint8Array.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);

const scratchRoot = await mkdtemp(join(tmpdir(), "sdk-headless-path-evals-"));
const fakeBin = join(scratchRoot, "bin");
const fakeNodeModule = join(scratchRoot, "fake-node.mjs");
const fakeNode = join(fakeBin, "node");
let unsearchableDirectory;

await chmod(scratchRoot, 0o755);
await mkdir(fakeBin);
await writeFile(fakeNodeModule, `
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import process from "node:process";

assert.equal(process.cwd(), process.env.SDK_PATH_EXPECTED_CWD);
const encoded = process.env.MISO_ENGINE_SDK_ARTIFACTS_HEX;
assert.match(encoded, /^(?:[0-9a-f]{2})+$/);
assert.equal(encoded, process.env.SDK_PATH_EXPECTED_ARTIFACTS_HEX);
const wasmPath = Buffer.concat([
  Buffer.from(encoded, "hex"),
  Buffer.from(${JSON.stringify(`/${wasmName}`)}, "ascii"),
]);
const wasm = readFileSync(wasmPath);
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

function invocationEnv({ expectedArtifactsHex, expectedCwd = expectedSdkRoot, nodeStatus } = {}) {
  const env = {
    ...process.env,
    PATH: `${fakeBin}:${process.env.PATH ?? ""}`,
    SDK_PATH_REAL_NODE: process.execPath,
    SDK_PATH_FAKE_NODE: fakeNodeModule,
    SDK_PATH_EXPECTED_CWD: expectedCwd,
  };
  if (expectedArtifactsHex !== undefined) {
    env.SDK_PATH_EXPECTED_ARTIFACTS_HEX = expectedArtifactsHex;
  }
  if (nodeStatus !== undefined) {
    env.SDK_PATH_NODE_STATUS = String(nodeStatus);
  }
  return env;
}

function invoke(args, {
  cdpath,
  cwd = repoRoot,
  expectedArtifactsHex,
  expectedCwd = expectedSdkRoot,
  gid,
  nodeStatus,
  script = scriptUnderTest,
  uid,
} = {}) {
  const env = invocationEnv({ expectedArtifactsHex, expectedCwd, nodeStatus });
  if (cdpath !== undefined) {
    env.CDPATH = cdpath;
  }
  const result = spawnSync("bash", [script, ...args], {
    cwd,
    env,
    encoding: "utf8",
    gid,
    uid,
  });
  assert.equal(result.signal, null, `script was signalled: ${result.signal}`);
  return result;
}

function invokeSupport(encoded) {
  const env = { ...process.env };
  delete env.MISO_ENGINE_SDK_ARTIFACTS;
  delete env.MISO_ENGINE_SDK_ARTIFACTS_HEX;
  if (encoded !== undefined) {
    env.MISO_ENGINE_SDK_ARTIFACTS_HEX = encoded;
  }
  return spawnSync(process.execPath, [
    "--input-type=module",
    "--eval",
    `import { moduleBytes } from ${JSON.stringify(supportModuleUrl)};
const bytes = await moduleBytes();
if (!WebAssembly.validate(bytes)) process.exit(9);`,
  ], { env, encoding: "utf8" });
}

function diagnostic(result) {
  return `stdout=${JSON.stringify(result.stdout)} stderr=${JSON.stringify(result.stderr)}`;
}

async function expectAccepted(name, { absolute = false } = {}) {
  const directory = await makeArtifacts(name);
  const oracle = await realpath(directory, { encoding: "buffer" });
  const argument = absolute ? directory : relative(repoRoot, directory);
  const result = invoke([argument], { expectedArtifactsHex: oracle.toString("hex") });
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

  test("workflow-relative invocation ignores CDPATH output and shadow selection", async () => {
    const directory = await makeArtifacts("workflow-relative");
    const oracle = await realpath(directory, { encoding: "buffer" });
    const shadowRoot = join(scratchRoot, "repo-root-shadow");
    await mkdir(join(shadowRoot, "scripts"), { recursive: true });
    await mkdir(join(shadowRoot, "sdk"));

    for (const cdpath of [".", shadowRoot]) {
      const result = invoke([directory], {
        cdpath,
        cwd: scriptRepoRoot,
        expectedArtifactsHex: oracle.toString("hex"),
        script: workflowScript,
      });
      assert.equal(result.status, 0, `${cdpath}: ${diagnostic(result)}`);
    }
  });

  test("repository-root capture preserves repeated terminal newlines", async () => {
    const copiedRepo = join(scratchRoot, "copied-repo\n\n");
    const copiedScript = join(copiedRepo, "scripts", "check-sdk-headless.sh");
    const copiedSdk = join(copiedRepo, "sdk");
    await mkdir(dirname(copiedScript), { recursive: true });
    await mkdir(copiedSdk);
    await copyFile(scriptAbsolute, copiedScript);
    const directory = await makeArtifacts("repo-root-newlines");
    const oracle = await realpath(directory, { encoding: "buffer" });
    const result = invoke([directory], {
      cdpath: ".",
      expectedArtifactsHex: oracle.toString("hex"),
      expectedCwd: await realpath(copiedSdk),
      script: copiedScript,
    });
    assert.equal(result.status, 0, diagnostic(result));
  });

  test("artifact resolution ignores CDPATH for relative and absolute forms", async () => {
    const caller = join(scratchRoot, "artifact-caller");
    const shadowRoot = join(scratchRoot, "artifact-shadow");
    const local = join(caller, "chosen-artifact");
    const shadow = join(shadowRoot, "chosen-artifact");
    await mkdir(local, { recursive: true });
    await mkdir(shadow, { recursive: true });
    await writeFile(join(local, wasmName), minimalWasm);
    await writeFile(join(shadow, wasmName), minimalWasm);
    const oracle = await realpath(local, { encoding: "buffer" });

    for (const argument of ["chosen-artifact", local]) {
      const result = invoke([argument], {
        cdpath: shadowRoot,
        cwd: caller,
        expectedArtifactsHex: oracle.toString("hex"),
      });
      assert.equal(result.status, 0, `${argument}: ${diagnostic(result)}`);
    }
  });

  test("an ancestor symlink is accepted and physically resolved", async () => {
    const realAncestor = join(scratchRoot, "real-ancestor");
    const directory = join(realAncestor, "artifacts");
    const ancestorLink = join(scratchRoot, "ancestor-link");
    await mkdir(directory, { recursive: true });
    await writeFile(join(directory, wasmName), minimalWasm);
    await symlink(realAncestor, ancestorLink, "dir");
    const argument = join(ancestorLink, "artifacts");
    const oracle = await realpath(directory, { encoding: "buffer" });
    const result = invoke([argument], { expectedArtifactsHex: oracle.toString("hex") });
    assert.equal(result.status, 0, diagnostic(result));
  });

  test("support rejects missing and malformed artifact-directory hex", () => {
    for (const [encoded, pattern] of [
      [undefined, /must encode a directory/],
      ["0", /canonical lowercase, even-length hex/],
      ["gg", /canonical lowercase, even-length hex/],
      ["2F", /canonical lowercase, even-length hex/],
    ]) {
      const result = invokeSupport(encoded);
      assert.equal(result.status, 1, diagnostic(result));
      assert.match(result.stderr, pattern);
    }
  });

  test("invalid-UTF-8 filename bytes survive ASCII hex transport", async (context) => {
    const invalidParent = join(scratchRoot, "invalid-utf8-parent");
    await mkdir(invalidParent);
    const invalidDirectory = Buffer.concat([
      Buffer.from(invalidParent),
      Buffer.from("/", "ascii"),
      Buffer.from([0xff]),
    ]);
    try {
      await mkdir(invalidDirectory);
    } catch (error) {
      const unsupported = ["EINVAL", "EILSEQ", "ENOTSUP"].includes(error?.code)
        || (error?.code === "EPERM" && process.platform !== "linux");
      if (unsupported) {
        context.skip(`filesystem rejected an invalid-UTF-8 filename: ${error.code}`);
        return;
      }
      throw error;
    }
    await writeFile(Buffer.concat([
      invalidDirectory,
      Buffer.from(`/${wasmName}`, "ascii"),
    ]), minimalWasm);
    const oracle = await realpath(invalidDirectory, { encoding: "buffer" });
    const expectedArtifactsHex = oracle.toString("hex");
    const helper = join(scratchRoot, "invoke-invalid-utf8.sh");
    await writeFile(helper, `#!/usr/bin/env bash
set -euo pipefail
invalid_component=$(printf '\\377')
exec bash "$SDK_PATH_SCRIPT_UNDER_TEST" "$SDK_PATH_INVALID_PARENT/$invalid_component"
`, { encoding: "ascii", mode: 0o755 });
    const env = invocationEnv({ expectedArtifactsHex });
    env.LC_ALL = "C";
    env.SDK_PATH_INVALID_PARENT = invalidParent;
    env.SDK_PATH_SCRIPT_UNDER_TEST = scriptUnderTest;
    const result = spawnSync("bash", [helper], { cwd: repoRoot, env, encoding: "utf8" });
    assert.equal(result.status, 0, diagnostic(result));

    const supportResult = invokeSupport(expectedArtifactsHex);
    assert.equal(supportResult.status, 0, diagnostic(supportResult));
  });

  test("usage and missing or non-directory inputs return 2", async () => {
    const nonDirectory = join(scratchRoot, "ordinary-file");
    await writeFile(nonDirectory, "not a directory", "utf8");
    for (const args of [["one", "two"], [join(scratchRoot, "missing")], [nonDirectory]]) {
      const result = invoke(args);
      assert.equal(result.status, 2, diagnostic(result));
    }
  });

  test("missing module and every direct-symlink directory spelling return 2", async () => {
    const missingModule = join(scratchRoot, "missing-module");
    await mkdir(missingModule);
    const valid = await makeArtifacts("symlink-target");
    const link = join(scratchRoot, "artifact-link");
    await symlink(valid, link, "dir");
    for (const argument of [missingModule, link, `${link}/`, `${link}/.`, `${link}//./`]) {
      const result = invoke([argument]);
      assert.equal(result.status, 2, diagnostic(result));
    }
  });

  test("unsearchable directory returns 2, including under a root test runner", async () => {
    unsearchableDirectory = await makeArtifacts("unsearchable");
    await chmod(unsearchableDirectory, 0o000);
    const rootRunner = typeof process.getuid === "function" && process.getuid() === 0;
    const result = invoke([unsearchableDirectory], rootRunner ? { gid: 65534, uid: 65534 } : {});
    await chmod(unsearchableDirectory, 0o700);
    unsearchableDirectory = undefined;
    assert.equal(result.status, 2, diagnostic(result));
  });

  test("validated Node failure propagates unchanged", async () => {
    const directory = await makeArtifacts("node-failure");
    const oracle = await realpath(directory, { encoding: "buffer" });
    const result = invoke([relative(repoRoot, directory)], {
      expectedArtifactsHex: oracle.toString("hex"),
      nodeStatus: 37,
    });
    assert.equal(result.status, 37, diagnostic(result));
  });
});
