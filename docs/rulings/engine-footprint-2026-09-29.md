# Engine footprint: owner rulings of 2026-09-29

Follow-ups to `engine-footprint-2026-09-28.md`, recorded from the owner's answers on 2026-09-29.

## Decision 9: browser observation paths (#1064)

Decision 8 assumed the browser adapter's "legacy" observation path was unused. #1064's step 1
found the reverse: the SDK, the shipped AudioWorklet and the app boot only the ordinary ("legacy")
path, which carries the track and master meters, resident gain reduction and the session's
spectrum; the "protected" path (#825/#828, built for a live EQ analyzer with its own bounded
ingress) has only test callers. Ruling: **keep the ordinary path and remove the unadopted
protected path** (its boot exports, `PreparedObservationStorage::Protected`, the observation
ingress, the protected dispatch and its tests). #824 (protected one-shot) closes as not planned.
A live EQ analyzer, if it returns, is designed on the ordinary path in its own issue.

## Decision 10: NaN encodings in class-A identity (#1065)

x86 and AArch64 produce different NaN bit patterns (and wasm leaves them unspecified), so raw
NaN bits cannot be identical across targets without canonicalizing every NaN at a per-sample
cost. Ruling: **class-A identity treats every NaN as one value.** Class-A comparisons, digests
and differentials fold NaNs to one canonical word before comparing; the promise is identical bits
on every target except a NaN's sign and payload. The existing NaN-safety rules stand: finite input
must not produce NaN, and each effect's documented NaN behaviour still holds. The engine does not
canonicalize NaNs at render.

## Decision 11: controlled observation activation and #882 (#1080)

After decisions 9 and 10's work landed, the graph and builtins-compiler "controlled observation
activation" machinery (about 5,000 lines, including a per-block hook on the render path) had no
production caller; #882 planned to reuse it to skip per-track meter work while no one holds the
meter lease. Ruling: **remove it (#1080) and rescope #882** to a simpler lease-driven skip of
per-track meter work, built on the ordinary path's existing lease flag, if measurement shows the
saving is worth it. Removing the machinery also removes render's only path to the deallocator.
#1080 also removed the builtins meter's matching half (`restart_observation`, `observation_generation`),
which the removal left unused.

## Decision 12: the console strip (session-level console slots, per-track inserts)

The owner's console-strip design (umbrella issue *Console strip: session-level console effects
with per-track inserts*, `.github/ISSUE_SPECS/CONSOLE-00-console-strip.md` until root numbers it)
emulates a hardware console: every channel runs the same built-in processing and sets only its own
knobs, and outboard gear is patched into a channel's insert point. Sol verified the design against
`5a1421a3` and returned SOUND WITH AMENDMENTS (`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`,
commit `03aceb94`; findings H1-H5, M1-M9, L1-L7 and amendments 1-14 below cite it). The owner
answered the amendments on 2026-09-29. Ruling:

