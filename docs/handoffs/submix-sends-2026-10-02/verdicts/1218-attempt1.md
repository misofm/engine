# #1218 *Let a route into a submix follow its source strip's mute in the session*: Sol verdict, attempt 1

- Reviewed: `git diff 071c6c14a 0da034dba` (75 files, +1702/-379). Branch `codex/batch-submix-k3`,
  worktree `/home/bl/misofm/wt-submix-k3`. The uncommitted #1217 attempt 2 changes in that worktree
  (`runtime.rs`, two test files) were ignored, as instructed.
- Binding: `AGENTS.md`; `.github/ISSUE_SPECS/1218-...md` with its Attempt 1 record; DESIGN P11 and
  P13; #1053's spec (the P13 annotation); #1224's spec (for the solo hand-off); `host-core/src/solo.rs`
  (#1211's `effective_mute`).
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1218` is OPEN, and its title
    matches the spec's H1.
  - I exported head `0da034dba` and parent `071c6c14a` with `git archive` under `/tmp/claude-1002/v1218/`,
    each with its own target directory. I ran mutations and probes in a third copy, so the gate tree
    stayed pristine. A fourth copy was head plus #1217 attempt 2's `#[inline(always)]` fix, used only to
    run the worklet gate.
  - My probe tests are saved beside this file as `1218-attempt1-verifier-scratch.rs`.
  - All scratch under `/tmp/claude-1002/v1218/` was deleted afterwards.

## Verdict: PASS

**There is no BLOCKER and no MAJOR attributable to this slice.** The semantics match D1-D5 and P11,
every spec gate is green, and no output bit moves for a session without a follow. Three MINORs and
three NITs follow.

**One batch-level fact you must know (inherited, not #1218's):** the required gate
`scripts/check-web-audioworklet.sh` **fails on head `0da034dba`**.

- It fails **identically on parent `071c6c14a`**: the same eight `reduce_group_into<f32x4, 1..8>`
  kernel-shape failures, with the same vector and scalar counts. This is #1217 attempt 1's BLOCKER-1.
- With #1217 attempt 2's fix applied on top of this head, the gate **passes**: `#[inline(always)]` on
  `reduce_group` and `reduce_group_into`. The output ends with "web AudioWorklet static/object checks
  passed".
- #1218 adds nothing to the failure. K3 cannot push until #1217 attempt 2 commits.

## Findings, by severity

### MINOR-1: a codec that confuses route fields 6 and 7 stays green (test-value gap)

Gate 7's test-value claim is "red if the field is ... lost in the codec". The only committed test that
encodes a `Route` with `follows_mute: true` also sets `mute: true`. That is
`every_route_and_automation_opcode_round_trips_canonically`, `crates/protocol/src/session_wire/tests.rs:1386-1391`.
Every fixture and corpus route has both fields `false`. So a copy-paste error between the two
booleans is invisible. I ran two such mutations:

- **M10**, `parse_route` reads `follows_mute` from `route::MUTE` (`session_wire.rs:1760`). The
  `protocol` suite (all binaries, 129 unit tests and more) and the `conformance` suite stayed **GREEN**.
- **M10b**, `tx_route` writes `value.mute` into field 7 (`session_wire.rs:1059`). The same suites
  stayed **GREEN**.

Either defect would turn the common send, `mute: false, follows_mute: true`, into a non-following send
on the C ABI wire.

**Fix.** In that round-trip test, give the upserted route distinct values: `mute: false,
follows_mute: true`. Better, add a second `UpsertRoute` with `mute: true, follows_mute: false`, so both
directions are covered. Then record M10 and M10b red.

### MINOR-2: a normative doc claim is false at the bit level

`docs/SESSION_SCHEMA_V1.md` says: "from a `post_fader` or `post_pan` tap the fader mute already
silences the lane, so the follow changes only which work is skipped". The spec's Context says the same.

That holds only for a partially muted source. It does not hold when **both** source lanes are muted.
The fully followed send is then inactive (the #1217 rule), so the bus gets the `+0.0` fill. Without the
follow, the send mixes `+0.0 * c`, which is a signed zero for a negative coefficient.

**Probe** (`verifier_post_fader_follow_bits`): track `t` with both lanes muted, a send with a
negative matrix into bus `b`, and `b` feeding the output.

- **Both lanes muted:** follow and no-follow differ in **2048 of 2048** output samples, for both
  `post_fader` and `post_pan`. Every difference is `-0.0` against `+0.0`. Nothing is audible, but the
  difference is digest-visible, which `solo.rs` treats as a correctness matter.
- **One lane muted:** 0 samples differ.

**Fix.** Change the sentence to: "...so the follow changes no audible sample. A send whose two source
lanes are muted is skipped, and its silent contribution becomes `+0.0` instead of a signed zero."
Amend the spec's Context sentence the same way.

### MINOR-3: the evidence omits `check-web-audioworklet.sh`

This slice changes `graph-compiler` and `session`, and both are compiled into the worklet module. The
K3 lesson (`K3-followups.md`) makes this gate mandatory, but the Attempt 1 record does not list it.

**Fix.** Add a gate row: "`check-web-audioworklet.sh`: FAIL on head, identical on parent `071c6c14a`
(#1217 BLOCKER-1: eight `reduce_group_into<f32x4, N>`); PASS with #1217 attempt 2's `#[inline(always)]`
applied over this head."

### NIT-1: an invariant is masked as a silent default

`compile.rs` does `strip_mutes.get(source).copied().unwrap_or([false; 2])`. Validation guarantees that
the source strip exists, so a miss is a broken invariant. As written, it silently produces a send that
does not follow. Use `.expect("a validated route source names a strip")`.

### NIT-2: #1058 does not know about `follows_mute`

Builtin automation of parameter 6 (`mute`) is valid and inert today (`validate.rs:690-712`). When
*Research: render stored session automation* (#1058) renders stored mute automation, a follow send must
follow the automated mute through the same route composition as #1224. Otherwise a saved session with
automated mute leaks its pre-fader sends. #1058's spec never mentions `follows_mute`.

**Fix.** Annotate #1058's spec, as slice 00 did for #1053.

### NIT-3: the D3 test misses the one case that moves the estimate row

`a_following_send_seals_one_route_follow_zeroed_row` filters out `estimate` rows, but none of its three
cases changes that row:

- `(true, false, false)` and `(false, true, false)` do not silence the send;
- `(true, true, true)` is already muted.

The case where only the follow silences the send, `(true, true, false)`, is not exercised. Adding it
would put the row under test in the shape where the plan's activity table also changes.

## Adversarial checks

### D1: the field and the output-route rule

- **Session.** `Route.follows_mute` is required, written after `mute`, and is key field 7 with walk
  count `[7]`.
  - `validate_routes` refuses `true` on an `OutputInput` route with `schema.invalid_enum` at
    `$.routes[<i>].follows_mute` and the frozen message.
  - The refusal is a validation, not a parse. The parser accepts any boolean.
  - The `InvalidEnum` doc comment now covers a value that is illegal in its context.
- **Protocol.** Route field 7 is `Wire::Bool` and required. `set_route_follows_mute` has `ROUTE_ID` 1
  and `FOLLOWS_MUTE` 2. `0x0507` has its enum, `from_raw`, variant, opcode map, apply arm, encode and
  decode.
  - The `0507` refusal runs on the final candidate. The console-session test commits "follow, then
    re-point into `bus`" and refuses "follow" alone, and the revision, model and snapshot do not move.
  - Opcode `0x0507` first appears in this commit (`git log -S`). Retired codes stay `None`, and route
    field 7 was never allocated before. Nothing was reused.
- **SDK.** An omitted `followsMute` writes `true` into a submix and `false` into the output.
  - `followsMute: true` into the output throws `MisoUsageError` with `schema.invalid_enum` at the
    route's path, and a non-boolean throws.
  - The writer emits the key after `mute`, enginectl admits it, and the `console-evals` rebuild helper
    carries `mute` and `followsMute`.
- **JSON Schema.** I checked it with `jsonschema` 4.10.3 (Draft 2020-12):
  - an output route with `follows_mute: true` is invalid;
  - a missing key is invalid;
  - an integer value is invalid;
  - a submix route with `true` is valid;
  - all 13 route-bearing `fixtures/session/v1` documents are valid.

  The `if` is vacuous when `destination` is absent, which is harmless because `destination` is
  required.

### D2: lowering

- **The source strip, for tracks and submixes.** The lowering maps `strips()` (tracks, then
  submixes) to `[left_mute, right_mute]` and looks up the route's *source* strip.
  - The gate is `{ mute, follow_zeroed }`, and the domain check calls `route_coefficients` with the
    same `follow_zeroed`.
  - The runtime binds `gated_route_coefficients(transform, gate)`, the same pure function a live
    producer reaches through `route_coefficients`. Zeroing is per column, never through the gain.
- **Fold.** `plain_route_gains` declines any non-`OPEN` gate, a partial follow included. A send that
  follows an unmuted source keeps the `OPEN` gate and may still fold.
- **Delayed routes.** The #1217 rule is keyed on `RouteGate::silences()`, so a send with both lanes
  followed is inactive when undelayed and mixed with `[+0.0; 4]` when delayed. Both cases are tested.
- **Bus to bus.** Gate 4 covers `s` to `b`. Gate 5 draws `bus -> aux` sends with random submix mutes.
- **Estimate.** The route-activity charge reads `gate.silences()` in the one estimator.

### D3: graph text

One `route-follow-zeroed\t<node>\t<l>\t<r>` row is written after `route-transform` and `route-mute`.
A route with no zeroed lane writes no row. Nothing parses the canonical text.

### Solo at preparation

A session never stores solo (`SKILL.md`: "Solo is live monitoring state and never appears in a session
document"). Preparation has no solo, and #1211's solo state seeds `user_mute` from "the compiled
session's baked fader mutes" (`solo.rs`). So at preparation:

`effective_mute == user_mute == session mute == the prepared follow_zeroed input`

That is consistent. The live composition is #1224 D1: the effective mute, solo-derived for tracks,
with submixes solo-safe. The spec states this, and so does DESIGN P11. This slice has no gap.

### D6

#1053 has not landed: `origin/main` holds only its docs commit `bdafbb437`, and no `live_builtin_delta`
exists. Every C ABI commit is a structural replacement, so a `020f` mute recompiles `follow_zeroed`.
#1053's spec carries the P13 rule for follow sources (`1053-...md:147-158`).

### Migration

- My scan of every `.json` file in the tree found **313 route objects**. Every one carries
  `follows_mute` immediately after `mute`, with no missing key and no misordering.
- No inline route in `.rs`, `.ts`, `.mjs`, `.js` or `.py` lacks the key. The JS evals clone a migrated
  template, and the host-web harness routes clone a template.
- This includes the K1 and K2 qualification sessions, the skill's worked session, the writer corpus,
  the `reduced-nobus` data and the 64-track console fixtures.

### Re-pins, recomputed

| Pin | Recomputed value |
| --- | --- |
| `canonical.json` | sha256 `d946f463...` |
| `prepare_256_tracks-48000.toml` | `372831d2...` |
| `prepare_256_tracks-96000.toml` | `d35b894e...` |
| `MANIFEST.tsv` | `867d9004...` |
| `browser-v1/session.json` | 1,955 bytes (1,926 + 29) |
| capi vector | 14,179 = 13,918 + 9 x 29 (`0x3763`) |
| `COMPLETE_SCHEMA_HASH` | moved at its four sites; the old value survives only in history prose |

- The graph PCM and meter digests are unchanged.
- `fixtures/graph` is unchanged: `graph_fixture --check` exits 0.

### No bit moved

- `fresh-process-determinism.json` is **byte-identical** to the parent's (`cmp`).
- `check-builtins-fixtures.sh` and `check-console-fixtures.sh` are ok on both trees.
- `audit capi` gives `pcm_digest ff6cdcb96cdcdad5` on both trees.
- **Independent gate 3** (`verifier_gate3_dump`): one session on both trees, every route
  `follows_mute: false`:
  - `t0` with both lanes muted, `t1` with its left lane muted, `t2` open behind a true-peak limiter;
  - a `pre_fader` send from each track into `verb`, whose right lane is muted;
  - `verb -> echo` from `input`.

  The 8,192-byte PCM is identical (`cmp`). 1,076 of 2,048 samples are nonzero, which is every frame
  after the 486-sample PDC.

## Gates (x86-64-v3 host, head `0da034dba`)

| Gate | Result |
| --- | --- |
| 8: `cargo build --release -p audit -p bench -p capi -p session-validator` | ok |
| 8: `check-graph-determinism.sh` | PASS 100/100; JSON identical to the parent's |
| 8: `graph_fixture -- --check` | exit 0 |
| 8: `check-console-fixtures.sh` | ok |
| 8: `check-builtins-fixtures.sh` | ok (50 files) |
| 8: `cargo test --release -p audit -p bench -p console-workload` | 110 passed, 0 failed |
| 10: `audit capi` | `pcm_digest ff6cdcb96cdcdad5`, `allocations 0`, `total_violations 0` |
| 9: test-debug-a (`--no-fail-fast`) | 1,192 passed, 0 failed, 9 ignored |
| 9: test-debug-b | 787 passed, 0 failed, 24 ignored |
| 9: `cargo fmt --check`; clippy `-D warnings` | ok; ok |
| 9: seven policy `check-*`/`test-*` pairs | all ok |
| 9: `check-protocol-wasm-parity.sh` | ok (simd128) |
| 9: `check-browser-expected-resources.py --self-test` and `--artifacts <A>` | ok; ok |
| 9: `check-sdk-types.sh` | ok |
| 9: `check-sdk-headless.sh <A>` | 352 of 352 |
| 9: `sdk-package.sh check <A>` | 17 of 17 |
| Extra: `build-web-audioworklet.sh --named-twin` | ok |
| Extra: **`check-web-audioworklet.sh`** | **FAIL, identical on the parent** (inherited #1217 BLOCKER-1); **PASS** with #1217 attempt 2's inline fix over this head |
| Extra: `check-scalar-oracle-absent.py` (named twin) | ok |
| Extra: browser qualification, `npm run qualify --check-matrix --self-test-mutations`, CI source mode, private pulseaudio sink | chromium 151, firefox 153 and webkit 26.5: "all qualification gates passed" |
| 10: `run-aarch64-tests.sh debug` | not runnable on this x86 host; CI `aarch64-debug` at the K3 push |
| D6 capi gates | do not apply (#1053 not landed) |

## Test value

Each new test, the defect it catches that no other test catches, and whether my mutations confirm it:

- **`invalid_matrix::route_follows_mute_is_required_and_legal_only_into_a_submix`** turns red if the
  key becomes optional or non-boolean, if an output route may follow, or if the writer drops a follow.
  - Confirmed by the implementer's mutations.
- **`console_session_edits::set_route_follows_mute_is_legal_only_on_a_route_into_a_submix`** turns red
  if `0507` can make an output route follow, or if the rule were checked per edit, which would refuse
  the re-pointing transaction.
  - The extended codec round trip is weaker than claimed: see MINOR-1.
- **Gate 5, extended `every_route_coefficient_comes_from_the_one_gated_function`** turns red if the
  compiler's `follow_zeroed` differs from the source strip's session mutes, so prepared and live
  coefficients would differ.
  - **M1** (lowering reads the *destination* strip's mutes): **RED** here and in the D3 test.
- **`a_following_send_seals_one_route_follow_zeroed_row`** turns red if a follow-zeroed plan shares a
  digest with an open one.
  - **M2** (row written only when both lanes are zeroed): **RED**.
- **Gates 1 and 4, `a_follow_send_zeroes_its_muted_source_lanes_column`** turns red if the follow
  zeroes the wrong column, goes through the gain, or reads mutes from tracks only.
  - **M1: RED.** It also guards against vacuity: the non-following session leaks.
- **Gate 2, `a_fully_follow_muted_undelayed_send_is_inactive`** turns red if a fully followed
  undelayed send is still mixed. Its counter stays 0, and the oracle is checked bit for bit.
  - **M1: RED.**
- **Gate 2, `a_fully_follow_muted_delayed_send_stays_active`** turns red if a fully followed delayed
  send is skipped.
  - **M1: RED.**
- **The SDK evals** (builder default, builder refusal, writer parity against `engineCanonical`, and
  the enginectl case) turn red on a wrong default, order, validation or forwarding.
  - Confirmed by the implementer's mutations.

My mutations:

| Mutation | Result |
| --- | --- |
| M1: lowering reads the destination strip's mutes | RED (2 graph-compiler tests, 3 host-core tests) |
| M2: canonical row only when both lanes are zeroed | RED |
| M10: decoder reads field 6 for `follows_mute` | GREEN, see MINOR-1 |
| M10b: encoder writes `mute` into field 7 | GREEN, see MINOR-1 |

Every mutation was reverted, and the mutation copy was checked clean against `0da034dba`.
