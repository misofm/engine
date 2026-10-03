# #1209 *Carry submix strips in the browser meter frame*: Sol verdict, attempt 1

- **Reviewed:** `git diff 60d3289b3 7f6e148f6` on branch `codex/batch-submix-k2` (worktree
  `/home/bl/misofm/wt-submix-k2`). 18 files, +650/-113.
- **Binding:** `AGENTS.md` and
  `.github/ISSUE_SPECS/1209-carry-submix-strips-in-the-browser-meter-frame.md` with its Attempt 1
  record. I also read #1213 (it owns the bus gain-reduction gate) and the #1208 verdict (NIT 3).
- **Paths:** every changed path is on the authorized list. `ffi.rs` and `qualification/*` are
  untouched, which is correct: their meter readers use only the existing fields.
- **How I ran it:**
  - I did not modify the worktree, commit, push or touch GitHub.
  - I exported `7f6e148f6` with `git archive` into `/tmp/claude-1002/v1209/src`, with its own
    `CARGO_TARGET_DIR` and `TMPDIR`, and copied `sdk/node_modules` in untracked.
  - After every mutation and probe I restored the file and checked it with `cmp` against
    `git show 7f6e148f6:<path>`.
  - The probe test is saved next to this file as `1209-attempt1-verifier-scratch.rs`.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There are two MINOR findings, both about tests and both
with a concrete fix I proved; neither is a code defect. There are also three NITs and one INFO
note.

D1 to D5 are implemented as frozen and are consistent on every reader and writer:

- **Rust:**
  - The poll uses `strips = T + S`, puts the master at `2(T + S)` and `gain_base` at
    `2(T + S) + 2`, and writes the master's gain reduction at `gain_base + strips`.
  - `boxed_zero_meter_frame(strip_count)` sizes the frame.
  - Meters are requested for `model.tracks` and then `model.submixes`. The normalized model sorts
    both segments, so this matches `handles.strips` and `strip_index`.
- **Worklet:**
  - It reads `submix_count` at offset 64 before the capacity check, `(3(T + S) + 3) * 4`.
  - It builds five construction-time views at the right offsets.
  - The master gain-reduction index is `3(T + S) + 2`.
- **Main-realm host:** the exact-field check covers 15 fields, plus the length and
  finite-non-negative rules for both bus arrays.
- **SDK `pollMeters`:** it sizes the frame by the header's `submixCount` and reads the master at
  `2(T + S)`.
- **Layout:**
  - The generator emits `[Field; 12]`.
  - The checker pins `meterHeader` at 72, and its rows tile the 72 bytes.
  - `abi.ts`, the layout asset and the self-test fixture are regenerated; the fixture is
    byte-identical to the asset.
  - `shipped-host.d.ts` is byte-identical to the web `.d.ts`.

## Adversarial checks

**The ABI and versioning question: compatible.**

- **What changed.** `WebMeterHeader` grows from 64 to 72 bytes, appending `submix_count@64` and
  `reserved_pad@68`. No earlier offset moved, as `frozen_layouts_and_values_are_exact` pins.
- **It follows the repo's rules.** This is an in-place V1 amendment with no `ABI_VERSION` bump,
  which is what the AGENTS version rule asks for before launch: the live identity stays `V1`, and
  no later generation is claimed. It is also the same pattern #143 D5 used to add the
  gain-reduction section.
- **A stale consumer fails closed.** `struct_size` describes the structure, and every consumer
  checks it exactly:
  - The SDK throws `abiMismatch`/`sdk.meter.header`.
  - The worklet refuses the boot.

  So an old reader refuses the new header; it never misreads it.
- **The new message fields cannot meet a skewed host.** The worklet and the main-realm host ship
  in one delivery closure, so the 15-field exact-field check cannot see version skew.
