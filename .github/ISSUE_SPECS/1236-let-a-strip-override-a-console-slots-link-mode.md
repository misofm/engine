# Let a strip override a console slot's link mode

## Mission

Let any strip -- a track or a submix -- run a console slot at its own detector link mode, as a DAW
lets each plugin instance choose stereo-linked or multi-mono, without splitting the slot's bank.
This is a small umbrella: it records the owner's answer and the frozen design, and names two
bounded slices, filed when this umbrella is scheduled.

## Owner direction

Owner question Q1 of decision 13 (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, "Owner
questions"), answered on 2026-10-03: "We should follow whatever modern day DAWs look like." DAWs
choose linked or unlinked detection per plugin instance, so a strip's console entry may override the
slot's `link_mode`. This reverses the planner's recommendation in DESIGN 8.2 Q1
(`docs/handoffs/submix-sends-2026-10-02/DESIGN.md`). It is outside *Submix strips and live aux
sends* (#1196), whose closure is fixed on slices 00-28, and outside the VCA umbrella.

## Today

Every anchor below is verified on the batch K3 follow-up tree (branch `codex/batch-submix-k3`).

- The link mode is declared once per console slot: `ConsoleSlot.link_mode`
  (`crates/session/src/model.rs:269-278`). A strip's `ConsoleEntry` carries only `slot`, `bypass`
  and `params` (`:281-289`), and the parser refuses `link_mode` on an entry with the reason
  `CONSOLE_ENTRY_EFFECT_FIELD` (`crates/session/src/parse.rs:898`, `:950-975`). Lowering copies the
  slot's mode into every strip's effect (`lower_section`, `model.rs:449-463`).
- The prepared link mode is part of `EffectProgramKey` (`crates/effect-contract/src/lib.rs:1171`
  onward), and a rack chain's bank program is the list of its slots' keys
  (`crates/graph-compiler/src/banks.rs:186-192`). So two strips that ran one console slot at
  different link modes would form two cohorts: a split bank, where decision 12 promises that a
  console slot always banks as one.
- The kernels already treat link as a lane mask: the compressor's `Invariants` splats `linked` and
  `averaged` from one `LinkMode` (`crates/compressor/src/kernel.rs:305-341`); `dual_mono` only
  selects a cheaper path (`:621-625`).
- Console-eligible effects (`CONSOLE_ELIGIBLE_EFFECTS`, `crates/effect-compiler/src/prepare.rs:223-230`)
  that support more than `dual_mono`: `miso.compressor` (`LinkModeSet::ALL`), `miso.gate-expander`
  (`ALL`), `miso.transient-shaper` (`ALL`) and `miso.true-peak-limiter` (`dual_mono` and `maximum`).
  `miso.parametric-eq` and `miso.soft-clip` are `dual_mono` only.
- **The precedent.** Issue #1087 lowered a session `bypass` to per-lane shunt state, so a bypassed
  and an enabled instance share one `EffectProgramKey` and one bank (`prepare.rs:40-55`,
  `banks.rs:188-191`). A link mode lowers the same way.

## Frozen design

- **D1. Per-lane link state.** For each console-eligible effect that supports more than `dual_mono`,
  the prepared link mode leaves `EffectProgramKey` and becomes per-lane state: the lane's `linked`
  and `averaged` mask bits. A bank whose lanes all share one mode keeps today's fast path and its
  bits (class A for every existing session); a mixed bank takes the linked path for the whole bank
  (DESIGN 8.2 Q1's cost note) and never moves another lane's bits.
- **D2. Grammar.** V1 has no optional fields (`docs/SESSION_SCHEMA_V1.md:177`), so each console entry
  gains a **required** key `link_mode` whose closed token set is `slot` (run the slot's mode) plus
  the three link tokens. The migration writes `"slot"` into every entry, so no document's meaning
  or render moves. A value the effect does not support refuses with `effect.link_mode.unsupported`
  at the entry's path, as an insert's does today. The SDK writes `"slot"` by default and accepts a
  per-strip `linkMode`. The change is structural (a recompile), never a live control.
- **D3. Wire.** The console entry message appends one field (an in-place V1 amendment, on the
  decision 12 and 13 precedent); nothing is renumbered and there is no `ABI_VERSION` bump.

## Slices (filed when this umbrella is scheduled)

1. **L1. Lower the link mode to per-lane state in the console-eligible linked effects' banks**
   (D1). No grammar change. Gates: a mixed-mode cohort binds one bank per console slot and every
   lane is bit-identical to a fresh plan of that lane's strip at its own mode; every checked-in
   render digest and the console benchmark workload counts are unchanged; per-effect bank
   conformance at both lane widths. If one effect's kernel cannot carry per-lane link in half a
   day, split that effect into its own slice.
2. **L2. Declare a strip's console link mode in the session, the wire and the SDK** (D2, D3).
   Depends on L1. Gates: the grammar, canonical writer, wire round trip and SDK builder accept the
   four tokens and refuse others; an unsupported mode refuses at the entry's path; migration
   changes every checked-in document by the added key alone; a session with a per-strip override
   renders bit-identical to one where that strip carries an insert of the same effect at that mode
   (pre-insert position), and the slot still binds one bank; the `author-session` skill and the app
   handoff drop the bus-compression hazard guidance (DESIGN 2.2b).
   **Deliverable (K3 follow-up amendment, from the K3 verdict's MINOR-2): amend the binding
   texts in L2's own PR.** D2 changes decision 12's slot declaration, so L2 lands, in the same
   PR as the grammar:
   - `AGENTS.md`, "Approved audio architecture", the console sentence "the session declares each
     console slot once (a stable slot ID, the native effect, quality and link mode), and every
     strip carries every slot, in that order, with only its own parameters and bypass": it gains
     the strip's link-mode override (the slot's `link_mode` becomes the default a strip's entry
     may override). Until L2 lands, any earlier `AGENTS.md` mention carries decision 13's
     qualifier for owner direction read by the planner, "Planned under decision 13 ..., subject to
     owner review"; L2's PR removes it.
   - Decision 12's ruling, `docs/rulings/engine-footprint-2026-09-29.md`, the **Shape** bullet
     ("Each slot is declared once, with `slot` ..., `quality` and `link_mode`. Every track has
     every slot: each track carries a `console` array of `{ "slot", "bypass", "params" }`"): an
     amendment note that the entry now also carries `link_mode` (`slot` or a link token), citing
     decision 13 Q1 and this issue. The rest of decision 12 is unchanged.
   - Decision 13's ruling, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, Q1: recorded
     as delivered.

## Invariants

- A console slot always banks, on every target and for every track count (decision 12). A link
  override never splits or declines a bank.
- Banking may couple lanes' cost, never their bits: a lane renders the same bits in a mixed bank
  as alone.
- No live link-mode control, no third-party effect, no insert change.

## Non-goals

- Overriding a slot's effect, quality or sidechain per strip.
- Link modes for `miso.parametric-eq`, `miso.soft-clip` or effects that are not console-eligible.
- The multiband compressor, which is not console-eligible.

## Objective gates (umbrella)

- L1 and L2 closed with Sol PASS, their evidence upstream and their GitHub issues synchronized.
- `AGENTS.md`'s console sentence and decision 12's Shape bullet carry the per-strip link-mode
  override, landed in L2's PR, with no decision-13 qualifier left on it.
- Decision 13's ruling records Q1 as delivered.
- Each slice's own gates, when filed, include (K3 verdict NIT): the touched effects'
  `KERNEL_ROSTER` rows; `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`, then
  `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
  `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` and
  `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-cross-targets.sh`; and
  `scripts/run-aarch64-tests.sh` (CI `aarch64-debug`/`aarch64-release` when no arm64 host).

## Dependencies

- None blocking. Schedule after batch K3 of #1196 (it touches the session grammar and the effect
  bank contract that K1-K3 changed).
