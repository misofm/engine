PASS

# #1456 attempt 2: adversarial verdict

- **Reviewed:** `git diff b5cfd2b60 34cd5d55f -- crates/true-peak-limiter scripts docs
  .github/ISSUE_SPECS/1456-*` on `codex/d15-stream-g2`. The #1456 commits are `8fc41c5b0`
  (attempt 1) and `34cd5d55f` (attempt 2). The other changes in that range belong to other slices:
  `crates/true-peak-limiter/tests/ramp_endpoint.rs` (#1458, `7c43b2513`) and `STREAMS.md`. The spec
  is read at `34cd5d55f`, with Amendment 1 and both attempt records. The limiter source at
  `8fc41c5b0^` is byte-identical to `b5cfd2b60`.
- **Method:** I exported `34cd5d55f` with `git archive` to `/tmp/claude-1002/v1456b/tree` and built
  only there (`CARGO_TARGET_DIR=/tmp/claude-1002/v1456b/target`, `CARGO_INCREMENTAL=0`). The base
  is a second export with the `8fc41c5b0^` `lib.rs` and `corpus.rs`, built in its own target dir. A
  third export, with its own target dir, held one timing variant. I did not build, edit or check out
  anything in `/home/bl/misofm/wt-d15-g2`. Before I ran the gates, I restored the head export and
  diffed it against `git archive 34cd5d55f`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1. The release profile is `lto = "fat"`, `codegen-units = 1`.
- **Verdict:** PASS. Form v7 is implemented as ruled. No bit moves, and the shipped iOS library has
  no limiter `memset_pattern16` call. No per-block code changed, and every gate passes, apart from
  the `cargo doc` failure that #1459 brought. MAJOR 1 of attempt 1 is resolved. Two MINORs and
  four NITs go to the batch follow-ups. MINOR 1 corrects a recorded cost number that does not
  reproduce.

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **One recorded reset-cost cell does not reproduce, and the source doc quotes the range built on
   it** (spec, Attempt 2, "Gate 3" table, row "W4 44.1 kHz | 457 | 476 | 1.04";
   `crates/true-peak-limiter/src/lib.rs:675-678`, "4 % to 25 % slower"; and the commit message,
   "+4 to +25 %").
   I measured `LimiterCore::<L>::reset(DiscontinuityKeepParameters)` the same way: both channels,
   x86-64-v3 release test build, `taskset` to one core, best of 7 x 20,000, base and head
   alternated five times. I did this in two different binary layouts. The first binary also
   carried my differential probe. The second was a timing-only probe in a fresh export. Min and
   median agree within 1 %:

   | cell | record (base -> head) | layout 1 | layout 2 |
   |---|---|---|---|
   | W1 44.1 kHz | 112 -> 130 (1.16) | 111 -> 131 (1.18) | 111 -> 131 (1.18) |
   | W1 96 kHz | 224 -> 248 (1.11) | 223 -> 248 (1.11) | 223 -> 248 (1.11) |
   | **W4 44.1 kHz** | **457 -> 476 (1.04)** | **455 -> 575 (1.26)** | **453 -> 578 (1.28)** |
   | W4 96 kHz (gated) | 961 -> 1,100 (1.14) | 965 -> 1,101 (1.14) | 959 -> 1,130 (1.18) |
   | W8 44.1 kHz | 915 -> 1,141 (1.25) | 922 -> 1,125 (1.22) | 911 -> 1,116 (1.23) |
   | W8 96 kHz (gated) | 1,919 -> 2,387 (1.24) | 1,939 -> 2,367 (1.22) | 1,918 -> 2,374 (1.24) |

   The two cells that the gate names (96 kHz, W4 and W8) reproduce. So do the other three. The
   W4 44.1 kHz head number does not: 575 to 579 ns, not 476. The mechanism agrees with my number.
   At W4 the head ring loop is 16-byte `vmovdqu xmm` stores, two planes per iteration. The base
   `fill(1.0)` used 32-byte `ymm` stores. So W4 is the cell where head writes the most store
   instructions per byte, and it should not be the cheapest cell. The measured range is therefore
   +11 % to +28 %, not +4 % to +25 %. Root's acceptance does not depend on this cell: root's
   reason was "reset-only, never per block", and that holds (see "No per-block cost" below).
   **Fix (batch follow-ups):** correct the W4 44.1 kHz row in the attempt record, and make the doc
   at `lib.rs:675-678` (and the range in the record) say "11 % to 28 %". Better still, make the
   doc quote only the two gated 96 kHz cells.

2. **The spec's Product outcome still states the pre-Q3 rule** (spec lines 14-15: "The realtime
   rules forbid any libc call in render. After this slice that path makes none"). Q3 restates the
   outcome to `memset_pattern16` only. At head, `clear_runtime` still makes 3 `bl _bzero` calls on
   iOS (`history`, `main_ring`, `phase`) and 3 `memset` calls on x86-64, in every instantiation.
   So "that path makes none" is false as written. The first sentence of the outcome was updated;
   this sentence was not. **Fix:** "...makes no `memset_pattern16` call" (the spec is an authorized
   path).

## NIT

1. `lib.rs:656-658`: "one copy serves every caller" is no longer exact. `clear_runtime` is now
   generic, so there is one out-of-line copy per lane width: f32, f32x4 and f32x8 on x86-64, and
   f32 and f32x4 on iOS.
2. `scripts/check-cross-targets.sh:102` (outside this slice's authorized lines 104-108, from #1451)
   still justifies the ratchet as "a libc call, which the realtime rules forbid in render". After
   Q3, root's reason is narrower, because `bzero`/`memcpy` are libc calls too. Root may want that
   line restated. This is not this slice's to fix.
3. `clear_runtime` now relies on `self.width == L::WIDTH`, and only a `debug_assert` holds it
   (`lib.rs:681`, `:512`). In release, a mismatch would leave ring words at the zeroed allocation's
   `0.0`, not `1.0`. Every constructor passes `L::WIDTH` (`LimiterCore::new`, `corpus::run_case`),
   and the uniform kernel already relies on the same debug-asserted coupling (`lib.rs:1583-1599`).
   So this is consistent with the crate's design. I note it only because the old `fill` did not
   depend on the width.
4. **The ratchet counts the pre-link rlib, not the shipped library** (attempt record, "Gate 2 /
   codegen"). This is evidence for root. The record is accurate for the code it read, and the
   change has no defect here. With `lto = "fat"`, the per-crate `--emit asm`, which the ratchet and
   the record read, is LLVM's pre-link output. In it, the f32 (W1) ring loop is one `str w` /
   `movl` per word per ring. The shipped iOS library is `capi` after fat LTO. I emitted that
   (`cargo rustc --release --target aarch64-apple-ios -p capi --lib --crate-type staticlib --
   --emit asm`, the shape of the script's Android row), at base and at head:
   - `bl _memset_pattern16`: base 11, head 5. The limiter's 6 are gone: base had 3 in
     `clear_runtime` and 3 in `ChannelState::new`. The 5 left are `builtins_compiler`'s
     `into_graph_artifact_with_banks` (3), `BuiltinChain::new` and `FaderMuteRampBuiltins::new`,
     all preparation.
   - The post-LTO `clear_runtime::<f32>` vectorizes the two-plane ring loop: `stp q0, q0` per
     plane, behind a run-time alias check, with the vectorizer's scalar remainder. The function
     still has no `memset_pattern16` call and no panic. The W1 "scalar where vector possible"
     concern that the pre-link assembly raises is therefore not present in the shipped code.

   The ratchet is still a sound guard: the pre-link and shipped counts agree for the limiter (6 ->
   0), and both ratchet mutants below move it. But the record should say that the shipped library was also checked, or say that it
   was not.

## Points checked

- **Form v7 as ruled:** yes. `clear_runtime<L: Lane>` writes `required_ring` and `box_ring`
  together over whole `chunks_exact_mut(L::WIDTH)` chunks with `L::splat(1.0)`, and `prefix` with
  `reduction` (`L::zero()`) over whole chunks. There is no scalar tail, no `cfg` and no new
  attribute (`#[inline(never)]` was already there, from #1091/#1452). The `corpus.rs:228-229`
  turbofish is the only change in that file. The trap comment is at `lib.rs:685-687`.
- **One shape on every target:** `new`, `reset_to_defaults`, `reset_keeping_parameters` and
  `clear_runtime` are generic in `L`. Every caller passes its own `L` (`LimiterCore::new`, `reset`,
  `reset_failed_lanes`, the corpus and one test). The function body has no target-specific code.
  The only per-target difference is the lane width.
- **MAJOR 1 of attempt 1 (no scalar fill where a vector fill works): resolved.** At W4/W8 the rings
  are written with full-width vector stores: `stp x, x` (16 B) per ring per chunk on iOS, and
  `vmovups xmm`/`ymm` on x86-64-v3. At W1 the shipped (post-LTO) code is vectorized too (NIT 4).
  `clear_lane_runtime` keeps its strided per-lane form. That code is unchanged and out of scope.
- **Prefix bounds-check branch impossible (gate 4):** shown. There is no `panic`,
  `panic_bounds_check`, `slice_index_fail`, `ud2` or `brk` in `clear_runtime` at f32, f32x4 and f32x8 in
  the x86-64-v3 rlib assembly or the release test binary. The same holds at f32 and f32x4 in the
  iOS rlib and the post-LTO `capi` (there is no Simd8 on AArch64, #1112). Attempt 1's
  `step_by` `cbz -> panic` is gone.
- **Q2, `new`'s zeroed planes:** a mutant that allocates `required_ring`, `box_ring`, `reduction`
  and `prefix` at `0.5` keeps every limiter test green, and the differential stays bit-identical
  to base.
  So those words are dead, as Q2 says.
- **No per-block cost.**
  - x86-64-v3 release test binaries: every per-block function has identical instructions in base
    and head. That is `process_block` and `process_block_mono` at f32, f32x4 and f32x8, and the
    bank `process`/`process_bank` entries. Only function placement moved.
  - A per-block timing (128 frames, noise, best of 9 x 4,000, six alternations) gives head/base
    0.99 (W1), 1.00 (W4) and 1.01 (W8). With the instructions identical, this is layout.
  - Post-LTO iOS `capi`: `process_block::<f32x4>` and `process_bank_inner` are unchanged.
    `process_block::<f32>` differs only in its inlined failed-block tail: the call target becomes
    `clear_runtime::<f32>`, and the registers around `seed_lane_defaults`/`reset_lane_to_defaults`
    change. The frame loop is untouched.
- **Render stays allocation-, lock- and syscall-free:** `clear_runtime` does stores plus
  `bzero`/`memset` of fixed planes. It allocates nothing. `check-realtime-policy.sh` passes.
- **TARGET_MATRIX "None of these is reachable from render":** checked. `SpectrumAnalyzer::analyze`
  and `analyze_continuous` have no caller outside `host-core`'s own tests. They do not appear in the
  post-LTO iOS `capi`. The `builtins` rows are constructors.
- **Acked-batch question:** no queue is touched.

## Bit differential (gate 1, my own, PR evidence)

I added an `#[ignore]` probe, identical in both builds, inside the limiter's `tests` module. It
drives `LimiterCore::process_block` at f32, Simd4 and Simd8, 44.1 and 96 kHz, quanta 1 and 128,
`DualMono` and `Maximum`, with uniform lanes and with mixed lanes (per-lane ceiling, release and
lookahead, and a left/right lookahead split). That is 48 cases, plus 32 padded-bank cases (last
lane padding, at W4/W8). Each case runs these phases:
- noise with ramp retargets every fifth block;
- `FullToDefaults`;
- `DiscontinuityKeepParameters` with ramps open;
- a NaN on every lane (whole D7 reset);
- a NaN on lane 0 (per lane at W4/W8);
- strict `+0.0` silence until the silent fast path engages (3,307 or 31 engagements per case),
  then a keep-reset in silence, more silence, and a full reset in silence;
- noise again;
- for padded banks: a NaN on every active lane (whole reset, charged once), a NaN on the padding
  lane alone (recovered, not charged), and a keep-reset.

After construction, after every reset and after every block, the probe hashes every output word,
every state word of both channels by bits, the cursors, the lane shapes, the lookahead bits, the
report, the link and silence flags, and the engagement count.

Result: **392,640 digest lines, identical base vs head** (`cmp`; sha256 of either file
`3617130e...`). The probe discriminates: each reset-word mutant below differs from base, at the
first construction line or at the first reset.

## Mutation runs (re-done; no new test)

| mutant (head) | `cargo test -p true-peak-limiter --all-targets` | probe vs base | iOS count / judge |
|---|---|---|---|
| ring loop `.skip(1)` | 11 red, incl. `a_lane_reset_is_the_whole_reset_at_one_lanes_stride`, `both_resets_return_the_runtime_state_to_a_silent_lane`, `passes_effect_contract_conformance` | differs at line 1 | -- |
| `prefix` written `0.0` | 3 red (`a_lane_reset_is_the_whole_reset_at_one_lanes_stride`, `a_failed_lane_is_recovered_and_reported_alone`, `a_padded_lane_stays_at_rest_through_the_lookahead`) | differs at line 1 | -- |
| `reduction` not cleared | 7 red, incl. `a_nonfinite_block_is_zeroed_reset_and_counted`, `both_resets_return_the_runtime_state_to_a_silent_lane` | differs at the first full reset | -- |
| last `box_ring` word `0.5` | 28 red | differs at line 1 | -- |
| `new`'s planes at `0.5` | 0 red | identical | -- |
| rings back to `fill(1.0)` | -- | -- | 4 calls; `judge-memset` rc 1 ("4 memset_pattern16 calls and no row") |
| rings as two one-plane `L::splat` loops | -- | -- | 4 calls; judge rc 1 (confirms the doc's "equally a loop of `L::splat(1.0)` stores over one plane") |
| head as committed | green (89 with `-p conformance`, gate run) | identical | 0 calls; judge rc 0 |

## Test value

This slice adds no test, so no test-value sentence is due. The guards are the existing tests,
which go red on every misplaced reset word above, and the `judge-memset` ratchet, which goes red
on a returning call.

## Gates run (on the `34cd5d55f` export)

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test --locked --all-targets -p true-peak-limiter -p conformance` | PASS, 89 passed |
| `cargo run --locked -p conformance --example conformance_fixtures -- --check` | PASS |
| `python3 -B scripts/lib/aarch64-known-defects.py --self-test` | PASS |
| `bash scripts/check-workspace-policy.sh` | PASS |
| `bash scripts/check-realtime-policy.sh` | PASS (89 marked regions in 25 files) |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | PASS |
| `bash scripts/check-cross-targets.sh` | PASS; limiter 0 calls and no row; builtins 5, host-core 4, soft-clip 1; no eight-lane code on iOS/Android |
| `bash scripts/run-wasm-gates.sh` | PASS (native + wasm simd128 + V8 EQ loops) |
| `test-debug-a` workspace command (`qualification.yml:625-634`) | PASS, 1,458 passed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | FAIL, not this slice: `crates/gate-expander/src/corpus.rs:64` links private `RAMP_FRAMES`, from #1459 (`ffcbba6b7`), already present at the parent `db4078c3a`. **Root:** the batch verifier will hit this. |
| the same with `--exclude gate-expander` | PASS |
| worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`) | PASS: module `8ec50f63...`; call graph, static/object and boot-budget checks; `expected.json` digests and rows agree; scalar oracle absent; hermetic tests leave no TMPDIR entries |
| iOS rlib assembly, limiter crate | 0 `bl _memset_pattern16` (base 6); `clear_runtime::<f32x4>` and `::<f32>`: 3 `bl _bzero` each, no panic |
| x86-64-v3 rlib assembly, limiter crate | `clear_runtime::<f32>`, `::<f32x4>`, `::<f32x8>`: 3 `memset` each, no panic or bounds call |
| post-LTO iOS `capi` staticlib assembly | 11 -> 5 `memset_pattern16` calls, none in the limiter; `clear_runtime::<f32>` and `::<f32x4>`: `bzero` only, no panic |

Evidence (small files): `/tmp/claude-1002/v1456b/evidence/`. It holds the gate log, the mutation
log and script (`mutate.py`), the reset and block timings (raw and summarised, both layouts), the
iOS and x86 `clear_runtime` assembly (rlib and post-LTO), the function-diff scripts and their
output, and `worklet.log`.
