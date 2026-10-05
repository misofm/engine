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
  `p_max_samples(fs, q) = 4 · ceil_q(L_max(fs))`, with `L_max(fs)` the true-peak limiter's
  `fs / 100 + 6` (`crates/true-peak-limiter/src/lib.rs:236-242`) unless the record finds a larger
  one. It is taken at the session's quantum, so it is a whole number of quanta.
- A ring that is not a whole number of quanta is refused (`host.source.ring_frames`,
  `crates/host-core/src/prepare.rs:167-170`), on both default paths
  (`crates/control-plane/src/compile.rs:709-711`, `hosts/host-web/src/lib.rs:2100-2101`). The SDK
  boots 96 kHz at quantum 127 (`sdk/test/boot-evals.mjs:71-84`).
- **The published rule.** The ABI layout document's `sourceRing` carries the rule's inputs
  (`tools/parameter-metadata/src/abi_layout.rs:109-115`, `:2266-2270`). These pin the rule or its
  values: `tools/parameter-metadata/tests/abi_layout.rs:262-292`,
  `scripts/check-abi-layout-v1.py:591-595` and its self-test mutations (`:652-656`),
  `scripts/fixtures/abi-layout-v1-self-test.json:666`, the generated
  `sdk/assets/miso-engine-v1-abi-layout.json:666` and `sdk/src/generated/abi.ts:2421-2424` (kept
  equal by `scripts/check-sdk-generated.sh`), the SDK's `defaultSourceRingFrames`
  (`sdk/src/core/abi.ts:206-212`), `hosts/host-web/src/tests.rs:1508-1539`,
  `crates/host-core/tests/prepare.rs:618-633` and `crates/capi/src/runtime/tests.rs:626-629`.
- **SDK and browser pins.**
  - `sdk/test/boot-evals.mjs:83-84` and `:318`, `sdk/test/browser-evals.mjs:276` and
    `sdk/test/builder-evals.mjs:296` assert 9,906 (78 · 127) at 96 kHz and quantum 127.
    `sdk/test/render-evals.mjs:159` asserts that the same ring is 78 quanta; its source region
    of 19,200 frames (`:135`) holds a full ring turnover (`:160-163`). `check-sdk-headless.sh:91`
    runs all of them.
  - `scripts/test-web-audioworklet.mjs:1103-1128` runs the browser host at 48 kHz and quantum 64
    (`:294`, `:1094`) with a zero override: it holds 77 unsettled chunks for one source, expects
    the 78th to be refused with `RESULT_BACKPRESSURE` (6), and submits start frame 78 after they
    settle.
  - The browser host's per-source in-flight bound mirrors the rule when the ring override is 0
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1728-1732`): 40 quanta at 48 kHz
    and quantum 128, 77 at quantum 64. Under the old bound the host would never fill the new
    57-quantum (110 at quantum 64) ring.
  - The host's declaration `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and its
    mirror `sdk/src/browser/shipped-host.d.ts` (kept equal by `scripts/check-sdk-generated.sh`)
    describe the old rule at `:145-147` and `:785`. `hosts/host-web/MUTATIONS.md:7` and
    `docs/derivations/243-sdk-boot.md` (section 3) quote its values.
- **The qualification record.** `hosts/host-web/qualification/run.mjs:211` and
  `qualification.js:6-8` pin 5,120, but the stall test passes that ring explicitly
  (`bootOptions(DEFAULT_RING_FRAMES)`, `qualification.js:497-501`) and tests the stall body, which
  stays 5,120. `hosts/host-web/qualification/rebuild-cost.mjs:66-74` and
  `scripts/web-mixing-automation-benchmark.mjs:105-110` compute the stall body from the two stall
  inputs and pass it explicitly. None of them changes. Only the recorded label does: the record
  writes `defaultRingFrames` (`qualification.js:715`, `run.mjs:926`), `results.json:8` holds it,
  the matrix prose calls it "the default source ring" (`generate-matrix.mjs:24`), and
  `BROWSER_DEPLOYMENT_MATRIX.md:12` is generated from them. `run.mjs:915-916` requires that
  document to equal `renderMatrix(results.json)`.

