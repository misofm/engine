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

The construction parameter defaults to `miso-stems-v1`:

```text
miso-stems-v1/
  sha256-<64 lowercase hex>       canonical PCM, and nothing else
  staging/<tab-id>-<hex>          untrusted in-flight bytes
  index.json                      {bytes,lastUsedAt,pins[]} per identity
```

The stem directory is one artifact namespace, not a generic content store.
CID effect packages will use the declared sibling namespace
`miso-effect-packages-v1`; they never share stem filenames or index rows. The
scheme prefix (`sha256:` for stems, CID vocabulary for packages) selects the
namespace before resolution; package storage remains deferred.

Only indexed final files are playable. A move is index-last: decoded bytes are
hashed while written, the staging file is reopened and fully verified, then a
successful `FileSystemFileHandle.move(destination, name)` atomically changes
its name. The P-2 extension observed this exact staging-to-parent move on
Chromium 151 and Firefox 153. When move is absent or explicitly unsupported,
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

Every indexed hit is streamed through incremental SHA-256 inside the loading
gate. The same pass counts bytes, so it also validates the document-derived
`frames × channels × bit_depth / 8` requirement. A truncation, same-length
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

Playback reads use `getFile()` snapshots, never long-lived access handles.
Where a caller probes sync read-only mode, it trusts only
`accessHandle.mode === "read-only"`; option acceptance is not evidence because
Firefox accepts the option while retaining exclusive behavior. A read failure
during playback releases every ring and reports a typed session error; it never
turns damage into silent audio.

Quota preflight counts the canonical bytes of misses only. Unpinned LRU rows
outside the opening session may be evicted; open-session and offline pins are
never victims. `storage.insufficient` reports required, available, shortfall,
and evictable bytes. Write-time quota failure removes the failing staging file,
keeps already promoted stems, keeps the gate closed, and permits a clean retry.
An environment without `navigator.storage.getDirectory` refuses typed
`storage.unavailable`; there is deliberately no RAM-resident degraded mode.

The local-store read deadline is 15 seconds and the network no-progress
watchdog is 30 seconds. Range retries are accepted only when the server proves
the requested `206 Content-Range`, so resumption can never duplicate bytes into
the decoder.

## Hash implementation and provenance

`incremental-sha256.js` is a repository-owned implementation of NIST FIPS
180-4 SHA-256. Its exact source digest and rationale are pinned in
`incremental-sha256.provenance.json`; the gate verifies the pin and the empty,
`abc`, million-`a`, irregular-chunk, and Node-crypto oracle vectors. This is
shipped code: SHA-256 is chosen for the existing digest vocabulary and Sui
verification, not because it is implementation-free.

## Loading gate and Worker pump

`StemSessionGate.open()` invokes the caller's `resume()` synchronously before
its first `await`, preserving the iOS user gesture, then holds interaction until
`OpfsStemStore.openSession()` returns its pinned lease. Superseding an open
aborts its fetch/decode work; its promoted stems survive.

The PCM pump is a dedicated module Worker. MSB1 ring descriptors cross the
Worker port after construction; they are never processor options. The Worker
reads bounded Blob slices, de-interleaves interleaved int16/int24 little-endian
PCM, converts by exact powers of two (`2^15`, `2^23`), and writes the unchanged
MSB1 layout. The pump module contains no network API and its runtime eval runs
with a throwing network tripwire.

## Latency evidence contract

The repeatable fixture eval records three numbers: cold open (ingest plus
verification), warm open (mandatory verify cost), and total canonical bytes.
Its deterministic 8-stem/2 MiB Node fixture budgets are 5 s cold and 2 s warm;
these are regression bounds, not product claims. The browser probe separately
records real OPFS ingest/hash throughput and verify cost for the #246 field
trend line.
