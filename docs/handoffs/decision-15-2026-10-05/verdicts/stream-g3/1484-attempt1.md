PASS

# #1484 attempt 1: split a node's flush stall into a peak stall and a tail stall

Commit under review: `03987d646` (parent `06915dc7c`), branch `codex/d15-stream-g3`. Verifier:
opus-xhigh, 2026-10-08. Exported with `git archive` into `/tmp/claude-1002/v1484/tree`;
`CARGO_TARGET_DIR=/tmp/claude-1002/v1484/target`. The worktree was not touched.

No BLOCKER and no MAJOR. Two MINORs (one test gap and one spec gap for root) and four NITs.

## Contract semantics (root's ruling of 2026-10-08, option (2))

I checked each use against (N1) and (N2) in #1379 H4 step 6, step 7 and the chain (i)-(v), in both
the spec and the design note:

- `S_out(v) = g_p(v) max_u S_out(u) + sigma_p,v` is correct. `S_out` feeds `X_v`, and
  `|x_v[n]| <= X_v` must hold at every frame (chain (i): "for every `n`"), so it is an (N1) use.
  Reading `sigma_t` here would be unsound for the live section: its flush part before `T` is
  55-61 dB above `stall(T)`.
- `sigma'_v = sigma_t,v + (3/4) eps 10^(-k) S_in(v)` (else `sigma_t,v`) is correct.
  `sigma'_v` is the node's own additive residual in (N2), so it holds only for
  `n >= M + L + T + k D`. It feeds `epsilon_w` downstream and `Sigma`, so it is an (N2) use. The
  `S_in(v)` part comes from the `(3/4) eps 10^(-k) X_v` term with `X_v <= P U_v + S_in(v)`. That
  term is an every-frame input bound, so it correctly carries the upstream `sigma_p`. The result
  is that the large live every-frame stall enters `Sigma` only multiplied by about
  `(3/4) eps`, which is about `2e-19`. That is the benefit the ruling intends.
- `X*(v) = 10^(P*_graph/2000) 10^(up/2000) + S_in(v)` (step 7) is correct. It bounds a node's
  input peak at every frame for the rest-branch selection, so it is an (N1) use through `S_in`,
  which means `sigma_p`.
- The other pairings are consistent with the readers. `up(v)` uses `G_p` (`peak_clause`).
  `down(v)` and `W_j` use `G_t` (`tail_clause`). `P^ = 4 sigma_t / eps` and "the node exports
  `sigma_t`; the range is derived from it" (finding 2) are (N2) uses.
