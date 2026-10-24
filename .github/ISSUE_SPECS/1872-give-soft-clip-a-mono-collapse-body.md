# Give the soft clip (saturator/clipper) a mono-collapse body

Filed by root after #973 (backlog sprint, 2026-10-24). A handoff promised these mono-body siblings, but they were never filed.

## Product outcome

A mono track whose chain holds `miso.soft-clip` can mono-collapse like the compressor, parametric EQ, true-peak limiter and (since #973) gate-expander. Today `soft-clip` keeps every default of the mono trio, so its bank never collapses and a mono strip with it pools Stereo.

## Smallest closable slice (follow #973 as the template)

- One kernel body with a const MONO parameter (one implementation shape, the same per-lane arithmetic): the collapsed form reads and writes only the left plane.
- `supports_mono_collapse`, `process_bank_mono` (sharing one inlined body with `process_bank`), `desymmetrize_channels` (copy every word the kernel reads), `channels_agree`, `lane_channel_symmetry`, and the scalar `channel_symmetry` on the same designed-words witness.
- D7 in a collapsed block re-seeds a failed lane on both channels.
- Wasm KERNEL_ROSTER: split the effect's f32x4 row into dual and collapsed rows if a second kernel appears (each pattern matches exactly one kernel; never loosen).
- console-workload `tests/paired_spans.rs`: add the effect's A3 per-effect obligation and its id to `expected`.

## Gates

- `crates/soft-clip/tests/mono_collapse.rs` (new): a collapse then a disengage equals a never-collapsed bank, bit for bit; the witness declines on exactly the asymmetric lane; and a test that the `channels_agree` word list is complete.
- The randomized differential with `witness: true`; `cargo test --locked --release -p console-workload --test paired_spans`; check-web-audioworklet.sh; check-cross-targets.sh.
- Class A: a one-time digest of dual outputs before and after (PR evidence).

## Owner rule

Only mono source files get dual-mono processing; no content detection.

## Also in this issue

Add the gate-expander to console-workload `paired_spans.rs` Part 1 chained differential (`Fx`); #973 added only its A3 obligation.

## Decision and evidence record (attempt 1, 2026-10-24)

* Scope note `scope/1872.md` accepted by root. Paths: `crates/soft-clip/src/lib.rs`,
  `tests/mono_collapse.rs` (new), `tests/randomized.rs` (`witness: true`), `tests/MUTATIONS.md`,
  `tools/console-workload/tests/paired_spans.rs`, the `channels_agree` note in
  `crates/effect-contract/src/lib.rs`, this spec. No kernel change.
* Root decision: "Also in this issue" (the gate in Part 1 `Fx`) is split to one successor that root
  files; not done here. paired_spans gains the soft clip's A3 obligation (`DualMono` only, three
  live PerLane parameters) and its id in the collapse-capable set.
* Body: `SoftClip::process::<MONO>` and `PreparedSoftClipBank::process_bank_inner::<MONO>`
  (`#[inline(always)]`); the arithmetic stays in the one `#[inline(never)]` `Channel::process`, so
  no arithmetic symbol is added. `MONO` renders the left channel on the left plane alone; the
  silent fast path reads the left plane and left rest and advances the left cursor only; D7 zeroes
  the left plane's failed lanes, recovers them on both channels and charges both counters.
* `desymmetrize_channels` copies `state`, `history.pos`, the whole `x`, `e` and `dry` rings (all 64
  double-written rows) and the ramps; `channels_agree` compares the same list bit for bit. The
  `DESIGNED` witness (bank and scalar) is the lane's 12 ramp words and 3 state words. The
  descriptor is `DualMono` only, so the collapsed body has no link term.
* Class A evidence (PR, not committed): a scratch digest of a native bank's and a scalar
  instance's dual output planes, reports and payloads over 40 blocks of asymmetric noise with
  signed zeros, subnormals, a NaN, an infinity and one-channel spans was `bda66352726e74de` on base
  `e68454874` and on this branch, debug and release. The committed corpus digests are unchanged
  (`corpus_digests.in` untouched, its tests green). That probe digest is this issue's class A
  evidence. The console benchmark's fixtures hold no soft clip: `grep -rl miso.soft-clip` over
  `fixtures/session/v1/console-*.json`, `fixtures/session/v1/parametric-eq-bank-console.json`,
  `tools/console-workload/src` and `tools/bench/src` finds nothing (the console strip is EQ +
  compressor, `scripts/check-console-benchmark-fixture.sh`). So the console `output_sha256`
  renders no soft clip and is not evidence for this effect; no timed console run was made.
* Verifier fold: the transition test keeps only what the randomized collapsed-against-dual oracle
  (`witness: true`) does not check: the right plane untouched while collapsed, agreement after the
  disengage copy, the Left-only span and the cursor-only difference. It is renamed
  `a_collapsed_bank_spares_the_right_plane_and_its_copy_makes_the_channels_agree`.
* CI fold (root ruling, 2026-10-24): `graph-compiler`'s
  `the_reduced_mono_console_from_the_970_probe_binds_at_every_width` failed on x86 at Simd4
  (1 misaligned slot, expected 0). The cause is #1872: t11 and t13 are mono-class now, and the
  session has no console, so #971's demotion decides their pool and keeps the move only when the
  re-plan binds more banks. At the build's own width the move binds the compressor bank, so the
  plan is unchanged (AArch64 Simd4: 0 misaligned, 1 bank; x86 Simd8: 0, 0). At a width the build
  does not execute nothing binds and the move is not kept, which leaves the #966 shape in the
  stereo group t0, t14, t18, t6. Root ruled this an expected-value change, not a defect, and
  authorized `crates/graph-compiler/tests/bank_levels.rs`: the every-width bind-and-render check
  stays, the native-width misaligned count stays exact at 0, the native bank count stays exact,
  and only the foreign-width row is dropped, with the reason in the test's doc.
