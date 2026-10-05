PASS

# #1417 attempt 1 verdict: Refuse a C allocator name anywhere in an unmangled worklet symbol

Commit under review: `a93b6c17f` (branch `codex/d15-stream-j2`, parent `f2dccfdd9`). Reviewed by an
export of the commit (`git archive a93b6c17f`) to `/tmp/claude-1002/v1417/tree`, built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1417/target` (and `TMPDIR` under it for the build script's own
`mktemp` target). The worktree was not edited or built in.

The diff touches only the two authorized paths: `scripts/check-web-audioworklet-callgraph.py`
(`FORBIDDEN`, the docstring paragraph, `self_test()` (a1)) and the spec's attempt record. The
regex at `scripts/check-web-audioworklet-callgraph.py:119` is D1 character for character, the
lookahead binds only to the first alternative, and the other alternatives stay unanchored
substrings (`:120-121`). The docstring (`:19-32`) states D3: why mangled names are exempt, why an
unmangled name with a C allocator name fails, the fail-safe (rename, do not relax), and that
`scripts/test-web-audioworklet.sh` runs `--self-test` (verified: `scripts/test-web-audioworklet.sh:41`).
`self_test()` (a1) carries D2's cases exactly (`:631-690`). The superseded bare `__rust_realloc`
row is deleted and the four `<name>_count` rows are rewritten in the same commit. Every number in
the attempt record reproduces.

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **Spec-level gap, for root (not this attempt's defect): the four stems do not cover every C
   allocator entry point.** `scripts/check-web-audioworklet-callgraph.py:119` (as D1 fixes it)
   matches only `free|malloc|calloc|realloc`. C allocator entry points of the same class that
   contain none of them stay admitted: `posix_memalign`, `aligned_alloc`, `memalign`, `valloc`,
   `pvalloc`, jemalloc's `sdallocx`/`dallocx`/`rallocx` (and `_rjem_sdallocx`), mimalloc's
   `mi_zalloc`/`mi_zalloc_aligned`, `tlsf_memalign`, `sn_rust_alloc`. This is the issue's own
   scenario: a C-backed `#[global_allocator]` whose `__rust_*` shim LTO inlines. For an
   over-aligned layout (SIMD banks are over-aligned) constant folding can leave only the aligned
   branch, so the render closure reaches `posix_memalign` or `aligned_alloc` with no `malloc` beside
   it. Today's default allocator shows the same split: `__rdl_alloc` calls dlmalloc's `memalign`
   and `malloc` as separate callees. Fixing it costs nothing in false refusals today: no unmangled
   name in the real named twin contains `alloc`, `align` or `sbrk` (110 unmangled names: 108
   `miso_engine_web_v1_*` exports, `memcmp`, `__multi3`). A precise successor would widen the
   unmangled alternative, for example to `^(?!_R|_ZN).*(?:free|alloc|memalign|sbrk)`, with
   self-test rows `posix_memalign`, `aligned_alloc` and `je_sdallocx`. D1 fixes the regex text, so
   the attempt could not do this; root decides on a spec amendment or a successor issue. #1333's
   runtime `RENDER_ALLOCATIONS` counter, when it lands, covers the exercised paths whatever the
   names, which lowers but does not remove the static gap.

## NIT

1. **`_ZN` is not only Rust.** `scripts/check-web-audioworklet-callgraph.py:20-21` says "a legacy
   (`_ZN`) name is a Rust item". `_ZN` is also the Itanium C++ nested-name prefix, so a C++
   allocator's namespaced internals (`_ZN8snmalloc...`) are exempt as well. The risk is small: the
   shipped module has no `_ZN` name (every mangled name starts `_RN` or `_RI`), and a C++
   allocator's `extern "C"` entry point is unmangled and caught. A precise Rust-legacy test is
   `_ZN...17h<16 hex>E`. This is D1's wording, so it is for root, not the attempt.
