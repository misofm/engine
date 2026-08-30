#!/usr/bin/env node
"use strict"

// The stem store's browser legs: incremental SHA-256 throughput, the cold/warm OPFS ingest
// budgets, and (issue #278) the pump Worker's opt-in `selfDriving` cadence in a real Worker.
//
// Not a sweep row: it needs playwright and downloaded browsers, and every sweep row is hermetic.
// `playwright` lives in `hosts/miso-engine-host-web/qualification/node_modules` rather than beside
// this script, and Node resolves a CommonJS `require` from the SCRIPT's directory, not the working
// directory -- so an invocation from the qualification directory alone is not enough:
//
//   cd hosts/miso-engine-host-web/qualification && npm ci
//   NODE_PATH=$PWD/node_modules node ../../../scripts/run-stem-store-browser-evals.cjs [leg...]
//
// Legs default to all three. A leg whose browser lacks OPFS reports `available: false` with a
// reason rather than failing: WebKit does that today for `navigator.storage.getDirectory`.
//
// Remove that `node_modules` again before running `scripts/sweep.sh`. `playwright-core` ships
// `lib/webp_codec.wasm`, and `check-effect-interchange-qualification.sh`'s "generated artifact
// exists under a source path" scan prunes only `./target`, so an installed qualification tree
// turns an unrelated policy row red. That is a defect in the scan's prune list rather than in this
// runner, and widening a policy gate is its own issue, so it is recorded here rather than fixed.

const { createHash } = require("node:crypto")
const { readFile, stat } = require("node:fs/promises")
const http = require("node:http")
const { extname, join, normalize, resolve } = require("node:path")
const { chromium, firefox, webkit } = require("playwright")

const repository = resolve(__dirname, "..")
const fixtureBytes = 16 * 1024 * 1024
const expected = createHash("sha256")
for (let offset = 0; offset < fixtureBytes; offset += 64 * 1024) {
  const size = Math.min(64 * 1024, fixtureBytes - offset)
  const chunk = Buffer.allocUnsafe(size)
  for (let index = 0; index < size; index += 1) {
    chunk[index] = ((offset + index) * 31) & 0xff
  }
  expected.update(chunk)
}
const digest = expected.digest("hex")

const mime = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
}

async function listen() {
  const server = http.createServer(async (request, response) => {
    try {
      const requested = request.url === "/" ? "/scripts/stem-store-eval.html" : request.url
      const path = normalize(join(repository, decodeURIComponent(requested.split("?")[0])))
      if (!path.startsWith(`${repository}/`)) throw new Error("path escaped repository")
      const info = await stat(path)
      if (!info.isFile()) throw new Error("not a file")
      response.writeHead(200, {
        "content-type": mime[extname(path)] ?? "application/octet-stream",
        "cross-origin-opener-policy": "same-origin",
        "cross-origin-embedder-policy": "require-corp",
      })
      response.end(await readFile(path))
    } catch (error) {
      response.writeHead(404, { "content-type": "text/plain" })
      response.end(error.message)
    }
  })
  await new Promise((resolveListen, reject) => {
    server.once("error", reject)
    server.listen(0, "127.0.0.1", resolveListen)
  })
  return { server, port: server.address().port }
}

