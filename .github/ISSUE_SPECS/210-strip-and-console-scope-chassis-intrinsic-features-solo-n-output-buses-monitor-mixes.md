Owner-ruled product direction (2026-08-27/28 discussion), consolidated for prioritization and to inform the in-flight strip architecture (the #202 overhead round's Jobs 2-5). Vocabulary ruling: the per-track container is the **strip** ('acts like a 500-series chassis — the builtins are built into the enclosure; the racks hold the swappable modules').

## Strip-intrinsic additions (chassis features)

1. **Track delay / time alignment** — per-track sample/ms delay for multi-mic alignment (drum overheads vs close mics; directly relevant to the mono-heavy workload). No strip presence today; #127 (nudge units) is the dormant roadmap item. Natural strip-intrinsic state; cheap (small ring, same family as the dynamics rings).
2. **Input-side liveness** — trim, polarity, HPF/LPF are PreparedOnly today; consoles have them live, and gain-riding trim pre-compressor is a real workflow fader automation cannot substitute. Resolve together with #178 as ONE decision: which strip parameters are live/automatable, decided for the container, not per-param.
3. **HPF/LPF slope options** — one SVF section each today (12 dB/oct); consoles offer 18/24. Floor-inventory recount trigger if adopted (same class as #191 items).
4. **Strip metering points** — input / pre-fader / post-fader taps mostly exist via MeterTap stages; confirm the tap menu matches a console UI's needs when #203 (metering cost) is designed.
5. **Solo — SIP + PFL together:**
   - **SIP (solo-in-place)**: strip-owned solo bit; effective gate = `user_mute OR (any_solo_active AND NOT my_solo)` — one control-plane global bit per block, zero cross-lane audio coupling, applied at the existing declicked mute gate. Keeps user-mute and solo as separate states (hardware semantics; snapshot/restore correct by construction). Matches Logic's complete solo story.
   - **PFL (pre-fader listen)**: soloed strips' pre-fader signal to the monitor bus — specced against the N-output ABI below (Pro Tools / Control Room class). The strip's scatter surface must accommodate a second destination (see design note to Jobs 2/3).

## Console-level: N output buses (owner ruling: model 2)

Build the ABI for **N output buses**; the app uses replace mode day one (monitor bus as the only active bus = the degenerate case, so nothing is throwaway). This is the foundation for: PFL monitoring, **monitor/cue mixes**, simultaneous stem printing, hardware output paths — the multi-output capability both Logic and Pro Tools have independent of solo. The session graph is already shaped for it (`outputs` is a plural root key; submixes + routes exist and work today — a drum submix is expressible in current Session V1). The gaps:
- Host ABI: the web render surface produces exactly one stereo pair; grows an output-count dimension (C-ABI, worklet outputs, offline/PCM-runner record format, SDK — coordinate with #207's Phase 2+).
- **Live send levels**: route gains are folded at bind (prepared-only); cue mixes need send level/pan on the live command surface like fader/pan. Command-vocabulary extension (`miso.command.v1` + metadata + the seven-spelling gate), not an architecture change.
  - Owned by *Submix strips and live aux sends* (#1196, decision 13): live send levels are delivered
    in the browser by slices 22-25 (#1220-#1223) and on the C ABI by slice 27 (#1225). The N-output
    part of this issue is unchanged.
- Dormant-bus cost must be ~zero (earned-silence gating pattern).

## Sequencing constraints

- **Design note for the in-flight #202 Jobs 2/3 (binding now)**: the strip's scatter surface must accommodate a second destination (solo/monitor bus) so PFL lands without retrofit.
- SIP can land immediately after the strip banking (Job 2).
- The N-output ABI is its own designed-and-adversarially-verified item after the strip round; PFL + cue mixes ride it.
- Track delay and the liveness ruling ideally precede Job 5 (strip representation cleanup) — both are chassis-intrinsic state.
- Nothing here blocks the current optimization wave.

All items are feature work (class-N), separate from the class-A optimization ledger; floor recounts triggered where noted.
