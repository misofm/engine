# 107 Audit: miso-engine-session

## Outcome and readiness

Close audit finding F1: canonical TOML must be a bit-exact inverse for every finite `f32` when read
through both direct `f32` parsing and the session parser's `f64`-then-cast path, while preserving
the sign of zero.

**READY / SOL XHIGH BRIEF PASS.** This dependency-free Step-0 slice was rebriefed on 2026-08-23
against `main` at `97e1a03`. It is independent of graph Issue 123 and multiband Issue 94. One
exhaustive `2^32` release sweep is authorized only after all non-exhaustive gates are green; it may
not be retried or tuned. Issue 107 remains open afterward for later audit findings.

## Defect and canonical law

The current shortest `f32` display is not always safe when TOML first parses it as `f64` and then
casts to `f32`: the two directed bit patterns `0x15ae43fd` and `0x95ae43fd` double-round to a
neighbor. The current zero normalization also loses `-0.0`.

Move the one canonical formatter to `value::write_f32(&mut String, f32)` and use it at every
canonical float site (the current source has 19 call sites):

1. Input is finite by the existing validation boundary.
2. Append the value's normal `f32` `Display` spelling.
3. Parse only that appended suffix as `f64`, cast it to `f32`, and compare `to_bits()` with the
   original value.
4. If the bits differ, replace the suffix with `Display` of exact `f64::from(value)`.
5. If the final spelling contains none of `.`, `e`, or `E`, append `.0` for canonical TOML float
   identity.
6. Never fold `-0.0`; remove zero folding from both the old writer and `bounded_f32`.

Keep normal values on shortest `f32` spelling. Only the positive/negative `0x15ae43fd` magnitude
uses the exact-f64 fallback. Canonical negative zero is exactly `-0.0`.

## Allowed tracked paths

- `crates/miso-engine-session/src/value.rs`
- `crates/miso-engine-session/src/canonical.rs`
- `crates/miso-engine-session/tests/canonical_schema.rs`
- `docs/SESSION_SCHEMA_V1.md`
- this spec and its tracked brief
- `crates/miso-engine-session/src/estimate.rs` only if the required maximal-float compile test
  proves the current canonical upper bound insufficient; the only authorized change is `1_024` to
  `2_048` with the measured derivation recorded here

Do not touch parser, compiler, model/public API, Cargo, protocol, graph, effects, Issue 004 or any
fixture. No fixture re-pin is authorized.

## Acceptance gates

### E1 — directed double-rounding and signed zero

`known_double_rounding_values_and_signed_zero_round_trip` covers:

```text
0x15ae43fd  0x95ae43fd  0x80000000  0x00000001
0x007fffff  0x7f7fffff  0x3f800000
```

For every value, the canonical spelling parsed directly as `f32` and parsed as `f64` then cast to
`f32` reproduces the exact bits. Negative-zero text is exactly `-0.0`. A direct assertion proves
`bounded_f32(-0.0, -1.0, 1.0, ...)` retains `0x80000000`.

### E2 — ten-million deterministic patterns

Use xorshift64* with seed `0x4d49_534f_3130_37` for 10,000,000 generated bit patterns, skipping
nonfinite values. Both parse routes are bit-exact. Every spelling has a decimal point and none has
`e` or `E`.

### E3 — one exhaustive release sweep

An ignored test partitions all `2^32` bit patterns using `available_parallelism()` and `u64` range
arithmetic; it never materializes the corpus. Skip nonfinite values. Require zero mismatch through
both parse paths, exactly two fallback bit patterns, and maximum spelling length no greater than
50. Record the expected observed maximum of 48.

Run this test exactly once, after E1/E2/E4 and every ordinary gate is green. On any mismatch,
fallback count other than two, or length over 50, stop and record the first bit pattern. No retry.

### E4 — session contract and immutable fixtures

Pin the checked-in canonical contracts byte-for-byte:

- `fixtures/session/v1/canonical-minimal.toml`
- `fixtures/session/v1/canonical.toml`
- `fixtures/session/v1/parametric-eq-nine-track.toml`

Add a session containing `lr=-0.0`, `trim_db=f32::from_bits(0x15ae43fd)` and
`gain_db=f32::from_bits(0x95ae43fd)`. Reparse, compiled normalized model and second
canonicalization preserve all three bit patterns. Compile a pan-shaped track whose ten float
fields are `f32::from_bits(1)` to prove the canonical-size estimate still admits the maximal
spelling. TOML escape and duplicate-key fixtures remain untouched.

## Required red mutations

Execute separately and revert each:

1. Remove the f64-survival fallback: E1 fails on `0x15ae43fd`.
2. Restore writer zero folding: E1/E4 fail on `-0.0`.
3. Restore `bounded_f32` zero folding: E1's direct helper assertion fails.
4. Break the `.0` rule: `1.0` or E2 fails.
5. Change surrounding canonical formatting: E4 fixture equality fails.

## Required commands and budgets

Run before the exhaustive gate:

```sh
cargo test --locked --release -p miso-engine-session --lib value::
cargo test --locked -p miso-engine-session --test canonical_schema
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p miso-engine-session
bash scripts/check-session-policy.sh
bash scripts/check-workspace-policy.sh
bash scripts/test-workspace-policy.sh
git diff --check
git diff --exit-code 97e1a03 -- fixtures/session/v1
```

Then invoke exactly once:

```sh
cargo test --locked --release -p miso-engine-session --lib -- --ignored exhaustive_f32_round_trip
```

There is no benchmark, fuzz, timing, target-matrix or fixture-write invocation in this slice.

## Portability and implementation hazards

- Use `Display`, never `Debug`, and do not always emit the f64 spelling.
- Compare floats only with `to_bits()`.
- Keep exhaustive range arithmetic in `u64` so 32-bit hosts are not assumed.
- Formatting and validation are control-plane-only; no render-path behavior changes.
- If the size estimate fails, derive the exact need from the maximal-float test before applying the
  sole authorized `1_024` to `2_048` adjustment. Do not change it speculatively.

## Evidence and completion

Record base/candidate/tree and exact paths; the two fallback bits/spellings; `-0.0` spelling and
bounded-validation bits; E1/E2 totals; the single E3 invocation, fallback count and maximum length;
E4 round-trip bits and unchanged fixture hashes; every red failure; ordinary command results; size
estimate result; zero benchmark/timing/fuzz invocations; and Sol High/Sol XHigh verdicts.

After Sol XHigh PASS and an upstream green evidence commit, report F1 complete on Issue 107 but
keep it open for later waves.

## Rollback / fallback

- Any finite mismatch after the fallback is an algorithm error; stop without adding more special
  cases or changing the gate.
- Fallback count other than two or maximum spelling over 50 is STOP, not authority to weaken E3.
- Any fixture delta is STOP; never re-pin it.
- Exhaustive runner/tooling failure consumes the one invocation. Preserve its raw output and move
  runner repair to a tooling issue rather than retrying.

## Explicit non-goals

Later Issue-107 validation/allocation findings; parser or public schema changes; fixture changes;
render behavior; benchmarks, timing, fuzzing, target matrix or performance claims.
