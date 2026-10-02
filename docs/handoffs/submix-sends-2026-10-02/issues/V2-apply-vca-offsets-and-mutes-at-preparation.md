# Apply VCA offsets and mutes at preparation

Slice V2 of *VCA groups*. It is in the VCA batch. **Drafted; anchors re-verified at filing:** they
were read on `fe8ac679`, before batches K1-K3 of *Submix strips and live aux sends* moved them; the
Context says which slice moved what.

## Product outcome

A VCA is audible on every host, including a fan's control-free playback. Pulling VCA `drums` to
-6 dB lowers every member lane's effective fader by 6 dB, and its post-fader sends drop with it.
Muting the VCA mutes every member, tracks and submixes alike, and silences every `follows_mute` send
from a member, while each member keeps its own mute.

- Solo composes on top and never "un-mutes" a VCA-muted member.
- Hosts can bound the VCA count as a configured resource.
- Render code does not change: the effective values are folded into the fader sections, the
  follow-mute inputs and the mute state that already exist.

## Context (verified on `fe8ac679`; re-verify at filing)

- **Fader preparation.** Strip parameters are built per strip by `strip_parameters` in
  `crates/builtins-compiler/src/lib.rs` (*Iterate session strips, not tracks, wherever strip
  semantics apply* renamed it from `track_parameters`, `:4706-4749` at `fe8ac679`).
  - The fader's gain conversion is `db_gain` (`crates/builtins/src/lib.rs:5173`): `10^(dB/20)`,
    designed in `f64` through `math` and rounded once.
  - The prepared range check is `checked_fader_gain` (`:4108`, used by `fader_lanes` `:4134`) and
    `prepare_sections` (`:3131`).
- **Follow-mute preparation** (*Let a route into a submix follow its source strip's mute in the
  session*). The graph compiler's route lowering computes each `follows_mute` route's
  `source_lane_muted` from its source strip's session fader mutes and passes it to
  `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)`, which zeroes the
  muted source columns and silences the route when both are muted (`RouteGate`, `DESIGN.md` 5.7).
  It reads the strip's **own** mutes. VERIFY-2 M12: if V2 baked the VCA mute into the fader only, a
  fresh plan of a VCA-muted member would leak through its follow send while the live path (V3)
  silences it.
- **The per-strip mute state.** After *Give every strip one mute owner and live-control producers in
  host-core*, `host_core::LiveControlSoloState` (`crates/host-core/src/solo.rs`) is sized per strip,
  tracks then submixes, each entry `{ mutes: [bool; 2], solo_safe: bool }`, with submix entries
  solo-safe.
  - At `fe8ac679` it is per track: `try_new(&[[bool; 2]])` (`:113`), `effective_mute` (`:180`),
    `set_solo` (`:191`), `set_user_mute` (`:207`), `record_emitted` (`:224`), `track_delta` (`:238`),
    `commit` (`:257`), `rollback` (`:262`).
  - It composes `user_mute || (any_solo && !solo_safe && !soloed)` after that slice;
    `effective_mute(strip, lane)` is the only composition (`DESIGN.md` P7).
  - It emits mute deltas only, never a redundant one (`:32-45`).
  - host-web seeds it from the session's prepared mutes (`hosts/host-web/src/lib.rs:5875-5898` and
    `:6129` at `fe8ac679`; *Address submix strips in browser live commands* made it per strip).
  - *Admit live send commands in the browser* initialises `LiveRouteState.source_lane_muted` from this
    state's effective mute at preparation, and *Let a send follow its source strip's mute live in the
    browser* feeds its changes through `LiveRouteMuteFollow::delta`.
