# Delete unrun one-shot scripts and the gate rules that guard retired features

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 10 and §7). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. The isolated-benchmark rows need owner
confirmation R6; the rest need none. **Land after issue 02**, which deletes the env-vocabulary
row-count pin (`test-env-vocabulary.sh:222-223`, `134`) that removing 20 rows would otherwise turn
red.

## Problem

**Scripts no workflow reaches.** 26 files under `scripts/` are not reached from any workflow or
`package.json`, even transitively. The derivation is [`../tools/reach.py`](../tools/reach.py); the
list is in `../data/script-gates-verify.md` §6.
- 8 are operator tools in `scripts/operator/`, by design.
- `scripts/run-console-benchmark.sh` (682 lines) is a live operator runner outside `operator/`.
- **17 are one-shot issue tooling, 3,958 lines** (two of them under `scripts/fixtures/`), whose
  records are already sealed:
  - issue 880 MQ-1/MQ-2, 9 files, 1,409 lines;
  - issue 606 input-symmetry capture, 4 files, 897 lines;
  - issue 746 `run-gate-active-benchmark.py` and its test, 1,394 lines;
  - issue 650 `check-prepared-effect-allocation-records.py`, 245 lines;
  - the `check-parametric-eq-targets.sh` compatibility shim.

`scripts/operator/README.md` claims that every script under `scripts/` is reachable from a workflow.
Nothing checks it.

**The unrun tooling is not free.**
- It keeps 20 rows alive in `docs/ENGINE_ENV_VOCABULARY.md` (9 `MISO_ENGINE_606_*` and 11
  `MISO_ENGINE_CAPTURE_*`), which the env gate then enforces.
- `tools/bench/src/input_symmetry.rs` (597 lines) and `input_symmetry_capture.rs` (226 lines) still
  compile into `bench`.

**Gate rules for retired features**, with file:line in `../data/script-gates-verify.md` §4:
- retired session-TOML spellings (`check-session-policy.sh:46-63`, `check-sdk-deletions.py:120-121`,
  `:409`);
- retired softfma definitions (`check-unfused-seal.sh:723-732`, plus self-test cases);
- the retired delta-bank EQ kernel names (`check-parametric-eq-render-contract.sh:25-28`);
- the pre-boot-v1 SDK spellings (`check-sdk-deletions.py:99-125`). **These fired 4 times in 73 runs,
  each time on new, legitimate code**: `"prepared"` in `prepared-control.js` and `pcm-feed`, and a
  `limits:` key in `live-response.ts` and `response.ts`;
- the superseded raw-WebDriver harness: `scripts/web-audioworklet-browser-correctness.py` (752
  lines), its runner, its operator seal, and the self-test lines `test-web-audioworklet.sh:146-157`
  that CI still runs;
- the builtins-less console rows in the benchmark validators (`test-console-benchmark.sh:531-536`,
  `test-wasm-console-benchmark.sh:164-170`);
- the ban on the word `nudge` (`scripts/check-step-vocabulary.py`), which is already refused as a
  wire key by `check-parameter-metadata-v1.py:733-735`.

## Outcome

- **Delete the 18 one-shot files.** Their records stay under `artifacts/` and are referenced by
  commit hash in their closed issue specs.
- **Move `run-console-benchmark.sh` into `scripts/operator/`,** or reach it from a workflow, so that
  the README is true.
- **Remove the env-vocabulary rows** that only the deleted files used.
- **Under R6,** also delete `tools/bench/src/input_symmetry*.rs`, `gate_active.rs` and
  `multiband_active.rs` (isolated, non-host-path benchmarks) with their tests.
- **Remove each retired-feature rule** above, and its self-test cases.
- **Keep `check-sdk-deletions.py`'s live rule:** no numeric ABI byte offset outside `src/generated/`.
  It has a real defect history, #207 N-13(d).
