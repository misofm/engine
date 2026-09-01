# Prelaunch benchmark schema reset: make current records V1

## Owner ruling

Benchmark and qualification records created before launch do not have a published second schema
generation. Reset every current schema-version 2 benchmark family to schema version 1 without
changing record shape or validation semantics.

## Smallest closable product slice

Reset the builtins, rack, and session benchmark producers, validators, preflight/nonbenchmark
records, fixtures, runner checks, and focused tests from schema version 2 to 1. Invalid-version
mutations use 0 or another non-product sentinel rather than inventing V2.

## Decision record

1. The version field remains because offline records need exact parser identity.
2. The sole current prelaunch record identity is schema version 1.
3. Record keys, workload definitions, cadence, measurement semantics, and digests do not change.
4. Existing schema-1 benchmark families remain unchanged.
5. Session TOML contract rejection fixtures and generic effect-state migration operands are not
   benchmark record schemas and remain successor inventory.

## Objective gates

- No current benchmark producer or validator requires or emits schema version 2.
- Builtins, rack, and session benchmark validator/preflight self-tests pass, including red
  mutations.
- Focused bench tool tests and workspace check pass.
- Record shape, measurement cadence, and digest semantics are unchanged.

## Workflow

Sol briefs and approves this stateless scope. Terra attempt 1 performs the mechanical identity
reset. Sol reviews every producer/validator pair and mutation coverage before PASS.

## Evidence record

- Sol brief approved; GitHub issue #317 matches this stateless local spec.
- Terra attempt 1 resets the builtins, rack, and session record producers plus the builtins/rack
  validators, preflights, lifecycle records, checked fixture, and test data to schema version 1.
  Invalid-version mutations use 0.
- All 29 `bench` tests pass. The builtins benchmark validator/runner/preflight lifecycle passes with
  zero real runner, workload, or timing invocations; the rack validator/lifecycle passes with zero
  audio workload launches. Formatting and diff hygiene pass.
