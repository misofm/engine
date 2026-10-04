# Deliver value-only VCA edits to the running C ABI plan

Slice V8 of *VCA groups* (#1239). **Blocked:** it starts only after #1053's core (#1257 and #1258; not
landed at filing),
*Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225) and *Let C ABI
sends follow their source strip's mute live* (#1226) have closed, and the VCA batch (#1240-#1246) is on
`main`. **Re-verify every anchor when it starts:** #1053, #1225 and #1226 add the classifier and the
live commit path this slice extends, and none of them exists at `8c6268967`. (Amended 2026-10-04,
when #1053 became an umbrella of slices #1253-#1266; the references below follow its decision
record and guards G1-G3.)

The design record cited below (`DESIGN`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A fan's personal mix on a phone rides a VCA ("drums -3 dB") through the C ABI without a plan
rebuild, with the browser's composition and bits. A member's own fader edit becomes value-only too,
because the VCA offset is composed instead of guarded off, and a VCA mute silences its members'
`follows_mute` sends in the same commit. Every other edit of a session that declares a VCA gets back
the value-only path #1053, #1225 and #1226 gave VCA-free sessions.

## Context (forward-looking; re-verify after the dependencies land)

- **At filing (`8c6268967`)** the C ABI has no live path: every committed transaction replaces the
  plan (`git grep -n classify_live_delta` is empty). VCAs apply at preparation on the C ABI since
  *Apply VCA offsets and mutes at preparation* (#1242), so a structural edit renders VCAs correctly.
- **After #1053's core (#1257):** capi classifies a committed-model delta as live through
  host-core's `classify_live_delta` (#1053 D1, #1255; behind the `control-provider` feature,
  `crates/host-core/Cargo.toml:15`), domain-checks, room-checks every queue, checks the protocol
  token, pushes, then commits, which cannot fail after the check (#1053 D6); every live fader and
  mute record uses #1053's ramp seam, `LiveRamps::for_session` (its D3; a step until #1054).
- **After #1225 and #1226:** strip faders and pans (tracks and submixes) and the gain, mute and
  matrix of routes into submixes are live; a strip mute change also pushes its follow records
  through `LiveRouteMuteFollow::delta` with a capi-owned `LiveRouteState`. #1225's D3 builds a route
  record's `source_lane_muted` from the source strip's committed mutes, and #1226's D1 hands `delta`
  the post-commit model's mutes as `effective_mute`.
- **The VCA guard (DESIGN P13, widened at filing; #1239; #1053's G3):** `classify_live_delta` classifies a delta
  as structural whenever the pre- or post-commit model declares a VCA. It exists because #1053
  would push a member's own fader and drop the offset, and #1225/#1226 would read raw mutes and
  reopen a VCA-muted member's following send. #1242 landed first, so #1053's #1255 implements it.
  This slice removes it.
- **After #1242:** `session::vca_effective_db`, `SessionModel::vca_reach` and
  `SessionModel::effective_strip_faders` (per strip `db`, `mute = own || vca_mute`, `vca_mute`).
- **After #1244:** `host_core::LiveVcaState` (`try_new` from a model, setters, `effective_db`,
  `vca_mute`, `fader_delta`, `record_emitted_db`, shadow, `commit`, `rollback`).
- **The C ABI has no solo**, so a strip's effective mute there is `own_mute || vca_mute` of the
  committed model (#1239 rule 6).
- **Opcodes** (#1241): `0700` upsert VCA (it can change membership), `0701` remove VCA, `0702` set VCA
  fader.
- **Resources.** `capi_resources` (`crates/capi/src/runtime/compile.rs:109-213` at `8c6268967`) and
  the `resource_lifecycle` oracles (`crates/capi/tests/resource_lifecycle.rs`).

## Decisions frozen for this slice

- **D1. Classification.** Remove the VCA guard. A delta is live when, besides the fields #1053,
  #1225 and #1226 made live, it changes only VCA `fader` values (`0702`, or a `0700` that keeps the
  VCA's membership). Adding or removing a VCA, and any membership change, is structural. A member's
  own fader delta is live, with its effective value composed.
- **D2. Effective mute on the C ABI.** Everywhere the C ABI live path reads a strip's mute -- the
  `LiveRouteState` seed at preparation (#1226 D1), the `effective_mute` it hands
  `LiveRouteMuteFollow::delta` (#1226 D1) and a following route record's `source_lane_muted` (#1225
  D3) -- it reads the model's `effective_strip_faders()` `mute` (`own || vca_mute`; the committed
  model at preparation, the post-commit model in a delta), never the raw
  `left_mute`/`right_mute`. So the mirror starts equal to the prepared gate (#1242 D2).
- **D3. Composition.** capi keeps one `LiveVcaState` per live plan, seeded at preparation from the
  committed model. A live VCA delta, or a member fader delta, updates it under its shadow; its member
  `FaderDb` records (`fader_delta`), the strip mute records (each changed strip's effective mute) and
  the follow records are domain-checked and room-checked together, then pushed, then committed, in
  #1053's order. A full queue is typed `Backpressure` with model, revision, replay and every mirror
  unchanged.
- **D4. Ramps.** Every record uses #1053's ramp seam (`LiveRamps::for_session`); a VCA change's ramp applies to every
  member record it emits.
- **D5. Resources.** `LiveVcaState::retained_bytes()` (its tables, mirrors and shadow) is charged in
  `capi_resources`, and the oracles change by exactly those rows (zero for a session without VCAs).

## Deliverables

- D1-D5 in capi's live commit path and host-core's classifier.
- `docs/C_ABI_V1_QUALIFICATION.md`: VCA fader edits and member fader edits are value-only;
  membership edits are structural.
- #1053's guard G3 marked superseded in its spec, if the spec is still in
  `.github/ISSUE_SPECS/`.

## Authorized paths

- `crates/capi/src/runtime/{control.rs,compile.rs}` and any capi runtime module #1053, #1225 or
  #1226 added for the live commit path
- `crates/capi/tests/` (`resource_lifecycle.rs` and one new test file)
- `crates/host-core/src/` (the classifier only; `LiveVcaState`, `LiveRouteState` and
  `LiveRouteMuteFollow` are reused, not changed) and its tests
- `docs/C_ABI_V1_QUALIFICATION.md`, `.github/ISSUE_SPECS/1053-*.md` (guard G3's bullet only)
- this spec

## Non-goals

- No new symbol, opcode or field.
- No VCA solo, no send trim and no automation.
- No change to the browser path.

## Hazards

- **#1053's L0 window.** A live VCA edit during a plan swap lands in the candidate.
- **Redundant records.** A VCA move that changes no effective value pushes nothing.
- **Reach drift.** The capi mirror's reach comes from preparation; a membership change must stay
  structural, or the pushed records target a reach the plan no longer has.
- **Raw mutes.** Any C ABI path left reading `left_mute`/`right_mute` directly reopens a VCA-muted
  member's following send (gates 2 and 3).
- **The iOS memset rule.** `check-cross-targets.sh` counts `bl _memset_pattern16` per product crate
  against `scripts/lib/aarch64-known-defects.py`; `capi` has no row, so one call fails it. New code
  stores no splatted non-zero constant to memory.

## Objective gates

1. **PCM through the C ABI.** New capi test file. `0702` on a nested VCA, a `0702` that mutes that
   VCA (with a `follows_mute` pre-fader send from one member), and a member's own `020f`
   (`SetTrackFader`) each render bit-identically to a plan compiled from the committed model after
   `latency_samples` plus the ramp (plus a send's compensation delay where it has one). No new epoch,
   no re-seek; the downstream is stateless. Run with 1 and 10 tracks at each of the four launch
   rates (44.1, 48, 88.2 and 96 kHz).
   *Test value: it turns red if a VCA or member fader edit stays structural, if capi's composition
   diverges from preparation's, or if a VCA mute leaves a member's follow send open.*
2. **The mirror starts at the prepared gate.** With a member VCA-muted and a `follows_mute` send from it, toggling its own mute
   (`020f` with `left_mute`/`right_mute` flipped while the VCA mute holds, so the effective mute is
   unchanged) pushes no strip-mute and no follow record, and renders bit-identically to an untouched
   plan.
   *Test value: it turns red if capi seeds its route mirror from raw mutes, so the next mute delta
   yields a redundant follow record that retargets a settled lane.*
3. **Effective mute in send edits.** With a member VCA-muted, a live `0505` (`SetRouteGainDb`) on its
   following send renders bit-identically to a fresh plan of the committed model (the column stays
   zeroed).
   *Test value: it turns red if a route record reads the raw committed mute instead of
   `own || vca_mute`.*
4. **Membership is structural.** `0700` changing members, `0700` adding a VCA, and `0701` each
   produce a new epoch.
   *Test value: it turns red if a membership change is pushed as values to a plan whose reach no
   longer matches.*
5. **No ack before a drop.**
   - A VCA move that would overfill a member queue or a follow route queue is typed `Backpressure`,
     with model, revision, replay and every mirror unchanged.
   - An exact replay pushes nothing.
   - A VCA move that changes no effective value (every member clamped) pushes nothing.

   *Test value: it turns red if any record is pushed before every queue's room is checked, or if
   composition re-emits unchanged targets.*
6. **Realtime.** The two-thread race (in `crates/capi/tests/resource_lifecycle.rs`, *Qualify live C
   ABI edits against a concurrently rendering plan*, #1258) races VCA edits against render and a structural swap over 20 runs: `allocations == 0`
   and `frees == 0` around each render call after warm-up, zero `INTERNAL` results, and a final
   block bit-identical to a fresh plan of the final committed model.
   *Test value: it turns red if VCA composition allocates on the render thread, or a racing VCA edit
   lands in the retiring plan.*
7. **Unchanged behaviour.**
   - Every existing capi test passes, except the guard cases (an edit of a VCA session was
     structural), which this slice inverts in the same PR.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, reports zero allocations, locks and syscalls.
   - The `resource_lifecycle` oracles change only by the D5 rows.
8. **Workspace, 4-lane and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the push.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- The `resource_lifecycle` row changes, with reasons.

## Dependencies

- *Enumerate VCA groups and drive them from the SDK* (#1246; the VCA batch on `main`), which
  includes *Edit VCA groups through session transactions* (#1241, the opcodes D1 classifies)
- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257) and
  *Qualify live C ABI edits against a concurrently rendering plan* (#1258), the core of the umbrella
  *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053) -- **not landed at filing**
- *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225)
- *Let C ABI sends follow their source strip's mute live* (#1226)

## Standing rules for the implementer

- Extend #1053's, #1225's and #1226's paths; do not fork them.
- No ack precedes a drop. The commit after the first push is infallible.
- Never emit a redundant record.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted, or inverted, in the
  same PR.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
