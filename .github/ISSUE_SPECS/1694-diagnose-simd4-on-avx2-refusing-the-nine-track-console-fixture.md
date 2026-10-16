# Diagnose Simd4 on AVX2 refusing the nine-track console fixture

Found by #1344's implementer (batch 15a). Code anchors verified on the batch 15a branch
`codex/controls-batch-15a` at `f74f39951`, on an AVX2 x86-64 host (`avx2` set in `/proc/cpuinfo`;
the workspace pins `+avx2,+fma` in `.cargo/config.toml`). Issue number: assigned at filing
(written here as #1694). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a
batch review.

## Product outcome

The statement "a console slot always banks, on every target and for every track count" (decision
12; AGENTS.md) and the doc comment that says `console.slot.unbanked` is "unreachable" for a valid
session are either both true of everything the engine runs, or one of them is corrected. A reader of
`crates/graph-compiler/src/banks.rs` learns the exact condition under which a valid session is
refused, and the refusal that a caller sees names that condition.

## Context

**Reproduction (done for this spec, then removed).** A throwaway test
`crates/graph-compiler/tests/probe_tmp.rs` ran the production pipeline (`parse_session_json` ->
`compile_session` -> `prepare_native_session_effects` with `launch_native_effect_registry` ->
`prepare_session_builtins` -> `GraphCompiler::compile_with_builtins`) over the unmodified
`fixtures/session/v1/parametric-eq-nine-track.json` once per backend. It copied the caps and the
steps of `compile_bind_render` (`crates/graph-compiler/tests/bank_levels.rs:247`). Command:

```
cargo test -p graph-compiler --test probe_tmp -- --nocapture
```

Output (the probe file is deleted; `git status` is clean apart from this spec):

```
PROBE Scalar: ok, 0 bound slots
PROBE Simd4: refused [("console.slot.unbanked", "$.console.pre_insert[slot=eq].bank[pool=stereo,level=2]")]
PROBE Simd8: ok, 2 bound slots
```

The same probe over `fixtures/session/v1/console-sixty-four-track-intended.json` gives the same
refusal at `Simd4` (`$.console.pre_insert[slot=eq].bank[pool=stereo,level=2]`) and 24 bound slots
at `Simd8`. So nine tracks are not special: no track count passes at `Simd4` on this build. Nine is
only the first fixture someone ran at the foreign width. (Nine tracks do pad: eight lanes bind eight
tracks and one padded bank of eight takes the ninth, which is why `Simd8` reports 2 bound slots for
the one `eq` slot.)

**Where `Backend::Simd4` can be selected on an AVX2 build.**

- `lane::Backend` (`crates/lane/src/backend.rs:26`) always has `Simd4`; `Simd8` exists only with
  `avx2`. `Backend::current()` (`:49`) is a compile-time constant: `Simd8` where `avx2` is set. The
  doc on `Backend::VECTOR` (`:74`, the constant at `:80`) says tests iterate it so each build runs every vector width it
  has.
- `GraphCompiler::compile_with_builtins` takes `dispatch` in its public request
  (`GraphBuiltinsCompileRequest`), so any caller may pass `Simd4`. Every production caller passes
  `Backend::current()`: `crates/host-core/src/prepare.rs` (the calls at `:1246`-`:1476`; the
  `*_with_backend` variants at `:1673` and `:1714` are `pub(crate)`), and no file under `hosts/`,
  `crates/capi/src` or `crates/control-plane/src` names a `Backend` variant (grep: no hit).
- The only non-test-only selections of a foreign width are bench and test code: `tools/
  console-workload/src/lib.rs:1284-1330` (`build_with_dispatch`; it folds the console into every
  track's inserts first when `dispatch != Backend::current()`, its doc says why: "a console slot
  never renders per node on a vector backend, so the console as written is refused there"),
  `crates/host-core/src/limiter_linked_session.rs:300` (`folded(backend)`, same fold, tests at
  `:686`), and `crates/graph-compiler/tests/bank_levels.rs:1077-1087` (`FOREIGN_CONSOLE_REFUSAL`,
  `compiled_at`: a foreign width may be refused with exactly `graph console.slot.unbanked`).

**The planner path that emits the code.**

1. `bind_rack_banks_indexed` (`crates/graph-compiler/src/banks.rs:137`) collects every console and
   insert chain, forms cohort groups, padded for console racks, and for each slot calls
   `entries[0].factory.bind_homogeneous_bank(request)` (`:641`). `request.width` is
   `BankWidth::for_backend(dispatch)` (`:208`), so `Four` for `Simd4`.
2. `let Some(...) = ... else { continue; }` (`:642-646`) treats `Ok(None)` as "the factory declined":
   the slot stays unbound.
3. `unbanked_console_slot` (`:505-540`) then finds the first console node in a collected chain that
   no bound slot carries and returns `console.slot.unbanked` at
   `$.console.<section>[slot=<id>].bank[pool=<class>,level=<level>]`. `uncollected_console_slot`
   (`:548-586`) is the twin for a node the walk never collected.
4. The factory answers `Ok(None)` because it binds only the build's own width. For the EQ:
   `ParametricEqFactory::bind_homogeneous_bank` (`crates/parametric-eq/src/lib.rs:3567`) calls
   `bind_bank(request, Backend::current().width())`, and `bind_bank` (`:3581-3592`) returns
   `Ok(None)` when `request.width.lanes() != native_lanes`, with the comment "a width this build
   does not execute is a capability gap and a legal `Ok(None)`". The compressor
   (`crates/compressor/src/lib.rs:1013`), the gate/expander (`crates/gate-expander/src/lib.rs:967`),
   the transient shaper (`crates/transient-shaper/src/lib.rs:924`) and the soft clip
   (`crates/soft-clip/src/lib.rs:1133`) test the same `Backend::current().width()` condition. The
   comment above `folded` in `limiter_linked_session.rs:290-299` states it as the rule: "a build's
   factories bind only the widths it executes (D4)".

**Why the code comment is wrong as written.** `crates/graph-compiler/src/banks.rs:96-101` says
"for a valid session that is unreachable" and `unbanked_console_slot`'s doc (`:491-504`) says the
same, "it guards a regression in the planner, the binder or a factory". The comment holds only for
`dispatch == Backend::current()`. At a foreign vector width the first factory that declines the
width makes the code fire for every valid session with a console. The doc at `:502-504` names only
the `Scalar` oracle as exempt, and `docs/SESSION_SCHEMA_V1.md:230-237` says "on every vector
width". The guard, the tests that pin it (`crates/graph-compiler/src/tests/console_banking.rs:737`
and `:858`, `crates/graph-compiler/src/lib.rs:5351` and `:5914`) and `bank_levels.rs` all treat the
code as a regression guard or as the accepted foreign-width refusal; none gives the foreign-width
case its own code.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Gate 1 verdict.** Which of two cases this is. Preliminary reading of the planner owner, to be
  confirmed or overturned by the implementer with evidence:
  - **Test-only mode (the likely case).** Nothing in production selects `Simd4` on an AVX2 build:
    the compile-time `Backend::current()` is the only dispatch every host path passes, and the
    engine's rule is one lane width per build (no runtime SIMD dispatch; lane widths keyed on
    target features). Every console-eligible native factory declines a width other than the
    build's. The only callers that reach the refusal are the fold-first bench and test helpers
    above, which already treat it as expected. The defect is then the diagnostic and the comment:
    a foreign-width refusal reports `console.slot.unbanked`, which reads as "the planner lost a
    slot", and the comment says it cannot happen.
  - **Planner defect (the case to rule out).** It would hold if the engine must run `Simd4` on an
    AVX2 build: for example if any product path (the browser build is `simd128`, not AVX2, so not
    that one) or an owner decision ("two shipped bank widths", the bench's `Simd4` leg, the
    #968 resource fixtures) requires a four-lane console plan on an eight-lane build. The
    implementer greps for such a requirement and records the answer. Evidence against: the
    `limiter` binds four-lane banks on `x86-64-v3` by design, but it is not a console-eligible
    slot in any fixture here, and the console slots (EQ, compressor, ...) decline.
- **If a test-only mode: where the typed refusal lives.** The check belongs before any slot is
  tried, once, in `bind_rack_banks_indexed`: a vector `dispatch` whose `BankWidth` is not
  `BankWidth::for_backend(Backend::current())` and a session with at least one console slot is
  refused with its own code (proposal: `graph.backend.foreign_bank_width`, path `$.dispatch`),
  distinct from `console.slot.unbanked`. The `console.slot.unbanked` guard stays for the build's own
  width, where it is a true regression guard, and its comment is corrected to say so. A session
  with no console slots compiles at a foreign width as today (insert remainders render per node).
  The implementer states whether the check keys on `Backend::current()` or on a `bank_widths`
  capability the factories publish; the proposal is `Backend::current()`, because that is what the
  factories test.
- **If a planner defect: the fix shape.** The factories bind the requested width when the build can
  execute it (`Four` on any vector build; `Eight` only with `avx2`), and the tests that expected the
  refusal at a foreign width are deleted in the same issue.

## Deliverables

Common to both cases:

1. Gate 1's verdict, with its evidence, in the decision record below (this section is filled in by
   the implementer).
