# 104 Audit: tools/* benchmark, audit, and fixture crates and scripts/ validators

One-line summary: Consolidate the tools and script validators: one benchmark runner contract, one audit harness, one fixture generator family.

**Authority: GitHub issue #104 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 104` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.

## Decision record

### Phase A -- stale-red script triage (2026-08-24)

Ten `scripts/check-*.sh` were red on `main` at `05a0822` with an unmodified tree. `#83` wave-4
decision W4-D2 governs the class: *an unrefreshable source-seal script is red-expected and must be
retired or converted, never "fixed" by re-sealing it.* Re-sealing is the one forbidden repair,
because a seal asserts "the accepted evidence was produced from these bytes" and recomputing it
against a tree that never produced the evidence turns a true statement into a false one.

| script | disposition | why it was red |
|---|---|---|
| `check-builtins-benchmark-109.sh` | retired | sha256 pins on `Cargo.lock`, `tools/miso-engine-builtins-bench/{Cargo.toml,src/main.rs}`, five `scripts/*-builtins-benchmark*.sh`, three fixture files, plus the byte identity of seven `target/issue72/` build artifacts. `Cargo.lock` changed in wave 1; `target/issue72/` has never existed in a fresh checkout. |
| `check-builtins-benchmark-110.sh` | retired | the 109 seal plus the sha256 of the five Issue-109 scripts and the `target/issue109/` repair seal. |
| `check-builtins-listening-033.sh` | retired | sha256 pins on `Cargo.lock`, `crates/miso-engine-builtins{,-compiler}/src/lib.rs` (both rewritten by the lane waves) and seven `target/issue110/` artifacts. |
| `check-builtins-listening-111.sh` | retired | sha256 pins on `Cargo.lock`, sixteen frozen paths and the same seven `target/issue110/` artifacts. |
| `check-effect-interchange-benchmark-108.sh` | converted | the seal half (Cargo.lock, the bench package manifest, the `ACCEPTED.sha256` identity+payload, six `target/issue081/` artifacts) is retired; the live half -- the four-rate migration envelope, the four workloads and their expected output digests, the focused regression tokens -- stays, and `test-effect-interchange-benchmark-108-policy.sh` still proves each of them red. |
| `check-effect-interchange-qualification.sh` | converted | `fixtures/effect-interchange/v1/ACCEPTED.sha256` sealed twelve `crates/miso-engine-effect-{compiler,package}` sources next to the corpus; waves 1-4 rewrote six of them. The twelve source rows are retired, the manifest is now exactly the 24 corpus/reference-script rows (all of which still verify), and its self-pin is re-stated so a silent refresh is still a failure. |
| `check-effect-interchange-targets.sh` | converted (transitively) | its only failure was the `check-effect-interchange-qualification.sh` call in its first line; the five-row native/Android/iOS/Wasm matrix it owns was always live. Unchanged file. |
| `check-capi-qualification-v1.sh` | converted | `sha256sum --check` over `fixtures/capi-qualification/v1/AUTHORITIES.sha256` and `EVIDENCE.sha256`: eleven of the twenty-six sealed paths were rewritten by #102/#103 and the lane waves. Both seals are retired and replaced by manifest *shape* gates (sort order, 26-row membership, `<64-hex>  <path>` rows). The evidence checker and the MATRIX row/evidence digests still run and still pass. |
| `check-builtins-fixtures.sh` | fixed | two drifts, no seal: (1) the manifest is byte-sorted but `[[ a > b ]]` and `sort` follow the caller's collation, so under `en_US.UTF-8` `pcm/matrix-ramp-1.f32le` sorted after `pcm/matrix-ramp-127.f32le`; (2) `tools/miso-engine-builtins-fixture` gained a second `[[bin]]`, so the bare `cargo run --manifest-path` became ambiguous. `export LC_ALL=C` and `--bin miso_engine_builtins_fixture`. |
| `check-web-audioworklet.sh` | fixed | not broken: it requires an `ARTIFACT_DIRECTORY` and exits 2 without one, which reads as red in any sweep that runs every `check-*.sh`. It now builds the artifact itself when called with no argument; CI keeps passing the directory it already built. |

