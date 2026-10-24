import { IncrementalBlake3 } from "./incremental-blake3.js"
import { StemResolverError, stemDigest, stemFileName } from "./identity.js"

export const DEFAULT_STEM_STORE_FOLDER = "miso-stems-blake3-v1"

const INDEX_FILE = "index.json"
const STAGING_DIRECTORY = "staging"
const INDEX_VERSION = 1
const FINAL_NAME = /^blake3-([0-9a-f]{64})$/
const LOCK_PREFIX = "miso:stem-store:v1"

/** A typed web-adapter storage refusal. */
export class StemStoreError extends Error {
  /**
   * @param {string} code
   * @param {string} message
   * @param {Record<string, unknown>} [details]
   * @param {unknown} [cause]
   */
  constructor(code, message, details = {}, cause) {
    super(message, cause === undefined ? undefined : { cause })
    this.name = "StemStoreError"
    this.code = code
    this.details = Object.freeze({ ...details })
  }
}

/**
 * The single-layer canonical-PCM store. The engine never imports this module.
 */
export class OpfsStemStore {
  #folderName
  #storage
  #locks
  #now
  #tabId
  #readDeadlineMs
  #ingestReadDeadlineMs
  #maintenanceBudgetMs
  #maintenanceFailures = 0
  #hooks
  #directory
  #staging
  #opening
  #inFlight = new Map()
  #localLockTails = new Map()

