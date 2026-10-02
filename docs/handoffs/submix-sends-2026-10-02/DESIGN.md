# Submix strips, live aux sends and VCA groups: design and decision record

Planner brief (role "Sol briefs"), revision 2, 2026-10-02, against `main` at `fe8ac679`. Nothing
here is filed, committed or pushed.

Revision 2 answers the second adversarial review, `VERIFY-2.md` (verdict PASS-WITH-FIXES: 2 new
BLOCKERs N1-N2, 19 MAJORs M1-M19, 25 MINORs and a scope recommendation). `REVISION-2.md` maps every
finding to its resolution and gives the old-to-new file map. Revision 1's answers to `VERIFY-1.md`
(`REVISION-1.md`) stand except where revision 2 says otherwise. Every anchor this revision adds or
changes was re-read on `fe8ac679` by the reviser.

The issue bodies are in `issues/`. Section 9 is the issue list, the DAG and the batches. Section 10
is the deferred list: work that is designed but deliberately **not** filed.

What changed in revision 2:

- **`follows_mute` is legal only on a route into a submix** (N1). A route into the output is never
  live, so a follow there could only be prepared, and a live strip unmute would leave the strip's
  main route silent until a plan replacement.
- **K1 keeps live-controlled browser boots working** (N2). Until slice 10 files bus effects in the
  browser, effect live controls and observation handles attach only to track-owned effects (P16).
- **One coefficient function, placed where it can live** (M1, M2). `route_coefficients` is in
  `crates/graph-compiler`, built on one pure `crates/graph` function that the runtime's bind also
  calls; a prepared route carries an explicit `RouteGate`, which the canonical text records.
- **Body-level correctness fixes**: the fold planner sees live routes (M3); the browser's mute
  composition has one implementation (M4); strip lookup is segment-aware (M5); host-web's
  per-strip effect tables move to the start of K2 (M6); ID staging covers submix and route IDs
  (M7); every authorized-path gap and gate command VERIFY-2 found is fixed in its body (M8, M9).
- **Scope** (VERIFY-2 section 4, adopted in full):
  - the benchmark rows and the baseline leave the umbrella as two successor performance issues,
    BM1-BM3; attribution uses `graph::test_only_phase_profile`, and the route-fusion decision is the
    weekly performance pass's, on BM3's numbers (M16, M17);
  - the boot-word rename is dropped (M14, P17);
  - the former slice 26 is deferred item O11;
  - VCA starts once batch K3 closes, not when umbrella A closes;
  - the former slices 03, 12, 13, 15 and 17 are split along the seams VERIFY-2 named, and 09 and
    25 along seams the reviser found (09 grew by M6; 25 by VERIFY-2 MINOR 23);
  - the former 07 and 08 merge, and the former 10 and the remainder of 11 merge.
