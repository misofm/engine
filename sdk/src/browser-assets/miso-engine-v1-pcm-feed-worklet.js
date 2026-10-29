/* Adapted verbatim from misofm/app 7485693e9bbcf2f65a91a4e5950e22d678d99062; see NOTICE. */
/**
 * The B3 shared-memory feed, worklet side — app-owned, not vendored.
 *
 * This module is added to the `AudioWorkletGlobalScope` *before* the engine's
 * own worklet module. It does two things and nothing else:
 *
 *  1. wraps the engine processor at the moment it registers itself, adding a
 *     `SharedArrayBuffer` drain at the top of `process()`;
 *  2. registers a second, tiny processor whose only job is to own a port the
 *     engine's strict message schema never sees, so the rings can be handed in
 *     without ever posting an unknown tag at the engine.
 *
 * ## Why a `registerProcessor` wrapper
 *
 * `public/wasm/engine-v1/miso-engine-v1-audio-worklet.js` is pinned engine
 * bytes and must not be edited, and it exports nothing — it declares a class
 * and calls `registerProcessor` at top level. A worklet module cannot import a
 * binding out of it. What it *can* do is share the global scope: every module
 * added to one `AudioWorklet` runs in the same `AudioWorkletGlobalScope`, so a
 * module that runs first can replace `registerProcessor` and receive the class
 * as an argument when the engine registers it. The engine's own bytes are
 * fetched, parsed and evaluated exactly as they are today; the only difference
 * is which function they hand their class to.
 *
 * Registration is total: whatever happens while wrapping, exactly one class is
 * registered under the engine's name, and if wrapping fails it is the engine's
 * own unmodified class. A broken prelude therefore degrades to the shipping
 * `postMessage` feed rather than to a context with no processor in it.
 *
 * ## Why the attach node, and why the rings arrive by port
 *
 * The engine processor's `receive()` turns any message it does not recognise
 * into a sticky `RESULT_INVALID_ARGUMENT` — a silent, permanent failure. So the
 * rings are not posted to it. They ride a separate node registered here, and
 * `new AudioWorkletNode(context, "miso-sab-feed-attach")` throwing is the main
 * thread's proof that this module is loaded and wrapping: the feature
 * detection is a constructor call, not a hopeful message. The caller
 * constructs it with no inputs and one output — Web Audio refuses a node with
 * neither — and never connects it, because construction alone is what builds
 * the processor that owns the port.
 *
 * They arrive on that node's **port**, and only on its port. Real Chromium
 * silently drops a `SharedArrayBuffer` passed through
 * `AudioWorkletNodeOptions.processorOptions`: the node constructs, the
 * processor is built, and the buffer is simply not in the options object it
 * receives. There is no error to catch on either side. A port message carries
 * it correctly, so there is exactly one route in and no second one to be
 * tempted by.
 *
 * ## What the drain does per block
 *
 * For each attached ring, once, at the top of `process()`, with no allocation
 * on the steady path and no `Atomics.wait` ever:
 *
 *   - apply a pending seek (a generation and a frame published through the
 *     ring, so a seek can never overtake or trail the planes around it); an
 *     anchored one (an anchor sample in the ring) is applied by the engine's
 *     message handler instead, and the drain reads no slot of that ring until
 *     the handler has applied it;
 *   - drain published chunks, dropping any whose generation tag is stale;
 *   - copy each chunk's planes into the engine's existing PCM staging buffer
 *     and call the existing `miso_engine_web_v1_source_submit` export — the
 *     same copy, the same call and the same arguments the `postMessage` path
 *     makes, which is why the two render bit-identically;
 *   - stop, leaving the chunk in the ring, when the engine answers
 *     `RESULT_BACKPRESSURE`;
 *   - maintain the engine ring's depth and count a block that found it empty
 *     as an underrun.
 *
 * ## `worker` mode (issue #1387)
 *
 * On a cross-origin-isolated page the engine runs its control plane in a
 * Worker and the worklet only renders. There the control Worker
 * (`miso-engine-v1-control-worker.js`) drains the rings by the same rules, on
 * its service tick, and the SDK's feed attaches through the engine host's
 * `openFeedPort()`. This prelude still loads and wraps the processor, but binds
 * no ring: it answers an attach with `attach-refused`, so `process()` never
 * calls a source export on the render-only instance.
 *
 * The engine never sees the shared buffer and never runs an atomic. Every
 * index, generation and counter in this file is JS, and its layout is pinned
 * against `src/lib/mixer/engine/sab-ring.ts` by `sab-ring-layout.test.ts`.
 */

const ENGINE_PROCESSOR_NAME = "miso-engine-v1-audio-worklet"
const ATTACH_PROCESSOR_NAME = "miso-sab-feed-attach"

const SAB_WRAP = 1 << 30
const SAB_WRAP_MASK = SAB_WRAP - 1
const SAB_MAGIC = 0x4d534231
const SAB_VERSION = 1

