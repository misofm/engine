Verdict: PASS

# #1263 attempt 1: Sol's adversarial verdict

Commit `986301820` (parent `31b53b62c`), reviewed from the export `/tmp/claude-1002/v1263-a1` and built with
`CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053` on x86-64-v3 (eight lanes). Logs and my scratch tests are in
`/tmp/claude-1002/v1263-a1-logs/`.

There are no BLOCKER or MAJOR findings. The code does what D1-D6 ask. Rendering, latency and tail are
unchanged, and render stays allocation- and syscall-free. Three MINORs are about accounting claims and
documentation, and they should be applied as follow-ups before #1264 lands.

## The caller's questions

### 1. Is the graph-row move a spec deviation that needs a rebrief or an owner ruling?

No rebrief is needed. The move is within scope. It should still go to the owner as an FYI, because of the
cost driver described below.

- **What this slice adds.** This slice adds no new mechanism. With `effects: true`, graph bind wraps every
  effect bank in the existing `rack::LiveControlEffectBankStage` (#1087/#1100) (`graph/src/runtime.rs:4678-4688`).
  The existing exact estimate charges it: `graph-compiler/src/estimate.rs:162-340`, `effect_control_resource`.
  That is the function the spec's own D3 cites, and its doc comment names the banked owner. So the brief
  under-read its own anchor. The product contract does not change.
- **AGENTS.md's split triggers do not apply.**
  - The slice has one product outcome.
  - It needs no new framework or fixture corpus.
  - It edits no file outside the authorized paths.
  - The budget raise is in `resource_lifecycle.rs`, which is authorized. It follows the owner's own budget
    mechanism (#1060, decision 4): a structural move raises the ceiling, with its reason, in the same
    commit.
- **The owner should still see the cost.** On the C ABI the dominant term scales with the caller's
  `maximum_automation_spans_per_block` (S).
  - The per-lane packed window and the one-lane staging window are both S spans of 40 bytes. The pairing
    rule of #1012 requires the window to equal the automation capacity, and the capacity is S
    (`effect-contract/src/lib.rs:2535`).
  - But the C ABI's lane depth is 16, so one drain can stage at most 16 spans per lane.
  - The browser derives S from its queue depth and its stored automation (`host-web/src/lib.rs:6573-6582`),
    so it pays about 16 spans per lane. The C ABI pays S.
  - From the struct sizes I measured (span 40, `Option<EffectControlLane>` 72, stage growth 176), for a
    zero-latency effect at a 128-frame quantum:

    | unit | bytes | at S = 128 | at S = 4,096 |
    |---|---|---|---|
    | eight-lane bank | 8,944 + 360·S | 55,024 (matches the attempt) | about 1.48 MB |
    | four-lane bank | 4,560 + 200·S | 30,160 | about 824 KB |

  - The question for root and the owner: should the C ABI bound its live windows by the lane depth? On the
    reference session that would cut the move from 128,984 to about 48 KB. This is not a fix this slice owes.

### 2. Are the derived four-lane ceilings sound upper bounds?

Yes. AArch64 CI should pass.

- **The four-lane move.** The closed form gives 30,160 bytes per four-lane bank. Nine console-slot EQs
  always bank, so four lanes make three banks (4 + 4 + a padded 1). The per-member term is width-free: the
  ring header is `align(64)` and fixed, and the EQ's target staging is 16 × 56. The four-lane move is
  therefore 9 × 2,104 + 3 × 30,160 = **109,416**, below the eight-lane 128,984 the attempt added.
  - The attempt's argument holds. The per-bank constant, 5,296 bytes, is below four per-lane terms of
    6,216 bytes each, so three four-lane banks cost less than two eight-lane banks.
- **The worst case.** Suppose the four-lane rows sat exactly at their old ceilings (253,952 and 62,528).
  They would rise to at most 363,368 and 171,944. Both are under the new ceilings of 413,952 and 204,544.
- **Assumptions.** All are LP64-identical: the struct sizes, the `align(64)` ring header, EQ latency 0, and
  capacity = S = 128.

### 3. Is the raise a ceiling with a documented reason, and not an exact pin?

Yes.

- **Eight lanes.** The eight-lane ceilings are the measured values plus 10 %, rounded up to 64. I checked
  both: 382,918 × 1.1 rounds up to 421,248, and 185,121 × 1.1 rounds up to 203,648. The printed rows
  match: graph 382,918 of 421,248, metadata 185,121 of 203,648, capi 273,640 of 282,432.
- **Four lanes.** The four-lane ceilings are looser than the 10 % rule (NIT 2).

### 4. Does the browser already pay this cost?

Yes, whenever live controls are on.

- `live_control_command_queue_records != 0` prepares with `HostLiveLanes::ALL`, which includes effects
  (`host-core/src/prepare.rs:688-745` for the spectrum entries, `:806-860` for the meters-only entry). That
  runs the same bind, the same `LiveControlEffectBankStage` and the same estimate.
- A fan's control-free playback pays nothing, and the browser's S is small (see question 1).
- So the C ABI now matches the browser's live-control shape. Two things differ:
  - The C ABI has no opt-out.
  - The C ABI's cost per bank follows the caller's S.

## Other checks the caller asked for

- **Bit identity.** I checked the mechanism.
  - A fresh channel lane seeds the same witness as a channel-less lane (`SYMMETRIC` with
    `UNBYPASSED = !bypass`, `effect-contract/src/live.rs:160-206`), so mono collapse and the kernel path
    do not change.
  - An empty drain stages no spans and no targets. The bank gets `packed[..0]`, just as `EffectBankStage`
    passes no automation.
  - Lanes do not enter `EffectProgramKey`, so banking does not change.
  - Each lane is seeded from `entry.initial_bypass`, which is the session bypass
    (`effect-compiler/src/prepare.rs:1526-1530`). The delay and the multiband keep their prepared bypass.
  - Evidence:
    - The committed gate 1 is green.
    - My scratch test is green at the four rates (`scratch-tests-rs.txt`). It puts bypassed and enabled
      delay, multiband and compressor inserts in mixed cohorts on the 10-track session.
    - `audit capi` gives `pcm_digest` `18e56b897a3abf17`. That equals #1258's recorded post-change value,
      over 100,000 render calls of the nine-track EQ fixture with live fader and pan edits. This is a
      one-time comparison, not a pin.
- **Mutation M5, re-run.** The non-target lane is seeded `false`. The committed test goes red at
  "parity 10 tracks at 44100 Hz: block 3", and so does my scratch test. Reverted, and the file is
  byte-identical to the commit.
- **Latency, `tail_kind`, `tail_samples`.** The test asserts each one against the lanes-free plan and
  against the C ABI selection without effect lanes. Both checks are green.
- **Render audit.** `audit capi` reports zero allocations, deallocations, locks, syscalls and violations.
- **Drop order.**
  - `ProviderEpoch` lives only in the control-plane session.
  - `synchronize_plan_epochs` drops a reclaimed plan before its provider (`control.rs:719-734`).
  - A cancelled candidate and session destroy both drop on the control thread.
  - The rings are shared `Arc`s, so whichever side drops last frees them, never the render thread.
  - The EQ owner holds only control-side state and the factory `Arc`.
- **`#[cfg_attr(not(test), allow(dead_code))]`** (`control.rs:37`). It is correct and has precedent
  (`builtins-compiler/src/lib.rs:5041`). `expect` is better (NIT 1).
- **Stale `FADER_AND_MATRIX` docs.** See MINOR 3. The implementer was right not to touch them.
- **The acked-batch question.** It does not apply here. Nothing pushes, and no admission or ack path
  changed.
- **Authorized paths.** All seven files are authorized. The `effect-compiler` change is the D5 comment
  only. No code compiled into the browser module changed, so skipping the worklet chain is justified.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

**MINOR 1. The effect-control payload is admitted against two caps, but the slice says it is charged once.**

- **Where.**
  - `crates/host-core/src/prepare.rs:1257-1268`: host-core's own preparation admission adds
    `effect_control_resources.total_bytes()` to graph + model against `maximum_graph_session_plus_plan_bytes`.
  - `crates/capi/src/runtime/compile.rs:171-187`: capi charges the same bytes in `capi_retained_bytes`.
  - D3 ("Every byte is charged once").
  - `docs/C_ABI_V1_QUALIFICATION.md:283-287`: "charged once, in `capi_retained_bytes`".
- **Evidence.** My scratch test is `scratch-resource-lifecycle-rs.txt`, with its log in
  `scratch-exact-graph-cap.log`. The nine-track fixture compiled at
  `graph cap = report.graph_session_plus_plan_bytes + compiled model` (402,352) is **refused**. At +15,360
  it is refused. At +15,361 it is admitted. 15,361 is exactly the effect-control total.
  - Before #1263 that payload was 0 on the C ABI, so graph row + model was the exact compile-time
    requirement.
  - #1060's budget test presumes that ("the row the budget bounds is the row admission enforces"). It only
    tests one byte below the requirement, so it cannot see this.
- **Impact.** The behaviour is conservative, and only the initial compile is affected: a replacement's
  two-plan peak and the live peak dominate. But no reported row now gives a caller its graph-cap
  requirement. The doc's "a caller whose ... was exact must raise it" points a caller at a value that is
  still refused.
- **Fix.** The minimum stays within the authorized paths. Record it as a conservative double admission, as
  #1256 D4 does for the strip vector:
  - one clause at the charge site;
  - one sentence in the doc's Charges bullet: at preparation host-core also admits this payload against
    the graph cap, so the initial compile needs graph row + compiled model + the payload capi charges;
  - a line in the attempt record.
  - Optionally, pin the boundary in the oracle.
  - The alternative is a spec amendment that lets host-core leave caller-charged effect controls out of
    its graph admission.

**MINOR 2. The qualification doc gives the fixture's figures as if they were general, and omits the cost
driver.**

- **Where.** `docs/C_ABI_V1_QUALIFICATION.md:297-300`, "2,104 bytes per effect instance plus 55,024 per
  eight-lane effect bank".
- **What is wrong.**
  - 2,104 is an EQ member. A non-EQ member is 1,208, because it has no 896-byte target staging.
  - 55,024 holds only at S = 128, a 128-frame quantum and zero latency. A latency L adds 64·L at eight
    lanes.
  - A four-lane bank is 30,160.
  - The bank term is linear in the caller's `maximum_automation_spans_per_block`: 8,944 + 360·S at eight
    lanes and 4,560 + 200·S at four. At S = 4,096 a four-lane bank costs about 824 KB.
- **Why it matters.** Mobile callers size `maximum_graph_session_plus_plan_bytes` from this document.
- **Fix.** Label the numbers as the reference session's at `limits()`, and state the dependence on S, the
  quantum, the latency and the lane width. A formula is better. Root may also raise question 1's window
  question with the owner.

**MINOR 3. Docs outside the authorized paths now state false facts.**

- **Where.**
  - `crates/host-core/src/prepare.rs:374`: `FADER_AND_MATRIX` is "the C ABI's selection, #1256".
  - `crates/host-core/src/prepare.rs:443-444`: "C ABI plans prepare with `FADER_AND_MATRIX`".
  - `crates/effect-contract/src/live.rs:112`: "A producer must be dropped before the plan that owns this".
    This is D5's twin. The rings are shared `Arc`s, and capi drops the plan first.
- **Fix.** The implementer was right not to touch them. Root should authorize a doc-only amendment of
  those three lines, in a follow-up or in #1264.

### NIT

1. **`allow(dead_code)`** (`control.rs:37`). Use
   `#[cfg_attr(not(test), expect(dead_code, reason = "#1264 pushes through these"))]`, as #1256's verdict
   NIT 2 advised. Then it fails once #1264 reads the field.
2. **The four-lane ceilings are looser than 10 %.** See `resource_lifecycle.rs:988-995` and `:1001`,
   `:1007`, `:1013`.
   - The exact four-lane move is 109,416, not 128,984, so the ceilings sit about 16-23 % above the
     probable four-lane values.
   - Either use 109,416 in the derivation, or tighten the ceilings from the rows the budget test prints
     on the first AArch64 run.
3. **Gate 1's sessions have no enabled non-EQ effect and no prepared-bypass effect.**
   - I ran mutation M6, which seeds every non-target lane `true`. It survives every capi test. It is
     caught upstream by `effect-compiler`'s `a_live_control_lane_starts_from_the_session_bypass` and by
     host-core's `limiter_linked_session`, `effect_observation` and `live_routes` tests.
   - Fix: add an enabled compressor insert and a bypassed delay and multiband to the 10-track session. My
     scratch test does exactly that. It is green on head and red under M5 and M6.
4. **`largest` takes the slice `epoch_rows[..len - 1]`** (`compile.rs:235`). That assumes the effect row
   is last. Name the rows, or chain them explicitly.

## Test value

- **`c_abi_plans_with_live_lanes_render_like_lanes_free_plans` (extended).** It turns red if attaching the
  C ABI's effect lanes changes a rendered bit, latency or the tail, for example a lane seeded off the
  session bypass (M5, re-run red). It also turns red if capi loses an effect producer or an EQ owner (M1,
  M4). Before this slice no capi test ran a C ABI plan that carried effect lanes.
- **`capi_retained_bytes_charge_every_byte_the_compile_retains` (updated oracle).** With
  `tiny_control_frame_still_accounts_three_provider_counters_exactly` and
  `double_live_oracle_drives_exact_and_one_below_c_caps`, it turns red if the effect producer table or its
  owned payload (IDs, EQ owners) is retained by capi but left out of the reported rows, or charged there
  twice. M2, re-run: the effect row charged as 0 turns all three red. Reverted, and the file is
  byte-identical to the commit.
- **`reference_session_retained_rows_stay_within_their_budgets` (ceilings raised).** It is not a new test.
  It still stops any later structural move over 10 % at eight lanes.

## Gates I re-ran

| gate | result |
|---|---|
| `cargo test --locked -p capi` | pass: 50 lib, 13 `resource_lifecycle`, doc |
| `cargo test --locked -p effect-compiler -p host-core` | pass |
| `cargo build --locked --release -p audit -p capi`, then `audit capi` | zero allocations, deallocations, locks, syscalls and `total_violations`; `pcm_digest` `18e56b897a3abf17` |
| `bash scripts/check-capi-abi.sh` | ok, shared and static linkage |
| `bash scripts/check-capi-abi.sh --self-test` | ok |
| `cargo fmt --all -- --check` | committed code clean; the only diff was my scratch test |
| budget rows printed | values as quoted above |

Mutations, each reverted and restoration verified with `cmp`:

| mutation | result |
|---|---|
| M5 | red |
| M2 | red, in three oracle tests |
| M6 (mine) | survives capi, red upstream |

Not re-run:
- workspace clippy, policy and cross-target scripts;
- the worklet chain (no browser-module code changed);
- AArch64 (CI only).
