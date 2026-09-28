# Research: the core engine owns the current session on every platform, and saves are snapshots

Owner rulings (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`):

* The portable core owns the current, edited session on every platform, browser included. Every edit goes through the engine, which keeps the session model and a revision; any app saves by asking for a canonical JSON snapshot and storing it itself (device, browser storage or server).
* A fan's edits are saved as a **personal mix**; the producer's original is untouched.

## Today

* **C ABI (mobile, native):** the protocol controller owns a `SessionStore` model and revision; `SessionSnapshotGet` returns the canonical snapshot (`crates/protocol/src/controller.rs`).
* **Browser:** the TypeScript SDK owns the document (`sdk/`; a canonical writer exists in `sdk/src/internal/session-json.ts` but is not exported). `hosts/host-web` keeps compiled plans and live lanes but no session model, and `protocol` is not in host-web's dependency closure.

## Questions to answer, with evidence

1. **Design.** What is the smallest way for the browser adapter to use the core's session model and revision (the same `SessionStore`/transaction path the C ABI uses, or a shared core type both adapters use), and what does the SDK become? Keep adapters thin; no browser-only session logic in the core.
2. **Cost.** Measure the shipped AudioWorklet artifact size and boot time with the session model in wasm; which crates enter host-web's closure; per-edit cost on the control thread (today the C ABI compiles the whole session on every edit).
3. **Live edits.** How value-only live edits (#1053) and structural edits both update the one model, with the acked-batch rule intact, on both adapters.
4. **Personal mixes.** How a personal mix is represented (a full session copy, or the producer's session plus the fan's changes), and what happens to a fan's personal mix when the producer later publishes a new version of the session. Present the options and their costs; this needs an owner ruling.
5. **Saves.** The snapshot API on both adapters, its cost, and whether it is safe to call during playback (no render-thread work).

## Output

A findings note under `docs/handoffs/` with the design, measured costs, a staged plan of small bounded issues, and the owner questions (personal-mix representation first). No product code change.