2. The probe above recorded as a committed test only where it defends something the existing tests
   do not (see Test value); no probe file is left behind.

If a test-only mode:

3. A typed refusal with its own code for a foreign vector width with a console, from
   `bind_rack_banks_indexed` (or one level up, at the compile entry, if the implementer shows the
   compile entry already knows the build's width); and wherever the new code is listed (the schema document's Banking paragraph names the old one).
4. The comments in `crates/graph-compiler/src/banks.rs` (`:97-104`, `:491-504`) corrected to name
   the condition: `console.slot.unbanked` is unreachable for a valid session at the build's own
   width, and a foreign width is refused earlier with the new code.
5. `docs/SESSION_SCHEMA_V1.md` (the Banking paragraph at `:230-237`) and `bank_levels.rs`
   (`FOREIGN_CONSOLE_REFUSAL`, its module doc at `:25-31`) follow the new code. The `console-workload`
   and `limiter_linked_session` fold helpers keep their fold.

If a planner defect:

3. The fix named in Decisions, and a regression test: the nine-track fixture prepares and banks
   every console slot on `Simd4` and on `Simd8`, bit-identical per lane to the scalar reference, as
   the existing parity tests (`bank_levels.rs`, "banking moved a rendered bit") require.
4. The comments in `banks.rs` and the schema document updated to say the width is no longer
   build-bound.