## Decisions frozen for this slice

- **D1. `p_max_samples`.** `pub const fn p_max_samples(sample_rate_hz: u32, quantum_frames: u32)
  -> u32` in `crates/host-core/src/prepare.rs`, beside `stall_ring_frames`, re-exported from
  `crates/host-core/src/lib.rs`, with #1286 D3 item 1's value; its comment names the record row.
  The name is snake_case, as every Rust function's is: clippy's `non_snake_case` refuses an
  upper-case `fn` under `-D warnings`, and no workspace lint relaxes it.
  #1358 D1, D2 below and the production `WarmConfig` (*Check the warm-successor deadline in
  miso_engine_v1_service and report its outcome*, #1360 D1) read it. No other spec restates it.
- **D2. Ring headroom.** `default_source_ring_frames(fs, q)` becomes
  `stall_ring_frames(fs, q) + p_max_samples(fs, q) + q`: #1354 D1's stall body, plus the largest
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
  - The four 9,906 pins (`boot-evals.mjs:84` and `:318`, `browser-evals.mjs:276`,
    `builder-evals.mjs:296`) assert 14,097 (111 · 127), and the comment at `boot-evals.mjs:83`
    says so. `render-evals.mjs:159` asserts 111 quanta; 14,097 frames still fit its 19,200-frame
    region. The `% quantumFrames == 0` checks at `boot-evals.mjs:85`, `builder-evals.mjs:297` and
    `render-evals.mjs:150-154` stay. `boot-evals.mjs:318` and `browser-evals.mjs:276` have none,
    and none is added: their literal is itself a whole number of quanta.
  - The browser host's in-flight bound with a zero override applies D3's rule from the ABI layout
    it already loads (`preparedAbiLayout`, `miso-engine-v1-audio-worklet-host.js:1720`), not a
    private copy: 57 quanta at 48 kHz and quantum 128, and 110 at quantum 64. The existing case at
    `scripts/test-web-audioworklet.mjs:1103-1128` is updated to 110 held chunks, the 111th
    refused and start frame 111 after they settle.
  - The two `.d.ts` comments, `MUTATIONS.md:7` (its expected value) and section 3 of
    `docs/derivations/243-sdk-boot.md` (a dated note naming this issue and the new rule; the
    derivation's history stays) state the new rule.
  - **Stale prose.** Each of these describes the default ring as the stall body alone, and is
    corrected to D2's rule (or, where the text means the stall body, renamed to the stall ring):
    - `crates/host-core/src/prepare.rs:59-63` (the doc comment of `default_source_ring_frames`);
    - the module docs at `tools/parameter-metadata/src/abi_layout.rs:42-48` and
      `tools/parameter-metadata/tests/abi_layout.rs:12-15`;
    - `hosts/host-web/src/lib.rs:1215` (the override field's doc: "the engine's 100 ms
      derivation");
    - `hosts/host-web/DEPLOYMENT.md:7` ("the two inputs to the default source-ring rule");
    - `scripts/web-mixing-automation-benchmark.mjs:105-106` (the comment calls the stall body the
      SDK's default ring; the code, which computes the stall body, stays);
    - `hosts/host-web/MUTATIONS.md:8` and the doc comment at `hosts/host-web/src/tests.rs:1545-1549`
      (`SOURCE_STALL_TOLERANCE_MS = 50` now gives a 38-quantum ring at 48 kHz and quantum 128,
      not 21: 21 + 16 + 1). The mutation is re-run, must still be red (the test's
      `stall_quanta == 38` assertion, `tests.rs:1557`, fails under it), and the row records the
      ring size and the failure the re-run shows;
    - `docs/derivations/241-browser-source-identities.md:178` ("the default ring" for the
      stall test's 5,120 frames, which is the stall ring).
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
  assertions and the comment at `boot-evals.mjs:83` only), `sdk/test/render-evals.mjs` (the
  quanta assertion only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the in-flight bound only),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and
  `sdk/src/browser/shipped-host.d.ts` (the ring comments only), `scripts/test-web-audioworklet.mjs`
  (gate 4's case at `:1103-1128` only)
- `hosts/host-web/src/tests.rs` (`default_ring_covers_stall_tolerance` and the doc comment of
  `ring_prefill_survives_stall` only), `hosts/host-web/MUTATIONS.md` (rows 7 and 8 only)
- `hosts/host-web/src/lib.rs` (the doc comment of the ring override field only),
  `hosts/host-web/DEPLOYMENT.md` (the ring sentence only),
  `scripts/web-mixing-automation-benchmark.mjs` (the comment at `:105-106` only),
  `docs/derivations/241-browser-source-identities.md` (the ring label at `:178` only)
- `hosts/host-web/qualification/qualification.js`, `run.mjs`, `generate-matrix.mjs` (the ring
  label only), `hosts/host-web/qualification/results.json` (the key rename only),
  `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` (regenerated only)
- `docs/derivations/243-sdk-boot.md` (section 3's note only)

## Non-goals

- The deadline and its step (#1358). The C ABI and browser wiring (#1360, #1361).
- No tuning of `p_max_samples` (#1286). No change to the stall body or the stall test.

## Objective gates

1. **Headroom.** In *Adopt a warm successor with a raw-frame prime at the first ready block*
   (#1355)'s gate 6 setup, with `ΣP + P = p_max_samples(fs, 128)` on a default ring
   (`source_ring_frames = 0`), a producer that keeps the ring full and then stalls for exactly the
   stall tolerance leaves readiness `true` at every block of the stall, and the predecessor never
   underruns. Check at all four launch rates.
2. **Every quantum.** `default_source_ring_frames(96_000, 127) == 14_097`, and at every launch
   rate crossed with the parameter-metadata test's ten quanta the result is a whole number of
   quanta, at least `stall_ring_frames(fs, q) + p_max_samples(fs, q) + q`, and accepted by
   preparation with `source_ring_frames = 0`.
3. **The rule is published.** The parameter-metadata test checks the published rule against
   `default_source_ring_frames` at every launch rate crossed with its ten quanta, and the layout
   check accepts the new `sourceRing` and rejects a document without `primeGrowths` or
   `primeReserveQuanta`.
4. **The browser host fills the ring.** In the existing case at
   `scripts/test-web-audioworklet.mjs:1103-1128`, with a zero override at 48 kHz and quantum 64,
   the host accepts 110 unsettled chunks for one source and rejects the 111th with
   `RESULT_BACKPRESSURE` (6); after they settle, start frame 111 is accepted.
5. **The SDK boots at quantum 127.** The boot, browser and builder evals pass with 14,097, and the
   render eval with 111 quanta.
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
- Gate 2: a `p_max_samples` taken at quantum 128 for every quantum gives 14,129 at quantum 127,
  which preparation refuses. Red.
- Gate 3: an SDK that derives the old ring, or keeps the extra quantum private, sizes its producer
  one prime or one quantum short of the engine's ring. Red.
- Gate 4: a host bound that mirrors the old rule caps the source at 77 chunks, and the ring never
  holds the prime; a bound that keeps the extra quantum private, or drops it, refuses or accepts
  the wrong chunk. Red.
- Gate 5: an SDK pin left at 9,906, or 78 quanta, fails at boot. Red.

## Dependencies

- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354):
  `stall_ring_frames` and the headroom check.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): gate 1's
  readiness.
- *Record the swap block's cost on the 64-track console* (#1286): `p_max_samples`.
