Verdict: PASS

# #1264 attempt 1 -- Sol adversarial verdict

Commit under review: `fbb311dfe` (parent `fe3bb37e8`), branch `codex/1053-live-updates`, reviewed
from an export (`git archive`), never in `wt-1053`. Held against the slice spec, umbrella #1053
(D1, D6-D9, D14, G1, G4), decision 14, `AGENTS.md` realtime rules and the acked-batch question.

No BLOCKER and no MAJOR finding. Two MINOR findings (documentation outside the authorized paths,
and one missing G1 regression case) and five NITs.

## Answers to the review questions

- **(a) Acked-batch.** Every refusal precedes the first push. In `commit_live`
  (`crates/capi/src/runtime/control.rs:984` on), the order is: classify, then live admission, then
  strip resolution, then effect resolution with the over-capacity hand-back (`:1039`), then the room
  check on strip and effect queues, then preflight of every effect record and the readback handle
  lookup (`:1074-1117`), then `check_prepared_structural`, then the `BeforeLivePush` fault point,
  then the pushes, then the commit. After the commit, `set_parameter_value` (`:1163-1166`;
  `crates/host-core/src/control_provider.rs:241`) does a binary search over `parameter_state`,
  which the catalog builder fills in strictly increasing handle order, and one store. It cannot
  fail or panic, and it allocates nothing. The `readback` vector was `try_reserve_exact`'d before
  the predicate, so the only post-commit memory event is its control-thread free.
  `provider_mut()` has no side effects (`protocol/src/controller.rs:2407`). The catalog is replaced
  only at a structural commit (`control.rs:949`), never at the plan swap. So a live edit that
  follows a pending candidate writes into the candidate's catalog. My scratch test confirmed that
  this readback still equals a rebuild's after the swap.
- **(b) Classifier vs render lane.** Values come from `resolve_initial_values`, the same function
  preparation now calls. It applies the unit check, channel policy, defaults, `normalize_zero`,
  `parameter_value_valid` and unknown IDs. The catalog's readback value is
  `bank_preparation.initial_values`, which is this same output. A `Shared` parameter yields one
  `Both` record and a `PerLane` one yields `Left` and `Right`. That is exactly the shape the effect
  validators accept, and the shape the browser's kind-5 lowering emits (`host-web/src/lib.rs`
  around `:4535`). Records carry plain values, not mapped ones, as the initial values do. No
  non-EQ launch effect has a `Block` parameter that is an enumeration or a boolean. The EQ's
  boolean `Block` parameters (`hpf-enabled`, `lpf-enabled`) are `Prepared` under G4, which a
  scratch test confirmed. Effect queue depth is `min(16, automation_capacity)`, and each lane's
  staging window is `automation_capacity`, so the render-side "staging full of distinct targets"
  drop (`effect-contract/src/live.rs`, `stage`) cannot fire. Preflight refuses only
  prepared-target owners, and the classifier keys on the same predicate
  (`factory.target_preparation().is_some()`, `live_delta.rs:373`; `prepare.rs:1408`).
- **(c) The rate rule.** `target_capable || !automatable || automation_rate != Block` gives
  `Prepared` (`live_delta.rs:380`). Every `None` parameter is a rebuild, and so is a `Sample` one,
  which the umbrella and decision 14 support. My scratch test checked the limiter lookahead, the
  multiband crossover, the gate attack (committed test), the EQ band gain (committed test) and the
  EQ HPF enable: all are `Prepared`. A live change beside a prepared one makes the whole delta
  `Prepared`. A prepared parameter rewritten at its own value stays live with no record.
- **(d) Over capacity.** A transaction whose records for one instance exceed `capacity()` hands
  the token back to the rebuild path. Gate 4 shows this, and I reproduced M7: with the check
  disabled, the 18-record transaction returns non-OK (red). The rebuild path's own pending-candidate
  `BACKPRESSURE` clears after one render, so it is never endless.
- **(e) Readback.** Gate 3 and my scratch test (a live edit while a candidate is pending, compared
  with the same edits made structurally, and again after the swap) show identical metadata and
  state records. I reproduced M6: skipping `set_parameter_value` turns both red.
