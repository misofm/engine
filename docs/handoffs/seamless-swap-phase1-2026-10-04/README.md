# Seamless plan swap, phase 1: verdicts

Phase 1 of *Swap a rebuilt plan without an audio gap* (#1269,
`.github/ISSUE_SPECS/1269-swap-a-rebuilt-plan-without-an-audio-gap.md`), built on the batch branch
`codex/seamless-swap` from `41517fc35`. This folder keeps each slice's adversarial verdicts as the
verifier wrote them; the verifiers' scratch logs and exports are not kept. Each slice's evidence
and decision record is its spec's Attempt record; the slices closed with PR #1299, so their specs
left `.github/ISSUE_SPECS/` and stand at
[`d2fe0555a`](https://github.com/misofm/engine/tree/d2fe0555a95cb531ef5b62bf44354301cfc09b4f/.github/ISSUE_SPECS).
Commits are listed from `git log --oneline 41517fc35..` on the batch branch; merge commits are left
out.

| Issue | Slice | Commit(s) | Attempts | Verdict | Verdict files |
|---|---|---|---|---|---|
| #1270 | Hand the outgoing plan to its successor at the swap block | `22c9bd5a1`; follow-ups `46bc191af` | 1 | PASS with minors | `1270-attempt1.md` |
| #1271 | Move a source consumer into a successor graph plan | `5488fb2f5`, `1a911e31f`; follow-ups `bbf4d8626` | 1 | PASS with minors | `1271-attempt1.md` |
| #1272 | Prepare a successor plan whose unchanged sources keep playing | `2a6f2c10c`; follow-ups `bf2b788e8`, `1d40c488d`, the release-shape cfg follow-up | 1 | PASS with minors | `1272-attempt1.md` |
| #1273 | Keep sources playing across a C ABI structural transaction | `448baae85`, `1338b063c`; follow-ups `41ed93566` | 1 | PASS with minors | `1273-attempt1.md` |
| #1274 | Hold an anchored source seek until its render sample | `0297efa8c`; follow-ups `c14fde0ce`, `2330610e6`, `41ed93566` | 1 | PASS with minors; follow-ups PASS with minors | `1274-attempt1.md`, `1274-followups.md` |
| #1275 | Start a newly added C ABI source at an exact render sample | `55690373d`; follow-ups `86ac2f359` | 1 | PASS with minors | `1275-attempt1.md` |
| #1276 | Carry the strip input section across a plan swap | `807b48547`, `f41388939`; `e6302d8a8`; follow-ups `63432c193`, `25954f771` | 2 | FAIL (one MAJOR), then PASS with minors | `1276-attempt1.md`, `1276-attempt2.md` |
| #1071 | Soft-clip refuses its own subnormal snapshot on restore | `1199b53f9`; `983ac85bd`; follow-ups `d6217a79d`, `41ed93566` | 2 | FAIL, then PASS with minors | `1071-attempt1.md`, `1071-attempt2.md` |
| #1278 | Make every banked effect's state restore allocation-free | `c2808bb04`, `46ef4f263`; `a070cfa7d`; `251113c8f`; follow-ups `a3561163a` | 3 | FAIL, FAIL, then PASS (NITs only) | `1278-attempt1.md`, `1278-attempt2.md`, `1278-attempt3.md` |
| #1289 | Measure a session rebuild on the browser's audio thread | `52ac9f88d`, `0b477c585`, `d169ef5f8`; follow-ups `f45c1878f`, the router follow-up | 1 | PASS with minors | `1289-attempt1.md` |

Follow-up commits apply a PASS verdict's minors and NITs; they are recorded in their slice's
Attempt record and have no verdict file of their own, except #1274's (`1274-followups.md`).

`41ed93566` records the #1071, #1273 and #1274 verdicts and follow-ups in their specs, so it is
listed under all three. Batch-level commits outside any one slice: `af48f0830` (this folder and
its verdict copies) and `6ead7b34f` (the #1269 phase 1 status in the umbrella spec). "The router
follow-up" is the commit that adds the rebuild harness's two inputs to the `console-benchmark`
routing key (#1289's Attempt record).
