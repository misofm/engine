# Sol implementation brief — issue 060 complete independent builtin corpus after recovery acceptance

## Decision and attempt budget

**STOPPED / RESCOPED — NO OVERALL PASS.** Terra attempt 1 and the single bounded Sol
correction/review are consumed. Clean checkpoint `10f0235` and the interrupted typed-checker
candidate are technical input only. Remaining response/scalar PCM, graph/PDC,
meter/diagnostic/resource and final corruption/read-only work moves to Issues 061–064 as recorded
in the Issue-060 decision record. Production DSP, audits, targets and benchmarks remain forbidden;
`workload_invocations=0` and `timed_benchmark_invocations=0`.

Own only `tools/miso-engine-builtins-fixture`, `fixtures/builtins/v1`, focused DSP-reference fixture
support if strictly needed, and Issue-060 evidence. Do not create another corpus, parser framework
or qualification tool.

## Checkpoint 1 — response reconciliation

Repair the stopped-#56 authoring/checking mismatch before touching another format class. The
Issue-035 fixed grid and Issue-059 semantics produce exactly:

- `1,470` single-section rows: `735` HPF and `735` LPF;
- `160` fixed-cascade rows for one 100-Hz HPF followed by one 1-kHz LPF;
- `1,630` response rows total, each repeated only over frozen rates, probes and quanta; and
- `1,652` `cases.toml` declarations: those exact `1,630` response IDs plus `22` functional cases.

The four rates are exactly `44100,48000,88200,96000`; quanta remain
`1,127,128,255,1024`. Keep the frozen single-section cutoff/probe construction and fixed-cascade
probe union. Response and case IDs must form the same exact set; remove the stale per-cutoff
cascade case construction rather than weakening the cross-reference.

Author candidate bytes only through an explicit scratch `--write` path, then accept them through
the read-only checker. For every legal response row, `recovery_count` is exactly `0`; rows sharing
`(rate_hz, section, cutoff_bits)` must agree. Finite subnormal canonicalizations are not recovery
events. Preserve independent RBJ/cast-state provenance, 17-significant-digit fields and the frozen
`0.005 dB` cast-state, `0.05 dB` impulse/fundamental, `-100 dB` residual, `-88 dB` attenuated-total
and nonnegative final-4096 tail gates.

Stop immediately for a nonzero legal recovery, a missing/unexpected coordinate, any numerical
gate failure or any requested production change. Do not proceed to the remaining corpus classes
until the response CSV, cases, checker tests and manifest are green and byte-stable.

## Checkpoint 2 — one complete typed corpus

Keep exactly `50` manifest-listed payloads (`51` manifest lines including the header):

- one `cases.toml`, one response CSV, one metadata TOML, one diagnostics JSONL and one resources
  JSONL;
- two meter JSONL files with exactly `7` graph-tap and `15` window/drop records;
- exactly `33` PCM payloads; and
- the ten already-accepted benchmark input TOMLs: five kinds at 48 and 96 kHz.

The 33 PCM paths remain the current frozen set: four single identity/gain/mute/filter files;
`matrix-corner.f32le` plus all 16 numbered corners; `matrix-ramp.f32le` plus the six frozen
`0,1,2,127,128,u32::MAX` rows; and one each for matrix retarget, reset, L/R isolation, partition
and graph taps. Do not add cases or payloads.

Extend the existing typed checker just enough to prove semantic completeness:

- every functional case resolves to its exact PCM/meter/diagnostic/resource payload set and no
  payload is orphaned;
- exact/closed-form identity, gain, polarity, mute, matrix, ramp and reset rules validate their
  PCM; filter/partition rows use the accepted independent retained-`f32` reference; graph taps
  preserve the seven distinct stage records and output relationship;
- metadata has exactly the current seven canonical keys and the exact four rates/five quanta;
- meter JSONL has canonical identities, ordering, bit fields, windows, counters, resets, wrap,
  drop, discontinuity, overflow and sanitation tuples;
- diagnostics has exactly the current `13` sorted typed code/path rows; resources has exactly the
  current `9` rows for tracks `1,4,65537` by meter sets `0,1,7`, with checked totals, largest
  allocation and allocation count; and
- benchmark inputs retain exact kind/rate/workload IDs, complete fields and referenced PCM hashes.

`--check FIXTURE_DIRECTORY` may only read and validate supplied bytes. Prove from source/call
structure that it cannot reach generation, production rendering or writes, and compare a complete
tree hash before/after a successful check. Scratch corruption roots must be unique and removed
after each test.

## Frozen corruption proof

Retain manifest grammar/order/safe-path/regular-file/length/hash/missing/unlisted rejection. For
each of TOML, `f32le`, CSV, meter JSONL, diagnostics JSONL and resources JSONL, execute exactly four
semantic mutations: delete, byte alter, unlisted add and manifest-valid coverage hole. The coverage
hole must remove one required tuple/path while leaving the remaining payload syntactically valid
and recomputing its manifest entry; an empty file is not sufficient evidence. Record exactly
`24/24` rejection with class/mutation/error identity.

## Ordered final gates and handoff

After both checkpoints are coherent, run focused fixture/reference tests; valid checked-corpus
read-only validation; format; warning-denied fixture/reference all-target Clippy; the applicable
nonbenchmark workspace/policy checks; and diff/static no-artifact/no-workload scans. Record exact
case/row/path/record/mutation counts, candidate and manifest hashes, tolerance maxima, recovery
total zero, and strict Terra/Sol verdicts.

Stop for production changes, a second corpus, changed formats/rates/domains/tolerances, new
functional cases, audit/target/object work, benchmark runner or workload execution, or work that
cannot close within these two checkpoints. PASS hands one immutable corpus/candidate identity to
**Builtin direct and graph realtime audit closure**; it does not itself qualify
realtime behavior, targets, performance or listening.
