# Make live strip records valid by construction

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Found while filing *Let only host-core build the live route records that hosts push* (#1416),
whose non-goal names this gap; it is the same class of defect on the strip lanes. No rendered bit
moves.

## Problem (verified on `main` at `0a1176b3b`)

- **The records.** A strip's live edits ride three record types in
  `crates/builtins-compiler/src/lib.rs`:
  - `TrackControlRecord` (`:105-110`): a 2x2 matrix and a ramp length, both public fields. Its doc
    says the matrix is "already domain-checked by the producer".
  - `TrackFaderRecord` (`:129`): `FaderDb { lanes, db, smoothing_samples }` or
    `Mute { lanes, muted, smoothing_samples }`. Enum variant fields are always public.
  - `TrackInputRecord` (`:175-211`): `TrimDb`, `PolarityInvert` or `PreparedFilter { target }`.
- **The producers.** `TrackControlProducer` (`:254-270`) holds the three raw queue producers as
  public fields (`producer`, `fader`, `input`). host-core re-exports the producer and two of the
  record types (`crates/host-core/src/lib.rs:236-237`). `hosts/host-web` depends on
  `builtins-compiler` directly and imports all three records (`hosts/host-web/src/lib.rs:25-27`).
- **Where the domain is checked today.** By each producer, separately:
  - the browser's admission (`into_track_record`, `hosts/host-web/src/lib.rs:4242-4351`) checks
    `pan_matrix`, `Matrix2x2::checked`, and spells `[-144, 24]` for `fader_db` (`:4284`) and for
    `trim_db` (`:4323`), with its own boolean checks for mute and polarity;
  - the live-delta classifier builds records from the committed model
    (`crates/host-core/src/live_delta.rs:313-335`), which the session validator checked;
  - and again by the render thread when it applies a record: `set_fader_db` through
    `checked_fader_gain` (`crates/builtins/src/lib.rs:4292`), the trim through `checked_trim_gain`
    (`:4310`, private to `builtins`), the matrix through `set_target_smoothed` (`:4056`).
- **The defect.** Any code that can reach the producers can push a record no check has seen:
  `producer.fader.try_push(TrackFaderRecord::FaderDb { lanes, db: f32::NAN, .. })`. The render
  thread then refuses it inside the drain, and the drain turns that refusal into a render error
  (`drain_fader_controls`, `crates/builtins-compiler/src/lib.rs:1064-1103`,
  `.map_err(render_error)?`; `drain_matrix_controls`, `:1108-1133`; the input bank's
  `drain_controls`, `:460-505`, through `begin_block`, `:527-530`). A command the host accepted
  then fails a render block. This is the acked-batch question in another form: an acknowledged
  value must never be refused later.
- **Decision 15 changes the lanes, not the records.** *Hold live values in latest-target cells on
  both hosts* (#1312, D5) and *Hold strip input-lane values in latest-target cells* (#1346, D6) turn
  these producers into infallible cell writers that take the same records. Validating the records
  where they are built keeps that guarantee whatever the lane is.

## Decisions

- **D1. Each record type is valid by construction.** In `crates/builtins-compiler/src/lib.rs`,
  each of the three record types gets private fields and fallible public constructors, one per
  kind of edit (for example `TrackFaderRecord::fader_db(lanes, db, smoothing_samples)`,
  `TrackFaderRecord::mute(..)`, `TrackInputRecord::trim_db(..)`, `polarity_invert(..)`,
  `prepared_filter(target)`, `TrackControlRecord::new(matrix, smoothing_samples)`). A constructor
  returns `Err(BuiltinParameterError)` for every value the render-side apply would refuse that
  depends on the record alone. It calls the same check the apply calls (`checked_fader_gain`,
  `checked_trim_gain`, `Matrix2x2::checked`, `validate_prepared_input_filter_target`, and any ramp
  bound the apply checks), never a new spelling of the domain. `checked_trim_gain` becomes `pub`
  for this.
- **D2. Enums become opaque.** `TrackFaderRecord` and `TrackInputRecord` become structs that wrap a
  private enum. Code outside the crate that reads a record (the browser's coalescing at
  `hosts/host-web/src/lib.rs:4746-4858`, `:5203-5312` and `:5683`) reads it through a public
  by-value view (for example `fn edit(&self) -> TrackFaderEdit`, a public enum with the same
  variants) and builds a changed record through the constructors again. A view cannot be turned
  back into a record without a constructor.
- **D3. Every builder uses the constructors.** The browser's admission maps a constructor's error
  to `COMMAND_REASON_DOMAIN`, as today, and drops its own `[-144, 24]` spellings for fader and trim.
  The live-delta classifier and every test helper use the constructors too.
- **D4. The render-side checks stay** as the bank's own contract. With D1 they cannot fail on a
  queued record; say so in the drain's comment. Do not remove them in this issue.
- **D5. The guarantee is a compile-time one, and the test says so.** A `compile_fail` doctest per
  record type shows that a record cannot be written as a struct or variant literal from outside
  the crate (pin the error code, for example `E0451` for a private field or `E0603` for a private
  enum). A plain doctest shows the constructor path compiles. These run in CI once the dependency
  below lands.
- **D6. Split if it does not fit.** If the three types do not fit half a day, the coordinator
  splits the input record (`TrackInputRecord`) into a successor before implementation.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the three record types, their constructors and views,
  the drains' comments, and the crate's tests that build records)
- `crates/builtins/src/lib.rs` (`checked_trim_gain`'s visibility only)
- `crates/host-core/src/live_delta.rs` (the record construction at `:313-335` only),
  `crates/host-core/src/lib.rs` (re-exports of the view types only)
- `hosts/host-web/src/lib.rs` (`into_track_record` and the coalescing sites named in D2 only),
  `hosts/host-web/src/tests.rs` (record construction only)
- `crates/capi/src/runtime/*` and `crates/control-plane/src/*` (record construction only, where
  #1312 left it)
- This spec

## Non-goals

- The producers' fields and the lanes themselves (#1312, #1346).
- Route records (#1416). Effect records.
- The fader and trim domain's remaining spellings outside record construction (*Route the builtins
  fader domain checks through checked_fader_gain*, #1303).

## Hazards

- **Hot files.** `crates/builtins-compiler/src/lib.rs`, `crates/host-core/src/live_delta.rs` and
  `hosts/host-web/src/lib.rs` are in `STREAMS.md`'s hot-file table. This issue lands after #1312
  and #1346 and rebases over whatever lands before it.
- **The witness.** `TrackInputRecord` implements `LiveControlRecord` (`:213`); the symmetry witness
  reads it. The view must give it the same fields.
- **No bit moves.** Valid records render exactly as before; gate 3 checks it.

## Objective gates

1. **Invalid values are refused at construction (new unit test in `builtins-compiler`).** For each
   constructor, the values the bank refuses (NaN, the infinities, each domain bound's next
   representable value outside it, a non-boolean where one applies through the browser's path)
   return `Err`, and the boundary values return `Ok`. Mutation (PR evidence): make
   `TrackFaderRecord::fader_db` skip `checked_fader_gain`; the test turns red.
2. **The bypass does not compile (new doctests, D5).** The tests pass under the doctest step that
   the dependency below adds. Mutation (PR evidence): make one record's field public again; its
   `compile_fail` doctest turns red.
3. **No behaviour moved.**
   - `cargo test --locked -p builtins-compiler --features test-support` and
     `cargo test --locked -p builtins-compiler`
   - `cargo test --locked -p host-core --features control-provider,test-support`
   - `cargo test --locked -p host-web --features test-support`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`: every
     violation count 0, and the same `pcm_digest` as the base (one-time PR evidence).
   - `bash scripts/build-web-audioworklet.sh --named-twin <N> <A>`, then
     `bash scripts/check-web-audioworklet.sh <A>
     <N>/miso-engine-v1-audio-worklet.simd128.named.wasm` and
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`.
4. **Workspace.** `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `cargo clippy --locked -p builtins-compiler --all-targets -- -D warnings`;
   `bash scripts/check-builtins-policy.sh`; `bash scripts/check-host-core-policy.sh`;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/check-workspace-policy.sh`.

*Test value.*
- Gate 1's test is red if a constructor admits a value the render-side apply refuses, which would
  turn an accepted edit into a failed render block; no test checks the records today, because any
  code may build them.
- Gate 2's doctests are red if a record can again be built without a constructor.

## Evidence

- Gate 1's and gate 2's mutation runs, gate 3's digests.
- The list of apply-time refusals that depend on the plan, not the record (they stay at render),
  with one line each on why a valid record cannot reach them.

## Dependencies

- *Run doctests in CI* (#1422): D5's guarantee counts only once CI runs doctests.
- *Hold live values in latest-target cells on both hosts* (#1312): the fader and matrix writers.
- *Hold strip input-lane values in latest-target cells* (#1346): the input writer.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
