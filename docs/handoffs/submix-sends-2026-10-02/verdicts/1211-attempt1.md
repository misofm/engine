# #1211 *Give every strip one mute owner and live-control producers in host-core*: Sol verdict, attempt 1

- Reviewed: `git diff 66b2c7dd7 4ee4d3f30` (branch `codex/batch-submix-k2`, worktree
  `/home/bl/misofm/wt-submix-k2`): 14 files, +628/-116.
- Binding: `AGENTS.md` (realtime rules, test-value rule, the acked-batch question) and
  `.github/ISSUE_SPECS/1211-give-every-strip-one-mute-owner-and-live-control-producers-in-host-core.md`
  with its Attempt 1 record; DESIGN P7 and section 5.9, and VERIFY-2 M4, for the solo semantics.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. The worktree carries #1212's uncommitted
    and committed work; I used only `git show`/`git archive`.
  - `gh issue view 1211`: OPEN, title matches the spec's H1.
  - I exported `4ee4d3f30` to `/tmp/claude-1002/v1211/src`, used my own `CARGO_TARGET_DIR`, and
    ran the mutations in place with byte-for-byte restores. Before writing this, `diff -r` of the
    export against a fresh archive: IDENTICAL. I deleted all scratch afterwards.
- My extra probes are in `submix-verdicts/1211-attempt1-verifier-scratch.rs`. To run it, drop it
  into `crates/host-core/tests/` and use `--features engine/realtime-audit`.

## Verdict: PASS

There is no BLOCKER and no MAJOR.

- D1 and D2 are implemented at the sites and in the shapes the spec freezes:
  - one `TrackControlRequest` per strip;
  - `strip_controls` parallel to `strips`;
  - `StripMuteSeed { mutes, solo_safe }` is re-exported;
  - the solo-safe guard sits before the shadow;
  - `effective_mute = user_mute || (any_solo && !solo_safe && !soloed)`, the one public
    composition.
- D3 is a compile fix only, and host-web's behaviour does not change. D4 correctly did not apply,
  because #1053 has not landed.
- Every gate I could run passes, and the reach counts match Terra's record exactly.
- All five bus builtin records (fader, mute, trim, polarity and matrix) render bit-identically to
  their prepared twins, in both the live and the between-render-calls delivery modes. Draining them
  allocates nothing under the realtime audit (scratch probes below).

There is one MINOR finding and four NITs. None of them needs another attempt.

## Solo semantics: correct per the ruling and per console practice

- **Ruling.** DESIGN P7 and section 5.9 freeze buses as solo-safe: never soloable, never
  solo-muted. Submix solo and implied upstream solo are follow-ups. The implementation is exactly
  that. `set_solo` refuses a solo-safe entry before `shadow()`, so a refused bus solo opens no
  transaction, never sets the bit and never bumps `solo_count`. `effective_mute` keeps a bus at its
  user mute whatever any track's solo does.
