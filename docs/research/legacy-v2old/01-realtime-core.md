<!--
Provenance: copied from misofm/engine-v2-old docs/research/01-realtime-core.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Real-time core

The executor is deliberately boring. `mount` verifies contract versions, capabilities, and required `sample_rate_hz`; calculates capacity with checked arithmetic; derives rate-dependent data; allocates flat buffers/state; binds parameters; and compiles the routing/rack plan. `process` walks that immutable plan by dense private indices. Offline rendering invokes exactly the same plan/executor.

Process-time prohibitions are allocation, lock acquisition, waiting, logging, formatting, destruction that can free, buffer growth, string lookup, and topology compilation. A parameter event writes a preallocated numeric target or is refused before process; it cannot trigger installation, parsing, or routing change. Telemetry is numeric preallocated counters/atomics outside the DSP walk.

Capacity is a requested finite session property:

`bytes = planes × padded_tracks × frames × sizeof(f64) + effect_state + route_state + event_storage`.

Each add/multiply is checked. Admission also carries declared CPU cost and max events/block; wasm32 may refuse an otherwise valid logical session due to address-space or deadline budget. The refusal reports requested, available, and the limiting resource.

The layout follows the locality case for structure-of-arrays transformations described by [Intel](https://www.intel.com/content/www/us/en/developer/articles/technical/memory-layout-transformations.html). TrackBank storage is flat SoA/AoSoA; nested `Vec<Vec<_>>` is prohibited because it undermines contiguous track-lane access and mount-time accounting.

Verification instruments allocation and prohibited control-plane activity around process. The result is an executor invariant, not a convention inferred from profiles.
