# session aborts the host on untrusted JSON containing an empty object {} (parser CodeMap volume bug, reachable from C API and wasm boot)

## Current approved scope — 2026-09-10

### 1. #387 — prevent malformed JSON from aborting the host

Product outcome: empty objects in invalid session documents return a useful diagnostic instead of aborting native hosts or trapping Wasm boot.

Current evidence: `crates/session/src/json_preflight.rs::Scanner::object` accepts an immediate closing brace. The existing typed parser uses the affected json-syntax CodeMap path. Open PR #390 preserves a candidate fix (source commit90109f67) and boundary tests; it has not merged. Its original named fuzz artifact was unavailable, so its tests use a content-equivalent reproducer; preserve that distinction. Historical verification in that PR is useful recovery evidence, not qualification of current main.

Smallest scope: resume the preserved lexical empty-object refusal on an isolated current-main branch; preserve valid empty arrays, escaped braces inside strings, Unicode, diagnostic paths and spans. Reuse useful parser/C-ABI/Wasm boundary regressions. Update the schema's workaround explanation. Review historical workflow changes separately and do not import stale artifact pins.

Acceptance: demonstrate the original failure safely in an isolated baseline process; malformed empty-object placements return diagnostics through parser, C API and Wasm boot; legal documents remain accepted; the issue's seeded 15-minute parser fuzz gate passes. Qualify the actual changed shipped artifact once, then required PR/main CI and synchronized closure. The upstream dependency fix is separately owned by #391; do not turn this mitigation into a dependency migration.

Risk/size: medium, dominated by boundary verification and one artifact qualification; substantial implementation already exists. First of the three because it concerns process survival on untrusted input.


Independent Astra XHIGH scoping PASS; user authorized implementation. Astra LOW implements, Astra XHIGH verifies. Five attempts maximum. Root checkpoints exact paths and pushes promptly. At most two active issues: #387 and #211; #376 queued. #211 is independent tooling, #387 owns session/parser and boundary regression source. Root owns artifact qualification/pinning. No overlapping production edits.

Implementation stops after focused green checks for root commit; independent review follows. Record actual commands, environments, source identity, exits and logs externally. No compiler-IR captures or timing campaigns. Preserve original PR history and failed evidence. Required exact-head PR qualification and corrected-main qualification precede synchronized closure and clean worktree removal.

Before implementation only: current source base14079d2c, no new code or PASS evidence. Historical instructions below are provenance; this current scope and user model/five-attempt routing supersede them.

## Historical issue body

Found by the Fable verifier of #385 (2026-09-04) while fuzzing `session_parse`. **Pre-existing**: identical on `main` with `jstrict 0.14.0` and on #385 with `json-syntax 0.12.5`; the fork inherited the bug from upstream.

## What happens

Any V1 session document containing an **empty object** `{}` (for example `"render_profile": {}`) makes `crates/session` read the parser's `CodeMap` one slot off for every entry after it. Two consequences:

1. **Wrong diagnostics on rejected documents**: `canonical-minimal.json` with `"render_profile": {}` reports a bogus `NumericOutOfSchemaRange` at `output_profile.channels` (the value is `2`) and degenerate spans `151..151` for the real missing-field errors.
2. **Panic**: `canonical.json` with `"render_profile": {}`, or with `output_profile` or a track's `builtins` replaced by `{}`, hits `called Option::unwrap() on a None value` at `json-syntax-0.12.5/src/object/mod.rs:795:67` (on main: `jstrict-0.14.0/src/object/mod.rs:882:67`), reached from `session::parse::Parser::keys` (`crates/session/src/parse.rs:100`). Fuzz artifact: `crash-52d9c906ce5ad7f1d1e67dad91b13ec69e2caab5`.

## Why it matters for launch

Reachable from untrusted input on both shipped hosts: C API `miso_engine_v1_compile_session` (`crates/capi/src/ffi.rs:311`) → `compile_children` (`crates/capi/src/runtime/compile.rs:442`) → `parse_host_session` (`crates/host-core/src/prepare.rs:381`) → `parse_session_json`; wasm `miso_engine_web_v1_boot` (`hosts/host-web/src/ffi.rs:236`) → `WebEngine::boot` (`hosts/host-web/src/lib.rs:790`) → same path. The workspace release profile is `panic = "abort"` (`Cargo.toml:132`), so `catch_result`'s `catch_unwind` (`ffi.rs:24`) does not contain it: **a malformed document aborts the host process or traps the AudioWorklet.** No legal V1 position admits `{}` (all 16 placements tried are rejected), so accepted documents are unaffected; this is a denial-of-service and wrong-diagnostic defect on the reject path. Control plane only; no rendered bit involved.

## Root cause (in the parser crate, not in our code)

json-syntax 0.12.5 never calls `parser.end_fragment(i)` for an empty **object** (`src/parse/object.rs:26-29`; the empty-**array** branch in `src/parse/array.rs` does), so the reserved entry keeps `CodeMap::reserve`'s defaults `span = p..p, volume = 0` (`src/code_map.rs:15-22`). `IterMapped::next` then advances `2 + 0` instead of `2 + 1` (`src/object/mod.rs:795`), and every later entry is read one slot off. Raw CodeMap, identical in both crates:

```
--- "{\"a\":{},\"b\":1}"
  [3] span=5..5 volume=0 text=""        # empty object: end never set
--- "{\"a\":[],\"b\":1}"
  [3] span=5..7 volume=1 text="[]"      # empty array: correct
```

Upstream `timothee-haudebourg/json-syntax` default branch still lacks the `end_fragment` call and its tracker has no report; there is no release to bump to.

## Candidate fixes, in order

