# Issue #804 account-switch handoff — 2026-09-14

## Pause and resume instruction

The user requested a pause to resume on another Codex account. Implementation is stopped, all source checkpoints are pushed, and all agents have completed. Do not infer that this issue is finished. Resume from this document and the numbered issue specs, not conversation memory or temporary files. No required running job remains.

Original authorization: complete #804, coordinate engine SDK and adapter releases, integrate and deploy HPF/LPF in misofm/app. Use **Astra XHIGH** for spec-driven design, **bounded Luna XHIGH** implementation assignments, and a fresh **Astra MEDIUM** final adversarial review. The user explicitly selected these models over the repository's default Sol/Terra workflow. One implementation tranche at a time; root commits exact paths and pushes each coherent checkpoint. The current mode is checkpoint pushes, not CI batching. Respect the five-attempt limit; #807 is still attempt 1, with no final whole-issue verdict.

## Authoritative delivery status

| Work | Remote status / evidence |
| --- | --- |
| #147 structured edits and unit metadata | CLOSED, PR #806 merged at `39288df47fa1e8d3df47286e1bcab03064f7413f`; required CI 34811007238 and main CI 34811451810 passed |
| #805 prepared dedicated EQ cuts | CLOSED, PR #810 merged at `80f2918b5aba5b2428c5f5cc76c24f46b4e0edde`; required CI 34820834881 and main CI 34821473670 passed; Astra MEDIUM PASS |
| #807 live EQ prepared targets | OPEN, assignments 1–3 pushed; assignments 4–10 pending; no PR yet and no public live-cut activation |
| #808 live builtin filters / disabled DSP elision | OPEN, brief complete, implementation not started |
| #809 SDK release | OPEN, brief complete; no new package published |
| engine-web-adapter #111 | OPEN, spec only; no dependency or source changes |
| app #222 | OPEN, spec only; no integration, release or deployment yet |
| #804 parent | OPEN until the complete release and app outcome is verified |

Do not report three assignments out of ten as a percentage: owner/host/browser integration and qualification remain substantial work.

## Checkouts and recoverability

Engine active worktree `/tmp/miso-engine-807`, branch **`codex/807-live-eq`**, source HEAD and upstream at pause **`e81df55b61f563fa26952608e7383d3c060cb956`**, clean before this documentation checkpoint. Fetch and check out that branch on another machine; this handoff will be a subsequent pushed documentation commit.

Engine primary `/home/bl/misofm/engine` is clean synchronized main at `80f2918b`. Completed `/tmp/miso-engine-804` was removed cleanly; history remains. Preserve unrelated worktrees `/tmp/issue687-stage2-source`, `/tmp/issue688-source`, `/tmp/miso-engine-774`, and `/tmp/miso-engine-selective-analysis-spec`. #774 is paused after failed review and is **not a dependency** of this work.

Adapter worktree `/tmp/miso-adapter-804`, branch `codex/804-filter-release`, clean and pushed at `23ad14f731166584f739915a3542fcdfa6d6a2cb`, base `d2492ef898c9285864eebf6903503a5fb73069e3`. Matching spec: `.github/ISSUE_SPECS/111-release-the-adapter-with-the-live-filter-engine-sdk.md`.

App worktree `/tmp/miso-app-804`, branch `codex/804-eq-cuts`, clean and pushed at `715a6c2b520fbea3775edd4624b2fb7187c9c7bd`, base `dc9407c2136b23c4b248f2ec13e3c42d67c7bd70`. Matching spec: `.github/ISSUE_SPECS/222-adopt-live-dedicated-eq-cuts-from-engine-804.md`. **Primary `/home/bl/misofm/app` contains unrelated dirty user work. Do not reset, stash, or edit it.** The primary adapter also has an unrelated research branch.

## Frozen product decisions

One existing EQ is HPF → original four bands → LPF. Dedicated cuts are fixed 12 dB/oct TPT SVFs with independent Q; higher slopes remain #191. Preserve original band IDs and semantics; no fake slope control. Original numeric IDs remain 1–6,17–22,33–38,49–54; new HPF enabled/frequency/Q are 65/66/67, LPF 81/82/83. Response IDs 1–4 remain original bands, 5 HPF, 6 LPF; physical mapping `[1,2,3,4,0,5]`.