- **Console practice.** This is solo-in-place with solo-isolated buses: Pro Tools aux inputs
  default to solo-safe, and large-format consoles isolate group and bus masters from SIP. In that
  model:
  - a soloed track stays audible through every bus and return it feeds;
  - the non-soloed contributors are cut at their own faders, so the bus carries only the soloed
    source (P11's `follows_mute` closes the pre-fader leak later);
  - a user-muted bus stays muted, because mute beats solo, which matches SIP practice.
- **End-to-end proof** (scratch `zz_soloed_track_heard_through_solo_safe_bus`):
  - setup: three tracks into `bus`, with a per-strip `LiveControlSoloState` seeded tracks-then-bus;
  - action: solo `t0` and push every strip's `track_delta` onto `strip_controls[strip].fader`;
  - result: the output is audible and bit-identical, from the commanded block on, to a twin
    prepared with `t1` and `t2` muted.
  - **Mutation X1** (drop `!self.solo_safe(strip)` from `effective_mute`, which is "a soloed track
    whose bus is muted by solo logic"): the probe goes red with `block 2: silent output under solo`,
    and the committed gate-1 test
    `a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition` goes red too.

## Transaction atomicity

- `solo_safe` is construction-only, with no shadow, and nothing writes it after `try_new`.
- `shadow`/`rollback` copy whole arrays (`solo`, `user_mute`, `emitted`, `solo_count`), so they
  cover every strip, buses included.
- `rollback_restores_every_strip_including_the_solo_safe_ones` mixes bus user-mute and emitted
  edits, a refused bus solo and track solo changes in one refused batch. It asserts a full per-strip
  snapshot plus `solo_count` after the rollback.
- My mutation M3' (rollback restores `user_mute`/`emitted` only for entries that are not
  solo-safe) turns it red.

## Live-control producers (the acked-batch question)

- **Bounded and charged.** Every bus gets the same three bounded SPSC rings at
  `control_queue_depth`. They are charged per request in `builtins-compiler::resource_plan`, which
  iterates `controls` (`crates/builtins-compiler/src/lib.rs` near `:3600-3640`), and they count
  against `maximum_builtin_retained_bytes`.
- **No ack can precede a drop.**
  - host-core acks nothing: `try_push` returns `Err` on a full ring.
  - host-web's admission is unchanged. The guard is `track >= ready.tracks.len()`; every slot is
    computed from a track index below `T`; the free-room pass runs before any push. Bus producers
    are therefore unreachable from commands until #1213, and `ready.controls[..T]` is
    index-identical to the old `track_controls`.
- **Render side.** No render code changed. Bus strips' consumers go through the same
  `StripControlConsumers` → bank/scalar lowering as tracks.
  - Scratch `zz_bus_records_drain_without_allocating` pushes a fader ramp, mute, trim, polarity
    and a matrix onto the bus every block, in both delivery modes. Under `engine/realtime-audit` and
    `bench_support::alloc`, blocks 1 and later read exactly `(0, 0, 0)`.
  - Sensitivity check: a 9-byte `vec!` inside the render scope aborts the process (SIGABRT), so the
    harness is live.
- **Tail.** A controlled bus's `PostInputBuiltins` tail becomes `Infinite`, as a controlled
  track's already is. It only feeds `output_tail` reporting (`graph-compiler/src/pdc.rs`), and
  with live controls the tracks already make that tail infinite. No behavioural change.

## Gates (re-run by me on `4ee4d3f30`, x86-64 AVX2+FMA)

| Gate | Command | Result |
|---|---|---|
| 1 | `cargo test -p host-core --lib solo` | 10/10 ok |
| 2 | `cargo test -p host-core --test strip_controls` | 1/1 ok |
| 3 | `cargo test -p host-core --test randomized -- --nocapture` | ok. `12 seeds: Reach { consoles: 12, refused: 0, armed_collapse_blocks: 53, live_records: 180, bus_console_entries: 11, bus_live_records: 6 }`, identical to the record |
| 4 | `cargo test -p host-core` and `cargo test -p host-web`, default features | all ok (host-core: 58 lib + every integration binary; host-web: 129 passed, 1 ignored) |
| 4 | `cargo test -p host-core -p host-web --all-targets --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,engine/realtime-audit` | all ok (host-web: 132 passed, 1 ignored) |
| 5 | `cargo fmt --all -- --check` | rc 0 |
| 5 | `check-/test-host-core-policy.sh`, `check-/test-realtime-policy.sh`, `check-/test-workspace-policy.sh` | all ok (realtime: 54 marked regions in 15 files) |
| 5 | `cargo clippy --locked -p host-core -p host-web --all-targets --all-features -- -D warnings`; `-p host-core --all-targets` (default features); `-p capi -p parameter-metadata --all-targets --all-features` (host-core's other reverse dependencies) | clean |
| 5 | `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core -p host-web --all-features --no-deps` | clean |

### Not run by me

- **The full DESIGN section 7 workspace command.** The disk is at about 17-20 GB free.
  - Terra records rc 0 (1162 passed).
  - host-core's only other reverse dependencies are `capi` and `parameter-metadata`. Neither reads
    the renamed field or `LiveControlSoloState`, and both pass clippy above.
- **`run-aarch64-tests.sh debug`.** There is no arm64 host; this is CI `aarch64-debug` at the K2
  push.
- **`cargo doc` on host-core without `control-provider`.** It fails on a broken intra-doc link
  `SessionControlProvider` at `crates/host-core/src/lib.rs:19`. That link predates this diff (the
  diff touches only `lib.rs:148`), and the workspace doc gate unifies the feature, so it does not
  affect this slice.

## Mutations (each applied, run, and restored byte-for-byte)

| # | Mutation | Result |
|---|---|---|
| X1 | drop `!self.solo_safe(strip)` from `effective_mute` (the bus is solo-muted) | RED: `a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition` (and my end-to-end probe: silent output under solo) |
| X2 | move `self.shadow()` above the solo-safe guard (a refused bus solo opens a transaction) | RED: `a_solo_safe_strip_cannot_be_soloed` (`transaction_open`) |
| M3' | `rollback` restores `user_mute`/`emitted` only for entries that are not solo-safe | RED: `rollback_restores_every_strip_including_the_solo_safe_ones` |
| M5 | control requests from `live_control_tracks` again | RED: `a_bus_fader_record_renders_as_the_session_with_that_fader` (`left: ["t0","t1","t2"]`); `handles_list_tracks_then_submixes_and_file_bus_effects` (`right: [... "aaa-bus", "zzz-bus"]`); `randomized` (`bus_live_records: 0`) |
| B1 | `builtins-compiler`: the bus's fader consumer is never bound (`fader: (strip.id != "bus").then_some(..)`), so records are accepted but never drained | RED: `a_bus_fader_record_renders_as_the_session_with_that_fader` at `block 2` |
| W1 | host-web seeds tracks `solo_safe: true` (a D3 violation) | RED: 12 host-web solo tests (e.g. `solo_is_bit_identically_mute_on_the_complement`, `a_refused_solo_submission_leaves_the_live_controls_untouched`) |

Gate 2's twin equivalence is not vacuous:
- blocks 0-1 assert `assert_ne!` against the -6 dB twin;
- an undrained bus queue (B1), a missing bus producer (M5) or a misplaced producer (M6 per the
  record) turns it red;
- a broadcast to every strip would render -12 dB on the tracks' path and also differ.

## Test value (one sentence each)

- `solo::tests::a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition` — red if a
  bus takes a solo-derived mute, the defect that would silence a soloed track's own bus path (X1).
- `solo::tests::a_solo_safe_strip_cannot_be_soloed` — red if a bus solo is accepted, counted
  toward `any_solo`, or opens a transaction on refusal (X2, M1, M4).
- `solo::tests::rollback_restores_every_strip_including_the_solo_safe_ones` — red if a refused
  batch's rollback skips a bus entry's user mute or emitted mirror (M3').
- `strip_controls::a_bus_fader_record_renders_as_the_session_with_that_fader` — red if a submix
  gets no producer, if its producer is not at its strip index, or if its fader queue is never
  drained or lands on a track (M5, M6, B1).
- `strip_handles::handles_list_tracks_then_submixes_and_file_bus_effects` (rewritten assertion) —
  red if the builtin controls stop being parallel to the strips, or put two submixes out of
  canonical order (M5). The old "none per bus" claim is superseded, and rewriting it in place is
  the right supersession.
- `randomized` `Reach.bus_live_records > 0` — red if the generator stops driving a bus producer
  (M5 gives `bus_live_records: 0`).

## Findings

### MINOR-1: `LiveControlSoloState::track_count()` now counts strips, beside `HostLiveControlHandles::track_count`, which counts tracks

- One crate now has the same name with two meanings. `track_count()` returns `solo.len()`, which
  is the strip count. `track_delta(strip)` is per strip.
- The deviation is honestly recorded, and the spec did not require the rename. Today the two are
  equal in host-web, because D3 seeds tracks only.
- The trap: #1213 seeds `T + S` entries. Any host code or test that reads `ready.solo.track_count()`
  as "tracks" then silently spans the buses. The existing callers are `hosts/host-web/src/tests.rs`
  (`:4259`, `:4325`) and `lib.rs`'s `track_delta` loop (`:4670`).
- **Fix:** rename them to `strip_count()`/`strip_delta()` in #1213, which already owns both
  host-web files and rewrites the solo emission loop. Add one line to #1213's spec so the rename
  is not lost.

### NIT-1: the "one place" doc claim is true only after #1213

The module and method docs say `effective_mute` is the only place the formula is written. host-web
still composes `muted || (any_solo && !solo(track))` inline at kind 4 (`lib.rs:4433` at this
commit). The copy is equivalent for tracks (`solo_safe` is false), the spec mandated the doc
wording, and #1213 deletes the copy (its gate 3). No action beyond #1213.

### NIT-2: thin bus reach, and an arm-versus-arm differential

- `bus_live_records: 6` over 12 seeds meets the gate (more than zero in at least one seed). But the
  three arms share any bus-record defect, so the differential guards collapse-arming on a bus, not
  bus-record correctness.
- The bit-level correctness of a bus's matrix, trim and polarity therefore has no committed oracle
  in this slice. My scratch twins show all of them correct in both delivery modes, and #1213 gate 1
  covers every kind on a bus with paired hosts.
- No action, unless the root wants the cheap guard of folding the scratch `drive(...)` variants
  into `strip_controls.rs`.

### NIT-3: a doc edit outside the cited lines

The `ReadyOwnership.controls` field doc edit in host-web is outside the cited lines. It is
doc-only, accurate ("parallel to the strips; commands address only the track prefix") and recorded.
Accept.

### NIT-4: each host spells the `solo_safe` rule itself

The rule (tracks `false`, submixes `true`) is left for each host to spell when it builds
`StripMuteSeed`s. host-web will in #1213, and capi will once it gains a solo state. A host-core
helper that derives the seeds from the compiled model, in strip order, would keep the source
semantics in one place. This is a candidate for #1213 or the capi slices, not this one.
