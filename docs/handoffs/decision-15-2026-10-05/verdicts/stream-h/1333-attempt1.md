PASS

# #1333 attempt 1 under Amendment 1 -- adversarial verdict

Reviewed: `git diff 1e658aa65 b55968d0d` (4830bed93 cherry-pick + b55968d0d completion), branch
`codex/d15-stream-h`. Built and run from an export of b55968d0d in `/tmp/claude-1002/v1333/tree`
(target `/tmp/claude-1002/v1333/target`); mutations ran in a second export with its own target.
The worktree was only read with `git show/diff/archive`. Evidence:
`/home/bl/misofm/submix-verdicts/evidence/1333-attempt1/`.

There is no BLOCKER and no MAJOR. Every objective gate (1-8) passes again. Every mutation the
attempt record names goes red when I apply it and green when I revert it. The open items are
outside the authorized paths and are follow-ups, but root must file each one as an issue before
#1333 closes (see MINOR 3-5).

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **Nothing defends the reservation in the hop boot export.**
   `hosts/host-web/src/ffi.rs:3822` (`miso_engine_web_v1_boot_with_spectrum_hop`) wraps its
   own `reserved_after_boot` call. Gate 8 boots only through `miso_engine_web_v1_boot`
   (`hosts/host-web/tests/render_locked_staging.rs:100`, `:127`). No raw browser workload uses a
   spectrum hop. The SDK `sdk-spectrum-hop` instance does use one, but it never reads its count
   (see MINOR 3).
   - Mutation: remove `reserved_after_boot` from the hop export. Gate 8 stays green, and so does
     every other gate.
   - Effect: for a worklet booted with a hop, gate 8's claim ("a staging still lazily allocated on
     first touch turns it red") is false.
   - D5 is met in the code, so this is a test gap, not a defect.
   - Cheap fix inside the authorized paths: move the reservation into the success path of
     `boot_staged`, so the one test covers both exports. Or boot phase 2 through
     `boot_with_spectrum_hop`.
2. **The test-value claim for `selected_channels_reports_the_selected_entrys_mask` is wrong.**
   - The attempt record (spec lines 497-499) and `hosts/host-web/MUTATIONS.md:563` name its
     unique catch as "reports another entry's mask (reads `captures[0]`)". An existing test also
     catches that mutation: `ffi::spectrum_ffi_tests::collection_selection_is_atomic_and_keeps_one_active_capture`
     (`ffi.rs:5638`, fails at `:5746`: left 1, right 3).
   - The test does have a unique catch: it is the only test that turns red when the accessor
     returns a mask while nothing is selected. Mutation:
     `Some(self.captures[self.selected.unwrap_or(0)].channels())`. Only this test goes red across
     host-web and host-core. A collection starts unselected after preparation, so production can
     reach this case.
   - Fix: correct the record and the MUTATIONS.md row. The test stays.
3. **Follow-up (open item 1, in scope for a successor, not for this slice): the SDK-path
   instances do not read their counts.**
   - `hosts/host-web/qualification/sdk-response-entry.ts` creates many `createEngine` instances
     (meters, spectrum streams, the hop-1024 boot, live bypass). None of them reads
     `renderAllocationCount()`.
   - Context: the real render-thread allocation was found in one of these instances. The new raw
     `staging-reads` workload covers that exact collection path.
   - That file is not in the authorized paths, and A2 scopes the reads to `qualification.js`, so
     this is correctly a follow-up.
   - Root must file it as an issue before close. It would also close the coverage hole in MINOR 1.
4. **Follow-up (open item 2): the hermetic boot contract does not witness the new boot caller.**
   - `qualificationBootContract` (`qualification.js:887`) and its witness list
     (`scripts/test-web-audioworklet.mjs:2650-2656`) do not include `runStagingReadRun`. That
     caller boots a new option shape (`spectrum: null` plus a two-entry `spectrumCollection`).
   - The new host method `renderAllocationCount()` has no hermetic test of its reply validation
     (tag, exact fields, `result === 0`, u32 `count`). Only the happy path in the browser legs
     exercises it.
   - `test-web-audioworklet.mjs` is not authorized, so this is a follow-up issue.
