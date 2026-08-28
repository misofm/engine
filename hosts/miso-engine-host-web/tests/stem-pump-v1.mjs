import assert from "node:assert/strict"
import { createHash } from "node:crypto"
import {
  CanonicalPcmPump,
  Msb1RingWriter,
  createFixtureMsb1Ring,
  deinterleaveCanonicalPcm,
} from "../web/stem-store/pcm-pump.js"
import { OpfsStemStore } from "../web/stem-store/opfs-store.js"
import { MemoryStemResolver } from "../web/stem-store/resolver.js"
import { StemSessionGate } from "../web/stem-store/session-gate.js"
import { FakeLockManager, FakeOpfsBackend } from "./stem-store-fakes.mjs"

function identity(bytes) {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`
}

function pcm16(samples, channels) {
  const bytes = new Uint8Array(samples.length * 2)
  const view = new DataView(bytes.buffer)
  samples.forEach((sample, index) => view.setInt16(index * 2, sample, true))
  assert.equal(samples.length % channels, 0)
  return bytes
}

function pcm24(samples, channels) {
  const bytes = new Uint8Array(samples.length * 3)
  samples.forEach((sample, index) => {
    const value = sample & 0xff_ffff
    bytes[index * 3] = value & 0xff
    bytes[index * 3 + 1] = (value >>> 8) & 0xff
    bytes[index * 3 + 2] = (value >>> 16) & 0xff
  })
  assert.equal(samples.length % channels, 0)
  return bytes
}

function inspectRing(shared, slot) {
  const control = new Int32Array(shared, 0, 128 / 4)
  const capacity = control[2]
  const channels = control[3]
  const frameCapacity = control[4]
  const headers = new Int32Array(shared, control[5], (capacity * 32) / 4)
  const planes = Array.from(
    { length: channels },
    (_, channel) =>
      new Float32Array(
        shared,
        control[6] + (slot * channels + channel) * frameCapacity * 4,
        frameCapacity
      )
  )
  return {
    control,
    frames: headers[slot * 8 + 2],
    flags: headers[slot * 8 + 3],
    planes,
  }
}

async function transformContract() {
  const stereo = pcm16([-32768, 32767, 0, -1, 16384, -16384], 2)
  const stereoPlanes = [new Float32Array(3), new Float32Array(3)]
  deinterleaveCanonicalPcm(stereo, 0, 3, 2, 16, stereoPlanes)
  assert.deepEqual(Array.from(stereoPlanes[0]), [-1, 0, 0.5])
  assert.deepEqual(Array.from(stereoPlanes[1]), [32767 / 32768, -1 / 32768, -0.5])

  const mono = pcm24([-8388608, -1, 0, 1, 8388607], 1)
  const monoPlanes = [new Float32Array(5)]
  deinterleaveCanonicalPcm(mono, 0, 5, 1, 24, monoPlanes)
  assert.deepEqual(Array.from(monoPlanes[0]), [
    -1,
    -1 / 8388608,
    0,
    1 / 8388608,
    8388607 / 8388608,
  ])
}

async function workerPumpContract() {
  const bytes = pcm16(
    [-32768, 32767, 0, -1, 16384, -16384, 32767, -32768, 1, -1],
    2
  )
  const stemIdentity = identity(bytes)
  const slices = []
  const lazyBlob = {
    slice(start, end) {
      slices.push(end - start)
      return new Blob([bytes.slice(start, end)])
    },
  }
  const shared = createFixtureMsb1Ring({
    channels: 2,
    frameCapacity: 3,
    capacity: 4,
  })
  const writer = new Msb1RingWriter(shared)
  const previousFetch = globalThis.fetch
  globalThis.fetch = () => {
    throw new Error("pump touched a network API")
  }
  try {
    const pump = new CanonicalPcmPump({
      lease: {
        async read(requested) {
          assert.equal(requested, stemIdentity)
          return lazyBlob
        },
      },
      windowFrames: 4,
      sources: [
        {
          sourceId: "source",
          identity: stemIdentity,
          channels: 2,
          bitDepth: 16,
          frames: 5,
          ring: writer,
        },
      ],
    })
    assert.deepEqual(await pump.pumpUntilFull(), {
      chunks: 3,
      frames: 5,
      finished: true,
    })
    const first = inspectRing(shared, 0)
    const second = inspectRing(shared, 1)
    const third = inspectRing(shared, 2)
    assert.equal(first.frames, 3)
    assert.equal(first.flags, 0)
    assert.deepEqual(Array.from(first.planes[0].subarray(0, 3)), [-1, 0, 0.5])
    assert.equal(second.frames, 1)
    assert.equal(second.flags, 0)
    assert.deepEqual(Array.from(second.planes[1].subarray(0, 1)), [-1])
    assert.equal(third.frames, 1)
    assert.equal(third.flags, 1)
    assert.deepEqual(Array.from(third.planes[1].subarray(0, 1)), [-1 / 32768])
    assert.ok(Math.max(...slices) <= 4 * 2 * 2, "pump RAM is bounded by one window")
    pump.stop()
  } finally {
    globalThis.fetch = previousFetch
  }
}

async function readFailureHardStops() {
  let released = false
  let surfaced
  const ring = {
    channels: 1,
    frameCapacity: 2,
    capacity: 1,
    occupancy: 0,
    engage() {},
    reserve() { return [new Float32Array(2)] },
    commit() {},
    seek() {},
    release() { released = true },
  }
  const pump = new CanonicalPcmPump({
    lease: { read: async () => { throw new Error("disk read died") } },
    sources: [
      {
        sourceId: "dead",
        identity: `sha256:${"0".repeat(64)}`,
        channels: 1,
        bitDepth: 16,
        frames: 2,
        ring,
      },
    ],
    onError(error) { surfaced = error },
  })
  await assert.rejects(pump.pumpUntilFull(), /disk read died/)
  assert.equal(released, true, "mid-play read failure hard-stops the writer")
  assert.match(surfaced.message, /disk read died/)
}

async function resumeInsideGestureGate() {
  const bytes = pcm16([0, 1, -1, 32767], 1)
  const stemIdentity = identity(bytes)
  const backend = new FakeOpfsBackend()
  const store = new OpfsStemStore({
    storage: backend.storage(),
    locks: new FakeLockManager(),
    tabId: "gesture",
  })
  const gate = new StemSessionGate(
    store,
    new MemoryStemResolver({ [stemIdentity]: bytes }, { chunkBytes: 1 })
  )
  let resumed = false
  const opening = gate.open({
    sessionId: "gesture",
    stems: [{ identity: stemIdentity, bytes: bytes.byteLength }],
    resume() {
      resumed = true
    },
  })
  assert.equal(resumed, true, "resume is invoked before open() yields")
  assert.equal(gate.state, "loading")
  const lease = await opening
  assert.equal(gate.state, "interactive")
  await lease.close()
  await gate.close()
}

async function refusedGateNeverBecomesInteractive() {
  const events = []
  let resumed = false
  const gate = new StemSessionGate(
    new OpfsStemStore({ storage: {}, locks: new FakeLockManager() }),
    new MemoryStemResolver({})
  )
  await assert.rejects(
    gate.open({
      sessionId: "refused",
      stems: [],
      resume() { resumed = true },
      onProgress(event) { events.push(event.stage) },
    }),
    (error) => error.code === "storage.unavailable"
  )
  assert.equal(resumed, true)
  assert.equal(gate.state, "refused")
  assert.equal(events.includes("interactive"), false)
}

await transformContract()
await workerPumpContract()
await readFailureHardStops()
await resumeInsideGestureGate()
await refusedGateNeverBecomesInteractive()
process.stdout.write("stem-pump-v1: PASS\n")
