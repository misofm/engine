# Walk the engine plan's retained bytes so capi's plan check is exact

Successor of #1060 (verdict attempt 2, MEDIUM). #1060 made memory tests "budgets plus a completeness check" (owner decision 4) and closed the catalog and canonical-JSON under-counts, but the engine plan is still bounded rather than observed: its bound has about 8-128 KB of slack, so an uncharged 1.8 KB graph table stays green.

## Smallest closable slice

Add an allocator-checked `retained_bytes()` walk over the graph runtime (plan, banks, schedule, tables), computed off the render thread. Then capi's plan check becomes: retained plan bytes == walked bytes <= the engine budget rows. A test with a counting allocator proves the walk equals what the plan actually holds.

## Gates

1. A planted uncharged table in the plan (the 1.8 KB case from #1060's verdict) goes red.
2. The walk equals the allocator's observed bytes for every console fixture, at Simd4 and Simd8, native and wasm32.
3. Render stays allocation-, lock- and syscall-free; console digests unchanged.

## Decision record (attempt 1, 2026-10-24)

Scope note `scratchpad/opt/scope/1074.md` (Fable, against main 9e99df434) governs paths and decisions.

- D1 (scope): the slice is a required `fn retained_bytes(&self) -> u64` (no default) on
  `PreparedPlanExecutor`, `PreparedNativeEffect`, `PreparedNativeEffectBank`, `BankStage` and
  `GraphPreparedBuiltinBankProcessor`, plus `PreparedRenderPlan::retained_bytes`, computed from
  capacities off the render thread. Convention: heap bytes the value owns, exclusive of itself; the
  owner of a box adds `size_of_val`. No render change, no ABI change, no host-core report row.
- D2 (root, R1): an `Arc` the plan holds (`RevisionGate`, the automation `Program`) is charged in full;
  the capi oracle drops the plan after every other holder. Strong-count-dependent walking is refused
  as data dependent.
- D3 (root, R2): `HostAttachment` is excluded from the walk and documented: capi binds none, host-web's
  `RenderCompanions` is host-owned and host-web has no plan oracle. Reopen if host-web gets one.
- D4 (root, R3): `Vec`/`String` spare capacity counts; a spare found is fixed at its owner, not
  tolerated.
- D5 (root, R4): gate 2's "wasm32" is dropped. The walk is target-neutral arithmetic; x86-64 (8 lanes)
  and `scripts/run-aarch64-tests.sh` (4 lanes, capi closure included) cover both widths in CI.
- D6 (gate 1): the capi oracle asserts `owners.plan == walked` and `walked + owners.meters <= engine_rows`
  on its four fixtures; gate 1's planted table is PR evidence, not a committed test.
- D7 (paths): root approved test-double impls only in paths the scope note does not name:
  `tools/console-workload/tests/paired_spans.rs`, `crates/effect-contract/src/live.rs` (test mod),
  `crates/rack/tests/{lane_rest,live_control_bank,mono_reengage,lane_swap}.rs`,
  `crates/compressor/tests/native_points.rs`, `crates/graph-compiler/tests/{scale,bypass_cohorts}.rs`,
  `crates/graph-compiler/src/lib.rs` (test mod).
- D8 (root, mid-attempt): the plan also boxes `GraphRuntimeProcessor`, `GraphRuntimeObserver` and
  `GraphPreparedSourceSetDriver`; each gets the same required `retained_bytes`. Paths extended, impls
  only: `crates/source/src/lib.rs`, `tools/audit/src`, `crates/host-core/src/spectrum.rs`
  (`SpectrumCaptureObserver`), `tools/console-workload/src/lib.rs` (`FrozenSourceDriver`), and every
  test double of the eight traits.
- D9 (root): walkers on plan-held types outside the first set, control-side only, no behaviour change:
  `EffectControlLane`/`BypassShunt` in `crates/effect-contract/src/live.rs`, and `automation::Program`
  /`Composed` in `crates/automation/src`, so an `Arc<Program>` is charged in full.
- D10 (refines D2): `engine::realtime::LiveValuesSuperseded` (an `Arc<AtomicU64>`) is one host-supplied
  counter cloned into every lane; the host allocates it before the plan and it outlives the plan, which
  never frees it. The walk excludes it and documents it as host-owned, like D3. Every Arc the plan
  builds (`RevisionGate`, `EffectCells`, the automation `Program`) stays charged in full.
- D11 (reading of D2): the session's `Arc<automation::Program>` is one allocation shared by every
  automated holder in a plan (each automated fader lane, each following route). "Charged in full"
  means each distinct allocation once: the graph runtime charges it once at the plan level, and each
  per-lane or per-route clone charges zero for it and says where it is charged.
- D12 (root, condition 1): builtins preparation builds the automation program per plan
  (`ProgramBuilder::build` in `prepare_fader_automation`, wrapped in a fresh `Arc`), so the plan owns
  it. A control-only `GraphExecutor.automation_program` clone, set at bind by graph-compiler, is
  where it is charged once; render never reads that field. Host-core's `vca_cells` clones outlive no
  plan in capi's drop order (the plan drops last).
- D13: meter rings (`Producer`/`Consumer` of `MeterSnapshot`) are charged in full by the plan walk;
  the capi oracle drops `meters` and every other owner before the plan so the ring frees with it.
- D14: `GraphFadeArmWatch`'s Arc is shared by the plan and the plan state inventory, whose host-core
  row (`inventory_retained_bytes`) already charges it. It is charged once, there: the plan walk charges
  it zero and the capi oracle drops the inventory after the plan. Every other owner drops before the
  plan.
- D15: the session's `PublishedGeneration` Arc that the source driver holds is held and charged by the
  plan state inventory and outlives the plan; the driver's walk charges it zero (D14's rule).

