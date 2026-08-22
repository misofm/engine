# Sol implementation brief — issue 066 reconcile builtin graph fixture and dependent benchmark input identities

## Decision and budget

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 066 synchronization.** Consume stopped Issue
062 checkpoint `2bbed6a` only as corrected technical input. One Terra attempt and one bounded Sol
correction/review are available; a second failure stops. Workload, benchmark-runner and timed
benchmark invocations are exactly zero.

## Frozen reconciliation

Keep the one-track 48-kHz/q128 topology in `2bbed6a`: deterministic source; conditioned HPF/LPF;
one exact 3-sample conformance delay in each of SIMD1, dynamic and SIMD2; prepared nonidentity
fader; prepared matrix; identity late route from PostMatrix; and one nonidentity early route from
PostInputBuiltins. Require report tuples `(9,0,9)` and `(0,9,9)` respectively and exactly one
9-sample inserted early-route delay.

The independent model uses retained `f32` operations in the production order, including the two
route transforms and the two-input pairwise sum. It must prove the early transformed contribution
is all positive-zero through frame 8, nonzero at frame 9, and matches every final PCM word. Derive
all seven complete meter snapshots independently and require pairwise-distinct summary tuples.

The mutable fixture-artifact set is exactly:

1. `fixtures/builtins/v1/pcm/graph-taps.f32le`;
2. `fixtures/builtins/v1/meters/graph-taps.jsonl`;
3. `fixtures/builtins/v1/benchmark/meter_success_full-48000.toml`;
4. `fixtures/builtins/v1/benchmark/meter_success_full-96000.toml`; and
5. their four rows in `fixtures/builtins/v1/MANIFEST.tsv`.

In each TOML, change only the existing `input_pcm_sha256` token to the new graph PCM hash. Do not
invent a meter-hash field; the graph-meter hash is sealed by its manifest row. Checker source and
focused tests may change only as required to validate this artifact set. Compare all other corpus
paths and all other manifest rows byte-for-byte with `2bbed6a`.

## Gates and stop rules

Author once to an explicit scratch root, pass its read-only check, and inspect the exact five-file
artifact delta before authorized checked-corpus regeneration. Require manifest-valid mutations of
one graph PCM word, one tap field, one dependent TOML hash and the exact frame-9/PDC invariant to
reject for semantic reasons. Then pass checked-corpus read-only validation, focused fixture/graph/
compiler tests, format, warning-denied focused Clippy and diff/static checks.

Stop for any production change, other payload/benchmark-input field, new schema or artifact,
audit/target/final-corruption work, benchmark runner/workload/timing, tolerance weakening or a
second failed attempt. PASS alone unblocks **Seal independent builtin corpus corruption and
read-only qualification**.
