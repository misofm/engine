FAIL

# #1456 attempt 1: adversarial verdict

- **Reviewed:** `git diff 2efac7185 8fc41c5b0` on `codex/d15-stream-g2` (one commit, `8fc41c5b0`). The
  limiter source and both scripts are the same at `2efac7185` and at the spec's base `b5cfd2b60`.
  The spec is read at `8fc41c5b0`, with its Attempt 1 record.
- **Method:** I exported `8fc41c5b0` with `git archive` to `/tmp/claude-1002/v1456/tree` and built
  only there (`CARGO_TARGET_DIR=/tmp/claude-1002/v1456/target`). A second export held the variants,
  the base source and the mutants. I did not build, edit or check out anything in
  `/home/bl/misofm/wt-d15-g2`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1. iOS counts come from `cargo rustc --release --target
  aarch64-apple-ios --crate-type rlib --emit asm`, the same command as the ratchet.
- **Verdict:** FAIL, on one MAJOR. The change removes all six iOS calls and moves no bit, and every
  gate passes. But the fill it uses is a scalar strided store loop, and a vector fill through the
  lane crate's `Lane::splat` path also gives 0 calls. That vector fill changes only the body of
  `clear_runtime`, which the spec authorizes, and it resets as fast as the base. Under the root
  prior and the owner's vector rule, the scalar form cannot stay.

## BLOCKER

None.

## MAJOR

