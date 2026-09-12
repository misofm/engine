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
const host = join(root, "hosts/host-web")
const runtime = join(host, "web/stem-store")
const tests = [
  "stem-store-hash-v1.mjs",
  "stem-store-core-v1.mjs",
  "stem-pump-v1.mjs",
]
const PROVENANCE_FIELDS = [
  "schema",
  "artifact",
  "sha256",
  "dependencies",
  "algorithmAuthority",
  "implementation",
  "reason",
]
const DEPENDENCY_FIELDS = ["artifact", "sha256"]
const PRIMARY_ARTIFACT = "incremental-sha256.js"
const PRIMARY_DEPENDENCY = "../hex-lower.js"
const ALGORITHM_AUTHORITY = "NIST FIPS 180-4, SHA-256"
const IMPLEMENTATION = "repository-owned independent JavaScript implementation"
const REASON =
  "Canonical-PCM stems exceed one-shot digest RAM budgets; SHA-256 remains the schema vocabulary and is verifiable by Sui std::hash::sha2_256. No zero-shipped-code claim is made."

// Wall-clock budget assertions in the stem-store test files (cold-open/verify-open latency, the
// fallback-write abort deadline) are opt-in via `--budgets`, so this gate stays hermetic and
// flake-free on the blocking lint path by default; pass `--budgets` (e.g. from a nightly job) to
// exercise the real thresholds too.
const budgetsEnabled = process.argv.includes("--budgets")
let skippedBudgetAsserts = 0

await runOperatorPathSelfTest(root)
await staticChecks(root)
for (const test of tests) {
  skippedBudgetAsserts += runNode(join(host, "tests", test), false, budgetsEnabled)
}

if (process.argv.includes("--self-test")) await runMutationLedger()

process.stdout.write(
  `stem-store-v1 gate: PASS (budget asserts skipped: ${skippedBudgetAsserts})\n`
)

async function staticChecks(repository) {
  await validateSourceProvenance(runtime)

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

  const engineRoots = [
    "crates",
    "hosts/host-native",
    "hosts/host-mobile",
  ]
  for (const relative of engineRoots) {
    for (const file of await sourceFiles(join(repository, relative), [".rs"])) {
      const text = await readFile(file, "utf8")
      assert.doesNotMatch(text, /miso-stems-v1|FileSystemFileHandle|\bOPFS\b/)
    }
  }
}

async function runOperatorPathSelfTest(repository) {
  const runner = join(repository, "scripts/operator/run-stem-store-browser-evals.cjs")
  const result = spawnSync(process.execPath, [runner, "--path-self-test"], {
    cwd: repository,
    encoding: "utf8",
  })
  if (result.status !== 0) {
    throw new Error(`operator path self-test failed:\n${result.stderr ?? ""}`)
  }
  process.stdout.write(result.stdout ?? "")
}