Retired with the four retired checkers, because each is the sealed lifecycle of the same run and
every one of them was already red for the same reason: `{preflight,run,test}-builtins-benchmark-109.sh`,
`test-builtins-benchmark-109-policy.sh`, the five `-110` siblings,
`{preflight,prepare,test}-builtins-listening-033.sh`, `test-builtins-listening-033-policy.sh`, the
five `-111` siblings, and `{preflight,run,test}-effect-interchange-benchmark-108.sh`.

No live coverage was lost with them:

- The Issue-033/111 listening validators (`check-builtins-listening-{033,111}.py`) keep their
  `--self-test`, and the packet-canonicality assertions from `check-builtins-listening-033.sh`
  moved into `scripts/check-builtins-listening.sh`, which has no pins and is green.
- `scripts/prepare-builtins-listening.sh` replaces the two retired `prepare-*` wrappers: it renders
  and validates a facilitator packet without a branch pin, a `target/issue*` namespace or a seal.
- The Issue-108 source-authority mutations stayed with their checker.

Three `scripts/test-*.sh` were red on `main` for the same or adjacent reasons and are handled here:

| script | disposition | why |
|---|---|---|
| `test-graph-benchmark.sh` + `promote-issue006-graph-benchmark.sh` | converted | both pinned `expected_bytes=10364` and the sha256 of one historical `target/issue6/graph-compiler-benchmark.raw.jsonl`, which has not existed since Issue 006 closed, so all eighteen hermetic cases were unreachable. The payload is now synthesized from the committed `scripts/fixtures/graph-benchmark-validator-record.json` (three benchmark ids x two rounds) and the promotion contract is the property that holds for every run: six records, LF terminated, accepted by the aggregate validator, byte-identical to its raw, published no-clobber. |
| `test-builtins-policy.sh` | fixed | the mutation fixture lagged #84 phase A, which added the `miso-engine-lane` edge to the checker's pinned compiler boundary. The harness was the only thing red. |
| `test-realtime-audit-hooks.sh` | fixed | it required a prebuilt `target/release/miso_engine_realtime_audit`; with no argument it now builds it. |

`scripts/{preflight,run}-effect-interchange-benchmark.sh` carry the same `ACCEPTED.sha256` identity
pin as the qualification checker and were re-pointed at the trimmed manifest in the same commit
(`6403ae62…` -> `1aaa96dc…`). No corpus byte changed; `sha256sum --check` over the manifest passes
before and after.

Known follow-up, bounded, not done here: `--seal`/`--write-seal` in
`scripts/check-builtins-listening-{033,111}.py` are now unreachable -- their `AUTHORITY_PATHS`
tables name ten retired scripts. They fail nothing (nothing calls them, and `--self-test` does not
touch them). The 111 validator pins the sha256 of the 033 validator, so removing the dead modes has
to touch both files and re-state that pin; it is left for a successor.

### Phase D -- repository hygiene (2026-08-24)

`main` carried a committed cargo target-dir spill at the repository root: 157 tracked files under
`wasm32-unknown-unknown/`, 77 under `release/`, and `.rustc_info.json` -- 235 files, 135 MB. They
are what `CARGO_TARGET_DIR=.` writes. `git grep` finds no reference to any of them: every script
that reads a build artifact builds it under its own `$target_dir` and the matches for
`wasm32-unknown-unknown/` are all `"$target_dir/wasm32-unknown-unknown/…"`. They are deleted,
`.gitignore` covers the root spill paths, and `scripts/check-workspace-policy.sh` now fails on
`.rustc_info.json`, `CACHEDIR.TAG` or any root-level directory containing `.fingerprint/`, because
an ignored spill still poisons every `find`/`rg` gate that walks the tree from the workspace root.
`scripts/test-workspace-policy.sh` proves all three mutations red.

That spill was also the reason `check-capi-qualification-v1.sh` and
`check-effect-interchange-qualification.sh` failed their "no generated artifact under a source path"
scan once their seals were converted: `release/deps/libserde_derive-*.so`.

### Phase C -- one environment and marker vocabulary (2026-08-24)

