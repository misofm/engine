# Compose VCA offsets with stored fader automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5 and A2, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R1.

## Product outcome

A producer automates a vocal fader; a fan rides the "vocals" VCA, in the browser or through a C ABI
host. The VCA move is heard on top of the ride: the member lane follows its curve plus the VCA's
offset, and after the fader jump ramp the output equals a plan prepared with that VCA offset and
the same automation, bit for bit, whatever the number of VCAs that reach the lane. On both hosts
the move goes through the shared commit's VCA row (*Deliver value-only VCA edits to the running C
ABI plan*, #1247; the browser through #1382), which writes the lane's offsets cell instead of a
fader record, and the path is `live`. A member whose fader is not automated keeps today's records.
No rendered bit moves for any session that exists today.

## Context

- **The one composition.** `vca_effective_db(member_db, offsets)` sums in `f64`, the member's own
  value first, then each offset in the order given, clamps to `[-144, 24]` and rounds once to `f32`;
  with no offset it returns the member bit for bit (`crates/session/src/vca.rs:19-34`).
  `effective_strip_faders` passes each strip's reaching VCAs' offsets in ascending VCA-ID order
  (`:96-120`). Its gate pins that order with a crafted case:
  `vca_effective_db(24.0, [-24.0, 1e-30]) == 1e-30` (`crates/session/tests/vca_composition.rs:53-83`,
  `:81`). So one precomputed sum of the offsets cannot give the same bits.
- **Draft 09a** creates and seeds the offsets cell (draft 09a D1): for each automated fader lane
  that at least one VCA reaches, a #1312 cell whose word count is set at preparation (the
  constructor draft 09a adds), one `f32` word per reaching VCA in the order
  `effective_strip_faders` adds them, seeded with the prepared offsets. Render computes each
  event's gain as `checked_fader_gain(vca_effective_db(v, offsets))` from the cell's newest slot.
  Nothing writes the cell after preparation yet.
- **The shared VCA row.** #1247 D2: the classifier's strip step compares `effective_strip_faders()`
  of `current` and `next`, lane by lane, and emits one `FaderDb` per lane whose effective gain bits
  change. #1247 makes VCA rides, mutes and membership live on the C ABI (D15-6), because the C ABI's
  render plane holds no VCA state. After #1382 the browser's VCA commands (kinds 16 and 17) are
  lowered to transactions and take the same row.
- **Draft 10 D3** makes that step emit no `FaderDb` for a lane that `next` automates, so until this
  slice a VCA move does not reach an automated member.
- **The cell primitive.** #1312 D1: a cell has three slots of words and a sequence; the writer fills
  every word of its back slot, then publishes; render reads the newest completed write in one pass
  and never sees a mixed slot.

## Decisions frozen for this slice

- **D1. The composition is unchanged.** `vca_effective_db` (`crates/session/src/vca.rs:19-34`), its
  member-first order and its pinned test stay as they are. Render calls it at each event, so a flat
  curve at `v` renders the bits a static `v` prepares, however many VCAs reach the lane.
- **D2. The offsets cell is draft 09a's.** This slice adds only the write that rewrites it, a whole
  slot at a time (D4), and the jump at the next block entry (D3). A lane no VCA reaches has no
  offsets cell, and render calls `vca_effective_db(v, [])`, which returns `v` bit for bit.
- **D3. Render.**
  - At each fader event, the gain is `checked_fader_gain(vca_effective_db(curve, offsets))`, with
    `offsets` the words of the cell's front slot (the newest write render has read), as draft 09a
    D1 composes it.
  - At block entry, when the cell is dirty, render reads the new slot. If the composed target at
    the jump's completion sample differs in bits from the lane's current target, the lane takes a
    jump at the block's first sample over the fader jump length (draft 12's `fader` word);
    otherwise nothing happens. Grid events during that jump are held (README "Held events").
- **D4. The shared commit writes the cell.**
  - In #1247's strip step, for a lane that `next` automates and that keeps the same reach, the
    classifier compares the ordered offsets of `current` and `next` (each reaching VCA's offset in
    `effective_strip_faders` order). If any bit differs, it emits one offsets-cell write that
    carries the whole slot from `next`, the moved word included. #1312's publication rule needs
    every word of the slot, so the writer never writes one word alone. The path is `live`.
  - **A reach change is a rebuild.** A transaction that changes which VCAs reach an automated lane
    (a membership edit, a VCA added or removed) changes the cell's word count, which no live write
    can do. Decision 14 rule 1 makes it a rebuild: new memory. Draft 10's carry keeps the fader
    cell's event state, preparation seeds the new cell, and D3's adoption jump applies, so the
    rebuild is seamless and completes `exact`. Every other lane keeps #1247's live membership
    (D15-6). README finding F16 records this narrowing of D15-6's premise for automated lanes.
  - For a lane that is not automated, #1247's `FaderDb` records are unchanged.
