# Soft-clip refuses its own subnormal snapshot on restore

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 3", seed 0).

## Problem

Soft-clip can produce an in-memory state snapshot containing a subnormal value, then refuses that same snapshot when it is restored (for example across a plan replacement). That breaks the snapshot/restore round trip and the effect's documented denormal behaviour: a state the effect produced must restore.

## Smallest closable slice

Either flush the subnormal when the snapshot is taken, consistent with the effect's denormal rule, or accept it on restore; state which and why against the effect contract. Un-ignore the reproducer.

## Gates

1. The reproducer passes; a snapshot/restore round trip is identity for every state the effect can produce, subnormals included.
2. Hostile (not self-produced) snapshots are still validated as before.
3. Console digests unchanged; render allocation-free.

## Attempt record

### Attempt 1 (implementer)

**Decision: accept on restore, not flush on snapshot.** The restore now accepts exactly the words
the effect itself can hold and still refuses the rest:

- ramp current and target: finite, not `-0.0`, inside the converted domain. The subnormal clause
  is gone; only the mix's `[0, 1]` can hold a subnormal (the gains' converted ranges start at
  `-24 dB`, a normal gain, so a subnormal gain is still refused as out of domain);
- ramp step: finite (a ramp toward or from a subnormal mix divides a subnormal difference);
- `X` and `e` histories: finite and zero-or-normal as before, because the kernel flushes both
  before they enter a history (D7), so a subnormal there is never self-produced;
- dry history: finite. It holds the input unflushed by design (the identity path reproduces any
  input sample), so a subnormal input sample is legal state.

Why not flush on snapshot: a flushed dry sample or mix changes rendered bits (a subnormal dry
sample is the output at `mix == 0`), which would break #1278 D2a's bit-exact continuation of a
restored lane. Accepting on restore keeps the payload layout (version 1) unchanged, so no fixture
moves.

Changes: `crates/soft-clip/src/lib.rs` (`converted_value_valid`, `decode_lane_words` and their
docs); `crates/soft-clip/tests/state_roundtrip.rs` (new
`a_snapshot_holding_subnormal_words_restores_and_continues_bit_for_bit`; three hostile words added
to the rejection test: subnormal `X` history word 12, subnormal drive current word 0, NaN dry word
73); `crates/soft-clip/tests/randomized.rs` (per-PR differential now `known: &[]`; the ignored
twin, which differed only by seed count, is deleted as superseded);
`crates/conformance/src/randomized.rs` (`Known::SubnormalStateRefusedOnRestore`, its two
narrowing sites and `SUBNORMAL_REFUSAL_CODES` removed; no other effect used them).

Mutation evidence (each applied, run, reverted; green after revert):

| # | Mutation | Test | Result |
|---|---|---|---|
| M1 | revert `lib.rs` to `41517fc35` | `randomized` (narrowing removed) | RED: `lane 0 refused its own snapshot` (Eight, block 9) |
| M1b | same | `state_roundtrip` new test | RED (restore refused) |
| M3 | treat `X` history as unflushed (accept subnormal) | rejection test, `bad(12, 1)` | RED |
| M4 | drop the dry history's finiteness check | rejection test, `bad(73, NaN)` | RED |
| M6 | keep `normal_or_zero(step)` | new test (subnormal mix step) | RED |
| M7 | treat dry history as flushed (refuse subnormal) | new test (subnormal newest dry) | RED |
| M5 | refuse subnormal only for the gains | all | GREEN, as expected: a subnormal gain is out of domain either way (equivalent mutant) |

Gates:

1. Reproducer passes: `cargo test -p soft-clip --test randomized` with `known: &[]` green; also
   `MISO_ENGINE_RANDOMIZED_SCALE=3` (24 seeds, as the deleted twin ran): green, 75 restores.
   Every other effect's `--test randomized` green.
2. Hostile snapshots still refused: the rejection test, with the three added words, green.
3. `cargo test -p console-workload --release` green (every digest row unchanged); soft-clip's
   `allocation` test green; `audit capi` 0 allocations, 0 syscalls. `cargo test -p soft-clip -p
   conformance` green; `conformance_fixtures --check` exit 0.

Inherited gates: fmt, clippy (workspace, all targets/features, `-D warnings`), rustdoc
`-D warnings`, `check-/test-workspace-policy`, `check-/test-realtime-policy`, `check-capi-abi`,
`check-cross-targets` (PASS, #1018 expected iOS failures only): all green.

Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`: all green.
**ARTIFACT CHANGED**: shipped module `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4`
(built with `lib.rs` at `41517fc35`) becomes
`726f429104606359f9eddaac0001b3a01936501287411d8e2329cc16dddbe4d7`. Per `docs/RELEASE.md` no
per-change re-pin; the pin file is untouched.
