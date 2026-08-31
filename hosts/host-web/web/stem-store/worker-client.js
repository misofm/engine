/**
 * Construct the required dedicated PCM pump Worker. Ring descriptors are sent
 * over this port after construction; they never enter AudioWorklet options.
 */
export function createStemPumpWorker(options = {}) {
  const WorkerConstructor = options.WorkerConstructor ?? globalThis.Worker
  if (typeof WorkerConstructor !== "function") {
    throw new TypeError("A dedicated Worker implementation is required")
  }
  return new WorkerConstructor(
    options.url ?? new URL("./pcm-pump-worker.js", import.meta.url),
    { type: "module", name: "miso-stem-pump-v1" }
  )
}
