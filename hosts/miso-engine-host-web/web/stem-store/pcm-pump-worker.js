import { OpfsStemStore } from "./opfs-store.js"
import { CanonicalPcmPump, Msb1RingWriter } from "./pcm-pump.js"

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

let pump
let lease
let messageTail = Promise.resolve()

// Self-driving state (#278). All three are undefined/false unless `initialize`
// asked for the cadence, so a host that does not opt in runs the exact
// pull-driven worker it ran before.
let selfDrivingIdleMs
let driveToken
let idleWake

self.onmessage = (event) => {
  messageTail = messageTail.then(() => handleMessage(event)).catch((error) => {
    reportSessionError(error, event.data?.requestId)
  })
}

async function handleMessage(event) {
  const message = event.data
  if (message?.type === "initialize") {
    stopDriving()
    pump?.stop()
    await lease?.close()
    const store = new OpfsStemStore({ folderName: message.folderName })
    await store.open()
    // The main-realm loading gate owns verification and pins. This narrow
    // lease facade limits the Worker to getFile()/slice reads only.
    lease = {
      read: (identity) => store.read(identity),
      close: async () => {},
    }
    pump = new CanonicalPcmPump({
      lease,
      windowFrames: message.windowFrames,
      generation: BigInt(message.generation ?? 1),
      sources: message.sources.map((source) => ({
        ...source,
        ring: new Msb1RingWriter(source.ring),
      })),
    })
    selfDrivingIdleMs = requestedIdleMs(message.selfDriving)
    self.postMessage({ type: "initialized", requestId: message.requestId })
    startDriving()
    return
  }
  if (message?.type === "pump") {
    if (pump === undefined) throw new Error("PCM pump is not initialized")
    self.postMessage({
      type: "pumped",
      requestId: message.requestId,
      ...(await pump.pumpUntilFull()),
    })
    return
  }
  if (message?.type === "seek") {
    if (pump === undefined) throw new Error("PCM pump is not initialized")
    await pump.seek(message.frame)
    self.postMessage({ type: "sought", requestId: message.requestId })
    // A seek moves the cursor off end-of-region, so a loop that had exited on
    // `finished` has work again. Waking the idle timer is the other half: a
    // seek backwards through a full ring is exactly the case where the loop is
    // asleep and the rings it filled are now the wrong generation.
    wakeIdle()
    startDriving()
    return
  }
  if (message?.type === "stop") {
    stopDriving()
    pump?.stop()
    pump = undefined
    await lease?.close()
    lease = undefined
    self.postMessage({ type: "stopped", requestId: message.requestId })
  }
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
 * It clears the token instead; the tick already queued (if any) runs to
 * completion against the pump it started with, and the one after it sees a
 * cleared token and exits.
 *
 * # Why it posts nothing
 *
 * A self-driven pass has no `requestId` to answer, and posting bare `pumped`
 * notifications would turn a reply into a broadcast -- the same word meaning
 * two things depending on who started the pass. The ring's own `endOfRegion`
 * flag already tells the reader where the region ends, so the loop stays
 * silent on the happy path. A failure still reports on `session-error`,
 * exactly as an externally driven `pump` would.
 */
function startDriving() {
  if (selfDrivingIdleMs === undefined || driveToken !== undefined) return
  if (pump === undefined) return
  const token = {}
  driveToken = token
  void drive(token).finally(() => {
    if (driveToken === token) driveToken = undefined
  })
}

function stopDriving() {
  selfDrivingIdleMs = undefined
  driveToken = undefined
  wakeIdle()
}

async function drive(token) {
  while (driveToken === token) {
    let outcome
    try {
      outcome = await enqueue(() =>
        driveToken === token ? pump?.pumpUntilFull() : undefined
      )
    } catch (error) {
      // `pumpUntilFull` already hard-stopped the writer on its way out.
      driveToken = undefined
      reportSessionError(error)
      return
    }
    if (driveToken !== token || outcome === undefined) return
    if (outcome.finished) {
      // Clear the token here rather than leaving it to the `finally`, which is a
      // microtask: a `seek` arriving before that microtask ran would see a token
      // that still looks live, decline to re-arm, and stall the session.
      driveToken = undefined
      return
    }
    if (outcome.chunks === 0) await sleep(selfDrivingIdleMs ?? DEFAULT_IDLE_MS)
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

function sleep(ms) {
  return new Promise((resolve) => {
    const finish = () => {
      clearTimeout(timer)
      if (idleWake === finish) idleWake = undefined
      resolve()
    }
    const timer = setTimeout(finish, ms)
    idleWake = finish
  })
}

function wakeIdle() {
  idleWake?.()
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

function reportSessionError(error, requestId) {
  pump?.stop()
  self.postMessage({
    type: "session-error",
    requestId,
    error: {
      name: error?.name ?? "Error",
      code: error?.code ?? "stem.pump.failed",
      message: error?.message ?? String(error),
      details: error?.details ?? {},
    },
  })
}
