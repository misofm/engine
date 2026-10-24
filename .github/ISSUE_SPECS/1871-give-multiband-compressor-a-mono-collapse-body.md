# Give the multiband compressor a mono-collapse body

Filed by root after #973 (backlog sprint, 2026-10-24). A handoff promised these mono-body siblings, but they were never filed.

## Product outcome

A mono track whose chain holds `miso.multiband-compressor` can mono-collapse like the compressor, parametric EQ, true-peak limiter and (since #973) gate-expander. Today `multiband-compressor` keeps every default of the mono trio, so its bank never collapses and a mono strip with it pools Stereo.

## Smallest closable slice (follow #973 as the template)

- One kernel body with a const MONO parameter (one implementation shape, the same per-lane arithmetic): the collapsed form reads and writes only the left plane.
- `supports_mono_collapse`, `process_bank_mono` (sharing one inlined body with `process_bank`), `desymmetrize_channels` (copy every word the kernel reads), `channels_agree`, `lane_channel_symmetry`, and the scalar `channel_symmetry` on the same designed-words witness.
- D7 in a collapsed block re-seeds a failed lane on both channels.
- Wasm KERNEL_ROSTER: split the effect's f32x4 row into dual and collapsed rows if a second kernel appears (each pattern matches exactly one kernel; never loosen).
- console-workload `tests/paired_spans.rs`: add the effect's A3 per-effect obligation and its id to `expected`.

## Gates

- `crates/multiband-compressor/tests/mono_collapse.rs` (new): a collapse then a disengage equals a never-collapsed bank, bit for bit; the witness declines on exactly the asymmetric lane; and a test that the `channels_agree` word list is complete.
- The randomized differential with `witness: true`; `cargo test --locked --release -p console-workload --test paired_spans`; check-web-audioworklet.sh; check-cross-targets.sh.
- Class A: a one-time digest of dual outputs before and after (PR evidence).

## Owner rule

Only mono source files get dual-mono processing; no content detection.