1. **The scalar strided fill stays, but a vector fill through `Lane::splat` also gives 0 iOS
   calls** (`crates/true-peak-limiter/src/lib.rs:665-675`, its doc at `:649-660`).

   **Evidence: vector forms.** I measured the forms below in the variant export. Each one replaces
   only the three `1.0` fills, and `new` stays as at head (zeroed). The counts are
   `^\tbl\t_memset_pattern16$` in the whole crate's iOS release assembly:

   | form | where the calls are | iOS calls |
   |---|---|---|
   | base: three `fill(1.0)` | `clear_runtime` | 3 |
   | head: lane by lane, `skip(lane).step_by(width)` scalar stores | -- | **0** |
   | (a) per plane: `<Simd4 as Lane>::splat(1.0)` over `chunks_exact_mut(4)`, plus a scalar tail | `clear_runtime`: each plane's vector loop is one whole-plane call (size `len & !15`), and each tail is another | 6 |
   | per plane: whole `chunks_exact_mut(4)` chunks, no tail (wrong at width 1; measured for the count only) | `clear_runtime`, one call per plane | 3 |
   | generic `L`: per plane, whole `chunks_exact_mut(L::WIDTH)` chunks, no tail | 3 in `clear_runtime::<f32>` + 3 in `::<f32x4>` | 6 |
   | both rings in one `<Simd4 as Lane>` chunk loop, a zipped tail, `prefix` by `fill_v2` | the two `prefix` loops | 2 |
   | **v9:** both rings in one `<Simd4 as Lane>::splat(1.0)` chunk loop, a zipped scalar tail, and `reduction`/`prefix` written in one per-lane loop | -- | **0** |
   | v10: as v9 with `lane::Native` (8 lanes under AVX2, 4 elsewhere) | -- | **0** |
   | v7: generic `L`: both rings zipped over whole `chunks_exact_mut(L::WIDTH)` chunks, no tail; `prefix` is one `L::splat(1.0).store` | -- | **0** |
   | v5: generic `L`: `L::splat(1.0).store` over `chunks_exact_mut(self.width)` (a run-time chunk stride) | -- | **0** |
   | v8: generic `L`: a slot-index loop, `L::splat(1.0).store(&mut ring[slot * width..])` | -- | **0** |

   v9 is the form that stays inside the authorized paths. It changes only `clear_runtime`'s body,
   and `new`, the signatures and `corpus.rs` stay as they are:

   ```rust
   let mut required = self.required_ring.chunks_exact_mut(4);
   let mut boxed = self.box_ring.chunks_exact_mut(4);
   for (required, boxed) in (&mut required).zip(&mut boxed) {
       <Simd4 as Lane>::splat(1.0).store(required);
       <Simd4 as Lane>::splat(1.0).store(boxed);
   }
   for (required, boxed) in required.into_remainder().iter_mut().zip(boxed.into_remainder()) {
       *required = 1.0;
       *boxed = 1.0;
   }
   for (reduction, prefix) in self.reduction.iter_mut().zip(self.prefix.iter_mut()) {
       *reduction = 0.0;
       *prefix = 1.0;
   }
   ```

   In iOS release, v9's ring loop is one 16-byte `stp x16, x16` per ring per four words. The tail
   has at most 3 words, and only at width 1, where `R = rate/100 + 1` is not a multiple of 4. A
   width-4 or width-8 plane has no tail. The `reduction` zero fill also leaves `bzero`
   (`evidence/v9_clear_runtime.s`). With v9, all `true-peak-limiter` tests pass (47 lib tests,
   including `a_lane_reset_is_the_whole_reset_at_one_lanes_stride`, the corpus digests and
   `passes_effect_contract_conformance`). v7 is the whole-chunk form with no scalar tail. It needs
   `ChannelState::new::<L>`, so `crates/true-peak-limiter/src/corpus.rs:228-229` also needs a
   turbofish, and that file is outside the authorized paths.

   **Evidence: cost on the render thread (point 3).** `clear_runtime` runs on the render thread. It
   runs at the D7 recovery of a failed block (`reset_failed_lanes` -> `reset_to_defaults`, inside
   `process`) and at `PreparedNativeEffect::reset`. Descriptive timing, x86-64-v3 release,
   `reset(DiscontinuityKeepParameters)`, both channels, best of 5 x 20,000, ns per reset
   (`evidence/timings.txt`):

   | form | W1 96 kHz | W4 96 kHz | W8 44.1 kHz | W8 96 kHz |
   |---|---|---|---|---|
   | base fills | 223 | 976 | 907 | 1,891 |
   | **head (scalar strided)** | **1,665** | **6,683** | **6,272** | **13,872** |
   | v9 | 289 | 1,218 | 1,147 | 2,353 |
   | v7 | 251 | 1,098 | 1,126 | 2,374 |

   So the head reset is 6.8 to 7.5 times the base, in every cell. At 96 kHz, width 8, that is
   13.9 us for one bank, about 1 % of a 128-frame quantum (1.33 ms), and a seek that resets many
   banks multiplies it. v9 is within 1.3 times the base. The iOS cost is not measured (there is no
   device). The head loop is five instructions per word (`str w`, two pointer updates, a compare
   and a branch), and the base used libc's vector `memset_pattern16`.

   **The doc states more than the evidence shows.** `:652-654` says that "equally a loop of
   `Lane::splat(1.0)` vector stores, whose 16-byte pattern is the same idiom" is rewritten. That
   is true only for a loop that writes one plane (rows 3 to 5 of the table). It is not true of
   vector stores in general (v5, v7, v8, v9, v10). The attempt record's "Shapes tried" has only
   (a) and (b).

   **Robustness, stated plainly.** No form avoids the call by construction, as #1451's loop-free
   array literal did. A ring has a run-time length, so some loop must remain. Head and v5/v8 avoid
   the idiom because the stride is the run-time `width`. v7, v9 and v10 avoid it because two planes
   are written in one loop, so loop-idiom cannot prove that one plane's store does not alias the
   other. In both cases the pass fails to prove something, and the ratchet guards both. So the
   vector form is not more robust than head. Its advantage is the cost (about 6x) and the owner
   rule "no scalar where vector possible".

   **Fix:** use v9, or v7 if root authorizes the `corpus.rs` turbofish. Rewrite the doc at
   `:649-660` with the real mechanism. Record the table above in the attempt record. Keep `new`
   zeroed, because `vec![1.0; n]` would bring back 3 preparation-path calls. Then re-run gate 1's
   differential and gate 2. **Trap:** `lane::Simd4::splat(1.0)` resolves to `wide::f32x4`'s
   *inherent* `splat` (the array-repeat loop that #1451 removed from `Lane::splat`), not to
   `Lane::splat`. With it, v9 gets one `bl _memset_pattern16` per chunk, into a 16-byte stack slot
   inside the loop. Write `<Simd4 as Lane>::splat` or `L::splat`.

## MINOR

1. **`ChannelState::new` now allocates the three `1.0` planes zeroed. This is correct, but it is a
   wider reading of the authorization** (`lib.rs:508-520`). I read the code for any read before
   the reset and found none. `new` builds the struct and then calls `reset_to_defaults`.
   `seed_lane_defaults` writes only `lookahead_ms`, `lane`, `limit` and `release`. `clear_runtime`
   then writes every word of both rings and `prefix` before anything reads them: its box-sum loop
   reads only `lane`. So the `1.0` that `vec![1.0; n]` wrote was dead, and zeroed allocation
   (`__rust_alloc_zeroed`, with no store loop) moves no bit. The base count of 6 = 3 in `new` + 3 in
   `clear_runtime` reproduces. But the spec allows `new` changes "only if the same shape applies",
   and D4 expects 6 -> 3. This is a different mechanism (the dead store is removed, the shape is
   not reused), so the row falls to 0 and not to 3. I find the change correct and better, and the
   comment at `:508-511` states the coupling. **Root:** confirm this reading of the "same shape"
   clause.
2. **The product outcome ("no libc call") is not true as written. It is true for
   `memset_pattern16`.** Head's `clear_runtime` still has four `bl _bzero` (`history`, `main_ring`,
   `reduction`, `phase`; `evidence/head_clear_runtime.s`). `bzero` is a Darwin libc call on the
   same render-reachable path. D2, the gates and the ratchet cover only the three `1.0` fills and
   `memset_pattern16`, and the implementer recorded the `_bzero` calls as an open item. So the
   spec's own claim is the issue, not the implementer's work. AGENTS.md forbids allocations,
   locks, syscalls and I/O in render. `docs/TARGET_MATRIX.md:136` says that no audit sees a libc
   call that is not an allocation, lock or syscall. Whether `bzero`/`memcpy` in render count as a
   defect is therefore a policy question, and #1018's framing ("a libc call, which the realtime
   rules forbid") is wider than that policy. **Root:** restate the outcome as "no
   `memset_pattern16` call", or have an issue own the `bzero`/`memcpy` question. (v9 makes the
   count three, because `reduction` moves into the per-lane loop.)