1. **Preflight refusal in `crates/session/src/json_preflight.rs`**: reject `{}` anywhere with a proper span and a new diagnostic code (or reuse the closest existing one), before the typed walk runs. Smallest change, fully in our code, and `{}` is never legal V1. Add the fuzz artifact and the three `sed` variants above as regression tests in `crates/session/tests/json_grammar.rs`; add `{}` to the fuzz corpus seeds.
2. Make `parse.rs` stop trusting `volume` for empty objects (treat `volume == 0` as 1 when computing child offsets). More invasive; touches the typed walk.
3. Upstream PR to json-syntax adding `end_fragment` for the empty-object branch, and a `[patch]` until it ships. Do this in addition to 1, not instead.

## Assignment

| | |
|---|---|
| Implementer | Sonnet |
| Verifier | Fable 5.1 (must reproduce the abort through the wasm boot path and the C API before and after) |
| Branch | `sonnet/387-empty-object-panic` off `main` |
| Land after | #385 |
| Class | N/A (control plane; diagnostics only; no rendered bit) |

## Done when

- The fuzz artifact and every `{}` placement produce a diagnostic, not a panic, through `parse_session_json`, through `miso_engine_v1_compile_session`, and through `miso_engine_web_v1_boot`.
- `cargo test --locked --workspace` count matches `main` plus the new regression tests.
- Fuzz `session_parse` for 15 minutes with `{}` seeded: no crashes.
- The wasm artifact re-pin and browser lineage refreshed (session ships in the artifact).

## Current implementation checkpoint

Astra LOW recovered the lexical refusal and parser/native boundary regressions
without historical workflow or artifact pin changes. Added explicit root,
Unicode-path, escaped-string and legal empty-array controls, schema explanation,
and intentional fuzz seed `fuzz/corpus/session_parse/empty-object.json`.
`cargo test --locked -p session -p capi -p host-web` passed 172 tests with four
pre-existing ignores; formatting and diff checks passed. Actual command/env,
source patch and logs: `/tmp/issue387-implementation-20260910`.
The original named crash artifact remains unavailable; preserved regressions
are content-equivalent reproducers. No claim of baseline/actual-Wasm/fuzz or
artifact qualification PASS yet. Root checkpoints; Astra XHIGH reviews next.

## Provisional artifact checkpoint

Root's first probe invocation refused before compilation because its output
directory did not exist (exit2); receipt retained. After creating the required
empty directory, one probe build passed on sourceee779e2f. New provisional Wasm
SHA-256: `80abeec2688be94807bf4086861639fa63e4111df1d1978e37b1288ff2151de4`; prior main pin `ea8f843b254bfc96c277f22d4946db0c33555a281113e221d17f8ec6067a468b`.
This is not artifact PASS: ordinary reproducibility build and the existing
artifact/resource/SDK/browser gates remain. Evidence root:
`/tmp/issue387-artifact-y1odl9dw`.

## Verification setup correction

Independent parser before/after control passed: isolated main14079d2c aborts
on empty render_profile; candidate returns correctly located json.syntax;
legal minimal input passes both. Initial fuzz launch failed before workload
execution because cargo-fuzz replaced target flags and lane D4 rejected missing
AVX2/FMA. Receipt `/tmp/issue387-xhigh-zpkzo4q1/fuzz.log` retained. Root authorizes
bounded verification attempt2 using the existing CI RUSTFLAGS target-feature
settings (+avx2,+fma) for one 900-second seeded invocation. No product change,
completed fuzz time, or fuzz PASS is claimed from the failed launch.

## Independent review and artifact qualification PASS

Astra XHIGH approved unchanged sourceee779e2f: isolated parser/native C ABI
baseline aborts and actual ea8f Wasm trap were reproduced; candidate refuses
with correct diagnostics while legal controls pass. Strict Clippy passed.
Corrected seeded ASAN parser fuzz completed 13,404,077 executions in901seconds,
exit0, no crash artifacts. Initial pre-workload flag failure remains preserved.
Review: `/tmp/issue387-xhigh-zpkzo4q1/review.md` and verdict/manifest receipts.

Root ordinary artifact build reproduced pin80abeec2688be94807bf4086861639fa63e4111df1d1978e37b1288ff2151de4.
All five non-Wasm artifact hashes match qualified main ea8f's payload. Existing
static, resource/native parity, hermetic host, SDK type/headless/package,
three-browser matrix and generated-matrix gates passed. Actual command/source,
exit/log and six-file hashes: `/tmp/issue387-artifact-y1odl9dw`.
No timing claim. Required PR/full-workspace CI and post-main PASS remain.

## Joint delivery scope

Root integrates the independently accepted #387 parser and #211 validator
changes for one delivery PR. Product paths are disjoint; #211 changes only the
validator's dependency edge, tests and authoring guidance, not shipped artifact
source. Preserve both histories and evidence; no new product change is authorized.
Verify validator behavior against the corrected parser on this combined tree,
then independent exact-head review and required PR/full-workspace/main CI.
Both issues remain open until qualification and upstream synchronization finish.

## Combined integration Astra XHIGH PASS

Reviewed headfce9c6276dd5caf277035a250966aa86599a637f preserves both accepted
implementations and histories. Combined validator suite passed all10 tests.
An empty-object CLI probe failed at grammar stage with json.syntax at
$.render_profile, stages2–5 skipped and no canonical stdout. Only the validator
adds the existing effect-compiler dependency; artifact package closures exclude
it. All six retained artifact hashes and eight qualification receipts remain
applicable. Evidence: `/tmp/issue211-387-combined-verification/integration.json`.
Only this review record follows; required PR/main qualification remains.
