# 122 Emit sorted graph dependency levels for valid native binding

## Outcome and readiness

Repair only the graph compiler's dependency-level member ordering so every valid compiled graph
binds through the accepted native single-thread and dependency-wave paths with deterministic,
strictly node-ID-sorted levels.

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE AFTER UPSTREAM AND CI SYNCHRONIZATION.** Sol High
completed the single implementation pass and Sol XHigh returned strict PASS without consuming the
bounded HOLD. Benchmark, timing and workload invocations were forbidden and remained zero. Once
this final evidence is upstream, CI is green and GitHub is synchronized, Issue 122 may close and
its dependency edge into Issue 026 is unblocked.

Remote Issue 122 was read-only confirmed unallocated on 2026-08-23. Root owns GitHub creation,
body synchronization and state changes after this docs checkpoint is committed and upstream. This
record authorizes no GitHub mutation.

## Accepted authorities and technical input

Accepted product authority comes from **Deterministic graph compiler, sends, submixes, sidechains,
and PDC** (Issue 006): implementation checkpoint
`40f0a2f3f5057e725e80715da18afb0e5f4d6bb3` (tree
`d8898cde03e4a7d12314e707369f67551607ea3b`) and its accepted product rescope
`e1211bba07d680a0a97dcfccc87ce0a167dbca50` (tree
`f31db1e03beed6c1b2b16f77a4c7093ae2338d18`). Preserve its graph semantics, deterministic
reduction, PDC, resource accounting, ownership and no-track-ceiling contract. Its historical
exactly-once runner failure and consumed benchmark authority remain closed history; Issue 122 does
not reopen or rerun them.

Accepted native execution authority comes from **Native graph scheduler qualification and
benchmark** (Issue 039): sealed candidate
`290037ccebc64204a743cd13f93e240a84f93040` (tree
`cb732a6def7516e8dac71f7f745df76ba321b028`) and final evidence commit
`157b3eae11d500a6d1bdc4cea37a36827461b8ac` (tree
`1caa1873fbe38674dd751ed38dde35479a86ca40`). Preserve its native single-thread/dependency-wave
ownership, partition, reduction, observer and realtime contracts. No scheduler benchmark or
qualification workload may be repeated.

Open audit **099 Audit: miso-engine-graph-compiler (level-order bug, plan lowering, cohort
duplication)** is technical input only. Its finding F1 was recorded against commit
`ae02d2abd9bd5e3e97b33152cfc943013325045e` (tree
`8fa639d0212171570e790bc2626bb622370a3fca`): `topo` appends each dependency-level member in
Kahn ready-pop order, while native binding requires each `DependencyLevel.nodes` list to be
strictly ascending by `GraphNodeId`. A valid parallel-submix graph can therefore compile but fail
native binding solely because route IDs become ready in reverse lexical order. Issue 099 supplies
no implementation authority or overall PASS and its other findings are out of scope.

The live briefing baseline is clean `main`
`9e489502d381d8b8c191882e1cf8018d748747e0`, tree
`2994a4a4e0c91e7b4cdba0bc70c1dcd47e531fbf`. At that baseline the defect remains present in
`crates/miso-engine-graph-compiler/src/lib.rs`: level members are pushed during the ready-pop loop,
and `NativeGraphBlueprint::prepare` still rejects non-ascending members as
`graph.scheduler.layout`.

## Smallest closable correction

Change only dependency-level construction in the graph compiler. Derive or sort each level's
members into strict `GraphNodeId` order after topological levels are known. The result must be
independent of the order in which nodes become ready and must retain the existing level number for
every node.

Add one focused valid session with a track feeding parallel submixes whose downstream route IDs
sort opposite to their readiness order. Compile it through the accepted public graph-compiler
path, then bind the resulting plan through both native `SingleThread` and `DependencyWaves`
configurations. The exact small regression proves both paths accept and produce the same expected
PCM, PDC and stable observer order. It is a focused correctness test, not a workload or benchmark.

Prove for the regression and existing deterministic graph fixtures that:

- dependency levels are ordered by increasing level number;
- every level is nonempty and its node IDs are strictly ascending;
- every compiled node appears in exactly one dependency level and exactly once in the canonical
  sequential schedule;
- every edge's source level is strictly lower than its destination level;
- fresh compilations emit byte-identical canonical graph bytes and SHA-256; and
- native single-thread and dependency-wave binding accept the valid graph without renaming IDs.

## Pinned pre-existing graph-fixture exception

Sol High's read-only preflight on the committed briefing candidate invoked exactly:

```sh
cargo run --quiet --locked -p miso-engine-graph-compiler --bin miso_engine_graph_fixture -- --check fixtures/graph
```