- **Caps.**
  - `HostPrepareCaps` (`crates/host-core/src/prepare.rs:99-137`) derives no `Default`. At
    `fe8ac679` there are 17 full struct literals:
    `crates/capi/src/runtime/compile.rs:367`; `crates/capi/tests/resource_lifecycle.rs:466`;
    `crates/host-core/src/limiter_linked_session.rs:255`; `crates/host-core/src/response.rs:997`;
    `crates/host-core/tests/`: `collapse_arming.rs:55`, `effect_live_controls.rs:31`,
    `effect_observation.rs:40`, `fp_environment.rs:32`, `input_liveness_live_controls.rs:70`,
    `live_addressing.rs:58`, `prepare.rs:22`, `randomized.rs:90`, `source_in_place.rs:25`,
    `spectrum.rs:19`, `symmetry_witness.rs:71`, `track_delay.rs:41`; `hosts/host-web/src/lib.rs:5675`.
    The `..caps()` functional-update literals survive a new field. *Count and cap submix strips in
    host preparation and the C ABI* added `maximum_submixes` and a test file, and later slices may
    add files: re-run `git grep -n "HostPrepareCaps {"` and update every full literal.
  - The C ABI's `miso_engine_v1_compile_limits` (`crates/capi/include/miso_engine_v1.h:107-133`; the
    Rust mirror `CompileLimits`, `crates/capi/src/abi.rs:84-134`, 208 bytes, `COMPILE_LIMITS_SIZE`
    `:296`) ended in `uint64_t reserved[4]` at offset 176 on `fe8ac679`. *Count and cap submix strips
    in host preparation and the C ABI* made the first word `maximum_submixes` (offset 176), leaving
    `uint64_t reserved[3]` at offset 184, pinned at `abi.rs:454` and
    `crates/capi/tests/c/abi_smoke.c:45`.
  - `limits_are_valid` (`crates/capi/src/runtime/compile.rs:354-359`) requires `reserved0 == 0`, the
    remaining reserved words zero, and `all_limits_nonzero` (a zero `maximum_submixes` is exempt,
    because 0 means "use `maximum_tracks`"). `prepare_caps` (`:366-390`) maps the limits field for
    field.
  - The `CompileLimits` struct literals (each `reserved: [0; 3]` after slice 09):
    `crates/capi/tests/resource_lifecycle.rs:160-187`, `crates/capi/src/runtime/tests.rs:17-`,
    `crates/capi/src/ffi.rs:1163-1190` and `tools/audit/src/capi.rs:238-265`. Every other
    `reserved: [0; 4]` in capi is another struct (`EngineConfig`, `Capabilities`,
    `PlanResourceReport`) and stays; in particular `crates/capi/src/ffi.rs:1353` sets
    `EngineConfig.reserved[2]`, which this slice does not touch.
  - `scripts/check-capi-abi.sh`'s layout self-test mutates the **first** `uint64_t reserved[4];` in
    the header (`:65`), the engine config at line 104, so this slice does not disturb it.
- **#1053.** Its D1 classifies a committed-model delta as live when only `fader` and `matrix_or_pan`
  differ (`.github/ISSUE_SPECS/1053-*.md:38-40`). If it has landed, its classifier is host-core's
  `live_builtin_delta`, behind the `control-provider` feature (`crates/host-core/Cargo.toml:15`).
  Slice 00 of the bus and send umbrella annotated #1053's spec with the VCA rule (`DESIGN.md` P13).
- **After *Declare VCA groups in the session*:** VCAs are parsed, validated (acyclic, in range) and
  canonical, and are inert.

## Decisions frozen for this slice

- **D1. One session helper.** In `crates/session`, one function computes, for every strip in
  `strips()` order (tracks, then submixes), the effective fader per lane and, separately, the VCA mute
  term per lane:

  ```rust
  pub struct EffectiveStripFader { pub db: [f32; 2], pub mute: [bool; 2], pub vca_mute: [bool; 2] }
  impl SessionModel {
      pub fn effective_strip_faders(&self) -> Vec<EffectiveStripFader>; // strips() order
  }
  ```

  - **Reach.** `reach(strip)` is the **set** of VCAs from which the strip is reachable through
    membership, directly or through nested VCAs; each VCA counts once.
  - **Gain.** `clamp(member_db + sum(v.lane_db for v in reach), -144.0, 24.0)`, summed in `f64` in a
    fixed order (the member, then the VCAs in ascending ID order), rounded once to `f32`. With an
    empty reach the result is the member's value, bit for bit.
  - **VCA mute.** `vca_mute[lane] = any(v.lane_mute for v in reach)`.
  - **Mute.** `mute[lane] = member_mute[lane] || vca_mute[lane]`.

  It runs at preparation, off the render thread, and is the only place reach is computed.
- **D2. Two consumers, one computation** (VERIFY-2 M12).
  - The builtins compiler's `strip_parameters` takes D1's `db` and `mute`; `db_gain` converts the
    gain. The member's own `fader_db` must still be in its domain, as today.
  - The graph compiler's route lowering takes D1's `mute` of the source strip as
    `source_lane_muted` for every `follows_mute` route. So a VCA-muted member's follow send is
    silent (and inactive when undelayed) in a fresh plan, exactly as V3's live path makes it.
- **D3. The mute state** gains a per-lane `vca_mute` input in its per-strip entry, seeded at
  preparation from D1's `vca_mute` for every strip, tracks **and** submixes.
  - Composition becomes `(user_mute || vca_mute) || (any_solo && !solo_safe && !soloed)`, still
    through `effective_mute(strip, lane)`.
  - The user-mute mirror stays the member's own mute.
  - Seeding emits nothing: the prepared fader already holds the effective mute.
  - host-web seeds `vca_mute` before it initialises `LiveRouteState`, so the live routes' prepared
    `source_lane_muted` equals D2's.
