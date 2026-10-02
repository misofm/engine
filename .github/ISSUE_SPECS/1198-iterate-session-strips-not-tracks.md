# Iterate session strips, not tracks, wherever strip semantics apply

Slice 01 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K0.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

Class A: no rendered bit, digest or canonical graph text moves.

## Product outcome

The compilers learn the word "strip": the chain after a source. That is the input section, the
console slots, the inserts, the fader and mute, and the pan or matrix.

Today only tracks are strips, so nothing changes. The next slices then give submixes a strip by
adding one variant, not by editing every compiler again.

## Context (verified on `fe8ac679`; no earlier slice moves these anchors)

- **The track.** `Track` (`crates/session/src/model.rs:220-241`) is a source mapping (`source_id`,
  `left_source_channel`, `right_source_channel`) plus the strip fields `builtins`, `console`,
  `inserts`, `fader` and `matrix_or_pan`. `SessionModel::lower_track` (`model.rs:335-345`) lowers
  `console` and `inserts` to three internal racks. `crates/session/src/lib.rs:27` re-exports
  `model::*`.
- **Order.** `compile_session` sorts tracks (`crates/session/src/compile.rs:137-139`) and submixes
  (`:140-142`) by ID, and canonicalizes track insert and console parameters (`:152-170`). Every lane
  assignment and digest depends on that order.
- **Every compiler iterates `model.tracks` for strip work.**
  - **graph-compiler** (`crates/graph-compiler/src/`):
    - `compile.rs:174-178`: the lowered racks; `:188-207`: the declared-effect set;
    - `:224-295`: the stage chains;
    - `:352-381`: sidechain edges, whose path is `format!("$.tracks[id={}].sidechain", ..)` at
      `:377`;
    - `:543-598`: response bindings;
    - `banks.rs:148-196`: rack chains;
    - `effect_path` (`ids.rs:232-241`, used at `compile.rs:196`, `:203` and `:277`) hard-codes
      `$.tracks[id=<track>].<rack>.effects[id=<effect>]`;
    - the literal collection string `"$.tracks"` at `compile.rs:249`, `:287`, `:293` and `:294`.

    These edge paths are written into the **sealed canonical graph text** (`canonical.rs:232`; each
    edge row of `fixtures/graph/v1/direct-route.canonical.txt:30-35` ends `\t$.tracks`, and the
    sidechain and effect paths carry `[id=..]`), so for tracks they must stay byte for byte.
  - **builtins-compiler** (`crates/builtins-compiler/src/lib.rs`):
    - `validate_for_session` `:2048-2069`;
    - `processor_seal` `:3069-3085`, re-checked at `:2071`;
    - `expected_tails` `:3117-3138`;
    - control-request validation `:3327-3345`, whose `known_tracks` set also validates meter
      requests;
    - the parameter preflight `:3350`;
    - the preparation loop `:3401-3473`;
    - the seal's `tracks` vector `:3518-3523`;
    - the resource plan `:3557-3600`;
    - `planned_strip_banks` `:1279-1294`, used at `:2296`, `:2321` and `:2445`, which builds
      `TrackStage` node IDs from track ID strings;
    - `track_parameters` `:4706-4749`, called at `:3124`, `:3351`, `:3402` and, in a test, at
      `:12185`;
    - the five diagnostic path builders, each taking `&Track`: `parameter_diagnostic` `:4777`
      (which formats `$.tracks[id=..]` at `:4791`), `gain_path` `:4805`, `cutoff_path` `:4820`,
      `filter_order_path` `:4835` and `matrix_path` `:4850`.
  - **effect-compiler** (`crates/effect-compiler/src/prepare.rs`):
    - `prepare_with_console_eligibility`'s strip loop `:331`, with its path at `:342`;
    - the only production `EffectPreparedEntry` literal, `:596-608` (the struct is `:28-69`);
    - the paths at `:1466` (`attach_effect_live_controls`) and `:1591` (`attach_effect_observation`),
      both `format!("$.tracks[id={}].effects[id={}]", entry.track_id, ..)`: they have no strip in
      reach, only the entry's owner ID;
    - `declared_live_addresses` `:1650`.
  - **session**: the estimate's track-only `effect_count` (`crates/session/src/estimate.rs:64-80`),
    `parameter_count` (`:81-99`) and per-track vectors (`:131-160`). The estimate reads the
    un-normalized model; order does not matter there.
  - **host-core**: `count_effects` (`prepare.rs:1313-1324`), which counts tracks' console entries
    and inserts, and feeds `shape.rs:81` (`effect_count`).