async function probe(browserType) {
  const browser = await browserType.launch({ headless: true })
  const page = await browser.newPage()
  const { server, port } = await listen()
  try {
    await page.goto(`http://127.0.0.1:${port}/`)
    return await page.evaluate(
      async ({ fixtureBytes: bytes, digest: expectedDigest }) => {
        if (typeof navigator.storage?.getDirectory !== "function") {
          return { available: false, reason: "getDirectory absent" }
        }
        const {
          IncrementalSha256,
          MemoryStemResolver,
          OpfsStemStore,
          createFixtureMsb1Ring,
          createStemPumpWorker,
        } = await import(
          "/hosts/miso-engine-host-web/web/stem-store/index.js"
        )
        const pcm = new Uint8Array(bytes)
        for (let index = 0; index < pcm.length; index += 1) {
          pcm[index] = (index * 31) & 0xff
        }
        const hashStarted = performance.now()
        const hasher = new IncrementalSha256()
        for (let offset = 0; offset < pcm.length; offset += 64 * 1024) {
          hasher.update(pcm.subarray(offset, offset + 64 * 1024))
        }
        const observedDigest = hasher.digestHex()
        const hashMs = performance.now() - hashStarted
        if (observedDigest !== expectedDigest) throw new Error("browser SHA-256 mismatch")

        const identity = `sha256:${expectedDigest}`
        const folderName = `miso-stem-browser-eval-${crypto.randomUUID()}`
        const resolver = new MemoryStemResolver(
          { [identity]: pcm },
          { chunkBytes: 64 * 1024 }
        )
        const store = new OpfsStemStore({ folderName })
        let firstHashedAt
        let finalHashedAt
        const coldStarted = performance.now()
        const cold = await store.openSession({
          sessionId: "cold",
          stems: [{ identity, bytes }],
          resolver,
          onProgress(event) {
            if (event.stage === "hashed") {
              firstHashedAt ??= performance.now()
              finalHashedAt = performance.now()
            }
          },
        })
        const coldMs = performance.now() - coldStarted
        const warmStarted = performance.now()
        const warm = await store.openSession({
          sessionId: "warm",
          stems: [{ identity, bytes }],
          resolver,
        })
        const warmVerifyMs = performance.now() - warmStarted
        const budgetsMs = { cold: 5_000, warmVerify: 2_000 }
        if (coldMs > budgetsMs.cold || warmVerifyMs > budgetsMs.warmVerify) {
          throw new Error(
            `browser latency budget exceeded: cold=${coldMs}, warm=${warmVerifyMs}`
          )
        }
        const read = await warm.read(identity)
        if (read.size !== bytes || resolver.requests.length !== 1) {
          throw new Error("browser store did not preserve the hard-gate contract")
        }
        await Promise.all([cold.close(), warm.close()])
        const root = await navigator.storage.getDirectory()
        await root.removeEntry(folderName, { recursive: true })

        // Issue #278: the opt-in Worker cadence, in a real Worker over real OPFS.
        //
        // `hosts/miso-engine-host-web/tests/stem-pump-v1.mjs` already drives the shipped worker
        // module end to end, but it does so in Node behind a `self` shim and a fake OPFS. The two
        // things it cannot exercise are the two this leg exists for: that the file loads as a
        // module Worker at all, and that the loop's `SharedArrayBuffer` writes are visible to the
        // main realm without a message telling it to look. A cadence that only worked under the
        // shim would pass every gate and stall in a browser tab.
        const selfDriving = await (async () => {
          if (typeof SharedArrayBuffer !== "function") {
            return { available: false, reason: "SharedArrayBuffer absent (COOP/COEP?)" }
          }
          const cadenceFolder = `miso-stem-cadence-${crypto.randomUUID()}`
          // 384 stereo 16-bit frames: three 128-frame chunks, which the four-slot ring holds
          // whole. Nothing drains the ring here, so a source larger than the ring would idle at
          // capacity forever and the fill assertion below would be asserting the wrong thing.
          const pcmBytes = 384 * 2 * 2
          const stem = new Uint8Array(pcmBytes)
          for (let index = 0; index < stem.length; index += 1) {
            stem[index] = (index * 17) & 0xff
          }
          const stemDigest = new IncrementalSha256()
          stemDigest.update(stem)
          const stemIdentity = `sha256:${stemDigest.digestHex()}`
          const frames = pcmBytes / (2 * 2)
          const store = new OpfsStemStore({ folderName: cadenceFolder })
          const lease = await store.openSession({
            sessionId: "cadence",
            stems: [{ identity: stemIdentity, bytes: pcmBytes }],
            resolver: new MemoryStemResolver({ [stemIdentity]: stem }),
          })
          const capacity = 4
          const frameCapacity = 128
          const shared = createFixtureMsb1Ring({ channels: 2, frameCapacity, capacity })
          const control = new Int32Array(shared, 0, 128 / 4)
          const WROTE = 14
          const worker = createStemPumpWorker()
          const received = []
          worker.onmessage = (event) => received.push(event.data)
          const settle = (type, timeoutMs) =>
            new Promise((resolveSettle, rejectSettle) => {
              const deadline = performance.now() + timeoutMs
              const poll = () => {
                if (received.some((message) => message.type === type)) return resolveSettle()
                if (performance.now() > deadline) {
                  return rejectSettle(new Error(`self-driving: no ${type} in ${timeoutMs}ms`))
                }
                setTimeout(poll, 5)
              }
              poll()
            })
          const drivenStarted = performance.now()
          try {
            worker.postMessage({
              type: "initialize",
              requestId: "init",
              folderName: cadenceFolder,
              windowFrames: frameCapacity,
              generation: 1,
              selfDriving: { idleMs: 2 },
              sources: [
                {
                  sourceId: "source",
                  identity: stemIdentity,
                  channels: 2,
                  bitDepth: 16,
                  frames,
                  ring: shared,
                },
              ],
            })
            await settle("initialized", 5_000)
            const expectedChunks = Math.ceil(frames / frameCapacity)
            const fillDeadline = performance.now() + 5_000
            while (control[WROTE] < expectedChunks) {
              if (performance.now() > fillDeadline) {
                throw new Error(
                  `self-driving wrote ${control[WROTE]} of ${expectedChunks} chunks unprompted`
                )
              }
              await new Promise((tick) => setTimeout(tick, 5))
            }
            const filledMs = performance.now() - drivenStarted
            if (received.some((message) => message.type === "pumped")) {
              throw new Error("the self-driven loop broadcast a `pumped` reply")
            }
            const stopStarted = performance.now()
            worker.postMessage({ type: "stop", requestId: "stop" })
            await settle("stopped", 2_000)
            const stopMs = performance.now() - stopStarted
            const settled = control[WROTE]
            await new Promise((tick) => setTimeout(tick, 50))
            if (control[WROTE] !== settled) {
              throw new Error("a tick ran after `stopped`")
            }
            return {
              available: true,
              chunks: control[WROTE],
              expectedChunks,
              filledMs,
              stopMs,
              messageTypes: received.map((message) => message.type),
            }
          } finally {
            worker.terminate()
            await lease.close()
            await root.removeEntry(cadenceFolder, { recursive: true }).catch(() => {})
          }
        })()

        const mebibytes = bytes / (1024 * 1024)
        return {
          available: true,
          fixtureBytes: bytes,
          coldMs,
          warmVerifyMs,
          incrementalHashMs: hashMs,
          incrementalHashMiBPerSecond: mebibytes / (hashMs / 1000),
          ingestHashWindowMs: finalHashedAt - firstHashedAt,
          ingestHashWindowMiBPerSecond:
            mebibytes / ((finalHashedAt - firstHashedAt) / 1000),
          resolverRequests: resolver.requests.length,
          budgetsMs,
          selfDriving,
        }
      },
      { fixtureBytes, digest }
    )
  } finally {
    await browser.close()
    await new Promise((resolveClose) => server.close(resolveClose))
  }
}

async function main() {
  const types = { chromium, firefox, webkit }
  const requested = process.argv.slice(2)
  const legs = requested.length === 0 ? Object.keys(types) : requested
  const report = {}
  for (const leg of legs) {
    if (!(leg in types)) throw new Error(`unknown browser leg: ${leg}`)
    try {
      report[leg] = await probe(types[leg])
    } catch (error) {
      report[leg] = {
        error: { name: error?.name, message: error?.message },
      }
    }
  }
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`)
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
