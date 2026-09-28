# Adversarial verification: the automation-cost diagnosis (EQ, compressor, limiter)

Subject: `AUTOMATION-DIAGNOSIS.md`, `automation-diagnosis-prototypes.patch`,
`automation-diagnosis-raw-timings.txt` and drafts `issues/automation-1..5-*.md`. Base:
`codex/batch-plumbing-floor-2` at `60d4f1b6`. For `crates/`, `hosts/` and `tools/` this is
code-identical to the diagnosis base `49f696c7`: only a limiter test and its `MUTATIONS.md` differ,
so every `file:line` in the drafts still holds. Nothing was pushed, and no issue was touched.

Raw output: `verify-automation-raw-timings.txt`. Every timed command held the shared lock, was
pinned with `taskset -c 31`, and printed its load average (7-15; others were building). Every hold
was under two minutes.

Harness additions: `verify-automation-harness.patch`, which applies on top of the diagnosis patch.
It adds:

* the V8 subjects `mono_eq`, `mono_comp` and `mono_lim`;
* the `P1BREAK` output-identity mode (`P1FX=eq|comp|lim`, `P1ORDER=split|pair|both|left`);
* the native `AUTO_DIAG_EQ_BOTH=1`, which sends an EQ edit as one `Both` owner edit;
* the spans-only P1 variant (two lines in `live.rs`).

The V8 artifacts were clean builds with the delivery flags of `scripts/build-web-audioworklet.sh`,
with each prototype switch compiled as a constant. The base artifact is built from this tip.

## Verdict

* **Holds.** Every cost and saving the diagnosis quotes reproduces within noise, natively at
  `Simd8` and under V8 through the shipped `host_web.wasm` render export. So do the EQ root cause
  (the 12-pass unelided schedule), the compressor root cause (scalar per-lane ramps and a per-frame
  redesign), the limiter's "about +1 µs", and the size of the mono-collapse loss (V8 149 → 248 µs).
* **Does not hold: P1 as prototyped and drafted renders wrong audio.** Its target half accepts a
  drain whose final left and right states differ. This is reproduced through `host_web.wasm`
  (finding 1).
* **Does not hold: the EQ is not part of the product's collapse loss.** The SDK already sends a
  symmetric both-channel EQ edit as one `Both` target, which the witness preserves. Only the
  compressor and the limiter retire the collapse on the web path (finding 2).
* **Holds, with scope corrections:**
  * E1 and E2 are correct.
  * C2 is correct on the dual body, but its saving does not reach mono stems (finding 4).
  * C2b is exact by an invariant that holds on today's tree (section 4).
* **The benchmark row is not a real host shape as drafted, and its in-run assertion would fail on
  its first run** (finding 3).

## 1. Findings, by severity

### F1 (critical, wrong audio): P1's target pairing ignores `Both` targets

`writes_pair` compares the *last* `(slot, Left)` target with the *last* `(slot, Right)` target. It
skips any `Both` target between them.

**Failure scenario.** A mono stem, and three EQ submissions through `prepared-control.js` before
one render quantum. This is ordinary AudioWorklet traffic: queued port messages are handled
together between two `process()` calls.

1. Left only: band-1 gain 4.5 dB. That is one `Left` target, `A`.
2. Both channels: 1.5 dB. The SDK coalesces it into one `Both` target, `C`
   (`crates/parametric-eq/src/control.rs`, `fill_targets` / `sections_can_be_both`).
3. Right only: 4.5 dB. That is one `Right` target whose words equal `A`.

The drain FIFO is `[L A, Both C, R A]`.

* P1 sees last L = `A` and last R = `A`, so it keeps `LIVE`.
* The actual state is left = `C` (the `Both` came after `L A`) and right = `A`.
* The collapse renders the left plane for both channels, so the right channel plays 1.5 dB where
  4.5 dB was acknowledged.
* When the collapse later disengages, the chain copies the left state over the right
  (`desymmetrize`, `parametric-eq/src/lib.rs:2324`). The right-only edit is therefore lost, not
  late: an acknowledged command is silently not applied.

**Reproduced** through `host_web.wasm`, 40 blocks, whole-output SHA-256 prefix:

