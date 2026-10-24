# Index the registry reads that still scan the automation table per entry

Filed by root from #1864's measurement (backlog sprint, 2026-10-24).

## Finding

After #1864, the control thread's `sources` stage of a first plan still grows about 15x for 4x strips (0.41 ms at 256 strips, 5.96 ms at 1024). The remaining cost is `registry::resolve` per entry: the fader row's `builtin_rows::model_fader_offsets` and the VCA row's `vca_offset_automation` both scan the whole automation table for each entry (`crates/controls/src/{strip.rs,builtin_rows.rs,vca.rs}`).

## Smallest closable slice

Route both readers through the #1710 `AutomationIndex` / `ModelView` reads that #1864 extended, so each lookup is indexed. Class A: byte-equal evidence on #1710's generated session and #1844's VCA-mute variant (PR evidence). One descriptive timing at 256 and 1024 strips (release, one warmup, two measured rounds, flock -x). No committed timing or scan-count test.

## Paths

`crates/controls/src/{strip.rs,builtin_rows.rs,vca.rs,view.rs}`, `crates/controls/tests/automation.rs` (at most one assertion row), this spec.

## Decision record

- D1 (root, from the scope note): the resolve path reads through a `ModelView`, not a
  descriptor-only resolve. A descriptor-only resolve would also drop the dead value work, but it
  needs a lazy `Resolved` across every owner module (strip, vca, route, session, effect rows): a
  wider registry refactor than this issue, for a value that is now an index lookup. Not taken.
- D2 (implementer, attempt 1): `strip::resolve` and `vca::resolve` take a `ModelView`; the
  registry gets `resolve_in(view, ..)`, and `resolve(model, ..)` stays as
  `resolve_in(ModelView::whole(model), ..)`. Reason: `diff.rs::changed_rows` also calls `resolve`
  and is outside the authorized paths; the wrapper keeps it, `get`, `set` and `check_monitor`
  unchanged (one scan per single read, as #1710 rules). `automation_diagnostics` builds
  `AutomationIndex::of(model)` once and calls `resolve_in` with the indexed whole view.
  `builtin_rows::model_fader_offsets` lost its only caller and is deleted. `view.rs` needs no
  change (the scope note's path list replaces the issue's).
- D3: one test row in the #1710 block of `crates/controls/tests/automation.rs`: `get` of vocal's
  `fader_db` with a hand offset on its own left entry reads the offset on the left and the static
  fader on the right (only a VCA ride reaches it). No existing `get` test reads a strip fader with
  a hand offset.

## Evidence (attempt 1)

- Class A (one time, PR evidence): `automation_diagnostics` output, the full control listing and
  `get` of every builtin and VCA row are byte-equal at base 6172dc4e6 and the branch on #1710's
  generated session (256 tracks), the same with hand offsets on a fader and the VCA ride, #1844's
  VCA-mute variant, and a refused variant (an out-of-domain offset and an HPF/LPF order fault: two
  diagnostics).
- Descriptive timing of the `sources` stage (#1864 harness, release, one warmup, two rounds,
  `flock -x`, load 1.7): first plan 0.15/0.14 ms at 256 strips and 0.70/0.69 ms at 1024 (#1864:
  0.41 and 5.96 ms); successor 0.15/0.19 and 0.71/0.70 ms. The stage now grows about 4.8x for 4x
  strips.
- Mutants on the D3 row: `strip::resolve` keying the mute entries, and swapping the lanes, both
  turn it red; the wrong-row mutant turns no other `controls` or `host-core` test red.
