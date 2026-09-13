/*
 * Incremental BLAKE3-256 for canonical-PCM ingest and verify-on-open.
 *
 * The browser has no streaming Web Crypto BLAKE3 interface. This is a
 * repository-owned, bounded-memory implementation of the BLAKE3 hash mode;
 * the corpus tests cover official vectors and hostile chunk boundaries.
 */

import { hexLower } from "../hex-lower.js"

const IV = new Uint32Array([
  0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
  0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
])
const MESSAGE_PERMUTATION = new Uint8Array([
  2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8,
])
const BLOCK_BYTES = 64
const CHUNK_BYTES = 1024
const MAX_CHUNKS = 1n << 64n
const CHUNK_START = 1
const CHUNK_END = 2
const PARENT = 4
const ROOT = 8

const rotateRight = (value, count) =>
  (value >>> count) | (value << (32 - count))

/** @param {Uint32Array} state @param {number} a @param {number} b @param {number} c @param {number} d @param {number} x @param {number} y */
function mix(state, a, b, c, d, x, y) {
  state[a] = (state[a] + state[b] + x) >>> 0
  state[d] = rotateRight(state[d] ^ state[a], 16)
  state[c] = (state[c] + state[d]) >>> 0
  state[b] = rotateRight(state[b] ^ state[c], 12)
  state[a] = (state[a] + state[b] + y) >>> 0
  state[d] = rotateRight(state[d] ^ state[a], 8)
  state[c] = (state[c] + state[d]) >>> 0
  state[b] = rotateRight(state[b] ^ state[c], 7)
}

/** @param {Uint8Array} block */
function blockWords(block) {
  const words = new Uint32Array(16)
  const view = new DataView(block.buffer, block.byteOffset, BLOCK_BYTES)
  for (let index = 0; index < words.length; index += 1) {
    words[index] = view.getUint32(index * 4, true)
  }
  return words
}

/** @param {Uint32Array} chainingValue @param {Uint32Array} words @param {bigint} counter @param {number} blockLength @param {number} flags */
function compress(chainingValue, words, counter, blockLength, flags) {
  const state = new Uint32Array(16)
  state.set(chainingValue)
  state.set(IV.subarray(0, 4), 8)
  state[12] = Number(counter & 0xffff_ffffn)
  state[13] = Number((counter >> 32n) & 0xffff_ffffn)
  state[14] = blockLength
  state[15] = flags
  let message = words
  for (let round = 0; round < 7; round += 1) {
    mix(state, 0, 4, 8, 12, message[0], message[1])
    mix(state, 1, 5, 9, 13, message[2], message[3])
    mix(state, 2, 6, 10, 14, message[4], message[5])
    mix(state, 3, 7, 11, 15, message[6], message[7])
    mix(state, 0, 5, 10, 15, message[8], message[9])
    mix(state, 1, 6, 11, 12, message[10], message[11])
    mix(state, 2, 7, 8, 13, message[12], message[13])
    mix(state, 3, 4, 9, 14, message[14], message[15])
    if (round !== 6) {
      const next = new Uint32Array(16)
      for (let index = 0; index < next.length; index += 1) next[index] = message[MESSAGE_PERMUTATION[index]]
      message = next
    }
  }
  const output = new Uint32Array(16)
  for (let index = 0; index < 8; index += 1) {
    output[index] = state[index] ^ state[index + 8]
    output[index + 8] = state[index + 8] ^ chainingValue[index]
  }
  return output
}

class Output {
  /** @param {Uint32Array} inputChainingValue @param {Uint32Array} words @param {bigint} counter @param {number} blockLength @param {number} flags */
  constructor(inputChainingValue, words, counter, blockLength, flags) {
    this.inputChainingValue = inputChainingValue
    this.words = words
    this.counter = counter
    this.blockLength = blockLength
    this.flags = flags
  }

  chainingValue() {
    return compress(
      this.inputChainingValue,
      this.words,
      this.counter,
      this.blockLength,
      this.flags,
    ).subarray(0, 8)
  }

  rootDigest() {
    const output = compress(
      this.inputChainingValue,
      this.words,
      0n,
      this.blockLength,
      this.flags | ROOT,
    )
    const digest = new Uint8Array(32)
    const view = new DataView(digest.buffer)
    for (let index = 0; index < 8; index += 1) view.setUint32(index * 4, output[index], true)
    return digest
  }
}

class ChunkState {
  /** @param {bigint} counter */
  constructor(counter) {
    this.chainingValue = new Uint32Array(IV)
    this.counter = counter
    this.block = new Uint8Array(BLOCK_BYTES)
    this.blockLength = 0
    this.blocksCompressed = 0
  }

  length() {
    return this.blocksCompressed * BLOCK_BYTES + this.blockLength
  }

