# #1276 attempt 2 verdict: PASS

PASS with one MINOR and four NITs. The fix closes MAJOR-1:

- All 12 of attempt 1's probes now pass at both widths.
- The three new regression arms each have a unique catch under mutation.
- MINOR-1 (per-value D1 tests) and MINOR-2 (carry drain) are closed with tests that discriminate.
- MINOR-4's `BTreeMap`s are gone, and the measured size matches the attempt record exactly.
- Every gate passes on my export.

The new MINOR is an unchecked precondition introduced by the MINOR-4 rewrite. The join now
binary-searches the committed model. A caller that passes an unsorted model loses every carry
silently. The one production caller passes a sorted model.

- **Commit reviewed:** `e6302d8a8` (parent `55690373d`) from `/home/bl/misofm/wt-swap`. I read
  the cumulative slice too: `807b48547` + `f41388939` + `e6302d8a8`. `c14fde0ce` and `55690373d`
  belong to other slices. `55690373d` touches only capi, docs and scripts, so it changes no
  browser-module code.
- **Exports** (`git archive e6302d8a8`):
  - `/tmp/claude-1002/v1276/attempt2/` is the pristine tree; every gate ran there.
  - `/tmp/claude-1002/v1276/attempt2/probe-tree/` is a second copy for probes and mutations. Its
    sources are restored after every mutation, which I checked with `diff -rq`.
- **Scope:** eight files, all on the authorized list plus the slice spec. No capi or `hosts/`
  file is touched, and the artifact pin file is unchanged.
- **Hygiene:** exact paths, the `Co-Authored-By` line is present, no artifacts.

## Gates re-run on the export

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (62 marked regions in 16 files) / ok |
| Focused (`-p builtins -p builtins-compiler -p graph -p rack -p host-core`, test-support features) | 581 passed, 0 failed, 5 ignored. `successor_swap` 28/28; the avx2 build ran the `Simd8` rows |
| `cargo test -p graph --features graph/test-support,engine/realtime-audit --test rt11_swap_carry_alloc` | 1/1 |
| `cargo test --locked -p capi` / `-p console-workload` | 51 + 66 passed, 0 failed (static digests unmoved) |
| `cargo build --locked --release -p audit -p capi && audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations; `pcm_digest` `c60671f6593fa603` |
| `trace-builtins-audit.sh` / `trace-builtins-graph-audit.sh` | PASS / PASS |
| `check-builtins-fixtures.sh` / `test-builtins-fixtures.sh` | ok (50 files) / ok |
| `check-capi-abi.sh` | ok (shared and static) |
| `check-cross-targets.sh` | exit 0. Only the 11 known #1018 iOS `memset_pattern16` expected failures, with call counts identical to attempt 1's log |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` | all exit 0. graphSessionPlusPlanBytes 29794 of 35648, sourceTotalBytes 3358 of 3648 |

**ARTIFACT CHANGED is confirmed.**

- The shipped module is `e0fef6988d08…` at 2,895,964 B, the same hash and size as the attempt
  record.
- Against the base `0297efa8c` module (`751122a9…`, 2,870,096 B) that is **+25,868 B**.
- Against attempt 1 (2,914,664 B) it is −18,700 B.
- The pin was not re-pinned, which is correct per `docs/RELEASE.md`.

**`audit capi` digest explained.** I re-ran `audit capi` with this slice's input moves deleted
from `adopt_predecessor` (the M1 mutant, release build). The digest returns to
`7281b6c931e05dcc`, the pre-#1276 value. So the move to `c60671f6593fa603` is caused entirely by
the carried input sections. Nothing live pins either value; they appear only in spec prose.

**Zero allocations at the swap block.**

- `the_input_carry_allocates_and_frees_nothing` passes at `Simd8` and `Simd4`.
- It still sees the drain path: a `Box` inside `drain_for_carry` turns it red at
  `((2, 0, 2), (2, 2))`.
- `rt11` and `audit capi` both report 0 allocations.

## MAJOR-1 re-check: attempt 1's probes at both widths

All 12 probes from `/tmp/claude-1002/v1276/attempt1/logs/probe_1276.rs` pass at `e6302d8a8`. The
copy is kept at `attempt2/logs/probe_1276.rs`.

| Probe | Collapsing vs forced-dual successor | Successor collapse counters |
|---|---|---|
| Unarmed predecessor (stereo neighbour), W4 / W8 | bit-identical | `[12, 3]` / `[6, 2]`. The extra-collapse signature `[18, 3]` is gone |
| Record five blocks early, W4 | bit-identical | – |
| Delay, W4 / W8 | bit-identical (Δ = 0 on peak 8.31) | – |
| Armed control, W4 / W8 | bit-identical | – |
| Submix gap-free and ramp-to-disabled | green at both widths | – |

