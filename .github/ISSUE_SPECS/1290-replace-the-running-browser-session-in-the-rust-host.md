# Replace the running browser session in the Rust host

Slice B2 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

The browser engine's Rust host can take a new session document while it plays and swap to it between
two render calls, with the C ABI's guarantees: unchanged sources keep their rings, unchanged nodes
keep their state, the clock continues, and a refused document leaves the running engine untouched.
Today the browser has no structural edit path; a structural change boots a new engine. This slice is
the Rust method, tested natively. *Keep browser live strip state across a session replacement* (B3)
and its effect counterpart (B3b) make live-edited values carry, *Export session replacement from the browser engine module* (B4) exports
it, and the worklet and SDK slices wire it.

## Context

- `AudioWorkletEngineHost` (`hosts/host-web/src/lib.rs:1986`) boots once
  (`boot_with_spectrum_config`, `:2039`): document size and parse-projection budgets,
  `compile_host_model`, the `require_sample_rate_hz`/`require_quantum_frames` shape check, the VCA
  bound, then `compile_ready` (`:6612`), which prepares through one of three branches (no spectrum,
  a single spectrum capture, a spectrum collection; `:6636-6680`), then the bridge buffers. It owns
  one `ReadyOwnership` (`:1509`) and a `host_generation` (`:2010`) that prepared companions are bound
  to; companions address strips by index.
- `render_next` (`:3208`) renders `ready.host.plan` with `RenderTime { absolute_sample:
  status.next_absolute_sample }` (`:3229-3234`).
- Successor preparation, the inventory, `SuccessorBase` and `SourceControlSet::adopt_persisting` come
  from *Prepare a successor plan whose unchanged sources keep playing* (#1272); the synchronous
  `PreparedRenderPlan::adopt_predecessor_plan` (which also continues the clock) from *Hand the
  outgoing plan to its successor at the swap block* (#1270); the builtin carry through
  *Carry fader, mute and pan ramps across a plan swap* (#1277). Effect and delay-line carries
  (slices 9-14) apply when they land.
- B1 (*Measure a session rebuild on the browser's audio thread*, #1289) states the expected cost of
  this method on the audio thread.

## Decisions frozen for this slice

- **D1. Method.** `AudioWorkletEngineHost::replace_session(&mut self, document: &[u8]) -> u32` (a
  result code; a refusal also writes the host diagnostic, as boot does). It runs the boot's document
  checks and budgets, refuses a different rate or quantum with `RESULT_REPREPARE_REQUIRED`, and
  prepares the successor through new host-core successor wrappers for **each** of `compile_ready`'s
  three preparation branches, keeping the current spectrum configuration. A document that removes the
  strip a spectrum capture observes is refused with a typed result (`web.replace.spectrum_target`);
  the app stops or reselects the capture first. Any refusal returns before the swap and changes
  nothing.
- **D2. Budget.** The projection charges the old plan and the new plan together (both exist until
  the old one is dropped), counting each carried ring once, plus the parse transient, against
  `maximum_memory_bytes`.
- **D3. Swap order**, after every fallible step:
  1. `new.host.sources.adopt_persisting(&mut old.host.sources)`;
  2. `new.host.plan.adopt_predecessor_plan(&mut old.host.plan)`;
  3. replace `ready`; keep `status.next_absolute_sample` and `rendered_quanta`; advance
     `host_generation`, so a companion prepared for the old strip indices is refused;
  4. drop the old `ReadyOwnership`.
- **D4. Values, until B3.** `SuccessorBase.committed` is the model the running plan was booted or
  last replaced from. An owner that received any live record since then is not carried (it starts
  at rest at the new document's value). B3 replaces this rule with the effective-model comparison.
- **D5. Observation.** Meter, observation and spectrum state restart; the meter generation advances;
  the meter lease carries; a window open at the swap is dropped and counted as a loss.

## Deliverables

1. D1-D5 in `hosts/host-web/src/lib.rs` and the host-core wrappers.
2. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`

## Non-goals

- No Wasm export, no worklet or SDK change (B4-B8). No session model or revision (#1057).

## Objective gates

1. **Gap-free acceptance.** Native host-web test, live controls on: boot A (nine tracks with enabled
   high-pass and low-pass filters and non-centre pans, no live record admitted), render 6 blocks
   feeding sources through `submit_source`, `replace_session(B)` with B = A plus a muted track whose
   ID sorts first, render 6 more. Every block equals a fresh boot of B fed the same PCM from frame 0.
   A response capture after the swap is stamped with the continued clock.
2. **Refusals change nothing.** A malformed document, a different rate, and a document over the
   memory budget each refuse with their typed result; the following blocks are bit-identical to a run
   without the call.
3. **Each preparation branch.** Gate 1 passes with a single spectrum capture and with a spectrum
   collection configured.
4. **Companions.** A prepared companion bound before the replace is refused after it.
5. **Realtime.** The render export reaches no allocator: rebuild the artifact and run
   `scripts/check-web-audioworklet.sh` (its call-graph check).
6. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support` and
     `cargo test --locked -p host-core --features host-core/test-support`
   - `rm -rf target/ci/b2 && mkdir -p target/ci/b2/a target/ci/b2/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/b2/n target/ci/b2/a && bash scripts/check-web-audioworklet.sh target/ci/b2/a target/ci/b2/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/test-web-audioworklet.sh`
   - the umbrella's inherited gates. The shipped artifact changes: report it.

## Test value

- Gate 1: a replacement that reboots internally (fresh rings and state), or a successor whose clock
  restarts at 0, turns it red.
- Gate 2: a refusal that runs after the producers moved, or after the old `ReadyOwnership` was
  dropped, turns it red.
- Gate 4: a companion addressed by an old strip index reaching the new plan turns it red.

## Dependencies

- *Hand the outgoing plan to its successor at the swap block* (#1270).
- *Prepare a successor plan whose unchanged sources keep playing* (#1272).
- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Measure a session rebuild on the browser's audio thread* (#1289), and owner question Q4 if B1
  reports a 64-track rebuild over one quantum's budget.
