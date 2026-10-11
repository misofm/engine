PASS

# #1432 follow-ups after the follow-ups verdict: commit 48a13bbc1 (parent 030478400), branch codex/d15-stream-b2

Verifier: adversarial, read-only on the worktree. The commit was exported to /tmp/claude-1002/v1432fu2/tree and built there.
Evidence (kept): /tmp/claude-1002/v1432fu2/ev/ (summary.txt, loom/loom_da/test/crp/trp/fmt/clippy/doc/wsp/cross logs, probes.log, mut/*.diff and mut/*.log, unit_list.txt).
Root's spec-only amendment 030478400 is not judged here. It is read only for the fit question.

The borrow is sound. It refuses every stale-stamp path I could build in safe code (16 probes). The compile-fail doctest goes red on the claimed mutation and on removal of its stale line. m1 and n1 are closed. One record in n2 is now wrong, which is a MINOR. I found no BLOCKER and no MAJOR.

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. The n2 "correction" makes the unit-test counts wrong. They were right before.**
  - Locations:
    - `.github/ISSUE_SPECS/1432-...md:300` says "17 unit tests (`latest_cell::tests`; the first record said 12 in error) and 5 loom models".
    - `:425` says "all 22 unit tests ... The count was 17 at attempt 1 and is 22".
    - `:495` says "The real counts are 17 before and 22 after".
    - The commit message repeats these counts.
  - Measured counts (`#[test]` in `latest_cell.rs`, loom models split out):

    | Commit | Unit tests | Loom models | All tests |
    |---|---|---|---|
    | 194fe80d8 | 12 | 5 | 17 |
    | 203ec4194 | 17 | 5 | 22 |
    | 48a13bbc1 | 17 | 5 | 22 |

  - `cargo test -p engine --lib latest_cell -- --list` lists 17 tests (`ev/unit_list.txt`).
  - So the first record ("12 unit tests") was correct. The real change is 12 unit tests before and 17 after (5 added, 2 rewritten, none deleted). The total goes from 17 to 22 only when the 5 loom models are counted.
  - The previous verdict's n2 said "the test list" and did not separate unit tests from loom models. That started the error.
  - Fix, spec only: on line 300, restore "12 unit tests"; on lines 425 and 495, say "17 unit tests (22 with the 5 loom models)". Do not rewrite the commit message.

## NIT

- n1. **Stable rustdoc does not check the doctest's `E0499` code.**
  - Mutation m2_errcode changed the code to `E0502`. The doctest stayed green (`ev/mut/m2_errcode.log`). Rustdoc checks error codes only on nightly, and the doctests run on the stable 1.97.1 pin.
  - The defence today is only that the example does not compile. A later API change could make it fail to compile for a different reason, and the test would pass with no real check.
  - The passing doctest is a partial control: it uses the same imports and calls.
  - A tighter control is the one-line difference. Mutation m3 removed the stale `write` line, and the example then compiled, so the test went red. That confirms the doctest is valid now.
  - Suggest one sentence in the spec: "the code is not checked on stable; the control is the example minus its last line".
- n2. **"The check guards a stamp from another pin" (`latest_cell.rs:473-474`) claims too much.**
  - `write`'s assert catches a stamp from another pin only when that stamp's revision is below one already written to the cell.
  - A stamp from another pin with a higher revision has wrong `pinned` and `published` facts. No check catches it, and its write can target the slot that render reads.
  - The real protection is #1502 D1: only the handle's holder stamps writes to that plan's cells. Suggested wording: "...catches a stamp from another pin only when its revision is below one already written".
- n3. **A residual path that is not a kept stamp: re-stamping a revision that is already published.**
  - The sequence `stamp(1)`, write, `publish(1)`, `stamp(1)`, write compiles (probe p15). That is correct: it is a fresh stamp, not a stale one.
  - Only `stamp`'s debug assert refuses it. In a release build, the in-place rule then rewrites the published slot.
  - This is not new. The previous verdict accepted the debug-only check, and #1312 D7 (as amended) plus #1503's bound keep callers from doing it.
  - It is recorded only so that no one reads "the compiler refuses a stale stamp" as covering this case.

## Judgments on the questions asked

1. **The borrow is sound.**
   - `LiveStamp<'p>` is covariant in `'p` and holds the `&mut self` loan of `stamp`. Every `Copy` of the stamp carries the same loan.
   - `publish` and `stamp` are the only `&mut self` methods of `GatePin`. `pinned` and `last_published` change only in `publish`. So "a later publication" is the only way a stamp's facts can go stale, and the borrow forbids exactly that.
   - `GatePin` is not `Clone`, its fields are private, and `pair` is its only constructor. So no second pin exists for one gate.
   - Probes (`ev/probes.log`). Each one compiles only when it should:
     - **Refused with E0499:** the stamp copied into a struct (p01), captured by a closure (p02), held in an `Option` (p14), pushed into a `Vec` (p16), or leaked to `'static` through `Box::leak` of the pin (p06). Also refused: `publish(&gate, stamp.revision())` inline (p12). The two-phase borrow does not help here, which confirms the "read `r` first" rule.
     - **Refused with other errors:** `thread::spawn` needs `'static` (p03, E0597). Moving the pin while a stamp is alive is E0505 (p05). A struct literal outside the module is E0451, private fields (p07). A function that extends the lifetime is a lifetime error (p08).
     - **Compiles, as it should:** a stamp used inside `thread::scope` and then `publish` (p04).
2. **The forging needs no test-only constructor.**
   - The test uses the struct literal, which only the module and its child `tests` module can use (fields private; p07).
   - The production code has one constructor site, `stamp` (`:299`). The loom models build no stamp by hand.
   - The rewritten `a_write_below_a_written_revision_panics` is still the only test that defends `write`'s below-written assert. Mutation m4 changes that assert to always true: only this test goes red, and the other 16 unit tests stay green.
3. **The shape fits #1502 D1, D5 and D6 and #1312 D7 as amended in 030478400.**
   - **Live commit:** `control.rs:1293-1298` already destructures `ProviderEpoch { strips, effects, .. }`. If D6 adds a gate field to it, the stamp from that field and the writers in `strips` are separate field borrows (probe p10 compiles).
   - **Candidate path:** stamp `r` inside preparation, write the retargets, and let the stamp die. The fallible steps, any moves of the handle and the later `publish(r)` on the same handle then all compile (probe p11). This includes moving the handle to another thread, publishing there, and stamping again.
   - **Refused moves:** moving the handle while a stamp is alive is refused (p05). D7's own rule ("a handle moves ... only with no open stamp") is about `GatePin.open`, not the borrow. It stays the structural and gate-2/gate-9 contract, and the borrow does not change it.
   - **One limit for the #1502 and #1312 implementers:** a `&mut self` method on a struct that holds both the handle and the writers cannot run while a stamp from that struct is alive (probe p09, E0499).
     - Use separate field borrows (destructure the struct) or one method that stamps and writes.
     - This is friction, not a blocker, and today's code already uses the destructuring form.
4. **Mutation re-runs** (`ev/mut/`; doctests run with `cargo test -p engine --doc realtime::latest_cell`).

   | Mutation | Result |
   |---|---|
   | m1: `stamp` returns `LiveStamp<'static>` (the claimed row) | Red. The compile-fail doctest "compiled successfully". |
   | m2: the doctest code changed from `E0499` to `E0502` | Green (NIT n1). |
   | m3: the stale `write` line deleted from the compile-fail example | Red. The defence is the stale write, not a typo. |
   | m4: `write`'s below-written assert made always true | Red: only `a_write_below_a_written_revision_panics`. |

   After each mutation the file was restored from a copy, so the export stays the committed source.
5. **m1 (spec D1): closed.**
   - Step 1 now says "`A` = greatest mirrored revision at or below `stamp.published`".
   - That matches the `stamp` bullet and `CellWriter::target` (`latest_cell.rs:449`).
6. **The previous n1 (gate-identity wording): closed.** It is softened correctly, and it points to #1502 D1 and D5 as the real protection.

## Test value

No new test. Test-value sentences for the changed tests:
- **Compile-fail doctest (`LiveStamp`, line 117):** it turns red when a stamp no longer borrows its pin, so a write with a stamp made stale by a later `publish` compiles again (m1). No other test catches that, because no run-time test can.
- **Passing doctest:** it is the control. It turns red if one stamp can no longer serve several cell writes before `publish`, for example if the stamp lost `Copy`.
- **`a_write_below_a_written_revision_panics` (rewritten):** it turns red if `write`'s debug assert against a revision below one already written is removed (m4). That assert is the only guard against a regressing stamp from another pin. No other test catches it.

## Gates run (all on the exported tree, toolchain 1.97.1)

| Gate | Result |
|---|---|
| CI loom command (`RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`) | 13 passed |
| The same command with `-C debug-assertions=on` | 13 passed |
| `cargo test --locked -p engine --features realtime-audit` | 75, 4 and 1 passed. Doctests: the `LiveStamp` pass example, plus both compile-fail examples (`LiveStamp` and `PreparedRenderPlan`), are green. |
| `check-realtime-policy.sh` | ok (96 regions, 27 files) |
| `test-realtime-policy.sh` | ok |
| `cargo fmt --all --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `check-workspace-policy.sh` | ok |
| `check-cross-targets.sh` | PASS, with the #1018 expected failures |
