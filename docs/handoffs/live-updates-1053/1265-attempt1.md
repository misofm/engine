Verdict: PASS

# #1265 attempt 1 -- Sol adversarial verdict

Commit under review: `b055a48d4` (parent `fbb311dfe`), branch `codex/1053-live-updates`. I
reviewed it from an export (`git archive` into `/tmp/claude-1002/v1265-a1`) and never worked in
`wt-1053`. I held it against:

- the slice spec, D1-D4 and gates 1-6;
- umbrella #1053: D6-D9, D14 and G4;
- decision 14 (`docs/rulings/live-update-versus-rebuild-2026-10-04.md`);
- the realtime rules in `AGENTS.md`;
- the acked-batch question.

There is no BLOCKER and no MAJOR finding. There are two MINOR findings, both test or evidence
gaps; the implementation is correct as measured. There are five NITs.

Only the six authorized paths changed: the spec, `crates/host-core/src/live_delta.rs`,
`crates/host-core/tests/live_delta.rs`, `crates/capi/src/runtime/control.rs`,
`crates/capi/src/runtime/live_tests.rs` and `docs/C_ABI_V1_QUALIFICATION.md`. No browser-module
code changed: `live_delta` is compiled only under host-core's `control-provider` feature, which
only capi enables (`crates/host-core/src/lib.rs:96-97`, `crates/capi/Cargo.toml:19`). So
skipping the worklet chain is justified.

## Answers to the review questions

### (a) The acked-batch question with EQ owners

`commit_live` (`crates/capi/src/runtime/control.rs:1034` on) runs in this order:

1. Classify, then the live admission.
2. Resolve the strips.
3. Resolve the effects, with the capacity hand-back (`:1094-1109`). It runs before any
   `begin_owner`, so a rebuild hand-back never leaves an owner begun.
4. The owner transactions (`:1111-1151`), guarded by `OpenOwners` (`:410-422`).
5. The strip room checks and the non-EQ room checks.
6. The non-EQ preflights and every readback handle.
7. `check_prepared_structural`, then the `BeforeLivePush` fault point.
8. The pushes, then `publish_candidate_targets`, the protocol commit and `commit_owner`.

**Nothing after the first push can fail.**

- The strip and non-EQ pushes had their room checked, and the control thread is their only
  producer.
- `publish` re-runs the same deterministic `preflight_publication` on the same open candidate
  (`crates/effect-compiler/src/control.rs:382-402`). The EQ queue's room only grows between the
  two calls.
- The protocol commit's predicate was checked under the same `&mut` borrow.
- `commit_owner` requires `Published`, which `publish` just set.

Each of these calls is `expect`ed, as D2.4 asks.

**No owner is left half-committed.** An owner goes Open, then Published, then Idle inside one
control-thread call. Only a panic at an `expect` above, which is unreachable, could stop it at
`Published`. `published = take(begun)` runs only after every publish, so the guard's `Drop` never
discards a published owner.

**Render does not allocate.** The render side copies targets into staging sized to the queue's
capacity (`crates/effect-contract/src/live.rs:186-211`, `:384-395`) and applies them with
arithmetic. My probe (below) measured 0 allocations and 0 frees on the render thread.

**"Internal" for a non-capacity owner refusal cannot be reached from valid input.**

- D1 already refuses out-of-domain values and values that are not automatable or not `Block`.
- The base revision is the owner's own `committed_revision()`.
- `begin` refuses only a phase other than `Idle`, which no path leaves behind, or a revision
  overflow.
- The only remaining non-capacity refusal is a semantic-word mismatch in
  `validate_prepared_targets` (`crates/parametric-eq/src/control.rs:396-456`). That means seed
  drift, which D9 rules out (see (b)).

