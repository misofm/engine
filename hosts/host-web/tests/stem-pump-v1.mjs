import assert from "node:assert/strict"
import {
  CanonicalPcmPump,
  CanonicalPcmPumpError,
  Msb1RingWriter,
  createFixtureMsb1Ring,
  deinterleaveCanonicalPcm,
} from "../web/stem-store/pcm-pump.js"
import { OpfsStemStore } from "../web/stem-store/opfs-store.js"
import { IncrementalBlake3 } from "../web/stem-store/incremental-blake3.js"
import { MemoryStemResolver } from "../web/stem-store/resolver.js"
import { StemSessionGate } from "../web/stem-store/session-gate.js"
import { FakeLockManager, FakeOpfsBackend } from "./stem-store-fakes.mjs"

function identity(bytes) {
  const hash = new IncrementalBlake3()
  hash.update(bytes)
  return `blake3:${hash.digestHex()}`
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
  const headersI64 = new BigInt64Array(shared, control[5], (capacity * 32) / 8)
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
    generation: headersI64[slot * 4 + 2],
    startFrame: headersI64[slot * 4 + 3],
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
      chunks: 2,
      frames: 5,
      finished: true,
    })
    const first = inspectRing(shared, 0)
    const second = inspectRing(shared, 1)
    assert.equal(first.frames, 3)
    assert.equal(first.flags, 0)
    assert.deepEqual(Array.from(first.planes[0].subarray(0, 3)), [-1, 0, 0.5])
    assert.equal(second.frames, 2)
    assert.equal(second.flags, 1)
    assert.deepEqual(Array.from(second.planes[1].subarray(0, 2)), [-1, -1 / 32768])
    assert.ok(Math.max(...slices) <= 4 * 2 * 2, "pump RAM is bounded by one window")
    pump.stop()
  } finally {
    globalThis.fetch = previousFetch
  }
}

async function unalignedSeekKeepsChunksQuantumShaped() {
  const channels = 3
  const frames = 13
  const samples = Array.from({ length: frames * channels }, (_, index) => index * 1000 - 7000)
  const bytes = pcm24(samples, channels)
  const stemIdentity = identity(bytes)
  const slices = []
  const lazyBlob = {
    slice(start, end) {
      slices.push({ start, end })
      return new Blob([bytes.slice(start, end)])
    },
  }
  const shared = createFixtureMsb1Ring({ channels, frameCapacity: 3, capacity: 4 })
  const writer = new Msb1RingWriter(shared)
  const pump = new CanonicalPcmPump({
    lease: { async read() { return lazyBlob } },
    windowFrames: 5,
    sources: [{
      sourceId: "unaligned",
      identity: stemIdentity,
      channels,
      bitDepth: 24,
      frames,
      ring: writer,
    }],
  })

  await pump.pumpUntilFull()
  const preSeekSliceCount = slices.length
  drainRing(shared)
  await pump.seek(2)
  assert.deepEqual(await pump.pumpUntilFull(), {
    chunks: 4,
    frames: 11,
    finished: true,
  })

  const frameBytes = channels * 3
  const postSeekSlices = slices.slice(preSeekSliceCount)
  assert.equal(postSeekSlices[0].start, 2 * frameBytes)
  assert.ok(postSeekSlices.length > 1, "the post-seek run reloads a later window")
  for (const [slot, start] of [2, 5, 8, 11].entries()) {
    const chunk = inspectRing(shared, slot)
    assert.equal(chunk.startFrame, BigInt(start))
    assert.equal(chunk.generation, 2n)
    assert.equal(chunk.frames, start === 11 ? 2 : 3)
    assert.equal(chunk.flags, start === 11 ? 1 : 0)
    for (let frame = 0; frame < chunk.frames; frame += 1) {
      for (let channel = 0; channel < channels; channel += 1) {
        const expected = samples[(start + frame) * channels + channel] / 8_388_608
        assert.equal(chunk.planes[channel][frame], expected)
      }
    }
  }

  for (const { start, end } of slices) {
    assert.equal(start % frameBytes, 0, "slice starts on a frame boundary")
    assert.equal(end % frameBytes, 0, "slice ends on a frame boundary")
    assert.ok(end - start <= 5 * frameBytes, "slice stays within one window")
    assert.ok(end <= frames * frameBytes, "slice stays within the source")
  }
  pump.stop()
}

// --- Issue #2118: window reads run concurrently and read ahead -----------------------------------

/** The pump's bound on window reads in flight (`MAX_READS_IN_FLIGHT` in pcm-pump.js). */
const READS_IN_FLIGHT = 8

/** MSB1 control words (`CONTROL` in pcm-pump.js). */
const WRITE_INDEX = 8
const READ_INDEX = 9

/** Consume every committed chunk, as the render thread would. */
function drainRing(shared) {
  const control = new Int32Array(shared, 0, 128 / 4)
  Atomics.store(control, READ_INDEX, control[WRITE_INDEX])
}