- **Source semantics, which must stay track-only:**
  - `track_mono_source` (`builtins-compiler/src/lib.rs:3796`);
  - `session_structural_symmetry` (`:3839-3862`);
  - `SessionPoolClasses::from_session` (`:3924`; `:3915` is the struct);
  - source bindings and claims;
  - `track_delays` (`crates/graph-compiler/src/compile.rs:460-471`): its runtime arm, `TrackDelay`,
    runs on a **source input** and returns before any reduction (`crates/graph/src/runtime.rs:2980-2989`,
    `node_kind` `:4026-4041`), so it cannot serve a strip without a source;
  - the host-core track counts and caps (`prepare.rs:765-777`) and `shape.rs:79-80`
    (`track_count`, which host-web uses to size per-track shadows);
  - the host-core live-control requests and live-control track list (`prepare.rs:912-928`) and the
    meter requests (`:929-974`), which stay track-only until later slices;
  - solo (`crates/host-core/src/solo.rs`).
- **The collapse guard.**
  - `arm_mono_collapse` (`crates/graph/src/runtime.rs:2326-2338`) arms a bank only when every lane's
    ID is in the host-supplied `eligible` set.
  - That set is `session_structural_symmetry` over tracks, filtered at
    `crates/host-core/src/prepare.rs:1284-1289`.
  - `gathers_track_input` (`runtime.rs:5208-5222`) is only a node-shape test, so this set is what
    keeps a future bus from collapsing (DESIGN 2.2b).
- **Validation paths.** Session validation reports index paths (`$.tracks[3].fader.left_db`,
  `crates/session/src/validate.rs:303`). Only the compilers use `[id=..]` paths. This slice does not
  touch session validation.

## Interface contract

In `crates/session/src/model.rs` (exported through `lib.rs`'s `pub use model::*`):

```rust
pub struct StripRef<'a> {
    pub id: &'a StableId,
    pub kind: StripKind<'a>,
    pub builtins: &'a DualMonoBuiltins,
    pub console: &'a [ConsoleEntry],
    pub inserts: &'a Rack,
    pub fader: &'a DualMonoFader,
    pub matrix_or_pan: &'a MatrixOrPan,
}
/// Deliberately exhaustive: *Render a submix strip on its summed input* adds `Submix`, and every
/// match on this enum must then fail to compile until it handles the new variant.
pub enum StripKind<'a> { Track(&'a Track) }

impl SessionModel {
    /// Every strip, in model order: `tracks`, then (later) `submixes`. On a normalized model that
    /// is canonical ID order within each segment.
    pub fn strips(&self) -> impl Iterator<Item = StripRef<'_>>;
    /// `lower_track`'s body, taking a strip; `lower_track` delegates to it.
    pub fn lower_strip<'a>(&self, strip: &StripRef<'a>) -> LoweredRacks<'a>;
}
impl StripRef<'_> {
    /// The compilers' path prefix: `$.tracks[id=<id>]` for a track (later `$.submixes[id=<id>]`).
    /// Session validation's index paths never use it.
    pub fn path_prefix(&self) -> String;
    /// The sealed collection path of the strip's chain edges: `"$.tracks"` for a track (later
    /// `"$.submixes"`), exactly the literal the four chain-edge sites write today.
    pub fn collection_path(&self) -> &'static str;
}
```

These names are fixed by this body. In addition:

- builtins-compiler: `track_parameters(&Track, ..)` becomes `strip_parameters(&StripRef<'_>, ..)` at
  every caller, including the test call at `lib.rs:12185`, whose argument becomes
  `&model.strips().next().unwrap()` (or equivalent) and whose assertions are unchanged.
  `parameter_diagnostic`, `gain_path`, `cutoff_path`, `filter_order_path` and `matrix_path` take
  the strip, and `parameter_diagnostic` builds its prefix from `path_prefix()`.
- effect-compiler: `EffectPreparedEntry` gains `pub strip_path: Box<str>`, the owning strip's
  `path_prefix()`, set at the literal `prepare.rs:596`. The diagnostics at `:342`, `:1466` and `:1591`
  format `"{strip_path}.effects[id={effect}]"`. The struct's `track_id` keeps its name (the graph's
  internal key, decision 12 kept internal names).
- graph-compiler: `effect_path` takes the strip's `path_prefix()` instead of a track ID; the four
  chain-edge literals use `collection_path()`; the sidechain path at `:377` is
  `format!("{}.sidechain", strip.path_prefix())`.