- **Keep these live rules in `check-workspace-policy.sh`:**
  - the naming rule for the retired `miso-engine-` prefix;
  - the delivery-codec identity ban (`:162-163`, `:218-272`; AGENTS.md: "delivery codecs live
    outside this repository");
  - the tracked-LLVM-IR ban (`:244-249`).

## Scope

Authorized paths:
- the listed `scripts/` files;
- `scripts/check-workspace-policy.sh`, `test-workspace-policy.sh`;
- `scripts/check-session-policy.sh`, `test-session-policy.sh`;
- `scripts/check-unfused-seal.sh`;
- `scripts/check-parametric-eq-render-contract.sh`;
- `scripts/check-sdk-deletions.py`, `scripts/check-step-vocabulary.py`;
- `scripts/test-web-audioworklet.sh`, `scripts/test-console-benchmark.sh`,
  `scripts/test-wasm-console-benchmark.sh`;
- `docs/ENGINE_ENV_VOCABULARY.md`;
- `tools/bench/src/` (R6 rows only);
- `.github/workflows/qualification.yml` (the step-vocabulary step);
- this issue's spec.

## Gates

1. **Reachability.** `python3 ../tools/reach.py <repo> strict <tracked-files.txt> <out.json>` on the
   result, with `git ls-files > tracked-files.txt`: every file at the top level of `scripts/` is
   reached from a workflow or `package.json`, and everything else is in `scripts/operator/`. Add the check as one line to `check-workspace-policy.sh` only if it is one
   line.
2. **Live rules still fail.** In a scratch branch, each of these fails its gate:
   - a numeric ABI offset in `sdk/src/` outside `generated/`;
   - a `mul_add` call outside `lane`;
   - a `MAX_TRACKS` constant;
   - a `core`-named package;
   - a package named `flacenc`;
   - a tracked `.ll` file under `artifacts/`.
3. **The env gate is consistent.** `check-env-vocabulary.sh` passes, with no documented name unused
   and no used name undocumented.
4. **Historical bugs.** Unaffected, since no Rust test is touched unless R6 applies. With R6, run
   `../tools/revert.py` for all four and confirm the red set is unchanged.

## Saving and risk

- **Saving:** ≈ 0 s of CI. Removes about 4,000 lines of scripts, about 1,000 lines of benchmark code
  under R6, and 4 false reds a month from the SDK word bans.
- **Risk:** nil. The records are sealed, and nothing live calls these files.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Shrink this draft to its unique remainder.** Filed issues own the rest:
   - the 17 one-shot files, the R6 benchmark code (`input_symmetry*.rs`, `gate_active.rs`,
     `multiband_active.rs`) and the env rows → #1027;
   - `run-console-benchmark.sh`'s move and arms → #1025 and #1027;
   - the reachability rule → #1022 (merged) and #1027 amendment 3;
   - the builtins-less console rows in the validators → #1025 and #1039.
2. **What stays here:**
   - the retired-feature gate rules: session-TOML spellings, softfma definitions, delta-bank EQ
     kernel names, the pre-boot-v1 SDK word bans, and the `nudge` ban;
   - the superseded raw-WebDriver harness `scripts/web-audioworklet-browser-correctness.py` and
     `test-web-audioworklet.sh:146-157`. #1027 deletes only its operator seal script.
   - Re-title it "Retire gate rules that guard retired features".
3. **Gate 2 keeps the live-rule seeds.** Drop the ones owned elsewhere: the reachability seed goes to
   #1022/#1027, and the tracked `.ll` seed stays.
4. **The dependency on issue 02 stands.** The env row-count pin goes there.

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1050-unrun-scripts-retired-gates`, base `351ca593` (`main`
`a8955ad4` plus the specs of #1069-#1073). Implementation commit `dbf4875e`: 13 files, +86/-1,485.

### Recount on this base

| body item | state on `351ca593` | action |
|---|---|---|
| 17 one-shot files, R6 bench code (`input_symmetry*.rs`, `gate_active.rs`, `multiband_active.rs`), 20 `606`/`CAPTURE` env rows | gone (#1027) | none |
| `run-console-benchmark.sh` outside `operator/`; operator seal of the WebDriver harness | moved / gone (#1025, #1027) | none |
| reachability rule | `scripts/check-script-reachability.py` (#1027) | none; no line added to `check-workspace-policy.sh` |
| env row-count pin | gone (#1043) | none |
| builtins-less validator rows | wasm half gone with `test-wasm-console-benchmark.sh` (#1039); native half `test-console-benchmark.sh:531-536` present | **kept**, see below |
| session-TOML, softfma, delta-bank, pre-boot-v1 SDK and `nudge` rules; the WebDriver harness | present | **removed** |

`test-console-benchmark.sh:531-536` stays. Amendment 1 gave it to #1025, and it is now the only
negative case for the live closed `session_kinds` enum (`console-benchmark-record-lib.jq:191`):
no other mutation feeds a session record an unknown `workload_kind`.

### Removed, and why each is dead

| item | lines | why |
|---|---:|---|
| `scripts/web-audioworklet-browser-correctness.py` | 752 | Superseded raw-WebDriver harness. CI used it for one thing only: `test-web-audioworklet.sh` self-testing the harness's own WebDriver response parsing. `check-browser-expected-resources.py --artifacts` (design §6.3) runs the `direct-oracle.mjs` parity its `--check` ran, and the `browser` job drives real browsers. |
| `scripts/run-web-audioworklet-browser-correctness.sh` | 53 | Its runner. Only a string in the harness reached it. It needs a Chromium and chromedriver pair that no workflow provides. |
| `test-web-audioworklet.sh:146-157` | 12 | The harness self-test and its `sed` mutation. |
| `MISO_ENGINE_CHROMIUM_BINARY`, `MISO_ENGINE_CHROMEDRIVER_BINARY` env rows | 2 | Only the runner read them; the env gate would otherwise report them unused. |
| `scripts/check-step-vocabulary.py` plus its `qualification.yml` step | 156 + 4 | The `nudge` ban. #242 retired the word. The one position where it could come back as a name, the metadata wire key, is refused by `check-parameter-metadata-v1.py:733-735`. |
| `check-session-policy.sh`: live-`*.toml` discovery and the retired-spelling scan, plus `scripts/session-policy-historical-allowlist.txt` | 20 + 13 | The session TOML format was deleted in #338. Kept: the `toml`/`serde` dependency ban on `crates/session`, which is what a revival would have to break. |
| `test-session-policy.sh` cases for those rules (find/sed/sort/rg shims, allowlist, four counter-mutants) | 47 net | Their rules are gone. The anchor counter-mutant stays. |
| `check-sdk-deletions.py` pre-boot-v1 bans: `DELETED_IDENTIFIERS`, `limits` key/member/root key, TOML surface, the `"config"`/`"prepared"` states, error-phase vocabulary, per-source fields, the literal `192`, the boot-v1 presence pins | 648 → 349 | These catch only exact re-adds of deleted code. In 73 runs they went red 4 times, each on new, legitimate code. Kept: the numeric-offset rule (`NUMERIC_OFFSET`, `OFFSET_CONSTANT`, ABI-touching files only, comments blanked), plus its positive half (`abi.ts` reads `ABI_LAYOUT.structures[structure].fields` and resolves `row.name === name`). The self-test goes from 37 mutations to 5, plus 1 commented-offset green control. The filename stays because the router and `check-ci-path-routing.py` pin it. |
| `check-unfused-seal.sh` rule 6 (softfma definition ban), the fixture's `softfma.rs`, 5 self-test cases (red restore, required file, producer full/empty, counter-mutant), the now-unused `fault_empty` parameter | 43 net | #163 phase 2 retired the software FMA. Rule 3 still refuses any *call* to a fused spelling, including `fma_f32_via_f64`/`fma_f32x[48]_soft`, which stay in `call_pattern`. Self-test goes from 62 to 57 cases. |
| `check-parametric-eq-render-contract.sh`: `PreparedDeltaBankKernelV1`, `DeltaBankKernelError`, `KernelBackendV1`, `process_delta` | 4 | The kernel was deleted in #84 and has 0 live occurrences. The lane-division ban (#87 F4) still refuses the defect that made it wrong. |

Consequential comment fixes:
- `clippy.toml`: the unfused seal's rule count, "four of six" becomes "three of five".
- `qualification.yml:245`: drops the stale "37-mutation" count.

The rules that stay live are:
- in `check-workspace-policy.sh`: the `miso-engine-` prefix, the sysroot names, the codec bans and the tracked `.ll` ban;
- in `check-session-policy.sh`: the dependency direction, the `json-syntax` pin, the publication and estimate bans, and the preflight order.

### Gates

1. **Reachability.**
   - `reach.py . strict` over `git ls-files` leaves exactly 2 files unreached, the same set as base. Both are in `scripts/operator/` (`preflight-console-benchmark.sh`, `prepare-builtins-listening.sh`); every top-level script is reached.
   - `check-script-reachability.py`: ok (124 reached, 6 operator-exempt).
   - `test-script-reachability.py`: 18 cases ok.
2. **Live rules still fail.** These seeds ran in a scratch worktree of `dbf4875e`, each reverted before the next. All three gates were green untouched.

   | seed | gate | result |
   |---|---|---|
   | `view.getUint32(24, true)` in `sdk/src/core/abi.ts` | `check-sdk-deletions.py` | red: "numeric byte offset on a DataView accessor" |
   | `a.mul_add(2.0, 1.0)` in `crates/compressor/src/lib.rs` | `check-unfused-seal.sh` | red: "fused multiply-add in crates/compressor/src/lib.rs" |
   | `pub const MAX_TRACKS` in `crates/engine` | `check-workspace-policy.sh` | red: "compiled track-capacity identifiers are forbidden" |
   | package `crates/core` | `check-workspace-policy.sh` | red: "collides with a Rust sysroot/prelude crate name" |
   | package `crates/flacenc` | `check-workspace-policy.sh` | red: "retired delivery-codec identity: flacenc" |
   | tracked `artifacts/seed/kernel.ll` | `check-workspace-policy.sh` | red: "tracked LLVM IR artifact is forbidden" |
3. **Env gate.**
   - `check-env-vocabulary.sh`: ok (63 names).
   - `test-env-vocabulary.sh`: ok.
4. **Historical bugs.** No Rust source or test is touched, and the R6 rows landed with #1027, so `revert.py` does not apply.

Other checks, all rc=0:
- the 11 policy gates (workspace, session, bench, host-core, protocol-control, realtime, lane, rack, builtins, graph, effect-runtime) and their self-tests, 22 commands;
- the unfused seal and its self-test;
- `test-gate-lib.sh`;
- the EQ render contract;
- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- `test-web-audioworklet.sh`;
- actionlint 1.7.7 on `qualification.yml` and `nightly.yml`;
- `clippy.toml` parses.

`cargo check` was not run, because no Rust source changed.

Self-test wall time, base → candidate:
- `check-sdk-deletions.py --self-test`: 19.1 s → 1.8 s;
- `test-session-policy.sh`: 4.2 s → 1.8 s;
- step vocabulary gate plus self-test: 0.8 s → 0;
- WebDriver response self-test: 0.6 s → 0;
- unfused seal self-test: unchanged at about 10 s.

### Follow-ups, outside this spec's paths

- `hosts/host-web/tests/browser-v1/index.html` (4 lines) was served only by the deleted harness.
- `browser-v1/browser-correctness.js` is now run by no browser. It is still read by `check-browser-expected-resources.py`.
- `docs/rulings/de-versioning-inventory.md:108` still says the render contract keeps the delta-bank spellings. It is a dated inventory, left as history.

## Sol verdict, attempt 1

**PASS.** Every removed rule either guards a feature that no longer exists or is covered by a check
that survives. No live script, trigger list or router entry was deleted. Every gate is green on the
merged tree. The findings below are LOW or informational and none needs another attempt.

Reviewer: Sol, 2026-09-29. I merged `9fca5c05` into a scratch detached checkout of batch head
`codex/batch-slim-4` `93105e18` (main plus #1060 and the specs). The merge was clean and had no
semantic conflict:
- #1060 rewrote `check-browser-expected-resources.py`.
- On the merge, `--artifacts` still runs `direct-oracle.mjs` and passes, and its self-test catches
  32 red mutations. This spec's claim that the check subsumes the harness's `--check` still holds.

No timed workload was run.

### Findings, by severity

1. **LOW: the `nudge` evidence overstates its cover.** The evidence says the metadata wire key is
   "the one position" where `nudge` could come back as a name. It is not the only one. I planted
   the word at each position the old gate policed:

   | position | what refuses it now |
   |---|---|
   | parameter metadata: top level, effect, effect row, builtin row, step object, `step` renamed | `check-parameter-metadata-v1.py`. All 6 are red, because every object has a closed key set. |
   | session document: root, track, fader, effect, effect param, automation row | The parser itself: `UnknownField $.….nudge` at all 6 (scratch test in `crates/session/tests/`) |
   | SDK: `export function nudge(…)` in `core/lattice.ts`, re-exported from `src/index.ts` | **Nothing.** `check-sdk-deletions.py` and `tsc` are both green. Only the deleted gate catches it. |
   | Rust identifier (`NudgeLadder`), prose, specs | Nothing. Only the deleted gate catches it. |

   **Why this is not a FAIL.**
   - The contract surfaces refuse the word by structure, not by spelling: the metadata document and
     the session document above, and the control protocol, which carries numeric IDs.
   - What is lost is naming discipline on SDK and Rust identifiers. That is #242's vocabulary
     ruling, not a behavioural claim.
   - Amendment 2 directs this removal explicitly.

   If the owner wants the SDK identifier surface held, one identifier regex in the kept
   `check-sdk-deletions.py` would do it. It runs with comments blanked.

2. **LOW: pins that only the WebDriver harness held, and that no workflow ever ran.**
   - The harness never ran in any workflow. Its runner needs a chromedriver that no workflow
     provides.
   - The only thing CI ran was `--self-test-webdriver-responses`, and `main()` returns from that
     before `load_inputs`.

   Where each claim the harness made now lives:

   | claim | surviving check |
   |---|---|
   | exact artifact set | `check-web-audioworklet.sh:161-163` |
   | raw-Wasm to native digest parity, all 3 legs; observation invariants | `check-browser-expected-resources.py --artifacts`, which runs `direct-oracle.mjs`; that script asserts the native digests and the invariants itself, at `:565-628` |
   | browser PCM equals the native pin in two fresh contexts, simd128 boot | Playwright gates `native-corpus-digest` and `AudioWorklet-boot` (`run.mjs:137-141`), in 3 browsers |
   | ack and backpressure transcript, ownership return | `test-web-audioworklet.mjs:1279-1290` and `:1628-1644` (hermetic, not a real browser) |
   | positive-zero silence; `miso.error.v1` on failure | `test-web-audioworklet.mjs:2814-2835` and `:2890-2896` (hermetic) |
   | resources, status, no memory growth | exact and ceiling rows in `check-browser-expected-resources.py`; `direct-oracle.mjs:443,526` |
   | `content` BLAKE3 identities of the three `tests/browser-v1` session documents; hard-coded command application samples | **none** (none in CI before this change either) |

   The last row loses its only reader. `docs/derivations/241-browser-source-identities.md:196` still
   names the deleted script as that pin.

   **Suggested successor:** extend `qualification/session-identities.mjs`, which already derives the
   qualification documents' identities, to the three `browser-v1` documents.

3. **INFO: softfma rule 6 was fully redundant.**
   - I planted `pub fn fma_f32_via_f64(…)` in `crates/lane/src/softfma.rs`.
   - Rule 3 refuses it: "fused multiply-add in crates/lane/src/softfma.rs". Its `call_pattern`
     matches the definition's `name(` too.
   - A `mul_add` seed in `crates/compressor` is still red.

4. **INFO: the other retired features are gone.**
   - **Delta-bank names.** They have 0 code occurrences. A planted `pub struct
     PreparedDeltaBankKernelV1;` now passes, as intended.
   - **TOML sessions.** The parser refuses a TOML session with `JsonSyntax`. No product crate or SDK
     file has a TOML session entry point, and `check-session-policy.sh` still refuses `toml` and
     `serde` in `crates/session`.
   - **SDK spellings.** None of the sentenced spellings is in SDK code. The per-source fields and a
     `limits` root key are refused by the strict parser. `ErrorPhase` types `phase`
     (`errors.ts:73,89`), so `tsc` refuses a retired phase.
   - **Kept numeric-offset rule.** It still fires on `view.getUint32(24, true)` in `abi.ts`.
   - **Dropped `find`/`sort` counter-mutants.** `test-gate-lib.sh:278-286` still covers them.

5. **INFO: two small out-of-scope edits and three stale references.**
   - The `clippy.toml` edit and the `qualification.yml:245` comment edit are outside the authorized
     paths. Both are comment corrections that follow from the removals, and I accept them.
   - `crates/graph-compiler/Cargo.toml:31` was already stale before this change. It names
     `softfma::fma_f32_via_f64` as the oracle; the oracle is now `unfused_multiply_add_via_f64`.
   - `hosts/host-web/tests/browser-v1/index.html` is now dead, as the evidence says.

### Deleted names in the merged tree

- **Removed scripts and env rows.** `web-audioworklet-browser-correctness`, `run-…`,
  `check-step-vocabulary`, `session-policy-historical-allowlist`, `MISO_ENGINE_CHROMIUM_BINARY`,
  `MISO_ENGINE_CHROMEDRIVER_BINARY` and `--self-test-webdriver-responses` are gone from every
  workflow, script, `package.json`, router table and #1043 `SELF_TEST_INPUTS` key.
- **What still names them.** Only dated records: audits, derivations 241, 242 and 281, the
  `qualification.yml:292` history comment, and specs.

### Gates on the merge, all exit 0

- **Routing.** `test-ci-path-routing.py` and `check-ci-path-routing.py`.
- **actionlint.** 1.7.12 over every workflow.
- **Script reachability.** `check-script-reachability.py` and `test-script-reachability.py`.
- **Env vocabulary.** `check-env-vocabulary.sh` and `test-env-vocabulary.sh`.
- **The `lint` job's hermetic policy commands, 58 in all:**
  - workspace, session, bench, test-support, host-core, protocol-control, realtime, the two leak
    checks, lane, rack, builtins, graph and effect-runtime, each with its self-test;
  - the unfused seal and its self-test (57 cases);
  - conformance boundaries, EQ render contract, release shape, npm publish modes and gate-lib;
  - DSP research, parameter-metadata self-test, command kind and reason vocabularies;
  - `test-web-audioworklet.mjs` and `test-web-audioworklet.sh`.
- **Web artifact gates.** `check-web-audioworklet.sh` and `check-browser-expected-resources.py
  --artifacts`, over a fresh `build-web-audioworklet.sh` build.
- **SDK.** `npm ci`, then:
  - `check-sdk-generated.sh`;
  - `check-sdk-deletions.py` and its self-test (5 mutations, 1 comment admitted);
  - `check-sdk-types.sh`;
  - `check-sdk-headless.sh` (285 of 285);
  - `sdk-package.sh check`.

I did not re-run the four `check-workspace-policy.sh` seeds. That file is untouched, and its gate and
mutation suite are green above.