Before: 91 distinct `MISO_*` identifiers across eight prefixes (`MISO_ENGINE_`, `MISO_RT_`,
`MISO_GRAPH_`, `MISO_ISSUE069_`, `MISO_039_`, `MISO_INTERCHANGE_`, `MISO_CAPI_`, `MISO_TEST_`,
plus one-offs `MISO_MATH_PIN`, `MISO_WEB_STRIP`, `MISO_REPIN_MULTIBAND_CORPUS`,
`MISO_PRINT_HELPER_MANIFEST`, `MISO_CHROMIUM_BINARY`, `MISO_CHROMEDRIVER_BINARY`,
`MISO_WEB_ORACLE_PRINT`, `MISO_ISSUE_031_TRANSCRIPT`, `MISO_ISSUE_045_TRANSCRIPT`), with eight
facts carrying two or three names each. Finding F2 is the realised cost of that: the "sole
authorized" builtins runner exported two of the sixteen names its binary read, so every accepted
record carried all-null environment metadata *and passed validation*, because the runner said
`MISO_ENGINE_BENCH_GOVERNOR` where the binary read `MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE`.

After: 86 names, all `MISO_ENGINE_*`, one per fact, listed in `docs/ENGINE_ENV_VOCABULARY.md`.

Synonyms collapsed (the surviving name is on the right; every reader and every writer moved in the
same commit):

| retired | survivor |
|---|---|
| `MISO_ENGINE_BENCH_CPU` | `MISO_ENGINE_BENCH_CPU_MODEL` |
| `MISO_ENGINE_BENCH_ARCHITECTURE` | `MISO_ENGINE_BENCH_CPU_ARCHITECTURE` |
| `MISO_ENGINE_BENCH_LOGICAL_CORES` | `MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT` |
| `MISO_ENGINE_BENCH_PHYSICAL_CORES` | `MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT` |
| `MISO_ENGINE_BENCH_COMPILER`, `MISO_INTERCHANGE_RUST_VERSION` | `MISO_ENGINE_BENCH_RUST_VERSION` |
| `MISO_ENGINE_BENCH_GOVERNOR`, `MISO_ENGINE_BENCH_POWER_MODE` | `MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE` |
| `MISO_ENGINE_BENCH_TARGET`, `MISO_INTERCHANGE_TARGET_TRIPLE` | `MISO_ENGINE_BENCH_TARGET_TRIPLE` |
| `MISO_ENGINE_BENCH_BACKGROUND_LOAD` | `MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE` |
| `MISO_INTERCHANGE_LLVM_VERSION` | `MISO_ENGINE_BENCH_LLVM_VERSION` |
| `MISO_INTERCHANGE_PROFILE` | `MISO_ENGINE_BENCH_PROFILE` |
| `MISO_ENGINE_{BUILTINS,RACK,SCHEDULER}_BENCH_BINARY_SHA256`, `MISO_INTERCHANGE_BINARY_SHA256` | `MISO_ENGINE_BENCH_BINARY_SHA256` |
| `MISO_ENGINE_BUILTINS_BENCH_CANDIDATE_COMMIT`, `MISO_INTERCHANGE_CANDIDATE_COMMIT` | `MISO_ENGINE_BENCH_CANDIDATE_COMMIT` |
| `MISO_ENGINE_{RACK,SCHEDULER}_BENCH_CANDIDATE_SHA256` | `MISO_ENGINE_BENCH_CANDIDATE_SHA256` |
| `MISO_ENGINE_{RACK,SCHEDULER}_BENCH_ROUND` | `MISO_ENGINE_BENCH_ROUND` |
| `MISO_BUILTINS_BENCH_PHASE`, `MISO_INTERCHANGE_BENCH_PHASE` | `MISO_ENGINE_BENCH_PHASE` |

Issue numbers left the names: `MISO_ISSUE069_DIRECT_RT_*` -> `MISO_ENGINE_BUILTINS_RT_*`,
`MISO_ISSUE069_GRAPH_RT_*` -> `MISO_ENGINE_BUILTINS_GRAPH_RT_*`, `MISO_039_PHASE_*` ->
`MISO_ENGINE_SCHEDULER_PHASE_*`, `MISO_ENGINE_ISSUE{8,37}_AUDIT` -> `MISO_ENGINE_AUDIT_{008,037}`,
`MISO_ISSUE_0{31,45}_TRANSCRIPT` -> `MISO_ENGINE_TRANSCRIPT_0{31,45}`. 59 files changed; every
emitter and every reader of a renamed marker moved together, which is why the trace gates and the
runner lifecycle tests still pass.