  /**
   * @param {{
   *   folderName?: string,
   *   storage?: StorageManager,
   *   locks?: LockManager,
   *   now?: () => number,
   *   tabId?: string,
   *   readDeadlineMs?: number,
   *   ingestReadDeadlineMs?: number,
   *   maintenanceBudgetMs?: number,
   *   hooks?: Record<string, Function>,
   * }} [options]
   */
  constructor(options = {}) {
    this.#folderName = validateFolderName(
      options.folderName ?? DEFAULT_STEM_STORE_FOLDER
    )
    this.#storage = options.storage ?? globalThis.navigator?.storage
    this.#locks = options.locks ?? globalThis.navigator?.locks
    this.#now = options.now ?? Date.now
    this.#tabId = options.tabId ?? randomTabId()
    this.#readDeadlineMs = positiveInteger(
      options.readDeadlineMs ?? 15_000,
      "readDeadlineMs"
    )
    this.#ingestReadDeadlineMs = positiveInteger(
      options.ingestReadDeadlineMs ?? 30_000,
      "ingestReadDeadlineMs"
    )
    this.#maintenanceBudgetMs = positiveInteger(
      options.maintenanceBudgetMs ?? this.#readDeadlineMs,
      "maintenanceBudgetMs"
    )
    this.#hooks = options.hooks ?? {}
  }

  get folderName() {
    return this.#folderName
  }

  /** Open, recover the crash-only index, and sweep dead staging/final debris. */
  async open() {
    if (this.#directory !== undefined) return this
    if (this.#opening !== undefined) return this.#opening
    this.#opening = this.#openOnce()
    try {
      await this.#opening
      return this
    } finally {
      this.#opening = undefined
    }
  }

  async #openOnce() {
    if (typeof this.#storage?.getDirectory !== "function") {
      throw new StemStoreError(
        "storage.unavailable",
        "Origin-private stem storage is unavailable"
      )
    }
    try {
      const root = await this.#storage.getDirectory()
      this.#directory = await root.getDirectoryHandle(this.#folderName, {
        create: true,
      })
      this.#staging = await this.#directory.getDirectoryHandle(
        STAGING_DIRECTORY,
        { create: true }
      )
      await this.#withIndexLock(async () => {
        const index = await this.#readIndex(true)
        const liveLocks = await this.#liveLockNames()
        const pinsChanged = this.#purgeDeadSessionPins(index, liveLocks)
        // Debris sweeps are best-effort: one overall budget, every await
        // deadlined, and a failure is reported and counted, never thrown, so a
        // wedged or failing directory listing cannot make every mix unopenable.
        // The next open retries whatever this one left.
        const budgetEnd = monotonicNow() + this.#maintenanceBudgetMs
        await this.#bestEffortSweep("staging", () =>
          this.#sweepStaging(liveLocks, budgetEnd)
        )
        await this.#bestEffortSweep("finals", () =>
          this.#removeUnindexedFinals(index, liveLocks, budgetEnd)
        )
        if (pinsChanged) await this.#writeIndex(index)
      })
    } catch (error) {
      this.#directory = undefined
      this.#staging = undefined
      if (error instanceof StemStoreError) throw error
      throw classifyStorageError(error, "opening the stem store")
    }
  }

  /**
   * Verify every referenced stem at this open, self-heal misses/corruption,
   * and pin the complete set only after the hard gate is green.
   *
   * @param {{sessionId: string, stems: Array<{identity: string, bytes: number}>, resolver: {resolve: Function}, signal?: AbortSignal, onProgress?: (event: object) => void}} options
   */
  async openSession(options) {
    await this.open()
    const sessionId = nonemptyText(options.sessionId, "sessionId")
    if (typeof options.resolver?.resolve !== "function") {
      throw new TypeError("openSession needs a StemResolver")
    }
    const stems = normalizeRequirements(options.stems)
    const protectedIdentities = new Set(stems.map((stem) => stem.identity))
    throwIfAborted(options.signal)
    const missing = []

    for (const stem of stems) {
      const present = await this.#withStemLock(stem.identity, options.signal, () =>
        this.#verifyIndexed(stem, options.onProgress, options.signal)
      )
      if (!present) missing.push(stem)
    }

    await this.#preflight(missing, protectedIdentities)

    const ingestAbort = new AbortController()
    const removeParentAbort = forwardAbort(options.signal, ingestAbort)
    try {
      await Promise.all(
        missing.map((stem) =>
          this.#ensureIngested(stem, options.resolver, {
            signal: ingestAbort.signal,
            onProgress: options.onProgress,
            protectedIdentities,
          }).catch((error) => {
            ingestAbort.abort(error)
            throw error
          })
        )
      )
    } finally {
      removeParentAbort()
    }

    throwIfAborted(options.signal)
    const pin = `session:${this.#tabId}:${sessionId}`
    const releasePinLock = await this.#acquireSessionPinLock(pin)
    try {
      await this.#mutateIndex((index) => {
        for (const stem of stems) {
          const row = index.stems[stem.identity]
          if (row === undefined) {
            throw new StemStoreError(
              "stem.gate.missing",
              `Stem disappeared before the loading gate could open: ${stem.identity}`,
              { identity: stem.identity }
            )
          }
          if (!row.pins.includes(pin)) row.pins.push(pin)
          row.lastUsedAt = this.#now()
        }
      })
    } catch (error) {
      releasePinLock()
      throw error
    }

    options.onProgress?.({ stage: "ready", stems: stems.length })
    return new StemSessionLease(
      this,
      sessionId,
      pin,
      stems,
      releasePinLock
    )
  }

  /** Explicit maintenance verification for an indexed or unreferenced file. */
  async verify(identity, options = {}) {
    await this.open()
    const digest = stemDigest(identity)
    return this.#withStemLock(identity, options.signal, async () => {
      const handle = await this.#fileHandle(stemFileName(identity), false)
      if (handle === null) return false
      const observed = await this.#hashFile(handle, {
        signal: options.signal,
        onChunk: options.onProgress,
        stage: "verify-maintenance",
        identity,
      })
      const valid = observed.hex === digest
      if (!valid) await this.#demote(identity)
      return valid
    })
  }

  /** Pin or unpin a stem for the user-facing keep-offline policy. */
  async setOfflinePin(identity, pinId, pinned) {
    await this.open()
    stemDigest(identity)
    const pin = `offline:${nonemptyText(pinId, "pinId")}`
    await this.#mutateIndex((index) => {
      const row = index.stems[identity]
      if (row === undefined) {
        throw new StemStoreError(
          "stem.pin.missing",
          `Cannot pin a missing stem: ${identity}`,
          { identity }
        )
      }
      row.pins = pinned
        ? Array.from(new Set([...row.pins, pin]))
        : row.pins.filter((candidate) => candidate !== pin)
    })
  }

  /** A snapshot Blob for the Worker pump. Reads use getFile(), never a lock. */
  async read(identity) {
    await this.open()
    stemDigest(identity)
    const index = await this.#withIndexLock(() => this.#readIndex(true))
    if (index.stems[identity] === undefined) {
      throw new StemStoreError(
        "stem.read.missing",
        `Stem is not indexed: ${identity}`,
        { identity }
      )
    }
    try {
      const handle = await this.#directory.getFileHandle(stemFileName(identity))
      return await withDeadline(
        handle.getFile(),
        this.#readDeadlineMs,
        "storage.read_stalled",
        `Reading ${identity} made no progress`
      )
    } catch (error) {
      await this.#demote(identity).catch(() => {})
      throw classifyStorageError(error, `reading ${identity}`, {
        code: "stem.read.failed",
        identity,
      })
    }
  }

  /** @private */
  async releaseSessionPin(pin, stems) {
    await this.#mutateIndex((index) => {
      for (const stem of stems) {
        const row = index.stems[stem.identity]
        if (row !== undefined) {
          row.pins = row.pins.filter((candidate) => candidate !== pin)
        }
      }
    })
  }

  async #ensureIngested(stem, resolver, options) {
    const existing = this.#inFlight.get(stem.identity)
    if (existing !== undefined) return existing
    const promise = this.#withStemLock(stem.identity, options.signal, async () => {
      if (await this.#verifyIndexed(stem, options.onProgress, options.signal)) return
      let lastError
      for (let attempt = 0; attempt < 2; attempt += 1) {
        try {
          await this.#ingestAttempt(stem, resolver, options)
          return
        } catch (error) {
          lastError = error
          if (!(error instanceof StemStoreError) || error.code !== "stem.ingest.integrity") {
            throw error
          }
        }
      }
      throw lastError
    })
    this.#inFlight.set(stem.identity, promise)
    try {
      return await promise
    } finally {
      if (this.#inFlight.get(stem.identity) === promise) {
        this.#inFlight.delete(stem.identity)
      }
    }
  }

  async #ingestAttempt(stem, resolver, options) {
    throwIfAborted(options.signal)
    const digest = stemDigest(stem.identity)
    const stagingName = `${this.#tabId}-${digest}`
    await removeEntry(this.#staging, stagingName)
    const handle = await this.#staging.getFileHandle(stagingName, { create: true })
    let leaveDebris = false
    try {
      // Prove the device writable before any network: an unwritable store
      // must refuse with a typed storage error, not after a full download.
      const writable = await this.#openStagingWritable(handle, stem, options)
      const hash = new IncrementalBlake3()
      let reader
      let bytes = 0
      try {
        let resolved
        try {
          resolved = await withDeadline(
            resolver.resolve(stem.identity, {
              signal: options.signal,
              onProgress: (event) =>
                options.onProgress?.({ ...event, identity: stem.identity }),
            }),
            this.#ingestReadDeadlineMs,
            "stem.resolve.stalled",
            `Resolving ${stem.identity} made no progress`,
            options.signal,
            StemResolverError
          )
        } catch (error) {
          if (error instanceof StemResolverError) throw error
          throw new StemResolverError(
            "stem.resolve.failed",
            `Resolver failed for ${stem.identity}`,
            { identity: stem.identity },
            error
          )
        }
        if (!(resolved?.stream instanceof ReadableStream)) {
          throw new StemResolverError(
            "stem.resolve.contract",
            "Resolver did not return a canonical-PCM ReadableStream",
            { identity: stem.identity }
          )
        }
        reader = resolved.stream.getReader()
        while (true) {
          throwIfAborted(options.signal)
          const result = await withDeadline(
            reader.read(),
            this.#ingestReadDeadlineMs,
            "stem.decode.stalled",
            `Decoded PCM for ${stem.identity} made no progress`,
            options.signal,
            StemResolverError
          )
          if (result.done) break
          if (!(result.value instanceof Uint8Array)) {
            throw new StemResolverError(
              "stem.resolve.contract",
              "Resolver emitted a non-byte canonical-PCM chunk",
              { identity: stem.identity }
            )
          }
          bytes += result.value.byteLength
          if (bytes > stem.bytes) {
            // The declaration is already contradicted: refuse at the chunk
            // that proves it rather than filling staging to the stream's
            // end, where the excess only charges the origin's quota.
            throw new StemStoreError(
              "stem.ingest.integrity",
              `Decoded PCM overran the declared length for ${stem.identity}`,
              {
                identity: stem.identity,
                expectedBytes: stem.bytes,
                observedBytes: bytes,
                expectedBlake3: digest,
              }
            )
          }
          hash.update(result.value)
          await withDeadline(
            writable.write(result.value),
            this.#readDeadlineMs,
            "storage.write_stalled",
            `Writing staging for ${stem.identity} made no progress`,
            options.signal
          )
          options.onProgress?.({
            stage: "hashed",
            identity: stem.identity,
            bytes,
            totalBytes: stem.bytes,
          })
          await this.#hooks.afterChunk?.({
            identity: stem.identity,
            bytes,
            stagingName,
          })
        }
        await withDeadline(
          writable.close(),
          this.#readDeadlineMs,
          "storage.write_stalled",
          `Closing staging for ${stem.identity} made no progress`,
          options.signal
        )
      } catch (error) {
        await withDeadline(
          writable.abort(error),
          this.#readDeadlineMs,
          "storage.write_stalled",
          `Aborting staging for ${stem.identity} made no progress`
        ).catch(() => {})
        void reader?.cancel(error).catch(() => {})
        if (error?.leaveStaging === true) leaveDebris = true
        throw error
      } finally {
        if (reader !== undefined) releaseReader(reader)
      }

      const streamedHex = hash.digestHex()
      if (bytes !== stem.bytes || streamedHex !== digest) {
        throw new StemStoreError(
          "stem.ingest.integrity",
          `Decoded PCM did not match ${stem.identity}`,
          {
            identity: stem.identity,
            expectedBytes: stem.bytes,
            observedBytes: bytes,
            expectedBlake3: digest,
            observedBlake3: streamedHex,
          }
        )
      }

      await this.#hooks.afterStagingWrite?.({
        identity: stem.identity,
        handle,
        stagingName,
      })
      const reopened = await this.#hashFile(handle, {
        signal: options.signal,
        onChunk: options.onProgress,
        stage: "verified",
        identity: stem.identity,
      })
      if (reopened.bytes !== stem.bytes || reopened.hex !== digest) {
        throw new StemStoreError(
          "stem.ingest.integrity",
          `Staged PCM failed pre-promote verification for ${stem.identity}`,
          {
            identity: stem.identity,
            expectedBytes: stem.bytes,
            observedBytes: reopened.bytes,
            expectedBlake3: digest,
            observedBlake3: reopened.hex,
          }
        )
      }

      await this.#promote(stem, handle, stagingName, options)
      await this.#indexPromoted(stem)
      await this.#hooks.afterIndex?.({ identity: stem.identity })
      options.onProgress?.({
        stage: "promoted",
        identity: stem.identity,
        bytes: stem.bytes,
        totalBytes: stem.bytes,
      })
    } catch (error) {
      if (error?.leaveStaging === true) leaveDebris = true
      if (isQuotaError(error)) {
        // The failing identity is not a survivor: a quota error may have
        // happened during fallback-final or index writing after its staging
        // bytes were complete. Remove any unindexed final so retry is clean.
        await removeEntry(
          this.#directory,
          stemFileName(stem.identity)
        ).catch(() => {})
        throw await this.#quotaError(
          stem.bytes,
          error,
          options.protectedIdentities
        )
      }
      throw error
    } finally {
      if (!leaveDebris) await removeEntry(this.#staging, stagingName)
    }
  }

  async #openStagingWritable(handle, stem, options) {
    try {
      return await withDeadline(
        handle.createWritable(),
        this.#readDeadlineMs,
        "storage.write_stalled",
        `Opening staging for ${stem.identity} made no progress`,
        options.signal
      )
    } catch (error) {
      // Abort and quota errors keep their own handling; any other refusal is
      // the device saying it cannot store this stem.
      if (
        error instanceof StemStoreError ||
        isQuotaError(error) ||
        options.signal?.aborted ||
        error?.name === "AbortError"
      ) {
        throw error
      }
      throw classifyStorageError(error, `opening staging for ${stem.identity}`, {
        identity: stem.identity,
      })
    }
  }

  async #promote(stem, stagingHandle, stagingName, options) {
    const finalName = stemFileName(stem.identity)
    if (typeof stagingHandle.move === "function") {
      try {
        await stagingHandle.move(this.#directory, finalName)
        await this.#hooks.afterMove?.({ identity: stem.identity, finalName })
        return
      } catch (error) {
        if (!isMoveUnsupported(error)) throw error
      }
    }

    const stagedFile = await withDeadline(
      stagingHandle.getFile(),
      this.#readDeadlineMs,
      "storage.read_stalled",
      `Reading staged ${stem.identity} made no progress`,
      options.signal
    )
    const finalHandle = await this.#directory.getFileHandle(finalName, {
      create: true,
    })
    let writablePromise
    let writable
    let reader
    try {
      writablePromise = finalHandle.createWritable()
      writable = await withDeadline(
        writablePromise,
        this.#readDeadlineMs,
        "storage.write_stalled",
        `Opening fallback final for ${stem.identity} made no progress`,
        options.signal
      )
      reader = stagedFile.stream().getReader()
      while (true) {
        throwIfAborted(options.signal)
        const result = await withDeadline(
          reader.read(),
          this.#readDeadlineMs,
          "storage.read_stalled",
          `Copying ${stem.identity} made no progress`,
          options.signal
        )
        if (result.done) break
        await withDeadline(
          writable.write(result.value),
          this.#readDeadlineMs,
          "storage.write_stalled",
          `Writing fallback final for ${stem.identity} made no progress`,
          options.signal
        )
      }
      await withDeadline(
        writable.close(),
        this.#readDeadlineMs,
        "storage.write_stalled",
        `Closing fallback final for ${stem.identity} made no progress`,
        options.signal
      )
    } catch (error) {
      // A timed-out OPFS operation may never settle. Start cleanup, but do not
      // re-await that same writable before returning typed cancellation. The
      // second removal attempt runs if a late create/abort eventually settles.
      if (writablePromise !== undefined) {
        void abortFallbackWritable(
          writablePromise,
          error,
          this.#directory,
          finalName
        )
      }
      void removeEntry(this.#directory, finalName).catch(() => {})
      void reader?.cancel(error).catch(() => {})
      throw error
    } finally {
      if (reader !== undefined) releaseReader(reader)
    }
    await this.#hooks.afterFallbackCopy?.({
      identity: stem.identity,
      finalHandle,
      finalName,
      stagingName,
    })
    const observed = await this.#hashFile(finalHandle, {
      signal: options.signal,
      onChunk: options.onProgress,
      stage: "verified-final",
      identity: stem.identity,
    })
    if (
      observed.bytes !== stem.bytes ||
      observed.hex !== stemDigest(stem.identity)
    ) {
      await removeEntry(this.#directory, finalName)
      throw new StemStoreError(
        "stem.ingest.integrity",
        `Fallback promotion verification failed for ${stem.identity}`,
        { identity: stem.identity }
      )
    }
  }

  async #verifyIndexed(stem, onProgress, signal) {
    const index = await this.#withIndexLock(() => this.#readIndex(true), signal)
    if (index.stems[stem.identity] === undefined) return false
    const handle = await this.#fileHandle(stemFileName(stem.identity), false)
    if (handle === null) {
      await this.#demote(stem.identity)
      return false
    }
    try {
      const file = await withDeadline(
        handle.getFile(),
        this.#readDeadlineMs,
        "storage.read_stalled",
        `Opening ${stem.identity} made no progress`,
        signal
      )
      // A wrong length is corruption without reading a byte: refuse before
      // the full re-hash. The hash below still checks the bytes it read.
      if (file.size !== stem.bytes) {
        await this.#demote(stem.identity)
        onProgress?.({
          stage: "corrupt",
          identity: stem.identity,
          expectedBytes: stem.bytes,
          observedBytes: file.size,
        })
        return false
      }
      const observed = await this.#hashOpenFile(file, stem, onProgress, signal)
      if (
        observed.bytes !== stem.bytes ||
        observed.hex !== stemDigest(stem.identity)
      ) {
        await this.#demote(stem.identity)
        onProgress?.({
          stage: "corrupt",
          identity: stem.identity,
          expectedBytes: stem.bytes,
          observedBytes: observed.bytes,
        })
        return false
      }
      await this.#mutateIndex((latest) => {
        const row = latest.stems[stem.identity]
        if (row !== undefined) row.lastUsedAt = this.#now()
      })
      return true
    } catch (error) {
      if (signal?.aborted || error?.name === "AbortError") throw error
      await this.#demote(stem.identity)
      return false
    }
  }

  async #hashFile(handle, options) {
    const file =
      options.file ??
      (await withDeadline(
        handle.getFile(),
        this.#readDeadlineMs,
        "storage.read_stalled",
        `Opening ${options.identity} made no progress`,
        options.signal
      ))
    const reader = file.stream().getReader()
    const hash = new IncrementalBlake3()
    let bytes = 0
    try {
      while (true) {
        throwIfAborted(options.signal)
        const result = await withDeadline(
          reader.read(),
          this.#readDeadlineMs,
          "storage.read_stalled",
          `Reading ${options.identity} made no progress`,
          options.signal
        )
        if (result.done) break
        hash.update(result.value)
        bytes += result.value.byteLength
        options.onChunk?.({
          stage: options.stage,
          identity: options.identity,
          bytes,
          totalBytes: file.size,
        })
      }
    } catch (error) {
      void reader.cancel(error).catch(() => {})
      throw error
    } finally {
      if (options.signal?.aborted) {
        void reader.cancel(options.signal.reason).catch(() => {})
      }
      releaseReader(reader)
    }
    return { bytes, hex: hash.digestHex() }
  }

  async #hashOpenFile(file, stem, onProgress, signal) {
    return this.#hashFile(undefined, {
      file,
      signal,
      onChunk: onProgress,
      stage: "verified-open",
      identity: stem.identity,
    })
  }

  async #indexPromoted(stem) {
    await this.#mutateIndex((index) => {
      index.stems[stem.identity] = {
        bytes: stem.bytes,
        lastUsedAt: this.#now(),
        pins: index.stems[stem.identity]?.pins ?? [],
      }
    })
  }

  async #preflight(missing, protectedIdentities) {
    if (missing.length === 0 || typeof this.#storage?.estimate !== "function") return
    const requiredBytes = checkedByteSum(
      missing.map((stem) => stem.bytes),
      "missing stem byte total"
    )
    let estimate
    try {
      estimate = await this.#storage.estimate()
    } catch {
      return
    }
    if (!finiteNonnegative(estimate.quota) || !finiteNonnegative(estimate.usage)) return
    let availableBytes = Math.max(0, estimate.quota - estimate.usage)
    if (availableBytes >= requiredBytes) return

    const index = await this.#withIndexLock(() => this.#readIndex(true))
    const victims = this.#evictableRows(index, protectedIdentities)
      .sort((left, right) => left[1].lastUsedAt - right[1].lastUsedAt)
    const evictableBytes = checkedByteSum(
      victims.map((entry) => entry[1].bytes),
      "evictable stem byte total"
    )
    for (const [identity, row] of victims) {
      if (availableBytes >= requiredBytes) break
      const evicted = await this.#evictUnpinned(identity)
      if (evicted) availableBytes += row.bytes
    }
    if (availableBytes < requiredBytes) {
      throw new StemStoreError(
        "storage.insufficient",
        `Stem store is short ${requiredBytes - availableBytes} bytes`,
        {
          requiredBytes,
          availableBytes,
          shortfallBytes: requiredBytes - availableBytes,
          evictableBytes,
        }
      )
    }
  }

  async #evictUnpinned(identity) {
    return this.#withStemLock(identity, undefined, async () => {
      let evicted = false
      await this.#mutateIndex(async (index) => {
        const row = index.stems[identity]
        if (row === undefined || row.pins.length !== 0) return
        await removeEntry(this.#directory, stemFileName(identity))
        delete index.stems[identity]
        evicted = true
      })
      return evicted
    })
  }

  #evictableRows(index, protectedIdentities) {
    return Object.entries(index.stems).filter(
      ([identity, row]) =>
        row.pins.length === 0 && !protectedIdentities.has(identity)
    )
  }

  async #quotaError(requiredBytes, cause, protectedIdentities) {
    let availableBytes = null
    let evictableBytes = 0
    try {
      const estimate = await this.#storage.estimate?.()
      if (finiteNonnegative(estimate?.quota) && finiteNonnegative(estimate?.usage)) {
        availableBytes = Math.max(0, estimate.quota - estimate.usage)
      }
    } catch {
      // The quota exception remains authoritative when the estimate races or fails.
    }
    try {
      const index = await this.#withIndexLock(() => this.#readIndex(true))
      evictableBytes = checkedByteSum(
        this.#evictableRows(index, protectedIdentities).map(
          ([, row]) => row.bytes
        ),
        "evictable stem byte total"
      )
    } catch {
      // Index damage cannot make the write-time quota refusal disappear.
    }
    const estimatedShortfall =
      availableBytes === null ? 0 : requiredBytes - availableBytes
    // A QuotaExceededError proves some nonzero shortfall even when estimate()
    // raced the write and still claims enough space. One byte is the only
    // universally truthful lower bound in that case.
    const shortfallBytes = Math.max(1, estimatedShortfall)
    return new StemStoreError(
      "storage.insufficient",
      `Stem store write is short at least ${shortfallBytes} bytes`,
      {
        requiredBytes,
        availableBytes,
        shortfallBytes,
        shortfallIsLowerBound:
          availableBytes === null || availableBytes >= requiredBytes,
        evictableBytes,
      },
      cause
    )
  }

  async #demote(identity) {
    // The row goes first, so a delete that hangs on a handle that already
    // failed cannot keep the stem indexed. The delete is deadlined and
    // best-effort: a final left behind is unindexed debris that the next
    // open sweeps.
    await this.#mutateIndex((index) => {
      delete index.stems[identity]
    })
    await withDeadline(
      removeEntry(this.#directory, stemFileName(identity)),
      this.#readDeadlineMs,
      "storage.remove_stalled",
      `Removing ${identity} made no progress`
    ).catch(() => {})
  }

  async #readIndex(repair) {
    let parsed
    try {
      const handle = await this.#directory.getFileHandle(INDEX_FILE)
      const file = await withDeadline(
        handle.getFile(),
        this.#readDeadlineMs,
        "storage.read_stalled",
        "Reading the stem index made no progress"
      )
      parsed = JSON.parse(
        await withDeadline(
          file.text(),
          this.#readDeadlineMs,
          "storage.read_stalled",
          "Reading the stem index body made no progress"
        )
      )
    } catch (error) {
      if (error?.name === "NotFoundError") {
        if (!repair) return emptyIndex()
        return this.#rebuildIndex()
      }
      if (!repair) throw error
      return this.#rebuildIndex()
    }
    if (validIndex(parsed)) return parsed
    if (!repair) throw new StemStoreError("storage.index.invalid", "Stem index is invalid")
    return this.#rebuildIndex()
  }

  async #rebuildIndex() {
    const rebuilt = emptyIndex()
    const liveLocks = await this.#liveLockNames()
    for await (const [name, handle] of this.#directory.entries()) {
      const match = FINAL_NAME.exec(name)
      if (match === null || handle.kind !== "file") continue
      const identity = `blake3:${match[1]}`
      // A final protected by an in-flight promote is not yet an unambiguous
      // crash artifact. Its owner will either index it or leave it for a later
      // recovery pass after releasing the lock.
      if (liveLocks?.has(this.#stemLockName(identity))) continue
      try {
        const observed = await this.#hashFile(handle, {
          stage: "verify-rebuild",
          identity,
        })
        if (observed.hex === match[1]) {
          rebuilt.stems[identity] = {
            bytes: observed.bytes,
            lastUsedAt: this.#now(),
            pins: [],
          }
        } else {
          await removeEntry(this.#directory, name)
        }
      } catch {
        await removeEntry(this.#directory, name)
      }
    }
    await this.#writeIndex(rebuilt)
    return rebuilt
  }

  async #mutateIndex(mutation) {
    return this.#withIndexLock(async () => {
      const index = await this.#readIndex(true)
      const result = await mutation(index)
      await this.#writeIndex(index)
      return result
    })
  }

  // Runs inside the index lock, so every await is deadlined: one wedged
  // write must not block every stem in the realm.
  async #writeIndex(index) {
    const deadline = (operation, step) =>
      withDeadline(
        operation,
        this.#readDeadlineMs,
        "storage.write_stalled",
        `${step} the stem index made no progress`
      )
    const handle = await deadline(
      this.#directory.getFileHandle(INDEX_FILE, { create: true }),
      "Opening"
    )
    const writable = await deadline(handle.createWritable(), "Opening")
    try {
      await deadline(writable.write(`${JSON.stringify(index)}\n`), "Writing")
      await deadline(writable.close(), "Closing")
    } catch (error) {
      await deadline(writable.abort(error), "Aborting").catch(() => {})
      throw error
    }
  }

  async #bestEffortSweep(sweep, work) {
    try {
      await work()
    } catch (error) {
      this.#reportMaintenanceFailure(sweep, error)
    }
  }

  #reportMaintenanceFailure(sweep, error) {
    this.#maintenanceFailures += 1
    try {
      this.#hooks.maintenanceFailure?.({
        sweep,
        error,
        failures: this.#maintenanceFailures,
      })
    } catch {
      // A reporting hook cannot make maintenance fatal.
    }
  }

  /** Each step is deadlined by the per-step and the overall sweep budget. */
  #sweepStep(operation, budgetEnd, what) {
    const remaining = Math.max(
      1,
      Math.min(this.#readDeadlineMs, Math.ceil(budgetEnd - monotonicNow()))
    )
    return withDeadline(
      operation,
      remaining,
      "storage.maintenance_stalled",
      `${what} made no progress`
    )
  }

  /** Iterate a directory with every step deadlined; the caller sees each entry. */
  async #sweepEntries(sweep, directory, budgetEnd, visit) {
    const iterator = directory.entries()
    try {
      while (true) {
        if (monotonicNow() >= budgetEnd) {
          throw new StemStoreError(
            "storage.maintenance_stalled",
            "Stem store maintenance ran out of its budget"
          )
        }
        const step = await this.#sweepStep(
          iterator.next(),
          budgetEnd,
          "Listing the stem store"
        )
        if (step.done) return
        // One failing entry is reported and skipped; the rest still sweep.
        try {
          await visit(step.value[0], step.value[1])
        } catch (error) {
          this.#reportMaintenanceFailure(sweep, error)
        }
      }
    } finally {
      // A timed-out step may never settle; do not wait on the iterator.
      void Promise.resolve()
        .then(() => iterator.return?.())
        .catch(() => {})
    }
  }

  async #removeUnindexedFinals(index, liveLocks, budgetEnd) {
    await this.#sweepEntries("finals", this.#directory, budgetEnd, async (name, handle) => {
      const match = FINAL_NAME.exec(name)
      if (match === null || handle.kind !== "file") return
      const identity = `blake3:${match[1]}`
      if (
        index.stems[identity] === undefined &&
        !liveLocks?.has(this.#stemLockName(identity))
      ) {
        await this.#sweepStep(
          removeEntry(this.#directory, name),
          budgetEnd,
          `Removing ${name}`
        )
      }
    })
  }

  async #liveLockNames() {
    if (typeof this.#locks?.query === "function") {
      try {
        const snapshot = await this.#locks.query()
        const liveLocks = new Set()
        for (const lock of [...(snapshot.held ?? []), ...(snapshot.pending ?? [])]) {
          liveLocks.add(lock.name)
        }
        return liveLocks
      } catch {
        return null
      }
    }
    return null
  }

  async #sweepStaging(liveLocks, budgetEnd) {
    await this.#sweepEntries("staging", this.#staging, budgetEnd, async (name, handle) => {
      if (handle.kind !== "file") return
      const digest = name.slice(-64)
      const lockName = this.#stemLockName(`blake3:${digest}`)
      if (liveLocks?.has(lockName)) return
      let oldEnough = true
      if (liveLocks === null) {
        try {
          const file = await this.#sweepStep(
            handle.getFile(),
            budgetEnd,
            `Reading staged ${name}`
          )
          oldEnough = this.#now() - file.lastModified >= 30_000
        } catch {
          oldEnough = true
        }
      }
      if (oldEnough) {
        await this.#sweepStep(
          removeEntry(this.#staging, name),
          budgetEnd,
          `Removing staged ${name}`
        )
      }
    })
  }

  #purgeDeadSessionPins(index, liveLocks) {
    if (liveLocks === null) return false
    let changed = false
    for (const row of Object.values(index.stems)) {
      const pins = row.pins.filter(
        (pin) =>
          !pin.startsWith("session:") ||
          liveLocks.has(this.#sessionPinLockName(pin))
      )
      if (pins.length !== row.pins.length) {
        row.pins = pins
        changed = true
      }
    }
    return changed
  }

  async #acquireSessionPinLock(pin) {
    if (typeof this.#locks?.request !== "function") return () => {}
    let acquired
    let acquisitionFailed
    let release
    const ready = new Promise((resolve, reject) => {
      acquired = resolve
      acquisitionFailed = reject
    })
    const hold = new Promise((resolve) => {
      release = resolve
    })
    const request = this.#locks.request(
      this.#sessionPinLockName(pin),
      { mode: "exclusive" },
      async () => {
        acquired()
        await hold
      }
    )
    request.catch(acquisitionFailed)
    await ready
    request.catch(() => {})
    return release
  }

  async #fileHandle(name, create) {
    try {
      return await this.#directory.getFileHandle(name, { create })
    } catch (error) {
      if (!create && error?.name === "NotFoundError") return null
      throw error
    }
  }

  #stemLockName(identity) {
    return `${LOCK_PREFIX}:${this.#folderName}:ingest:${stemDigest(identity)}`
  }

  #indexLockName() {
    return `${LOCK_PREFIX}:${this.#folderName}:index`
  }

  #sessionPinLockName(pin) {
    return `${LOCK_PREFIX}:${this.#folderName}:pin:${pin}`
  }

  #withStemLock(identity, signal, work) {
    return this.#withLock(this.#stemLockName(identity), signal, work)
  }

  #withIndexLock(work, signal) {
    return this.#withLock(this.#indexLockName(), signal, work)
  }

  async #withLock(name, signal, work) {
    throwIfAborted(signal)
    if (typeof this.#locks?.request === "function") {
      return this.#locks.request(name, { mode: "exclusive", signal }, async () => {
        throwIfAborted(signal)
        return work()
      })
    }
    const previous = this.#localLockTails.get(name) ?? Promise.resolve()
    let release
    const gate = new Promise((resolve) => {
      release = resolve
    })
    const tail = previous.then(() => gate)
    this.#localLockTails.set(name, tail)
    await previous
    try {
      throwIfAborted(signal)
      return await work()
    } finally {
      release()
      if (this.#localLockTails.get(name) === tail) this.#localLockTails.delete(name)
    }
  }
}