## Deliverables

1. Add the types above, with the doc comments shown.
2. Switch every strip-semantics site in the Context to `strips()` and `StripRef`, building each
   compiler path from `path_prefix()` or `collection_path()`.
3. Leave the source-semantics sites as they are, with a one-line comment at each:
   `// Source semantics: tracks only.` At `session_structural_symmetry`, add one more line: this set
   is the mono-collapse guard, and a strip with no source must never enter it. At `track_delays`,
   add: the `TrackDelay` arm runs on a source input; a submix's delay is a separate arm
   (*Delay a submix strip's summed input*, #1201).
4. host-core: `count_effects` iterates strips; `shape.rs`'s `track_count` keeps track meaning.
5. Change no public behaviour, diagnostic code or path spelling. A track's compiler path stays
   `$.tracks[id=..]`, and its sealed edge paths are byte-identical.

## Authorized paths

- `crates/session/src/{model.rs,lib.rs,estimate.rs}`
- `crates/graph-compiler/src/{compile.rs,banks.rs,ids.rs}`
- `crates/builtins-compiler/src/lib.rs` (including the test call at `:12185`, argument only)
- `crates/effect-compiler/src/prepare.rs`
- `crates/host-core/src/prepare.rs` (only `count_effects`, and the source-semantics comments)
- `crates/host-core/src/shape.rs` (comment only, if any)
- this spec

## Non-goals

- No submix strip, and no schema or wire change.
- No rename of the graph-internal `track_id`, `TrackStage` or `EffectNodeId`. Decision 12 kept
  internal names, and so does this umbrella.
- No change to session validation.
- No change to `track_delays` beyond its comment.

## Hazards

- **Losing a source check.** `track_mono_source`, `session_structural_symmetry` and the pool classes
  must keep seeing tracks only. If a strip with no source ever entered the eligibility set, a stereo
  bus could arm the mono collapse and copy its left lane over its right.
- **Order.** `strips()` must yield `self.tracks` in vector order, then `self.submixes`. On a
  normalized model that is the order `compile_session` sorted.
- **Sealed paths.** The canonical edge paths are part of the sealed graph text. Building the four
  chain-edge paths from `path_prefix()` (which returns `$.tracks[id=<id>]`) instead of
  `collection_path()` would move every graph digest. Gate 1 catches it.
- **`#[non_exhaustive]` is forbidden on `StripKind`.** It would let other crates add `_` arms that
  silently absorb the submix variant in the next slice instead of failing to compile at the sites
  that slice must change.

## Objective gates

1. **Class A, as PR evidence and not a committed test** (`AGENTS.md` test rules). Run on this branch
   and on its base, and attach both outputs:
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`, then
     `diff <base>/target/issue6/fresh-process-determinism.json target/issue6/fresh-process-determinism.json`
     (the file the script writes): no difference
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`

   Every canonical graph text, `output_sha256` and fixture digest must be identical.
2. **The workspace test command passes with no test assertion edited** (the one test call at
   `builtins-compiler/src/lib.rs:12185` changes its argument only):
   ```
   cargo test --locked --workspace --all-targets \
     --exclude lane --exclude math --exclude effect-runtime --exclude delay \
     --exclude compressor --exclude multiband-compressor --exclude gate-expander \
     --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip \
     --exclude parametric-eq --exclude builtins --exclude dsp-reference \
     --exclude conformance --exclude audit --exclude bench --exclude console-workload \
     --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus \
     --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit
   ```
   and `cargo test --locked --release -p audit -p bench -p console-workload` passes unchanged.
3. **Formatting, lint, docs and policy**:
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `bash scripts/check-session-policy.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-graph-policy.sh`, `bash scripts/check-host-core-policy.sh` and
     `bash scripts/check-workspace-policy.sh`, each with its `test-*` twin
4. **No new test is required.** The claim is "nothing moved", and the existing suite plus gate 1
   carry it. If the implementer adds one, its PR names the defect that only it catches.

## Evidence

- The two gate-1 outputs and the empty `diff`.
- The list of switched sites.
- The list of sites deliberately left track-only, each with its comment.

## Dependencies

- *Record the submix, send and VCA ruling* (#1197)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every "unchanged" or "identical" gate is a hard stop, not a tolerance. NaNs are folded to
  one value (decision 10).
- Render stays allocation-, lock- and syscall-free (`scripts/check-realtime-policy.sh`).
- Commit on the batch branch the root names, from synchronized `main`. Touch no path outside the
  authorized list.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