async function windowReadsOverlapAcrossSources() {
  // N sources whose window reads each take `delay`. One pass needs one window per source, so a
  // pump that awaits each read in turn takes N * delay; this one takes ceil(N / 8) * delay.
  const sources = 24
  const delay = 40
  const windowFrames = 8
  const frames = 3 * windowFrames
  const bytes = pcm16(Array.from({ length: frames }, (_, frame) => frame), 1)
  let inFlight = 0
  let maxInFlight = 0
  // After the cold pass, reads stay pending until the test releases them.
  let held = null
  const slowBlob = {
    slice(start, end) {
      return {
        arrayBuffer() {
          inFlight += 1
          maxInFlight = Math.max(maxInFlight, inFlight)
          const finish = (resolve) => {
            inFlight -= 1
            resolve(bytes.slice(start, end).buffer)
          }
          return new Promise((resolve) => {
            if (held === null) setTimeout(() => finish(resolve), delay)
            else held.push(() => finish(resolve))
          })
        },
      }
    },
  }
  const rings = Array.from({ length: sources }, () =>
    createFixtureMsb1Ring({ channels: 1, frameCapacity: 4, capacity: 2 })
  )
  const pump = new CanonicalPcmPump({
    lease: { async read() { return slowBlob } },
    windowFrames,
    sources: rings.map((ring, index) => ({
      sourceId: `slow-${index}`,
      identity: `blake3:${String(index).padStart(64, "0")}`,
      channels: 1,
      bitDepth: 16,
      frames,
      ring: new Msb1RingWriter(ring),
    })),
  })

  const started = performance.now()
  assert.deepEqual(await pump.pumpUntilFull(), { chunks: 2 * sources, frames: 8 * sources, finished: false })
  const coldMs = performance.now() - started
  const rounds = Math.ceil(sources / READS_IN_FLIGHT)
  assert.equal(maxInFlight, READS_IN_FLIGHT, "reads overlap up to the bound and never past it")
  assert.ok(coldMs >= rounds * delay - 5, `a pass waited ${coldMs} ms, under ${rounds} read rounds`)

  // Each source's next window was read ahead after the pass: once the rings drain, the next pass
  // writes it while every read it starts stays pending.
  while (inFlight > 0) await new Promise((resolve) => setTimeout(resolve, delay))
  held = []
  rings.forEach(drainRing)
  let stall
  const warm = await Promise.race([
    pump.pumpUntilFull(),
    new Promise((resolve) => { stall = setTimeout(() => resolve("waited on a read"), 2000) }),
  ])
  clearTimeout(stall)
  assert.deepEqual(warm, { chunks: 2 * sources, frames: 8 * sources, finished: false },
    "a pass over read-ahead windows waited on a read")
  for (const [index, ring] of rings.entries()) {
    for (const slot of [0, 1]) {
      const chunk = inspectRing(ring, slot)
      assert.equal(chunk.startFrame, BigInt(8 + 4 * slot), `source ${index} keeps its chunk order`)
      assert.equal(chunk.planes[0][0], (8 + 4 * slot) / 32_768)
    }
  }
  pump.stop()
  while (held.length > 0 || inFlight > 0) {
    held.splice(0).forEach((release) => release())
    await new Promise((resolve) => setImmediate(resolve))
  }
}

async function truncatedReadAheadStopsOnce() {
  // The Blob holds 10 of the 16 declared frames. The cold pass writes window [0, 8) and reads
  // [8, 16) ahead; that read comes back short while no pass awaits it. The next pass must refuse
  // it once, stop the pump and release the ring, not re-read the short window without end.
  const declared = 16
  const bytes = pcm16(Array.from({ length: 10 }, (_, frame) => frame), 1)
  let reads = 0
  const shortBlob = {
    slice(start, end) {
      reads += 1
      return {
        arrayBuffer: () =>
          new Promise((resolve) => setImmediate(() => resolve(bytes.slice(start, end).buffer))),
      }
    },
  }
  const shared = createFixtureMsb1Ring({ channels: 1, frameCapacity: 4, capacity: 2 })
  const errors = []
  const pump = new CanonicalPcmPump({
    lease: { async read() { return shortBlob } },
    windowFrames: 8,
    onError(error) { errors.push(error) },
    sources: [{
      sourceId: "short",
      identity: `blake3:${"2".repeat(64)}`,
      channels: 1,
      bitDepth: 16,
      frames: declared,
      ring: new Msb1RingWriter(shared),
    }],
  })
  assert.deepEqual(await pump.pumpUntilFull(), { chunks: 2, frames: 8, finished: false })
  while (reads < 2) await new Promise((resolve) => setImmediate(resolve))
  await new Promise((resolve) => setTimeout(resolve, 10))
  assert.equal(errors.length, 0, "a failed read ahead waits for the pass that needs it")
  drainRing(shared)
  let stall
  const outcome = await Promise.race([
    pump.pumpUntilFull().then(() => "resolved", (error) => error),
    new Promise((resolve) => { stall = setTimeout(() => resolve("re-read a short window without end"), 2000) }),
  ])
  clearTimeout(stall)
  assert.ok(outcome instanceof RangeError, `a truncated window ${outcome}`)
  assert.match(outcome.message, /PCM window is truncated/)
  assert.equal(errors.length, 1, "the truncation reaches onError exactly once")
  assert.equal(Atomics.load(new Int32Array(shared, 0, 128 / 4), WRITER_STATE), 0, "the pump released its ring")
  assert.equal(reads, 2, "the short window was read once")
}

