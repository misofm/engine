import { StemStoreError } from "./opfs-store.js"

/**
 * Session-open orchestration. `resume()` is invoked synchronously, before the
 * first await, so a browser counts it inside the user's gesture; interaction
 * remains closed until the store lease proves every stem at this open.
 */
export class StemSessionGate {
  #store
  #resolver
  #opening
  #lease
  #state = "idle"

  constructor(store, resolver) {
    if (typeof store?.openSession !== "function") {
      throw new TypeError("StemSessionGate needs an OpfsStemStore")
    }
    if (typeof resolver?.resolve !== "function") {
      throw new TypeError("StemSessionGate needs a StemResolver")
    }
    this.#store = store
    this.#resolver = resolver
  }

  get state() {
    return this.#state
  }

  /**
   * @param {{sessionId: string, stems: Array<{identity: string, bytes: number}>, resume: () => unknown, onProgress?: (event: object) => void, signal?: AbortSignal}} options
   */
  async open(options) {
    if (typeof options.resume !== "function") {
      throw new TypeError("Session open needs a user-gesture resume callback")
    }

    this.#opening?.abort(
      new DOMException("Superseded by another session open", "AbortError")
    )
    const opening = new AbortController()
    this.#opening = opening
    const removeParentAbort = forwardAbort(options.signal, opening)

    // This call must stay above every `await` in this method.
    let resumeResult
    try {
      resumeResult = options.resume()
    } catch (error) {
      this.#opening = undefined
      removeParentAbort()
      throw new StemStoreError(
        "session.resume.failed",
        "The audio context could not resume inside the user gesture",
        {},
        error
      )
    }
    const resumeOutcome = Promise.resolve(resumeResult).then(
      () => ({ ok: true }),
      (error) => ({ ok: false, error })
    )

    this.#state = "loading"
    options.onProgress?.({ stage: "loading", sessionId: options.sessionId })
    let lease
    let storeOpening
    try {
      // The lease holds a Web Lock whose name includes the session ID. Release
      // an identically named predecessor before asking the store to acquire its
      // replacement; other session IDs keep their old lease until the new gate
      // is green so their pins remain protected during a mix switch.
      if (this.#lease?.sessionId === options.sessionId) {
        const replaced = this.#lease
        this.#lease = undefined
        await replaced.close()
      }
      storeOpening = this.#store.openSession({
        sessionId: options.sessionId,
        stems: options.stems,
        resolver: this.#resolver,
        signal: opening.signal,
        onProgress: options.onProgress,
      })
      const [opened, resumed] = await Promise.all([
        storeOpening,
        resumeOutcome,
      ])
      if (!resumed.ok) throw resumed.error
      lease = opened
      if (this.#opening !== opening) {
        await lease.close()
        throw new DOMException("Superseded by another session open", "AbortError")
      }
      await this.#lease?.close()
      this.#lease = lease
      this.#state = "interactive"
      options.onProgress?.({
        stage: "interactive",
        sessionId: options.sessionId,
      })
      return lease
    } catch (error) {
      opening.abort(error)
      const orphanedLease = await storeOpening?.catch(() => undefined)
      if (orphanedLease !== lease) await orphanedLease?.close().catch(() => {})
      await lease?.close().catch(() => {})
      if (this.#opening === opening) this.#state = "refused"
      throw error
    } finally {
      removeParentAbort()
      if (this.#opening === opening) this.#opening = undefined
    }
  }

  async close() {
    this.#opening?.abort(new DOMException("Session gate closed", "AbortError"))
    this.#opening = undefined
    await this.#lease?.close()
    this.#lease = undefined
    this.#state = "idle"
  }
}

function forwardAbort(signal, controller) {
  if (signal === undefined) return () => {}
  const abort = () => controller.abort(signal.reason)
  if (signal.aborted) abort()
  else signal.addEventListener("abort", abort, { once: true })
  return () => signal.removeEventListener("abort", abort)
}