The fix reads `BankChain::carried_channel_agreement()`, which is `can_collapse() && flag`.

- **Right predicate.** `can_collapse()` is `collapse_prefix > 0 && collapse_source`. Both are
  bind-time facts, and they are exactly the guard under which `run` maintains the flag
  (`rack/src/lib.rs:2521`).
- **Forced-off chains are covered.** `collapse_forced_off` is not part of `can_collapse`, and a
  forced-off chain still maintains the flag through its lazy publication.
- **Read order is safe.** The flag is read after `disengage_for_carry`, and the drain does not
  touch it.
- **The input stage is always covered by the flag.** Seam-side slots must form a suffix, so the
  input stage is in the prefix whenever the prefix is non-empty.

### Mutation runs (mine, both widths, each reverted)

| Mutant | Stereo arm | Armed arm | Delay arm | Other red tests |
|---|---|---|---|---|
| MA1: carry reads the raw flag (the attempt-1 code) | red, block 6 | green (its flag is maintained, as expected) | red | probes red |
| MA5: `carried_channel_agreement` checks `collapse_prefix > 0` only | red | green | red | – |
| MA2: `inherit_channel_agreement` ignores `agree` | red | red | red | – |
| MA6: `carried_channel_agreement` returns `can_collapse()`, ignoring the flag | green | **red** | green | only the armed arm and the armed probe |

Every bit failure is `Some(6)`, the swap block, not just a counter.

**Is the delay arm's oracle adequate? Yes, for the claim it makes.**

- The forced-dual successor never reads the agreement flag, so collapsing against forced-dual
  tests exactly the class-A claim, "collapse moves no bit".
- The arm is not vacuous. MA1 and MA5 turn it red at block 6, and the successor demonstrably
  collapses elsewhere.
- What it cannot see is eq6's own continuity across the swap, because the delay line restarts.
  That is slices 13-14 and outside this slice.
- The carry itself is shared by both arms of the comparison. Its continuity is defended by the
  gate-1 and gate-2 tests against true no-swap references.
- The stereo and armed arms are stronger: they compare both successor modes with a genuine no-swap
  reference (A plus the muted track, fed the same record).

## Test value: one sentence per new or rewritten test (mutations run by me)

- **`a_diverged_lane_keeps_its_chain_dual_after_the_swap_at_{eight,four}_lanes`:** red when a
  predecessor chain that cannot collapse hands over its unmaintained `true` (MA1, MA5: stereo and
  delay arms), when a cleared but maintained flag is ignored (MA6: armed arm only), or when the
  successor ignores the handed-over value (MA2: all arms). In each case the successor collapses a
  lane whose channels disagree, and bits move at block 6.
- **`a_strip_whose_input_section_changed_starts_at_rest`** (replaces the hpf-only test, which is
  deleted): red when any one of D1's compared values is dropped (Mh, Ml, Mt, Mp) or when either
  channel is skipped (Mleft, Mright). Each mutant fails at its own named case, so an acked edit
  can no longer be silently overridden by carried state.
- **`a_changed_control_kind_carries_no_input_section`:** red when the live-queue clause is
  dropped (Md: 152 bytes, not 8).
- **`a_track_replaced_by_a_submix_of_its_name_does_not_carry`:** red when a committed strip is
  found by ID regardless of kind (Mk: 40 bytes, not 24).
