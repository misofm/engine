#!/usr/bin/env node

import assert from "node:assert/strict"
import { createHash } from "node:crypto"
import {
  cp,
  mkdtemp,
  readFile,
  readdir,
  rm,
  writeFile,
} from "node:fs/promises"
import { tmpdir } from "node:os"
import { join, resolve } from "node:path"
import { spawnSync } from "node:child_process"
import { fileURLToPath } from "node:url"

const root = resolve(fileURLToPath(new URL("..", import.meta.url)))
const host = join(root, "hosts/miso-engine-host-web")
const runtime = join(host, "web/stem-store")
const tests = [
  "stem-store-hash-v1.mjs",
  "stem-store-core-v1.mjs",
  "stem-pump-v1.mjs",
]

await staticChecks(root)
for (const test of tests) runNode(join(host, "tests", test))

if (process.argv.includes("--self-test")) await runMutationLedger()

process.stdout.write("stem-store-v1 gate: PASS\n")

async function staticChecks(repository) {
  const provenancePath = join(runtime, "incremental-sha256.provenance.json")
  const provenance = JSON.parse(await readFile(provenancePath, "utf8"))
  const hasher = await readFile(join(runtime, provenance.artifact))
  assert.equal(
    createHash("sha256").update(hasher).digest("hex"),
    provenance.sha256,
    "incremental SHA-256 source provenance moved without a re-pin"
  )

  const files = await sourceFiles(runtime)
  const allRuntime = (
    await Promise.all(files.map((file) => readFile(file, "utf8")))
  ).join("\n")
  assert.doesNotMatch(
    allRuntime,
    /decodeAudioData\s*\(/,
    "the stem adapter must never use the platform audio decoder"
  )
  assert.doesNotMatch(
    allRuntime,
    /crypto\.subtle\.digest\s*\(/,
    "canonical PCM hashing must remain incremental"
  )

  const pump = ["pcm-pump.js", "pcm-pump-worker.js"]
    .map((name) => files.find((file) => file.endsWith(name)))
    .map((file) => readFile(file, "utf8"))
  const pumpText = (await Promise.all(pump)).join("\n")
  assertPumpHasNoNetwork(pumpText)

  const storeSource = await readFile(join(runtime, "opfs-store.js"), "utf8")
  assert.match(storeSource, /access\?\.mode === "read-only"/)
  const modeProbe = storeSource.slice(
    storeSource.indexOf("export async function detectSharedReadOnlyMode"),
    storeSource.indexOf("function normalizeRequirements")
  )
  assert.doesNotMatch(
    modeProbe,
    /NoModificationAllowedError|InvalidStateError|error\?*\.name/,
    "access-handle contention detection must not match an engine-specific error name"
  )
  assert.match(storeSource, /await stagingHandle\.move\(this\.#directory, finalName\)/)
  assert.match(storeSource, /const observed = await this\.#hashOpenFile/)
  const workerClient = await readFile(join(runtime, "worker-client.js"), "utf8")
  assert.match(workerClient, /new WorkerConstructor/)
  const worker = await readFile(join(runtime, "pcm-pump-worker.js"), "utf8")
  assert.match(worker, /type: "session-error"/)

  const engineRoots = ["crates", "hosts/miso-engine-host-native", "hosts/miso-engine-host-mobile"]
  for (const relative of engineRoots) {
    for (const file of await sourceFiles(join(repository, relative), [".rs"])) {
      const text = await readFile(file, "utf8")
      assert.doesNotMatch(text, /miso-stems-v1|FileSystemFileHandle|\bOPFS\b/)
    }
  }
}

function assertPumpHasNoNetwork(text) {
  assert.doesNotMatch(
    text,
    /\bfetch\s*\(|\bXMLHttpRequest\b|\bWebSocket\b|\bEventSource\b/,
    "the Worker pump must stream only from the verified store"
  )
}

async function runMutationLedger() {
  const mutations = [
    {
      name: "content-key -> session-shaped key",
      file: "web/stem-store/identity.js",
      search: "return `sha256-${stemDigest(identity)}`",
      replace: "return `sha256-${stemDigest(identity)}-session-keyed`",
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "remove Web Lock single-flight",
      file: "web/stem-store/opfs-store.js",
      search:
        'if (typeof this.#locks?.request === "function") {\n      return this.#locks.request(name, { mode: "exclusive", signal }, async () => {',
      replace:
        'if (false) {\n      return this.#locks.request(name, { mode: "exclusive", signal }, async () => {',
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "skip pre-promote staging verification",
      file: "web/stem-store/opfs-store.js",
      search:
        "const reopened = await this.#hashFile(handle, {\n        signal: options.signal,\n        onChunk: options.onProgress,\n        stage: \"verified\",\n        identity: stem.identity,\n      })",
      replace: "const reopened = { bytes: stem.bytes, hex: digest }",
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "insert index before promote verification",
      file: "web/stem-store/opfs-store.js",
      search:
        "await this.#promote(stem, handle, stagingName, options)\n      await this.#indexPromoted(stem)",
      replace:
        "await this.#indexPromoted(stem)\n      await this.#promote(stem, handle, stagingName, options)",
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "open interaction gate while stems are missing",
      file: "web/stem-store/session-gate.js",
      search: 'this.#state = "loading"',
      replace: 'this.#state = "interactive"',
      test: "stem-pump-v1.mjs",
    },
    {
      name: "trust warm filename without verify-on-open",
      file: "web/stem-store/opfs-store.js",
      search:
        "const observed = await this.#hashOpenFile(handle, stem, onProgress, signal)",
      replace:
        "const observed = { bytes: stem.bytes, hex: stemDigest(stem.identity) }",
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "match one access-handle contention error name",
      file: "web/stem-store/opfs-store.js",
      search:
        '    return access?.mode === "read-only"\n  } catch {\n    // Do not inspect the error name: WebKit and Blink/Gecko disagree on it.\n    return false',
      replace:
        '    return access?.mode === "read-only"\n  } catch (error) {\n    if (error?.name === "NoModificationAllowedError") return false\n    throw error',
      test: "stem-store-core-v1.mjs",
    },
  ]

  for (const mutation of mutations) {
    const temporary = await mkdtemp(join(tmpdir(), "miso-stem-store-mutation-"))
    try {
      const mutatedHost = join(temporary, "miso-engine-host-web")
      await cp(host, mutatedHost, { recursive: true })
      const target = join(mutatedHost, mutation.file)
      const source = await readFile(target, "utf8")
      const mutated = replaceExactlyOnce(source, mutation.search, mutation.replace)
      await writeFile(target, mutated)
      runNode(target, true)
      const result = spawnSync(
        process.execPath,
        [join(mutatedHost, "tests", mutation.test)],
        { encoding: "utf8" }
      )
      if (result.status === 0) {
        throw new Error(`mutation stayed green: ${mutation.name}`)
      }
      process.stdout.write(`RED: ${mutation.name}\n`)
    } finally {
      await rm(temporary, { recursive: true, force: true })
    }
  }

  const pumpSource = await readFile(join(runtime, "pcm-pump.js"), "utf8")
  assert.throws(
    () => assertPumpHasNoNetwork(`${pumpSource}\nglobalThis.fetch("/mutation")\n`),
    /must stream only from the verified store/
  )
  process.stdout.write("RED: pump network API tripwire\n")
}

function replaceExactlyOnce(source, search, replacement) {
  const first = source.indexOf(search)
  if (first < 0 || source.indexOf(search, first + search.length) >= 0) {
    throw new Error(`mutation anchor does not occur exactly once: ${search}`)
  }
  return source.slice(0, first) + replacement + source.slice(first + search.length)
}

function runNode(file, checkOnly = false) {
  const args = checkOnly ? ["--check", file] : [file]
  const result = spawnSync(process.execPath, args, {
    cwd: root,
    encoding: "utf8",
    stdio: checkOnly ? "pipe" : "inherit",
  })
  if (result.status !== 0) {
    throw new Error(
      `${checkOnly ? "syntax check" : "eval"} failed for ${file}:\n${result.stderr ?? ""}`
    )
  }
}

async function sourceFiles(directory, extensions = [".js"]) {
  const output = []
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name)
    if (entry.isDirectory()) output.push(...(await sourceFiles(path, extensions)))
    else if (extensions.some((extension) => entry.name.endsWith(extension))) output.push(path)
  }
  return output
}