- **Shape.** The session declares `console { "pre_insert": [...], "post_insert": [...] }`. Each
  slot is declared once, with `slot` (a stable ID unique across both sections), the native effect
  `identity`, `quality` and `link_mode`. Every track has every slot: each track carries a `console`
  array of `{ "slot", "bypass", "params" }` in exactly the session's slot order (`pre_insert`,
  then `post_insert`). Per-track `inserts` replace `dynamic` with the same semantics; the per-track
  `simd1` and `simd2` are retired. Either section and `inserts` may be empty (amendments 2 and 4,
  H4's canonical-order objection).
- **Chain.** input section (polarity, trim, HPF/LPF) -> `console.pre_insert` -> `inserts` ->
  `console.post_insert` -> fader/mute -> pan/matrix -> routes. AGENTS.md's chain line, tap list
  and #163 sentence are amended in the same commit as this ruling (M7).
- **Sidechain.** Console slots have no sidechain. A keyed effect is an insert. A banked sidechain
  port would be a future contract issue (#96 F9), with its own level-lift problem (H3).
- **Banking.** Console slots **always** bank, on every target and for every track count, with
  partial groups padded with inactive lanes. There is no member threshold. This is the owner's
  simpler design, chosen knowing that a one- or two-member remainder costs more than per-node
  rendering at W=8 (H5: about 9.3 µs for a padded bank of the three strip slots against about
  4.2 µs per per-node track, arithmetic on the uncontrolled `final-0400` record). There is one
  bank group per (slot, pool class, dependency level), each padded; differing insert counts
  therefore split `post_insert` into several groups and can cost chain fusion (H2, M3,
  amendments 3 and 9). Inserts bank opportunistically, as today.
- **Padding contract.** A padded lane is a clone of an active member's prepared request, never
  zeros; it is fed `+0.0` and its output is discarded. D7 recovery and reports are masked by active
  lanes (M1, amendment 6).
- **Bypass.** Bypass is per lane, through the existing `BypassShunt`, so a bypassed track stays in
  its bank: a session `bypass` lowers to prepared `bypass = false` plus the lane's initial shunt
  state, and the shunt is built whenever any lane is bypassed (H1, amendment 1). A bypassed lane
  runs the wet path; that cost is accepted. Latency is always paid: a bypassed slot still carries
  its fixed latency on every track (L4).
- **Coupling rule.** Banking may couple lanes' cost, never their bits. Every whole-bank decision
  (fast-path gates, D7 recovery, shunt selection) must be bit-neutral per lane (M2, amendment 7).
- **Eligibility.** A console slot may be the parametric EQ, compressor, gate/expander, soft-clip,
  transient shaper or true-peak limiter (`miso.parametric-eq`, `miso.compressor`,
  `miso.gate-expander`, `miso.soft-clip`, `miso.transient-shaper`, `miso.true-peak-limiter`). The
  delay never banks. The multiband compressor is excluded until #1069 closes. A compile diagnostic
  enforces the list (L2, amendment 8).
- **Silence.** Accepted trade-off: the silent fast path is bank-wide, so one active track keeps its
  bank-mates processing (M4). The baseline measures it with a sparse-activity row; a per-lane
  silence skip is a later issue if the measurement warrants it. An all-lanes-bypassed skip stays
  out of scope and would still have to feed latency lines (amendment 11).
- **Naming.** The session key is `console`. The engine's existing "console" names for the
  live-control attachment are renamed to the "live controls" vocabulary (M6, amendment 13):
  `HostConsoleRequest`/`HostConsoleHandles`, `ConsoleEffectBankStage`, the
  `miso_engine_web_v1_console_track_*` exports and their four pinned `console*` boot-option words,
  `sdk/src/core/console.ts`, and every other identifier whose "console" means that attachment. The
  console benchmark and the console fixtures keep their names, because they describe console
  sessions.
  - The two exports are sealed contract identity: exported wasm symbols, pinned by
    `scripts/check-abi-layout-v1.py` and `sdk/assets/miso-engine-v1-abi-layout.json`, and listed
    as class 2 in `docs/rulings/de-versioning-inventory.md`. The version-suffix rule governs only
    their `_v1` suffix, which stays: no V2 is claimed. Renaming the stem is a contract change,
    admitted as an in-place V1 amendment under the wire-identity ruling below. The boot-option words
    keep their offsets and types and change only their spellings. A retired spelling is never
    exported again, for any meaning.
  - The builtins automation token (`rack: "builtins"`, `effect_id: "strip"`) is unchanged. The
    owner's naming decision does not touch it, and the `rack` token keeps it distinct from a console
    slot that happens to be named `strip`. This is R0's reading, not a separate owner decision.
- **Wire identity.** The change is an in-place V1 amendment on the #1063 precedent (amendment 5).
  Nothing is renumbered, and every retired code is refused, never reallocated.
  - Send-tap tokens are renamed with wire codes 1-7 unchanged: `input`, `post_input`,
    `insert_send`, `insert_return`, `pre_fader`, `post_fader`, `post_pan`.
  - `dynamic` -> `inserts` keeps rack code 2 and BTLV track field 7. The `simd1`/`simd2` rack
    codes 1 and 3 and BTLV track fields 6 and 8 are retired. `builtins` keeps code 4.
  - `console` is appended: a rack code after `builtins`, a session root field and a track field,
    each the next unallocated ID in its registry. The browser's 48-byte command record keeps `1`
    for `inserts`, retires `0` and `2` and appends a console value; the observation and
    live-response rack encodings follow the same rule.
  - There is no `ABI_VERSION` generation bump. The prelaunch identity stays V1, and the app and SDK
    update in lockstep.
- **Class A by lowering.** Internally `pre_insert` lowers to `Simd1`, `inserts` to `Dynamic` and
  `post_insert` to `Simd2`, so an equivalent session produces the identical graph, including the
  sealed `MISO-GRAPH-V1` canonical text, and the identical render (M9, amendment 14). The internal
  Rust names `RackId`, `TrackStage`, `MeterTap` and `RackLocation` stay.
- **#971 under padding.** Once every console group binds, the stranded-mono demotion's "binds more
  banks" objective measures nothing (M3). The banking slice chooses between restating the objective
  (for example, minimise planes x banks) and retiring the demotion for console slots, and records
  why (amendment 10).
- **The app.** The problem statement is corrected (M8, amendment 12): the app compiles EQ ->
  compressor onto every track's dynamic rack and marks unselected tracks `bypass`. The baseline's
  app-shaped row is "every track carries EQ -> compressor, a subset bypassed". The app moves the
  pair into `console.pre_insert` after the SDK slice lands.

The slices, their dependencies and their gates are in the umbrella issue. Sol's revised plan is kept
with one addition: the live-controls rename is its own slice (S1r) ahead of live addressing (S1c),
because it reaches about 80 code, script and SDK files (about 780 lines), including two sealed
exports, four boot-option words and the SDK's public live-console API. That is more than S1c's own
half day, and it keeps a class-A rename out of a semantic addressing verdict.
