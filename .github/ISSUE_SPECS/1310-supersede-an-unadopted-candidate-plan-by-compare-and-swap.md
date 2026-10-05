# Supersede an unadopted candidate plan by compare-and-swap

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host submits a structural transaction while an earlier one's candidate plan has not been
adopted by render yet, including on a paused host, and it commits. The control plane withdraws
the unadopted candidate, prepares one successor that carries both transactions, and publishes it.
Repeated structural edits on a paused host all commit; none returns `BACKPRESSURE` because a
candidate is pending.

## Context

- **The refusal.** `command` refuses every structural transaction while a candidate is pending:
  `if !self.pending_providers.is_empty() { return Err(CommandError::Backpressure) }`
  (`crates/capi/src/runtime/control.rs:959-960`). A paused host never adopts, so every later
  structural edit is refused for as long as it stays paused.
- **Its relatives.** Three more refusals come from candidate bookkeeping, all as `Backpressure`:
  the #1042 lag check (`control.rs:897`); a full publication or retirement queue
  (`PublicationFull`, `RetirementFull`, `:961-974`); a full report table or pending list
  (`:992-996`). Their capacities: exchange publication 1 and retirement 1 (`compile.rs:170-171`,
  `:758-759`); report rows 2 (`:765`); pending providers 1 and retired providers 1 (`:800-807`).
- **The exchange cannot take a candidate back.** Publication is an SPSC queue the render thread
  pops (`crates/engine/src/realtime/plan_exchange.rs:375-381`); the control thread has no way to
  withdraw what it pushed.
- **Base and producers.** A candidate is prepared against the newest epoch's inventory and the
  current committed model (`control.rs:907-914`, `SuccessorBase`,
  `crates/host-core/src/prepare.rs:641`). The carry program names its predecessor's plan identity
  (`prepare.rs:1792`), so a candidate prepared against an unadopted plan cannot carry from the
  plan render actually runs. Persisting source producers move to the candidate after the commit
  (`adopt_persisting`, `control.rs:1019`; `crates/host-core/src/source.rs:286`).
- **Host-fed state lives in a candidate.** From the commit on, submissions and seeks address the
  newest committed session (`crates/capi/include/miso_engine_v1.h`, "Sources across a structural
  transaction"). A source the candidate added already holds PCM, a generation and possibly a held
  `seek_at` in the candidate's ring (`PcmSourceConsumer::held_seek`,
  `crates/source/src/lib.rs:1018`).
- **The predecessor's model is dropped at commit.** `SessionStore::commit_prepared` overwrites the
  compiled session (`crates/protocol/src/model.rs:966`).

## Decisions frozen for this slice

- **D1. Withdraw first, then prepare.** When a transaction needs a rebuild and the newest epoch is
  an unadopted candidate A, the control plane withdraws A from the exchange before preparing. The
  withdrawal has two outcomes:
  - **Withdrawn:** A comes back unrendered and control-owned. The new candidate B is prepared
    against the plan render runs (P0), with A as a donor (D2).
  - **Taken:** render has claimed A. A is the base, exactly as today's path with A newest.

  This replaces the plan's "CAS, then re-target if render took it": B's preparation must know its
  render predecessor (the carry program names it), and A's host-fed rings can move only while
  the control thread owns A.
- **D2. Base across a withdrawn candidate.** B is prepared with `inventory` = P0's inventory
  restricted to the rows A carries from P0, and `committed` = P0's committed model (D3). So an
  owner carries from P0 only if A carried it and B leaves it unchanged; anything A restarted, added
  or removed is fresh in B. Every source A created (not carried from P0) that B keeps with the same
  declaration and ring configuration is donated: B takes A's consumer (ring, generation, read
  position, held seek) and producer, and its own fresh ones go to A. Persisting producers move
  from A's set into B's (`adopt_persisting(&mut A.sources)`; A holds P0's moved producers).
  The two host-core functions this needs come from #1344.
- **D3. Keep the predecessor's model.** The protocol's structural commit returns the compiled
  session it displaces. The control plane keeps it in the epoch being succeeded for as long as a
  successor of that epoch is pending, and drops it when that epoch retires. In the withdrawn case
  P0 already holds its model, so the model B's commit displaces (A's) is dropped. A live commit
  drops its displaced model as today.
