# Add the notSoloable command reason to every vocabulary spelling

Slice 15 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The browser command vocabulary gains reason 12, `notSoloable`: "this strip cannot be soloed". An
agent that sends a solo to a bus learns exactly why it was refused, instead of a reason that names
the wrong thing. This slice adds the reason in every spelling the vocabulary gates hold together,
and moves the drift self-test that already uses 12 as its "unallocated" value, so the gates keep
testing what they claim. Nothing emits the reason yet: *Address submix strips in browser live
commands* (#1213) refuses a kind-9 solo at a submix index with it.

## Context (verified on `fe8ac679`)

No earlier K2 slice touches the reason vocabulary.

- **Reasons are 0-11** (`hosts/host-web/src/lib.rs:926-966`; `COMMAND_REASON_OBSERVATION_UNBOUND = 11`
  at `:966`). An appended reason takes the next value, 12 (in-place V1 amendment; DESIGN 5.11).
- **The spellings** that `scripts/check-command-reason-vocabulary.py` holds to one another:
  1. the Rust constants (`hosts/host-web/src/lib.rs:926-966`);
  2. the host JS `COMMAND_REASONS` table (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:223-236`),
     whose acknowledgement bound is derived from its length;
  3. the `.d.ts` enum `MisoCommandReason`
     (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `ObservationUnbound = 11` at `:316`),
     mirrored byte for byte in `sdk/src/browser/shipped-host.d.ts`;
  4. the metadata generator's rows (`tools/parameter-metadata/src/lib.rs:193-211`);
  5. the schema gate's list (`scripts/check-parameter-metadata-v1.py:53-56`);
  6. the shipped metadata JSON (generated).

  Also: the layout generator's rows (`tools/parameter-metadata/src/abi_layout.rs:1757-1770`, and its
  import at `:58`), the layout gate's list (`scripts/check-abi-layout-v1.py:81-84`), and the
  generated or fixture copies: `sdk/assets/miso-engine-v1-parameter-metadata.json:32`,
  `sdk/assets/miso-engine-v1-abi-layout.json:616`, `sdk/src/generated/{abi.ts,catalog.ts}`,
  `scripts/fixtures/abi-layout-v1-self-test.json:616`, and
  **`scripts/fixtures/parameter-metadata-v1-self-test.json:32`** (validated at
  `scripts/check-parameter-metadata-v1.py:233`, `:544-545`; VERIFY-2 M8).
- **Self-test mutations that use 12 as an unallocated value** (they would still go red with a real
  reason 12, but through a duplicate or contiguity rule rather than the drift rule they claim):
  - `check-command-reason-vocabulary.py:326-334`: "a Rust reason is bumped without the other five
    spellings" inserts `COMMAND_REASON_FUTURE_TAP: u32 = 12` after the reason-11 line;
  - `:336-341`: "renumbered out of the contiguous run" moves `UNKNOWN_TAP` to 12;
  - `:409-414`: "the worklet JS renumbers the reason it produces itself" moves it to 12;
  - `scripts/test-web-audioworklet.sh:245-256` performs the FUTURE_TAP insertion on a copied tree,
    documented at `hosts/host-web/MUTATIONS.md:132`;
  - `scripts/check-parameter-metadata-v1.py:637`: "reason value renumbered" sets `[11]`'s value to 12.
- **Gate commands.** `check-command-reason-vocabulary.py` runs bare and with `--self-test`; both run
  inside `scripts/test-web-audioworklet.sh` (`:229-230`). `check-parameter-metadata-v1.py` and
  `check-abi-layout-v1.py` need `--self-test` or a document path; a bare call exits 2 (VERIFY-2 M9).

## Decisions frozen for this slice

- **D1.** Reason 12 is `notSoloable`: Rust `COMMAND_REASON_NOT_SOLOABLE: u32 = 12`, JS and metadata
  name `notSoloable`, `.d.ts` `NotSoloable = 12` with a doc line ("the addressed strip is solo-safe:
  a submix is never soloed"). It is added in every spelling listed in the Context, and the generated
  files are regenerated.
- **D2. The drift self-tests keep testing drift.**
  - The FUTURE_TAP mutation is re-anchored on the new last line and inserts
    `COMMAND_REASON_FUTURE_TAP: u32 = 13`, in `check-command-reason-vocabulary.py`,
    `test-web-audioworklet.sh` and `hosts/host-web/MUTATIONS.md:132`.
  - The renumbering mutations at `:336-341` and `:409-414` use 13.
  - `check-parameter-metadata-v1.py:637` renumbers the new last reason (`[12]`, value 13).
- **D3.** Nothing emits the reason in this slice; the Rust constant is public, so no dead-code lint
  fires. When *Address submix strips in browser live commands* emits it, its result code is
  `RESULT_INVALID_ARGUMENT` (1): the `refuse` closure's default arm (`hosts/host-web/src/lib.rs:4301-4310`)
  maps every reason other than the unsupported, unbound and backpressure ones to it.

## Deliverables

- D1 and D2.
- Regenerate the SDK assets and generated code (`node codegen/assets.mjs && node codegen/generate.mjs`
  in `sdk/`), and the self-test fixtures where the generator owns them.
- A `hosts/host-web/MUTATIONS.md` row update for the moved drift mutation.
- A `{ reason: 12, result: 1, what: "a strip that cannot be soloed" }` row in
  `scripts/test-web-audioworklet.mjs`'s reasons loop (`:1820-1827`).

## Authorized paths

- `hosts/host-web/src/lib.rs` (the reason constant and its doc only), `hosts/host-web/web/**`,
  `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/src/{lib.rs,abi_layout.rs}` and `tools/parameter-metadata/tests/`
- `scripts/check-command-reason-vocabulary.py`, `scripts/check-parameter-metadata-v1.py`,
  `scripts/check-abi-layout-v1.py`, `scripts/test-web-audioworklet.sh`,
  `scripts/test-web-audioworklet.mjs` (the reasons-loop row only),
  `scripts/fixtures/abi-layout-v1-self-test.json`, `scripts/fixtures/parameter-metadata-v1-self-test.json`
- `sdk/src/browser/shipped-host.d.ts`, `sdk/assets/**`, `sdk/src/generated/**` (regenerated only)
- this spec

## Non-goals

- No admission change: nothing refuses with reason 12 yet.
- No command kind, export or layout structure change.

## Hazards

- **A missed spelling** fails the vocabulary gate, and a missed self-test anchor makes a mutation
  "match nothing", which fails the self-test. Both are intended; fix them, never silence them.
- **The mirror.** `sdk/src/browser/shipped-host.d.ts` must stay byte-identical to the host `.d.ts`.

## Objective gates

1. **Vocabulary.**
   - `python3 -B scripts/check-command-reason-vocabulary.py --self-test`
   - `python3 -B scripts/check-command-reason-vocabulary.py`
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test`
   - `bash scripts/test-web-audioworklet.sh` (the FUTURE_TAP mutation, now 13, still goes red through
     the drift rule, and the new `{ reason: 12 }` row in `scripts/test-web-audioworklet.mjs`'s reasons
     loop shows the host accepts reason 12 with `RESULT_INVALID_ARGUMENT` in a refusal)

   *Test value (the reason-12 row): it turns red if the host's reason vocabulary or its refusal
   validator rejects reason 12, so the refusal *Address submix strips in browser live commands*
   emits would fail the host whole.*
2. **Artifact and SDK.**
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`
   - `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh <A>`
   - `bash scripts/check-sdk-types.sh`
3. **No behaviour moves.** The workspace test command (DESIGN.md section 7) and
   `cargo test --locked -p parameter-metadata` pass with no test edited other than the re-anchored
   self-tests. No new test is added: the vocabulary gates and their self-tests are the test, and their
   mutation results are the evidence.
4. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`

## Evidence

- The output of every gate command above, from the PR's head commit.
- The re-anchored mutation list, with each mutation's observed red result and the rule that fired.
- The regenerated asset diff (one reason row).

### Attempt 1 record (Terra)

- **D1.** `COMMAND_REASON_NOT_SOLOABLE: u32 = 12` after reason 11 (`host-web/src/lib.rs`, found by
  symbol at `:984`; the doc says why it is not `MALFORMED`/`UNKNOWN_TRACK`); `"notSoloable"`
  appended to the host JS `COMMAND_REASONS`; `NotSoloable = 12` with the D1 doc line in the host
  `.d.ts` and its mirror (`cmp` identical); a row in both generators (`lib.rs`, `abi_layout.rs`,
  with their imports); `"notSoloable"` appended to both schema-gate lists. Regenerated: both
  `sdk/assets/*.json`, `sdk/src/generated/{abi,catalog}.ts` (one reason row each; `provenance.ts`
  unchanged), and both self-test fixtures, which are byte-identical to the generator's `--print` /
  `--print-abi-layout` output.
- **D2, re-anchored mutations and the rule each fires** (printed by wrapping `validate`):
  - FUTURE_TAP, now inserted after `NOT_SOLOABLE = 12` as `= 13` (the in-memory self-test and the
    on-disk copy in `test-web-audioworklet.sh`) -> `host JS table disagrees with the Rust host
    constants` (authority ends `(12, notSoloable), (13, futureTap)`): the drift rule.
  - `UNKNOWN_TAP` renumbered to 13 -> `the Rust reason constants are not contiguous from 0`.
  - worklet `COMMAND_REASON_UNSUPPORTED_KIND = 13` -> `the worklet JS names reasons the Rust host
    constants do not: [(13, 'unsupportedKind')]`.
  - metadata `commandReasons[12]` value 13 -> `command reason values`.
  - **Also re-anchored (not named in D2; same file, same reason):** "the host JS table stops at
    wrongState" and "the schema gate's list stops at wrongState" now remove `notSoloable` too, so
    they still truncate at `wrongState` as named. "The host JS bound stops deriving from the
    table" now writes `reason <= 12`, the literal that behaves like the 13-entry table. All three
    still fire the rules they fired before.
  - The self-test totals are unchanged: 20 reason mutations, 22 abi-layout mutations, and the
    metadata self-test passes.
  - `MUTATIONS.md`: the drift row names `= 13` after reason 12; the self-test rows name
    `reason <= 12` and "renumber reason 12 to `13`".
- **Reasons-loop row** `{ reason: 12, result: 1, what: "a strip that cannot be soloed" }`. Test
  value: it goes red if the host's reason vocabulary or refusal validator rejects reason 12, so
  #1213's refusal would kill the host. Mutation: drop `"notSoloable"` from the host JS table ->
  `node scripts/test-web-audioworklet.mjs` goes red with
  `{ tag: 'miso.error.v1', requestId: 286, result: 255 }` (the sticky signature).
- **Kind-vocabulary collision** (`COMMAND_SOLO_MODE = 12` vs `INPUT_FILTERS = 12` in
  `check-command-kind-vocabulary.py`): this file is not in this slice's authorized paths.
  DESIGN.md:839 assigns it to slice 24 (#1222), which moves it to 16. Left untouched here.
- **Gates** (all rc 0):
  - `check-command-reason-vocabulary.py --self-test` (20 red mutations) and the bare run
    (spellings agree).
  - `check-parameter-metadata-v1.py --self-test`; `check-abi-layout-v1.py --self-test`
    (22 caught).
  - `test-web-audioworklet.sh`: every stage passes, including the reason gates.
  - `build-web-audioworklet.sh --named-twin <B> <A>` (shipped module `19d19812…`); both schema
    gates on `<A>`; `check-web-audioworklet.sh <A> <B>/…named.wasm`; `check-sdk-generated.sh <A>`
    (assets and generated code current); `check-sdk-types.sh` (the mirror pin passes).
  - `cargo test --locked -p parameter-metadata` (5 + 5 pass, including the fixture-current test).
  - The DESIGN section 7 workspace test command: 103 test binaries, 1162 tests, 0 failed.
  - `cargo fmt --all -- --check`; `cargo clippy … -D warnings`; `check-workspace-policy.sh`;
    `test-workspace-policy.sh`.
  - `cargo fmt` rewrapped the two generator `use` lists after the build and web gates ran. The
    change is whitespace only. Clippy, the workspace tests and the policy gates ran on the final
    tree.

## Dependencies

- *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205, batch K1 closed and
  pushed)

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- In-place V1 amendment for reasons: append, never renumber or reuse.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
