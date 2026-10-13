PASS

# #1447 attempt 1: adversarial verdict

Commit `da6041810` (parent `8c0deee2c`), branch `codex/d15-stream-b3`. I exported the commit to
`/tmp/claude-1002/v1447/tree` and the parent to `/tmp/claude-1002/v1447/base`. I built in separate
target directories. I did not touch the worktree.

## Verdict summary

The acquire matches spec D1 token for token. `available` is counted once at entry. Phase 1 breaks on
the first unobserved block. `observe_seek_at_block_boundary(clock)` is called once, outside both
loops. Phase 2 is bounded by `available_at_entry().min(available - popped)`. `settle_block` is the
old loop tail without change. `note_sanitized_samples` holds no pop. The placement matches D1 and
D4: acquire, then `settle_block`, then the helper, then the blank line before
`is_unobserved_generation`. The change adds no allocation, lock, syscall, index, `expect`, `unwrap`
or panic. The `debug_assert!` is the only assertion.

I checked the one-slot argument against the code:
- `observe_seek_at_block_boundary` is the only code that pops the command queue. It is called at
  `:1054` (prepare_seek), `:1131` (begin_block_observe) and `:1430` (the acquire), and all three run
  on the consumer thread.
- `is_unobserved_generation` compares with the active generation and the held seek.
- Generations only increase.

So at the count, at most one generation is unobserved, and the phase-2 `debug_assert!` holds. In a
release build, a violation would only discard the block (counted), never cause UB.

Acked-batch question: this change adds no queue and no ack. Leaving a block that was published
after the count for the next block does not drop it.

No BLOCKER or MAJOR finding.

## Findings

### MINOR-1: the proof comment names the wrong function (`crates/source/src/lib.rs:1426`)

