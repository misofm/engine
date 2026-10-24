# Report a spectrum-arm queue in the BACKPRESSURE record

Filed by root from the #1595a verification (backlog sprint, 2026-10-24).

## Finding

The wire makes the backpressure record mandatory on a `BACKPRESSURE` reply (`crates/protocol/src/typed_frame.rs` near line 344). When a spectrum command (SPECTRUM_CONFIGURE, and SPECTRUM_CAPTURE's `Busy`) is refused because the spectrum arm queue is full, `non_ok` fills that record with the `ReplayCache` kind and the replay cache's occupancy, which misnames the queue that is actually full. Nothing is acked, so this is a diagnostics defect, not a correctness one.

## Smallest closable slice

Add a backpressure queue kind for the spectrum arm queue (the next free value in the existing enum, additive under V1), fill it with that queue's capacity and occupancy on a spectrum refusal, and add one assertion to the existing Busy refusal test. Corpus re-pin only if a frame lists the kinds (old frames byte-equal).

## Paths

`crates/protocol/src/{typed_frame.rs,...}`, `crates/control-plane/src/control.rs` (the refusal), the existing spectrum_monitor test, the corpus re-pin set if needed, this spec. After #1595a.