async function seekDuringReadCommitsNoStaleWindow() {
  // A window read is in flight when a seek lands. The pre-seek window must never reach the ring:
  // every chunk is the post-seek generation, at its own frame, with that frame's samples.
  const frames = 16
  const samples = Array.from({ length: frames }, (_, frame) => frame * 100)
  const bytes = pcm16(samples, 1)
  let openFirstRead
  const firstRead = new Promise((resolve) => { openFirstRead = resolve })
  let reads = 0
  const blob = {
    slice(start, end) {
      const read = reads
      reads += 1
      return {
        async arrayBuffer() {
          if (read === 0) await firstRead
          return bytes.slice(start, end).buffer
        },
      }
    },
  }
  const shared = createFixtureMsb1Ring({ channels: 1, frameCapacity: 4, capacity: 4 })
  const pump = new CanonicalPcmPump({
    lease: { async read() { return blob } },
    windowFrames: 8,
    sources: [{
      sourceId: "seek-in-flight",
      identity: `blake3:${"1".repeat(64)}`,
      channels: 1,
      bitDepth: 16,
      frames,
      ring: new Msb1RingWriter(shared),
    }],
  })
  const pass = pump.pumpUntilFull()
  while (reads === 0) await new Promise((resolve) => setImmediate(resolve))
  await pump.seek(4)
  openFirstRead()
  assert.deepEqual(await pass, { chunks: 3, frames: 12, finished: true })
  for (const [slot, start] of [4, 8, 12].entries()) {
    const chunk = inspectRing(shared, slot)
    assert.equal(chunk.generation, 2n, "no chunk carries the pre-seek generation")
    assert.equal(chunk.startFrame, BigInt(start))
    assert.equal(chunk.flags, start === 12 ? 1 : 0)
    assert.deepEqual(
      Array.from(chunk.planes[0]),
      samples.slice(start, start + 4).map((sample) => sample / 32_768),
      "a seek during a window read committed the pre-seek window"
    )
  }
  pump.stop()
}

async function rejectsWindowSmallerThanRingBeforeEngaging() {
  let engaged = false
  const ring = {
    channels: 1,
    frameCapacity: 3,
    capacity: 1,
    engage() { engaged = true },
  }
  assert.throws(
    () => new CanonicalPcmPump({
      lease: { async read() { throw new Error("unreachable") } },
      windowFrames: 2,
      sources: [{
        sourceId: "invalid-window",
        identity: `blake3:${"0".repeat(64)}`,
        channels: 1,
        bitDepth: 16,
        frames: 1,
        ring,
      }],
    }),
    /windowFrames must be at least/
  )
  assert.equal(engaged, false)
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
        identity: `blake3:${"0".repeat(64)}`,
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

// --- Issue #2263: a stopped pump refuses work ----------------------------------------------------

const SEEK_EPOCH = 11
const GENERATION_TAG = 10

async function stoppedPumpRefusesSeekAndPump() {
  // A stopped pump has released its rings. A seek that still succeeded would re-tag and re-seek a
  // ring no writer owns, and every later pass would report zero chunks and `finished: false`, which
  // is exactly what a full ring reports: the caller cannot tell a dead pump from a busy one.
  const shared = createFixtureMsb1Ring({ channels: 1, frameCapacity: 2, capacity: 2 })
  const control = new Int32Array(shared, 0, 128 / 4)
  const pump = new CanonicalPcmPump({
    lease: { async read() { throw new Error("unreachable: the pump never reads") } },
    windowFrames: 2,
    sources: [{
      sourceId: "stopped",
      identity: `blake3:${"3".repeat(64)}`,
      channels: 1,
      bitDepth: 16,
      frames: 8,
      ring: new Msb1RingWriter(shared),
    }],
  })
  pump.stop()
  const epoch = Atomics.load(control, SEEK_EPOCH)
  const tag = Atomics.load(control, GENERATION_TAG)
  const stopped = (error) =>
    error instanceof CanonicalPcmPumpError && error.code === "stem.pump.stopped"
  await assert.rejects(pump.seek(0), stopped, "a stopped pump accepted a seek")
  await assert.rejects(pump.seek(0, 9n), stopped, "a stopped pump accepted a session seek")
  await assert.rejects(pump.pumpUntilFull(), stopped, "a stopped pump reported a pass")
  assert.equal(Atomics.load(control, SEEK_EPOCH), epoch, "a refused seek published nothing")
  assert.equal(Atomics.load(control, GENERATION_TAG), tag, "a refused seek re-tagged the ring")
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
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
  )
  assert.deepEqual(index.stems[stemIdentity].pins, [
    "session:same-reopen:same",
  ])
  await gate.close()
  index = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
  )
  assert.deepEqual(index.stems[stemIdentity].pins, [])
}

