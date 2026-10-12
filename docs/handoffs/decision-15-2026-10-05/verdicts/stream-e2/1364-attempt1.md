FAIL

# #1364 attempt 1 -- adversarial verdict

Range: `5abf225f2..6e4254588` (4 commits) on `codex/d15-stream-e2`. Export at `6e4254588`, own
`CARGO_TARGET_DIR`. Verifier: fresh opus, 2026-10-11.

The engine half is correct. The four host-web changes, the `LiveRamps` move, the SDK builder change
and the parity gate do what the spec says, and every gate is green. The attempt fails on one MAJOR:
a public SDK path still writes 0 (a step) when an edit gives no length. Decision 15 D15-1 forbids
this ("SDK defaults stop being 0"; "the caller must not decide smoothness by omission"). The spec
did not authorize the two files that need the fix, so root must amend the spec. The fix is small.

## MAJOR

**M1. A literal `LaneEdit` with no `smoothingSamples` still encodes 0 (a step) on every ramped kind.**
- `sdk/src/core/writer.ts:160`: `encodeLaneEdits` writes `edit.smoothingSamples ?? 0`. The headless
  engine (`sdk/src/headless/engine.ts:266`) and `LiveControlWriter` (`writer.ts:201`) use it.
- `sdk/src/browser/live-controls.ts:29`: `browserCommand` writes `edit.smoothingSamples ?? 0`. This
  is the path that sends edits to the worklet.
- `LaneEdit` is a public export (`sdk/src/index.ts:65`), and `smoothingSamples` is optional on it
  (`writer.ts:56`, no doc comment). `EngineLiveControls.submit(...edits: LaneEdit[])` and
  `LiveControlWriter.stage` take plain objects, and `sdk/test/writer-evals.mjs:105-116` builds them
  as literals. So an app that writes `{ kind: "faderDb", trackIndex, rack: 255, channel: 2, values }`
  with no length gets a hard step on both the browser and the headless engine. With the builder it
  gets a ramp.
- Why this is MAJOR: the product outcome says "a browser app that sends a live fader ... edit
  without `smoothingSamples` hears it ramp" and "Decision 14's F7 (the SDK half) is closed". D15-1
  says "SDK defaults stop being 0". Only the builder path now meets these statements. The attempt
  record says F7's SDK half is closed and does not name this path.
- Not the implementer's fault: the spec's D3 names only `smoothing()`, and its authorized paths do
  not list either file. But the gap stays when the issue closes, and a later issue would have to
  fix it. The owner principle does not allow that.
- Fix (spec amendment first): authorize `sdk/src/core/writer.ts` and `sdk/src/browser/live-controls.ts`.
  Make the absent default depend on the kind: `SESSION_DEFAULT_RAMP` for the twelve ramped kinds,
  0 for the others. The bypass and input filters must stay 0, and the observation kinds read the
  word as a window in blocks. Use one shared helper so the two encoders cannot drift. Document
  `LaneEdit.smoothingSamples`. Add one eval: a literal ramped `LaneEdit` with no length encodes
  `0xFFFFFFFF` through both encoders, and a literal bypass with no length encodes 0.

## MINOR

**m1. `a_followed_mute_send_ramps_over_the_resolved_mute_length` has no unique catch**
(`hosts/host-web/src/tests.rs:16259`).
- Mutation M1 builds the follow record with a step. It turns this test red, and also six other
  tests: gate 1's test (its `solo` and `vcaMute` rows stage the follow records of `bass-room` and
  `drums-verb` and check them) and five existing #1224 follow tests.
- The test's doc names a different red mutation: "resolving the strip mute after the follow pass
  has copied its word". The current code cannot have that defect, because the follow pass reads
  staged records, and resolution runs at decode.
- I could not find a plausible defect that only this test catches. Delete it, or fold its one new
  case (a direct kind-4 mute on a strip that has a `follows_mute` send) into gate 1's mute row.
  For example, put the mute row on `bass` (index 0), not `vocal`.

## NIT