5. **Follow-up (open item 3): the policy document now has a gap.**
   - The header comment in `scripts/check-realtime-policy.sh:23-29` now names `render_lock.rs`.
     It also still says that `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership",
     "carries the full justification".
   - That section (`:45`) does not name `render_lock.rs`. A4 authorized the listing only, so this
     is a follow-up. File it or fold it into stream J's realtime-policy slice.

## NIT

- `SpectrumCaptureCollection::selected_entry()` (`crates/host-core/src/spectrum.rs:871`) now has
  no caller in the workspace. It is the cloning accessor that caused A1. Leaving it public beside
  the non-cloning accessors invites the same regression. Remove it or mark it off-render in a
  follow-up (A1 authorized only the new accessor and its test).
- `native_staging` (`ffi.rs:4566`) is a public, `doc(hidden)` module in the native rlib, not
  behind host-web's existing `test-support` feature. This is acceptable: the spec's gate-8
  command has no `--features`, so a `required-features` gate would skip the test. The cfg that
  keeps it out of wasm is defensive only.
- `unhashed()` (`scripts/check-web-audioworklet-callgraph.py:466`) removes hashes but keeps v0
  back-reference offsets (`B5_`). A change in hash length could move an offset. The result is a
  false red, never a false green.
- After a trap inside a locked window, `RENDER_LOCKED` stays set, so later control exports are
  counted. The spec froze this ("panic is abort"), and the error is on the safe side. Also, a
  sticky instance cannot answer `miso.renderallocations.v1`.
- `scripts/check-realtime-policy.sh` changed its header comment as well as the listing. This is
  slightly wider than "A4's listing only", but it is consistent and harmless.

## The implementer's choices, judged against the spec

| Choice | Verdict | Reason |
|---|---|---|
| D2 locked set excludes the pre-boot `spectrum_request_*` and `spectrum_collection_*` | Accepted | D2's next paragraph frames the set as post-boot ("#1332 makes the set the worklet's whole post-boot export set"), and D15-10 says "after boot". The worklet calls these only in `stageSpectrumRequest` and `stageSpectrumCollectionRequest`. `_collection_entry_ptr` sizes its staging by allocating, so locking it would make every collection boot read nonzero. I audited every post-boot export the worklet calls that is a staging pointer, capacity or byte accessor: all 29 locked exports match D2. |
| Stagings reserved only on a successful boot | Accepted | D5's stated invariant ("a booted instance never allocates a staging later") holds. A refused boot has no render thread to protect. `check-web-boot-budget.mjs:76-77` limits a typed pre-parse refusal to one page of growth. Defense gap for the hop export: see MINOR 1. |
| `closure()` prefers an exact name match | Accepted | Required by D3's mandated name: `_render` is a prefix of `_render_allocation_count`. An exact symbol match is strictly more precise. Self-test (h4) defends it. |
| `#[doc(hidden)]` non-wasm `native_staging` | Accepted (NIT) | A 64-bit `pointer_u32` returns 0, so the test needs native writers. The writers store only what the worklet writes. |
| Global allocator under `cfg(all(wasm, not(test)))` | Accepted | Prevents two `#[global_allocator]`s in a wasm unit-test build. The shipped build is non-test wasm. Gate 5 proves the counter is live in the browser. |
| `INDIRECT_SITES` entries and reasons | Accepted | Re-measured: render 2 in 2 members, meter_poll 0, command_submit 13 in 5. Each reason matches the source: `plan.rs:861` and `:943`, `live_route_state.rs:189` and `:261`, `EffectControlOwner::edit`, `prepare.rs:884` and `:898`. |
| New workload `runStagingReadRun`, gates `staging-reads` and `render-allocations` with their mutations | Accepted | Deliverable 3 needs each of the four reads before the count read, and no existing raw workload made them. Both gates go red under `--self-test-mutations` in all three browsers. |
| Open items 1-3 | Follow-ups | All three are outside the authorized paths. File them as issues before close (MINOR 3-5). |

Acked-batch question: there is no new queue. `renderAllocationCount()` is a read-only request
with one in flight at most. Saturation is refused with typed backpressure (`#request` at
`miso-engine-v1-audio-worklet-host.js:1226-1227`), and the worklet answers in `receive`, in the
same call. No ack can precede a drop.

Naming and version suffixes: the internal names (`RenderLockedAllocator`, `render_locked`,
`RENDER_ALLOCATIONS`, `MisoRenderAllocations`) have no version. `miso_engine_web_v1_render_allocation_count`
and `miso.renderallocations.v1` are contract identity. Digests: only the three A3
`memoryBytes` rows moved. Each is +17 pages (1310720->2424832 twice, 1376256->2490368), and the
spec record and the commit message give the reason. No PCM digest moved.
`check-browser-expected-resources.py` compares every other row exactly. All changed paths are
inside the spec's authorized paths plus Amendment 1's added paths.

## Test value, per new test (one sentence each)

- `render_lock::tests::render_lock_counts_each_entry_point_only_inside_its_own_window`: it turns
  red for a wrapper that counts outside its window or skips any one of `alloc`, `alloc_zeroed`,
  `realloc` or `dealloc`. Outside the window covers an inverted flag, a flag never cleared, or a
  process-global flag. Nothing else exercises the wrapper, and a wrapper that never counts would
  pass the browser gate silently.
- The five `needs_drop` const assertions: they turn red, as compile errors, when one of the thread
  locals goes back to a type that needs drop (for example `LIVE_HOST` back to
  `RefCell<Option<LiveHost>>`). On today's non-atomic build nothing else can see this.
- Gate 8 phase 1 (`tests/render_locked_staging.rs`): it turns red when a booted instance's
  response or observation staging is still allocated lazily inside a locked window (7 or 6
  calls). It does not cover the spectrum staging (boot touches it) or the hop boot export
  (MINOR 1).
