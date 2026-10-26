/*
 * Incremental BLAKE3-256 for canonical-PCM ingest and verify-on-open.
 *
 * The browser has no streaming Web Crypto BLAKE3 interface. Issue #2111: a hash
 * runs in the engine's WebAssembly SIMD module (`hosts/blake3-web`, shipped in
 * the delivery closure as `miso-engine-v1-blake3.simd128.wasm`) once `prepareBlake3`
 * has loaded it, and otherwise in the repository-owned, bounded-memory
 * JavaScript implementation of the BLAKE3 hash mode below. Both spell the same
 * digest; the corpus tests cover official vectors and hostile chunk boundaries
 * through each.
 */

import { hexLower } from "../hex-lower.js"

const IV = new Uint32Array([
  0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
  0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
])
const MESSAGE_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]
const BLOCK_BYTES = 64
const BLOCKS_PER_CHUNK = 16
const CHUNK_START = 1
const CHUNK_END = 2
const PARENT = 4
const ROOT = 8
const WORDS_PER_CV = 8
// 2^54 chunks is 2^64 bytes, BLAKE3's input limit; the stack holds one chaining value per level.
const MAX_DEPTH = 54
const TWO_POW_32 = 2 ** 32

// The message word order of each of the seven rounds: round r reads word SCHEDULE[16r + i] where
// round 0 reads word i, and each later round permutes the one before by MESSAGE_PERMUTATION.
const SCHEDULE = new Uint8Array(7 * 16)
for (let index = 0; index < 16; index += 1) SCHEDULE[index] = index
for (let round = 1; round < 7; round += 1) {
  for (let index = 0; index < 16; index += 1) {
    SCHEDULE[round * 16 + index] = SCHEDULE[(round - 1) * 16 + MESSAGE_PERMUTATION[index]]
  }
}

// Issue #2111: the hash allocates nothing per block or per chunk. Every compression works in
// these module scratch arrays (a hash is synchronous, so no two compressions interleave), and a
// hasher owns fixed-size buffers for its block, chaining values and tree stack.
const STATE = new Uint32Array(16)
const WORDS = new Uint32Array(16)
const CHUNK_CV = new Uint32Array(WORDS_PER_CV)

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

/**
 * Compress `words` under the chaining value `cv[cvAt..cvAt+8]` and write the first eight output
 * words (the next chaining value, or the root digest's words) to `out[outAt..outAt+8]`. `out` may
 * be `cv` at the same offset.
 *
 * @param {Uint32Array} cv @param {number} cvAt @param {Uint32Array} words @param {number} counter
 * @param {number} blockLength @param {number} flags @param {Uint32Array} out @param {number} outAt
 */
function compress(cv, cvAt, words, counter, blockLength, flags, out, outAt) {
  const state = STATE
  for (let index = 0; index < 8; index += 1) state[index] = cv[cvAt + index]
  state[8] = IV[0]
  state[9] = IV[1]
  state[10] = IV[2]
  state[11] = IV[3]
  state[12] = counter % TWO_POW_32
  state[13] = Math.floor(counter / TWO_POW_32)
  state[14] = blockLength
  state[15] = flags
  for (let base = 0; base < SCHEDULE.length; base += 16) {
    mix(state, 0, 4, 8, 12, words[SCHEDULE[base]], words[SCHEDULE[base + 1]])
    mix(state, 1, 5, 9, 13, words[SCHEDULE[base + 2]], words[SCHEDULE[base + 3]])
    mix(state, 2, 6, 10, 14, words[SCHEDULE[base + 4]], words[SCHEDULE[base + 5]])
    mix(state, 3, 7, 11, 15, words[SCHEDULE[base + 6]], words[SCHEDULE[base + 7]])
    mix(state, 0, 5, 10, 15, words[SCHEDULE[base + 8]], words[SCHEDULE[base + 9]])
    mix(state, 1, 6, 11, 12, words[SCHEDULE[base + 10]], words[SCHEDULE[base + 11]])
    mix(state, 2, 7, 8, 13, words[SCHEDULE[base + 12]], words[SCHEDULE[base + 13]])
    mix(state, 3, 4, 9, 14, words[SCHEDULE[base + 14]], words[SCHEDULE[base + 15]])
  }
  for (let index = 0; index < 8; index += 1) out[outAt + index] = state[index] ^ state[index + 8]
}

/** Load 16 little-endian words from `bytes[at..at+64]`. @param {Uint8Array} bytes @param {number} at */
function loadWords(bytes, at) {
  for (let index = 0; index < 16; index += 1) {
    const byte = at + index * 4
    WORDS[index] = bytes[byte] | (bytes[byte + 1] << 8) | (bytes[byte + 2] << 16) | (bytes[byte + 3] << 24)
  }
}

