# Reverb: restore accepts a crossfade between two equal taps; one TapCrossfade owns the delay and reverb tap crossfades

Tracker: #2258, item FX-3. Found by the implicit-state-machine audit of 2026-11-06; re-verified at
base `d441cd408`.

## Mission
The queued tap crossfade has four words: the active tap, the tap being faded towards, the pending
tap that the last control maps to, and an update count. It is written three times, each copy with
its own transitions and its own restore validator. They become one type with one validator.

## Verified defect
The three copies at base `d441cd408`:
- **Delay.** `DelayLane` (`crates/delay/src/lib.rs:709-722`).
  - Transitions: `begin_transition` (`:784-789`, called at chunk boundaries, `:1659-1663`), the
    chunk-end advance (`:1260-1265`) and `reset_to` (`:749-763`).
  - Restore: `read_lane_header` (`:1913-1916`) refuses both illegal phases. It refuses an idle
    crossfade between two different taps, and a running crossfade between two equal taps.
- **Reverb.** The pre-delay (`pre_*`) and the 32 lines (`line_*` plus a `line_queued` cache) live in
  `crates/reverb/src/lib.rs:792-806`. Their transitions are per sample (`kernel.rs:237-246` and
  `:442-454`); the resets are in `lib.rs:1037-1045`.
  - Restore: `read_common_header` (`state.rs:285-294`) refuses only the first illegal phase, for
    both crossfades: `(fade == 0 && active != transition)`. It accepts `fade != 0 && active ==
    transition`.

**Unreachable live.** A crossfade only starts when the pending tap differs from the active one,
and it sets the transition tap to the pending one. For the lines, `line_queued` is
`line_pending != line_active` at every write. So no live state, and no snapshot, fades between two
equal taps.

**Observable once restored.** Such a fade blends a tap with itself, so for finite input it renders
the active tap. But while it runs it blocks a queued tap change, for up to 128 samples, and no
reachable state does that. A probe on base:
- 48 kHz, defaults, 4096 samples rendered;
- the snapshot changed to queue a pre-delay change (target 50 ms, its pending tap);
- restored once with `pre_fade = 128` (a fade between equal taps), and once idle (`pre_fade = 0`).

Both restores return `Ok`. The two renders differ from output sample 956 on.

