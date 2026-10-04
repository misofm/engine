# #1053 live updates: verdict records

The adversarial verdicts for the #1053 umbrella batch (live value edits on the running C ABI plan),
copied from the batch's scratch area so they survive it. Each slice passed on its first attempt.
The two `*-verifier-scratch.rs.txt` files are the verifiers' uncommitted probe tests, kept as text
so cargo never compiles them; the committed follow-up tests were shaped from them.

| issue | slice | attempt commit | attempts | verdict | follow-ups |
|---|---|---|---|---|---|
| #1253 | bound the builtin fader and matrix drains | `b5489f00d` | 1 | PASS ([verdict](1253-attempt1.md)) | `2142efb98` |
| #1254 | host-core attaches fader and matrix lanes alone | `7436a64f9` | 1 | PASS ([verdict](1254-attempt1.md)) | `2142efb98` |
| #1255 | classify a committed delta as live or a rebuild | `a2a417c9a` | 1 | PASS ([verdict](1255-attempt1.md)) | `23c3048da` |
| #1256 | C ABI plans carry fader and matrix lanes | `f02d09b24` | 1 | PASS ([verdict](1256-attempt1.md)) | `b6a91ef63`, MINOR 1 in #1258 |
| #1257 | live fader, mute and pan on the C ABI | `ef5ac335c` | 1 | PASS ([verdict](1257-attempt1.md)) | `978463341`, gates `0539791a1` |
| #1258 | qualify live edits against a rendering plan | `0fdb4f886` | 1 | PASS ([verdict](1258-attempt1.md)) | `fe3bb37e8` |
| #1260 | model-only commits without a rebuild | `31b53b62c` | 1 | PASS ([verdict](1260-attempt1.md)) | `fe3bb37e8` |
| #1263 | C ABI plans carry effect lanes | `986301820` | 1 | PASS ([verdict](1263-attempt1.md)) | final batch follow-ups |
| #1264 | live effect parameters on the C ABI | `fbb311dfe` | 1 | PASS ([verdict](1264-attempt1.md), [scratch](1264-attempt1-verifier-scratch.rs.txt)) | final batch follow-ups |
| #1265 | live parametric EQ parameters through prepared targets | `b055a48d4` | 1 | PASS ([verdict](1265-attempt1.md)) | final batch follow-ups |
| #1266 | live effect bypass on the C ABI | `5c8de7880` | 1 | PASS ([verdict](1266-attempt1.md), [scratch](1266-attempt1-verifier-scratch.rs.txt)) | final batch follow-ups |
| #1261 | live input trim and polarity | none | 0 | skipped: waits for owner Q4 | none |
| #1262 | live input HPF and LPF | none | 0 | blocked: depends only on #1261 | none |

Each slice spec's "Attempt record" in `.github/ISSUE_SPECS/` holds the evidence, the mutation runs,
the verdict line and the follow-ups applied.

## Open owner questions

Q1-Q4 are the umbrella's (`.github/ISSUE_SPECS/1053-*.md`, "Owner questions"). The batch used each
recommended default.

- **Q1. Steps first?** Live fader and mute changes ship as steps until #1054 gives them ramps.
  Recommended: ship steps first.
- **Q2. A paused transport.** Each strip lane takes 16 live edits without render calls; the 17th is
  `BACKPRESSURE`. Recommended: accept it.
- **Q3. Tell the host which path ran?** A "rebuilt" flag would need a protocol change.
  Recommended: not now.
- **Q4. An infinite tail for live input filters?** A live input lane makes every C ABI plan report
  `TAIL_INFINITE`. #1261 (and so #1262) waits for this answer. Recommended: yes.
- **Q5 (new, an FYI from the #1263 verdict). C ABI live effect windows scale with the caller's
  `maximum_automation_spans_per_block` (S).** Each effect bank's live-control window costs
  8,944 + 360 * S bytes at eight lanes and 4,560 + 200 * S at four, though one drain of a 16-deep
  lane stages at most 16 spans. Bounding the windows by the lane depth would cut the nine-track
  reference session's graph move from 128,984 bytes to about 48 KB, and much more at a large S.
  This is a mobile memory question, not a defect.

## Follow-up candidates (recorded, not implemented)

- **#1253 N1.** The realtime-policy unbounded `try_pop` rule is a single-line regex. Replace it
  with a structural rule (a `try_pop(` in a marked body needs `available_at_entry` in the same
  region), with mutation cases, or narrow umbrella D11's wording.
- **#1255 N1.** Route builtins' `prepare_sections` and builtins-compiler's `gain_path` through
  `checked_fader_gain`, so the `[-144, 24]` fader domain is spelled once.
- **#1263 owner FYI (Q5).** Bound the C ABI's live effect windows by the lane depth.
- **#1263 N2.** The derived four-lane graph ceilings are looser than 10 %; tighten them from the
  rows the budget test prints on the first AArch64 CI run.
- **#1264 N2 and N3.** Control-thread cost: skip effect lowering when every `params` is bit-equal,
  and replace the per-record linear catalog and producer scans. Weekly performance pass.