- **D4. Caps.**
  - `HostPrepareCaps.maximum_vcas`, checked beside the track and submix checks with the same refusal
    (`host.resource.count`), and `HostPrepareReport.vca_count`. host-web sets `u64::MAX`, as for its
    other count caps.
  - In the C ABI, `reserved[0]` of the three-word array becomes `uint64_t maximum_vcas` (offset 184),
    where 0 means "use `maximum_tracks`" (exempt from `all_limits_nonzero`), and `reserved` shrinks to
    `uint64_t reserved[2]` at offset 192. The size (208) and the exported symbols are unchanged.
    `limits_are_valid` requires `reserved == [0; 2]`; `prepare_caps` maps the field. The offset pins
    (`abi.rs`, `abi_smoke.c`) and the four `CompileLimits` literals move with it. The header and the
    Rust mirror say "0 means use `maximum_tracks`".
- **D5. The #1053 guard (`DESIGN.md` P13).** If #1053 has landed, its classifier treats a delta that
  changes **any** fader field of a VCA member (reach not empty, in the post-commit model) as
  structural, and also a delta to any VCA's fader. The guard holds until *Deliver value-only VCA edits
  to the running C ABI plan* removes it. If #1053 has not landed, verify that its spec still carries
  the rule from *Record the submix, send and VCA ruling*.

## Deliverables

1. D1 in `crates/session`, with unit tests (reach of diamonds, order, clamp).
2. D2 in the builtins compiler and the graph compiler.
3. D3 in host-core's mute state, and host-web seeding it.
4. D4 in host-core, capi (header, Rust mirror, `limits_are_valid`, `prepare_caps`) and every struct
   literal.
5. D5 if #1053 has landed.
6. Docs:
   - `docs/BUILTINS_AND_METERING_V1.md`: VCA composition beside solo, and that follow sends see the
     VCA mute;
   - `docs/C_ABI_V1_QUALIFICATION.md`: `maximum_vcas`, its zero meaning and the unchanged size;
   - `docs/SESSION_SCHEMA_V1.md`: the effective-value rule.

## Authorized paths

