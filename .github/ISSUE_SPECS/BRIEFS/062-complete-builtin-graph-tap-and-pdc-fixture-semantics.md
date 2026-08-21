# Sol implementation brief — issue 062 complete builtin graph-tap and PDC fixture semantics

## Decision and budget

**READY FOR TERRA ATTEMPT 1 after the Issue-060 rescope is committed.** One Terra attempt and one
bounded Sol correction/review are available; a second failure stops. The stopped Issue-060 graph
fixture is technical input only. Do not edit production DSP or begin realtime audit work.

## Frozen fixture vertical

Retain the current one-track, 48-kHz/q128 graph fixture and its two existing payload paths. Bind the
genuine deterministic source and explicit deterministic nonidentity fixture processors at each
declared rack boundary; do not clear the rack topology or substitute fourteen observations of one
buffer. Freeze each processor's per-lane operation in the checker so all seven boundary summaries
are pairwise distinguishable. Builtin HPF/LPF expected words come from the accepted independent
retained-`f32` recurrence; all other fixture transforms use closed-form `f32` order.

Add exactly one fixture-only positive-latency route with a fixed integer delay and an unambiguous
impulse marker. Require compiled PDC metadata and independently derived output placement to agree.
The direct expected model must compute final PCM before reading `pcm/graph-taps.f32le`; expected
PostMatrix meter values derive from that model, never from candidate PCM.

Parse every graph meter record canonically and require exact tap, handle, generation, sequence,
interval, frame, peak/held/energy/RMS and counter words. Preserve exactly seven records, one PCM
payload and the existing total corpus shape.

## Gates and stop rules

Run focused mutations of each tap and output/PDC coordinate, checked scratch and checked-in fixture
validation, focused fixture/graph/compiler tests, format, warning-denied relevant Clippy and diff
checks. Record exact topology, latency and hashes. Stop for production changes, graph audit/swap
lifecycle, new payloads, timing or a second failed attempt.
