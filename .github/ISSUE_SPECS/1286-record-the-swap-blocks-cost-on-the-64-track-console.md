# Record the swap block's cost on the 64-track console

Slice 16 of *Swap a rebuilt plan without an audio gap* (#1269). Tooling and evidence only: no engine
change. Code anchors verified on `54b0a1bf8` (unchanged at `24029badb`). It may run beside the
umbrella's current feature slice.

## Product outcome

The owner and the weekly performance pass get one measured answer: how much longer the render call
that applies a carrying swap takes than an ordinary render call, on the 64-track app-shape console
at native 8-lane (AVX2), the primary benchmark target. This number decides owner question Q2
(pre-roll costs `k` more ordinary blocks in the swap callback) and whether a whole-bank move
(umbrella Deferred) earns a brief.

## Context

- The native console benchmark is `tools/bench` (`src/console.rs`) over the rows of
  `tools/console-workload/src/lib.rs`, run only through `scripts/operator/run-console-benchmark.sh
  --step NAME` (one untimed warmup, two measured rounds, records under `artifacts/steps/<NAME>/`),
  with `scripts/operator/preflight-console-benchmark.sh` checking everything that can fail without
  timing. `tools/console-workload` links no host crate: it builds plans with the graph compiler
  directly, so it cannot prepare a successor.
- The successor preparation and the carry live in host-core (*Prepare a successor plan whose
  unchanged sources keep playing*, slice 3, and slices 7-14). The 64-track app-shape fixture is
  `fixtures/session/v1/console-sixty-four-track-app.json`.
- AGENTS.md: benchmarks are descriptive; freeze the workload and validator before timing; one
  invocation, one warmup, two measured rounds; no tuning, no retry; a runner defect moves to a
  tooling issue.

## Decisions frozen for this slice

- **D1. Subject.** Session A is the 64-track app-shape fixture. Session B is A plus one muted track
  whose ID sorts first, on an existing source, so every bank shifts a lane (the worst carry shape for
  lane copies). Both are prepared through host-core as the C ABI prepares them (no live controls).
- **D2. Measurement.** One run alternates A → B → A ... with one carrying swap every 32 blocks. The
  successor is prepared off the timed path. Each observation times one render call. The record
  reports the swap-block calls and the other calls as two separate distributions (p50, p90, p99,
  max), the carry's copied bytes and moves per swap, and the host and build facts the console
  records already carry.
- **D3. Home.** A new `bench swap` subcommand in `tools/bench` (add `host-core` to its dependencies)
  with its own validator, and a `--swap` mode of the operator runner, or another home the bench
  policy accepts. `bash scripts/check-bench-policy.sh` must pass either way.
- **D4. Order.** First commit: the subcommand, the validator, a short untimed self-test, and the
  preflight, with no timed run. Second commit: one operator run
  (for the console runner, `run-console-benchmark.sh` with the swap mode and `--step swap-carry-base`), its record, and a short report stating
  the swap-block overhead per carried lane. No tuning between them.

## Deliverables

1. D1-D4.
2. `artifacts/steps/swap-carry-base/` with the record and the report.
3. A comment on the umbrella with the two numbers (swap-block p50 and p99 against the ordinary
   p50 and p99).

## Authorized paths

- `tools/bench/` (new subcommand, Cargo.toml dependency)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
- new `scripts/*swap*` validator files, `scripts/check-bench-policy.sh` (only if the policy must
  learn the new subcommand), `scripts/test-bench-policy.sh`
- `artifacts/steps/swap-carry-base/`

## Non-goals

- No optimisation of the carry. A large number opens a weekly-pass issue; it is not chased here.
- No browser number (B1 covers the browser's preparation cost).

## Objective gates

1. The preflight passes and the self-test renders A and B through one swap without timing.
2. The validator refuses a record with a missing distribution, a swap count different from the
   frozen one, or a run with no carrying swap (`carry != Carried`).
3. Exactly one timed invocation, recorded with its warmup and two rounds.
4. Commands:
   - `cargo test --locked -p bench`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
   - the preflight of the home D3 chose (for the console runner, `bash scripts/operator/preflight-console-benchmark.sh` with the swap mode's flag and `--step swap-carry-base`)
   - the umbrella's inherited gates.

## Test value

- Gate 2: a run whose successors silently stopped carrying (every swap cold) would time the wrong
  thing; the validator turns red on it.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284): every state family
  carries, so the measured swap is the real one.
