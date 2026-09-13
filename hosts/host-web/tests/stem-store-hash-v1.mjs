import assert from "node:assert/strict"
import { hexLower } from "../web/hex-lower.js"
import { IncrementalBlake3, blake3Stream } from "../web/stem-store/incremental-blake3.js"
import {
  FetchStemResolver,
  MemoryStemResolver,
} from "../web/stem-store/resolver.js"

assert.equal(hexLower(new Uint8Array()), "")
assert.equal(
  hexLower(new Uint8Array([0x00, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff])),
  "000123456789abcdefff",
)

const vectors = [
  [new Uint8Array(), "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"],
  [new TextEncoder().encode("abc"), "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"],
  [new Uint8Array(1_000_000).fill("a".charCodeAt(0)), "616f575a1b58d4c9797d4217b9730ae5e6eb319d76edef6549b46f4efe31ff8b"],
]

// Independent BLAKE3-256 KATs from hash-wasm 4.12.0. Their endings exercise a partial final
// block after an already-compressed block and the 1,024-byte chunk boundary.
const irregularEndingVectors = [
  [65, "7f55325c3368e44f79edddb7b1a079b8aeae7ab43a0254b3012e564d75c4c1ae"],
  [1023, "7cc12c8435bc5cdb011ba62b7367601fb7d30b23b32e177f7e41907b210a8673"],
  [1025, "b8c5c46b114817810a6ed499350cb4d2423cd23dd08d32c137b226d8559b8ab0"],
]

for (const [length, expected] of irregularEndingVectors) {
  const bytes = new Uint8Array(length)
  for (let index = 0; index < bytes.length; index += 1) bytes[index] = (index * 31 + 7) & 0xff
  for (const chunkBytes of [1, 17, 64, 127, 1024]) {
    const hash = new IncrementalBlake3()
    for (let offset = 0; offset < bytes.byteLength; offset += chunkBytes) {
      hash.update(bytes.subarray(offset, offset + chunkBytes))
    }
    assert.equal(hash.digestHex(), expected, `irregular ${length}-byte KAT with ${chunkBytes}-byte chunks`)
  }
}

for (const [bytes, expected] of vectors) {
  for (const chunkBytes of [1, 7, 63, 64, 65, 4093, bytes.byteLength || 1]) {
    const hash = new IncrementalBlake3()
    for (let offset = 0; offset < bytes.byteLength; offset += chunkBytes) {
      hash.update(bytes.subarray(offset, offset + chunkBytes))
    }
    assert.equal(hash.digestHex(), expected)
  }
}

const fixture = new Uint8Array(512 * 1024 + 31)
for (let index = 0; index < fixture.length; index += 1) fixture[index] = index * 31
const fixtureHex = "37dd28058dfeeb01cdba9b4f7f04d990980353eeae9fb4998e975e301350d67b"
const identity = `blake3:${fixtureHex}`
const resolver = new MemoryStemResolver({ [identity]: fixture }, { chunkBytes: 8191 })
const resolved = await resolver.resolve(identity)
assert.equal(resolved.canonicalBytes, fixture.byteLength)
assert.deepEqual(await blake3Stream(resolved.stream), {
  bytes: fixture.byteLength,
  hex: fixtureHex,
})
assert.deepEqual(resolver.requests, [identity])

const delivered = new Uint8Array([1, 2, 3, 4, 5, 6])
const deliveredIdentity = "blake3:828a8660ae86b86f1ebf951a6f84349520cc1501fb6fcf95b05df01200be9fa2"
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
assert.deepEqual(await blake3Stream(resumed.stream), {
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
const stalledStem = await stalled.resolve(`blake3:${"0".repeat(64)}`)
await assert.rejects(blake3Stream(stalledStem.stream), (error) => {
  assert.equal(error.code, "stem.resolve.stalled")
  return true
})

const finalized = new IncrementalBlake3()
finalized.update(new Uint8Array())
finalized.digestHex()
assert.throws(() => finalized.update(new Uint8Array()), /already finalized/)