- **n1.** `resolving_a_sentinel_allocates_nothing` (`tests.rs:16419`): the doc names the wrong
  mutation. An allocation in every record (M9) turns red 8 existing allocation tests too. The
  test's real unique catch is an allocation only on the sentinel path. Mutation M9b (allocate only
  when the word is `0xFFFFFFFF`) turns only this test red. Change the doc to name M9b.
- **n2.** `crates/host-core/src/live_ramps.rs:3` links to ``[`crate::live_delta`]``. That module
  does not exist without `control-provider`, so `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core
  --no-deps` fails on it. The workspace doc build passes because features unify. The crate already
  has the same problem (`SessionControlProvider`, line 19), so this is not new. Use plain code text.
- **n3.** `SessionRamps::try_new` (`hosts/host-web/src/lib.rs:1537`) reserves 4 bytes per strip even
  when the session has no live controls. In that case `LiveVcaState` allocates nothing. The bytes
  are charged and stay within budget, so this is cosmetic.
- **n4.** Gate 1, input filters (deviation (e)): a prepared path exists
  (`submit_prepared_commands`, `lib.rs:3139`), so an end-to-end refusal was possible. The
  decode-level test is enough: resolution leaves the word unchanged for this kind, and
  `into_input_filter_edit` is the only check that reads it. M6 turns the test red.
- **n5.** The attempt record lists `sdk/README.md` as an open item but names no successor issue.
  Root should file one, or fold it into the M1 amendment.

## The five deviations (a)-(e)

- **(a) `LiveRamps` / `LiveRampRow` moved to an ungated `live_ramps.rs`: correct shape, accepted.**
  - The move is byte-identical. I compared the old `live_delta.rs` lines 35-147 at `5abf225f2`
    with the new file. The new file adds only the module header and the `use session::SessionModel`
    line, and one trailing blank line goes away.
  - The re-exports keep the same public names.
  - The browser artifact has no classifier or protocol code. The named twin has 0 occurrences of
    `classify_live_delta`, `live_delta` and `protocol`. `cargo tree -p host-web --target
    wasm32-unknown-unknown -e normal` shows `host-core` with only its default features, and
    `-i protocol` prints nothing.
  - The other choice was to turn on `control-provider` for host-web. That would link `protocol`
    and the classifier into the shipped module. The move is the right choice.
- **(b) Files outside the authorized paths: all forced by gates, accepted.**
  - The two `.d.ts` copies are byte-identical (`diff` is empty), and `check-sdk-generated.sh`
    requires them to be.
  - The `check-realtime-policy.sh` row adds one test file to the list of files that may use
    `unsafe`, beside the existing capi test rows. The parity test calls the exported C entries, so
    it needs `unsafe`.
  - `protocol` is a dev-dependency only, used to build the transaction frames. `Cargo.lock`
    changes by exactly two lines.
- **(c) The per-strip table: control-thread only, charged correctly, within budget. Accepted.**
  - The table is allocated once in `compile_ready`. Its only caller is `boot` (`lib.rs:2274`), and
    `ReadyOwnership` is built in one place (`lib.rs:7385`). It never runs in `process()` or render.
  - The table is T+S entries of 4 bytes, added to bridge metadata, bridge retained bytes, the
    largest bridge allocation and the largest named allocation.
  - Mutation M5 drops the charge, and both exact-sum tests go red. Those test shapes have no
    submixes, so their `track_count * 4` term is exact.
  - `check-browser-expected-resources.py --artifacts` and `retained_ceilings.rs` are green with
    `expected.json` unchanged.
- **(d) The prepared pan window: a gap the spec accepts, not a defect.**
  - D2 says "the strip's `smoothing_samples` in the session host-web prepared". The Non-goals give
    the committed-model version to #1382.
  - The browser's pan command cannot change a strip's window, so the browser has no edit that
    matches it. A rebuild runs `compile_ready` again, which recomputes the windows.
  - The parity test keeps the prepared window on the C ABI side. That is the comparison the spec
    asks for.
- **(e) The input-filter sentinel checked at decode level only: accepted** (see n4).

