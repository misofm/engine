# Prelaunch browser ABI reset: make the shipped host V1

## Owner ruling

The shipped browser host is prelaunch. Its V1 export and artifact names must not return or require
an ABI 2.0 number. Reset the sole browser ABI identity from `0x0002_0000` to `0x0001_0000`
atomically.

## Smallest closable product slice

Change the browser host ABI constant, Wasm/direct-oracle pins, generated TypeScript declarations,
SDK copies, metadata/resources/digests, tests, and active prose that directly identify ABI 2.0. Do
not add compatibility aliases or accept the superseded prelaunch number.

## Decision record

1. Export spelling and returned ABI number form one boundary identity and must both be V1.
2. The prelaunch ABI 2.0 number has no compatibility standing and receives no reader or alias.
3. Generated SDK/browser artifacts must be rebuilt from their canonical source where generators
   exist; checked-in copies must not be edited into disagreement.
4. Session-format rejection fixtures and unrelated record schemas are successor inventory.

## Objective gates

- `miso_engine_web_v1_abi_version()` returns `0x0001_0000`.
- Default and explicit preparation/report structs consistently carry the V1 value.
- Static/object, hermetic, WebDriver, SDK generation, resource and digest checks pass.
- A wrong-version mutation remains red.
- Live browser/SDK trees contain no ABI version 2 claim or `0x0002_0000` pin.

## Workflow

Sol briefs and approves this stateless scope. Terra attempt 1 performs the atomic reset. Sol
adversarially reviews generated-artifact provenance and all ABI call paths before PASS.

## Evidence record

- Sol brief approved; GitHub issue #316 matches this stateless local spec.