- **D5. Memory and work.** Each offsets cell holds 4 bytes per reaching VCA in each of #1312's three
  slots, 12 bytes per reaching VCA per automated lane, plus the cell's fixed sequence and index
  words. The reach of one lane is at most the session's VCA count: 256 in the browser, `maximum_vcas`
  on the C ABI. Render adds one `f64` per reaching VCA per event; nothing depends on song length.
- **D6. Out of scope here:** a VCA mute on a member whose mute is automated (draft 13a).
- **D7. The acked-batch question: can an ack ever precede a drop? No.** The commit checks the whole
  transaction before the first cell write, and a cell write cannot fail; a replaced slot is in the
  committed state and counted (#1312 D2).

## Deliverables

1. D3: render's dirty read at block entry and the jump.
2. D4 in the shared classifier and the commit path: the offsets comparison, the cell write and the
   reach-change rebuild.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the strip step's automated-lane branch),
  `crates/host-core/tests/live_delta.rs`, `crates/host-core/tests/vca_live.rs`
- `crates/control-plane/src/` (the offsets-cell write)
- `crates/builtins-compiler/src/lib.rs` (the fader bank's dirty read of the offsets cell and the
  jump only)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/vca_automation_realtime.rs` (new)

`crates/session/src/vca.rs` is called, never changed.

## Non-goals

- VCA automation (out of scope by decision 13, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:129`).
- A VCA mute on an automated mute lane (draft 13a).
- Any change to `vca_effective_db`'s order.

## Hazards

- **A redundant jump moves bits.** D3 compares bits before it jumps; D4 writes only on a change.

## Objective gates

1. **Automated member, both hosts** (`crates/capi/src/runtime/live_tests.rs` and
   `hosts/host-web/src/tests.rs`, new). Track 0 has a linear fader ride and is a member of VCA `v`;
   track 1 is a member with no automation. A transaction (C ABI) and a kind-16 command (browser)
   move `v` by -6 dB on a playing engine: path `live`. From the completion sample of the first grid
   ramp after the jump, the output equals a plan prepared with `v` at -6 dB and the same
   automation, fed the same PCM from frame 0. Track 1 receives exactly one `FaderDb`, as today.
2. **Many moves, one block** (browser, same file). Ten kind-16 moves of `v` before one render: the
   output equals a twin that made only the last. Nested VCAs `v`, `w` reaching track 0: a move of
   `w` writes the slot with `v`'s word unchanged, in VCA-ID order.
3. **No jump for an unchanged offset** (same files). A move of a VCA that does not reach track 0, or
   that moves `v` to its current value, writes no cell and moves no bit.
4. **Reach change** (C ABI, same file). Adding track 0 to a second VCA: path `rebuild`, completes
   `exact`; from the end of the adoption jump the output equals a fresh plan of the new session at
   the same timeline sample. The same membership edit on track 1 stays `live`.
5. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). The move of gate 1 gives one
   offsets-cell write for track 0's lanes and one `FaderDb` per lane of track 1.
6. **Realtime** (`hosts/host-web/tests/vca_automation_realtime.rs`, new integration binary; it links
   `bench_support::alloc` and calls `assert_installed()` first). Gate 1's browser script on the
   render thread: `allocations == 0 && frees == 0` around every render call after warm-up.
7. **Commands:**
   - `cargo test --locked -p session` (unchanged, `vca_composition.rs` included),
     `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
8. **No rendered bit moves** for any session without stored automation, VCA sessions included:
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` gives the same
   `pcm_digest` at base and head (PR evidence), and the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a VCA move does not reach the offsets cell (the lane stays on the old
  composition), if the hosts differ, or if a non-automated member loses its record.
- Gate 2: red if render reads an older slot, the writer writes one word alone (a slot mixes old and
  new offsets), or nested VCAs land in the wrong words.
- Gate 3: red if an unchanged offset still writes the cell or retargets the lane.
- Gate 4: red if a reach change writes a slot of the wrong word count, or rebuilds a lane that is
  not automated.
- Gate 5: red if the classifier emits a `FaderDb` that the next curve event would contradict.
- Gate 6: red if the cell read or the composition allocates on render.
- The composition order and the no-VCA rule are draft 09a's gate 1.

## Dependencies

- Draft 09a *Prepare stored fader automation and render it flat* (the offsets cell it creates and
  seeds). It brings *Admit browser live edits in the Worker through the committed model* (#1382),
  which brings #1247's shared VCA row.
- Draft 10 *Classify fader automation edits as carried rebuilds* (the carry and the strip step's
  automated-lane rule).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the `fader` jump length).
- Batch: R1.
