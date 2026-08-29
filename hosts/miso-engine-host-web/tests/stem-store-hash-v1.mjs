import assert from "node:assert/strict"
import { createHash } from "node:crypto"
import { IncrementalSha256, sha256Stream } from "../web/stem-store/incremental-sha256.js"
import {
  FetchStemResolver,
  MemoryStemResolver,
} from "../web/stem-store/resolver.js"

const vectors = [
  [new Uint8Array(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"],
  [new TextEncoder().encode("abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"],
  [new Uint8Array(1_000_000).fill("a".charCodeAt(0)), "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"],
]

for (const [bytes, expected] of vectors) {
  for (const chunkBytes of [1, 7, 63, 64, 65, 4093, bytes.byteLength || 1]) {
    const hash = new IncrementalSha256()
    for (let offset = 0; offset < bytes.byteLength; offset += chunkBytes) {
      hash.update(bytes.subarray(offset, offset + chunkBytes))
    }
    assert.equal(hash.digestHex(), expected)
  }
}

const fixture = new Uint8Array(512 * 1024 + 31)
for (let index = 0; index < fixture.length; index += 1) fixture[index] = index * 31
const fixtureHex = createHash("sha256").update(fixture).digest("hex")
const identity = `sha256:${fixtureHex}`
const resolver = new MemoryStemResolver({ [identity]: fixture }, { chunkBytes: 8191 })
const resolved = await resolver.resolve(identity)
assert.equal(resolved.canonicalBytes, fixture.byteLength)
assert.deepEqual(await sha256Stream(resolved.stream), {
  bytes: fixture.byteLength,
  hex: fixtureHex,
})
assert.deepEqual(resolver.requests, [identity])

const delivered = new Uint8Array([1, 2, 3, 4, 5, 6])
const deliveredIdentity = `sha256:${createHash("sha256").update(delivered).digest("hex")}`
const ranges = []
let firstRead = true
const resumable = new FetchStemResolver({
  urlForIdentity: () => "https://fixtures.invalid/stem.flac",
  decode: (stream) => stream,
  async fetcher(_url, init) {
    ranges.push(init.headers?.Range ?? null)
    if (init.headers === undefined) {
      return new Response(
        new ReadableStream({
          pull(controller) {
            if (firstRead) {
              firstRead = false
              controller.enqueue(delivered.slice(0, 3))
            } else {
              controller.error(new Error("connection reset"))
            }
          },
        }),
        { status: 200, headers: { "content-length": delivered.byteLength } }
      )
    }
    return new Response(new Blob([delivered.slice(3)]), {
      status: 206,
      headers: { "content-range": "bytes 3-5/6" },
    })
  },
})
const resumed = await resumable.resolve(deliveredIdentity)
assert.deepEqual(await sha256Stream(resumed.stream), {
  bytes: delivered.byteLength,
  hex: deliveredIdentity.slice(7),
})
assert.deepEqual(ranges, [null, "bytes=3-"])

const stalled = new FetchStemResolver({
  urlForIdentity: () => "https://fixtures.invalid/stalled.flac",
  decode: (stream) => stream,
  readDeadlineMs: 5,
  maximumResumeAttempts: 0,
  fetcher: async () =>
    new Response(new ReadableStream({ pull() {} }), {
      status: 200,
      headers: { "content-length": "1" },
    }),
})
const stalledStem = await stalled.resolve(`sha256:${"0".repeat(64)}`)
await assert.rejects(sha256Stream(stalledStem.stream), (error) => {
  assert.equal(error.code, "stem.resolve.stalled")
  return true
})

const finalized = new IncrementalSha256()
finalized.update(new Uint8Array())
finalized.digestHex()
assert.throws(() => finalized.update(new Uint8Array()), /already finalized/)
