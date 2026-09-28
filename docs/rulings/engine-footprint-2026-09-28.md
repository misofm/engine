# Owner rulings for the engine-footprint cleanup (2026-09-28)

Context: the dead-code audit and its verification,
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md` and `VERIFY-DEAD-CODE.md`. The sprint's
aim, in the owner's words, is "minimizing the engine footprint".

## Product scope

* **The engine is cross-platform.** "The production/mixer UI will be primarily in a browser, but
  fans still need to use the engine to open sessions and that will happen on web app or mobile
  app." Producers mix in the browser; fans open and play sessions in the web app or in iOS and
  Android apps.
* **Native AArch64 is an official target:** "Yes ARM64 will be an official target." iOS and
  Android 64-bit builds need CI coverage, the realtime rules and bit-identity with the browser.
  This reopens the ground #378 ("native AArch64 unsupported") and #023 closed.
* **Mobile is 64-bit only:** "Yes let's go 64bit only." iOS arm64 and Android arm64-v8a ship;
  32-bit ARM (armeabi-v7a) does not, and a 32-bit build is refused at compile time.
* **A desktop app and a cloud service are expected eventually, "not within the next 6 months".**
  Code that only they would use is not live scope now.
* **Removals:** "The removals look good if they're not actively used." Code that nothing actively
  uses is removed even if a later product might want it; git history keeps it.

## Decisions on the audit's rulings

| ruling | decision |
|---|---|
| R1 native host shells, target smoke, AArch64 arms | Keep the AArch64 arms (mobile). Remove the two unused native host shells. |
| R2 C ABI and native PCM runner | Keep the C ABI (the mobile interface). Remove the native PCM runner (desktop tooling, not in use). |
| R3 control protocol | Keep: the C ABI's command path is the protocol. Delete the items rustc proves unused. The WebSocket sidecar is out of scope (AGENTS.md text only). |
| R4 native WAV/RF64 decode workers | Remove, including Part B. Mobile hosts decode with the platform and submit planar `f32`. |
| R5 extended sample rates | Remove; accept only the launch rates. |
| R6 third-party effect packages and state migration | Remove. |
| R7 eight-lane wasm measurement build | Remove (reverses the #183 ruling). |
| R8 whole-plan scalar backend | Keep as a test-only correctness reference for vector banking (decision 3), a named exception to "modes production never needs are removed"; compile it only for tests (#1059 replaces R8). |
| R9 wasmtime console benchmark | Retire it, and the nightly descriptive benchmarks if nothing uses them. The native console `--step` rows and the V8 rows on the shipped artifact are the benchmarks. |
| R10 closed issue specs | Keep only open specs locally, with the verification's corrections (re-point citations; keep `BRIEFS/`). |
| Draft 02 unwired control endpoints (#140) | Delete, including the protocol delivery files (#1056). Mobile live control uses the core's console lanes instead (#1053, after #1042). #140 closes as descoped, not superseded. |

## Live control and ramps (decision 1, 2026-09-28)

* The C ABI adapter moves onto the core's existing live-control lanes for fader, mute and pan
  (#1053), after the plan-swap fix (#1042). Browser-specific code stays in the browser adapter
  (`hosts/host-web`, `sdk/`); the lanes and ramps are portable core code.
* Ramp lengths are **not hardcoded**: they are optional session settings
  (`controlSmoothing`: mute, fader, pan in milliseconds), so a session carries them to every
  platform, with one documented default table in the session schema and a per-change override
  (#1054). The default values come from cited research, measurement and listening (#1055); the
  starting proposal is about 5 ms for mute/solo and about 20 ms for fader and pan moves.

## Automation, session ownership and saves (decision 2, 2026-09-28)

* A producer's automation is rendered by the core engine from the session file, identically on
  every platform (#1058, research first).
* The core engine owns the current, edited session on every platform, browser included; apps save
  by asking for a canonical JSON snapshot and store it themselves (#1057, research first).
* A fan's edits are saved as a personal mix; the producer's original is untouched (#1057 asks how a
  personal mix is represented and what happens when the producer updates the session).
* #140 closes as descoped: its stored-automation outcome moves to #1058; sample-timed
  `AutomationEnqueue` delivery has no product consumer.
