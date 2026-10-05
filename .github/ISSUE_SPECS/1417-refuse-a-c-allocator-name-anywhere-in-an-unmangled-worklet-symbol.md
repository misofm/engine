# Refuse a C allocator name anywhere in an unmangled worklet symbol

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the verdict of *Anchor the worklet callgraph checker's C allocator names* (#1234), MINOR 1 and NIT 3
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1234-attempt1.md`). No engine code
changes.

## Problem (verified on `main` at `0a1176b3b`)

**The rule.** `scripts/check-web-audioworklet-callgraph.py` fails a guarded export when any member
of its direct-call closure has a name that `FORBIDDEN` matches (`:113-117`):

```
^(free|malloc|calloc|realloc)$|dealloc|dlmalloc|drop_glue|drop_in_place|drop_slow|unlink_chunk
|insert_large_chunk|memory_grow|__rust_alloc|__rust_realloc
```

#1234 anchored the four C names to the whole symbol, so that an out-of-line Rust method named
`free` (a v0 name ending in `4free`) is not read as the allocator.

**The gap.** The anchor also admits every *prefixed* C or third-party allocator spelling that the
earlier substring form refused: `dlfree`, `__libc_free`, `__libc_malloc`, `mi_free`,
`mi_malloc_aligned`, `je_malloc`, `je_free`, `tlsf_free`, `emscripten_builtin_free`. None of
`dealloc`, `dlmalloc` or `__rust_alloc` matches them. The docstring's argument (`:19-26`) shows
that a bare `free` must be a C allocator. It does not show that a C allocator must be spelled bare,
and that is false. A future `#[global_allocator]` backed by C, with its `__rust_*` shim inlined by
LTO, would reach such a name with nothing in the list matching it.

Today's shipped module has none (#1234 verdict): every defined function is a Rust v0 name
(`_R...`), a `miso_engine_web_v1_*` export, `memcmp` or `__multi3`, and the module has no imports
(`scripts/check-web-audioworklet.sh` refuses any). So this is a hole in the gate, not a missed
allocation today.

**The self-test encodes the gap.** `self_test()` (`:601`) has four cases that require unmangled
names which merely contain a C allocator name to pass (`free_count`, `malloc_count`,
`calloc_count`, `realloc_count`; `:641-647`). They were added to kill a mutant that drops the
trailing `$`. Under this issue's rule those names fail, as a C allocator spelled `free_list` or
`malloc_usable` should.

**A wrong script name in #1234's non-goal.** #1234's spec says `check-web-audioworklet.sh` runs the
analyser's self-test. The self-test runs from `scripts/test-web-audioworklet.sh:41`, which
`.github/workflows/qualification.yml:349` runs. Coverage is intact. #1234 is closed, so its spec is
not edited; this body records the correct wiring.

## Decisions

- **D1. The C names match anywhere in an unmangled name, and never in a mangled Rust name.**
  `FORBIDDEN`'s C alternative becomes `^(?!_R|_ZN).*(?:free|malloc|calloc|realloc)`. A Rust v0
  (`_R`) or legacy (`_ZN`) name is a Rust item; the Rust allocator's own symbols are matched by the
  other alternatives (`dlmalloc`, `dealloc`, `__rust_alloc`, `__rust_realloc`), which stay as they
  are. Any other name is C or a `#[no_mangle]` symbol, and a C allocator name anywhere in it fails.
  This fails safe: an unmangled engine function whose name contains `free` fails too, and the fix
  for that is to rename it, not to relax the rule.
- **D2. Self-test cases.** In `self_test()`:
  - **Pass:** the four out-of-line v0 accessors (`...4free`, `...6malloc`, `...6calloc`,
    `...7realloc`) stay; add one legacy accessor,
    `_ZN5graph25GraphRouteControlProducer4free17h0123456789abcdefE`.
  - **Fail:** the four bare names stay. Add the prefixed names `dlfree`, `__libc_free`, `mi_free`,
    `mi_malloc_aligned`, `je_malloc`, `tlsf_free`, `__libc_calloc` and `je_realloc`.
  - **Changed:** the four `<name>_count` cases now **fail**, with the label saying why: an
    unmangled name that contains a C allocator name is refused.
  - **Changed:** the bare `__rust_realloc` row now also matches the C alternative, so it no longer
    shows that the `__rust_realloc` alternative is needed. Replace it with the v0 shim spelling
    `_RNvCs0_7___rustc14___rust_realloc`.