const LOCKS = "miso:stem-store:v1:miso-stems-blake3-v1"
const tick = () => new Promise((resolve) => setTimeout(resolve, 0))

// Clears its timer, so a later test that counts live timers sees none of these.
async function settledWithin(promise, milliseconds) {
  let timer
  try {
    return await Promise.race([
      promise.then(
        (value) => ({ value }),
        (error) => ({ error })
      ),
      new Promise((resolve) => {
        timer = setTimeout(() => resolve("timed-out"), milliseconds)
      }),
    ])
  } finally {
    clearTimeout(timer)
  }
}

// Bounded, so a regression goes red here instead of hanging the gate and its mutation ledger.
async function until(predicate, what, ticks = 1_000) {
  for (let tried = 0; !(await predicate()); tried += 1) {
    if (tried === ticks) throw new Error(`timed out waiting for ${what}`)
    await tick()
  }
}

function untilPending(locks, name) {
  return until(
    async () => (await locks.query()).pending.some((lock) => lock.name === name),
    `a queued ${name} request`
  )
}

// Issue #2266: the claim matrix. An open aborted by any source, in any phase, rejects with an
// AbortError, never commits and never keeps its session pin, and the gate ends in the state that
// source implies. Sources: a newer open of the same session, a newer open of another session,
// close(), and the caller's own signal (the one source that leaves `#opening` unchanged). Phases:
// queued on a session pin another lease holds; during a resume wait that never settles; from the
// store's synchronous "ready" callback, after openSession's last signal check, so no abort event
// reaches the resume wait; and during the predecessor lease's close. The last phase, between the last
// await and the commit, is `abortAsResumeSettlesNeverCommits`.
const ABORT_SOURCES = ["same-session open", "other-session open", "close()", "caller signal"]
const OPEN_PHASES = ["queued on the pin", "resume wait", "from ready", "predecessor close"]

