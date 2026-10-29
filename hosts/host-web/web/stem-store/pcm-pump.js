const MSB1_MAGIC = 0x4d534231
const MSB1_VERSION = 1
const MSB1_WRAP = 1 << 30
// Issue #1297b D2: the SDK's MSB1 V1 layout (`sdk/src/browser/pcm-ring.ts`), amended in place:
// three i64 at 112 (seek generation, frame, anchor sample; -1 is a plain seek), the ID at 144 and
// the slot headers at 272. The feed drain refuses a ring with any other header offset.
const CONTROL_BYTES = 144
const CONTROL_I64_OFFSET = 112
const HEADER_OFFSET = 272
const NO_ANCHOR = -1n
const SLOT_HEADER_BYTES = 32
const FLAG_END_OF_REGION = 1

const CONTROL = Object.freeze({
  MAGIC: 0,
  VERSION: 1,
  CAPACITY: 2,
  CHANNELS: 3,
  FRAME_CAPACITY: 4,
  HEADER_OFFSET: 5,
  PCM_OFFSET: 6,
  WRITE_INDEX: 8,
  READ_INDEX: 9,
  GENERATION_TAG: 10,
  SEEK_EPOCH: 11,
  WRITER_STATE: 12,
  WROTE: 14,
  OVERFLOW: 15,
})

/**
 * A typed pump refusal. The pump owns it, so the module imports nothing and the SDK publishes it
 * as a verbatim copy (issue #2102).
 */
export class CanonicalPcmPumpError extends Error {
  /**
   * @param {string} code
   * @param {string} message
   * @param {Record<string, unknown>} [details]
   */
  constructor(code, message, details = {}) {
    super(message)
    this.name = "CanonicalPcmPumpError"
    this.code = code
    this.details = Object.freeze({ ...details })
  }
}

/** The Worker-side writer for the MSB1 V1 ring format. It publishes plain seeks only. */
export class Msb1RingWriter {
  #control
  #controlI64
  #headers
  #headersI64
  #planes
  #reserved = -1

