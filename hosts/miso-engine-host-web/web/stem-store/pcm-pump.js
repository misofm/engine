import { StemStoreError } from "./opfs-store.js"

const MSB1_MAGIC = 0x4d534231
const MSB1_VERSION = 1
const MSB1_WRAP = 1 << 30
const CONTROL_BYTES = 128
const CONTROL_I64_OFFSET = 112
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

/** The Worker-side writer for the unchanged MSB1 ring format. */
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
    const control = new Int32Array(shared, 0, CONTROL_BYTES / 4)
    if (
      Atomics.load(control, CONTROL.MAGIC) !== MSB1_MAGIC ||
      control[CONTROL.VERSION] !== MSB1_VERSION
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
    this.#controlI64 = new BigInt64Array(shared, CONTROL_I64_OFFSET, 2)
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

  seek(generation, frame) {
    this.#controlI64[0] = generation
    this.#controlI64[1] = frame
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
 * Bounded-window store pump. A Blob snapshot is lazy; only one read-ahead
 * window per source becomes an ArrayBuffer. It never resolves a URL.
 */
export class CanonicalPcmPump {
  #lease
  #states
  #windowFrames
  #generation
  #stopped = false
  #onError

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
    this.#states = options.sources.map((source) => ({
      ...validateSource(source),
      cursor: 0,
      blob: null,
      window: null,
      windowStart: -1,
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
    let chunks = 0
    let frames = 0
    try {
      for (const state of this.#states) {
        while (!state.finished && state.ring.occupancy < state.ring.capacity) {
          const written = await this.#writeOne(state)
          if (written === 0) break
          chunks += 1
          frames += written
        }
      }
      return { chunks, frames, finished: this.finished }
    } catch (error) {
      this.stop()
      this.#onError?.(error)
      throw error
    }
  }

  async seek(frame) {
    const target = positiveOrZeroInteger(frame, "frame")
    this.#generation += 1n
    for (const state of this.#states) {
      state.cursor = Math.min(target, state.frames)
      state.window = null
      state.windowStart = -1
      state.finished = state.cursor === state.frames
      state.ring.seek(this.#generation, BigInt(state.cursor))
    }
  }

  stop() {
    if (this.#stopped) return
    this.#stopped = true
    for (const state of this.#states) state.ring.release()
  }

  async #writeOne(state) {
    if (state.cursor >= state.frames) {
      state.finished = true
      return 0
    }
    const windowStart =
      Math.floor(state.cursor / this.#windowFrames) * this.#windowFrames
    const frames = Math.min(
      state.ring.frameCapacity,
      state.frames - state.cursor,
      windowStart + this.#windowFrames - state.cursor
    )
    const planes = state.ring.reserve(frames)
    if (planes === null) return 0
    const bytesPerSample = state.bitDepth / 8
    const frameBytes = state.channels * bytesPerSample
    if (state.window === null || state.windowStart !== windowStart) {
      state.blob ??= await this.#lease.read(state.identity)
      const windowEnd = Math.min(state.frames, windowStart + this.#windowFrames)
      const firstByte = windowStart * frameBytes
      const finalByte = windowEnd * frameBytes
      state.window = new Uint8Array(
        await state.blob.slice(firstByte, finalByte).arrayBuffer()
      )
      state.windowStart = windowStart
    }
    const localFrame = state.cursor - state.windowStart
    deinterleaveCanonicalPcm(
      state.window,
      localFrame,
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
    return frames
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
  const headerOffset = 256
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
    throw new StemStoreError(
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
