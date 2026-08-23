# 107 Audit: miso-engine-session

## Outcome and readiness

Close audit finding F1: canonical TOML must be a bit-exact inverse for every finite `f32` when read
through both direct `f32` parsing and the session parser's `f64`-then-cast path, while preserving
the sign of zero.

**F1 COMPLETE / SOL XHIGH PASS.** This dependency-free Step-0 slice was rebriefed on 2026-08-23
against `main` at `97e1a03`. Sol High implemented it at `0acfbc8` and Sol XHigh returned strict
PASS on attempt 1. The one exhaustive `2^32` release sweep ran only after all non-exhaustive gates
were green and was not retried or tuned. Issue 107 remains open for later audit findings.

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

## F1 implementation and Sol XHigh evidence — 2026-08-23

Sol High candidate `0acfbc87d59e3472b1216beddaa484cdc6529d3e` (tree
`76eb7047bdfa5682fe302ac7c494982955301cf3`) received strict Sol XHigh PASS on attempt 1. Its
exact binary diff SHA-256 from the rebrief checkpoint is
`42f78cca153226ef68d1de2fcb64121f3345437ec917338771a6fe51a35eaca9`.

Exact changed paths and SHA-256 identities are:

- `crates/miso-engine-session/src/value.rs`:
  `4f6ec6f76648b5a1c4352f5753339d246057f76308531b15d16d4c804adb8aed`;
- `crates/miso-engine-session/src/canonical.rs`:
  `7d94bea4510eee72c1a74f43cb08fc19455a56cc3a1eae88ce37ee8779a16532`;
- `crates/miso-engine-session/tests/canonical_schema.rs`:
  `d3c5c8cd3ead3d6b2a73d1fb79f0072a46e2a1e12e663c2e33a4c1632f5cbe6f`;
- `docs/SESSION_SCHEMA_V1.md`:
  `48dc22dc372df905aefc4dc938040138ef69c3f18dc07db1ce63c3c99126f488`.

All 19 canonical float sites use `write_f32`. The independently checked exact-f64 fallbacks are
`0x15ae43fd -> 0.00000000000000000000000007038530691851209` and
`0x95ae43fd -> -0.00000000000000000000000007038530691851209`; normal values retain shortest f32
Display. Direct and f64-then-f32 parsing, `.0` identity, writer `-0.0` and `bounded_f32(-0.0)` all
preserve exact bits.

The directed gate and 10,000,000-pattern gate passed. After every ordinary command was green, the
ignored exhaustive release gate ran exactly once over all `2^32` patterns using `u64` partitions:
zero mismatch, exactly two fallbacks and maximum spelling length 48. Invocation count is one and
retry count zero; Sol XHigh did not rerun it.

The signed-zero/double-rounding session reparsed, compiled and recanonicalized bit-exactly. The
ten-field maximal-float session proved the existing canonical size estimate sufficient, so
`estimate.rs` did not change. The three canonical fixtures recanonicalized byte-exactly. The
intentionally noncanonical parametric-EQ fixture remained exactly 9,475 bytes with its frozen
FNV-1a identity, and the exact Git fixture diff was empty. No fixture was re-pinned.

Executed/reverted mutations removed fallback (`0x15ae43fd` became `0x15ae43fe` through session
parsing), folded writer zero, folded bounded zero, removed `.0`, and changed a surrounding
canonical separator. Their E1/E4/fixture gates failed exactly as intended; no mutation remains.

PASS commands include release value tests, canonical-schema tests, formatting, warning-denied
workspace all-target/all-feature Clippy, full workspace all-target tests, warning-denied rustdoc,
session/workspace policies and mutation tests, diff check and exact fixture diff. No benchmark,
fuzz, timing or target-matrix invocation ran.

This evidence completes only Issue-107 F1. Later session findings remain open; Issue 107 must not
close at this checkpoint.
