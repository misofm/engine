import { StemResolverError, stemDigest } from "./identity.js"

/**
 * A test/server-neutral resolver backed by canonical-PCM bytes in memory.
 * The map is copied at construction; each resolve returns a fresh bounded
 * stream, so consumers never share a mutable cursor.
 */
export class MemoryStemResolver {
  #stems
  #chunkBytes
  requests = []

  /**
   * @param {Map<string, Uint8Array> | Record<string, Uint8Array>} stems
   * @param {{chunkBytes?: number}} [options]
   */
  constructor(stems, options = {}) {
    const entries = stems instanceof Map ? stems.entries() : Object.entries(stems)
    this.#stems = new Map(
      Array.from(entries, ([identity, bytes]) => [identity, new Uint8Array(bytes)])
    )
    this.#chunkBytes = positiveInteger(options.chunkBytes ?? 64 * 1024, "chunkBytes")
  }

  /**
   * @param {string} identity
   * @param {{signal?: AbortSignal, onProgress?: (event: object) => void}} [options]
   */
  async resolve(identity, options = {}) {
    stemDigest(identity)
    throwIfAborted(options.signal)
    this.requests.push(identity)
    const bytes = this.#stems.get(identity)
    if (bytes === undefined) {
      throw new StemResolverError(
        "stem.resolve.missing",
        `No canonical PCM is available for ${identity}`,
        { identity }
      )
    }
    let offset = 0
    const chunkBytes = this.#chunkBytes
    return {
      canonicalBytes: bytes.byteLength,
      stream: new ReadableStream({
        pull(controller) {
          throwIfAborted(options.signal)
          if (offset === bytes.byteLength) {
            controller.close()
            return
          }
          const end = Math.min(bytes.byteLength, offset + chunkBytes)
          const chunk = bytes.slice(offset, end)
          offset = end
          options.onProgress?.({
            stage: "decoded",
            bytes: offset,
            totalBytes: bytes.byteLength,
          })
          controller.enqueue(chunk)
        },
      }),
    }
  }
}

/**
 * Web resolver: identity-to-URL policy and decoding are construction inputs,
 * never session fields. The decoder is supplied by #245 and must return a
 * canonical-PCM stream. This module deliberately has no container decoder.
 */
export class FetchStemResolver {
  #urlForIdentity
  #decode
  #fetch
  #readDeadlineMs
  #maximumResumeAttempts

  /**
   * @param {{
   *   urlForIdentity: (identity: string) => string | URL,
   *   decode: (stream: ReadableStream<Uint8Array>, context: object) => Promise<ReadableStream<Uint8Array>> | ReadableStream<Uint8Array>,
   *   fetcher?: typeof fetch,
   *   readDeadlineMs?: number,
   *   maximumResumeAttempts?: number,
   * }} options
   */
  constructor(options) {
    if (typeof options?.urlForIdentity !== "function") {
      throw new TypeError("FetchStemResolver needs urlForIdentity(identity)")
    }
    if (typeof options.decode !== "function") {
      throw new TypeError("FetchStemResolver needs a provenance-pinned decoder")
    }
    this.#urlForIdentity = options.urlForIdentity
    this.#decode = options.decode
    this.#fetch = options.fetcher ?? globalThis.fetch?.bind(globalThis)
    if (typeof this.#fetch !== "function") {
      throw new StemResolverError(
        "stem.resolve.network_unavailable",
        "This adapter has no network fetch implementation"
      )
    }
    this.#readDeadlineMs = positiveInteger(
      options.readDeadlineMs ?? 30_000,
      "readDeadlineMs"
    )
    this.#maximumResumeAttempts = nonnegativeInteger(
      options.maximumResumeAttempts ?? 3,
      "maximumResumeAttempts"
    )
  }

  /**
   * @param {string} identity
   * @param {{signal?: AbortSignal, onProgress?: (event: object) => void}} [options]
   */
  async resolve(identity, options = {}) {
    stemDigest(identity)
    const url = String(this.#urlForIdentity(identity))
    const delivered = resumableFetchStream({
      url,
      fetcher: this.#fetch,
      signal: options.signal,
      onProgress: options.onProgress,
      readDeadlineMs: this.#readDeadlineMs,
      maximumResumeAttempts: this.#maximumResumeAttempts,
    })
    let decoded
    try {
      decoded = await this.#decode(delivered, {
        identity,
        signal: options.signal,
        onProgress: options.onProgress,
      })
    } catch (error) {
      throw new StemResolverError(
        "stem.decode.failed",
        `Stem decoder refused ${identity}`,
        { identity, url },
        error
      )
    }
    if (!(decoded instanceof ReadableStream)) {
      throw new StemResolverError(
        "stem.decode.contract",
        "Stem decoder did not return a ReadableStream",
        { identity, url }
      )
    }
    return { stream: progressStream(decoded, options) }
  }
}

