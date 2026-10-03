# Research: render stored session automation in the engine, identically on every platform

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`): a producer's automation (fades, sweeps, rides) is **rendered by the core engine from the session file**, so fans hear it bit-identically on web and mobile; apps do not play it back.

## Today

The session schema's `automation` table accepts builtin (fader, mute, pan) and effect-parameter targets and the SDK can author them, but it "renders nothing" (`docs/SESSION_SCHEMA_V1.md:97-100`; `docs/REALTIME_MEMORY.md:13`), gated on #140, which is being closed as descoped. The engine already has absolute sample-time parameter events, bounded point batches and step/linear/exponential segments (AGENTS.md "Interfaces and transports"), the live console lanes, and per-effect automation spans.

## Questions to answer, with evidence

1. **Design.** How the compiled plan evaluates stored automation sample-accurately in render with no allocation, locks or unbounded work: where curves are compiled (control plane), how they feed the existing fader/matrix ramps and effect spans, and how seeking and looping work.
2. **Live changes against automation.** What happens when a fan (or producer) moves a control that has automation: override until release, offset, or latch. Present the options for an owner ruling; the personal-mix ruling (see the session-ownership research issue) interacts with this.
3. **Format.** Whether today's `automation` table is enough (curve shapes, resolution, units), aligned with the session's `controlSmoothing` settings (#1054).
4. **Cost.** Memory per automated lane independent of song length, CPU per block on the native console and the shipped browser artifact, and a benchmark row (the `console_mixing_automation` row is the natural base).
5. **Bit-identity.** How browser and C ABI render identical bits for the same session and seek position.

## Coordination with *Submix strips and live aux sends* (#1196, decision 13)

A route with `follows_mute: true` (*Let a route into a submix follow its source strip's mute in the session*, #1218) zeroes the matrix columns of its source strip's muted lanes. Builtin automation of the fader's `mute` (parameter 6) is valid and inert today (`BUILTIN_AUTOMATION_TARGETS`, `crates/session/src/validate.rs:683-699`; `validate_builtin_automation_target` documents the inertness). When stored mute automation renders, a following send must follow the automated mute through the same gated route composition the live path uses (*Let a send follow its source strip's mute live in the browser*, #1224); otherwise a saved session with automated mute leaks its pre-fader sends. The design must answer how.

## Output

A findings note under `docs/handoffs/` with the design, costs, a staged plan of small bounded issues, and the owner questions. No product code change.
