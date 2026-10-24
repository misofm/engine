# Report a spectrum-arm queue in the BACKPRESSURE record

Filed by root from the #1595a verification (backlog sprint, 2026-10-24).

## Finding

The wire makes the backpressure record mandatory on a `BACKPRESSURE` reply (`crates/protocol/src/typed_frame.rs` near line 344). When a spectrum command (SPECTRUM_CONFIGURE, and SPECTRUM_CAPTURE's `Busy`) is refused because the spectrum arm queue is full, `non_ok` fills that record with the `ReplayCache` kind and the replay cache's occupancy, which misnames the queue that is actually full. Nothing is acked, so this is a diagnostics defect, not a correctness one.

## Smallest closable slice

Add a backpressure queue kind for the spectrum arm queue (the next free value in the existing enum, additive under V1), fill it with that queue's capacity and occupancy on a spectrum refusal, and add one assertion to the existing Busy refusal test. Corpus re-pin only if a frame lists the kinds (old frames byte-equal).

## Paths

`crates/protocol/src/{typed_frame.rs,...}`, `crates/control-plane/src/control.rs` (the refusal), the existing spectrum_monitor test, the corpus re-pin set if needed, this spec. After #1595a.

## Decision record (scope note and root, 2026-10-24)

- Correction: there is no spectrum arm queue. `Busy` means the capture's one-shot arm slot is held (or a stream runs, or the caller holds an arm or an unread window). The record names that slot.
- D1: `BackpressureQueueKind::SpectrumArm = 7`, additive under V1; `2` stays retired.
- D2: one constant record: capacity 1, occupancy 1, requested items 1, no generation (`has_generation` false, as for the replay cache). Known imprecision: a collection with no selection also answers `Busy` and reports the slot as held. #1595 D6's status table freezes that status; reopen only with D6.
- D3: additive `ProtocolController::complete_session_command_refused_backpressure`; `complete_session_command_refused` gains `debug_assert_ne!(status, Backpressure)`, so no session refusal takes the replay-cache default. `SpectrumRefusal::Reply` carries `Option<Backpressure>`, filled only for `E::Busy`; the configure, capture and stream completions share one helper.
- D4: `ProtocolController::process`'s own `non_ok(Backpressure, None)` (a structural generation in flight) still names the replay cache. Out of scope.
- Paths: crates/protocol/src/{message_wire.rs,message_wire/tests.rs,controller.rs}, crates/control-plane/src/control.rs, crates/control-plane/tests/spectrum_monitor.rs, docs/CONTROL_PROTOCOL_REGISTRY.md. No `typed_frame.rs` edit and no corpus re-pin: no corpus frame lists kinds, the enum is not in schema.rs, and the SDK decodes `queueKind` as a number (`frozen_corpus_bytes_and_typed_decoders_are_unchanged` passes unchanged).

## Evidence

- Acked-batch question: the record is a constant chosen from the service's returned error, after the refusal and with no state access; the refusal reply is recorded for replay as before, and `each_capture_refusal_answers_its_status_and_changes_nothing` still shows the refused arm consumed nothing.
- Test value, `each_capture_refusal_answers_its_status_and_changes_nothing` (one new assertion): red if a spectrum `Busy` refusal reports the replay cache, or no record, instead of the arm slot with capacity 1 and occupancy 1; no other test decodes a spectrum refusal's record.
- Test value, `a_backpressure_payload_naming_the_retired_automation_queue_is_refused` (kind 7 moved from the refused set to the allocated set, refused set now `[0, 2, 8]`): red if code 7 stops decoding, or decodes with a queue generation.
- The stream path has no refusal row in control-plane tests; it shares `spectrum_service_refusal` and the one completion helper with the capture, so no stream row was added.