function assertPumpHasNoNetwork(text) {
  assert.doesNotMatch(
    text,
    /\bfetch\s*\(|\bXMLHttpRequest\b|\bWebSocket\b|\bEventSource\b/,
    "the Worker pump must stream only from the verified store"
  )
}

async function runMutationLedger() {
  await runProvenanceSelfTest()

  // STEM_IDENTITY_V1 §4's mandatory open-time length check has four arms.
  // `bytes` is shape-derived and never enters the hash preimage, so these
  // comparisons are all that stand between a lying declaration and a promoted
  // stem. The reopen and fallback-final arms are defense in depth behind the
  // streamed arm: while an earlier arm is intact they never see the lie, so
  // they are pinned cumulatively -- each mutation must trip a strictly later
  // gate than the one before it.
  const streamedArm = {
    search: "      if (bytes !== stem.bytes || streamedHex !== digest) {",
    replace: "      if (streamedHex !== digest) {",
  }
  const reopenArm = {
    search:
      "      if (reopened.bytes !== stem.bytes || reopened.hex !== digest) {",
    replace: "      if (reopened.hex !== digest) {",
  }
  const fallbackFinalArm = {
    search:
      "    if (\n      observed.bytes !== stem.bytes ||\n      observed.hex !== stemDigest(stem.identity)\n    ) {\n      await removeEntry(this.#directory, finalName)",
    replace:
      "    if (observed.hex !== stemDigest(stem.identity)) {\n      await removeEntry(this.#directory, finalName)",
  }
  const verifyOnOpenArm = {
    search:
      "      if (\n        observed.bytes !== stem.bytes ||\n        observed.hex !== stemDigest(stem.identity)\n      ) {\n        await this.#demote(stem.identity)",
    replace:
      "      if (observed.hex !== stemDigest(stem.identity)) {\n        await this.#demote(stem.identity)",
  }
  const mutations = [
    {
      name: "content-key -> per-session store directory",
      file: "web/stem-store/opfs-store.js",
      search:
        '    await this.open()\n    const sessionId = nonemptyText(options.sessionId, "sessionId")',
      replace:
        '    await this.open()\n    this.__mutationRoot ??= this.#directory\n    const mutationSession = encodeURIComponent(String(options.sessionId))\n    this.#directory = await this.__mutationRoot.getDirectoryHandle(`session-${mutationSession}`, { create: true })\n    this.#staging = await this.#directory.getDirectoryHandle("staging", { create: true })\n    const sessionId = nonemptyText(options.sessionId, "sessionId")',
      test: "stem-store-core-v1.mjs",
      expectedFailure: "mix B fetches only its two misses",
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
    {
      name: "disable wedged decoder read deadline",
      file: "web/stem-store/opfs-store.js",
      search:
        '            reader.read(),\n            this.#ingestReadDeadlineMs,\n            "stem.decode.stalled",',
      replace:
        '            reader.read(),\n            60_000,\n            "stem.decode.stalled",',
      test: "stem-store-core-v1.mjs",
      expectedFailure: "wedged decoder escaped its deadline",
    },
    {
      name: "omit AbortSignal from wedged decoder read race",
      file: "web/stem-store/opfs-store.js",
      search:
        '            `Decoded PCM for ${stem.identity} made no progress`,\n            options.signal,\n            StemResolverError',
      replace:
        '            `Decoded PCM for ${stem.identity} made no progress`,\n            undefined,\n            StemResolverError',
      test: "stem-store-core-v1.mjs",
      expectedFailure: "wedged decoder ignored mix-switch abort",
    },
    {
      name: "retain predecessor pin on same-session replacement",
      file: "web/stem-store/session-gate.js",
      search: "if (this.#lease?.sessionId === options.sessionId) {",
      replace: "if (false && this.#lease?.sessionId === options.sessionId) {",
      test: "stem-pump-v1.mjs",
      expectedFailure: "same-session replacement waited on its own predecessor pin lock",
    },
    {
      name: "treat missing index as an empty authoritative index",
      file: "web/stem-store/opfs-store.js",
      search:
        '      if (error?.name === "NotFoundError") {\n        if (!repair) return emptyIndex()\n        return this.#rebuildIndex()\n      }',
      replace:
        '      if (error?.name === "NotFoundError") return emptyIndex()',
      test: "stem-store-core-v1.mjs",
      expectedFailure: "missing crash-only index must adopt a self-verifying final without re-ingest",
    },
    {
      name: "count an opening-session survivor as write-time evictable",
      file: "web/stem-store/opfs-store.js",
      search:
        "        row.pins.length === 0 && !protectedIdentities.has(identity)",
      replace: "        row.pins.length === 0",
      test: "stem-store-core-v1.mjs",
      expectedFailure: "write-time quota accounting counted an opening-session survivor as evictable",
    },
    {
      name: "allow a stale estimate to report zero write-time shortfall",
      file: "web/stem-store/opfs-store.js",
      search: "const shortfallBytes = Math.max(1, estimatedShortfall)",
      replace: "const shortfallBytes = Math.max(0, estimatedShortfall)",
      test: "stem-store-core-v1.mjs",
      expectedFailure: "write-time quota race reported a zero shortfall",
    },
    {
      name: "restore the pre-successor direct fallback final write",
      file: "web/stem-store/opfs-store.js",
      search:
        '        await withDeadline(\n          writable.write(result.value),\n          this.#readDeadlineMs,\n          "storage.write_stalled",\n          `Writing fallback final for ${stem.identity} made no progress`,\n          options.signal\n        )',
      replace: "        await writable.write(result.value)",
      test: "stem-store-core-v1.mjs",
      expectedFailure: "fallback final write ignored mix-switch abort",
    },
    {
      name: "drop the streamed-ingest byte-length check",
      file: "web/stem-store/opfs-store.js",
      edits: [streamedArm],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lying declaration is refused before the pre-promote reopen",
    },
    {
      name: "drop the streamed and pre-promote byte-length checks",
      file: "web/stem-store/opfs-store.js",
      edits: [streamedArm, reopenArm],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lying declaration must never be promoted",
    },
    {
      name: "drop the streamed, pre-promote, and fallback-final byte-length checks",
      file: "web/stem-store/opfs-store.js",
      edits: [streamedArm, reopenArm, fallbackFinalArm],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lying declaration must never survive fallback promotion",
    },
    {
      name: "drop the verify-on-open byte-length check",
      file: "web/stem-store/opfs-store.js",
      edits: [verifyOnOpenArm],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "verify-on-open must demote a lying declaration to a miss",
    },
    {
      name: "fill staging to the stream end before refusing an over-length delivery",
      file: "web/stem-store/opfs-store.js",
      search: "          if (bytes > stem.bytes) {",
      replace: "          if (false) {",
      test: "stem-store-core-v1.mjs",
      expectedFailure:
        "over-length delivery is refused before staging fills past the declaration",
    },
    {
      name: "drop every open-time byte-length check",
      file: "web/stem-store/opfs-store.js",
      edits: [streamedArm, reopenArm, fallbackFinalArm, verifyOnOpenArm],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lying declaration must never survive fallback promotion",
    },
    // Issue #278's opt-in Worker cadence. Two mutations, because "off by default" and "stop
    // interrupts" are two claims and one of them staying green would hide the other. The first is
    // the compatibility claim in particular: nothing about the shipped worker changed for a host
    // that never sends `selfDriving`, and the way to hold that is to make the loop start unasked
    // and require a gate to notice.
    {
      name: "drive the pump loop by default",
      file: "web/stem-store/pcm-pump-worker.js",
      search: "  if (requested === undefined || requested === false) return undefined",
      replace:
        "  if (requested === undefined) return DEFAULT_IDLE_MS\n  if (requested === false) return undefined",
      test: "stem-pump-v1.mjs",
      expectedFailure: "an un-driven worker writes nothing on its own",
    },
    {
      name: "let stop leave the idle sleep pending",
      file: "web/stem-store/pcm-pump-worker.js",
      search:
        "function stopDriving() {\n  selfDrivingIdleMs = undefined\n  driveToken = undefined\n  wakeIdle()\n}",
      replace:
        "function stopDriving() {\n  selfDrivingIdleMs = undefined\n  driveToken = undefined\n}",
      test: "stem-pump-v1.mjs",
      expectedFailure: "stop cancelled the idle timer rather than waiting it out",
    },
  ]

  for (const mutation of mutations) {
    const temporary = await mkdtemp(join(tmpdir(), "miso-stem-store-mutation-"))
    try {
      const mutatedHost = join(temporary, "host-web")
      await cp(host, mutatedHost, { recursive: true })
      const target = join(mutatedHost, mutation.file)
      const source = await readFile(target, "utf8")
      const edits = mutation.edits ?? [
        { search: mutation.search, replace: mutation.replace },
      ]
      let mutated = source
      for (const edit of edits) {
        mutated = replaceExactlyOnce(mutated, edit.search, edit.replace)
      }
      await writeFile(target, mutated)
      runNode(target, true)
      const result = spawnSync(
        process.execPath,
        budgetsEnabled
          ? [join(mutatedHost, "tests", mutation.test), "--budgets"]
          : [join(mutatedHost, "tests", mutation.test)],
        { encoding: "utf8" }
      )
      if (result.status === 0) {
        throw new Error(`mutation stayed green: ${mutation.name}`)
      }
      const failureOutput = `${result.stdout ?? ""}\n${result.stderr ?? ""}`
      if (
        mutation.expectedFailure !== undefined &&
        !failureOutput.includes(mutation.expectedFailure)
      ) {
        throw new Error(
          `mutation went red outside its target gate: ${mutation.name}\n${failureOutput}`
        )
      }
      process.stdout.write(
        `RED: ${mutation.name}${
          mutation.expectedFailure === undefined
            ? ""
            : ` [target: ${mutation.expectedFailure}]`
        }\n`
      )
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

async function validateSourceProvenance(provenanceRoot) {
  const provenancePath = join(provenanceRoot, "incremental-sha256.provenance.json")
  const provenance = JSON.parse(await readFile(provenancePath, "utf8"))
  assert.deepEqual(
    Object.keys(provenance).sort(),
    [...PROVENANCE_FIELDS].sort(),
    "incremental SHA-256 provenance shape changed"
  )
  assert.equal(provenance.schema, 1, "incremental SHA-256 provenance schema changed")
  assert.equal(
    provenance.artifact,
    PRIMARY_ARTIFACT,
    "incremental SHA-256 provenance artifact changed"
  )
  assertSha256(provenance.sha256, "incremental SHA-256 source hash")
  assert.equal(
    provenance.algorithmAuthority,
    ALGORITHM_AUTHORITY,
    "incremental SHA-256 algorithm authority changed"
  )
  assert.equal(
    provenance.implementation,
    IMPLEMENTATION,
    "incremental SHA-256 implementation authority changed"
  )
  assert.equal(provenance.reason, REASON, "incremental SHA-256 provenance reason changed")
  assert.ok(Array.isArray(provenance.dependencies), "incremental SHA-256 dependencies are not an array")
  assert.equal(
    provenance.dependencies.length,
    1,
    "incremental SHA-256 provenance must contain exactly one dependency"
  )
  const [dependency] = provenance.dependencies
  assert.deepEqual(
    Object.keys(dependency).sort(),
    [...DEPENDENCY_FIELDS].sort(),
    "incremental SHA-256 dependency shape changed"
  )
  assert.equal(
    dependency.artifact,
    PRIMARY_DEPENDENCY,
    "incremental SHA-256 dependency artifact changed"
  )
  assertSha256(dependency.sha256, "incremental SHA-256 dependency hash")

  const hasher = await readFile(join(provenanceRoot, PRIMARY_ARTIFACT))
  assert.equal(
    createHash("sha256").update(hasher).digest("hex"),
    provenance.sha256,
    "incremental SHA-256 source provenance moved without a re-pin"
  )
  const helper = await readFile(join(provenanceRoot, PRIMARY_DEPENDENCY))
  assert.equal(
    createHash("sha256").update(helper).digest("hex"),
    dependency.sha256,
    "incremental SHA-256 dependency provenance moved without a re-pin"
  )
}

function assertSha256(value, label) {
  assert.equal(typeof value, "string", `${label} must be a string`)
  assert.match(value, /^[0-9a-f]{64}$/, `${label} must be 64 lowercase hexadecimal characters`)
}

async function runProvenanceSelfTest() {
  const mutations = [
    {
      name: "primary bytes mutated",
      mutate: async (temporary) => {
        const target = join(temporary, PRIMARY_ARTIFACT)
        await writeFile(target, `${await readFile(target, "utf8")}\n`)
      },
      expectedFailure: "incremental SHA-256 source provenance moved without a re-pin",
    },
    {
      name: "helper bytes mutated",
      mutate: async (temporary) => {
        const target = join(temporary, PRIMARY_DEPENDENCY)
        await writeFile(target, `${await readFile(target, "utf8")}\n`)
      },
      expectedFailure: "incremental SHA-256 dependency provenance moved without a re-pin",
    },
    {
      name: "dependency roster removed",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies = []
        })
      },
      expectedFailure: "incremental SHA-256 provenance must contain exactly one dependency",
    },
    {
      name: "extra dependency",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies.push({ artifact: "../extra.js", sha256: "0".repeat(64) })
        })
      },
      expectedFailure: "incremental SHA-256 provenance must contain exactly one dependency",
    },
    {
      name: "renamed dependency",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies[0].artifact = "../renamed-hex-lower.js"
        })
      },
      expectedFailure: "incremental SHA-256 dependency artifact changed",
    },
    {
      name: "different well-formed dependency hash",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies[0].sha256 = "0".repeat(64)
        })
      },
      expectedFailure: "incremental SHA-256 dependency provenance moved without a re-pin",
    },
  ]

  for (const mutation of mutations) {
    const temporaryRoot = await mkdtemp(join(tmpdir(), "miso-stem-store-provenance-"))
    const temporary = join(temporaryRoot, "stem-store")
    try {
      await cp(runtime, temporary, { recursive: true })
      await cp(join(host, "web/hex-lower.js"), join(temporaryRoot, "hex-lower.js"))
      await mutation.mutate(temporary)
      await assert.rejects(
        validateSourceProvenance(temporary),
        new RegExp(escapeRegExp(mutation.expectedFailure))
      )
      process.stdout.write(`RED: provenance ${mutation.name}\n`)
    } finally {
      await rm(temporaryRoot, { recursive: true, force: true })
    }
  }
}