## Other checks

- **Row map against #1054 D3.** `with_session_ramp` (`lib.rs:4350-4371`) maps:
  - fader, trim, send gain and VCA offset to the fader key;
  - mute, solo, send mute and VCA mute to the mute key;
  - polarity to `PolarityInvert`, which is twice the mute length;
  - pan and matrix to the strip window, else `pan_ms`;
  - send matrix to `pan_ms`.

  All other kinds return the word unchanged.
- **Polarity at twice the mute length.** Gate 1's custom table (mute 5 ms) expects 480 for
  polarity. M2 (polarity on the plain mute row) turns the test red.
- **`follows_mute`.** The follow record takes the ramp of the resolved strip mute: 240 at
  `mute_ms` 5, and 480 under the default table for the solo and VCA-mute rows. The browser has no
  command that toggles `follows_mute`, so the C ABI's `SendFollowsMute` toggle row has no browser
  counterpart. This is consistent.
- **An explicit length, 0 included, is kept.** Gate 1 checks 37 and 0 for every kind, and M8 turns
  it red. The SDK's engine-level eval shows that an explicit 0 steps under the default table.
- **`u32::MAX` from the SDK is a `MisoUsageError`.** S2 turns the eval red.
- **Gate 3 is bit for bit.** The test compares `to_bits()` with `differing == 0` on every block
  0..47, at 44.1, 48, 88.2 and 96 kHz, with no tolerance. It covers the ramps and the time after
  them, and a stepped control run must differ in at least 8 blocks.
  - M4 (strip windows ignored) and M7 (the table rounded at 48 kHz at every rate) turn it red.
    M7 is its unique catch, because gate 1 runs only at 48 kHz.
  - Limit: gate 3 uses only the default table and the fader, mute and pan rows. That is what the
    spec asks for.
- **The acked-batch question: no.**
  - No queue, admission order or ack changed. Resolution rewrites one word before any check, and
    refusal is still whole-batch.
  - A resolved length is at most 192,000 samples (polarity at 1000 ms and 96 kHz). The route ramp
    bound is 2^22, so no new `DOMAIN` drop can appear after an ack.
- **Wording.** No producer, fan or listener wording, nothing that relies on decision 16, and no
  new version suffix.

## Test value (one sentence per new test)

- `an_absent_ramp_resolves_to_the_sessions_row_and_a_given_one_is_kept`: red if a ramped kind
  skips resolution, is on the wrong key (polarity on the fader key or at the plain mute length,
  which only the custom table shows), or loses a finite length (M2, M3, M8 each red, no other test
  red).
- `a_pan_or_matrix_without_a_length_takes_its_strips_window`: red if the browser ignores a strip's
  own pan or matrix window (#1054 D5), or gives a send matrix a strip window (M4: only this test
  and gate 3 are red).
- `the_kinds_without_a_ramp_keep_the_sentinel_and_refuse_it`: red if resolution also rewrites the
  word for bypass, input filters, effect parameters or observation (M6: only this test is red).
- `resolving_a_sentinel_allocates_nothing`: red if the sentinel path alone allocates, for example
  a table built lazily (M9b: only this test is red; see n1).
- `a_followed_mute_send_ramps_over_the_resolved_mute_length`: no unique catch (m1).
- `control_smoothing_parity.rs`: red if the two hosts disagree on a length because of rate
  rounding or the strip window (M7: only this test is red).
- SDK `every ramped builder method writes the session-default marker...`: red if `smoothing()`
  still defaults to 0 (S1).
