# Add the toggle and one-active-lane-per-bank console rows

Split from #1107 (D5 item 4, 2026-10-23). Qualification for #1782 (packing).

## Problem

Packing (swap-to-pack) needs two workloads that no row measures: a lane whose activity toggles every few blocks (the hysteresis and swap-budget case, #1107 D4-1), and a console where every bank has exactly one active lane (the best case for packing).

## Smallest closable slice

- Native: two rows in the console benchmark on the standing 64-track console, `sixty_four_track_console_toggle` and `sixty_four_track_console_one_per_bank`, with frozen input patterns, through production paths only; record count and validator updated with only rules a plausible defect can break.
- V8: the same two documents in `scripts/web-mixing-automation-benchmark.mjs`.
- One descriptive run each, before #1782's timing.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs`, the console-benchmark scripts
- `scripts/web-mixing-automation-benchmark.mjs`, `scripts/web-mixing-automation-lib.jq`
- this spec

## Objective gates

1. The preflights pass and launch no workload.
2. One native and one V8 descriptive run, attached.

## Test value

The bench unit test turns red when a row's record drifts from its validator; no other test.

## Dependencies

#1107 merged (so the rows read against the whole-bank skip).

## Decision record (2026-10-24, attempt 1)

- Root accepted the scope note's R1-R3: the one-per-bank pattern is keyed on the build's lane width (`track % Backend::current().width() != 0` is silent; the record states its backend; wasm is always four lanes); the toggle flips odd tracks only, every `TOGGLE_HALF_PERIOD_BLOCKS = 8` blocks, starting silent; the rows sit in a new `PACKING_WORKLOADS` array emitted after `CONSOLE_STRIP_WORKLOADS`, which stays frozen.
- Record names: `sixty_four_track_console_toggle` (input `odd_tracks_toggle_8_blocks`) and `sixty_four_track_console_one_per_bank` (input `one_track_per_bank`). Both are synthetic, on the standing fixture, with no warmup and no floor (`not_derived`, as the sparse row).
- The frozen driver holds a second preallocated block per claim for a toggling input and picks the phase in `begin_block` from the block's first source sample; render only selects between two slices. The graph reads `played_planes` per block (`ArenaMembers::gather_planes`), so the phase reaches every gather.
- Validator: 78 records, 50 `console_session` records (two rows times two rounds; the brief's "46 -> 48" counted rows, not records). Digest rules only: toggle != console and != sparse; one-per-bank != console and != idle. V8: `web_documents_valid` takes six documents, positions 4 and 5 pinned.
- Paths: `tools/bench/src/floor.rs` and `scripts/operator/{run,preflight}-console-benchmark.sh` changed as consequences (the floor match is exhaustive; the runner and preflight pin the record count).

### Stated limit

After real signal the standing console's compressor needs about 620 blocks to rest (#1784: 79,200 samples). With a half period of 8 blocks no toggling lane ever rests, so the toggle row cannot thrash a packer: it measures the activity test under flipping input and, after #1782, that the packer adds nothing on lanes that never rest. A row that cycles rest needs a period above the rest bound, which a 1,000-block run cannot cycle meaningfully.

## Evidence (descriptive; nothing tuned)

Both runs at `22147c7ba`, `artifacts/steps/b14-1783/`, one warmup and two measured rounds each, uncontrolled (`MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`: the machine is shared, loadavg about 9 against the 0.50 ceiling). Both preflights passed with no workload launched.

- Native (`console-benchmark.accepted.jsonl`, 78 records, validator PASS, Simd8), p50 us/block, rounds 1/2: console 100.9/100.5, sparse 101.0/101.4, toggle 99.9/100.4, one-per-bank 100.7/101.1, idle 5.6/5.4. Every bank of the toggle and one-per-bank rows holds an active lane, so the whole-bank skip (#1107) cannot serve them and they cost the console row's time, as expected before packing (#1782).
- V8 (`web-mixing-automation.jsonl`, two rounds, validator PASS), p50 us/block, rounds 1/2: console 238.0/295.2, sparse 241.3/300.2, toggle 237.1/296.5, one-per-bank 240.9/292.4. Round 2 ran about 25% slower on every document (host load); the rows are read within a round.
