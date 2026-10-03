# #1243 *Count and cap VCA groups in host preparation and the C ABI*: Sol verdict, attempt 1

- Reviewed: `git diff 210bd252f 4ce7767fc` (branch `codex/batch-vca`, worktree
  `/home/bl/misofm/wt-vca`). 35 files, +310/-25.
- Binding: `AGENTS.md`, `.github/ISSUE_SPECS/1243-count-and-cap-vca-groups-in-host-preparation-and-the-c-abi.md`
  and its Attempt 1 record. DESIGN P12 (`docs/handoffs/submix-sends-2026-10-02/DESIGN.md:238`).
  Precedent: `1206-attempt1.md`.
- Every changed path is on the authorized list. `crates/host-core/tests/vca.rs` is #1242's file;
  the spec anticipated it ("24, plus any #1242 added").
- How I ran it:
  - I did not touch the worktree, the branch or GitHub.
  - I exported `4ce7767fc` with `git archive` to `/tmp/claude-1002/v1243/src` and gave it its own
    target directories under `/tmp/claude-1002/v1243/`.
  - I ran the mutations in a second export (`/tmp/claude-1002/v1243/mut`). After each one I
    restored the file and checked it with `diff`.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There is one MINOR, a test gap that should be fixed before
close; the fix is about five lines and I have checked it works. There are three NITs and one INFO.

- D1, D2 and D3 are implemented exactly as frozen.
- The header, the Rust mirror, `abi_smoke.c`, the qualification doc and the spec agree on the
  field, its offset (184), `reserved[2]` at 192..207, the zero rule and the unchanged 208-byte
  size.
- Every gate I re-ran passes.
- Seven of my eight mutations turn a test red. The eighth survives, which is MINOR-1.

## Gates (re-run by me on `4ce7767fc`, x86-64 AVX2)

