PASS

# #1479 attempt 1 -- adversarial verdict

Commit `9dd5ae182` (parent `0e01f7b4c`), worktree `/home/bl/misofm/wt-d15-misc`. I reviewed an exported tree
(`git archive 9dd5ae182`) at `/tmp/claude-1002/v1479/tree` and never built in or changed the worktree. The other
implementer's uncommitted edits in the worktree are not part of this review. The diff touches four files, all in
the spec's authorized paths: `crates/host-core/src/spectrum.rs` (`entry`, its doc, `selected_entry` removed, one
private helper `owned_entry`, and `select`'s two `self.entry(index)` sites), `crates/host-core/tests/spectrum.rs`
(the two `entry` assertions only), `hosts/host-web/MUTATIONS.md` (row `:563` only) and the spec.

## Amendment

The spec's Amendment text agrees word for word with root's ruling in
`scratchpad/misc/1479-root-amendment.md` (whitespace-normalized comparison: equal). The context paragraph agrees
too. Gate 4's wording now names `select` as the remaining internal caller, as root asked. The Authorized-paths
line was widened to the Amendment's grant and no further.

## Decisions

- **D1 holds.** `selected_entry()` is gone. No Rust caller remains on this branch. `git grep` over every local
  and `origin/` branch finds `selected_entry()` only on branches whose base predates #1333 A1, at the same
  `lib.rs` line that main already replaced. None of the active d15 stream branches (a, b, j, j2,
  followups-j) changes `crates/host-core/src/spectrum.rs` against its merge base, so no caller was added
  elsewhere and no hot-file conflict is pending.
- **D2 holds.** `entry(index)` returns `Option<(&SpectrumTarget, SpectrumChannels)>` from the borrowing
  `target()` and `channels()`. It clones and allocates nothing. The public collection API now has no cloning
  accessor. No parallel public variant was added: `entry` changed in place, and `owned_entry` is private.
- **Root's Amendment holds.** `select`'s signature is unchanged. Both sites call `owned_entry(index)`, which
  builds `SpectrumCaptureCollectionEntry { target: target.clone(), channels }` from `entry`'s pair. This is the
  same single `String` clone as before, at the same two points, with the same `UnknownEntry` mapping. No
  allocation is added. The borrowed-form `select` (out of scope) was not attempted.
- **D3 holds.** The new row's mutation compiles and is red with the recorded message and count (below).
- **Callers.** `entry` is called only at `tests/spectrum.rs:738/742` and in `owned_entry`. host-web, capi and
  the TS SDK do not call it. The SDK reaches the collection only through the wasm exports, whose Rust side
  (`select_spectrum`, `lib.rs:2589`) calls `select` and discards its `Ok` value.

## Findings

No BLOCKER, MAJOR or MINOR.

- **NIT 1 -- `crates/host-core/src/spectrum.rs:869` says "on the control thread".** In the browser,
  `select` runs from the `AudioWorkletProcessor`'s `port.onmessage` (`miso-engine-v1-audio-worklet.js:266`,
  `:1313`). That is the audio rendering thread, between quanta, outside `process()`. The struct's own
  vocabulary at `:830` ("an exclusive control-side operation ... between render calls") is accurate. "on the
  control thread" is not accurate for the browser host. The attempt record has the same phrase. Behaviour is
  not affected, and root ruled the clone itself unchanged.
- **NIT 2 -- `docs/handoffs/decision-15-2026-10-05/STREAMS.md:83` is stale.** The hot-file row still says
  #1479 edits "`entry` and `selected_entry` only". After the Amendment, the slice also edits `select`'s two
  return sites and adds `owned_entry`. Both are outside every marked region (the only `REALTIME_POLICY` markers
  in the file are at `:1574-1630`). The file is root's and is outside this slice's paths, so root refreshes the
  row.

Observations (not findings against this attempt):
- `select`'s existing clone therefore allocates on the browser's audio thread, outside the render callback.
  Root ruled this path unchanged and the borrowed form out of scope. If root wants that allocation gone, it
  needs a separate issue.
- I did not run the Chromium `render-allocations` gate. The spec's statement that it stays a runtime guard
  is from before this change, and I did not verify it again here.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core` alone fails at `lib.rs:19` (`SessionControlProvider`
  is behind the `control-provider` feature). This failure predates the commit, and `lib.rs` is untouched.
  CI's workspace `cargo doc` turns the feature on through feature unification. With
  `--features host-core/control-provider`, `host-core` and `host-web` document cleanly, so the new
  `[`SpectrumCaptureCollectionEntry`]` link resolves.

## Test value

- **The rewritten assertions (`tests/spectrum.rs:737-744`).** Assume `entry` reports the wrong channel mask
  for a prepared index (mutation: `.map(|capture| (capture.target(), SpectrumChannels::Stereo))`). That turns
  `:741` red (`left: Some((TrackPostInputBuiltins("eq0"), Stereo))`, `right: ... Left`). Every other host-core
  and host-web test stays green, because no test checks the entry that `select` returns. The implementer's
  recorded `index ^ 1` mutation also turns `:737` red, as recorded. The lib test
  `explicit_collection_start_uses_the_checked_hop` catches it too, through `select`, and the record says so.
  The record's claim ("red if `entry` reports a wrong target or mask; no new catch") is true.
- **Phase 2 (`render_locked_staging`, row rewritten by D3).** A collection spectrum read that clones the
  selected target turns phase 2 red. With D3 applied, the whole native host-web suite has phase 2 as its only
  red test (lib 185 passed, `boot_transient_budget` 2, `retained_ceilings` 1).

## Gates run (on the export; native debug builds with `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0` for disk)

1. `cargo test --locked -p host-core`: all binaries ok (lib 73; `spectrum` 9; one ignored test each in
   `effect_observation` and `prepare`, which predate the commit). `cargo test --locked -p host-web --test
   render_locked_staging`: 1 passed.
2. `cargo clippy --locked -p host-core -p host-web --all-targets -- -D warnings`: clean. With
   `-p capi --all-features` added: clean. `cargo fmt --all -- --check`: clean.
   `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (89 marked regions in 25
   files).
3. D3 mutation in `PreparedSpectrumCapture::channels` (`hosts/host-web/src/lib.rs:1503`): red,
   `render_locked_staging.rs:233`, "a collection capture's spectrum read allocated", `left: 4`, `right: 0`.
   I restored the file (byte-identical to the commit) and the test was green again (1 passed).
4. `cargo build --locked --workspace --all-targets`: exit 0, no warnings.
- `scripts/check-cross-targets.sh`: PASS (the expected #1018 failures only).
- Worklet chain (`host-core` is in the worklet), with the CI invocations into empty output directories:
  `build-web-audioworklet.sh --named-twin` exit 0 (shipped `6175e70c63e5...`, 3,129,044 B);
  `check-web-audioworklet.sh --without-metadata-regeneration` exit 0;
  `check-browser-expected-resources.py --artifacts` exit 0 (digests and exact rows agree; self-test with 32 red
  mutations); `test-web-audioworklet.sh` exit 0, and the private TMPDIR was empty after the run.
- Mutations: `entry` with `index ^ 1` and with a hard-coded `Stereo` mask, as given above. I restored each
  mutated file byte-identical to the commit after its run.

Evidence logs: `/tmp/claude-1002/v1479/*.log`.
