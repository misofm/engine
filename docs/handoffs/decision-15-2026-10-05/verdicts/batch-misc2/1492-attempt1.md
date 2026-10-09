PASS

# #1492 attempt 1 -- adversarial verdict

Commit `b0256b89d` (parent `40a1ead6f`), branch `codex/d15-batch-misc2`, worktree `/home/bl/misofm/wt-d15-misc2`.
I worked only on an exported tree (`git archive b0256b89d`), plus exports of `40a1ead6f` and `1d294c700` for the
row `:563` re-measurement. I did not build in, edit or check out the worktree. I verified the export matched the
commit byte for byte before the gate runs. Evidence (logs, mutation runner, probe note, browser runner):
`/home/bl/misofm/submix-verdicts/evidence/1492-attempt1/`.

## Decisions

- **D1 holds.** `SpectrumTargetRef<'a>` (`spectrum.rs:222`, `Clone, Copy, Debug, Eq, PartialEq`) has the same three
  kinds over `&'a str`. `SpectrumTarget::as_ref()` (`:205`) maps kind to kind and borrows the `Box<str>`. Both
  enums derive `PartialEq`, so `capture.target.as_ref() == target` compares the variant and then the identity bytes,
  exactly as `capture.target == *target` did. The mask comparison is unchanged. `select` and
  `selection_would_change` take `SpectrumTargetRef<'_>`. The old signatures are gone: no `select(&`,
  `selection_would_change(&` or `select_spectrum(&` remains in the workspace, and no other host or crate calls them.
  No parallel API.
- **D2 holds.** `select` returns `Result<usize, _>`; the repeat path returns `Ok(index)` where it used to build an
  owned entry that was always `Some`. Same behaviour otherwise. `owned_entry` is removed. The `entry` and `select`
  docs name the browser audio thread.
- **D3 holds.** `select_spectrum_with_smoothing` calls `spectrum_target_ref` (`ffi.rs:937`); `select_spectrum` and
  `spectrum_selection_would_change` take `SpectrumTargetRef<'_>`. `spectrum_target` (`:950`) keeps its two
  boot-time callers (`:993`, `:1058`) and now builds its owned target from `spectrum_target_ref`. Same validation,
  same result codes; it no longer allocates before it refuses an unknown kind.
- **D4 holds.** Both exports wrap their whole bodies in `render_locked` (`ffi.rs:2974`, `:2990-2999`). Nothing
  else calls `select_spectrum_internal` or `select_spectrum_with_smoothing`, so the windows cannot nest. The `ffi.rs`
  header now lists the two selects in the spectrum-observer bullet and names two exceptions (`dispose`,
  `render_allocation_count`). With #1488's export-by-export map (verdict `1488-attempt1.md`, D2), the two selects
  were the only other post-boot unwrapped calls, so both headers are now exactly true. `render_lock.rs` says the
  same.
- **D5 did not trigger.** Gate 1 reads zero. A probe (not committed) adds three more select legs after phase 2: a
  smoothing-only stream select on the same entry (the bridge's `restart_spectrum_stream` branch), a plain select
  while the stream is active, and a refused select. Each one reads zero (`probe-extra-legs.*`). The attempt
  record's "No other allocation is on the select path" is true.
- **Helpers.** The private `position` (`spectrum.rs:898`) is the shared lookup of `select` and
  `selection_would_change`. It sits where `owned_entry` was. `spectrum_target_ref` is D3's "builds a
  `SpectrumTargetRef` from the staged bytes" put in a function, so validation stays in one place. Both are inside
  the scope of D1 and D3. No `REALTIME_POLICY` region moved: every marked region in `spectrum.rs`, `ffi.rs` and
  `lib.rs` is byte-identical to the parent's. The `spectrum.rs` region moves down 18 lines only (1575-1631 to
  1593-1649), and nothing pins those lines. `check-realtime-policy.sh`: 89 regions in 25 files before and after.
- **Acked-batch question.** No queue changed. A select commits or returns a refusal code; nothing is acked.

## Findings

No BLOCKER. No MAJOR.

