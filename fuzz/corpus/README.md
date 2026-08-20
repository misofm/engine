# Issue-005 protocol fuzz seeds

The protocol corpus is checked in as `hex:` text rather than opaque binary so the frozen BTLV
headers are auditable in review. `protocol_support.rs` converts valid prefixed records to bytes;
arbitrary libFuzzer mutations that break the prefix remain raw BTLV decoder input.

- `protocol_command/capabilities.hex`: empty `CAPABILITIES_GET` command golden.
- `protocol_session_transaction/set_session_id.hex`: canonical typed transaction golden.
- `protocol_event/counter_snapshot.hex`: `COUNTER_SNAPSHOT` typed-event decoder seed.
- `protocol_response/non_ok.hex`: typed-response decoder seed.

The complete checked-in schema corpus and its hash are documented in
`complete-schema-manifest.md`; the native targets select their typed command, response, event,
or transaction decoder rather than a duplicate generic outer decoder.

The bounded sanitizer command and fixed seeds are recorded in `scripts/run-protocol-fuzz.sh`.