I forced that refusal by seeding the designer with the defaults (the implementer's M1). An
ignored probe, `sol_probe_internal_changes_nothing`, then saw these results:

- the transaction, an EQ edit beside a fader edit, returned `INTERNAL`;
- the model bytes, revision, replay length and reliable-lane report did not change;
- every owner's revision, phase, committed rows and candidate rows were unchanged;
- the EQ lane's room and the fader lane's room were unchanged;
- the owner was `(0, Idle)`.

So an Internal after a begun owner changes nothing.

**Gate 3: a refusal leaves the owner idle and the render plane untouched.** The committed test
shows this for a fader-room refusal and for the EQ's own `Capacity` refusal. My probe
`sol_probe_two_owners_second_refuses` shows it for two begun owners where the second one refuses.
`sol_probe_fault_before_push_with_eq` shows it for the fault before the push. See MINOR-1 for the
coverage gap.

### (b) Designing the targets: the seed is the running owner's state

The seeds are `resolve_initial_values(descriptor, &before.params)` on the committed model
(`crates/host-core/src/live_delta.rs:393-418`). That is the same function preparation used, with
the same `normalize_zero`. The design rate is `next.sample_rate_hz`. The mask forbids a rate
change, and the plan is prepared at `compiled.sample_rate()` (`crates/capi/src/runtime/compile.rs:594`).

The owner validates each touched section's semantic words bit for bit against its own candidate.
So seed drift is never rendered silently: it is refused as above.

The coefficient words are only range-checked. A design at the wrong rate passes the owner
silently, and gate 2 is the only guard against it. I re-ran M3 (design always at 48 kHz): gate 2
went red at the bit-exact assert, and it was green again after the revert.

I ran `sol_probe_eq_edit_while_candidate_pending`. Its steps:

1. A live EQ edit, then a structural edit, so a candidate is pending.
2. Two EQ edits while the candidate is pending, with no render between them. The second also
   enables the insert's HPF.
3. The candidate's owner ends at `(2, Idle)`, and the current plan's owner stays at `(1, Idle)`:
   D7, the newest epoch.
4. The swap, then an EQ edit after it (`(3, Idle)`), then a Q edit after a render (`(4, Idle)`).
5. The owner's committed rows equal, bit for bit, the rows that a fresh preparation of the
   committed snapshot gives its owner. This is the D9 invariant.

Gate 3's sixteen consecutive EQ edits without a render cover consecutive seeding as well.

### (c) Gate 2's reference is independent of the code under test

The reference is a host-core `HostLiveLanes::ALL` plan. It takes its seeds from its own owner's
committed rows. The browser's companion reads the same rows through `copy_eq_target_config` and
also designs with `EqTargetPreparer`. The reference uses hand-written edits and calls
`EqTargetPreparer::prepare` directly.

Neither the classifier nor capi is involved, so the reference is not circular. The C ABI side
seeds from the model, and the reference seeds from the owner. M3 and M1 both turn the gate red.

### (d) Band `enabled` and `kind` stay prepared; the HPF and LPF `enabled` are live

This is consistent with the spec and decision 14:

- The HPF and LPF `enabled` descriptors are `Block` and automatable
  (`crates/parametric-eq/src/lib.rs:531-561`).
- The decision 14 EQ row lists them as live.
- `apply_target_lane` exempts the HPF and LPF sections from the `enabled`/`kind` check
  (`lib.rs:2816-2830`).
- A band's `enabled` and `kind` are `AutomationRate::None` and stay `Prepared` (M2 turns the
  rewritten test red).

The tail and latency do not change: every EQ quality declares `latency: 0` and
`tail: TailSamples::Infinite` (`lib.rs:648-655`), so an enable moves neither the reported tail nor
the latency.

### (e) A capacity overflow rebuilds and never returns an endless BACKPRESSURE

- The check counts an EQ's targets, not its records (`control.rs:1097-1101`).
- I re-ran M5 (count the records instead): gate 4 went red, and it was green after the revert.
- In gate 4, five targets against a capacity of 4 give a pending candidate, and the rendering
  plan's owners stay untouched.
- A full lane whose targets would fit after a render is `BACKPRESSURE` until that render. This is
  the paused-host behaviour of #1053 D4, and gate 3 shows the retry succeeding.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

