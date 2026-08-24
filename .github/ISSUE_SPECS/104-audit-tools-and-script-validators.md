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
