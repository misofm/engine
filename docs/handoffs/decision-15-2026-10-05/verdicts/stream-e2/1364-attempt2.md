PASS

# #1364 attempt 2 -- adversarial verdict

Range: attempt 1 (`5abf225f2..6e4254588`) plus attempt 2 (`c7382644c`, on `b16fb4d14`), branch
`codex/d15-stream-e2`. I reviewed `git diff b16fb4d14 c7382644c` against the attempt-1 verdict and
Amendment 1, and checked the whole #1364 change as it is at `c7382644c`. I exported `c7382644c` to
`/tmp/claude-1002/v1364/tree` and built it with its own `CARGO_TARGET_DIR`. Verifier: fresh opus,
2026-10-12.

Attempt 2 fixes M1. A literal `LaneEdit` without a length now encodes the marker on every ramped
kind, through the writer and through the browser transport. Both paths use one helper,
`laneEditSmoothingWord` (`sdk/src/core/writer.ts:110`). The bypass, input-filter, effect-parameter
and observation kinds stay at 0. Every gate is green. The wire bytes of every builder path are the
same as in attempt 1. Each named mutation goes red. There is no BLOCKER and no MAJOR. Root must
ratify three MINOR items before the issue closes, because each is a decision or a label, not code.

## MINOR

**m1. The D3 text no longer matches the code (deviation (a)). The fix is sound, but root must
ratify it.**
- Frozen D3 says "`smoothing()` returns `SESSION_DEFAULT_RAMP`" and "`trackEdit`'s default stays 0".
  The code is different:
  - `smoothing()` (`sdk/src/core/live-controls.ts:311`) returns the checked length or `undefined`.
  - `trackEdit` (`:431`) leaves an absent length absent.
  - Only `laneEditSmoothingWord` turns an absent length into a word.
- The fix is sound. Amendment 1 requires an explicit `u32::MAX` on a ramped kind to throw "on every
  path". If a builder edit carried `0xFFFFFFFF`, the encoder could not tell it from a hand-written
  explicit `u32::MAX`. The encoder would then have to accept one of them, or the builder would
  break. So the frozen D3 text and Amendment 1 cannot both hold. The implementer kept the later
  rule and the observable contract.
- **The wire bytes are the same.** I built every builder method at `b16fb4d14` (attempt 1) and at
  `c7382644c`. That is 22 methods:
  - the 12 ramped kinds, plus the submix fader, mute and pan;
  - `hpfHz`, `lpfHz`, `inputFilters`;
  - an effect parameter, bypass, observe on and observe off.

  I passed each one 7 option sets: absent, 0, 37, `0xFFFFFFFE`, `0xFFFFFFFF`, -1 and 1.5. Then I
  compared the `encodeLaneEdits` bytes and the browser `MisoCommand` objects (154 cases). They are
  identical, and the throw messages are identical too.
- **One observable change.** A builder edit's `smoothingSamples` property is now `undefined`. It
  was `0xFFFFFFFF` in attempt 1, and 0 before #1364. The public `LiveControlBeforeSubmit` hook
  receives these objects, so it sees the change. Nothing in `sdk/src` reads the field outside the
  two encoders.
- Root must reword D3 bullets 1 and 2 to state the rule as the code implements it.

**m2. Two unspecified public exports.**
- `index.ts:65` does `export * from "./core/writer.ts"`. So `checkedRampLength` and
  `laneEditSmoothingWord` are now exported from the package root. I diffed the `Object.keys` of the
  root module at `b16fb4d14` and at `c7382644c`:
  - added: `checkedRampLength`, `laneEditSmoothingWord`;
  - removed: nothing.
- Neither name is in the spec, and the SDK has no API report that would catch this.
- Fix: put the two helpers in a module the package does not re-export (for example
  `sdk/src/internal/`), or root ratifies them as public API. A new file needs a path amendment.

**m3. The authority labels on the SessionRamps answer (item (d)).**
- The text is in the "Amendment 1 (root, 2026-10-12)" sections of the #1364 spec and the #1365 spec.
- My brief says the implementer wrote these notes. If so, they carry a root label that root has not
  confirmed. Also, the #1365 spec is not in #1364's authorized paths.