- **D3. Docstring.** Rewrite the paragraph at `:19-26` to D1: why mangled names are exempt, why any
  unmangled name with a C allocator name fails, and that `scripts/test-web-audioworklet.sh` runs
  the self-test.

## Authorized paths

- `scripts/check-web-audioworklet-callgraph.py` (`FORBIDDEN`, the docstring paragraph,
  `self_test()`)
- This spec

## Non-goals

- Any other callgraph rule, the trap allow-list or the kernel-shape gate.
- CI wiring: `scripts/test-web-audioworklet.sh:41` already runs the self-test.
- `__rdl_realloc` (#1234 verdict): it is caught through its callees.
- The duplicate `_ZN8dlmalloc4free17h0E` row (#1234 NIT 2).

## Hazards

- **No real member may newly fail.** Before changing `FORBIDDEN`, list every defined function of
  the rebuilt named twin. Every name the new rule refuses must be one the old rule refused too, or
  the issue stops and reports the name (a real allocator reach keeps the gate red; a legitimate
  name is a finding for root).
- **Python `re` semantics.** The lookahead applies only to the first alternative; the other
  alternatives stay unanchored substrings. Keep the alternation grouped so it does.

## Objective gates

1. `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test` exits 0 with D2's cases.
2. **Red on revert (PR evidence).** With `main`'s `FORBIDDEN`, the self-test exits 1 on each new
   prefixed-name case and each changed `<name>_count` case.
3. **Mutations (PR evidence), each applied alone to a copy, each exits 1 on the named cases:**
   - drop the lookahead (`^.*(?:free|...)`): the out-of-line accessor cases;
   - drop `|_ZN` from the lookahead: the legacy accessor case;
   - drop `.*` (`^(?!_R|_ZN)(?:free|...)`): the prefixed-name cases whose prefix is not a C
     allocator name (`dlfree`, `__libc_free`, `mi_free`, ...);
   - drop `|__rust_realloc`: the v0 `__rust_realloc` row.
4. **The shipped module.** `bash scripts/build-web-audioworklet.sh --named-twin <N> <A>`, then
   `bash scripts/check-web-audioworklet.sh <A> <N>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   exit 0. Over every defined function of the named twin, the set refused by the new `FORBIDDEN`
   equals the set refused by `main`'s; record both counts.
5. `bash scripts/test-web-audioworklet.sh` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The prefixed-name cases are red if the C alternative is anchored at the start of the name again,
  which admits `dlfree` or `je_malloc` and which no other case catches.
- The legacy accessor case is red if the exemption covers only v0 names, which refuses an ordinary
  `_ZN`-mangled method named `free`.
- The v0 `__rust_realloc` row is red if `__rust_realloc` is dropped from the list: no other
  alternative matches that mangled shim.

## Evidence

- Gate 2's and gate 3's runs: the failing case labels of each.
- Gate 4's function counts and the shipped module's digest (it must not change: only the script
  changes).

## Amendment 1 (root, 2026-10-05)

Attempt 1 passed (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j2/1417-attempt1.md`).
Its verifier found that D1 still admits C allocator entry points that are not spelled with one of
the four names: `posix_memalign`, `aligned_alloc`, `memalign`, jemalloc's `sdallocx` and
`rallocx`, mimalloc's `mi_zalloc` and snmalloc's `sn_rust_alloc`. An LTO-inlined C-backed
allocator on an over-aligned path can call `posix_memalign` with no `malloc` beside it. Root ruled
that this is the same issue's purpose and folds it into the batch follow-ups:

- **A1. Widen D1.** The C alternative refuses any unmangled name that contains `free`, `alloc`,
  `memalign` or `sbrk`, for example `^(?!_R|_ZN).*(?:free|alloc|memalign|sbrk)` (still in its own
  group, so the lookahead applies to it alone). The fail-safe rule of D1 stands: an unmangled
  engine name that matches is renamed, not exempted.
- **A2. Self-test cases.** Add fail cases `posix_memalign`, `aligned_alloc`, `memalign`,
  `sdallocx`, `rallocx`, `mi_zalloc` and `sn_rust_alloc`. Each passes under attempt 1's rule (red
  on revert) and is refused under A1. Re-check every existing case under A1, and record any whose
  catch A1 makes redundant, with the action taken.
