# Engine V2 documentation

The issue bodies in [`.github/ISSUE_SPECS`](../.github/ISSUE_SPECS/README.md) retain scope and acceptance authority. These are checked-in contracts and evidence records.

| Topic | Document |
| --- | --- |
| BTLV v1 bytes, ownership, and compatibility | [Control BTLV v1](CONTROL_BTLV_V1.md) |
| IDs, enums, and message schemas | [Control protocol registry](CONTROL_PROTOCOL_REGISTRY.md) |
| Controller, replay, revisions, events, and queues | [Control protocol semantics](CONTROL_PROTOCOL_SEMANTICS.md) |
| Capacity equations and admission sizing | [Control protocol sizing](CONTROL_PROTOCOL_SIZING.md) |
| Typed provider and adapter boundary | [Control provider boundary](CONTROL_PROVIDER_BOUNDARY.md) |
| Corpus and recorded Issue 005 evidence | [Control protocol conformance](CONTROL_PROTOCOL_CONFORMANCE.md) |
| Session TOML model edited by the protocol | [Session schema v1](SESSION_SCHEMA_V1.md) |
| Render lifetime and SPSC foundation | [Realtime memory](REALTIME_MEMORY.md) |
| Native effect factory and process boundary | [Effect contract V1](EFFECT_CONTRACT_V1.md) |
| Provisional issue-029 descriptor representation | [Effect descriptor wire V1](EFFECT_DESCRIPTOR_WIRE_V1.md) |
| Provisional issue-029 package identity and artifact hashes | [Effect package V1](EFFECT_PACKAGE_V1.md) |
| Provisional issue-029 prepared-state envelope | [Effect state V1](EFFECT_STATE_V1.md) |

`miso-engine-protocol` is control-plane-only. It has no renderer, `PreparedRenderPlan`, PCM payload, transport framing, or exported C ABI.
