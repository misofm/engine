# Ship the browser module without its function-name section

## Problem

The shipped AudioWorklet module (`miso-engine-v1-audio-worklet.simd128.wasm`, 3,289,705 B at
`7d030945`) keeps the wasm `name` custom section. It was about 375 KB when measured on
2026-09-29. `scripts/build-web-audioworklet.sh` strips DWARF (`-C strip=debuginfo`) but keeps the
names on purpose, because three gates find functions by name in the shipped module:
- `scripts/check-web-audioworklet-callgraph.py` refuses a module with no names (`:245`, self-test
  `(e)`);
- `scripts/check-web-audioworklet-v8-spill.py` (`:717-743`);
- `scripts/check-scalar-oracle-absent.py`.

Nothing in the browser runs on those names:
- no code in `hosts/host-web/web/*.js` or `sdk/src` reads `Error.stack` or wasm frame names;
- errors cross the boundary as typed codes;
- Rust panic messages carry file and line from data, not the name section.

The names only make devtools stack traces readable (`host_web::…` instead of `wasm-function[N]`).
Every browser page load downloads them. Owner, 2026-09-30: do this "if it doesn't actually help
run any of the browser code".

## Smallest closable slice

1. **Two outputs from one build.** The build script produces the **named** module exactly as today.
   It then writes the **shipped** module as the named module with the `name` custom section
   removed. Every other section is copied byte for byte. Measure the other custom sections
   (`producers`, `target_features`, anything else) and record their sizes. Remove them too only if
   nothing reads them at load time or run time, and say why in the evidence.
2. **The gates read the named twin.** The callgraph, V8 spill and scalar-oracle-absent gates
   analyse the named module. A new check asserts that the shipped module equals the named module
   minus the removed sections, with every non-custom section byte-identical. So what the gates
   prove about the code holds for what ships.
3. **The stripper** is a small, dependency-free tool: a Python script beside the gates, or a Rust
   tool under `tools/`, whichever fits the repo. It needs a self-test covering:
   - a module with and without a name section;
   - a malformed section length, which is refused;
   - a section-order check.
4. **Keep the named module for debugging.** CI and release keep the named twin as a non-shipped
   artifact, so a production stack trace can be symbolicated. `docs/RELEASE.md` says how.
5. **The pin.** The release pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`,
   checked at release since #1061) and the PR artifact report refer to the shipped module. Do not
   re-pin in this issue; the release step does that.

Authorized paths: `scripts/build-web-audioworklet.sh`, the three gates and their self-tests, the
new stripper and its test, the CI steps that call them (`.github/workflows/qualification.yml`,
`nightly.yml` if it builds the module), `scripts/web-audioworklet-identity.py` if it hashes the
module, `docs/RELEASE.md`, and this spec.

## Objective gates

1. The shipped module is smaller by the removed sections' size. Record the before and after bytes
   and the name section's size.
2. The shipped module's code, data and every other non-custom section are byte-identical to the
   named module's, by the new check, and the check goes red on a planted one-byte change to the
   code section.
3. The callgraph, V8 spill and scalar-oracle-absent gates pass on the named twin, and their
   self-tests pass. A gate fed the shipped (nameless) module still refuses it with its typed
   message, rather than passing blind.
4. The browser qualification with `--check-matrix --self-test-mutations` passes in Chromium,
   Firefox and WebKit on the shipped module, and every render digest and the V8 harness digests
   equal `7d030945`'s.
5. The SDK checks (`check-sdk-headless`, `sdk-package.sh check`) and `test-web-audioworklet.sh`
   pass. `scripts/test-ci-path-routing.py` and every lint-job step pass.
6. The build stays reproducible: two builds give the same shipped digest.

## Non-goals

- No change to Rust code or to `[profile.release]`: native artifacts keep their line tables.
- No `wasm-opt` pass.
- Removing the eight-lane code is its own issue.