It exited nonzero with the sole output `graph fixture manifest mismatch`. This is a pre-existing,
out-of-scope fixture drift caused only by five newly reported zero resource-estimate fields; no
dependency-level, schedule, topology, PCM, PDC or graph behavior differs.

The exact checked/generated mismatch is frozen:

| Path | Checked bytes / SHA-256 | Generated bytes / SHA-256 |
| --- | --- | --- |
| `v1/direct-route.canonical.txt` | `3726` / `7ae045dceca0490f4607817a2a44739492cd4a3cf68718f11a865571871ea9bb` | `3734` / `40bd3d4c126bf3cc8aa1730ebdda12371ffca4ecba2d2b1c94da3f1e9b0579e3` |
| `v1/direct-route.report.json` | `331` / `c45d1065a90ab157100b36458ec6393f4c1ea63d974683203519f5516e2448c0` | `331` / `d2546a263146537b5da0786e7c793977912a4eb70bc2c858785ad4d0776c948c` |

The canonical file has one and only one changed line:

```text
-estimate\t9\t9\t8\t9\t9\t0\t1\t0\t2561\t0\t0\t5336\t0\t5336\t15580\t27868
+estimate\t9\t9\t8\t9\t9\t0\t1\t0\t2561\t0\t0\t5336\t0\t0\t0\t0\t0\t5336\t15580\t27868
```

The report changes only its embedded canonical byte count and SHA-256. All dependency-level and
sequential-schedule rows are byte-identical; deleting the sole `estimate` row yields SHA-256
`2683c125be905b87170172715956589e998f374bc2d15f1c2b17b4f1181d50e5` for both canonical files.
There are no new or missing fixture paths. Every other generated payload is byte-identical:

- `direct-route.dot`: `1359` bytes,
  `a5febba237458a01737653eeb4221634a3b827e4e5661fee44a4c28e2abe0499`;
- `direct-route.resources.json`: `350` bytes,
  `2ed780ba9e3a90a4b38ba241ae6dcc287dd9566d7fe144d6a5101ec765dc2b80`;
- `invalid-scc-diagnostics.json`: `296` bytes,
  `9b43dbd8d62935e3eb1d96e39c0114b0dfd0bec91d84a6a786cdc0902a4b4600`;
- `main-sidechain-pdc.csv`: `358` bytes,
  `c08654d06220ecec7e0730298725392bba3afeb2b4779fc20c597d003bfead29`;
- `summation-residuals.json`: `859` bytes,
  `ee39298ed192f19af249c1e6b550618851dc3af31b2619413c5d9f5293348b2f`.

Issue 122 must not edit, regenerate or bless these fixtures or their manifest. The broad fixture
check is an expected nonzero baseline observation, not a PASS gate. It is acceptable only when the
same command reproduces the exact sole output and the complete mismatch remains exactly the two
rows and identities above, with no additional, missing or changed difference.

## Frozen boundaries

The canonical sequential schedule is frozen byte-for-byte and must continue to use its existing
Kahn schedule. Do not concatenate dependency levels into a replacement schedule, change buffer
coloring, move bank execution, change reduction order or alter either executor. Open audit Issue
098's stale/zero bank-lane defect, level-major schedule proposal and executor/buffer-coloring work
are explicitly deferred to a separate stateless successor.

Freeze graph node/port/edge meanings, topology, level-number calculation, PDC/latency/tail laws,
buffer/resource estimates, canonical field grammar, DOT/report semantics, diagnostics, plan
ownership, scheduler configuration and every public interface. No core, graph-runtime, scheduler,
session, source, effect, builtin, protocol, C ABI, host, runner or benchmark behavior may change.

## Allowed tracked paths

- `crates/miso-engine-graph-compiler/src/lib.rs`;
- its existing focused graph-compiler tests;
- this issue spec and its tracked brief; and
- minimal Issue-122 routing in `.github/ISSUE_SPECS/README.md`,
  `docs/IMPLEMENTATION_PLAN.md` and the Issue-026 dependency list.

Any edit to `crates/miso-engine-graph/**`, `crates/miso-engine-native-scheduler/**`, Cargo
manifests/lock, accepted benchmark artifacts, unrelated fixtures or another production crate is
STOP and requires a new issue or amended brief before implementation.

## Dependencies by exact title

- **Deterministic graph compiler, sends, submixes, sidechains, and PDC**
- **Native graph scheduler qualification and benchmark**

Issue 122 gates **End-to-end release, performance, and listening qualification**. It does not
reopen Issues 006 or 039 and does not depend on Issue 098's broader executor correction.

## Acceptance gates

