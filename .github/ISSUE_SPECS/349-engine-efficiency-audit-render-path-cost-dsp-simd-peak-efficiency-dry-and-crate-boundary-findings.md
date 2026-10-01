One-line summary: A read-only audit of the whole workspace at `main` `4797a544` for render-path
cost, DSP and math efficiency at the frozen numeric contract, SIMD utilisation, duplicated code, and
crate-boundary scope creep, recorded as self-contained, individually closable findings so that
implementation agents can execute each one from this body alone.

**This is an audit tracker, not implementation authority.** Every finding below is a candidate for
its own smallest-closable issue (or a small batch of same-crate findings). No cross-cutting "apply
the audit" branch is permitted. Findings are ranked; the ranking is advisory. Class-B findings are
flagged for owner ruling and MUST NOT be implemented from this body.

## Current execution status — 2026-09-11

Current authority; older sections historical. #737 causal compressor merged via
PR743 at8b1f0cb3 after Astra XHIGH scope, Luna XHIGH implementation and Sol XHIGH
final PASS. Exact-head PR qualification34593871669 PASS. Exact-main qualification
34594472401 PASS; #737 CLOSED COMPLETED, remotely verified. Both clean delivered
worktrees removed; pushed branches/history and external evidence retained. Zero added compressor
latency, removed lookahead/history/staging, retained SIMD; compatibility/PDC,
realtime, native/Wasm/browser and one bounded CPU qualification PASS. No speedup
or human listening claim; listening remains#26. #738 gate/#739 multiband queued;
limiter/softclip quality decisions remain separate. Root owns pins/GH/cleanup.
Two active slots; one launch feature; five attempts. Earlier #228/#360/#213/#522
CLOSED COMPLETED atmaina2e14c2e with required CI PASS and delivered tree removal.
#352 NOT_PLANNED(adapter/#394). Preserve #552/#558/#542 order and RT6/RT7/resident
meter delivery; broader RT10 open, #714 superseded. No original-finding credit.

## Authority and baseline

