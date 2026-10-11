PASS

# #1471 attempt 3 -- adversarial verdict

Verifier: fresh Opus, 2026-10-11. Reviewed `25d350a96` (attempt 3) against the attempt-2 verdict
(`1471-attempt2.md`: M1, m1-m3, n1-n3). I also checked the whole slice end state at `25d350a96`
(cumulative commits 28de9b791 dbcf9c46c ecdc3a936 8c61cb459 6d56b1f15 32241583c 5fa3f0c69 1227b9090
e44b8e8b2 25d350a96) against the spec, with Amendments 1 and 2 binding. I exported the code with
`git archive 25d350a96` to `/tmp/claude-1002/v1471/tree` and built it with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1471/target`. Nothing was built or edited in the worktree.

M1 is fixed. Both gate-6 tests go red on their named defects (I re-ran each mutant below), and
gates 1 to 6 hold. There is no BLOCKER or MAJOR. Two MINORs are false descriptive text and do not
change what a test catches. Fold them with the NITs into the follow-ups commit.

## MINOR

**m1. A mechanism sentence in the new test's doc comment is false.**
`crates/capi/tests/resource_lifecycle.rs:1519-1521` says: "B's second compile has no filter to record,
so the first compile's record would stay and B would free more than A". `begin_journal` discards
an open journal and restarts it empty (`crates/builtins/src/tail.rs:536,543-545`). So when the
journal is left open, B's second compile drops B's 200-design record and A keeps its record. A then
frees more than B. My mutant A run showed `once` (engine A) = 168,048 B and `twice` (engine B) =
84,208 B. The test is still red because it asserts equality, so the defect claim at :1523-1524 is
true. The parenthetical at :1525-1527 ("the second compile must not restart away") describes the
real mechanism, so the comment contradicts itself. Fix: "B's second compile restarts the journal
with nothing to record, so A keeps its 200-design record and frees more than B."

**m2. `tail.rs` keeps a measured cache figure that is stale for the current entry layout.**
`crates/builtins/src/tail.rs:404-406`: "A full cache of 8,192 entries holds about 2.4 MiB for one
fill order (2,486,520 bytes measured, about 304 bytes an entry)". #1457 attempt 2 measured this
figure when the value held an `InputSectionBound`. Later, #1464 made the value a `NodeTailBound`, so
the value is now 112 B, the leaf 2,608 B and the internal node 2,704 B. With those node sizes the
total cannot be 2,486,520: 2,608a + 96b = 2,486,520 reduces to 326a + 12b = 310,815, and the left
side is even while the right side is odd. This slice rewrote that sentence (attempt 2, in the
"cap wording" exception), so the stale figure is this slice's text. The slice's own qualification
doc gives 3,602,112 B for its fill order, and I measured the same value today. This fault dates
from attempt 2, and the attempt-2 verifier did not find it. Drop the figure, or replace it with the
current measurement and its fill order. The ceiling sentence that follows it is correct. (The
#1457 and #1470 spec bodies quote the same old figure. They are other issues' records and are out
of scope here.)

## NIT

- **n1. Gate 5's committing edit is B, not "the same edit".** Spec :179 says "The same edit submitted
  again commits". Through the limit route the gate allows, this cannot happen. The cap is fixed for
  the session at compile time, so edit A, resubmitted at the same peak, is refused again.
  Substituting B is correct, and it still catches the named defect.
  `builtins_like(7, 7, 55.0)` gives the same `builtins` as `builtins_edit(7, Some(55.0))`, and strip
  8 gets an identical copy, so A's one new design is B's only new design. Under my mutant E (the
  rebuild writes its record into the newest seed after `prepare_runtime`, before
  `validate_replacement_peak`), B computes 0 designs instead of 1. The test is red at
  `design_bound_cache.rs:579`, and the other 4 tests are green. The attempt record (spec, Attempt 3
  "n2") gives the reason. Gate 5 is not weakened, so no root decision is needed. The coordinator
  may still ask root to add a one-line spec note that, through the limit route, a different edit
  that reuses the refused design is the committing edit.
- **n2. The raw benchmark log is only in the session scratchpad.** Spec :454 cites
  `bench-1471-a3.log (scratchpad)`. I read it, and every figure in the m3 table and the
  first-compile and cache-bytes lines matches it. A scratchpad does not survive the session.
  Keeping the record table is sufficient, but do not cite the log as if it were permanent.

## Attempt-2 findings: resolution

- **M1, fixed.** `a_compile_leaves_nothing_open_on_the_engine_cache` (:1529) is an
  allocator-observed equality (84,208 = 84,208 B). It counts requested bytes through this file's
  counting allocator, so the platform allocator does not change it. Both engines insert the same
  designs in the same order, so the result is deterministic. Runtime is about 0.9 s.
  `the_engine_cache_stays_within_its_stated_ceiling` (:1570) now measures the leaf from a
  one-design compile on a fresh engine (exactly 2,608 B). It then derives
  `WORST_CASE_NODES x leaf + WORST_CASE_INTERNAL_NODES x 96` from the cap constant. The derived
  value equals the 4,300,720 ceiling, so a leaf even 1 byte larger turns the test red. The
  one-design measurement does not depend on fill order. If the engine came to keep other
  memory after a compile, the measured "leaf" would grow and the test would fail red. It would
  never pass falsely. Runtime is unchanged: the 16,384-design fill takes about 35.7 s in debug,
  and the `resource_lifecycle` suite takes 36.1 s, as in attempt 2. Every claim in the
  ceiling test's doc comment is true. The other doc comment has the m1 error above.
- **m1 (allocator wording), fixed.** The header (`miso_engine_v1.h:111-124`) and the qualification
  doc now say "requested heap" and that allocator rounding is platform-specific. They give no
  figure for it.
- **m2 (transient journal), fixed and correct.** At most 1,805 designs are recorded per preparation.
  After a spent budget, `input_section_bounds_within` (`builtins/src/lib.rs:3573-3577`) does no
  lookup. Every journaled design either lowers `remaining` by its charge (at least 837) or sets it
  to 0, and 1,510,000 / 837 = 1,804, plus 1. The journal map only inserts, so every non-root node
  holds at least 5 entries. That gives 1 + 1,804/5 = 361 nodes, and (361-3)/6 + 1 = 60 internal
  nodes. 361 x 2,608 + 60 x 96 = 947,248. "Between calls" is now stated. The header change is
  comment lines only (`check-capi-abi.sh` ok).
- **m3, fixed.** The 9- and 64-strip rows were re-recorded on the final code: one invocation per
  document and trim setting, one warmup round, two measured rounds. The figures match the raw log
  exactly. The record states why the 65,537-strip row was not re-run.
- **n1, fixed.** Against the slice base, the `compile.rs` hunk is one line
  (`.chain([effect_controls.largest_allocation_bytes(), bound_seed_bytes])`).
- **n2, done.** It now uses the limit route (see NIT n1).
- **n3, done.** The edits outside the listed paths are recorded in one paragraph.
- I found no "producer" or "fan" wording added in any of the ten slice commits.

## Mutations re-run by this verifier (each applied to the export and then reverted; logs in `/tmp/claude-1002/v1471/a3mut/`)

| # | Mutant | `a_compile_leaves_nothing_open...` | `the_engine_cache_stays...` | Other |
|---|---|---|---|---|
| A | `take_journal` clones (journal left open) | **red** (168,048 vs 84,208) | **red** (leaf 5,216, derived 8,575,232) | -- |
| B | `ChargedInputBound` gains a `u64` (+8 B value) | green (87,024 = 87,024) | **red** (leaf 2,696, derived 4,444,952) | -- |
| C | `INPUT_BOUND_CACHE_ENTRIES` = 16,384 | green | **red** (derived 8,598,832) | -- |
| D | cap applied as `ENTRIES * 2`, constant unchanged | green | **red** (fill frees 7,143,856) | -- |
| E | rebuild writes its record into the newest seed before the peak check | -- | -- | `a_refused_rebuild_leaves_the_seed_as_it_was` **red** (0 designs, not 1); other 4 `design_bound_cache` tests green |

Attempt 2's verdict showed that no older test catches A or B. My M6b and M1 analysis there found
the old fill-only check green for both.

## Test-value sentences (tests new or rewritten in attempt 3)

- `resource_lifecycle::a_compile_leaves_nothing_open_on_the_engine_cache`: catches a journal that
  stays open on the engine's cache after a compile, so that the engine keeps a second, uncounted
  copy of the designs (mutant A). It gives an exact before-and-after comparison that no other test
  makes.
- `resource_lifecycle::the_engine_cache_stays_within_its_stated_ceiling` (rewritten): catches an
  entry whose key or value grows by as little as one byte per leaf, so that the header's 4,300,720
  becomes false (mutant B, which the old fill-only check missed). It also catches a raised cap
  constant (C) and a cap that is no longer applied (D).
- `design_bound_cache::a_refused_rebuild_leaves_the_seed_as_it_was` (rewritten): catches a rebuild
  that writes its designs into the newest seed before its commit, here refused by the C limit one
  byte below its bisected peak (mutant E).

## Gates run (on the export at 25d350a96)

- `cargo fmt --all --check`: ok.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: ok.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: ok.
- `cargo test`: `builtins --features test-support`, `builtins-compiler`, `host-core`,
  `control-plane --features test-support` and `capi` (all suites): 63 suites ok, 542 tests passed,
  0 failed. This includes `design_bound_cache` (5, 18.5 s) and `resource_lifecycle` (13, 36.1 s).
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (93 marked regions
  in 26 files).
- `scripts/check-capi-abi.sh` (`MISO_ENGINE_CAPI_SKIP_BUILD=1`, release `libcapi.so`/`.a` from the
  external target): ok, x86_64-unknown-linux-gnu, shared and static.
- Release `audit capi`: calls 100,000, allocations 0, deallocations 0, locks 0, syscalls 0,
  `pcm_digest` `cb10fbface44a3a4`, total_violations 0.
- Not re-run: `check-cross-targets.sh` and the worklet chain (not in this verification's list). The
  attempt record's reasoning about the browser sha is sound: `host-web` uses neither changed crate,
  and `builtins` did not change after `e44b8e8b2`.
