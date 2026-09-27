# Adversarial verification of EQ briefs #976-#980

Tree: `codex/batch-plumbing-floor-2` at `457cfc84`. `crates/parametric-eq` is byte-identical to the
briefs' base `6ca203f8`, so every `lib.rs` line the briefs cite still matches. The work ran in a
detached scratch worktree with its own target directories, and the worktree has since been
removed. Host: EPYC 7313P (Zen 3). The runs were pinned to cpu 31 under `timing.lock`, each hold
lasting 14 s. Evidence files (the tests, patches, raw timing and digests) are in
`scratchpad/verify-eq-evidence/`.

## Findings, most severe first

### 1. HIGH: #979 as briefed is not class A (the inert upper bound is wrong)

The brief admits a dead section whose integrator magnitude lies in `[FLUSH_EPS, 0x7f80_0000)`. Its
proof says "`|v0| <= 1e30` cannot push a finite `ic2` past `f32::MAX`". That bound holds only for
section 0. A dead section in the middle of the cascade takes a live section's output, and leg (a)
does not bound that output: a +24 dB bell turns a 9e29 input into about 1.4e31. Once
`|v0| >= 2^103` (1.0141e31, half an ulp of `f32::MAX`) and `ic2 = -f32::MAX`, the executed identity
section computes `v3 = v0 - ic2 = inf`. From there `d1 = 0*inf = NaN`, so both `y` and the state
are NaN. The elided section passes `v0` through unchanged.

Reproduction: `zz_verify.rs::zz_979_large_inert_state`, public API, scalar.
- Restore `ic2 = -f32::MAX` into the disabled band 1 (physical section 2). `restore_track` accepts
  any finite integrator (`lib.rs:1904`).
- Put a +24 dB bell at 12 kHz ahead of it and a 10 Hz LPF behind it.
- Feed an admitted 9e29 sine at 12 kHz.

| arm | digest | failures |
|---|---|---|
| base | `3dbea8f1…` | block 0 left: zeroed, reset, reported |
| #979 as briefed | `df8ec34d…` | none (renders audio) |
| #979 capped at `ELISION_MAGNITUDE_CEILING` | `3dbea8f1…` | same as base |

The fix is to cap inert magnitudes at `ELISION_MAGNITUDE_CEILING` (1e30 < 2^103). Then
`v0 - ic2` is finite for every finite `v0`. A non-finite `v0` gives a non-finite result in both
arms on the same lane, so the 4.4 verdict, the zeroing, the reset and `nonfinite_lane_mask` all
stay identical. The capped rule still reproduces the cliff fix: `diag_disable_cliff` shows
`(false,true,0)` and `(true,true,9)`, and the relaxed arm equals the full cascade in dev and in
release. States frozen by the kernel are always `+0.0` or at least `FLUSH_EPS`.

Other #979 findings:
- `a_non_zero_state_in_a_dead_section_refuses_elision` (`lib.rs:4140`) fails under #979, because
  its `1.0` case becomes admissible. The brief does not list this test.
- The gate at `:3396` also fails. The brief does foresee that one.

### 2. HIGH: #977 as briefed is not class A (the "for any finite state" premise is unproven)

A dry lane's state can become NaN and stay NaN. The sequence is:
1. Restore `ic2 = -f32::MAX` on a lane whose HPF is disabled, while the HPF is live on the other
   channel.
2. Render a refused block with one word of 1.1e31 (at least 2^103) on that lane. `v3` overflows,
   so `ic1` and `ic2` become `0xffc00000`, and that NaN is published in the snapshot.
3. The dry output (1.1e31) passes through a 10 Hz LPF and comes out below 1e30, so the 4.4 check
   passes and no reset happens.

Leg (c) (`lane_has_no_negative_zero`, `lib.rs:965`) rejects only `-0.0`, so every later block is
admitted. The select-free kernel then emits `0*NaN + x = NaN` on the dry lane.

Reproduction: `zz_977_poisoned_dry_lane`.

| arm | digest | failures |
|---|---|---|
| base | `fd6842fe…` | none |
| EQ-1 + #977 as briefed | `2a9b06f8…` | block 2 left: zeroed, reset, reported |
| + leg (c) also requires finite integrators | `fd6842fe…` | none |

With that amendment, every scenario in this file equals base, and the full suite fails only on
EQ-1's five pins.

The case of a dry LPF lane that overflows needs `|x| >= 2^103`. The LPF is the last section, so
`x` is the final output and fails 4.4 in both arms with the same lane mask
(`nonfinite_lane_mask` also flags `|x| >= BLOCK_LIMIT`). That case is therefore exact.

On the rest of the question, the select is dead on exactly the admitted blocks:
- A ramp never reaches the interleaved path, because `stationary` requires `no_ramp_in_flight` on
  both channels.
- After an enable, a disable or a parameter change, the next stationary block's dry lanes hold the
  exact identity words with `remaining == 0`.
- The state update never reads the mask (`kernels.rs:294`).
- `length < 6` is equivalent to "the gate admitted" once EQ-1 lands.

Contract 2 therefore changes: `cascade_sections` and `cascade_sections_mono` gain the finiteness
term.

### 3. MEDIUM: counter gates in integration tests cannot compile

#976 M3 and #977 gate 2, M2 and M3 read a `#[cfg(test)]` counter from `tests/bank.rs`. `cfg(test)`
code in `lib.rs` is not compiled for integration tests. The counter should use the existing
pattern at `lib.rs:122-148`: `#[cfg(any(test, feature = "test-support"))]` plus a
`#[cfg(feature = "test-support")] pub fn test_only_*`, with the gate run under
`cargo test -p parametric-eq --features test-support`. The alternative is to assert the counter
from an in-crate test.

