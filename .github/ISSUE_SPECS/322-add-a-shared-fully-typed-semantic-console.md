# Add a shared fully typed semantic console

## Objective

Give agents one generated, semantic live-control surface over both headless Wasm and the shipped
browser host. Normal callers must address tracks, racks, effects, parameters, lanes, and observation
taps by typed names; numeric wire addresses remain an internal transport detail.

This is issue #207's smallest closable typed-control slice. Effect lifecycle integration remains a
successor because it is opt-in orchestration over this surface, not part of command correctness.

## Product contract

- A console is bound to the engine's compiled session map and resolves stable track IDs locally.
- Track controls cover pan, matrix, fader, mute, solo, trim, and polarity inversion.
- Effect controls are generic in `EffectId`; live parameter names, input value types, lane policy,
  and observation tap names derive from the generated catalog. Prepared-only parameters do not
  compile as live edits.
- Effect parameter labels/booleans are converted through their descriptor rows, and numeric domains
  are checked before transport. Callers never supply a parameter or tap ID.
- One `submit(...edits)` is one engine transaction and returns generated result/reason names,
  rejected index, admitted count, and exact application sample on headless and browser hosts.
- The low-level `ConsoleWriter` remains available for specialized coalescing, but its public kind
  and refusal-name types are narrowed to generated vocabularies.

## Scope

- A DOM/Node-neutral semantic console and edit builder in SDK core.
- Thin headless and browser transports over the existing direct ABI and shipped host.
- Runtime, strict compile-time red, barrel, tarball, and documentation coverage.

No session mutation, effect-slot introspection ABI, queue-size change, Effect dependency, source
pump, browser host byte change, or registry publication is in scope.

## Objective gates

1. A live headless session admits and applies all eleven wire command kinds built without numeric
   rack/channel/parameter/tap values, and stays usable after a typed refusal.
2. A browser-host adapter produces the same wire records and maps the same whole-batch ack shape.
3. Strict TypeScript red probes reject an unknown track-control option, prepared-only parameter,
   wrong effect parameter, wrong tap, enumeration label, and shared-parameter lane.
4. Runtime domain probes refuse non-finite/out-of-range numbers and unknown track IDs before the
   transport is called.
5. A batch refusal admits zero; an acknowledgement is created only after the transport answers.
   The acked-batch question is asserted directly in tests.
6. Existing SDK behavior, type, generated-surface, package, browser, and deletion gates stay green.

## Decision record

- The semantic builder is bound to `sessionMap()`, so track IDs become canonical indices without a
  TOML parser. Effect positions remain explicit rack/index pairs because the frozen ABI does not
  expose compiled effect-slot identities; the caller supplies `effectId` only to select generated
  parameter/tap metadata, and the engine remains authority on whether that instance matches.
- Edits are data. Building one performs local validation but no I/O; `submit` is the only operation
  that mutates engine state, preserving whole-batch transaction boundaries.
- Numeric wire records stay exported only as the existing expert-level `LaneEdit`/`ConsoleWriter`
  seam. The new documented path contains no magic values such as `255`.

## Evidence

Implementation attempt 1:

- `EngineConsole` binds a pure semantic edit builder to either direct Wasm or the shipped browser
  host. Headless exposes `engine.console()`; browser exposes a lazy, once-bound async equivalent.
- Strip methods cover pan, matrix, fader, mute, solo, trim, and polarity. Generic effect methods
  derive live parameter names/value types/lane policy and tap names from the generated catalog.
- Browser request IDs continue from the session-map acknowledgement; both transports map the same
  generated result/reason names and enforce whole-batch acknowledgement consistency.
- The expert `LaneEdit`/`ConsoleWriter` seam remains, but kind and refusal names are now generated
  unions and its encoder is shared by the semantic headless transport.

Local gates on 2026-09-02:

- `check-sdk-headless.sh`: PASS, 109 tests / 27 suites against live Wasm.
- `check-sdk-types.sh`: PASS, including wrong option, prepared-only parameter, wrong effect
  parameter/tap, absent tap, and shared-lane red probes.
- `check-sdk-deletions.py`: PASS over 40 SDK source files.
- `sdk-package.sh check`: PASS; 59-file tarball, all entry/declaration/embedded-boot checks green.
- Focused console eval: PASS for exact eleven-kind coverage, one live ten-edit transaction plus
  observation unsubscribe, typed engine refusal and recovery, pre-transport domain rejection,
  browser wire/ack parity, and torn-ack discrimination.

Adversarial review:

- PASS locally on generated metadata use, headless/browser parity, request-ID monotonicity, pure
  edit construction, whole-batch admission, and the acked-batch question. Final closure requires
  the implementation commit's upstream workflows.