function abortFixture() {
  const bytes = pcm16([5, -5, 7, -7], 1)
  const stemIdentity = identity(bytes)
  const stems = [{ identity: stemIdentity, bytes: bytes.byteLength }]
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const store = new OpfsStemStore({ storage: backend.storage(), locks, tabId: "matrix" })
  const resolver = new MemoryStemResolver({ [stemIdentity]: bytes })
  const env = {
    stems,
    locks,
    store,
    resolver,
    gate: new StemSessionGate(store, resolver),
    caller: new AbortController(),
    pins: () =>
      JSON.parse(new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json")))
        .stems[stemIdentity].pins,
  }
  // Abort the first open (session "same") from `source`; a newer open is kept on `env.newer`.
  env.abortBy = (source) => {
    if (source === "close()") env.closing = env.gate.close()
    else if (source === "caller signal") env.caller.abort(new DOMException("Caller gave up", "AbortError"))
    else {
      env.newer = env.gate.open({
        sessionId: source === "same-session open" ? "same" : "other",
        stems,
        resume() {},
        onProgress(event) {
          if (event.stage === "ready") env.stateAtNewerReady = env.gate.state
        },
      })
    }
  }
  return env
}

// What every aborted open must leave behind, whatever aborted it.
async function assertAbortedOpenLeftNothing(env, source, label) {
  if (env.closing !== undefined) await env.closing
  if (env.newer !== undefined) {
    const newer = await settledWithin(env.newer, 500)
    assert.notEqual(newer, "timed-out", `the newer open never settled ${label}`)
    assert.equal(newer.error, undefined, `the newer open failed ${label}`)
    assert.equal(
      env.stateAtNewerReady,
      "loading",
      `the aborted open's refusal overwrote the newer open's loading state ${label}`
    )
    assert.equal(env.gate.state, "interactive", label)
  } else {
    assert.equal(
      env.gate.state,
      source === "close()" ? "idle" : "refused",
      `the gate's state after the abort ${label}`
    )
  }
  const owner = { "same-session open": "same", "other-session open": "other" }[source]
  assert.deepEqual(
    env.pins(),
    owner === undefined ? [] : [`session:matrix:${owner}`],
    `the pins after the abort ${label}`
  )
  if (owner !== "same") {
    const direct = await settledWithin(
      env.store.openSession({ sessionId: "same", stems: env.stems, resolver: env.resolver }),
      500
    )
    assert.notEqual(direct, "timed-out", `the aborted open kept its session pin ${label}`)
    assert.equal(direct.error, undefined, label)
    await direct.value.close()
  }
  await env.gate.close()
}

async function everyAbortSourceInEveryPhase() {
  for (const phase of OPEN_PHASES) {
    for (const source of ABORT_SOURCES) {
      const label = `(${source}, ${phase})`
      const env = abortFixture()
      const { gate, stems, locks } = env
      if (phase === "predecessor close") await gate.open({ sessionId: "previous", stems, resume() {} })
      const holder =
        phase === "queued on the pin"
          ? await env.store.openSession({ sessionId: "same", stems, resolver: env.resolver })
          : undefined
      let ready = false
      let unstall
      let stall
      const first = gate.open({
        sessionId: "same",
        stems,
        signal: env.caller.signal,
        // The predecessor close comes after the resume wait, so that phase needs a resume.
        resume: phase === "predecessor close" ? () => {} : () => new Promise(() => {}),
        onProgress(event) {
          if (event.stage !== "ready" || ready) return
          ready = true
          if (phase === "from ready") env.abortBy(source)
          // Hold the index so the predecessor's close, which rewrites it, waits.
          if (phase === "predecessor close") {
            stall = locks.request(`${LOCKS}:index`, {}, () => new Promise((resolve) => (unstall = resolve)))
          }
        },
      })
      const firstSettled = settledWithin(first, 500)
      if (phase === "queued on the pin") {
        await untilPending(locks, `${LOCKS}:pin:session:matrix:same`)
        env.abortBy(source)
      } else if (phase === "resume wait") {
        await until(() => ready, `the first lease ${label}`)
        await tick()
        env.abortBy(source)
      } else if (phase === "predecessor close") {
        await untilPending(locks, `${LOCKS}:index`)
        env.abortBy(source)
        unstall()
        await stall
      } else {
        await until(() => ready, `the first lease ${label}`)
      }
      const outcome = await firstSettled
      assert.notEqual(outcome, "timed-out", `the aborted open never settled ${label}`)
      assert.equal(outcome.error?.name, "AbortError", `the aborted open committed ${label}`)
      await holder?.close()
      await assertAbortedOpenLeftNothing(env, source, label)
    }
  }

  // A caller's signal that is already aborted refuses the open before it starts.
  const env = abortFixture()
  await assert.rejects(
    env.gate.open({ sessionId: "same", stems: env.stems, resume() {}, signal: AbortSignal.abort() }),
    (error) => error.name === "AbortError",
    "an open whose caller's signal was already aborted went ahead"
  )
  assert.equal(env.gate.state, "refused")
}

// Issue #2266: the last phase of the matrix. With no current lease, one check stands between an
// open's resume settling and its commit. Every abort source lands at each microtask offset 0-8
// after the first open's resume settles (its lease already granted). Before the commit the first
// open must lose and leave nothing; from the commit on, the abort is an ordinary later event.
async function abortAsResumeSettlesNeverCommits() {
  for (const source of ABORT_SOURCES) {
    let committedOffsets = 0
    for (let offset = 0; offset <= 8; offset += 1) {
      const label = `(${source}, after the resume settles, offset ${offset})`
      const env = abortFixture()
      let settleResume
      let resuming
      let ready = false
      const first = env.gate.open({
        sessionId: "same",
        stems: env.stems,
        signal: env.caller.signal,
        resume: () => (resuming = new Promise((resolve) => (settleResume = resolve))),
        onProgress(event) {
          if (event.stage === "ready") ready = true
        },
      })
      await until(() => ready, `the first lease ${label}`)
      await tick()
      let fired = false
      let committedFirst
      let chain = resuming
      for (let step = 0; step < offset; step += 1) chain = chain.then(() => {})
      void chain.then(() => {
        committedFirst = env.gate.state === "interactive"
        env.abortBy(source)
        fired = true
      })
      const firstSettled = settledWithin(first, 500)
      settleResume()
      await until(() => fired, `the abort ${label}`)
      const outcome = await firstSettled
      assert.notEqual(outcome, "timed-out", `the aborted open never settled ${label}`)
      if (committedFirst) {
        assert.equal(outcome.error, undefined, label)
        committedOffsets += 1
        await env.closing
        if (env.newer !== undefined) {
          const newer = await settledWithin(env.newer, 500)
          assert.notEqual(newer, "timed-out", `the newer open never settled ${label}`)
          assert.equal(newer.error, undefined, `the newer open failed after the commit ${label}`)
        }
        await env.gate.close()
      } else {
        assert.equal(outcome.error?.name, "AbortError", `the aborted open committed ${label}`)
        await assertAbortedOpenLeftNothing(env, source, label)
      }
    }
    // The sweep must reach past the commit, or it never crossed the window it is here to probe.
    assert.ok(committedOffsets > 0 && committedOffsets < 9, `the offset sweep spans the commit (${source})`)
  }
}

// Issue #2266: a resume() that throws synchronously refuses its open after that open has already
// aborted the one in flight, so it must leave a terminal state itself; with nothing in flight it
// must leave the current session alone.
async function throwingResumeLeavesATerminalState() {
  const bytes = pcm16([4, 4, -4, -4], 1)
  const stemIdentity = identity(bytes)
  const stems = [{ identity: stemIdentity, bytes: bytes.byteLength }]
  const gate = new StemSessionGate(
    new OpfsStemStore({
      storage: new FakeOpfsBackend().storage(),
      locks: new FakeLockManager(),
      tabId: "resume-throws",
    }),
    new MemoryStemResolver({ [stemIdentity]: bytes })
  )
  const throwing = {
    sessionId: "b",
    stems,
    resume() {
      throw new Error("no gesture")
    },
  }
  const inFlight = settledWithin(gate.open({ sessionId: "a", stems, resume() {} }), 500)
  await assert.rejects(gate.open(throwing), (error) => error.code === "session.resume.failed")
  assert.equal((await inFlight).error?.name, "AbortError")
  assert.equal(gate.state, "refused", "a refused open left the gate loading with no open in flight")

  const lease = await gate.open({ sessionId: "a", stems, resume() {} })
  await assert.rejects(gate.open(throwing), (error) => error.code === "session.resume.failed")
  assert.equal(gate.state, "interactive", "a refused resume displaced a session it never replaced")
  assert.equal((await lease.read(stemIdentity)).size, bytes.byteLength)

  // An open in flight while a lease is current: "loading" is the open in flight, not the lease.
  const behindLease = settledWithin(gate.open({ sessionId: "b", stems, resume() {} }), 500)
  await assert.rejects(
    gate.open({ ...throwing, sessionId: "c" }),
    (error) => error.code === "session.resume.failed"
  )
  assert.equal((await behindLease).error?.name, "AbortError")
  assert.equal(
    gate.state,
    "refused",
    "a refused open left the gate loading behind a current lease with no open in flight"
  )
  await gate.close()
}

// Issue #2266: close() then open() at once. The open owns the state from its first line; close()
// must not report idle over it once the old lease has closed.
async function closeNeverOverwritesALaterOpen() {
  const bytes = pcm16([1, 3, 5, 7], 1)
  const stemIdentity = identity(bytes)
  const stems = [{ identity: stemIdentity, bytes: bytes.byteLength }]
  const backend = new FakeOpfsBackend()
  const gate = new StemSessionGate(
    new OpfsStemStore({ storage: backend.storage(), locks: new FakeLockManager(), tabId: "reopen" }),
    new MemoryStemResolver({ [stemIdentity]: bytes })
  )
  await gate.open({ sessionId: "x", stems, resume() {} })
  const closing = gate.close()
  assert.equal(gate.state, "idle")
  const reopening = gate.open({ sessionId: "x", stems, resume() {} })
  await closing
  assert.equal(gate.state, "loading", "close() reported idle while a later open was loading")
  await reopening
  assert.equal(gate.state, "interactive")
  assert.deepEqual(
    JSON.parse(new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json")))
      .stems[stemIdentity].pins,
    ["session:reopen:x"]
  )
  await gate.close()
}

// Issue #2266: once a superseded open has settled, close() must still reach the newest open (here
// queued on a session pin another lease holds), and leave the gate idle.
async function closeReachesTheNewestOpen() {
  const bytes = pcm16([2, 4, 6, 8], 1)
  const stemIdentity = identity(bytes)
  const stems = [{ identity: stemIdentity, bytes: bytes.byteLength }]
  const locks = new FakeLockManager()
  const store = new OpfsStemStore({
    storage: new FakeOpfsBackend().storage(),
    locks,
    tabId: "close-newest",
  })
  const resolver = new MemoryStemResolver({ [stemIdentity]: bytes })
  const holder = await store.openSession({ sessionId: "held", stems, resolver })
  const gate = new StemSessionGate(store, resolver)
  let ready = false
  const first = settledWithin(
    gate.open({
      sessionId: "other",
      stems,
      resume: () => new Promise(() => {}),
      onProgress(event) {
        if (event.stage === "ready") ready = true
      },
    }),
    500
  )
  await until(() => ready, "the first lease")
  const newest = settledWithin(gate.open({ sessionId: "held", stems, resume() {} }), 500)
  assert.equal((await first).error?.name, "AbortError")
  await untilPending(locks, `${LOCKS}:pin:session:close-newest:held`)
  await gate.close()
  const outcome = await newest
  assert.notEqual(outcome, "timed-out", "close() no longer reached the newest open")
  assert.equal(outcome.error?.name, "AbortError")
  assert.equal(gate.state, "idle", "a closed open's refusal overwrote the closed gate's idle")
  await holder.close()
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
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
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
    expectedChunks: 2,
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
    { type: "pumped", requestId: "pump", chunks: 2, frames: 5, finished: true },
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

  // `until`'s own 1000 ms deadline (below) is the functional assertion that stop is prompt: it
  // throws "timed out waiting for the stop reply" if the idle sleep is not actually interrupted.
  // A follow-up `Date.now() - started < 1000` here would be a tautology -- it can only observe
  // what `until` has already enforced by not throwing.
  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply", 1000)
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

// --- Issue #2263: a failed Worker session drives nothing and refuses seek and pump ---------------

/** The reply that answers `requestId`, whatever its type. */
function answer(worker, requestId) {
  return worker.posted.find((message) => message.requestId === requestId)
}

function idleTimers() {
  return process.getActiveResourcesInfo().filter((resource) => resource === "Timeout").length
}

async function failedSessionRefusesSeekAndPump() {
  // The source declares nine frames but the stored stem holds five, so the cadence's second window
  // comes back truncated and the session fails on the drive path. A `seek` after that used to be
  // answered `sought` and re-arm the loop against the stopped pump: zero chunks and
  // `finished: false` on every pass, a 1 ms sleep, forever, with nothing reported.
  const fixture = { ...cadenceFixture(), frames: 9 }
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "failed-session")
  const shared = createFixtureMsb1Ring(fixture.ring)
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("failed-session")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "failed-session",
      selfDriving: { idleMs: 1 },
    })
  )
  await worker.until(() => worker.reply("session-error") !== undefined, "the truncation to fail the session")
  assert.match(worker.reply("session-error").error.message, /PCM window is truncated/)
  assert.equal(Atomics.load(control, WRITER_STATE), 0, "the failure released the ring")
  const epoch = Atomics.load(control, SEEK_EPOCH)

  const refused = (requestId) => {
    const reply = answer(worker, requestId)
    assert.equal(reply?.type, "session-error", `a failed session answered \`${requestId}\` with ${reply?.type}`)
    assert.equal(
      reply.error.code,
      "stem.pump.session_failed",
      "the refusal must come from the failed session, not from a pump it still held"
    )
    assert.match(
      String(reply.error.details?.failure?.message),
      /PCM window is truncated/,
      `the \`${requestId}\` refusal names the failure that ended the session`
    )
  }
  worker.send({ type: "seek", requestId: "seek", frame: 0 })
  await worker.until(() => answer(worker, "seek") !== undefined, "the seek answer")
  refused("seek")
  worker.send({ type: "pump", requestId: "pump" })
  await worker.until(() => answer(worker, "pump") !== undefined, "the pump answer")
  refused("pump")
  assert.equal(Atomics.load(control, SEEK_EPOCH), epoch, "a refused seek published nothing")
  await new Promise((resolve) => setImmediate(resolve))
  assert.equal(idleTimers(), 0, "a failed session left the cadence sleeping")

  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()
}