- **MINOR-1. No committed test covers a refusal with more than one begun owner.**
  - **Where:** `crates/capi/src/runtime/live_tests.rs:2141`, `a_refused_eq_transaction_leaves_its_owner_idle`.
  - **The gap:** the spec's hazard says "the rollback must cover every refusal path". Gate 3
    exercises only one owner. I mutated `OpenOwners::drop` (`control.rs:415-422`) to discard only
    the last begun owner. All 58 committed capi lib tests stayed green. Only my probe
    `sol_probe_two_owners_second_refuses` went red: eq0's owner was begun and preflighted, then
    eq1's EQ lane was full.
  - **Fix:** add that case to gate 3: an EQ edit on eq0 and on eq1 while eq1's EQ lane is full.
    Assert `BACKPRESSURE`, assert that every owner, the model and both rooms are unchanged, and
    assert that the retry after a render commits.
  - **Optional:** a live EQ edit while a candidate is pending. My probe
    `sol_probe_eq_edit_while_candidate_pending` passes.
- **MINOR-2. The C ABI's realtime gate does not exercise the render path this slice makes
  reachable.**
  - **The gap:** the live editor of `audit capi` cycles only a mute and a pan (#1258;
    `tools/audit/src/capi.rs:181-208`). Its "0 allocations, 0 syscalls" therefore says nothing
    about draining and applying EQ targets on a C ABI plan.
  - **My measurement:** I closed the allocation half by measuring it myself. A test-only counting
    global allocator was armed on the render thread around `miso_engine_v1_render_f32_planar`
    only, with a positive control of 2 counted events. Twelve blocks each applied targets for
    three EQ instances: a console gain, a console Q, and the insert's HPF frequency, with the HPF
    enabled on the first block. Each block drained the lanes completely, and the total was 0
    allocations and frees.
  - **Not covered:** locks and syscalls were not measured by the probe. That render code is
    unchanged and shared with the browser.
  - **Fix:** this is outside the authorized paths, so it belongs in a follow-up. Add an
    effect-parameter edit and an EQ edit to the cadence of `tools/audit capi`'s live editor.

### NIT

- **NIT-1. Gate 2's eq4 edit is inaudible.** Its band-1 Q edit (`live_tests.rs:2051`, `:2087`)
  lands on a disabled band: the fixture's eq4 has no params, and band `enabled` defaults to 0. So
  the PCM comparison cannot see where eq4's targets went. eq0's edit and the insert's HPF edit
  carry the gate's addressing claim.
  - **Fix:** enable band 1 on one more console EQ in `eq_session`, or edit an enabled band.
- **NIT-2. A docstring overclaims.** The docstring of
  `an_eq_band_gain_change_carries_its_edits_and_designed_targets`
  (`crates/host-core/tests/live_delta.rs:1242`) says the test turns red if the classifier "designs
  at another rate". At the fixture's 48 kHz, a classifier hardcoded to 48 kHz stays green; M3 is
  caught only by gate 2. The attempt record says this correctly.
  - **Fix:** reword the docstring, or run the case at 44.1 kHz.
- **NIT-3. The new Q test mostly repeats an existing one, and one D1 mapping is untested.**
  `an_out_of_domain_eq_q_needs_a_rebuild` (`:1304`) largely overlaps #1264's
  `params_preparation_refuses_need_a_rebuild`, because `resolve_initial_values` refuses first. Its
  unique catch is an EQ branch that skips that resolution.
  - D1's "a design refusal gives `Domain`" mapping (`live_delta.rs:458-462`) is untested: I mapped
    it to `Prepared`, and all 25 `live_delta` tests stayed green.
  - This is harmless, because capi rebuilds on any `Err`. The test is the spec's own gate 1(d), so
    it stays.
- **NIT-4. Two added lines run past the wrap width.** `crates/host-core/src/live_delta.rs:189` is
  124 characters and `docs/C_ABI_V1_QUALIFICATION.md:279` is 114. Rewrap them.