- SDK `the marker is not a length`: red if `4294967295` passes as a length (S2: only this eval).
- SDK `the kinds that carry no ramp still write 0`: red if `trackEdit` defaults to the marker
  (implementer's run; not repeated).
- SDK `a fader move without a length ramps...` (shipped module): S1 turns it red together with
  the unit eval. Its unique catch is the JS host or wasm path refusing or changing `0xFFFFFFFF`
  (`validU32` in `miso-engine-v1-audio-worklet-host.js:270`). No native test reaches that path.
- builder-evals `explicit controlSmoothing round-trips` / `refused at its path`: red if the option
  is dropped (S3).
- builder-evals strict request: red if the request key is dropped or two lengths are swapped (S4
  and S5: only this eval).

## Mutation runs (verifier's own)

Each mutation was applied to a separate export of `6e4254588`. For each one I ran
`cargo test -p host-web --features test-support --no-fail-fast` (202 tests), then restored the file.
The SDK mutations ran `node --test test/live-controls-evals.mjs test/builder-evals.mjs`
(72 evals) against the built artifact.

| ID | Mutation | Red |
|---|---|---|
| M1 | follow record built with ramp 0 | 7: the follow test, gate 1 and 5 existing #1224 follow tests |
| M2 | polarity on `LiveRampRow::Mute` | gate 1 only |
| M3 | solo on `LiveRampRow::Fader` | gate 1 only |
| M4 | strip pan windows ignored | strip-window test, parity |
| M5 | ramp table not charged | the two exact-sum resource tests |
| M6 | every other kind resolved as `Mute` | kinds-without-a-ramp test only |
| M7 | table rounded at 48 kHz at every rate | parity only |
| M8 | a finite word 37 replaced | gate 1, strip-window test |
| M9 | one `Box` in every record | 8 existing allocation tests, the new test and `render_locked_source` |
| M9b | one `Box` only on the sentinel | the new allocation test only |
| S1 | `smoothing()` defaults to 0 | marker eval, engine-level eval |
| S2 | marker accepted as a length | marker-not-a-length eval only |
| S3 | builder ignores `controlSmoothing` | round-trip and refusal evals |
| S4 | request drops `controlSmoothing` | strict-request eval only |
| S5 | request `muteMs` reads `faderMs` | strict-request eval only |

## Gates run (all green at `6e4254588`)

- **`lint` job, complete.** fmt; clippy `--workspace --all-targets --all-features -D warnings`;
  `cargo doc -D warnings`. Then the workspace, session, env-vocabulary, bench, test-support CI
  (check and mutation tests), scalar-oracle self-test, script reachability, console fixture,
  builtins fixture manifest mutation tests, bench preconditions, host-core, protocol-control,
  realtime (check, mutations, audit-leak, evidence-leak), realtime trace validator, lane, unfused
  seal, rack, builtins and graph, effect-runtime and fixtures, conformance, parametric-EQ seal,
  release-shape, npm publish modes, stem store and stem identity gates. Last, the three x86
  sub-v3 and AVX2+FMA probes, each in its own target directory.
- **`test-release` job, complete.** Lane, math and wasm-gates (G5/G6); `filter_liveness`;
  `tail_contract`; `ramp_endpoint` (8 crates); `designer_total --ignored`; the M3 FMA cfg check;
  the loom SPSC leg in its own target directory; M1 exhaustive; F1 fast dB. 210 tests passed,
  0 failed.
- **Spec gates.**
  - `test-debug-a`, the CI feature list with `--no-fail-fast`: 1905 passed, 0 failed, 12 ignored,
    and its doctest leg passed.
  - `cargo test -p host-web --features test-support` and `cargo test -p capi`.
- **Worklet chain.** `build-web-audioworklet.sh --named-twin`; `strip-wasm-names.py` self-test and
  check; `check-web-audioworklet.sh --without-metadata-regeneration`;
  `check-browser-expected-resources.py --artifacts`; `check-scalar-oracle-absent.py --wasm`;
  `test-web-audioworklet.sh` with a private `TMPDIR` (nothing left in it); the V8 spill self-test
  and check (Node 22.23.2).
- **SDK job.** Offline `npm ci`; `check-sdk-generated.sh`; `check-sdk-deletions.py`;
  `check-sdk-types.sh`; `check-sdk-headless.sh` (371/371); `sdk-package.sh check`.
- **`check-cross-targets.sh`:** PASS.
- **Not run:** the Chromium browser legs, AArch64-only jobs and `test-debug-b`. The spec does not
  ask for these, and no DSP crate changed.
