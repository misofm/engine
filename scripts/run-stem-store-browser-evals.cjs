#!/usr/bin/env node
"use strict"

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