async function mutateProvenance(provenanceRoot, mutate) {
  const path = join(provenanceRoot, "incremental-sha256.provenance.json")
  const provenance = JSON.parse(await readFile(path, "utf8"))
  mutate(provenance)
  await writeFile(path, `${JSON.stringify(provenance, null, 2)}\n`)
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
}

function replaceExactlyOnce(source, search, replacement) {
  const first = source.indexOf(search)
  if (first < 0 || source.indexOf(search, first + search.length) >= 0) {
    throw new Error(`mutation anchor does not occur exactly once: ${search}`)
  }
  return source.slice(0, first) + replacement + source.slice(first + search.length)
}

function runNode(file, checkOnly = false, budgets = false) {
  const args = checkOnly ? ["--check", file] : budgets ? [file, "--budgets"] : [file]
  const result = spawnSync(process.execPath, args, {
    cwd: root,
    encoding: "utf8",
    stdio: checkOnly ? "pipe" : ["inherit", "pipe", "inherit"],
  })
  if (result.status !== 0) {
    throw new Error(
      `${checkOnly ? "syntax check" : "eval"} failed for ${file}:\n${result.stderr ?? ""}`
    )
  }
  if (checkOnly) return 0
  process.stdout.write(result.stdout ?? "")
  const skipped = /budget asserts skipped: (\d+)/.exec(result.stdout ?? "")
  return skipped ? Number(skipped[1]) : 0
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