- Rule (g), `tail_stall <= peak_stall`, holds for every node that will state it:
  - The fixed design: equal, from one value.
  - Filters disabled: equal, one underflow.
  - Every H3 row: equal.
  - The live section: `stall(T)` is 3.77e-15 to 3.85e-15 and `stall(0)` is 2.04e-12 to
    4.49e-12 at the four rates (#1466's record). `ceil_mB` is monotone, so the order survives
    rounding.
  - `NodeTailBound::max` preserves (g): if each operand has `t <= p`, then
    `max(t_l, t_r) <= max(p_l, p_r)`.
- The spec's figures check out: `P^` is 1.29e-4 to 2.84e-4 (-77.8 to -71 dBFS) with one sigma,
  against 2.4e-7.

## Bits of every existing bound (compared against `06915dc7c`)

- I added a probe test, in the exports only, that prints `{:?}` of `input_section_bound` and
  `input_section_bounds`:
  - all 4 launch rates;
  - HPF/LPF from {0, 10, 31.5, 1 kHz, 5 kHz, max-1ulp, max};
  - trims {-24, -3, 0, +6, +24} dB, plus a 0/+6 dB channel split;
  - 671 lines, every one `Stated`.
- I built it at the commit and at the parent. The parent build used a separate target dir: a
  shared target dir silently reused the head's binary, because `git archive` mtimes are older.
- After mapping `stall: X` to `peak_stall: X, tail_stall: X`, the two outputs are byte-identical.
  Every line has `peak_stall == tail_stall` equal to the old `stall`.
- `math::tail` has no diff.
- No rendered bit moved:
  - `audit capi` `pcm_digest` is `cb10fbface44a3a4`, the same as earlier verdicts.
  - `check-builtins-fixtures.sh` passes. It regenerates and compares every PCM, meter and
    response fixture.
  - `run-wasm-gates.sh` passes with its digests unchanged.

## The four byte re-pins (each correct and individually reasoned)

1. `BOXED_TAIL_ENTRY_BYTES` 104 -> 112 (`tools/audit/src/fixture_builtins.rs:337`, with a note).
   On 64-bit, `CompositionBound` goes from 32 to 40 (a second `FlushStall`, 8 B; the `Unstated`
   niche stays in `PeakGain`'s tag), `NodeTailBound` from 88 to 96, and the
   `(Box<str>, NodeTailBound)` entry from 104 to 112. `verify_pinned_native_resource_abi` checks
   the tuple's `size_of`, and it ran green inside `fixture-builtins --check`.
2. `fixtures/builtins/v1/resources.jsonl`: both processor-payload counts and the retained counts
   rise by exactly 16 B per track. That is 2 tail entries per track (the payload's and the seal's,
   `add_vector_layout` at `builtins-compiler/src/lib.rs:3832`, `:3835`) of +8 each:
   - 1 track: +16;
   - 4 tracks: +64;
   - 65,537 tracks: +1,048,592 (= 16 x 65,537).

   The largest allocation and the allocation counts do not move, and the length stays 2,370 B.
   The values are measured: `fixture-builtins --check` regenerates the rows and compares them.
3. `MANIFEST.tsv`: `sha256sum resources.jsonl` = `19f7e3ca...a623`, as pinned. It moves because
   of re-pin 2 alone.
4. The two identity pins (`builtins_graph.rs:54` `ACCEPTED_MANIFEST_SHA256` and the
   `fixture_builtins.rs` test literal) both equal `sha256sum MANIFEST.tsv` = `d0bf6198...eff9`.
   They move because of re-pin 3 alone, and the test site has its own re-pin note.
   `check-builtins-fixtures.sh` cross-checks the builtins_graph constant.

`builtinRetainedBytes` is true: `check-browser-expected-resources.py --artifacts` prints
"builtinRetainedBytes 2033 of 2048 (15 bytes free)". That is 2,017 + 16 (two wasm32 entries of
+8). No ceiling changed and `expected.json` is unchanged. The headroom is now 15 B, which is less
than one more 8-byte field in `NodeTailBound` (that would add +16 on wasm32). See NIT 4.

Render-owned memory is unchanged. The tail entries are control-side (`builtins-compiler`
`PreparedBuiltinsSession.tails` and the seal). The effect processors no longer hold
`PreparedEffectMetadata` (#1461; the compressor's and the EQ's copies are now preparation-time
only).

## Mutation runs (redone, M1-M11 plus 9 of my own)

Each mutant was applied in the export. I ran
`cargo test -p effect-contract --all-features --all-targets` (the whole package) and the release
`tail_contract`, then restored the file (contents and a fresh mtime). The restored baseline is
green, and the restored sources are byte-identical to `03987d646`.

| mutant | red (effect-contract package) | red (release tail_contract) |
|---|---|---|
| M1 `peak_clause` returns `sigma_t` | S1 only | none |
| M2 `tail_clause` returns `sigma_p` | S1 only | none |
| M3 rule (g) dropped | S2 only | none |
| M4 `Zero` above a `Level` in (g) | S2, (e), (f) | none |
| M5 `max` tail stall from the peak stalls | S3 | none |
| M6 `max` tail stall from the left only | S3 | none |
| M7 `max` peak stall from the left only | S3 | none |
| M8 fixed `tail_stall` left at one underflow | none | F3 only |
| M9 disabled-channel `peak_stall` at 0 mB | none | disabled-filter test only |
| M10 (g)'s operands swapped | S2, (e), (f), **and** `tail_bound.rs`'s `a_request_with_another_rows_entry_is_refused`, `prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` | none |
| M11 `peak_clause` returns `G_t` | S1 | none |
| X1 `tail_clause` returns `G_p` | S1 | none |
| X2 `peak_clause` is `Some` for `Unstated` | S1 | none |
| X3 fixed `peak_stall` left at one underflow | none | F3 |
| X4 `max` peak stall from the tail stalls | S3 | none |
| X5 `max` tail stall by minimum | S3 | none |
| X6 (g) admits a `Level` tail over a `Zero` peak | S2 | none |
| **X7 (g) strict `tail < peak`** | **none (survives)** | **none** |
| X8 `max` peak stall from the right only | S3 (existing first assertion) | none |
| X9 `max` tail stall from the right only | S3 (existing first assertion) | none |

Root's gate "a mutant that uses `sigma_t` for N1 goes red" is met: M1 is red at S1, and only at
S1. M1 and M2 are green in `tail_contract`, which confirms the spec's claim that only S1's
distinct fixture can tell the readers' stalls apart.

### Test value, one sentence per new or rewritten test

- **S1 `readers_pair_each_stall_with_its_clause`** (`tail_bound.rs`): a reader that pairs the
  wrong stall or gain with its clause turns it red, which nothing else catches. Two cases matter
  most: `peak_clause` handing `sigma_t` to #1379's `S_out` or to #1466's L1, and `tail_clause`
  putting `sigma_p` into `Sigma`. The fixed section's stalls are equal, so `tail_contract` cannot
  see it (M1, M2, M11, X1, X2).
- **S2 `a_tail_stall_above_the_peak_stall_is_refused`** (`registry.rs`): a registry that drops
  (g), admits a `Level` tail stall over a `Zero` peak stall, or checks only some rate rows turns
  it red. It is the only test of (g) (M3, X6; M4 and M10 also redden (e) and (f)). It does not
  pin equality (MINOR 1).
- **S3, the rewritten `max_states_a_composition_only_when_both_channels_state_one`**: a `max` that
  takes one stall from the other stall field, from one fixed side, or by minimum turns it red
  through the crossed operands in both orders. The pre-#1484 test had a single stall field (M5-M7,
  X4, X5).
- **F3 and the disabled-filter test (rewritten)**: a writer that states the two fixed stalls from
  different values turns them red, and so does a disabled-channel stall that is not one
  underflow. The `effect-contract` suite cannot see these (M8, M9, X3).

## Findings

### MINOR 1: rule (g)'s equality boundary is not pinned

`crates/effect-contract/tests/registry.rs:372` with `crates/effect-contract/src/lib.rs:2720-2724`.

- Mutant X7, `(Level(tail), Level(peak)) => tail < peak`, survives the whole effect-contract
  package and `tail_contract`.
- Equal `Level` stalls are exactly the shape the fixed section states, and the likely shape of
  every effect slice (#1372-#1376). Only the registry checks (g), and no admitted registry
  fixture has `sigma_p = sigma_t` as `Level`s:
  - `COMPOSITION` is -14,000 / -14,100;
  - `LAUNCH_SHAPE_STATED` is a `Zero` tail;
  - the `tail_bound.rs` per-rate fixture is strictly ordered.
- The fix: admit one equal-`Level`-stalls twin in S2. The first effect slice that states a
  composition would hit such a defect quickly, but nothing pins it now.
- (f)'s gain equality has the same pre-existing gap. A follow-up could close both together.

### MINOR 2 (spec gap, for root): one-sigma text outside this slice's named sections

#1484's S-D6 and its Authorized paths name only H1, H3's column, H4 step 6 and the chain. The
implementer stayed inside them, except for H2's carrier line (NIT 3). It recorded the rest as open
items. One-sigma text is still in these places:

- **#1379's spec**:
  - D2a, `:79`: "`sigma` `Zero`" (now both stalls).
  - H2, `:525`: the canonical `tail` row lists "both gains, stall".
  - H8, `:717-719`: "B1 certifies `D`, `G_p` and `sigma` ... B2 ... states all four" (now
    five).
- **Design note**:
  - H2, `:303`: the same canonical-text list.
  - H7, `:588-589`: #1376's restated D4 lists `sigma`.
  - H8, `:622-624`: "B1 certifies `D`, `G_p` and `sigma`".
- **Sibling specs** (root-owned):
  - #1467: `:11` "states all four", `:21`, `:36`, `:49-50` (L-D7 "State all four"),
    `:61`, `:89`, `:108`, `:114` (L-D7 asks for `stall` equal to B1's accessor).
  - #1375: `:28`, `:73`, `:113`.
  - #1376: `:111`.
  - #1468: `:38`, `:68`, `:145`.

Should this slice have changed them? No. They are outside its authorized paths, and the spec
problems belong to root.

Is the canonical plan-text question a genuine open design question? No. H2 already says the
`tail` row "carries the whole bound", so it must carry both stalls, and the list is stale.

Root needs to authorize the text fix. #1467's L-D7 should go first: it is the next consumer, and
as written it tells the implementer to fill one `stall` field that no longer exists.

Rule (g) lives only in `NativeEffectRegistry::new`, and builtin input sections are not registry
rows. So #1466 and #1467 should gate `sigma_t <= sigma_p` for the live bound in `tail_contract`
themselves. #1466's ruling already states it as a requirement.

### NIT 1: the M10 row in the attempt record leaves out two red tests

The Attempt record's M10 row says "S2, plus the (e) and (f) tests (as M4)". Swapping (g)'s
operands also reddens `tail_bound.rs`'s `a_request_with_another_rows_entry_is_refused` and
`prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate`. Their per-rate fixture
states `sigma_t < sigma_p`, so the swapped rule refuses the registry. This is evidence accuracy
only.

### NIT 2: a short line in a doc comment

`crates/effect-contract/src/lib.rs:310-311`: "`0` for\n/// `Zero`. Each value is a" leaves a
short line in the middle of the paragraph. A reflow is cosmetic.

### NIT 3: H2's carrier line edited outside S-D6's list

The implementer edited H2's carrier line in #1379's spec (`:510-512`) and H2's code block in the
design note. Neither is in S-D6's list. The edits are correct, needed, and recorded in the
Attempt record. But the same section's canonical-text line was left (MINOR 2), so H2 is now half
updated.

### NIT 4: retained-bytes headroom, for root's planning

`builtinRetainedBytes` has 15 B of headroom (2,033 of 2,048). Any further 8-byte growth of
`NodeTailBound` (two wasm32 entries, +16) will exceed the ceiling. A later slice that grows the
carrier must stop and report, as this one was ready to.

### Other checks

- `peak_clause` and `tail_clause` are "the only way later slices read the stalls" by convention
  only. Enum variant fields are public, so verifiers of #1466, #1467 and #1379 must enforce it.
- Naming: no version suffix, and no `stall` field or alias is left. No source-grepping or
  prose-grepping test was added.
- Realtime: no render-path change. Both readers are `const fn`.
- The acked-batch question does not apply: there is no queue.

## Gates run (from the export, all green)

- The `test-debug-a` workspace command (all-targets, the CI excludes and features): 1,470
  passed, 0 failed, 10 ignored. Its doctests: 22 passed. `builtins-compiler --no-run` built.
- `cargo test --locked --release -p builtins --features builtins/test-support --test
  tail_contract`: 17 passed, 112 F3 rows.
- `cargo test -p effect-contract --all-features --test tail_bound --test registry`: 4 + 8 passed.
  The whole package passes on the restored baseline.
- `bash scripts/check-effect-contract.sh`: ok, 8 production factories, 0 failed gates.
- `cargo build --locked --release -p audit && audit capi`: 0 allocations, 0 syscalls, 0
  violations, `pcm_digest cb10fbface44a3a4`.
- `bash scripts/check-builtins-fixtures.sh . target/release/audit`: ok (50 files), with the
  corrupted-canary refusal.
- `cargo test --locked -p audit`: 33 passed.
- `cargo run --locked -p conformance --example conformance_fixtures -- --check`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-realtime-policy.sh`: ok.
- `bash scripts/run-wasm-gates.sh`: ok (native + wasm simd128 + V8 EQ loops).
- The worklet chain, all ok, with nothing left in the private TMPDIR:
  - `build-web-audioworklet.sh --named-twin`: shipped module `160cfc8f...14fa5`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`: `builtinRetainedBytes` 2,033 of 2,048,
    32 red self-test mutations;
  - `test-web-audioworklet.sh`.
- `bash scripts/check-cross-targets.sh`: PASS, with the expected #1018 memset failures.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: clean.
  `cargo fmt --all -- --check`: clean.
- Not run: test-debug-b, which the spec's S5 does not list. Its builtins debug `tail_contract` is
  the 48 kHz subset of the release run above. AArch64 runs only in CI.

Disk: free space fell to 16 GB during the debug phase (the run started at 29 GB). I deleted
`target/debug` before the release, wasm and worklet phases.