- **Size.** Umbrella A has 30 slices (00-28, with 18 split into 18a and 18b along VERIFY-3 MINOR
  4.1's seam). Three successor performance issues (BM1-BM3) sit
  outside it, and the VCA umbrella has six files (V0-V5).

---

## 1. Summary

**Submixes become strips.** A submix carries exactly what a track carries after its source:

- the input section (`polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz`, `delay_samples` per lane);
- every session console slot, each with its own `{slot, bypass, params}`;
- ordered inserts;
- a per-lane fader and mute;
- `pan` or `matrix`.

Its input is the D9 sum of the routes that target it. The graph lowers it with the track's stage
nodes and racks, keyed by the submix's ID, so its console slots bank with the same programs at their
own dependency level, padded, exactly as decision 12 requires. A submix can be tapped at any of the
seven send points.

**Sends are routes.** An aux send is a route from any strip tap to a submix input:

- fan-out is several routes;
- nesting is a route from one submix's tap to another's input;
- there is no compiled count anywhere, and acyclicity is the graph compiler's existing check.

Every route gains `mute`; a route into a submix may also set `follows_mute`. When a plan is
prepared **with live controls** (opt-in: the browser's default is 0, and the producer's mixer opts
in), route gain, mute and matrix into a submix are **live**: they ramp on the render plane with the
*indexed ramp* law (5.7). Adding, removing or re-pointing a route, and any edit to a route into the
output, stay plan replacements.

**SIMD.** The correctness slices keep today's route ops and reductions, so nothing is optimised
blind. Two successor performance issues outside the umbrella put a real bus-and-send session on
both benchmark paths (native AVX2 and the shipped module under V8) and record its baseline with a
phase-profile attribution of the route work. Whether route fusion (O1) earns a brief is the weekly
performance pass's decision on those numbers (6.3). One implementation shape serves every target;
only the lane width differs.

**Delegated decisions** (2.2, both upheld by VERIFY-1 with corrections folded in):

- **(a) VCA groups are in scope, as a separate umbrella that starts once batch K3 closes.**
- **(b) The submix strip is the same dual-mono strip as a track**, with all five input-section keys.
  It is never mono-collapsed, because the host's collapse eligibility set is built from tracks only.

---

## 2. Decisions

### 2.1 Owner decisions (binding)

| # | Decision | Where it lands |
|---|---|---|
| O1 | Submixes carry the same session console as tracks: every submix carries every console slot, with its own `{slot, bypass, params}` per slot, plus ordered inserts, fader/mute and `pan`/`matrix`. | 02, 03, 04, 05 |
| O2 | Reverb is out of scope. `miso.delay` is the send effect in tests. It declares zero latency, so PDC fixtures use the true-peak limiter (486 samples at 48 kHz). | fixtures throughout |
| O3 | VCA and the submix channel model were delegated to the planner and adversarially verified (VERIFY-1 upheld both). | 2.2 |
| O4 | Agent-driven: no human-UX limits. Arbitrary bus counts, nesting, any tap, a full 2x2 per send, fan-out; acyclic; configured resources only, never a compiled maximum. | throughout |
| O5 | SIMD is a core philosophy. | section 6 |

Standing rulings that bind this design:

- **Decision 12** (`docs/rulings/engine-footprint-2026-09-29.md`):
  - console slots always bank, padded, one group per (slot, pool class, dependency level);
  - a bypassed lane stays in its bank;
  - latency is always paid;
  - banking couples cost, never bits.
- **Dual mono by source file.** Only a declared mono source takes the mono collapse. There is no
  content detection.
- **Summation order.** A class-B reordering of a sum is allowed only with a measured win, a pinned
  new order and re-baselined digests.
- **Benchmark real paths only; a fold or fusion is committed only on a measurement from a real host
  path.**
- **The 2 % allowance.** It is a judgement tolerance for a genuinely better change, never a pass/fail
  gate on one run.
- **Live control (decision 1, 2026-09-28).** Ramp lengths come from the session's smoothing table
  (#1054, open), with a per-change override. Until #1054 lands, a browser change uses the smoothing
  its record carries, else 0. On the C ABI, every live record uses the fixed ramp #1053 rules (its
  A2 D3 recommends 5-10 ms derived from the rate at preparation), and a per-change smoothing only
  where the wire carries one; route edits carry none (VERIFY-2 M18).
- **The acked-batch question.** No queue may ever ack before a drop.

### 2.2 Delegated decisions

#### (a) VCA groups: in scope, filed as their own umbrella once K3 closes

**Verdict.** Yes. The VCA umbrella (`issues/V0`-`V5`) is filed when batch K3 closes (slice 26's
verdict and the K3 push), with its anchors re-verified then. V1-V4 need only K3's per-strip mute
state and follow composition; only V5 waits on slice 28 and #1053. Nothing in the bus and send work
depends on VCA.

**Why yes** (the arguments VERIFY-1 upheld):

1. **The intent must live in the session.**
   - The engine owns the edited session on every platform, and apps save the engine's canonical
     JSON.
   - V1 has no free-form metadata.
   - An agent that emulates a VCA by rewriting member faders has nowhere to keep the grouping, or
     each member's own value, where a fan's phone or the next agent can see it.
   - Once a member clamps at the `[-144, 24]` dB edge, the saved session has lost the balance for
     good.
2. **The audio semantics are the industry's.** dB offsets sum over a reach set, each VCA counted
   once. Mute is an implicit OR that keeps the member's own mute. There is no audio path. Post-fader
   sends follow, because the offset lands on the member's fader. Pro Tools, S6L, DiGiCo and SSL all
   behave this way (section 4). Clamping to the fader domain is the engine's form of S6L's cap.
3. **It needs no render code.** The effective gain is folded into the fader coefficient the member
   already applies. A live VCA move is member fader records through queues that already exist.

Two earlier arguments are withdrawn as weak:

- "32 commands per gesture instead of one": the browser admits a batch atomically, and #1053's
  transaction carries many edits.
- "It overwrites the producer's values in a fan's personal mix": a personal mix is a copy, by
  ruling.

**Semantics** (the VCA umbrella carries them):

- `vcas: [{ id, fader: { left_db, right_db, left_mute, right_mute }, members: [id...] }]`, a required
  root key.
- Members are strips or other VCAs, acyclic (`vca.cycle`).
- `reach(strip)` is the *set* of reaching VCAs.
- Effective gain per lane: `clamp(member_db + sum(v.lane_db for v in reach), -144, 24)`, summed in
  `f64` in a fixed order (the member, then the VCAs by ascending ID), rounded once.
- Effective mute per lane: `member_mute || any(v.lane_mute)`. It is computed once, by one session
  helper, and feeds both the member's prepared fader section and the prepared `source_lane_muted`
  of every `follows_mute` route the member is the source of (VERIFY-2 M12).
- **Strip mute ownership:** VCA mute joins the strip-mute state of P7 as a per-lane `vca_mute` input.
  It reaches tracks **and** submixes, so a submix member is not left without an owner.
- **#1053 coordination:** P13.
- **C ABI cap:** `maximum_vcas` takes `reserved[1]` of the compile limits (P12).
- **Out of scope:** VCA solo, VCA trim of send levels, and VCA automation.

#### (b) Submix channel model: the dual-mono strip, never collapsed

**Verdict.** A submix strip is the same strip a track has after its source mapping. Under
`AGENTS.md`'s law it is dual-mono:

- L and R have independent state and parameters;
- a processor links channels only through its declared `link_mode`;
- cross-channel flow is only the explicit smoothed 2x2 or pan.

In the owner's source-file vocabulary it is a *stereo* strip: it never takes the mono collapse.

**Why.**

1. **One strip shape.** The bus reuses the track's parse, validation, canonical form, BTLV,
   lowering, bank programs, live commands and meters. A "stereo" strip with one shared parameter set
   would need a second grammar and a second family of bank programs, and would buy nothing an insert
   with `link_mode: maximum` or `average` cannot already do.
2. **It matches the consoles.** DiGiCo builds stereo buses from linked mono legs with per-leg sends
   (D-SD pp.39-41, 59). S6L and SSL link their stereo strips. That is what `link_mode` is.
3. **No collapse, and the guard is the eligibility set.**
   - `arm_mono_collapse` (`crates/graph/src/runtime.rs:2326-2338`) arms a chain only when all of
     these hold:
     - `gathers_track_input()`;
     - the lane list is non-empty;
     - every lane's ID is non-empty **and** in the host-supplied `eligible` set.
   - `gathers_track_input` is a pure node-shape test (slot 0 of every lane is a `PostInputBuiltins`
     stage, `runtime.rs:5208-5222`), so it **will** hold for every bus chain.
   - The `eligible` set is `session_structural_symmetry` over **tracks only**
     (`crates/builtins-compiler/src/lib.rs:3839-3862`), filtered at
     `crates/host-core/src/prepare.rs:1284-1289`.
   - The guard is therefore: *the eligibility set stays tracks-only.* Seeding the bus as pool class
     `Stereo` explicitly is harmless but not sufficient. Slice 03 names the set as the guard and
     gates it.

**Consequence for bus compression: a hazard, not a footnote.** A console slot's `link_mode` is
session-level, and every bus carries every console slot (O1). If the session declares the console
compressor `dual_mono` for tracks, that compressor runs **unlinked** on every stereo bus and shifts
the image under asymmetric material. The guidance, carried by slice 08's `author-session` skill
update and its app handoff, is: bypass the console compressor on buses, and use a linked insert for
bus glue. Whether a submix may override a slot's `link_mode` is owner question Q1 (8.2).

**Input section: all five keys.**

- `trim_db` gain-stages the bus into its own processing (DiGiCo gives every output trim and
  polarity, D-SD p.12).
- `polarity_invert` is the sign of the trim coefficient, so it costs nothing.
- A disabled HPF or LPF is the arithmetic identity.
- `delay_samples` lowers no node at zero, and gives buses the delay DiGiCo, S6L and SSL offer. It
  stays a musical shift that PDC never compensates.
- The input stage is where D7 sanitizes, and a bus input is a new entry point.
- A reduced bus input section would need its own grammar, codec and kernel arm for no gain.

### 2.3 Planner decisions (revised)

| # | Decision | Reason |
|---|---|---|
| P1 | A bus strip lowers to the existing `TrackStage` nodes and `EffectNodeId`s keyed by the submix's ID. `GraphNodeId::Submix` is no longer emitted by the graph compiler; the graph crate keeps the variant for its own corpus. `reduction_records` and the graph-compiler estimate count a submix strip's `Input` reduction (otherwise bus sums vanish from the canonical text). Compiler paths (including the sealed sidechain edge path, `compile.rs:377`) come from the strip's path prefix, and the four sealed collection edge paths from its collection path (`$.tracks` or `$.submixes`, VERIFY-2 M10). A nonzero submix `delay_samples` delays the **sum**, in its own runtime arm (slice 04). | Tracks, submixes and outputs share one ID namespace (`crates/session/src/validate.rs:69-100`). Banks, builtins, meters, PDC and taps then work unchanged. `reduction_records` (`crates/graph-compiler/src/ids.rs:77-105`) records only `Submix` and `Output` nodes today. The track's `TrackDelay` arm runs on a source input and returns before any reduction (`runtime.rs:2980-2989`, `:4026-4041`), so it cannot delay a sum. |
| P2 | Route source `{ kind: "submix", submix_id, tap }` replaces `{ kind: "submix_output", submix_id }`. Wire tag 2 is kept, and its `TAP` (field 3) becomes required. `submix_output` becomes an unknown token. The same spec is the sidechain source (`route_source::KNOWN`, `crates/protocol/src/schema.rs:826-856`), so a sidechain keyed from a bus also names a tap. | Symmetric with `{ kind: "track", track_id, tap }`. An in-place V1 amendment on the #1063 and decision-12 precedent. |
| P3 | `Route` gains `mute: bool` (route field 6). A muted route stays in the graph. | Send on/off is universal (section 4). Keeping the edge makes unmute value-only. |
| P4 | **Contribution and activity** (see 5.4 and 5.7). A route is *inactive for a block* when its `RouteGate` silences it (muted, or follow-muted on both source lanes), its edge carries **no** compensation delay, and (if live) its ramp had settled at the start of that block. An inactive route is neither mixed nor loaded. A sum is `first + sum(active later inputs)` in D9 edge order, and the first input in edge order always owns the store (`+0.0` when inactive). When an inactive route is a destination's only input and would be read in place, the destination fills `+0.0` (VERIFY-2 MINOR 1). **A delayed route is never inactive:** silenced, it mixes its zero target, so its compensation line holds the fade and then zeros, never stale audio. Activity is decided once per block, after the live drain, and never changes inside a block. | It is class A by definition against an oracle that implements it. A static store owner keeps future folds possible. The delayed-route rule fixes VERIFY-1 BLOCKER-1 at zero extra machinery, and the block-start rule fixes VERIFY-1 MAJOR-1. Route destinations are never bank members, so a bank gather never bypasses the reduction (`bank_gather_source`, `runtime.rs:2949`). |
| P5 | **The indexed ramp law** (it is not D11). A live route's four coefficients are a pure function of the frame index within the ramp. `length` is at most 2^22, and `position` saturates at `length` (5.7). A record whose `round(target - start)` is not finite applies as a step, so no step is ever NaN (VERIFY-2 MINOR 3). | The deferred fused and fold designs in section 10 need coefficients computable in any traversal order. VERIFY-1 simulated 700 ramps: no monotonicity or overshoot violation, and worst error 2.26 ulp against D11's 17,180. VERIFY-2 confirmed no overshoot while `(L - 1) * 3u < 1`, so `L <= 2^22` is safe. |
| P6 | Route ramp lengths: in the browser, the change's own smoothing, until #1054 gives a session table (then gain uses the fader entry, mute the mute entry, and matrix the pan entry). On the C ABI, #1053's ruled fixed ramp (2.1). | No new session field here. A fan's device renders the producer's ramps once #1054 lands. |
| P7 | **Strip mute has one owner.** `host_core::LiveControlSoloState` is sized per **strip** (tracks, then submixes). Submix entries are solo-safe: never soloable, never solo-muted. Command kind 4 (mute) on any strip goes through it. Effective mute is `user_mute || (any_solo && !solo_safe && !soloed)`, and the VCA umbrella adds `vca_mute`. `effective_mute(strip, lane)` is the **only** composition: host-web's inline copy at kind 4 (`hosts/host-web/src/lib.rs:4374`) is deleted (slice 16, VERIFY-2 M4). Follow-mute composition reads this effective mute for every strip. | host-web lowers kind 4 through this state (`lib.rs:4349-4380`). A separate submix mirror would split mute ownership. A soloed track stays audible through its buses and returns (Pro Tools defaults aux inputs to solo-safe, PT p.367). The inline copy ignores solo-safe, so a bus could not be unmuted while any track is soloed. |
| P8 | Session-edit opcodes that address a track's strip (`0203`-`0211`) address a **strip** ID: a submix ID resolves to the submix's strip. `0200`, `0201` and `0202` stay track-only. Inside a transaction, a strip ID resolves tracks first, then submixes (VERIFY-2 MINOR 14). | No parallel opcode family. An in-place V1 amendment. A candidate model is validated only at the end of its transaction, so the order must be fixed. |
| P9 | The browser's 48-byte record index word addresses a strip index (tracks, then submixes, each in canonical ID order) for strip kinds, and a live-route index for route kinds. host-web resolves an ID to a strip index with a segment-aware lookup: a binary search of the track prefix, then of the submix suffix; each segment is sorted, the concatenation is not (VERIFY-2 M5). `live_control_tracks()` stays the track prefix. | Existing track indices are unchanged, and a session without submixes is byte-identical. |
| P10 | The browser meter frame becomes `3 * (T + S) + 3` words. `WebMeterHeader` **appends** `submix_count` and a reserved pad (both reserved words are in use), growing from 64 to 72 bytes. The `miso.meter.v1` message keeps `peaks` (`2T + 2` words), `trackGrDb` (`T`) and `masterGrDb` exactly as today and appends `submixCount`, `submixPeaks` (`2S`) and `submixGrDb` (`S`) (VERIFY-2 M8). | `S = 0` leaves every frame word and every existing message field where it is today; only the header grows. |
| P11 | **Follow mute is a send's property.** A route into a submix carries `follows_mute: bool` (route field 7), in its own slice (20). A route into the output must have `follows_mute: false`: `true` refuses with `schema.invalid_enum` at `$.routes[<i>].follows_mute`, on the precedent of a closed token that is illegal in its context (`crates/session/src/validate.rs:646-652`). The SDK builder defaults `followsMute` to `true` for a route into a submix and always writes `false` for a route into the output. The one coefficient function is `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted: [bool; 2])`. It zeroes a source column whose lane is muted, and the route is inactive when both lanes are muted (subject to P4's delay rule). It is built on `graph::gated_route_coefficients(&RouteTransform, RouteGate)`, the pure function the runtime's bind also calls, so prepared and live bits cannot drift (VERIFY-2 M1, M2). | It closes the pre-fader leak under mute and solo (the probe measured 0.3155/0.3549 where 0 was expected). SSL defaults Follow Mute on. A follow-muted send is inactive, so the solo case gets its skip at no extra cost. A route into the output is never live (5.7): a follow there could only be prepared, and a live unmute of its source would leave the strip's main route silent until a plan replacement, on the browser and after slice 28 on the C ABI (VERIFY-2 N1). A `post_pan` main route is already gated by the fader. `crates/graph` cannot hold the function: the gain conversion is `math::db_to_gain_f32`, and graph's production dependencies are pinned (`scripts/check-graph-policy.sh:21-22`). |
| P12 | Host caps gain `maximum_submixes` (and, in the VCA umbrella, `maximum_vcas`). In the C ABI, `miso_engine_v1_compile_limits.reserved[0]` becomes `maximum_submixes`, where 0 means "use `maximum_tracks`" (and VCA later takes `reserved[1]`). The struct stays 208 bytes; `maximum_submixes` takes offset 176 and the shrunk `reserved[3]` moves to 184, which re-pins `crates/capi/src/abi.rs:454` and `crates/capi/tests/c/abi_smoke.c:45`. | It is a configured resource, never a compiled maximum. It does not silently change the meaning of `maximum_tracks` (VERIFY-1 MINOR-6). Every existing caller already passes zero (`limits_are_valid`, `crates/capi/src/runtime/compile.rs:354-358`). Because zero is meaningful, `maximum_submixes` (and later `maximum_vcas`) stays out of `all_limits_nonzero` (`compile.rs:326-352`); `prepare_caps` maps 0 to `maximum_tracks`. |
| P13 | **#1053 coordination.** #1053 classifies a committed-model delta as live when `fader` and `matrix_or_pan` are the only fields that differ, and pushes only track lanes. Until the C ABI composes the dependants, these deltas are **structural**: any change to a field of a **submix strip** (until slice 27); a change to `left_mute` or `right_mute` of a strip that, in the post-commit model, is the source of a `follows_mute` route (until slice 28); and (VCA umbrella) any fader field of a VCA member (until V5). Slice 00 annotates #1053's spec with the rule. If #1053 is already on `main`, the slice that creates the dependency implements it in #1053's classifier: slice 02 (submix-strip fields), slice 20 (follow sources) and V2 (VCA members). Slices 10 and 14 rename host-core handle fields #1053's capi code reads (`tracks` → `strips`, `track_controls` → `strip_controls`); whichever of a renaming slice and #1053 lands second updates the other's uses (VERIFY-2 M11). | Whichever lands second carries the guard, so `main` never renders other than its committed model (VERIFY-1 MAJOR-3). |
| P14 | **No performance tier is committed blind.** The correctness slices keep route ops. Three successor issues outside the umbrella, BM1-BM3, put a real bus-and-send session on both benchmark paths and record its baseline with a phase-profile attribution. Route fusion (O1) and the other tiers stay deferred; the weekly performance pass decides on BM3's numbers. | The owner's rules: benchmark real paths only, and justify a fold or fusion with a measurement. `AGENTS.md`: benchmark machinery must not keep a usable product slice open, and systematic optimisation belongs to the weekly pass. #957 deleted a fused route-reduction family that no host reached. |
| P15 | `Submix::unity(id, console)` and the SDK's spec-less `submix(id)` build a **transparent** strip: identity input section, no inserts, 0 dB unmuted fader, identity matrix, and every console entry `bypass: true` with default parameters. Latency is still paid (decision 12). | A migrated bare bus keeps its sound. The SDK requires explicit entries for a track (`normalizeConsoleEntries`, `sdk/src/core/session.ts:1161-1224`); a spec-less submix is the one place a default is needed. |
| P16 | **K1 interim for live controls.** From slice 03 until slice 10, `effect_compiler::attach_effect_live_controls` and `attach_effect_observation` attach only to prepared effects whose owning strip is a track. A bus effect renders through its live-control-free path meanwhile. Slice 10 files bus effects in the browser and removes the rule. | After slice 03 the effect compiler prepares bus effects, and host-web refuses any producer or observation handle it cannot file (`web.live_controls.effects`, `hosts/host-web/src/lib.rs:5921-5927`; `.observation`, `:5990-5995`). Without the rule, the K1 push would break every live-controlled browser boot of a session whose bus carries an effect (VERIFY-2 N2). Lanes with and without a channel already share banks (`control: None` entries, `crates/effect-compiler/src/prepare.rs:605`). An observation lane exists only beside a control lane (`runtime.rs:4051-4053`), so both attach functions use the same rule. |
| P17 | **Track-named spellings keep strip meaning.** The browser boot word `live_control_master_track_plus_one`, the meter header's `master_track_plus_one` and `track_count`, the SDK option `liveControls.masterTrackPlusOne`, the record's `track_index`, host-core's `HostLiveControlRequest.master_track`, and the codes `builtin.meter.unknown_track` and `host.observation.master_track` keep their spellings. Where one now indexes strips, its doc says "strip index: tracks first, then submixes". Rust-internal names that are no wire or SDK spelling may be renamed where it clarifies (`HostLiveControlHandles.strips`, `strip_controls`, `HostMeterRequest.strip_id`). | Tracks lead the strip order, so every existing index is unchanged. Renaming a public SDK option or a layout name forces an app migration for no wire change (VERIFY-2 M14: 28 tracked files spell the boot word). |

