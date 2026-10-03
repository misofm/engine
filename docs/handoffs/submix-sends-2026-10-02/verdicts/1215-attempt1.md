# #1215 *Gate every route's coefficients through one function*: Sol verdict, attempt 1

- Reviewed: `git diff f77b6159a 6701cd55e` (12 files, +438/-43). Branch `codex/batch-submix-k3`,
  worktree `/home/bl/misofm/wt-submix-k3`.
- Binding: `AGENTS.md`, `.github/ISSUE_SPECS/1215-gate-every-routes-coefficients-through-one-function.md`
  with its Attempt 1 record, and DESIGN 5.4 and 5.7 / P11 in `docs/handoffs/submix-sends-2026-10-02/`.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1215`: OPEN, and the title
    matches the spec's H1.
  - I exported the base `f77b6159a` to `/tmp/claude-1002/v1215/src` and ran the base side of the
    gates there. Then I applied the commit's diff in place, so only the changed files got new
    mtimes, and `diff -r` against `git archive 6701cd55e` showed the tree matched. My own
    `CARGO_TARGET_DIR` was under `/tmp/claude-1002/v1215/`.
  - Mutations were scripted, then restored from backups. After restoring, all four mutated files
    diffed clean against `git show 6701cd55e:<path>`. Scratch tests were deleted.

## Verdict: PASS

**No BLOCKER and no MAJOR.** The slice is class A. I found no output bit, plan or digest that
moved. `gated_route_coefficients` is the only production derivation of a route's coefficients, and
both production readers go through it. I judge the recorded deviation (the overflow check runs on
the open fold) sound, and better than the literal alternative. Five non-blocking items follow: two
MINORs and three NITs.

## Gates (run from `6701cd55e`, x86-64-v3, all rc 0)

| Gate | Result |
|---|---|
| `check-graph-determinism.sh` | PASS (100/100). `fresh-process-determinism.json` is **`cmp`-identical** to the one I captured at base `f77b6159a` |
| `graph_fixture -- --check` | exit 0 at base and at head |
| `cargo build --locked --release -p audit`, then `audit capi` | output **`cmp`-identical** to base (`pcm_digest ff6cdcb96cdcdad5`, 0 violations) |
| `check-builtins-fixtures.sh . target/release/audit` | ok (50 files), no re-pin |
| `cargo test --locked --release -p audit -p bench -p console-workload` | 110 passed, 0 failed. No `route_folds` count was edited (the diff touches none of these crates) |
| `cargo test -p graph -p graph-compiler -p builtins-compiler --all-targets --features builtins-compiler/test-support,graph/test-support` | 308 passed, 0 failed, 4 ignored |
| `cargo test -p host-core --all-targets` (test-support features) | 167 passed, 0 failed, 2 ignored |
| `cargo clippy -p graph -p graph-compiler -p builtins-compiler --all-targets --all-features -- -D warnings` | clean |
| `cargo fmt --all -- --check` | clean |
| graph, builtins, realtime and workspace `check-`/`test-*-policy.sh` | all ok. No `Cargo.toml` or `Cargo.lock` changed, so `crates/graph` still has exactly `effect-contract`, `engine`, `lane` and `rack` (no `math`) |

Not run by me: the full test-debug-a workspace command, the release builds of `bench`, `capi` and
`session-validator` beyond what the tests above link, and `run-aarch64-tests.sh` (an x86 host; this
goes to CI `aarch64-debug` at the batch push).

The scope of what I skipped is bounded. A workspace-wide grep shows every `PreparedRoute`
constructor and destructuring site is in `graph`, `graph-compiler` and `builtins-compiler`, and I
compiled all three with `--all-targets --all-features`. `PlanningMetadata` and
`route_folds_over_program` have no callers outside `crates/graph`.

**No-bit-moved proof, independent of the fixtures.** Every checked-in route is 0 dB (gain `1.0`),
so the fixtures barely exercise the fold. I therefore ran a scratch property test, not committed:
5,000,000 random bit-pattern `RouteTransform`s, about 6% of them drawn from {±0, ±1, ±inf, NaN,
subnormal, `f32::MAX`, 1e-30}.

- `gated_route_coefficients(t, RouteGate::OPEN)` and `RouteGate::default()` both equal, bit for
  bit, a verbatim copy of the base `folded_route`.
- For all 7 non-open gates, every coefficient is either the open bits or exactly `0x0000_0000`. It
  is zero precisely when `mute`, when both lanes are zeroed, or when `follow_zeroed[index % 2]` is
  set.

## Single-function hunt

Production derivations of route coefficients in the commit:

- **`node_kind`** (`runtime.rs:4094`) binds `NodeKind::Route(gated_route_coefficients(&transform, gate))`.
- **`plain_route_gains`** (`runtime.rs:6140-6155`) also goes through `gated_route_coefficients`. It
  is the only source of `FoldLane.coefficients` in production: `route_fold` builds `FoldLane`s only
  from the `(route, gains)` candidates that `plain_route_gains` produced. Every other `FoldLane {` and
  `NodeKind::Route([..])` literal sits under `#[cfg(test)]`, after `runtime.rs:6836`.
- **`route_coefficients`** (`ids.rs`) builds on `route_transform` and `gated_route_coefficients`.
  The compiler's lowering calls it for its domain check.
- **Nothing else.** `folded_route` is deleted. No crate outside `graph` and `graph-compiler` reads
  `RouteTransform` fields except test literals. `canonical.rs` writes the unfolded bits (D3,
  unchanged). No host, SDK or JS path computes route coefficients: the only JS `10 **` is an EQ
  oracle in `sdk-response-entry.ts:606`.
- **The one independent re-derivation** is the test oracle at `program/tests.rs:1904-1907`
  (`transform.gain * transform.ll`, ...). The spec keeps it untouched, and it is useful: see M-X4
  below.

## Gate semantics

- **Columns.** `mix2x2_block` computes `l' = fma(lr', r, ll'*l)` and `r' = fma(rr', r, rl'*l)`, so
  the left input is scaled by `ll` and `rl`, and the right by `lr` and `rr`. `follow_zeroed[0]`
  zeroes `ll` and `rl`, and `[1]` zeroes `lr` and `rr`. This is correct, matching DESIGN 5.7 and
  the spec's D1.
- **Signed zeros.**
  - An open gate keeps the product's own sign: for example, `0 dB × -0.0` gives `0x8000_0000`
    (probed).
  - A gated coefficient is always `+0.0`. With a left follow, `[-0.0, 0, -0.0, 1]` gives
    `[0, 0, 0, 0x3f80_0000]` (probed).
  - A silencing gate returns `[+0.0; 4]` without reading the transform. This meets
    `RouteControlRecord::new`'s later "mute implies all `+0.0`" rule (#1220/#1221) by construction.
- **Readiness for #1216, #1217, #1218 and #1220.** The gate reaches `RuntimeParts.routes` and both
  `PlanningMetadata` impls, so later slices can do their work without touching this interface:
  - #1216 needs only the `plain_route_gains` decline, its D4;
  - #1218 sets only `follow_zeroed` at lowering;
  - #1217 and #1220 read `gate.silences()` at bind.

## The deviation: overflow is checked on the open fold

**Sound, and I would keep it.** If the check ran after gating, a muted (or follow-zeroed) route with
`700 dB × 1e10` would be accepted by `route_coefficients`, and an ack would follow. That leaves a
model that can never be unmuted, because the unmute record is refused with `Domain`. It also could
never be recompiled once unmuted: the lowering checks the open fold here, and in #1216 it checks
with `route.mute`, which is the same thing under this reading.

Checking the open fold makes "in domain" a property of `(gain_db, matrix)` alone. That is the
property the acked-batch rule needs. The committed test pins it under all 8 gates (mutation M7/X10
is red), so a later slice cannot quietly flip it.

## Mutations (each applied, run, and reverted)

| # | Mutation | Result |
|---|---|---|
| X4 | `gated_route_coefficients` open output swaps `lr` and `rl` | **RED**: `program::tests::cohort_chain_merging_preserves_dataflow_on_random_graphs` and `route_fold_shadowed_clauses_over_valid_programs`. The new test stays green, as expected: both of its sides share the function. The class-A "exact bits" hazard is held by the existing independent oracle |
| X6 | `route_coefficients` ignores `source_lane_muted` | **RED**: the new test at `route_coefficients.rs:171` (column bits) |
| X9 | lowering drops `route_coefficients` and keeps only `route_transform` | **RED**: the new test only (`left: None`, the 700 dB × 1e10 session compiles). All other graph-compiler tests are green, so this catch is **unique** |
| X10 | overflow checked on the gated output (implementer's M7) | **RED**: the new test at `:196` (muted overflow gives `Ok([0.0; 4])`) |
| X1 | `node_kind` and `plain_route_gains` bind `RouteGate::OPEN`, ignoring the gate | **GREEN** across graph, graph-compiler and builtins-compiler (22 test targets). Expected: no non-open gate can reach a plan in this slice. See MINOR 2 |

**Test value.** `every_route_coefficient_comes_from_the_one_gated_function` is the only test that
turns red if the compiler accepts a route whose `gain × coefficient` fold overflows to infinity
(X9). It is also the only one that turns red if the domain-checked layer derives or gates
coefficients differently from the runtime's function: X6, M3, and the column and signed-zero rules.
**PASS.**

## Findings (severity-ranked)

### MINOR 1: the open-fold reading is recorded only in the attempt record, not in the frozen D1 text that later slices copy

D1 here, DESIGN 5.7 and #1216's verbatim D1 copy still say "returns `Domain` if ... any folded
coefficient is not finite". Read after gating, that admits a muted overflow, the reading this
attempt rightly rejects. The test pins the behaviour, so this is not a correctness risk. But #1221's
error-order tests and the DESIGN 5.7 admission prose are written against this text.

**Fix:** amend D1 in this spec, and the D1 copy in #1216, to say "any coefficient of the **open**
fold (`gated_route_coefficients(&transform, RouteGate::OPEN)`) is not finite, so domain validity
never depends on the gate". Add the same clause to the `route_coefficients` bullet of DESIGN 5.7.
This is a doc-only change.

### MINOR 2: the gate-2 "constant the runtime binds" is a proxy, not the bound constant

The test compares against `gated_route_coefficients(&prepared.transform, prepared.gate)`, not
against what `node_kind` or `plain_route_gains` actually bind. X1 (the runtime ignores the gate) is
green today. This is acceptable for this slice, because the spec defines gate 2 this way and every
gate is open. It is first defended by #1216's gate 1 (a host-core render oracle of a muted route
with `[+0.0; 4]`) and gate 4 (no folded muted lane).

**Fix:** none for #1215. #1216's verifier should confirm that X1 turns red under #1216 gates 1 and 4
before it records a PASS. #1216's gate-2 extension inherits the same proxy, so it is not that
defence.

### NIT 1: wrong number in a test comment

`route_coefficients.rs:202` says "a unit matrix folds to a finite 3.2e34". The probed value is
`1.0000022e35`: `10^(700/20)`, and `exp2f` gives `1e35`. `3.2e34` would be 690 dB.

**Fix:** "folds to a finite 1.0e35".

### NIT 2: the lowering derives the transform twice

`compile.rs:328-330` calls `route_coefficients(..)`, which calls `route_transform` internally, then
discards the result and calls `route_transform` again for the `PreparedRoute`. The result is
bit-identical (same pure function), so this is cosmetic.

**Fix (optional):** have a crate-private `route_values(gain_db, matrix) -> Result<(RouteTransform,
[f32; 4]), RouteValueError>` back both `route_coefficients` and the lowering.

### NIT 3: `pub const fn` versus the spec's `pub fn`; one off-path import edit

- `gated_route_coefficients` is a `const fn` (as `folded_route` was). That is a strict superset of
  the frozen signature and harmless, but it is now part of the frozen interface: removing `const`
  later is a breaking change for any `const` caller.
- `builtins-compiler/src/lib.rs:4929` adds `RouteGate` to an import, outside the authorized
  ":6178 only". The literal cannot compile without it, so I record it rather than object to it.

### Informational: subnormal folded coefficients are accepted (pre-existing; not this slice's)

`route_transform` refuses a subnormal gain or matrix entry, but neither layer refuses a subnormal
*product*. `route_coefficients(-120.0, [1.2e-38, 0, 0, 1], ..)` returns `Ok([1.3e-44, ...])`, and
that was already true of `folded_route` at bind. Changing it would break class A here, and D1
specifies "not finite" only.

Before live sends ship, the owner of the domain (DESIGN 5.7, #1221) should decide whether a
subnormal coefficient is in domain. On wasm there is no FTZ.
