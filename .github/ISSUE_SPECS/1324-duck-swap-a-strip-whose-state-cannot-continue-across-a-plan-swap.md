# Duck-swap a strip whose state cannot continue across a plan swap

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit changes how a playing strip processes audio (an insert added, removed or
reordered, an effect's quality or link mode, `delay_samples`, a value the plan keeps prepared, a
bypass the plan keeps prepared), the strip ducks out over the session mute ramp, the new plan
is swapped in after the ramp, and the strip fades back in. Today the edited strip's new chain
starts at rest at the swap block, a step from the old chain's output to the new one's. Strips the
edit did not touch keep playing, bit for bit.

## Context

- D15-7: a changed prepared (rule-3) value restarts its owner behind a D15-9 transition; an owner
  carries only when the join of stream A's carry slices (#1277, #1279-#1284) finds its values and
  layout equal. Successor preparation: `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`), `prepare_host_runtime_with_live_lanes_successor`
  (`:818`).
- The classifier sends these edits to a rebuild: `LiveRebuild::Structure` for insert order,
  identity, quality, link mode and every other non-value field, `Prepared` for a non-`Block`
  parameter, `PreparedBypass` for the delay and the multiband (`crates/host-core/src/live_delta.rs:118-147`,
  steps at `:150-180`). `delay_samples` is a channel-builtin field
  (`crates/session/src/visit.rs:99`).
- Round 2 (Q-C2): warm state removes at-rest artefacts, but a switch between two different chains is
  still a step of `new - old` at the swap, so the duck stays even with a warm successor (#1287); the
  duck runs on the predecessor before `S` and the fade-in on the successor after it.
- The shared step: *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325) writes
  ramped mutes into the displaced plan's strip lanes, reads `p = render_sample`
  (`crates/capi/src/runtime/plan.rs:10`, `:236`) and schedules the successor at
  `S = ceil_q(p + q + N)`. Its host-core function `plan_strip_transition` returns an empty "arm"
  set; this issue fills it.
