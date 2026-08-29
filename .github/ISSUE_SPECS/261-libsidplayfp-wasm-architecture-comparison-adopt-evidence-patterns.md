# 261 libsidplayfp-wasm architecture comparison: adopt claim-specific differential evidence only

One-line summary: libsidplayfp-wasm contributes useful provenance, native/Wasm differential, chunk
and soak test patterns, but its emulator scheduler, replay seeking, large PCM cache, mutable buffers
and actual parity gates do not fit V2 or substantiate a general bit-deterministic engine claim.

**This is a completed research and decision record, not implementation authority.**

**Authority: GitHub issue #261.** This local file mirrors its source-backed decision record.

## Authority, pins and method

- Engine V2: [`90c3b9a598f1244938d9cdcce04c4a4641c6b758`](https://github.com/misofm/engine-v2/tree/90c3b9a598f1244938d9cdcce04c4a4641c6b758).
- Wrapper: [`d967e68de7a57933547eacae41291799ba716cf1`](https://github.com/chrisgleissner/libsidplayfp-wasm/tree/d967e68de7a57933547eacae41291799ba716cf1).
- Pinned upstream libsidplayfp: [`d7f7f0e78e09351ad53ff15a4cfb362c4f1c8339`](https://github.com/libsidplayfp/libsidplayfp/tree/d7f7f0e78e09351ad53ff15a4cfb362c4f1c8339).
- Pinned libresidfp: [`a5cd8f2486d627c40ea8c7c7a25827db73837002`](https://github.com/libsidplayfp/libresidfp/tree/a5cd8f2486d627c40ea8c7c7a25827db73837002).
- Source was inspected without builds/benchmarks or legacy Miso access.

## Findings

The wrapper records exact upstream pins and a reproducible patch process
([upstream.json](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/upstream.json#L1-L16),
[modifications](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/MODIFICATIONS.md#L9-L34)).
That is a sound provenance pattern.

Its runtime executes 6510 music code in a cycle emulator, using an allocation-free event list suited
to fixed C64 hardware rather than a mixing DAG
([scheduler](https://github.com/libsidplayfp/libsidplayfp/blob/d7f7f0e78e09351ad53ff15a4cfb362c4f1c8339/src/EventScheduler.h#L34-L142)).
Upstream seeds randomized power-on behavior from wall time; the wrapper overrides it to a fixed
setting ([upstream player](https://github.com/libsidplayfp/libsidplayfp/blob/d7f7f0e78e09351ad53ff15a4cfb362c4f1c8339/src/player.cpp#L77-L166),
[wrapper config](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/src/bindings/bindings.cpp#L358-L402)).
Determinism is therefore configured, not an automatic property.

Seeking reloads and fast-forwards; high-level rendering allocates requested output, low-level
rendering can resize a mix buffer, and the optional PCM cache defaults near 106 MiB
([seek](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/src/player.ts#L706-L825),
[render](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/src/player.ts#L588-L682),
[cache](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/src/player.ts#L14-L46)).
Those are poor fits for V2's duration-independent rings and fixed callback storage.

The test evidence is mixed. One synthetic tune has exact repeat/chunk assertions
([binding test](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/test/binding-surface.test.ts#L133-L185)),
but broader checks use correlation and acknowledge a noise floor
([health test](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/test/engine-health.test.ts#L144-L155)).
The native-parity script describes SIDLite as bit-identical yet applies tolerant thresholds and
compares only the shorter output prefix, so it does not enforce full-length bit identity
([parity script](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/scripts/native-parity.mjs#L36-L82),
[comparison](https://github.com/chrisgleissner/libsidplayfp-wasm/blob/d967e68de7a57933547eacae41291799ba716cf1/scripts/native-parity.mjs#L171-L233)).

## Decision

- **Adopt:** exact upstream/compiler/flag/asset provenance and claim-specific native/Wasm
  differential, repeat, chunk-partition, constrained-browser and soak patterns—only after each result
  becomes an enforcing gate.
- **Preserve:** V2's fixed graph, bounded source rings, duration-independent memory, per-instance
  state, content identity and exact claim-specific numeric gates.
- **Reject:** the cycle emulator as general mixer architecture, replay seeking, large PCM caches,
  callback buffer growth/copies, process-global controls and ROM/path handling as V2 asset identity.

## Gates for borrowed evidence

1. Record identical source, patch, compiler, flags and asset digests for native/Wasm artifacts.
2. Reject output-length mismatch before samples; a bit-identity claim requires every sample equal and
   maximum delta zero. Tolerant algorithms get a distinct justified contract.
3. Exact repeat and multiple chunk sizes cover a broad, non-vacuous corpus; RNG/power-on state is
   explicit, never wall-time seeded.
4. Browser callback deadlines and memory ceilings are asserted, not logged; render memory cannot grow
   after preparation.
5. Any future SID source/effect separately defines latency, tail, reset, state, isolation, fault
   bypass, provenance and GPL-2.0-or-later licensing.

## Limitation

This is an emulator/player for executable SID programs, not a general production engine. Its useful
contribution is evidence discipline; its headline wording must be interpreted through the actual
gate strength.