/** A hard-gate lease whose pins live exactly as long as its open session. */
export class StemSessionLease {
  #store
  #pin
  #stems
  #closed = false

  constructor(store, sessionId, pin, stems, releasePinLock) {
    this.#store = store
    this.sessionId = sessionId
    this.#pin = pin
    this.#stems = stems
    this.releasePinLock = releasePinLock
  }

  get stems() {
    return this.#stems.map((stem) => ({ ...stem }))
  }

  async read(identity) {
    if (this.#closed) throw new StemStoreError("stem.session.closed", "Session is closed")
    if (!this.#stems.some((stem) => stem.identity === identity)) {
      throw new StemStoreError(
        "stem.session.unreferenced",
        `Session does not reference ${identity}`,
        { identity }
      )
    }
    return this.#store.read(identity)
  }

  async close() {
    if (this.#closed) return
    this.#closed = true
    try {
      await this.#store.releaseSessionPin(this.#pin, this.#stems)
    } finally {
      this.releasePinLock()
    }
  }
}

/**
 * Chromium exposes a real shared-reader mode through `handle.mode`. Firefox
 * accepts the option but omits that property, and WebKit ignores the option;
 * both remain exclusive. Existing-file contention errors have engine-specific
 * names, so every rejection declines this mode and leaves reads on `getFile()`.
 */
export async function detectSharedReadOnlyMode(fileHandle) {
  if (typeof fileHandle?.createSyncAccessHandle !== "function") return false
  let access
  try {
    access = await fileHandle.createSyncAccessHandle({ mode: "read-only" })
    return access?.mode === "read-only"
  } catch {
    // Do not inspect the error name: WebKit and Blink/Gecko disagree on it.
    return false
  } finally {
    await access?.close?.()
  }
}

function normalizeRequirements(input) {
  if (!Array.isArray(input)) throw new TypeError("stems must be an array")
  const unique = new Map()
  for (const stem of input) {
    const identity = nonemptyText(stem?.identity, "stem.identity")
    stemDigest(identity)
    const bytes = nonnegativeInteger(stem?.bytes, "stem.bytes")
    const prior = unique.get(identity)
    if (prior !== undefined && prior.bytes !== bytes) {
      throw new StemStoreError(
        "stem.declaration.conflict",
        `Stem ${identity} has conflicting byte lengths`,
        { identity, firstBytes: prior.bytes, secondBytes: bytes }
      )
    }
    unique.set(identity, { identity, bytes })
  }
  return Array.from(unique.values())
}

function emptyIndex() {
  return { version: INDEX_VERSION, stems: {} }
}

function validIndex(value) {
  if (
    value === null ||
    typeof value !== "object" ||
    value.version !== INDEX_VERSION ||
    value.stems === null ||
    typeof value.stems !== "object" ||
    Array.isArray(value.stems)
  ) {
    return false
  }
  return Object.entries(value.stems).every(([identity, row]) => {
    try {
      stemDigest(identity)
    } catch {
      return false
    }
    return (
      row !== null &&
      typeof row === "object" &&
      nonnegativeSafe(row.bytes) &&
      nonnegativeSafe(row.lastUsedAt) &&
      Array.isArray(row.pins) &&
      row.pins.every((pin) => typeof pin === "string")
    )
  })
}

async function removeEntry(directory, name) {
  try {
    await directory.removeEntry(name)
  } catch (error) {
    if (error?.name !== "NotFoundError") throw error
  }
}

async function abortFallbackWritable(writablePromise, reason, directory, name) {
  try {
    const writable = await writablePromise
    await writable.abort(reason)
  } catch {
    // The original typed abort/deadline error remains authoritative.
  }
  await removeEntry(directory, name).catch(() => {})
}

function classifyStorageError(error, action, overrides = {}) {
  if (error instanceof StemStoreError) return error
  if (isQuotaError(error)) {
    return new StemStoreError(
      "storage.insufficient",
      `Origin-private storage ran out while ${action}`,
      overrides,
      error
    )
  }
  return new StemStoreError(
    overrides.code ?? "storage.unavailable",
    `Origin-private storage failed while ${action}`,
    overrides,
    error
  )
}

function isQuotaError(error) {
  return error?.name === "QuotaExceededError"
}

function isMoveUnsupported(error) {
  return ["NotSupportedError", "InvalidModificationError", "TypeError"].includes(
    error?.name
  )
}

function randomTabId() {
  return globalThis.crypto?.randomUUID?.().replaceAll("-", "") ??
    Math.random().toString(36).slice(2)
}

function validateFolderName(value) {
  const name = nonemptyText(value, "folderName")
  if (name === "." || name === ".." || name.includes("/")) {
    throw new RangeError("folderName must be one OPFS path component")
  }
  return name
}

function nonemptyText(value, name) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(`${name} must be non-empty text`)
  }
  return value
}