---

## 3. What exists today (verified)

### 3.1 Code facts

- **A submix is `Submix { id }`** (`crates/session/src/model.rs:577`), parsed with only `["id"]`
  (`parse.rs:1283-1288`). Its wire message has field 1 only (`crates/protocol/src/schema.rs:1011-1018`).
  - The compiler emits one bare zero-latency node (`crates/graph-compiler/src/compile.rs:296-306`).
  - It runs as `NodeKind::Identity`, a reduction and nothing else (`crates/graph/src/runtime.rs:4025-4067`).
- **Pan or matrix.** A track carries exactly one of the JSON keys `pan` or `matrix`
  (`parse.rs:1222-1281`, track keys `:977-998`).
  - The Rust model field is `matrix_or_pan: MatrixOrPan` (`model.rs:240`, `:550-574`).
  - On the wire it is track field 10, tagged pan 1 and matrix 2 (`schema.rs:983-984`, `:938-962`;
    the track field registry is `crates/session/src/visit.rs:94`).
  - The canonical track key order is `id, source_id, left_source_channel, right_source_channel,
    builtins, console, inserts, fader, pan|matrix` (`crates/session/src/visit.rs:209-212`).
  - builtins-compiler diagnostics spell the model name instead: `$.tracks[id=..].matrix_or_pan.<field>`
    (`crates/builtins-compiler/src/lib.rs:4798`, `:4850-4865`). That is a pre-existing quirk, kept.
- **Validation paths.** Session validation uses index paths: `$.submixes[0].id`,
  `$.routes[0].source.submix_id`, `$.tracks[i]...` (`validate.rs:85-91`, `:303`, `:473-474`).
  The pinned tests are `crates/session/tests/invalid_matrix.rs:376`, `:602`. Only the compilers use
  `[id=..]` paths (`graph-compiler/src/ids.rs:144`, `:157`; `builtins-compiler/src/lib.rs:4791`;
  `effect-compiler/src/prepare.rs:342`, `:1466`, `:1591`).