**MINOR 1. The `MUTATIONS.md:563` edit is outside the authorized paths, but its content is correct.** The spec
allows "`hosts/host-web/MUTATIONS.md` (the new rows)". STREAMS row 101 gives row `:563` to A #1479 ("each adding or
editing its own rows"). The attempt record discloses the edit and leaves the move to root. I re-measured the
row's mutation (`r563` in `mut.py`):
4 at `1d294c700` (before #1488), 6 at `40a1ead6f`, and 6 at `b0256b89d`. The failing assertion is the same each
time (`a collection capture's spectrum read allocated`). The stated cause is true. #1488 (`cca903769`) wrapped
`spectrum_stream_start`, whose body reads `AudioWorkletEngineHost::spectrum_channels` (`ffi.rs:3099`). The mutated
clone's two allocator calls there are now counted, so the row was false from `cca903769` on, and #1488's verdict
did not catch it. Recommendation: root ratifies the edit (6 is the true number), or gives it to A.

**NIT 1. The `render_lock.rs` header edit is outside the spec's authorized-path list, but it is needed.** The spec
lists only the `ffi.rs` header. Without this edit, `render_lock.rs:7-9` would still name the two selects as
exceptions, which would be false. The edit is correct, and stream H owns `hosts/host-web`. The spec left this file
out of its list.

**NIT 2. `render_lock.rs:9` is 124 columns.** The rest of the header wraps at 99 or fewer columns. The paragraph
was not reflowed after the edit. rustfmt does not wrap comments here, so no gate sees this.

**NIT 3. `ffi.rs:37` is a short line (74 columns).** The text was removed from the "named exceptions" paragraph
but the paragraph was not reflowed.

**NIT 4. Phase 2 does not measure the smoothing-only stream select.** Gate 1 lists the legs exactly (a change, a
repeat, and a stream select that changes the entry and its smoothing), and the test has those legs. The bridge
has one more select branch: a stream select that changes only the smoothing (`ffi.rs:2903`,
`restart_spectrum_stream`). A mutation that allocates only there (`mrestart`) leaves gate 1 green. The probe above
shows it reads zero today. The test-value sentence is still true as written: its two named defects (a cloned
owned entry, an owned target built from the staged identity) are on the shared path. A fourth leg would close the
gap.

## Test value

- Phase 2 select legs (`hosts/host-web/tests/render_locked_staging.rs`): **Which plausible defect turns this red
  that no existing test catches?** An allocation on the browser audio thread in a collection select, in host-core
  `select` or in the ffi bridge's select path. Examples: building an owned entry or cloning the target, building
  an owned `SpectrumTarget` from the staged bytes, an allocation on the repeat path only, or one in the continuous
  restart when the selection changes. Before this slice, phase 2 read `before` after its select and neither export
  was render-locked, so no native test counted a select.

Re-done mutations, each applied alone to the export, with every touched file's mtime refreshed before each build
and again after each restore (`runmut.sh`):

| Mutation | Gate 1 | Other host-core / host-web tests |
|---|---|---|
| m1: `select` runs `let _ = self.entry(index).map(\|(t, c)\| (t.clone(), c));` after it finds the entry | red, `the first select allocated`, 2 | all green (`m1-all.txt`) |
| m2: `select_spectrum_with_smoothing` runs `let _ = spectrum_target(target, id);` before it borrows | red, `the first select allocated`, 2 | all green (`m2-all.txt`) |
| m3: m1 kept, both exports unwrapped (D4 reverted) | green (the wrap is what lets the count see it) | -- |
| m2u: m2 kept, both exports unwrapped | green | -- |
| mrepeat: allocate only on `select`'s repeat (no-change) path | red, `the repeated select allocated`, 2 | -- |
| mcadence: allocate only in `select`'s continuous-restart branch | red, `the stream select allocated`, 2 | -- |
| mrestart: allocate only in the bridge's smoothing-only restart branch | green (NIT 4) | -- |
| r563 (row `:563`) | red, spectrum read, 6 (4 at `1d294c700`) | -- |

The rows recorded in `MUTATIONS.md` (2, 2, green) match. One earlier run of m2 read 4. The cause was in my runner:
the restore gave `spectrum.rs` an old mtime, so cargo reused a host-core build that still had m1 in it. The
touched re-run reads 2.

## Gates run (on the export, `CARGO_TARGET_DIR=/tmp/claude-1002/v1492/target`)

- Gate 1, `cargo test --locked -p host-web --test render_locked_staging`: 1 passed.
- `cargo test --locked -p host-core`: exit 0, 0 failed. `cargo test --locked -p host-web --all-targets`: exit 0
  (lib 185 passed, 2 ignored; `boot_transient_budget`, `render_locked_source`, `render_locked_staging`,
  `retained_ceilings` pass). Also with CI's features (`host-web/test-support,host-core/test-support,
  engine/realtime-audit`), `--all-targets` and `--doc`: exit 0.
- `cargo fmt --all -- --check`: exit 0. `cargo clippy --locked --workspace --all-targets -- -D warnings`: exit 0.
  `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p host-core -p host-web --features
  host-core/control-provider`: exit 0.
- `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-realtime-policy.sh`: ok (89 marked regions in
  25 files). `bash scripts/check-cross-targets.sh`: `cross-target matrix: PASS` (the #1018 expected failures only).
- Worklet chain, qualification.yml's exact invocations: `build-web-audioworklet.sh --named-twin ...` exit 0, shipped
  module `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939` (3129171 B; the implementer's digest
  reproduces), named twin `3d6c0068...`; `check-web-audioworklet.sh --without-metadata-regeneration ...` exit 0;
  `check-browser-expected-resources.py --artifacts ...` exit 0; `test-web-audioworklet.sh` under a fresh private
  `TMPDIR` exit 0, nothing left in it.
- Browser qualification in the CI step's shape (private pulseaudio null sink, node_modules copied into the export,
  `--check-matrix --self-test-mutations`): Chromium 151, Firefox 153.0 and WebKit 26.5 all exit 0, `all
  qualification gates passed`. Every `render-allocations` and `sdk-render-allocations` row reads 0, and
  `spectrum-collection=0` in all three.
