# Adversary round 2: answers to root's round-2 reply

Root accepted round-1 items 3, 4, 6, 7, 8, 9 and 10, decided OQ7 (parallel coordinators in
separate worktrees with sequenced merges; AGENTS.md amended in the same PR), and decided B4 as
(a) fix the limit cycle, (b) engine-wide -144 dB relative tail contract, (c) #1261/#1262 report the
bounded tail, never `Infinite`. This round answers Q-C2, Q-C4, B4(a) and Q-scope.

## Q-C2. Latency growth: off-thread catch-up ("warm successor") — feasible, and better than R

**Root's literal idea (fill the grown lines from history) is still impossible**, for the round-1
reason: the samples a grown line needs are not past samples. After growth `Δ`, keeping the output
continuous *and* aligned requires every unchanged path's output for the **next** `Δ` samples
(`y[s-c] … y[s-c+Δ)` measured on the old mapping), i.e. the graph must have processed `Δ` samples
of **future** source PCM through every path's DSP. History never contains them.

**But root's two enablers — the source rings' data and an off-render-thread computation — make
an exact, spike-free design possible.** It is #1287's pre-roll, executed on the control thread
instead of the render thread:

1. **Prepare** the successor off-thread as today (floors at predecessor + `P`, `P = k·quantum ≥ Δ`,
   #1287's proof obligation unchanged).
2. **Snapshot at block B.** The successor is sent to the render thread with "copy, do not adopt".
   At block `B` the render thread runs the carry program in **copy mode** (lanes through the
   allocation-free payload calls of #1278; per-node instances and delay/compensation lines by
   `memcpy` into the successor's preallocated storage) and returns the successor through a bounded
   return queue (capacity 1). Render cost: one state copy at memory bandwidth, bounded and known
   when the program is built (#1286 measures it) — not `k` blocks of DSP.
3. **Catch up off-thread.** The control thread (C ABI: inside the structural submit; browser: the
   Worker of C4) renders the successor from graph time `B`, discarding output, with the canonical
   FP environment pinned (`crates/lane/src/fpenv.rs`, as every native render entry does), reading
   sources through a **read-only peek cursor** on each ring. The ring's release index becomes
   `min(render read, peek cursor)` while a catch-up is active, so the producer cannot overwrite a
   frame the peek still needs; render pays one atomic load per block. No render-side copy, no
   second consumer, no history window to size.
4. **Exact-sample adoption.** When the control thread's graph time `G` exceeds the render clock
   `h` by `P` plus a margin, it fixes `S = G − P` and publishes the successor with "adopt exactly at
   block `S`, else return it". At `S` the render thread adopts by pointer swap, moves each source
   consumer into the successor with its read index set to `S + P` (frames `[S, S+P)` were consumed
   by the catch-up), and retires the predecessor. If render is already past `S`, the candidate is
   returned and the control thread continues and retries. Bit-exact: the same code on the same
   inputs from the same state.
5. **Live edits during the window** are held on the control thread and written to the successor's
   cells at publication, so they apply at `S` (delayed by the window, never lost; the B3 watermark
   reports the sample). **A structural edit during the window** supersedes the catch-up (C5's CAS
   supersession) and restarts it from a new `B`.
6. **Bounds and fallbacks.** The accumulated read-ahead `ΣP` is bounded by the rings' headroom (a
   session bound `P_max`, sized with the ring capacity); floors and `ΣP` reset at a host-declared
   discontinuity (stop, global seek). A catch-up that cannot finish within a timeout (overloaded
   device), a host that is not rendering (paused without declaring stop), or a non-isolated
   browser (no Worker) falls back to render-thread pre-roll bounded by `k_max`, then to the
   documented transition, each counted and reported in B3's path field. The paused-and-declared
   case needs no continuity at all.

**Why it wins:** exact for every unchanged path, the edited path too (a new latent effect
processes real signal from `B`, so its lookahead is full at `S`), no permanent latency, and render
cost is one bounded state copy. It also needs no new idea for submix-internal growth (the whole
successor runs ahead by `P`). With D1-a (anchor on the source-read clock) hosts never see `P`.

**Costs:** copy-mode carry (a mode flag on the carry program; #1282 and #1284 currently specify
`mem::swap` moves, so their specs must require a copy mode too), the ring peek cursor and gated
release (`crates/source`), the render-to-control return queue and exact-sample adoption
(`crates/engine/src/realtime/plan_exchange.rs`), and an off-thread executor for a plan. All are
portable core code, one shape on both hosts.

**R is rejected** (root's objection holds: a permanent cost for a case the catch-up covers
exactly). **Render-thread spread pre-roll is rejected** as the primary: it is the P6 dual-render
cost on the render thread. Both survive only as the counted fallback above.

**Duck-swap (round-1 C5) stays** for an edited strip whose *processing* changes (a changed prepared
value, an insert added/removed/reordered, quality, link mode, `delay_samples`): warm state removes
the at-rest artefacts, but switching between two different chains is still a step of
`new − old` at `S`. The catch-up and duck-swap compose: the duck ramp runs on the predecessor before
`S`, the fade-in on the successor after `S`.

## Q-C4. Toolchain and the allocation gate

**Pick: a pinned dated nightly with `-Zbuild-std` on `wasm32-unknown-unknown`, for the browser
artifact only; everything else stays on stable 1.97.1.** A stable route exists and was built and
run, but it is stable only in its flags. Evidence (scratch builds and Playwright runs in Chromium
151, Firefox 153, WebKit 26.5; repo untouched):

- **`wasm32-wasip1-threads` on stable works for rendering:** the real `host-web` built with
  `--locked`, imported shared memory with a max, needed seven WASI imports (a ~20-line shim), and
  the worklet's digest over 350 blocks of `console-sixty-four-track-app` equalled the shipped
  module's in all three browsers while the worker booted and disposed 28-206 64-track sessions.
- **But the second instance must drive wasi-libc internals by hand:** no reactor `_initialize`
  (its once-guard lives in shared memory and traps on a second call); every instance must call the
  internal `__wasi_init_tp`, without which even the first instance spins at 100 % in
  `__pthread_key_delete`; even with it, the second instance's first drop-carrying thread_local
  wrote through a null pthread-specific pointer into instance A's stack region. Every export is
  wrapped with `__wasm_call_dtors`, which breaks the callgraph gate's export naming. Render's direct
  closure reaches `__wasilibc_futex_wait`, and **Chromium traps any wait in an AudioWorklet**. The
  platform doc names only Wasmtime/WAMR and the wasi-threads proposal is marked legacy.
- **Nightly build-std** needs no libc, no shim and no `__wasi_init_tp`; it passed the same harness.
  The repo already pins `nightly-2026-08-20` for fuzzing. wasm-bindgen documents the same recipe for
  browser threads.
- **Bit identity:** PCM digests for 5 fixtures × 370 blocks were identical across the shipped
  module, wasip1-threads and nightly build-std (LLVM 22 vs 23), because the engine uses its own
  `crates/math`. The repo's native and AArch64 parity gates were not run; H(a) must run them.
- **Size:** shipped 2,894,202 B; wasip1-threads +33 KB; nightly build-std −5.6 KB.
- **Other stable options fail:** `+atomics` with the prebuilt std refuses to link
  (`--shared-memory is disallowed … not compiled with 'atomics'`); `#![no_std]` is not viable (20 of
  23 crates in host-web's closure use std, plus json-syntax, indexmap and ahash).
- **Policy cost:** `rust-toolchain.toml`, `qualification.yml` and `npm-publish.yml`'s
  `RUSTUP_TOOLCHAIN` (artifact identity, `docs/RELEASE.md:14,129`) gain an explicit
  browser-artifact toolchain entry; a bump re-records the three-browser matrix, as a stable bump
  already does (#877).

**The allocation rule is enforceable, but not by the current gate.**

- **The current static gate is nearly blind:** render's direct closure is 8 functions; the whole
  executor sits behind `Box<dyn PreparedPlanExecutor>` (`crates/engine/src/realtime/plan.rs:550,941`)
  as a `call_indirect`, which `check-web-audioworklet-callgraph.py` never follows (`:96`,
  `:294-313`). Resolving indirect calls by signature flags 59 forbidden names (mostly drop glue); a
  vtable heuristic is unsound. This gap exists today, independent of threads.
- **Gate set:**
  1. **Runtime proof:** a `GlobalAlloc` wrapper reading an instance-local, const, drop-free
     render-locked flag (one load and branch per allocator call). Browser qualification asserts
     exactly 0 after a workload with swaps, commands and meters, plus a mutation self-test (measured:
     0 after lock in three browsers; a planted allocation reads 2; trap mode raises `unreachable`).
  2. **Static:** keep the direct-call gate, and fail on thread_local destructor registration
     (`destructors…register`, `guard…enable`) and on any `memory.atomic.wait` in a worklet export's
     closure. Pin the set of `call_indirect` sites on the render path so new dynamic dispatch is
     reviewed. One real finding already: render-reachable `LIVE_HOST` / `BOOT_STAGING`
     (`hosts/host-web/src/ffi.rs:503-512`) register a destructor lazily, which allocates. They must
     become const-initialised with no destructor.
  3. The native allocation counters stay.
- On build-std, std-internal `System` allocations bypass `GlobalAlloc`, so check 2 covers them.

## B4(a). The limit-cycle fix

**A correct fix exists: a joint state flush inside `svf_step`, `REST_EPS = 1e-14`, keeping the
per-word 1e-20 flush.** One recurrence serves the input HPF/LPF, the parametric EQ and the
multiband LR4 crossover. Evidence: a scratch copy of `lane::svf_step` that matched the real kernel
bit for bit on 200k random steps, then scanned under the candidate laws (scratch
`tailsim/src/bin/fix.rs`; no repository change).

- **Law.** After the new words `n1, n2`: `rest = (|n1| < τ) & (|n2| < τ)`; each word is zeroed when
  `|word| < 1e-20` **or** `rest`. Two compares, not `Lane::max` (`crates/lane/src/lib.rs:358-360`
  is `select(gt)` and would drop a NaN, hiding it from the per-block check). +5 lane ops per
  section-sample, branch-free, one body at every width.
- **Why τ = 1e-14 is provably enough.** The per-word flush perturbs the state by ≤1e-20 per word
  per step, so a trajectory can only stall inside a ball of radius `κ·√2·1e-20 / (1 − ρ − 6εκ)`.
  Largest radius per domain: input filter 6.66e-16 (maximum cutoff), EQ 3.385e-15 (bell 10 Hz,
  Q 18, +24 dB, 96 kHz), LR4 9.2e-18. τ = 1e-14 clears all three with ≥3x margin; τ = 1e-17 still
  left 566-614 cycles per rate.
- **Scans, all four rates** (amplitudes 0.37 … 1.6e31): input filter 566-614 never-resting runs per
  rate → 0; **a second trap in the EQ** (low shelf 10 Hz +24 dB / high shelf −24 dB at 96 kHz sticks
  at `ic1 = 0, ic2 = 6.01e-20`, output −361 dBFS forever) → 0; LR4 slowest rest 517k → 28k
  samples; top-band ramps 40/40 never rest → all rest within 616k samples.
- **Exact-rest bound with the fix** (worst-case drive; analytic bound agrees): input section, peak
  +24 dBFS: ≤934,193 samples (44.1/88.2 kHz), ≤929,225 (48/96 kHz); any sanitized input: ≤2.27M.
  Contract figures: exact rest within 1.0M samples for peaks ≤ +24 dBFS, 2.4M for any sanitized
  input; the −144 dB relative tail stays 383,571 / 381,428. EQ worst case (bell 10 Hz, Q 18,
  +24 dB): ~89 s at every rate for a +24 dBFS peak — the tail contract must compute the EQ's −144 dB
  tail too (not yet done).
- **Bits moved (class B, owner-delegated and accepted by root):** 240 noise-then-silence runs at
  0 … −180 dBFS: no sample moved while input was non-silent; in tails the first moved sample is at
  ≤ −204.8 dBFS and the largest change 2.2e-13 (−253 dBFS). Corpus digests whose tails reach that
  range move: re-baseline the single cross-target corpus owner, recorded.
- **Cost:** AVX2 scratch, descriptive: one isolated section +10 % (latency-bound), four
  interleaved sections (production shape) no measurable change. A per-block check would be cheaper
  but breaks gate P1 (`crates/lane/tests/p1_partition.rs:1-6`: block boundaries are never numeric
  events), so the per-sample law it is.
- **Ramps in flight are safe** (the flush only zeroes state; the step matrix stays non-expansive).
- **Rejected:** dropping the per-word flush (1,876/6,400 DC runs left subnormal words; loses the
  `g6_ftz_inert.rs` law); capping the cutoff domain (empirical only, leaves 48.3M-sample rests,
  changes a public domain, misses the EQ trap); "flush ic1 when ic2 flushes" (zeroed a −24 dBFS
  partner word twice in 64M steps — a click); a rest detector on exact-zero input (optional, not
  needed); magnitude truncation (moves audible bits).
- **Where:** `crates/lane/src/kernels.rs:649-650` (`svf_step`; the crossover reaches it via
  `crates/multiband-compressor/src/lib.rs:504-506`), a `flush_pair` beside `flush`
  (`crates/lane/src/lib.rs:163-188`), frozen-order docs (`kernels.rs:111-124`, `:636-641`), the oracle
  twin (`crates/dsp-reference/src/tpt.rs:17-52`, `:166`), `crates/lane/tests/g4_flush.rs`,
  `docs/BUILTINS_AND_METERING_V1.md:55-58`. Unchanged: the EQ restore predicate, `section_is_identity`,
  the delay's one-pole flush.
- **Regression tests, both red on revert:** 44.1 kHz LPF at 22049.482 Hz, impulse 1234.5, must
  reach `(0, 0)` (today: period-2 cycle from sample 858,748); EQ low shelf 10 Hz +24 dB S 0.1 at
  96 kHz, impulse 1, must reach rest (today: stuck at `ic2 = 6.01e-20`).
- **Consequence for skip-on-silence:** an enabled section now reaches exact rest within a stated
  bound, so silence skipping (#1107 and successors) can rely on exact rest.

## Q-scope

**Yes, run the whole program now, with a cap.** Every item is either a correctness gap the owner
principle forbids deferring or a dependency of one. Two limits keep it safe: at most five
concurrent implementation coordinators (verification and merge capacity, not agent capacity, is the
bottleneck; every past batch's final full-gate run caught a blocker), and file ownership with named
merge orders for the five hot files. The stream table, dependencies, parallel-safety and the S0
list are in `PLAN-2026-10-05-agreed.md` §2-§3.
