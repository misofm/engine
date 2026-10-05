# Compose VCA offsets with stored fader automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5 and A2, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A producer automates a vocal fader; a fan rides the "vocals" VCA in the browser. The VCA move is
heard on top of the ride: the member lane follows its curve plus the VCA's offset, and after the
fader jump ramp the output equals a plan prepared with that VCA offset and the same automation,
bit for bit, whatever the number of VCAs that reach the lane. A member whose fader is not automated
keeps today's records. On the C ABI, where any VCA session rebuilds until *Deliver value-only VCA
edits to the running C ABI plan* (#1247), the rebuild carries the cell and jumps to the new
composed value; #1247 then makes the same move live through the same offsets cell. No rendered bit
moves for any session that exists today.

## Context

- **The one composition.** `vca_effective_db(member_db, offsets)` sums in `f64`, the member's own
  value first, then each offset in the order given, clamps to `[-144, 24]` and rounds once to `f32`;
  with no offset it returns the member bit for bit (`crates/session/src/vca.rs:19-34`).
  `effective_strip_faders` passes each strip's reaching VCAs' offsets in ascending VCA-ID order
  (`:96-120`). Its gate pins that order with a crafted case:
  `vca_effective_db(24.0, [-24.0, 1e-30]) == 1e-30` (`crates/session/tests/vca_composition.rs:53-83`,
  `:81`). So one precomputed sum of the offsets cannot give the same bits.
- **Draft 09a** creates and seeds the offsets cell (draft 09a D1): for each automated fader lane
  that at least one VCA reaches, a #1312 cell whose word count is set at preparation, one `f32`
  word per reaching VCA in the order `effective_strip_faders` adds them, seeded with the prepared
  offsets. Render computes each event's gain as `checked_fader_gain(vca_effective_db(v, offsets))`
  from the cell's newest slot. Nothing writes the cell after preparation yet.
- **The browser's live VCA state.** `LiveVcaState` (`crates/host-core/src/vca.rs:1-8`, struct
  `:84-106`) keeps each VCA's offset and mute, each strip's own dB and its reach in ascending VCA-ID
  order (`reach_of`, `:413`). `effective_db` calls the same composition (`:295-305`); `fader_delta`
  yields the lanes whose effective dB changed (`:322-339`).
- **Browser admission.** Kind 16 sets the VCA's offset (`hosts/host-web/src/lib.rs:4909-4918`); the
  VCA fader pass stages one `FaderDb` per changed member lane (`:5185-5214`). Draft 10b refuses only
  kind 3 on an automated lane; kind 16 is this slice's. The browser refuses a session with more than
  256 VCAs or 16,384 reach pairs at boot (`MAXIMUM_BROWSER_VCAS`, `:89`;
  `MAXIMUM_BROWSER_VCA_REACH_PAIRS`, `:99`).