App uses dedicated EQ cuts, hides builtins, and authors both builtin cutoffs as zero. Disabled builtins must do no SVF/integrator work even during trim or polarity ramps; no zero-storage claim. Preserve imported authoritative builtin sound instead of silently zeroing or migrating it. Engine still provides live builtin filter controls for other consumers.

#147 uses existing metadata name as machine key, authoritative Rust units, and typed object `key` + `value` edits with positional compatibility. No wholesale threshold/threshold_db rename. Sessions remain strict canonical versioned JSON. Never inspect a legacy engine implementation.

## #807 completed implementation and evidence

1. `6376fb2f364ab0c336a4cce6bc478bf336bf3f63`: 56-byte Copy `PreparedEffectTarget` (slot, channel, 12 words), preparation request/trait/errors and default unsupported hooks; SPSC `available_at_entry()` freezes one acquired producer snapshot. Effect-contract 46 tests, SPSC 6, Loom 1, native/Wasm checks passed.
2. `c947199ff43ccbe32dafde422548be70b44ec4ec`: actual EQ control preparer/validator over canonical 60 rows, maximum 12 ordered/coalesced targets. Validates whole candidate before writes. Fixed stack preparation, shared rounded-SVF validator, conservative mix bound 128. Numerical validation cannot prove semantic/coefficient equivalence: a stable forgery test demonstrates why matched Rust preparer provenance remains mandatory. Complete EQ tests, contract, Wasm and strict Clippy passed.
3. `e81df55b61f563fa26952608e7383d3c060cb956`: scalar/bank target application, cached reset coefficients, dedicated enable state and current-then-advance ramps. Sample A uses current coefficients; exact target first used at A+64. Serialized state is **464 bytes/channel, 936 total including 8-byte common header**. Refuse old payloads and malformed state atomically. Restore checks only actually processed ramp states, exact +0 settled steps and exact identity bits; failed restore preserves both channels and fixed-point witness. General-band kind/enabled remain immutable. Actual retained response observes targets.

Assignment 3 gates: 32 unit, 7 analytic, 6 bank, 1 conformance, 18 contract, 3 determinism, 3 mono, 18 response, 3 silent, 4 stationary, 5 time-domain tests passed; existing ignored tests unchanged. Strict all-target Clippy, Wasm, formatting and eight graph track-delay tests passed. No benchmark was run. The focused Astra MEDIUM restore advisory is **not** the final issue review.

Intentional ramp timing changed only ramped-noise corpus SHA from `c7d053d96d0810cf6eb9ecf820d68a738133d24ae34a27e4e8c90db390d50465` to `452fc6b5dc02f8fdb626aebd3eae8bea508f41d5f6338319828eca204419b79e`. Static four-band disabled-cut compatibility remains covered. Nine EQs add 9×16=144 declared-state bytes; only canonical estimate fields changed: 8280→8424 and both 150415→150559 totals. Independently derived fixture hash is `eb3ca77606e93cf9aa13f475415ecbf6e70ee1cdd074a0cca9e46cbb18e0ea10`; expected canonical file is preserved here.

**Not yet active:** factory `target_preparation()` remains None. No prepared queue variant/owner shadow/host or SDK lowering exists. The old numeric `automate` path still designs in process until assignment 10. A test currently detects that old route; at cutover replace its design-positive assertion with refusal/no-design coverage. Do not expose live metadata early.

## Exact next action

Read the full numbered #807 spec and `807-bounded-tranches.md`. **Before assigning implementation 4, resolve and record the staging-ownership choice in `807-assignment-4-resume-amendment.md` in this directory.** That file preserves final Astra XHIGH findings. The copied original assignment-4 brief predates these findings and is incomplete on producer safety/accounting. No assignment-4 implementation has started. This is a root design decision within existing authorization, not a need to ask the user again.

Then delegate one bounded Luna XHIGH tranche, checkpoint it after proportional tests, and continue assignments 5–10 sequentially. Do not give one agent all remaining integration work. Queue targets must remain FIFO across batches (Left X, Both Y, Left Z => left Z/right Y), and bank application must follow desymmetrization. No acknowledged target may be dropped.

