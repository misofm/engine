# Overnight report, 2026-09-27 to 2026-09-28

Everything below is on the local batch branch `codex/batch-plumbing-floor-2`
(`/home/bl/misofm/worktrees/engine-batch4`). Nothing was pushed. Every merged change passed an
adversarial review and renders bit-identical output (every console digest equals the batch base).
The detailed running log is `scratchpad/overnight-log.md` in the session scratchpad; each issue's
evidence and verdict are in its spec under `.github/ISSUE_SPECS/`.

## Result

Native `Simd8`, microseconds per 64-track block, final clean run (load 4-9, rounds within 1 %):

| workload | start of batch | now | change |
|---|---:|---:|---:|
| full console | 137.8 | 99.7 | -28 % |
| console with meters (browser shape) | 141.6 | 104.0 | -27 % |
| 128-track console | 275.3 | 199.6 | -27 % |
| EQ + compressor | 87.2 | 58.7 | -33 % |
| compressor only | 70.0 | 48.7 | -30 % |
| EQ only | 46.7 | 35.0 | -25 % |
| mixing automation, 8 of 64 mono tracks automated | 129.7 | 75.2 | -42 % |

The start-of-batch metered figure is from the first run that recorded that row. In the browser
(V8, shipped `host_web.wasm`) the effect-level gains are larger than natively; see each issue.

## What landed tonight

* **Compressor.** #995: a sidechained compressor is 61 % faster in the browser, 65 % natively.
  #1006: automating a compressor costs 4-7x less (browser, 8 of 64 mono tracks: +32.6 -> +6.7 us).
* **Limiter.** #1013 and #1014: 3-9 % faster natively and about 9 % in the browser when settings
  are not moving. #996/#997: the linked-pair session test now runs at every width.
* **EQ.** #999 (fold the block check into the last pass), #1005 (skip dead sections while a
  setting moves: browser 8 of 64 stereo +64.7 -> +11.2 us, mono +30.1 -> +3.9 us), #1007 (vector
  lane writes: all 64 automated, a further -12 to -15 us), #1015 (a class-A bug fix, below).
* **Mono and automation.** #1004: editing a compressor or limiter no longer knocks a mono stem off
  its one-channel path (browser mono console, 8 of 64 automated: 292 -> 175 us). #1012 enforces its
  precondition. #1003 and #1011: a new benchmark row that automates 8 of 64 tracks exactly as the
  browser SDK sends edits.
* **Compile.** #1001 (a failed trial regroup keeps the original plan) and #1002 (limiter peak
  memory at 65,537 tracks back from +24 % to +0.1 %).
* **Tooling.** #1000 and #1009: a wasm gate that fails when V8 spills the EQ's hot loops in the
  shipped artifact (the #977 attempt-1 slowdown was invisible to every other gate).

## Bugs found and fixed

* #1015: a restored near-zero EQ filter state made the fast path output `-0.0` where the full path
  outputs `+0.0` (inaudible, restore-only, but a class-A break). Fixed; follow-up test in progress.
* The first prototype of the mono fix played wrong audio for one edit order; the review caught it
  before any code was written, and #1004 uses the exact form.

## Decisions for you

1. **#998 (EQ check cache): held, not merged.** It passes its gates, but saves about 0.5 us on the
   browser one-band EQ (under 0.2 % of the console) while every future EQ change must keep its
   cache fresh, and a miss is silent in release. Reviewer and I recommend dropping it.
2. **Limiter timing tolerance.** #1014 uses a provisional +2 % allowance on its unchanged ramping
   path (it measured -0.4 to +1.3 %). A strict +0 % rule would fail it in 3 of 4 runs.
3. **Target-specific code.** A browser-only limiter detector shape would gain about 2 % more in the
   browser and lose 0-3 % natively. Not filed: it would be the first target-specific loop shape.
4. **Limiter floor accounting.** The research's named reason for the detector gap ("frozen
   summation order") was shown wrong; the reviewer asks you to reject it as the recorded reason.
5. **Compressor C2b** (per-word curve design): exact, worth 1-2 us, left out pending your call.
6. **Batch boundary.** When you want this on `main`: refresh the AudioWorklet artifact pin once,
   run `scripts/run-wasm-gates.sh`, push the batch, and open one PR. The first CI run is also the
   first observation of the V8 spill gate on GitHub's runner.

Carried from earlier in the session, still open: the silence minimal path, ruling 07 (mono track
ids sort first), the EQ floor of 27 ops, per-host floors, a wasm CI run for 4-lane effect banks,
the 29 GB `/home/bl/misofm/audits` folder, and two `/tmp/issue687*` and `/tmp/issue688*`
worktrees with uncommitted changes (left alone).

## Held or queued

* Dual-mono work (#973, #974, #975, #987) is on hold at your request. Before a Codex handoff they
  need the batch on `main`, refreshed line references and a review; #987 needs a design first.
* #972 is deferred by your dual-mono ruling (mono source files only; no content detection).
* Follow-ups filed tonight: #1008 (release test builds collide), #1010 (EQ dual pair V8 registers,
  weekly), and the #1015 boundary test (running).

## Process notes

* All timings ran one at a time under a shared lock, with two exceptions noted in the log: a
  verifier ran one 10-second untimed run outside the lock by mistake (timings discarded).
* Stale worktree cleanup freed about 85 GB at the start of the night (61 worktrees and two idle
  build directories).
