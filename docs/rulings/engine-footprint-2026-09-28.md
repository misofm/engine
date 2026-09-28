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
| R8 whole-plan scalar backend | 32-bit is not a target (below). Removal waits on one more ruling: keep an unbanked per-node plan as a test-only oracle for "banking never moves a bit", or accept a weaker guarantee. |
| R9 wasmtime console benchmark | Retire it, and the nightly descriptive benchmarks if nothing uses them. The native console `--step` rows and the V8 rows on the shipped artifact are the benchmarks. |
| R10 closed issue specs | Keep only open specs locally, with the verification's corrections (re-point citations; keep `BRIEFS/`). |
| Draft 02 unwired control endpoints (#140) | Held. Mobile live control ("they should be able to change it live") needs an engine path; decide after checking whether the C ABI can reuse the browser's live-control lane. |
