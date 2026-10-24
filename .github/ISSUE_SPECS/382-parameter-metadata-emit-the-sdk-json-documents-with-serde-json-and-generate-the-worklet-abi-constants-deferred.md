Child of #377 (Part 2, finding 3, plus finding 8). **Deferred past launch** by the orchestrator's judgement on 2026-09-04: it re-pins two shipped SDK JSON assets and the generated TypeScript a week after `@misofm/engine-sdk` 0.1.0 shipped, for a moderate robustness gain. Owner may pull it forward.

## Scope

- Replace the ~360 lines of `push_str`/`format!` JSON assembly in `tools/parameter-metadata/src/lib.rs:129-273` and `src/abi_layout.rs:~435-650` with `#[derive(Serialize)]` models and `serde_json` (`preserve_order`, already in `Cargo.lock` as a dev-dependency elsewhere). Decide float spelling: `serde_json` uses `ryu` for `f32`, the current `number()` uses `{:?}`; both are shortest-round-trip but exponent thresholds differ, so either accept a one-time re-pin or emit numbers through `RawValue`.
- While there, emit a constants prelude from `abi_layout.rs` so the hand-typed `COMMAND_RECORD_BYTES = 48` in `hosts/host-web/web/miso-engine-v1-audio-worklet.js:20` and `-host.js:51` come from the generator (finding 8), or add them to `check-session-map-shape.py`'s equality set.
- Re-pin `sdk/assets/*.json`, `sdk/src/generated/{catalog,abi}.ts`, and run `check-parameter-metadata-v1.py`, `check-abi-layout-v1.py --self-test`, `check-sdk-generated.sh`, `check-web-audioworklet.sh`.

## Not scheduled

Do not claim before launch without an owner comment here.

## Decision record (2026-10-24, backlog sprint)

- Pulled forward by the backlog sprint (owner goal of 2026-10-24); the launch hold is discharged.
- Root decisions: byte identity of every generated document, no re-pin; part 2 is one assertion
  in an existing gate, not a generated constants prelude. No new test.
- Emission: `Serialize` models (serde derive) written by `serde_json`'s standard pretty printer.
  `preserve_order` is not used: struct field order is the schema order, and the feature would
  unify into every workspace `serde_json` user. `raw_value` carries each `f32` in `number()`'s
  spelling. New direct dependency `serde =1.0.228` (already in `Cargo.lock`; one lock line).
- Change of decision (root, after the attempt-1 verifier PASS): attempt 1 kept the bytes with a
  custom block formatter (continuation keys, one-line wrappers). Nothing pins the asset bytes by
  digest, and the owner's rule is that fragile machinery is not worth a small gain, so the
  simpler shape wins: the standard pretty printer and one whitespace-only re-pin of the
  generated documents. The verifier's minor finding on non-string keys in that formatter goes
  away with it. The tests that scanned the old line layout (`tests/abi_layout.rs`,
  `tests/round_trip.rs`) now read the parsed JSON; their claims are unchanged.
- Part 2: `scripts/check-web-audioworklet.sh` requires each shipped JS file to declare
  `COMMAND_RECORD_BYTES` exactly once, equal to the shipped layout's `commandRecord.bytes`.

## Evidence

- Whitespace-only re-pin: each old and new document, parsed with keys in order and numbers kept as
  text, is equal. Bytes: `miso-engine-v1-parameter-metadata.json` 217,854 -> 335,632;
  `miso-engine-v1-abi-layout.json` and `scripts/fixtures/abi-layout-v1-self-test.json`
  27,915 -> 39,863; session keys and response frame unchanged. `sdk/src/generated` is unchanged;
  the built artifact's two JSON documents equal `sdk/assets`.
- Red first: `COMMAND_RECORD_BYTES = 273` in the worklet JS (tree and artifact) passes the gate
  before the assertion and fails it after; the same mutant in the host JS fails it too.
