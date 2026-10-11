PASS

# #1472 attempt 1: adversarial verdict

- **Reviewed:** `git diff 04fc3cc8f ae3fcbf16` on `codex/d15-stream-g3` (`scripts/check-cross-targets.sh`,
  `scripts/lib/aarch64-known-defects.py`, `docs/TARGET_MATRIX.md`, the spec's Attempt record), against
  the spec at `ae3fcbf16`, AGENTS.md, the coordinator's verifier rules and root's standing rulings.
  The uncommitted #1464 changes in the worktree were ignored.
- **Method:** `git archive ae3fcbf16` to `/tmp/claude-1002/v1472/tree`, built there with
  `CARGO_TARGET_DIR=/tmp/claude-1002/v1472/target`. Nothing was built, edited or checked out in
  `/home/bl/misofm/wt-d15-g3`. After the engine mutations the three mutated files were diffed
  against `git show ae3fcbf16:<path>` (identical). rustc 1.97.1 (LLVM 22.1.6). No aarch64 binary
  was run; I linked a Mach-O dylib only to symbolize it.
- **Verdict:** PASS. D1-D4 are implemented as frozen. The post-LTO count reproduces exactly (5
  calls, lines 54806, 85369, 627638, 628665, 629997; builtins 4, lane 1). An independent check
  with LLVM's own DWARF reader agrees with the pure-Python parser on every sampled call site. No
  BLOCKER or MAJOR. Two MINORs and six NITs go to the batch follow-ups; three of them need root
  (a path outside the authorized set, or a frozen decision).

## Independent check of the parser and the attribution

- **Five calls, by LLVM's tools.** I emitted `asm` and `obj` from one `cargo rustc` (same command as
  the script), linked the object with the toolchain's `rust-lld -flavor darwin -dylib -undefined
  dynamic_lookup`, ran `dsymutil` (LLVM 18) on it, and symbolized each call with
  `llvm-symbolizer --inlines`. This gets round the unrelocated-object problem the record names.
  Innermost frames: `builtins::zero` (in `db_gain` < `prepare_sections::{closure#0}` <
  `prepare_sections` < `BuiltinChain::new`), `FaderMuteRampBuiltins::new`, `BuiltinInputBank::new`
  (file `core/slice/iter/macros.rs:0`), `BuiltinFaderBank::new` (file `core/iter/adapters/mod.rs:0`),
  `lane::kernels::builtins::lanes_below::<f32x4>` (< `InputStage::new` < `BuiltinInputBank::new` <
  `builtins_compiler::build_input_bank` < `into_graph_artifact_with_banks`). All five are charged
  as the record says (builtins x4, lane x1); the record's `.loc` and `builtins::zero` remarks are true.
- **Broad agreement.** The `--emit obj` object is not byte-identical code to the `--emit asm` text
  (register and stack-slot differences in some functions), so I assembled the emitted `.s` itself
  with `llvm-mc` (LLVM 18, `-mattr=+sha2,+aes,...`), linked and `dsymutil`-ed that, and mapped each
  asm line to its address by (function label, k-th `bl`). On 6,004 sampled `bl` sites (seeded, all
  2,120 functions with calls matched): the parser's inline-frame chain, innermost to outermost,
  equals LLVM's on 5,910 sites; 0 differ (after removing LLVM's `.NNN` symbol-table suffix on the
  outermost name); the other 94 lie in code with no DWARF (`rustc_demangle`, `core::str::pattern`),
  where LLVM also has no file information and the parser refuses (0 described bodies).
- **Crate rule.** For 23,781 distinct frames reached from 20,000 sampled `bl` sites, the outermost
  `DW_TAG_namespace` equals the crate root of the frame's v0 linkage name in every case except 7
  `__rustc` allocator-shim frames that DWARF places under `std` (both non-product).
- **Robustness.** The parser is fail-closed on what it cannot read: an undescribed function, a
  numeric range entry, a symbolic DIE reference, DWARF 5, an unknown directive or section, a
  missing label each refuse or raise, and the script fails. Count time on the real file: 8 s, 572 MB
  RSS. With 2,200 planted calls: 27 s. With 20,000 calls: about 3 min (the frame loop is
  intervals x calls). The cross-target job's timeout is 25 min.

## D1-D4

- **D1.** The counted artifact is `capi`'s `aarch64-apple-ios` release staticlib assembly, in the
  Android row's form. Fat LTO is real in it: cross-crate inline records (`builtins` into
  `builtins_compiler`) and the allocator shim are present. The label crate comes from the concrete
  subprogram's namespace rather than from demangling the label; the crate rule check above shows
  these agree.
- **D2.** Innermost product-crate frame; every non-product frame (core, alloc, std, and also `wide`
  or any other dependency) is skipped; no product frame charges the label crate, which the judge
  refuses. Recorded with the rule, the reason, the rejected alternatives and an example (629997).
  This matches D2 read with D3 ("a call attributed to no product crate fails"): a literal
  innermost-frame rule would charge `<[T]>::fill` to `core` and fail every run.