1. The reverse-route-ID parallel-submix regression reproduces the pre-correction unsorted level
   structurally, then passes through production compile and both native binding modes after the
   correction. Exact PCM, PDC and observer order match between modes.
2. Independent assertions prove node-once membership, strict level/member order and strict
   source-level-before-destination-level for every edge. A mutation that restores ready-pop member
   order or reverses one level must fail before binding evidence can pass.
3. The existing sequential schedule is byte-identical to the baseline for the regression and
   checked fixtures. Existing buffer-color assignments, canonical grammar, graph diagnostics,
   PDC/reduction results and resource rows remain unchanged. No checked fixture or manifest byte
   changes in this issue.
4. Repeated fresh compilations of the focused graph have one canonical identity. Focused
   level/canonical checks and effective ready-pop/reversed-level mutations pass. The pinned full
   fixture command reproduces exactly its baseline-only `graph fixture manifest mismatch`, the two
   exact resource-zero-derived rows and no other/new/missing difference. Corruption tests relevant
   to level membership, sequential schedule and canonical identity pass; they may use disposable
   scratch bytes but cannot regenerate or modify `fixtures/graph/**`.
5. Existing focused graph-compiler tests, locked package tests, warning-denied package
   Clippy/rustdoc, format, graph/realtime policies and applicable mutation checks pass.
6. Exact-path diff/static scans prove the allowed fence, no Issue-098 executor/schedule work, no
   generated artifacts and counters of `benchmark_invocations=0`,
   `timed_benchmark_invocations=0`, `workload_invocations=0`.
7. Sol High freezes one clean exact-path checkpoint. Sol XHigh performs a read-only adversarial
   review and returns strict PASS or the sole bounded HOLD. After a HOLD, the one correction is
   terminal PASS or STOP; gates may not be weakened.

## Target matrix and evidence

Run focused native Linux correctness only. Cross-compilation, browser/device execution, long-run
audits and target qualification remain outside this issue because no target-specific code changes.
Record the clean candidate and tree; exact changed-path hashes; accepted authority hashes; the
reverse-ID graph topology; ordered level transcript; node/edge invariants; sequential schedule and
canonical before/after identities; native mode PCM/PDC/observer identities; mutation outcomes;
the exact pinned fixture mismatch reproduction; relevant corruption outcomes; strict gate results;
zero prohibited counters; and Sol High/Sol XHigh verdicts.

## Explicit non-goals

Issue-098 executor or buffer-coloring correction; plan lowering; bank/cohort unification; graph or
scheduler optimization; new features or APIs; fixture expansion; target/browser/device matrix;
benchmark, timing, workload, soak or listening execution; or V1/legacy inspection.

## Sol High pass-1 evidence — 2026-08-23

The product correction is limited to sorting each completed `topo` dependency-level member vector.
The existing Kahn ready-pop loop, node-level calculation and sequential schedule are unchanged.

The focused production-path regression compiles this exact topology:

```text
track:vocal:post-matrix
  -> route:to-a-submix -> submix:a-submix -> route:z-downstream --+
  -> route:to-z-submix -> submix:z-submix -> route:a-downstream --+-> output:main-out
```

The frozen Kahn schedule is:

```text
track:vocal:input, track:vocal:post-input-builtins, track:vocal:post-simd1,
track:vocal:post-dynamic, track:vocal:post-simd2-pre-fader, track:vocal:post-fader,
track:vocal:post-matrix, route:to-a-submix, route:to-z-submix, submix:a-submix,
route:z-downstream, submix:z-submix, route:a-downstream, output:main-out
```

Its corrected dependency-level transcript is levels 0–6 containing the corresponding single
track stages above, level 7 `[route:to-a-submix, route:to-z-submix]`, level 8
`[submix:a-submix, submix:z-submix]`, level 9
`[route:a-downstream, route:z-downstream]`, and level 10 `[output:main-out]`.
Reconstructing members in frozen schedule order produces the pre-correction level-9 mutation
`[route:z-downstream, route:a-downstream]`; the independent contract rejects it before binding.
Explicit reversed, omitted, duplicated, schedule-swapped and canonical-byte mutations are also
rejected.

The contract proves increasing nonempty levels, strictly ascending members, exactly-once node and
schedule membership, and strict source-level-before-destination-level for every edge. Fresh
compilations preserve identical schedules, buffer assignments, route timings, PDC, canonical bytes
and hashes. The reconstructed pre-correction canonical SHA-256 is
`6676779806af8bb20c9abb287a39488512fc7c0972e96f6cd300f469539bd770`; the corrected identity is
`3e5c3e43fc220ec91eb159d18749bec44fd96fba3f6ef908850c850d995582ce`. Removing dependency-level
rows makes the two canonical payloads byte-identical.