The gate is `scripts/check-env-vocabulary.sh`, wired into CI beside the workspace policy. It is
bidirectional: an undocumented name fails, and so does a documented row nothing uses. An unused row
is the same defect as an undocumented name seen from the other side -- a name nobody agreed to stop
using. `scripts/test-env-vocabulary.sh` carries seven mutations; deleting each of the checker's
three `fail` calls in turn makes `stray-prefix`, `undocumented-name` and `unused-row` escape, so
each rule is proven red. Rule 1 exempts exactly two paths -- `docs/ENGINE_ENV_VOCABULARY.md`, which
has to be able to name a retired prefix in order to say it is retired, and
`.github/ISSUE_SPECS/`, whose job is to record what a name used to be. The `stray-prefix-in-doc`
case pins that the exemption is that narrow: the same stray name in a script fails.

Not done in phase C: metadata is still gathered from the environment rather than in-process, so a
runner that forgets to export a name still produces a `missing_metadata` entry rather than a
correct value. Making `Metadata::gather()` read `/proc/cpuinfo`, `rustc -vV` and `scaling_governor`
itself -- which is what actually closes F2 -- belongs with the shared support library of phase B.

### Phase B -- one harness under `tools/` (2026-08-24)

**Delivered:** the shared harness that finding F4 asks for, and the structural form of F1.
**Not delivered:** the 19-package to 2-package collapse (F5). Read the honest scope note at the end.

`tools/miso-engine-bench-support` is a test-only library with five modules and 22 unit tests:

| module | replaces | note |
|---|---|---|
| `alloc` | 17 copies of the audited `GlobalAlloc` wrapper in three behavioural variants | one `#[global_allocator]`, `Mode::{Abort, Count}`, always-on process counters |
| `json` | 7 escapers, two of which emitted invalid JSON | RFC 8259 section 7 in full |
| `stats` | 6 nearest-rank percentiles with three edge behaviours | Hyndman and Fan type 1, `clamp(1, len)` |
| `digest` | the scattered `sha2` use in the timed subjects | `Sha256Sink` counts its own updates |
| `timing` | `Instant::now` around a workload | `timed` panics if the timed body hashed anything |

All three historical allocator behaviours are preserved and each is exercised:

* abort on an armed violation (12 tools) -- `scripts/test-realtime-audit-hooks.sh` and
  `scripts/test-builtins-{audit,graph-audit}-probes.sh` still abort on all nine probes;
* count and continue (`builtins-audit`'s `ABORT_ALLOCATOR_VIOLATION` switch, now
  `alloc::set_mode(Mode::Count)`) -- the probes report their own deliberate violation;