- **D3.** Rows `builtins` 4 and `lane` 1 equal the measured counts (no slack). The `host-core` and
  `soft-clip` rows were pre-link artifacts: `SpectrumAnalyzer` has 0 occurrences in the shipped iOS
  assembly, `capi` references no spectrum code, and no `soft_clip` corpus symbol ships. The new
  `lane` row re-labels the `lanes_below` call that the pre-link scan charged to `builtins` (its
  instantiating crate); the total falls 10 -> 5, as D3 orders.
- **Removing the pre-link scan is justified.** `capi` is the only native library a phone links
  (`check-release-shape.py`: shipped cdylib/staticlib set is `capi`, `host-web` (browser) and the
  wasm guest test cdylib); product crates are defined as `capi`'s closure. A call LTO removes
  cannot reach a phone; one it keeps is counted. Only one counting path remains (no parallel variant).
- **D4.** The iOS eight-lane scan reads the same post-LTO file, with the Android row's exit-status
  handling. Coverage of #1112's product claim ("no eight-lane function in the iOS and Android
  libraries", owner: "no point in having code that can't be run in the iPhone build") is kept;
  #1112's own gate 1 measured exactly this post-LTO library. What the pre-link scan alone saw was
  compiled but unshipped code (a non-generic eight-lane item no `capi` export reaches), which LTO
  strips and which never reaches a phone.
