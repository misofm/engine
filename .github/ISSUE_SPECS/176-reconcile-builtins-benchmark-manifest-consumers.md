# Builtins benchmark identity pins stale since efcfa0f: preflight cannot pass on main

## Approved current scope — 2026-09-11

Correct the CURRENT manifest-only mismatch, not the historical three-pin description.
Actual fixtures/builtins/v1/MANIFEST.tsv SHA-256 is
31798260263396c242c0b90042e01abb18624f383fd88029341dffecde662796; tools/bench already
uses it and required CI exercises its input-hash test. PCM508c8e94 and meter958a7026
identities already match; preserve all fixture bytes. Remaining outdated ddb4b201
consumers are scripts/preflight-builtins-benchmark.sh, scripts/test-builtins-benchmark.sh
(including its stub/seal rows), scripts/builtins-benchmark-record-validator.jq and
tools/audit/src/builtins_graph.rs. Reconcile those exact provenance values. Add one
cheap real-tree consumer-hash regression with a stale-constant negative control,
using existing test/preflight machinery and required CI wiring only if needed. Do
not repin unrelated validator/source seals or launch timed work; if existing runner
defects demand broader repair, preserve evidence and report before expanding scope.
Verify real-tree freshness, existing hermetic serializer/validator controls and
focused audit tests/Clippy appropriate to the constant change. Explain current
versus historical provenance; no claim that all benchmark infrastructure is fixed.

Astra LOW implements; Astra XHIGH independently verifies. Five attempts maximum.
Root commits exact paths at each coherent compiling/focused-green tranche and pushes
promptly before more edits. At most two active issues (#285/#176); isolated worktrees
and no overlapping paths. No timed benchmark, fixture regeneration, compiler captures,
DSP/runtime change, or performance claim. Preserve actual commands/environment/source/
exit/output evidence externally; ordinary compiler feedback is corrected within the
unfinished pass, while substantive failed gates receive bounded adversarial review.
Required exact-head PR and main qualification, upstream evidence and verified GitHub
closure precede clean delivered-worktree removal. Root owns delivery and any artifact
qualification; these tooling-only slices should not require a new shipped pin.

## Attempt 1 implementation record — 2026-09-11

The current mismatch is manifest provenance only: the four outdated consumers now
use `31798260263396c242c0b90042e01abb18624f383fd88029341dffecde662796`,
matching the unchanged real manifest and the already-current bench consumer. The
historical three-pin account below describes an earlier tree, not this repair.
Fixture bytes and graph PCM/meter pins are unchanged.

The existing benchmark test script now exposes `--check-manifest-consumers`, a
no-build real-tree check of seven declarations across five consumers (including
the synthetic preflight hash stub and seal row). It computes the manifest digest,
requires exactly one matching declaration per selector, and rejects a separately
mutated stale constant through the same predicate for every declaration. Required
qualification runs only this cheap mode; the full existing hermetic lifecycle
suite remains available without running a real benchmark.

No historical lock, validator, source, or artifact seals were repinned. This repair
does not claim that the historical exactly-once preflight is runnable on the current
branch or that all benchmark infrastructure is repaired. No fixture generation,
timed benchmark, DSP change, or performance claim belongs to this checkpoint.
Validation command/output evidence is preserved externally by the implementation
workflow; independent review and delivery evidence follow separately.

## Historical issue body

## Summary

Three builtins-benchmark identity pins have been stale since commit `efcfa0f` ("wip: re-pin builtins fixtures from the reference oracles"), which moved `fixtures/builtins/v1/` but did not update the consumers that pin its hashes. On `origin/main` at `edbbeeb` the builtins benchmark preflight could not have passed.

## Evidence (measured on `origin/main`, before any #163 phase-2 change)

| consumer pin | expected | actual file hash on main |
|---|---|---|
| `scripts/preflight-builtins-benchmark.sh` `manifest_sha256` | `bfcc7bbe…` | `c33781cf…` |
| `scripts/preflight-builtins-benchmark.sh` `graph_pcm_sha256` | `508c8e94…` | `e6294eba…` |
| `scripts/preflight-builtins-benchmark.sh` `graph_meter_sha256` | `958a7026…` | `03cc3979…` |

The same three values are duplicated in `scripts/test-builtins-benchmark.sh`, `scripts/builtins-benchmark-record-validator.jq`, `tools/miso-engine-audit/src/builtins_graph.rs` and `tools/miso-engine-bench/src/builtins.rs`.

`preflight-builtins-benchmark.sh` calls `require_hash fixtures/builtins/v1/MANIFEST.tsv "$manifest_sha256"`, so it fails immediately. It went unnoticed because `preflight-*.sh` is deliberately outside `scripts/sweep.sh` (it is minutes rather than seconds, and every gate it calls is an independent sweep row), and no builtins benchmark has been authorised since `efcfa0f`.

## The corroboration that identifies the cause

Regenerating the fixtures under the #163 phase-2 unfused numeric contract returns **two of the three** to the exact values the consumers still expected, byte for byte and without those constants being touched:

* `fixtures/builtins/v1/pcm/graph-taps.f32le` → `508c8e94…`
* `fixtures/builtins/v1/meters/graph-taps.jsonl` → `958a7026…`

Both are byte-identical to the fixture as it stood at commit `092ded7`, one commit before `efcfa0f`. That is independent evidence that `efcfa0f` re-pinned the fixtures against **fused** arithmetic while its consumers kept the pre-fusion bits, rather than the two sides having drifted for unrelated reasons.

Only `MANIFEST.tsv` needed a genuinely new value, because it also covers `pcm/filters-asymmetric.f32le`, `pcm/partition.f32le` and the four benchmark descriptors, which differ from the pre-`efcfa0f` era for other reasons.

## Disposition

The stale pins are repaired on branch `floor-phase2-contract` in commit `6fa5453`, because that commit's fixture regeneration touches the same files and leaving them knowingly divergent across it was not an option. They are **not** a phase-2 change and should be reviewed as a separate concern: the phase-2 contract change did not cause them, and would not have surfaced them if it had not regenerated the same corpus.

## Follow-up worth considering

1. The three values live in five places each. A single source (the manifest row itself, read at run time) would make this class of rot impossible.
2. `preflight-*.sh` being outside the sweep is a deliberate, documented cost decision. It is correct for runtime, but it means a preflight can rot silently for months. A cheap hermetic row that checks only the `require_hash` constants against the working tree — no build, no launch — would have caught this the day it landed.

## Independent review and joint delivery checkpoint

Astra XHIGH attempt1 SOURCE PASS atcc11fea32076d275647256f5028c959424d64963.
The real CLI rejects the original stale preflight pin and passes after exact
restoration. Required CI routing contract/mutations pass. All builtins fixture
tree objects and PCM/meter identities are unchanged; LOW lifecycle/audit/Clippy
receipts match the committed diff and carry forward. Source/GH/current-base
parity verified. Evidence: /tmp/issue176-xhigh-cc11fea3/review.md.

Root merges the two accepted histories for one tooling delivery. No shipped
engine/SDK source or artifact pin changed. Combined exact-head review, required
PR/main qualification and synchronized closure remain pending.
