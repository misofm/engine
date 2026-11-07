import { StemStoreError } from "./opfs-store.js"

/**
 * Session-open orchestration. `resume()` is invoked synchronously, before the
 * first await, so a browser counts it inside the user's gesture; interaction
 * remains closed until the store lease proves every stem at this open.
 *
 * Each open owns one AbortController, `opening`, and it owns the gate only
 * while that controller is unaborted: a newer open, `close()` and the caller's
 * own signal all abort it. An open writes `#lease` and `#state` only after
 * `opening.signal.throwIfAborted()` with no await in between, so a superseded
 * open can never commit, and its store lease (with its session pin lock) is
 * closed on the way out instead of blocking a newer open of the same session.
 * The wait for `resume()` ends at that abort too, and the store open takes the
 * same signal; only the bounded lease closes are awaited to completion.
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
      // An open in flight was aborted above and will no longer write the
      // state, so this refusal is the gate's terminal state.
      if (this.#state === "loading") this.#state = "refused"
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
      lease = await this.#store.openSession({
        sessionId: options.sessionId,
        stems: options.stems,
        resolver: this.#resolver,
        signal: opening.signal,
        onProgress: options.onProgress,
      })
      // A resume that never settles must not keep a superseded open's lease
      // (and its pin) alive, so the wait for it ends at the abort.
      const resumed = await abortable(resumeOutcome, opening.signal)
      if (!resumed.ok) throw resumed.error
      // The abort can land after `resumed` settles and before this line runs.
      opening.signal.throwIfAborted()
      const displaced = this.#lease
      if (displaced !== undefined) {
        this.#lease = undefined
        await displaced.close()
        opening.signal.throwIfAborted()
      }
      this.#lease = lease
      this.#state = "interactive"
      options.onProgress?.({
        stage: "interactive",
        sessionId: options.sessionId,
      })
      return lease
    } catch (error) {
      // Every path here has seen the store open settle, or never started it,
      // so `lease` is the only lease this open can hold.
      await lease?.close().catch(() => {})
      if (this.#opening === opening) this.#state = "refused"
      throw error
    } finally {
      removeParentAbort()
      if (this.#opening === opening) this.#opening = undefined
    }
  }

  // Every write happens before the await, so an open started while the lease
  // closes owns the state from then on.
  async close() {
    this.#opening?.abort(new DOMException("Session gate closed", "AbortError"))
    this.#opening = undefined
    const lease = this.#lease
    this.#lease = undefined
    this.#state = "idle"
    await lease?.close()
  }
}

/** Settles as `promise` does, or rejects with the abort reason once `signal` aborts. */
function abortable(promise, signal) {
  if (signal.aborted) return Promise.reject(signal.reason)
  let abort
  return new Promise((resolve, reject) => {
    abort = () => reject(signal.reason)
    signal.addEventListener("abort", abort, { once: true })
    promise.then(resolve, reject)
  }).finally(() => signal.removeEventListener("abort", abort))
}

function forwardAbort(signal, controller) {
  if (signal === undefined) return () => {}
  const abort = () => controller.abort(signal.reason)
  if (signal.aborted) abort()
  else signal.addEventListener("abort", abort, { once: true })
  return () => signal.removeEventListener("abort", abort)
}
