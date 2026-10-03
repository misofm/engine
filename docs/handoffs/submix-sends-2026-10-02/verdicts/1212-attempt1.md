# #1212 *Add the notSoloable command reason to every vocabulary spelling*: Sol verdict, attempt 1

- Reviewed: `git diff 4ee4d3f30 047a503d6` (branch `codex/batch-submix-k2`, worktree
  `/home/bl/misofm/wt-submix-k2`): 19 files, +124/-44.
- Binding: `AGENTS.md` (test-value rule, in-place V1 amendment) and
  `.github/ISSUE_SPECS/1212-add-the-notsoloable-command-reason-to-every-vocabulary-spelling.md`
  with its Attempt 1 record; DESIGN 5.11 and its allocation table (`DESIGN.md:839-840`).
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. The worktree carries #1213's uncommitted
    edits; I used only `git show` and `git archive`.
  - `gh issue view 1212`: OPEN, and the title matches the spec's H1.
  - I exported `047a503d6` to `/tmp/claude-1002/v1212/src`, copied `sdk/node_modules`, and used my
    own `CARGO_TARGET_DIR`. Mutations were run on copies, or in place with byte-for-byte restores,
    each restore checked with `cmp` against `git show 047a503d6:<path>`.
- The amend (`74cc98de0` -> `047a503d6`) adds only the Attempt 1 record to the spec. The code
  tree is the same in both commits, so the commit is coherent.

## Verdict: PASS

There is no BLOCKER, no MAJOR and no MINOR finding, and there are three NITs. No further attempt
is needed.

## D1: reason 12 is identical in every spelling

| Spelling | At `047a503d6` |
|---|---|
| Rust authority (`host-web/src/lib.rs:989`) | `pub const COMMAND_REASON_NOT_SOLOABLE: u32 = 12;`, public, with a doc that says why it is not `MALFORMED` or `UNKNOWN_TRACK` |
| Host JS `COMMAND_REASONS` | `"notSoloable"` is appended at index 12. The bound is still derived from the table length |
| Host `.d.ts` and SDK mirror | `NotSoloable = 12` with the D1 doc line. `cmp` shows the two files are identical |
| Metadata generator (`parameter-metadata/src/lib.rs`) | Row and import added |
| Layout generator (`abi_layout.rs`) | Row and import added. `cargo fmt` rewrapped the `use` list, whitespace only |
| `check-parameter-metadata-v1.py` and `check-abi-layout-v1.py` | `"notSoloable"` appended to both lists |
| `sdk/assets/*.json` and `sdk/src/generated/{abi,catalog}.ts` | One reason row each. Byte-equal to `--print` and `--print-abi-layout` from the commit's generator |
| Both self-test fixtures | Byte-equal to the generator output. The parameter-metadata fixture has no currency test, so I compared it with `cmp` myself |

