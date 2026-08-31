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

async function sameSessionReplacementReleasesPredecessorPin() {
  const bytes = pcm16([0, 1, -1, 32767], 1)
  const stemIdentity = identity(bytes)
  const backend = new FakeOpfsBackend()
  const gate = new StemSessionGate(
    new OpfsStemStore({
      storage: backend.storage(),
      locks: new FakeLockManager(),
      tabId: "same-reopen",
    }),
    new MemoryStemResolver({ [stemIdentity]: bytes })
  )
  const options = {
    sessionId: "same",
    stems: [{ identity: stemIdentity, bytes: bytes.byteLength }],
    resume() {},
  }
  const first = await gate.open(options)
  const replacement = await Promise.race([
    gate.open(options),
    new Promise((resolve) => setTimeout(() => resolve("timed-out"), 100)),
  ])
  assert.notEqual(
    replacement,
    "timed-out",
    "same-session replacement waited on its own predecessor pin lock"
  )
  assert.equal(gate.state, "interactive")
  await assert.rejects(first.read(stemIdentity), (error) => {
    assert.equal(error.code, "stem.session.closed")
    return true
  })
  let index = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-v1/index.json"))
  )
  assert.deepEqual(index.stems[stemIdentity].pins, [
    "session:same-reopen:same",
  ])
  await gate.close()
  index = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-v1/index.json"))
  )
  assert.deepEqual(index.stems[stemIdentity].pins, [])
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

async function rejectedResumeDoesNotLeakPins() {
  const bytes = pcm16([0, 1, 2, 3], 1)
  const stemIdentity = identity(bytes)
  const backend = new FakeOpfsBackend()
  const gate = new StemSessionGate(
    new OpfsStemStore({
      storage: backend.storage(),
      locks: new FakeLockManager(),
      tabId: "resume-reject",
    }),
    new MemoryStemResolver({ [stemIdentity]: bytes })
  )
  await assert.rejects(
    gate.open({
      sessionId: "resume-reject",
      stems: [{ identity: stemIdentity, bytes: bytes.byteLength }],
      resume: () =>
        new Promise((_resolve, reject) =>
          setTimeout(() => reject(new Error("route changed")), 10)
        ),
    }),
    /route changed/
  )
  const index = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-v1/index.json"))
  )
  assert.deepEqual(index.stems[stemIdentity].pins, [])
}

// --- Issue #278: the Worker's opt-in cadence -----------------------------------------------------
//
// `pcm-pump-worker.js` was pull-driven and nothing in the repo scheduled the next `pump`, so the
// shipped Worker was mountable only by a consumer that had also re-implemented a cadence over its
// message vocabulary. `selfDriving` is that cadence, shipped once and off by default.
//
// These four run the REAL worker module rather than a re-implementation of it. In Node that costs
// two shims -- a `self` with a `postMessage` and a settable `onmessage`, and a `navigator` carrying
// the fake OPFS backend the Worker's own `new OpfsStemStore({ folderName })` reaches for -- and a
// cache-busting import specifier so each case gets a module instance with its own top-level state.
// The alternative, testing a copy of the loop, would prove nothing about the file that ships.

const WROTE = 14
const WRITER_STATE = 12

/** Load one fresh instance of the shipped worker against a fake `self`. */
async function mountWorker(instance) {
  const posted = []
  const realm = {
    postMessage(message) {
      posted.push(message)
    },
  }
  globalThis.self = realm
  await import(`../web/stem-store/pcm-pump-worker.js?instance=${instance}`)
  return {
    posted,
    send(data) {
      realm.onmessage({ data })
    },
    /** Resolve when `predicate` holds, or throw after `timeoutMs` of real time. */
    async until(predicate, description, timeoutMs = 2000) {
      const deadline = Date.now() + timeoutMs
      while (!predicate()) {
        if (Date.now() > deadline) throw new Error(`timed out waiting for ${description}`)
        await new Promise((resolve) => setTimeout(resolve, 1))
      }
    },
    reply(type) {
      return posted.find((message) => message.type === type)
    },
  }
}

/** A fake OPFS holding one ingested stem, plus the `navigator` the Worker will read. */
async function stagedStore(bytes, folderName) {
  const stemIdentity = identity(bytes)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  Object.defineProperty(globalThis, "navigator", {
    configurable: true,
    value: { storage: backend.storage(), locks },
  })
  const store = new OpfsStemStore({ folderName, storage: backend.storage(), locks })
  const lease = await store.openSession({
    sessionId: "staged",
    stems: [{ identity: stemIdentity, bytes: bytes.byteLength }],
    resolver: new MemoryStemResolver({ [stemIdentity]: bytes }),
  })
  return { identity: stemIdentity, lease }
}

/** The five-frame stereo fixture `workerPumpContract` uses, and its ring shape. */
function cadenceFixture() {
  return {
    bytes: pcm16([-32768, 32767, 0, -1, 16384, -16384, 32767, -32768, 1, -1], 2),
    frames: 5,
    ring: { channels: 2, frameCapacity: 3, capacity: 4 },
    windowFrames: 4,
    expectedChunks: 3,
  }
}

function initializeMessage({ identity: stemIdentity, shared, fixture, folderName, selfDriving }) {
  return {
    type: "initialize",
    requestId: "init",
    folderName,
    windowFrames: fixture.windowFrames,
    generation: 1,
    ...(selfDriving === undefined ? {} : { selfDriving }),
    sources: [
      {
        sourceId: "source",
        identity: stemIdentity,
        channels: fixture.ring.channels,
        bitDepth: 16,
        frames: fixture.frames,
        ring: shared,
      },
    ],
  }
}

