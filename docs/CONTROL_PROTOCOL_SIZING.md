# MISO control protocol bounded sizing

The effective limits advertised by `CAPABILITIES_GET` are the minima of codec, controller, queue, replay, response, and provider limits. They are resource configuration, not a compiled track limit. All sums/products below are checked before allocation, copying, slicing, or admission.

## Byte calculations

For a value of `n` bytes, `pad8(n) = (8 - (n mod 8)) mod 8` and one TLV consumes `8 + n + pad8(n)`. A nested message value consumes `8 + sum(child TLVs)`. A complete frame is `48 + payload_bytes`, and must not exceed advertised `max_frame_bytes`.

The control-state record array is `8n` bytes and the meter record array `16n` bytes (a control list page has no fixed record: each row is a nested message of at most 34 fields plus its address, so a page is bounded by `max_frame_bytes` and by the endpoint's response reservation `maximum_cached_response_bytes`, and `limit` is the most rows a page holds, not the number it must: the controller trims a page to the leading rows whose response fits the endpoint's response bound, answers it with `eof` false and the cursor at the last row kept, and a caller pages on from `last-handle`; one row that cannot fit a page alone is `LIMIT_EXCEEDED`, and a page whose content breaks its message rules is `INTERNAL`, never trimmed). Count/stride fields validate the exact product. A transaction is the outer header plus each edit's nested BTLV size; it is bounded by maximum edits, frame bytes, TLV count, string bytes, and nesting depth—not by tracks. A `SESSION_REPLACE` (`0013`, #1386) request is the outer header plus two fixed fields (a `U32` length and a 32-byte digest), 104 bytes, under every frame bound; its document travels beside the frame, so the document's only bound is the boot's `maximum_document_bytes`, never the frame. Its diff's edit count is bounded by maximum edits as a transaction's is, and its reply has the transaction reply's size.

A ready `SPECTRUM_CAPTURE` reply (`0011`, #1595) has a fixed layout but for the target identity and the planes: the outer header, fourteen scalar fields, the 1,025-word axis and one 1,025-word dB plane per requested side, and with the window one 2,048-word plane per side. For an identity of up to 8 bytes it is 8,496 bytes for one side, 12,608 for both, 16,696 for one side with its window and 29,008 for both with the window; every further 8 bytes of identity add 8 (`ProtocolCodec::spectrum_capture_ready_frame_len`, which the encoder's own count gives). A ready `SPECTRUM_STREAM` reply (`0012`) has the outer header, nineteen scalar fields, the axis and one dB plane per side of the entry: 8,576 bytes for one side and 12,688 for both, for an identity of up to 8 bytes, every further 8 bytes adding 8 (`ProtocolCodec::spectrum_stream_ready_frame_len`). A session whose frame limit is below a read's size refuses that read (`LIMIT_EXCEEDED`, `spectrum.frame`) before it pops a window or consumes a status, and a configure (and a boot) refuses a tap whose one-side read, one-shot or stream, the limit cannot carry, so a session that reads spectrum needs a frame limit of at least 8,576 bytes plus its longest tap identity's padding (12,688 to stream a stereo entry or read both sides; 29,008 with the window).

An `OBSERVATION_READ` reply (`000f`, #1586) has a fixed width per row: the outer header (48 bytes) and the observed sample (16) and, per row, its message (16) and at most eleven fields of 16 bytes each, so the widest reply of `n` rows (every row ready with both lanes) is `64 + 192 n` bytes: 4,096 bytes for 21 rows and 49,216 for the 256 one request may name (`ProtocolCodec::observation_read_largest_frame_len`, which the encoder's own count gives). A read whose widest reply exceeds the frame limit is refused whole, `LIMIT_EXCEEDED` `observation.frame`, before any slot is read, so a session that reads every one of 256 taps at once needs a frame limit of at least 49,216 bytes; with the default 4,096 a client reads at most 21 taps per request. The request is 112 bytes per selection plus its two IDs, each padded to 8 bytes.

A `RESPONSE_GET` curve reply has a fixed layout: the outer header, eight scalar fields (`4+8+4+1+4+4+4+8` value bytes, each with its prefix and padding), the axis of `4p` bytes and, for each requested side, a total of `4p` bytes and, when sections are requested, `4sp` bytes of section curves, for `p` points and `s` sections. `response_result_encoded_len` gives the exact frame length, and the provider refuses a request whose reply would exceed the response bound before it computes any curve (#1573).

**`RESPONSE_GET` worst cases.** The reply is `49,368 + 32,768s` bytes for `p = 4,096` points, both sides and `s` sections requested. The largest owner is the parametric EQ with `s = 6` sections, so the worst reply is 245,976 bytes (about 240 KiB, `response_result_encoded_len(4096, both, 6, true)`); the input filters have 2 sections (114,904 bytes). An endpoint whose response bound (the smaller of `max_frame_bytes` and `maximum_cached_response_bytes`) is below that refuses the largest requests with `response.reply_too_large` before any curve is computed, and a caller that must always be answered sets `points` or `sections` to fit. The request is bounded by 256 override rows, the outer header, the configuration's other fields, the required and the optional scalar fields: 16,672 bytes plus the effect ID's field, which costs `8 + n + pad8(n)` for an ID of `n` bytes. That is 16,936 bytes for a 255-byte ID and 82,216 bytes at the default 65,536-byte string limit (measured with `encoded_response_request_len`).

A `RESPONSE_READY` event (`8040`, #1578) is bounded by `response_ready_encoded_bound(p, channels, n)` for `p` points and `n` owners: the outer header and fields 1 to 4 (232 bytes with a 127-byte strip ID), four scalar captures (64 bytes), the axis and one or two sides of `8 + 4p` bytes each (padded to 8), and 384 bytes per member with 127-byte slot and effect IDs; a refusal with a 127-byte diagnostic is 368 bytes. At `p = 4,096` points and both sides that is `49,472 + 384n` bytes. The bound is exact for those identities (`response_ready_encoded_bound_is_the_largest_frame_exactly`), so a provider that refuses a request whose bound exceeds the smaller of the endpoint's frame limit and `maximum-reply-bytes` never arms a capture whose event it cannot send (#1578 R3).

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
