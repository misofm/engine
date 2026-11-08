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
const PRIMARY_ARTIFACT = "incremental-blake3.js"
const PRIMARY_DEPENDENCY = "../hex-lower.js"
const ALGORITHM_AUTHORITY = "BLAKE3 specification, 32-byte default output"
const IMPLEMENTATION =
  "repository-owned independent JavaScript implementation, and the blake3 crate's WebAssembly SIMD build (hosts/blake3-web, miso-engine-v1-blake3.simd128.wasm) once prepareBlake3 loads it"
const REASON =
  "Canonical-PCM stems exceed one-shot digest RAM budgets, so the browser ships an incremental BLAKE3-256 implementation. Issue #2111 adds the WebAssembly SIMD module for speed and keeps this JavaScript as its fallback. No zero-shipped-code claim is made."

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
  ]
  for (const relative of engineRoots) {
    for (const file of await sourceFiles(join(repository, relative), [".rs"])) {
      const text = await readFile(file, "utf8")
      assert.doesNotMatch(text, /miso-stems-blake3-v1|FileSystemFileHandle|\bOPFS\b/)
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
  // The cheap size pre-check is the first verify-on-open length arm; the
  // comparison after the hash is the second. Both go, or neither is tested.
  const verifyOnOpenPreCheck = {
    search: "    if (file.size !== stem.bytes) {",
    replace: "    if (false) {",
  }
  const verifyOnOpenArm = {
    search:
      "    if (\n      observed.bytes !== stem.bytes ||\n      observed.hex !== stemDigest(stem.identity)\n    ) {\n      await this.#demote(stem.identity, hold)",
    replace:
      "    if (observed.hex !== stemDigest(stem.identity)) {\n      await this.#demote(stem.identity, hold)",
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
      edits: [
        {
          search:
            'if (typeof this.#locks?.request === "function") {\n      return this.#locks.request(name, { mode: "exclusive", signal }, async () => {',
          replace:
            'if (false) {\n      return this.#locks.request(name, { mode: "exclusive", signal }, async () => {',
        },
        {
          search: 'if (typeof this.#locks?.request === "function") {\n      return this.#holdLock(\n        name,',
          replace: 'if (false) {\n      return this.#holdLock(\n        name,',
        },
      ],
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
        "await this.#promote(stem, handle, stagingName, options, hold)\n      await this.#indexPromoted(stem)",
      replace:
        "await this.#indexPromoted(stem)\n      await this.#promote(stem, handle, stagingName, options, hold)",
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
        "const observed = await this.#hashOpenFile(file, stem, onProgress, signal).catch(unreadable)",
      replace:
        "const observed = { bytes: stem.bytes, hex: stemDigest(stem.identity) }",
      test: "stem-store-core-v1.mjs",
    },
    {
      name: "match one access-handle contention error name",
      file: "web/stem-store/opfs-store.js",
      search:
        '    return access?.mode === "read-only"\n  } catch {\n    // Do not inspect the error name: WebKit and Blink disagree on it.\n    return false',
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
      name: "commit a superseded open after its predecessor close",
      file: "web/stem-store/session-gate.js",
      search:
        "        await displaced.close()\n        opening.signal.throwIfAborted()\n",
      replace: "        await displaced.close()\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open committed (same-session open, predecessor close)",
    },
    {
      name: "commit a superseded open once its resume settles",
      file: "web/stem-store/session-gate.js",
      search:
        "      // The abort can land after `resumed` settles and before this line runs.\n      opening.signal.throwIfAborted()\n",
      replace: "",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open committed (same-session open, after the resume settles",
    },
    {
      name: "wait out a superseded open's resume",
      file: "web/stem-store/session-gate.js",
      search: "const resumed = await abortable(resumeOutcome, opening.signal)",
      replace: "const resumed = await resumeOutcome",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open never settled (same-session open, resume wait)",
    },
    {
      name: "wait on the resume of an open aborted before the wait began",
      file: "web/stem-store/session-gate.js",
      search: "  if (signal.aborted) return Promise.reject(signal.reason)\n  let abort\n",
      replace: "  let abort\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open never settled (same-session open, from ready)",
    },
    {
      name: "leave the gate loading after a throwing resume aborts the open in flight",
      file: "web/stem-store/session-gate.js",
      search: '      if (this.#state === "loading") this.#state = "refused"\n',
      replace: "",
      test: "stem-pump-v1.mjs",
      expectedFailure: "a refused open left the gate loading with no open in flight",
    },
    {
      name: "let close() write idle after its await",
      file: "web/stem-store/session-gate.js",
      search: "    await closing\n  }\n",
      replace: '    await closing\n    this.#state = "idle"\n  }\n',
      test: "stem-pump-v1.mjs",
      expectedFailure: "close() reported idle while a later open was loading",
    },
    {
      name: "stop forwarding the caller's signal once the lease is granted",
      file: "web/stem-store/session-gate.js",
      search: "        onPinning,\n      })\n",
      replace: "        onPinning,\n      })\n      removeParentAbort()\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open never settled (caller signal, resume wait)",
    },
    {
      name: "check gate ownership by identity instead of the opening's signal",
      file: "web/stem-store/session-gate.js",
      edits: [
        {
          search:
            "      // The abort can land after `resumed` settles and before this line runs.\n      opening.signal.throwIfAborted()\n",
          replace:
            "      // The abort can land after `resumed` settles and before this line runs.\n      if (this.#opening !== opening) throw new DOMException(\"x\", \"AbortError\")\n",
        },
        {
          search: "        await displaced.close()\n        opening.signal.throwIfAborted()\n",
          replace:
            "        await displaced.close()\n        if (this.#opening !== opening) throw new DOMException(\"x\", \"AbortError\")\n",
        },
      ],
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open committed (caller signal, predecessor close)",
    },
    {
      name: "refuse a throwing resume only when no lease is current",
      file: "web/stem-store/session-gate.js",
      search: '      if (this.#state === "loading") this.#state = "refused"\n',
      replace: '      if (this.#lease === undefined) this.#state = "refused"\n',
      test: "stem-pump-v1.mjs",
      expectedFailure: "a refused open left the gate loading behind a current lease",
    },
    // Issue #2372: a resolved close() means a released pin. A lease closed twice shares one release;
    // gate.close() waits for the current lease, a replaced predecessor still closing, and every open
    // from the store's pin phase on (and only from then: before it an aborted open never takes a pin);
    // and a failed release still rejects the close.
    {
      name: "let a second lease close() resolve before the first releases the pin",
      file: "web/stem-store/opfs-store.js",
      search: "  close() {\n    this.#closing ??= this.#release()\n",
      replace:
        "  close() {\n    if (this.#closing !== undefined) return Promise.resolve()\n    this.#closing ??= this.#release()\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "a second close() resolved while the first still held the pin",
    },
    {
      name: "let gate.close() wait only for the current lease",
      file: "web/stem-store/session-gate.js",
      search: "    await Promise.allSettled([closing, ...this.#releasing])\n",
      replace: "    await Promise.allSettled([closing])\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "an aborted open's lease still held its pin (aborted by close())",
    },
    {
      name: "never report a session open's pin phase",
      file: "web/stem-store/opfs-store.js",
      search: "    throwIfAborted(options.signal)\n    options.onPinning?.()\n",
      replace: "    throwIfAborted(options.signal)\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "an aborted open's lease still held its pin (aborted by close())",
    },
    {
      name: "report a session open's pin phase only once the pin is written",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search: "    throwIfAborted(options.signal)\n    options.onPinning?.()\n",
          replace: "    throwIfAborted(options.signal)\n",
        },
        {
          search: '    options.onProgress?.({ stage: "ready", stems: stems.length })\n',
          replace: '    options.onPinning?.()\n    options.onProgress?.({ stage: "ready", stems: stems.length })\n',
        },
      ],
      test: "stem-pump-v1.mjs",
      expectedFailure: "gate.close() resolved while an open in its pin write held the pin lock",
    },
    {
      name: "close a replaced predecessor without gate.close() waiting for it",
      file: "web/stem-store/session-gate.js",
      search: "        await this.#track(replaced.close())\n",
      replace: "        await replaced.close()\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "gate.close() resolved while a replaced predecessor still held its pin",
    },
    {
      name: "let gate.close() wait for every open from its start",
      file: "web/stem-store/session-gate.js",
      search: "      return await this.#open(options, () => this.#track(settled))\n",
      replace: "      this.#track(settled)\n      return await this.#open(options, () => {})\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "gate.close() waited on an aborted open's wedged getDirectory",
    },
    {
      name: "let gate.close() hide a failed release of the current lease",
      file: "web/stem-store/session-gate.js",
      search: "    await Promise.allSettled([closing, ...this.#releasing])\n    await closing\n",
      replace: "    await Promise.allSettled([closing, ...this.#releasing])\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "gate.close() hid a failed pin release",
    },
    {
      name: "track an open's settlement on the caller's promise",
      file: "web/stem-store/session-gate.js",
      search:
        "  async open(options) {\n    // Settlement is tracked on a private promise, never on the caller's, so a\n    // rejected open the caller drops is still reported as unhandled.\n    let settle\n    const settled = new Promise((resolve) => (settle = resolve))\n    try {\n      return await this.#open(options, () => this.#track(settled))\n    } finally {\n      settle()\n    }\n  }\n",
      replace:
        "  open(options) {\n    const opened = this.#open(options, () => this.#track(opened))\n    return opened\n  }\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the dropped open's unhandled rejection (after the pin phase)",
    },
    // Issue #2381: every OPFS step under the index lock is deadlined and fails typed; a final is
    // removed only inside a hold of its stem lock, which outlives the removal; a stalled or failed
    // read demotes only a stem no other session pins; and an aborted open evicts nothing once its
    // abort is seen. One entry per guard.
    {
      name: "put an eviction's raw removal back under the index lock",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      await this.#mutateIndex((index) => {\n        throwIfAborted(signal)\n",
          replace: "      await this.#mutateIndex(async (index) => {\n        throwIfAborted(signal)\n",
        },
        {
          search:
            "        delete index.stems[identity]\n        evicted = true\n      }, signal)\n      if (evicted) {\n        await this.#removeFinal(identity, hold, (removal) => abortable(removal, signal))\n      }\n",
          replace: "        await removeEntry(this.#directory, stemFileName(identity))\n        delete index.stems[identity]\n        evicted = true\n      }, signal)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lease close waited on an eviction's wedged removal",
    },
    {
      name: "open the index file with no deadline",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      const handle = await withDeadline(\n        this.#directory.getFileHandle(INDEX_FILE),\n        this.#readDeadlineMs,\n        \"storage.read_stalled\",\n        \"Opening the stem index made no progress\"\n      )\n",
          replace: "      const handle = await this.#directory.getFileHandle(INDEX_FILE)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lease close waited on a wedged index open",
    },
    {
      name: "rebuild the index after a stalled index read",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    } catch (error) {\n      if (error instanceof StemStoreError && error.code === \"storage.read_stalled\") throw error\n      if (error?.name === \"NotFoundError\") {\n",
          replace: "    } catch (error) {\n      if (error?.name === \"NotFoundError\") {\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a stalled index read rebuilt the index and dropped a live pin",
    },
    {
      name: "list the store for an index rebuild with no deadline",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        const step = await withDeadline(\n          iterator.next(),\n          this.#readDeadlineMs,\n          \"storage.read_stalled\",\n          \"Listing the stem store made no progress\"\n        )\n",
          replace: "        const step = await iterator.next()\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lease close waited on a rebuild's wedged listing",
    },
    {
      name: "remove an unverifiable final in an index rebuild with no deadline",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    const remove = (identity, name) =>\n      this.#removeFinal(identity, undefined, (removal) =>\n        withDeadline(\n          removal,\n          this.#readDeadlineMs,\n          \"storage.remove_stalled\",\n          `Removing ${name} made no progress`\n        )\n      )\n",
          replace: "    const remove = (identity, name) => this.#removeFinal(identity, undefined, (removal) => removal)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a lease close waited on a rebuild's wedged removal",
    },
    {
      name: "remove a good final whose hash stalled in a rebuild",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "          if (error instanceof StemStoreError && error.code === \"storage.read_stalled\") throw error\n          observed = undefined\n",
          replace: "          observed = undefined\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a rebuild removed a good final whose hash stalled",
    },
    {
      name: "let an eviction's wait for its removal ignore the open's abort",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        await this.#removeFinal(identity, hold, (removal) => abortable(removal, signal))\n",
          replace: "        await this.#removeFinal(identity, hold, (removal) => removal)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "an eviction's wait for its removal ignored the open's abort",
    },
    {
      name: "remove an evicted final outside the hold",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        await this.#removeFinal(identity, hold, (removal) => abortable(removal, signal))\n",
          replace: "        await abortable(removeEntry(this.#directory, stemFileName(identity)), signal)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "an open verified a final that an eviction was still removing (another tab)",
    },
    {
      name: "release a stem lock as soon as its work settles",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      void settled(hold.fences).then(release)\n",
          replace: "      release()\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a verify ran while a demote's delete of its final was unsettled",
    },
    {
      name: "remove a demoted final outside the hold",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    await this.#removeFinal(identity, hold, (removal) =>\n      withDeadline(\n        removal,\n        this.#readDeadlineMs,\n        \"storage.remove_stalled\",\n        `Removing ${identity} made no progress`\n      )\n    ).catch(() => {})\n",
          replace: "    await withDeadline(\n      removeEntry(this.#directory, stemFileName(identity)),\n      this.#readDeadlineMs,\n      \"storage.remove_stalled\",\n      `Removing ${identity} made no progress`\n    ).catch(() => {})\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a verify ran while a demote's delete of its final was unsettled",
    },
    {
      name: "let a demotion wait on its removal with no deadline",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    await this.#removeFinal(identity, hold, (removal) =>\n      withDeadline(\n        removal,\n        this.#readDeadlineMs,\n        \"storage.remove_stalled\",\n        `Removing ${identity} made no progress`\n      )\n    ).catch(() => {})\n",
          replace: "    await this.#removeFinal(identity, hold, (removal) => removal).catch(() => {})\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a demote awaited a wedged delete without a deadline",
    },
    {
      name: "leave a fallback writable's late abort and removal unfenced",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        this.#fence(\n          hold,\n          abortFallbackWritable(",
          replace: "        void (\n          hold,\n          abortFallbackWritable(",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a fallback cleanup released the stem lock before its writable abort settled",
    },
    {
      name: "leave a fallback final's immediate removal unfenced",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      this.#fence(hold, removeEntry(this.#directory, finalName))\n",
          replace: "      void removeEntry(this.#directory, finalName).catch(() => {})\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a fallback cleanup released the stem lock before its immediate removal settled",
    },
    {
      name: "fence a holdless removal into whichever hold of this store is live",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    const hold = { identity, fences: new Set() }\n",
          replace: "    const hold = { identity, fences: new Set() }\n    ;(this.__live ??= new Map()).set(identity, hold)\n",
        },
        {
          search:
            "      void settled(hold.fences).then(release)\n",
          replace: "      void settled(hold.fences).then(() => {\n        if (this.__live?.get(identity) === hold) this.__live.delete(identity)\n        release()\n      })\n",
        },
        {
          search:
            "  async #removeFinal(identity, hold, wait) {\n",
          replace: "  async #removeFinal(identity, hold, wait) {\n    hold ??= this.__live?.get(identity)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a demotion with no hold removed a final another hold of this store had (Web Locks)",
    },
    {
      name: "let a holdless removal release its stem lock before the removal settles",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "(own) => this.#removeFinal(identity, own, wait), true)",
          replace: "() => this.#removeFinal(identity, { fences: new Set() }, wait), true)",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a rebuild released a final's stem lock before its removal settled",
    },
    {
      name: "remove a final whose stem lock another holder has",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    const release = await this.#acquireStemLock(identity, signal, ifAvailable)\n    if (release === null) return false\n",
          replace: "    const release = (await this.#acquireStemLock(identity, signal, ifAvailable)) ?? (() => {})\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a final was removed while another holder had its stem lock",
    },
    {
      name: "remove swept debris without its stem lock",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        await this.#removeFinal(identity, undefined, (removal) =>\n          this.#sweepStep(removal, budgetEnd, `Removing ${name}`)\n        )\n",
          replace: "        await this.#sweepStep(removeEntry(this.#directory, name), budgetEnd, `Removing ${name}`)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a final was removed while another holder had its stem lock",
    },
    {
      name: "remove a rebuild's unverifiable final without its stem lock",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    const remove = (identity, name) =>\n      this.#removeFinal(identity, undefined, (removal) =>\n",
          replace: "    const remove = (identity, name) =>\n      ((identity, _hold, wait) => wait(removeEntry(this.#directory, stemFileName(identity))))(identity, undefined, (removal) =>\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a rebuild released a final's stem lock before its removal settled",
    },
    {
      name: "write a final while this hold's own removal of it is unsettled",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    await settled(hold.fences)\n    if (typeof stagingHandle.move",
          replace: "    if (typeof stagingHandle.move",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a hold wrote a final while its own removal of that final was unsettled",
    },
    {
      name: "let the local stem lock wait ignore its signal",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      await abortable(previous ?? Promise.resolve(), signal)\n",
          replace: "      await previous\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a wait for a removing stem lock ignored its abort (no Web Locks)",
    },
    {
      name: "leave an abandoned local stem lock place blocking the queue",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      // An abandoned place passes straight through: the queue behind it waits only for `previous`.\n      open()\n      throw error\n",
          replace: "      throw error\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "the stem lock stayed taken after the removal settled (no Web Locks)",
    },
    {
      name: "never drop a finished local stem lock place",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    void tail.then(() => {\n      if (this.#localLockTails.get(name) === tail) this.#localLockTails.delete(name)\n    })\n",
          replace: "",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a demotion with no hold left a final whose lock was free (no Web Locks)",
    },
    {
      name: "let a local ifAvailable stem lock request wait",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    if (ifAvailable && previous !== undefined) return null\n",
          replace: "",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a read failure's demotion waited on another hold (no Web Locks)",
    },
    {
      name: "demote an unreadable stem another session pins",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      if (row !== undefined && row.pins.some((pin) => pin !== ownPin)) return\n",
          replace: "",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a stalled verify demoted a pinned stem",
    },
    {
      name: "let a stalled verify-on-open keep an unpinned final",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      await this.#demoteUnreadable(stem.identity, hold, undefined, error)\n      return false\n",
          replace: "      throw error\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a stalling unpinned final never healed (open)",
    },
    {
      name: "let the reading session's own pin protect its unreadable stem",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "row.pins.some((pin) => pin !== ownPin)",
          replace: "row.pins.length !== 0",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a demotion with no hold left a final whose lock was free (Web Locks)",
    },
    {
      name: "demote on any read() failure",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      else await this.#demoteUnreadable(identity, undefined, options.ownPin, error).catch(() => {})\n",
          replace: "      else await this.#demote(identity).catch(() => {})\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a stalled read demoted a stem another session pins",
    },
    {
      name: "let a stalled verify() keep an unpinned final",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        await this.#demoteUnreadable(identity, hold, undefined, error)\n        return false\n",
          replace: "        throw error\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a stalled verify() kept an unpinned final it cannot read",
    },
    {
      name: "demote when verify-on-open's lastUsedAt write fails",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    await this.#mutateIndex((latest) => {\n      const row = latest.stems[stem.identity]\n      if (row !== undefined) row.lastUsedAt = this.#now()\n    })\n    return true\n",
          replace: "    try {\n      await this.#mutateIndex((latest) => {\n        const row = latest.stems[stem.identity]\n        if (row !== undefined) row.lastUsedAt = this.#now()\n      })\n    } catch (error) {\n      return unreadable(error)\n    }\n    return true\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a failed lastUsedAt write demoted the stem",
    },
    {
      name: "give an open's evictions no signal",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    await this.#preflight(missing, protectedIdentities, options.signal)\n",
          replace: "    await this.#preflight(missing, protectedIdentities)\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "an eviction's wait for its removal ignored the open's abort",
    },
    {
      name: "queue an eviction on the victim's stem lock without the open's signal",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "    return this.#withStemLock(identity, signal, async (hold) => {\n      let evicted = false\n",
          replace: "    return this.#withStemLock(identity, undefined, async (hold) => {\n      let evicted = false\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "the aborted open waited for the stem lock holder",
    },
    {
      name: "queue an eviction on the index lock without the open's signal",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "        evicted = true\n      }, signal)\n",
          replace: "        evicted = true\n      })\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "the aborted open waited for the index lock holder",
    },
    {
      name: "skip an eviction's signal check after its index read",
      file: "web/stem-store/opfs-store.js",
      edits: [
        {
          search:
            "      await this.#mutateIndex((index) => {\n        throwIfAborted(signal)\n",
          replace: "      await this.#mutateIndex((index) => {\n",
        },
      ],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "an aborted open evicted a stem (index read)",
    },
    {
      name: "omit AbortSignal from the session pin lock wait",
      file: "web/stem-store/opfs-store.js",
      search: 'this.#holdLock(this.#sessionPinLockName(pin), { mode: "exclusive", signal })',
      replace: 'this.#holdLock(this.#sessionPinLockName(pin), { mode: "exclusive" })',
      test: "stem-pump-v1.mjs",
      expectedFailure: "the aborted open never settled (same-session open, queued on the pin)",
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
      name: "hash a wrong-size stem before refusing it at open",
      file: "web/stem-store/opfs-store.js",
      edits: [verifyOnOpenPreCheck],
      test: "stem-store-core-v1.mjs",
      expectedFailure: "a truncated stem was hashed before its size refused it",
    },
    {
      name: "drop the verify-on-open byte-length check",
      file: "web/stem-store/opfs-store.js",
      edits: [verifyOnOpenPreCheck, verifyOnOpenArm],
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
      edits: [
        streamedArm,
        reopenArm,
        fallbackFinalArm,
        verifyOnOpenPreCheck,
        verifyOnOpenArm,
      ],
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
        "function leaveRunning(running) {\n  running.drive = undefined\n  running.idleWake?.()\n",
      replace: "function leaveRunning(running) {\n  running.drive = undefined\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "stop cancelled the idle timer rather than waiting it out",
    },
    // Issue #2263: a stopped pump refuses work, and a failed Worker session drives nothing. Six
    // mutations for six claims: the pump's two refusals, the Worker's `failed` phase, the failure
    // ending the cadence (the drive-path failure ends its own loop, so only the message-path test
    // can see that one), `selfDriving` refused before any ring engages, and a cadence failure
    // recorded inside the message tail.
    {
      name: "let a stopped pump seek",
      file: "web/stem-store/pcm-pump.js",
      search: "    this.#refuseIfStopped()\n    const target",
      replace: "    const target",
      test: "stem-pump-v1.mjs",
      expectedFailure: "a stopped pump accepted a seek",
    },
    {
      name: "let a stopped pump report a pass",
      file: "web/stem-store/pcm-pump.js",
      search: "  async pumpUntilFull() {\n    this.#refuseIfStopped()\n",
      replace: "  async pumpUntilFull() {\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "a stopped pump reported a pass",
    },
    {
      name: "leave a failed Worker session running",
      file: "web/stem-store/pcm-pump-worker.js",
      search:
        "    leaveRunning(failing)\n    session = { kind: \"failed\", error: record, lease: failing.lease }\n",
      replace: "",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the refusal must come from the failed session, not from a pump it still held",
    },
    {
      name: "fail a Worker session without ending its cadence",
      file: "web/stem-store/pcm-pump-worker.js",
      search: "    leaveRunning(failing)\n",
      replace: "",
      test: "stem-pump-v1.mjs",
      expectedFailure: "a message-path failure left the cadence sleeping",
    },
    {
      name: "check selfDriving after the pump engages its rings",
      file: "web/stem-store/pcm-pump-worker.js",
      edits: [
        { search: "    const idleMs = requestedIdleMs(message.selfDriving)\n", replace: "" },
        {
          search: "      idleMs,\n      drive: undefined,\n",
          replace: "      idleMs: requestedIdleMs(message.selfDriving),\n      drive: undefined,\n",
        },
      ],
      test: "stem-pump-v1.mjs",
      expectedFailure: "a refused selfDriving option left a ring engaged",
    },
    {
      name: "record a cadence failure outside the message tail",
      file: "web/stem-store/pcm-pump-worker.js",
      search: "    reportSessionError(error)\n    return undefined\n",
      replace: "    setTimeout(() => reportSessionError(error), 5)\n    return undefined\n",
      test: "stem-pump-v1.mjs",
      expectedFailure: "the failed pass was recorded before the next message ran, and only once",
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
  const provenancePath = join(provenanceRoot, "incremental-blake3.provenance.json")
  const provenance = JSON.parse(await readFile(provenancePath, "utf8"))
  assert.deepEqual(
    Object.keys(provenance).sort(),
    [...PROVENANCE_FIELDS].sort(),
    "incremental BLAKE3 provenance shape changed"
  )
  assert.equal(provenance.schema, 1, "incremental BLAKE3 provenance schema changed")
  assert.equal(
    provenance.artifact,
    PRIMARY_ARTIFACT,
    "incremental BLAKE3 provenance artifact changed"
  )
  assertSha256(provenance.sha256, "incremental BLAKE3 source hash")
  assert.equal(
    provenance.algorithmAuthority,
    ALGORITHM_AUTHORITY,
    "incremental BLAKE3 algorithm authority changed"
  )
  assert.equal(
    provenance.implementation,
    IMPLEMENTATION,
    "incremental BLAKE3 implementation authority changed"
  )
  assert.equal(provenance.reason, REASON, "incremental BLAKE3 provenance reason changed")
  assert.ok(Array.isArray(provenance.dependencies), "incremental BLAKE3 dependencies are not an array")
  assert.equal(
    provenance.dependencies.length,
    1,
    "incremental BLAKE3 provenance must contain exactly one dependency"
  )
  const [dependency] = provenance.dependencies
  assert.deepEqual(
    Object.keys(dependency).sort(),
    [...DEPENDENCY_FIELDS].sort(),
    "incremental BLAKE3 dependency shape changed"
  )
  assert.equal(
    dependency.artifact,
    PRIMARY_DEPENDENCY,
    "incremental BLAKE3 dependency artifact changed"
  )
  assertSha256(dependency.sha256, "incremental BLAKE3 dependency hash")

  const hasher = await readFile(join(provenanceRoot, PRIMARY_ARTIFACT))
  assert.equal(
    createHash("sha256").update(hasher).digest("hex"),
    provenance.sha256,
    "incremental BLAKE3 source provenance moved without a re-pin"
  )
  const helper = await readFile(join(provenanceRoot, PRIMARY_DEPENDENCY))
  assert.equal(
    createHash("sha256").update(helper).digest("hex"),
    dependency.sha256,
    "incremental BLAKE3 dependency provenance moved without a re-pin"
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
      expectedFailure: "incremental BLAKE3 source provenance moved without a re-pin",
    },
    {
      name: "helper bytes mutated",
      mutate: async (temporary) => {
        const target = join(temporary, PRIMARY_DEPENDENCY)
        await writeFile(target, `${await readFile(target, "utf8")}\n`)
      },
      expectedFailure: "incremental BLAKE3 dependency provenance moved without a re-pin",
    },
    {
      name: "dependency roster removed",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies = []
        })
      },
      expectedFailure: "incremental BLAKE3 provenance must contain exactly one dependency",
    },
    {
      name: "extra dependency",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies.push({ artifact: "../extra.js", sha256: "0".repeat(64) })
        })
      },
      expectedFailure: "incremental BLAKE3 provenance must contain exactly one dependency",
    },
    {
      name: "renamed dependency",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies[0].artifact = "../renamed-hex-lower.js"
        })
      },
      expectedFailure: "incremental BLAKE3 dependency artifact changed",
    },
    {
      name: "different well-formed dependency hash",
      mutate: async (temporary) => {
        await mutateProvenance(temporary, (provenance) => {
          provenance.dependencies[0].sha256 = "0".repeat(64)
        })
      },
      expectedFailure: "incremental BLAKE3 dependency provenance moved without a re-pin",
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
  const path = join(provenanceRoot, "incremental-blake3.provenance.json")
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