- **NIT-5. Bookkeeping outside the authorized paths, for root.**
  - Mark umbrella G4 as lifted by #1265, as the umbrella's guard section asks the lifting slice to
    do.
  - Two items carried from #1264 stay stale: the C header's "Live edits" comment and
    `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`.
  - `LiveEffectRecords` is still reachable only as `host_core::live_delta::LiveEffectRecords`.

## Test value, one sentence per new or rewritten test

- **`an_eq_band_gain_change_carries_its_edits_and_designed_targets`:** it turns red if the
  classifier seeds `EqTargetPreparer` with anything other than the rows preparation gave the
  owner (for example the defaults, M1), or passes edits other than the changed rows.
- **`an_eq_hpf_enable_change_carries_targets`:** it turns red if a cut filter's on/off row is kept
  prepared, as band `enabled` is, or is emitted without targets.
- **`prepared_parameter_changes_need_a_rebuild` (rewritten):** it turns red if a band's `enabled`
  or `kind` rides live beside a live gain (M2).
- **`an_out_of_domain_eq_q_needs_a_rebuild`:** it turns red if the EQ branch admits a Q that
  preparation refuses because it skips `resolve_initial_values` (thin; NIT-3).
- **`live_eq_parameter_edits_render_like_the_browsers_lane`:** it turns red if the C ABI designs at
  another rate (M3, which the owner's validation does not catch), from other seeds (M1), or to
  another owner than the browser's lane would. It runs at all four launch rates.
- **`a_refused_eq_transaction_leaves_its_owner_idle`:** it turns red if a refused transaction leaves
  an EQ owner begun (M4), maps the owner's `Capacity` to `INTERNAL` (M6), or skips `commit_owner`
  (M8).
- **`an_eq_edit_designing_more_targets_than_its_queue_rebuilds`:** it turns red if capi counts an
  EQ's records instead of its targets against the capacity (M5), which gives a wrong rebuild or an
  endless `BACKPRESSURE`.
- **`the_eq_parameter_readback_after_a_live_edit_equals_a_rebuilds`:** it turns red if the readback
  keeps an EQ value's old state after a live edit (M7).

## Mutations I re-ran (each red, then green after the revert)

| Mutation | What turned red |
|---|---|
| M1: the seeds are the defaults | Gates 2, 3, 4 and 5 (`INTERNAL`). My INTERNAL probe passed. |
| M3: design always at 48 kHz | Gate 2 only. |
| M4: `OpenOwners` discards nothing | Gate 3, and my two-owner and fault probes. |
| M5: count records instead of targets | Gate 4. |
| Own: discard only the last begun owner | My probe only; the committed suite stayed green (MINOR-1). |
| Own: a design refusal maps to `Prepared` | Nothing; all 25 `live_delta` tests stayed green (NIT-3). |

## Gates I re-ran (export of `b055a48d4`, `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053`)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test --locked -p host-core --features control-provider,test-support` | pass: 28 suites, `live_delta` 25 |
| `cargo test --locked -p capi` | pass: 58 lib, 13 `resource_lifecycle`, doc |
| `cargo clippy --locked -p capi -p host-core --all-targets --all-features -- -D warnings` | pass |
| `cargo build --locked --release -p audit -p capi`, then `audit capi` | allocations 0, deallocations 0, locks 0, syscalls 0, `total_violations` 0, `render_errors` 0 |
| `bash scripts/check-capi-abi.sh` | ok, shared and static |
| `bash scripts/check-capi-abi.sh --self-test` | ok |
| `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so` | pass |
| Probes (removed afterwards) | all pass: pending candidate and consecutive edits with D9 equality; two owners, the second refusing; fault before the push; INTERNAL under M1; zero render-thread allocations while applying EQ targets |

Not run, by the lean brief: the workspace test command, workspace clippy and doc, the policy
scripts and `check-cross-targets.sh`. The implementer reports them passing. AArch64 is CI-only.

Logs: `/tmp/claude-1002/v1265-a1-logs/`.
