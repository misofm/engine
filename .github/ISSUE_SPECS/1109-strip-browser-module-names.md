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

## Attempt 1 evidence

Terra, 2026-09-30, on `381202ec` (`7d030945` plus specs). Commit `65b513b1` is the change.

**The owner's condition, re-verified.** Nothing that runs in the browser reads the names.
`hosts/host-web/web/*.js` (host, worklet, `prepared-control.js`), `sdk/src` and `sdk/codegen`
contain no `.stack`, `Error.captureStackTrace`/`prepareStackTrace`, `console.trace`,
`WebAssembly.Module.customSections` or `wasm-function` read. Their `catch` blocks drop the error or
map it to a typed code. A wasm trap's `message` names no function in any engine; only `stack` does.
The `.stack` reads in the tree are diagnostics in test tooling (`qualification/run.mjs`,
`suspended-host.mjs`, `sdk/test/package-tarball-smoke.mjs`) and decide nothing.

**What the change does.**
- `scripts/strip-wasm-names.py` (new, standard library only): `strip NAMED SHIPPED`,
  `check NAMED SHIPPED` (the twin check), `sections MODULE` and `--self-test`.
- `scripts/build-web-audioworklet.sh`: one cargo build. rustc's output is the named twin. The
  shipped module is the twin minus its `name` section, and is the only module the script digests,
  pins and delivers. `--named-twin DIR`, valid with any mode, also writes
  `miso-engine-v1-audio-worklet.simd128.named.wasm` into DIR. DIR must be empty, not a symlink and
  not the output directory, and is refused before anything is built. The seven-file delivery
  closure is unchanged.
- The gates:
  - `check-web-audioworklet.sh` now takes `ARTIFACT_DIRECTORY NAMED_TWIN`. It runs the twin check
    first and reads names for the call graph from the twin. Every other check reads the shipped
    module.
  - The call-graph, V8 spill and scalar-oracle-absent gates read the twin. Each one's refusal of a
    nameless module now says to pass the twin.
  - `run-wasm-gates.sh`'s V8 leg uses `--module-only --named-twin`.
- CI (`qualification.yml`):
  - `artifact` builds with `--named-twin`, publishes `named_sha256`, and uploads the twin as
    `audioworklet-named-<sha>`.
  - `artifact-gates` downloads the twin and checks it against `named_sha256`. It then runs the
    stripper's self-test and the twin check against the digest-verified shipped module, before the
    three name-reading gates.
  - `nightly.yml` builds no module and is unchanged. So is `npm-publish.yml`, which publishes only
    the shipped module and checks it against `EXPECTED_WORKLET_SHA256`, as before.
- `docs/RELEASE.md` has a new section, "The named twin", on keeping the twin (the CI artifact) and
  rebuilding it reproducibly from a release commit to symbolicate `wasm-function[N]`.
- The pin was not changed.

**Paths outside the listed set, each forced by the change:**
- `scripts/check-web-audioworklet.sh`: the caller of the call-graph gate.
- `scripts/run-wasm-gates.sh`: the local caller of the V8 gate.
- `scripts/test-sdk-artifact-builder-output-contract.sh`: its mock `cargo` wrote a non-wasm file,
  which the stripper refuses. It now writes a real module, and the test covers `--named-twin`.
- `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py`: they pin the changed
  build, V8 and `--without-metadata-regeneration` lines. The V8 rule now also needs the twin's
  digest check before the gate reads it.
- `scripts/ci-path-router.py`: `strip-wasm-names.py` joined the `console-benchmark` self-test key,
  because the build script, which that key covers, mentions it.

None of these paths is one #1110 touches, but #1110 may edit `run-wasm-gates.sh`, which is a
possible merge conflict.

**Gate 1: sizes.** From `strip-wasm-names.py sections`. Each size includes the section's id byte
and size encoding.