## Authorized paths

- `crates/graph-compiler/src/banks.rs`, `crates/graph-compiler/src/lib.rs` (its tests) and `crates/graph-compiler/src/tests/console_banking.rs`;
  `crates/graph-compiler/tests/bank_levels.rs`.
- `docs/SESSION_SCHEMA_V1.md` (Banking paragraph) and any other place the new code must be listed
  (`grep -rn console.slot.unbanked docs` finds only that paragraph today).
- If a planner defect only: the console-eligible native factories' `bind_homogeneous_bank`
  (`crates/parametric-eq`, `crates/compressor`, `crates/gate-expander`, `crates/transient-shaper`,
  `crates/soft-clip`) and their padding tests.
- This spec file's decision record.

Nothing under `hosts/`, `crates/capi`, `crates/host-core` or `tools/` changes in the test-only case.

## Non-goals

- No second console bank width on an AVX2 build as a product feature, and no runtime SIMD dispatch.
- No change to what `Backend::current()` selects or to which widths a build has.
- No per-node fallback for a console slot on a vector backend (decision 12 stands).
- No change to the `Scalar` oracle's exemption.
- No benchmark or optimisation work for a mode production does not run.

## Objective gates

1. **Verdict recorded.** The decision record states "test-only mode" or "planner defect", with the
   grep evidence for who selects a foreign width in production (none, expected) and the reproduction
   output above re-run on the implementer's tree.
2. **If a test-only mode:** at a foreign vector width, a valid session with a console is refused
   with the new code and not with `console.slot.unbanked`, at the build's own width every console
   slot still binds, and the comments in `banks.rs` match. `cargo test -p graph-compiler`,
   `-p host-core --all-features`, `-p console-workload` green; `bank_levels.rs`'s `compiled_at`
   expects the new code.
3. **If a planner defect:** the nine-track fixture prepares and banks every console slot at `Simd4`
   and at `Simd8`, with no refusal, bit-identical per lane to the scalar reference; the existing
   parity tests pass unchanged; the tests that expected the foreign-width refusal are gone.
4. `cargo clippy -D warnings` and `cargo fmt --check` on the touched crates; the realtime policy
   check; `scripts/check-workspace-policy.sh`.

## Test value

- Test-only case: a test that compiles the nine-track fixture at the foreign vector width and
  asserts the new code (and that the build's own width compiles). Plausible defect that turns it
  red and no existing test catches: the foreign-width refusal falling back to
  `console.slot.unbanked`, or the new check firing at the build's own width. The existing tests
  accept `console.slot.unbanked` at a foreign width (`compiled_at`) and so cannot see either.
  On a 4-lane (NEON/simd128) build there is no foreign vector width, so the test is a no-op there
  and says so.
- Planner-defect case: the regression test is exempt from the new-test rule while its PR records it
  red on the fix's revert, and it defends the nine-track fixture banking at both widths. The
  implementer deletes the tests that pinned the old refusal.

## Dependencies

- None. Independent of #1344, which found it and does not need it.

## Size

Half a day.

## Decision record: #1694

To be filled by the implementer. Gate 1's verdict goes first.