- **CI.** `qualification.yml`'s `cross-target` job still runs `bash scripts/check-cross-targets.sh`
  (required, unchanged). The two lines that `check-ci-path-routing.py` pins are kept byte for byte.
  Dropping the unpinned `count-memset` line fails closed (judge-memset cannot read the counts file).

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **`docs/TARGET_MATRIX.md:10` is now wrong about the artifact** (implementer's open item). The
   platform table still says "the release-assembly scan `ios-asm-memset-pattern16` over every
   product crate". The scan now reads one file, the post-LTO `capi` staticlib, and charges calls
   to product crates. The line is outside the authorized paths, so the implementer correctly left
   it; root should authorize a one-line follow-up, e.g. "the release-assembly scan
   `ios-asm-memset-pattern16` of the post-LTO `capi` staticlib, charged per product crate".
2. **The record's reason for the missing unit-end case is not true** (implementer's open item:
   `scripts/lib/aarch64-known-defects.py:469`). The Attempt record says a short unit "is also
   refused by the section bounds check, so no case isolates it". A case does isolate it: delete the
   compile unit's closing `.byte 0` in `synthetic_assembly()` (`text.replace("\t.byte\t0\nLdebug_info_end0:",
   "Ldebug_info_end0:")`). The length still matches the labels, the DIE tree is left open, and only
   this check refuses; with the check dropped the count passes (`{'crate-a': 1, 'crate-b': 2}`). Root's
   ruling is that every test-value claim is true: add the case, or correct the sentence. The case
   has low value (the count is right without the check; it is an integrity tripwire), so correcting
   the sentence is enough.

## NIT

1. **Tail call `b _memset_pattern16` is not counted** (implementer's open item; D1 freezes `bl`).
   It is not reachable today: a probe `#[inline(never)] fn tail_fill_probe(buf: &mut [f32]) {
   buf.fill(1.0) }` called from `into_graph_artifact_with_banks` compiles to `bl _memset_pattern16;
   ldp; ret` with a frame, not a sibcall (the call comes from the late lowering of the pattern
   intrinsic, which is not marked `tail`), and the shipped file has no `_memset_pattern16`
   reference other than the five `bl` lines. A toolchain change could alter this. A cheap
   hardening for root to rule on: count every `_memset_pattern16` reference and refuse any that is
   not a `bl` line.
2. **Two attribution rules are defended only by the real-assembly rows, not by the self-test.**
   (a) `crate_of`'s outermost-namespace rule: taking the innermost namespace keeps `--self-test`
   green (the synthetic has one namespace level); on the real file it charges `lanes_below`'s call
   to `builtins`, because the module `lane::kernels::builtins` has the name of the product crate
   `builtins`. (b) unit-relative references (`read_form`'s `value += unit`): dropping it keeps the
   self-test green (one unit at offset 0); on the real file it refuses. Gate 1's rows catch both,
   so neither is silent in CI. A nested module namespace named like another crate, and a second
   unit, in `synthetic_assembly()` would defend them hermetically.
3. **`count_memset` adds non-product crates to `counts` while it iterates**
   (`aarch64-known-defects.py:513`). After one call falls back to a non-product label crate (for
   example an out-of-line `core` generic), a later call whose chain has a frame of that crate inside
   its product frame is charged to that crate, not to the product. The run is refused either way,
   so only the message is wrong. Test membership against the product set, not `counts`.
4. **A non-numeric `.byte`/`.short` operand is stored as a zero-filled symbol, not refused**
   (`aarch64-known-defects.py:289-293`). The LEB and children reads use the raw bytes, so a hex or
   commented operand (for example verbose asm) would be misread instead of refused. All operands
   are decimal today. Only `.long`/`.quad` need symbols.
5. **Two new comment lines exceed the file's 100-column wrap**: `aarch64-known-defects.py:58` (131)
   and `:64` (111).
6. **For root (spec wording, not the implementer's claim):** the spec's Test value says gates 2 and
   3 defend "that the shipped library, not a pre-link rlib, is what is counted". Both mutations
   would also be red under the removed pre-link scan. What pins the counted artifact is the row
   values (post-LTO builtins 4, lane 1 against pre-link builtins 5, host-core 4, soft-clip 1).
   Related: with innermost-frame attribution, every fill in a generic `lane` helper, inlined into any
   crate, is charged to `lane`'s single row, so a swap (fix `lanes_below`, add a render-path fill
   in a `lane` helper) is invisible to a count ratchet. The pre-link scan charged the instantiating
   crate. This is inherent to per-crate counts and the log names each call's function.

## Test value (new self-test cases)

- `count(text, ["crate-a", "crate-b"]) == {"crate-a": 1, "crate-b": 2}`: red when the call is
  charged to the function label's crate (D2 lost), to the innermost frame with `core` included, by
  outermost-first frame order, or without following abstract origins; no other test reads the
  post-LTO format.
- `count(text, ["crate-a"]) == {"crate-a": 3}`: red, and the only case red, when only `core`,
  `alloc` and `std` are skipped by name instead of every non-product frame (mutation run here).
- `count(text, ["other"]) == {"other": 0, "crate_a": 3}`: red when calls with no product frame are
  dropped instead of charged to the label crate for the judge to refuse.
- "a call outside every described function": red when the one-body check is dropped (as an
  uncaught `IndexError`, not as an assertion; in production that also fails closed).
- "an attribute form the reader does not know": red when an unknown form is skipped as zero bytes,
  which would leave the DIEs aligned and the count silently trusted.
- "a high_pc that is not an end label minus low_pc": red when `high_pc` is trusted without checking
  that it is relative to `low_pc`.
- judge `dict(counts, core=1)`: red when the judge reads only product crates, so calls charged
  outside the closure would pass; the existing "a new crate has calls" case uses a product crate.

## Gates run

1. `scripts/check-cross-targets.sh` on the export: PASS, 3m40s from a cold target dir. Log lists
   the five calls and `expected failure (#1018): builtins 4 calls`, `lane 1 calls`.
2. Limiter mutation (`self.required_ring.fill(1.0); self.box_ring.fill(1.0)` in `clear_runtime`):
   RED, `true-peak-limiter: 4 memset_pattern16 calls and no row` (2 in `clear_runtime::<f32x4>`, 2
   in `::<f32>`). Reverted: green (gate 1).
3. `vec![0.0_f32; black_box(levels.len())]` + `fill(1.0)` in `into_graph_artifact_with_banks`: RED,
   `builtins-compiler: 1 memset_pattern16 calls and no row`; the other five charged as before.
4. `python3 -B scripts/lib/aarch64-known-defects.py --self-test`: PASS. Refusal mutants, each
   reproduced: label crate charged RED; innermost incl. core RED; no product frame dropped RED;
   one-body check dropped RED (IndexError); unknown-form refusal dropped RED; judge reads products
   only RED; high_pc check dropped RED; non-product branch dropped GREEN (wording only, as recorded);
   no-row refusal dropped RED. Extra mutants: depth sort reversed RED; abstract origin ignored RED;
   hyphen mapping dropped RED; no-count check dropped RED; skip core/alloc/std by name RED;
   innermost namespace GREEN (NIT 2a); `+= unit` dropped GREEN (NIT 2b); unit-end check dropped
   GREEN (MINOR 2); base check, symbol-size check, names-no-crate check, closed interval end GREEN
   (consistency checks; on the real file the count is unchanged or refused).
5. Eight-lane mutation (`black_box(wide::f32x8::splat(black_box(2.0)))` in `lanes_below`): RED,
   `eight-lane code is back in the iOS library (#1112): 2 lines` (the `black_box<wide::f32x8_::f32x8>`
   names in `__debug_str`), from the iOS row, before the Android row runs.

Also: `scripts/check-ci-path-routing.py` passed; `scripts/test-ci-path-routing.py` passed (28 s);
`scripts/check-workspace-policy.sh` ok. No engine source changed; no realtime, C ABI or wasm gate
applies. No queue is touched (acked-batch question not applicable).

Evidence kept: `/tmp/claude-1002/v1472/logs/` (gate logs), `/tmp/claude-1002/v1472/xcheck/`
(cross-check scripts). The build directory and the export are deleted.