- `crates/session/src/{model.rs,lib.rs}` and one new module beside them, and `crates/session/tests/`
- `crates/builtins-compiler/src/lib.rs`
- `crates/graph-compiler/src/{compile.rs,ids.rs}` (route lowering's `source_lane_muted` only) and
  `crates/graph-compiler/tests/` (one new test file)
- `crates/host-core/src/{prepare.rs,solo.rs,lib.rs,limiter_linked_session.rs,response.rs}`, the
  classifier #1053 added (D5 only), and `crates/host-core/tests/` (the caps literals and one new test
  file)
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/{abi.rs,ffi.rs,runtime/compile.rs,runtime/tests.rs}`,
  `crates/capi/tests/` (`resource_lifecycle.rs`, `c/abi_smoke.c` and one new test file)
- `tools/audit/src/capi.rs` (the `CompileLimits` literal only)
- `hosts/host-web/src/lib.rs` (seeding and the caps literal), `hosts/host-web/src/tests.rs`
- `scripts/check-capi-abi.sh`, only if a pinned spelling of the compile-limits tail must follow D4
- the three docs above
- this spec

## Non-goals

- No live VCA moves (V3, V4) and no C ABI value-only VCA path (V5).
- No render-code change.
- No VCA solo, no send trim and no automation.

## Hazards

- **Summation order.** D1's `f64` order is the contract. A different order, or `f32` accumulation, is
  a different rounding.
- **The diamond.** A strip reachable from VCA C along two paths counts C once.
- **Two consumers.** If either compiler recomputes reach or reads the member's own mute instead of
  D1's, a fresh plan and the live path diverge (gate 3).
- **The mute state.** If `vca_mute` is not seeded at preparation, the first solo toggle "un-mutes" a
  VCA-muted member; if it is seeded after `LiveRouteState`, the live follow mirror starts stale.
- **Clamp and balance.** The member's own value is stored, never the effective one. A VCA that clamps
  a member and is then restored returns the member to its own value.
- **`all_limits_nonzero`.** A zero `maximum_vcas` is valid ("use `maximum_tracks`"); do not let the
  nonzero rule refuse every existing caller.

## Objective gates

1. **The effective fader equals a plain fader.** New test in `crates/host-core/tests/`: random VCA
   forests and diamonds render bit-identically (NaNs folded) to the same session with no VCAs and each
   strip's fader set to D1's value, computed in the test by an independent `f64` reference. The
   generator draws depth up to 4 and up to 16 members, overlapping memberships, values across the
   whole domain including clamping, both lanes, and tracks and submixes as members. 32 seeds at
   48 kHz.
   *Test value: it turns red if reach double-counts a diamond, if the sum's order or precision
   differs, if the clamp is missing, or if an offset reaches the wrong lane or misses a submix
   member.*
2. **A post-fader send follows the VCA.** Track `kick` is in VCA `drums` at -6 dB and sends
   `post_fader` to bus `verb`. The bus input is bit-identical to a VCA-free session with
   `kick.fader = -6`.
   *Test value: it turns red if the VCA is applied after the sends, as a bus fader would be.*
3. **A VCA mute silences a member's follow send in a fresh plan** (VERIFY-2 M12). Track `snare` is in
   VCA `drums`, which is muted (left lane only in one case, both lanes in another); `snare` sends
   `pre_fader` with `follows_mute: true` to bus `verb`. The plan prepared from that session renders
   `verb`'s input bit-identically to the same session with no VCA and `snare`'s own
   `left_mute`/`right_mute` set to the VCA's; with both lanes muted, the send's route op is inactive
   (its executed-mix counter does not advance).
   *Test value: it turns red if route lowering reads the member's own mute instead of the effective
   one, so a fresh plan leaks a VCA-muted member's pre-fader send that the live path silences.*
4. **A VCA mutes a submix member.** A VCA muting bus `drums` makes the bus's output exactly `+0.0`,
   bit-identical to the bus's own mute.
   *Test value: it turns red if VCA mute reaches tracks only.*
5. **Solo keeps a VCA mute.** In host-web's native harness (`render_pair_and_compare`,
   `hosts/host-web/src/tests.rs:6681`), with VCA `fx` muted, soloing and then un-soloing a member
   leaves it muted, bit-identical to a host that never soloed; with a `follows_mute` send from that
   member, the send stays silent throughout.
   *Test value: it turns red if the mute state drops `vca_mute`, or seeds it after the live route
   mirror.*
6. **No VCA moves no bit.** Every `output_sha256` and every canonical graph text of a session with
   `vcas: []` is unchanged:
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
7. **Caps.**
   - A session with `maximum_vcas + 1` VCAs refuses at preparation with `host.resource.count`, and
     one at the cap prepares with `report.vca_count` equal to the VCA count.
   - Through `miso_engine_v1_compile_session`: `maximum_vcas = 0` bounds VCAs by `maximum_tracks`; a
     nonzero value bounds by itself; a nonzero `reserved[0]` or `reserved[1]` of the two-word array
     still refuses.
   - `crates/capi/tests/c/abi_smoke.c` asserts `offsetof(.., maximum_submixes) == 176`,
     `offsetof(.., maximum_vcas) == 184`, `offsetof(.., reserved) == 192`, and size 208.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.

   *Test value: it turns red if VCAs are uncounted, if the C bound silently reuses another field or
   ignores the new one, or if the remaining reserved words stop being refused.*
8. **The #1053 guard, if #1053 has landed.** A value-only fader transaction on a VCA member, and one
   on a VCA's fader, each produce a new epoch and render like a plan compiled from the committed
   model.
   *Test value: it turns red if the C ABI's live path pushes the member's own value and drops the VCA
   offset.*
9. **Allocation.** With VCAs present, `allocations == 0` on the render thread after warm-up for gate
   1's sessions, measured with `bench_support::alloc`'s thread-scoped counters.
   *Test value: it turns red if VCA composition leaked onto the render path (for example a per-block
   reach walk) instead of staying at preparation.*
10. **4-lane, workspace and policy.**
    - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host (session, builtins-compiler,
      graph-compiler, host-core and capi are in it), or CI's `aarch64-debug` at the batch push.
    - the workspace test command (`DESIGN.md` section 7)
    - `cargo test --locked --release -p audit -p bench -p console-workload` (`tools/audit` literal)
    - `./target/release/audit capi`
    - `bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`,
      `bash scripts/check-builtins-policy.sh`, `bash scripts/test-builtins-policy.sh`,
      `bash scripts/check-graph-policy.sh`, `bash scripts/test-graph-policy.sh`,
      `bash scripts/check-session-policy.sh`, `bash scripts/test-session-policy.sh`
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The updated `HostPrepareCaps` and `CompileLimits` literal lists.
- Whether #1053 had landed, and therefore whether D5 was implemented here.

## Dependencies

- *Declare VCA groups in the session*

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free.
- No new exported C symbol and no C struct size change.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
