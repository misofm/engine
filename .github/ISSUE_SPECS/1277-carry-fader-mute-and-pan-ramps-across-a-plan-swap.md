# Carry fader, mute and pan ramps across a plan swap

Slice 8 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A strip whose fader, mute and pan or matrix the transaction did not change keeps its exact state
through a plan swap, including a fader, mute or pan ramp that is in flight at the swap block. A
producer pulling a fader while a track is added hears the ramp continue, not jump. A live record
admitted before the swap is applied, not lost.

## Context

- The seam-side builtins render in banks: `FaderBankProcessor` (`crates/builtins-compiler/src/lib.rs:622`,
  impl `:632`), `MatrixBankProcessor` (`:685`, impl `:694`) and, in plans prepared between render
  calls (the browser), the fused `FaderMatrixBankProcessor` (`:738`, impl `:1063`). Kernels and
  state: `FaderStage` (`crates/builtins/src/lib.rs:2440`), `FaderRampStage` (`:2510`, with its
  `remaining` count), `MatrixStage` (`:2741`, with its settled-flag sync).
- These banks drain their live queues inside `process` and apply each record to lane state. *Bound
  the builtin fader and matrix drains to the records present at block entry* (#1253) bounds those
  drains.
- Effective fader values include VCA offsets (`SessionModel::effective_strip_faders`).
- The input-section carry, the rack slot accessor, `as_any_mut` and the collapse rule come from
  *Carry the strip input section across a plan swap* (#1276).

## Decisions frozen for this slice

- **D1. Keys and rule.** Owner keys `(strip ID, PostFader)` and `(strip ID, PostMatrix)`, or the
  fused stage's key. A stage carries when its effective fader and mute values (VCA offsets included)
  and its `matrix_or_pan` (pan or matrix coefficients and smoothing) are bit-equal in the committed
  model before the transaction and in the successor's model, and both plans attach the same control
  kind. Padding lanes never carry.
- **D2. Lane state.** Fixed-size, plain-data lane states for the fader/mute stage, the matrix stage
  and the fused stage: current gains, ramp targets and positions (`remaining`), matrix ramp state.
  `export_lane` and `import_lane` on each bank; import re-syncs derived state (the matrix stage's
  settled flags, the ramp's `remaining`).
- **D3. Drain, then carry.** Drain each predecessor bank's live queues once with #1253's bounded
  drain (records present at entry), then export.
- **D4. Fused and split forms.** A lane may move between the split and the fused form only if a host
  prepares both kinds for one session; no host does, so a cross-form pair is not carried (it starts
  at rest) and the join records it.

## Deliverables

1. D2 in `crates/builtins`; D3 in `crates/builtins-compiler`; the inventory rows, join and carry
   section in `crates/host-core` and `crates/graph`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No solo or VCA host state (B3 owns the browser's solo continuity).
- No effects or delay lines.

## Objective gates

1. **Gap-free acceptance, both widths.** At `Backend::Simd8` and `Backend::Simd4`: session A has nine
   tracks with non-unity faders and non-centre pans (per-track, per-channel values) plus slice 7's
   input settings. Session B adds a muted track whose ID sorts first. Every block of the swapped run
   equals a fresh B fed from frame 0.
2. **Ramp in flight.** Prepared with live controls: a fader record with a ramp of several blocks is
   admitted to A between blocks 5 and 6 and committed in B's model; the swap is at block 6. A pan
   record is admitted the same way. Every block equals the reference A plus the muted track fed the
   same records at the same blocks. A second case admits the records after the successor is pending,
   to the successor's queues: also bit-identical.
3. **Fused form.** Gate 2 again with both runs prepared between render calls
   (`FaderMatrixBankProcessor`), the browser's form.
4. **Rule.** A strip whose fader the transaction changed is not carried; its stage starts at rest at
   the new value.
5. **Realtime.** The swap block makes zero allocations and frees.
6. Commands:
   - `cargo test --locked -p builtins -p builtins-compiler -p graph -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - the umbrella's inherited gates.

## Test value

- Gate 2: a ramp imported without its `remaining` count, or a record lost because the carry exported
  before draining, turns it red.
- Gate 3: a carry that handles only the split banks leaves the browser's fused form at rest; it turns
  red.
- Gate 4: carrying a stage whose value the transaction changed keeps the old gain; it turns red.

## Dependencies

- *Carry the strip input section across a plan swap* (#1276).
- *Bound the builtin fader and matrix drains to the records present at block entry* (#1253).