Remaining sequence: 4 queue/runtime delivery; 5 owner shadow/transaction/accounting; 6 stateless preparation workspace and additive FFI; 7 host whole-batch admission; 8 browser private helper/worklet transport; 9 headless/package helper; 10 atomic activation, removal of audio-thread design, six live metadata flags, rebuilt artifacts, actual browser/headless ACK/PCM/response and adversarial gates, fresh Astra MEDIUM final review. Then required CI, merge, and synchronized issue closure.

Browser preparation must run off AudioWorklet, using a lazy no-boot/no-render helper with the verified WebAssembly.Module; headless reuses its existing instance between renders. No second engine/graph/source owner. Shared `prepared-control.js` will serve #808. Whole mixed batch validates/preflights before publication; shadows commit after publication; ACK last. Global maximum 512 targets derives from 2×256 commands, not a track limit. One pending helper EQ batch; typed busy and cache invalidation/reseed on failed or ambiguous ACK. Public APIs accept no coefficient sidecars.

## Following slices and release

#808 reuses #807 helper and transaction owner. Live builtin HPF/LPF IDs 3/4, +0 disabled, fixed Butterworth Q, pair invariant hpf==0 || lpf==0 || hpf<lpf. Proposed command kind 12 must be checked for availability at implementation time. Atomic pair form enables crossing updates. Filters use current-then-advance 64 updates; existing trim timing remains. Preserve target/current/ramp during collapse and copy integrators appropriately on desymmetrization. Settled disabled cuts skip filter state traffic during trim ramps. See numbered #808 and preserved detailed brief.

#809 chooses an unused minor version at freeze (0.3.0 only an example). #147/#805/#807/#808 must be accepted first. Existing npm-publish qualify→publish→verify uses exact merged source, actual browser matrix, one immutable archive, and existing OIDC/provenance guards. Publish the same archive without rebuilding; ambiguous publication means verify only. **No new SDK/adapter release has happened.** Do not modify immutable SDK 0.2.6 guards until release planning calls for it.

Adapter #111 pins the accepted SDK exactly, preserves the existing borrowed session.engine, publishes/verifies the exact qualified archive and tests a fresh ordinary consumer with one SDK. App #222 adopts actual registry packages and provenance, no copied Wasm or sibling links. Read app design principles. Preserve four-band frequency/gain/Q tuples (fix frequency-only sorting), generate bindings from installed CATALOG, map HPF/LPF IDs above, add Q, remove fake slope controls, use t() en/ja. Preserve imported builtin response once; no local filter response math. Actual continuous PCM episode must cover enable/sweep/Q/disable, ACK/response, bypass, pause/resume and old-band independence. Main deploys to testnet; verify deployed revision/provenance and live-cut smoke, then notify user.

App baseline check/lint/typecheck/test/build was investigated: all except `bun run check` passed; pre-existing formatting failure `src/lib/hls-audio.test.ts` in isolated base must be resolved before final delivery. Detailed app plan is preserved here and in app #222.

## Artifact and evidence cautions

Copied supplemental plans/evidence in this directory are historical checkpoint records; numbered issue specs and the resume amendment govern next work. Some copied files mention /tmp paths; required decisions/results are preserved here, but old logs are not prerequisites. Final qualification must run on the final source/artifact.

The #805 artifact remains on this machine at `/tmp/804-805-response-artifacts`, SHA `cc128e5f26df1bb13d981d5700c387de9a09143ca2cc0acfa8ad7c36b426c5e0`. It passed 254 headless tests, package/artifact gates and Chromium 151.0.7922.34, Firefox 153.0, WebKit 26.5. **It is not a #807 final artifact.** Rebuild and repin at assignment 10. `/tmp/805-run-browser.sh` references the removed #805 worktree and must be adapted, not run unchanged. Browser tooling was installed locally; another machine may need installation.

Use the repository build/codegen/package scripts and frozen qualification workflow. The AudioWorklet repin command needs an existing empty output directory and deliberately stops after printing its digest before copying; update the pin and then run the normal builder. No additional broad benchmark framework is authorized. Preserve meaningful tests and record follow-up tooling separately.

After resumed merged delivery, synchronize numbered local specs and GitHub bodies/states, verify closure, and clean completed worktrees only when all work/evidence is pushed. Keep active and unrelated worktrees and dirty user changes intact.
