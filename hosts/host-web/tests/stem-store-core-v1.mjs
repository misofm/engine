import assert from "node:assert/strict"
import { performance } from "node:perf_hooks"
import { IncrementalBlake3 } from "../web/stem-store/incremental-blake3.js"
import {
  OpfsStemStore,
  StemStoreError,
  detectSharedReadOnlyMode,
} from "../web/stem-store/opfs-store.js"
import { MemoryStemResolver } from "../web/stem-store/resolver.js"
import { StemSessionGate } from "../web/stem-store/session-gate.js"
import { FakeLockManager, FakeOpfsBackend } from "./stem-store-fakes.mjs"

const encoder = new TextEncoder()

// Wall-clock budget assertions are opt-in (`--budgets`, or run via
// scripts/check-stem-store-v1.mjs --budgets): they are real thresholds worth tracking, but a
// flaky wall-clock failure must never block the qualification lint shard. Functional assertions
// (typed errors, staging cleanliness, index contents, byte counts, ...) always run.
const budgetsEnabled = process.argv.includes("--budgets")
let skippedBudgetAsserts = 0

function budgetAssert(condition, message) {
  if (budgetsEnabled) {
    assert.ok(condition, message)
  } else {
    skippedBudgetAsserts += 1
  }
}

function fixture(label, bytes = 4096) {
  const seed = encoder.encode(label)
  const pcm = new Uint8Array(bytes)
  for (let index = 0; index < pcm.length; index += 1) {
    pcm[index] = seed[index % seed.length] ^ ((index * 29) & 0xff)
  }
  const hash = new IncrementalBlake3()
  hash.update(pcm)
  return { identity: `blake3:${hash.digestHex()}`, bytes: pcm.byteLength, pcm }
}

function stemMap(stems) {
  return Object.fromEntries(stems.map((stem) => [stem.identity, stem.pcm]))
}

function requirements(stems) {
  return stems.map(({ identity, bytes }) => ({ identity, bytes }))
}

function indexStems(backend) {
  return JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
  ).stems
}

function store(backend, locks, options = {}) {
  return new OpfsStemStore({
    storage: backend.storage(),
    locks,
    tabId: options.tabId,
    hooks: options.hooks,
    now: options.now,
    readDeadlineMs: options.readDeadlineMs ?? 1_000,
    maintenanceBudgetMs: options.maintenanceBudgetMs,
    ingestReadDeadlineMs: options.ingestReadDeadlineMs ?? 1_000,
  })
}

const never = () => new Promise(() => {})

function settledWithin(promise, milliseconds) {
  return Promise.race([
    promise.then(
      (value) => ({ value }),
      (error) => ({ error })
    ),
    new Promise((resolve) => setTimeout(() => resolve("timed-out"), milliseconds)),
  ])
}

async function twoMixNarrative() {
  const stems = Array.from({ length: 12 }, (_, index) => fixture(`stem-${index}`))
  const mixA = stems.slice(0, 8)
  const mixB = [...stems.slice(0, 6), stems[8], stems[9]]
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap(stems), { chunkBytes: 257 })
  const opfs = store(backend, locks, { tabId: "two-mix" })

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
  assert.equal(backend.names("miso-stems-blake3-v1/staging").length, 0)

  let release
  const held = locks.request(
    `miso:stem-store:v1:miso-stems-blake3-v1:ingest:${other.identity.slice(7)}`,
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

  const path = `miso-stems-blake3-v1/blake3-${stem.identity.slice(7)}`
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
  const truncatedProgress = []
  const third = await opfs.openSession({
    sessionId: "truncated",
    stems: requirements([stem]),
    resolver,
    onProgress(event) {
      truncatedProgress.push(event)
    },
  })
  assert.equal(resolver.requests.length, 4, "verify-on-open catches and heals truncation")
  assert.deepEqual(
    truncatedProgress.filter((event) => event.stage === "verified-open"),
    [],
    "a truncated stem was hashed before its size refused it"
  )
  assert.equal(await opfs.verify(stem.identity), true)
  await third.close()
}

async function wedgedDemoteDeleteIsBounded() {
  for (const blocker of ["createWritableBlocker", "writeBlocker", "closeBlocker"]) {
    const stem = fixture(`wedged-demote-${blocker}`, 4096)
    const backend = new FakeOpfsBackend()
    const opfs = store(backend, new FakeLockManager(), {
      tabId: "wedged-demote",
      readDeadlineMs: 10,
    })
    const lease = await opfs.openSession({
      sessionId: "seed",
      stems: requirements([stem]),
      resolver: new MemoryStemResolver(stemMap([stem])),
    })
    await lease.close()
    const finalName = `blake3-${stem.identity.slice(7)}`
    const corrupted = new Uint8Array(stem.pcm)
    corrupted[5] ^= 1
    backend.setBytes(`miso-stems-blake3-v1/${finalName}`, corrupted)
    let failDelete
    backend.removeEntryBlocker = ({ name }) =>
      name === finalName
        ? new Promise((_resolve, reject) => (failDelete = () => reject(new Error("fake delete failure"))))
        : undefined
    const outcome = await settledWithin(opfs.verify(stem.identity), 500)
    assert.notEqual(outcome, "timed-out", "a demote awaited a wedged delete without a deadline")
    assert.equal(outcome.value, false)
    assert.equal(indexStems(backend)[stem.identity], undefined, "a demoted stem stayed indexed")

    // Issue #2381: the timed-out delete may still land, so the stem lock outlives it and the next
    // verify of that stem waits until the delete settles. Here it fails and leaves the corrupt final,
    // so the second verify demotes again; now one step of the index write under the index lock hangs.
    backend.removeEntryBlocker = undefined
    backend[blocker] = ({ fileName }) => (fileName === "index.json" ? never() : undefined)
    let secondSettled = false
    const second = opfs.verify(stem.identity)
    second.then(
      () => (secondSettled = true),
      () => (secondSettled = true)
    )
    await new Promise((resolve) => setTimeout(resolve, 40))
    assert.equal(secondSettled, false, "a verify ran while a demote's delete of its final was unsettled")
    failDelete()
    const wedgedWrite = await settledWithin(second, 500)
    assert.notEqual(
      wedgedWrite,
      "timed-out",
      `an index write under the index lock had no deadline (${blocker})`
    )
    assert.equal(wedgedWrite.error?.code, "storage.write_stalled")
  }
}

async function lateIndexWriteCannotOverwriteANewerOne() {
  const stem = fixture("late-index-write", 4096)
  const backend = new FakeOpfsBackend()
  const opfs = store(backend, new FakeLockManager(), { tabId: "late", readDeadlineMs: 20 })
  const lease = await opfs.openSession({
    sessionId: "seed",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
  })
  await lease.close()
  let lateOnce = true
  backend.closeBlocker = ({ fileName }) => {
    if (fileName !== "index.json" || !lateOnce) return undefined
    lateOnce = false
    return new Promise((resolve) => setTimeout(resolve, 80))
  }
  await assert.rejects(opfs.setOfflinePin(stem.identity, "older", true), (error) => {
    assert.equal(error.code, "storage.write_stalled")
    return true
  })
  // The newer write either is refused while the late close is pending, or
  // lands; it must still be on disk after the late close settles.
  for (let attempt = 0; ; attempt += 1) {
    try {
      await opfs.setOfflinePin(stem.identity, "newer", true)
      break
    } catch (error) {
      assert.equal(error.code, "storage.write_stalled")
      assert.ok(attempt < 50, "a stalled index write never settled")
      await new Promise((resolve) => setTimeout(resolve, 10))
    }
  }
  await new Promise((resolve) => setTimeout(resolve, 150))
  assert.ok(
    indexStems(backend)[stem.identity].pins.includes("offline:newer"),
    "a late index close overwrote a newer index"
  )
}

