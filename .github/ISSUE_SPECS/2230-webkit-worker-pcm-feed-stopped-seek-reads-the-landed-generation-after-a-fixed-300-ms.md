# WebKit worker: pcm-feed-stopped-seek reads the landed generation after a fixed 300 ms

Filed by root from the #1713 build (2026-11-01). Seen once; the rerun passed. Not caused by #1713 (route code), per its verify.

## Problem
The browser qualification gate `pcm-feed-stopped-seek` failed once on the WebKit `worker` leg with "did not land seek 5". The check runs inside the suspended probe (`sdk/.../pcm-feed-suspended-entry.ts:299`): after `play()` it waits a fixed `wait(300)` and then reads the landed generation. That is a wall-clock assumption on the #2205 seek path (60efcf8b4). On a slow WebKit run the generation can land after 300 ms, so the gate reads too early.

## Scope
Make the check wait for the condition, not for a fixed time: poll the landed generation (or await the SDK's own landing signal) with a bounded deadline that names the reason when it expires. Keep the gate's claim (the stopped seek lands generation 5 at the target, with 0 underrun frames). Chromium and WebKit only.

## Gate
- The check passes 20 consecutive WebKit worker runs on the CI shape, with no rerun.
- It is red when the seek never lands (the deadline expires with the named reason).

## Evidence (attempt 1, 2026-11-02)

Branch `codex/issue-2230-seek-wait`, from `d201e3082`.

- **No landing signal in the SDK.** `engine.seek` resolves at accept, and `position()` reports an
  unlanded seek as in effect. So the row polls the engine's own seek report.
- **The change** (`hosts/host-web/qualification/pcm-feed-suspended-entry.ts`). A new helper,
  `untilLanded`, reads every 10 ms until the seek has landed and at least 300 ms (the old fixed
  window) have passed. Its deadline is 4 s. At the deadline it throws
  `the source's seek <g> after play() did not land within <ms> ms (deadline 4000 ms): source generation <g'>`.
  The underrun frames are read in the same report that sees the landing, so the claim stays: seek
  `g` lands at the resume sample, with 0 underrun frames (gated on Chromium, as before). The
  timeline is read once, after the source landed; it lands in the same block, so a split still
  fails `run.mjs`.
- **Fixed waits in this file:**
  - stopped seek, `wait(300)` before the landed read: changed (this issue).
  - transport seek, `wait(300)` before the landed read: changed. It had the same flaw and the same
    `did not land` throw.
  - plain ring seek, `wait(300)`: changed. `run.mjs` gates `seeksApplied === 5`, read after the last
    fixed wait. The row now polls the ring's `SEEKS_APPLIED` count for each seek.
  - `wait(300)` after resume and `wait(500)` for the baseline: kept. They are measurement windows,
    not reads of an async result.
  - stopped seek, `wait(50)` before the resume-sample read: kept. `stop()` has suspended the context
    when it resolves, and no signal tells when the last block's status is published. An early read
    gives a resume sample before the landing sample, so the Chromium gate goes red, not green.
  - `pcm-feed-suspended.mjs` has only bounded 30 s row deadlines. No change.
- **Red.** A scratch mutation (the stopped row waits for `generation + 100n`, a seek that never
  lands) made `--browser webkit --leg single --pcm-feed-suspended-only` exit 1 after about 32 s:
  `webkit: pcm-feed-stopped-seek worker: ... the source's seek 2 after play() did not land within 4010 ms (deadline 4000 ms): source generation 2`
  (and the same for `single`). A producer that never publishes is not a red: a stopped seek lands
  anyway, with underruns (16,128 frames on WebKit, not gated there). The suite's
  `--self-test-mutations` does not cover this file, so no mutation was added.
- **Stability** on this host, with a private PulseAudio null sink as CI has it. The suspended feed
  rows run only in the `single` leg (`run.mjs`); each run has both instance modes and all three
  phases. `--browser webkit --leg single --pcm-feed-suspended-only`: 20 of 20 passed, no rerun
  (100 WebKit `worker` stopped-seek rows, all 0 underrun frames).
  `--browser chromium ...`: 5 of 5 passed.
- **Checks.** The five SDK qualify commands of `qualification.yml` passed;
  `bash scripts/check-workspace-policy.sh` passed; one `--browser all --leg all --check-matrix --self-test-mutations` run passed all four legs (Chromium and WebKit, single and worker).
