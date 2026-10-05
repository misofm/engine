# Grow the default source ring by the warm-prime headroom

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment)).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the ring
headroom and every pin of the ring rule outside host-core, split from *Fall back to the transition
when a warm successor is not ready by its deadline* (#1358) by the round-6 review (M5). Code anchors
verified on `main` at `6fb211594`.

## Product outcome

A host that keeps its source rings full on the default ring (`source_ring_frames = 0`) always has
room for a warm successor's prime, at every launch rate and every quantum the engine accepts. The
engine, the published ABI layout, the SDK and the browser host derive the same ring.

## Context

- `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65-77`) is the stall tolerance
  (`SOURCE_STALL_TOLERANCE_MS = 100`, `:57`) rounded up to quanta, plus two quanta. It has no room
  for `P + q` frames queued past the consumer beyond that tolerance. *Prepare a warm successor
  whose carried nodes lead the predecessor by P* (#1354) D1 adds `stall_ring_frames(fs, q)` with
  that body, and its D2 step 7 refuses warm preparation (`LeadBound`) on a ring whose frames above
  `stall_ring_frames` are fewer than `P + q`.
- *Record the swap block's cost on the 64-track console* (#1286) D3 item 1 defines
  `P_MAX_SAMPLES(fs, q) = 4 · ceil_q(L_max(fs))`, with `L_max(fs)` the true-peak limiter's
  `fs / 100 + 6` (`crates/true-peak-limiter/src/lib.rs:236-242`) unless the record finds a larger
  one. It is taken at the session's quantum, so it is a whole number of quanta.
- A ring that is not a whole number of quanta is refused (`host.source.ring_frames`,
  `crates/host-core/src/prepare.rs:167-170`), on both default paths
  (`crates/capi/src/runtime/compile.rs:705-707`, `hosts/host-web/src/lib.rs:2100-2101`). The SDK
  boots 96 kHz at quantum 127 (`sdk/test/boot-evals.mjs:71-84`).
- **The published rule.** The ABI layout document's `sourceRing` carries the rule's inputs
  (`tools/parameter-metadata/src/abi_layout.rs:109-115`, `:2266-2270`). These pin the rule or its
  values: `tools/parameter-metadata/tests/abi_layout.rs:262-292`,
  `scripts/check-abi-layout-v1.py:591-595` and its self-test mutations (`:653-656`),
  `scripts/fixtures/abi-layout-v1-self-test.json:666`, the generated
  `sdk/assets/miso-engine-v1-abi-layout.json:666` and `sdk/src/generated/abi.ts:2421-2424` (kept
  equal by `scripts/check-sdk-generated.sh`), the SDK's `defaultSourceRingFrames`
  (`sdk/src/core/abi.ts:206-212`), `hosts/host-web/src/tests.rs:1508-1539`,
  `crates/host-core/tests/prepare.rs:618-633` and `crates/capi/src/runtime/tests.rs:626-629`.
- **SDK and browser pins.**
  - `sdk/test/boot-evals.mjs:83-84` and `:318`, `sdk/test/browser-evals.mjs:276` and
    `sdk/test/builder-evals.mjs:296` assert 9,906 (78 · 127) at 96 kHz and quantum 127.
  - The browser host's per-source in-flight bound mirrors the rule when the ring override is 0
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1728-1732`): 40 quanta at 48 kHz
    and quantum 128. Under the old bound the host would never fill the new 57-quantum ring.
  - The host's declaration `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and its
    mirror `sdk/src/browser/shipped-host.d.ts` (kept equal by `scripts/check-sdk-generated.sh`)
    describe the old rule at `:145-147` and `:785`. `hosts/host-web/MUTATIONS.md:7` and
    `docs/derivations/243-sdk-boot.md` (section 3) quote its values.
- **The qualification record.** `hosts/host-web/qualification/run.mjs:211` and
  `qualification.js:6-8` pin 5,120, but the stall test passes that ring explicitly
  (`bootOptions(DEFAULT_RING_FRAMES)`, `qualification.js:497-501`) and tests the stall body, which
  stays 5,120. `hosts/host-web/qualification/rebuild-cost.mjs:66-74` and
  `scripts/web-mixing-automation-benchmark.mjs:106-110` compute the stall body from the two stall
  inputs and pass it explicitly. None of them changes. Only the recorded label does: the record
  writes `defaultRingFrames` (`qualification.js:715`, `run.mjs:926`), `results.json:8` holds it,
  the matrix prose calls it "the default source ring" (`generate-matrix.mjs:24`), and
  `BROWSER_DEPLOYMENT_MATRIX.md:12` is generated from them. `run.mjs:915-916` requires that
  document to equal `renderMatrix(results.json)`.

## Decisions frozen for this slice

- **D1. `P_MAX_SAMPLES`.** `pub const fn P_MAX_SAMPLES(sample_rate_hz: u32, quantum_frames: u32)
  -> u32` in `crates/host-core/src/prepare.rs`, beside `stall_ring_frames`, re-exported from
  `crates/host-core/src/lib.rs`, with #1286 D3 item 1's value; its comment names the record row.
  #1358 D1, D2 below and the production `WarmConfig` (*Check the warm-successor deadline in
  miso_engine_v1_service and report its outcome*, #1360 D1) read it. No other spec restates it.
- **D2. Ring headroom.** `default_source_ring_frames(fs, q)` becomes
  `stall_ring_frames(fs, q) + P_MAX_SAMPLES(fs, q) + q`: #1354 D1's stall body, plus the largest
  `ΣP`, plus one quantum. That is the most frames readiness asks to be queued (`(k + 1) · q` with
  `k · q <= P_MAX`), on top of the stall tolerance the ring already gives the producer. Nothing is
  held for a deadline, so the deadline adds no term. Every term is a whole number of quanta, so
  the ring is one at every quantum, and `prepare.rs:167-170` accepts it.
  - At quantum 128 the default becomes 6,912, 7,296, 12,800 and 14,080 frames at 44.1, 48, 88.2
    and 96 kHz (from 4,736, 5,120, 9,088 and 9,856).
  - At 96 kHz and quantum 127 it is 9,906 + 4,064 + 127 = 14,097 = 111 · 127
    (`ceil_127(966) = 1,016`).
- **D3. The published rule.** `sourceRing` keeps `stallToleranceMs` and `reserveQuanta` (the
  stall body, unchanged) and adds `primeGrowths` (4), `primeReserveQuanta` (1) and
  `primeLatencySamples`, `L_max(fs)` per launch rate (447, 486, 888 and 966). The ring in quanta
  is `ceil(stall / q) + reserveQuanta + primeGrowths · ceil(L_max / q) + primeReserveQuanta`. The
  extra quantum is published, so no consumer holds a private `+ 1`. The SDK's
  `defaultSourceRingFrames` applies the whole rule. The stall-body derivations in the context stay
  as they are.
- **D4. Every pin follows the rule.**
  - Every pin in the context's published-rule list is updated to D2's rule, and the generated SDK
    files are regenerated. The source-report assertions are updated.
  - The four SDK evals assert 14,097 (111 · 127), and each keeps its `% quantumFrames == 0` check.
  - The browser host's in-flight bound with a zero override applies D3's rule from the ABI layout
    it already loads (`preparedAbiLayout`, `miso-engine-v1-audio-worklet-host.js:1720`), not a
    private copy: 57 quanta at 48 kHz and quantum 128.
  - The two `.d.ts` comments, `MUTATIONS.md:7` (its expected value) and section 3 of
    `docs/derivations/243-sdk-boot.md` (a dated note naming this issue and the new rule; the
    derivation's history stays) state the new rule.
- **D5. The qualification label.** The record's ring label is renamed `defaultRingFrames` ->
  `stallRingFrames` in `qualification.js:715` and `run.mjs:926`, the prose of
  `generate-matrix.mjs:24` says "stall source ring", and its value stays 5,120 (the stall body at
  48 kHz, q = 128). `results.json` gets the same key rename and no other change, since the run it
  records is unchanged, and `BROWSER_DEPLOYMENT_MATRIX.md` is regenerated from it with
  `node hosts/host-web/qualification/generate-matrix.mjs`.
- **D6. Acked-batch question.** A larger ring holds more queued frames and drops none. No queue
  changes its admission rule. An ack can never precede a drop.

## Deliverables

1. D1 and D2 in `crates/host-core/src/prepare.rs`, with the re-export and the source-report
   assertions updated.
2. D3 in `tools/parameter-metadata`, `scripts/check-abi-layout-v1.py` and its fixture, the
   regenerated SDK files, and `sdk/src/core/abi.ts`.
3. D4's pins, the browser host's bound and its test, the comments and the docs.
4. D5's label rename and the regenerated matrix.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-export only),
  `crates/host-core/tests/prepare.rs`, `crates/host-core/tests/warm_successor.rs` (gate 1)
- `crates/capi/src/runtime/tests.rs` (the source-report assertions only)
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`
- `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts` (regenerated only),
  `sdk/src/core/abi.ts` (`defaultSourceRingFrames` only)
- `sdk/test/boot-evals.mjs`, `sdk/test/browser-evals.mjs`, `sdk/test/builder-evals.mjs` (the ring
  assertions only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the in-flight bound only),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and
  `sdk/src/browser/shipped-host.d.ts` (the ring comments only), `scripts/test-web-audioworklet.mjs`
  (gate 4's case only)
- `hosts/host-web/src/tests.rs` (`default_ring_covers_stall_tolerance` only),
  `hosts/host-web/MUTATIONS.md` (its row only)
- `hosts/host-web/qualification/qualification.js`, `run.mjs`, `generate-matrix.mjs` (the ring
  label only), `hosts/host-web/qualification/results.json` (the key rename only),
  `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` (regenerated only)
- `docs/derivations/243-sdk-boot.md` (section 3's note only)

## Non-goals

- The deadline and its step (#1358). The C ABI and browser wiring (#1360, #1361).
- No tuning of `P_MAX_SAMPLES` (#1286). No change to the stall body or the stall test.

## Objective gates

1. **Headroom.** In *Adopt a warm successor with a raw-frame prime at the first ready block*
   (#1355)'s gate 6 setup, with `ΣP + P = P_MAX_SAMPLES(fs, 128)` on a default ring
   (`source_ring_frames = 0`), a producer that keeps the ring full and then stalls for exactly the
   stall tolerance leaves readiness `true` at every block of the stall, and the predecessor never
   underruns. Check at all four launch rates.
2. **Every quantum.** `default_source_ring_frames(96_000, 127) == 14_097`, and at every launch
   rate crossed with the parameter-metadata test's ten quanta the result is a whole number of
   quanta, at least `stall_ring_frames(fs, q) + P_MAX_SAMPLES(fs, q) + q`, and accepted by
   preparation with `source_ring_frames = 0`.
3. **The rule is published.** The parameter-metadata test checks the published rule against
   `default_source_ring_frames` at every launch rate crossed with its ten quanta, and the layout
   check accepts the new `sourceRing` and rejects a document without `primeGrowths` or
   `primeReserveQuanta`.
4. **The browser host fills the ring.** With a zero override at 48 kHz and quantum 128, the host
   accepts 57 unsettled chunks for one source and rejects the 58th with `RESULT_BACKPRESSURE` (6).
5. **The SDK boots at quantum 127.** The four SDK evals pass with 14,097.
6. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`, `cargo test --locked
     -p capi`, `cargo test --locked -p parameter-metadata`, `cargo test --locked -p host-web`
   - `python3 scripts/check-abi-layout-v1.py --self-test`, `bash scripts/check-sdk-generated.sh`,
     `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh`,
     `bash scripts/test-web-audioworklet.sh`
   - `node hosts/host-web/qualification/generate-matrix.mjs --check`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: rings grown by `P_MAX` without the extra quantum, or not grown, fail readiness inside
  the stall tolerance. Red.
- Gate 2: a `P_MAX_SAMPLES` taken at quantum 128 for every quantum gives 14,129 at quantum 127,
  which preparation refuses. Red.
- Gate 3: an SDK that derives the old ring, or keeps the extra quantum private, sizes its producer
  one prime or one quantum short of the engine's ring. Red.
- Gate 4: a host bound that mirrors the old rule caps the source at 40 chunks, and the ring never
  holds the prime. Red.
- Gate 5: an SDK pin left at 9,906 fails at boot. Red.

## Dependencies

- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354):
  `stall_ring_frames` and the headroom check.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): gate 1's
  readiness.
- *Record the swap block's cost on the 64-track console* (#1286): `P_MAX_SAMPLES`.