- **D4. Every check runs while A is withdrawn.** Preparation, the resource admission (D6), the
  publication reservation and the protocol's commit predicate all run before anything moves. On
  any refusal A is republished unchanged, with its own retirement credit, and the refusal is
  returned. Republishing cannot fail: the control thread is the only publisher and the mailbox is
  empty.
- **D5. Then, infallibly, in this order:** protocol commit; donation and producer moves (D2);
  publish B; drop A's plan and provider epoch on the control thread; remove A's report row. B is
  published only after it owns the donated rings, because render may adopt it at once.
- **D6. Admission counts three plans, as a resource ceiling.** While superseding, the running
  plan, the withdrawn A and B coexist, and so do three compiled models (P0's kept one, the current
  one, B's). `validate_replacement_peak` (`compile.rs:339-420`) adds the third plan to each
  double-live row it checks today:
  - `maximum_graph_session_plus_plan_bytes` (three plans plus three compiled models),
    diagnostic `graph.resource.limit`;
  - `maximum_source_total_bytes` and `maximum_source_overhead_bytes` (a carried ring counted
    once), `source.resource.limit`;
  - `maximum_effect_state_bytes` and `maximum_effect_scratch_bytes`, `effect.resource.limit`;
  - `maximum_builtin_retained_bytes` and the control-retained cap (`maximum_capi_retained_bytes`
    in the C ABI struct), `capi.resource.limit`.

  A three-plan peak above one of these is refused `RESULT_COMPILE_REJECTED` with that cap's
  diagnostic, and A is republished (D4). It is a ceiling refusal: it fires only for a host whose
  caps cannot hold three plans of its session. The C ABI has no engine default caps (every
  `CompileLimits` field is caller-chosen and nonzero, `compile.rs:482-515`), so the header states
  the sizing rule (each of those caps at least three times one plan's row, plus three compiled
  models for the graph cap), and the repository's reference limits admit three plans of their
  sessions: `tools/audit/src/capi.rs:580-600`, `crates/capi/tests/plan_swap_race.rs:33` and
  `crates/capi/tests/resource_lifecycle.rs:172`. Gates 1, 2 and 5 run on those limits and never
  see the refusal; a reference limit that does not admit three plans is raised in this slice.
  The browser's caps follow the same rule when it adopts the control plane (#1332).
- **D7. Capacities.** Retirement capacity 2 (one in-flight retirement of P0 plus the one B
  reserves), report rows 3 (the lagging active epoch, A, B), pending providers 1, retired
  providers 2. With them the `:897`, `:961-974` and `:992-996` refusals cannot fire for a
  candidate; each becomes `CommandError::Internal`. `plan_alive == false` (`:876`) stays
  `BACKPRESSURE`.
- **D8. Live edits, submits and seeks** keep addressing the newest epoch, which is B after D5.
  A live edit made while A was pending is in the committed model B is prepared from.
- **D9. The acked-batch question: can an ack ever precede a drop? No.** A's revision was acked
  and stays committed; its edits are in the model B is compiled from, and its host-fed sources move
  into B (D2). If B is refused, A is republished untouched (D4). B is acked only after every
  fallible step (D4). A's revision is in effect when B is adopted; reporting it as `superseded`
  is *Publish an applied-revision watermark and complete edits asynchronously* (#1314).

## Deliverables

1. The supersession path in the control plane's `command`, with D3's model retention and D7's
   capacities.
2. The protocol change of D3.
3. Header text (`miso_engine_v1.h`, after "Sources across a structural transaction"): a
   transaction submitted while an earlier replacement is unadopted commits and supersedes it;
   the three-plan sizing rule of D6; a donated source keeps playing.
4. `docs/CONTROL_PROTOCOL_SEMANTICS.md` and `docs/C_ABI_V1_QUALIFICATION.md`: the same, briefly.

## Authorized paths

- `crates/control-plane/src/` (the moved `control.rs` and `compile.rs`).
- `crates/protocol/src/model.rs`, `crates/protocol/src/controller.rs` (D3 only).
- `crates/capi/include/miso_engine_v1.h` (prose only), capi tests.
- `docs/CONTROL_PROTOCOL_SEMANTICS.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- The exchange mailbox itself (#1343) and the host-core base and donation functions (#1344).
- Scheduled or exact-sample adoption (#1311), the watermark and outcome flags (#1314).
- The browser (it adopts the control plane in #1332).

## Objective gates

1. **Supersession equals one combined transaction (new capi test).** Compile, render one block.
   Without rendering, submit T1 (add a source `s` and insert an effect on track `t1`), T2 (change
   `t2`'s insert quality), T3 (remove `t3`'s insert). Each returns `RESULT_OK`. Feed `s` and every
   persisting source; render 8 blocks. The PCM is bit-identical to a twin that submits T1∪T2∪T3
   as one transaction at the same point. *Red if B carries from A's identity while P0 runs, if the
   base model is the current one instead of P0's, or if any of T1-T3 refuses.*
2. **A donated source keeps its PCM and its held seek (new capi test).** T1 adds `s`; submit 4
   blocks of PCM for `s` and `seek_at(s, 2, 0, A)` with `A` four blocks ahead; T2 changes another
   track's insert; render 8 blocks. `s` is heard from `A` exactly as in a twin where T1 was adopted
   before T2. *Red if B allocates a fresh ring for `s`, dropping acked PCM and the held seek.*
3. **A refused successor restores the candidate (new capi test).** T1 pending; T2 is refused by
   preparation (a cap one byte below its need). The response is the compile rejection, the
   revision is T1's, and after one render the output equals the T1-only twin. Repeat with each
   test fault phase of `TestStructuralFaultPhase` armed during T2: owners and credits balance as in
   `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits`
   (`crates/capi/src/runtime/tests.rs:2094`), which this extends to the withdrawn path.
4. **Three-plan ceiling (new capi test).** A `maximum_graph_session_plus_plan_bytes` that fits two
   plans and their models but not three: T1 pending, T2 returns `RESULT_COMPILE_REJECTED` with
   `graph.resource.limit`, revision unchanged, and after one render the output equals the T1-only
   twin (A was republished). After that render (A adopted, a two-plan peak) T2 again returns
   `RESULT_OK`. The same case with each other D6 cap one byte short.
5. **Concurrent render (extend `crates/capi/tests/plan_swap_race.rs`).** A render thread and a
   control thread submitting 64 structural edits back to back: every submit returns `RESULT_OK`,
   and after quiescence the output equals the twin of the final model. The run reaches both
   withdrawal outcomes (a `test-support` counter per outcome; both nonzero over the run). The
   harness's tolerance of a transient structural `RESULT_BACKPRESSURE` (`plan_swap_race.rs:391`)
   is removed: any structural `RESULT_BACKPRESSURE` now fails it. (The source-submit tolerance at
   `:294` is a full ring and stays.)
6. **Superseded tests.** Delete the assertions that a second structural edit is
   `RESULT_BACKPRESSURE` while one is pending (find them under `crates/capi/`); gate 1 replaces them.
7. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`:
   allocations, deallocations, locks, syscalls and `total_violations` 0.
8. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features
   test-support`; `cargo test --locked -p protocol --features test-support`;
   `bash scripts/check-capi-abi.sh`; `bash scripts/check-protocol-control-policy.sh`;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/check-workspace-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a successor that carries from the wrong predecessor, or compares against the wrong
  committed model; no test supersedes today, because supersession does not exist.
- Gate 2: a supersession that drops a withdrawn candidate's host-fed ring, which is an acked
  submission lost.
- Gate 3: a refusal path that drops or leaks the withdrawn candidate (an acked revision lost).
- Gate 4: a three-plan peak admitted beyond a cap (a missing third term), or a refusal that
  loses the withdrawn candidate.
- Gate 5: a race between withdrawal and render's claim; judged by reaching both outcomes.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Prepare a successor across a withdrawn candidate plan* (#1344).