/** The JavaScript BLAKE3-256: the engine when the WebAssembly module is not loaded. */
class JavaScriptBlake3 {
  #cv = new Uint32Array(IV)
  /** The current chunk's index. */
  #chunk = 0
  /** Whole blocks of the current chunk already compressed into `#cv`. */
  #blocks = 0
  /** The current chunk's newest block, compressed only once more input shows it is not the last. */
  #block = new Uint8Array(BLOCK_BYTES)
  #blockLength = 0
  /** The chaining values of the completed subtrees, one per level, eight words each. */
  #stack = new Uint32Array(MAX_DEPTH * WORDS_PER_CV)
  #depth = 0
  #finished = false

  /** @param {ArrayBuffer | ArrayBufferView} input */
  update(input) {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    const bytes = asBytes(input)
    const length = bytes.length
    let offset = 0
    while (offset < length) {
      // More input follows a full buffered block, so it is not the hash's last block.
      if (this.#blockLength === BLOCK_BYTES) {
        loadWords(this.#block, 0)
        this.#compressBlock()
        this.#blockLength = 0
      }
      if (this.#blockLength === 0) {
        // Whole blocks straight from the input while more input follows each.
        while (length - offset > BLOCK_BYTES) {
          loadWords(bytes, offset)
          this.#compressBlock()
          offset += BLOCK_BYTES
        }
      }
      const block = this.#block
      let filled = this.#blockLength
      const end = Math.min(offset + BLOCK_BYTES - filled, length)
      while (offset < end) block[filled++] = bytes[offset++]
      this.#blockLength = filled
    }
    return this
  }

  /** Compress the block in WORDS, known not to be the hash's last, into the current chunk. */
  #compressBlock() {
    const flags = this.#blocks === 0 ? CHUNK_START : 0
    if (this.#blocks === BLOCKS_PER_CHUNK - 1) {
      compress(this.#cv, 0, WORDS, this.#chunk, BLOCK_BYTES, flags | CHUNK_END, CHUNK_CV, 0)
      this.#pushChunk()
    } else {
      compress(this.#cv, 0, WORDS, this.#chunk, BLOCK_BYTES, flags, this.#cv, 0)
      this.#blocks += 1
    }
  }

  /** Merge the completed chunk's chaining value (CHUNK_CV) into the tree stack; start the next. */
  #pushChunk() {
    const stack = this.#stack
    let total = this.#chunk + 1
    if (total > Number.MAX_SAFE_INTEGER) throw new RangeError("BLAKE3 input exceeds its chunk counter")
    // Each trailing zero bit of the chunk count completes one subtree: merge it with its left
    // sibling on the stack, in place in CHUNK_CV.
    while (total % 2 === 0) {
      this.#depth -= 1
      const left = this.#depth * WORDS_PER_CV
      for (let index = 0; index < 8; index += 1) {
        WORDS[index] = stack[left + index]
        WORDS[index + 8] = CHUNK_CV[index]
      }
      compress(IV, 0, WORDS, 0, BLOCK_BYTES, PARENT, CHUNK_CV, 0)
      total /= 2
    }
    if (this.#depth >= MAX_DEPTH) throw new RangeError("BLAKE3 tree depth exceeds its chunk counter")
    stack.set(CHUNK_CV, this.#depth * WORDS_PER_CV)
    this.#depth += 1
    this.#chunk += 1
    this.#cv.set(IV)
    this.#blocks = 0
  }

  digest() {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    this.#finished = true
    // The last chunk's last block, zero past its length, then each parent up the stack; the
    // final node is compressed with ROOT and counter 0.
    this.#block.fill(0, this.#blockLength)
    loadWords(this.#block, 0)
    let cv = this.#cv
    let counter = this.#chunk
    let blockLength = this.#blockLength
    let flags = (this.#blocks === 0 ? CHUNK_START : 0) | CHUNK_END
    for (let level = this.#depth - 1; level >= 0; level -= 1) {
      compress(cv, 0, WORDS, counter, blockLength, flags, CHUNK_CV, 0)
      for (let index = 0; index < 8; index += 1) {
        WORDS[index] = this.#stack[level * WORDS_PER_CV + index]
        WORDS[index + 8] = CHUNK_CV[index]
      }
      cv = IV
      counter = 0
      blockLength = BLOCK_BYTES
      flags = PARENT
    }
    compress(cv, 0, WORDS, 0, blockLength, flags | ROOT, CHUNK_CV, 0)
    const digest = new Uint8Array(32)
    for (let index = 0; index < 8; index += 1) {
      const word = CHUNK_CV[index]
      digest[index * 4] = word
      digest[index * 4 + 1] = word >>> 8
      digest[index * 4 + 2] = word >>> 16
      digest[index * 4 + 3] = word >>> 24
    }
    return digest
  }
}

const DIGEST_BYTES = 32
const MODULE_NAME = "miso-engine-v1-blake3.simd128.wasm"
const EXPORTS = ["memory", "new", "input", "input_capacity", "update", "finalize", "free"]

/**
 * The loaded module, or `undefined` while no preparation has succeeded in this realm.
 * @type {{ memory: WebAssembly.Memory, create: () => number, input: (handle: number) => number, update: (handle: number, length: number) => number, finalize: (handle: number) => number, free: (handle: number) => void, capacity: number, registry: FinalizationRegistry<number> | undefined } | undefined}
 */
let loaded
/** One preparation per module URL. @type {Map<string, Promise<"simd128" | "javascript">>} */
const preparations = new Map()

/**
 * Load the WebAssembly SIMD BLAKE3 so that every later `IncrementalBlake3` and
 * `blake3Stream` hashes in it. The default URL is the SDK package's layout: this
 * file is `dist/browser/stems/incremental-blake3.js` and the module is the gated
 * closure's copy, `dist/assets/miso-engine-v1-blake3.simd128.wasm`. Idempotent: one load per URL per realm. Resolves
 * `"simd128"` when the module is loaded, and `"javascript"` when the fetch,
 * the compile (a browser without simd128) or the instantiation fails; it never
 * rejects, and hashing then stays in JavaScript.
 *
 * @param {{ url?: string | URL }} [options]
 * @returns {Promise<"simd128" | "javascript">}
 */
export function prepareBlake3(options = {}) {
  const url = String(options.url ?? new URL(`../../assets/${MODULE_NAME}`, import.meta.url))
  let preparation = preparations.get(url)
  if (preparation === undefined) {
    preparation = load(url).then(
      (module) => {
        loaded = module
        return "simd128"
      },
      () => "javascript",
    )
    preparations.set(url, preparation)
  }
  return preparation
}

/** @param {string} url */
async function load(url) {
  const response = await fetch(url)
  if (!response.ok) throw new Error(`BLAKE3 module fetch failed: ${response.status}`)
  const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), {})
  const names = Object.fromEntries(
    EXPORTS.map((name) => [name, instance.exports[name === "memory" ? name : `miso_engine_blake3_v1_${name}`]]),
  )
  if (!(names.memory instanceof WebAssembly.Memory)) throw new Error("BLAKE3 module has no memory")
  for (const name of EXPORTS.slice(1)) {
    if (typeof names[name] !== "function") throw new Error(`BLAKE3 module has no ${name} export`)
  }
  const free = names.free
  return {
    memory: names.memory,
    create: names.new,
    input: names.input,
    update: names.update,
    finalize: names.finalize,
    free,
    capacity: names.input_capacity(),
    // A hash dropped before its digest releases its Wasm memory when collected.
    registry:
      typeof FinalizationRegistry === "function"
        ? new FinalizationRegistry((handle) => free(handle))
        : undefined,
  }
}

/** The WebAssembly SIMD BLAKE3-256 (`hosts/blake3-web`). */
class WasmBlake3 {
  #module
  #handle
  #input
  #finished = false

  /** @param {NonNullable<typeof loaded>} module */
  constructor(module) {
    this.#module = module
    this.#handle = module.create()
    this.#input = module.input(this.#handle)
    module.registry?.register(this, this.#handle, this)
  }

  /** @param {ArrayBuffer | ArrayBufferView} input */
  update(input) {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    const bytes = asBytes(input)
    const module = this.#module
    let offset = 0
    while (offset < bytes.byteLength) {
      const take = Math.min(module.capacity, bytes.byteLength - offset)
      // A view per copy: an allocation by another hash may have grown, and so replaced, the buffer.
      new Uint8Array(module.memory.buffer, this.#input, take).set(bytes.subarray(offset, offset + take))
      module.update(this.#handle, take)
      offset += take
    }
  }

  digest() {
    if (this.#finished) throw new Error("BLAKE3 digest is already finalized")
    this.#finished = true
    const module = this.#module
    module.finalize(this.#handle)
    const digest = new Uint8Array(module.memory.buffer, this.#input, DIGEST_BYTES).slice()
    module.registry?.unregister(this)
    module.free(this.#handle)
    return digest
  }
}

/**
 * A bounded-memory incremental BLAKE3-256. It hashes in the WebAssembly SIMD
 * module when `prepareBlake3` has loaded it, and in JavaScript otherwise.
 */
export class IncrementalBlake3 {
  #engine = loaded === undefined ? new JavaScriptBlake3() : new WasmBlake3(loaded)

  /** @param {ArrayBuffer | ArrayBufferView} input */
  update(input) {
    this.#engine.update(input)
    return this
  }

  digest() {
    return this.#engine.digest()
  }

  digestHex() {
    return hexLower(this.digest())
  }
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
  await prepareBlake3()
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
