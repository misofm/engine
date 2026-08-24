<!--
Provenance: copied from misofm/engine-v2-old docs/research/08-agent-session-api.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Agent session API

`miso-engine-abi` owns JSON Schema (Draft 2020-12), descriptors, session/events, errors, and capabilities. Canonical request/fixture serialization follows RFC 8785 where a canonical byte representation is required ([JSON Schema](https://json-schema.org/draft/2020-12), [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html)). Event envelopes may map to CloudEvents transport conventions without making CloudEvents the DSP ABI ([CloudEvents](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)).

IDs are validated immutable strings and are never dense ordinals. Display names and display order are separate. Descriptors declare ports/taps/layouts, parameters and their units/ranges/scales/defaults/enums/smoothing/lifecycle/dependencies, identity/bypass, latency/tail/state/schema versions, capacity/cost/determinism, operations, telemetry, capability booleans, and refusal formats.

Illustrative only — **non-normative until the schemas issue**:

```json
{
  "schema_version": "0.1", "session_id": "session.mix-001", "sample_rate_hz": 47999,
  "tracks": [{"id": "track.vocal", "name": "Vocal", "layout": "mono", "display_order": 0}],
  "capacity": {"max_events_per_block": 64, "byte_budget": 4194304},
  "operations": [{"op": "mount"}]
}
```

```json
{
  "code": "unsupported_sample_rate", "message": "sample_rate_hz is outside the certified range",
  "path": "/sample_rate_hz", "requested": 1, "supported": {"min_hz": 8000, "max_hz": 192000},
  "action": "choose a supported finite session sample_rate_hz and remount", "retryable": false
}
```

Topology mutation errors identify the mounted object and return `requires_remount`; unsupported horizon features return `not_implemented` with `capability: false`. These are action requests, not opaque strings.