"Only this function pops the one-slot queue" is inside `acquire_current_block`, so "this function"
reads as the acquire. The acquire does not pop the command queue itself.
`observe_seek_at_block_boundary` pops it, and two other callers also reach it before the count:
`prepare_seek` (`:1054`) and `begin_block_observe` (`:1131`). Spec D1/D2 states it correctly ("only
`observe_seek_at_block_boundary` pops it"). This comment is the correctness argument that the
`debug_assert!` and the single re-observe depend on, so it must name the right function.

Fix: "Only `observe_seek_at_block_boundary` pops the one-slot queue, on this consumer's thread, so
...".

### MINOR-2: the D5 test catches its defect only sometimes, mostly in release (for root, not the implementer)

My measurements:

| Code under test | Profile | Red in |
|---|---|---|
| Parent + D5 | debug | 6 of 10 runs (the full-suite run was green) |
| Bound-reverted mutation | debug | 8 of 10 runs (+1 red full-suite run) |
| Parent + D5 | release | 3 of 10 runs |
| Bound-reverted mutation | release | 1 of 10 runs |

This satisfies the spec's gate 2 ("at least once"), and the spec forbids tuning without root, so this
is not a fault of attempt 1. CI runs the test in debug (test-debug-a includes `source`), where a
reintroduced capacity bound goes green in about 1 run in 3.

Advisory data (not committed):
- With `BLOCKS = 200_000`, the parent goes red in 10 of 10 debug runs and 9 of 10 release runs.
- On the head, 200,000 blocks take 0.44 s in debug and 0.26 s in release.

A deterministic test cannot be written single-threaded: the defect needs a block published during
the call, and no seam exists. A loom model of the acquire could make it exhaustive, but that is a new
CI leg (the `source` crate under `--cfg loom`), a successor issue and not an amendment here.

Recommendation to root: authorise `BLOCKS = 200_000` (or similar) as a one-line amendment.

### NIT-1: the test doc does not say the test is probabilistic (`crates/source/src/lib.rs:2395-2399`)

"Red if the acquire is bounded by the ring's capacity" states the result as certain. Add "on most
runs; it races a real host thread", so a reader who sees it green on a revert does not take it as
proof.

### NIT-2: "with no blank line between them" in the attempt record does not match the code

The attempt record says `settle_block` and the helper follow the acquire "with no blank line
between them and the blank line before `is_unobserved_generation`". The code has one blank line
between each item. The layout is correct per D1; only the wording is wrong.

Wording (#1509): the new code comments say "host", not "producer". The remaining "producer" spellings
are:
- the existing `PcmSourceProducer`, `data_producer` and `let (producer, ...)` idiom (21 other tests
  use it);
- the allowed `bench_support::producer` helper;
- "producer/render" in the spec and commit message, which means the SPSC endpoint and is not a person
  role.

No finding.

## Test value

`the_acquire_does_not_pop_a_block_published_after_its_count`: a capacity- or loop-bounded acquire
that pops a block the host publishes during the call (the producer-chase defect R1 named) turns it
red in most runs. No other test runs a concurrent host during the acquire: under the bound-reverted
mutation, every other test in the crate stayed green.

No other new test. The existing in-window seek test covers D6, and I confirmed it below.

## Gates run (all on the export of `da6041810`)

### Spec gates

**Gate 1:** `cargo test --locked -p source` passes (38 lib tests + 2 integration tests). In release
the result is the same.

Dependents also pass: `cargo test -p control-plane -p capi -p host-core -p graph` with test-support
and realtime-audit features, 500 passed, 0 failed.

Head D5 stress: 200 of 200 debug runs green, 30 of 30 green pinned to one CPU, and the full lib suite
30 of 30 green on one CPU.

**Gate 2:** the parent with only D5 added is red in 6 of 10 debug runs. Failures say "the acquire
popped 2 blocks" at blocks 563 to 17,855. See MINOR-2.

**Gate 3:**
- Markers around the head's `acquire_current_block` (which also cover `settle_block` and the
  helper): `realtime policy: ok (101 marked regions in 27 files)`. The unmarked baseline is 100, so
  the region was counted.
- Markers around `observe_seek_at_block_boundary`: ok (101).
- Control, the same markers on the parent's acquire: `marked realtime unbounded try_pop drain (bound
  it with available_at_entry)` at `:1394`.

**Gate 4 (each mutation alone; I ran the full lib suite each time):**
- Phase 2 removed: only `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm` is red.
- Re-observe removed: only that same test is red.
- Bound reverted to `self.transfer_block_count`: only D5 is red. Debug: red in the full-suite run and
  in 8 of 10 repeats. Release: 1 of 10.

**Gate 5:**
- `audit capi` on the head and on the parent: `allocations 0`, `deallocations 0`, `syscalls 0`,
  `total_violations 0`, `pcm_digest cb10fbface44a3a4` on both.
- Worklet build (`build-web-audioworklet.sh --named-twin`): the shipped module changed from base
  `99c743a6…` to head `8e2bb4ab…`, which matches the implementer's hashes (ARTIFACT CHANGED, as the
  spec expects).
- Function comparison of the two named twins (`wasm-objdump -d`, call indices normalised):
  2,800 → 2,801 functions. One function is added (`PcmSourceConsumer::settle_block`) and one changed
  (`PcmSourceConsumer::acquire_current_block`). No other function body differs.
- On the head these pass:
  - `strip-wasm-names.py --self-test` and `check`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`, including the call-graph gate;
  - `check-browser-expected-resources.py --artifacts`, with the base's pins;
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh`, with a private TMPDIR, 0 leftovers;
  - `web-audioworklet-identity.py --self-test`;
  - the V8 spill gate, self-test and module, on Node 22.23.2.
- No pin, fixture or digest changed.
- Not run locally: the identity `report` step and the three browser legs. They need GitHub run
  statuses and Playwright browsers, so they run in CI at the batch push.

**Gate 6:** fmt, `cargo clippy -p source --all-targets -D warnings`, `check-realtime-policy.sh` and
`check-workspace-policy.sh` all pass. Clippy prints the two existing `clippy.toml` `fast_db`
warnings, which the parent also prints.

### Whole `lint` job (qualification.yml)

All steps pass:
- fmt; clippy on the workspace with `--all-targets --all-features -D warnings`; doc with
  `-D warnings`;
- workspace policy and its mutations; session policy; env vocabulary; bench policy;
- test-support CI coverage and its mutations; the scalar-oracle self-test;
- script reachability and its mutations. In the plain export this step failed only because `git
  ls-files` had no repository. I re-ran it in a git-initialised copy: ok, 120 files, 20 mutation
  cases.
- console benchmark fixture; builtins fixture-manifest mutation tests; bench preconditions;
- host-core policy; protocol-control policy; realtime policy, audit-leak and artifact-evidence-leak,
  each with its mutations; realtime trace validator;
- lane policy; unfused seal and its self-test;
- check-rack-policy, builtins policy and graph policy, each with its mutations;
- effect-runtime policy and fixtures, each with its mutations;
- conformance boundaries; PEQ render-contract seal; release-shape self-test; npm publish modes;
- stem store V1; stem-identity drift;
- the sub-v3 scalar and AVX2-without-FMA refusal probes; the AVX2+FMA compile probe.

### Whole `test-release` job

All pass:
- lane, math and wasm-gates (G5, G6) with `math/lane`;
- `filter_liveness`; `tail_contract`; the `ramp_endpoint` set (8 crates);
- `designer_total --ignored`;
- the M3 FMA cfg check;
- the engine SPSC loom leg (`--cfg loom`, `spsc_loom`).

M1 and F1 are skipped, as the route intends: `math_closure` is false because no math path changed.