| module | bytes | sha256 | gzip -9 |
| --- | ---: | --- | ---: |
| named twin (byte-identical to `7d030945`'s shipped module) | 3,289,705 | `ac3a9353…02efc9` | 978,081 |
| shipped | 2,909,561 | `81c23a64…772c96` | 926,514 |
| difference | −380,144 (−11.6 %) | | −51,567 (−5.3 %) |

Custom sections of the named twin:
- `name`: 380,144 B (payload 380,140), removed.
- `producers`: 79 B, kept.
- `target_features`: 160 B, kept.

The non-custom sections are the same bytes in both modules: type 940, function 2,507, table 9,
memory 5, global 11, export 5,115, element 1,017, code 2,801,611 and data 98,099.

Why `producers` and `target_features` stay:
- Together they are 239 B, 0.008 % of the module.
- Proving that no browser reads them at load or run time would mean auditing three engines'
  decoders. Only V8 could be checked here: Node 22's binary holds no identifier for either section,
  while it does hold `sourceMappingURL`, `external_debug_info` and `compilationHints`. SpiderMonkey
  and JavaScriptCore could not be checked.
- With them kept, the invariant stays "the twin minus exactly `name`".

**Gate 2: twin check.**
- `strip-wasm-names.py check` passes on the pair: "9 non-custom and 2 other custom sections
  byte-identical".
- One byte flipped at offset `0x100000` (inside the code section) of the real shipped module: red,
  "section 7 differs … code".
- The twin passed as the shipped module: red.
- An independent `head`/`tail` + `cmp` of the byte ranges around the removed section agrees.
- `--self-test`: 33 cases. Among them, a one-byte change to code and to data, a dropped, added,
  swapped or re-encoded custom section, a malformed size (truncated, six-byte, past the end, one
  short), and order (export before memory, repeated memory, tag after global, data count after
  code).
- A mutation sweep of the tool itself: 13 of its rules deleted or weakened in a copy, each red in
  the self-test (two by a crash rather than a named case). The rules: the order rule, the end
  bound, the five-byte bound, the name bound, the UTF-8 decode, the magic check, the unknown-id
  refusal, the exact-bytes comparison, the extra-section, order and custom-section parts of the
  comparison, the refusal of a nameless strip input, and stripping only `name`. Two redundant rules
  had no case that could catch them and were deleted: a 32-bit bound the end bound subsumes, and a
  "shipped keeps its names" test the comparison subsumes.

**Gate 3: the name-reading gates.**
- On the named twin all three pass:
  - call graph: render closure=8 traps=5, meter_poll, command_submit, and 11 roster rows (15
    kernels ≥ 11);
  - scalar-oracle-absent: 2,502 symbols, all 4 controls;
  - V8 spill (Node 22.23.2): 3 held loops, no carried slot.
- Their self-tests pass.
- Fed the shipped module, each refuses with its typed message:
  - call graph: "name section required: func[0] has no <name> …", exit 1;
  - scalar-oracle-absent: "FAIL the module has no `name` section …", exit 1;
  - V8 spill: "the module has no function names …", exit 2.
- `check-web-audioworklet.sh`:
  - passes over the new closure and twin;
  - refuses a directory with no twin (usage, exit 2);
  - refuses the shipped module passed as the twin (twin check, exit 1).

**Gate 4: browsers.**
- `npm run qualify -- --check-matrix --self-test-mutations` over the shipped closure: chromium
  151.0.7922.34, firefox 153.0 and webkit 26.5 each report "all qualification gates passed".
- The render digests are the committed pins. The corpus digest matches `expected.json`'s native
  pin. The live-control and stall digests match their computed expectations.
  `check-browser-expected-resources.py --artifacts` also passes (the direct oracle under Node, all
  three PCM digests).
- The V8 harness (`web-mixing-automation-benchmark.mjs preflight`) gives the same 7 arm digests and
  2 document digests for `7d030945`'s module (`ac3a9353`) and the shipped module (`81c23a64`).

**Gate 5: SDK and CI.**
- From a clean `npm ci` in `sdk/`, these pass over the shipped closure:
  - `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`;
  - `check-sdk-headless.sh`;
  - `sdk-package.sh check`, including `test-sdk-artifact-builder-output-contract.sh`;
  - `test-web-audioworklet.sh`.
- Five mutations of the build script each turn the builder contract red: ship the named module,
  write no twin, write the shipped module as the twin, skip validating the twin directory, allow
  the output directory as the twin's.
- These pass:
  - `cargo fmt --all --check`;
  - every `lint`-job script gate and self-test (46 commands);
  - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
  - the self-test suites this change selects (`test-console-benchmark.sh`,
    `test-env-vocabulary.sh`);
  - `web-audioworklet-identity.py --self-test`.

**Gate 6: reproducible.** A second `--module-only --named-twin` build, from a `git archive` of
`65b513b1` at another path, gives the same shipped digest `81c23a64…` and the same twin `ac3a9353…`.
CI's `artifact-identity` twin adds the `CARGO_HOME` variation.

**Test value.**
- `strip-wasm-names.py --self-test` goes red when a stripper removes anything but the `name`
  section, or when a check misses any byte, section or order change between the two modules. It is
  the tool's only test.
- The builder-contract additions go red when the build ships the named module, writes a twin that
  is not the shipped module's, or overwrites or reuses a directory for the twin. No other test
  covers the build's two outputs.
- The new `test-ci-path-routing.py` mutation goes red when `artifact-gates` stops verifying the
  twin's download before the V8 gate reads it.