### 4. MEDIUM: #976 gate 3 digest mechanics

- The brief says to "apply eq-diagnosis-prototypes.patch" for the digests harness. That patch
  rewrites `lib.rs` with diagnostic flags, conflicts with the change under test, and leaves the
  in-crate tests uncompiled.
- A flag-free harness is 20 lines: `zz_eq_verify.rs::zz_digests`.
- An x86 build refuses four-lane EQ banks (`lib.rs:2400`), so the native `Simd4` digests never run
  a four-lane EQ bank. The wasm guest digests (`DIGESTS=1 node wasm_variants.mjs`) are the only
  coverage of the browser shape and should be mandatory.
- In the guest arm, the pushed source makes row 2 (console) and row 8 (idle) hash identically
  (`43574fed…`). Rows 9 and 10, and rows 12 and 13, are twins by design. That leaves 12 distinct
  wasm digests, not 15.

### 5. LOW

- **#976, live=5.** Five live sections used to skip the gate (`kept = 6`). Now they pay the scan
  and elide one section. This is exact but a new engagement class, so gate 1 should cover a dead
  section in the first, middle and last position.
- **#976, floor formula.** The depth-1 tail costs 25 ops, not 24, when its mask is non-empty.
- **#976, mono M1.** A mutation that restores rounding only in `cascade_sections_mono` is not
  caught by M1, so add a mono `kept == live` assert.
- **#978, wasm gate.** Gate 5's "two-band variant" needs a workload outside the authorized paths.
  Say it is scratch-only and not committed.
- **#978, mono skew.** Skew at `S = 1` (mono) was never measured: the diagnosis table has no
  "S1 D2 skewed" column. Either measure it or keep `interleave_mono` on today's kernel.
- **#980.** State the initial values `nearest = u32::MAX` and `largest = 0`.
- **All five, "both NaN, or equal bits".** NaN state words are published (see finding 2). Every
  digest here hashed raw bits, including NaN, and matched. Gates can therefore demand exact bits
  for state words.

### 6. Out of scope, relevant to the #888 amendment

A fault on one lane zeroes and resets all W lanes of an EQ bank (`lib.rs:2120-2121` works on the
whole plane). Per-node rendering zeroes only the faulty track.

Verified with `zz_888_one_lane_fault_zeroes_the_bank`: inf on track 3 left bank lane 0 zeroed,
while scalar track 0 was not zeroed.

Partial cohorts must also exclude absent lanes from the 4.4 verdict, zeroing and reset, and from
gate (a). Identity coefficients are not enough, and dry selection alone is not enough either.

## What was verified sound

### #976 (EQ-1): exact

- The padding is a dead section that leg (b) already holds at `+0.0`, so the existing proof covers
  dropping it.
- Pairing does change (live `{1, 2, 3}` used to run `(0,1)(2,3)` and now runs `(1,2)` then `(3)`).
  Each (stream, section) chain still runs `svf_step`, then the mix, then `choose` in the same order
  (`kernels.rs:259-297`). Values are handed between sections through a register or through an exact
  f32 store and load.
- There is no latency change.
- The full dual and mono implementation (`eq1.patch`) passed these checks:
  - Exactly the five listed tests fail, in dev and in release.
  - The seven-shape hostile scenario (1, 2, 3 and 5 live sections, dry lanes, subnormals, `-0.0`,
    inf) at bank W8 and scalar equals base in dev and in release.
  - All 45 native digests and all 15 wasm digests are identical.
  - `KERNEL_ROSTER` rule 1 holds (dual vector 312, collapsed 156, 14 kernels, rule 3 ok).
  - The `miso_engine_web_v1_render` callgraph is unchanged (`closure=8 traps=5`).
  - The realtime, lane and EQ render-contract scripts pass.
- `check-builtins-fixtures.sh` covers `fixtures/builtins` only, not the EQ. No EQ PCM golden
  fixtures exist.

Timing A/B, clean builds, three alternating rounds:

| row | base | nopad | change | brief |
|---|---:|---:|---:|---:|
| native isolate | 15.51, 14.85, 14.95 us | 9.36, 9.71, 8.80 us | median −5.6 us | −5.8 |
| wasm isolate (V8) | 43.5 us | 20.2 us | −23.3 us | −22.4 |
| wasm `eq_only` row | 94.1 us | 71.3 us | −24 % | |

### #978 (skew): purely a schedule

- The prototype was bit-identical in dev and in release on all scenarios, in the full EQ suite, and
  on all 45 + 15 digests.
- Roster ok (vector 384). The callgraph is unchanged, with no new trap owner.
- Wasm isolate went from 43.6 to 37.6 us (−6.0; the brief says −6.3). Native went from 14.97 to
  13.8 us (−1.2; the brief says −1.3).
- Wasm fp is IEEE-deterministic apart from NaN payloads, so a V8 reschedule cannot move non-NaN
  bits.

### #980: the same predicate

- Exhaustive over all 2^32 words at one position (55 s in release): 0 mismatches. The predicate is
  a per-word conjunction, so this covers every block.
- `+0.0` gives `0x80000000` after the xor, which is not 0; `-0.0` gives 0. NaN and inf have
  magnitude bits of at least `0x7f800000`, which is above the ceiling.
- Cost fell from 423 to 301 cycles per two planes. The loop uses `vpminud`/`vpmaxud`.

## Recommended order

1. #980 can land at any time, in parallel.
2. #976.
3. #977 with the leg (c) finiteness term.
4. #978.
5. #979 with the cap. It edits the same legs and proof doc in `cascade_sections` as #976 and #977,
   so sequence it after them and do not run it in parallel.

None of these should merge. Keep #976 and #977 separate: #977 carries the correctness risk.