| Gate | Command | Result |
|---|---|---|
| 1 | `cargo test --locked -p host-core --test vca_caps --test submix_caps --test vca` | 1 + 1 + 3 passed |
| 2, 3 | `cargo test --locked -p capi` | 35 unit tests (including `frozen_sizes_alignments_and_representative_offsets_match`) and 11 `resource_lifecycle` tests passed. No oracle or budget was edited: the file's diff is only literals, one comment, the loop bound and the new test |
| 3 | `bash scripts/check-capi-abi.sh` | ok, shared and static |
| 3 | `bash scripts/check-capi-abi.sh --self-test` | ok. Its header legs are still vacuous (#1232), so I do not count this; M7 below is the layout evidence |
| 4 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then `./target/release/audit capi` | allocations 0, locks 0, syscalls 0, total_violations 0. `pcm_digest` is `ff6cdcb96cdcdad5`, the same as #1206 and the record |
| 4 | `cargo test --locked --release -p audit -p bench -p console-workload` | 110 passed, 0 failed |
| 5 | the spec's `test-debug-a` workspace command | rc 0: 114 binaries, 1265 passed, 0 failed (matches the record) |
| 5 | `cargo fmt --all -- --check` | ok |
| 5 | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| 5 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | clean |
| 5 | `check-`/`test-` pairs for `host-core-policy`, `realtime-policy` and `workspace-policy` | all six rc 0 |
| 5 | `bash scripts/check-cross-targets.sh` | PASS |
| 5 | `build-web-audioworklet.sh --named-twin <B> <A>`, then `check-web-audioworklet.sh <A> <B>/…named.wasm` | build rc 0 (shipped module `d6f6db9c…`), check rc 0 |
| 6 | `run-aarch64-tests.sh debug` | not run (no arm64 host). It stays "at batch push", as the record says |

## C ABI review

- **Layout.**
  - `maximum_submixes` is at 176, `maximum_vcas` at 184 and `reserved[2]` at 192..207. The size is
    208.
  - `abi.rs` pins all three offsets and the size (`COMPILE_LIMITS_SIZE`). `abi_smoke.c:45-47` pins
    the same offsets, and `:25` checks the size against the header macro (208).
  - The old `reserved == 184` pin is replaced, not left alongside the new ones. `header_smoke.cpp`
    is unchanged and still pins the size only.
- **`limits_are_valid`** requires `reserved == [0; 2]` (`compile.rs:357`).
  - `maximum_vcas` is in neither `limits_are_valid` nor `all_limits_nonzero`.
  - `prepare_caps` maps it exactly as it maps `maximum_submixes`, and its doc comment names both
    exceptions. That closes #1206 NIT-1 for this word.
- **Old caller, new library.** A caller written before this change zero-fills the tail, so it gets
  `maximum_vcas = 0`, which means `maximum_tracks`.
  - Every previously valid input is still valid. The only input whose meaning changes is a nonzero
    old `reserved[0]`, and that was refused before.
  - Zero cannot mean "no VCAs", because `all_limits_nonzero` forces `maximum_tracks >= 1`.
  - Zero cannot mean "unbounded" unless the caller already chose `maximum_tracks = u64::MAX`.
  - Unlike #1206 (its MINOR-1), zero tightens nothing for an existing caller. No released library
    accepts a VCA: `main` (`6f1788f3a`) has no `vcas` in its session model, and the grammar is
    strict. The qualification doc says exactly this.
  - M4 shows the zero rule is load-bearing: when zero is refused, ten `ffi` unit tests go red.
- **New caller, old library.** A library from #1206 to #1242 refuses a nonzero word with
  `RESULT_INVALID_ARGUMENT`; it is that library's `reserved[0]`. That is the safe direction, and
  the doc states it.
- **Leaving `ABI_VERSION` unchanged is justified.** No symbol, size or existing offset moves, and
  no valid input is reinterpreted. The prelaunch rule keeps live boundaries at V1.
- **Structural edits.** A replacement plan is prepared through
  `prepare_runtime(.., self.limits)` (`runtime/control.rs:749`, `:782`), which calls
  `prepare_caps`. So an edit that adds VCAs past the bound gets the same count refusal.

## Cap semantics

- **host-core.**
  - `vca_count` is `model.vcas.len()`, taken from the normalized model and checked in the same
    `if` as the other count caps.
  - The refusal is `host.resource.count`, with `PrepareRejection::Resource`.
  - The count is reported once, in `HostPrepareReport.vca_count`. Nothing reads it yet, and no
    reader breaks.
  - VCAs are counted apart from tracks and submixes, and every nested VCA counts once.
- **host-web** sets `u64::MAX` (D3).
  - The count check can never refuse there. The 1 MiB document cap (`MAXIMUM_DOCUMENT_BYTES`) and
    the memory budget are the bound.
  - Existing VCA tests catch a finite value (M6).

### Does the cap bound a real resource? Yes.

VCAs are control-only and own no DSP state. Their cost is the composition:

- `SessionModel::vca_reach` keeps one reach vector per strip, so it is O(strips × VCAs) in time
  and memory.
  - The worst case is a nested chain: `vca0` holds every track, and each later VCA holds the one
    before it. Every strip then reaches every VCA.
  - Session validation sets no limit on nesting depth.
- The composition runs several times per preparation: in graph-compiler `compile.rs:331`,
  builtins-compiler `:3135` and `:3399`, and host-web `:6299`.
- The count check (`prepare.rs:815-829`) runs on the `CompiledSession` before the graph and
  builtins compilers. So in the C ABI, `maximum_tracks × maximum_vcas` (T² when the word is zero)
  bounds that work before it starts.
- #1244's `LiveVcaState` will retain flattened reach tables, again O(strips × VCAs), and charge
  them as retained bytes in #1245 and #1247. So the cap will also bound retained live state.

## Deviations

None from the frozen decisions.

The VCA C ABI test repeats the reserved-word loop. Gate 2 requires that, and the record states the
overlap. See NIT-3.

## Mutations (mine)

| # | Mutation | Result |
|---|---|---|
| M1 | `prepare.rs`: `vca_count` = the sum of `vca.members.len()` (counts membership edges, not groups) | `vca_caps` red (`:73`, report 0 vs 1); capi test red ("zero word, three VCAs over two tracks", left 0, right 5) |
| M2 | `prepare.rs`: `vca_count >= caps.maximum_vcas` (off by one) | `vca_caps` red (`:68`, at-cap session refused); capi test red ("zero word, three VCAs within three tracks") |
| M3 | `prepare_caps`: `maximum_vcas: limits.maximum_vcas.max(limits.maximum_tracks)` (merges the word with the track cap) | capi test red at "the word overrides a larger track cap" |
| M4 | `all_limits_nonzero` also requires `maximum_vcas` (zero refused) | 10 `ffi` unit tests red, plus the capi tests |
| M5 | `limits_are_valid`: `limits.reserved[1] == 0` only | the VCA and submix capi tests both red at `reserved[0]` (this shows the overlap in NIT-3) |
| M6 | host-web `prepare_caps`: `maximum_vcas: 0` | `a_live_send_edit_keeps_a_vca_muted_column_zeroed` and `a_vca_muted_member_stays_muted_through_solo_and_an_explicit_unmute` red |
| M7 | Header: `uint64_t reserved_pad; uint64_t maximum_vcas; uint64_t reserved[1];`. The size stays 208 but both fields move. Saved as `<dir>/miso_engine_v1.h` and passed through `MISO_ENGINE_CAPI_HEADER` | `check-capi-abi.sh` red: the `abi_smoke.c` static assertions for `maximum_vcas == 184` and `reserved == 192` fail. Positive control: the unmutated header, copied the same way, passes |
| M8 | `prepare.rs`: count only root VCAs, that is, those no other VCA lists as a member | **survives**: `vca_caps` green, capi test green (MINOR-1) |

## Test value

- **`host-core/tests/vca_caps.rs::vcas_are_counted_capped_and_reported_apart_from_tracks_and_submixes`**
  turns red if:
  - host preparation stops counting VCAs;
  - it counts them against `maximum_tracks` (the track cap equals the track count, and the cap of
    4 is above it);
  - it counts them against `maximum_submixes` (held at 0);
  - it applies the bound off by one (M2);
  - it counts membership edges instead of groups (M1);
  - it leaves `vca_count` out of the report.

  No existing test catches these, because before this change nothing counted VCAs. The gap is
  MINOR-1.
- **`capi/tests/resource_lifecycle.rs::maximum_vcas_bounds_vcas_and_zero_defers_to_maximum_tracks`**
  turns red if the C bound:
  - ignores the word;
  - treats zero as "no VCAs" or as "unbounded";
  - merges the word with the track cap (M3);
  - reads `maximum_submixes` in its place;
  - stops refusing a remaining reserved word (M5).

  Before this change no test set the word at offset 184.
- **The re-pinned offsets** in `abi.rs` and `abi_smoke.c` turn red when `maximum_vcas` or
  `reserved` moves, even if the size stays the same (M7).

## Findings

### MINOR-1: no test counts a nested VCA

**The gap.**
- Every session in `vca_caps.rs` and in the capi test uses flat, empty VCAs.
- So a count that skips nested VCAs (M8, roots only) passes both tests and the workspace.
- A count that adds membership edges to the VCA count would also pass, because every VCA has zero
  members.

**Why it matters.**
- Nested VCAs drive the O(strips × VCAs) reach work that this cap exists to bound.
- "VCA groups = top-level groups" is a plausible misreading.
- The test-value sentence claims "red if VCAs go uncounted", and that claim covers this case.

**Fix (before close; checked in my export).** In `vca_caps.rs::session`:
- give `vca0` every track as a member;
- give each `vca{i}` the member `vca{i-1}`, so the groups form a chain;
- that makes the membership count differ from the VCA count.

```rust
members: if index == 0 {
    model.tracks.iter().map(|track| track.id.clone()).collect()
} else {
    vec![StableId::parse(&format!("vca{}", index - 1)).expect("stable id")]
},
```

With this change:
- the original code passes;
- M8 is red (`:84`, one over the cap prepares);
- M1 is red (`:72`, the at-cap session is refused);
- the attempt's own mutations stay red: the dropped clause, counting against tracks, counting
  against submixes, and `vca_count: 0`.

Unity faders keep the session's output bits unchanged.

### NIT-1: two header fields both say "Formerly reserved[0]"

- `miso_engine_v1.h:133` (`maximum_submixes`) and `:136` (`maximum_vcas`) both say "Formerly
  reserved[0]". `abi.rs` says the same for both.
- Each is true only against a different revision of the array.
- DESIGN P12 calls the VCA word `reserved[1]`.

**Fix:** for `maximum_vcas`, write "Formerly reserved[1] of the original reserved[4] (offset 184)",
in the header and in `abi.rs`. The qualification doc's "the next former reserved word" is already
unambiguous.

### NIT-2: an overlong comment line

`crates/capi/tests/resource_lifecycle.rs:1787` is 128 columns (`max_width = 100`). rustfmt does not
wrap comments, so `fmt --check` passes. **Fix:** rewrap the line.

### NIT-3: the reserved-word loop is duplicated

- M5 turns both loops red: the submix test's and the VCA test's.
- The VCA loop has only one catch of its own. It runs with the named word set (3), so it would
  catch a defect where setting `maximum_vcas` stops the reserved check. That defect is unlikely.
- Gate 2 mandates the loop, and the record states the overlap, so I accept it.
- **For future briefs:** let one test own the remaining-reserved-words leg. When a later word is
  freed, change that one loop.

### INFO-1 (outside this slice): in the browser, only the 1 MiB document bounds VCAs

**Context.**
- The #1242 verdict called the 4x composition cost "harmless until #1243's cap".
- D3 sets host-web's cap to `u64::MAX`. So in the browser the bound is still the 1 MiB document,
  not a count.

**What I measured.** I wrote a scratch session test (release, this host, now deleted). Canonical
JSON costs about 802 B per minimal track and 202 B per chained VCA.

| Tracks | Chained VCAs | Document | Reach entries | Reach memory | One `vca_reach` pass |
|---|---|---|---|---|---|
| 1,000 | 4,000 | 1.65 MB | 4.0 M | 30.5 MiB | 0.56 s |

That document is over 1 MiB. The 1 MiB worst case is about 650 tracks and 2,600 chained VCAs. By
the same proportions, that gives about 1.7 M entries, about 13 MiB transient and about 0.25 s per
pass. The composition runs about four times per preparation, so expect about 1 s of preparation
CPU natively, and more under wasm.

**Assessment.**
- This is acceptable. It sits well inside the 512 MiB default budget.
- It is not charged to the `PARSE_TRANSIENT_MULTIPLIER` projection, which was measured before VCAs
  existed.
- No action is needed in #1243.
- #1244's `retained_bytes` charge covers the retained copy. #1244 or #1245 should cite this worst
  case when they size and charge the reach tables.

## Scratch

I deleted `/tmp/claude-1002/v1243/` after the run: the two exports, the target directories, the
web build outputs, the mutated headers and the measurement test.