const CONTROL_MAGIC = 0
const CONTROL_VERSION = 1
const CONTROL_CAPACITY = 2
const CONTROL_CHANNELS = 3
const CONTROL_FRAME_CAPACITY = 4
const CONTROL_HEADER_OFFSET = 5
const CONTROL_PCM_OFFSET = 6
const CONTROL_ID_LENGTH = 7
const CONTROL_WRITE_INDEX = 8
const CONTROL_READ_INDEX = 9
const CONTROL_GENERATION_TAG = 10
const CONTROL_SEEK_EPOCH = 11
const CONTROL_WRITER_STATE = 12
const CONTROL_ATTACHED = 13
const CONTROL_WROTE = 14
const CONTROL_OVERFLOW = 15
const CONTROL_SUBMITTED = 16
const CONTROL_STALE = 17
const CONTROL_REFUSED = 18
const CONTROL_LAST_RESULT = 19
const CONTROL_UNDERRUNS = 20
const CONTROL_DRAIN_BLOCKS = 21
const CONTROL_SEEKS_APPLIED = 22
const CONTROL_DEPTH = 23
const CONTROL_TORN = 24
const CONTROL_FINISHED = 25
const CONTROL_ERRORS = 26
const CONTROL_SUBMITTED_GENERATION_TAG = 27

// Issue #1297b D2 (MSB1 V1, amended in place): three i64 at 112 -- the seek's generation, frame
// and anchor sample (at 128; -1 is a plain seek) -- the ID at 144 and the slot headers at 272. A
// ring with any other header offset (the old 256) is refused at attach. Issue #2184 (amended in
// place): SEEK_EPOCH is a parity seqlock over that tuple and GENERATION_TAG. The writer makes it
// odd, stores the tuple and the tag, and makes it even again, so one seek steps it by 2; a reader
// uses the tag and the tuple only when it read the same even epoch before and after them.
const CONTROL_BYTES = 144
const CONTROL_I64_OFFSET = 112
const CONTROL_I64_SEEK_GENERATION = 0
const CONTROL_I64_SEEK_FRAME = 1
const CONTROL_I64_SEEK_ANCHOR = 2
const NO_ANCHOR = -1n

const ID_OFFSET = 144
const HEADER_OFFSET = 272

const SLOT_HEADER_BYTES = 32
const SLOT_SEQUENCE = 0
const SLOT_GENERATION_TAG = 1
const SLOT_FRAMES = 2
const SLOT_FLAGS = 3
// Two and three: the i64 pair sits after the four i32 words, and both views
// address the same 32 bytes.
const SLOT_I64_GENERATION = 2
const SLOT_I64_START_FRAME = 3
// The same generation as two little-endian `i32` words: its low and its high half.
const SLOT_GENERATION_LOW = 4
const SLOT_GENERATION_HIGH = 5
const FLAG_END_OF_REGION = 1

// Issue #2185 (root ruling 2): the longest a live plain seek waits for its first new-generation
// chunk, in milliseconds of rendered audio; past it the seek applies as it did before #2185.
const SEEK_DEFERRAL_MS = 50

const RESULT_OK = 0
const RESULT_INVALID_ARGUMENT = 1
const RESULT_BACKPRESSURE = 6
const RESULT_WRONG_STATE = 3
// The core's code for a seek at a generation the source already has (issue
// #1644 D2), as the bytes a refusal writes at the start of the session
// diagnostic. Built once at load: the worklet scope has no TextEncoder.
const STALE_GENERATION_CODE = Uint8Array.from("source.generation.stale", (character) => character.charCodeAt(0))

/** The one engine processor in this scope, and any rings that arrived before
 *  it existed. Both nodes are constructed by the same main thread in a fixed
 *  order, but the queue costs one field and removes the ordering assumption. */
const registry = { engine: null, pending: null, pendingNode: null }

/** Issue #1387: `worker` mode binds no ring; the attach node that brought them is told so. */
function refuseIfWorkerMode(engine, node) {
  if (engine?.workerMode === true && node !== null) node.port.postMessage({ op: "attach-refused" })
}

function deliver(rings, node) {
  if (registry.engine === null) {
    registry.pending = rings
    registry.pendingNode = node
    return
  }
  registry.engine.attachSharedRings(rings)
  refuseIfWorkerMode(registry.engine, node)
}

function withdraw(rings) {
  if (registry.engine === null) {
    registry.pending = null
    registry.pendingNode = null
    return
  }
  registry.engine.detachSharedRings(rings)
}

/** Whether `bytes` is the UTF-8 encoding of `sourceId` (a lone surrogate as U+FFFD, as the
 *  engine's staging writes it). Control path only: the worklet scope has no TextEncoder. */
function sameSourceId(bytes, sourceId) {
  if (typeof sourceId !== "string") return false
  let offset = 0
  for (const character of sourceId) {
    let code = character.codePointAt(0)
    if (code >= 0xd800 && code <= 0xdfff) code = 0xfffd
    const width = code <= 0x7f ? 1 : code <= 0x7ff ? 2 : code <= 0xffff ? 3 : 4
    if (offset + width > bytes.length) return false
    const lead = width === 1 ? 0 : width === 2 ? 0xc0 : width === 3 ? 0xe0 : 0xf0
    if (bytes[offset] !== (lead | (code >>> (6 * (width - 1))))) return false
    for (let at = 1; at < width; at += 1) {
      if (bytes[offset + at] !== (0x80 | ((code >>> (6 * (width - 1 - at))) & 0x3f))) return false
    }
    offset += width
  }
  return offset === bytes.length
}

