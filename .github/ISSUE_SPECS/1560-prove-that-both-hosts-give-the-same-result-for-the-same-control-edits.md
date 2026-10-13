# Prove that both hosts give the same result for the same control edits

Slice S8 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C1:
"a behaviour that exists on one host only is a defect"). Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

One test, run on every pull request, sends the same script of control edits to the C ABI and to
the browser host, both native in one process, and compares the outcome of each edit and the
rendered bits of each block. When a value lowers, ramps, composes or refuses differently on one
host, the test turns red. This is behaviour parity before code parity: the browser leg still uses
today's command records; #1562 switches it to `SET_CONTROL` and adds the list and snapshot bytes.

## Context

- **Two cross-host tests exist, each for a narrow claim.**
  - `hosts/host-web/tests/control_smoothing_parity.rs` (#1364 gate 3): fader, mute and pan edits
    with no per-edit ramp, on two tracks (one pan with its own `smoothing_samples`, one without),
    at 44.1, 48, 88.2 and 96 kHz, through the exported `miso_engine_v1_*` entries and
    `AudioWorkletEngineHost`.
  - `hosts/host-web/tests/strip_cells_cross_host.rs` (#1399): fader, pan and one EQ parameter
    through `control_plane` and through the browser's command admission; equal PCM and equal
    `live_values_superseded` counts.
- **The browser's command kinds** (`hosts/host-web/src/lib.rs:837-949`): pan 1, matrix 2, fader 3,
  mute 4, effect parameter 5, bypass 6, solo 9, trim 10, polarity 11, input filters 12 (through
  prepared targets), send gain 13, send mute 14, send matrix 15, VCA offset 16, VCA mute 17.
  There is no kind for `follows_mute`, VCA membership or a pan/matrix mode switch: those are
  structural in the browser until #1562.
- **The browser's refusal reasons** are `COMMAND_REASON_*` words in the command report; the
  C ABI's are the control error codes of `SET_CONTROL` (#1553) in the non-OK response.
- **host-web already dev-depends on `capi`, `control-plane` and `protocol`**
  (`hosts/host-web/Cargo.toml:35-48`), never linked into the browser artifact.
- **Lane width.** Both legs run natively, so they run the same lane width. Lane-width parity
  (native against `simd128`) stays with the G5 corpus and the #106 F4 session parity gate.
- **Bypass.** *Crossfade the bypass switch over the session ramp* (#1341) changes the C ABI's
  bypass to a crossfade; *Crossfade the browser's live bypass command over the session ramp*
  (#1393) makes the browser match. Neither is on `main`.

## Decisions frozen for this slice

- **D1. One file.** `hosts/host-web/tests/control_parity.rs`. The C ABI leg drives the exported
  `miso_engine_v1_*` entries (as `control_smoothing_parity.rs` does) with `SET_CONTROL` in
  `SESSION_TRANSACTION_APPLY`. The browser leg drives `AudioWorkletEngineHost` with today's command
  records (and, for EQ and input filters, the prepared targets the browser's own preparers
  design, as `strip_cells_cross_host.rs` does).
- **D2. The session.** Tracks `t1`, `t2`, `t3`; submix `b` routed to the output; `t1` sends
  `pre_fader` into `b` with `follows_mute: true`; VCA `v` holds `t2`. `t1` has a pan with its own
  `smoothing_samples` (200), `t2` a pan with none, `t3` a matrix. Inserts: one instance of every
  launch native effect, spread over the three tracks (each insert takes its default parameters).
  Each source is a distinct, non-constant signal on each lane, long enough for the script.
- **D3. The script.** A fixed list of `(block, edit)` pairs, each edit expressed in both forms:
  - strip level: `t1` fader both lanes; `t2` fader left lane; `t3` mute; `b` fader (a submix
    strip);
  - strip matrix: `t2` pan (no window: `pan_ms` applies); `t1` pan (its own window applies);
    `t3` matrix;
  - strip input: `t1` trim, `t2` polarity, `t3` HPF from 0 to 80 Hz and LPF to 8 kHz;
  - effect: one block-rate parameter of every launch effect except the parametric EQ; one
    parametric EQ band gain; the bypass of one effect whose session bypass lowers
    (`effect_compiler::lowers_session_bypass` true);
  - route: the send's gain, mute and matrix;
  - VCA: `v` offset both lanes, `v` mute left lane;
  - solo: `t2` on, then off;
  - ramps: one fader edit with ramp 0, one with ramp 480, every other edit with no ramp (the
    session default; the browser's `SESSION_DEFAULT_RAMP`);
  - refusals, one each: a fader of 30 dB; solo on `b`; a VCA index past the last VCA (browser) /
    an unknown VCA ID (C ABI).
  No edit of a row that is `rebuild_only` or structural on either host is in the script.
- **D4. Per edit, compare.** Accepted or refused on both legs; for a refusal, the code pair is in
  a fixed table in the test, one row per scripted refusal: the 30 dB fader is `control.domain` and
  `COMMAND_REASON_DOMAIN`; the solo on `b` is `control.address.unknown` (a submix has no solo row)
  and `COMMAND_REASON_NOT_SOLOABLE`; the unknown VCA is `control.address.unknown` and
  `COMMAND_REASON_UNKNOWN_VCA`.
  For an accepted edit, the C ABI reply's path is `live` (`monitor` for solo) and the plan epoch is
  unchanged.
- **D5. Per block, compare.** Each leg renders 64 blocks of 128 frames after boot, with the script's
  edits between blocks; every block's two planes are bit-identical across the legs. At the end,
  the two hosts' `live_values_superseded` counts are equal.
- **D6. Rates.** The whole run repeats at 44.1, 48, 88.2 and 96 kHz.
- **D7. Absorb and delete.** `control_smoothing_parity.rs` and `strip_cells_cross_host.rs` are
  deleted in the same PR: D3 holds both pan-window cases and the absent-ramp edits at four rates
  (#1364 gate 3), and D5 holds the equal superseded counts and the EQ edit (#1399).
- **D8. Bypass order.** The bypass edit stays in the script. If #1341 reaches `main` before
  #1393, the browser and the C ABI differ there, and this test turns red: that is a real parity
  defect, and root orders #1341 with #1393 (superseded by #1563, so in practice with #1563) or
  after #1562, which moves the browser leg's bypass onto `SET_CONTROL` through the core. This
  slice does not weaken the comparison.

## Deliverables

1. `hosts/host-web/tests/control_parity.rs`.
2. The two absorbed tests deleted.

## Authorized paths

- `hosts/host-web/tests/control_parity.rs` (new),
  `hosts/host-web/tests/control_smoothing_parity.rs` and
  `hosts/host-web/tests/strip_cells_cross_host.rs` (deleted).
- `hosts/host-web/Cargo.toml` (the dev-dependency comments that name the deleted tests).
- Fixture files under `hosts/host-web/tests/` that the new test needs.

## Non-goals

- No production code change. If the test finds a divergence, the fix goes into the core under its
  own issue, and this slice waits for it.
- No lane-width parity (G5 corpus), no browser artifact run (#1564 adds a control script to the
  direct-oracle digest).
- No control-list or snapshot bytes yet (#1562).

## Objective gates

1. `cargo test --locked -p host-web --features host-web/test-support --test control_parity`
   passes.
2. The deleted tests' claims are held (PR evidence, not a committed test): revert the browser's
   strip-pan window precedence in a scratch commit (resolve `pan_ms` before the strip's own window)
   and record that `control_parity` turns red; restore.

## Test value

- Gate 1 turns red if a value lowers to a different lane or cell, ramps over a different length,
  composes VCA or solo differently, or refuses with a different code on one host. No existing test
  compares send, VCA, solo, input or effect edits across the two hosts.
- Gate 2 records that the absorbed `#1364` claim (the strip window's precedence over `pan_ms`) is
  still caught after the two narrow tests are deleted.

## Dependencies

- #1553, #1554, #1555, #1556, #1557, #1558, #1559 (every scripted row is live on the C ABI).
- #1548 (the browser's checks and composition call the core).
- The bypass order of D8: *Crossfade the bypass switch over the session ramp* (#1341) and
  *Crossfade the browser's live bypass command over the session ramp* (#1393, superseded by
  #1563).

## Supersedes and shrinks

- None. (It absorbs two tests, not issues.)
