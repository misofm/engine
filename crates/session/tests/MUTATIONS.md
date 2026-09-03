# Red-mutation log — `session` schema v1

## Issue #241 — source identity and canonical source shape

Each mutation was applied alone on 2026-08-29, the named production-surface gate was run, RED was
observed, and the mutation was reverted.

| gate | mutation | observed red |
|---|---|---|
| `source_identity_format_diagnostics_are_byte_identical_at_validator_and_web_boot` | relax lowercase digest bytes to `is_ascii_hexdigit()` | the uppercase-digest case is accepted by both validator and boot; the validator assertion reports failing stage `None` instead of typed-model stage `1` |
| `canonical_round_trip_is_byte_stable` | serialize `SourceBitDepth::Float32` as JSON number `24` | checked-in `"bit_depth": "32f"` rewrites to `"bit_depth": 24`, failing the byte-stable canonical fixture before the reparsed fixed-point arm |
| `fixtures/stem-identity/v1/generate.py --check` | change one hex digit in the checked-in f32 NaN/subnormal/negative-zero vector identity | exits 1 with `fixture drift: VECTORS.tsv`; the mutation was reverted and `--check` returned green |

The non-mutated identity gate covers wrong prefix, wrong length, uppercase digest, and non-hex
digest with exact `source.content.identity_format` / `$.sources[0].content` parity between the
validator report and the production web boot diagnostic bytes.