  constructor(shared) {
    if (!(shared instanceof SharedArrayBuffer)) {
      throw new TypeError("MSB1 ring must be a SharedArrayBuffer")
    }
    if (shared.byteLength < HEADER_OFFSET) {
      throw new TypeError("Shared buffer is not an MSB1 source ring")
    }
    const control = new Int32Array(shared, 0, CONTROL_BYTES / 4)
    if (
      Atomics.load(control, CONTROL.MAGIC) !== MSB1_MAGIC ||
      control[CONTROL.VERSION] !== MSB1_VERSION ||
      control[CONTROL.HEADER_OFFSET] !== HEADER_OFFSET
    ) {
      throw new TypeError("Shared buffer is not an MSB1 source ring")
    }
    const capacity = control[CONTROL.CAPACITY]
    const channels = control[CONTROL.CHANNELS]
    const frameCapacity = control[CONTROL.FRAME_CAPACITY]
    if (!isPowerOfTwo(capacity) || channels <= 0 || frameCapacity <= 0) {
      throw new TypeError("MSB1 ring header is invalid")
    }
    this.capacity = capacity
    this.channels = channels
    this.frameCapacity = frameCapacity
    this.#control = control
    this.#controlI64 = new BigInt64Array(shared, CONTROL_I64_OFFSET, 3)
    this.#headers = new Int32Array(
      shared,
      control[CONTROL.HEADER_OFFSET],
      (capacity * SLOT_HEADER_BYTES) / 4
    )
    this.#headersI64 = new BigInt64Array(
      shared,
      control[CONTROL.HEADER_OFFSET],
      (capacity * SLOT_HEADER_BYTES) / 8
    )
    this.#planes = Array.from({ length: capacity }, (_, slot) =>
      Array.from(
        { length: channels },
        (_, channel) =>
          new Float32Array(
            shared,
            control[CONTROL.PCM_OFFSET] +
              (slot * channels + channel) * frameCapacity * 4,
            frameCapacity
          )
      )
    )
  }

  engage(generation) {
    this.#control[CONTROL.GENERATION_TAG] = Number(BigInt.asIntN(32, generation))
    Atomics.store(this.#control, CONTROL.WRITER_STATE, 1)
  }

  get occupancy() {
    const write = this.#control[CONTROL.WRITE_INDEX]
    const read = Atomics.load(this.#control, CONTROL.READ_INDEX)
    return (write - read) & (MSB1_WRAP - 1)
  }

  reserve(frames) {
    if (!Number.isInteger(frames) || frames <= 0 || frames > this.frameCapacity) {
      throw new RangeError("PCM chunk does not fit the MSB1 slot")
    }
    if (this.occupancy >= this.capacity) {
      this.#control[CONTROL.OVERFLOW] += 1
      return null
    }
    const index = this.#control[CONTROL.WRITE_INDEX]
    this.#reserved = index
    return this.#planes[index & (this.capacity - 1)]
  }

  commit({ generation, startFrame, frames, endOfRegion }) {
    const index = this.#reserved
    if (index < 0) throw new Error("MSB1 commit without a reservation")
    this.#reserved = -1
    const slot = index & (this.capacity - 1)
    const word = slot * (SLOT_HEADER_BYTES / 4)
    const word64 = slot * (SLOT_HEADER_BYTES / 8)
    this.#headers[word] = index
    this.#headers[word + 1] = Number(BigInt.asIntN(32, generation))
    this.#headers[word + 2] = frames
    this.#headers[word + 3] = endOfRegion ? FLAG_END_OF_REGION : 0
    this.#headersI64[word64 + 2] = generation
    this.#headersI64[word64 + 3] = startFrame
    this.#control[CONTROL.WROTE] += 1
    Atomics.store(
      this.#control,
      CONTROL.WRITE_INDEX,
      (index + 1) & (MSB1_WRAP - 1)
    )
  }

  /** Issue #2184: the tuple and the tag are published between two steps of the epoch (a
   *  parity seqlock): odd while they are written, even again after, so one seek steps it by 2. */
  seek(generation, frame) {
    Atomics.add(this.#control, CONTROL.SEEK_EPOCH, 1)
    Atomics.store(this.#controlI64, 0, generation)
    Atomics.store(this.#controlI64, 1, frame)
    Atomics.store(this.#controlI64, 2, NO_ANCHOR)
    Atomics.store(
      this.#control,
      CONTROL.GENERATION_TAG,
      Number(BigInt.asIntN(32, generation))
    )
    Atomics.add(this.#control, CONTROL.SEEK_EPOCH, 1)
  }

  release() {
    Atomics.store(this.#control, CONTROL.WRITER_STATE, 0)
  }
}

/**
 * The most window reads one pump has in flight at once, over all its sources (issue #2118).
 *
 * Each read is one `Blob.slice().arrayBuffer()`, which the browser serves off the calling thread,
 * so reads of different stems overlap and a 64-stem window wave costs ceil(64 / 8) read latencies
 * instead of 64. The bound keeps the request count the browser's blob and file I/O pool must queue
 * small and caps the read buffers being filled at once at eight windows; the per-source rule below
 * (one window resident, one ahead) bounds the rest.
 */
const MAX_READS_IN_FLIGHT = 8

/**
 * Bounded-window store pump. A Blob snapshot is lazy; per source at most two windows become
 * ArrayBuffers: the one being written and the next one, read ahead. A read a seek superseded may
 * still finish, so the bound adds at most `MAX_READS_IN_FLIGHT` windows briefly. It never resolves
 * a URL.
 *
 * Window reads run concurrently across sources, at most `MAX_READS_IN_FLIGHT` at once and the rest
 * queued in issue order. Every ring write (reserve, deinterleave, commit) is synchronous, so each
 * ring's chunk order is exact. A read carries the generation it was issued under and its own start
 * frame; a seek supersedes it, and a superseded read is never installed, so no pre-seek window can
 * reach a ring.
 */
export class CanonicalPcmPump {
  #lease
  #states
  #windowFrames
  #generation
  #stopped = false
  #onError
  #readsInFlight = 0
  #queuedReads = []

  /**
   * @param {{lease: {read: Function}, sources: Array<{sourceId: string, identity: string, channels: number, bitDepth: 16 | 24, frames: number, ring: object}>, windowFrames?: number, generation?: bigint, onError?: (error: unknown) => void}} options
   */
  constructor(options) {
    if (typeof options.lease?.read !== "function") {
      throw new TypeError("CanonicalPcmPump needs an open store lease")
    }
    this.#lease = options.lease
    this.#windowFrames = positiveInteger(options.windowFrames ?? 4096, "windowFrames")
    this.#generation = options.generation ?? 1n
    this.#onError = options.onError
    const sources = options.sources.map(validateSource)
    for (const source of sources) {
      if (this.#windowFrames < source.ring.frameCapacity) {
        throw new RangeError("windowFrames must be at least the MSB1 ring frame capacity")
      }
    }
    this.#states = sources.map((source) => ({
      ...source,
      frameBytes: (source.channels * source.bitDepth) / 8,
      cursor: 0,
      blob: null,
      window: null,
      windowStart: -1,
      ahead: null,
      finished: false,
    }))
    for (const state of this.#states) state.ring.engage(this.#generation)
  }

  get finished() {
    return this.#states.every((state) => state.finished)
  }

  /** Fill every ring until it is full or its source reaches end-of-region. */
  async pumpUntilFull() {
    if (this.#stopped) return { chunks: 0, frames: 0, finished: this.finished }
    const totals = { chunks: 0, frames: 0 }
    try {
      await Promise.all(this.#states.map((state) => this.#fill(state, totals)))
      return { ...totals, finished: this.finished }
    } catch (error) {
      this.stop()
      this.#onError?.(error)
      throw error
    }
  }

  /**
   * Reposition every stem to `frame`, clamped to its region, under a new generation. With
   * `generation` (issue #1644 D3) the pump uses that one, which must be above its own: pass the
   * generation of a session seek the engine accepted, and only after it did. Without one it uses
   * its own next generation. A window read still in flight is superseded and never installed.
   */
  async seek(frame, generation) {
    const target = positiveOrZeroInteger(frame, "frame")
    if (generation !== undefined) {
      if (typeof generation !== "bigint" || generation <= this.#generation) {
        throw new RangeError("seek generation must be a bigint above the pump's own")
      }
      this.#generation = generation
    } else {
      this.#generation += 1n
    }
    for (const state of this.#states) {
      state.cursor = Math.min(target, state.frames)
      state.window = null
      state.windowStart = -1
      state.ahead = null
      state.finished = state.cursor === state.frames
      state.ring.seek(this.#generation, BigInt(state.cursor))
    }
  }

  stop() {
    if (this.#stopped) return
    this.#stopped = true
    for (const state of this.#states) state.ring.release()
  }

  /** One source's share of a pass: write chunks while the window covers them, else await one. */
  async #fill(state, totals) {
    while (!this.#stopped && !state.finished) {
      if (state.cursor >= state.frames) {
        state.finished = true
        return
      }
      if (state.ring.occupancy >= state.ring.capacity) return
      if (this.#windowCovers(state)) {
        if (!this.#writeChunk(state, totals)) return
        continue
      }
      let read = state.ahead
      if (read === null || read.start !== state.cursor) read = this.#startRead(state, state.cursor)
      const bytes = await read.done
      if (this.#stopped) return
      // A seek (or another pass) moved on while this read was in flight: drop it, re-decide.
      if (read.generation !== this.#generation || state.ahead !== read) continue
      state.window = bytes
      state.windowStart = read.start
      state.ahead = null
      this.#readAhead(state)
    }
  }

  #windowCovers(state) {
    if (state.window === null || state.cursor < state.windowStart) return false
    const frames = Math.min(state.ring.frameCapacity, state.frames - state.cursor)
    const windowEnd = state.windowStart + state.window.byteLength / state.frameBytes
    return state.cursor + frames <= windowEnd
  }

  /** Reserve, deinterleave and commit one chunk, with no await in between. */
  #writeChunk(state, totals) {
    const frames = Math.min(state.ring.frameCapacity, state.frames - state.cursor)
    const planes = state.ring.reserve(frames)
    if (planes === null) return false
    deinterleaveCanonicalPcm(
      state.window,
      state.cursor - state.windowStart,
      frames,
      state.channels,
      state.bitDepth,
      planes
    )
    const startFrame = state.cursor
    state.cursor += frames
    state.finished = state.cursor === state.frames
    state.ring.commit({
      generation: this.#generation,
      startFrame: BigInt(startFrame),
      frames,
      endOfRegion: state.finished,
    })
    totals.chunks += 1
    totals.frames += frames
    return true
  }

  /**
   * Start reading the window the ring will need after the resident one: it begins at the first
   * chunk the resident window cannot hold whole, since chunks step by the ring's frame capacity.
   */
  #readAhead(state) {
    const windowEnd = state.windowStart + state.window.byteLength / state.frameBytes
    if (windowEnd >= state.frames) return
    const step = state.ring.frameCapacity
    const next = state.cursor + step * Math.floor((windowEnd - state.cursor) / step)
    this.#startRead(state, next)
  }

  #startRead(state, start) {
    const read = { start, generation: this.#generation, done: null }
    state.ahead = read
    read.done = this.#schedule(async () => {
      // Superseded while it queued: skip the I/O.
      if (this.#stopped || read.generation !== this.#generation || state.ahead !== read) {
        return null
      }
      state.blob ??= await this.#lease.read(state.identity)
      const end = Math.min(state.frames, start + this.#windowFrames)
      const bytes = new Uint8Array(
        await state.blob
          .slice(start * state.frameBytes, end * state.frameBytes)
          .arrayBuffer()
      )
      // A Blob shorter than its declared frames would never cover the chunk, and the fill would
      // re-read the same short window without end.
      if (bytes.byteLength !== (end - start) * state.frameBytes) {
        throw new RangeError("PCM window is truncated")
      }
      return bytes
    })
    // A read ahead may fail while no pass awaits it; the pass that needs it rethrows.
    read.done.catch(() => {})
    return read
  }

  /** Run `task` now if fewer than `MAX_READS_IN_FLIGHT` reads are in flight, else queue it FIFO. */
  #schedule(task) {
    return new Promise((resolve, reject) => {
      const job = { task, resolve, reject }
      if (this.#readsInFlight < MAX_READS_IN_FLIGHT) this.#run(job)
      else this.#queuedReads.push(job)
    })
  }

  #run(job) {
    this.#readsInFlight += 1
    Promise.resolve()
      .then(job.task)
      .then(job.resolve, job.reject)
      .finally(() => {
        this.#readsInFlight -= 1
        const next = this.#queuedReads.shift()
        if (next !== undefined) this.#run(next)
      })
  }
}

/** Canonical interleaved little-endian integer PCM to planar f32. */
export function deinterleaveCanonicalPcm(
  bytes,
  firstFrame,
  frames,
  channels,
  bitDepth,
  planes
) {
  if (!(bytes instanceof Uint8Array)) throw new TypeError("PCM window must be bytes")
  if (
    planes.length !== channels ||
    planes.some((plane) => !(plane instanceof Float32Array) || plane.length < frames)
  ) {
    throw new RangeError("PCM plane shape mismatch")
  }
  const bytesPerSample = bitDepth / 8
  const frameBytes = channels * bytesPerSample
  const needed = (firstFrame + frames) * frameBytes
  if (needed > bytes.byteLength) throw new RangeError("PCM window is truncated")
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)
  for (let frame = 0; frame < frames; frame += 1) {
    for (let channel = 0; channel < channels; channel += 1) {
      const offset = (firstFrame + frame) * frameBytes + channel * bytesPerSample
      let sample
      if (bitDepth === 16) {
        sample = view.getInt16(offset, true) / 32_768
      } else if (bitDepth === 24) {
        sample = bytes[offset] | (bytes[offset + 1] << 8) | (bytes[offset + 2] << 16)
        if ((sample & 0x80_0000) !== 0) sample |= 0xff00_0000
        sample /= 8_388_608
      } else {
        throw new RangeError("Canonical PCM bit depth must be 16 or 24")
      }
      planes[channel][frame] = sample
    }
  }
}

/** Test/helper constructor for an MSB1 ring without app-owned plumbing. */
export function createFixtureMsb1Ring({ channels, frameCapacity, capacity = 4 }) {
  if (!isPowerOfTwo(capacity)) throw new RangeError("MSB1 capacity must be a power of two")
  const headerOffset = HEADER_OFFSET
  const pcmOffset = headerOffset + capacity * SLOT_HEADER_BYTES
  const shared = new SharedArrayBuffer(
    pcmOffset + capacity * channels * frameCapacity * 4
  )
  const control = new Int32Array(shared, 0, CONTROL_BYTES / 4)
  control[CONTROL.CAPACITY] = capacity
  control[CONTROL.CHANNELS] = channels
  control[CONTROL.FRAME_CAPACITY] = frameCapacity
  control[CONTROL.HEADER_OFFSET] = headerOffset
  control[CONTROL.PCM_OFFSET] = pcmOffset
  control[CONTROL.VERSION] = MSB1_VERSION
  new BigInt64Array(shared, CONTROL_I64_OFFSET, 3)[2] = NO_ANCHOR
  Atomics.store(control, CONTROL.MAGIC, MSB1_MAGIC)
  return shared
}

function validateSource(source) {
  if (typeof source.sourceId !== "string" || source.sourceId.length === 0) {
    throw new TypeError("pump sourceId must be non-empty")
  }
  if (typeof source.identity !== "string") throw new TypeError("pump identity is missing")
  const channels = positiveInteger(source.channels, "channels")
  const frames = positiveOrZeroInteger(source.frames, "frames")
  if (source.bitDepth !== 16 && source.bitDepth !== 24) {
    throw new RangeError("bitDepth must be 16 or 24")
  }
  if (
    source.ring?.channels !== channels ||
    !Number.isSafeInteger(source.ring?.frameCapacity) ||
    source.ring.frameCapacity <= 0
  ) {
    throw new CanonicalPcmPumpError(
      "stem.pump.ring_shape",
      `MSB1 ring shape does not match ${source.sourceId}`,
      { sourceId: source.sourceId }
    )
  }
  return { ...source, channels, frames }
}

function isPowerOfTwo(value) {
  return Number.isInteger(value) && value > 0 && (value & (value - 1)) === 0
}

function positiveInteger(value, name) {
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new RangeError(`${name} must be a positive safe integer`)
  }
  return value
}

function positiveOrZeroInteger(value, name) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new RangeError(`${name} must be a nonnegative safe integer`)
  }
  return value
}
