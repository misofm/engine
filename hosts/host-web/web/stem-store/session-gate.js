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
 *
 * A resolved `close()` has released every pin the gate's opens hold or can
 * still take (#2372). It waits for the current lease's close, for a
 * same-session predecessor an open took out of the gate and is closing, and
 * for every open that reached the store's pin phase (`onPinning`), since only
 * those can hold a pin. Such an open settles only after closing every lease it
 * was granted or displaced, and from the pin phase on each of its waits ends at
 * its abort or is the store's own index work, which a lease close also waits
 * for. An open aborted before the pin phase holds no pin and never takes one,
 * so `close()` does not wait on it, whatever the store work it is still in.
 */
export class StemSessionGate {
  #store
  #resolver
  #opening
  #lease
  #state = "idle"
  // Settle when a lease or a pin of this gate is released; close() waits for them.
  #releasing = new Set()

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
    // Settlement is tracked on a private promise, never on the caller's, so a
    // rejected open the caller drops is still reported as unhandled.
    let settle
    const settled = new Promise((resolve) => (settle = resolve))
    try {
      return await this.#open(options, () => this.#track(settled))
    } finally {
      settle()
    }
  }

  #track(promise) {
    const released = promise.then(
      () => {},
      () => {}
    )
    this.#releasing.add(released)
    void released.then(() => this.#releasing.delete(released))
    return promise
  }

  async #open(options, onPinning) {
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
        // Before the pin phase, so this close is tracked on its own.
        await this.#track(replaced.close())
      }
      lease = await this.#store.openSession({
        sessionId: options.sessionId,
        stems: options.stems,
        resolver: this.#resolver,
        signal: opening.signal,
        onProgress: options.onProgress,
        onPinning,
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
        // After the pin phase, so close() already waits for this open.
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
  // closes owns the state from then on. Every open in flight is aborted by now
  // (the newest one just above), and each closes its own leases on the way out.
  async close() {
    this.#opening?.abort(new DOMException("Session gate closed", "AbortError"))
    this.#opening = undefined
    const lease = this.#lease
    this.#lease = undefined
    this.#state = "idle"
    const closing = lease?.close()
    await Promise.allSettled([closing, ...this.#releasing])
    await closing
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