- **No retired code is reused.**
  - `git log -G 'COMMAND_REASON_[A-Z_]+: u32 = 1[2-9]'` finds only self-test mutation text
    (#151) and the design record (#1197). No reason 12 or higher ever shipped.
  - `NOT_SOLOABLE` first appears in this commit.
- **The SDK picks up the new reason by itself.** `commandReasonName()` reads the regenerated
  `ABI_LAYOUT`, and `CommandReasonName` is derived from `CATALOG`, so a reason-12 acknowledgement
  maps to `"notSoloable"` and does not throw.
- **D3 is confirmed.** The `refuse` closure's default arm (`lib.rs:4367-4377`) maps reason 12 to
  `RESULT_INVALID_ARGUMENT`. No Rust code bounds reasons at 11.

## D2: each moved drift mutation still fires the rule it claims

I wrapped `validate` and printed the rule each self-test mutation fires. All 20 reason mutations
go red, and each one goes red through the rule it is named for:

| Mutation | Rule that fires now | With the old value on the new tree |
|---|---|---|
| FUTURE_TAP `= 13` after `NOT_SOLOABLE` | Drift: `host JS table disagrees with the Rust host constants`; the authority ends `(12, notSoloable), (13, futureTap)` | `= 12` would fire **contiguity**, not drift. The move was necessary |
| `UNKNOWN_TAP` set to 13 | Contiguity | `= 12` would also fire contiguity, but because it duplicates 12 |
| Worklet `UNSUPPORTED_KIND = 13` | `the worklet JS names reasons the Rust host constants do not: [(13, 'unsupportedKind')]` | `= 12` would collide with a real reason |
| Host JS table truncated (anchor now includes `notSoloable`) | Host JS table disagrees | The old anchor would leave `notSoloable` at index 10, so it would no longer be "stops at wrongState" |
| Schema list truncated (re-anchored the same way) | Schema gate list disagrees | Same as the row above |
| `reason <= 12` | `validCommandReason() no longer derives its bound` | Unchanged rule |
| Metadata `commandReasons[12].value = 13` | `command reason values` | `[11] = 12` would fire the same rule through a duplicate value |

- **The on-disk copy in `test-web-audioworklet.sh`.** I replayed its `sed`. The diff is the one
  inserted line, and the gate exits 1 with the drift message, not the contiguity one.
- **`MUTATIONS.md`.** The drift row now says `= 13` after reason 12, and the self-test rows say
  `reason <= 12` and "renumber reason 12 to `13`". This is accurate.
- **The other self-tests have nothing to move.**
  - `check-abi-layout-v1.py --self-test` has no reason mutation.
  - `check-command-kind-vocabulary.py` is untouched, and its self-test passes with 32 red
    mutations.

## The kind-vocabulary collision belongs to #1222, and it masks nothing here

- **It already exists on `main`.** The kind self-test's "a Rust kind is added without the other
  spellings" mutation (`check-command-kind-vocabulary.py:395`) inserts `COMMAND_SOLO_MODE = 12`.
  `COMMAND_INPUT_FILTERS = 12` has existed since `0e98fd5c6`, so the mutation is on `main` today
  and is not new in K2.
- **I measured it.** As committed, the mutation fires *contiguity*. Moved to 13, it fires the
  `.d.ts` drift rule.
- **It does weaken a test, but a different one.** The mutation is no longer the drift test it
  claims to be. That is a real gap in the kind gate, but it is in a different vocabulary and a
  different file, and the file is not on #1212's authorized paths. It has no effect on the reason
  vocabulary.
- **#1222 owns the fix, by name.** `DESIGN.md:839` assigns it to slice 24, and the #1222 spec
  (`:73`, `:159-163`) moves it to 16 and also moves FUTURE_TAP from 13 to 14.

## Gates I re-ran on `047a503d6` (all rc 0)

- `check-command-reason-vocabulary.py --self-test` (20 red) and the bare run. I also ran it with
  `--artifacts <A>` against the built metadata JSON.
- `check-parameter-metadata-v1.py --self-test`; `check-abi-layout-v1.py --self-test` (22
  caught); `check-command-kind-vocabulary.py --self-test` (32).
- `bash scripts/test-web-audioworklet.sh`: every stage passes.
- `build-web-audioworklet.sh --named-twin <B> <A>`. The shipped module is `19d19812…`, which
  matches Terra's record. Then:
  - both schema gates on `<A>`;
  - `check-web-audioworklet.sh <A> <B>/…named.wasm`;
  - `check-sdk-generated.sh <A>`;
  - `check-sdk-types.sh`.
- `check-sdk-headless.sh <A>`: 339 pass. This gate is not in the spec; I ran it as a bonus.
- `cargo test --locked -p parameter-metadata` (5 + 5), `-p host-web --all-targets` (129 + 3), and
  `-p session-validator --all-targets`. `session-validator` and `parameter-metadata` are the only
  reverse dependencies of `host-web`.
- `cargo fmt --all -- --check`.
- `cargo clippy --locked -p host-web -p parameter-metadata --all-targets --all-features -- -D warnings`.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p host-web -p parameter-metadata --no-deps`.
- `check-workspace-policy.sh` and `test-workspace-policy.sh`.

**Not re-run:**
- the full-workspace `cargo test` and the full-workspace `clippy`, because of the disk budget.
- **Why that is safe.** The Rust change is one public constant and two generator rows. Clippy
  covered every changed Rust file, and every crate that depends on `host-web` was tested. Terra
  records both full-workspace gates green.

## Test value

The new `{ reason: 12, result: 1 }` row in the reasons loop. **Which plausible defect turns it red
that no existing test catches?** A host acknowledgement validator that refuses a reason-12 ack.
The static vocabulary gate cannot see it when the refusal sits outside the
`validCommandReason`/stray-bound regexes. In that case #1213's solo refusal would trip the sticky
255 and fail the whole host.

- **Mutation C (the unique catch).** In `#receive`, I added
  `&& COMMAND_REASONS[message.reason] !== "notSoloable"` to `validCommandAck`.
  - The vocabulary gate stays **green**.
  - `test-web-audioworklet.mjs` goes **red** with `{ tag: 'miso.error.v1', requestId: 286, result: 255 }`.
  - With the reason-12 row deleted, the same mutated host passes (rc 0). The row is the only
    catch.
- **The vehicle is the right path.** `observe()` goes through `this.command()`, so the row
  exercises the same `validCommandAck` a kind-9 refusal will reach.
- **Mutation A.** Dropping `"notSoloable"` from the host JS table makes the mjs go red with the
  same sticky signature, which reproduces Terra's evidence. The vocabulary gate also catches
  this one.

## Other mutations (each restored, `cmp`-verified)

- **E: drop the `abi_layout.rs` reason row.** `the_checked_in_self_test_fixture_is_current` goes
  red ("abi-layout-v1-self-test.json is stale"). The reason gate stays green, because by design
  it does not read the layout generator, so the layout spelling is held by this test and by the
  layout gate.
- **F: rename `NotSoloable` to `SoloSafe` in the SDK mirror only.** `check-sdk-generated.sh` goes
  red: "shipped-host.d.ts is not the shipped declaration".
- **G: misspell `notSoloable` in `check-abi-layout-v1.py`.** Its self-test baseline is rejected,
  and the gate refuses `<A>`'s layout (`constants.commandReasons is not the contiguous vocabulary`).

## Findings

- **NIT-1 (already wrong before this slice).** The `MUTATIONS.md:133` row still says "eighteen
  in-memory mutations". The self-test has 20, at both the base and this commit. The row was
  edited here, so this was the moment to fix the count, or to drop it.
- **NIT-2.** Gate 1's prose says the reasons-loop row "shows the host accepts reason 12 with
  `RESULT_INVALID_ARGUMENT`". The `result: 1` is injected by the mock, and the host JS does not
  check that reason and result agree. The D3 mapping (Rust `refuse` sends 12 to 1) is therefore
  untested until #1213 emits the reason. **#1213 should assert `result === 1` alongside
  `reason === 12`** on a real kind-9 solo at a submix index.
- **NIT-3 (note only).** The constant pin in `hosts/host-web/src/tests.rs:353-368` still lists
  reasons 0-11. That file is not on the authorized paths, the reason gate's contiguity rule covers
  the Rust authority, and the kinds pin has the same pattern. No action is needed.

## Root ledger lines

- **Merge.** #1212 PASS with NITs only, so it can join the K2 batch.
- **For #1213.** Gate the kind-9 submix solo refusal on both `reason === 12` and `result === 1`
  (the D3 mapping).
- **For #1222.** It must move the kind self-test's `COMMAND_SOLO_MODE` drift mutation from 12 to
  16 (it fires contiguity today, measured) and move FUTURE_TAP from 13 to 14, as its spec already
  says.
