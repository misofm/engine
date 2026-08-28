import assert from "node:assert/strict"
import { createHash } from "node:crypto"
import { performance } from "node:perf_hooks"
import {
  OpfsStemStore,
  StemStoreError,
  detectSharedReadOnlyMode,
} from "../web/stem-store/opfs-store.js"
import { MemoryStemResolver } from "../web/stem-store/resolver.js"
import { FakeLockManager, FakeOpfsBackend } from "./stem-store-fakes.mjs"

const encoder = new TextEncoder()

function fixture(label, bytes = 4096) {
  const seed = encoder.encode(label)
  const pcm = new Uint8Array(bytes)
  for (let index = 0; index < pcm.length; index += 1) {
    pcm[index] = seed[index % seed.length] ^ ((index * 29) & 0xff)
  }
  const digest = createHash("sha256").update(pcm).digest("hex")
  return { identity: `sha256:${digest}`, bytes: pcm.byteLength, pcm }
}

function stemMap(stems) {
  return Object.fromEntries(stems.map((stem) => [stem.identity, stem.pcm]))
}

function requirements(stems) {
  return stems.map(({ identity, bytes }) => ({ identity, bytes }))
}

function store(backend, locks, options = {}) {
  return new OpfsStemStore({
    storage: backend.storage(),
    locks,
    tabId: options.tabId,
    hooks: options.hooks,
    now: options.now,
    readDeadlineMs: 1_000,
    ingestReadDeadlineMs: options.ingestReadDeadlineMs ?? 1_000,
  })
}

async function fanMixNarrative() {
  const stems = Array.from({ length: 12 }, (_, index) => fixture(`stem-${index}`))
  const mixA = stems.slice(0, 8)
  const mixB = [...stems.slice(0, 6), stems[8], stems[9]]
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap(stems), { chunkBytes: 257 })
  const opfs = store(backend, locks, { tabId: "fan" })

  const leaseA = await opfs.openSession({
    sessionId: "A",
    stems: requirements(mixA),
    resolver,
  })
  assert.equal(resolver.requests.length, 8)
  const leaseB = await opfs.openSession({
    sessionId: "B",
    stems: requirements(mixB),
    resolver,
  })
  assert.equal(resolver.requests.length, 10, "mix B fetches only its two misses")

  const repeatedA = await opfs.openSession({
    sessionId: "A-repeat",
    stems: requirements(mixA),
    resolver,
  })
  const repeatedB = await opfs.openSession({
    sessionId: "B-repeat",
    stems: requirements(mixB),
    resolver,
  })
  assert.equal(resolver.requests.length, 10, "pinned A↔B reopens never re-ingest")

  const freshBackend = new FakeOpfsBackend()
  const freshResolver = new MemoryStemResolver(stemMap(stems))
  const fresh = store(freshBackend, new FakeLockManager(), { tabId: "fresh" })
  const warmSix = await fresh.openSession({
    sessionId: "seed",
    stems: requirements(stems.slice(0, 6)),
    resolver: freshResolver,
  })
  const mixC = [...stems.slice(0, 6), stems[10], stems[11]]
  const leaseC = await fresh.openSession({
    sessionId: "C",
    stems: requirements(mixC),
    resolver: freshResolver,
  })
  assert.equal(freshResolver.requests.length, 8, "mix C fetches only what profile lacks")

  await Promise.all([
    leaseA.close(),
    leaseB.close(),
    repeatedA.close(),
    repeatedB.close(),
    warmSix.close(),
    leaseC.close(),
  ])
}

async function twoTabCollision() {
  const stem = fixture("shared-tab", 64 * 1024)
  const other = fixture("locked-elsewhere")
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap([stem, other]), {
    chunkBytes: 1024,
  })
  const tabA = store(backend, locks, { tabId: "tab-a" })
  const tabB = store(backend, locks, { tabId: "tab-b" })
  const [leaseA, leaseB] = await Promise.all([
    tabA.openSession({ sessionId: "A", stems: requirements([stem]), resolver }),
    tabB.openSession({ sessionId: "B", stems: requirements([stem]), resolver }),
  ])
  assert.equal(resolver.requests.length, 1, "Web Lock single-flight prevents double download")
  assert.equal(backend.names("miso-stems-v1/staging").length, 0)

  let release
  const held = locks.request(
    `miso:stem-store:v1:miso-stems-v1:ingest:${other.identity.slice(7)}`,
    { mode: "exclusive" },
    () => new Promise((resolve) => {
      release = resolve
    })
  )
  while (release === undefined) await Promise.resolve()
  const read = await Promise.race([
    leaseB.read(stem.identity),
    new Promise((_resolve, reject) =>
      setTimeout(() => reject(new Error("getFile playback read stalled")), 50)
    ),
  ])
  assert.equal(read.size, stem.bytes)
  release()
  await held
  await Promise.all([leaseA.close(), leaseB.close()])
}

