PASS

# #1422 *Run doctests in CI*: verdict, attempt 1, the deferred `plan.rs` pair (12 of 12)

Commit `08b5f478b` on `codex/d15-stream-j3` (parent `95cb7bad5`), reviewed as `git show 08b5f478b`
against the spec (D2, Amendment 1, the Attempt record and its new "The `plan.rs` pair" section), the 11
earlier pairs, AGENTS.md, decision 15 and the owner principle. (`git diff 95cb7bad5 08b5f478b` also
shows root's `9bbc551a0`, two spec-cite pins in #1438 and #1446; not part of this slice.) I exported
`08b5f478b` to `/tmp/claude-1002/v1419/tree` and the parent to `/tmp/claude-1002/v1419/ptree`, and
built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1419/target`. I did not write to the worktree. The
other 11 pairs are verdict `1422-attempt1.md`, unchanged here.

No BLOCKER. No MAJOR.

## Findings

- **MINOR-1: the recorded `test-debug-a` doctest count does not match the tree.** Spec, "The `plan.rs`
  pair", Gate 1 paragraph: "the `test-debug-a` doctest command's expected count is 19 (graph-compiler
  6 + 6, host-core 3 + 2, plan.rs 1 + 1), plus whatever stream B and later slices add." At the commit
  the exact `qualification.yml:637-647` command runs **23** doctests, all ok: the 19 named plus
  `crates/host-core/src/route_controls.rs` lines 43, 58, 72 (`compile_fail`) and 86 (twin), which #1416
  landed (`34274bf34`, `b42e6b8c1`, `b587ba04a`) and which are already on `main` at `2a589d360`, the
  branch base. They are not a "later slice". Correct the count to 23 and name the #1416 pairs (or drop
  the number). Not a defect in the code.

## Scope and shape

- `crates/engine/src/realtime/plan.rs`: only the `PreparedRenderPlan` doc comment changed (`:557-573`):
  two comment lines naming E0277 and saying stable rustdoc does not check it, the helper renamed
  `requires_sync` to `requires`, and a twin with `T: Send`. No code line moved except by the doc
  lines. This is the same shape as the `PreparedHost` pair (`host-core/src/prepare.rs:511-529`): same
  comment wording, same shared helper name, same one-construct difference (bound), same "issue #1422
  D2" twin sentence. No error code on the fence; no `RUSTC_BOOTSTRAP` anywhere.
- Concurrency with stream B batch 2: `codex/d15-stream-b2` (`203ec4194`) and its worktree
  `/home/bl/misofm/wt-d15-b2` have no committed or uncommitted change to `plan.rs` against
  `2a589d360`, so there is no overlap today. A later stream B edit near `:555-573` rebases on a
  doc-only hunk.
- The Status line and the two "pending" references in the earlier attempt record are updated to point
  at the new section; acceptable as a status update.

## Gate 3 and the reason check, reproduced (`cargo test --locked -p engine --doc`; each reverted, file
compared byte for byte with the commit afterwards)

| Run | Result |
|---|---|
| Base | exit 0; line 560 `compile_fail` ok, line 569 twin ok |
| Rename `PreparedRenderPlan` to `PreparedRenderPlam` in both `use` lines | exit 101; twin 569 FAILED (E0432, E0425); `compile_fail` 560 still ok (the wrong-reason defect) |
| Delete: `T: Sync` to `T: Send` in the `compile_fail` | exit 101; `compile_fail` 560 FAILED; twin ok |
| Guarantee broken: `#[allow(unsafe_code)] unsafe impl Sync for PreparedRenderPlan {}` | exit 101; `compile_fail` 560 FAILED; twin ok |
| Reason: fence flipped to a plain doctest | exactly two errors, both E0277 (`Cell<()>` and `dyn PreparedPlanExecutor` cannot be shared between threads safely); no other error |

*Test value (new twin).* A rename or moved re-export of `engine::realtime::PreparedRenderPlan` (or a
typo in the shared snippet) turns the twin red while the lone `compile_fail` stays green for an
unresolved-import reason; nothing else catches that on stable. The record's test-value sentence is
true.

## Artifact

- Shipped AudioWorklet module, built by `scripts/build-web-audioworklet.sh`: parent `95cb7bad5`
  `7cbdcff00d6e2720543b85c1aa22f943559826f81f73db91a927cb352f537e44`, commit `08b5f478b`
  `9784d3509dba6bfd8188b00fc48f9ae4f20020ef9583cbc409cea1d079878d9c`, both 3129194 B (named twins both
  3561702 B). The record's `9784d350...` (3129194 B) is correct; the change is the panic `Location`
  line shift, as NIT-2 of the first verdict. Expect ARTIFACT CHANGED in `artifact-identity`.
- No committed pin needs an update. The only digest pin,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` (`6c952a2c...`), is the release
  fingerprint: it is checked only by `npm-publish.yml` and by `build-web-audioworklet.sh --check-pin`
  (#1061, docs/RELEASE.md), not by `qualification.yml`; a doc change does not re-pin it.
  `hosts/host-web/tests/browser-v1/expected.json` holds exact rows and ceilings, not the module digest,
  and `check-browser-expected-resources.py --artifacts` is green on the new module.

## Gates run (at the commit's tree)

| Gate | Result |
|---|---|
| `cargo test --locked -p engine --doc` | exit 0, 2 doctests (1 `compile_fail`, 1 twin) |
| `test-debug-a` doctest step command (`qualification.yml:639-647`, exact) | exit 0, 23 doctests (see MINOR-1) |
| Gate 3 rename / delete, guarantee break, reason check | as the table above |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked -p engine --all-targets -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p engine --no-deps` | exit 0 |
| `bash scripts/check-workspace-policy.sh` | exit 0 |
| `bash scripts/check-realtime-policy.sh` | exit 0 (93 marked regions in 26 files) |
| `build-web-audioworklet.sh --named-twin` (delivery closure) | exit 0, `9784d350...` |
| `check-web-audioworklet.sh --without-metadata-regeneration` | exit 0 |
| `check-browser-expected-resources.py --artifacts` | exit 0 (self-test 32 red mutations) |

Not run: the AArch64 leg (no AArch64 host; the pair is not target-specific) and gate 5 (the batch's PR
run).
