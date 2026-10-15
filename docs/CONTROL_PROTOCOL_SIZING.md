# MISO control protocol bounded sizing

The effective limits advertised by `CAPABILITIES_GET` are the minima of codec, controller, queue, replay, response, and provider limits. They are resource configuration, not a compiled track limit. All sums/products below are checked before allocation, copying, slicing, or admission.

## Byte calculations

For a value of `n` bytes, `pad8(n) = (8 - (n mod 8)) mod 8` and one TLV consumes `8 + n + pad8(n)`. A nested message value consumes `8 + sum(child TLVs)`. A complete frame is `48 + payload_bytes`, and must not exceed advertised `max_frame_bytes`.

The control-state record array is `8n` bytes and the meter record array `16n` bytes (a control list page has no fixed record: each row is a nested message of at most 34 fields plus its address, so a page is bounded by `max_frame_bytes` and by the endpoint's response reservation `maximum_cached_response_bytes`, and `limit` is the most rows a page holds, not the number it must: the controller trims a page to the leading rows whose response fits the endpoint's response bound, answers it with `eof` false and the cursor at the last row kept, and a caller pages on from `last-handle`; one row that cannot fit a page alone is `LIMIT_EXCEEDED`, and a page whose content breaks its message rules is `INTERNAL`, never trimmed). Count/stride fields validate the exact product. A transaction is the outer header plus each edit's nested BTLV size; it is bounded by maximum edits, frame bytes, TLV count, string bytes, and nesting depth—not by tracks.

A `RESPONSE_GET` curve reply has a fixed layout: the outer header, eight scalar fields (`4+8+4+1+4+4+4+8` value bytes, each with its prefix and padding), the axis of `4p` bytes and, for each requested side, a total of `4p` bytes and, when sections are requested, `4sp` bytes of section curves, for `p` points and `s` sections. `response_result_encoded_len` gives the exact frame length, and the provider refuses a request whose reply would exceed the response bound before it computes any curve (#1573).

**`RESPONSE_GET` worst cases.** The reply is `49,368 + 32,768s` bytes for `p = 4,096` points, both sides and `s` sections requested. The largest owner is the parametric EQ with `s = 6` sections, so the worst reply is 245,976 bytes (about 240 KiB, `response_result_encoded_len(4096, both, 6, true)`); the input filters have 2 sections (114,904 bytes). An endpoint whose response bound (the smaller of `max_frame_bytes` and `maximum_cached_response_bytes`) is below that refuses the largest requests with `response.reply_too_large` before any curve is computed, and a caller that must always be answered sets `points` or `sections` to fit. The request is bounded by 256 override rows, the outer header, the configuration's other fields, the required and the optional scalar fields: 16,672 bytes plus the effect ID's field, which costs `8 + n + pad8(n)` for an ID of `n` bytes. That is 16,936 bytes for a 255-byte ID and 82,216 bytes at the default 65,536-byte string limit (measured with `encoded_response_request_len`).

## Prepared resources

| Resource | Sizing rule |
| --- | --- |
| decode scratch | at least the declared top-level/nested field count being decoded |
| caller output for a pure codec or already-materialized response/event | first obtain the exact required canonical bytes; retry with output of at least that size |
| caller output for a new stateful command ingress | provide at least advertised `maximum_cached_response_bytes` before replay admission or dispatch; this endpoint reservation is intentionally distinct from the eventual exact frame length |
| control queue | slots for complete decoded commands and a copied-byte budget at least their simultaneous encoded payloads |
| reliable queues | capacity for every response/event that must survive until its endpoint consumes it |
| telemetry | bounded staging/ring slots for distinct coalescing keys; replacement needs no new key slot |
| replay cache | entries for completed requests and bytes for exact request plus exact response retention |

A full retirement or render-plan queue is outside this protocol and remains governed by [Realtime memory](REALTIME_MEMORY.md).

## Admission sequence

1. Check the correlatable outer header, caller scratch, and new-request caller-output reservation.
2. Check remaining outer/TLV bounds and validate typed schema, revision, domain, time ordering, and all count/byte products.
3. Reserve replay retention and every reliable queue/event slot needed by the outcome.
4. Check queue slot/byte budgets, returning a typed report before mutation.
5. Copy the fully accepted typed item into prepared storage, then commit/return canonical response.

No sizing formula assumes a fixed track count, stem duration, PCM block, or transport packet.
