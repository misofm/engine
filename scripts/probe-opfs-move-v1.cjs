#!/usr/bin/env node
"use strict"

const http = require("node:http")
const { chromium, firefox, webkit } = require("playwright")

const PAGE = `<!doctype html><meta charset="utf-8"><title>OPFS move probe</title>`

async function listen() {
  const server = http.createServer((_request, response) => {
    response.writeHead(200, {
      "content-type": "text/html; charset=utf-8",
      "cross-origin-opener-policy": "same-origin",
      "cross-origin-embedder-policy": "require-corp",
    })
    response.end(PAGE)
  })
  await new Promise((resolve, reject) => {
    server.once("error", reject)
    server.listen(0, "127.0.0.1", resolve)
  })
  const address = server.address()
  if (typeof address !== "object" || address === null) {
    throw new Error("probe server did not bind a TCP address")
  }
  return { server, url: `http://127.0.0.1:${address.port}/` }
}

async function closeServer(server) {
  await new Promise((resolve, reject) =>
    server.close((error) => (error ? reject(error) : resolve()))
  )
}

async function probe(browserType, name) {
  const browser = await browserType.launch({ headless: true })
  const page = await browser.newPage()
  try {
    const { server, url } = await listen()
    try {
      await page.goto(url)
      return await page.evaluate(async () => {
        const storage = navigator.storage
        if (typeof storage?.getDirectory !== "function") {
          return { available: false, reason: "getDirectory absent" }
        }

        const root = await storage.getDirectory()
        const probeName = `miso-stem-move-probe-${crypto.randomUUID()}`
        const probe = await root.getDirectoryHandle(probeName, { create: true })
        const staging = await probe.getDirectoryHandle("staging", {
          create: true,
        })

        const write = async (directory, fileName, bytes) => {
          const handle = await directory.getFileHandle(fileName, { create: true })
          const writable = await handle.createWritable()
          await writable.write(bytes)
          await writable.close()
          return handle
        }
        const read = async (directory, fileName) =>
          Array.from(
            new Uint8Array(
              await (await directory.getFileHandle(fileName)).getFile().then((file) =>
                file.arrayBuffer()
              )
            )
          )
        const missing = async (directory, fileName) => {
          try {
            await directory.getFileHandle(fileName)
            return false
          } catch (error) {
            return error?.name === "NotFoundError"
          }
        }

        const same = await write(probe, "same-before.bin", new Uint8Array([1, 2, 3]))
        const supported = typeof same.move === "function"
        const result = {
          available: true,
          moveFunction: supported,
          sameDirectory: null,
          stagingToFinal: null,
        }
        if (!supported) {
          await root.removeEntry(probeName, { recursive: true })
          return result
        }

        try {
          await same.move("same-after.bin")
          result.sameDirectory = {
            moved: true,
            oldMissing: await missing(probe, "same-before.bin"),
            bytes: await read(probe, "same-after.bin"),
          }
        } catch (error) {
          result.sameDirectory = {
            moved: false,
            error: { name: error?.name, message: error?.message },
          }
        }

        const cross = await write(
          staging,
          "tab-digest",
          new Uint8Array([4, 5, 6, 7])
        )
        try {
          await cross.move(probe, "sha256-digest")
          result.stagingToFinal = {
            moved: true,
            oldMissing: await missing(staging, "tab-digest"),
            bytes: await read(probe, "sha256-digest"),
          }
        } catch (error) {
          result.stagingToFinal = {
            moved: false,
            error: { name: error?.name, message: error?.message },
          }
        }

        await root.removeEntry(probeName, { recursive: true })
        return result
      })
    } finally {
      await closeServer(server)
    }
  } finally {
    await browser.close()
  }
}

async function main() {
  const requested = process.argv.slice(2)
  const legs = requested.length > 0 ? requested : ["chromium", "firefox", "webkit"]
  const types = { chromium, firefox, webkit }
  const report = {}
  for (const name of legs) {
    if (!(name in types)) throw new Error(`unknown browser leg: ${name}`)
    try {
      report[name] = await probe(types[name], name)
    } catch (error) {
      report[name] = {
        launchError: { name: error?.name, message: error?.message },
      }
    }
  }
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`)
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