async function messagePathFailureStopsTheCadence() {
  // A one-slot ring fills at once and the loop sleeps for 30 s. A `seek` the pump refuses fails the
  // session on the message path; the loop must end with it rather than wake against a stopped pump
  // and keep sleeping on it for as long as the Worker lives.
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "message-failure")
  const shared = createFixtureMsb1Ring({ channels: 2, frameCapacity: 3, capacity: 1 })
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("message-failure")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "message-failure",
      selfDriving: { idleMs: 30_000 },
    })
  )
  await worker.until(() => control[WROTE] === 1, "the one-slot ring to fill")
  worker.send({ type: "seek", requestId: "bad-seek", frame: -1 })
  await worker.until(() => answer(worker, "bad-seek") !== undefined, "the bad seek answer")
  assert.equal(answer(worker, "bad-seek").type, "session-error")
  await new Promise((resolve) => setImmediate(resolve))
  assert.equal(idleTimers(), 0, "a message-path failure left the cadence sleeping")
  assert.equal(Atomics.load(control, WRITER_STATE), 0, "a message-path failure released the ring")

  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()
}

async function refusedSelfDrivingEngagesNoRing() {
  // `selfDriving` is checked before the pump is built. Checked after, the pump would already have
  // engaged its rings when the option is refused, and a session that never began owns nothing that
  // would release them: the reader would see a live writer that never writes.
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "refused-option")
  const shared = createFixtureMsb1Ring(fixture.ring)
  const control = new Int32Array(shared, 0, 128 / 4)
  const worker = await mountWorker("refused-option")
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared,
      fixture,
      folderName: "refused-option",
      selfDriving: { idleMs: -1 },
    })
  )
  await worker.until(() => answer(worker, "init") !== undefined, "the initialize answer")
  assert.equal(answer(worker, "init").type, "session-error")
  assert.match(answer(worker, "init").error.message, /selfDriving\.idleMs/)
  assert.equal(
    Atomics.load(control, WRITER_STATE),
    0,
    "a refused selfDriving option left a ring engaged"
  )
  await lease.close()
}

