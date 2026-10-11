# Revision-bounded live cells: design for decision 15 stream B

> **Status (root, 2026-10-05).** Accepted. The adversarial review (`review.md`, beside this file)
> found it SOUND, with findings. Root's rulings supersede three parts of this document:
> - the gate lives in each plan, one gate per plan, not in the mailbox cell (sections 2.6, 3.6,
>   4.1-4.2 and 10.1; spec: the plan-gate slice);
> - the browser renders from its plan's gate like the C ABI, with no `LiveSnapshot::ALL` path
>   (section 4.1; review M1);
> - the revision ceiling is enforced in the protocol session store (sections 4.5 and 10.2).
>
> The issue specs (#1432 Amendment 1, the plan-gate, ceiling and drain slices, and #1312
> Amendment 2) are authoritative. Lemma 1 below carries the review's m1 correction.

Design author's record for root's binding requirement (2026-10-05): a live-edit transaction
committed while render is mid-block must not be torn across blocks, and the applied-revision
watermark's `first_sample` must be exact. This document is input for an adversarial reviewer and
for amendments to #1432, #1312 and #1314. It changes no repository file.

Evidence base: `wt-d15-b` at `65a33e9a4`. Throwaway prototype and loom models in an export of that
tree (`/tmp/claude-1002/d1432/tree`, module `crates/engine/src/realtime/latest_cell.rs`). A copy of
the prototype source is beside this file: `revision-bounded-cells-prototype.rs`. Section 6 gives
the measured mutation results.

---

## 0. Verdict in one screen

- **Chosen encoding: a three-slot, revision-tagged cell behind an announcing revision gate.**
  - Each cell has three slots. Each slot holds the value words, the revision that wrote it and a
    per-cell write sequence.
  - Each mailbox cell's revision word (#1314 D1) becomes a *gate*. Bit 63 is render's
    announcement (`ANNOUNCED`); bits 0..63 hold the revision.
  - Render takes the block's snapshot `S` with one `fetch_or(ANNOUNCED)`. That single
    read-modify-write both reads `S` and tells control "a block is reading at `S`".
  - Control publishes a revision with one `swap`. The swap's return value tells control whether
    render started a block since the last publication, and so which older snapshot render can
    still be reading (the *pin*).
  - A cell write never overwrites two slots: the newest value with a published revision, and the
    newest value at or below the pin. The third slot is always free. So writes never fail, never
    wait and use bounded memory.
  - Render applies, per cell, the slot with the greatest revision `<= S`. Slots it does not choose
    may be mid-write, but it never reads their words.
- **Why the alternatives lose** (section 2):
  - The triple buffer (current #1432 D1), a two-entry cell (candidate (a)) and "publish after all
    writes, check per-cell tags against `S`" (candidate (c)) all fail the same interleaving:
    render takes `S`, then control commits two more transactions that rewrite a cell before
    render drains it. A bounded cell with no reader announcement must then destroy the value
    `<= S`.
  - Moving drains to block start (candidate (b)) shrinks that window but does not close it.
  - So the reader announcement is necessary. It must be the same atomic operation that reads
    `S`; two separate operations fail.
- **Key ordering argument.**
  - Every access to a gate word is a read-modify-write, so the release sequences never break.
    Render's `fetch_or` synchronizes with every earlier control `swap`, and every control `swap`
    synchronizes with every earlier render `fetch_or`.
  - Lemma 1: at any moment, render's open snapshot is either the last published revision or the
    pin control learned at its last swap.
  - So the two protected slots hold every value render can still need. Render reads only the words
    of a protected slot, and those words are stable and visible.
- **Watermark.** Render publishes `(S, block start)` from the same snapshot every drain used. So
  `first_sample` is exact: every value `<= S` is applied at or before this block, and no value
  `> S` is applied. This removes #1314's attempt-record finding that `first_sample` could be one
  block late.
- **Cost.** Render pays one read-modify-write per plan per block, in place of #1314's `Acquire`
  load. An idle lane costs the same as today: one `Relaxed` load per dirty word. A dirty cell costs
  three `Relaxed` tag loads plus the word loads. The triple buffer needed a load plus a
  read-modify-write per dirty cell, so this is cheaper per cell. Control cost is unchanged in
  order.
- **Size.** #1432, rewritten to this design (the primitive, three loom models and the unit tests,
  engine only), fits half a day; the prototype already exists. Integration does not fit in the
  same half day. Two new slices are proposed (section 9):
  - **X1** puts the gate into the mailbox and takes `S` in render (engine).
  - **X2** hands `S` to every live drain (graph, rack and builtins-compiler signatures).
  - #1312 then puts the strip lanes on cells and drains them under `S`.

---

## 1. Contract and vocabulary

