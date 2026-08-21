# Sol implementation brief — issue 056 complete independent builtin corpus and corruption proof

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** Start from clean checkpoint `0edc51c` or a descendant containing
Issue 035's typed read-only fixture validator. This issue permits one Terra implementation/review
and at most one bounded Sol correction/review. A second failure stops. No benchmark binary,
runner, audit main or timing command is authorized; workload and timed invocation counts remain
zero.

## Smallest closable slice

Own only one immutable `fixtures/builtins/v1` corpus and its checker. Finish the current corpus
rather than building another generator framework. Expected numerical values must come from the
independent DSP-reference path or exact closed-form fixture rules; production builtins may be the
actual side of a test but never generate expected bytes during `--check`.

Launch rates are exactly `44100,48000,88200,96000`. Preserve Issue 035's frozen response quanta,
cutoff/probe construction, analytic/impulse/sustained/tail thresholds and functional PCM/meter/
diagnostic/resource tuples. Add only the ten frozen benchmark input TOMLs for five kinds at 48 and
96 kHz; they are inputs for Issue 058, not timing evidence and do not reduce four-rate conformance.

## Checker contract

Keep `--check FIXTURE_DIRECTORY` read-only. It must parse the manifest and payload formats,
validate exact regular relative paths/lengths/hashes, reject missing/unlisted/duplicate/unsafe
entries, prove the full tuple/row/path coverage and cross-reference cases to CSV/PCM/meter/
benchmark inputs. Static/unit proof must show `--check` has no production-generation or write
reachability. Scratch-only `--write` may help author candidate bytes but its output is accepted only
after the independent checker/oracle validates it.

Do not add general TOML/CSV/JSON schema infrastructure. Extend the existing typed V1 checker only
as required by frozen fields and identifiers. Reject malformed/unknown fields when their
acceptance would create an unproved row; avoid a parallel generic serialization library.

## Frozen proof matrix

1. Exact four-rate response grid and Issue-035 tolerances pass against independent reference data.
2. All functional PCM/meter/diagnostic/resource cases resolve to manifest-listed bytes and pass
   their existing exact/tolerance comparison.
3. Ten benchmark input bundles have exact kind/rate IDs, complete parameters and referenced PCM
   hashes; no benchmark process launches.
4. Six format classes each reject delete, byte alter, unlisted add and manifest-valid coverage
   hole: exactly 24 failures. Manifest grammar/order/path/length/hash/missing/unlisted mutations
   also reject.
5. A valid corpus check is byte-for-byte read-only. Corruption tests operate only in unique scratch
   directories and leave checked-in fixtures unchanged.

## Ordered gates and stop rules

Run focused fixture/reference format/tests/Clippy first, then applicable nonbenchmark workspace and
policy checks. Record exact file/row/mutation counts and hashes. Stop for production DSP edits,
self-generated expected values, changed rate/domain/tolerance, audit/target/benchmark work, corpus
duplication or a third attempt. PASS hands one sealed corpus/candidate identity to Issue 057; it is
not machine qualification, benchmark acceptance or listening evidence.
