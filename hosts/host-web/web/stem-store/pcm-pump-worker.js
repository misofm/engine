import { OpfsStemStore } from "./opfs-store.js"
import { CanonicalPcmPump, CanonicalPcmPumpError, Msb1RingWriter } from "./pcm-pump.js"

/**
 * The idle backoff when a `pumpUntilFull()` pass writes nothing (issue #278).
 *
 * Zero chunks means every ring is full, so the only thing that can change the
 * answer is the render thread consuming a slot. Four milliseconds is under two
 * quanta at 48 kHz / 128 frames, so a freed slot is refilled well inside the
 * ring's remaining depth, and it is long enough that a full ring costs a couple
 * of hundred wakeups a second rather than a spin.
 */
const DEFAULT_IDLE_MS = 4

let messageTail = Promise.resolve()

/**
 * The Worker's one session, as an explicit phase (issue #2263). Exactly one of:
 *
 * - `IDLE`: no session, before the first `initialize` and after `stop`. `seek`
 *   and `pump` are refused as not initialized.
 * - `{ kind: "running", pump, lease, idleMs, drive, idleWake }`: a live pump.
 *   `idleMs` is the opt-in cadence's backoff (#278), `undefined` for the
 *   pull-driven Worker; `drive` is the token of the loop now running, if any,
 *   and `idleWake` cancels that loop's idle sleep.
 * - `{ kind: "failed", error, lease }`: the session answered a `session-error`
 *   and its pump is stopped. It holds no pump and no cadence, so nothing can
 *   drive it: `seek` and `pump` are refused with `stem.pump.session_failed`
 *   until `initialize` starts a new session, and `stop` returns to `IDLE`.
 *
 * Every transition runs as a link in `messageTail`, so a message and a cadence
 * tick never observe each other half-done. A running session is left only
 * through `leaveRunning`, which ends its loop, cancels its idle timer and stops
 * its pump, whichever path the session ends on.
 */
const IDLE = Object.freeze({ kind: "idle" })
let session = IDLE

self.onmessage = (event) => {
  messageTail = messageTail.then(() => handleMessage(event)).catch((error) => {
    reportSessionError(error, event.data?.requestId)
  })
}

async function handleMessage(event) {
  const message = event.data
  if (message?.type === "initialize") {
    await endSession()
    // Validated before any ring is engaged, so a refused option leaves nothing
    // engaged that no session owns.
    const idleMs = requestedIdleMs(message.selfDriving)
    const store = new OpfsStemStore({ folderName: message.folderName })
    await store.open()
    // The main-realm loading gate owns verification and pins. This narrow
    // lease facade limits the Worker to getFile()/slice reads only.
    const lease = {
      read: (identity) => store.read(identity),
      close: async () => {},
    }
    const pump = new CanonicalPcmPump({
      lease,
      windowFrames: message.windowFrames,
      generation: BigInt(message.generation ?? 1),
      sources: message.sources.map((source) => ({
        ...source,
        ring: new Msb1RingWriter(source.ring),
      })),
    })
    const running = {
      kind: "running",
      pump,
      lease,
      idleMs,
      drive: undefined,
      idleWake: undefined,
    }
    session = running
    self.postMessage({ type: "initialized", requestId: message.requestId })
    startDriving(running)
    return
  }
  if (message?.type === "pump") {
    const running = runningSession()
    self.postMessage({
      type: "pumped",
      requestId: message.requestId,
      ...(await running.pump.pumpUntilFull()),
    })
    return
  }
  if (message?.type === "seek") {
    const running = runningSession()
    await running.pump.seek(message.frame)
    self.postMessage({ type: "sought", requestId: message.requestId })
    // A seek moves the cursor off end-of-region, so a loop that had exited on
    // `finished` has work again. Waking the idle timer is the other half: a
    // seek backwards through a full ring is exactly the case where the loop is
    // asleep and the rings it filled are now the wrong generation.
    running.idleWake?.()
    startDriving(running)
    return
  }
  if (message?.type === "stop") {
    await endSession()
    self.postMessage({ type: "stopped", requestId: message.requestId })
  }
}

/** The running session a `seek` or `pump` acts on; every other phase refuses. */
function runningSession() {
  if (session.kind === "running") return session
  if (session.kind === "failed") {
    throw new CanonicalPcmPumpError(
      "stem.pump.session_failed",
      "The PCM pump session failed; send initialize to start a new one",
      { failure: session.error }
    )
  }
  throw new Error("PCM pump is not initialized")
}

/** End the current session, whatever its phase, and return to `IDLE`. */
async function endSession() {
  const ending = session
  session = IDLE
  if (ending.kind === "running") leaveRunning(ending)
  await ending.lease?.close()
}

/** End a running session's loop and idle sleep, and stop its pump. */
function leaveRunning(running) {
  running.drive = undefined
  running.idleWake?.()
  running.pump.stop()
}