/** One bound ring: every view cut once, so the drain allocates nothing. */
function bindRing(shared, quantumFrames, maximumChannels) {
  if (
    typeof SharedArrayBuffer === "undefined" ||
    !(shared instanceof SharedArrayBuffer)
  ) {
    return null
  }
  if (shared.byteLength < HEADER_OFFSET) return null
  const control = new Int32Array(shared, 0, CONTROL_BYTES / 4)
  if (
    Atomics.load(control, CONTROL_MAGIC) !== SAB_MAGIC ||
    control[CONTROL_VERSION] !== SAB_VERSION
  ) {
    return null
  }
  const capacity = control[CONTROL_CAPACITY]
  const channels = control[CONTROL_CHANNELS]
  const frameCapacity = control[CONTROL_FRAME_CAPACITY]
  const headerOffset = control[CONTROL_HEADER_OFFSET]
  const pcmOffset = control[CONTROL_PCM_OFFSET]
  const idLength = control[CONTROL_ID_LENGTH]
  // A ring the engine could not accept a chunk from is worse than no ring: it
  // would refuse every block forever. Refuse it here instead, once.
  if (
    capacity <= 0 ||
    (capacity & (capacity - 1)) !== 0 ||
    channels <= 0 ||
    channels > maximumChannels ||
    frameCapacity !== quantumFrames ||
    idLength <= 0 ||
    headerOffset !== HEADER_OFFSET ||
    headerOffset + capacity * SLOT_HEADER_BYTES !== pcmOffset ||
    pcmOffset + capacity * channels * frameCapacity * 4 > shared.byteLength
  ) {
    return null
  }
  const planes = []
  for (let slot = 0; slot < capacity; slot += 1) {
    const slotPlanes = []
    for (let channel = 0; channel < channels; channel += 1) {
      slotPlanes.push(
        new Float32Array(
          shared,
          pcmOffset + (slot * channels + channel) * frameCapacity * 4,
          frameCapacity
        )
      )
    }
    planes.push(slotPlanes)
  }
  return {
    shared,
    control,
    controlI64: new BigInt64Array(shared, CONTROL_I64_OFFSET, 3),
    headers: new Int32Array(
      shared,
      headerOffset,
      (capacity * SLOT_HEADER_BYTES) / 4
    ),
    headersI64: new BigInt64Array(
      shared,
      headerOffset,
      (capacity * SLOT_HEADER_BYTES) / 8
    ),
    idBytes: new Uint8Array(shared, ID_OFFSET, idLength),
    idLength,
    planes,
    capacity,
    channels,
    frameCapacity,
    /** The seek epoch this ring has already applied. */
    seenEpoch: Atomics.load(control, CONTROL_SEEK_EPOCH),
    /** Issue #2184: the last whole seek publication `readSharedSeek` read -- its even epoch, the
     *  tag, the tuple, the seek generation's two unsigned 32-bit halves and the write index read
     *  inside the same window -- kept here so the drain allocates nothing. */
    seekEpoch: 0,
    seekTag: 0,
    publishedGeneration: 0n,
    publishedGenerationHigh: 0,
    publishedGenerationLow: 0,
    seekFrame: 0n,
    seekAnchor: NO_ANCHOR,
    seekWrite: 0,
    /** Issue #2185: the blocks rendered since the ring's live plain seek was first deferred, or 0. */
    deferredBlocks: 0,
    /** Issue #1297b: the anchored seek `(generation, frame, anchor)` the engine's message handler
     *  applied for this ring's source (`source_seek_at` answered OK), not yet consumed by the
     *  drain; `anchoredSample` -1 is none. Written only by the handler, between blocks. */
    anchoredGeneration: 0n,
    anchoredFrame: 0n,
    anchoredSample: NO_ANCHOR,
    /** Chunks the engine holds for this source but has not yet rendered. */
    depth: 0,
    finished: false,
    /** The staging-buffer id view, cut against the Wasm memory it was cut
     *  from; a memory that grew invalidates it and the drain re-cuts it. */
    idTarget: null,
    idTargetBuffer: null,
  }
}

/**
 * Add the drain to the engine processor.
 *
 * Only three methods exist here, and none of them changes anything the engine
 * does with a chunk: `process` runs the drain and then calls the engine's own
 * `process`, unmodified, for the same block.
 */
