import { OpfsStemStore } from "./opfs-store.js"
import { CanonicalPcmPump, Msb1RingWriter } from "./pcm-pump.js"

let pump
let lease
let messageTail = Promise.resolve()

self.onmessage = (event) => {
  messageTail = messageTail.then(() => handleMessage(event)).catch((error) => {
    reportSessionError(error, event.data?.requestId)
  })
}

async function handleMessage(event) {
  const message = event.data
  if (message?.type === "initialize") {
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
    self.postMessage({ type: "initialized", requestId: message.requestId })
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
    return
  }
  if (message?.type === "stop") {
    pump?.stop()
    pump = undefined
    await lease?.close()
    lease = undefined
    self.postMessage({ type: "stopped", requestId: message.requestId })
  }
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