Native `SingleThread` selects the explicit sequential fallback and `DependencyWaves` selects the
parallel scheduler. Both bind without ID changes and render bit-identical q128 PCM: left frame 0 is
`2.0` (`0x40000000`), right frame 0 is `-2.0` (`0xc0000000`), and every other sample is positive
zero. Both have output latency 0, no inserted delays, and execute observer handles 1 then 2 with
two observations of nonzero post-matrix audio.

Focused Issue-122, direct-route render, semantic-hash, buffer-coloring, timing and scratch-only
fixture-corruption tests pass. Warning-denied graph-compiler Clippy/rustdoc, format, graph policy,
realtime policy and its mutations, and exact diff/static checks pass. The amended fixture audit
reproduces exit 1 with sole output `graph fixture manifest mismatch`, exactly two mismatched paths,
the pinned sizes/hashes and sole five-zero estimate-row change, with no new or missing path and all
level/schedule rows byte-identical. No fixture, generator or manifest byte changed.

Final counters are `benchmark_invocations=0`, `timed_benchmark_invocations=0`, and
`workload_invocations=0`.

## Sol XHigh final review — strict PASS

The bounded pre-handoff correction at commit
`d985a5191b0a6402dbe28c545cc7780fb0c27552` (tree
`da6b67975a814c49914685dcae2991f474ed79b4`) amended only this spec and its tracked brief to pin
the already-existing graph-fixture mismatch. It occurred before the immutable implementation
handoff, changed no gate to excuse Issue-122 behavior, and is a briefing amendment rather than a
HOLD. The implementation then passed the first Sol XHigh review strictly; no HOLD was issued or
consumed.

The accepted technical checkpoint is commit
`776e2cbbc7d68fd7ac3dc95825dfe99651df5be1`, tree
`f032193af3ff11499003c6bc91e71dcb828acc07`. Its exact two-path fence and SHA-256 identities are:

- `.github/ISSUE_SPECS/122-emit-sorted-graph-dependency-levels-for-valid-native-binding.md`:
  `124b9db78be5d0c2d5fdd638fbe2608ca9f8b3d7f4c36044812948392105a00c`;
- `crates/miso-engine-graph-compiler/src/lib.rs`:
  `269f1a0300e4756104164ee0ed2b60ab1868dfd9ea74caf187f06642f2e9e87f`.

The sole production change sorts each completed dependency-level vector; all other compiler-file
changes are focused `#[cfg(test)]` evidence. The frozen sequential schedule, executor selection,
buffer assignment, timing/PDC/resource calculations, graph runtime, native scheduler, fixtures,
Cargo files and Issue-098 scope remain unchanged. The reverse-route-ID topology is nonvacuous:
its frozen ready-pop reconstruction produces level 9
`[route:z-downstream, route:a-downstream]`, while production emits the required sorted level 9
`[route:a-downstream, route:z-downstream]`. Independent checks prove increasing nonempty levels,
strict member order, node-once and schedule-once membership, and strict source-before-destination
edge levels. Ready-pop, reversed-level, omitted-node, duplicate-node, schedule-swap and canonical
mutations all reject.

Fresh compilations preserve the exact sequential schedule, buffer assignment, route timing, PDC
and canonical identity. The reconstructed pre-correction and corrected canonical SHA-256 values
are respectively `6676779806af8bb20c9abb287a39488512fc7c0972e96f6cd300f469539bd770`
and `3e5c3e43fc220ec91eb159d18749bec44fd96fba3f6ef908850c850d995582ce`;
removing dependency-level rows makes their bytes identical. Native `SingleThread` and
`DependencyWaves` both bind and produce identical q128 PCM, zero latency/no inserted delays, and
observer order 1 then 2 exactly as recorded above.

The focused Issue-122, direct-route render, semantic-hash, buffer-coloring, timing and
scratch-corruption tests passed, as did warning-denied graph-compiler Clippy/rustdoc, format,
graph/realtime policies and mutations, and exact diff/static checks. The pinned fixture command
remained the sole expected exception: exit 1 with only `graph fixture manifest mismatch`, exactly
the two previously recorded resource-zero-derived path identities, five inserted zero fields,
byte-identical level/schedule rows and other payloads, and no new or missing path. Fixtures,
generator and manifest remained read-only.

Final prohibited counters are exactly `benchmark_invocations=0`,
`timed_benchmark_invocations=0`, and `workload_invocations=0`. Sol XHigh therefore records strict
PASS with no HOLD. Issue 122 is COMPLETE and READY TO CLOSE after the technical and final evidence
are upstream, CI passes and GitHub is synchronized; that synchronization unblocks Issue 026's
Issue-122 dependency.