function wrapEngineProcessor(Base) {
  return class MisoSabFeedProcessor extends Base {
    constructor(options) {
      super(options)
      this.sabRings = []
      // The `miso.seek-at.v1` message the engine's handler is answering, or null (#1297b).
      this.pendingSeekAt = null
      registry.engine = this
      if (registry.pending !== null) {
        const pending = registry.pending
        const node = registry.pendingNode
        registry.pending = null
        registry.pendingNode = null
        this.attachSharedRings(pending)
        refuseIfWorkerMode(this, node)
      }
    }

    /**
     * Bind a set of rings. Called off the audio path, from an attach node's
     * construction.
     *
     * Additive and idempotent: a buffer already bound is rebound rather than
     * bound twice, and rings from an earlier attach are left alone. One
     * session per context is the only shape this app builds, but "the second
     * session silently unbinds the first" is not a failure worth leaving
     * available for the cost of a filter.
     */
    attachSharedRings(rings) {
      // Issue #1387 (S11): in `worker` mode the control Worker drains the rings and this instance
      // only renders: it binds none, so `process()` drains nothing and calls no source export.
      // The attach node tells the feed (`attach-refused`); the SDK's feed attaches through the
      // engine's host instead.
      if (!Array.isArray(rings) || this.workerMode === true) return
      this.rebindMemoryViews()
      // Issue #2185: the deferral bound in blocks, computed here, off the render path.
      const rate = this.sampleRateHz ?? globalThis.sampleRate
      const blocks = Math.ceil((SEEK_DEFERRAL_MS * rate) / (1000 * this.quantumFrames))
      // A scope with no rate defers nothing: every seek applies as before #2185.
      this.seekDeferralBlocks = Number.isSafeInteger(blocks) && blocks > 0 ? blocks : 0
      const bound = []
      const incoming = []
      for (let index = 0; index < rings.length; index += 1) {
        const ring = bindRing(
          rings[index],
          this.quantumFrames,
          this.maximumSourceChannels
        )
        if (ring === null) continue
        incoming.push(ring.shared)
        bound.push(ring)
      }
      const kept = this.sabRings.filter(
        (ring) => incoming.indexOf(ring.shared) < 0
      )
      for (let index = 0; index < bound.length; index += 1) {
        // Cut the first-drain id view here, on the attach/control path. The
        // steady process path must never allocate a typed-array view.
        bound[index].idTarget = new Uint8Array(
          this.memoryBuffer,
          this.sourceIdPointer,
          Math.min(bound[index].idLength, this.sourceIdCapacity)
        )
        bound[index].idTargetBuffer = this.memoryBuffer
        Atomics.store(bound[index].control, CONTROL_ATTACHED, 1)
      }
      this.sabRings = kept.concat(bound)
    }

    /**
     * The engine processor re-cuts its own views when a control call grows the
     * module's memory; the rings hold one more view of that memory, the
     * staging-buffer id view each ring cut at attach, so it is re-cut here, in
     * the same message handler and never in `process()`. The engine's
     * `rebindMemoryViews` is the single entry both its handlers and this
     * class's between-block handler call.
     */
    rebindMemoryViews() {
      const before = this.memoryBuffer
      super.rebindMemoryViews()
      if (this.memoryBuffer === before) return
      const rings = this.sabRings ?? []
      for (let index = 0; index < rings.length; index += 1) {
        const ring = rings[index]
        ring.idTarget = new Uint8Array(
          this.memoryBuffer,
          this.sourceIdPointer,
          Math.min(ring.idLength, this.sourceIdCapacity)
        )
        ring.idTargetBuffer = this.memoryBuffer
      }
    }

    /** Unbind exactly the rings an attach node brought, and nothing else. */
    detachSharedRings(rings) {
      if (!Array.isArray(rings)) return
      this.sabRings = this.sabRings.filter((ring) => {
        if (rings.indexOf(ring.shared) < 0) return true
        Atomics.store(ring.control, CONTROL_ATTACHED, 0)
        return false
      })
    }

    process(inputs, outputs) {
      const rings = this.sabRings
      if (
        rings !== undefined &&
        rings.length !== 0 &&
        this.ready === true &&
        this.disposed !== true &&
        this.stickyResult === RESULT_OK &&
        this.exports.memory.buffer === this.memoryBuffer
      ) {
        for (let index = 0; index < rings.length; index += 1) {
          const ring = rings[index]
          try {
            this.drainSharedRing(ring)
          } catch (_) {
            // A feed that throws must not take the render with it: the block
            // still renders from whatever the engine already holds, and the
            // counter says the feed is the reason it may run dry.
            ring.control[CONTROL_ERRORS] += 1
          }
        }
      }
      return super.process(inputs, outputs)
    }

    /** One ring, one block. Reads are `Atomics.load` on the writer's words and
     *  plain loads on its own; the single `Atomics.store` of the read index at
     *  the end is what releases the slots back to the writer. */
    drainSharedRing(ring) {
      const control = ring.control
      // Memory growth after attach invalidates the pre-cut view. Refuse this
      // drain rather than allocating a replacement inside process().
      if (ring.idTargetBuffer !== this.memoryBuffer) return
      if (ring.idTarget.length !== ring.idLength) return

      // Issue #2184: a pass reads the seek publication whole or not at all. A publication in
      // flight skips the pass, as a backpressured seek does; the next block reads it whole.
      if (!this.readSharedSeek(ring)) return
      if (this.deferSharedSeek(ring)) {
        // The old generation keeps playing from the engine's ring meanwhile.
        this.countSharedBlock(ring)
        return
      }
      if (this.applySharedSeek(ring) !== RESULT_OK) return

      const write = ring.seekWrite
      const capacityMask = ring.capacity - 1
      const staging = this.sourcePcm
      const quantumFrames = this.quantumFrames
      let read = control[CONTROL_READ_INDEX]
      let accepted = 0
      let remaining = ring.capacity

      // Fill internal queues gradually. Complete shared runways must not turn
      // one startup/seek callback into thousands of source submissions.
      while (read !== write && accepted < 2 && remaining-- > 0) {
        const slot = read & capacityMask
        const word = slot * (SLOT_HEADER_BYTES / 4)
        if (ring.headers[word + SLOT_SEQUENCE] !== read) {
          // The slot the index points at is not the chunk the index names.
          // Only a second writer can do that; stop rather than submit it.
          control[CONTROL_TORN] += 1
          break
        }
        const age = this.sharedSlotAge(ring, word)
        if (age < 0) {
          read = (read + 1) & SAB_WRAP_MASK
          control[CONTROL_STALE] += 1
          continue
        }
        // A newer seek's chunk: never freed or submitted here; the next pass reads its seek.
        if (age > 0) break
        const frames = ring.headers[word + SLOT_FRAMES]
        const flags = ring.headers[word + SLOT_FLAGS]
        if (frames <= 0 || frames > ring.frameCapacity) {
          read = (read + 1) & SAB_WRAP_MASK
          control[CONTROL_ERRORS] += 1
          continue
        }
        const slotPlanes = ring.planes[slot]
        for (let channel = 0; channel < ring.channels; channel += 1) {
          const plane = slotPlanes[channel]
          // A full quantum is the steady case and copies the view as it is;
          // only a region's last chunk is short, and only it cuts a subview.
          // Every slot plane is pre-zeroed by the writer. Copy its full fixed
          // quantum even for the legal tail, avoiding a tail subview allocation.
          staging.set(plane, channel * quantumFrames)
        }
        ring.idTarget.set(ring.idBytes)
        const word64 = slot * (SLOT_HEADER_BYTES / 8)
        const result = this.exports.miso_engine_web_v1_source_submit(
          this.handle,
          ring.idLength,
          ring.headersI64[word64 + SLOT_I64_GENERATION],
          ring.headersI64[word64 + SLOT_I64_START_FRAME],
          ring.channels,
          frames,
          (flags & FLAG_END_OF_REGION) !== 0 ? 1 : 0
        )
        if (result === RESULT_BACKPRESSURE) {
          // The engine's ring is full. Leave the chunk where it is — this is
          // the backpressure that keeps the feeder from running ahead.
          break
        }
        read = (read + 1) & SAB_WRAP_MASK
        if (result === RESULT_OK) {
          // Publish the generation only after the engine accepted its PCM.
          // The main-thread start gate compares this with GENERATION_TAG, so
          // lifetime-cumulative SUBMITTED can never validate a later seek.
          Atomics.store(
            control,
            CONTROL_SUBMITTED_GENERATION_TAG,
            ring.headers[word + SLOT_GENERATION_TAG]
          )
          control[CONTROL_SUBMITTED] += 1
          accepted += 1
          ring.depth += 1
          if ((flags & FLAG_END_OF_REGION) !== 0) {
            ring.finished = true
            control[CONTROL_FINISHED] = 1
          }
        } else {
          control[CONTROL_REFUSED] += 1
          control[CONTROL_LAST_RESULT] = result
        }
      }

      Atomics.store(control, CONTROL_READ_INDEX, read)
      this.countSharedBlock(ring)
    }

    /** The block about to render, counted for one ring. */
    countSharedBlock(ring) {
      const control = ring.control
      control[CONTROL_DRAIN_BLOCKS] += 1
      // The block about to be rendered consumes one quantum of this source.
      // Reaching zero with a live feeder and audio still to come is exactly
      // what an underrun is, and it is counted here because it is the only
      // place that knows it.
      if (ring.depth > 0) {
        ring.depth -= 1
      } else if (
        !ring.finished &&
        Atomics.load(control, CONTROL_WRITER_STATE) === 1
      ) {
        control[CONTROL_UNDERRUNS] += 1
      }
      control[CONTROL_DEPTH] = ring.depth
    }

    /**
     * Issue #2184: read the ring's seek publication through its seqlock into the ring's fields.
     * False when the epoch is odd (a publication in flight) or moved during the reads: then the
     * tag and the tuple may belong to two publications, and nothing of them is used. JS atomics
     * are sequentially consistent, so an even epoch read unchanged around the reads proves that
     * every store of the last complete publication is visible and none of a later one is.
     */
    readSharedSeek(ring) {
      const control = ring.control
      const epoch = Atomics.load(control, CONTROL_SEEK_EPOCH)
      if ((epoch & 1) !== 0) return false
      // #2185 attempt 2: the write index is read inside the window too, so every slot below it
      // was published under this snapshot's seek or an earlier one, never a later one.
      const tag = Atomics.load(control, CONTROL_GENERATION_TAG)
      const write = Atomics.load(control, CONTROL_WRITE_INDEX)
      if (epoch === ring.seekEpoch) {
        // The cached tuple is this epoch's: no `i64` load, so an idle block creates no BigInt.
        if (Atomics.load(control, CONTROL_SEEK_EPOCH) !== epoch) return false
        ring.seekTag = tag
        ring.seekWrite = write
        return true
      }
      const generation = Atomics.load(ring.controlI64, CONTROL_I64_SEEK_GENERATION)
      const frame = Atomics.load(ring.controlI64, CONTROL_I64_SEEK_FRAME)
      const anchor = Atomics.load(ring.controlI64, CONTROL_I64_SEEK_ANCHOR)
      if (Atomics.load(control, CONTROL_SEEK_EPOCH) !== epoch) return false
      ring.seekEpoch = epoch
      ring.seekTag = tag
      ring.seekWrite = write
      ring.publishedGeneration = generation
      ring.publishedGenerationHigh = Number(BigInt.asUintN(32, generation >> 32n))
      ring.publishedGenerationLow = Number(BigInt.asUintN(32, generation))
      ring.seekFrame = frame
      ring.seekAnchor = anchor
      return true
    }

    /**
     * #2185 attempt 2: a slot's generation against the snapshot's seek, as `releaseStaleSharedSlots`
     * states it: below it (-1) the slot is stale and freed; equal (0) it is current; above it (1)
     * it belongs to a newer seek and is neither freed nor submitted. The u64 is compared as its
     * two unsigned 32-bit halves, read as `i32` words, so no BigInt is made. Before any seek was
     * published (epoch 0) the writer's engaged tag is the only reference, as before.
     */
    sharedSlotAge(ring, word) {
      if (ring.seekEpoch === 0) return ring.headers[word + SLOT_GENERATION_TAG] === ring.seekTag ? 0 : -1
      const high = ring.headers[word + SLOT_GENERATION_HIGH] >>> 0
      if (high !== ring.publishedGenerationHigh) return high < ring.publishedGenerationHigh ? -1 : 1
      const low = ring.headers[word + SLOT_GENERATION_LOW] >>> 0
      if (low !== ring.publishedGenerationLow) return low < ring.publishedGenerationLow ? -1 : 1
      return 0
    }

    /**
     * Issue #2185: whether this block defers the ring's published live plain seek. The writer
     * seeks while the shared ring is full of the old generation, and it can publish the new one
     * only into slots the drain frees; applied at once, the seek makes render drop the old chunks
     * the engine holds before any new chunk exists, which is a gap. So, until the seek's
     * generation has a runway published -- `seekDeferralBlocks` chunks (at most a ring), or a
     * chunk that ends its region -- the drain frees every slot of an older generation (counted as
     * STALE), asks no seek and submits nothing, and the old generation plays on from the engine's
     * ring. The first pass that finds the runway applies the seek, then drains from its first
     * chunk; a slot of a newer generation stops the scan. A seek applied at its first chunk alone
     * left a gap whenever the writer's next chunks came a writer tick later (attempt 2). A newer
     * seek published meanwhile replaces the deferred one (the skipped generation never reaches the
     * engine) and inherits its count. A writer that publishes nothing for `seekDeferralBlocks`
     * (50 ms of rendered audio) gets the seek applied as before #2185. An anchored seek is never
     * deferred: the engine holds its old generation until the anchor. Root decision (#2185): up to
     * 50 ms of old-position audio is the smaller artefact than a silence insert, which is two cuts
     * and a hole; the deferral is one cut, at most 50 ms late. The prelude does not know the
     * engine ring's size, so this bound is not capped at half of it as the Worker's is.
     */
    deferSharedSeek(ring) {
      if (ring.seekEpoch === ring.seenEpoch || ring.seekAnchor !== NO_ANCHOR) return false
      if (ring.deferredBlocks >= this.seekDeferralBlocks) return false
      const control = ring.control
      const write = ring.seekWrite
      // #2185 attempt 2: the runway the new generation must have published before the seek
      // applies -- as much audio as the bound would have played of the old one, at most a ring.
      const runway = Math.min(this.seekDeferralBlocks, ring.capacity)
      let read = control[CONTROL_READ_INDEX]
      let current = 0
      let ended = false
      for (let index = read, remaining = ring.capacity; index !== write && remaining > 0; remaining -= 1) {
        const word = (index & (ring.capacity - 1)) * (SLOT_HEADER_BYTES / 4)
        if (ring.headers[word + SLOT_SEQUENCE] !== index) {
          control[CONTROL_TORN] += 1
          break
        }
        const age = this.sharedSlotAge(ring, word)
        // A newer seek's chunk stays; the next pass defers or applies that seek instead.
        if (age > 0) break
        if (age < 0) {
          // Only the older chunks ahead of the seek's first one are freed.
          if (current !== 0) break
          index = (index + 1) & SAB_WRAP_MASK
          read = index
          control[CONTROL_STALE] += 1
          continue
        }
        current += 1
        if ((ring.headers[word + SLOT_FLAGS] & FLAG_END_OF_REGION) !== 0) {
          ended = true
          break
        }
        if (current >= runway) break
        index = (index + 1) & SAB_WRAP_MASK
      }
      Atomics.store(control, CONTROL_READ_INDEX, read)
      if (current >= runway || ended) return false
      ring.deferredBlocks += 1
      return true
    }

    /** Shared by render and the between-block control handler; never consumes PCM. The defaults
     *  are the publication `readSharedSeek` read last; the caller reads it first. */
    applySharedSeek(ring, epoch = ring.seekEpoch, generation = ring.publishedGeneration, frame = ring.seekFrame, anchor = ring.seekAnchor) {
      const control = ring.control
      if (epoch !== ring.seenEpoch) {
        const requested = generation
        const requestedFrame = frame
        if (anchor !== NO_ANCHOR) {
          // Issue #1297 amendment (review M4): an anchored seek is applied by the engine's message
          // handler (`source_seek_at` runs the service step), never here. Until the handler
          // recorded an OK for exactly this tuple, the epoch stays unseen and no slot of this ring
          // is read: its chunks at the new generation wait for the seek they belong to. After it,
          // the epoch is consumed without any export call. The #1644 stale-generation rule below
          // is the plain seek's alone: an anchored epoch never reaches an export, so a refusal can
          // never consume it, and a refused `seek-at` leaves it unseen until a newer ring seek
          // supersedes it (the writer's reply names the refusal).
          if (
            ring.anchoredSample !== anchor ||
            ring.anchoredGeneration !== requested ||
            ring.anchoredFrame !== requestedFrame
          ) {
            return RESULT_BACKPRESSURE
          }
          ring.anchoredSample = NO_ANCHOR
          return this.consumeSharedSeek(ring, epoch)
        }
        ring.idTarget.set(ring.idBytes)
        let result = this.exports.miso_engine_web_v1_source_seek(
          this.handle,
          ring.idLength,
          requested,
          requestedFrame
        )
        // Issue #1644 D2 (R4 amended): a session seek the engine accepted at this generation
        // already moved this source, so its own seek at the same generation is refused as stale,
        // and the ring would stall for good. That one refusal is the seek applied. The source is
        // always asked first: one that the session seek did not move (a source a later plan
        // added) accepts the seek, and every other refusal is handled as before. A per-source
        // seek at g+1 that reaches the engine by another path (a `miso.seek.v1` message) between
        // a session seek at g and this ring's epoch at g also makes the epoch's seek stale, and
        // the epoch is consumed too. The source is then at g+1: the ring's chunks at g are
        // refused as stale and counted in REFUSED, and its PCM at g+1 is the job of whoever
        // sought g+1. Without the rule this ring would stall on the seek instead. A ring seek
        // at g+1 itself supersedes the epoch at g (one seek slot) and is asked as usual. The
        // words compare as unsigned 64-bit, as `releaseStaleSharedSlots` reads them.
        if (
          result === RESULT_INVALID_ARGUMENT &&
          BigInt.asUintN(64, requested) === this.lastSessionSeekGeneration &&
          this.refusedAsStaleGeneration()
        ) {
          result = RESULT_OK
        }
        if (result === RESULT_BACKPRESSURE) {
          // Ordinary flow control. Leave the epoch unseen and retry the same
          // seek before touching any slots on the next process call.
          return result
        }
        if (result !== RESULT_OK) {
          control[CONTROL_REFUSED] += 1
          control[CONTROL_LAST_RESULT] = result
          return result
        }
        return this.consumeSharedSeek(ring, epoch)
      }
      return RESULT_OK
    }

    /** Mark a ring's seek epoch applied: the engine dropped everything it held for this source. */
    consumeSharedSeek(ring, epoch) {
      const control = ring.control
      ring.seenEpoch = epoch
      ring.deferredBlocks = 0
      control[CONTROL_SEEKS_APPLIED] += 1
      ring.depth = 0
      ring.finished = false
      control[CONTROL_FINISHED] = 0
      control[CONTROL_DEPTH] = 0
      return RESULT_OK
    }

    /**
     * Issue #1297b: the engine handler's anchored source seek (#1297a), run between blocks. The
     * engine's own handler validates the message and calls `source_seek_at` once; this override
     * only watches its reply, and on an OK records the tuple on the ring of that source, which is
     * what lets the drain consume the ring's anchored epoch without an export call.
     */
    receiveSeekAt(message) {
      this.pendingSeekAt = message
      try {
        super.receiveSeekAt(message)
      } finally {
        this.pendingSeekAt = null
      }
    }

    acknowledge(requestId, result, planes) {
      const seekAt = this.pendingSeekAt
      if (seekAt != null && seekAt.requestId === requestId && result === RESULT_OK) {
        const rings = this.sabRings ?? []
        for (let index = 0; index < rings.length; index += 1) {
          const ring = rings[index]
          if (!sameSourceId(ring.idBytes, seekAt.sourceId)) continue
          // The ring holds the words as signed i64; the message's are u64 bigints.
          ring.anchoredGeneration = BigInt.asIntN(64, seekAt.generation)
          ring.anchoredFrame = BigInt.asIntN(64, seekAt.sourceFrame)
          ring.anchoredSample = BigInt.asIntN(64, seekAt.anchorSample)
        }
      }
      super.acknowledge(requestId, result, planes)
    }

    /** Whether the last refusal wrote `source.generation.stale` into the session diagnostic.
     *  A byte compare over the engine's fixed view: no allocation. */
    refusedAsStaleGeneration() {
      const view = this.diagnosticView
      if (view == null || view.buffer !== this.memoryBuffer || view.length < STALE_GENERATION_CODE.length) {
        return false
      }
      for (let index = 0; index < STALE_GENERATION_CODE.length; index += 1) {
        if (view[index] !== STALE_GENERATION_CODE[index]) return false
      }
      return true
    }

    /** Consumer-owned stale release, called only by the attachment port between blocks. */
    prepareSharedSeeks(delivered, seeks) {
      // A growth of the module's memory is no fault: re-cut the views here, between blocks.
      this.rebindMemoryViews()
      if (!this.ready || this.disposed || this.stickyResult !== RESULT_OK || !Array.isArray(seeks) || seeks.length !== delivered.length) return { kind: "refused", result: RESULT_WRONG_STATE }
      const rings = delivered.map((shared) => this.sabRings.find((ring) => ring.shared === shared))
      if (rings.some((ring) => !ring || ring.idTargetBuffer !== this.memoryBuffer || ring.idTarget.length !== ring.idLength)) return { kind: "refused", result: RESULT_WRONG_STATE }
      // #2184: the live publication is read whole (`readSharedSeek`), so its anchor, which the
      // apply reads from the ring, is the matched seek's own.
      const matches = (ring, seek) => seek && seek.epoch !== 0 && this.readSharedSeek(ring) && ring.seekEpoch === seek.epoch && ring.publishedGeneration === seek.generation && ring.seekFrame === seek.frame
      if (!rings.every((ring, index) => matches(ring, seeks[index]))) return { kind: "superseded" }
      for (let index = 0; index < rings.length; index += 1) {
        const ring = rings[index]
        if (!matches(ring, seeks[index])) return { kind: "superseded" }
        const result = this.applySharedSeek(ring, seeks[index].epoch, seeks[index].generation, seeks[index].frame)
        if (result !== RESULT_OK) return { kind: "refused", result }
        if (!this.releaseStaleSharedSlots(ring, seeks[index].generation)) return { kind: "refused", result: 1 }
      }
      return rings.every((ring, index) => matches(ring, seeks[index])) ? { kind: "confirmed", seeks } : { kind: "superseded" }
    }

    releaseStaleSharedSlots(ring, generation) {
      const control = ring.control
      const write = Atomics.load(control, CONTROL_WRITE_INDEX)
      const capacityMask = ring.capacity - 1
      let read = control[CONTROL_READ_INDEX]
      for (let remaining = ring.capacity; read !== write && remaining > 0; remaining -= 1) {
        const slot = read & capacityMask
        const word = slot * (SLOT_HEADER_BYTES / 4)
        if (ring.headers[word + SLOT_SEQUENCE] !== read) {
          // The slot the index points at is not the chunk the index names.
          // Only a second writer can do that; stop rather than submit it.
          control[CONTROL_TORN] += 1
          Atomics.store(control, CONTROL_READ_INDEX, read)
          return false
        }
        const word64 = slot * (SLOT_HEADER_BYTES / 8)
        // A producer may supersede this request while its control message runs.
        // Future-generation PCM belongs to that seek and must never be discarded.
        if (BigInt.asUintN(64, ring.headersI64[word64 + SLOT_I64_GENERATION]) < BigInt.asUintN(64, generation)) {
          read = (read + 1) & SAB_WRAP_MASK
          control[CONTROL_STALE] += 1
          continue
        }
        // Retain current-generation work without submitting or consuming it.
        break
      }
      Atomics.store(control, CONTROL_READ_INDEX, read)
      return true
    }
  }
}

