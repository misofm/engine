# MISO control protocol bounded sizing

The effective limits advertised by `CAPABILITIES_GET` are the minima of codec, controller, queue, replay, response, and provider limits. They are resource configuration, not a compiled track limit. All sums/products below are checked before allocation, copying, slicing, or admission.

## Byte calculations

For a value of `n` bytes, `pad8(n) = (8 - (n mod 8)) mod 8` and one TLV consumes `8 + n + pad8(n)`. A nested message value consumes `8 + sum(child TLVs)`. A complete frame is `48 + payload_bytes`, and must not exceed advertised `max_frame_bytes`.

The fixed automation command payload is `16 + 16 + (8 + 32n) = 40 + 32n`; its complete frame is `88 + 32n`. For the v1 maximum `n=256`, that is 8,280 bytes before any transport framing. The parameter-state and meter record arrays are `16n` bytes, with count/stride fields validating the exact product. A transaction is the outer header plus each edit's nested BTLV size; it is bounded by maximum edits, frame bytes, TLV count, string bytes, and nesting depth—not by tracks.

## Prepared resources

| Resource | Sizing rule |
| --- | --- |
| decode scratch | at least the declared top-level/nested field count being decoded |
| caller output for a pure codec or already-materialized response/event | first obtain the exact required canonical bytes; retry with output of at least that size |
| caller output for a new stateful command ingress | provide at least advertised `maximum_cached_response_bytes` before replay admission or dispatch; this endpoint reservation is intentionally distinct from the eventual exact frame length |
| control queue | slots for complete decoded commands and a copied-byte budget at least their simultaneous encoded payloads |
| automation queue | one fixed 256-record slot per accepted wire batch; `ceil(records / 256)` slots for a burst |
| reliable queues | capacity for every response/event that must survive until its endpoint consumes it |
| telemetry | bounded staging/ring slots for distinct coalescing keys; replacement needs no new key slot |
| replay cache | entries for completed requests and bytes for exact request plus exact response retention |
| density | count all queued and proposed record starts in each quantum block, not just the proposed batch |

For the mandated 10,000-record fixture, `ceil(10000 / 256) = 40`; configure at least 40 automation slots plus a density allowance that admits its known start distribution. A single over-density batch or aggregate queued frontier rejects atomically. A full retirement or render-plan queue is outside this protocol and remains governed by [Realtime memory](REALTIME_MEMORY.md).

## Admission sequence

1. Check the correlatable outer header, caller scratch, and new-request caller-output reservation.
2. Check remaining outer/TLV bounds and validate typed schema, revision, domain, time ordering, and all count/byte products.
3. Reserve replay retention and every reliable queue/event slot needed by the outcome.
4. Check queue slot/byte and density budgets, returning a typed report before mutation.
5. Copy the fully accepted typed item into prepared storage, then commit/return canonical response.

No sizing formula assumes a fixed track count, stem duration, PCM block, or transport packet.