2. **The docstring overstates the other alternatives.** `:22-24` says the Rust allocator's own
   symbols "are matched by the other alternatives". The twin's `__rdl_alloc`,
   `__rdl_alloc_zeroed` and `__rdl_realloc` (`_RNvCs..._7___rustc11___rdl_alloc` etc.) match no
   alternative. They are caught only through their dlmalloc callees (verified on the twin: every
   one calls dlmalloc's `memalign`/`malloc`, which `dlmalloc` matches). This is the spec's
   `__rdl_realloc` non-goal; the sentence could say "or through their dlmalloc callees".
3. **Partial overlap between new rows.** The end-anchor mutation (`...realloc)$`) turns red both
   the four `<name>_count` rows and `prefixed C allocator mi_malloc_aligned`. The `_count` rows
   still have a unique kill (below), so this is not dead code, only a shared one. Both are
   required by D2.

## Test value (one sentence per new or changed case)

- `(a1) out-of-line legacy accessor named free passes` (new, `:648`): red if the exemption
  covers only v0 names (`(?!_R)`), which refuses an ordinary `_ZN`-mangled method named `free`;
  verified as the only red case under that mutation.
- `(a1) prefixed C allocator {dlfree, __libc_free, mi_free, mi_malloc_aligned, je_malloc,
  tlsf_free, __libc_calloc, je_realloc}` (new, `:661-675`): red if the C alternative is anchored at
  the start of the name again (drop `.*`, or #1234's `^(...)$`), which admits `dlfree` or
  `je_malloc`; under drop-`.*` these eight are the only red cases.
- `(a1) unmangled {free,malloc,calloc,realloc}_count fails` (changed from pass to fail,
  `:679-683`): red if the rule admits an unmangled name that begins with a C allocator name,
  either by an end anchor or by widening the lookahead to admit an engine `free_*` function
  instead of renaming it (`(?!_R|_ZN|free_|malloc_|calloc_|realloc_)` or `(?!.*_count$)`); under
  the lookahead widening these four are the only red cases.
- `(a1) Rust allocator _RNvCs0_7___rustc14___rust_realloc` (changed, `:686`): red if
  `|__rust_realloc` is dropped from `FORBIDDEN`, which no other alternative covers for the mangled
  shim; it is the only red case under that mutation, and the replaced bare `__rust_realloc` row
  stays green under it (verified: dead under the new rule). The spelling is the real one: the twin
  carries `_RNvCs9wFQrvczXsK_7___rustc14___rust_realloc`.
- `(a1) out-of-line v0 accessor named {free,malloc,calloc,realloc} passes` (label changed only,
  `:640`): red if the lookahead loses `_R` (only these four red), loses `^` (search then exempts
  nothing past position 0), or the alternation is ungrouped (malloc, calloc and realloc red).

## Mutation runs (each alone on a copy of the script, `--self-test`)

| Mutation of `:119` (or of `:121`) | exit | red cases |
|---|---|---|
| `main`'s `^(free\|malloc\|calloc\|realloc)$` (gate 2) | 1 | 8 prefixed, 4 `_count` |
| drop lookahead `^.*(?:...)` | 1 | 4 v0 accessors, legacy accessor |
| drop `\|_ZN` | 1 | legacy accessor only |
| drop `_R` (`(?!_ZN)`) | 1 | 4 v0 accessors only |
| drop `.*` | 1 | 8 prefixed only |
| drop `\|__rust_realloc` (`:121`) | 1 | v0 `__rust_realloc` row only |
| old bare `__rust_realloc` row with `\|__rust_realloc` dropped | 0 | none (confirms the old row is dead) |
| ungroup `^(?!_R\|_ZN).*free\|malloc\|calloc\|realloc` | 1 | v0 malloc, calloc, realloc |
| end anchor `...realloc)$` | 1 | 4 `_count`, `mi_malloc_aligned` |
| word end `...realloc)(?![_a-z])` | 1 | 4 `_count`, `mi_malloc_aligned` |
| `.+` instead of `.*` | 1 | 4 bare, 4 `_count` |
| no `^` | 1 | 4 v0 accessors, legacy accessor |
| exempt `free_` etc. in the lookahead | 1 | 4 `_count` only |
| exempt `.*_count$` | 1 | 4 `_count` only |
| drop the whole C alternative | 1 | 4 bare, 8 prefixed, 4 `_count` |
| drop `\|dlmalloc` (`:120`) | 1 | (a), `_ZN8dlmalloc4free17h0E`, (b1), (b1b) |

Every mutation the spec names (gates 2 and 3) is red on exactly the cases the attempt record
lists.

## The real twin (hazard "no real member may newly fail")

Over all 2741 defined functions of the rebuilt named twin (`wasm-objdump -d` headers, all named,
2694 unique): `main`'s `FORBIDDEN` refuses 236 and the new one refuses 236, and the sets are equal
(no new-only and no old-only name). The 110 unmangled names are 108 `miso_engine_web_v1_*`
exports, `memcmp` and `__multi3`; none contains a C allocator name. The name section holds mangled
names (prefixes `_RN`, `_RI`), so the `_R` exemption reads what the tool prints. Real names that
the exemption must admit are present: `GraphRouteControlProducer4free` and
`RouteControlProducer4free` (v0), both admitted.

## Gates run (from the export)

1. `python3 -B scripts/check-web-audioworklet-callgraph.py --self-test`: exit 0.
2. Red on revert, `main`'s regex: exit 1 on the 8 prefixed and 4 `_count` cases (table above).
3. Mutations: all four spec mutations exit 1 on the named cases (table above).
4. `bash scripts/build-web-audioworklet.sh --named-twin N A`: exit 0; shipped module
   `e4d822a62e397b32283d3c6e0b06d3e3b4b5ffafbf74c2ddcb728c7301c21844`, named twin
   `6a8f3ee25d5803fc77b8f362936f8fec92ff3cfd575873a29cab8447af266d61`, both equal to the attempt
   record (only the script changed, so the module cannot move).
   `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration A N/...named.wasm`
   (the qualification.yml invocation): exit 0 ("web AudioWorklet static/object checks passed").
   `python3 -B scripts/check-browser-expected-resources.py --artifacts A`: exit 0.
5. `bash scripts/test-web-audioworklet.sh`: exit 0 (includes "web AudioWorklet call-graph
   analyser self-test passed"). `bash scripts/check-workspace-policy.sh`: exit 0.

Not run: browser legs and the full qualification set (batch-end verifier's job); no Rust, CI or
engine file changed, so cross-targets, realtime policy and CI routing do not apply.