**Where it can come from.** The restore is the in-memory plan-swap carry (#1269), between two
instances of one build. Its payloads are live snapshots, so production never carries this state.
It is not persisted state (R6b), and the layout is not an on-disk or wire format. Hostile payloads
reach it only through tests (the conformance crafting hooks). So the defect is a hardening gap:
the validator admits a state the model cannot reach.

## Decision
- **`effect_runtime::tap_crossfade::TapCrossfade<T>`.** It has private fields: `active`,
  `transition`, `pending`, `remaining` and a `queued` cache.
  - Transitions: `settled(tap)`, `queue(tap)`, `begin()` and `advance(updates)`.
  - Accessors: `active()`, `transition()`, `pending()` and `remaining()`.
  - The one validator, `from_words(active, transition, pending, remaining)`, accepts exactly the
    reachable phases: `remaining <= TRANSITION_SAMPLES`, and `remaining == 0` exactly when
    `transition == active`. Each tap's domain stays the caller's check.
  - `TRANSITION_SAMPLES = 128` moves there. Each crate's own constant is now an alias.
- **The users.** The delay's two lanes are `TapCrossfade<u32>`. The reverb's pre-delay is
  `TapCrossfade<u32>`, and its lines are one `TapCrossfade<[u32; 32]>`: the lines share one fade.
- **The `queued` cache.** It replaces the reverb's `line_queued`, so a per-sample `begin` on 32
  taps compares no taps. It is `false` while fading and set again when the fade ends. So it is a
  function of the four words, and the derived `PartialEq` (the delay's `lanes_symmetric`) compares
  the same thing as before.
- **No phase enum.** `remaining` is the phase word (`0` idle, `k` fading with `k` updates left).
  The kernels read it per chunk (the delay) or per sample (the reverb). An enum would be matched at
  the same points to re-derive the same word, and the payload holds the words. So privacy plus the
  single validator is what makes illegal states unrepresentable.
- **The blend arithmetic stays in each kernel.**
  - The delay renders in chunks and accumulates `alpha` by exact `1/128` additions (`chunk_of`,
    `tap_sample`).
  - The reverb renders per sample with the closed form `fade_alpha` and `blend`.

  Both give the exact weight `j / 128`, but in different operation orders, so they are not merged.
  Neither kernel's arithmetic changes.
- **The payload layout does not change**, word for word. Only the reverb's validator tightens.
  - The error codes are unchanged (`effect.state.lane`).
  - The order of the error codes is unchanged: the reverb still checks the parameter targets before
    any lane word.

## Non-goals
- The `dsp-reference` delay and reverb oracles, which are independent by design.
- The crossfade length, the blend law and the payload layout.
- FX-1, FX-2 and FX-4 to FX-7.

## Gates
- Red on base, through `restore_state_payload`:
  `reverb::tests::restore_refuses_a_crossfade_between_equal_taps`. A fade between equal taps is
  refused for the pre-delay and for the lines. A restore that is refused writes nothing. A real
  crossfade on both is accepted.
- The validator's own claim: `effect_runtime::tap_crossfade::tests::from_words_accepts_exactly_the_reachable_phases`.
- A one-time no-bit-moved comparison against base, recorded below and not committed.
- `cargo test --locked -p effect-runtime -p delay -p reverb`.
- `cargo clippy --locked -p effect-runtime -p delay -p reverb --all-targets -- -D warnings`.
- `cargo fmt --check`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p effect-runtime -p delay -p reverb`.
- `bash scripts/check-workspace-policy.sh`.
- `./target/release/realtime_policy`.
- `bash scripts/test-web-audioworklet.sh`, because the reverb's render is in the browser
  artifact's render closure.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `d441cd408`.

**Red on base.** The new reverb test, run against the base code:
```
thread 'tests::restore_refuses_a_crossfade_between_equal_taps' panicked at crates/reverb/src/tests.rs:519:9:
assertion `left == right` failed: pre-delay: a crossfade between equal taps is refused
  left: Ok(())
 right: Err(StatePayloadError { code: "effect.state.lane" })
```
With the two arms swapped, the lines arm is red on base too:
```
assertion `left == right` failed: lines: a crossfade between equal taps is refused
  left: Ok(())
 right: Err(StatePayloadError { code: "effect.state.lane" })
```

**No bit moved.** A temporary integration test (not committed) ran on base and then on the fix,
in a release build, through the public contract (`DelayFactory`, `ReverbFactory`). It hashes,
with FNV-1a 64, every output word, every `ProcessReport` and every full state snapshot after every
block.
- Delay: 3 rates × 12 seeds × 400 blocks.
- Reverb: 3 rates × 8 seeds × 300 blocks.
- Blocks: random lengths from 1 to 512, a quarter of them 1 to 8 samples long.
- Automation: drawn at a third of the block starts. Delay times and pre-delay or size are drawn
  often and mostly short, so crossfades queue behind running ones.
- Faults: an infinity injected into about 1 block in 60, so the recovery resets run.
- Restores: about 1 block in 10 snapshots and restores into a fresh instance prepared at other
  values (the plan-swap carry), and checks that the restored snapshot is identical.

| | spans | restores | restores mid-crossfade | faults | hash, base | hash, fix |
|---|---|---|---|---|---|---|
| delay | 13224 | 1459 | 242 | 239 | `8efe8f354db23308` | `8efe8f354db23308` |
| reverb | 5979 | 753 | 115 | 121 | `3d701db95e402429` | `3d701db95e402429` |