- **No 64-byte header assumption is left in live code.** I grepped the worklet, the host JS, the
  SDK, `direct-oracle.mjs`, the harness, the generator and the checks:
  - **Memory.** The frame and header are views into Wasm memory; there is no `SharedArrayBuffer`
    in the meter path, and `new SharedArrayBuffer` is banned by `check-web-audioworklet.sh`.
  - **Docs.** The only stale `64` is in a historical derivation note (NIT 2).

**The worklet's `process()` stays allocation-free.**

- **The new code is static.** `postMeterFrame` sits inside the
  `PROCESS_POLICY_BEGIN/END` body. The new lines are four `TypedArray.set` calls on views built at
  construction; one of them passes an offset rather than taking a `subarray`. There is no `new`,
  `subarray`, spread or closure, and the static policy gate passes.
- **Views stay current across reboots.** They are rebuilt in the per-boot construction routine.

**The gain-reduction skip is removed correctly (D5).**

- **Why the removal is right.** `observation_tracks[slot]` holds the strip index, so a bus effect
  folds into `gain_base + T + j`, which is that bus's own word. `observation_present` is sized to
  `strip_count`. A bus designated master (allowed since #1208) now reads its own word.
- **It is untested here, as planned.** No bus effect can be armed until #1213. Mutation R2 below
  shows that no current test covers it, and #1213's gate 5 owns it (NIT 1).

**Sessions with `S = 0` are unchanged.**

- Every pre-existing host-web frame, parity and digest test passes unchanged. Only the
  `gain_reduction` helper changed, and it reads `S = 0` there.
- `check-browser-expected-resources.py --artifacts` reproduces every pinned direct-oracle digest.

**The shipped module is reproduced.**

- My build gives `f5d36ba04e52e9e94775f15ab12ce1e1e5b73e03cbec0ed3d12b36278dd00ad0`
  (2 693 332 B), equal to the record's ARTIFACT CHANGED value.
- The change is expected, because the frame, the header and the poll changed. No re-pin is
  needed, since this is not a release change.

**The #1208 verifier's NIT 3 (are `SAMPLE_PEAK`-only bus meters tested?) is addressed.**

- **The committed gate.** Gate 1 runs the browser's real request, `PostMatrix` with
  `SAMPLE_PEAK` on every strip. It compares each bus pair with its feeders' sum and the master
  with an `S = 0` twin; it does not just check for nonzero values.
- **Its limit.** The source is DC.
- **My probe closes that gap.** `verifier_probe_bus_sample_peak_varying_source` uses six seeds,
  with sign-changing noise and one spike per lane at a drawn sample, some of them negative. Every
  track lane is a positive multiple of one source lane, so a bus's peak is exactly the sum of its
  feeders' peaks. Over at least two windows per seed:
  - track words are bit-equal to the twin's;
  - each bus equals its feeder sum, within a relative 2e-6;
  - the master equals the twin's.

  All pass. Mutation R1 also turns the probe red.

## Gates re-run on `7f6e148f6` (x86_64 AVX2)

`A` = `/tmp/claude-1002/v1209/A`, `B` = `/tmp/claude-1002/v1209/B`.

| Gate | Result |
|---|---|
| `python3 -B scripts/check-abi-layout-v1.py --self-test` | ok (22 mutations caught) |
| `bash scripts/build-web-audioworklet.sh --named-twin B A` | ok; shipped `f5d36ba0…00ad0`, matching the record |
| `check-abi-layout-v1.py A/miso-engine-v1-abi-layout.json` | ok |
| `check-web-audioworklet.sh A B/…named.wasm` | ok (static/object checks, process policy, boot budget) |
| `check-browser-expected-resources.py --artifacts A` | ok (digests agree; 32 red mutations) |
| `bash scripts/test-web-audioworklet.sh` | ok (hermetic, safe-integer mutation, opcode and call-graph self-tests) |
| `check-sdk-generated.sh A` | ok |
| `check-sdk-types.sh` | ok (includes the shipped-host mirror pin) |
| `check-sdk-headless.sh A` | ok: 334 pass, 0 fail, gate-2 subtest included |
| `cargo test -p host-web` | unit 128 + 1 ignored, `boot_transient_budget` 2, `retained_ceilings` 1 |
| `cargo test -p parameter-metadata` | ok |
| `cargo test -p session-validator` (the other host-web dependent) | ok |
| `cargo clippy -p host-web -p parameter-metadata -p session-validator --all-targets --all-features -D warnings` | clean |
| `cargo fmt --all -- --check` | rc 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc -p host-web --all-features --no-deps` | clean |
| `check-/test-realtime-policy.sh`, `check-workspace-policy.sh` | ok |
| Workspace test command and workspace-wide clippy | **not re-run**: disk was 22-29 GB free and is shared with the #1210 implementer. `WebMeterHeader` and the frame are consumed only by host-web, parameter-metadata and session-validator (a reverse-dependency grep), and all three were tested and linted in full. The record reports workspace rc 0. |

## Mutations

Each was applied alone, run, and reverted. I added these to the implementer's own 15.

| # | Mutation | Result |
|---|---|---|
| R1 | request the bus meters in reverse order (`model.submixes.iter().rev()`) | **red**: gate 1 (`aaa-bus` against `t0 + t1`) and my probe |
| R2 | restore #1207's `track as usize >= T` gain-reduction skip | **green**: all host-web tests (expected; #1213 gate 5 owns it, see NIT 1) |
| R3 | `Box::new` per *bus* snapshot in the poll's commit loop | **red**: gate 5, `render/poll allocated` |
| W0 | worklet: `if (this.submixCount === 0) return false;`, which refuses every no-bus session | **green**: `test-web-audioworklet.mjs` (MINOR 2) |
| W1 | worklet: point the bus gain-reduction view at the track gain-reduction words | **red**: `submixGrDb` reads `[6.5]` |
| H1 | host: drop the `submixPeaks` finite-non-negative rule | **red**: `expected rejection` |
| S1 | SDK `pollMeters`: copy `peaks`' master pair from `frame[2T..2T + 2]`, which is bus-a's slot | **green**: 334/334 (MINOR 1) |
| S2 | SDK `pollMeters`: `trackGrDb` from the old base `2T + 2`, which returns bus-b's peaks and master L as gain reduction | **green**: 334/334 (MINOR 1) |

## Test value, one sentence per new or rewritten test

- **`the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix`.** It turns red if a host-web
  writer keeps `3T + 3` or writes the master at `2T`. It also turns red if the bus meters are
  missing or in the wrong order (R1). No other test boots a metered session with `S > 0` and reads
  its frame words.
- **`bus_meters_render_and_poll_without_allocating`.** It turns red if the widened poll allocates
  per window for the bus meters (R3). It is the only test that measures `poll_meters` under the
  thread-scoped allocation counters with `S > 0`; #1207's gate 4 measured only admission and
  render.
- **`capability-evals.mjs`, "a frame with submixes keeps the track shape and carries every bus".**
  It turns red if the SDK headless reader keeps the `3T + 3` capacity check, which refuses every
  frame with buses, or drops or empties the bus sections. It cannot see a wrong master or
  gain-reduction offset (MINOR 1).
- **`test-web-audioworklet.mjs`, gate 3 and the reworked worklet decode.**
  - **The validator.** It turns red if the main-realm validator accepts a malformed bus section
    (H1, plus the record's 16 variants) or still refuses the three new fields.
  - **The worklet.** It turns red if the worklet reads the master after the tracks or reads a bus
    word from the wrong section (W1). Each section of the stub carries a distinct value.

## Findings

### MINOR 1: gate 2 does not cover the hazard the spec assigns to it

- **What the spec says.** The Hazards section says "A missed reader reads the master from a
  submix's slot. Gate 2 decodes a frame with `S > 0` through the shipped module."
- **What survives.** For the SDK headless reader, the implementation is correct, but two
  realistic offset mutations pass all 334 evals:
  - S1, which reads the master pair from bus-a's slot;
  - S2, which reads `trackGrDb` from the old `2T + 2` base and so reports bus peaks as decibels
    of gain reduction.
- **Why.** The gate checks only lengths, finiteness and positivity, so a value read from the wrong
  section still passes. The implementer followed the spec's gate text; the gate as written is too
  weak.
- **Fix (proved).** In the gate-2 test, add relations derived from `busDocument()`'s routing:
  three identical tracks share one source, `bus-a` sums two of them and `bus-b` one, and nothing
  is armed. These are not hard-coded rendered values.

  ```js
  const close = (actual, expected) => Math.abs(actual - expected) <= Math.abs(expected) * 1e-5;
  const master = trackCount * 2;
  for (const lane of [0, 1]) {
    assert.ok(close(frame.submixPeaks[lane], 2 * frame.submixPeaks[2 + lane]), `bus-a lane ${lane}`);
    assert.ok(close(frame.peaks[master + lane],
      frame.submixPeaks[lane] + frame.submixPeaks[2 + lane]), `master lane ${lane}`);
  }
  assert.ok(frame.trackGrDb.every((value) => value === 0), "nothing is armed");
  assert.ok(frame.submixGrDb.every((value) => value === 0), "nothing is armed");
  ```

- **Verified.** On `7f6e148f6` it is green; the observed values are bus-a = 2 x bus-b and
  master = bus-a + bus-b, exactly. S1 turns red on `master lane 0` and S2 on `nothing is armed`.
- **When.** Land it before the K2 batch push. It can ride with #1210, which edits the same SDK
  reader.

### MINOR 2: the worklet's `S = 0` decode lost its only per-PR coverage

- **The cause.** `createFakeExports` now hard-codes `submixCount = 1`, and every processor test
  uses that stub.
- **The consequence.** No per-PR suite runs the worklet JS with meters and `S = 0`:
  - `browser-pcm-evals.mjs` attaches no meters;
  - the SDK headless suite uses `WasmBoundary`;
  - `qualification/run.mjs` is not a per-PR job.

  So W0, a worklet that refuses every session without buses (today's production case), stays
  green. Before this commit the stub was `S = 0`, so this is a coverage regression. The code
  itself is uniform in `S` and correct.
- **Fix.**
  - Parameterize the stub as `createFakeExports(quantum, backend, liveControlsAttached,
    submixCount = 1)`.
  - Run the processor's meter-frame assertions once with `submixCount = 0`. Expect `peaks`
    `[0.5, 0.5, 0.5, 0.5, 0.625, 0.625]`, empty `submixPeaks` and `submixGrDb`, and `masterGrDb`
    7.5.
  - Keep the `S = 1` leg.

### NIT 1: D5 has no gate in this slice

R2, which restores the `>= T` skip, survives. This is by design: a bus effect cannot be armed in
the browser before #1213. #1213's verifier must apply R2 and see #1213's gate 5 turn red.

### NIT 2: a stale `64` in a historical note

`docs/derivations/243-sdk-boot.md:134` still lists `meterHeader` as 64. It is a historical
derivation note outside the authorized paths, so either annotate it ("72 since #1209") at a
convenient docs pass or leave it.

### NIT 3: `reserved_pad` is unchecked

`reserved_pad` is documented "Always zero", but no reader checks it. Optionally, the SDK's
`pollMeters` header guard could require `reservedPad === 0`, which would make a misplaced future
field visible.

### INFO 1: the meter frame is not charged to the resource report

The meter frame box, `4 * (3(T + S) + 3)` bytes, is not charged to `bridge_retained_bytes`; only
the pending and master delivery arrays are. This predates the slice, which only scales the frame
by `S`, so I note it for completeness.

## Scratch cleanup

`/tmp/claude-1002/v1209`, which held the export, the target (about 3.8 GB) and the artifacts, is
deleted after this verdict. The probe is kept as `1209-attempt1-verifier-scratch.rs`.