- Audited tree: `main` at `4797a544` (merge of #346), 2026-09-03.
- Method: seven parallel read-only reviews, one per scope (lane+math; EQ+dynamics; limiter/clip/
  transient/delay+references; render path engine/builtins/rack/graph; control plane compilers/
  session/host-core; protocol/capi/hosts/source; tools/scripts/CI). Each review was required to
  cite exact file:line ranges with verbatim snippets and to check its proposals against the
  standing rulings in `docs/rulings/`. No benchmark was run; no rendered bit was moved.
- Standing constraints every finding respects (implementers must too):
  - Unfused `(a*b)+c` everywhere; bit identity across scalar/W4/W8/wasm
    (`docs/rulings/unfused-multiply-add-audit.md`). Class A = same arithmetic and operation order,
    fewer instructions/loads/stores/copies/branches. Class B = moves a rendered bit; flag only.
  - No runtime SIMD dispatch; only `crates/lane` names `wide` or intrinsics
    (`scripts/check-lane-policy.sh`); `unsafe_code = "deny"` workspace-wide.
  - Render is zero alloc/free/lock/syscall/I/O/log/panic-path (`scripts/check-realtime-policy.sh`,
    `docs/REALTIME_DEPENDENCY_POLICY.md`).
  - Honest-null rulings are not re-proposed without new evidence (`docs/rulings/README.md`).
  - Already-named floor gap terms (`docs/rulings/effect-floor-accounting.md` boundaries 2–5):
    (a) `bitselect` lowering, (b) `vbroadcastss` re-splat, (c) compressor detector gather,
    (d) lookahead rings > L1, (e) AoSoA transpose round trip per bank, (f) elision-gate integer
    compares, (g) per-node graph dispatch. Findings that locate the code for a gap term say so.

## How to use this issue

1. Pick a finding (or a same-crate cluster). Create a numbered spec in `.github/ISSUE_SPECS/` and
   a matching GitHub issue whose body is the finding verbatim plus the gates listed under it.
2. Implement only that finding. The verification gate named in the finding is the acceptance gate;
   for class-A render-path findings the standing bit-identity fixtures and
   `scripts/check-realtime-policy.sh` are mandatory in addition.
3. A finding whose premise turns out false on inspection is closed with a one-paragraph null note
   appended to this tracker, not silently skipped.
4. Do not implement a class-B finding. Open an owner-ruling request that cites it.

## Verified in this audit session (facts, not proposals)

- `cargo test -p lane --release --test g1_op_identity` **fails on an AArch64 host** at `main`
  `4797a544` (`g1_random_vectors_are_lane_identical`: `exp2_int` differs from the scalar oracle at
  `Simd4` in 7 659 of 8 000 000 lanes, all signalling-NaN inputs, e.g. `a=0xff90882d
  oracle=0x7f000000 actual=0x00800000`). The same test **passes in debug**. The release test
  binary contains `fmaxnm`/`fminnm`. No CI job executes tests on aarch64. See LANE-3.
- `crates/capi/src/runtime/control.rs:196` types the production controller as
  `ProtocolController<MockProvider>` and `compile.rs:485` constructs it. See IO-4.
- `ProtocolQueues::try_dequeue_automation` (`crates/protocol/src/queue.rs:832`) has no caller
  outside `crates/protocol` and `tools/audit`; no host or render path drains the accepted-automation
  queue. See IO-5.
- `ArenaLease::set_muted`/`is_muted`/`wave` (`crates/engine/src/realtime/disjoint.rs`) have no
  production caller; `effective()` still pays the mute redirect per buffer access. See RT-14.
- `BankChain::scatter_tiled` (`crates/rack/src/lib.rs:2034-2095`) writes through a staging block and
  then copies per lane; `ArenaMembers::fold_plane` (`crates/graph/src/runtime.rs:686-697`) does one
  master read-modify-write pass per lane; `reduce_plane` (`runtime.rs:88-110`) does one pass per
  extra input. See RT-1/2/3.
- `tools/bench/src/session.rs:446` defines a private `sha256_hex` beside
  `bench_support::digest::sha256_hex`. See TOOL-1.

## Executive summary

123 findings across seven scopes. The picture is consistent across scopes:

1. **The render-path gap is memory traffic and dispatch, not arithmetic.** The floor ruling already
   said this for the compressor and EQ; this audit locates the code for every named gap term and
   finds the same shape in the plumbing rows. The idle row (18 % of floor) and the plumbing row
   (6.7 %) are dominated by: a doubled scatter (RT-1), per-lane master folds (RT-2), per-input
   reduction passes (RT-3), three block passes for six lane-ops of fader+matrix (RT-4), per-block
   register spills in symmetry bookkeeping (RT-5, #238), `dyn` walks over cached flags (RT-6, RT-8,
   RT-12/CP-13), and constant splats that lower to a libc `memset_pattern16` call on Apple targets
   (LANE-1). All are class A.
2. **Gap term (c), the compressor detector gather, is live and has spread.** It moved into
   `fill_taps`/`copy_lane` (DYN-1) and the gate/expander (DYN-2) and multiband (DYN-3) carry worse
   copies of it. One shared `effect_runtime::ring::gather_tap` with a uniform-`D` vector-load arm
   fixes all three. Gap term (b), constant re-splat, is written in source in multiband (DYN-4),
   gate/expander (DYN-6), transient shaper / soft-clip / limiter (FX-4). Gap term (a),
   `bitselect`→`vblendvps`, is already closed in code but still priced as open in the floor tables
   and bench constants (LANE-13).
3. **Three block-invariant decisions are branched per frame** in the limiter (FX-1), delay (FX-11)
   and the mixed builtins chain (RT-7); soft-clip has no stationary split at all (FX-6). All are
   `const`-generic unswitch fixes with no arithmetic change.
4. **Realtime-safety is true but unenforced.** No allocation, `%`, or panic edge was found on the
   render path in `rack`/`builtins`/`graph`/effects, but the gate that would prove it scans only
   `crates/engine/src/realtime` (RT-16 / IO-14). Live exceptions found: a 2 KB stack zero-fill per
   block in delay (FX-10), a `(3T+E)`-entry `fill(0)` per block in host-web (IO-7), 512 B of scratch
   zeroed per bank per block in the EQ (DYN-9), 16 `expect` edges in the source pull path (IO-13).
5. **Crate boundaries are inverted in specific, fixable places:** the shipped C ABI's control
   provider is a conformance mock (IO-4); the accepted-automation queue and the effect control lanes
   are two disconnected automation models (IO-5); ~600 lines of transport-neutral console ABI live
   only in the wasm host (IO-18); `rack` exports the planner's vocabulary (RT-15);
   `builtins-compiler` holds ten render-path `process` impls while `graph` holds the compiler's caps,
   diagnostics and lowering pass (CP-12); `effect-package` restates ~400 lines of
   `effect-contract`'s descriptor law (CP-8).
6. **DRY debt is concentrated and countable:** ~430 lines of identical descriptor boilerplate across
   eight effect crates (DYN-12/FX-12), 20 copies of one xorshift generator (LANE-11/FX-15), seven
   near-identical `determinism.rs` files (FX-16), four hand-rolled counting allocators with
   `#![allow(unsafe_code)]` (FX-17), four rack enums and two stable-id newtypes (CP-5), three
   diagnostic shapes (CP-6/CP-9), 42 hand-written protocol encoders over one table (IO-2), seven
   spellings of the 17-message registry (IO-3), nine RIFF writers (TOOL-6/IO-20), seven
   `MANIFEST.tsv` implementations (TOOL-5), 21 gate scripts sharing one skeleton and 35 mutation
   harnesses with 12 verbatim `expect_failure` copies (TOOL-11/12).
7. **The control plane allocates by the tens of thousands per compile** because the front half is
   keyed by heap-`String` ids that the lowering pass interns to `u32` at the end (CP-1), `lower`
   runs three times per plan (CP-2), and the runtime un-flattens the exemplary SoA
   `ExecutionProgram` back into per-op boxes (CP-14).
8. **CI builds cold** (zero Rust caching, 27 wasm32 builds per PR, the shipped worklet built three
   times, `cargo-fuzz` installed from source three times: TOOL-13), `tools/audit` carries 4 720
   lines no gate invokes (TOOL-3), and the browser artifact ships without `wasm-opt` (IO-19).

## Dependency-ordered program (advisory ranking)

Each wave is independently closable; within a wave, rows are independent unless noted.

**Wave 0 — correctness and gate integrity (do first; each is a bounded issue).**
LANE-3 (aarch64 D8 max/min fold; add an aarch64 CI execution leg), IO-4 (real control provider in
`capi`), IO-5 (decide and wire the automation drain, or retire one model), IO-9 (= #292),
RT-16/IO-14 (root-agnostic realtime gate; then add markers to `rack`/`builtins`), LANE-9
(vectorization allowlist still requires the retired fused FMA), TOOL-11 (= DRY half of #306),
LANE-13 (floor tables/bench constants still price gap term (a) as open).

**Wave 1 — render-path class A against the worst floor rows.** Measure each with
`scripts/run-console-benchmark.sh` on the row the finding names.
RT-1, RT-2, RT-3, RT-4, RT-5 (= #238), RT-14, LANE-1, LANE-2, DYN-1 → DYN-2 → DYN-3 (share one
kernel), DYN-4, DYN-6, FX-1, FX-10, IO-7, RT-7, FX-4.

**Wave 2 — remaining render-path and compile-path class A.**
RT-6, RT-9, RT-10 (step 1 only; step 2(b) is flagged), RT-11, RT-12/CP-13 (one change),
CP-14, DYN-5, DYN-8, DYN-9, DYN-10, DYN-11, DYN-14, DYN-15, DYN-17, FX-2, FX-6, FX-7, FX-8, FX-9,
FX-11, LANE-4, LANE-5, LANE-6, IO-12, IO-13, IO-16, IO-19, CP-1, CP-2, CP-3, CP-4, CP-18, CP-19.

**Wave 3 — boundaries, DRY, dead code, structure.** Pure moves and deletions; gate is
`cargo test --workspace` identical pass count plus the named policy scripts.
RT-13, RT-15, RT-17, CP-5 … CP-12, CP-15 … CP-17, CP-20, DYN-7, DYN-12, DYN-13, DYN-16, FX-5,
FX-12 … FX-19, LANE-7, LANE-8, LANE-10, LANE-11, IO-1, IO-2, IO-3, IO-8, IO-10, IO-11, IO-15,
IO-17, IO-18, IO-20, TOOL-1 … TOOL-10, TOOL-12 … TOOL-16.

**Class B — flagged for owner ruling, never implemented from this body.**
FX-3 (limiter per-sample `div` by block-constant window), LANE-12 (soft-clip flushes two
feed-forward FIR history words), RT-10 step 2(b) (an `f64` lane type for meter energy), and the
standing compressor-`D`-in-program-key question from `effect-floor-accounting.md`.

## Cross-references (same defect seen from two scopes — implement once)

| pair | note |
|---|---|
| RT-12 ↔ CP-13 | executor block loop double-indexing and observer walk |
| RT-16 ↔ IO-14 | realtime gate scans only `engine/src/realtime` |
| RT-5 ↔ #238 | symmetry refresh O(records×lanes) |
| IO-9 ↔ #292 | track ids read through the source-id buffer |
| TOOL-11 ↔ #306 | gate-script skeleton duplication is the DRY half of #306's fix |
| FX-15 ↔ LANE-11 | xorshift64* copies (20 total) |
| FX-12 ↔ DYN-12 | effect descriptor id boilerplate |
| FX-17 ↔ #332 | counting allocators; #332 deferred the consolidation |
| TOOL-6 ↔ IO-20 | RIFF/RF64 fixture writers |
| RT-10 ↔ IO-8 | meters / master-peak fold should be one lane-generic kernel |
| DYN-7 ↔ FX "already fine" | envelope followers are shared; the *branching one-poles and detector links* are not |
| LANE-4 ↔ DYN-1 | 4 of the compressor's 94 floor ops are `exp2_int`'s redundant clamp |

## Findings index

Full self-contained findings (current code, proposal, expected effect, rulings checked,
verification gate) are in the comments below, one comment set per scope. Each scope's report also
ends with a **"Things I checked that are fine"** list — read it before re-auditing a crate.

### Render path (engine / builtins / rack / graph executor) — `RT-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| RT-1 | A | high | `crates/rack/src/lib.rs:2034-2095` | Tiled bank scatter writes every word twice through a staging block; 2 loads + 2 stores per lane-sample where 1 + 1 suffices |
| RT-2 | A | high | `crates/graph/src/runtime.rs:686-697` | Route fold accumulates into the master once per lane — 8 read-modify-write passes per cohort per plane |
| RT-3 | A | high | `crates/graph/src/runtime.rs:88-110` | `reduce_plane` is `n−1` block passes; a fan-in-64 output node costs 126 loads + 63 stores per word |
| RT-4 | A | high | `crates/builtins/src/lib.rs:1926-1948, 2170-2181` | Fader and matrix are three block passes for six lane-ops; fuse the settled arms into one |
| RT-5 | A | high | `crates/builtins/src/lib.rs:1036-1044, 1632-1690` | `refresh_channel_symmetry` spills up to 240 SIMD registers per record and per ramping block (issue #238) |
| RT-6 | A | medium | `crates/rack/src/lib.rs:1428-1486` | Collapse dispatch makes `lanes × slots` `dyn` calls per block; publish a `[u8; 8]` per stage |
| RT-7 | A | medium | `crates/lane/src/kernels/builtins.rs:857-916` | `mixed_chain_block` reads its plan-invariant elision plan inside the frame loop |
| RT-8 | A | medium | `crates/rack/src/lib.rs:1565, 1684, 1709` | **Delivered:** #478/PR #523 replaced the three invariant walks with prepared packed-mask predicates at merge `70ce3d7b`; required/post-main runs `34031771524`/`34032117797` passed |
| RT-9 | A | medium | `crates/graph/src/runtime.rs:2686-2731` | One armed observer or send tap declines the chain merge and costs two extra whole-block transposes; tee instead |
| RT-10 | A | medium | `crates/builtins/src/lib.rs:3625-3663` | Meters are a scalar per-track third pass; meter the resident AoSoA block lane-generically instead |
| RT-11 | A | medium | `crates/effect-contract/src/lib.rs:234-250` | Tile transpose crosses a 256-byte `[[f32;8];8]` by-value boundary in both directions; add a `[L; W]` form |
| RT-12 | A | medium | `crates/graph/src/lib.rs:1417-1420`, `crates/graph/src/runtime.rs:938-953` | Block loop double-indexes `units` and walks every bank member for observers that are not bound |
| RT-13 | N/A | medium | `crates/engine/src/realtime/buffer.rs:36-125` | `BufferArena` is allocated, passed to `render` and never used by the only production executor |
| RT-14 | A | medium | `crates/engine/src/realtime/disjoint.rs:208-232` | Lease wave/mute machinery is unreachable; the mute redirect costs a load + mask + select per buffer access |
| RT-15 | N/A | medium | `crates/rack/src/lib.rs:20-140` | `RackProgram`/`BankSlotKey`/`RackLocation` are compiler-only vocabulary living in the render crate |
| RT-16 | N/A | medium | `scripts/check-realtime-policy.sh:14-46` | The realtime gate scans only `engine/src/realtime`; the marked regions in `graph` and all of `rack`/`builtins` are ungated |
| RT-17 | N/A | medium | `crates/builtins/src/lib.rs`, `crates/graph/src/lib.rs` | Concrete module splits; two thirds of `graph/src/lib.rs` and `program.rs` is an inline test module |
| RT-18 | N/A | low | (sweep) | No `%`, alloc or panic surface found on the render path — reported as a negative result with two caveats |

### Effects: parametric EQ, compressor, gate/expander, multiband — `DYN-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| DYN-1 | A | high | `crates/compressor/src/kernel.rs:816-878` | Per-lane detector gather is `W` scalar loads + `W` scalar stores per channel-frame in both `fill_taps` and `gather_detector`; a uniform-`D` arm makes it one vector load. |
| DYN-2 | A | high | `crates/gate-expander/src/kernel.rs:259-293` | Four tap gathers per frame: two duplicate each other when the channels' taps agree, and two are dead under `DualMono`. |
| DYN-3 | A | high | `crates/multiband-compressor/src/lib.rs:865-880`, `:1000-1006` | `detector_tap` runs four times per frame with two distinct index sets and no uniform-offset path. |
| DYN-4 | A | high | `crates/multiband-compressor/src/lib.rs:902-932` | `band_amplitude` rebuilds `GainComputerCoef` and re-splats seven constants four times per frame — 28 broadcasts. |
| DYN-5 | A | medium | `crates/compressor/src/kernel.rs:906-967`, `:1066-1074` | Link mode and bypass are prepared-time constants spent as per-lane-sample mask selects; const-generic them as the multiband already does. |
| DYN-6 | A | medium | `crates/gate-expander/src/kernel.rs:316-357`, `:392-398` | Seven constant splats and three block-invariant mask derivations per channel-sample; no `Invariants` equivalent. |
| DYN-7 | N/A | medium | `compressor/src/kernel.rs:1028`, `gate-expander/src/kernel.rs:392`, `multiband-compressor/src/shim.rs:52`,`:70` | Three branching one-poles and three detector links; one kernel each in `effect-runtime::dynamics` (the shim's own header says so). |
| DYN-8 | A | medium | `crates/parametric-eq/src/lib.rs:822-830` | Elision gate is a scalar five-int-op-per-word scan; two thirds of it is `bank::check_block` in the vector domain. |
| DYN-9 | A | medium | `crates/parametric-eq/src/lib.rs:1858-1863`, `:1913-1917` | 512 bytes of stack scratch zeroed per bank per block for state only read on silent blocks. |
| DYN-10 | A | medium | `crates/multiband-compressor/src/lib.rs:694`, `:1169-1281` | Track-major ramp storage costs ~800 strided scalar accesses per block to transpose into and out of lanes. |
| DYN-11 | A | low | `crates/compressor/src/kernel.rs:258-264` | `advance_cursor` runs two integer `%` on the silent fast path, against the crate's own "no `%`" rule. |
| DYN-12 | N/A | medium | eight `crates/*/src/lib.rs` descriptor headers | ~430 lines of byte-identical `effect_id`/`port_id`/`parameter_id`/`parameter` boilerplate. |
| DYN-13 | N/A | medium | `compressor/src/design.rs:81-118`, `multiband-compressor/src/lib.rs:425-457`, `gate-expander/src/lib.rs:326-343` | Three `ParameterSpec` translation tables routing around a predicate that is now public. |
| DYN-14 | A | low | `crates/parametric-eq/src/lib.rs:1872-1885`, `:1923-1932` | Open-codes `bank::finish_channel`, which `bank.rs:241-245` names as the divergence it exists to stop. |
| DYN-15 | A | low | `crates/compressor/src/kernel.rs:83-104` | 1 152 bytes of control-plane arrays sit ahead of every render-read word of `Channel`. |
| DYN-16 | N/A | low | `effect-runtime/src/ramp.rs:31-41`, `gate-expander/src/kernel.rs:66-89`, `multiband-compressor/src/lib.rs:662-678` | Three representations of one D11 ramp law, enforced three different ways. |
| DYN-17 | A | low | `crates/parametric-eq/src/lib.rs:1505-1540` | Interleave's load/store per pass is fixed by the register ceiling; only the per-pass coefficient gather is takeable. |

### Effects: true-peak limiter, soft-clip, transient shaper, delay, references, conformance — `FX-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| FX-1 | A | high | `crates/true-peak-limiter/src/lib.rs:1890-1906`, `:1701-1717`, `:3086`, `:3197` | The block-invariant `stationary` hoist is branched per frame inside the innermost loop; make it `const STATIONARY: bool` |
| FX-2 | A | medium | `crates/true-peak-limiter/src/lib.rs:1603-1619` | The 79 %-of-kernel detector loads/stores through open-ended slices, keeping ~8 bounds compares per lane-frame |
| FX-3 | B (flag) | medium | `crates/true-peak-limiter/src/lib.rs:1551`, `:1308` | `box_sum.div(hot.window)` is a `vdivps` per lane-frame by a block-constant — the ruling's named class-B residual |
| FX-4 | A | medium | `crates/transient-shaper/src/lib.rs:292,317-331`; `crates/soft-clip/src/kernel.rs:150-157`; `crates/true-peak-limiter/src/lib.rs:1528,1543,1545` | Loop-invariant constants re-splatted per sample while each crate already has a prepared coefficient struct |
| FX-5 | A | medium | `crates/true-peak-limiter/src/lib.rs:3030-3242` vs `:1575-1963` | The mono-collapse path is a hand-copied 213-line duplicate of the dual block bodies |
| FX-6 | A | medium | `crates/soft-clip/src/kernel.rs:188-215` | Soft-clip has no stationary split: 9 block-invariant lane-ops per channel-frame, and no `settle` to pay for the hoist |
| FX-7 | A | medium | `crates/soft-clip/src/kernel.rs:194,204`; `crates/lane/src/kernels/halfband.rs:126-133` | The dry history is a 64-row double-written ring where 32 rows and one store suffice |
| FX-8 | A | low | `crates/lane/src/kernels/halfband.rs:182-197` | `halfband2x_decim_even` compares a counter against a constant on every one of 30 taps |
| FX-9 | A | medium | `crates/transient-shaper/src/lib.rs:333-335`, `:513-528` | The identity mask's `bypass`/`mix == 0` terms are block-invariant on the dominant tail loop |
| FX-10 | A | high | `crates/delay/src/lib.rs:1266`, `:729-746` | 2 KB of stack tap windows zero-filled on the render thread every block; move into `PreparedDelay` |
| FX-11 | A | medium | `crates/delay/src/lib.rs:1139-1152` | `tap_sample` branches per sample on two chunk-invariant flags, twenty lines after three siblings were hand-unswitched |
| FX-12 | N/A | medium | 8 effect crates + `crates/effect-package/src/wire.rs:1929` | Nine byte-identical `effect_id`/`port_id`/`parameter_id` const panic-wrappers; add them to `effect-contract` |
| FX-13 | N/A | medium | `crates/soft-clip/src/lib.rs:883-890` + 5 more | Six `bind_homogeneous_bank`s hand-roll the check `PrepareEffectBankRequest::validate_shape` documents as universal |
| FX-14 | N/A | medium | `crates/soft-clip/src/lib.rs:903-913` | `width_is_native` re-derives the backend→width law and disagrees with `Backend::current()` under `miso_wasm_simd8` |
| FX-15 | N/A | medium | 8 `corpus.rs` modules incl. `crates/effect-runtime/src/corpus.rs:57-73` | Eight copies of the same frozen `xorshift64*` that drives every determinism corpus |
| FX-16 | N/A | medium | 7 × `crates/*/tests/determinism.rs` (681 L) | One digest-vs-pins pattern written seven times; make it a macro next to `effect_conformance_test!` |
| FX-17 | N/A | medium | 4 × `crates/*/tests/allocation.rs` (~950 L) vs `tools/bench-support/src/alloc.rs` | Four hand-rolled counting `GlobalAlloc`s and four `#![allow(unsafe_code)]` under `crates/`; #332 deferred this |
| FX-18 | A | low | `crates/delay/src/lib.rs:505-522`; `crates/lane/src/kernels.rs:527-544`; `crates/true-peak-limiter/src/lib.rs:1042` | Ring segment/wrap arithmetic written three times; the delay ring and the PDC line are correctly *not* one kernel |
| FX-19 | N/A | low | `crates/conformance/src/effect.rs:28-660` vs `:661-1562` | A mock effect and the conformance harness share one 1 562-line file |

### SIMD foundation and math (lane / math) — `LANE-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| LANE-1 | A | high | `crates/lane/src/wide_impl.rs:144-148` | `splat(CONST)` emits a `memset_pattern16` libc call per use on Apple targets; 2 per frame inside `svf_step` |
| LANE-2 | A | high | `crates/lane/src/kernels.rs:472-491,497-513,563-590` | mix kernels carry 4-6 bounds branches per frame because only `debug_assert` relates the slice lengths |
| LANE-3 | A | high | `crates/lane/src/wide_impl.rs:278-330` | AArch64 release folds D8 `max`/`min` into `fmaxnm`/`fminnm`; G1 is red today and no CI job runs on aarch64 |
| LANE-4 | A | medium | `crates/lane/src/wide_impl.rs:332-340` | `exp2_int` re-clamps an argument both callers already clamped — 4 of the compressor's 94 floor ops |
| LANE-5 | A | medium | `crates/lane/src/kernels.rs:450-465` | `ramp_block` evaluates a per-frame vector select of a monotone predicate; split the loop |
| LANE-6 | A | medium | `crates/lane/src/kernels.rs:295-322` | `svf_block_ramped` recomputes `-c1` and the ramp test on every frame including settled ones |
| LANE-7 | A | medium | `crates/lane/src/kernels/builtins.rs:408-1096` | six near-identical copies of the input-chain frame body (~330 lines) |
| LANE-8 | N/A | medium | `crates/math/src/fast_db.rs:123,128` + 3 crates | `20*log10(2)` and its inverse declared 4x each in 3 spellings; belongs in `math` |
| LANE-9 | N/A | medium | `tools/audit/vectorization-allowlist.tsv:4,7` | the vectorization gate requires fused multiply-add that #163 phase 2 removed; masked by `continue-on-error` |
| LANE-10 | N/A | low | `crates/math/src/lib.rs:38` | test-only `corpus` module compiled into every production consumer of `math` |
| LANE-11 | N/A | low | `crates/math/src/corpus.rs:70-90` + 19 files | one xorshift64* generator copy-pasted twenty times |
| LANE-12 | B (flag) | low | `crates/soft-clip/src/kernel.rs:195,199` | `flush` applied to two feed-forward FIR history words; documented but outside the D7 law's wording |
| LANE-13 | N/A | low | `docs/rulings/effect-floor-accounting.md:134-136,183-186` | gap term (a) is closed in code; the floor tables and bench constants still price it as open |

### Control plane (session, compilers, graph control half, host-core, effect-package) — `CP-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| CP-1 | N/A | high | `graph-compiler/src/schedule.rs:9-278`, `ids.rs:50-110` | Whole compile front-half keyed by heap-`String` `GraphNodeId`; ~15-20k allocations per 64-track compile that `lower` then interns to `u32` |
| CP-2 | N/A | high | `graph/src/lib.rs:840-868,906,824,1017` | `program::lower` runs 3× per plan; cached program discarded at bind because 8 fields are `pub` for tests |
| CP-3 | N/A | medium | `graph/src/lib.rs:630-716` | `has_valid_structural_layout` re-runs the topological/permutation checks the compiler and `lower` both already prove |
| CP-4 | N/A | medium | `graph/src/lib.rs:958-996` | Bind builds six string-keyed `BTreeSet`s and then `.clone()`s one of them |
| CP-5 | N/A | medium | `session/src/model.rs:536`, `effect-compiler/src/prepare.rs:844`, `graph/src/lib.rs:28,50`, `rack/src/lib.rs:42` | Four rack enums (two with conflicting `Ord`), two identical stable-id newtypes, two conversions, two token tables |
| CP-6 | N/A | medium | `effect-compiler/src/diagnostic.rs:1-20`, `builtins-compiler/src/lib.rs:233-250`, `graph/src/lib.rs:284-303` | Three identical `{code, path}` diagnostic types; the three `sorted()` copies already disagree about `dedup` |
| CP-7 | N/A | low | `host-core/src/prepare.rs:382,419,568,613,641,716,747` | Seven verbatim copies of the same 9-line diagnostic-mapping closure |
| CP-8 | N/A | high | `effect-package/src/wire.rs:1303-1633` vs `effect-contract/src/lib.rs:460-773` | `effect-package` restates ~400 lines of `effect-contract`'s descriptor law; the `_parts` fix pattern already exists for one rule |
| CP-9 | N/A | medium | `effect-package/src/diagnostic.rs:1-197` | Three near-identical `#[repr(C)]` wire diagnostics; only one has `as_str` and a frozen-numbering test |
| CP-10 | N/A | medium | `effect-package/src/wire.rs:403-470,786-1030` | ~200 naked wire-offset literals hand-kept in three parallel tables |
| CP-11 | N/A | medium | `graph-compiler/src/lib.rs:264-10166` and 3 others | Four giant files (one 97% inline tests); concrete per-item module assignments given |
| CP-12 | N/A | medium | `builtins-compiler/src/lib.rs:364-666,2963-3131`; `graph/src/lib.rs:160-303`; `graph/src/program.rs:487` | Compiler crate holds 10 render `process`/`observe` impls; render crate holds the compiler's caps, diagnostics and the lowering pass |
| CP-13 | A | medium | `graph/src/lib.rs:1416-1419`, `graph/src/runtime.rs:890-958` | Two bounds checks + two discriminant loads per unit per block; observer walk runs on every unit including unobserved banks |
| CP-14 | A | medium | `graph/src/runtime.rs:415-430,2564-2588` vs `graph/src/program.rs:106-124` | Runtime un-flattens `ExecutionProgram`'s contiguous SoA into three `Box<[..]>` per op (~900 small allocations) |
| CP-15 | N/A | medium | `graph/src/runtime.rs:2618-2637` vs `graph/src/program.rs:1624-1643` | Property-test "independent oracle" is a verbatim copy of the implementation it checks |
| CP-16 | N/A | low | `session/src/compile.rs:17-243`, `graph/src/lib.rs:250-283`, `host-core/src/prepare.rs:732` | 4/6 `CompileCaps` fields documented-inert; 8/11 `GraphCompileCaps` fields always `u64::MAX`; unused `_session` params; speculative `active_mask` |
| CP-17 | N/A | medium | `graph-compiler/src/ids.rs:108-138`, `compile.rs:354-411` | Total functions returning `Option`; three unreachable `graph.port.unknown` branches; the real failure is a `.expect()` panic |
| CP-18 | N/A | low | `host-core/src/prepare.rs:656-698,900-906` | `session_structural_symmetry` computed twice per prepare; each track id heap-allocated three times |
| CP-19 | N/A | medium | `graph-compiler/src/compile.rs:169,553,572,652` | 294 KB `CompiledSession` deep-cloned per structural compile (tracked #162); `GraphResourceEstimate` is 23 `u64`s but not `Copy` |
| CP-20 | N/A | low | `graph-compiler/src/canonical.rs:27-91,363-372` + 12 sites | 14 hex encoders in 3 styles; `node_text`/`node_text_len` twin kept in step by a test when a `fmt::Write` sink already exists in the file |

### Boundaries and I/O (protocol, capi, hosts, source, flac sidecar) — `IO-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| IO-1 | N/A | medium | `crates/protocol/src/{lib.rs:74-77,conformance.rs:1-487,controller.rs:618-810}` | conformance corpus + `MockProvider` ship inside the library; 33.8 % of the crate is inline tests |
| IO-2 | N/A | medium | `crates/protocol/src/message_wire.rs:862-1980` | 42 encode entry points repeat one three-statement body; one generic codec + the existing `MessageSpec` table replaces ~350 lines |
| IO-3 | N/A | medium | `wire.rs:98-165`, `typed_frame.rs:41-106`, `controller.rs:1036-1080`, `:2340-2402`, `schema.rs:259+` | the 17-message registry is spelled out in seven hand-maintained tables |
| IO-4 | N/A | high | `crates/capi/src/runtime/compile.rs:485-508`, `control.rs:196` | the shipped C ABI uses `protocol::MockProvider` — a conformance fixture — as its production control provider |
| IO-5 | N/A | high | `crates/protocol/src/queue.rs:196-283`, `controller.rs:3643-3675`, `crates/effect-contract/src/live.rs:44-90` | the accepted-automation queue has no render-side drain; two disconnected automation models exist |
| IO-6 | N/A | medium | `crates/protocol/src/queue.rs:243-278`, `:1098-1211` | admission is ~5.3 M iterations per 256-record batch and validates every batch twice |
| IO-7 | A | high | `hosts/host-web/src/lib.rs:1244` | `(3T + E)`-entry `fill(0)` on the render thread every block, unconditionally |
| IO-8 | A | medium | `crates/capi/src/ffi.rs:823-837`, `hosts/host-web/src/lib.rs:1252-1269` | two scalar master-peak folds; one lane-generic `host-core` kernel replaces both |
| IO-9 | N/A | high | `crates/host-core/src/shape.rs:26-28`, `hosts/host-web/src/lib.rs:1091-1109` | issue #292: track IDs read through the source-ID buffer; silent `RESULT_INTERNAL` |
| IO-10 | N/A | medium | `crates/capi/src/abi.rs:11-29`, `hosts/host-web/src/lib.rs:69-100`, `sidecars/flac-decoder/src/lib.rs:23-39` | codes 3 and 5 mean different things natively and in the browser with no test saying so; `capi` discards `PrepareRejection`; two ungenerated JS copies |
| IO-11 | N/A | medium | `effect-contract/src/lib.rs:133-142`, `session/src/model.rs:327-357`, `protocol/src/message_wire.rs:183-350` | six parameter vocabularies declared 2-3× plus ~180 lines of conversions between the copies |
| IO-12 | A | medium | `crates/source/src/native_source.rs:1690-1695`, `crates/source/src/lib.rs:849-854` | the decode worker memcpys a whole planar quantum into the ring slot it could decode into |
| IO-13 | A | medium | `crates/source/src/lib.rs:1160-1210`, `:1074-1100` | 16 `expect` panic edges and plan-invariant shape re-derivation in the render pull path |
| IO-14 | N/A | medium | `scripts/check-realtime-policy.sh:12-48` | three files' `REALTIME_POLICY` markers are never scanned by the gate that owns them |
| IO-15 | N/A | medium | `crates/capi/src/runtime/compile.rs:106-205`, `hosts/host-web/src/lib.rs:2362-2434` | two hand-maintained resource projections that must each track every allocation |
| IO-16 | N/A | medium | `crates/protocol/src/controller.rs:2407-2418`, `typed_frame.rs:278-314` | every response zeroes a `max_response_bytes` `Vec` and is serialised twice |
| IO-17 | N/A | low | `crates/engine/src/realtime/spsc.rs:456-556`, `crates/target-smoke/src/lib.rs:31-41` | `LocalRing` is unused by any host and keeps two `unsafe` blocks alive for a smoke test |
| IO-18 | N/A | high | `hosts/host-web/src/lib.rs:1594-2190` | ~600 lines of transport-neutral console-command ABI live in the wasm host; the C ABI cannot reach it |
| IO-19 | A | medium | `scripts/build-web-audioworklet.sh:37-60` | the 1.94 MB browser artifact ships without any `wasm-opt` pass |
| IO-20 | N/A | low | 5 sites, ~361 lines (see finding) | the RIFF/RF64 test-fixture writer is reimplemented five times |
| IO-21 | N/A | low | `crates/host-core/src/prepare.rs:36`, `:129-133`, `crates/host-core/src/diagnostics.rs:15` | `host-core` re-checks the launch rate with its own array on a path where the check provably cannot fire |

### Tools, scripts, CI, artifacts — `TOOL-*`

| id | class | sev | location | finding |
|---|---|---|---|---|
| TOOL-1 | N/A | high | `tools/bench/src/session.rs:446-585` | 140-line hand-written SHA-256 beside `bench_support::digest::sha256_hex`; the policy gate's regex cannot see it |
| TOOL-2 | N/A | medium | `tools/audit/src/vectorization.rs:293-295` | a tenth JSON escaper, and one of the invalid kind (control chars raw); six `json_string*` copies the gate's anchor misses |
| TOOL-3 | N/A | medium | `tools/audit/src/main.rs:31-50` | six audit subjects (4,720 lines, 31% of the crate) invoked by no script, workflow or doc |
| TOOL-4 | N/A | medium | `tools/audit/src/fixture_builtins.rs:2675-2870` | two JSON readers in one 6,025-line file for the same records; neither is the workspace's |
| TOOL-5 | N/A | medium | `scripts/check-effect-runtime-fixtures.sh:5-24` | `MANIFEST.tsv` has 4 Rust owners + 3 shell verifiers; the `LC_ALL=C` fix exists in only one copy |
| TOOL-6 | N/A | medium | `tools/audit/src/source_duration.rs:231-248` | nine hand-rolled RIFF/WAVE writers; `crates/source` owns the reader and nothing owns the writer |
| TOOL-7 | N/A | medium | `tools/native-pcm-runner/src/lib.rs:393-431` | re-implements `stem-hasher`'s wave→canonical-PCM→SHA-256 and the launch bit-depth set without depending on it |
| TOOL-8 | N/A | medium | `tools/bench/src/floor.rs:47-131` ↔ `scripts/console-benchmark-record-lib.jq:53-72` | the two floor tables are compared only in an operator-only run, never in CI, and never by key set |
| TOOL-9 | N/A | medium | `tools/bench/src/console.rs:1678-1712` | 7 `Metadata` structs, 4 `Percentiles`, 4× three formatters, 18 mega `format!` record templates |
| TOOL-10 | N/A | low | `tools/wasm-gates/src/lib.rs:251-262` ↔ `tools/wasm-console/src/main.rs:299-325` | two wasmtime embeddings, two digest-word protocols, two runtime pins; the 5-crate split itself is justified |
| TOOL-11 | N/A | high | `scripts/check-workspace-policy.sh:20-50` | `scan_forbidden` fixes #306's bug and 20 sibling gates re-implement the broken form; 5 copies of one awk |
| TOOL-12 | N/A | medium | `scripts/test-rack-policy.sh:20` | 35 mutation harnesses / 5,610 lines with 12 verbatim `expect_failure` copies and no shared harness |
| TOOL-13 | N/A | high | `.github/workflows/ci.yml:68-70,376-388,440` | zero Rust caching, 12 cold jobs, 27 wasm32 builds/PR, shipped worklet built 3×, `cargo-fuzz` built 3× |
| TOOL-14 | N/A | medium | `fixtures/flac-delivery/v1/generate.py:92,124` | 3 uninvoked binaries, 2 dead crate names breaking a generator, an orphan validator, a deleted `sweep.sh` cited 24× |
| TOOL-15 | N/A | low | `artifacts/` (`scripts/run-console-benchmark.sh:266-268`) | 70/70 `raw`/`accepted` jsonl pairs byte-identical: ~4.5 MB of the 9.85 MB is a second copy |
| TOOL-16 | N/A | medium | `sdk/src/internal/session-json.ts:13-62` | the canonical-JSON leaf-type tables are hand-transcribed from Rust beside a working Rust→TS codegen pipeline |

## Non-goals

- No rendered bit moves under any class-A row; a row that turns out to need one is re-classed B and
  stops.
- No new crate names, no renames for their own sake, no runtime SIMD dispatch, no `unsafe` outside
  the existing allowlist without a ruling.
- No benchmark-driven tuning loops: one measurement per row on the named workload, recorded, not
  chased.
- Multicore render, `f64` lanes, ADAA, reciprocal divides, and any change to a frozen operation
  order are outside this tracker.

## Evidence requirements per implemented row

1. The finding's own verification gate, green.
2. For render-path rows: `scripts/check-realtime-policy.sh` and the workspace fixture bit-identity
   suite green; the named console-benchmark row re-measured once (descriptive).
3. A one-paragraph note on this tracker: row id, commit, measured before/after where applicable, or
   the null result if the premise did not hold.


https://claude.ai/code/session_01LkmW681maQhUqEVSA6vZSj

## Active/passive lanes

Active: #220 route-fold coverage and #162 compiler borrowing; #221 queued. #387/#211/#376 delivered and closed. See current
#559/#560 checkpoint before selecting the next bounded issue. Broader audit and
program trackers remain open until their own closure conditions are satisfied.