- The content is correct (see (d)). Root must confirm the words as root's, or relabel them as the
  implementer's finding that root accepts.

## NIT

- **n1. The unramped kinds do not check their word (this predates #1364).**
  - `laneEditSmoothingWord` passes an unramped kind's word through unchecked (`writer.ts:114`).
  - On the headless path, `DataView.setUint32` then wraps -1 to `0xFFFFFFFF` and 1.5 to 1. The
    browser host's `validU32` refuses both.
  - The bytes are identical before and after #1364 (the 154-case diff above). The engine refuses a
    non-zero bypass or input-filter word as `MALFORMED`, so nothing is lost silently.
  - A u32 check on every kind would be a separate small fix.
- **n2.** Verdict 1's n5 is still open. The `sdk/README.md` follow-up has no issue number, and
  `gh issue list --search` finds none. Root should file it.
- **n3.** The new browser eval's `literal()` gives the observation and bypass kinds `rack: 255`. The
  mock host does not check this, so it does not matter. It only reads oddly.

## The items in the brief

- **(a) Shape deviation.** Sound. The wire bytes are the same on every builder path. Root must
  ratify the D3 wording (m1).
- **(b) The `u32::MAX` throw on ramped kinds only: consistent with the engine.**
  - D2 says a sentinel on input filters or bypass is `MALFORMED`, as any non-zero word is.
    `into_input_filter_edit` (`hosts/host-web/src/lib.rs:4376`) and the bypass check refuse it as
    a typed, whole-batch refusal.
  - The effect-parameter path never reads the word. Observation reads it as a window in blocks.
  - Amendment 1 limits the throw to "a ramped kind". So the SDK carries the word, and the engine
    refuses it where it is malformed. No edit is dropped silently, and no ack comes before a drop.
  - The SDK's 12 kinds (`writer.ts:81-84`) match `with_session_ramp` (`lib.rs:4350-4371`) one to
    one.
- **(c) `SESSION_DEFAULT_RAMP` moved to `writer.ts`.**
  - The root module exports it at both commits, with value 4294967295.
  - Neither `./browser` nor `./headless` exports it at either commit, so nothing changed there.
  - `check-sdk-generated.sh` passes. The only additions are the two exports in m2.
- **(d) Can host-web commit a model change without a rebuild? No. I confirmed this from the code.**
  - `compile_ready` has one call site (`lib.rs:2274`, inside the boot chain).
  - `SessionRamps::try_new` has one call (`:7203`). `ReadyOwnership.ramps` is assigned only where
    it is built (`:7406`) and is read only by `with_session_ramp` (`:4942`).
  - host-web and its ffi contain no transaction, model-commit or `control_smoothing` edit path.
  - So no code was needed. Whether #1382 drops or refreshes `SessionRamps` is recorded in its
    hand-off.
- **(e) The deleted follow test had no unique catch. Confirmed.**
  - At `c7382644c` I made the follow record step (ramp 0, `lib.rs:5582`). Six tests went red: gate 1
    and five #1224 follow tests (`a_full_send_queue_refuses_the_strip_mutes_it_follows`,
    `a_delayed_send_follows_like_an_explicit_send_mute`,
    `a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index`,
    `a_solo_follow_ramps_at_the_solo_window`, `each_follow_takes_its_own_source_strips_window`).
  - The n1 doc fix (the allocation test now names its sentinel-only catch) and the n2 doc fix
    (`live_ramps.rs` has no `crate::live_delta` link) are in.

## Test value (new or changed tests in attempt 2)

- **SDK eval: "a literal LaneEdit without a length encodes the marker through the writer's encoder,
  by kind".**
  - Unique catch: `LiveControlWriter.stage` accepts a ramp length that the encoder would refuse
    (SE: only this eval goes red).
  - It also guards three more defects, which other evals catch too:
    - the writer's `?? 0` revert;
    - a kind moved across the ramped line;
    - the marker accepted as a length.
