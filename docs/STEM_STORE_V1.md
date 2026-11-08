# Stem store V1

This document specifies the standalone web-adapter store from issue #244. The
engine remains storage-blind: no engine crate knows OPFS, URLs, fetch, or
Workers. Final engine/SDK boot wiring waits for #243; these modules are tested
with canonical-PCM fixtures and a structural ring consumer.

## Resolver boundary

`StemResolver.resolve(identity)` returns a bounded `ReadableStream<Uint8Array>`
whose bytes are the canonical-PCM serialization specified by #241. Identity to
URL policy is supplied when the web resolver is constructed; a session carries
no locator. `MemoryStemResolver` is the environment-neutral conformance arm.
`FetchStemResolver` owns resumable range fetch and delegates the provenance-
pinned lossless decode to #245's injected decoder.

## Layout and trust

The construction parameter defaults to `miso-stems-blake3-v1`. This clean prelaunch namespace
ensures an obsolete identity index can never retain unreachable finals:

```text
miso-stems-blake3-v1/
  blake3-<64 lowercase hex>       canonical PCM, and nothing else
  staging/<tab-id>-<hex>          untrusted in-flight bytes
  index.json                      {bytes,lastUsedAt,pins[]} per identity
```

The stem directory is one artifact namespace, not a generic content store.
Third-party effect packages (CID identities) are out of scope until a new issue
reopens them (owner ruling R6a, #1037); such an issue must give them a sibling
namespace that never shares stem filenames or index rows. The `blake3:` scheme
prefix selects the stem namespace before resolution.

Only indexed final files are playable. A move is index-last: decoded bytes are
hashed while written, the staging file is reopened and fully verified, then a
successful `FileSystemFileHandle.move(destination, name)` atomically changes
its name. The P-2 extension observed this exact staging-to-parent move on
Chromium 151. When move is absent or explicitly unsupported,
the adapter copies to the final name, reopens it, fully hashes it, and inserts
the index row last. A crash can therefore leave staging or an unindexed final,
but never a trusted partial; open sweeps both after consulting live Web Locks.

Index parsing is crash-only. A structurally invalid index is rebuilt by scanning
and hashing the self-named files; invalid files are discarded. Active session
pins hold Web Locks, so a later tab removes crash-stale session pins without
touching a live tab or a durable user offline pin.

## Verify-on-open invariant

The final owner ruling on #244 overrides N-14:

> interactive implies every referenced stem was fully hashed at this open.

Every indexed hit is streamed through incremental BLAKE3-256 inside the loading
gate. The same pass counts bytes, so it also validates the document-derived
`frames × channels × bytes_per_sample(bit_depth)` requirement, where
`bytes_per_sample` is STEM_IDENTITY_V1's closed table: 2 for `16`, 3 for `24`,
and 4 for `32f`. A truncation, same-length
bit flip, missing file, read failure, or shape-length mismatch demotes the row
to a miss and automatically invokes the resolver. The session refuses only if
self-healing cannot re-ingest (offline, decoder, integrity, or quota failure).
`store.verify()` remains a maintenance operation for unreferenced-file audits;
it is not the correctness boundary for referenced stems.

## Concurrency, cancellation, and failure floor

The per-hash Web Lock is the cross-tab single flight. A waiter rechecks the
indexed file after it acquires the lock, so one tab downloads while every joiner
uses the promoted result. Per-tab staging names remain distinct as a second
line of defense. Switching mixes aborts outstanding resolver streams and
removes their staging entries; already promoted content remains reusable.
Store open queries held and pending locks before removing crash debris.
Reopening the currently interactive session under the same session ID first
closes its predecessor lease, because both leases name the same exclusive pin
lock; the replacement then verifies and installs exactly one fresh pin.

Playback reads use `getFile()` snapshots, never long-lived access handles.
Where a caller probes sync read-only mode, it trusts only
`accessHandle.mode === "read-only"`; option acceptance is not evidence because
WebKit ignores `mode` entirely and another engine may accept the option while
retaining exclusive behavior. The existing-file contention contract is deliberately
error-name-agnostic: **any** `createSyncAccessHandle()` rejection declines the
sync path and falls back to a `getFile()` snapshot. It must never match a named
exception: WebKit reports `InvalidStateError`, while Chromium reports
`NoModificationAllowedError`. Real-device probes confirm that the fallback can
read flushed bytes while the writer remains held. A read failure during
playback releases every ring and reports a typed session error; it never turns
damage into silent audio.

The real-device P-2 two-tab harness must foreground both iOS Safari tabs at
least once before coordinating them; otherwise iOS tab throttling can make a
contention cell appear stalled. This is a harness prerequisite, not a store
semantic or a timeout relaxation.

Quota preflight counts the canonical bytes of misses only. Unpinned LRU rows
outside the opening session may be evicted; open-session and offline pins are
never victims. `storage.insufficient` reports required, available, shortfall,
and evictable bytes. Every identity in the currently opening session is already
protected from eviction and evictable-byte accounting before its durable
session pin is installed. Write-time quota failure is authoritative over a
racing `estimate()`: its shortfall is always a truthful nonzero value (marked as
a lower bound when the estimate is absent or still claims enough space). It
removes the failing staging file and any failing unindexed final, keeps earlier
promoted stems, keeps the gate closed, and permits a clean retry that reuses
those survivors.
The index is crash-only metadata: if it is missing or structurally corrupt, the
store scans final names, fully hashes each unlocked candidate, adopts an exact
`hash(file) == filename` match with empty recovered pins, and removes candidates
that cannot self-verify. With a structurally valid index, an absent row still
means the unindexed final is promote debris and the sweep removes it. A stalled
index read is not damage: it fails typed `storage.read_stalled` and never
rebuilds, because a rebuild resets every pin and would do more work against a
store that just stalled (#2381).
An environment without `navigator.storage.getDirectory` refuses typed
`storage.unavailable`; there is deliberately no RAM-resident degraded mode.

WebKit coverage is a stated asymmetry (owner ruling on #284, 2026-11-04). Playwright WebKit on
Linux, the CI stand-in for Safari, has no `navigator.storage.getDirectory`, so CI runs no
stem-store browser gate on WebKit and the store refuses there with `storage.unavailable`. The
store logic is covered in Node with fakes (`hosts/host-web/tests/stem-store-core-v1.mjs`), and
real-device Safari probes are the WebKit evidence. No macOS CI leg is added for it.

The local-store read/write deadline is 15 seconds. Resolver establishment,
network delivery, and decoded-stream output use the 30-second ingest
no-progress watchdog; both deadlines race the mix-switch abort signal, abandon
a wedged reader cancellation, and remove staging before refusing typed. Range
retries are accepted only when the server proves the requested `206
Content-Range`, so resumption can never duplicate bytes into the decoder.

The [authorized bounded #244 successor](https://github.com/misofm/engine/issues/244#issuecomment-5458432638)
applies that same abort/deadline race to fallback-final `createWritable`,
`write`, and `close`. On interruption, writable abort and final-file removal
start immediately but are not re-awaited: cleanup cannot turn the original
typed cancellation or `storage.write_stalled` refusal into another wedge. A
late writable settlement retries removal, and the unindexed-final sweep remains
the crash backstop.

Every OPFS step taken while holding the index lock has that local-store
deadline (#2381): opening, reading and writing the index, and a rebuild's
directory listing, hashing and removal of a final that cannot self-verify. On
expiry the step fails typed (`storage.read_stalled`, `storage.write_stalled` or
`storage.remove_stalled`) and the lock is released, so one wedged step cannot
stall every later index mutation, every lease close's pin release among them.
Under the index lock a stall is never treated as damage: a rebuild whose hash
of a final stalls fails typed and keeps that final, and a stalled index read
never rebuilds.

The Web Locks query that the first open and a rebuild make under the index lock
has the same deadline (#2396). A query that throws or makes no progress leaves
the lock state unknown, and a late answer is dropped. With the lock state
unknown, the open purges no session pin. Where Web Locks requests exist (the
same test that decides whether stem locks are cross-tab), its staging sweep also
removes nothing, even when the lock query itself is missing: under Web Locks a
staged file is dead only when its stem lock is free, and its age is no proof,
because a live download's writable need not update the file's modification time
before it closes. In a spec-compliant browser an open writable's File System
lock also makes the sweep's raw removal fail, but the store does not rely on
that lock, and the Node fakes do not model it. The sweep reports
`storage.maintenance_stalled` and the next open retries. The staged-file age
check (older than 30 seconds is dead) applies only with no Web Locks at all. The
finals sweep and a rebuild still run, because each removal of a final is proven
by taking its stem lock without waiting.

Outside the index lock, a read that stalls or fails is not proof of damage
either. In verify-on-open, `verify()` and a session's `read()`, such a read
demotes the stem only if, re-checked under the index lock, no session other
than the reading one pins it. Otherwise the call fails typed with the read's own
error (`storage.read_stalled` for a stall, `stem.read.failed` for a read error)
and the row and file stay. A final whose reads keep failing is therefore demoted
and re-ingested unless another session plays from it. A keep-offline pin does
not block that demotion and is lost with the row, as on base. A dead session pin
does block it until the pin is purged at a store open; without Web Locks nothing
purges a crashed tab's pin, so that stem stays unhealed. An
abort is not a failed read and never demotes. Corruption, a wrong length and a
missing file still demote unconditionally.

A final is removed only inside a hold of its stem lock, and the hold is released
only when the removal settles; a removal has no deadline. A caller that already
holds the stem lock removes inside its own hold. Every other caller (a rebuild,
the open-time sweep, a demotion after a read failure) takes the lock without
waiting, and leaves the final alone when the lock is held, by another tab or by
another hold in this store. How long a caller waits for its own removal is
separate from the lock: an eviction's open waits until the removal settles or
the open is aborted, a demotion waits at most the local-store deadline (it is
best-effort), and a rebuild or sweep under the index lock waits at most its
deadline and then fails typed. An ingest's own removals of a final, after a
quota error or after a fallback final fails verification, wait at most the
local-store deadline and are best-effort, like a demotion's (#2397); a retry
waits for a removal still in flight, as described below.
Eviction and demotion drop the row first, under the index lock, so no removal
runs under it. A failed fallback copy's unawaited removals hold the stem lock
the same way, and within one hold no final is written until every removal that
hold started has settled; that wait ends at the open's abort.

The other OPFS steps a stem-lock hold awaits have the local-store deadline as
well (#2397): the lookup of the final in verify-on-open and `verify()`, an
ingest's clearing, creation and cleanup of its staged file, the staging
writable's open, writes, close and abort, and the promote's `move()`,
fallback-final creation and fallback writable's writes and close. On expiry the
holder's call fails typed (`storage.read_stalled`, `storage.write_stalled` or
`storage.remove_stalled`). Every one of these steps except the lookup changes
the store and may still land after its deadline, so each is fenced into its
hold like a removal: the stem lock stays held until the step settles, and a
staging writable that opens after its wait ended is aborted before the lock is
released. A stalled clearing of the staged file ends the ingest. The staging
abort and cleanup are best-effort: their failure or stall never replaces the
attempt's own outcome, and a later ingest of the stem clears the staged file
first. An ingest that fails `stem.ingest.integrity` is attempted once more in
the same hold, and the retry runs only once every step that hold fenced has
settled. That wait has the local-store deadline; on expiry the open fails
`storage.write_stalled` and no retry runs, because a step still in flight could
land on the staged file or the final the retry writes. A step that rejected has
already settled, so it does not stop the retry. This also bounds a recheck
demotion's removal still in flight at retry time, which before waited until the
open's abort. A lookup lands nothing, so it is not fenced; a stalled or failed
lookup is a stalled or failed read under the rule above.

A wait for a stem lock, in this store or another tab, simply waits for the
holder; it ends when the lock is released or at the waiter's own abort. A
caller without a signal, such as `verify()` without one, can therefore wait as
long as a wedged removal does. Without Web Locks the store serialises its stem
locks with a local queue that honours the same signal and without-waiting
rules.

An eviction takes its open's signal on the victim's stem lock and on the index
lock, and checks it again after the index read, so an aborted open evicts
nothing once its abort is seen.

## Hash implementation and provenance

`incremental-blake3.js` is a repository-owned bounded-memory BLAKE3-256
implementation. Its exact source digest and rationale are pinned in
`incremental-blake3.provenance.json`; the gate verifies that pin plus official
empty, `abc`, million-`a`, and independent irregular-ending vectors. This is
shipped code: source identity uses BLAKE3-256, while unrelated package and
artifact integrity hashes retain their own algorithms.

## Loading gate and Worker pump

`StemSessionGate.open()` invokes the caller's `resume()` synchronously before
its first `await`, preserving the iOS user gesture, then holds interaction until
`OpfsStemStore.openSession()` returns its pinned lease. Superseding an open
aborts its fetch/decode work; its promoted stems survive. An open that a newer
open, `close()` or its own signal aborted never becomes the gate's session,
even when the abort lands after its lease was granted: it closes that lease and
rejects with the abort reason. Its waits for `resume()` and for the session pin
lock end at the same abort, so an aborted open never keeps its lease for a
resume that does not settle, nor waits on another holder of that pin. `close()`
reports `idle` at once, and an open started while the old lease closes owns the
state from then on (#2266).

A resolved close means a released pin (#2372). Every `StemSessionLease.close()`
call shares the first call's release, so a release that fails rejects that
call and every later one: a lease never reports a release that did not happen,
and `StemSessionGate.close()` rejects with the same error when the current
lease's release fails.

`StemSessionGate.close()` waits only for what holds a pin or can still take
one:
- the current lease's close;
- a same-session predecessor that an open took out of the gate and is closing;
- every open that has reached the store's pin phase, including one a newer open
  had already superseded.

An open reaches the pin phase when `openSession` passes its last signal check
before the session pin lock. From then on it may hold that lock or write its
pin, and it settles only after closing every lease it was granted or displaced.
Every wait it has from then on either ends at its abort or is the store's
index work, which a lease close waits for too. An open aborted before the pin
phase holds no pin and never takes one, because that signal check refuses it.
`close()` therefore does not wait for it, whatever store work it is still in:
the store's first open, the quota estimate, an ingest it joined, its own
staging cleanup, or an eviction. Those can be long or have no deadline. The
aborted open rejects whenever that work ends.

The PCM pump is a dedicated module Worker. MSB1 ring descriptors cross the
Worker port after construction; they are never processor options. The Worker
reads bounded Blob slices, de-interleaves interleaved int16/int24 little-endian
PCM, converts by exact powers of two (`2^15`, `2^23`), and writes the unchanged
MSB1 layout (its seek epoch is the parity seqlock of #2184: odd while a seek is
written, 2 steps per seek). The pump module contains no network API and its runtime eval runs
with a throwing network tripwire.

### Cadence: `selfDriving` (issue #278, opt-in)

The Worker is pull-driven by default and nothing in it schedules the next
`pump`, so a consumer that mounted the shipped module also had to re-implement a
cadence over its message vocabulary. `initialize` therefore accepts one optional
field, `selfDriving`: `true` for the default backoff, `{ idleMs }` for an
explicit one, absent or `false` for the unchanged pull-driven Worker. It adds no
message type and changes no reply.

While driving, the Worker calls `pumpUntilFull()` in a loop, sleeps `idleMs`
when a pass writes zero chunks (every ring is full, so only the render thread
consuming a slot can change the answer), and breaks when a pass reports
`finished`. A `seek` re-arms a loop that broke, because a seek moves the cursor
off end-of-region. The loop **posts nothing on the happy path**: a self-driven
pass answers no request, and a bare `pumped` notification would make one word
mean both a reply and a broadcast. The ring's own `endOfRegion` flag already
tells the reader where the region ends. A failure still reports on
`session-error`.

Each tick is enqueued onto the Worker's existing message tail rather than run
beside it. `pumpUntilFull()` and `seek()` both mutate the pump's per-source
cursor, window and generation, so a loop running concurrently with message
handling would interleave those mutations and commit PCM under the pre-seek
generation into a ring the reader has already re-tagged -- a race that reads as
a glitch rather than as an error. Borrowing the tail means a `seek` or `stop`
lands strictly between two ticks, and no second lock is needed. For the same
reason `stop` does not await the loop -- it runs *as* a link in that tail, so
awaiting would deadlock. It clears the loop's token and cancels the idle timer,
and the next tick sees the cleared token and exits.

### Failure (issue #2263)

The Worker's session is one phase: `idle`, `running` or `failed`. Any error,
from a message or from a cadence pass, answers `session-error`, stops the pump,
ends the cadence and cancels its idle timer, and moves a running session to
`failed`. A failed session holds no pump: `seek` and `pump` are answered
`session-error` with code `stem.pump.session_failed` (the first failure is in
its `details.failure`) until `initialize` starts a new session; `stop` returns
the Worker to `idle`. The pump itself refuses the same way: `seek` and
`pumpUntilFull` on a stopped `CanonicalPcmPump` reject with
`stem.pump.stopped` and publish nothing to its released rings, so a dead
producer passed to `BrowserEngine.seek` surfaces as `SessionSeekProducerError`
instead of a silent success.

### Documented future arm: `bit_depth = 32f` (not implemented)

STEM_IDENTITY_V1 admits the closed token set `{16, 24, 32f}`. The web launch
scope is deliberately integer-only: the pump validator types `bitDepth` as
`16 | 24` and refuses everything else, and the ingest/verify length checks
exercise only the 2- and 3-byte widths today. When the `32f` arm is built, it
changes no invariant in this document -- only the two tables below gain their
already-specified third row:

| `bit_depth` | bytes/sample | pump transform to planar f32 |
| ----------- | ------------ | ---------------------------- |
| `16` | 2 | signed int ÷ `2^15` |
| `24` | 3 | signed int ÷ `2^23` |
| `32f` (future) | 4 | identity bit-pattern copy: each little-endian 4-byte IEEE-754 binary32 pattern is reproduced exactly; NaN payloads and `-0.0` are preserved, never canonicalized |

Every length formula in this document already means
`frames × channels × bytes_per_sample(bit_depth)` with the 4-byte `32f` row
included; there is no formula for which `32f` needs a special case. Adding the
arm means widening the pump's `bitDepth` type and adding the copy branch --
nothing about hashing, verify-on-open, the ingest gate, or promotion changes,
because canonical bytes are canonical bytes at every depth.

## Latency evidence contract

The repeatable fixture eval records three numbers: cold open (ingest plus
verification), warm open (mandatory verify cost), and total canonical bytes.
Its deterministic 8-stem/2 MiB Node fixture budgets are 5 s cold and 2 s warm;
these are regression bounds, not product claims. The browser probe separately
records real OPFS ingest/hash throughput and verify cost for the #246 field
trend line.