async function corruptionSelfHeals() {
  const stem = fixture("corruption", 8192)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  let corruptStagingOnce = true
  const opfs = store(backend, locks, {
    tabId: "corrupt",
    hooks: {
      afterStagingWrite({ handle }) {
        if (!corruptStagingOnce) return
        corruptStagingOnce = false
        handle.node.bytes[17] ^= 0x80
      },
    },
  })
  const resolver = new MemoryStemResolver(stemMap([stem]), { chunkBytes: 131 })
  const first = await opfs.openSession({
    sessionId: "first",
    stems: requirements([stem]),
    resolver,
  })
  assert.equal(resolver.requests.length, 2, "staging corruption forces a fresh resolve")
  await first.close()

  const path = `miso-stems-v1/sha256-${stem.identity.slice(7)}`
  const corrupted = new Uint8Array(backend.bytes(path))
  corrupted[stem.bytes - 3] ^= 1
  backend.setBytes(path, corrupted)
  const second = await opfs.openSession({
    sessionId: "same-length-bitflip",
    stems: requirements([stem]),
    resolver,
  })
  assert.equal(resolver.requests.length, 3, "verify-on-open catches and heals same-length rot")
  assert.deepEqual(backend.bytes(path), stem.pcm)
  await second.close()

  backend.setBytes(path, stem.pcm.subarray(0, stem.bytes - 2))
  const third = await opfs.openSession({
    sessionId: "truncated",
    stems: requirements([stem]),
    resolver,
  })
  assert.equal(resolver.requests.length, 4, "verify-on-open catches and heals truncation")
  assert.equal(await opfs.verify(stem.identity), true)
  await third.close()
}

async function indexRecoveryAdoptsSelfVerifyingFinals() {
  for (const damage of ["missing", "corrupt"]) {
    const stem = fixture(`index-${damage}`, 8192)
    const backend = new FakeOpfsBackend()
    const locks = new FakeLockManager()
    const seeded = store(backend, locks, { tabId: `seed-${damage}` })
    const seedLease = await seeded.openSession({
      sessionId: "seed",
      stems: requirements([stem]),
      resolver: new MemoryStemResolver(stemMap([stem])),
    })
    await seedLease.close()

    if (damage === "missing") backend.remove("miso-stems-v1/index.json")
    else backend.setBytes("miso-stems-v1/index.json", encoder.encode("{not-json"))

    const resolver = new MemoryStemResolver(stemMap([stem]))
    const recovered = store(backend, locks, { tabId: `recover-${damage}` })
    const lease = await recovered.openSession({
      sessionId: "recovered",
      stems: requirements([stem]),
      resolver,
    })
    assert.equal(
      resolver.requests.length,
      0,
      `${damage} crash-only index must adopt a self-verifying final without re-ingest`
    )
    const index = JSON.parse(
      new TextDecoder().decode(backend.bytes("miso-stems-v1/index.json"))
    )
    assert.equal(index.stems[stem.identity].bytes, stem.bytes)
    assert.deepEqual(
      backend.bytes(`miso-stems-v1/sha256-${stem.identity.slice(7)}`),
      stem.pcm
    )
    await lease.close()
  }
}