- **SDK eval: "a literal LaneEdit without a length reaches the worklet host with the marker, by
  kind".** Red if `browserCommand` writes `?? 0` again, so a literal edit to the worklet steps (SB:
  only this eval goes red).
- **Changed builder eval: "every ramped builder method ...".**
  - It now asserts that the record leaves the length absent, and that `encodeLaneEdits` writes the
    marker.
  - It still goes red on S1-style defaults (SA, SC and SC2 turn it red).
- **Deleted Rust test:** see (e).

## Mutation runs (mine, at `c7382644c`)

Each SDK mutation was applied in my export. I then ran the full `test/*-evals.mjs` (373 evals)
against the built artifact and restored the file. The restore was checked with a diff against git.

| ID | Mutation | Red |
|---|---|---|
| SA | `encodeLaneEdits` writes `smoothingSamples ?? 0` | 3: the writer eval, the builder wire eval, the engine-level fader eval |
| SB | `browserCommand` writes `smoothingSamples ?? 0` | 1: the browser eval |
| SC | `vcaMute` dropped from `RAMPED_KINDS` | 3: the builder, writer and browser evals |
| SC2 | `routeMatrix` dropped from `RAMPED_KINDS` | 3: the same evals |
| SD | marker accepted (`checkedRampLength` check off) | 3: the marker-not-a-length eval, the writer and browser evals |
| SE | `stage()` does not check the length | 1: the writer eval |
| SF | unramped kinds default to the marker | 9, among them "the kinds that carry no ramp still write 0" and both new evals |
| R-M1 | follow record built with ramp 0 (Rust) | 6: gate 1 and five #1224 follow tests |

## Gates run (all green at `c7382644c`)

- **`lint` job, every step.**
  - fmt; clippy `--workspace --all-targets --all-features -D warnings`; `cargo doc -D warnings`.
  - The policy gates and their mutation tests: workspace, session, env vocabulary, bench,
    test-support CI, host-core, protocol-control, realtime (check, mutations, audit-leak,
    evidence-leak), lane, rack, builtins, graph, effect-runtime and its fixtures.
  - The scalar-oracle self-test, the console fixture, the builtins manifest mutation tests, the
    bench preconditions, the realtime trace validator, the unfused seal and its self-test, the
    conformance boundaries, the parametric-EQ seal, the release-shape self-test, the npm publish
    modes, the stem store and stem identity.
  - The three x86 probes, each in its own target directory.
  - Script reachability needs a git checkout. It failed in the tar export ("not a git
    repository"), so I ran it in a git-initialized copy of the same export: ok, 120 files, 20
    mutation cases.
- **`test-release` job, every step.**
  - Lane, math and wasm-gates; `filter_liveness`; `tail_contract`; `ramp_endpoint` (8 crates);
    `designer_total --ignored`; the FMA cfg check.
  - The loom SPSC leg in its own target directory; M1 exhaustive; F1 fast dB.
- **`test-debug-a`.** The CI feature list with `--no-fail-fast`: 1580 passed, 0 failed, 10 ignored.
  The doctest leg: 24 passed.
- **Other cargo gates.** `cargo test --locked -p capi`; `check-cross-targets.sh` PASS.
- **Worklet chain.**
  - `build-web-audioworklet.sh --named-twin`; the `strip-wasm-names.py` self-test and check.
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
    `check-browser-expected-resources.py --artifacts`; `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh` with a private `TMPDIR`, which was empty afterwards.
  - The V8 spill self-test and check (Node 22.23.2).
- **SDK job.**
  - `npm ci --offline`; `check-sdk-generated.sh`; `check-sdk-deletions.py`; `check-sdk-types.sh`.
  - `check-sdk-headless.sh`: 373 of 373 passed.
  - `sdk-package.sh check`.
- **Wording.** The #1364 diff has no producer, fan or listener wording, nothing that relies on
  decision 16, and no new version suffix.
- **Not run.** The Chromium browser legs, the AArch64-only jobs and `test-debug-b`. No DSP crate
  changed in attempt 2.