/** Every committed slot's frames, flags and planes, as plain data for comparison. */
function ringTranscript(shared, slots) {
  return Array.from({ length: slots }, (_, slot) => {
    const view = inspectRing(shared, slot)
    return {
      frames: view.frames,
      flags: view.flags,
      planes: view.planes.map((plane) => Array.from(plane)),
    }
  })
}

async function selfDrivingFillsWithoutPumpMessages() {
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "self-driving")
  const shared = createFixtureMsb1Ring(fixture.ring)
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("self-driving")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "self-driving",
      selfDriving: { idleMs: 1 },
    })
  )
  await worker.until(() => control[WROTE] === fixture.expectedChunks, "the rings to fill")
  assert.equal(
    worker.posted.filter((message) => message.type === "pumped").length,
    0,
    "a self-driven pass answers no request, so it posts no `pumped`"
  )
  assert.deepEqual(
    worker.posted.map((message) => message.type),
    ["initialized"],
    "the cadence adds no message to the vocabulary"
  )
  const driven = ringTranscript(shared, fixture.expectedChunks)

  // The loop must stop on `finished` rather than spinning on a source it has consumed.
  const settled = control[WROTE]
  await new Promise((resolve) => setTimeout(resolve, 40))
  assert.equal(control[WROTE], settled, "the loop broke on finished instead of re-reading")

  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()

  // Cadence changes when the bytes are written, never which bytes. Same fixture, same ring shape,
  // pulled by explicit `pump` messages: the transcripts must be equal element for element.
  const pulled = await pullDrivenTranscript(fixture)
  assert.deepEqual(driven, pulled, "self-driving wrote exactly what pumping writes")
}

/** The same fixture through the same worker, driven by `pump` messages only. */
async function pullDrivenTranscript(fixture) {
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "pull-driven")
  const shared = createFixtureMsb1Ring(fixture.ring)
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("pull-driven")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "pull-driven",
    })
  )
  await worker.until(() => worker.reply("initialized") !== undefined, "the initialize reply")

  // The default is unchanged, which is the same claim as: nothing happens until asked.
  await new Promise((resolve) => setTimeout(resolve, 40))
  assert.equal(control[WROTE], 0, "an un-driven worker writes nothing on its own")

  worker.send({ type: "pump", requestId: "pump" })
  await worker.until(() => worker.reply("pumped") !== undefined, "the pump reply")
  assert.deepEqual(
    { ...worker.reply("pumped") },
    { type: "pumped", requestId: "pump", chunks: 3, frames: 5, finished: true },
    "the pull-driven reply is the pre-#278 reply, field for field"
  )
  const transcript = ringTranscript(shared, fixture.expectedChunks)
  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()
  return transcript
}

async function stopInterruptsTheIdleLoop() {
  // A one-frame ring and a five-frame source: the loop fills the ring, then every pass writes zero
  // chunks and sleeps. The sleep is deliberately longer than this test is willing to wait, so a
  // `stopped` that arrives promptly can only mean the loop was interrupted rather than woken.
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "stop-interrupt")
  const shared = createFixtureMsb1Ring({ channels: 2, frameCapacity: 3, capacity: 1 })
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("stop-interrupt")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "stop-interrupt",
      selfDriving: { idleMs: 30_000 },
    })
  )
  await worker.until(() => control[WROTE] === 1, "the one-slot ring to fill")
  assert.equal(Atomics.load(control, WRITER_STATE), 1, "the writer is engaged while driving")

  const started = Date.now()
  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply", 1000)
  assert.ok(
    Date.now() - started < 1000,
    "stop did not wait on the idle sleep it interrupted"
  )
  assert.equal(Atomics.load(control, WRITER_STATE), 0, "stop released the ring writer")

  const settled = control[WROTE]
  await new Promise((resolve) => setTimeout(resolve, 60))
  assert.equal(control[WROTE], settled, "no tick ran after stop")

  // "Interrupts" has to mean the sleep was cancelled, not merely outlived. A stop that only
  // cleared the token would leave this thirty-second timer pending: the Worker would answer
  // `stopped`, write nothing more, and still hold its realm awake long after the session ended.
  // That is invisible to every assertion above and is exactly the leak a browser tab would feel.
  await new Promise((resolve) => setImmediate(resolve))
  assert.equal(
    process.getActiveResourcesInfo().filter((resource) => resource === "Timeout").length,
    0,
    "stop cancelled the idle timer rather than waiting it out"
  )
  await lease.close()
}

async function seekReArmsAFinishedLoop() {
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "seek-rearm")
  const shared = createFixtureMsb1Ring(fixture.ring)
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("seek-rearm")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "seek-rearm",
      selfDriving: { idleMs: 1 },
    })
  )
  await worker.until(() => control[WROTE] === fixture.expectedChunks, "the first fill")

  // The loop has broken on `finished`. A seek moves the cursor off end-of-region, so the cadence
  // has work again -- and if `seek` did not re-arm it, this session would stall silently, which is
  // the exact failure a consumer would blame on the ring rather than on the Worker.
  worker.send({ type: "seek", requestId: "seek", frame: 0 })
  await worker.until(() => worker.reply("sought") !== undefined, "the seek reply")
  await worker.until(
    () => control[WROTE] > fixture.expectedChunks,
    "the loop to resume after the seek"
  )

  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()
}

await transformContract()
await workerPumpContract()
await readFailureHardStops()
await resumeInsideGestureGate()
await sameSessionReplacementReleasesPredecessorPin()
await refusedGateNeverBecomesInteractive()
await rejectedResumeDoesNotLeakPins()
await selfDrivingFillsWithoutPumpMessages()
await stopInterruptsTheIdleLoop()
await seekReArmsAFinishedLoop()
process.stdout.write("stem-pump-v1: PASS\n")