- The armed fade-in: *Fade in a strip that a swap adds during playback* (#1288): an armed fader
  channel is prepared muted and fires an unmute ramp of `N` at the first block boundary at or after
  `played block + D`.

## Decisions frozen for this slice

- **D1. Which strips.** Exactly `PreparedHost::restarted_strips()` (#1277 D6): the strips in both
  plans with at least one owner the carry join restarts for a prepared difference, which each later
  carry slice extends with its family. This is the D15-9 list on the carry rules, and it stays
  correct for any future reason an owner cannot carry. A strip whose owners all
  carry is never ducked; its value changes stay live or carried-then-retargeted (D15-7).
- **D2. Whole pre-fader restart.** For a duck-swapped strip, preparation marks every pre-fader
  owner and the fader stage not carried, after the join. A partial carry upstream of a restarted
  latent owner would emit its carried tail, then the restarted owner's at-rest zeros: a hole with a
  step at each edge. Post-fader owners (matrix/pan, sends, compensation lines) carry as the join
  says; they carry the ducked tail.
- **D3. Arm.** Each duck-swapped strip is armed as in #1288 D2 (channels the model leaves unmuted),
  with `D` per #1288 D4 (its pre-fader latency; for a submix plus the longest compensation delay of
  an input route whose line starts at rest). A playing source fires at the adoption block, so the
  fade starts at `S + ceil_q(D)`.
- **D4. Duck.** `plan_strip_transition` returns the duck-swapped strips with the removed ones; the
  control plane writes their ramped mutes and schedules `S` exactly as #1325 D2-D3, in the same
  transaction, with one `S`.
- **D5. Edits during the window.** A live edit to a ducked strip goes to the newest candidate's
  cells and applies at adoption (D15-17). A mute disarms the strip (#1288 D2), so the user's mute
  wins; a fader edit sets the gain the fade reaches. A structural edit supersedes (#1325 D6): the
  duck mutes are records pushed to the displaced plan (P1.4 base), so a newer transaction that
  reverts the edit carries the ducked strip and its unmute is a ramped retarget.
- **D6. Reporting.** As #1325 D5: path `rebuild`, completion at `S` as `EXACT` (or `SUPERSEDED`),
  never a fallback flag or counter; those belong only to the catch-up fallback (#1358).
- **D7. Realtime and the acked-batch question.** As #1325 D8. The arm table is sized at
  preparation.

## Deliverables

1. D1-D3 in host-core successor preparation, after the carry join; D4 in
   `crates/host-core/src/transition.rs`.
2. The control plane passes the duck set through #1325's path (no new control-plane code beyond
   the call).
3. Header comment and `docs/C_ABI_V1_QUALIFICATION.md`: which edits duck-swap, the dip length
   (`N`, the wait to `S`, `D`, `N`).
4. Tests below.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/transition.rs`,
  `crates/host-core/tests/successor_swap.rs`
- `crates/capi/tests/strip_transitions.rs`
- `crates/capi/include/miso_engine_v1.h` (comments only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Added strips (#1288) and removed strips (#1325).
- Composing with a warm successor: *Pre-roll a successor whose latency grows* (#1287) fixes `S`
  for the catch-up and must not pick an `S` below this duck's bound; it also sets `D = 0` for a
  strip its catch-up has warmed.
- No crossfade. A true crossfade with ghost strips is deferred by D15-9; it reopens on a measured,
  audible dip in a listening test.

## Hazards

- The duck set is as large as the carry join makes it. While a state family has no carry slice
  (for example per-node effect instances before #1282), every strip holding such an owner is
  duck-swapped on any structural edit. That is correct (no click), and each of stream A's carry
  slices shrinks it.

## Objective gates

Through the exported C entries, quantum 128, 48 kHz, `N = 2000`, single-threaded, as #1325's gates.

1. **Duck, swap, fade.** Track A muted in the model, track B audible. Between blocks `k` and
   `k + 1` a transaction adds a compressor insert to B. (a) Every block up to `S` equals a reference
   that live-mutes B at the same point. (b) From `S` on, every block equals a fresh plan of the
   successor session, fed the same source frames, with B muted and live-unmuted with `N` at its
   first block. (c) The same with a true-peak limiter insert: the fresh plan's unmute lands at
   block `ceil_q(D)/q`. All bit-identical.
2. **Only edited strips duck.** A and B audible, B's source fed exact zeros; the gate 1 transaction
   leaves every block equal to a run with no transaction.
3. **Every trigger ducks.** For each edit (insert added; removed; reordered; an insert's quality;
   its link mode; `delay_samples`; the true-peak limiter's `lookahead`, which is
   `AutomationRate::None`, `crates/true-peak-limiter/src/lib.rs:198-210`), B's output holds at
   least `q` consecutive exact-zero frames per channel ending at `S`. A value-only fader edit
   produces no such run.
4. **Whole restart.** In `successor_swap.rs`, at `Backend::Simd8`, `Backend::Simd4` and prepared
   between render calls: a strip whose second insert is replaced has all its pre-fader owners and
   its fader not carried and is armed; an untouched strip carries every owner.
5. **Realtime.** As #1325 gate 4, over the fade blocks too.
6. Commands:
   - `cargo test --locked -p capi --test strip_transitions`
   - `cargo test --locked -p host-core -p capi --features host-core/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1(b): a successor that carries the ducked fader (the strip stays muted) or does not arm it
  (it pops in at full gain) turns it red. Gate 1(c): a fade that ignores `D` turns it red.
- Gate 2: a duck applied to every strip, or an untouched strip restarted, turns it red.
- Gate 3: an edit kind that restarts an owner without the duck (a step at `S`) turns it red.
- Gate 4: a partial carry upstream of a restarted owner (the hole in D2) turns it red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Carry fader, mute and pan ramps across a plan swap* (#1277).