  /** @param {Uint8Array} input */
  update(input) {
    let offset = 0
    while (offset < input.byteLength) {
      if (this.blockLength === BLOCK_BYTES) {
        const flags = this.blocksCompressed === 0 ? CHUNK_START : 0
        this.chainingValue = compress(
          this.chainingValue,
          blockWords(this.block),
          this.counter,
          BLOCK_BYTES,
          flags,
        ).subarray(0, 8)
        this.blocksCompressed += 1
        this.blockLength = 0
        // `output()` reads a complete 64-byte block even when its declared length is
        // shorter. Clear the tail left by the previous compressed block before filling it.
        this.block.fill(0)
      }
      const take = Math.min(BLOCK_BYTES - this.blockLength, input.byteLength - offset)
      this.block.set(input.subarray(offset, offset + take), this.blockLength)
      this.blockLength += take
      offset += take
    }
  }

  output() {
    const flags = (this.blocksCompressed === 0 ? CHUNK_START : 0) | CHUNK_END
    return new Output(
      this.chainingValue,
      blockWords(this.block),
      this.counter,
      this.blockLength,
      flags,
    )
  }
}

/** A bounded-memory incremental BLAKE3-256 implementation. */
export class IncrementalBlake3 {
  #chunk = new ChunkState(0n)
  #chainingValues = []
  #finished = false

  /** @param {ArrayBuffer | ArrayBufferView} input */
  update(input) {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    const bytes = asBytes(input)
    let offset = 0
    while (offset < bytes.byteLength) {
      if (this.#chunk.length() === CHUNK_BYTES) {
        const chunkChainingValue = this.#chunk.output().chainingValue()
        const totalChunks = this.#chunk.counter + 1n
        if (totalChunks >= MAX_CHUNKS) throw new RangeError("BLAKE3 input exceeds its chunk counter")
        this.#addChunkChainingValue(chunkChainingValue, totalChunks)
        this.#chunk = new ChunkState(totalChunks)
      }
      const take = Math.min(CHUNK_BYTES - this.#chunk.length(), bytes.byteLength - offset)
      this.#chunk.update(bytes.subarray(offset, offset + take))
      offset += take
    }
    return this
  }

  digest() {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    this.#finished = true
    let output = this.#chunk.output()
    for (let index = this.#chainingValues.length - 1; index >= 0; index -= 1) {
      output = parentOutput(this.#chainingValues[index], output.chainingValue())
    }
    return output.rootDigest()
  }

  digestHex() {
    return hexLower(this.digest())
  }

  /** @param {Uint32Array} newChainingValue @param {bigint} totalChunks */
  #addChunkChainingValue(newChainingValue, totalChunks) {
    let value = newChainingValue
    let count = totalChunks
    while ((count & 1n) === 0n) {
      value = parentOutput(this.#chainingValues.pop(), value).chainingValue()
      count >>= 1n
    }
    if (this.#chainingValues.length >= 64) throw new RangeError("BLAKE3 tree depth exceeds its chunk counter")
    this.#chainingValues.push(value)
  }
}

/** @param {Uint32Array | undefined} left @param {Uint32Array} right */
function parentOutput(left, right) {
  if (left === undefined) throw new Error("BLAKE3 tree state is inconsistent")
  const words = new Uint32Array(16)
  words.set(left)
  words.set(right, 8)
  return new Output(new Uint32Array(IV), words, 0n, BLOCK_BYTES, PARENT)
}

/** @param {ArrayBuffer | ArrayBufferView} input */
function asBytes(input) {
  if (input instanceof ArrayBuffer) return new Uint8Array(input)
  if (ArrayBuffer.isView(input)) {
    return new Uint8Array(input.buffer, input.byteOffset, input.byteLength)
  }
  throw new TypeError("BLAKE3 input must be an ArrayBuffer or view")
}

/**
 * Hash a byte stream without retaining it.
 *
 * @param {ReadableStream<Uint8Array>} stream
 * @param {{signal?: AbortSignal, onChunk?: (bytes: number) => void}} [options]
 */
export async function blake3Stream(stream, options = {}) {
  const hash = new IncrementalBlake3()
  const reader = stream.getReader()
  let bytes = 0
  try {
    while (true) {
      throwIfAborted(options.signal)
      const result = await reader.read()
      if (result.done) break
      const chunk = asBytes(result.value)
      hash.update(chunk)
      bytes += chunk.byteLength
      options.onChunk?.(bytes)
    }
  } finally {
    if (options.signal?.aborted) await reader.cancel(options.signal.reason).catch(() => {})
    reader.releaseLock()
  }
  return { bytes, hex: hash.digestHex() }
}

/** @param {AbortSignal | undefined} signal */
function throwIfAborted(signal) {
  if (signal?.aborted) {
    throw signal.reason ?? new DOMException("Stem operation aborted", "AbortError")
  }
}
