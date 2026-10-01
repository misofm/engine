Child of #377 (Part 2, finding 3, plus finding 8). **Deferred past launch** by the orchestrator's judgement on 2026-09-04: it re-pins two shipped SDK JSON assets and the generated TypeScript a week after `@misofm/engine-sdk` 0.1.0 shipped, for a moderate robustness gain. Owner may pull it forward.

## Scope

- Replace the ~360 lines of `push_str`/`format!` JSON assembly in `tools/parameter-metadata/src/lib.rs:129-273` and `src/abi_layout.rs:~435-650` with `#[derive(Serialize)]` models and `serde_json` (`preserve_order`, already in `Cargo.lock` as a dev-dependency elsewhere). Decide float spelling: `serde_json` uses `ryu` for `f32`, the current `number()` uses `{:?}`; both are shortest-round-trip but exponent thresholds differ, so either accept a one-time re-pin or emit numbers through `RawValue`.
- While there, emit a constants prelude from `abi_layout.rs` so the hand-typed `COMMAND_RECORD_BYTES = 48` in `hosts/host-web/web/miso-engine-v1-audio-worklet.js:20` and `-host.js:51` come from the generator (finding 8), or add them to `check-session-map-shape.py`'s equality set.
- Re-pin `sdk/assets/*.json`, `sdk/src/generated/{catalog,abi}.ts`, and run `check-parameter-metadata-v1.py`, `check-abi-layout-v1.py --self-test`, `check-sdk-generated.sh`, `check-web-audioworklet.sh`.

## Not scheduled

Do not claim before launch without an owner comment here.