/**
 * Wrap `navigator.storage` so the first window read of `fileName` calls `beforeReject` and then
 * rejects. Only that stem's file is wrapped; the store's index and directories are the fake's own.
 */
function failFirstWindowRead(fileName, beforeReject) {
  const storage = globalThis.navigator.storage
  let fired = false
  const forward = (target, wrap) =>
    new Proxy(target, {
      get(handle, property) {
        const wrapped = wrap(handle, property)
        if (wrapped !== undefined) return wrapped
        const value = Reflect.get(handle, property)
        return typeof value === "function" ? value.bind(handle) : value
      },
    })
  const file = (handle) =>
    forward(handle, (target, property) => {
      if (property !== "getFile") return undefined
      return async () => {
        const blob = await target.getFile()
        return {
          slice(start, end) {
            if (fired) return blob.slice(start, end)
            fired = true
            return {
              arrayBuffer() {
                beforeReject()
                return Promise.reject(new Error("synthetic window read failure"))
              },
            }
          },
        }
      }
    })
  const directory = (handle) =>
    forward(handle, (target, property) => {
      if (property === "getDirectoryHandle") {
        return async (...args) => directory(await target.getDirectoryHandle(...args))
      }
      if (property === "getFileHandle") {
        return async (name, ...rest) => {
          const opened = await target.getFileHandle(name, ...rest)
          return name === fileName ? file(opened) : opened
        }
      }
      return undefined
    })
  Object.defineProperty(globalThis, "navigator", {
    configurable: true,
    value: {
      ...globalThis.navigator,
      storage: { ...storage, getDirectory: async () => directory(await storage.getDirectory()) },
    },
  })
}