/** @param {ReadableStream<Uint8Array>} stream @param {object} options */
function progressStream(stream, options) {
  const reader = stream.getReader()
  let decoded = 0
  return new ReadableStream({
    async pull(controller) {
      throwIfAborted(options.signal)
      const result = await reader.read()
      if (result.done) {
        controller.close()
        return
      }
      decoded += result.value.byteLength
      options.onProgress?.({ stage: "decoded", bytes: decoded })
      controller.enqueue(result.value)
    },
    async cancel(reason) {
      await reader.cancel(reason)
    },
  })
}

/**
 * A byte stream that resumes only when the server proves the requested range.
 * The decoder sees one continuous delivered stream and never knows a retry
 * occurred.
 */
function resumableFetchStream(options) {
  let response
  let reader
  let offset = 0
  let totalBytes
  let attempts = 0

  const open = async () => {
    throwIfAborted(options.signal)
    const headers = offset === 0 ? undefined : { Range: `bytes=${offset}-` }
    try {
      response = await options.fetcher(
        options.url,
        headers === undefined
          ? { signal: options.signal }
          : { signal: options.signal, headers }
      )
    } catch (error) {
      throw new StemResolverError(
        "stem.resolve.network",
        `Stem download failed at byte ${offset}`,
        { url: options.url, offset },
        error
      )
    }
    const expectedStatus = offset === 0 ? 200 : 206
    if (!response.ok || (offset !== 0 && response.status !== expectedStatus)) {
      throw new StemResolverError(
        offset === 0
          ? "stem.resolve.http"
          : "stem.resolve.resume_unsupported",
        `Stem server refused byte ${offset} with HTTP ${response.status}`,
        { url: options.url, offset, status: response.status }
      )
    }
    if (offset !== 0) {
      const contentRange = response.headers.get("content-range")
      if (!contentRange?.startsWith(`bytes ${offset}-`)) {
        throw new StemResolverError(
          "stem.resolve.resume_mismatch",
          `Stem server returned the wrong byte range for ${offset}`,
          { url: options.url, offset, contentRange }
        )
      }
    }
    if (offset === 0) {
      const header = Number(response.headers.get("content-length"))
      if (Number.isSafeInteger(header) && header >= 0) totalBytes = header
    }
    if (response.body === null) {
      throw new StemResolverError(
        "stem.resolve.empty_body",
        "Stem response had no byte stream",
        { url: options.url }
      )
    }
    reader = response.body.getReader()
  }

  return new ReadableStream({
    async pull(controller) {
      while (true) {
        if (reader === undefined) await open()
        try {
          const result = await withDeadline(
            reader.read(),
            options.readDeadlineMs,
            "stem.resolve.stalled",
            `Stem download made no progress for ${options.readDeadlineMs}ms`
          )
          if (result.done) {
            controller.close()
            return
          }
          offset += result.value.byteLength
          options.onProgress?.({
            stage: "fetched",
            bytes: offset,
            totalBytes,
          })
          controller.enqueue(result.value)
          return
        } catch (error) {
          await reader.cancel(error).catch(() => {})
          reader = undefined
          attempts += 1
          if (attempts > options.maximumResumeAttempts) {
            if (error instanceof StemResolverError) throw error
            throw new StemResolverError(
              "stem.resolve.resume_exhausted",
              `Stem download exhausted ${options.maximumResumeAttempts} resume attempts`,
              { url: options.url, offset },
              error
            )
          }
        }
      }
    },
    async cancel(reason) {
      await reader?.cancel(reason).catch(() => {})
    },
  })
}

/** @param {Promise<unknown>} operation */
async function withDeadline(operation, milliseconds, code, message) {
  let timer
  const timeout = new Promise((_resolve, reject) => {
    timer = setTimeout(
      () => reject(new StemResolverError(code, message, { milliseconds })),
      milliseconds
    )
  })
  try {
    return await Promise.race([operation, timeout])
  } finally {
    clearTimeout(timer)
  }
}

function positiveInteger(value, name) {
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new RangeError(`${name} must be a positive safe integer`)
  }
  return value
}

function nonnegativeInteger(value, name) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new RangeError(`${name} must be a nonnegative safe integer`)
  }
  return value
}

/** @param {AbortSignal | undefined} signal */
function throwIfAborted(signal) {
  if (signal?.aborted) {
    throw signal.reason ?? new DOMException("Stem operation aborted", "AbortError")
  }
}