- **Exactly one output.** `compile_graph` refuses `model.outputs.len() != 1` with
  `graph.output.cardinality`. N output buses (#210) stay out of scope.
- **Routes.** `Route { id, source, destination, channel_matrix, gain_db }` (`model.rs:591`), with no
  mute.
  - `route_transform` (`crates/graph-compiler/src/ids.rs:273-289`) converts the gain with
    `math::db_to_gain_f32` and checks finite, non-subnormal gain and coefficients. Session validation
    refuses non-finite values first (`validate.rs:481-491`).
  - `PreparedRoute { node, transform }` (`crates/graph/src/lib.rs:2204-2207`) carries the
    **unfolded** `RouteTransform` (`:742`). The canonical text writes those unfolded bits as one
    `route-transform` row per route (`crates/graph-compiler/src/canonical.rs:262-273`).
  - The gain is folded into the 2x2 once at bind by the private `const fn folded_route`
    (`runtime.rs:5990`). Its callers are `node_kind` (`:4063`) and `plain_route_gains` (`:6122`),
    through which `route_fold` reaches it (D3, `runtime.rs:22-24`).
  - Route arithmetic is `l' = fma(lr, r, ll * l)`, `r' = fma(rr, r, rl * l)`
    (`crates/lane/src/kernels.rs:1027`). `Lane::fma` is two IEEE roundings on every target. So the
    left source column is `(ll, rl)` and the right is `(lr, rr)`.
  - **All six route edits `0500`-`0505` are structural** (plan replacement).
- **`crates/graph`'s dependencies are pinned** to `effect-contract`, `engine`, `lane` and `rack`
  (`scripts/check-graph-policy.sh:21-22`). host-core depends on graph-compiler
  (`crates/host-core/Cargo.toml:25`); host-web reaches graph types only through host-core.
- **D9.** A reduction sums its inputs in stable edge-ID order, left to right; the first input stores
  (`reduce_plane` `runtime.rs:399`, `reduce_many` `:426`, groups of `REDUCE_GROUP = 8`). A single input
  is read in place when its op is the sole reader of a non-dedicated buffer (`crates/graph/src/program.rs:715-745`,
  `is_dedicated` `:231`), and today's `[single] if *single == out` arm is then a no-op
  (`runtime.rs:401-406`; `:568-572` for the Output). So a route that is the sole reader of a
  mid-chain tap owns **no** buffer.
- **PDC staging belongs to the consumer.**
  - `execute_op` (`runtime.rs:2970`) copies each delayed input's producer buffer into a staging
    scratch and runs `delays[line]` **inside the consuming op**, before its reduction
    (`:2996-3006`). Staging runs unconditionally for every `op.staged` entry.
  - `DelayRef { line, staging }` is per consumer input (`program.rs:49-58`).
  - A route node has exactly one input, so its compensation 0 (`pdc.rs:63-73`); a compensation delay
    can sit only on its `RouteDestination` edge, and the consumer's `InputRef.delay` is a complete
    test of "delayed".
  - A compensation line therefore holds **post-route-mix** samples. Arena buffers are coloured and
    reused, so a buffer not written this block holds another op's samples.
- **Fold planning.** The fold asks `PlanningMetadata` (trait `runtime.rs:6007`, impls for `RuntimeParts`
  `:6016` and `BorrowedPlanningMetadata` `:6089`), whose doc requires its exclusions to stay coupled to `node_kind`
  arms. `plain_route_gains` (`:6110-6122`) checks only source, bank membership, binding and effect.
- **The route fold** (#218, #916; `route_fold`, `runtime.rs:6351`):
  - It scatter-accumulates folded lanes into **one** master per plan.
  - It is all or nothing per chain.
  - The master must be a plain, source-less, non-member op with no delayed input. A bus `Input`
    can be a fold master.
  - The template for any generalisation is `FoldLane`, `fold_plane`, `fold_cohort` and
    `fold_resident_tiles` (`:1613-1946`). The rack's aux seam (`crates/rack/src/lib.rs:1704`, `:2107`)
    holds one optional destination per lane, adds raw results with `+=` in a scalar strided loop, and
    is reserved for #210's PFL. It is **not** that template.
- **A strip is track-only at every layer.**
  - Banks: `graph-compiler/src/banks.rs:148-196`.
  - Builtins preparation, seals and banks: `builtins-compiler/src/lib.rs` (the seal's `tracks`
    `:3518-3523`, `processor_seal` `:3069-3085`, `planned_strip_banks` `:1279-1294` used at `:2296`,
    `:2321`, `:2445`, `track_parameters` `:4706-4749`, and the diagnostic path builders
    `parameter_diagnostic` `:4777`, `gain_path` `:4805`, `cutoff_path` `:4820`,
    `filter_order_path` `:4835`, `matrix_path` `:4850`).
  - Effect and edge paths: `effect_path`, `graph-compiler/src/ids.rs:232-241`, the literal
    collection string `"$.tracks"` at `compile.rs:249`, `:287`, `:293`, `:294`, and the sidechain
    edge path `$.tracks[id=<track>].sidechain` at `compile.rs:377`. These are written
    into **sealed canonical edge paths** (`canonical.rs:232`; each edge row of
    `fixtures/graph/v1/direct-route.canonical.txt:30-35` ends `\t$.tracks`).
  - The session estimate: `crates/session/src/estimate.rs:64-99`, `:131-160`.
  - host-core shape and counts: `crates/host-core/src/shape.rs:79-81`, `count_effects`
    `prepare.rs:1313-1324`.
  - Meters: the graph refuses observers off a `TrackStage` or the Output (`crates/graph/src/lib.rs:1591-1596`).
  - Live controls: `host-core/src/prepare.rs:887-928`. With any queue depth, host-core attaches a
    live channel to **every** prepared effect (`attach_effect_live_controls`,
    `crates/effect-compiler/src/prepare.rs:1457-1534`) and, with observation taps, an observation
    lane (`attach_effect_observation`, `:1577-1641`).
  - Solo: `host-core/src/solo.rs:89`, sized per track.
  - host-web: its per-track effect tables (`rack_effects`, `effect_base`, `hosts/host-web/src/lib.rs:5872-5913`)
    are built from `model.tracks`; it files every producer and observation handle by binary search
    over `handles.tracks` and refuses a miss (`:5921-5927`, `:5990-5995`; also `resolve_observation`
    `:5076-5078`); the command guard is at `:4323-4326`, and `queue_count` at `:5970-5973`.
  - host-web's kind 4 composes the effective mute inline (`lib.rs:4374`:
    `muted || (any_solo && !solo(track))`).
  - The ID staging buffer is sized from `max(longest_source_id_bytes, longest_track_id_bytes)`
    (`hosts/host-web/src/lib.rs:1918-1925`, `crates/host-core/src/shape.rs:62-77`).
- **Live controls are opt-in.**
  - `live_control_command_queue_records == 0` means no control channel, and
    `WebBootOptions::explicit_defaults()` sets 0 (`hosts/host-web/src/lib.rs:1102-1104`, `:1137`).
  - The SDK also defaults to 0 (`sdk/src/core/abi.ts:186`).
  - The producer's mixer (misofm/app) opts in with the published constant
    `defaultCommandQueueRecords` = 64, and so does the V8 mixing benchmark.
  - The C ABI prepares with no live controls (`crates/capi/src/runtime/compile.rs:408-410`) until
    #1053 attaches them. #1053's A1.2 is still open: it may add a builtins-only live-control request
    instead of attaching effect lanes.
- **Strip mute in the browser** is lowered through the solo state:
  `ready.solo.set_user_mute(track, ..)` (`hosts/host-web/src/lib.rs:4349-4380`). The state emits mute
  deltas only, never `FaderDb` (`host-core/src/solo.rs:82`, `:236-254`). Kind 3 (fader dB) goes
  directly to the fader queue (`:4330-4348`).
- **The bounded-drain pattern** is `available_at_entry` (`crates/engine/src/realtime/spsc.rs:420`),
  used by the input drain. #1053 bounds the four unbounded `while let Ok(..) = try_pop()` drains
  (`builtins-compiler/src/lib.rs:978`, `:1014`, `:4163`, `:4229`).
- **Silence.** The graph has no silence flags. Effects skip a positive-zero block bank-wide
  (`block_is_positive_zero`, `crates/effect-runtime/src/bank.rs:143`). The silence handoff's A0 and S4
  are unfiled drafts. Its S10 draft names `route_reduce`, which #957 deleted. #1107 is open with no
  code.
- **Profiling.** `perf` cannot run on the bench host (`/proc/sys/kernel/perf_event_paranoid` is 4;
  recorded in `docs/handoffs/plumbing-floor-2026-09-26/PLAN.md:25` and
  `docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md:56`). The supported attribution instrument
  is `graph::test_only_phase_profile` (`crates/graph/src/lib.rs:35-206`): phases `ROUTE` (plain route
  units), `OUTPUT`, `IDENTITY_COPY`, `IDENTITY_ALIAS`, `OTHER_OP` and `BANK`, with a bank sub-phase
  profile that includes the route/master fold. `tools/console-workload/tests/gain_pan_profile.rs`
  (#960) drives it as an ignored, descriptive test.

### 3.2 Probe of today's behaviour

The original planner probe ran 33 Session V1 documents in a scratch copy of the repository through
four paths: host-core, the C ABI, native host-web and the wasm module under node/V8. VERIFY-1
re-ran the host-core and C ABI legs and reproduced every claim below. It did not re-run wasm or V8.
Cross-path equality was checked to printed decimals, not bit for bit.

- **Bare submixes work end to end.** Every legal shape parsed, compiled, prepared and rendered, and
  matched an `f64` oracle to `f32` rounding. The shapes covered:
  - nesting;
  - all seven taps as route sources;
  - fan-out;
  - a dangling submix;
  - a compressor insert keyed from a submix.
- **Refusals.** There are three:
  - `graph.cycle`, with a witness path;
  - `graph.output.cardinality`;
  - `graph.gain.non_finite` for -800 dB and +1000 dB.

  -1000 dB is accepted and is exactly zero gain.
- **PDC is exact across buses and sends.** With a limiter on one contributor, 486-sample
  compensation landed on exactly the early edges, and impulses aligned at sample 486.
- **Fold coverage.**
  - Eight tracks into one bus fold eight lanes.
  - Eight tracks routed post-pan both to the output and to a bus fold zero.
  - Bus-to-bus and bus-to-output routes stay separate route ops.
- **What is missing.**
  - **C ABI route edit.** A route-gain edit (`0505`) replaces the plan. The boundary block is all
    zeros, the next submit fails with `source.frame.noncontiguous` until a seek, and the gain lands as
    a hard step.
  - **Leaks.** Solo and mute leak through pre-fader sends.
  - **Submix strips.** Submixes cannot be metered, muted, soloed or faded.
  - **Dangling submixes.** A dangling submix is computed every block (read in the code, not
    measured).

The probe sources stay outside the repository: `scratchpad/gen_probe.py`, `probe-fixtures/` and
`probe-engine/`.

---

## 4. Research: consoles and DAWs

Unchanged from the original brief, and accepted by VERIFY-1 ("these match consoles and DAWs, with
the noted caveats"). The repository corpus (`dsp-research/console-daw-architecture.md`) already
compares DiGiCo, SSL, Lawo, Avid and Logic at the level of taps, typed acyclic routing and PDC.

Sources (retrieved 2026-10-02):

- SSL Live online help (S-*);
- DiGiCo SD & Quantum Software Reference, Issue E V1528 (D-SD);
- Avid VENUE S6L System Guide 8.2 (S6L);
- Pro Tools Reference Guide 2025.12.1 (PT);
- Logic Pro 12.3 (L-*);
- REAPER User Guide 7.81 (R).

Page numbers are the printed ones. Each claim is a paraphrase; "[unverified]" marks what the fetched
pages did not state.

### 4.1 Per product

| Topic | SSL Live | DiGiCo SD/Quantum | Avid S6L | Pro Tools | Logic Pro | REAPER |
|---|---|---|---|---|---|---|
| Bus processing | Stems, auxes and masters are "paths"; a Full path has filters, EQ, gate, comp, delay (S-PathProc) | Aux, group and matrix outputs have trim, polarity, delay, HPF/LPF, EQ, dynamics, inserts (D-SD p.97, p.12) | Outputs have HPF/LPF, EQ, comp, GEQ; groups have delay (pp.196, 202, 473) | Aux Input = audio track whose input is a bus (pp.1408-1410) | Aux strips for subgroups and returns (L-aux) | Every track is the same kind; folders are submixes (pp.98-99) |
| Stereo bus | Mono/stereo paths; linking not documented | Stereo bus = linked mono legs with per-leg sends (pp.39-41, 59); compressor stereo link (p.53) | Two mono strips combine into a linked stereo strip (pp.202, 296) | Two pan controls on a stereo output (p.1444) | Balance or stereo pan | Track pan and width |
| Send taps | Pre-fader, post-trim, post-insert A/B, post-fader, post-all (S-BusRouting) | Pre, post, pre-mute; Quantum six points (p.31) | Pre point chosen per bus (pp.226-227) | Pre or post fader (p.1428) | Pre fader, post fader, post pan (L-sends) | Three pre/post modes (p.339) |
| Send on/off | Level and in/out per send; "Follow Mute" on by default (S-BusRouting) | Level and on/off per send (pp.41-42) | Pan follows channel or own (p.227) | Level, mute, pan per send (p.1428) | Independent pan; send mute [unverified] | Volume, pan, mute envelopes (p.~360) |
| VCA | Offset on members' output and send levels; implicit mute (S-BusRouting, S-Mute) | Control groups pass dB; mutes in series (pp.130-133) | No audio; own + all VCAs, capped +12 dB; implicit mute (pp.220-222) | No audio; nested; sum; implicit mute (pp.1414-1416) | Members' volume, so post-fader sends follow (L-VCA) | Adds dB to followers; chaining (pp.107-108) |
| Bus to bus | Fixed hierarchy | Group to group and to aux (pp.41, 98) | Any to any; loops auto-muted (p.238) | Any output or send to a bus (p.89) | Aux to aux | Feedback opt-in (p.80) |
| Delay compensation | [unverified] | [unverified] | Off / mix / mix and inserts (pp.471-474) | Covers "bussing and sends" (p.1457) | "All" covers aux and sidechains | Per chain or plug-in (p.118) |
| Solo with buses | SIP Isolate (S-Solo) | Auto-solo for returns (pp.19-20) | Solo-safe (p.267) | New aux inputs solo-safe (p.367) | Solo-safe by control-click | Per-track solo modes |

### 4.2 Common patterns, conflicts and the engine's choice

| Pattern or conflict | Engine choice | Reason |
|---|---|---|
| Bus strips are ordinary strips (all six) | **Adopted**, full parity (2.2b) | One kernel path; slice 03's gate proves a bus equals a track fed the same sum, bit for bit. |
| Stereo bus construction differs | **Adopted**: two independent lanes plus a declared `link_mode` | `AGENTS.md`'s dual-mono law. |
| Tap set size differs (2 to 6) | **Kept**: seven taps on every strip, per route | A superset; no hidden per-bus pickoff. |
| Send on/off universal | **Adopted**: `mute` per route (P3) | Unmute is value-only. |
| Send relation to mute differs | **Adopted**: `follows_mute` (P11), SDK default on, as SSL | Closes the pre-fader leak at zero render cost. |
| Send pan follow vs independent | **Independent 2x2 per route**; a `post_pan` tap follows by construction | An agent can write either today. |
| Returns: no special type anywhere | **Adopted**: a return is a submix whose inserts hold the effect | One fewer node. |
| VCA = summed dB offset, implicit mute | **Adopted** (2.2a), separate umbrella | Zero render code. |
| Loops: auto-mute vs refuse | **Refuse at compile** (`graph.cycle`) | Transactional; a silent auto-mute would hide an agent's mistake. |
| Delay compensation per strip vs per edge | **Per edge, exact longest path** (unchanged `pdc.rs`) | Aligns cases per-strip delay cannot. |
| Solo with buses | **Buses solo-safe** (P7); implied solo is a follow-up | Keeps a soloed source audible through its bus and return. |
| Every product caps counts | **No cap**; configured resources (P12) | `AGENTS.md`. |

---

## 5. Design

### 5.1 Vocabulary

- **Strip**: the per-channel chain after the input: the input section, `console.pre_insert`, inserts,
  `console.post_insert`, fader/mute and pan/matrix. Tracks and submixes are strips.
- **Submix** (bus): a strip fed by the D9 sum of the routes that target it.
- **Route**: a typed edge from a strip tap to a submix input or the output, with a 2x2, a gain, a
  `mute` and a `follows_mute` (which only a route into a submix may set).
  - A **send** is a route to a submix.
  - A **return** is a submix with an effect insert.
  - A **main route** is a route to the output.
- **VCA**: a control-only group (2.2a; separate umbrella).

### 5.2 The submix strip in the session

```json
"submixes": [
  { "id": "drums",
    "builtins": { "left":  { "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 0.0,
                             "lpf_hz": 0.0, "delay_samples": 0 },
                  "right": { "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 0.0,
                             "lpf_hz": 0.0, "delay_samples": 0 } },
    "console": [ { "slot": "eq", "bypass": false, "params": [ ... ] }, ... ],
    "inserts": { "effects": [ { "id": "glue", "identity": { "kind": "native",
                 "effect_id": "miso.compressor" }, "link_mode": "maximum", ... } ] },
    "fader": { "left_db": -3.0, "right_db": -3.0, "left_mute": false, "right_mute": false },
    "pan": { "left": -1.0, "right": 1.0, "smoothing_samples": 0 } }
],
"routes": [
  { "id": "kick-to-drums", "source": { "kind": "track", "track_id": "kick", "tap": "post_pan" },
    "destination": { "kind": "submix_input", "submix_id": "drums" },
    "channel_matrix": { "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 }, "gain_db": 0.0,
    "mute": false, "follows_mute": true },
  { "id": "drums-to-verb", "source": { "kind": "submix", "submix_id": "drums", "tap": "pre_fader" },
    "destination": { "kind": "submix_input", "submix_id": "verb" }, "...": "...",
    "mute": false, "follows_mute": true },
  { "id": "drums-main", "source": { "kind": "submix", "submix_id": "drums", "tap": "post_pan" },
    "destination": { "kind": "output_input", "output_id": "main-out" }, "...": "...",
    "mute": false, "follows_mute": false }
]
```

The `builtins`, `console`, `inserts`, `fader` and `pan`/`matrix` spellings are the track's,
verbatim; the snippet abbreviates.

- **Canonical order** of a submix: `id, builtins, console, inserts, fader, pan|matrix`. That is the
  track's order minus its three source fields.
- **Field IDs** (the submix message, appended):
  - `id` 1 (exists);
  - `builtins` 2;
  - `console` 3 (repeated entry);
  - `inserts` 4;
  - `fader` 5;
  - pan-or-matrix 6 (tagged as the track's field 10: pan 1, matrix 2).
- **Route fields:** `mute` 6 and `follows_mute` 7, both required on every route; `follows_mute` may be
  `true` only on a route into a submix (P11).
- **Route source:** tag 2 keeps its code with the spelling `submix`, and its `tap` (field 3) is
  required.
- **Validation:** every rule a track's strip obeys applies to a submix. Paths are by index, as
  session validation reports them: `$.submixes[<i>].fader.left_db`, `$.submixes[<i>].console[<j>]`.
  An automation target's `entity_id` may name a submix (still inert until #1058).
- **Graph roles:** a submix may be:
  - a route source (any tap);
  - a route destination;
  - a sidechain source (any tap; sidechains reuse `RouteSource`).

### 5.3 Lowering, levels and banking

**Lowering (P1).** For every strip in canonical order (tracks, then submixes), the track's chain of
`TrackStage` nodes and racks is built, keyed by the strip ID.

- A track's `Input` stage has a source binding. A submix's `Input` has none: its inputs are the
  `RouteDestination` edges that name it, so it is a D9 reduction (fan-in 0 is a `+0.0` fill).
- A nonzero submix `delay_samples` delays the **sum**, after the reduction. That is a new runtime arm
  (slice 04): the track's `TrackDelay` arm is keyed by source inputs and returns before any
  reduction (`runtime.rs:2980-2989`). Between slices 03 and 04, inside batch K1 only, a submix's
  `delay_samples` is not lowered.
- A route source `{ submix, tap }` maps to that strip's tap stage through `stage(SendTap)`
  (`graph-compiler/src/ids.rs:199-209`).

**Levels.** Levels are longest-path (`graph-compiler/src/schedule.rs:9-81`), so a bus's stages sit
after every contributor, and a nested bus after its parent.

**Banking.** The rules are unchanged; buses are new members.

- Console slots bank per (slot, pool class, level), padded (decision 12). Bus lanes join whichever
  group their level and class select.
- The builtins input, fader and matrix banks take bus lanes the same way.
- Bus inserts bank opportunistically.
- A bus is pool class `Stereo`, and it is never eligible for collapse (2.2b).

**Cost consequence.** Buses at different levels pad separately. Level alignment is deferred (section
10, O5) until BM3 reports the padded-bank cost.

### 5.4 Sums, activity and signed zero (P4)

- A bus input and the output sum their inputs in D9 edge order.
  - The first input in edge order owns the store: its contribution if it is active, else `+0.0`.
  - Later inputs are added only when active.
  - Fan-in 0 is a `+0.0` fill.
  - With every input active this is exactly today's `reduce_plane`, so an active lone `-0.0` input
    keeps its sign.
- **Inactive** means: silenced by its `RouteGate` (muted, or follow-muted on both lanes), settled if
  live, **and undelayed**.
  - A delayed silenced route is active, with zero coefficients.
  - Its contribution is the delayed zero-coefficient mix, whose zeros may be signed.
  - The oracle in every gate implements exactly this.
- **In place.** A route that is the sole reader of its tap mixes in place and owns no buffer. When
  such a route is inactive and is its destination's only input, the destination does not alias the
  buffer (which holds the raw tap): it fills `+0.0`. Route destinations are never bank members, so
  `bank_gather_source` (`runtime.rs:2949`) never bypasses the reduction; slice 19 asserts it.
- **NaN through zero coefficients.** `0 * inf` is NaN. The `input` tap carries the raw source, so a
  delayed silenced route can carry a NaN into a bus sum where an undelayed one is skipped. The bus
  input section's D7 sanitizes it, but a meter at the bus's `input` boundary sees it. This is
  documented, not engineered around.
- **Activity is per block.** It is decided at the block's start (after the live drain, 5.7) and
  read by the destination later in the same block. It never changes mid-block.
- The bus's input section then runs its usual D7 sanitization. A session whose bus was bare and is
  now a transparent strip (P15) changes audio in two places only:
  - a `-0.0` normalized to `+0.0` by the identity input section (the disabled-SVF identity);
  - a non-finite or `>= 1e30` value sanitized to `+0.0`.

  The canonical graph text changes as well: `submix:` nodes become `track:<id>:<stage>` chains.

### 5.5 Latency and PDC

`pdc.rs::timings` (`graph-compiler/src/pdc.rs:38-161`) is already per edge, exact and longest-path.
Bus-strip effects are `Effect` nodes with their prepared latency, so:

- a bus's output arrives at `max(contributor arrivals) + bus strip latency`;
- every early edge into a sum gets an integer compensation delay, staged by the consumer (3.1), so
  the line holds post-route-mix samples;
- a bypassed latent slot keeps its latency (decision 12, L4);
- `delay_samples` contributes nothing.

**Latency accumulates with nesting.** A latent console slot (the limiter's `rate/100 + 6` samples)
is paid on every strip, so a session with a console limiter and bus depth `d` adds `(d + 1)`
lookaheads. Playback renders ahead, but live edits are heard `latency_samples` later, and that grows
with depth. This is owner question Q3 (8.2).

### 5.6 Sends

- A send is a route. There is no new entity and no per-strip send count.
- **Taps.** Seven on every strip.
  - Pre-fader taps read the un-gated signal: fader mute and solo apply at the fader.
  - `post_fader` and `post_pan` read the gated signal.
  - A `post_pan` send follows the strip's pan by construction.
- **Mute** (P3) and **follows_mute** (P11) are route fields, both required in the session. The SDK
  defaults `mute` to `false`. It defaults `followsMute` to `true` for a route into a submix, and
  always writes `false` for a route into the output, where `true` is refused.
- **Fan-out and nesting.** Any number of routes leave any tap. Bus to bus is allowed. Cycles refuse
  in the graph compiler, including cycles through a mid-chain tap.

### 5.7 Live sends

**Classification.** A change to the `gain_db`, `mute` or `channel_matrix` of a route **into a
submix** is value-only in a live-controlled plan. Everything else is structural (plan replacement):

- a change to its source, tap, destination or `follows_mute`;
- a change to the route set;
- any change to a route into the output.

**One coefficient function (P11), in two layers.**

```rust
// crates/graph: the pure layer the runtime's bind calls.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RouteGate { pub mute: bool, pub follow_zeroed: [bool; 2] }
impl RouteGate {
    pub const OPEN: Self = Self { mute: false, follow_zeroed: [false; 2] };
    /// Muted, or follow-zeroed on both lanes: the route contributes nothing.
    pub const fn silences(self) -> bool;
}
pub fn gated_route_coefficients(transform: &RouteTransform, gate: RouteGate) -> [f32; 4];

// crates/graph-compiler: the domain-checked layer the compiler and every live producer call.
pub fn route_coefficients(gain_db: f32, matrix: [f32; 4], mute: bool,
                          source_lane_muted: [bool; 2]) -> Result<[f32; 4], RouteValueError>
```

- `gated_route_coefficients` returns `[+0.0; 4]` when the gate silences. Otherwise it returns the
  folded `[gain * ll, gain * lr, gain * rl, gain * rr]` (today's `folded_route` bits), with the left
  source column (`ll`, `rl`) set to `+0.0` when `follow_zeroed[0]`, and the right column (`lr`, `rr`)
  when `follow_zeroed[1]`. It replaces the private `folded_route`: `node_kind` binds
  `gated_route_coefficients(&transform, gate)`, and `plain_route_gains` (the fold's source of
  constants) declines any route whose gate is not `OPEN`.
- `route_coefficients` runs today's `route_transform` checks, then `gated_route_coefficients` with
  `RouteGate { mute, follow_zeroed: source_lane_muted }`, and refuses (`Domain`) a folded coefficient
  that is not finite. The compiler's lowering and every live producer call it, so domain checks and
  bits cannot drift.
- `PreparedRoute` gains `gate: RouteGate`; the runtime binds `gated_route_coefficients(&transform,
  gate)`. The canonical text adds a `route-mute` row (slice 18b) and a `route-follow-zeroed` row
  (slice 20) only for a gate that sets them, so no existing digest moves.
- `source_lane_muted` is `[false; 2]` unless the route has `follows_mute`; then it is the source
  strip's effective `[left_mute, right_mute]`.

**Record.**

```rust
pub struct RouteControlRecord { target: [f32; 4], mute: bool, length: u32 }
```

- `mute` is true when the route's gate silences it (muted, or follow-muted on both lanes).
- `length` is the ramp in samples, at most `ROUTE_RAMP_LENGTH_MAXIMUM = 1 << 22`.
- `RouteControlRecord::new` refuses a longer length, and refuses `mute = true` with a target that is
  not all `+0.0`. The producer maps both to a typed error (VERIFY-2 MINOR 4).

**The indexed ramp law (P5).** Per live route, the render plane holds `start[4]`, `step[4]`,
`target[4]`, `length`, `position` and `mute`.

- Coefficient at ramp index `k` (exact in `f32`, because `k <= 2^22`):
  - `c(0) = start`;
  - `c(k) = round(round(k * step) + start)` for `1 <= k < length`, evaluated as `Lane::fma(k, step, start)`
    (two roundings on every target);
  - `c(k) = target` for `k >= length` (assigned, never computed).
- `step = round(round(target - start) / length)` is computed once per record when `length > 0`.
  With `length == 0`, `c(k) = target` for every `k`. If `round(target - start)` is not finite for any
  coefficient, the record applies with `length = 0` (a step), so no step is ever NaN.
- `current(position) = c(position)`. This is exact, so the oracle reproduces a mid-ramp retarget.
- **At bind:** `start = target =` the prepared coefficients, `mute =` the prepared gate's
  `silences()`, `position = length = 0`. An attached, idle lane renders the prepared bits.
- **On a record:** `start = current(position)`, `target = record.target`, `mute = record.mute`,
  `length = record.length`, `step` as above, `position = 0`. Several records in one drain reduce to
  "last record wins, ramping from the pre-drain current".
- **Per block:** frame `f` (0-based) uses `k = position + f + 1`. After the block,
  `position = min(position + quantum, length)`. Position saturates, so it never wraps.
- **Activity (P4):** after the drain, at block start, the route is inactive for this block iff
  `mute && position >= length && edge undelayed`.
  - The block in which `k` reaches `length` is mixed whole; frames after the snap use exact `target`.
  - A `length == 0` mute is inactive in the very block whose drain applied it (`position == length == 0`).
  - An unmute record is active at once and ramps from the current coefficients.
- **Kernel.** `route_mix_ramp_block<L: Lane>` in `crates/lane/src/kernels.rs`, beside
  `mix2x2_block` (slice 21):
  - it applies the D3 mix with per-frame coefficients;
  - it is vectorised over frames with a frame-index vector at `L = FrameLane`;
  - it finishes with an `f32` tail outlined as a non-generic `#[inline(never)]` function, so that
    `check-web-audioworklet.sh`'s kernel-shape rule 3 holds (the #926 lesson);
  - the settled remainder is `mix2x2_block` unchanged.

  The law is width-independent by construction.
- **Name.** "Indexed ramp", documented in a "Live routes" section of
  `docs/BUILTINS_AND_METERING_V1.md`. It is distinct from master-plan D11
  (`current = select(done, target, current + step)`, `crates/lane/src/kernels/builtins.rs:185-202`),
  which stays the law for faders, matrices and effects. No pinned contract ties route ramps to D11.

**PDC-delayed sends (VERIFY-1 BLOCKER-1).** The compensation line is the consumer's and holds
post-mix samples (3.1). Under P4 a delayed route is never inactive:

- silenced, its route op keeps mixing its zero target into its own buffer, and the consumer keeps
  staging it;
- the fade therefore arrives at the bus whole, `d` samples later, aligned with the other
  contributors;
- after the fade the line holds only zero-coefficient output;
- on unmute no stale sample can reach the bus.

The cost is that a muted delayed send does the work it does today. Skipping it after its line drains
(deferred deactivation) is deferred item O6 (section 10), to be taken only on a measurement.

**Drain.** At its op's start, every block, a live route pops at most `available_at_entry()` records
and applies each in turn, **including while inactive**, so an unmute is seen. Nothing is dropped; a
record that arrives later is applied in a later block.

**Graph shape.**

- A live route is `NodeKind::LiveRoute(Box<LiveRoute>)`, with a `GraphRouteControlBinding`
  (consumer, route node) carried to the plan the way `GraphEffectControlBinding` is, through a
  method on `PreparedGraphPlan` modelled on `with_builtin_banks` (`crates/graph/src/lib.rs:1278`); no
  field is added to `PreparedGraphPlanParts` or to `GraphBuiltinsCompileRequest` (38 literals).
- It is not a `GraphNodeBinding`: a processor binding would make it `Bound(processor)` and skip the
  route mix (VERIFY-1 MINOR-11).
- A live route never folds. `PlanningMetadata` gains `has_route_control`, and `plain_route_gains`
  declines a node that has one (VERIFY-2 M3): a bus `Input` can be a fold master, and a folded live
  route would never drain. The single-master fold keeps working for routes into the output, which
  stay prepared constants.

**Which routes are live.** In a plan prepared with live controls, every route whose destination is
a **submix** gets a control lane; routes into the **output** stay prepared constants. A session never
states liveness.

- Live controls are **opt-in** (3.1). This applies to the producer's mixer, to the V8 benchmark and,
  after #1053 and slice 27, to the C ABI. A fan's control-free playback keeps constants everywhere.
- Consoles give master assignments only an on/off (SSL: sends to masters have an in/out switch and no
  level), and strip faders do that job live.
- Live output routes are deferred item O9.

**The acked-batch question.** Admission is all or nothing:

1. decode and domain-check every record (`route_coefficients`, length bound);
2. check room in every destination queue;
3. push;
4. commit the mirrors or ack.

A full queue is typed `Backpressure` before any push. On the C ABI the commit after the first push is
infallible (#1053 A1.4). The render drain never drops. So no ack precedes a drop.

**Snapshots.** On the C ABI, value-only route edits update the committed model and revision, like
#1053's fader edits. In the browser, live moves do not write back to the model
(`docs/BUILTINS_AND_METERING_V1.md:153-166`, ruling D1). That ruling is in tension with the
owner's "the engine owns the edited session" (#1057, open); routes follow whatever #1057 settles.

**Resources.** Every byte a live route adds is charged: the activity table and the per-input route
indices (slice 19, in the graph-compiler estimate), the boxed `LiveRoute` owners and their queues
(slices 22 and 23, on the #1100 precedent of charging every live-owner byte,
`crates/graph-compiler/src/estimate.rs:154-172`). host-core reports them in a
`route_control_resources` row and checks them against `maximum_graph_session_plus_plan_bytes`
(`host.graph.resource.limit`, `crates/host-core/src/prepare.rs:1088`).

### 5.8 Metering and the master strip

- Meters attach to any of a bus strip's seven boundaries through `HostMeterRequest`, whose
  `track_id` becomes `strip_id`. The boundaries are `TrackStage` nodes, so the graph's refusal
  (`graph/src/lib.rs:1591-1596`) does not bite.
- `host.meter.order` (`host-core/src/prepare.rs:1177-1220`) refuses in two cases only: a bound meter
  names an ID outside the canonical strip order, or a selected meter comes back at a different position
  than requested. Default meters are sorted into canonical order. An unknown ID is refused earlier by the
  builtins compiler (`builtin.meter.unknown_track`, `builtins-compiler/src/lib.rs:3327-3336`).
- **Host-web frame** (P10): `3 * (T + S) + 3` words.
  - Peaks: tracks, then submixes, then master L and R.
  - Gain reduction: tracks, then submixes, then the master's.
  - `WebMeterHeader` appends `submix_count` (72 bytes).
  - The `miso.meter.v1` message keeps its fields and appends `submixCount`, `submixPeaks` and
    `submixGrDb`.
- **The master designation** is a strip index: host-core's `HostLiveControlRequest.master_track` and
  the browser boot word `live_control_master_track_plus_one` keep their spellings (P17) and accept a
  strip index (tracks first, then submixes).
  - A mix bus with a limiter can then be the master whose gain reduction the frame reports. In the
    browser that is reachable once slice 16 lets observation commands address a bus.
  - It is still a designation by index. It widens the "designation, not discovery" stopgap
    (`host-core/src/prepare.rs:305-309`); it does not retire it (VERIFY-1 MINOR-12).
- The C ABI keeps its single output peak. Per-strip C ABI meters stay out of scope, as for tracks.

### 5.9 Solo with buses

- Solo-in-place composes over the per-strip mute state (P7).
- Submix entries are solo-safe: never soloable and never solo-muted.
- Soloing a submix, implied upstream solo and a `solo_safe` flag on tracks are follow-ups.
- Pre-fader sends of solo-muted tracks leak unless the send has `follows_mute`, which the SDK sets by
  default on every route into a submix. It is prepared in slice 20 and live in slice 26 (browser) and
  slice 28 (C ABI).

### 5.10 VCA groups

These live in the separate umbrella (`issues/V0`-`V5`), filed when batch K3 closes.

- **Schema:** the root key `vcas` takes the next unallocated root field ID at implementation. 16 is
  free today: root IDs are 1-7 and 9-15, with 8 an unrecorded gap from the removed `limits` key that
  must never be reused.
- **Opcodes:** `0700`-`0702`.
- **Preparation:** one session helper computes each strip's effective fader and mute. It feeds the
  members' fader sections (builtins compiler) **and** the prepared `source_lane_muted` of every
  `follows_mute` route a member is the source of (graph compiler), so a fresh plan of a VCA-muted
  member's send equals the live path (VERIFY-2 M12). `vca_mute` seeds the strip-mute state.
- **Live:** member `FaderDb` and `Mute` records are emitted through that state, net and never
  redundant. An out-of-range VCA index refuses with a new browser reason, 14 `unknownVca`, on
  slice 24's `unknownRoute` precedent.
- **C ABI:** after #1053 and slice 28, with `maximum_vcas` in `reserved[1]`.

### 5.11 Wire identity, diagnostics and metadata

All changes are in-place V1 amendments on the #1063 and decision-12 precedent:

- nothing is renumbered;
- a retired code or spelling is refused and never reallocated;
- an appended code takes the next unallocated value;
- there is no `ABI_VERSION` bump.

| Surface | Change | Slice |
|---|---|---|
| Session submix fields | `builtins` 2, `console` 3, `inserts` 4, `fader` 5, pan-or-matrix 6 | 02, 05 |
| Route source | Spelling `submix` for tag 2; `tap` required; `submix_output` refused (also as a sidechain source) | 06 |
| Route | `mute` 6 (slice 18b), `follows_mute` 7 (slice 20), both required, each migrated in its slice. `follows_mute: true` on a route into the output refuses with `schema.invalid_enum` at `$.routes[<i>].follows_mute`. | 18b, 20 |
| Session-edit opcodes | `0203`-`0211` address a strip ID (07). New: `0506` set route mute (18b), `0507` set route follows-mute (20). Count 41 → 42 → 43. | 07, 18b, 20 |
| Protocol pins | The opcode count is pinned at `crates/protocol/src/model.rs:1315`, `controller/tests.rs:326`, `conformance/src/protocol_corpus.rs:664`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`, `docs/CONTROL_PROTOCOL_REGISTRY.md:74` and `fuzz/corpus/complete-schema-manifest.md:4`, `:19`. `COMPLETE_SCHEMA_HASH` is at `protocol_corpus.rs:666`, `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and `fuzz/corpus/complete-schema-manifest.md:10`. **Any submix or route wire change re-pins the hash**, even with no new opcode, because the corpus encodes `UpsertSubmix`, `UpsertRoute` and `SetRouteSource`. | 02, 05, 06, 18b, 20 |
| Graph canonical text | `route-mute` (18b) and `route-follow-zeroed` (20) rows, written only when set | 18b, 20 |
| Browser command kinds | 13 `routeGainDb`, 14 `routeMute`, 15 `routeMatrix` (24). The kind-vocabulary self-test mutations re-anchor, and the pre-existing `COMMAND_SOLO_MODE = 12` drift mutation, which already collides with `INPUT_FILTERS = 12`, moves to 16 (24). | 24 |
| Browser command reasons | 12 `notSoloable` (15) and 13 `unknownRoute` (24); the VCA umbrella appends 14 `unknownVca` (V3). The `COMMAND_REASON_FUTURE_TAP` drift mutation moves to 13, then 14 (then 15 in V3). | 15, 24 |
| Browser exports | `…_live_control_submix_count`/`_submix_id` (13), `…_live_control_route_count`/`_route_id` (25) | 13, 25 |
| Boot word and header | Spellings unchanged (P17). The boot word and the header's `master_track_plus_one` carry a strip index plus one; the docs say so (16). | 16 |
| Meter frame | `3(T + S) + 3` words; `WebMeterHeader` appends `submix_count` (72 bytes); the `miso.meter.v1` message appends `submixCount`, `submixPeaks`, `submixGrDb` | 12 |
| Session map | `miso.sessionmap.v1` and `SessionMap` gain `submixes` (13) and `routes` (25) | 13, 25 |
| C ABI | `compile_limits.reserved[0]` becomes `maximum_submixes` (0 = `maximum_tracks`) at offset 176, `reserved[3]` at 184; size 208 unchanged; no symbol change | 09 |
| Diagnostics | Existing codes reused at new paths: `reference.missing_entity`, `id.duplicate`, `console.entry_*`, `graph.cycle`, `schema.invalid_enum`, `host.observation.master_track`. No temporary refusal codes. | 02-07, 20 |

---

## 6. SIMD and performance

### 6.1 What is already vector

Track and bus strips run in banks: lanes are strips, and vectors span the bank (4 lanes on simd128
and NEON, 8 on AVX2), with a scalar tail for every count. Routes and reductions are vectorised over
**frames** (`FrameLane = lane::Native`, `runtime.rs:316`). Neither changes shape here.

### 6.2 Tier 0: correctness (slices 02-26)

Routes stay route ops: `mix2x2_block` from the tap buffer into a route buffer (or in place), then
the destination's D9 reduction.

- The single-master fold keeps working where its proof holds.
- Inactive routes (P4) are skipped: neither mixed nor loaded. That is part of mute's definition, not
  an optimisation, and it is where the solo case gets its saving (a follow-muted send is inactive).
- The live ramp is the only new kernel. It is vectorised over frames (P5).

### 6.3 Measurement (successor issues BM1-BM3, outside the umbrella)

- **Rows.** A real bus-and-send session runs on the two benchmark paths R9 keeps
  (`docs/rulings/engine-footprint-2026-09-28.md:35`):
  - the native console `--step` rows, a static plan, which is the C ABI fan-playback shape (until
    #1053 and slice 27 attach live controls on the C ABI; BM3 states which shape the C ABI ran at
    measurement time);
  - the V8 mixing benchmark on the shipped artifact, which boots through host-core **with** live
    controls (64 queue records), the producer-mixer shape.
- **Fixture.** The session is a committed, derived fixture held by `check-console-fixtures.sh`, on
  the #1085 precedent (BM1; BM2 adds it to the V8 benchmark as a third document).
- **Baseline** (BM3):
  - one invocation per path (one warmup, two measured rounds), recorded in
    `artifacts/steps/bus-send-base/`, after `preflight-console-benchmark.sh --step bus-send-base`;
  - plan facts read from the compiled plan by BM1's facts test: route ops per block, delayed route
    edges, folded routes;
  - one untimed phase profile of the native row with `graph::test_only_phase_profile`, driven as
    `tools/console-workload/tests/gain_pan_profile.rs` drives it: the `ROUTE` phase, the reduction
    units (`OUTPUT`, `OTHER_OP`, `IDENTITY_COPY`) and the bank route/master-fold sub-phase. It is
    descriptive and is never compared with the timed records.
- **No decision is taken inside this plan.** BM3 hands the numbers to the weekly performance pass.
  Its guidance, recorded with the numbers: route ops plus route-input reductions below about 5 % of
  the native row's profiled block time put fusion's best case inside the owner's 2 % allowance, where
  a new kernel family is "flimsy machinery"; at or above it, O1 is a candidate for a brief. The V8
  row is reported beside it; V8 has no equivalent profile.
- **History that O1 must cite.** #957 (`bf3bacab`) deleted the fused Output route-fold family
  (#926 fused fold, #937 grouped kernels, #927 in-place source read) because "only a plan with no
  bank at all reached it". The real host path O1 would serve now exists: every send from a mid-chain
  tap (pre-fader, post-fader, insert taps), and every live send, is an unfolded route op followed by
  a reduction on every host. That is why O1 is a measured candidate, not a revival. It must carry the
  family's lessons:
  - the non-generic `#[inline(never)]` `f32` tail (#926, `67649092`);
  - group-of-eight chaining (#937, `40c62101`).

### 6.4 What each rule owes

| Rule | How it is met |
|---|---|
| No unnecessary copies | No new block copy. A muted undelayed route skips its mix and load. Route-buffer removal is O1, gated on 6.3. |
| No scalar where vector is possible | The live ramp is vectorised over frames, with coefficients a pure function of `k`. Settled coefficients are broadcasts. |
| No target-specific code | One kernel, generic over `Lane`, instantiated at `FrameLane`, with an outlined `f32` tail. |
| Skip work on silence | Muted and follow-muted sends are inactive (no mix, no load). Empty sums store `+0.0`, and bus strips then see a silent block and take each effect's silent fast path. The data-driven half waits on the silence architecture (A0/S10; O2). |
| Benchmark real paths only | The rows are a real session on the two R9 paths; the V8 one boots through host-core with live controls. No diagnostic row is added. The benchmark work is a successor, so it never holds a product slice open. |
| Allocation-free render | Every new state is sized at preparation. The gates are `allocations == 0` on render after warm-up, measured with `bench_support::alloc`'s thread-scoped counters; each such gate states its test value (a count that *is* the claim still names the defect it catches). |

---

## 7. Gates every issue inherits

Run from the repository root. The commands are copied from `.github/workflows/qualification.yml` and
`scripts/`, and were checked on `fe8ac679`. `<A>` is the artifact directory and `<B>` the named-twin
directory that `build-web-audioworklet.sh --named-twin <B> <A>` writes.

- **Format, lint, docs:**
  - `cargo fmt --all -- --check`
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
  - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- **Policy:**
  - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
  - the policy script of every crate touched, with its `test-*` twin: `check-{session,host-core,
    protocol-control,realtime,lane,rack,builtins,graph,effect-runtime,bench}-policy.sh`. This is a list,
    not a command: a gate runs each `check-*`/`test-*` pair as a separate command.
- **Workspace tests** (CI's `test-debug-a`):

  ```
  cargo test --locked --workspace --all-targets \
    --exclude lane --exclude math --exclude effect-runtime --exclude delay \
    --exclude compressor --exclude multiband-compressor --exclude gate-expander \
    --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip \
    --exclude parametric-eq --exclude builtins --exclude dsp-reference \
    --exclude conformance --exclude audit --exclude bench --exclude console-workload \
    --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus \
    --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit
  ```
- **DSP crates and conformance** (CI's `test-debug-b`):

  ```
  cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor \
    -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip \
    -p parametric-eq -p builtins -p dsp-reference -p conformance \
    --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support
  ```
- **Tools that test-debug-a excludes** (`qualification.yml:710`; the only CI command that runs
  `tools/audit`'s tests, VERIFY-2 M9; `console-workload`'s tests also run in CI's `aarch64-release`
  job, `qualification.yml:981` and `scripts/run-aarch64-tests.sh:160`):
  `cargo test --locked --release -p audit -p bench -p console-workload`
- **4-lane (`Simd4`) coverage:**
  - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host. It refuses another host unless
    `CARGO_BUILD_TARGET=aarch64-*` is set. Otherwise use CI's `aarch64-debug` job at the batch push,
    and record "at batch push".
  - It covers 25 product crates, including `graph`, `graph-compiler`, `host-core`, `lane`,
    `builtins-compiler` and `capi`, but **not** `host-web`.
  - A test that iterates `Backend::Simd4` explicitly also runs 4-lane on x86 in `test-debug-a`/`-b`
    (`Backend::Simd4` is unconditional, `crates/lane/src/backend.rs:30-37`). A test on
    `Backend::current()`, such as host-core's `randomized.rs`, gets 4-lane only on arm64.
  - A console slot binds only at the build's own width: a compile at a foreign vector width may be
    refused with `console.slot.unbanked`, which `crates/graph-compiler/tests/bank_levels.rs:923-941`
    tolerates as `FOREIGN_CONSOLE_REFUSAL`. A test requires "no refusal" at `Backend::current()` and
    `Backend::Scalar` only, and at a foreign width accepts either a compile or exactly
    `FOREIGN_CONSOLE_REFUSAL` (`Backend::Scalar` binds no banks, so bank-group counts are checked at
    `Backend::current()` only).
    No test returns early on a width (`run-aarch64-tests.sh` refuses a silent skip).
  - host-web's 4-lane coverage is the shipped simd128 module, through `check-sdk-headless.sh` and
    `check-browser-expected-resources.py --artifacts`.
  - `scripts/run-wasm-gates.sh` runs only the frozen lane corpus (`tools/wasm-gate-corpus`). It
    never reaches graph, host-core or console code.
- **Wire:** `bash scripts/check-protocol-wasm-parity.sh`, and conformance as above.
- **Render artifacts:**
  - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
  - `./target/release/audit capi`
  - `bash scripts/trace-graph-audit.sh target/release/audit`
  - `bash scripts/check-graph-determinism.sh`: 100 fresh processes print the same `graph_fixture`
    output, written to `target/issue6/fresh-process-determinism.json`. A class-A claim diffs that file
    between the branch and its base.
  - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`: the checked-in graph
    fixtures under `fixtures/graph/` are the generated bytes (`--write` regenerates them for a listed
    re-pin).
  - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
  - `bash scripts/check-console-fixtures.sh target/release/session_validator`
- **Browser:**
  - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
    (the build refuses unless `<A>` and `<B>` exist, are empty and are not symlinks,
    `scripts/build-web-audioworklet.sh:55-64`; CI does the same, `qualification.yml:130-132`).
    Every gate below that names the build means this form.
  - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`.
    It also runs `check-parameter-metadata-v1.py` and `check-abi-layout-v1.py` on the artifact's
    JSON documents.
  - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (the shipped module,
    through `hosts/host-web/tests/browser-v1/direct-oracle.mjs`)
  - `bash scripts/test-web-audioworklet.sh`. This is **hermetic**: vocabulary, layout and drift gates
    over a stubbed module. It never renders the shipped wasm. It runs each vocabulary script with
    `--self-test` and bare.
  - Standalone forms (each needs its argument; a bare call of the first two exits 2):
    - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
      `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`;
    - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` and
      `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`;
    - `python3 -B scripts/check-command-kind-vocabulary.py --self-test` and
      `python3 -B scripts/check-command-kind-vocabulary.py`;
    - `python3 -B scripts/check-command-reason-vocabulary.py --self-test` and
      `python3 -B scripts/check-command-reason-vocabulary.py`;
    - `python3 -B scripts/check-session-map-shape.py --self-test` and
      `python3 -B scripts/check-session-map-shape.py`.
- **SDK** (`npm ci` in `sdk/` first):
  - `bash scripts/check-sdk-generated.sh <A>`
  - `bash scripts/check-sdk-types.sh`
  - `bash scripts/check-sdk-headless.sh <A>`, which runs every `sdk/test/*-evals.mjs` against the
    shipped module and is the real-wasm behaviour gate
  - `bash scripts/sdk-package.sh check <A>`, which also runs `sdk/test/enginectl-cli.mjs`
- **C ABI:** `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
  (`qualification.yml:815-816`).

**How gates compare against a "fresh plan".** A live edit's result is compared with a plan freshly
prepared from the edited session and fed the same sources from sample 0, **only** where everything
downstream of the edited value is stateless:

- no console slots (decision 12 allows an empty console);
- no stateful inserts on the destination strip;
- an identity input section on the destination strip (HPF and LPF off; VERIFY-2 MINOR 7);
- or the comparison point is the destination's `Input` meter.

Otherwise, gates use the host-web harness's paired-host comparison (`render_pair_and_compare`,
`hosts/host-web/src/tests.rs:6681`): a bus strip and a reference track receive the same command
sequence and the same summed input, and must render the same bits.

**Signals in live-route gates** (VERIFY-2 M13). The harness's `feed_and_render`
(`hosts/host-web/src/tests.rs:2667-2684`) feeds one constant to identical L and R planes of one
shared source, which hides column, lane, route-index and delay-alignment errors. Every gate that
claims to catch such an error feeds distinct, non-constant signals per track and per lane (distinct
source channels, a per-sample pattern), uses asymmetric matrices, and edits routes that reach
different buses.

Every new or rewritten test carries its one-sentence test-value answer in its issue, including an
`allocations == 0` gate. Digests and canonical graph text are re-pinned only where the issue lists
them, each with its reason.

---

## 8. Risks and owner questions

### 8.1 Risks

- **R1. The schema batch is large.** Slices 02-08 change the grammar, the wire, the lowering and the
  SDK. They follow the #1084 C3 precedent: one batch, pushed once, with the SDK out of step until
  slice 08. Each slice migrates the documents its own grammar change breaks.
- **R2. Latency grows with bus depth** (5.5; owner question Q3).
- **R3. Padded bus banks.** Buses at many levels each pad a bank per console slot. BM3 reports the
  cost, and level alignment is O5.
- **R4. Fixture churn, twice in K3.** Slices 18b and 20 each add a required route key.
  - That touches 21 route-declaring documents, the embedded writer-corpus document and the inline
    sessions (each slice lists them).
  - `fixtures/session/v1/canonical.json`'s SHA-256 chain re-pins: `prepare_256_tracks-{48000,96000}.toml:12`,
    `tools/audit/src/fixture_builtins.rs:5102`, `fixtures/builtins/v1/MANIFEST.tsv:10-11`, and the
    MANIFEST digest at `tools/audit/src/builtins_graph.rs:52` and `fixture_builtins.rs:5275`.
  - The graph fixtures generated from it (`fixtures/graph/v1/direct-route.*`, pinned in
    `fixtures/graph/MANIFEST.tsv`) re-pin as well.
  - The browser's staged document size (`hosts/host-web/tests/browser-v1/expected.json:57`) moves.
  - It is mechanical and diffable. The two slices sit in one batch, so `main` sees one migration.
- **R5. #1054's smoothing table** is written as an optional, camelCase root object
  (`controlSmoothing`, `muteMs`). That contradicts V1's "no optional fields"
  (`docs/SESSION_SCHEMA_V1.md:149`) and the snake_case schema. It claims no field ID. P6 depends on
  #1054 only through its table, so this plan carries per-change smoothing until #1054 resolves its own
  shape. The VCA umbrella takes the next free root ID at implementation (16 today).
- **R6. Strip indices grow under existing tracks.** P9 keeps track indices. An app that hard-codes
  `3T + 3` must move with slice 12; every in-repo spelling is listed there.
- **R7. Output routes are not live.** A main route's gain, mute or matrix change is still a plan
  replacement (a silent block on the C ABI). Strip faders do that job live (O9).
- **R8. Unacked automation.** `AUTOMATION_ENQUEUE` is admitted and never applied (open #349 IO-5).
  This design adds no automation path.
- **R9. A live-controlled plan loses folds on its sends.** Bound routes never fold, so after slice 23
  a producer-mixer plan cannot fold its track-to-bus routes. The standing console rows route only to
  the output and keep their folds; slice 23 gates that. BM3's V8 row shows the cost.
- **R10. Browser live moves vs the committed model** (5.7). This stays with #1057.
- **R11. The K1 interim** (P16). Between the K1 push and slice 10, a live-controlled host cannot
  move or observe a bus effect. Nothing could address one before K2 anyway; the interim only keeps
  boots from failing.

### 8.2 Owner questions

None of these blocks a filed slice. Each dependent piece is deferred or made independent of the
answer.

- **Q1. Per-strip console `link_mode`.** Should a submix be able to override a console slot's
  `link_mode` (for example, unlinked on tracks and linked on buses)?
  - In the compressor kernel, link is already a per-bank lane mask (`linked` and `averaged`,
    `crates/compressor/src/kernel.rs:330-339`; `dual_mono` only selects a cheaper path at `:624`).
    So a per-lane link would be a per-lane mask, **not** a bank split.
  - Its real costs:
    - a schema change to decision 12's slot declaration;
    - the slower linked path for the whole bank whenever any lane links;
    - the same generalisation in every effect that has a link mode.
  - Recommendation: **no**. Bus glue is a linked insert, and the console compressor is bypassed on
    buses (the hazard in 2.2b, carried by slice 08's guidance).
  - Nothing filed depends on the answer.
- **Q2. Route value domains.** Today a route's `gain_db` is accepted wherever its linear gain is a
  normal finite `f32`: roughly -758 to +770 dB, plus anything low enough to round to exactly 0
  (-1000 dB is accepted). Its matrix is checked for finiteness only. Should routes be bounded to the
  fader's `[-144, 24]` dB and the track matrix's `[-1, 1]`, refused with `numeric.out_of_schema_range`?
  - No checked-in route leaves those domains: every route gain is 0.0 and every route matrix is in
    range.
  - The live path does not depend on the answer: live and prepared domains are the same function
    (`route_coefficients`).
  - Only the published live-control metadata row for routes waits on it: deferred item O11.
  - Recommendation: **yes**, with one gain law across faders and sends.
- **Q3. Latency with bus depth.** Every strip pays every latent console slot, even when bypassed
  (decision 12, L4). With a console limiter, each bus level adds 486 samples at 48 kHz, so a live
  edit on a deep session is heard later. Is that accepted?
  - The alternative is a per-strip "slot absent" rule, which contradicts O1.
  - Recommendation: **accept**. Playback renders ahead.
  - Nothing filed depends on the answer.
- **Q4 (informational, decided).** The SDK builder defaults `followsMute` to `true` on every route
  into a submix (P11), as SSL does, so that muting or soloing silences a track's sends, pre-fader ones
  included. A monitor-style pre-fader send that must ignore the mute sets `followsMute: false`. A
  route into the output never follows. Say if you want the default reversed.
- **Q5 (later, only if measured).** If BM3 or a later fold slice finds that D9's route-ID order is
  what blocks folding, a class-B change of bus-sum order to a width-independent render order would
  need your summation-order procedure. Nothing is filed for it.

Deferred follow-ups (not questions):

- `follows_pan` for pre-pan sends;
- bus solo and implied upstream solo;
- VCA trim of sends;
- live output routes.

---

## 9. Issues, dependencies and order

Titles are the exact published titles that dependency lines name. Every slice body was written
against `fe8ac679`; each says which earlier slices moved its anchors.

### 9.1 Umbrella A: Submix strips and live aux sends

| # | File (`issues/`) | Title | Depends on |
|---|---|---|---|
| 00 | `00-record-the-submix-send-and-vca-ruling.md` | Record the submix, send and VCA ruling | owner acceptance of this design (open questions may stay open) |
| 01 | `01-iterate-session-strips-not-tracks.md` | Iterate session strips, not tracks, wherever strip semantics apply | 00 |
| 02 | `02-declare-the-submix-strip-in-the-session-grammar-and-wire.md` | Declare the submix strip in the session grammar and wire | 01 |
| 03 | `03-render-a-submix-strip-on-its-summed-input.md` | Render a submix strip on its summed input | 02 |
| 04 | `04-delay-a-submix-strips-summed-input.md` | Delay a submix strip's summed input | 03 |
| 05 | `05-carry-every-console-slot-on-every-submix-strip.md` | Carry every console slot on every submix strip | 04 |
| 06 | `06-tap-a-submix-strip-at-any-send-point.md` | Tap a submix strip at any of the seven send points | 05 |
| 07 | `07-address-submix-strips-in-session-edits.md` | Address submix strips in session edits | 06 |
| 08 | `08-build-submix-strips-in-the-sdk-and-teach-agents-to-author-them.md` | Build submix strips and bus taps in the SDK and teach agents to author them | 07 |
| 09 | `09-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md` | Count and cap submix strips in host preparation and the C ABI | 08 (K1 pushed) |
| 10 | `10-list-every-strip-in-the-live-control-handles-and-file-bus-effects-in-the-browser.md` | List every strip in the live-control handles and file bus effects in the browser | 09 |
| 11 | `11-meter-and-designate-submix-strips-in-host-core.md` | Meter any boundary of a submix strip and designate a master strip in host-core | 10 |
| 12 | `12-carry-submix-strips-in-the-browser-meter-frame.md` | Carry submix strips in the browser meter frame | 11 |
| 13 | `13-name-submix-strips-in-the-browser-session-map-and-sdk-measurement.md` | Name submix strips in the browser session map and the SDK measurement | 12 |
| 14 | `14-give-every-strip-one-mute-owner-and-live-control-producers-in-host-core.md` | Give every strip one mute owner and live-control producers in host-core | 10 |
| 15 | `15-add-the-not-soloable-command-reason.md` | Add the notSoloable command reason to every vocabulary spelling | 08 (K1 pushed) |
| 16 | `16-address-submix-strips-in-browser-live-commands.md` | Address submix strips in browser live commands | 12, 14, 15 |
| 17 | `17-drive-submix-strips-from-the-sdk-live-controls.md` | Drive submix strips from the SDK live controls | 13, 16 |
| 18a | `18a-gate-every-routes-coefficients-through-one-function.md` | Gate every route's coefficients through one function | 08 (K1 pushed); merges after 17 |
| 18b | `18b-mute-a-route-in-the-session.md` | Mute a route in the session | 18a |
| 19 | `19-skip-an-inactive-route-in-its-destinations-sum.md` | Skip an inactive route in its destination's sum | 18b |
| 20 | `20-let-a-route-into-a-submix-follow-its-source-strips-mute-in-the-session.md` | Let a route into a submix follow its source strip's mute in the session | 19 |
| 21 | `21-ramp-a-sends-coefficients-with-the-indexed-ramp-kernel.md` | Ramp a send's coefficients with the indexed ramp kernel | 08 (K1 pushed); may be built beside 18a-20 |
| 22 | `22-ramp-live-send-coefficients-on-the-render-plane.md` | Ramp live send coefficients on the render plane | 20, 21 |
| 23 | `23-produce-live-send-records-from-host-core.md` | Produce live send records from host-core | 22, 14 |
| 24 | `24-admit-live-send-commands-in-the-browser.md` | Admit live send commands in the browser | 23, 17 |
| 25 | `25-enumerate-sends-and-drive-them-from-the-sdk.md` | Enumerate sends and drive them from the SDK | 24 |
| 26 | `26-let-a-send-follow-its-source-strips-mute-live-in-the-browser.md` | Let a send follow its source strip's mute live in the browser | 25 |
| 27 | `27-deliver-value-only-send-and-submix-strip-edits-to-the-running-c-abi-plan.md` | Deliver value-only send and submix-strip edits to the running C ABI plan | 26, #1053 |
| 28 | `28-let-c-abi-sends-follow-their-source-strips-mute-live.md` | Let C ABI sends follow their source strip's mute live | 27 |

The umbrella closes on shipped product when 00-28 (18a and 18b included) have closed; 27 and 28 wait on #1053.

### 9.2 Successor performance issues (outside the umbrella)

Slice 00 files all three, as standalone issues that name their dependency. They are tooling and
evidence, not launch-critical features, and never hold the umbrella open.

| # | File | Title | Depends on |
|---|---|---|---|
| BM1 | `BM1-add-a-bus-and-send-row-to-the-native-console-benchmark.md` | Add a bus-and-send row to the native console benchmark | 26 (K3 pushed) |
| BM2 | `BM2-add-the-bus-and-send-session-to-the-browser-mixing-benchmark.md` | Add the bus-and-send session to the browser mixing benchmark | BM1 |
| BM3 | `BM3-record-the-bus-and-send-baseline-and-its-route-work-profile.md` | Record the bus-and-send baseline and its route-work profile | BM2 |

VERIFY-2 recommended one rows slice (its 22 and 23 merged). With its own M17 fixes the merged body
came to about a day of tooling work across two runners, so the rows are two slices again, each with
the `test-console-benchmark.sh` cases of its own runner (REVISION-2 section 5).

### 9.3 Umbrella B: VCA groups (filed when K3 closes)

| # | File | Title | Depends on |
|---|---|---|---|
| V0 | `V0-vca-groups-umbrella.md` | VCA groups | 26 (K3 closed and pushed) |
| V1 | `V1-declare-vca-groups-in-the-session.md` | Declare VCA groups in the session | V0 |
| V2 | `V2-apply-vca-offsets-and-mutes-at-preparation.md` | Apply VCA offsets and mutes at preparation | V1 |
| V3 | `V3-ride-vca-groups-live-in-the-browser.md` | Ride VCA groups live in the browser | V2 |
| V4 | `V4-enumerate-vca-groups-and-drive-them-from-the-sdk.md` | Enumerate VCA groups and drive them from the SDK | V3 |
| V5 | `V5-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md` | Deliver value-only VCA edits to the running C ABI plan | V4, 28, #1053 |

V1 is implementation-ready as written (anchors read on `fe8ac679`, with the K1-K3 changes to them
stated). V2-V5 are drafted with every VERIFY-2 gap closed; the root re-verifies their anchors when
it files the umbrella, because K1-K3 move them. The browser VCA slice is split as the route slices
are (admission in V3, enumeration and the SDK in V4).

### 9.4 DAG

```
00 -> 01 -> 02 -> 03 -> 04 -> 05 -> 06 -> 07 -> 08 ==K1 pushed==+
                                                                 |
  K2:  09 -> 10 -+-> 11 -> 12 -> 13 ------------------+-> 17 ----+
                 |              \                      |          |
                 +-> 14 ---------+-> 16 (also 15) -----+          |
       15 (needs only K1) ------/                                 |
                                                                  |
  K3:  18a -> 18b -> 19 -> 20 -+-> 22 -> 23 (also 14) -> 24 (also 17) -> 25 -> 26 ==K3 pushed==+
       21 (lane only) ---------+                                                               |
                                                                                               |
  after #1053:  27 (26, #1053) -> 28                                                           |
  successors:   BM1 (K3 pushed) -> BM2 -> BM3                                                  |
  VCA:          V0 -> V1 -> V2 -> V3 -> V4 -> V5 (also 28, #1053)  <---------------------------+
```

### 9.5 Batches (CI-conscious mode: one push per batch)

| Batch | Issues, in merge order | Why this boundary |
|---|---|---|
| K0 | 00, 01 | The ruling and a class-A refactor. Pushable alone. `AGENTS.md` routes CI `full` (`scripts/ci-path-router.py`), so K0 runs the Rust jobs once. |
| K1 | 02-08 | The submix-strip schema change, pushed once. The SDK is out of step from 02 to 08 (the #1084 C3 precedent). Live-controlled boots keep working through P16. |
| K2 | 09-17 | Host surfaces for submix strips. The browser artifact changes once. |
| K3 | 18a-26 | Route mute and follow-mute (two migrations, one push), the activity rule, live sends, browser sends, live follow-mute. |
| C1 (after #1053) | 27, 28 | The C ABI value-only path, then its follow-mute composition. |
| BM (after K3) | BM1, BM2, BM3 | Successor tooling and evidence; not launch-critical. |
| VCA (after K3) | V1-V4, then V5 after 28 and #1053 | One root-key migration, preparation, the browser and the SDK; then the C ABI. |

`AGENTS.md` allows one launch-critical implementation at a time, and K2 and K3 touch overlapping
host-web and host-core files. So the batches run in order.

- K3's slices 18a-22 may be drafted during K2 review, but are implemented on the K2 head. They do
  not touch only K3-private crates: 18b edits `host-core/tests/{randomized,collapse_arming}.rs`,
  which slice 14 also edits, plus host-web fixtures, tools and scripts (VERIFY-3 MINOR 9).
- C1 and the VCA batch both touch host-core. If #1053 closes while the VCA batch is in flight, C1
  waits for the VCA batch boundary, or the reverse; the root never runs both at once.

### 9.6 First product slice

The smallest audible product is **K1**: a drum bus with console EQ and compressor, a glue-compressor
insert and a fader, and a delay return fed by sends. It is authored from the SDK and rendered by the
browser and the C ABI. Live buses (K2) and live sends (K3) follow. VCA follows K3, and BM1-BM3
measure what shipped.

---

## 10. Deferred work (designed, not filed)

Each item names its trigger. None is filed by slice 00.

- **O1. Fuse each undelayed route's 2x2 into its destination's reduction.**
  - **Trigger:** the weekly performance pass, on BM3's numbers and 6.3's guidance.
  - **Design:** the destination's reduction applies each fused input's D3 mix inline, rounded before
    it meets the running sum, in D9 order (class A).
    - Eligibility: the route's only reader is that reduction, and its edge is undelayed.
    - A fused `LiveRoute` moves its lane to the reducer, with the same drain, indexed ramp and
      activity.
  - **The issue, when filed, must:**
    - cite #957 and the real host path (6.3);
    - outline its `f32` tail non-generically (`#[inline(never)]`) and chain groups of eight (#926,
      #937);
    - gate class A with host-core's randomized differential, run at 4-lane through
      `run-aarch64-tests.sh debug`;
    - count **retired route ops**, not arena buffers: an in-place route owns none;
    - report timing descriptively against BM3, with Sol judging any regression beyond the 2 %
      allowance.
- **O2. Skip sends from strips settled-muted without `follows_mute`, and data-driven silent inputs.**
  - **Trigger:** the owner's silence-architecture decision (A0) and S10, and #1072.
  - **Corrections it must carry:**
    - at `post_pan`, "silent-muted" requires the fader settled-muted **and** the matrix settled for
      the whole block, because `matrix2x2_ramp_block` has no identity select and a ramp through zero
      changes the zero's sign;
    - #940's mutation list is the original one: drop the fix-up; fill `+0.0` regardless; fix up
      whenever anything is skipped; **swap the two inputs within a pair**; start the first pair with
      `initial_store = false` (#940 spec, `git show 5b5299f6`). #940 was a brief and was never
      implemented;
    - the tail-shape rule is `check-web-audioworklet.sh` kernel-shape rule 3;
    - skip counts are derived from the compiled plan, never hard-coded;
    - #1072 moves the banked matrix's zero sign.
  - Most of the solo saving is already delivered by `follows_mute` (P11, SDK default on into a
    submix).
- **O3. Fold post-pan routes into every destination from the chain epilogue.**
  - **Trigger:** O1 landed, and a re-measurement shows the `post_pan` write plus the reducer's re-read
    is still at least 5 % of a real row.
  - Template: `FoldLane`/`fold_plane`/`fold_cohort`/`fold_resident_tiles` (`runtime.rs:1613-1946`),
    never the rack's aux seam (#210's PFL).
  - It splits into multi-destination folding with one route per lane, then fan-out.
- **O4. Carry live route lanes in the epilogue fold.** **Trigger:** O3 landed, and the V8
  live-controlled row shows the loss in R9.
- **O5. Align bus console banks by nesting depth.** **Trigger:** BM3 reports padded-bank cost above
  5 % of the bus row.
- **O6. Deferred deactivation of delayed muted sends** (VERIFY-1 BLOCKER-1 option b). A delayed send
  goes inactive after `d` more samples of zero-coefficient output have entered its line, and neither
  its op nor its staging runs while inactive. **Trigger:** a measurement shows muted delayed sends
  matter.
- **O7. Prune or skip a submix with no outgoing route and no meter** (VERIFY-1 MINOR-10).
  **Trigger:** the next silence slice, or a real session that has one.
- **O8. Fold a master that has an inactive contributor.** Today a muted main route declines the whole
  master's fold, which is correct and conservative. **Trigger:** a real session measured slower for
  it.
- **O9. Live output routes.** **Trigger:** owner product request; it needs O4 to keep the master fold
  in live plans.
- **O10. Class-B bus-sum order.** **Trigger:** Q5.
- **O11. Bound route values to the fader and matrix domains, and publish their live metadata**
  (the former slice 26; VERIFY-2 section 4). **Trigger:** the owner answers Q2 "yes".
  - Session validation refuses `numeric.out_of_schema_range` (through `validate_finite_range`,
    `crates/session/src/validate.rs:830-849`) for a route `gain_db` outside `[-144, 24]` at
    `$.routes[<i>].gain_db`, and a `channel_matrix` coefficient outside `[-1, 1]` at
    `$.routes[<i>].channel_matrix.<ll|lr|rl|rr>`. Non-finite values keep their code.
  - `graph_compiler::route_coefficients` refuses the same domains with `RouteValueError::Domain`, so a
    command-record value and a session value share one domain on every path.
  - Parameter metadata gains a top-level `routes` key with three rows (`gain_db`: dB `[-144, 24]`,
    block target, the fader's mapping; `mute`: boolean; `matrix`: four coefficients `[-1, 1]`,
    linear); the schema gate's key set widens by exactly that.
  - No migration: no checked-in document leaves the domains. The writer corpus's track matrix
    `ll = 1.25` is a **track** matrix and stays.
  - Gates when filed: boundary values (24.0, -144.0, ±1.0 accepted; 24.5, -144.5, 1.5 refused) on
    the session path and the live path; nothing checked in moves; `python3 -B
    scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json` and
    `cargo run --locked -p parameter-metadata -- --check <A>`.