/**
 * The in-Worker cadence, opt-in per `initialize` (issue #278).
 *
 * # Why this exists
 *
 * `pcm-pump-worker.js` was pull-driven and nothing scheduled the next `pump`.
 * The shipped worker was therefore mountable only by a consumer that had also
 * re-implemented a cadence over the message vocabulary, which is the half
 * everyone would have written slightly differently. This is that half, shipped
 * once, and off unless asked for.
 *
 * # Why the loop borrows `messageTail` instead of running beside it
 *
 * `pumpUntilFull()` and `seek()` both mutate the pump's per-source cursor,
 * window and generation. A loop awaiting `pumpUntilFull()` while a `seek`
 * message ran concurrently would interleave those mutations, and the visible
 * damage would be PCM committed under the pre-seek generation into a ring the
 * reader has already re-tagged -- a data race that reads as a glitch, not as an
 * error. `messageTail` is the Worker's one serialization point, so each tick is
 * enqueued onto it rather than running alongside it: a `seek` or `stop` lands
 * strictly between two ticks, never inside one, and needs no second lock.
 *
 * That is also why `stop` does not await the loop. `stop` runs *as* a link in
 * the tail, so awaiting a loop that enqueues onto the same tail would deadlock.
 * It leaves the session instead (`leaveRunning`): the tick already queued (if
 * any) sees that its session is no longer current and does nothing, and the
 * loop exits on the cleared token.
 *
 * # Why it posts nothing
 *
 * A self-driven pass has no `requestId` to answer, and posting bare `pumped`
 * notifications would turn a reply into a broadcast -- the same word meaning
 * two things depending on who started the pass. The ring's own `endOfRegion`
 * flag already tells the reader where the region ends, so the loop stays
 * silent on the happy path. A failure still reports on `session-error`,
 * exactly as an externally driven `pump` would, and fails the session.
 */
function startDriving(running) {
  if (session !== running || running.idleMs === undefined) return
  if (running.drive !== undefined) return
  const token = {}
  running.drive = token
  void drive(running, token).finally(() => {
    if (running.drive === token) running.drive = undefined
  })
}

async function drive(running, token) {
  while (running.drive === token) {
    const outcome = await enqueue(() => tick(running, token))
    if (running.drive !== token || outcome === undefined) return
    if (outcome.finished) {
      // Clear the token here rather than leaving it to the `finally`, which is a
      // microtask: a `seek` arriving before that microtask ran would see a token
      // that still looks live, decline to re-arm, and stall the session.
      running.drive = undefined
      return
    }
    if (outcome.chunks === 0) await sleep(running, running.idleMs)
  }
}

/**
 * One cadence pass, run as a link in the message tail. A failure fails the
 * session here, inside the tail, so no message lands between the pass that
 * failed and the phase that records it.
 */
async function tick(running, token) {
  if (session !== running || running.drive !== token) return undefined
  try {
    return await running.pump.pumpUntilFull()
  } catch (error) {
    reportSessionError(error)
    return undefined
  }
}

/** Run `work` as the next link in the message tail, and keep the tail alive. */
function enqueue(work) {
  const queued = messageTail.then(work)
  // The caller owns this rejection; the tail must not carry it, or the next
  // message would be skipped by a chain that is already rejected.
  messageTail = queued.then(
    () => {},
    () => {}
  )
  return queued
}

function sleep(running, ms) {
  return new Promise((resolve) => {
    const finish = () => {
      clearTimeout(timer)
      if (running.idleWake === finish) running.idleWake = undefined
      resolve()
    }
    const timer = setTimeout(finish, ms)
    running.idleWake = finish
  })
}

function requestedIdleMs(requested) {
  if (requested === undefined || requested === false) return undefined
  if (requested === true) return DEFAULT_IDLE_MS
  const idleMs = requested?.idleMs
  if (idleMs === undefined) return DEFAULT_IDLE_MS
  if (!Number.isInteger(idleMs) || idleMs < 0) {
    throw new RangeError("selfDriving.idleMs must be a nonnegative integer")
  }
  return idleMs
}

/**
 * Answer a failure on `session-error`, and fail a running session (issue
 * #2263): `leaveRunning` ends its loop, cancels its idle timer and stops its
 * pump, and the session becomes `failed`. A refusal from `IDLE` or from a
 * session that already failed changes no phase; a failed session keeps its
 * first error.
 */
function reportSessionError(error, requestId) {
  const record = {
    name: error?.name ?? "Error",
    code: error?.code ?? "stem.pump.failed",
    message: error?.message ?? String(error),
    details: error?.details ?? {},
  }
  if (session.kind === "running") {
    const failing = session
    leaveRunning(failing)
    session = { kind: "failed", error: record, lease: failing.lease }
  }
  self.postMessage({ type: "session-error", requestId, error: record })
}