- **A3. The real tree.** The named twin's unmangled names still pass under A1 (the verifier found
  none containing `alloc`, `align` or `sbrk`), and the refused set of defined functions is
  recorded against attempt 1's.
- **A4. The `_ZN` docstring.** A `_ZN` prefix is Itanium C++ mangling, used by Rust's legacy
  scheme and by C++ namespaced names alike. Say what the rule assumes: the worklet links no C++,
  so a `_ZN` name in it is a Rust legacy item, and a C++ dependency would need this rule
  revisited.

The batch verifier re-checks A1 to A4 (gates 1 to 5 and the mutations, with A2's cases). GitHub
#1417 is synced at the follow-up commit.

## Dependencies

- None. *Anchor the worklet callgraph checker's C allocator names* (#1234) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The self-test runs the analyser on synthetic
  disassembly, which is not source grepping.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt record

### Attempt 1 (implementer, stream J batch 2, branch `codex/d15-stream-j2`)

Changed only `scripts/check-web-audioworklet-callgraph.py`: `FORBIDDEN`'s C alternative is now
`^(?!_R|_ZN).*(?:free|malloc|calloc|realloc)` (D1, grouped so the lookahead binds only to it); the
docstring paragraph is rewritten to D1 and names `scripts/test-web-audioworklet.sh` as the
self-test's runner (D3); `self_test()` (a1) carries D2's cases exactly.

- **Gate 1.** `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test`: exit 0.
- **Gate 2 (red on revert).** With `main`'s `^(free|malloc|calloc|realloc)$`: exit 1, failing
  `prefixed C allocator` dlfree, __libc_free, mi_free, mi_malloc_aligned, je_malloc, tlsf_free,
  __libc_calloc, je_realloc, and `unmangled {free,malloc,calloc,realloc}_count fails`.
- **Gate 3 (mutations, each alone on a copy, each exit 1).**
  - drop the lookahead (`^.*(?:...)`): `out-of-line v0 accessor named` free, malloc, calloc,
    realloc and `out-of-line legacy accessor named free passes`.
  - drop `|_ZN`: `out-of-line legacy accessor named free passes` only.
  - drop `.*`: all eight `prefixed C allocator` cases.
  - drop `|__rust_realloc`: `Rust allocator _RNvCs0_7___rustc14___rust_realloc` only.
  - (extra, the Python `re` hazard) ungroup the alternation (`^(?!_R|_ZN).*free|malloc|...`):
    `out-of-line v0 accessor named` malloc, calloc, realloc.
- **Gate 4.** `build-web-audioworklet.sh --named-twin` then `check-web-audioworklet.sh`: exit 0.
  Over all 2741 defined functions of the named twin (`wasm-objdump -d` headers), `main`'s
  `FORBIDDEN` refuses 236 and the new one refuses 236; the sets are equal (no new-only, no
  old-only name). 110 names are unmangled (108 `miso_engine_web_v1_*` exports, `memcmp` and
  `__multi3`); none contains a C allocator name. Shipped module
  `e4d822a62e397b32283d3c6e0b06d3e3b4b5ffafbf74c2ddcb728c7301c21844` (named twin
  `6a8f3ee25d5803fc77b8f362936f8fec92ff3cfd575873a29cab8447af266d61`); only the script changed,
  so the module is unchanged.
- **Gate 5.** `bash scripts/test-web-audioworklet.sh`: exit 0. `bash scripts/check-workspace-policy.sh`:
  exit 0.