- **`tests::the_carry_drain_applies_every_record_past_a_refused_one`:** red when the carry drain
  stops at a refused apply (Mdrain: lane 0's later trim stays queued) or propagates it (Mdrain2).
- **Byte assertions:** they are written as `8 + n * size_of::<GraphLaneMove>()`, so they state a
  move count, which is the claim, and are not a digest.

## Findings

### MINOR-1: the binary-search join relies on a sorted committed model and never checks it, so a violation silently restarts every strip at rest

The code (`crates/host-core/src/prepare.rs`):

- `committed_input_section` (`:1798`) binary-searches `committed.tracks` and
  `committed.submixes`.
- `SuccessorBase::committed` (`:588`, a public field of a public type) documents that it must be
  normalized, but nothing checks that.

**My probe.** In `probe-tree`, `vprobe_unsorted_committed_model` prepares the same successor
twice, at both widths:

- with the committed model as authored (sorted): 152 carry bytes, so all 9 sections carry;
- with that model's tracks reversed: 8 bytes, so 0 sections carry, with no assertion.

**The assumption holds in production today:**

- The only production caller (`crates/capi/src/runtime/control.rs:766`) passes
  `normalized_model()`.
- The normalized model sorts tracks and submixes by `StableId`, whose derived `Ord` is the
  `String`, and so `str`, order that the search uses.
- Tracks, submixes, outputs and VCAs share one ID namespace (`session/src/validate.rs:80-117`).

**Why it still matters:**

- The failure is fail-safe: a strip is never given another strip's state. But the symptom is the
  very click this slice removes, and it is silent.
- The test harness (`tests/support/successor.rs`, `Session.model`) passes **raw** models. They
  happen to be sorted, because the fixture is `eq0…eq8` and the muted track is only ever appended
  to the successor.
- The attempt-1 `BTreeMap` join had no such precondition.

**Fix:** pick one.

- Add a `debug_assert!` that `committed.tracks` and `committed.submixes` are strictly sorted by
  ID when `successor` is `Some` (`prepare.rs:1145`), and make the harness pass
  `compiled.normalized_model()` as production does.
- Or type the field so the property is structural, for example `&CompiledSession`. That would
  touch a capi call site.

### NIT-1: "`false` costs at most one proof" is not true for a chain with an effect in its prefix

`crates/rack/src/lib.rs:2809`, and the same claim in the attempt record's open points.

Every launch effect declines `channels_agree` (`effect-contract/src/lib.rs:2243`). So a successor
chain with a console or insert effect in its collapse prefix that receives `false` stays dual for
the rest of the plan; one proof is not enough.

That is the correct, conservative behaviour. In practice `false` arrives only when a lane moves
from a stereo-class chain, which means the track's class changed in the transaction (asymmetric
trim, polarity, filter or delay became symmetric), so its state really does differ. The comment
should say that instead.

### NIT-2: in a release build the carry drain's refused count is discarded

`crates/graph/src/runtime.rs:4198-4203`.

A debug build panics on the render thread through `debug_assert_eq!`. There is precedent for
this, `disengage_collapse`, and the realtime policy accepts it. A release build drops a refused
record silently, whereas the block drain surfaces a `RenderError`.

It is unreachable today:

- host-web admission checks the trim domain `[-144, 24]` exactly as `checked_trim_gain` does;
- only populated lanes have queues;
- prepared filter targets are trusted, prepare-time values.

So no ack can precede a drop. A saturating plan counter would make any future divergence visible.
Optional.

### NIT-3: the remaining +25.9 KB browser-module growth

From `twiggy diff`, base named twin against candidate named twin. Net code is +26.4 KB, plus
+9.0 KB of `name` section that does not ship.

| Part | Growth |
|---|---|
| Two new sort monomorphizations: `GraphLaneMove` in `install_builtin_input_carry` | +5.5 KB |
| Two new sort monomorphizations: `InputSectionInventoryRow` | +4.5 KB |
| builtins `export_lane` / `import_lane` | +4.8 KB |
| graph | +3.9 KB |
| host-core | +1.5 KB |

The `BTreeMap` collects are gone. The browser calls none of this yet (Q6). A follow-up could:

- push inventory rows in strip order;
- build the moves already grouped by predecessor;

which would save about 10 KB. Not blocking: attempt 1 already judged this not a "modes production
never runs" violation, because the C ABI runs the carry.

### NIT-4: two new tests run at one width

- `a_changed_control_kind_carries_no_input_section` runs only at `Simd4`.
- The drain unit test runs only at `Backend::current()`.

Both claims are width-independent (a prepare-time join, a queue loop), so this is acceptable.

## Attempt-1 items closed

| Item | Status |
|---|---|
| MAJOR-1 | Fixed, with the three-arm regression tests above |
| MINOR-1 | Fixed: per-value tests plus the control-kind and strip-kind clauses, all defended |
| MINOR-2 | Fixed: every record present at entry is consumed past a refusal |
| MINOR-3 | Fixed: a fifth "Coming back" clause, and the flag documentation points at the new accessor |
| MINOR-4 | Addressed: −18.7 KB |
| NIT-1 | Fixed: `debug_assert_eq!` restored at `prepare.rs:1681` and for the retained bytes |
| NIT-2 | Fixed: regions added, 58 → 62 |
| NIT-3 | Fixed: comment added |
| NIT-4 | Stands as a forward note: the first slice that gives a host input live edits across a published successor must route admission to the newest plan |

Export `target/` directories deleted after this verdict. Sources, logs, the probe file, the
probe-tree test additions (`logs/successor_swap.with-arms.rs`) and the mutation logs
(`logs/mut/`) are kept.