/** The rings' way in. Owns a port the engine's schema never reads, so nothing
 *  here can put the engine into a sticky state. */
class MisoSabFeedAttachProcessor extends AudioWorkletProcessor {
  constructor() {
    super()
    // What this node brought, so a detach can withdraw exactly that and not
    // another session's rings. It starts empty: nothing is delivered at
    // construction, because `processorOptions` cannot carry a ring.
    this.delivered = []
    this.port.onmessage = (event) => {
      const data = event.data
      if (data === null || typeof data !== "object") return
      if (data.op === "attach" && Array.isArray(data.rings)) {
        this.delivered = data.rings
        deliver(data.rings, this)
      } else if (data.op === "detach") {
        withdraw(this.delivered)
        this.delivered = []
      } else if (data.op === "detach-source") {
        // Issue #1297b D5: withdraw one ring this node brought, named by its index in the attach
        // list. A port clones every message, so a SharedArrayBuffer it carries is a new object
        // each time and can never name a ring this node holds; the index survives the clone. The
        // reply is posted after the engine stopped reading the ring (this handler runs between
        // blocks, on the drain's thread) and says whether a ring was withdrawn.
        const index = data.index
        const ring = Number.isSafeInteger(index) && index >= 0 && index < this.delivered.length
          ? this.delivered[index]
          : null
        if (ring != null) {
          if (registry.engine === null) {
            if (registry.pending !== null) registry.pending = registry.pending.filter((entry) => entry !== ring)
          } else {
            registry.engine.detachSharedRings([ring])
          }
          this.delivered = this.delivered.map((entry, at) => (at === index ? null : entry))
        }
        this.port.postMessage({ op: "source-detached", requestId: data.requestId, detached: ring != null })
      } else if (data.op === "prepare-seek") {
        const result = registry.engine === null ? { kind: "refused", result: RESULT_WRONG_STATE } : registry.engine.prepareSharedSeeks(this.delivered.filter((ring) => ring !== null), data.seeks)
        registry.engine?.rebindMemoryViews()
        this.port.postMessage({ op: "seek-prepared", requestId: data.requestId, ...result })
      }
    }
  }