## Evidence (attempt 1)

- Gate 1, `cargo test --locked -p capi --test resource_lifecycle` (x86-64, 8 lanes): 10 passed. Walked
  == freed by drop, bytes: EQ nine-track 220,484 (meters 675, engine rows 443,361); soft-clip nine-track
  184,051 (675, 406,251); browser identity 44,793 (77, 84,423); routed submix 140,033 (376, 269,988).
- Root condition 2 (automation coverage): `fixtures/session/v1/builtins-automation.json` as a fifth
  row holds the plan equality (96,771 == 96,771) but is red on a pre-existing host-core gap outside
  this issue: the strip producer table retains 1,016 bytes against `strip_control_table_bytes` 936 on an
  automated session. The row is not kept; it needs that host-core charge fixed first (successor).
- Gate 1 planted table (PR evidence, reverted): a `Box<[u64]>` of 16 words per unit added to graph
  `Runtime` with no walk entry turns the capi test red at the first fixture: "parametric-eq-nine-track:
  dropping the prepared plan freed 222036 bytes; its retained-bytes walk names 220500".
- Effects outside the capi fixtures (scratch, deleted): for all eight native effects, bytes freed by
  dropping the boxed effect == `size_of_val` + walk, per node and as full and padded banks at every
  width this x86-64 host binds, at 48 and 96 kHz; the same for the 30 builtins bank processors of a
  live-control/automation session and a VCA session, and the meter observer.
- Console digests: one untimed warmup run of `bench console` at base 9ac7d0d6b and at branch
  ed3c04c3a: all 42 digest lines identical.
- D16 (root): #15 S1 merged first, so this branch adds the method to `crates/de-esser/src` (impl only):
  both the per-node and the bank `DeEsser` own no heap (every field, `Coef` and `Chan` included, is
  inline) and return 0.

## Fold after the verifier's PASS

- A fifth capi row, "every native insert" (three tracks of the EQ fixture; inserts parametric-eq,
  compressor, gate-expander / delay, multiband-compressor / true-peak-limiter, transient-shaper,
  soft-clip; defaults, no automation): walk == freed, 927,078 bytes; every other owner equality holds.
  The de-esser is left out: it is not in the native registry on this tree (`effect-compiler`'s factory
  list), so no session can name it yet. Test value: a heap owner that the compressor, gate, delay,
  multiband, limiter or transient shaper walk misses or over-counts turns this row red; before it, no
  committed test caught that.
- Mutant (PR evidence, restored): drop `+ slice(&self.main_ring)` from the limiter's `ChannelState`
  walk; the new row goes red: "every native insert: dropping the prepared plan freed 927078 bytes; its
  retained-bytes walk names 923190".
- Successor (root files it): the automation walkers stay uncovered until host-core charges
  `FaderCells.offsets` (`crates/builtins-compiler/src/lib.rs:301`), which `strip_control_table_bytes`
  (`crates/host-core/src/prepare.rs:645-651`) leaves out. That successor's gate is the
  builtins-automation capi row this attempt dropped (strip producers 1,016 freed vs 936 charged).
- The control-only spsc walkers now sit outside the REALTIME_POLICY region; `PcmSourceConsumer`'s walk
  says it counts queued transfer blocks, so only a quiescent walk means anything.