async function unwritableStoreRefusesBeforeNetwork() {
  const stem = fixture("unwritable", 4096)
  const stagingName = `unwritable-${stem.identity.slice(7)}`
  const backend = new FakeOpfsBackend({
    createWritableBlocker({ fileName }) {
      if (fileName !== stagingName) return
      const error = new Error("fake read-only device")
      error.name = "NoModificationAllowedError"
      throw error
    },
  })
  const resolver = new MemoryStemResolver(stemMap([stem]))
  await assert.rejects(
    store(backend, new FakeLockManager(), { tabId: "unwritable" }).openSession({
      sessionId: "unwritable",
      stems: requirements([stem]),
      resolver,
    }),
    (error) => {
      assert.equal(error.code, "storage.unavailable")
      return true
    }
  )
  assert.equal(resolver.requests.length, 0, "a download started before staging was writable")
}

async function openSurvivesFailingSweeps() {
  const stem = fixture("failing-sweeps", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const seed = await store(backend, locks, { tabId: "sweep-seed" }).openSession({
    sessionId: "seed",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
  })
  await seed.close()
  const staging = backend.root.entries
    .get("miso-stems-blake3-v1")
    .entries.get("staging")
  // The staging listing hangs; the final-name listing throws.
  backend.entriesBlocker = ({ node }) => {
    if (node === staging) return never()
    throw new Error("fake listing failure")
  }
  const failures = []
  const opfs = store(backend, locks, {
    tabId: "sweep-open",
    readDeadlineMs: 20,
    maintenanceBudgetMs: 200,
    hooks: { maintenanceFailure: (event) => failures.push(event.sweep) },
  })
  const opened = await settledWithin(opfs.open(), 500)
  assert.notEqual(opened, "timed-out", "a wedged open-time sweep escaped its deadline")
  assert.equal(opened.error, undefined, "a failing open-time sweep made the store unopenable")
  assert.deepEqual(failures, ["staging", "finals"])
  backend.entriesBlocker = undefined
  const resolver = new MemoryStemResolver(stemMap([stem]))
  const lease = await opfs.openSession({
    sessionId: "after-failing-sweeps",
    stems: requirements([stem]),
    resolver,
  })
  assert.equal(resolver.requests.length, 0)
  await lease.close()
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

    if (damage === "missing") backend.remove("miso-stems-blake3-v1/index.json")
    else backend.setBytes("miso-stems-blake3-v1/index.json", encoder.encode("{not-json"))

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
      new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
    )
    assert.equal(index.stems[stem.identity].bytes, stem.bytes)
    assert.deepEqual(
      backend.bytes(`miso-stems-blake3-v1/blake3-${stem.identity.slice(7)}`),
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
  assert.equal(crashBackend.names("miso-stems-blake3-v1/staging").length, 1)
  assert.equal(
    crashBackend.has(`miso-stems-blake3-v1/blake3-${crashStem.identity.slice(7)}`),
    true,
    "copy may leave final-name debris"
  )
  await store(crashBackend, crashLocks, { tabId: "next-tab" }).open()
  assert.deepEqual(crashBackend.names("miso-stems-blake3-v1/staging"), [])
  assert.equal(
    crashBackend.has(`miso-stems-blake3-v1/blake3-${crashStem.identity.slice(7)}`),
    false,
    "unindexed final debris is never trusted"
  )
}

async function fallbackWritesObeyAbortAndDeadline() {
  const stem = fixture("fallback-write-abort", 8192)
  const finalName = `blake3-${stem.identity.slice(7)}`
  const never = new Promise(() => {})
  let writeStarted
  const started = new Promise((resolve) => { writeStarted = resolve })
  const abortBackend = new FakeOpfsBackend({
    moveSupported: false,
    writeBlocker({ fileName }) {
      if (fileName !== finalName) return undefined
      writeStarted()
      return never
    },
    abortBlocker({ fileName }) {
      return fileName === finalName ? never : undefined
    },
  })
  const abortController = new AbortController()
  const localDeadlineMs = 100
  const abortStore = store(abortBackend, new FakeLockManager(), {
    tabId: "fallback-write-abort",
    readDeadlineMs: localDeadlineMs,
  })
  const opening = abortStore.openSession({
    sessionId: "fallback-write-abort",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
    signal: abortController.signal,
  })
  await started
  const abortStartedAt = performance.now()
  abortController.abort(new DOMException("mix switched", "AbortError"))
  const abortOutcome = await Promise.race([
    opening.then(
      () => "opened",
      (error) => error
    ),
    new Promise((resolve) => setTimeout(() => resolve("timed-out"), 300)),
  ])
  const abortElapsedMs = performance.now() - abortStartedAt
  assert.notEqual(
    abortOutcome,
    "timed-out",
    "fallback final write ignored mix-switch abort"
  )
  assert.equal(abortOutcome.name, "AbortError", "fallback abort must stay typed")
  budgetAssert(
    abortElapsedMs < localDeadlineMs,
    `fallback abort escaped its ${localDeadlineMs} ms local-store deadline: ${abortElapsedMs} ms`
  )
  assert.deepEqual(abortBackend.names("miso-stems-blake3-v1/staging"), [])
  assert.equal(
    abortBackend.has(`miso-stems-blake3-v1/${finalName}`),
    false,
    "aborted fallback promotion left unindexed final debris"
  )

  for (const operation of ["createWritable", "write", "close"]) {
    const deadlineStem = fixture(`fallback-${operation}-deadline`, 4096)
    const deadlineFinal = `blake3-${deadlineStem.identity.slice(7)}`
    const blocker = ({ fileName }) =>
      fileName === deadlineFinal ? never : undefined
    const backend = new FakeOpfsBackend({
      moveSupported: false,
      ...(operation === "createWritable"
        ? { createWritableBlocker: blocker }
        : operation === "write"
          ? { writeBlocker: blocker }
          : { closeBlocker: blocker }),
    })
    const opfs = store(backend, new FakeLockManager(), {
      tabId: `fallback-${operation}-deadline`,
      readDeadlineMs: 20,
    })
    const outcome = await Promise.race([
      opfs.openSession({
        sessionId: `fallback-${operation}-deadline`,
        stems: requirements([deadlineStem]),
        resolver: new MemoryStemResolver(stemMap([deadlineStem])),
      }).then(
        () => "opened",
        (error) => error
      ),
      new Promise((resolve) => setTimeout(() => resolve("timed-out"), 200)),
    ])
    assert.notEqual(
      outcome,
      "timed-out",
      `wedged fallback ${operation} escaped its local-store deadline`
    )
    assert.equal(outcome.code, "storage.write_stalled")
    assert.deepEqual(backend.names("miso-stems-blake3-v1/staging"), [])
    assert.equal(
      backend.has(`miso-stems-blake3-v1/${deadlineFinal}`),
      false,
      `wedged fallback ${operation} left unindexed final debris`
    )
  }
}

// Issue #2381: a demotion with no hold of the stem lock (a read failure's) takes the lock without
// waiting, even when the holder is another hold in this same store. Here that hold is re-ingesting
// the stem, so the demotion must leave the final to it, and must not wait for it either.
async function aHoldlessDemoteLeavesAFinalAnotherHoldOwns() {
  for (const leg of ["Web Locks", "no Web Locks"]) {
    const stem = fixture("holdless-demote", 4096)
    const backend = new FakeOpfsBackend()
    const memory = new MemoryStemResolver(stemMap([stem]))
    let resolverReached
    let releaseResolver
    const reached = new Promise((resolve) => (resolverReached = resolve))
    const gate = new Promise((resolve) => (releaseResolver = resolve))
    const resolver = {
      async resolve(identity, options) {
        resolverReached()
        await gate
        return memory.resolve(identity, options)
      },
    }
    const locks = leg === "Web Locks" ? new FakeLockManager() : {}
    const opfs = store(backend, locks, { tabId: "holdless", readDeadlineMs: 20 })
    await (
      await opfs.openSession({ sessionId: "seed", stems: requirements([stem]), resolver: memory })
    ).close()
    const lease = await opfs.openSession({ sessionId: "L1", stems: requirements([stem]), resolver: memory })
    let failRead
    backend.getFileBlocker = ({ name }) =>
      name === finalName(stem) && failRead === undefined
        ? new Promise((_resolve, reject) => (failRead = reject))
        : undefined
    const reading = settledWithin(lease.read(stem.identity), 500)
    await until(() => failRead !== undefined, "the lease's read")
    backend.getFileBlocker = undefined
    // B's verify demotes the corrupt final, and B re-ingests the stem under a new hold.
    backend.setBytes(`${STORE_FOLDER}/${finalName(stem)}`, encoder.encode("corrupt"))
    const reopening = opfs.openSession({ sessionId: "B", stems: requirements([stem]), resolver })
    await reached
    let removalStarted = false
    backend.removeEntryBlocker = ({ name }) => {
      if (name === finalName(stem)) removalStarted = true
    }
    const failure = new Error("transient read failure")
    failure.name = "NotReadableError"
    failRead(failure)
    const read = await reading
    assert.notEqual(read, "timed-out", `a read failure's demotion waited on another hold (${leg})`)
    assert.equal(read.error?.code, "stem.read.failed")
    assert.equal(
      removalStarted,
      false,
      `a demotion with no hold removed a final another hold of this store had (${leg})`
    )
    backend.removeEntryBlocker = undefined
    releaseResolver()
    const reopened = await reopening
    assert.equal((await reopened.read(stem.identity)).size, stem.bytes)
    // With every hold gone, the next read failure's demotion does take the lock and remove the file.
    for (let step = 0; step < 5; step += 1) await tick()
    backend.getFileBlocker = ({ name }) => {
      if (name !== finalName(stem)) return undefined
      throw failure
    }
    await assert.rejects(reopened.read(stem.identity), (error) => error.code === "stem.read.failed")
    backend.getFileBlocker = undefined
    assert.equal(
      backend.has(`${STORE_FOLDER}/${finalName(stem)}`),
      false,
      `a demotion with no hold left a final whose lock was free (${leg})`
    )
    await reopened.close()
  }
}

// Issue #2381: one hold never writes a final while a removal of that final it started is unsettled.
// Here an open's ingest recheck finds a corrupt row that another tab indexed after the open's own
// verify loop, demotes it, and the removal wedges. The open must wait for that removal, not promote
// a final that the late removal would then delete; once it lands, the open promotes and reads.
async function aHoldNeverWritesAFinalItIsStillRemoving() {
  const stem = fixture("recheck-fence", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  const resolver = new MemoryStemResolver(stemMap([stem]))
  let estimateReached
  let releaseEstimate
  const reached = new Promise((resolve) => (estimateReached = resolve))
  const gate = new Promise((resolve) => (releaseEstimate = resolve))
  const shared = backend.storage()
  const a = new OpfsStemStore({
    storage: {
      getDirectory: shared.getDirectory,
      async estimate() {
        estimateReached()
        await gate
        return shared.estimate()
      },
    },
    locks,
    tabId: "recheck-a",
    readDeadlineMs: 20,
  })
  const opening = settledWithin(
    a.openSession({ sessionId: "A", stems: requirements([stem]), resolver }),
    1_000
  )
  // A's verify loop found a miss; it waits in its quota preflight while another tab indexes the stem.
  await reached
  await (
    await store(backend, locks, { tabId: "recheck-t" }).openSession({
      sessionId: "T",
      stems: requirements([stem]),
      resolver,
    })
  ).close()
  const corrupted = new Uint8Array(stem.pcm)
  corrupted[9] ^= 1
  backend.setBytes(`${STORE_FOLDER}/${finalName(stem)}`, corrupted)
  let land
  backend.removeEntryBlocker = ({ name }) =>
    name === finalName(stem) && land === undefined ? new Promise((resolve) => (land = resolve)) : undefined
  let settledEarly = false
  opening.then(() => (settledEarly = !landed))
  let landed = false
  releaseEstimate()
  await until(() => land !== undefined, "the recheck's removal")
  await new Promise((resolve) => setTimeout(resolve, 60))
  assert.equal(settledEarly, false, "a hold wrote a final while its own removal of that final was unsettled")
  backend.removeEntryBlocker = undefined
  landed = true
  land()
  const outcome = await opening
  assert.notEqual(outcome, "timed-out")
  assert.equal(outcome.error, undefined)
  assert.equal((await outcome.value.read(stem.identity)).size, stem.bytes)
  await outcome.value.close()
}

// Issue #2381: a read that stalls is not proof of damage. A stalled verify-on-open read of a stem
// another session pins fails the open typed and leaves the row, the pin and the file; a demotion
// would drop that pin and remove the file that session plays from.
async function aStalledVerifyKeepsAPinnedStem() {
  const stem = fixture("stalled-verify", 4096)
  const backend = new FakeOpfsBackend()
  const resolver = new MemoryStemResolver(stemMap([stem]))
  const opfs = store(backend, new FakeLockManager(), { tabId: "stalled-verify", readDeadlineMs: 20 })
  const pinned = await opfs.openSession({ sessionId: "T", stems: requirements([stem]), resolver })
  const requests = resolver.requests.length
  let stalls = 1
  backend.getFileBlocker = ({ name }) =>
    name === finalName(stem) && stalls-- > 0 ? never() : undefined
  const outcome = await settledWithin(
    opfs.openSession({ sessionId: "A", stems: requirements([stem]), resolver }),
    500
  )
  assert.notEqual(outcome, "timed-out")
  assert.ok(
    indexStems(backend)[stem.identity]?.pins.includes("session:stalled-verify:T") &&
      resolver.requests.length === requests,
    "a stalled verify demoted a pinned stem"
  )
  assert.equal(outcome.error?.code, "storage.read_stalled")
  assert.equal((await pinned.read(stem.identity)).size, stem.bytes)
  await pinned.close()
}

// Issue #2381: a final no session pins whose reads always stall heals, as it did on base: the stalled
// verify-on-open, or a stalled verify(), demotes it, and the next open re-ingests it.
async function aStallingUnpinnedFinalHeals() {
  for (const leg of ["open", "verify"]) {
    const stem = fixture(`stalling-${leg}`, 4096)
    const backend = new FakeOpfsBackend()
    const resolver = new MemoryStemResolver(stemMap([stem]))
    const opfs = store(backend, new FakeLockManager(), { tabId: "stalling", readDeadlineMs: 20 })
    await (await opfs.openSession({ sessionId: "seed", stems: requirements([stem]), resolver })).close()
    // Every read stalls until the final is removed.
    let broken = true
    backend.getFileBlocker = ({ name }) => (name === finalName(stem) && broken ? never() : undefined)
    backend.removeEntryBlocker = ({ name }) => {
      if (name === finalName(stem)) broken = false
    }
    if (leg === "verify") {
      const verified = await settledWithin(opfs.verify(stem.identity), 500)
      assert.notEqual(verified, "timed-out")
      assert.equal(verified.value, false, "a stalled verify() kept an unpinned final it cannot read")
    }
    const opened = await settledWithin(
      opfs.openSession({ sessionId: "after", stems: requirements([stem]), resolver }),
      1_000
    )
    assert.notEqual(opened, "timed-out")
    assert.equal(opened.error, undefined, `a stalling unpinned final never healed (${leg})`)
    assert.equal((await opened.value.read(stem.identity)).size, stem.bytes)
    await opened.value.close()
  }
}

// Issue #2381: a stalled read() of a stem another session pins fails typed and keeps the row and
// the file, so the other session still reads.
async function aStalledReadKeepsAStemAnotherSessionPins() {
  const stem = fixture("stalled-read", 4096)
  const backend = new FakeOpfsBackend()
  const resolver = new MemoryStemResolver(stemMap([stem]))
  const opfs = store(backend, new FakeLockManager(), { tabId: "stalled-read", readDeadlineMs: 20 })
  const a = await opfs.openSession({ sessionId: "a", stems: requirements([stem]), resolver })
  const b = await opfs.openSession({ sessionId: "b", stems: requirements([stem]), resolver })
  let stalls = 1
  backend.getFileBlocker = ({ name }) =>
    name === finalName(stem) && stalls-- > 0 ? never() : undefined
  const read = await settledWithin(a.read(stem.identity), 500)
  assert.notEqual(read, "timed-out")
  assert.equal(read.error?.code, "storage.read_stalled")
  assert.ok(
    backend.has(`${STORE_FOLDER}/${finalName(stem)}`) &&
      indexStems(backend)[stem.identity]?.pins.includes("session:stalled-read:b"),
    "a stalled read demoted a stem another session pins"
  )
  assert.equal((await b.read(stem.identity)).size, stem.bytes)
  await a.close()
  await b.close()
}

// Issue #2381: a holdless removal (a read failure's demotion) keeps the stem lock until the removal
// settles. A later wait for that lock, here verify(), waits for it rather than running, and the
// wait ends at its own abort; once the removal settles the lock is free again, past the abandoned
// place in the no-Web-Locks queue too.
async function aHoldlessRemovalKeepsTheStemLockUntilItSettles() {
  for (const leg of ["Web Locks", "no Web Locks"]) {
    const stem = fixture(`holdless-removal-${leg}`, 4096)
    const backend = new FakeOpfsBackend()
    const resolver = new MemoryStemResolver(stemMap([stem]))
    const locks = leg === "Web Locks" ? new FakeLockManager() : {}
    const opfs = store(backend, locks, { tabId: "holdless-removal", readDeadlineMs: 20 })
    const lease = await opfs.openSession({ sessionId: "s", stems: requirements([stem]), resolver })
    backend.getFileBlocker = ({ name }) => {
      if (name !== finalName(stem)) return undefined
      backend.getFileBlocker = undefined
      const failure = new Error("transient read failure")
      failure.name = "NotReadableError"
      throw failure
    }
    let land
    backend.removeEntryBlocker = ({ name }) =>
      name === finalName(stem) ? new Promise((resolve) => (land = resolve)) : undefined
    // The demotion's own wait is deadlined and best-effort, so the read still fails promptly.
    const read = await settledWithin(lease.read(stem.identity), 500)
    assert.notEqual(read, "timed-out")
    assert.equal(read.error?.code, "stem.read.failed")
    assert.notEqual(land, undefined)
    const abort = new AbortController()
    let verified = false
    const verifying = settledWithin(opfs.verify(stem.identity, { signal: abort.signal }), 500)
    verifying.then(() => (verified = true))
    await new Promise((resolve) => setTimeout(resolve, 40))
    assert.equal(verified, false, `a verify ran while a holdless removal of its final was unsettled (${leg})`)
    abort.abort(new DOMException("Done", "AbortError"))
    const aborted = await verifying
    assert.notEqual(aborted, "timed-out", `a wait for a removing stem lock ignored its abort (${leg})`)
    assert.equal(aborted.error?.name, "AbortError")
    backend.removeEntryBlocker = undefined
    land()
    const after = await settledWithin(opfs.verify(stem.identity), 500)
    assert.notEqual(after, "timed-out", `the stem lock stayed taken after the removal settled (${leg})`)
    assert.equal(after.value, false)
    await lease.close()
  }
}

// Issue #2381: verify-on-open demotes only on what it read. A failed index write of its `lastUsedAt`
// stamp is not a read of the stem: it fails the open and never demotes the stem.
async function aFailedLastUsedWriteNeverDemotes() {
  const stem = fixture("last-used-write", 4096)
  let failOnce = false
  const backend = new FakeOpfsBackend({
    writeBlocker({ fileName }) {
      if (fileName !== "index.json" || !failOnce) return undefined
      failOnce = false
      throw new Error("index write failed")
    },
  })
  const resolver = new MemoryStemResolver(stemMap([stem]))
  const opfs = store(backend, new FakeLockManager(), { tabId: "last-used" })
  await (await opfs.openSession({ sessionId: "seed", stems: requirements([stem]), resolver })).close()
  const requests = resolver.requests.length
  failOnce = true
  const outcome = await settledWithin(
    opfs.openSession({ sessionId: "A", stems: requirements([stem]), resolver }),
    500
  )
  assert.notEqual(outcome, "timed-out")
  assert.ok(
    resolver.requests.length === requests && indexStems(backend)[stem.identity] !== undefined,
    "a failed lastUsedAt write demoted the stem"
  )
  assert.match(String(outcome.error), /index write failed/)
}

// Issue #2381: a failed fallback copy starts two removals of its final without awaiting them, the
// immediate one and the one after the late writable's abort. The stem lock stays held until both
// settle. Each leg leaves exactly one of them unsettled.
async function aFallbackCleanupHoldsTheStemLockUntilItsRemovalsSettle() {
  for (const leg of ["writable abort", "immediate removal"]) {
    const stem = fixture(`fallback-fence-${leg}`, 4096)
    const final = finalName(stem)
    let unwedge
    const wedge = () => new Promise((resolve) => (unwedge = resolve))
    let finalRemovals = 0
    const backend = new FakeOpfsBackend({
      moveSupported: false,
      // The writable never opens until released, so its abort and second removal wait for it.
      createWritableBlocker: ({ fileName }) =>
        leg === "writable abort" && fileName === final ? wedge() : undefined,
      writeBlocker: ({ fileName }) =>
        leg === "immediate removal" && fileName === final ? never() : undefined,
      // Only the first removal of the final, the immediate one, wedges.
      removeEntryBlocker: ({ name }) => {
        if (leg !== "immediate removal" || name !== final) return undefined
        finalRemovals += 1
        return finalRemovals === 1 ? wedge() : undefined
      },
    })
    const locks = new FakeLockManager()
    const opfs = store(backend, locks, { tabId: "fallback-fence", readDeadlineMs: 20 })
    await assert.rejects(
      opfs.openSession({
        sessionId: "fence",
        stems: requirements([stem]),
        resolver: new MemoryStemResolver(stemMap([stem])),
      }),
      (error) => error.code === "storage.write_stalled"
    )
    const stemLock = `${STORE_LOCKS}:ingest:${stem.identity.slice("blake3:".length)}`
    const isHeld = async () => (await locks.query()).held.some((lock) => lock.name === stemLock)
    for (let step = 0; step < 5; step += 1) await tick()
    assert.ok(await isHeld(), `a fallback cleanup released the stem lock before its ${leg} settled`)
    unwedge()
    await until(async () => !(await isHeld()), `the stem lock's release after the ${leg}`)
  }
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
  const finalPath = `miso-stems-blake3-v1/blake3-${stem.identity.slice(7)}`
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
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
  )
  assert.deepEqual(index.stems[stem.identity].pins, [])
}

async function quotaFailureKeepsSurvivors() {
  const evictableStem = fixture("quota-unrelated-evictable", 2048)
  const firstStem = fixture("quota-first", 4096)
  const secondStem = fixture("quota-second", 4096)
  const shortfallBytes = 37
  const backend = new FakeOpfsBackend({ quota: 100_000, reportedQuota: 100_000 })
  const locks = new FakeLockManager()
  let firstPromoted
  const firstIsPromoted = new Promise((resolve) => { firstPromoted = resolve })
  const opfs = store(backend, locks, {
    tabId: "quota",
    hooks: {
      afterIndex({ identity }) {
        if (identity !== firstStem.identity) return
        backend.quota = backend.usage + secondStem.bytes - shortfallBytes
        firstPromoted()
      },
    },
  })
  const unrelated = await opfs.openSession({
    sessionId: "unrelated",
    stems: requirements([evictableStem]),
    resolver: new MemoryStemResolver(stemMap([evictableStem])),
  })
  await unrelated.close()

  const memory = new MemoryStemResolver(stemMap([firstStem, secondStem]))
  const resolver = {
    requests: memory.requests,
    async resolve(identity, options) {
      if (identity === secondStem.identity) await firstIsPromoted
      return memory.resolve(identity, options)
    },
  }
  const gate = new StemSessionGate(opfs, resolver)
  const progress = []
  await assert.rejects(
    gate.open({
      sessionId: "write-race",
      stems: requirements([firstStem, secondStem]),
      resume() {},
      onProgress(event) { progress.push(event) },
    }),
    (error) => {
      assert.equal(error.code, "storage.insufficient")
      assert.equal(error.details.requiredBytes, secondStem.bytes)
      assert.ok(
        error.details.availableBytes >= secondStem.bytes,
        "the eval must preserve the stale estimate/write race"
      )
      assert.equal(
        error.details.shortfallBytes,
        1,
        "write-time quota race reported a zero shortfall"
      )
      assert.equal(error.details.shortfallIsLowerBound, true)
      assert.equal(
        error.details.evictableBytes,
        evictableStem.bytes,
        "write-time quota accounting counted an opening-session survivor as evictable"
      )
      return true
    }
  )
  assert.equal(backend.quotaFailures, 1, "later stem must fail inside the OPFS write")
  assert.equal(gate.state, "refused", "quota failure must keep the gate closed")
  assert.equal(progress.some((event) => event.stage === "interactive"), false)
  assert.equal(
    progress.some(
      (event) => event.stage === "promoted" && event.identity === firstStem.identity
    ),
    true,
    "an earlier opening-session stem must promote before the later write fails"
  )
  assert.equal(
    progress.some(
      (event) => event.stage === "promoted" && event.identity === secondStem.identity
    ),
    false
  )
  assert.deepEqual(backend.names("miso-stems-blake3-v1/staging"), [])
  const failedIndex = JSON.parse(
    new TextDecoder().decode(backend.bytes("miso-stems-blake3-v1/index.json"))
  )
  assert.deepEqual(
    failedIndex.stems[firstStem.identity].pins,
    [],
    "survivor protection must precede durable session-pin installation"
  )
  assert.equal(await opfs.verify(firstStem.identity), true, "promoted survivor remains")
  assert.equal(await opfs.verify(evictableStem.identity), true, "unrelated row remains reusable")
  assert.equal(await opfs.verify(secondStem.identity), false, "failing stem never promotes")

  backend.quota = 100_000
  backend.reportedQuota = 100_000
  await gate.open({
    sessionId: "write-race",
    stems: requirements([firstStem, secondStem]),
    resume() {},
  })
  assert.equal(
    resolver.requests.filter((identity) => identity === firstStem.identity).length,
    1,
    "retry must reuse the promoted opening-session survivor"
  )
  assert.equal(
    resolver.requests.filter((identity) => identity === secondStem.identity).length,
    2,
    "retry must resolve only the stem whose write failed"
  )
  await gate.close()
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

const STORE_FOLDER = "miso-stems-blake3-v1"
const STORE_LOCKS = `miso:stem-store:v1:${STORE_FOLDER}`
const tick = () => new Promise((resolve) => setTimeout(resolve, 0))

async function until(predicate, what, ticks = 1_000) {
  for (let tried = 0; !(await predicate()); tried += 1) {
    if (tried === ticks) throw new Error(`timed out waiting for ${what}`)
    await tick()
  }
}

function finalName(stem) {
  return `blake3-${stem.identity.slice("blake3:".length)}`
}

// Issue #2381: an eviction's file removal is deadlined, and no OPFS step of it holds the index lock,
// so a wedged removal cannot stall every later lease close behind the index.
async function aWedgedEvictionNeverHoldsTheIndex() {
  const victim = fixture("evict-wedged-victim", 4096)
  const held = fixture("evict-wedged-held", 4096)
  const incoming = fixture("evict-wedged-incoming", 4096)
  const backend = new FakeOpfsBackend()
  const resolver = new MemoryStemResolver(stemMap([victim, held, incoming]))
  const opfs = store(backend, new FakeLockManager(), { tabId: "evict-wedged", readDeadlineMs: 20 })
  await (
    await opfs.openSession({ sessionId: "victim", stems: requirements([victim]), resolver })
  ).close()
  const lease = await opfs.openSession({ sessionId: "held", stems: requirements([held]), resolver })
  // The unpinned victim fills the store, so the incoming open must evict it.
  backend.reportedQuota = backend.usage
  let reached = false
  backend.removeEntryBlocker = ({ name }) => {
    if (name !== finalName(victim)) return undefined
    reached = true
    return never()
  }
  const abort = new AbortController()
  const opening = opfs.openSession({
    sessionId: "incoming",
    stems: requirements([incoming]),
    resolver,
    signal: abort.signal,
  })
  await until(() => reached, "the eviction's removal")
  const closed = await settledWithin(lease.close(), 500)
  assert.notEqual(closed, "timed-out", "a lease close waited on an eviction's wedged removal")
  assert.equal(closed.error, undefined)
  // A removal has no deadline; the open's wait for it ends at the open's abort.
  abort.abort(new DOMException("Switched mix", "AbortError"))
  const aborted = await settledWithin(opening, 500)
  assert.notEqual(aborted, "timed-out", "an eviction's wait for its removal ignored the open's abort")
  assert.equal(aborted.error?.name, "AbortError")
}

// Issue #2381: an eviction keeps the victim's stem lock until its removal settles, even once its own
// open stopped waiting at an abort. An open of the victim's own stem started meanwhile, from another
// tab or from this store, must not verify and pin the file the removal is about to delete: it waits
// for the lock, then finds a miss and re-ingests.
async function anEvictionHoldsTheStemLockUntilItsRemovalSettles() {
  for (const leg of ["another tab", "same tab", "same tab, no Web Locks"]) {
    const victim = fixture("late-evict-victim", 4096)
    const incoming = fixture("late-evict-incoming", 4096)
    const backend = new FakeOpfsBackend()
    // Without Web Locks the store falls back to its own queue, which only one store shares.
    const locks = leg === "same tab, no Web Locks" ? {} : new FakeLockManager()
    const resolver = new MemoryStemResolver(stemMap([victim, incoming]))
    const opfs = store(backend, locks, { tabId: "late-evict", readDeadlineMs: 20 })
    const other =
      leg === "another tab" ? store(backend, locks, { tabId: "late-evict-b", readDeadlineMs: 20 }) : opfs
    await (
      await opfs.openSession({ sessionId: "seed", stems: requirements([victim]), resolver })
    ).close()
    // The other store's first open, and its debris sweep, run before the eviction starts.
    await other.open()
    backend.reportedQuota = backend.usage
    let land
    backend.removeEntryBlocker = ({ name }) =>
      name === finalName(victim) ? new Promise((resolve) => (land = resolve)) : undefined
    const abort = new AbortController()
    const evicting = opfs.openSession({
      sessionId: "A",
      stems: requirements([incoming]),
      resolver,
      signal: abort.signal,
    })
    await until(() => land !== undefined, "the eviction's removal")
    let reopened = false
    const reopening = settledWithin(
      other.openSession({ sessionId: "B", stems: requirements([victim]), resolver }),
      1_000
    )
    reopening.then(() => (reopened = true))
    abort.abort(new DOMException("Switched mix", "AbortError"))
    await assert.rejects(evicting, (error) => error.name === "AbortError")
    await new Promise((resolve) => setTimeout(resolve, 40))
    assert.equal(
      reopened,
      false,
      `an open verified a final that an eviction was still removing (${leg})`
    )
    backend.removeEntryBlocker = undefined
    land()
    const lease = await reopening
    assert.notEqual(lease, "timed-out", `the stem lock stayed taken after the removal settled (${leg})`)
    assert.equal((await lease.value.read(victim.identity)).size, victim.bytes)
    await lease.value.close()
  }
}

// Issue #2381: opening the index file is deadlined like reading it, and a stalled index read fails
// typed instead of rebuilding the index. A rebuild after a stall would do more OPFS work under the
// lock against a store that just stalled, and would drop every live pin.
async function aWedgedIndexReadFailsTypedAndKeepsEveryPin() {
  const closing = fixture("index-wedged-closing", 4096)
  const kept = fixture("index-wedged-kept", 4096)
  const backend = new FakeOpfsBackend()
  const resolver = new MemoryStemResolver(stemMap([closing, kept]))
  const opfs = store(backend, new FakeLockManager(), { tabId: "index-wedged", readDeadlineMs: 20 })
  const lease = await opfs.openSession({ sessionId: "closing", stems: requirements([closing]), resolver })
  const other = await opfs.openSession({ sessionId: "kept", stems: requirements([kept]), resolver })
  // Only the read side wedges; a rebuild's index write would still land.
  backend.getFileHandleBlocker = ({ name, create }) =>
    name === "index.json" && !create ? never() : undefined
  const closed = await settledWithin(lease.close(), 500)
  assert.notEqual(closed, "timed-out", "a lease close waited on a wedged index open")
  assert.deepEqual(
    indexStems(backend)[kept.identity].pins,
    ["session:index-wedged:kept"],
    "a stalled index read rebuilt the index and dropped a live pin"
  )
  assert.equal(closed.error?.code, "storage.read_stalled")
  backend.getFileHandleBlocker = undefined
  await other.close()
}

// Issue #2381: a rebuild of a damaged index runs under the index lock too, so its directory listing
// and its removal of a final that fails to self-verify are deadlined as well.
async function aWedgedIndexRebuildNeverHoldsTheIndex() {
  for (const leg of ["listing", "removal", "hash"]) {
    const pinned = fixture(`rebuild-wedged-${leg}`, 4096)
    const unverifiable = fixture(`rebuild-wedged-${leg}-corrupt`, 4096)
    const backend = new FakeOpfsBackend()
    const locks = new FakeLockManager()
    const resolver = new MemoryStemResolver(stemMap([pinned, unverifiable]))
    const opfs = store(backend, locks, { tabId: "rebuild-wedged", readDeadlineMs: 20 })
    await (
      await opfs.openSession({ sessionId: "seed", stems: requirements([unverifiable]), resolver })
    ).close()
    const lease = await opfs.openSession({ sessionId: "rebuild", stems: requirements([pinned]), resolver })
    backend.setBytes(`${STORE_FOLDER}/index.json`, encoder.encode("{"))
    const corrupted = new Uint8Array(unverifiable.pcm)
    corrupted[3] ^= 1
    backend.setBytes(`${STORE_FOLDER}/${finalName(unverifiable)}`, corrupted)
    const folder = backend.root.entries.get(STORE_FOLDER)
    if (leg === "listing") {
      backend.entriesBlocker = ({ node }) => (node === folder ? never() : undefined)
    } else if (leg === "removal") {
      backend.removeEntryBlocker = ({ name }) =>
        name === finalName(unverifiable) ? never() : undefined
    } else {
      // A slow device: the good pinned final's first read stalls.
      let stalls = 1
      backend.getFileBlocker = ({ name }) =>
        name === finalName(pinned) && stalls-- > 0 ? never() : undefined
    }
    const closed = await settledWithin(lease.close(), 500)
    assert.notEqual(closed, "timed-out", `a lease close waited on a rebuild's wedged ${leg}`)
    if (leg === "removal") {
      const stemLock = `${STORE_LOCKS}:ingest:${unverifiable.identity.slice("blake3:".length)}`
      assert.ok(
        (await locks.query()).held.some((lock) => lock.name === stemLock),
        "a rebuild released a final's stem lock before its removal settled"
      )
    }
    if (leg === "hash") {
      assert.ok(
        backend.has(`${STORE_FOLDER}/${finalName(pinned)}`),
        "a rebuild removed a good final whose hash stalled"
      )
    }
    assert.equal(
      closed.error?.code,
      leg === "removal" ? "storage.remove_stalled" : "storage.read_stalled"
    )
  }
}

// Issue #2381: a removal of a final made under the index lock (the open-time sweep's here, and a
// rebuild's through the same code) takes the final's stem lock without waiting, and skips a final
// whose lock another holder has. The sweep's liveLocks snapshot alone is stale: here the holder
// takes the lock after it, while the store is listed.
async function aSweepNeverRemovesAFinalWhoseStemLockIsTaken() {
  const stem = fixture("locked-final", 4096)
  const backend = new FakeOpfsBackend()
  const locks = new FakeLockManager()
  await (
    await store(backend, locks, { tabId: "locked-seed" }).openSession({
      sessionId: "seed",
      stems: requirements([stem]),
      resolver: new MemoryStemResolver(stemMap([stem])),
    })
  ).close()
  // A valid index with no row makes the final promote debris.
  backend.setBytes(`${STORE_FOLDER}/index.json`, encoder.encode('{"version":1,"stems":{}}\n'))
  const stemLock = `${STORE_LOCKS}:ingest:${stem.identity.slice("blake3:".length)}`
  const folder = backend.root.entries.get(STORE_FOLDER)
  const held = {}
  backend.entriesBlocker = async ({ node }) => {
    if (node !== folder || held.done !== undefined) return
    held.done = locks.request(stemLock, {}, () => new Promise((resolve) => (held.release = resolve)))
    await until(() => held.release !== undefined, "the held stem lock")
  }
  await store(backend, locks, { tabId: "locked-open" }).open()
  assert.ok(
    backend.has(`${STORE_FOLDER}/${finalName(stem)}`),
    "a final was removed while another holder had its stem lock"
  )
  held.release()
  await held.done
}

// Issue #2381: an aborted open evicts nothing once its abort is seen, wherever its eviction waits:
// queued on the victim's stem lock, queued on the index lock, or in the index read under it. The
// waits on the two locks end at the abort, so the open refuses without waiting for their holders.
async function anAbortedOpenEvictsNothing() {
  for (const leg of ["stem lock", "index lock", "index read"]) {
    const victim = fixture("aborted-evict-victim", 4096)
    const incoming = fixture("aborted-evict-incoming", 4096)
    const backend = new FakeOpfsBackend()
    const locks = new FakeLockManager()
    const resolver = new MemoryStemResolver(stemMap([victim, incoming]))
    const opfs = store(backend, locks, { tabId: "aborted-evict" })
    await (
      await opfs.openSession({ sessionId: "victim", stems: requirements([victim]), resolver })
    ).close()
    backend.reportedQuota = backend.usage
    const hold = async (name) => {
      const held = {}
      held.done = locks.request(name, {}, () => new Promise((resolve) => (held.release = resolve)))
      await until(() => held.release !== undefined, `the held ${name}`)
      return held
    }
    const queued = (name) =>
      until(
        async () => (await locks.query()).pending.some((lock) => lock.name === name),
        `a queued ${name} request`
      )
    const stemLock = `${STORE_LOCKS}:ingest:${victim.identity.slice("blake3:".length)}`
    const indexLock = `${STORE_LOCKS}:index`
    const stemHold = await hold(stemLock)
    const abort = new AbortController()
    let settled = false
    const opening = settledWithin(
      opfs.openSession({
        sessionId: "incoming",
        stems: requirements([incoming]),
        resolver,
        signal: abort.signal,
      }),
      1_000
    ).finally(() => (settled = true))
    await queued(stemLock)
    let release = () => stemHold.release()
    if (leg === "index lock") {
      const indexHold = await hold(indexLock)
      stemHold.release()
      await queued(indexLock)
      release = () => indexHold.release()
    } else if (leg === "index read") {
      let unwedge
      backend.getFileHandleBlocker = ({ name, create }) =>
        name === "index.json" && !create
          ? new Promise((resolve) => (unwedge = resolve))
          : undefined
      stemHold.release()
      await until(() => unwedge !== undefined, "the eviction's index read")
      release = () => unwedge()
    }
    abort.abort(new DOMException("Switched mix", "AbortError"))
    for (let step = 0; step < 20 && !settled; step += 1) await tick()
    const settledWhileHeld = settled
    release()
    backend.getFileHandleBlocker = undefined
    const outcome = await opening
    assert.notEqual(outcome, "timed-out", `the aborted open never settled (${leg})`)
    assert.equal(outcome.error?.name, "AbortError", leg)
    for (let step = 0; step < 5; step += 1) await tick()
    assert.ok(
      indexStems(backend)[victim.identity] !== undefined &&
        backend.has(`${STORE_FOLDER}/${finalName(victim)}`),
      `an aborted open evicted a stem (${leg})`
    )
    if (leg !== "index read") {
      assert.ok(settledWhileHeld, `the aborted open waited for the ${leg} holder`)
    }
  }
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
  assert.equal(backend.names("miso-stems-blake3-v1/staging").length, 1)
  await store(backend, locks, { tabId: "replacement" }).open()
  assert.deepEqual(backend.names("miso-stems-blake3-v1/staging"), [])
  assert.equal(
    backend.names("miso-stems-blake3-v1").filter((name) => name.startsWith("blake3-")).length,
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
  assert.deepEqual(backend.names("miso-stems-blake3-v1/staging"), [])
  assert.equal(
    backend.names("miso-stems-blake3-v1").some((name) => name.startsWith("blake3-")),
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
  assert.deepEqual(abortBackend.names("miso-stems-blake3-v1/staging"), [])

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
  assert.deepEqual(deadlineBackend.names("miso-stems-blake3-v1/staging"), [])
}

// STEM_IDENTITY_V1 §4: the preimage length is shape-derived and never enters
// the hash, so a declaration that lies about `bytes` is the one corruption a
// digest match cannot catch. Every arm that compares an observed length with
// the declaration is load-bearing; these narratives reach them in store order,
// fallback-final first, so that each arm is the only one left holding.
async function lyingDeclarationIsRefused() {
  const stem = fixture("lying-declaration", 8192)
  const finalPath = `miso-stems-blake3-v1/blake3-${stem.identity.slice(7)}`
  const lied = { identity: stem.identity, bytes: stem.bytes + 64 }

  const fallbackBackend = new FakeOpfsBackend({ moveSupported: false })
  const fallbackResolver = new MemoryStemResolver(stemMap([stem]), {
    chunkBytes: 1024,
  })
  const fallback = store(fallbackBackend, new FakeLockManager(), {
    tabId: "lying-fallback",
  })
  await assert.rejects(
    fallback.openSession({
      sessionId: "lying-fallback",
      stems: [lied],
      resolver: fallbackResolver,
    }),
    (error) => {
      assert.equal(error.code, "stem.ingest.integrity")
      assert.equal(error.details.identity, stem.identity)
      return true
    },
    "a lying declaration must never survive fallback promotion"
  )
  assert.equal(fallbackBackend.has(finalPath), false, "no final may be adopted")
  assert.deepEqual(fallbackBackend.names("miso-stems-blake3-v1/staging"), [])
  assert.deepEqual(indexStems(fallbackBackend), {})

  const backend = new FakeOpfsBackend()
  const resolver = new MemoryStemResolver(stemMap([stem]), { chunkBytes: 1024 })
  const opfs = store(backend, new FakeLockManager(), { tabId: "lying-fresh" })
  const progress = []
  await assert.rejects(
    opfs.openSession({
      sessionId: "lying-fresh",
      stems: [lied],
      resolver,
      onProgress(event) {
        progress.push(event)
      },
    }),
    (error) => {
      assert.equal(error.code, "stem.ingest.integrity")
      assert.equal(error.details.identity, stem.identity)
      assert.equal(error.details.expectedBytes, lied.bytes)
      assert.equal(error.details.observedBytes, stem.bytes)
      assert.equal(error.details.expectedBlake3, stem.identity.slice(7))
      return true
    },
    "a lying declaration must never be promoted"
  )
  assert.deepEqual(
    progress.filter((event) => event.stage === "verified"),
    [],
    "a lying declaration is refused before the pre-promote reopen"
  )
  assert.deepEqual(
    progress.filter((event) => event.stage === "promoted"),
    []
  )
  assert.equal(backend.has(finalPath), false, "a refused ingest indexes nothing")
  assert.deepEqual(backend.names("miso-stems-blake3-v1/staging"), [])
  assert.deepEqual(indexStems(backend), {})
  assert.equal(resolver.requests.length, 2, "the typed integrity refusal retries once")

  const warmBackend = new FakeOpfsBackend()
  const warmLocks = new FakeLockManager()
  const seeded = store(warmBackend, warmLocks, { tabId: "lying-seed" })
  const seedLease = await seeded.openSession({
    sessionId: "seed",
    stems: requirements([stem]),
    resolver: new MemoryStemResolver(stemMap([stem])),
  })
  await seedLease.close()
  assert.equal(warmBackend.has(finalPath), true)
  assert.equal(indexStems(warmBackend)[stem.identity].bytes, stem.bytes)

  const reopenResolver = new MemoryStemResolver(stemMap([stem]))
  const gate = new StemSessionGate(
    store(warmBackend, warmLocks, { tabId: "lying-reopen" }),
    reopenResolver
  )
  const reopenProgress = []
  await assert.rejects(
    gate.open({
      sessionId: "lying-reopen",
      stems: [lied],
      resume() {},
      onProgress(event) {
        reopenProgress.push(event)
      },
    }),
    (error) => {
      assert.equal(error.code, "stem.ingest.integrity")
      return true
    },
    "verify-on-open must demote a lying declaration to a miss"
  )
  assert.equal(gate.state, "refused")
  assert.deepEqual(
    reopenProgress.filter((event) => event.stage === "interactive"),
    [],
    "a lying declaration never opens the interaction gate"
  )
  assert.ok(
    reopenProgress.some(
      (event) =>
        event.stage === "corrupt" &&
        event.identity === stem.identity &&
        event.expectedBytes === lied.bytes &&
        event.observedBytes === stem.bytes
    ),
    "verify-on-open must report the byte-length mismatch as corruption"
  )
  assert.equal(reopenResolver.requests.length, 2, "the demoted stem re-ingests and is refused")
  assert.equal(warmBackend.has(finalPath), false, "the demoted final is removed")
  assert.deepEqual(indexStems(warmBackend), {})
  await gate.close()

  // The mirror lie: a declaration shorter than the digest-named content is
  // contradicted mid-stream, so staging must never fill to the stream's end
  // before the same typed refusal.
  const short = { identity: stem.identity, bytes: stem.bytes - 64 }
  const overBackend = new FakeOpfsBackend()
  const overResolver = new MemoryStemResolver(stemMap([stem]), { chunkBytes: 1024 })
  const over = store(overBackend, new FakeLockManager(), { tabId: "lying-over" })
  const overProgress = []
  await assert.rejects(
    over.openSession({
      sessionId: "lying-over",
      stems: [short],
      resolver: overResolver,
      onProgress(event) {
        overProgress.push(event)
      },
    }),
    (error) => {
      assert.equal(error.code, "stem.ingest.integrity")
      assert.equal(error.details.identity, stem.identity)
      assert.equal(error.details.expectedBytes, short.bytes)
      assert.ok(error.details.observedBytes > short.bytes)
      return true
    },
    "an over-length delivery must never be promoted"
  )
  const staged = overProgress
    .filter((event) => event.stage === "hashed")
    .map((event) => event.bytes)
  assert.ok(staged.length > 0, "the over-length ingest must report chunk progress")
  assert.ok(
    Math.max(...staged) <= short.bytes,
    "over-length delivery is refused before staging fills past the declaration"
  )
  assert.equal(overBackend.has(finalPath), false)
  assert.deepEqual(overBackend.names("miso-stems-blake3-v1/staging"), [])
  assert.deepEqual(indexStems(overBackend), {})
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
  budgetAssert(coldMs < 5_000, `fixture cold-open budget exceeded: ${coldMs}ms`)
  budgetAssert(verifyMs < 2_000, `fixture verify-open budget exceeded: ${verifyMs}ms`)
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

await twoMixNarrative()
await twoTabCollision()
await wedgedDemoteDeleteIsBounded()
await lateIndexWriteCannotOverwriteANewerOne()
await unwritableStoreRefusesBeforeNetwork()
await openSurvivesFailingSweeps()
await indexRecoveryAdoptsSelfVerifyingFinals()
await promoteAtomicityAndCrashSweep()
await fallbackWritesObeyAbortAndDeadline()
await openerDoesNotSweepActivePromote()
await crashedSessionPinsAreRecoverable()
await quotaFailureKeepsSurvivors()
await lruEvictsOnlyUnpinned()
await aWedgedEvictionNeverHoldsTheIndex()
await anEvictionHoldsTheStemLockUntilItsRemovalSettles()
await aWedgedIndexReadFailsTypedAndKeepsEveryPin()
await aWedgedIndexRebuildNeverHoldsTheIndex()
await aSweepNeverRemovesAFinalWhoseStemLockIsTaken()
await aFallbackCleanupHoldsTheStemLockUntilItsRemovalsSettle()
await aHoldlessDemoteLeavesAFinalAnotherHoldOwns()
await aHoldNeverWritesAFinalItIsStillRemoving()
await aStalledVerifyKeepsAPinnedStem()
await aStallingUnpinnedFinalHeals()
await aStalledReadKeepsAStemAnotherSessionPins()
await aHoldlessRemovalKeepsTheStemLockUntilItSettles()
await aFailedLastUsedWriteNeverDemotes()
await anAbortedOpenEvictsNothing()
await tabCloseMidStaging()
await abortCleansStaging()
await wedgedDecoderObeysAbortAndDeadline()
await lyingDeclarationIsRefused()
// After lyingDeclarationIsRefused: with both verify-on-open length arms
// mutated away, the lying-declaration gate must be the one that goes red.
await corruptionSelfHeals()
await storageUnavailable()
await latencyRows()
await modeDetection()
process.stdout.write(
  `stem-store-core-v1: PASS (budget asserts skipped: ${skippedBudgetAsserts})\n`
)
