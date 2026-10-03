# Apply VCA offsets and mutes at preparation

Slice V3 of *VCA groups* (#1239). It is in the VCA batch, after *Declare VCA groups in the session*
(#1240). It makes a VCA audible on every host at preparation, the browser and the C ABI alike,
including a fan's control-free playback. Render code does not change: the effective values fold into
the fader sections, the follow-mute gates and the strip-mute state that already exist.

The design record cited below (`DESIGN`, `VERIFY-2`) is committed in
`docs/handoffs/submix-sends-2026-10-02/`. This slice closes VERIFY-2 M12: a prepared follow-mute
must include the VCA's mute.

## Product outcome

- Pulling VCA `drums` to -6 dB lowers every member lane's effective fader by 6 dB in a freshly
  prepared plan, and its post-fader sends drop with it.
- Muting the VCA mutes every member, tracks and submixes alike, and silences every `follows_mute`
  send from a member, while each member keeps its own mute in the session.
- In the browser, solo composes on top and never "un-mutes" a VCA-muted member, and a live send's
  mirror starts from the same effective mute the plan was prepared with.

## Context (verified on `8c6268967`)

- **After #1240:** `SessionModel.vcas: Vec<Vca { id, fader: DualMonoFader, members: Vec<StableId> }>`
  is parsed, validated (acyclic, references resolve, offsets in `[-144, 24]`) and canonical; a
  normalized model (`CompiledSession::normalized_model`) has `vcas` sorted by ID. Nothing reads it
  yet. `SessionModel::strips()` (`crates/session/src/model.rs:412-422`) is the strip order: tracks,
  then submixes, each in model order.
- **The fader preparation** (`crates/builtins-compiler/src/lib.rs`):
  - `strip_parameters` (`:4734-4776`) builds a strip's `BuiltinParameters` from the strip's own
    `fader.left_db`, `right_db`, `left_mute`, `right_mute` (`:4743-4752`).
  - It has three production callers: `expected_tails` (`:3129-3150`, the seal check at `:2090`),
    the domain preflight (`:3361-3370`, which reports `builtin.gain.domain` at the strip's own
    `fader.<lane>_db` through `gain_path`, `:4833-4847`) and the lowering loop (`:3412-3420`), all
    in `prepare_session_builtins_with_live_controls_and_policy` (`:3282`) or its seal; and one
    unit test (`:12214`).
  - The gain conversion is `db_gain` (`crates/builtins/src/lib.rs:5173`); the prepared range check
    is `checked_fader_gain` (`:4108`).
- **The follow-mute preparation** (*Let a route into a submix follow its source strip's mute in the
  session*, #1218): the graph compiler's route lowering builds a strip-ID -> `[left_mute, right_mute]`
  map from each strip's **own** session fader, only when some route follows
  (`crates/graph-compiler/src/compile.rs:323-336`), and sets each following route's
  `RouteGate.follow_zeroed` from it (`:347-361`). `gated_route_coefficients` zeroes the muted source
  columns, and a route silenced on both lanes is inactive when undelayed (#1217). The canonical
  graph text writes a `route-follow-zeroed` row for a gate with a zeroed lane.
- **The per-strip mute state** (`crates/host-core/src/solo.rs`):
  - `StripMuteSeed { mutes: [bool; 2], solo_safe: bool }` (`:99-108`, derives `Default`);
    `LiveControlSoloState::try_new(&[StripMuteSeed])` (`:142-162`) seeds `user_mute` **and**
    `emitted` from `mutes`.
  - `effective_mute(strip, lane)` (`:216-222`) is `user_mute || (any_solo && !solo_safe && !solo)`,
    the one composition (DESIGN P7); `strip_delta` (`:284-301`) emits only lanes whose effective mute
    differs from `emitted` ("never a redundant record", `:46-60`).
  - `StripMuteSeed` literals: `hosts/host-web/src/lib.rs:6309`, `:6320`, and the module's tests
    (`solo.rs:357`, `:510-517`).
- **The browser seeding** (`hosts/host-web/src/lib.rs`): one `StripMuteSeed` per strip from the
  session's own fader mutes (`:6282-6324`), then `LiveControlSoloState::try_new` (`:6404`), then
  `LiveRouteState::try_new(model, &|strip, lane| solo.effective_mute(strip, lane))` (`:6411`), which
  seeds each following live send's `source_lane_muted` (`crates/host-core/src/live_route_state.rs:97-133`).
  A later live send edit (kinds 13-15) builds its record from that mirror.
- **#1053** (*Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes*) has **not landed**: `git grep -n live_builtin_delta` is empty at
  `8c6268967`. The filing commit of #1239 amended #1053's spec (A2 D1, "Coordination"), #1225's D1
  structural list and #1226's D5 to carry the widened VCA guard: a C ABI delta is structural while
  the pre- or post-commit model declares a VCA, until #1247. It is needed because #1053 would push a
  member's own fader value, and *Deliver value-only send and submix-strip edits to the running C ABI
  plan* (#1225) and *Let C ABI sends follow their source strip's mute live* (#1226) read the
  source's raw committed mutes.
- **Harnesses.** host-core render tests prepare through `prepare_host_session`, the path the C ABI
  and the browser share (pattern: `crates/host-core/tests/submix_strip.rs`, `route_mute.rs`). The
  host-web native harness is `hosts/host-web/src/tests.rs`, with `render_pair_and_compare`
  (`:7280`), the solo helpers `stage_solo` (`:7234`) and `stage_lane_mute` (`:11870`), the per-strip
  distinct feeder `submit_strip_source` (`:9232`, `strip_planes` `:9222`), and #1224's follow fixture
  (`SOLO_FOLLOW_TRACKS`, `:11915-11930`). `feed_and_render` (`:2675`) feeds one constant to both
  planes of one source and must not carry a lane or member claim (DESIGN section 7, VERIFY-2 M13).

## Decisions frozen for this slice

- **D1. One composition, in `crates/session`** (a new module, re-exported):

  ```rust
  /// decision 13 (a): `clamp(member_db + sum(offsets), -144, 24)`, summed in f64 in the order
  /// given (the member's own value first, then each offset), clamped in f64, rounded once to f32.
  /// With no offset it returns `member_db` bit for bit.
  pub fn vca_effective_db(member_db: f32, offsets_db: impl IntoIterator<Item = f32>) -> f32;

  #[derive(Clone, Copy, Debug, PartialEq)]
  pub struct EffectiveStripFader { pub db: [f32; 2], pub mute: [bool; 2], pub vca_mute: [bool; 2] }

  impl SessionModel {
      /// Per strip, in `strips()` order: the indices into `self.vcas` of every VCA from which the
      /// strip is reachable through membership (directly or through nested VCAs), each once,
      /// sorted by VCA ID. Terminates on any model (a visited set), cyclic or not.
      pub fn vca_reach(&self) -> Vec<Vec<usize>>;
      /// Per strip, in `strips()` order: `db[l] = vca_effective_db(own_db[l], reach offsets[l] in
      /// ascending VCA-ID order)`, `vca_mute[l] = any(reach v.l_mute)`,
      /// `mute[l] = own_mute[l] || vca_mute[l]`.
      pub fn effective_strip_faders(&self) -> Vec<EffectiveStripFader>;
  }
  ```

  These are the only places reach and the effective values are computed. They run at preparation,
  off the render thread. #1244 reuses `vca_effective_db` and `vca_reach` for live moves.
- **D2. Two consumers, one computation** (VERIFY-2 M12).
  - **Builtins compiler.** `strip_parameters` takes the lane dB and mute it bakes as arguments.
    `expected_tails` and the lowering loop pass `effective_strip_faders()` of the normalized model
    (strip `i` is entry `i`). The domain preflight keeps passing the strip's **own** values, so an
    out-of-domain member fader still refuses with `builtin.gain.domain` at its own
    `fader.<lane>_db` path, and the clamp never hides it. The unit test at `:12214` passes the own
    values.
  - **Graph compiler.** The follow map (`compile.rs:323-336`) takes each strip's effective `mute`
    from `effective_strip_faders()` instead of its own fader mutes, still only when some route
    follows. So a VCA-muted member's follow send is zeroed (and inactive when undelayed and muted on
    both lanes) in a fresh plan, and its canonical text carries the `route-follow-zeroed` row.
- **D3. The mute state.**
  - `StripMuteSeed` gains `vca_mute: [bool; 2]` (default `[false; 2]`), and `LiveControlSoloState`
    keeps it per strip. `try_new` seeds `user_mute = mutes` (the member's own intent) and
    `emitted[l] = mutes[l] || vca_mute[l]` (what the prepared fader bakes), so seeding emits nothing
    and the first solo toggle emits no redundant record for a VCA-muted strip.
  - `effective_mute(strip, lane)` becomes
    `user_mute || vca_mute || (any_solo && !solo_safe && !solo)`, still the one composition. Solo
    never clears a VCA mute, a solo-safe submix is still VCA-muted, and a VCA mute never counts
    toward `any_solo`. A `vca_mute(strip, lane)` getter is added. Nothing changes `vca_mute` in this
    slice (#1244 adds the setter and its shadow).
  - host-web seeds each strip's `vca_mute` from the normalized model's `effective_strip_faders()`
    **before** `LiveControlSoloState::try_new` and `LiveRouteState::try_new`, so every following
    live send's seeded `source_lane_muted` equals D2's prepared gate.
- **D4. The C ABI guard (DESIGN P13, widened).** If #1053 has landed when this slice starts,
  `live_builtin_delta` classifies a delta as structural whenever the pre- or post-commit model's
  `vcas` is non-empty, until *Deliver value-only VCA edits to the running C ABI plan* (#1247). If it
  has not landed, confirm that the specs of #1053, #1225 and #1226 still carry that rule (if a spec
  has left `.github/ISSUE_SPECS/`, read it in git history) and record it in the PR.

## Deliverables

1. D1 in `crates/session`, with unit tests.
2. D2 in the builtins compiler and the graph compiler.
3. D3 in host-core's mute state and host-web's seeding.
4. D4 if #1053 has landed.
5. Docs:
   - `docs/SESSION_SCHEMA_V1.md`: the effective-value rule in the VCA paragraph #1240 wrote, which
     says VCAs now apply at preparation;
   - `docs/BUILTINS_AND_METERING_V1.md`: a "VCA groups" paragraph under "Solo in place" (`:124`):
     the composition and its precedence (mute wins; solo-safe is not VCA-safe; a VCA has no solo),
     and in "Sends follow mute" (`:166`) that a following send sees the VCA mute.

## Authorized paths

- `crates/session/src/lib.rs` and one new module beside it (for example `vca.rs`), and
  `crates/session/tests/` (one new file)
- `crates/builtins-compiler/src/lib.rs`
- `crates/graph-compiler/src/compile.rs` (the follow map only) and `crates/graph-compiler/tests/`
  (one new file)
- `crates/host-core/src/{solo.rs,lib.rs}`, the module holding `live_builtin_delta` (D4 only), and
  `crates/host-core/tests/` (one new file)
- `hosts/host-web/src/{lib.rs,tests.rs}` (the seeding and its tests)
- `docs/SESSION_SCHEMA_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No caps (#1243). No live VCA moves (#1244, #1245) and no C ABI value-only VCA path (#1247).
- Inside the batch only, until #1245: a browser kind 3 (`faderDb`) on a VCA member stages the
  member's own value and drops the offset.
- No render-code change. No VCA solo, no send trim and no automation.

## Hazards

- **Summation order.** D1's `f64` order is the contract; a different order or `f32` accumulation is
  a different rounding.
- **The diamond.** A strip reachable from one VCA along two paths counts it once.
- **Two consumers.** If either compiler recomputes reach or reads the member's own mute, a fresh
  plan and the live path diverge (gates 3 and 6).
- **The domain check.** Feeding the effective value to the preflight would clamp an out-of-domain
  member fader into a silent acceptance (gate 7).
- **Seeding order and value.** Seeding `vca_mute` after `LiveRouteState`, or `emitted` from the own
  mute only, makes the live follow mirror or the first solo toggle disagree with the plan (gates 5
  and 6).
- **Clamp and balance.** The member's own value is stored, never the effective one.
- **The iOS memset rule.** `check-cross-targets.sh` (`:121-139`) counts `bl _memset_pattern16` per
  product crate against `scripts/lib/aarch64-known-defects.py` (host-core's ceiling is 4; `session`,
  `builtins-compiler` and `graph-compiler` have no row, so one call fails). New code stores no
  splatted non-zero constant to memory.

## Objective gates

Every render comparison feeds distinct, non-constant signals per track and per lane, and compares at
a destination with no stateful downstream (empty console, no stateful inserts, identity input
section), or uses `render_pair_and_compare` (DESIGN section 7).

1. **The composition.** Unit tests in `crates/session/tests/`: `vca_effective_db` equals an
   independent `f64` reference for random inputs; returns `member_db` bit for bit (including `-0.0`)
   with no offset; clamps at both edges; and on the crafted order case (member 24.0, offsets -24.0
   from the lower VCA ID and 1e-30 from the higher) returns `(24 - 24) + 1e-30 = 1e-30`, where the
   reversed order would give `(24 + 1e-30) - 24 = 0`; `vca_reach` counts a diamond's top VCA once, includes a
   nested VCA's parents, is sorted by VCA ID whatever the declared order, and terminates on a
   cyclic (unvalidated) model.
   *Test value: it turns red if the sum is accumulated in `f32` or in another order, if the clamp is
   missing, or if reach double-counts a diamond or misses a nested parent.*
2. **The effective fader equals a plain fader.** New host-core test: random VCA forests (depth up
   to 4, up to 16 members, overlapping memberships, diamonds, values across the whole domain
   including clamping, both lanes, tracks and submixes as members) render bit-identically (NaNs
   folded) to the same session with `vcas: []` and each strip's fader set to the test's own `f64`
   reference value; 32 seeds at 48 kHz.
   *Test value: it turns red if an offset reaches the wrong strip or lane, misses a submix member,
   or the builtins compiler bakes another value than the composition's.*
3. **A post-fader send follows the VCA, and a VCA mute silences a member's follow send in a fresh
   plan** (VERIFY-2 M12). Track `kick` is in VCA `drums` at -6 dB and sends `post_fader` to bus
   `verb`: `verb`'s input is bit-identical to a VCA-free session with `kick`'s fader at -6. Track
   `snare` is in VCA `drums`, which is muted (left lane only in one case, both lanes in another),
   and sends `pre_fader` with `follows_mute: true` to `verb`: `verb`'s input is bit-identical to the
   same session with `vcas: []` and every member's own fader dB and mutes set to its effective
   values (`kick` included); with both lanes muted, the
   route op is inactive (its `graph::test_only_route_mix_counts` entry does not advance), and the canonical graph text
   carries `snare`'s `route-follow-zeroed` row.
   *Test value: it turns red if the VCA is applied after the sends, or if route lowering reads the
   member's own mute, so a fresh plan leaks a VCA-muted member's pre-fader send.*
4. **A VCA mutes a submix member.** A VCA muting bus `drums` makes the bus output exactly `+0.0`,
   bit-identical to the bus's own mute.
   *Test value: it turns red if VCA mute reaches tracks only.*
5. **Precedence in the mute state.** host-core unit tests over every combination of user mute, VCA
   mute, `any_solo`, solo and solo-safe: `effective_mute` matches the D3 formula; right after
   `try_new`, `strip_delta` is empty for every strip; soloing another track emits no record for a
   VCA-muted strip; and in host-web's native harness, with VCA `fx` muted, soloing, un-soloing and
   explicitly un-muting (kind 4, `false`) a member, each with smoothing 0 (an explicit kind 4 always
   stages a record, and a non-zero ramp on a settled lane can turn `+0.0` into `-0.0`, which is not
   this gate's claim), leaves it muted, bit-identically (via `render_pair_and_compare`) to a host
   that received none of those commands.
   *Test value: it turns red if solo clears a VCA mute, if solo-safe exempts a submix from it, or if
   `emitted` is seeded without the VCA mute.*
6. **The live send mirror starts at the prepared gate.** In host-web's native harness, a session
   whose follow source is VCA-muted boots with live controls; a kind 13 (`routeGainDb`) edit on that
   send, with smoothing 0 at a block boundary, renders `verb`'s input bit-identically to a fresh plan
   of the session with the edited gain, so the zeroed column stays zeroed.
   *Test value: it turns red if host-web seeds `vca_mute` after `LiveRouteState`, or seeds the
   mirror from the own mute, so the first live send edit reopens a VCA-muted column.*
7. **The member's own domain still refuses.** A track at `fader.left_db = 24.5` in a VCA at -6 dB
   refuses with `builtin.gain.domain` at `$.tracks[id=<id>].fader.left_db`.
   *Test value: it turns red if the preflight checks the clamped effective value.*
8. **No VCA moves no bit.** Every `output_sha256` and every canonical graph text of a session with
   `vcas: []` is unchanged:
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `./target/release/audit capi`
   - `bash scripts/check-graph-determinism.sh` (`target/issue6/fresh-process-determinism.json`
     identical to the base's)
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
9. **D4, only if #1053 has landed.** A value-only fader transaction on a VCA member, one on a VCA's
   fader and one on an unrelated track of a session that declares a VCA each produce a new epoch and
   render like a plan compiled from the committed model.
   *Test value: it turns red if the C ABI's live path pushes a member's own value and drops the
   VCA.*
10. **Browser, 4-lane, workspace and policy** (`npm ci` in `sdk/` first; `<A>`, `<B>` fresh empty
    directories):
    - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
    - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
    - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (no re-pin)
    - `bash scripts/check-sdk-headless.sh <A>`
    - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
    - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
    - `for x in session builtins graph host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
    - `bash scripts/check-cross-targets.sh`
    - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host (`session`, `builtins-compiler`,
      `graph-compiler` and `host-core` are in it), or CI's `aarch64-debug` at the batch push
      (recorded "at batch push").

No allocation gate: this slice adds no render-thread state; the composition runs at preparation.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- Whether #1053 had landed, and therefore whether D4 was implemented or its spec rule confirmed.

## Dependencies

- *Declare VCA groups in the session* (#1240)
- *Edit VCA groups through session transactions* (#1241), for gate 9 only (a `0702` transaction)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free; no render-code change.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