* process totals (`effect-contract-bench`'s statics, now `counters()`/`delta_since`), and the
  protocol audit/bench thread-local armed counter, now a mark-and-delta over the same totals.

Every converted binary calls `alloc::assert_installed()` before it arms anything. A
`#[global_allocator]` registered by a dependency that is never named may not be linked at all, and
an audit that is silently off reports success for every gate below it.

Counts, `05a0822` to here (`grep -rlE` over `tools/**/*.rs`):

| | before | after |
|---|---|---|
| `unsafe impl GlobalAlloc` | 17 | 1 |
| `#[global_allocator]` | 17 | 1 |
| JSON escapers | 7 | 1 |
| nearest-rank percentiles | 6 | 1 |
| files with `#![allow(unsafe_code)]` | 18 | 5 |
| `tools/` Rust lines | 27,746 | 27,467 |

The unsafe ownership list in `scripts/check-realtime-policy.sh` lost eleven tool paths and gained
one: it is now `bench-support/src/alloc.rs`, `capi-audit`, `native-pcm-runner`, `protocol-bench`
(flatbuffers `unsafe fn follow`) and `wasm-gate-guest`.

F1 made structural: `timing::timed` samples `digest`'s thread-local update counter on both sides of
the clock and panics if the body hashed anything, so "the timed region is the workload and
arithmetic, never evidence collection" is a property of the run rather than of a review.
`rack-bench` is converted and is the first entry of the conversion ratchet in
`scripts/check-bench-policy.sh`: a listed subject must measure through `timing::timed` and must own
no `Instant::now`, `Sha256::new` or `sha2::` of its own. The list grows as subjects convert; it
never shrinks. `rack-bench`'s `input_sha256` and `output_sha256` are unchanged -- `Sha256Sink`
wraps the same `sha2::Sha256` and feeds it the same bytes in the same order.

Gates: `scripts/check-bench-policy.sh` + `scripts/test-bench-policy.sh` (12 mutations), wired into
CI beside the vocabulary gate. Deleting each of the checker's three `fail` calls in turn lets
`second-allocator`, `new-unsafe-owner` and `production-dependency` escape, so each rule family is
proven red. Separately, adding a non-elidable `vec![7u8; 64]` inside
`protocol-audit`'s `assert_zero_allocations` window aborts the run (exit 134), which proves the
allocation counter substitution is still sensitive rather than trivially zero.

Two gates outside the sweep regressed on the phase-C marker rename and were repaired here: the
renamed trace markers are longer than strace's default 32-byte string limit, so
`trace-builtins-graph-audit.sh` and `trace-scheduler-audit.sh` could no longer find
`MISO_ENGINE_BUILTINS_GRAPH_RT_BEGIN` (35 bytes) or `MISO_ENGINE_SCHEDULER_PHASE_PREPARED` (36
bytes) in the trace. Every `scripts/trace-*.sh` now passes `-s 200`. `trace-source-audit.sh` was
already broken before this branch -- `miso-engine-source-audit` gained a second `[[bin]]` and the
bare `cargo run -p` became ambiguous, the same drift as `check-builtins-fixtures.sh` -- and is
fixed with `--bin`. All seven trace gates pass.

**Scope note -- what phase B did not do.** The 19-package to 2-package collapse (`miso-engine-bench`
and `miso-engine-audit` with subcommands) is *not* done. It was not attempted, deliberately, and
this is the reason rather than an excuse:

* It is 27.5k Rust lines across 19 packages with ~80 invocation sites in `scripts/` and
  `.github/workflows/ci.yml`, and every subject owns a frozen record schema with its own validator.
* The plan comment on #104 makes the move conditional on an E1 golden comparison -- build the old
  binaries at the base commit, run every subject under both, and diff `jq -S` output with a named
  exemption list -- precisely because "preserve every currently-green gate's behaviour" is not
  checkable by review at that size. Doing the move without E1 would risk silently changing an
  accepted record's key set, which the plan forbids outright.
* Merging 19 packages into one also unifies their cargo features, so `miso-engine-graph/test-support`
  and `miso-engine-source/test-support` would compile into subjects that today do not have them.
  That needs the feature partition the plan specifies (`native`, `protocol-corpus`,
  `flatbuffers-comparison`), not a mechanical move.

Phase B delivered the plan's step 1, which is the named prerequisite for step 2 ("M, mostly
mechanical moves once F4 exists"). The remaining work is step 2 onward: the package collapse, the
in-process metadata gatherer that actually closes F2, one `run-benchmark.sh`, one `trace-audit.sh`,
and the typed `cargo metadata` dependency-boundary check. It should be a bounded successor issue
with the E1 golden harness as its first commit.


---

**Superseded (wave-scheduler removal).** The `miso-engine-native-scheduler` crate, the graph
crate's `bind_native` family and native dependency-wave executor, and this issue's gate scripts and
benchmark runner were removed from the tree as production-unreachable: nothing outside the graph
crate's own tests and the audit/bench scheduler subjects ever engaged them, every graph-side use
was `cfg(not(target_arch = "wasm32"))`, and host-core bound sequentially with no worker lease. This
document is retained unchanged as consumed history -- its findings, verdicts and measurements
describe the tree as it stood, and none of them are re-litigated by the removal. The sealed
measurements under `artifacts/issue009/` are retained for the same reason.
