# Probe the unarmed SVF block form in the nightly vectorization report

Stream G follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-09 by root from the #1481 attempt-1 verdict, item (b)
(`/home/bl/misofm/submix-verdicts/1481-attempt1.md`, "Item (b), D4: ruling").

Root's ruling (2026-10-09), verbatim:

> (1) #1481: a vectorization probe for the production svf_block::<Simd8, UnarmedRest> form
> (stream G).

Smallest slice, as the verdict names it: one AVX2 probe for `svf_block::<Simd8, UnarmedRest>`, one
allowlist row with the same columns as `recursive-svf`, and the `Simd4` twin. No kernel change.

## Problem (verified on `codex/d15-batch-misc` at `e9798393e`, which carries #1481's fix)

- **Two forms, one probed.** `svf_block` (`crates/lane/src/kernels.rs:278-291`) dispatches on the
  rest thresholds: a rest plane (`&[f32]`) runs `svf_block_form::<L, true>` (the armed form, with
  the per-frame threshold load and the joint flush); `UnarmedRest` (`:77-89`) runs
  `svf_block_form::<L, false>` (`:295-312`), a separate loop with no threshold load.
- **The production instantiation.** `parametric-eq` passes `UnarmedRest` on every block where no
  lane can arm the joint flush (`crates/parametric-eq/src/lib.rs:3191` in the stereo path, `:3261`
  in the mono path). Those reach `svf_block::<L, R>` at `:1663` through `process_section`, on
  non-stationary (ramping) blocks, for sections that are not HPF/LPF. The armed form runs only on
  blocks near silence, so the unarmed form is the one every live-audio ramping block runs.
- **The report sees only the armed form.** `tools/audit/src/vectorization.rs` probes
  `svf_block::<lane::Simd8, &[f32]>` (`probe_svf_simd8`, `:76-91`) and its NEON twin
  (`probe_svf_simd4`, `:113-128`). `ACTIVE_REGISTRY` (`:42-45`) and the allowlist
  (`tools/audit/vectorization-allowlist.tsv`, the `recursive-svf` row) name only those. No probe
  instantiates `svf_block_form::<L, false>`.
- **Codegen today is clean, unguarded.** The #1481 verifier disassembled a scratch
  `svf_block::<Simd8, UnarmedRest>` probe: one 8-lane `ymm` frame loop, zero calls, zero scalar
  `*ss` ops, zero `vfmadd*`. It would pass the `recursive-svf` row. That objdump was a one-time
  observation; a regression unique to the unarmed branch (a call, a scalar fallback or a fused
  multiply-add in the `!ARMED` loop) is not seen by the nightly.

## Decisions

- **D1. One AVX2 probe.** `probe_svf_unarmed_simd8`, `#[cfg(target_feature = "avx2")]`,
  `#[inline(never)]`, beside `probe_svf_simd8`, calls
  `svf_block::<lane::Simd8, UnarmedRest>(io, PROBE_FRAMES, black_box(coefficients),
  black_box(state), UnarmedRest)` on a `&mut [f32; PROBE_FRAMES * 8]`, with the same `black_box`
  treatment of coefficients and state as `probe_svf_simd8` (no rest plane: the form loads none).
  `execute_probes` calls it once, after `probe_svf_simd8`.
- **D2. Its `Simd4` twin.** `probe_svf_unarmed_simd4`, `#[cfg(target_feature = "neon")]`, the same
  shape on `lane::Simd4`, called from the NEON block of `execute_probes`.
- **D3. Registry.** Both `ACTIVE_REGISTRY` lists gain their new name.
- **D4. One allowlist row.** `tools/audit/vectorization-allowlist.tsv` gains one `x86_64-avx2` row:
  family `recursive-svf-unarmed`, symbol `probe_svf_unarmed_simd8`, and the `recursive-svf` row's
  required, forbidden-scalar and forbidden-call columns copied unchanged. The allowlist has no
  `aarch64-neon` rows today; this slice adds none (as for the existing three probes).
- **D5. No kernel change.** `crates/lane` is not edited. If the new probe is red on the real kernel,
  the slice stops and reports the disassembly to root; it does not loosen the row.

## Authorized paths

- `tools/audit/src/vectorization.rs` (D1-D3: the two probes, `execute_probes`, `ACTIVE_REGISTRY`,
  and the import of `UnarmedRest`)
- `tools/audit/vectorization-allowlist.tsv` (D4's row)
- this spec

## Non-goals

- Any change to `crates/lane`, to the armed probe, or to the report's rules or JSON shape.
- `aarch64-neon` allowlist rows, or running the report on AArch64 (CI only; not emulated).
- New mutation cases in `scripts/test-native-vectorization-report.sh`.
- The fused builtins chain kernels' unarmed body (`crates/lane/src/kernels/builtins.rs:669` ff.):
  same per-word step, different kernel; not in this slice.

## Hazards

- The probe must stay call-free in its captured body: keep the array's static length (as #372 and
  #1481 do) so no `slice_index_fail` trampoline enters it. `UnarmedRest` reads no plane, so no
  `&rest[..]` check is emitted.
- `tools/audit/src/vectorization.rs` was last edited by J #1481 in this batch; this slice lands
  after it.

## Objective gates

1. **Green on the real kernel.** `bash scripts/run-native-vectorization-report.sh` reports
   `"status":"pass"`, `"kernel_rules":4`, `"failures":[]`; the attempt record holds the new
   probe's loop excerpt (8-lane `ymm` ops, no `call`, no `*ss`, no `vfmadd*`).
2. **The rule bites the unarmed form alone (mutation, recorded; PR evidence).** In a scratch copy,
   add an `#[inline(never)]` out-of-line call inside the `!ARMED` path of `svf_block_form`'s frame
   loop only (for example behind `if !ARMED`). The report is red with
   `recursive-svf-unarmed / probe_svf_unarmed_simd8: forbidden call 'call|callq' is present`, and
   `probe_svf_simd8` is not named in the failures. The same mutation on the tree without this
   slice reports `"status":"pass"`. Revert: green.
3. **Existing gates.** `bash scripts/test-native-vectorization-report.sh <target>/release/audit`
   prints `native vectorization red mutations: ok`; `cargo test --locked -p audit vectorization`,
   `cargo clippy --locked -p audit --all-targets -- -D warnings`, `cargo fmt --all -- --check` and
   `bash scripts/check-workspace-policy.sh` exit 0.
4. **The twin compiles.** `cargo check --locked -p audit --target aarch64-unknown-linux-gnu`
   (or `bash scripts/check-cross-targets.sh` if it covers `audit`) exits 0. Its disassembly is
   not a gate here.

*Test value.* The new allowlist row is red when the unarmed SVF loop that every live ramping
parametric-EQ block runs gains a call, a scalar fallback or a fused multiply-add; no existing probe
instantiates `svf_block_form::<L, false>`, so today nothing catches it (gate 2 records both runs).

## Evidence

- Gate 1's report line and loop excerpt; gate 2's two red/green runs; gates 3-4 outputs.

## Dependencies

- After (other streams): J #1481 (the call-free `probe_svf_simd8` and the nightly job that can
  fail).
- After (same stream): none.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and #1481's verdict item (b) first.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: under two hours.

## Attempt record