3. **Two documents outside the authorized paths are now stale** (point 5):
   - `scripts/check-cross-targets.sh:104-108` still lists the limiter's "three `fill(1.0)` in
     `ChannelState::new` and three in `clear_runtime`" among "the 16 left".
   - `docs/TARGET_MATRIX.md:186-198` still has the `true-peak-limiter | 6` row and "Only the
     limiter's `clear_runtime` is reachable from render". After this slice no remaining row is
     render-reachable, and the total is 10.

   The #1451 verdict's MINOR 2 already found that the TARGET_MATRIX #1018 entry has no owner.
   **Root:** authorize both paths in this slice or the batch follow-up.

## NIT

1. **`step_by(width)` adds a panic branch to render-reachable code** (`lib.rs:668`, `:671`). The
   iOS assembly has `cbz x16, LBB16_26` -> `core::panicking::panic` ("step != 0"). Width is never
   0, so the branch is dead, but it is new. A move to v9 removes it.

## Answers to the six points

1. **`new` zeroed:** in scope only under a wide reading of the "same shape" clause (MINOR 1). It is
   correct, and no read comes before the reset.
2. **Row deleted at zero:** correct. At count 0, `judge_memset` refuses a row ("now passes: delete
   its row", `aarch64-known-defects.py:152-154`), so a row at 3 would fail the check. I ran the
   judge with this commit's table: limiter 0 -> rc 0; limiter 1 -> rc 1 ("1 memset_pattern16 calls
   and no row"); limiter 3 -> rc 1.
3. **Render thread:** yes, through D7 recovery inside `process`, and through `reset`. The scalar
   form costs 1.7 / 6.7 / 13.9 us per reset at widths 1, 4 and 8, 96 kHz, on x86-64-v3, against
   0.2 / 1.0 / 1.9 us for the base and 0.3 / 1.2 / 2.4 us for v9 (MAJOR 1). That is about 31k
   scalar stores at width 8, 96 kHz, as the record says.
4. **`_bzero`:** four remain, so "no libc call" is not true as written (MINOR 2).
5. **`check-cross-targets.sh` comment:** stale, and so is `TARGET_MATRIX.md` (MINOR 3).
6. **No committed test:** confirmed, none is needed. The judge-memset ratchet turns red if the call
   comes back (shown above). The existing tests turn red if a reset word is misplaced. I ran two
   mutants of head's new loop, and both failed:
   - `prefix.iter_mut().enumerate().skip(1)` (lane 0's rings and `prefix` are not written): 34
     tests red (24 lib tests and 10 in other targets), including `a_lane_reset_is_the_whole_reset_at_one_lanes_stride` and
     `both_resets_return_the_runtime_state_to_a_silent_lane`.
   - `required_ring...skip(lane + width)` (slot 0 is not written): 6 lib tests red, plus
     `passes_effect_contract_conformance`.

   (`evidence/mutations.txt`.) I did not re-run the implementer's 5 GB base-versus-head
   differential, which was not committed. The class-A argument is direct: the same `1.0` goes to
   every index of both rings and `prefix`, and the mutants show that the suite catches a
   misplaced word.

## Test value

This slice adds no test, so no test-value sentence is due. The ratchet that guards the change is
`judge-memset` in `scripts/check-cross-targets.sh`. With the row deleted, any returning call makes
the judge red ("calls and no row"), as verified above.

## Gates run (on the `8fc41c5b0` export)

| gate | result |
|---|---|
| `cargo test --locked --all-targets -p true-peak-limiter -p conformance` | PASS, 89 passed (rerun; see the process note) |
| `test-debug-a` workspace command (`qualification.yml:625-634`) | PASS, 1,458 passed |
| `cargo run --locked -p conformance --example conformance_fixtures -- --check` | PASS |
| `bash scripts/run-wasm-gates.sh` | PASS (native + wasm simd128 + V8 EQ loops) |
| `bash scripts/check-cross-targets.sh` | PASS; the limiter has 0 calls and no row; builtins 5, host-core 4, soft-clip 1 |
| `bash scripts/check-workspace-policy.sh` | PASS |
| `bash scripts/check-realtime-policy.sh` | PASS |
| `python3 -B scripts/lib/aarch64-known-defects.py --self-test` | PASS |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| iOS release assembly of the limiter crate, whole crate | 0 `bl _memset_pattern16` (base: 6, which is 3 in `new` + 3 in `clear_runtime`) |

**Process note:** the first run of `cargo test -p true-peak-limiter -p conformance` was red (6
failed). That run shared its target directory with my mutant export. Cargo hashes a path package by
its workspace-relative path, so the build reused the `m2` mutant's binary: it had the same 6
failures. I touched every source in the export and ran the gates again. The rerun is the result
above.

Evidence (small files): `/tmp/claude-1002/v1456/evidence/`. It holds `ios_counts.txt`,
`variant.py` (every form, exactly; it reads head's `lib.rs`, which is `git show 8fc41c5b0:crates/true-peak-limiter/src/lib.rs`), `timings.txt`, `mutations.txt`, `head_clear_runtime.s`,
`v9_clear_runtime.s`, `base_memset_sites.txt` (the base's six call sites), `cross_targets_tail.txt` and `gates.log`.