async function promoteAtomicityAndCrashSweep() {
  const stem = fixture("fallback", 8192)
  const backend = new FakeOpfsBackend({ moveSupported: false })
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap([stem]))
  const working = store(backend, locks, { tabId: "fallback-ok" })
  const lease = await working.openSession({
    sessionId: "fallback",
    stems: requirements([stem]),
    resolver,
  })
  await lease.close()
  assert.equal(await working.verify(stem.identity), true)

  const crashStem = fixture("copy-crash", 4096)
  const crashBackend = new FakeOpfsBackend({ moveSupported: false })
  const crashLocks = new FakeLockManager()
  const crash = new Error("simulated tab death after fallback copy")
  crash.leaveStaging = true
  const dying = store(crashBackend, crashLocks, {
    tabId: "dead-tab",
    hooks: { afterFallbackCopy: () => { throw crash } },
  })
  await assert.rejects(
    dying.openSession({
      sessionId: "dies",
      stems: requirements([crashStem]),
      resolver: new MemoryStemResolver(stemMap([crashStem])),
    }),
    /simulated tab death/
  )
  assert.equal(crashBackend.names("miso-stems-v1/staging").length, 1)
  assert.equal(
    crashBackend.has(`miso-stems-v1/sha256-${crashStem.identity.slice(7)}`),
    true,
    "copy may leave final-name debris"
  )
  await store(crashBackend, crashLocks, { tabId: "next-tab" }).open()
  assert.deepEqual(crashBackend.names("miso-stems-v1/staging"), [])
  assert.equal(
    crashBackend.has(`miso-stems-v1/sha256-${crashStem.identity.slice(7)}`),
    false,
    "unindexed final debris is never trusted"
  )
}

async function openerDoesNotSweepActivePromote() {
  const stem = fixture("active-promote", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  let moved
  let continuePromote
  const reachedMove = new Promise((resolve) => { moved = resolve })
  const holdPromote = new Promise((resolve) => { continuePromote = resolve })
  const ingesting = store(backend, locks, {
    tabId: "promoting",
    hooks: {
      async afterMove() {
        moved()
        await holdPromote
      },
    },
  })
  const opening = ingesting.openSession({
    sessionId: "promoting",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
  })
  await reachedMove
  const finalPath = `miso-stems-v1/sha256-${stem.identity.slice(7)}`
  assert.equal(backend.has(finalPath), true)
  await store(backend, locks, { tabId: "observer" }).open()
  assert.equal(
    backend.has(finalPath),
    true,
    "an opener must not sweep a final protected by a live ingest lock"
  )
  continuePromote()
  const lease = await opening
  await lease.close()
}

async function crashedSessionPinsAreRecoverable() {
  const stem = fixture("crashed-pin", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const lease = await store(backend, locks, { tabId: "crashed" }).openSession({
    sessionId: "gone",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
  })
  lease.releasePinLock()
  while ((await locks.query()).held.length !== 0) await Promise.resolve()
  await store(backend, locks, { tabId: "recovery" }).open()
  const index = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-v1/index.json"))
  )
  assert.deepEqual(index.stems[stem.identity].pins, [])
}

async function quotaFailureKeepsSurvivors() {
  const firstStem = fixture("quota-first", 4096)
  const secondStem = fixture("quota-second", 4096)
  const backend = new FakeOpfsBackend({ quota: 30_000, reportedQuota: 30_000 })
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap([firstStem, secondStem]))
  const opfs = store(backend, locks, { tabId: "quota" })
  const survivor = await opfs.openSession({
    sessionId: "survivor",
    stems: requirements([firstStem]),
    resolver,
  })
  backend.reportedQuota = backend.usage + secondStem.bytes - 1
  await assert.rejects(
    opfs.openSession({
      sessionId: "too-big",
      stems: requirements([firstStem, secondStem]),
      resolver,
    }),
    (error) => {
      assert.equal(error.code, "storage.insufficient")
      assert.equal(error.details.shortfallBytes, 1)
      assert.equal(error.details.evictableBytes, 0)
      return true
    }
  )
  assert.equal(await opfs.verify(firstStem.identity), true, "promoted survivor remains")
  backend.reportedQuota = 30_000
  const retry = await opfs.openSession({
    sessionId: "retry",
    stems: requirements([firstStem, secondStem]),
    resolver,
  })
  assert.equal(
    resolver.requests.filter((identity) => identity === firstStem.identity).length,
    1,
    "retry reuses the promoted survivor"
  )
  await Promise.all([survivor.close(), retry.close()])
}