- **(f) Addressing and submix strips.** Producers resolve by `(strip_id, address)` in the newest
  epoch. A `post_insert` slot is `console(pre_insert.len() + j)`, which agrees with
  `declared_live_addresses`. I reproduced M11 on the capi gates (2 and 3 go red; the test fixture's
  `pre_insert` slot is an EQ, so the misaddressed records hit its preflight and return `INTERNAL`).
  My scratch host-core test with a `pre_insert` transient-shaper also caught it. Submix strips:
  the mask and `effect_records` walk `tracks` only. My scratch tests confirm that a submix console
  entry's and a submix insert's parameter changes are `Structure` in host-core, and a rebuild
  through the C ABI (a new candidate, and the rendering plan's strip queues are untouched; a `Structure` delta emits no effect record).
- **(g) Bit-identical preparation.** The `prepare.rs` diff is a faithful extraction: same order,
  same codes, same early outs. There are no changes under `crates/effect-compiler/tests/` and no
  test-module hunks, and `cargo test -p effect-compiler --features test-support` passes. Browser
  module: I built it at base and at head. Base is `d4d023d5…` (2,857,637 B); head is `5045e3bb…`
  (2,857,420 B), which reproduces the implementer's digest exactly. The module moved by 217 bytes,
  as an effect-compiler code change should, and `check-browser-expected-resources.py --artifacts`
  passes on the head artifacts: PCM digests and exact rows unchanged, self-test green.
- **(h) Stale docs.** See MINOR-1.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **Stale live-edit contract text outside the authorized paths.** These two documents still say
   only fader, mute and pan values are live:
   - the header (`crates/capi/include/miso_engine_v1.h:34-41`), whose "...and every other field
     takes the replacement path" is now false for live effect parameters;
   - `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`, whose "a committed `SESSION_TRANSACTION_APPLY` whose
     delta is only track fader, mute and pan or matrix values is lowered...".

   The implementer correctly left both alone, since they are not authorized paths, and disclosed
   it. **Fix:** a follow-up (or the umbrella's doc slice) adds "and live effect parameters
   (automation rate `Block`, not the parametric EQ)" to both, and runs `check-capi-abi.sh` for the
   header.
2. **G1 for effects has no committed regression case.** No committed test shows that a submix
   strip's console-entry or insert `params` change is a rebuild. The fader case
   (`a_submix_fader_change_is_structural`) does not cover the new effect mask. A plausible
   regression would make submix effect edits live before #1267 with every committed test green:
   walking `strips()` in the params mask or in `effect_records`, as `declared_live_addresses`
   already does. **Fix:** add two assertions to `crates/host-core/tests/live_delta.rs`, a
   `Structure` result for a submix console-entry param and for a submix insert param. The shape is
   in `/home/bl/misofm/submix-verdicts/1264-attempt1-verifier-scratch.rs`
   (`v_submix_effect_params_are_structural`), and it passes on this commit.

### NIT

1. `crates/host-core/src/live_delta.rs:380` treats `AutomationRate::Sample` as `Prepared`. The
   slice spec D2 names only `None`; the umbrella D14 and decision 14 say "live when `Block`". This
   is correct and conservative, and the Attempt record says so, but the D2 text could be amended
   to say "not `Block`".
2. `effect_records` (`live_delta.rs:295-301`) lowers every track twice per transaction. That
   clones every console entry's `params`, even for a fader-only edit. **Fix:** skip the lowering
   when every console entry's and insert's `params` are bit-equal (`same_params`). This is
   control-thread cost only.
3. In `commit_live`, `parameter_handle` is a linear scan of the whole catalog per record, and the
   effect producer lookup is a linear `position` per instance. Read-only lookups go through
   `provider_mut()` (`control.rs:1074`). All of it is control-thread only and fine at launch
   sizes. Note it for the weekly pass if catalogs grow.
4. Gate 3 forces the comparison rebuild with `content_edit`, which edits the read source
   (`live_tests.rs:350`). The spec asked for an unread source. This is harmless, since the catalog
   never reads sources, but it differs from the brief's letter.
5. No committed test covers a sign-of-zero `params` rewrite. Mutation M12 (dropping
   `normalize_zero` on the `PerLane` path of `resolve_initial_values`) stays green on every
   committed effect-compiler and host-core test, and only my scratch `-0.0` makeup case turned
   red. Because preparation and the classifier share the function they cannot diverge, so this is
   a coverage nicety, not a defect: add `param(6, Both, Db, -0.0)` gives `Ok((vec![], vec![]))`
   to gate 1(b).

## Test value (one sentence per new test)

- `a_live_parameter_change_is_one_record_on_its_lane`: red if a live change gives anything other
  than one record on its lane, descriptor index and instance. Examples: a record for an unchanged
  lane (M3) or console-entry `params` left unmasked (M10).
- `a_both_value_split_into_lanes_records_only_the_changed_lane`: red if a value rewritten in
  another representation emits a redundant record (M3, M4).
- `a_removed_parameter_returns_to_its_default`: red if a removed entry does not return to the
  default a rebuild prepares (M4).
- `prepared_parameter_changes_need_a_rebuild`: red if a `None`-rate parameter or an EQ parameter
  goes live as an acked edit the effect never applies (M1, M2; I reproduced M1).
- `params_preparation_refuses_need_a_rebuild`: red if the classifier admits `params` that
  preparation refuses, leaving a committed model its own rebuild rejects. I ran M13 (the
  post-commit resolve error swallowed): red, and nothing else catches it.
- `an_insert_reorder_is_structural`: red if the params mask hides an instance change, such as a
  reorder or a bypass flip. I ran M14 (the mask copies the whole effect): red.
- `live_effect_parameter_edits_render_like_the_browsers_lane` (gate 2): red if the C ABI path
  addresses a different instance, lane or parameter index than the browser lane (M5, M11; I
  reproduced M11).
- `the_parameter_readback_after_a_live_edit_equals_a_rebuilds` (gate 3): red if the readback keeps
  the old value or maps lanes wrongly (M6, M9; I reproduced M6).
- `an_effect_edit_larger_than_its_queue_rebuilds` (gate 4): red if an over-capacity transaction
  returns `BACKPRESSURE` forever instead of rebuilding (M7; I reproduced it).
- `a_full_effect_lane_refuses_before_anything_changes`: red if the room check skips effect queues,
  so that an acked record would fail to push (M8).

No test greps source or prose. No digest or byte pin was added.

## Gates I re-ran (export of `fbb311dfe`, `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053`)

- `cargo fmt --all -- --check`: pass.
- `cargo test --locked -p effect-compiler --features test-support`: pass.
- `cargo test --locked -p host-core --features control-provider,test-support`: pass
  (`live_delta` 22).
- `cargo test --locked -p capi`: pass (54 lib, 13 `resource_lifecycle`, doc).
- `cargo clippy --locked -p effect-compiler -p host-core -p capi --all-targets --all-features -- -D warnings`:
  pass.
- `cargo build --locked --release -p audit -p capi`, then `audit capi`: allocations 0,
  deallocations 0, locks 0, syscalls 0, `total_violations` 0.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: pass.
- `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so`: pass.
- `scripts/check-{host-core,realtime,workspace}-policy.sh`: pass.
- `build-web-audioworklet.sh --module-only` at base and at head: the module moved, as reported
  under (g).
- `build-web-audioworklet.sh --named-twin` and `check-browser-expected-resources.py --artifacts`
  at head: pass.

Mutations I ran (each introduced, run, then reverted, with the export checked byte-equal to the
commit afterwards):

| Mutation | Result |
|---|---|
| M1 | red |
| M6 | red |
| M7 | red |
| M11 | red in capi gates 2 and 3, and in my scratch host-core test; green in the committed host-core tests, as the implementer disclosed |
| M12 | green on the committed tests (NIT-5) |
| M13 | red |
| M14 | red |

**Scratch tests.** All pass on the commit; saved in `1264-attempt1-verifier-scratch.rs`:

- submix effect params are `Structure` (host-core) and a rebuild (C ABI);
- a `pre_insert` offset, a `Shared` `Both` record, the limiter, multiband and EQ HPF cases all
  `Prepared`, and the `-0.0` no-record case;
- a live effect edit while a candidate is pending reaches the candidate, and the readback equals
  a rebuild's before and after the swap.

**Not re-run (lean build, as instructed):**

- the workspace test command;
- the protocol tests;
- `check-cross-targets.sh`;
- `check-web-audioworklet.sh` and `test-web-audioworklet.sh`;
- `cargo doc`.

For these, the implementer's logs in `/tmp/claude-1002/w1264-a1/` show a pass. The 4-lane
(NEON) run is CI-only.

**Scope.** All eight changed paths are authorized paths. `control_provider.rs` changes D4 only.