- Gate 8 phase 2: it turns red when a spectrum or spectrum-stream read on a collection capture
  allocates (the A1 clone gives 4). It is the only native test that does, and the only check that
  runs in cargo CI.
- `host-core::spectrum::tests::selected_channels_reports_the_selected_entrys_mask`: it turns red
  when the accessor reports a mask while no entry is selected. The wrong-entry case is already
  caught by `collection_selection_is_atomic_and_keeps_one_active_capture` (MINOR 2).
- Callgraph self-test cases (h1), (h1a), (h1b), (h1c), (h1d), (h2), (h3) and (h4): each turns red
  when its own rule is disabled. The rules are: compare indirect sites, refuse an unpinned
  export, remove each hash scheme, detect thread-local destructor registration, detect atomic
  waits, and resolve an exact export name first. No other rule examines indirect calls, these std
  paths, or atomic waits inside a closure.
- Browser gate `render-allocations` and its mutation: it turns red for an allocator call in any
  render-thread export, including behind the executor's `call_indirect`. The planted
  `Vec::with_capacity(1)` gives 2 to 260 calls per instance. The static gate cannot see past the
  indirect call.
- Browser gate `staging-reads` and its mutation: it turns red when the staging-read workload stops
  completing a real read. Without it, the count could cover nothing on those paths, and no other
  raw gate reads them.

## Gates run (all on the b55968d0d export)

1. `cargo test --locked -p host-web --lib render_lock`: pass.
2. `bash scripts/test-web-audioworklet.sh` (private TMPDIR, empty afterwards): pass.
   `check-web-audioworklet-callgraph.py --self-test`: pass.
3. `build-web-audioworklet.sh --named-twin`: pass. The shipped module is `1c8782cd...`.
   `check-web-audioworklet.sh <out> <twin>`: pass. It reports render indirect_sites=2,
   meter_poll 0 and command_submit 13 in 5 members, with 0 destructor registrations and 0 atomic
   waits; the render closure is still 8. `check-browser-expected-resources.py --artifacts`: pass.
4. `npm run qualify -- --artifacts <out> --sdk-root <tree>/sdk --browser <b> --check-matrix
   --self-test-mutations` (node_modules from `npm ci`, no stray `sdk/dist`): pass in all three
   browsers. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 each print
   `render-allocations: corpus-0-0=0 ... staging-reads-stream=0`, 10 rows, all 0.
5. Scratch build with `Vec::with_capacity(1)` (through `black_box`) in `render_next`, Chromium:
   the `render-allocations` gate fails with corpus 2/4/2/4, live-control 260, observation 32/32,
   stall 80 and staging-reads 32/32. This equals the attempt record.
6. `check-sdk-generated.sh <out>` and `parameter-metadata -- --check <out>`: pass.
7. `cargo fmt --all -- --check`,
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
   `check-workspace-policy.sh`, `check-realtime-policy.sh` and `test-realtime-policy.sh`: pass.
8. `cargo test --locked -p host-web --test render_locked_staging`: pass.

Also run, all passing:
- `cargo test --locked -p host-web -p host-core`, and again with `--all-features` (480 passed).
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- `check-cross-targets.sh`: PASS.
- `check-sdk-types.sh`, `check-sdk-headless.sh <out>`, `sdk-package.sh check <out>` and
  `check-sdk-deletions.py`.
- `check-abi-layout-v1.py --self-test`, `check-test-support-ci.py`,
  `check-script-reachability.py` and `check-ci-path-routing.py`.

AArch64 was not run, as instructed.

## Mutation re-runs (applied, observed, reverted)

- `render_lock`: an inverted flag, no `fetch_add`, `dealloc`, `realloc`, `alloc_zeroed` or `alloc`
  left uncounted, a flag never cleared, a process-global flag: all red, each on its own
  assertion.
- `needs_drop`: `LIVE_HOST` back to `RefCell<Option<LiveHost>>` gives
  `E0080 assertion failed: !needs_drop::<LiveHostSlot>()`. A needs-drop staging type trips the
  assertion the same way.
- Gate 8: without the response reservation it reads 7; without the observation reservation, 6;
  without `reserved_after_boot` in `boot`, 13; with the A1 clone restored, 4 (phase 2). All red.
  Without the spectrum reservation it stays green, as the record says. Without the reservation in
  the hop export it stays **green** (MINOR 1).
- A1 clone against all host-web tests: only `render_locked_staging` goes red.
- `selected_channels` reading `captures[0]`: the new unit test goes red, and so does the existing
  `collection_selection_is_atomic_and_keeps_one_active_capture`; gate 8 stays green. With "Some
  while unselected", only the new unit test goes red.
- Callgraph: (h2), (h3), (h1)/(h1a)/(h1b), (h1c), (h1d) (both schemes, and each scheme alone) and
  (h4) each go red with only their own cases.
- Browser, Chromium, module rebuilt with the A1 clone (no `--sdk-root`): `render-allocations`
  fails with `staging-reads-one-shot=4`, `staging-reads-stream=2`, every other instance 0.
  `--self-test-mutations` proves `render-allocations` and `staging-reads` red in all three
  browsers.