| build | EQ `[L, Both, R]` | EQ `[L, R]` | EQ `Both` | EQ left only |
|---|---|---|---|---|
| base | `39d2dc01…` | `4a47ef63…` | `4a47ef63…` | `59aefc46…` |
| P1 as prototyped | **`22e07bf0…`** | `4a47ef63…` | `4a47ef63…` | `59aefc46…` |
| P1 spans only (target half removed) | `39d2dc01…` | `4a47ef63…` | `4a47ef63…` | `59aefc46…` |

The four orders for the compressor (threshold) and for the limiter (ceilings −20 and −17 dB, which
engage) are identical across all three builds. Of the twelve fx × order cases, only the EQ
`[L, Both, R]` case differs, and only under P1 as prototyped.

**Why the diagnosis missed it.** Its harnesses never put a `Both` target and one-channel targets in
one drain:

* the native harness makes one owner transaction per channel;
* the V8 harness makes one submission per block.

**Fix.** Drop the target half, which is what `automation-1` is amended to do:

* one-channel `PreparedTarget`s fold into the witness as today;
* only `Parameter` spans pair.

Measured: the spans-only build restores exactly what P1 restored (V8 mono settled 148.2 µs;
8 of 64 173.4 µs, against P1's 148.9 and 174.2 µs). The alternative is an effective-last-writer
rule in which a `Both` target writes both channels.

### F2 (high, claim wrong): the EQ does not retire the collapse on the product path

Diagnosis section 1 says "the EQ owner emits one `PreparedTarget` per channel". Its verdict says
"any live write to an EQ … parameter retires … the web host's both-channel command".

* On the web path, `COMMAND_EFFECT_PARAM` with `channel = 2` becomes one owner transaction that
  edits both rows.
* `fill_targets` emits one `Both` target whenever the two sections' semantic words and coefficients
  are bit-equal. That holds whenever the track's two channels carry the same band settings, which
  is every mono stem the fixtures and the SDK create.
* The exception: a band whose left and right differ only in a field its design ignores, such as a
  disabled band's frequency. It still gets two targets and retires the collapse, as it does
  today.
* The witness preserves a `Both` target (`symmetry.rs:330`).

V8, mono console, every track written once and then left settled (row µs, base):

| written | row µs |
|---|---:|
| nothing | 148.8 |
| EQ band-1 gain only | **148.7** |
| compressor threshold only | 248.0 |
| limiter ceiling only | 247.8 |
| all three (the diagnosis's arm) | 247.7 |

An EQ ride on 8 of 64 mono tracks keeps the collapse at base: 180.3 µs, the ramp cost alone.

The diagnosis's native EQ figures come from `SessionRuntime::push_parameter`, which runs one owner
transaction per channel and so produces `Left` + `Right` targets. The web SDK never does that for a
symmetric edit.

**Consequences:**

* P1 needs no target half.
* The native benchmark row must push the EQ as one `ParameterChannel::Both` owner edit
  (`effect-compiler/src/control.rs:224` accepts it), or it measures a loss the product does not
  have.
* The ordinary operations that do retire the collapse are listed in section 3.

### F3 (high, benchmark): `automation-5` is not a real host shape, and its assertion fails

1. **The EQ traffic is the harness's, not the host's.** Left then Right through `push_parameter`
   produces two one-channel targets. The product sends one `Both` target (F2).
2. **`quiet == restated` is red on its first run.**
   * The drafted bases (3 dB, −24 dB, −3 dB) are not the fixture's held values. On `ch00` of the
     mono fixture those are EQ band-1 −7.5 dB, compressor threshold −6 dB and limiter ceiling
     −0.5 dB, and the other tracks are heterogeneous.
   * So "restated" changes the sound, and its digest cannot equal `quiet`'s.
3. **The wasm arm is not the shipped path.**
   * The draft records it through a new control export on the wasmtime console guest.
   * The real browser path is `host_web.wasm` under V8, through `miso_engine_web_v1_command_submit`
     and `prepared-control.js`. The diagnosis's own V8 harness already does this.

### F4 (medium-high, scope): C2 does not speed up mono stems

The prototype rewrote the dual prefix only. A collapsed bank still runs
`frames_loop_mono::<L, true>`.

V8, compressor threshold ride on the mono console, with P1 so that the banks stay collapsed:

| arm | P1 | P1 + C2 |
|---|---:|---:|
| 8 of 64 | +33.7 µs | +32.0 µs |
| all 64 | +97.1 µs | +95.3 µs |

Both differences are noise.

* The diagnosis's row "mono console, 8 of 64, P1 + E1 + E2 + C2" contains no C2 contribution.
* Draft contract 4 (the collapsed rewrite) is unprototyped, unmeasured and uncovered by the
  differential, whose `Mono` arm has no new path to exercise.
* It is the half that matters for the owner's common case. With P1, E1 and E2 in, the collapsed
  compressor prefix is the largest remaining automation cost on mono stems: `all4` mono all 64 is
  +79 µs over settled, against +46 µs on the stereo console.

### F5 (medium, the acked-batch question): the host-`Both` form can drop after an ack

The web host derives `maximum_automation_spans_per_block` as `max(session automation segments,
console_command_queue_records, 1)` (`hosts/host-web/src/lib.rs:7674`). The queue depth is
`min(records, capacity)` (`effect-compiler/src/prepare.rs:2067`).

**Failure scenario.**

1. A host boots with `console_command_queue_records = 2` and no timeline automation, so capacity
   and depth are both 2.
2. One submission carries `channel = 2` threshold and ratio commands for one compressor.
3. Under the `Both` form that is 2 records, so admission passes and acknowledges.
4. The drain expands them to 4 distinct `(parameter, channel)` spans into a 2-span window: 2 are
   `dropped`. The effect would also refuse `span_index >= automation_capacity`.

An ack precedes a drop. Counting a `Both` as 2 records of room does not close it, because room is
measured in queue slots across submissions.

The form is safe only if both of these hold:

* capacity is derived as at least 2 × the queue depth, and preparation asserts it;
* the drain knows each parameter's channel policy, because a `Shared` `Both` must stay one span.

P1 (spans only) changes neither admission nor staging, and its `dropped` stays unreachable.

### F6 (medium, evidence): some identity arms cannot fail

In the V8 output digests (150 blocks, `DIGEST` mode):

* `lim_ceiling` gives the **same digest on every arm**: settled, one Point, one segment, left
  only, 8 of 64 and all 64. A ceiling of −3 ± 0.25 dB never engages on this fixture.
* `comp_attack`'s one-track arms equal `settled`.

So "identical output on every console arm" says nothing about limiter automation or single-track
attack rides. The P1 bug also passed every digest. Every identity arm in the drafts now has to
assert `automated != settled` in-run and use values that engage.

### F7 (medium, drafts not self-contained): the V8 gates point at a scratch harness

* `web_auto.mjs` and `gen_docs.py` hard-code a deleted worktree (`agent-ae53afa6e6b89a559`) and
  scratch root (`scratchpad/automation-diag`).
* The drafts' timing gates cite the harness without saying so.
* No timing gate has a mono row, although the collapsed bodies are the common case.

The amended drafts carry the recipe and the mono rows.

### F8 (low-medium): the C2 differential's NaN relaxation was widened

The prototype's `wet_arm_relaxed` grants the NaN-payload relaxation to **every** ramping, `Main`,
unbypassed block, whether or not the prefix took the wet arm. It stays bounded to blocks that
`finish_channel` rejects, so the output is unaffected, but the gate is weaker than the draft
claims. It should be granted only when a wet-arm counter says the prefix took the arm.

### F9 (low): P1's pairing is quadratic over an undeduplicated FIFO

* The target FIFO's length is the queue depth, which is host-configured with no small bound on the
  web.
* `writes_pair` is O(n²) over it, on the render thread.

Spans-only pairing runs over the staged window, which is deduplicated and sorted by
`(parameter, channel)`. A twin is therefore the adjacent entry: an O(n) check with
n ≤ 2 × parameters.

## 2. Reproduction of the cost and saving claims

µs per 64-track block. Δ is against `settled` in the same run.

**V8** (`host_web.wasm`, Node 22.23.2, `--no-liftoff`, 6 rounds × 500 blocks):

| row | diagnosis | reproduced |
|---|---|---|
| mono console: untouched / all written, settled | 151.2 / 243.2 | 148.8 / 247.7 |
| … with P1 | 151.5 | 148.3 (spans only: 148.2) |
| mono, 8 of 64: base / P1 | 284.8 / 177.3 | 290.5 / 174.6 (spans only: 173.4) |
| stereo console, 8 of 64: base / P1 | 281.9 / 282.6 | 284.7 / 284.5 |
| EQ Δ one / 8 of 64 / all: base | +10.5 / +64.9 / +146.0 | +9.25 / +64.0 / +145.4 |
| … E1 | +2.9 / +11.2 / +38.7 | +3.04 / +11.4 / +38.4 |
| … E1 + E2 | +2.3 / +9.1 / +23.9 | +3.05 / +9.85 / +24.0 |
| compressor threshold Δ: base | +6.3 / +45.0 / +147.7 | +6.05 / +44.6 / +142.8 |
| … C2 | +2.6 / +13.6 / +28.3 | +2.81 / +14.4 / +29.9 |
| compressor attack Δ all: base / C2 | +112.3 / +28.8 | +106.9 / +31.0 |
| stereo mix 8 of 64: base / E1 + E2 + C2 | 285.8 / 251.5 | 285.6 / 254.7 |
| mono mix 8 of 64 with P1 + E1 + E2 + C2 | 168.5 | 163.1 (P1 spans only) |
| mono EQ 8 of 64 Δ: base / E1 (collapsed body) | not measured | +31.6 / +4.2 |
| mono compressor 8 of 64 Δ, P1: base / C2 | not measured | +33.7 / +32.0 (F4) |

**Native `Simd8`** (the diagnosis's in-process harness, 6 rounds × 800 blocks, load 13):

| row | diagnosis | reproduced |
|---|---|---|
| mono: untouched / written / P1 | 69.2 / 107.6 / 69.3 | 70.0 / 108.3 / 70.2 |
| mono 8 of 64: base / P1 | 147.8 / 91.9 | 148.8 / 92.5 |
| EQ Δ one / 8 of 64 / all: base | +9.0 / +67.7 / +88.7 | +8.62 / +67.3 / +89.3 |
| … E1 / E1 + E2 (all) | +1.8 / +8.9 / +29.8; +18.0 | +1.39 / +8.9 / +30.6; +18.8 |
| compressor Δ: base | +5.8 / +40.3 / +79.6 | +5.11 / +39.3 / +81.5 |
| … C2; C2 + C2b | +1.2 / +8.4 / +9.9; C2b quoted for 8 of 64 only: +8.9 | +1.00 / +10.3 / +12.2; +1.08 / +8.2 / +10.9 |

The collapse counters behave as the diagnosis says: 0 block-cohorts collapse after a write at
base, and 42,112 of 42,112 do with P1.

**Identity.**

* The compressor differential (`diag_ramping_prefix_randomized`, release, 320 seeds × 128 blocks
  × {`f32`, `Simd4`, `Simd8`} × {C2, C1, C2 + lean + C2b}) passed.
* The EQ bank differential (release, 300 scenarios × 96 blocks × W4/W8 × E1, E2, E1 + E2) passed.
  E1 elided sections on 4,040 ramping blocks at W4 and 3,859 at W8.
* The V8 digests of `base`, `p1spans`, `e12c2` and `all4` agree on 56 subject × arm pairs, with
  F6's caveat.

**Wasm.**

* `KERNEL_ROSTER` and the render callgraph pass unchanged on `p1`, `e1` and `e12`.
* Without the two `#[inline(always)]`, E1 **fails** rule 1: no arithmetic-carrying function
  matches `parametric_eq … f32x4 … process_bank`. Contract 3 is confirmed.
* C2: the dual kernel's vector count goes 516 → 1,084, and the collapsed kernel's scalar count
  0 → 9 (budget 26).
* Artifact growth over base (3,373,005 bytes):

| build | growth (bytes) |
|---|---:|
| P1 | +895 |
| E1 | +8,125 |
| E1 + E2 | +5,363 |
| C2 (carries C1 on sidechain blocks) | +53,316 |

**Bit-identity coverage (question 2):**

* Covered by both differentials:
  * ramps crossing block ends (lengths 1-127);
  * ramps ending mid-block (64-sample ramps in 128-frame blocks);
  * lanes starting and stopping (random retargets, 1-4 per block);
  * hostile audio;
  * both reset kinds (compressor only).
* Not covered:
  * **`Both` EQ targets** (the product path);
  * EQ resets mid-ramp;
  * hostile restored EQ integrators;
  * the compressor's restored `remaining = 0, current != target` state;
  * **boundary-value targets**. For example, a compressor knee below `MIN_SOFT_KNEE_DB` is inside
    the `[0, 24]` domain and takes `design_curve`'s `inv < +inf` select; a 24 dB high shelf is
    E1's unsafe-ramp path.
* NaN *targets* are unreachable: host domain checks, effect validation, and the EQ preparer's
  finite designs refuse them.
* The amended drafts require the missing cases.

## 3. The mono-collapse mechanism (question 1)

* **Path.**
  * Web: `COMMAND_EFFECT_PARAM`, `channel = 2`, `PerLane` → `into_effect_records`
    (`host-web/src/lib.rs:6197-6208`) → a `Left` and a `Right` record, both admitted or both
    refused (the three-pass room check), onto one SPSC queue.
  * `EffectControlLane::stage` (`effect-contract/src/live.rs:284`) → `admit` → `Desymmetrize`
    clears `LIVE`.
  * The EQ owner path is different: `prepared-control.js` → `eq_target_prepare` → one `Both`
    target, which is `Preserve`.
  * **The C ABI, the native host and the mobile host attach no effect console at all.**
    `HostConsoleRequest::control_queue_depth` is `None` everywhere except `host-web` and
    host-core's builtin batch endpoint. The web host is the only product path.
* **Retired for good?**
  * `LIVE` is set only in `EffectControlLane::with_target_staging`, at preparation, and nothing
    re-earns it.
  * Mono-collapse M3's recovery restores *channel agreement*, but engagement still needs the
    witness.
  * Resets, seeks, restores and bypass cycles leave `LIVE` cleared, and the web host has no
    plan-swap export.
  * So the loss lasts until the app disposes and re-boots the engine.
* **Ordinary operations that retire it:**
  * any compressor or limiter parameter command, including a restatement of the held value and
    every automation Point;
  * a left-only or right-only EQ edit;
  * a left-only or right-only trim or polarity edit (the builtins `LIVE` latches too).
* **Ordinary operations that do not:**
  * a both-channel EQ edit through the SDK;
  * trim, polarity or input filters with `channel = 2` (`Both`);
  * mute, fader, pan, matrix and solo, which are seam-side;
  * observation subscribe;
  * bypass, which declines only while it is engaged.
* **Split pairs.** The web host submits and renders on the AudioWorklet thread, so the two halves
  of a submission are always in one drain. A threaded host could split them. Spans-only P1 then
  clears `LIVE`, which is conservative and exact.
* **Queue at capacity.** Admission is all-or-nothing over both halves. Spans-only P1 touches
  neither admission nor staging, so an ack never precedes a drop. The host-`Both` form breaks this
  (F5).
* **One-sided edits still unlink.** A lone `Left` span has no twin, so `LIVE` clears. A lone
  one-channel target folds as today. Covered by the P1-break checks (`left` order) and required in
  the amended gate.
* **Safer form.** Spans-only drain pairing is recommended. It is exact, needs no admission,
  capacity or record-type change, and its only obligation is that `apply_automation` is
  channel-symmetric for a twin pair.
  * That holds for the compressor's and the limiter's validators, which I read: their order,
    capacity, kind, sample, value and duplicate checks are symmetric, and staging bounds the window
    by the capacity.
  * `automation-1` amendment A3 makes every other launch effect with live `PerLane` parameters
    prove the same. The host-`Both` form is also exact but carries F5's two preconditions. The owner
  rules.

## 4. C2b's invariant (question 3)

**Invariant.** `words[0..4][lane] == GainComputerCoef::new(ramps[0..3][lane].current)`, bit for
bit.

**Proven by enumeration on today's tree.** Every write of a curve ramp's `current` or of a curve
word is one of these:

* `design_lane` with a curve bit, which rewrites all four words from the current values. That
  covers `seed_from_defaults`, `full_reset`, `discontinuity_reset` (snap, then all four),
  `redesign` on restore (`state.rs:113`) and `advance_ramps`.
* `copy_state_from`, which copies words and ramps together.
* `LinearRamp::set_target`, which writes `current` only when it is already bit-equal
  (`stationary_at`) or when `samples == 0`. The compressor always passes `SMOOTHING_SAMPLES = 64`.

Each curve word depends on exactly one parameter (`effect-runtime/src/dynamics.rs:133`). No
reachable state breaks the invariant, including `-0.0` thresholds, hard and sub-`MIN_SOFT_KNEE_DB`
knees, and restored `remaining = 0, current != target` payloads.

It is still a maintenance invariant. A future write site would break C2b silently, for example an
instant `set_target(…, 0)`, a new reset kind, or a partial `design_lane`. The differential goes red
only if the reference kernel re-derives the words, so the named mutation is: `discontinuity_reset`
designs with `changed = 0`.

The value is small: natively, 8 of 64 goes +10.3 → +8.2 µs and all 64 goes +12.2 → +10.9 µs. The
owner rules.

## 5. The drafts

Each draft now carries an `Amendments` section. In summary:

* **`automation-1`**:
  * scope cut to spans-only pairing;
  * product outcome corrected (the EQ is not affected);
  * F1's counterexample added as a gate, both natively and through `host_web.wasm`;
  * O(n) adjacency pairing;
  * mono V8 rows;
  * the host-`Both` alternative's F5 preconditions recorded.
* **`automation-2`**:
  * `Both` targets, mid-ramp resets and boundary shelves added to the differential;
  * a collapsed V8 row, and a no-regression gate on the mono settled row;
  * mutations M6 (the dual list's `dead` without the not-ramping term) and M7 (`unit_m0` without
    the step check).
* **`automation-3`**:
  * the collapsed rewrite (contract 4) made the primary, measured deliverable;
  * knee and ratio boundary cases, restored-state and relaxation fixes;
  * a C2b invariant mutation;
  * mono V8 rows.
* **`automation-4`**: `Both` targets and the last lane in the unit test, a non-panicking
  `lane_mask`, and mono rows.
* **`automation-5`**:
  * the traffic rewritten to the host's shapes;
  * held-value bases;
  * the browser arm on `host_web.wasm`.

## 6. Fix order

1. **`automation-5`, amended.** It is measurement only; land it first so that the others show.
2. **`automation-1`, spans only.**
   * V8 mono console once any compressor or limiter is touched: 248 → 148 µs.
   * 8 of 64 automated: 290 → 173 µs.
3. **`automation-2` (E1).**
   * Stereo 8 of 64 EQ: +64 → +11 µs.
   * Collapsed: +31.6 → +4.2 µs.
4. **`automation-3` (C2), collapsed body included.**
   * Dual: +44.6 → +14.4 µs.
   * Collapsed: unmeasured, and it must be measured.
5. **`automation-4` (E2).** All 64: +38 → +24 µs under V8. It follows E1.

## 7. For the owner

1. **P1's form.**
   * Recommended: spans-only drain pairing.
   * Alternative: the host `Both` record, which needs capacity ≥ 2 × queue depth and
     parameter-policy knowledge at the drain (F5).
   * The prototype's target half must not ship either way.
2. **C2b.** It is exact by an invariant that holds today. Is a debug assertion plus the named
   mutation enough, for about 1-2 µs natively?
3. **The benchmark's browser arm.**
   * The console benchmark's wasm arm is a wasmtime guest, not a host path.
   * Should `console_mixing_automation`'s browser arm be `host_web.wasm` under V8 through the SDK?
   * Recommended: yes; that is the only real browser path.
4. **Unchanged from the diagnosis:** the law-changing ideas in its section 8 need DSP evidence. No
   projection is quoted.