async function cadenceFailureIsRecordedBeforeTheNextMessage() {
  // The cadence's first window read fails, and the host's next `initialize` arrives while that
  // read is still in flight. The failed pass must be recorded inside the message tail, before that
  // `initialize` runs. Recorded later, it would land on the session `initialize` just started and
  // fail it.
  const fixture = cadenceFixture()
  const { identity: stemIdentity, lease } = await stagedStore(fixture.bytes, "tail-failure")
  const first = createFixtureMsb1Ring(fixture.ring)
  const second = createFixtureMsb1Ring(fixture.ring)
  const worker = await mountWorker("tail-failure")
  const reinitialize = {
    ...initializeMessage({ identity: stemIdentity, shared: second, fixture, folderName: "tail-failure" }),
    requestId: "second-init",
  }
  failFirstWindowRead(`blake3-${stemIdentity.slice("blake3:".length)}`, () => worker.send(reinitialize))
  worker.send(
    initializeMessage({
      identity: stemIdentity,
      shared: first,
      fixture,
      folderName: "tail-failure",
      selfDriving: { idleMs: 1 },
    })
  )
  await worker.until(() => answer(worker, "second-init") !== undefined, "the second initialize answer")
  await new Promise((resolve) => setTimeout(resolve, 30))
  assert.deepEqual(
    worker.posted.map((message) => message.type),
    ["initialized", "session-error", "initialized"],
    "the failed pass was recorded before the next message ran, and only once"
  )
  worker.send({ type: "pump", requestId: "second-pump" })
  await worker.until(() => answer(worker, "second-pump") !== undefined, "the second pump answer")
  assert.equal(
    answer(worker, "second-pump").type,
    "pumped",
    "the first session's failure failed the session after it"
  )

  worker.send({ type: "stop", requestId: "stop" })
  await worker.until(() => worker.reply("stopped") !== undefined, "the stop reply")
  await lease.close()
}

await transformContract()
await workerPumpContract()
await unalignedSeekKeepsChunksQuantumShaped()
await windowReadsOverlapAcrossSources()
await seekDuringReadCommitsNoStaleWindow()
await truncatedReadAheadStopsOnce()
await rejectsWindowSmallerThanRingBeforeEngaging()
await readFailureHardStops()
await stoppedPumpRefusesSeekAndPump()
await resumeInsideGestureGate()
await sameSessionReplacementReleasesPredecessorPin()
await everyAbortSourceInEveryPhase()
await abortAsResumeSettlesNeverCommits()
await throwingResumeLeavesATerminalState()
await closeNeverOverwritesALaterOpen()
await closeReachesTheNewestOpen()
await refusedGateNeverBecomesInteractive()
await rejectedResumeDoesNotLeakPins()
await selfDrivingFillsWithoutPumpMessages()
await stopInterruptsTheIdleLoop()
await seekReArmsAFinishedLoop()
await failedSessionRefusesSeekAndPump()
await messagePathFailureStopsTheCadence()
await refusedSelfDrivingEngagesNoRing()
await cadenceFailureIsRecordedBeforeTheNextMessage()
process.stdout.write("stem-pump-v1: PASS\n")