- **Revision.** The session's committed revision. A live commit writes cells, then publishes its
  revision as its last write (#1314 D2). A model-only commit publishes with no cell write.
- **Write.** One call of `CellWriter::write`: one value of one cell, tagged with the committing
  transaction's revision `r` and the cell's next sequence number `q` (strictly increasing per
  cell).
- **Gate.** The revision word of one mailbox cell. It is the revision of the plan in that cell,
  plus render's announcement bit.
- **Snapshot `S` of a block.** The revision render reads from the active plan's gate once, at the
  start of the block.
- **Section.** The time from render's `fetch_or` on a gate to its next `fetch_or` on that gate,
  or to its claim of another plan. Every cell read of a block falls inside the section that
  block's snapshot opened.
- **Binding contract** (root, 2026-10-05):
  - C1. Per cell, a block applies the newest write whose revision is `<= S`, if it has not been
    applied already.
  - C2. A block never applies a write whose revision is `> S`. That write waits for a later block.
  - C3. So every live value of one revision takes effect in the same block. That is the first
    block whose snapshot is `>= ` that revision, or the adoption block of the candidate that
    carries it.
  - C4. A write superseded before a covering snapshot is never applied, and it is counted exactly
    once (`live_values_superseded`).
  - C5. Render is wait-free, allocation-free, bounded and retry-free. Control may do more work,
    but a write never fails and never waits.
  - C6. The watermark publishes `(S, first sample of this block)`, and that sample is exact.

---

## 2. Alternatives and why they lose

Each counterexample uses one cell `X` (two cells `X`, `Y` where noted). Control commits
transactions in revision order, writes their cells, then publishes the revision.

### 2.1 The current triple buffer (#1432 D1 as written), read under `S`

The triple buffer keeps one unread value (`middle`) and the writer's private back slot. With a
revision filter, the counterexample is:

1. Before block `k`: `X` holds `x3` (revision 3) in `middle`, marked `FRESH`. Revision 4 (which
   wrote only `Y`) is published.
2. Render: `S = 4`. It drains `Y` (revision 4 applied).
3. Control: writes `x5` into `back`; `middle.swap` makes `middle = x5`, and `back` becomes
   `x3`'s slot. Control publishes 5, then writes `x6` into `back`, overwriting `x3`, and swaps
   it in, so `middle = x6` and `back` holds `x5`.
4. Render drains `X`. `middle` holds `x6` (revision 6 > `S`), and `x3` is gone.
   - If render applies `x6`, it breaks C2. Block `k` then holds revision 6's `X` without
     revision 6's other values, so revision 6 is torn across blocks `k` and `k+1`.
   - If render applies nothing, it breaks C1. Revision 3's `X` value is not in effect at a block
     whose watermark says 4.

No reordering of the triple buffer's operations helps. The value render needs went to the writer's
private slot, and the writer cannot know render needs it.

### 2.2 Candidate (a): a two-entry cell, `(newest <= published, newest overall)`, seqlock tags

Writer rule: on write `(v, r)`, if `pending.rev <= published`, then `stable := pending`; then
`pending := (v, r)`. Counterexample, where render lags two publications inside one block:

1. `X`: `pending = (x3, 3)`; revisions 3 and 4 are published. Render's previous block had `S = 2`.
2. Render: `S = 4`.
3. Control: writes `x5`, so `stable := (x3, 3)` and `pending := (x5, 5)`. It publishes 5.
4. Control: writes `x6`, so `stable := (x5, 5)` (this overwrites `x3`) and `pending := (x6, 6)`.
5. Render drains `X`: `stable` is revision 5 and `pending` is revision 6, both `> S`. The value
   `x3` that C1 requires is gone.

Separately, step 3's copy `stable := pending` races render's read of `stable`. A torn read is
detected by the seqlock, but the reader would have to retry, which C5 forbids. Adding a third
entry only moves the lag needed from two publications to three. No fixed depth survives a reader
the OS preempts for `k` commits.

### 2.3 Candidate (b): drain every lane at one point at block start

This is a placement, not an encoding. The drain pass reads many cells one after another, and
control runs concurrently. The 2.1 interleaving needs only that control commits between render's
snapshot and its read of `X`, and that window exists inside a block-start pass as it does mid-block.
It is shorter, but correctness cannot rest on a window's length. With the chosen encoding, drain
placement does not affect correctness (section 4.4). Drains can stay at their nodes, which is the
lowest-churn choice.

### 2.4 Candidate (c): publish after all writes; render compares per-cell tags with `S`

This is the chosen design's first half: it is necessary, but not sufficient. With per-cell tags and
no reader announcement, the writer does not know which old value render still needs. Render's
snapshot `S` can be any revision it read before control published `k` more, so the writer would
have to keep every value since the last revision it knows render has passed. That needs unbounded
memory, or a write that fails (backpressure, which D15-2 forbids). Any bounded `k`-slot version
fails the 2.2 interleaving with `k` publications during one drain.

### 2.5 A separate announcement word (Dekker style)

Render does `pin.store(PENDING); S = gate.load()`, and control does `gate.store(r);
p = pin.load()`, all `SeqCst`. If control reads `PENDING`, render's load may come before or after
control's store, so `S` is unknown. Control would need to keep every value back to render's
previous pin, which is unbounded again, or re-read until resolved, which is waiting. Only one
read-modify-write that both reads `S` and records the reading closes this. That is the chosen
`fetch_or`.

### 2.6 Others considered

- **A bounded per-cell FIFO of revision-tagged values.** When the writer laps the reader (a paused
  host, or a preempted render), the FIFO either refuses (backpressure, forbidden) or overwrites
  (it can destroy the value `<= S`). Rejected.
- **A seqlock over all of a stage's cells, re-read on a tear.** Render would retry, which C5
  forbids. Rejected.
- **Left-right double banks of all live state.** The writer fills the bank render does not use,
  then flips. It still needs the announcement to know when render left the old bank, a third
  bank when render has not left, and a copy of every unchanged cell on each commit (O(all cells),
  against O(written cells)). Rejected on cost.
- **A gate per plan instead of per mailbox cell.** Also correct. It makes `S` a property of the
  plan (`render_inner` takes it itself), deletes #1314's mailbox revision words and their routing,
  and needs no `LiveSnapshot::ALL` for direct hosts. It costs a rewrite of #1314's landed,
  verified code: I7, `store_revision`, `write_revision`, the withdrawn revision, `RevisionTarget`,
  gates 1 and 8, and the loom models. I chose the mailbox-cell gate, which keeps #1314's routing
  and proofs and changes only the word's encoding and its two operations. Section 10 lists this as
  an owner choice. The cell proof is identical either way.

---

## 3. The state machine

### 3.1 The gate (one `AtomicU64` per mailbox cell)

```
bit 63      : ANNOUNCED   set by render's fetch_or; cleared by control's swap
bits 0..63  : revision    written only by control; GATE_REVISION_MAX = 2^63 - 1
```

| Operation | Thread | Atomic | Ordering | Notes |
|---|---|---|---|---|
| `snapshot()` | render, once per block | `fetch_or(ANNOUNCED)` | `AcqRel` | `S = old & REVISION_MASK` |
| `publish(pin, r)` | control, the last write of a commit | `swap(r)` | `AcqRel` | if `old & ANNOUNCED`: `pin := Some(old & MASK)`; else pin unchanged |
| `rearm(pin, r)` | control, publication into an `Empty` cell | `swap(r)` | `AcqRel` | `pin := None`; only for a gate render no longer reads (3.6) |
| withdraw's revision | control, `Full` cell | `load` | `Relaxed` | render never touches a `Full` cell's gate (#1314 I7) |

**Rule G: every write to a gate word is a read-modify-write.** This matters in three ways:

1. A release sequence (C++20 `[intro.races]`) is the release operation plus every later
   read-modify-write on the location. With only read-modify-writes on the gate, each render
   `fetch_or` (acquire) synchronizes with every earlier control `swap` (release), and each control
   `swap` (acquire) synchronizes with every earlier render `fetch_or` (release). A plain store
   would end those sequences.
2. A read-modify-write always reads the last value in modification order. That is what makes
   Lemma 1's case analysis total.
3. Loom 0.7.2 handles a plain store racing a read-modify-write too permissively. A probe model,
   where a thread does `store(1); store(2)` racing `fetch_or`, then joins, then does `fetch_or`
   again, let the final read return the first `fetch_or`'s write. The C++/Rust model forbids that
   (`[atomics.order]`: a read-modify-write reads the immediately preceding value in modification
   order, and write-read coherence holds through the `join`). Models that mix a plain store into a
   gate race therefore report spurious failures. Rule G avoids the case entirely.

### 3.2 Control-side pin and stamps

- `GatePin { pinned: Option<u64> }` is private to control, one per gate (per mailbox cell). It is
  reset to `None` by `rearm`, and updated by `publish` as in the table above.
- `LiveStamp { revision: u64, pinned: Option<u64> }` is passed to every cell write of one
  transaction.
  - `PlanPublisher::live_stamp(revision) -> (LiveStamp, GateId)` routes like
    `PlanPublisher::set_revision`: the `Full` cell if there is one, else the `Active` cell. It
    returns that cell's pin.
  - `set_revision` also returns its `GateId`. Control debug-asserts that the two agree.
  - A concurrent claim does not change the cell; it only turns `Full` into `Active`.
  - `LiveStamp::unpinned(revision)` (`pinned: None`) is for a plan no concurrent reader holds a
    snapshot of. That is a plan render has not adopted (a fresh successor before publication,
    #1277 D5 retargets, a reserved or withdrawn candidate), or a plan rendered on the writing
    thread (the browser until #1382).
- **Stamp revision.** This is the revision the transaction commits as (the prepared token's
  prospective revision). A write before a plan's first publication carries a revision `<=` the
  revision the plan is published with (#1277's retargets carry the structural revision).
  Per cell, stamp revisions are non-decreasing; the writer debug-asserts this.

### 3.3 The cell

```
LatestCell<N>   (shared, Sync; allocated on the control thread)
  slots[3]: { words: [AtomicU32; N], revision: AtomicU64, sequence: AtomicU64 }
            revision == UNWRITTEN (u64::MAX) until the slot is first written
CellWriter<N>   (control-private)
  revisions[3]: u64       mirror of the slot tags (only the writer stores them)
  next_sequence: u64      starts at 1
CellReader      (render-private)
  last_applied: u64       sequence of the last applied write; 0 = none
DirtyWord       (shared, AtomicU64; one bit per cell, shared by a stage's cells)
DirtyReader     (render-private)
  deferred: u64           cells whose newest write was above the last snapshot
```

**Write `(stamp, words)` on cell `c` with dirty bit `b`** (control; never fails, never waits):

1. Choose the target slot:
   - if a slot's mirrored revision equals `stamp.revision`, rewrite that slot in place, so one
     revision occupies at most one slot;
   - otherwise let `A` be the slot with the greatest mirrored revision `< stamp.revision` (the
     newest published value), and `B` the slot with the greatest mirrored revision
     `<= stamp.pinned` (if `pinned` is `Some`);
   - choose any slot that is neither `A` nor `B`. It exists, because there are three slots.
2. Store the words, `sequence := next_sequence++` and `revision := stamp.revision`, all
   `Relaxed`, in any order. The prototype's `tag_first` run shows the order is free (section 6).
   Update the mirror.
3. `dirty.fetch_or(1 << b, Release)`. This must follow step 2 (mutation `dirty_before_slot`).

**Block drain of a stage** (render), after the block's snapshot `S`:

1. `pending := deferred | take(dirty)`, where `take` is: if `dirty.load(Relaxed) != 0` then
   `dirty.swap(0, Acquire)`, else 0. An idle stage pays one `Relaxed` load. `deferred := 0`.
2. For each set bit, in the stage's canonical cell order, read the cell under `S`:
   1. Load the three slot revisions (`Relaxed`). Choose the slot with the greatest revision
      `<= S`, skipping `UNWRITTEN`. Note `newer := any revision > S` (not `UNWRITTEN`).
   2. If a slot was chosen, load its `sequence` (`Relaxed`). If `sequence > last_applied`, load
      its words (`Relaxed`), report `superseded = sequence - last_applied - 1` (saturating) and set
      `last_applied := sequence`. Otherwise the read is `Unchanged`.
   3. If `newer`, set the bit in `deferred`.
3. The stage applies each `Applied` read in its canonical order (#1312 D4, #1345 D3, #1346 D2) and
   adds `superseded` to the session counter (#1312 D2).

`take` must come after the snapshot in program order. If it came before, a write `<= S` whose bit
is set between the take and the snapshot would wait a block, which breaks C1 (mutation
`dirty_before_snapshot`).

### 3.4 Why it is correct

The proofs below fix one plan `P` in one mailbox cell (one *epoch* of that gate). Let `w_0` be
the `rearm` that started the epoch, `w_1, w_2, ...` control's later swaps on the gate, and
`f_1, f_2, ...` render's `fetch_or`s on it.

**Lemma 0 (render only reads `P`'s cells inside sections of `P`'s gate, or quiescent).**

- Render reads a plan's cells only in its blocks (after that block's `fetch_or`), and in the carry
  when the plan is a predecessor.
- At that point control has stopped writing the predecessor's cells. Control writes only the
  newest plan (#1053 D7), and the successor's publication (`Release` compare-and-swap on the
  mailbox word) happened-before render's claim (`Acquire`).

**Lemma 1 (which snapshots a concurrent read can belong to).** Let control write a slot of `P`
at a point between `w_j` and `w_{j+1}` in its program, while writing transaction `j+1`'s cells.
Let `pin_j` be control's pin after `w_j`. Let `x` be any render read of `P`'s cells that does not
happen-before that write, and `f` the `fetch_or` that opened `x`'s section, with snapshot `S_f`,
and suppose the write's revision is above `S_f`, which is the only case the proof needs. Then
`S_f in {rev(w_j), pin_j}`.

(Review m1: without that restriction, the hypothesis also admits sections opened by an `f` that
reads `w_{j+1}` or later. There the write happens-before `x`, and its revision is `<= S_f`, so
it is `N` or older and harmless. The dirty word's synchronization cannot place `f` after
`w_{j+1}` for a write whose revision is above `S_f`.)

*Proof.*

- If `f` follows `w_j` in the gate's modification order, it reads `w_j`'s value, because only
  render's own `fetch_or`s, which set bit 63 only, lie between them. So `S_f = rev(w_j)`.
- Otherwise `f` precedes `w_j`. No other render `fetch_or` `f'` lies between `f` and `w_j`. If one
  did, `x` would be sequenced before `f'` (render's read-modify-writes on the gate follow its
  program order), and `f'` is a release whose release sequence (Rule G) contains the value `w_j`
  acquires. So `x` would happen-before `w_j`, and so before the write, which contradicts the
  choice of `x`.
- So `f` is render's last `fetch_or` before `w_j`. Let `w_m` (`m <= j`) be the first swap after
  `f`.
  - `w_m` reads `ANNOUNCED` with revision `rev(w_{m-1}) = S_f`, since only swaps clear the bit.
    So `pin_m = S_f`.
  - Each later `w_l` (`m < l <= j`) reads no `ANNOUNCED` and keeps the pin. Hence
    `pin_j = S_f`.
- `f` cannot precede `w_0`: render claims the plan after the publication compare-and-swap, which
  follows `rearm` in control's program and synchronizes with the claim. ∎

**Lemma 2 (the needed slot is protected, visible and stable).** During render's section with
snapshot `S_R`, let `N` be the cell's newest write with revision `<= S_R`.

- *Protected.* By Lemma 1, `S_R` is either the newest published revision, in which case `N` is
  `A` (every write with revision `<` the in-flight one is published), or the pin, in which case
  `N` is `B`. The write rule never targets `A` or `B`.
  - `A` and `B` are computed from control's mirror, which is exact (control is the only writer).
- *Visible.* `N`'s stores precede, in control's program, the swap that published `rev(N)`, and
  that swap is at or before the swap whose value the section's `fetch_or` read (which published
  `S_R >= rev(N)`). By Rule G that swap heads a release sequence that contains the value the
  `fetch_or` read. So `N`'s stores happen-before every read in the section.
- *Stable.* Lemma 1 applies to every control write that a read of this section does not
  happen-before. At each such write, `N` is `A` or `B`, so control never targets it. Every other
  control write to `N`'s slot happens after the section's reads. ∎

**Lemma 3 (render chooses `N`).**

- Render reads `N`'s revision exactly: its last write happens-before the read and no later write
  exists in the section, so write-read coherence fixes the value.
- Any value render can read from another slot `X` was written to `X` at some time. Its revision is
  not in `(rev(N), S_R]`, because a write with such a revision would be newer than `N` with a
  revision `<= S_R`, against the choice of `N`. Its revision is not `rev(N)` either, because one
  revision occupies one slot (in-place rule).
- This holds for stale reads and for a slot mid-rewrite: the new revision is `> S_R`, and the old
  one is one of `X`'s earlier values.
- So the maximum over revisions `<= S_R` is `N`'s, and render reads only `N`'s words. ∎

**Theorems.**

- **T1. No torn value.** Render loads the words of `N` only, which are visible and stable
  (Lemma 2).
- **T2. Nothing newer than `S`.** Render chooses only revisions `<= S` (C2).
- **T3. The newest `<= S` is always applied (no skip).** Let `v` be a write `<= S` not yet
  applied.
  - Its dirty `fetch_or` happens-before the publishing swap, which happens-before the block's
    snapshot, which precedes `take`. So `take`'s load returns that bit or a later value of the
    word (coherence).
  - The exception is a `swap(0)` of an earlier block that consumed the bit. That swap acquired
    `v`'s stores. So that block either applied `v` (`<= ` its `S`) or saw `rev(v) >` its `S` and
    deferred the cell.
  - Either way the cell is scanned in this block, and Lemma 3 picks `N`, which is `v` or newer
    (C1).
- **T4. At most once.** Render applies only when `sequence > last_applied`, then raises
  `last_applied`. Later snapshots are `>=` this one, so their `N` has a sequence `>=`.
- **T5. Supersession exact.** Each write `v` is resolved exactly once, at the first block whose
  snapshot covers `rev(v)`.
  - If it is that cell's `N`, it is applied.
  - Otherwise a newer write `w` with `rev(w) <= S` exists, and when render applies `N` it adds
    `seq(N) - last_applied - 1`, which counts `v` once.
  - Writes not yet covered are not counted. Values on a predecessor are resolved by the carry
    (section 5). This gives C4.
- **T6. A value `> S` is neither applied nor lost.** It sits in `A` or in the in-flight slot.
  Control's next target excludes `A`. Its bit is either still set or in `deferred`. The next
  snapshot `>=` its revision applies it or supersedes it.
  - *Acked-batch question: can an ack ever precede a drop? No.* Control overwrites only slots
    outside `{A, B}`. Their values are never `N` for any snapshot render holds or will take: later
    snapshots are `>=` the published revision, whose `N` is `A` or newer. So every overwritten
    value is superseded, and counted by T5.
- **T7. The writer laps the reader without failing.** Each write takes O(1) time and touches no
  shared state but its own slot and the dirty bit. Memory is three slots per cell, fixed at
  construction.
  - For a paused host, render's pin stays at its last snapshot. Control alternates between the two
    slots that are not `B`, `A` follows the newest, and nothing grows.
  - Prototype unit test `a_paused_reader_is_lapped_without_failure`: 9,999 commits, then one
    block applies revision 10,000 and reports 9,998 superseded.
- **T8. Render is bounded and wait-free.** Per block: one `fetch_or` per active plan, one load
  (and at most one swap) per dirty word, and for each pending cell three loads, one sequence load
  and `N` word loads. Nothing loops on a shared value; there is no retry and no spin.

### 3.5 Orderings, one table

| Access | Ordering | Why |
|---|---|---|
| gate `swap` (control) | `AcqRel` | release: publish cell stores; acquire: see render's section end |
| gate `fetch_or` (render) | `AcqRel` | acquire: see cell stores up to `S`; release: end the previous section's reads |
| slot words, `sequence`, `revision` stores | `Relaxed` | visibility comes from the gate (Lemma 2) |
| slot loads | `Relaxed` | same |
| dirty `fetch_or` (control) | `Release` | a block that takes the bit sees the slot's revision (T6) |
| dirty pre-check `load` (render) | `Relaxed` | coherence with the gate's happens-before is enough (T3) |
| dirty `swap(0)` (render) | `Acquire` | sees the slot written before the bit (T6) |

### 3.6 The mailbox's gates across epochs

- A mailbox cell's gate is reused by successive plans. `rearm` (a swap, Rule G) writes a new
  candidate's revision into the `Empty` cell before the publishing compare-and-swap, and resets
  that cell's pin to `None`.
- Render's last `fetch_or` on that cell, made for the plan that left it, happens-before the
  `rearm`: render's claim of the other cell is a release, which control's `try_reserve` acquires.
- Render touches the cell again only after it claims the new candidate, which acquires the
  publication.
- A `Full` cell's gate is touched only by control: render's `active` index never points at a
  `Full` cell (#1314 I7). `withdraw` reads its revision with a plain load.
- `republish` rearms whichever cell is `Empty` with the candidate's revision. A withdrawn
  candidate was never claimed, so render never read its cells, and `pinned = None` is exact.

---

## 4. How render obtains `S`, and the exact watermark

### 4.1 Where `S` is taken

- `RealtimePlanOwner::{render, render_contiguous}`: after `enter_block` (the claim decision, the
  clock adoption and `carry_from`) and before `render_inner`, render calls
  `self.publication.snapshot()`, which is a `fetch_or` on the gate of render's `active` cell.
  - This replaces `active_revision()`'s `Acquire` load at the same place.
  - `render_inner(io, time, S)` validates the shape and the clock, then calls
    `PreparedPlanExecutor::begin_live_block(S)` (a new hook, default no-op), then
    `executor.render`.
- The graph executor stores `S` and hands it to every drain of the block (section 4.4). The carry
  inside `enter_block` reads only the quiescent predecessor (section 5), so it needs no `S`.
- *Hazard for #1355:* a prime that runs successor drains in the adoption block must run them
  under that block's `S`. If it runs inside `enter_block`, take the snapshot right after the claim,
  before the prime. Today no prime drains a live lane.
- **Direct render** (`PreparedRenderPlan::render`/`render_contiguous`, with no exchange) passes
  `LiveSnapshot::ALL` (`GATE_REVISION_MAX`): it applies every written value.
  - This is exact only when no control write can run concurrently: tests, the audit tools, and
    the browser until #1382 moves its live edits to the Worker behind #1381's exchange.
  - The browser stamps each admitted batch with a host-local, strictly increasing batch number,
    `LiveStamp::unpinned`, until #1382 brings the committed revision.

### 4.2 The pending candidate (#1343, #1311, #1053 D7)

- Every revision committed while a candidate is pending goes to that candidate: its cell writes
  (stamp from the `Full` cell, pin `None`) and its gate (#1314 D2, unchanged). The running plan's
  gate does not move, so its blocks keep their `S` and apply nothing new.
- At the adoption block render claims the candidate (#1311: due and ready, in one step with the
  claim), carries, then takes `S` from the candidate's gate.
  - `S >=` the candidate's publication revision, so every retarget (#1277 D5) and every live edit
    published to the candidate before that `fetch_or` applies in the adoption block.
  - A live commit to the candidate that is still in flight has revision `> S` and applies at the
    next block. Its revision is then published at that block, which is exact too.
- A withdrawn (held) candidate is written with `LiveStamp::unpinned` and
  `UnadoptedCandidate::set_revision`. `republish` rearms its new cell. Supersession (#1310) and
  the fallback (#1358, #1397) publish fresh plans, prepared and rearmed the same way.
- `PlanAdoption::{NoEarlierThan, Primed}` only delay the claim; nothing above depends on when it
  happens.

### 4.3 Why `first_sample` is exact

- Render publishes `advance(S, block start)` with the `S` every drain of that block used, after
  `render_inner` returns `Ok` (#1314 D3's placement is unchanged).
- If `S` is above the published revision `W`, then:
  1. every value with revision `<= S` is applied at or before this block (C1, T3);
  2. no block before this one applied a value with revision `> W`, because each earlier block
     applied only values `<=` its own `S'`, and every `S' <= W` (the watermark is the greatest
     snapshot of a successful block, and blocks that error are handled below);
  3. so revision `S` (and every revision in `(W, S]`) is first wholly in effect at this block's
     first sample.
- Model-only revisions satisfy this trivially. The adoption block covers every predecessor
  revision applied by the carry (section 5).
- **Blocks that error.**
  - Shape, clock and source refusals all happen before any drain (`render_inner`'s checks,
    `source_set.begin_block`), so such a block applies nothing and publishes nothing. That is
    exact.
  - The executor's later errors (`InvalidEnvelope`, `Buffer`) are invariant failures that no
    valid plan reaches. If one happened after some drains, those values entered the render state
    in a block that rendered no output.
    - For `render_contiguous` the clock does not advance, so the next block starts at the same
      sample and publishes the same `first_sample`. It is exact.
    - For `render` with explicit host time, a host that skips past the failed block sees the
      watermark name the next rendered block.
  - This is recorded as a defect path, not handled with machinery.
- **Lane families still on queues.** The contract holds for every lane held in cells. Until #1345
  (effect lanes), a queue drain can pop a record of revision `> S` and apply it early. The
  watermark is then late for that record, never early. #1346 and #1347 put the input and route
  lanes on cells before the C ABI attaches them (#1261, #1225), so on the C ABI the contract is
  whole once #1312 and #1345 have landed. This order is forced by the dependencies; it is not a
  workaround.

### 4.4 Drain placement (candidate (b) as a delivery choice)

- Correctness does not depend on where drains run within a block: every read in a section is
  protected (Lemma 2). So drains stay at their stage's per-block entry:
  - the bank stage `begin_block` (`rack::BankStage`, `GraphPreparedBuiltinBankProcessor`);
  - the fader and matrix banks, whose drains move from `process` to `begin_block`, which that
    trait's doc already names as the drain point;
  - `GraphBindingBlock` for per-node live processors;
  - the route op's drain.
- `S` reaches them as an explicit argument. A block-start pass over every live owner was
  considered. It is equally correct, and it would also make D11's silence-skip hazard structural,
  but it moves every drain and adds a registration list. The explicit argument is the smaller
  change.

### 4.5 The revision ceiling

Bit 63 is the announcement, so a gate holds revisions up to `2^63 - 1`. The session store's
revision ceiling becomes `GATE_REVISION_MAX`, and a commit above it is refused with
`RevisionExhausted` before any write (D15-2 condition 3). No session can reach it, but the
boundary is stated and tested instead of left to overflow.

---

## 5. The carry under `S` (#1277, #1280)

- **Quiescence (Q).** Control writes only the newest plan's cells (#1053 D7). Every write to the
  predecessor happened before the successor's publication, which render's claim acquired. So
  during the carry the predecessor's cells are stable and wholly visible.
- **Order (R).** Every predecessor write was published to the predecessor's gate with a revision
  below the successor's publication revision. Every revision committed while the successor was
  pending went to the successor (#1314 D2). So every predecessor value is `<=` the adoption block's
  `S`.
- **`apply_pending(&mut self)`** (#1312 D6) is the stage's block drain with `LiveSnapshot::ALL`. By
  (Q) and (R), that equals "the newest `<= S`" for the predecessor, so the predecessor's unapplied
  values take effect in the adoption block, at its first sample, as the watermark reports.
- **`peek_unread(&self, cell) -> Option<(words, sequence)>`** returns the written slot with the
  greatest sequence if that sequence is `> last_applied`, and changes nothing: no load is a
  read-modify-write, and `last_applied` and the dirty word stay as they were. It is valid only
  under (Q). It needs no `S`, by (R). `last_applied(&self) -> u64` stays.
- **#1280 D2's resolution under `S`.** The successor's own cell wins only if its read under the
  adoption block's `S` is `Applied` (a newest-`<= S` value that is unread). Otherwise the carried
  value applies, and a successor value `> S` stays pending for a later block.
  - The successor's values are all newer than the predecessor's, by publication order.
  - The counting rule is unchanged: add `n` when the successor's own value wins and `n - 1` when
    the carried value applies, and the successor's own reads count their own gaps.

---

## 6. Loom models and unit tests

### 6.1 #1432's models (`spsc_loom_cells_*` in `latest_cell.rs`; the CI filter `spsc_loom` runs them)

All three are prototyped. Release build, loom 0.7.2: about 0.15 s of model time for the three,
plus compilation. L2 also passes with no preemption bound (0.07 s).

- **L1 `spsc_loom_cells_one_block_never_tears`.** Two cells, one gate.
  - Control: for `r` in 1..=2, write `X(r)`, write `Y(r)`, publish `r`.
  - Render: one block (snapshot, then drain both). It asserts both cells hold `S`'s value (or
    neither, if `S = 0`). After join, one block at `S = 2` asserts both cells hold 2, and
    `superseded + applications == 4`.
  - Reaches: `S` taken between transactions; a cell drained after control's next commit.
- **L2 `spsc_loom_cells_a_pinned_reader_survives_a_lapping_writer`.** One cell.
  - Before the threads: control writes `X(1)` and publishes 1.
  - Control: writes and publishes 2, 3 and 4. Render: one block asserts it applied exactly
    revision `S`, then a final block asserts 4 and the exact count.
  - Reaches: render pinned at 1 while control laps three times. The third write needs the pin
    learned at publication 2 and kept through publication 3, with no new announcement.
- **L3 `spsc_loom_cells_a_dirty_bit_is_never_lost`.** One cell; control writes and publishes 1 and
  2. Render runs two blocks, then one after join.
  - It asserts each block applied `S`'s value, the final value is 2, and the count is exact.
  - Reaches: a block whose `swap(0)` takes the bit of a write whose revision is `> S`.

**Measured mutation results** (prototype; each mutation applied alone; `red` means the model
fails):

| Mutation | L1 | L2 | L3 | Unit |
|---|---|---|---|---|
| reader ignores `S` (newest overall, the triple-buffer behaviour) | red | red | red | |
| reader loads the gate (no announcement) | green | red | green | |
| control drops the pin on a swap without `ANNOUNCED` | green | red | green | |
| control protects only `A` | green | red | green | |
| gate operations `Relaxed` | red | green | red | |
| dirty bit set before the slot stores | red | red | red | |
| reader never defers a cell whose newest is `> S` | red | red | red | |
| dirty `swap(0)` `Relaxed` | red | red | red | |
| dirty take before the snapshot | red | green | red | |
| control's publication a plain `store` | red* | red* | red* | |
| a fresh slot per write within one revision (no in-place rule) | green | green | green | red (`rewrite_in_one_revision_applies_the_last`) |
| slot tag stored before the words (claim: store order is free) | green | green | green | |

(*) Part of this red comes from loom's store/read-modify-write imprecision (3.1, point 3). The
defect it stands for, a control side that never learns the pin, is caught cleanly by the
"protects only `A`" and "drops the pin" rows.

### 6.2 #1432's unit tests (prototyped, green)

- `peek_is_pure_and_read_counts`: three writes and no read. `peek_unread` returns the third
  write's words and sequence, and a second peek returns the same. A read at `S = 3` then applies
  it with 2 superseded, and a third peek returns `None`.
- `rewrite_in_one_revision_applies_the_last`: two writes in revision 1. The read applies the
  second, with 1 superseded.
- `a_paused_reader_is_lapped_without_failure`: 9,999 commits after a read. One read applies
  10,000 with 9,998 superseded.
- To add: a write whose revision is below the cell's newest written revision panics in debug, and
  `LiveSnapshot::ALL` applies the newest write.

### 6.3 X1's models (in `spsc.rs`, with the mailbox; described, not prototyped)

- **L4 `spsc_loom_plan_mailbox_gate_snapshot_follows_the_claim`.** Initial plan `P0` in cell 0
  with one cell written at revision 1.
  - Control publishes candidate `A` (cell 1, revision 7, its cell written at 7), then commits 8 to
    `A` (stamp from the `Full` cell, write, `set_revision`).
  - Render runs two blocks: claim, then `snapshot()`, then drain `A`'s cell.
  - Assert: after the claim, the applied value's revision equals `S`, and `S` is 7 or 8. `P0`'s
    blocks never apply 7 or 8.
  - Mutations, each red:
    - render keeps snapshotting the cell it ran before the claim (a stale `active`): `S = 1`, so
      `A`'s cell is never applied;
    - `set_revision` swaps the `Active` cell's gate while a cell is `Full` (#1314's routing
      mutation, now with a cell read behind it).
- **Gate reuse across epochs needs no model of its own.** A pin left over from a cell's previous
  plan is the revision of a value older than anything the new plan's cells hold. Keeping it
  protects nothing, and dropping it loses nothing. So the only epoch property is 3.6's
  happens-before from render's last `fetch_or` on the old plan to `rearm`, which the claim and
  `try_reserve` already give. That is covered by #1343's claim-race models with `rearm` as a swap.
- The existing #1314 loom models keep their assertions: `store` becomes `swap`, and the load
  becomes `fetch_or`.

### 6.4 #1312's end-to-end gates (section 8.2)

A deterministic two-strip transaction test, and the converse of #1314's race test.

---

## 7. Cost

Render, per block:

| Item | This design | Triple buffer (old #1432 D1) | Today (queues) |
|---|---|---|---|
| per active plan | 1 `fetch_or` `AcqRel` (gate) | 1 `Acquire` load (#1314) | 1 `Acquire` load |
| per idle dirty word (one per strip stage, #1312 D3) | 1 `Relaxed` load | 1 load + 1 swap (`dirty.swap(0)` unconditional) | 1 `Acquire` load per lane (`available_at_entry`) |
| per dirty word with work | + 1 `swap(0)` `Acquire` | 1 `swap(0)` | n/a |
| per pending cell | 3 `Relaxed` tag loads + 1 sequence load + `N` word loads | 1 `middle` load + 1 `middle.swap` `AcqRel` + `N` + 1 loads | per record: pop + apply |
| per deferred cell (rare: a commit in flight) | rescan next block | n/a | n/a |

- A `fetch_or` costs one uncontended read-modify-write per block per plan, about 20 cycles (x86
  `lock or`, AArch64 LSE `ldsetal`, wasm `i64.atomic.rmw.or`). It contends only with a commit's
  swap in the same instant.
- At 48 kHz and a quantum of 128, that is 375 read-modify-writes per second, which is negligible.
- A dirty cell is cheaper than in the triple buffer: three loads in place of a load and a
  read-modify-write.
- **Banked strips and SIMD.** Drains are per lane and scalar, as today. They retarget ramp state
  (`set_fader_db`, `set_target_smoothed`), and the SIMD kernels read that state per sample. No
  per-sample cost changes. Idle banks pay one `Relaxed` load per strip stage, the same count as
  today's `available_at_entry`.
  - A per-bank dirty word (64 cells) would cut this to one load per bank stage. That couples
    strips' producers at preparation, so it is a possible later optimisation, not part of these
    slices.

Control:

- Per write: choose a slot from three mirrored tags, `N + 2` `Relaxed` stores and one `Release`
  `fetch_or`. The triple buffer used `N + 1` stores, one `AcqRel` swap and one `fetch_or`.
- Per commit: one `swap` on the gate (it was a store), one stamp per transaction and a pin update.

Memory, per cell:

- Shared: `3 × (4N + 16)` bytes before padding. That is 72 B for a fader cell (`N = 2`), 108 B
  for a matrix cell (`N = 5`) and 192 B for an EQ target (`N = 12`).
- Control mirror: 32 B. Render: 8 B.
- The triple buffer was `3 × (4N + 8) + 4`, so this adds about 20 B per cell for the revision
  tags.

---

## 8. Spec amendments (drop-in text)

### 8.1 #1432: replacement text

**Title** (unchanged): *Add the latest-target cell primitive and its loom model.*

**Product outcome** (replace the paragraph):

> The engine has a revision-bounded latest-target cell and a revision gate: safe-Rust,
> allocation-free primitives that one control thread writes and render reads. Control publishes
> each committed revision through the gate after the transaction's cell writes. Render takes one
> snapshot `S` of the gate per block, and every read in that block applies, per cell, the newest
> value whose revision is `<= S`: never a newer one, never a torn one, never one twice, and never
> skipping it. So every value of one revision takes effect in one block. A write cannot fail and
> never waits, however far control laps render. The reader reports how many writes a later write
> replaced before any snapshot covered them. A carry can peek an unread value without consuming
> it. #1312 puts the strip fader, mute and matrix lanes on these primitives; #1345, #1346 and #1347
> reuse them; X1 puts the gate into the plan mailbox.

**D1** (replace):

> - **D1. The gate, the cell and their orderings** (design: `revision-bounded-cells-design.md`
>   sections 3.1-3.5). New module `crates/engine/src/realtime/latest_cell.rs`, safe Rust (atomics
>   only, no `unsafe`).
>   - **Gate.** `RevisionGate` is one `AtomicU64`, embeddable (no `Arc`). Bit 63 is `ANNOUNCED`;
>     bits 0..63 are the revision (`GATE_REVISION_MAX = 2^63 - 1`).
>     - `snapshot(&self) -> LiveSnapshot` (render): `fetch_or(ANNOUNCED, AcqRel)`, returning the
>       revision bits.
>     - `publish(&self, &mut GatePin, revision)` (control): `swap(revision, AcqRel)`. If the old
>       word had `ANNOUNCED`, the pin becomes `Some(old revision)`; otherwise it is unchanged.
>     - `rearm(&self, &mut GatePin, revision)` (control): the same swap, with the pin reset to
>       `None`. It is only for a gate render no longer reads.
>     - Every write to the gate word is a read-modify-write.
>     - `GatePin` is control-private. `GatePin::stamp(revision) -> LiveStamp { revision,
>       pinned }`. `LiveStamp::unpinned(revision)` is for a plan no concurrent reader holds a
>       snapshot of. `LiveSnapshot::ALL` is `GATE_REVISION_MAX`.
>   - **Cell.** `LatestCell<const N: usize>` has three slots. Each slot holds `N` `AtomicU32`
>     words (the target and its ramp, one unit), an `AtomicU64` revision (`u64::MAX` until
>     written) and an `AtomicU64` sequence.
>     - `CellWriter<N>` is control-private: a mirror of the three revisions and the next sequence.
>     - `CellReader` is render-private: the last applied sequence.
>     - `DirtyWord` is one `AtomicU64` that a stage shares among its cells, one bit per cell.
>       `DirtyReader` is render-private and holds a deferred mask.
>   - **Write** `(stamp, words)`, which cannot fail:
>     - Target the slot already holding `stamp.revision`, if any. Otherwise target a slot that is
>       neither the slot with the greatest revision `< stamp.revision` nor the slot with the
>       greatest revision `<= stamp.pinned`.
>     - Store the words, the next sequence and the revision (`Relaxed`), then
>       `dirty.fetch_or(bit, Release)`.
>     - Stamp revisions per cell are non-decreasing (debug-asserted).
>   - **Read** (render, after the block's snapshot, in the stage's canonical order):
>     - The pending set is the deferred mask, plus `dirty.swap(0, Acquire)` if
>       `dirty.load(Relaxed) != 0`. Every take of a block follows its snapshot.
>     - Per pending cell: load the three revisions (`Relaxed`) and choose the greatest `<= S`. If
>       its sequence is above the last applied, load its words and report `Applied { words,
>       sequence, superseded }`; otherwise report `Unchanged`.
>     - Any revision `> S` keeps the cell in the deferred mask.
>     - Render never loads the words of a slot it did not choose.
>   - **Peek** (carry only): `peek_unread(&self) -> Option<(words, sequence)>` returns the written
>     slot with the greatest sequence when that sequence is above the last applied, and `None`
>     otherwise. It changes nothing: no read-modify-write, the last applied sequence, the dirty
>     word and the deferred mask stay as they were. It is valid only once the writer is quiescent
>     (no write to this cell can start before the peek ends). `last_applied(&self) -> u64` stays.

**D2** (replace):

> - **D2. The supersession count** (#1312 D2's unit). A *write* is one `CellWriter::write` call. A
>   block *covers* a write when the block's snapshot is `>=` the write's revision. At the first
>   block that covers it, a write is either applied (it is the cell's newest write `<= S`) or
>   superseded.
>   - When render applies sequence `s` after `p`, the read reports `s - p - 1` (saturating) for
>     that cell. A write not yet covered is not counted.
>   - The caller adds the count to its counter; this module holds no session counter.

**D3** (keep, append): "`latest_cell.rs` also exports `RevisionGate`, `GatePin`, `LiveStamp`,
`LiveSnapshot` and `GATE_REVISION_MAX`. Putting the gate into the plan mailbox is X1's."

**Authorized paths** (unchanged): `latest_cell.rs` (new); `mod.rs` (exports); `spsc.rs` only if
the loom shim needs a type it lacks. A local `cfg(loom)` shim in `latest_cell.rs` is allowed.

**Hazards** (replace):

> - A reader that takes `S` with a plain load, or a gate written with a plain store, loses the
>   announcement or the release sequence. Control can then overwrite the slot render's snapshot
>   needs (loom L2 red).
> - A reader that chooses the newest slot regardless of `S` reintroduces the torn transaction (L1
>   red).
> - Setting the dirty bit before the slot stores, or not deferring a cell whose newest revision is
>   `> S`, loses a committed value forever (L3 red).
> - Taking the dirty bits before the snapshot delays a covered value by one block (L1 red).
> - A peek that swaps or clears anything consumes the value: #1280's carry relies on a peek that
>   changes nothing.
> - Loom 0.7.2 handles a plain store racing a read-modify-write too permissively, so no model
>   races a plain store against the gate.

**Objective gates** (replace gate 1; gates 2 and 3 unchanged):

> 1. **Revision-bounded, never torn, never lost (loom, `spsc_loom_cells_*` in `latest_cell.rs`).**
>    - (L1) Two cells, two transactions writing both, racing one block: the block applies both
>      cells at `S` or neither, never one.
>    - (L2) A reader pinned at revision 1 while the writer commits 2, 3 and 4 applies exactly
>      revision `S`'s words.
>    - (L3) A write committed while a block drains is applied by a later block.
>    - In each model, after a final quiescent block, `superseded + applications == writes`.
>    - Command: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)'
>      cargo test --locked --release -p engine --lib spsc_loom`.
>    - Unit tests in the same module:
>      - three writes and no read: `peek_unread` returns the third write's words and sequence
>        twice, and a read at `S = 3` applies it with 2 superseded;
>      - two writes in one revision: the read applies the second, with 1 superseded;
>      - 9,999 commits after one read: one read applies the last, with 9,998 superseded;
>      - a write below the cell's newest revision panics in debug.

**Test value** (replace):

> - Gate 1 (L1): a reader that ignores `S` (the triple buffer), a dirty take before the snapshot,
>   or orderings weakened to `Relaxed`.
> - L2: a reader without the announcement, or a writer that protects only the newest published
>   slot or forgets the pin.
> - L3: a dirty bit set before the slot stores, a `Relaxed` take, or a reader that does not defer.
> - The in-place unit test: a writer that puts two writes of one revision into two slots, so the
>   reader can apply the older.
> - The peek test: a peek that consumes.

### 8.2 #1312: affected decisions and gates

**Non-goals**: delete "Per-transaction atomicity across cells: as today, one transaction's values
may land one block apart." Add: "Atomicity for lanes still on queues (effect lanes until #1345)."

**D1** (replace):

> - **D1. The cell** is #1432's D1 (Amendment 2): a revision-bounded three-slot cell behind the
>   plan's revision gate. A block applies, per cell, the newest value whose revision is `<=` the
>   block's snapshot `S`. So every value of one committed revision takes effect in the same
>   block: the first block whose snapshot covers it, or, while a successor is pending, the
>   adoption block. That is D15-2 condition 2 and D15-2's ack meaning as amended (D9). This slice
>   builds the strip stages' cells and dirty words on it (D3).

**D2** (replace the first two sentences):

> - **D2. One counter unit: writes superseded before a covering snapshot.**
>   `live_values_superseded` counts, per cell, writes that the first block whose snapshot covers
>   them does not apply, because a later write to the **same cell** with a revision `<=` that
>   snapshot exists. When render applies sequence `s` after `p`, it adds `s - p - 1`.

The rest of D2 (`Both` counts in each cell, 40 edits add 78, one `Arc<AtomicU64>`) is unchanged.

**D4** (replace):

> - **D4. Canonical drain order.** Fader before mute, left before right, in every block, under
>   the block's snapshot. When both channel cells of a kind were `Applied` in this block with
>   equal words, render applies one `Both` call, as today's single record does. A cell whose read
>   is `Unchanged` this block, because its newest revision is `> S`, is not paired.

**D6** (replace):

> - **D6. The contract the carry slices rely on.**
>   - A stage exposes `apply_pending(&mut self)`: the block drain with `LiveSnapshot::ALL`, in D4's
>     order, the same code the block drain runs.
>   - #1277's carry calls it on the predecessor before exporting a lane. Two facts make `ALL` equal
>     to "the newest `<=` the adoption block's `S`" there:
>     - control writes only the newest plan's cells, so every predecessor write happened before
>       the successor's publication, which render's claim acquired;
>     - every predecessor write carries a revision below the successor's publication revision.
>   - #1277 writes its retarget values into the successor's cells with the D5 writers, which
>     cannot fail, stamped `LiveStamp::unpinned(r)`, where `r` is the structural revision the
>     successor is published with.

**D7** (append):

> Each cell write carries the transaction's stamp:
> - `PlanPublisher::live_stamp(revision)` for a published plan (the `Full` cell if there is one,
>   else the `Active` cell, as `set_revision` routes);
> - `LiveStamp::unpinned(revision)` for a plan render has not adopted.
>
> `revision` is the revision the transaction commits as. The revision publication (#1314 D2)
> stays the commit's last write. In debug, control asserts that the stamp's `GateId` and the
> publication's agree.
>
> The browser, until #1382, stamps each admitted batch with a host-local, strictly increasing
> batch number, `LiveStamp::unpinned`, and renders with `LiveSnapshot::ALL`. That is sound because
> its control and render share one thread until #1382 moves edits behind #1381's exchange.

**D9** (replace the header text sentence):

> Header text: every live value of one committed transaction takes effect in the same block: the
> first block whose live snapshot is taken after the submit returns, or, while a successor is
> pending, its adoption block; the watermark reports that block's first sample (D15-2, D15-17).
> A later value for the same lane committed before that snapshot replaces it, and
> `LIVE_VALUES_SUPERSEDED` counts it.

The rest of D9 is unchanged.

**D11** (replace):

> - **D11. The acked-batch question: can an ack ever precede a drop? No.**
>   - Every fallible check precedes the first cell write (D7), and writes cannot fail.
>   - A write never overwrites the newest published value of its cell, nor the value render's open
>     snapshot needs (#1432 D1). So a value that is not yet applied is either applied by a later
>     block or superseded by a newer committed value: the committed model holds the newer value,
>     render applies it, and the counter records the replacement (D2).
>   - A value whose revision is above a block's snapshot is not dropped: its cell stays pending.
>   - A value written to a plan that is swapped out before it drains is applied by the carry's
>     `apply_pending` (D6), or is in the committed model its successor is prepared from (#1053 D7,
>     D15-17).

**New D12 (drain placement):**

> - **D12. Where the drains run.**
>   - The fader and matrix drains move from `process` to the bank's `begin_block`, which already
>     carries the drain contract (`GraphPreparedBuiltinBankProcessor::begin_block`); the fused
>     `FaderMatrixBankProcessor` forwards to both. The test-only scalar processors drain at the top
>     of `process`.
>   - Each drain reads under the block's snapshot `S`, which X2 hands to `begin_block` and to
>     `GraphBindingBlock`.
>   - Every dirty take of a block follows the block's snapshot.

**Gates** (gates 1, 2 and 4-7 unchanged; add):

> 8. **One transaction, one block (new host-core test, deterministic).**
>    - Two strips `A` and `B` on the exchange. Write a transaction (mute `A`, unmute `B`) into
>      their cells with `live_stamp(r)`, and do not publish `r`. Render one block: neither change
>      is applied, and the watermark stays below `r`.
>    - Publish `r` and render the next block: both are applied, and the watermark reads
>      `(r, that block's first sample)`.
>    - Mutation (PR evidence): the drains read with `LiveSnapshot::ALL`. This turns the first
>      block red.
> 9. **The watermark names exactly the first block of a racing transaction (extend
>    `crates/capi/tests/plan_swap_race.rs`'s `a_watermark_advance_names_a_block_that_applied_the_edit`).**
>    - Each commit is a two-strip transaction (fader `-6 dB` on `eq0`, `0 dB` on the other, then
>      swapped).
>    - Every rendered block holds both strips at the old values or both at the new ones, never a
>      mix.
>    - At each advance to `r`, the block equals a single-threaded replay that applied `r` at that
>      block, which is the converse the attempt-1 record found missing.
>    - Mutations, each red: drains under `LiveSnapshot::ALL`; the snapshot taken after
>      `render_inner`.

**Test value** (add):

> - Gate 8: a drain that applies a value above the block's snapshot, so a transaction lands
>   across two blocks.
> - Gate 9: the same under a real race, and a watermark whose `first_sample` names a block that
>   applied only part of the revision.

**Dependencies** (add): X1 and X2.

### 8.3 #1314: affected decisions

**D1** (replace the first bullet):

> - **D1. The revision travels with its plan.** Each mailbox cell of #1343 gains a `RevisionGate`
>   (#1432 D1) in place of a plain revision word, plus `superseded: u64` and `outcome: u32`. The
>   gate's revision bits are the newest committed revision whose content that cell's plan carries.
>   Its bit 63 is render's announcement.

The rest of D1 is unchanged. "Writes the initial revision" becomes `rearm`.

**D2** (replace the store sentences):

> - `PlanPublisher::set_revision(revision)` publishes through the routed cell's gate with
>   `RevisionGate::publish` (a swap) and keeps that cell's `GatePin`.
> - Publication into an `Empty` cell (`MailboxPermit::write_revision`, `republish`) uses
>   `rearm`, which resets the pin.
> - `PlanPublisher::live_stamp(revision)` returns the stamp for the same routed cell (#1312 D7).
> - Every write to a gate word is a read-modify-write.

The routing rules (`Pending`, `Active`, a held candidate) are unchanged.

**D3** (replace whole):

> - **D3. Render takes the block's live snapshot, then drains under it.**
>   - In `render_contiguous` and `render`, after `enter_block` and before `render_inner`, render
>     takes `S = snapshot()` of the `Active` cell's gate: one `fetch_or`. It never touches the
>     other cell.
>   - `render_inner(io, time, S)` hands `S` to the executor (`begin_live_block`) after its shape
>     and clock checks. Every drain of the block applies, per cell, the newest value whose
>     revision is `<= S`. A value with a revision `> S` waits for a later block.
>   - If `S` is above the published revision, render publishes `(S, this block's first absolute
>     sample, flags)` after `render_inner` returns `Ok`, and adds the covered revision count to
>     the counters. Otherwise it writes nothing.
>   - **`first_sample` is exact:** it is the first rendered sample at which every live value of
>     every revision in `(previous watermark, S]` is in effect, and no earlier block applied any
>     value of those revisions. A transaction is never split across blocks.
>   - A block refused before its drains (shape, clock, source) publishes nothing.
>   - A rebuilt plan renders from the adoption block, whose `S` covers every revision its carry
>     applies.
>   - "In effect" means a live value's ramp has started at that sample.
>   - Lanes still on queues are the exception: until #1345, an effect record of a revision `> S`
>     can apply one block before the watermark reports it, so the watermark is late for it,
>     never early.

**Hazards** (add):

> Render must take `S` with the read-modify-write, never a load. Control must write the gate with
> a swap, never a store. Either change lets control overwrite a value render's snapshot needs
> (#1432 L2).

**Attempt record**: the follow-up finding "the converse does not hold" is resolved by D3 as
amended, and #1312 gate 9 asserts the converse.

### 8.4 Consequential amendments (outside the asked list; root decides where they land)

- **#1345 D4 (span window).** Deferring a dirty parameter cell past `automation_capacity` would
  split a revision across blocks. Replace it: the lane's staging window holds every live
  parameter cell (sized from the cell count at preparation), and there is no deferral path. Gate 3
  becomes "the window is at least the live cell count for every descriptor".
- **#1346 D5, #1347 D3.** "Applies every dirty cell" / "applies its cell if dirty" become "under
  the block's snapshot". The carry drain (`drain_for_carry`) becomes `apply_pending` with
  `LiveSnapshot::ALL`, by #1312 D6's argument.
- **#1280 D2.** Use the resolution and counting text of section 5 of this design: the successor's
  own value wins only if its read under the adoption block's `S` is `Applied`.
- **#1277 D5.** Retargets are stamped `LiveStamp::unpinned(r_structural)`. "Applies its own dirty
  cells at block entry" means under the adoption block's `S`.
- **#1053 D2.** Delete "#444's block-atomic batch claim is declined knowingly", and write
  "transactions are block-atomic for every lane held in cells (#1312, #1345-#1347)". In D4, "a
  value is superseded only by a later commit before the same drain" becomes "... before the same
  block's snapshot".
- **Decision 15, D15-2 condition 2, and the AGENTS.md sentence.** "superseded by a later committed
  value before the same render drain" becomes "before the same block's live snapshot", and "every
  live value of one committed transaction takes effect in the same block" is added. That is root's
  record to amend.
- **#1355.** Section 4.1's prime hazard.

---

## 9. Size and split

- **#1432 (amended, engine only): fits half a day.**
  - Scope: `latest_cell.rs` with the gate, the pin and stamp, the cell, the writer, the reader, the
    dirty word and reader, and the peek (about 350 lines with docs); three loom models and four
    unit tests (about 300 lines); the policy floors.
  - The prototype beside this file is a working draft of all of it. Its writer target rule, reader
    rule and models carry over.
  - Risk: the realtime-policy region floors, and loom compile time (about 30 s).
- **X1 (new, stream B, engine and protocol): "Take each block's live snapshot through the mailbox
  revision gate".** About half a day.
  - `spsc.rs`: revision words become `RevisionGate`s, a `GatePin` per cell, `rearm` on
    publication, `withdraw`'s load.
  - `plan_exchange.rs`: `snapshot()` replaces `active_revision()`; `live_stamp`; `set_revision`
    returns its `GateId`.
  - `plan.rs`: `begin_live_block` and the new `render_inner` argument. Direct render passes
    `ALL`.
  - The session store's ceiling becomes `GATE_REVISION_MAX`, in `crates/protocol` or as a
    control-plane pre-check.
  - Adapt #1314's gates 1-8 and its loom models (store to swap, load to `fetch_or`). Add L4, and
    an engine test in which a probe executor records the `S` it was handed and equals the
    watermark's revision on every advancing block.
  - Dependencies: #1432, #1314, #1311, #1343.
- **X2 (new, stream B, graph, rack and builtins-compiler): "Hand each block's live snapshot to
  every live drain".** About half a day, mostly mechanical.
  - The graph executor implements `begin_live_block` and passes `S` into bank `begin_block` (rack
    `BankStage`, `GraphPreparedBuiltinBankProcessor`), into `GraphBindingBlock` (33 construction
    sites, 27 of them in builtins-compiler tests) and into the route op's drain.
  - Queue drains ignore `S` until their lanes move to cells.
  - Gate: a probe bank stage and a probe per-node processor record the `S` they receive. It
    equals the block's snapshot (mutation: pass `ALL`, red).
  - Dependencies: X1. Paths: `crates/graph`, `crates/rack` (stream A's crates, sequenced),
    `crates/builtins-compiler` (signatures only).
- **#1312 then depends on X1 and X2** and keeps its scope, with D12's drain move and gates 8-9
  added. Its size does not grow beyond Amendment 1's estimate by more than the two tests.
- Merge order: #1432 → X1 → X2 → #1312 → (#1277, #1345, #1346, #1347). For the file-owner rows,
  `plan_exchange.rs` gains X1 after #1314, #1311 and #1310, and `graph`/`rack` gain X2 before
  #1347.

---

## 10. Open points for root

1. **Gate home.** Mailbox-cell gate (chosen: minimal change to the landed #1314) or a per-plan gate
   (cleaner; it removes the mailbox revision words and `LiveSnapshot::ALL`). Both are proved by
   section 3.4.
2. **The revision ceiling** `2^63 - 1`: put it in the protocol session store, or in a control-plane
   pre-check.
3. **Executor invariant errors after drains** (4.3): accepted as a defect path, or does root want
   the block-start pass of 4.4, which makes it exact by construction?
4. **Counter gap that exists today and is not introduced here.** Writes to a candidate that is
   withdrawn and dropped (#1310) are never covered and never counted by
   `live_values_superseded`. The revision-level `SUPERSEDED` outcome reports them. Root should
   confirm that the cell counter's unit excludes them.
5. **#1345 D4** must change (8.4). It is the only place where a spec as written would split a
   transaction under the new contract.