- **The C ABI.** `classify_live_delta` refuses every delta while either model declares a VCA
  (`crates/host-core/src/live_delta.rs:216-218`), until #1247 removes that guard and diffs effective
  faders (#1247 D2). The caller caps VCAs with `maximum_vcas` (`crates/capi/include/miso_engine_v1.h:194`).
- **The cell primitive.** *Hold live values in latest-target cells on both hosts* (#1312) D1: a cell
  has three slots of words and a sequence; the writer fills every word of its back slot, then
  publishes; render reads the newest completed write in one pass and never sees a mixed slot.
  #1312 D1-D3 size each cell's word count per stage. **This draft amends #1312 D1** (amendment
  row): the primitive gains a constructor whose word count is set at construction, here at
  preparation, with all three slots allocated then and never resized, under #1312's loom gate
  (#1312 gate 3). Draft 09a, which lands before this slice, is the constructor's first user, so
  the amendment lands with #1312.

## Decisions frozen for this slice

- **D1. The composition is unchanged.** `vca_effective_db` (`crates/session/src/vca.rs:19-34`), its
  member-first order and its pinned test stay as they are. Render calls it at each event, so a flat
  curve at `v` renders the bits a static `v` prepares, however many VCAs reach the lane.
- **D2. The offsets cell is draft 09a's.** Its form is frozen: a #1312 cell whose word count is set
  at construction (preparation), one `f32` word per reaching VCA, in the order
  `effective_strip_faders` adds them (ascending VCA-ID order, `:96-120`). Draft 09a creates it and
  seeds it with the prepared offsets. This slice adds only the live VCA move that rewrites it, a
  whole slot at a time (D4), and the jump at the next block entry (D3). A lane no VCA reaches has no
  offsets cell, and render calls `vca_effective_db(v, [])`, which returns `v` bit for bit.
- **D3. Render.**
  - At each fader event, the gain is `checked_fader_gain(vca_effective_db(curve, offsets))`, with
    `offsets` the words of the cell's front slot (the newest write render has read), as draft 09a
    D1 composes it.
  - At block entry, when the cell is dirty, render reads the new slot. If the composed target at
    the jump's completion sample differs in bits from the lane's current target, the lane takes a
    jump at the block's first sample over the fader jump length (draft 12's `fader` word);
    otherwise nothing happens. Grid events during that jump are held (README "Held events").
- **D4. Browser admission.**
  - Preparation records which strip lanes have an offsets cell. A kind-16 move of VCA `v` writes,
    for each strip `v` reaches whose lane has an offsets cell, that lane's whole offsets slot from
    `LiveVcaState`'s offsets in `reach_of` order, the moved word included. #1312's publication rule
    needs every word of the slot, so the writer never writes one word alone. It writes only when the
    moved word's bits change.
  - For those lanes `fader_delta` yields nothing; the VCA fader pass writes the cells after every
    check of the batch, instead of staging `FaderDb`. Draft 10b does not refuse kind 16, so no
    refusal is removed.
  - A member's own fader move on an automated lane stays refused (draft 10b D2). A member that is
    not automated keeps today's `FaderDb` records.
- **D5. The C ABI.** Two halves, frozen:
  - **The rebuild.** While a VCA session rebuilds on the C ABI, preparation seeds the new offsets,
    draft 10a's carry keeps the fader cell's event state, and D3's adoption jump applies. This needs
    nothing from #1247.
  - **Live VCA edits on the C ABI** belong to *Deliver value-only VCA edits to the running C ABI
    plan* (#1247), not to this slice. #1247 D2 is amended (amendment row in the #1058 note): for a
    lane that the post-commit model automates, its effective-fader diff writes the lane's whole
    offsets slot, through this slice's writer, instead of a `FaderDb` record; its gates include
    gate 2 of this slice run on the C ABI.
  - This slice therefore has no dependency on #1247, and #1247 depends on this slice.
- **D6. Memory and work.** Each offsets cell holds 4 bytes per reaching VCA in each of #1312's three
  slots, 12 bytes per reaching VCA per automated lane, plus the cell's fixed sequence and index
  words. The reach of one lane is at most the session's VCA count: 256 in the browser, `maximum_vcas`
  on the C ABI. The browser's 16,384-pair bound also bounds the total. Render adds one `f64` per
  reaching VCA per event; nothing depends on song length.
- **D7. Out of scope here:** a VCA mute on a member whose mute is automated (draft 13a).
- **D8. The acked-batch question: can an ack ever precede a drop? No.** The browser checks the whole
  batch before the first cell write, and a cell write cannot fail; a replaced slot is in the
  committed state and counted (#1312 D2).

## Deliverables

1. D3: render's dirty read at block entry and the jump.
2. D4 in host-core and host-web: the record of which lanes have an offsets cell, and the writer.
3. The offsets-cell writer that #1247 calls (D5).

## Authorized paths

- `crates/host-core/src/vca.rs`, `crates/host-core/src/prepare.rs` (the record of lanes with an
  offsets cell only), `crates/host-core/tests/vca_live.rs`
- `crates/builtins-compiler/src/lib.rs` (the fader bank's dirty read of the offsets cell and the
  jump only)
- `hosts/host-web/src/lib.rs` (the VCA fader pass and kind 16 only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/vca_automation_realtime.rs` (new)

`crates/session/src/vca.rs` is called, never changed.

## Non-goals

- VCA automation (out of scope by decision 13, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:129`).
- A VCA mute on an automated mute lane (draft 13a). Live VCA edits on the C ABI (#1247).
- Any change to `vca_effective_db`'s order.

## Hazards

- **#1312's cell has a fixed word count today** (#1312 D1-D3 size it per stage). The #1312 D1
  amendment (Context) gives the construction-time word count; it must be on `main` before draft
  09a, its first user.
- **A redundant jump moves bits.** D3 compares bits before it jumps; D4 writes only on a change.

## Objective gates

1. **Browser, automated member** (same file, new). Track 0 has a linear fader ride and is a member
   of VCA `v`; track 1 is a member with no automation. Kind 16 moves `v` by -6 dB on a playing
   engine. From the completion sample of the first grid ramp after the jump, the output equals a
   plan prepared with `v` at -6 dB and the same automation, fed the same PCM from frame 0. Track 1
   receives exactly one `FaderDb`, as today.
2. **Many moves, one block** (same file). Ten kind-16 moves of `v` before one render: the output
   equals a twin that made only the last. Nested VCAs `v`, `w` reaching track 0: a move of `w` writes
   the slot with `v`'s word unchanged, in VCA-ID order.
3. **No jump for an unchanged offset** (same file). A kind-16 move of a VCA that does not reach
   track 0, or that moves `v` to its current value, writes no cell and moves no bit.
4. **Realtime** (`hosts/host-web/tests/vca_automation_realtime.rs`, new integration binary; it links
   `bench_support::alloc` and calls `assert_installed()` first). Gate 1's script on the render
   thread: `allocations == 0 && frees == 0` around every render call after warm-up.
5. **Commands:**
   - `cargo test --locked -p session` (unchanged, `vca_composition.rs` included),
     `cargo test --locked -p host-core --features host-core/test-support --test vca_live`,
     `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
6. **No rendered bit moves** for any session without stored automation, VCA sessions included:
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` gives the same
   `pcm_digest` at base and head (PR evidence), and the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a VCA move does not reach the offsets cell (the lane stays on the old
  composition), or if a non-automated member loses its record.
- Gate 2: red if render reads an older slot, the writer writes one word alone (a slot mixes old and
  new offsets), or nested VCAs land in the wrong words.
- Gate 3: red if an unchanged offset still writes the cell or retargets the lane.
- Gate 4: red if the cell read or the composition allocates on render.
- The composition order and the no-VCA rule are draft 09a's gate 1.

## Dependencies

- Draft 09a *Prepare stored fader automation and render it flat* (the offsets cell it creates and
  seeds).
- Draft 10a *Classify fader automation edits as carried rebuilds* (the carry).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the `fader` jump length).
- *Hold live values in latest-target cells on both hosts* (#1312), amended at D1: a cell whose word
  count is set at construction.
- Batch: R1, in one push with draft 10b *Refuse browser live commands on automated fader lanes*.