async function lruEvictsOnlyUnpinned() {
  const oldest = fixture("lru-oldest", 4096)
  const newer = fixture("lru-newer", 4096)
  const incoming = fixture("lru-incoming", 4096)
  const backend = new FakeOpfsBackend({ quota: 100_000, reportedQuota: 100_000 })
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap([oldest, newer, incoming]))
  let now = 1
  const opfs = store(backend, locks, { tabId: "lru", now: () => now })
  const oldLease = await opfs.openSession({
    sessionId: "old",
    stems: requirements([oldest]),
    resolver,
  })
  await oldLease.close()
  now = 2
  const newLease = await opfs.openSession({
    sessionId: "new",
    stems: requirements([newer]),
    resolver,
  })
  await newLease.close()
  backend.reportedQuota = backend.usage
  now = 3
  const incomingLease = await opfs.openSession({
    sessionId: "incoming",
    stems: requirements([incoming]),
    resolver,
  })
  assert.equal(await opfs.verify(oldest.identity), false, "oldest unpinned row evicts first")
  assert.equal(await opfs.verify(newer.identity), true, "newer row survives one-stem shortfall")
  await incomingLease.close()
}

async function tabCloseMidStaging() {
  const stem = fixture("tab-close", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const crash = new Error("tab closed")
  crash.leaveStaging = true
  let first = true
  const dying = store(backend, locks, {
    tabId: "doomed",
    hooks: {
      afterChunk() {
        if (first) {
          first = false
          throw crash
        }
      },
    },
  })
  await assert.rejects(
    dying.openSession({
      sessionId: "close",
      stems: requirements([stem]),
      resolver: new MemoryStemResolver(stemMap([stem]), { chunkBytes: 64 }),
    }),
    /tab closed/
  )
  assert.equal(backend.names("miso-stems-v1/staging").length, 1)
  await store(backend, locks, { tabId: "replacement" }).open()
  assert.deepEqual(backend.names("miso-stems-v1/staging"), [])
  assert.equal(
    backend.names("miso-stems-v1").filter((name) => name.startsWith("sha256-")).length,
    0
  )
}

async function abortCleansStaging() {
  const stem = fixture("abort-cleanup", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const controller = new AbortController()
  let aborted = false
  const opfs = store(backend, locks, {
    tabId: "abort",
    hooks: {
      afterChunk() {
        if (!aborted) {
          aborted = true
          controller.abort(new DOMException("mix switched", "AbortError"))
        }
      },
    },
  })
  await assert.rejects(
    opfs.openSession({
      sessionId: "aborted",
      stems: requirements([stem]),
      resolver: new MemoryStemResolver(stemMap([stem]), { chunkBytes: 64 }),
      signal: controller.signal,
    }),
    (error) => error.name === "AbortError"
  )
  assert.deepEqual(backend.names("miso-stems-v1/staging"), [])
  assert.equal(
    backend.names("miso-stems-v1").some((name) => name.startsWith("sha256-")),
    false
  )
}

async function wedgedDecoderObeysAbortAndDeadline() {
  const stem = fixture("wedged-decoder", 4096)
  const makeResolver = (onPull) => ({
    async resolve() {
      return {
        stream: new ReadableStream({
          pull() {
            onPull()
            return new Promise(() => {})
          },
        }, { highWaterMark: 0 }),
      }
    },
  })

  const abortBackend = new FakeOpfsBackend()
  const abortController = new AbortController()
  let readStarted
  const started = new Promise((resolve) => { readStarted = resolve })
  const abortStore = store(abortBackend, new FakeLockManager(), {
    tabId: "wedge-abort",
    ingestReadDeadlineMs: 1_000,
  })
  const opening = abortStore.openSession({
    sessionId: "wedge-abort",
    stems: requirements([stem]),
    resolver: makeResolver(readStarted),
    signal: abortController.signal,
  })
  await started
  abortController.abort(new DOMException("mix switched", "AbortError"))
  const abortOutcome = await Promise.race([
    opening.then(
      () => "opened",
      (error) => error
    ),
    new Promise((resolve) => setTimeout(() => resolve("timed-out"), 100)),
  ])
  assert.notEqual(
    abortOutcome,
    "timed-out",
    "wedged decoder ignored mix-switch abort"
  )
  assert.equal(abortOutcome.name, "AbortError")
  assert.deepEqual(abortBackend.names("miso-stems-v1/staging"), [])

  const deadlineBackend = new FakeOpfsBackend()
  const deadlineStore = store(deadlineBackend, new FakeLockManager(), {
    tabId: "wedge-deadline",
    ingestReadDeadlineMs: 20,
  })
  const deadlineOutcome = await Promise.race([
    deadlineStore.openSession({
      sessionId: "wedge-deadline",
      stems: requirements([stem]),
      resolver: makeResolver(() => {}),
    }).then(
      () => "opened",
      (error) => error
    ),
    new Promise((resolve) => setTimeout(() => resolve("timed-out"), 200)),
  ])
  assert.notEqual(deadlineOutcome, "timed-out", "wedged decoder escaped its deadline")
  assert.equal(deadlineOutcome.code, "stem.decode.stalled")
  assert.deepEqual(deadlineBackend.names("miso-stems-v1/staging"), [])
}

async function storageUnavailable() {
  const opfs = new OpfsStemStore({ storage: {}, locks: new FakeLockManager() })
  await assert.rejects(opfs.open(), (error) => {
    assert.equal(error.code, "storage.unavailable")
    return true
  })
}

async function latencyRows() {
  const stems = Array.from({ length: 8 }, (_, index) => fixture(`latency-${index}`, 256 * 1024))
  const backend = new FakeOpfsBackend()
  const opfs = store(backend, new FakeLockManager(), { tabId: "latency" })
  const resolver = new MemoryStemResolver(stemMap(stems), { chunkBytes: 16 * 1024 })
  const startedCold = performance.now()
  const cold = await opfs.openSession({
    sessionId: "cold",
    stems: requirements(stems),
    resolver,
  })
  const coldMs = performance.now() - startedCold
  const startedWarm = performance.now()
  const warm = await opfs.openSession({
    sessionId: "warm",
    stems: requirements(stems),
    resolver,
  })
  const verifyMs = performance.now() - startedWarm
  assert.equal(resolver.requests.length, 8)
  assert.ok(coldMs < 5_000, `fixture cold-open budget exceeded: ${coldMs}ms`)
  assert.ok(verifyMs < 2_000, `fixture verify-open budget exceeded: ${verifyMs}ms`)
  process.stdout.write(
    `${JSON.stringify({ eval: "cold-open-latency", stems: 8, canonicalBytes: stems.reduce((sum, stem) => sum + stem.bytes, 0), coldMs: Number(coldMs.toFixed(3)), warmVerifyMs: Number(verifyMs.toFixed(3)), budgetsMs: { cold: 5_000, warmVerify: 2_000 } })}\n`
  )
  await Promise.all([cold.close(), warm.close()])
}

async function modeDetection() {
  const flushed = new Uint8Array([0x49, 0x44, 0x33, 0x04])
  let snapshotReads = 0
  const safariWriterContention = {
    async createSyncAccessHandle(options) {
      assert.deepEqual(options, { mode: "read-only" })
      const error = new Error("existing file already has WebKit's exclusive writer")
      error.name = "InvalidStateError"
      throw error
    },
    async getFile() {
      snapshotReads += 1
      return new Blob([flushed])
    },
  }
  assert.equal(await detectSharedReadOnlyMode(safariWriterContention), false)
  const fallback = await safariWriterContention.getFile()
  assert.deepEqual(new Uint8Array(await fallback.arrayBuffer()), flushed)
  assert.equal(snapshotReads, 1, "contention must fall back to a flushed getFile snapshot")

  let ignoredModeClosed = false
  const safariIgnoredMode = {
    async createSyncAccessHandle(options) {
      assert.deepEqual(options, { mode: "read-only" })
      return { close() { ignoredModeClosed = true } }
    },
  }
  assert.equal(await detectSharedReadOnlyMode(safariIgnoredMode), false)
  assert.equal(ignoredModeClosed, true)

  assert.equal(
    await detectSharedReadOnlyMode({
      async createSyncAccessHandle() {
        return { mode: "read-only", close() {} }
      },
    }),
    true
  )
}

await fanMixNarrative()
await twoTabCollision()
await corruptionSelfHeals()
await indexRecoveryAdoptsSelfVerifyingFinals()
await promoteAtomicityAndCrashSweep()
await openerDoesNotSweepActivePromote()
await crashedSessionPinsAreRecoverable()
await quotaFailureKeepsSurvivors()
await lruEvictsOnlyUnpinned()
await tabCloseMidStaging()
await abortCleansStaging()
await wedgedDecoderObeysAbortAndDeadline()
await storageUnavailable()
await latencyRows()
await modeDetection()
process.stdout.write("stem-store-core-v1: PASS\n")
