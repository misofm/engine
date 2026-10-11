# Remove producer and fan roles from the engine

**Product outcome.** No live engine document, open issue, ruling reading or code comment describes a
producer, fan, listener or personal mix as an engine concept. Every live text names a generic role (a C
ABI host, an iOS or Android app, an app, a plan booted without live controls). The engine's behaviour,
API, ABI, session grammar and rendered bits do not change. The personal-mix owner question is closed as
app-layer in the rulings that list it as open.

**Owner direction (2026-10-11).** "It's crazy to have 'producer' and 'fan' concepts in a generic audio
engine. That makes no sense. This should be something we do on the application layer."

**Context.** The roles exist only in prose and in two test identifiers (`hosts/host-web/tests/stem-store-core-v1.mjs`: `fanMixNarrative` and the tab ID `"fan"`, renamed to `twoMixNarrative` and `"two-mix"`; amendment 1 after attempt 1's verdict): 3 rulings, 9 open specs, `docs/TARGET_MATRIX.md` and one
`//!` comment in `tools/bench/src/console.rs`. No type, field, mode, boot option, gate or test is named
for a role. The generic capabilities the prose motivated stay: a plan prepared without live controls
(browser boot option `liveControls.commandQueueRecords` = 0), AArch64 as an official target with
cross-target bit identity, the core-owned committed session with canonical JSON snapshots. #1057 and
#1058 are closed; decision 16 was not adopted and was never filed.

**Decisions frozen.**
- D1. Handoffs under `docs/handoffs/`, dated audits under `docs/audits/` and closed verdicts are records and are not edited.
- D2. Rulings are not rewritten. Each of the three rulings gets one dated note (text below); owner quotes stay verbatim.
- D3. Open specs and current docs are reworded in place with the exact replacements in the table below. Each
  edited open spec's GitHub body is synced in the same batch.
- D4. The personal-mix question is closed as "not an engine question". The generic
  "live edit versus stored automation on one control" question (#1058 OQ1) stays open, without roles,
  and is not decided here.
- D5. No code, ABI, SDK, session schema or behaviour change. W-11 is a comment edit only.
- D6. The decision-16 branch is out of scope; root decides its disposal separately (D-4).

**Authorized paths (exact).**
- `docs/rulings/engine-footprint-2026-09-28.md` (append notes only)
- `docs/rulings/live-update-versus-rebuild-2026-10-04.md` (append note only)
- `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (append note only)
- `docs/TARGET_MATRIX.md` (lines 100-101 only)
- `tools/bench/src/console.rs` (lines 168-169, the `//!` comment only)
- `.github/ISSUE_SPECS/` files for #1019, #1113, #1225, #1239, #1247, #1277, #1326, #1327, #1390 (the
  quoted line only; in any order with the stream branches that also edit #1277 and #1326: no shared hunks)
- `hosts/host-web/tests/stem-store-core-v1.mjs` (the two identifiers only; no behaviour change)
- `AGENTS.md` (the one Mission sentence only)
- this issue's own spec file

**Slices.** One slice; it is prose only and fits well under half a day. It includes one sentence
added to the Mission paragraph of `AGENTS.md`, under the owner's direction above: "The engine has no
notion of person roles such as producer, fan or listener; apps own them."

**Gates (no committed test that greps prose).**
- G1. `scripts/check-workspace-policy.sh` passes, and `cargo check -p bench` and `cargo fmt --check -p
  bench` pass (the comment edit compiles and is formatted).
- G2. `git diff --stat origin/main` touches only the authorized paths, and the diff of each ruling is
  additions only (no removed line).
- G3. For each of the nine issues, `gh issue view <n> --json body` equals the local spec after the
  batch push (review evidence, not a committed test).
- G4. Review evidence, not a committed test: the verifier re-runs `git grep -n -i -E '\\b(fans?|producers?|listeners?|personal mix)\\b'` (ignoring queue producer/consumer and fan-in/fan-out senses) on the merged
  tree; every remaining person-role hit is under `docs/handoffs/` or `docs/audits/`, in this issue's own spec (which quotes the old text), or inside a ruling's original text
  that a dated note covers.
- No benchmark, fixture or digest changes; no rendered bit can move (no code change).

**Non-goals.** Rewording handoffs; changing the live-control opt-in default; deciding #1058 OQ1 or OQ2;
merging, salvaging or deleting `codex/d15-decision-16`; editing the auto-memory files outside the repo.

## Rewordings (exact)

| # | file:line | text | proposed action |
|---|---|---|---|
| W-1 | `.github/ISSUE_SPECS/1019-make-the-compressor-s-aarch64-release-build-bit-identical-to-the-browser-lane-3.md:7` | "So a fan's phone renders a compressor mix whose bits differ from the producer's browser mix, and gate G1 is red there" | "So an iOS or Android app renders a compressor mix whose bits differ from the browser's render of the same session, and gate G1 is red there". Body sync #1019. |
| W-2 | `.github/ISSUE_SPECS/1113-avx512-desktop-build.md:34-35` | "so a producer and a listener on different builds hear the same bits" | "so two hosts on different builds render the same bits". Body sync #1113. |
| W-3 | `.github/ISSUE_SPECS/1225-deliver-value-only-send-edits-to-the-running-c-abi-plan.md:14` | "A C ABI host, such as a fan's phone, can change a send's ..." | "A C ABI host, such as an iOS or Android app, can change ...". Body sync #1225. |
| W-4 | `.github/ISSUE_SPECS/1239-vca-groups.md:25-26` | "... saved in the canonical session, so a fan's phone, the next agent and the producer's mixer see the same intent." | "... so every host and every agent that loads the session sees the same intent." Body sync #1239. |
| W-5 | `.github/ISSUE_SPECS/1247-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md:16` | "A C ABI host, such as a fan's personal mix on a phone, can make all of these VCA edits" | "A C ABI host, such as an iOS or Android app, can make ...". Body sync #1247. |
| W-6 | `.github/ISSUE_SPECS/1277-carry-fader-mute-and-pan-ramps-across-a-plan-swap.md:11` | "A producer who pulls a fader while a track is added hears the ramp go on, not jump." | "A fader move made while a track is added keeps ramping; it does not jump." Body sync #1277. **Edited on `codex/d15-stream-b2`** (hunks at :68+, not :11). |
| W-7 | `.github/ISSUE_SPECS/1326-give-every-browser-plan-live-strip-fader-and-mute-lanes.md:29` | "so fan playback has no live mute." | "so a plan booted without live controls has no live mute." Body sync #1326. **Edited on `codex/d15-batch-dfg`** (hunk at :132+); if that batch closes #1326, only the GitHub body matters. |
| W-8 | `.github/ISSUE_SPECS/1327-carry-meter-and-effect-observation-state-across-a-plan-swap.md:12` | "A producer watching a strip's level or an effect's gain reduction sees no reset" | "An app that reads a strip's level or an effect's gain reduction sees no reset". Body sync #1327. |
| W-9 | `.github/ISSUE_SPECS/1390-deliver-value-only-submix-strip-fader-mute-and-pan-edits-to-the-running-c-abi-pl.md:14` | "A C ABI host, such as a fan's phone, can change a bus's ..." | "A C ABI host, such as an iOS or Android app, can change ...". Body sync #1390. |
| W-10 | `docs/TARGET_MATRIX.md:100-101` | "Fans open and play sessions in iOS and Android apps, which embed the engine through the C ABI." (outside the owner quote on :99) | "iOS and Android apps embed the engine through the C ABI." (matches AGENTS.md Mission) |
| W-11 | `tools/bench/src/console.rs:168-169` (`//!` doc comment) | "the plan the C ABI prepares for fan playback, less the fader and matrix lanes the C ABI now attaches (#1256)" | "a plan prepared without live controls: the C ABI's plan less the fader and matrix lanes it attaches (#1256)". Comment only; no behaviour. `codex/d15-stream-j3` adds a frozen copy at `tools/realtime-policy/tests/fixtures/base/tools/bench/src/console.rs.txt` that does not contain this line, so no conflict. |

## Ruling notes

Append this note (adapted per file) to each ruling; do not edit the existing text.

| # | file:line | text | proposed note |
|---|---|---|---|
| R-1 | `docs/rulings/engine-footprint-2026-09-28.md:9-12` | Owner quote "The production/mixer UI will be primarily in a browser, but fans still need to use the engine ..." + "Producers mix in the browser; fans open and play sessions ..." | One note after "Product scope" and one under decision 2 (below). |
| R-1b | `docs/rulings/engine-footprint-2026-09-28.md:52` | "A producer's automation is rendered by the core engine from the session file, identically on every platform" | covered by the same note: read as "a session's stored automation". |
| R-1c | `docs/rulings/engine-footprint-2026-09-28.md:56-57` | "A fan's edits are saved as a personal mix; the producer's original is untouched (#1057 asks ...)" | covered by the decision-2 note: the bullet is withdrawn as engine scope (D-1). |
| R-2 | `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:397-398`, `:568` | "Its personal-mix owner question stays open and blocks nothing here."; "**Still open:** #1057's personal-mix owner question." | Note at the end of the "owner's earlier answers"/"Still open" block (below). |
| R-3 | `docs/rulings/live-update-versus-rebuild-2026-10-04.md:181-183` | "They exist only in a plan prepared with live controls, which are opt-in at boot; a fan's control-free playback keeps every value prepared" | Root's application text, not an owner quote, but a ruling record: add the note rather than edit. |

> **Note, 2026-10-11 (owner direction).** "It's crazy to have 'producer' and 'fan' concepts in a
> generic audio engine. That makes no sense. This should be something we do on the application layer."
> The engine has no producer, fan or listener role. Where this ruling names one, read it as a generic
> host or app: "a plan booted without live controls" for fan playback, "a session's stored automation"
> for a producer's automation, and "an iOS or Android app" for a fan's phone. The personal-mix question
> (#1057, section 7) is not an engine question and is closed: an app keeps its users' edits itself, with
> canonical JSON snapshots and transactions. Decision 16 (fan level offsets, personal mixes) was not
> adopted.