function positiveInteger(value, name) {
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new RangeError(`${name} must be a positive safe integer`)
  }
  return value
}

function nonnegativeInteger(value, name) {
  if (!nonnegativeSafe(value)) {
    throw new RangeError(`${name} must be a nonnegative safe integer`)
  }
  return value
}

function nonnegativeSafe(value) {
  return Number.isSafeInteger(value) && value >= 0
}

function finiteNonnegative(value) {
  return typeof value === "number" && Number.isFinite(value) && value >= 0
}

function checkedByteSum(values, label) {
  let total = 0
  for (const value of values) {
    total += value
    if (!Number.isSafeInteger(total)) {
      throw new StemStoreError(
        "storage.size_overflow",
        `${label} exceeds JavaScript's exact integer range`,
        { label }
      )
    }
  }
  return total
}

function releaseReader(reader) {
  try {
    reader.releaseLock()
  } catch {
    // A timed-out read may remain pending forever. Its Blob is abandoned; the
    // deadline would be meaningless if cleanup waited on the same operation.
  }
}

function throwIfAborted(signal) {
  if (signal?.aborted) {
    throw signal.reason ?? new DOMException("Stem operation aborted", "AbortError")
  }
}

function monotonicNow() {
  return globalThis.performance?.now?.() ?? Date.now()
}

function forwardAbort(signal, controller) {
  if (signal === undefined) return () => {}
  const abort = () => controller.abort(signal.reason)
  if (signal.aborted) abort()
  else signal.addEventListener("abort", abort, { once: true })
  return () => signal.removeEventListener("abort", abort)
}

async function withDeadline(
  operation,
  milliseconds,
  code,
  message,
  signal,
  TimeoutError = StemStoreError
) {
  throwIfAborted(signal)
  let timer
  let abort
  const interruption = new Promise((_resolve, reject) => {
    timer = setTimeout(
      () => reject(new TimeoutError(code, message, { milliseconds })),
      milliseconds
    )
    if (signal !== undefined) {
      abort = () =>
        reject(
          signal.reason ??
            new DOMException("Stem operation aborted", "AbortError")
        )
      signal.addEventListener("abort", abort, { once: true })
      if (signal.aborted) abort()
    }
  })
  try {
    return await Promise.race([operation, interruption])
  } finally {
    clearTimeout(timer)
    if (abort !== undefined) signal.removeEventListener("abort", abort)
  }
}