  process() {
    return true
  }
}

const originalRegister = globalThis.registerProcessor
if (typeof originalRegister !== "function") {
  throw new TypeError("miso-sab-feed: no registerProcessor in this scope")
}

function interceptingRegister(name, constructor) {
  if (name !== ENGINE_PROCESSOR_NAME) {
    originalRegister.call(globalThis, name, constructor)
    return
  }
  globalThis.registerProcessor = originalRegister
  let Registered = constructor
  try {
    Registered = wrapEngineProcessor(constructor)
  } catch (_) {
    // Wrapping failed. The engine's own class is registered unchanged and the
    // attach node is simply never told about any rings, which is the shipping
    // `postMessage` feed — not a context with nothing registered in it.
    Registered = constructor
  }
  originalRegister.call(globalThis, name, Registered)
}

globalThis.registerProcessor = interceptingRegister
// A scope that will not take the patch must fail here, while the main thread
// is still awaiting `addModule` and can still choose the message path — never
// later, when the engine has already registered itself unwrapped.
if (globalThis.registerProcessor !== interceptingRegister) {
  throw new TypeError("miso-sab-feed: registerProcessor is not patchable")
}

originalRegister.call(
  globalThis,
  ATTACH_PROCESSOR_NAME,
  MisoSabFeedAttachProcessor
)
