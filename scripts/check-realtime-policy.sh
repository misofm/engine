#!/usr/bin/env bash
# Check the explicitly marked issue-003 realtime call graph and approved unsafe ownership boundary.
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GATE_FAILURE_PREFIX='realtime policy failure'
source "$script_directory/lib/gate.sh"
cd "$workspace_root"

fail() {
    printf 'realtime policy failure: %s\n' "$1" >&2
    exit 1
}

realtime_root="crates/engine/src/realtime"
[[ -d "$realtime_root" ]] || fail "missing realtime module"

# Issue #146 adds `crates/lane/src/fpenv.rs`, the canonical floating-point environment
# pinned at every native render entry. On `x86` it carries no `unsafe` of its own -- it reuses the
# already-listed `_mm_getcsr`/`_mm_setcsr` helpers of `softfma.rs` -- and its one unsafe site is the
# AArch64 `mrs`/`msr FPCR` pair, for which the standard library exposes no `core::arch` intrinsic
# (Arm Architecture Reference Manual for A-profile, `FPCR`, Floating-point Control Register).
# Issue #240's exact `boot_transient_budget.rs` fixture owns a `GlobalAlloc` forwarding wrapper so
# it can measure the parser/model-builder high-water mark; it changes no allocation operation and
# is the only non-FFI web-host file admitted here.
# `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", carries the full justification.
unsafe_raw="$(gate_scan_collect 'unsafe source scan' 'unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{' '*.rs' crates hosts tools)" || exit $?
unsafe_matches="$(gate_filter_exclude 'unsafe source exclusions' '^crates/engine/src/realtime/spsc.rs:|^crates/engine/src/realtime/disjoint.rs:|^crates/lane/src/softfma.rs:|^crates/lane/src/fpenv.rs:|^crates/builtins-compiler/tests/allocation_tracker.rs:|^crates/session/tests/allocation_budget.rs:|^crates/soft-clip/tests/allocation.rs:|^crates/transient-shaper/tests/allocation.rs:|^crates/capi/src/ffi.rs:|^crates/capi/tests/resource_lifecycle.rs:|^crates/capi/tests/plan_swap_race.rs:|^crates/true-peak-limiter/tests/allocation.rs:|^crates/multiband-compressor/tests/no_alloc_render.rs:|^hosts/host-web/src/ffi.rs:|^hosts/host-web/tests/boot_transient_budget.rs:|^tools/bench-support/src/alloc.rs:|^tools/audit/src/capi.rs:|^tools/wasm-gate-guest/src/lib.rs:' "$unsafe_raw")" || exit $?
[[ -z "$unsafe_matches" ]] || {
    printf '%s\n' "$unsafe_matches" >&2
    fail "unsafe code exists outside the issue-approved ownership/audit files"
}

scratch_file="$(mktemp)"
trap 'rm -f -- "$scratch_file"' EXIT

# Issue #371 (RT-16/IO-14): the scan is root-agnostic. Every file that carries a
# REALTIME_POLICY_BEGIN marker is scanned, wherever it sits under `crates hosts tools`, instead
# of only the files under crates/engine/src/realtime. Markers in a file
# outside that directory are no longer decorative: the unmatched-marker check and the
# forbidden-surface regex both reach them. The directory-existence assertion above stays on
# its own: the issue-003 realtime module is the one directory that must exist whether or not
# anything else is marked.
marker_count=0
marked_file_count=0
if marked_files_raw="$(rg -l 'REALTIME_POLICY_BEGIN' crates hosts tools --glob '*.rs' 2>&1)"; then marked_files_rc=0; else marked_files_rc=$?; fi
[[ $marked_files_rc == 1 ]] && marked_files_raw=''
[[ $marked_files_rc -le 1 ]] || { printf '%s\n' "$marked_files_raw" >&2; fail "realtime marker discovery failed (rg exit $marked_files_rc)"; }
marked_files="$(gate_sort_lines 'realtime marker discovery' "$marked_files_raw")" || exit $?
if [[ -n "$marked_files" ]]; then
while IFS= read -r source; do
    if begins="$(rg -c 'REALTIME_POLICY_BEGIN' "$source" 2>&1)"; then :; else rc=$?; [[ $rc == 1 ]] && begins=0 || { printf '%s\n' "$begins" >&2; fail "BEGIN marker count failed for $source (rg exit $rc)"; }; fi
    if ends="$(rg -c 'REALTIME_POLICY_END' "$source" 2>&1)"; then :; else rc=$?; [[ $rc == 1 ]] && ends=0 || { printf '%s\n' "$ends" >&2; fail "END marker count failed for $source (rg exit $rc)"; }; fi
    [[ "$begins" == "$ends" ]] || fail "$source has unmatched realtime policy markers"
    marker_count=$((marker_count + begins))
    marked_file_count=$((marked_file_count + 1))
    if body="$(awk '
        /REALTIME_POLICY_BEGIN/ { inside = 1; next }
        /REALTIME_POLICY_END/ { inside = 0; next }
        inside { print FILENAME ":" FNR ":" $0 }
    ' "$source" 2>&1)"; then :; else rc=$?; printf '%s\n' "$body" >&2; fail "realtime body extraction failed for $source (awk status $rc)"; fi
    [[ -z "$body" ]] || printf '%s\n' "$body" >>"$scratch_file" || fail "realtime body persistence failed for $source"
done <<<"$marked_files"
fi

# The floors are the merged tree's own counts after #1269 phase 1 (the swap carry and the
# #1071/#1278 effect-restore regions) and #1053 (#1253's builtins-compiler fader and matrix
# drains) and #1418 (the builtin input bank's per-lane `drain_controls`): twenty-five files and
# ninety regions (they were twelve and forty-one after #664's complete LocalRing removal).
# Deleting a marker to silence the gate fails here -- the file leaves the discovered set or a
# region leaves the marked set -- instead of passing with less coverage. Raising a floor is part
# of the change that adds a marker.
[[ "$marked_file_count" -ge 25 ]] || fail "expected at least twenty-five marked realtime files"
[[ "$marker_count" -ge 90 ]] || fail "expected at least ninety marked realtime regions"

gate_scan_forbidden 'marked realtime forbidden-body predicate' \
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp|\.expect\(|\.unwrap\(|panic!\(|unreachable!\(|todo!\(|unimplemented!\(' '' "$scratch_file" || exit $?

# #1253, #1302, #1418: every render-thread drain pops at most the records its queue held when its
# `available_at_entry` count was read, and each drain reads a given queue at most once per block:
# every loop around a drain must drain a different queue on each pass. Two drains of one queue in
# sequence each pop at most their own count; that is outside this rule (see below). A loop that
# pops until the queue is empty keeps popping whatever a concurrent producer publishes while it runs (the C ABI
# prepares with `Concurrent` delivery) -- data-dependent, unbounded render work. The pass below
# reads each marked region as tokens, not lines, so layout plays no part: comments and string,
# byte-string, C-string, raw-string (any number of `#`) and character literals are blanked, a
# brace stack gives every block its header -- the tokens of its statement since the previous `;`
# outside brackets or the start of the enclosing block, across lines, each closed inner block one
# `{}` token -- and a block whose header holds `loop`, `while`, `for .. in` or `from_fn` is a
# loop. It refuses whatever it cannot prove bounded. What it proves, per marked region:
#   - a `try_pop` (the word: `Consumer::try_pop` counts, and the `fn try_pop` definition sits in
#     no loop) in the body of a loop has an innermost loop whose whole header, after optional
#     attributes and label, is `for P in 0..<count>`, `for P in 0..<path>.available_at_entry()`,
#     or `while <count> != 0` (or `> 0`) whose body opens with `<count> -= 1;` and never names
#     `<count>` again. `<count>` is the nearest in-scope
#     `let [mut] <count>[: T] = <path>.available_at_entry()[.min(<cap>)];` (or
#     `= <path>.map_or(0, <Type>::available_at_entry)[.min(<cap>)];`; `<path>` may index, as in
#     `self.controls[lane]`), or one plain alias `let [mut] <count> = <entry count>;`. From that
#     binding to the loop every other mention of `<count>` is a read: the left operand of a
#     comparison, outside any macro call but `debug_assert!` (no rebinding, assignment, borrow,
#     method call or macro argument; a raw-identifier macro such as `r#while!` is a macro, not
#     the keyword). The binding carries no `cfg`-family attribute (`#[cfg(..)]`,
#     `#[cfg_attr(..)]`): a binding that may not be compiled leaves an earlier one as the count.
#     No `static`, `const` or `use` in the region may name `<count>`, and a `let` made at the
#     region's outermost level leaves scope at the first `}` that closes a block opened before
#     the region;
#   - every other loop around that pop is finite: a bounded loop as above, `for P in 0..<bound>`
#     (`<bound>` an integer literal or a path: `self.lanes`, `LANES`, `self.controls.len()`),
#     `&<fields>`, `&mut <fields>`, `<fields>.iter()`/`.iter_mut()`, or a slice or array
#     parameter of the enclosing `fn` that nothing names before the loop and no `static`,
#     `const` or `use` in the region names (each optionally `.enumerate()`d or `.zip(..)`ped),
#     or the closure of `core::array::from_fn`;
#   - and every such loop drains a different queue on each pass (#1418): finite is not small, so
#     a loop around a drain that re-reads its count (`0..usize::MAX`, `0..control.capacity()`,
#     `&mut` of an open range, a `[(); usize::MAX]` parameter) would pop until the queue is
#     empty. Walking outward from the count's receiver (`<path>` in
#     `<path>.available_at_entry()`), each loop passes only if, in the loop's body before the
#     receiver is read, the receiver is
#       - a root bound by the loop's pattern (`for control in ..`, `for (lane, control) in ..`),
#         then only fields and plain indexes;
#       - a base indexed by exactly the identifier that takes a different value on each pass:
#         the lone pattern of a `0..<bound>` loop, the index slot of a trailing `.enumerate()`
#         (not over `&mut` of an iterator) or the parameter of an `array::from_fn` closure
#         (`controls[lane]`, `self.controls[lane]`; not `controls[lane % 2]`, not `controls[0]`,
#         not an index read from a table, `for &lane in order`, which can repeat), that
#         identifier rebound by no `let`, header, macro or item in the loop and not `mut`; or
#       - a root whose nearest binding in scope is `let <root> = ..`, `let Some(<root>) = ..
#         else {..}` or `if let Some(<root>) = ..` (`Ok` too) from `&`/`&mut` of a form above or
#         from such a root, alone or followed by `.as_mut()`/`.as_ref()`
#         (`let Some(control) = control.as_mut() else { continue; };`), at most 64 links deep.
#     And the loop's body leaves the queue in place: every mention there of the root of a
#     spelling the walk read (`control`, `controls`, `self`) is in a binding or header the walk
#     read (less the `else` block of a `let .. else`), or reads that spelling, then fields and
#     plain indexes, as the receiver of `available_at_entry`, `try_pop` or `map_or(0, ..)`. So a
#     base changed between passes (`controls.swap(0, 1)`, `controls = ..`, a `let controls`) or
#     a queue moved out of its slot (`mem::swap(control, spare)`) is refused, as is any other use
#     (`self.apply(..)` beside `self.controls[lane]`; a known over-refusal).
#     The next loop out is walked the same way from what the loop's pass selects: the collection
#     it iterates when the receiver is its pattern (`for bank in &mut self.banks { for control
#     in &mut bank.controls { .. } }`), or the base it indexes (`for bank in &mut self.banks {
#     for lane in 0..N { bank.controls[lane] .. } }`); a range or `array::from_fn` pattern
#     names no collection and refuses every loop outside it. A pattern that binds nothing (`_`,
#     `()`) never selects. A binding of the name the walk cannot read is refused: a struct or
#     tuple pattern, a closure, `match` arm or other block header naming it (an `if` condition
#     without `let`, `|` or `=>` binds nothing), a macro call naming it (but `debug_assert!`),
#     and a `static`, `const` or `use` that can name it. Collections are taken at their word:
#     iterating `&mut <fields>` yields distinct elements and an SPSC consumer is not `Clone`, so
#     each element is a different queue, as is each index of a base the loop leaves in place;
#     the gate does not prove that a collection holds each queue once (shared handles behind a
#     method call are refused only because a method call ends the receiver forms). A drain that
#     must read one queue twice in a block is one counted drain, not an outer loop;
#   - a `try_pop` in the header of a loop (its condition, iterator or pattern, a block nested
#     there included) is refused: it runs once per iteration;
#   - `from_fn`, `repeat_with` and `successors` are refused outright, path-qualified or bare
#     (`core::array::from_fn` and `std::array::from_fn` excepted): they run a pop in a loop without
#     a loop keyword. A pop directly in an `array::from_fn` closure is refused (a constant count);
#   - a region still open where its file ends (markers out of order) is checked as it stands (a
#     loop still open there selects no queue), and
#     a region that ends inside a literal or a block comment is refused.
# A header ends early only where Rust must start a new statement or item. After a closed block
# that is: an identifier other than `as`, `else` or `in`, a label or an attribute; or a `{`, when
# the closed block was the body of a statement-leading `if`, `match`, `while`, `for`, `loop`,
# `unsafe`, label or bare block whose head ends where an expression can end (not on an operator,
# a keyword or a macro `!`, before which a `{` opens a block operand). Elsewhere it joins, which
# can only make a header look like a loop or miss a bounded form: both refuse. It is not a Rust
# parser. It takes these at their word: `available_at_entry`, `.min(..)`, `.len()`, `.iter()`,
# `.iter_mut()`, `.enumerate()`, `.zip(..)`, `.as_mut()`, `.as_ref()` and `debug_assert!` (a
# local macro or method of the same name could differ), and `&<fields>` and `&mut <fields>` as
# collections (`&mut` of an iterator held in a field or local, such as
# `let mut rounds = 0_u32..; for _ in &mut rounds`, can be infinite; around a drain the
# queue-selection walk accepts it only when its pattern binds the queue itself, whose `&mut`
# elements are distinct and run out, never by an index, and refuses `for lane in &mut lanes`).
# These stay outside it: recursion; a pop in a helper function, through a function value, or in
# a closure that a loop calls or that is handed to a repeating adapter (`map_while` over
# `iter::repeat`, `for_each`, a renaming `use` of an iterator constructor); a loop around the
# drain outside the marked region (the rule is per region); a macro that expands to a loop; and
# two drains of one queue in sequence, outside any loop that selects it, or in one pass of such a
# loop (`let a = c.available_at_entry(); for _ in 0..a {..} let b = c.available_at_entry(); ..`):
# each is bounded by its own count, so the total stays finite, but the second count is not read
# at block entry. Two spellings can name one queue, so the gate cannot refuse this soundly.
# It refuses these bounded drains (known over-refusals; restructure the drain to pass): a count
# read other than as the left operand of a comparison before its loop (`0 == available`,
# `apply(available)`, `available as u32`), a count capped other than by `.min(..)`
# (`cmp::min`, `.clamp(..)`), an outer `for` over a non-zero start, an inclusive range or any
# other adapter (`1..n`, `0..=n`, `.rev()`, `.take(n)`), a bare local or field iterated by value
# (`for control in self.controls`), and a pop once per pass of a collection loop with no count
# (D2 refuses it too).
if [[ -n "$marked_files" ]]; then
    mapfile -t marked_list <<<"$marked_files"
    if drain_hits="$(LC_ALL=C awk -v q="'" '
        function tw(x) { return "(^| )" x "( |$)" }
        function trim(s) { sub(/^ +/, "", s); sub(/ +$/, "", s); return s }
        function wc(ch) { return ch ~ /[A-Za-z0-9_]/ }
        # Comments, string and character literals become spaces, line by line, keeping every
        # other character in place; a block comment or a string may span lines.
        function clean(k,  s, out, i, len, ch, nx, j, h) {
            s = c[k]; len = length(s); out = ""
            if (!st && s !~ /["*]/ && index(s, q) == 0) {
                i = index(s, "//"); if (!i) return s
                out = substr(s, 1, i - 1); while (i++ <= len) out = out " "; return out
            }
            for (i = 1; i <= len; i++) {
                ch = substr(s, i, 1)
                if (st == 1) {
                    if (ch == "/" && substr(s, i + 1, 1) == "*") { bd++; out = out "  "; i++; continue }
                    if (ch == "*" && substr(s, i + 1, 1) == "/") { if (--bd == 0) st = 0; out = out "  "; i++; continue }
                    out = out " "; continue
                }
                if (st == 2) {
                    if (ch == "\\") { out = out "  "; i++; continue }
                    if (ch == "\"") { st = 0; out = out ch; continue }
                    out = out " "; continue
                }
                if (st == 3) {
                    if (ch == "\"" && substr(s, i + 1, rh) == rhs) {
                        st = 0; out = out ch substr(s, i + 1, rh); i += rh; continue
                    }
                    out = out " "; continue
                }
                nx = substr(s, i + 1, 1)
                if (ch == "/" && nx == "/") { while (i <= len) { out = out " "; i++ }; break }
                if (ch == "/" && nx == "*") { st = 1; bd = 1; out = out "  "; i++; continue }
                if (ch == "\"") { st = 2; out = out ch; continue }
                # A raw string (`r`, `br` or `cr`) closes at a `"` followed by as many `#` as
                # opened it, however many that is.
                if ((ch == "r" || (ch == "b" || ch == "c") && nx == "r") && (i == 1 || !wc(substr(s, i - 1, 1)))) {
                    j = i + (ch != "r") + 1; h = 0; rhs = ""
                    while (substr(s, j, 1) == "#") { j++; h++; rhs = rhs "#" }
                    if (substr(s, j, 1) == "\"") { st = 3; rh = h; out = out substr(s, i, j - i + 1); i = j; continue }
                }
                if (ch == q) {
                    if (nx == "\\") j = index(substr(s, i + 3), q) + i + 2
                    else if (substr(s, i + 2, 1) == q) j = i + 2
                    else if (nx ~ /[A-Za-z_]/ || nx == "") j = 0
                    else j = index(substr(s, i + 2, 5), q) + i + 1
                    if (j > i + 1) { out = out q; while (++i < j) out = out " "; out = out q; continue }
                }
                out = out ch
            }
            return out
        }
        # One token of the open statement at the current depth `D`, top level only (a closed
        # brace group is one `{}` token), so `segtop[D]` reads as a header when a `{` follows.
        function tok(t) { segtop[D] = segtop[D] " " t }
        function reset(at) { segtop[D] = ""; segpops[D] = ""; segstart[D] = at }
        # Whether `x` is an entry count for the header (or alias statement) at `hs`..`he`: its
        # nearest in-scope `let` before `before` takes it from `available_at_entry` in a
        # recognised form (or, at depth 0, aliases one such count in one plain step); `hs`..`he`
        # names `x` exactly once; between the binding and `hs` every mention of `x` is a read; and
        # no `static`, `const` or `use` in the region can name `x` (a block item shadows a `let`
        # of an enclosing block, even when it is declared after the loop).
        function counted(x, before, hs, he, depth,   k, t, b, ident, num, recv, cap, a) {
            for (k = ns; k >= 1; k--) {
                if (!onstack[st_frame[k]] || st_end[k] >= before) continue
                t = st_top[k]; sub(/^(# (! )?\[[^]]*\] )*/, "", t)
                if (t !~ ("^let( [^=]*)? " x "( |$)")) continue
                b = k; break
            }
            if (!b || mentions(x, hs, he) != 1 || !reads(x, st_end[b] + 1, hs - 1) || item(x)) return 0
            # A binding under a `cfg`-family attribute (`cfg`, `cfg_attr`, or any path to one) may
            # not be compiled, leaving an earlier binding of the same name as the count.
            a = substr(st_top[b], 1, length(st_top[b]) - length(t))
            if (a ~ /(^| )cfg[A-Za-z0-9_]*( |$)/) return 0
            ident = "[A-Za-z_][A-Za-z0-9_]*"; num = "[0-9][0-9_]*(u8|u16|u32|u64|usize)?"
            recv = ident "( \\. " ident "( \\( \\))?| [[] (" num "|" ident "( \\. " ident ")*) []])*"
            cap = "( \\. min \\( (" num "|" ident "( (\\.|: :) " ident ")*) \\))?"
            if (t ~ ("^let (mut )?" x "( : " ident ")? = " recv " \\. (available_at_entry \\( \\)|map_or \\( 0 , " ident " : : available_at_entry \\))" cap "$")) {
                # The receiver of the count and where it is read, for the queue-selection walk.
                crecv = t; sub(/^[^=]*= /, "", crecv); sub(/ \. (available_at_entry|map_or) \(.*$/, "", crecv)
                sub(/ \. as_(ref|mut) \( \)$/, "", crecv); cpos = st_start[b]
                return 1
            }
            if (depth == 0 && t ~ ("^let (mut )?" x "( : " ident ")? = " ident "$")) {
                sub(/^.* = /, "", t); return counted(t, st_start[b], st_start[b], st_end[b], 1)
            }
            return 0
        }
        function mentions(x, from, to,   s, m) {
            s = substr(F, from, to - from + 1); m = 0
            while (match(s, "(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) { m++; s = substr(s, RSTART + RLENGTH) }
            return m
        }
        # Whether every mention of `x` in `from`..`to` is a read: the left operand of `==`, `!=`,
        # `<`, `<=`, `>` or `>=` (never `=`, a compound assignment, a shift, a borrow-taking
        # method or a pattern, none of which a comparison operator can follow), and in no macro
        # call but `debug_assert!`, since a macro may expand its tokens into a new binding.
        function reads(x, from, to,   p, e, j, c2, k, d, ch, w) {
            p = from
            while (p <= to && match(substr(F, p, to - p + 1), "(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) {
                p += RSTART - 1; if (substr(F, p, length(x)) != x) p++
                e = p + length(x); j = e; while (substr(F, j, 1) == " ") j++
                c2 = substr(F, j, 2)
                if (c2 != "==" && c2 != "!=" && c2 != "<=" && c2 != ">=" && (c2 !~ /^[<>]/ || substr(c2, 2, 1) ~ /[<>=]/)) return 0
                d = 0
                for (k = p - 1; k >= from; k--) {
                    ch = substr(F, k, 1)
                    if (ch == ")" || ch == "]" || ch == "}") { d++; continue }
                    if (ch != "(" && ch != "[" && ch != "{") continue
                    if (d) { d--; continue }
                    j = k - 1; while (substr(F, j, 1) == " ") j--
                    if (substr(F, j, 1) != "!") continue
                    j--; while (substr(F, j, 1) == " ") j--
                    w = ""; while (j >= 1 && wc(substr(F, j, 1))) w = substr(F, j--, 1) w
                    # `r#while!` is a macro whose name is a raw identifier, not the keyword.
                    if (w != "" && substr(F, j, 1) == "#" && substr(F, j - 1, 1) == "r") return 0
                    if (w != "" && w !~ /^(if|while|match|return|break|in|else|let|mut)$/ && w != "debug_assert") return 0
                }
                p = e
            }
            return 1
        }
        # Whether a `static` or `const` item, or a `use` declaration, anywhere in the region can
        # name `x`.
        function item(x) {
            return F ~ ("(^|[^A-Za-z0-9_])(static|const) +(mut +)?" x "([^A-Za-z0-9_]|$)") || F ~ ("(^|[^A-Za-z0-9_])use [^;]*([^A-Za-z0-9_]" x "([^A-Za-z0-9_]|$)|[*])")
        }
        # A block whose header holds `loop`, `while`, `for .. in` or `from_fn` is a loop. A loop
        # bounds a pop only if its whole header, after optional attributes and label, is one of
        # these: `for P in 0..<count>`, `for P in 0..<path>.available_at_entry()`, or
        # `while <count> != 0` (or `> 0`) whose body opens with `<count> -= 1;` and never names
        # it again. Around such a drain, an enclosing loop may also be `for P in 0..<bound>`
        # (`<bound>` an integer literal or a path such as `self.lanes`, `LANES` or
        # `self.controls.len()`), `&<fields>`, `&mut <fields>`, `<fields>.iter()`/`.iter_mut()`,
        # or a slice or array parameter of the enclosing `fn` that nothing names before the
        # loop, each optionally `.enumerate()`d or `.zip(..)`ped (a zip ends with its receiver),
        # or the closure of `array::from_fn`, which runs a constant number of times but never
        # bounds a pop in its own body. Finite is not enough: such a loop must also select the
        # queue its drain reads on each pass (`selwalk`, below). `lpat` keeps the pattern of the loop
        # (or closure parameters) and `itrecv` the collection its iterable names, if any.
        function classify(f, H,   x, pat, ident, num, fields, path, adapt) {
            sub(/^(# (! )?\[[^]]*\] )*/, "", H)
            isl[f] = H ~ tw("loop") || H ~ tw("while") || H ~ /(^| )for( | .* )in( |$)/ || H ~ tw("from_fn")
            inner[f] = 0; outer[f] = 0; wid[f] = ""; lpat[f] = ""; itrecv[f] = ""; lidx[f] = ""
            if (!isl[f]) return
            ident = "[A-Za-z_][A-Za-z0-9_]*"; num = "[0-9][0-9_]*(u8|u16|u32|u64|usize)?"
            pat = "( [A-Za-z0-9_(),&]+)+"
            fields = ident "( \\. " ident ")*"
            path = ident "( (\\.|: :) " ident "( \\( \\))?| [[] (" num "|" fields ") []])*"
            adapt = "( \\. enumerate \\( \\)| \\. zip \\( (& (mut )?)?(" num "|" path ") \\))*"
            sub("^" q " " ident " : ", "", H)
            x = H; if (gsub(/ in( |$)/, " ", x) > 1) return
            if (H ~ /^for .* in /) { x = H; sub(/^for /, "", x); sub(/ in .*$/, "", x); lpat[f] = x }
            if (H ~ /(^| )array : : from_fn \( [|][^|]*[|]$/ && H !~ /(^| )(loop|while|for)( |$)/) {
                x = H; sub(/ ?[|]$/, "", x); sub(/^.*[|]/, "", x); lpat[f] = trim(x)
                outer[f] = 1; if (lpat[f] ~ ("^" ident "$")) lidx[f] = lpat[f]
            } else if (H ~ ("^for" pat " in 0 \\. \\. " path " \\. available_at_entry \\( \\)$")) {
                inner[f] = 1; outer[f] = 1; if (lpat[f] ~ ("^" ident "$")) lidx[f] = lpat[f]
                x = H; sub(/^.* in 0 \. \. /, "", x); sub(/ \. available_at_entry \( \)$/, "", x)
                selwalk(f, x, hstart[f])
            } else if (H ~ ("^for" pat " in 0 \\. \\. " ident "$")) {
                if (lpat[f] ~ ("^" ident "$")) lidx[f] = lpat[f]
                x = H; sub(/.* /, "", x); inner[f] = counted(x, hstart[f], hstart[f], hopen[f] - 1, 0); outer[f] = 1
                if (inner[f]) selwalk(f, crecv, cpos)
            } else if (H ~ ("^for" pat " in 0 \\. \\. (" num "|" path ")$")) {
                outer[f] = 1; if (lpat[f] ~ ("^" ident "$")) lidx[f] = lpat[f]
            } else if (H ~ ("^for" pat " in (& (mut )?" fields "|" fields " \\. (iter|iter_mut) \\( \\))" adapt "$")) {
                outer[f] = 1; enumidx(f, H)
                # The collection the loop walks: its leading fields, less a method that follows.
                x = H; sub(/^.* in /, "", x); sub(/^& (mut )?/, "", x); match(x, "^" fields)
                itrecv[f] = substr(x, 1, RLENGTH); if (substr(x, RLENGTH + 1, 2) == " (") sub(/ \. [^ ]*$/, "", itrecv[f])
            } else if (H ~ ("^for" pat " in " ident adapt "$")) {
                x = H; sub(/.* in /, "", x); sub(/ .*/, "", x); outer[f] = slice(x, f); itrecv[f] = x; enumidx(f, H)
            } else if (H ~ ("^while " ident " (! =|>) 0$")) {
                x = H; sub(/^while /, "", x); sub(/ .*/, "", x)
                if (counted(x, hstart[f], hstart[f], hopen[f] - 1, 0)) { wid[f] = x; selwalk(f, crecv, cpos) }
            }
        }
        # The index slot of a loop whose iterable ends in `.enumerate()`, when the pattern is
        # `(<ident>, ..)`: like a `0..<bound>` pattern or an `array::from_fn` parameter, it takes
        # a different value on each pass. Not over `&mut` of an iterator, which may never end.
        function enumidx(f, H,   x) {
            if (H !~ / \. enumerate \( \)$/ || H ~ / in & mut /) return
            x = lpat[f]; if (x !~ /^\( [A-Za-z_][A-Za-z0-9_]* , /) return
            sub(/^\( /, "", x); sub(/ .*/, "", x); lidx[f] = x
        }
        # #1418: which loops around the bounded drain `f` select the queue it reads. `R` is the
        # receiver of the count, read at offset `u`. Walking outward over the enclosing frames,
        # each loop `g` must select the current receiver (`selects`), which also names what the
        # next loop out must select (`NXT`): the collection `g` iterates when the receiver is the
        # pattern of `g`, or the base when it is the base indexed by the index of `g`. A range or
        # `from_fn` pattern names no collection, so every loop outside it is refused. Each step
        # is only a candidate: when `g` closes, `uses` must find every mention, in the body of
        # `g`, of the names the step read harmless, and only then is `sel[f, g]` set. A loop
        # still open at the region end never selects.
        function selwalk(f, R, u,   k, g, cur, pos, kp, hs, he) {
            cur = R; pos = u; kp = D + 1; hs = 0
            for (k = D; k >= 1; k--) {
                g = fid[k]; if (!isl[g]) continue
                if (cur == "" || pos <= hopen[g]) return
                WS = ""; WR = ""; NXT = ""
                # The header of the loop just walked names the receiver once, and runs once.
                if (hs) WR = hs " " he
                if (!selects(cur, k, kp, pos)) return
                pn++; pf[pn] = f; pws[pn] = WS; pwr[pn] = WR; plist[g] = plist[g] " " pn
                cur = NXT; pos = hstart[g]; kp = k; hs = hstart[g]; he = hopen[g] - 1
            }
        }
        # Whether the receiver `cur`, read at offset `pos` in the body of the loop at stack depth
        # `kg` (frames `kg`..`kp - 1` enclose the read), changes with each pass of the loop: a
        # root that `resolve`s to the pattern, then only fields and plain indexes; or a base
        # indexed by exactly the identifier that takes a new value on each pass (`lidx`: the
        # lone pattern of a `0..<bound>` loop, the index slot of `.enumerate()` or an
        # `array::from_fn` parameter) and rebound by nothing in the loop, then only fields. That
        # the base is the same on every pass is checked by `uses`, at the close (a base the
        # pattern binds is the first form). Each spelling read goes to `WS`, for `uses`.
        function selects(cur, kg, kp, pos,   x, ident, num, ws, wr) {
            ident = "[A-Za-z_][A-Za-z0-9_]*"; num = "[0-9][0-9_]*(u8|u16|u32|u64|usize)?"
            if (cur ~ ("^" ident "( \\. " ident "| [[] (" ident "|" num ") []])*$")) {
                x = cur; sub(/ .*/, "", x); ws = WS; wr = WR; WS = WS "\034" cur
                if (resolve(x, kg, kp, pos)) return 1
                WS = ws; WR = wr
            }
            if (cur ~ ("^" ident "( \\. " ident ")* [[] " ident " []]( \\. " ident ")*$")) {
                x = cur; sub(/^[^[]*[[] /, "", x); sub(/ .*/, "", x)
                NXT = cur; sub(/ [[].*$/, "", NXT); WS = WS "\034" NXT
                return x == lidx[fid[kg]] && binder(x, kg, kp, pos) == ""
            }
            return 0
        }
        # Whether `x`, read at `pos` in the loop at depth `kg`, names a queue selected by the
        # loop: its nearest binding inside the loop body is an accepted `let`, `let .. else` or
        # `if let` from a selected form, or, with no binding there, the pattern of the loop binds
        # it (`mut` too: reassigning it is a mention `uses` refuses). The text of each binding
        # read, less the `else` block of a `let .. else`, goes to `WR`.
        # A chain of more than 64 rebindings is refused: no drain needs one, and the cap bounds
        # the walk (each link rescans the loop body) even if a later edit lets a step look at its
        # own offset again.
        function resolve(x, kg, kp, pos,   v) {
            if (rdepth >= 64) return 0
            rdepth++; v = resolve1(x, kg, kp, pos); rdepth--
            return v
        }
        function resolve1(x, kg, kp, pos,   r, t, at, e, k, rhs, ident) {
            ident = "[A-Za-z_][A-Za-z0-9_]*"
            r = binder(x, kg, kp, pos)
            if (r == "?") return 0
            if (r == "") { NXT = itrecv[fid[kg]]; return lpat[fid[kg]] ~ tw(x) }
            k = substr(r, 2) + 0
            if (r ~ /^L/) { t = st_top[k]; at = st_start[k]; e = st_end[k] }
            else { t = hdr[fid[k]]; at = hstart[fid[k]]; e = hopen[fid[k]] - 1; if (t !~ /^if let /) return 0; sub(/^if /, "", t) }
            sub(/^(# (! )?\[[^]]*\] )*/, "", t)
            if (t ~ ("^let " x "( : [^=]*)? = ")) {
                rhs = t; sub(/^[^=]*= /, "", rhs)
            } else if (t ~ ("^let (Some|Ok) \\( " x " \\) = ")) {
                rhs = t; sub(/^[^=]*= /, "", rhs)
                if (r ~ /^L/) {
                    if (!sub(/ else \{\}$/, "", rhs)) return 0
                    if (!match(substr(F, at, e - at + 1), /[^A-Za-z0-9_]else[^A-Za-z0-9_]/)) return 0
                    e = at + RSTART - 1
                }
            } else return 0
            WR = WR " " at " " e
            if (rhs ~ /^& (mut )?/) { sub(/^& (mut )?/, "", rhs); return selects(rhs, kg, kp, at) }
            if (rhs ~ ("^" ident "( \\. as_(mut|ref) \\( \\))?$")) { sub(/ .*/, "", rhs); WS = WS "\034" rhs; return resolve(rhs, kg, kp, at) }
            return 0
        }
        # Whether the loop body `from`..`to` leaves the candidate step `p` sound: every mention
        # of the root of each spelling the step read (`pws`) lies in a binding or header the step
        # read (`pwr`), or reads that spelling, then only fields and plain indexes, as the
        # receiver of `available_at_entry`, `try_pop` or `map_or(0, ..)`. Any other mention may
        # move, swap, reassign or rebind the queue or its collection between passes
        # (`controls.swap(0, 1)`, `controls = ..`, `mem::swap(control, spare)`), so it refuses.
        function uses(p, from, to,   m, i, sp, r, a, pat, j, ch, s, nr, rg, ok) {
            m = split(pws[p], sp, "\034"); nr = split(pwr[p], rg, " ")
            for (i = 1; i <= m; i++) {
                if (sp[i] == "") continue
                r = sp[i]; sub(/ .*/, "", r); s = sp[i]; gsub(/ /, "", s); pat = ""
                for (j = 1; j <= length(s); j++) { ch = substr(s, j, 1); pat = pat (ch ~ /[A-Za-z0-9_]/ ? ch : "[" ch "]") }
                pat = "^" pat "([.][A-Za-z_][A-Za-z0-9_]*|[[][A-Za-z0-9_]+[]])*([.]as_(mut|ref)[(][)])?[.](available_at_entry[(]|try_pop[(]|map_or[(]0,)"
                a = from
                while (a <= to && match(substr(F, a, to - a + 1), "(^|[^A-Za-z0-9_])" r "([^A-Za-z0-9_]|$)")) {
                    a += RSTART - 1; if (substr(F, a, length(r)) != r) a++
                    ok = 0
                    for (j = 1; j < nr; j += 2) if (a >= rg[j] + 0 && a <= rg[j + 1] + 0) ok = 1
                    if (!ok) { s = substr(F, a, 400); gsub(/ /, "", s); ok = s ~ pat }
                    if (!ok) return 0
                    a += length(r)
                }
            }
            return 1
        }
        # The nearest binding of `x` in the body of the loop at depth `kg` before offset `pos`:
        # "" for none, "L<n>" for the `let` statement `n`, "H<k>" for the header of the enclosing
        # frame at depth `k`, or "?" where a macro call (but `debug_assert!`), a `static`,
        # `const` or `use` item, or anything else may bind it unseen. A `let` binds `x` when its
        # raw text before its `=` names it, so a struct pattern, whose braces read as one `{}`
        # token, is still seen; a header binds it when its raw text names it, unless it is an
        # `if` whose condition holds no `let`, `|` or `=>`. Only headers that start before `pos`
        # count, so each step of `resolve` looks strictly earlier and the walk ends.
        function binder(x, kg, kp, pos,   g, s, k, h, t, bp, best) {
            g = fid[kg]
            if (item(x) || macro_named(x, hopen[g] + 1, pos - 1)) return "?"
            delete inr; for (k = kg; k < kp; k++) inr[fid[k]] = 1
            best = ""; bp = 0
            for (s = ns; s >= 1; s--) {
                if (!inr[st_frame[s]] || st_start[s] <= hopen[g] || st_end[s] >= pos) continue
                t = st_top[s]; sub(/^(# (! )?\[[^]]*\] )*/, "", t)
                if (t !~ /^let /) continue
                t = substr(F, st_start[s], st_end[s] - st_start[s] + 1)
                if (match(t, /[^=!<>]=[^=>]/)) t = substr(t, 1, RSTART)
                if (t ~ ("(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) { best = "L" s; bp = st_start[s]; break }
            }
            for (k = kp - 1; k > kg; k--) {
                h = fid[k]; if (hstart[h] >= pos) continue
                if (hstart[h] <= bp) break
                t = substr(F, hstart[h], hopen[h] - hstart[h])
                if (t !~ ("(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) continue
                if (hdr[h] ~ /^if / && t !~ /(^|[^A-Za-z0-9_])let([^A-Za-z0-9_]|$)|[|]|=>/) continue
                return "H" k
            }
            return best
        }
        # Whether a mention of `x` in `from`..`to` sits in a macro call other than
        # `debug_assert!` (a macro may expand its tokens into a binding of `x`).
        function macro_named(x, from, to,   p, k, d, ch, j, w) {
            p = from
            while (p <= to && match(substr(F, p, to - p + 1), "(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) {
                p += RSTART - 1; if (substr(F, p, length(x)) != x) p++
                d = 0
                for (k = p - 1; k >= from; k--) {
                    ch = substr(F, k, 1)
                    if (ch == ")" || ch == "]" || ch == "}") { d++; continue }
                    if (ch != "(" && ch != "[" && ch != "{") continue
                    if (d) { d--; continue }
                    j = k - 1; while (substr(F, j, 1) == " ") j--
                    if (substr(F, j, 1) != "!") continue
                    j--; while (substr(F, j, 1) == " ") j--
                    w = ""; while (j >= 1 && wc(substr(F, j, 1))) w = substr(F, j--, 1) w
                    if (w != "" && substr(F, j, 1) == "#" && substr(F, j - 1, 1) == "r") return 1
                    if (w != "" && w !~ /^(if|while|match|return|break|in|else|let|mut)$/ && w != "debug_assert") return 1
                }
                p += length(x)
            }
            return 0
        }
        # Whether `x` is a slice or array parameter (`&[T]`, `&mut [T]` or `[T; N]`) of the
        # innermost enclosing `fn`, named nowhere between the `{` of that function and the loop
        # at `f`, and named by no `static`, `const` or `use` in the region (a block item shadows
        # the parameter for its whole block, even when it is declared after the loop).
        function slice(x, f,   k, g, h) {
            for (k = D; k >= 1; k--) {
                g = fid[k]; h = hdr[g]; sub(/^(# (! )?\[[^]]*\] )*/, "", h)
                if (h !~ /(^| )fn [A-Za-z_]/) continue
                return h ~ ("[(,] (mut )?" x " : (& (" q " [A-Za-z_][A-Za-z0-9_]* )?(mut )?)?[[]") && mentions(x, hopen[g] + 1, hstart[f] - 1) == 0 && !item(x)
            }
            return 0
        }
        function openblock(at,   f, H, k, m) {
            f = ++nf; H = trim(segtop[D]); hdr[f] = H; hstart[f] = segstart[D]; hopen[f] = at
            classify(f, H)
            # A pop in a loop header runs once per iteration, and no bounded header holds one.
            if (isl[f]) { m = split(segpops[D], tmp, " "); for (k = 1; k <= m; k++) bad[tmp[k]] = 1 }
            D++; fid[D] = f; onstack[f] = 1; segtop[D] = ""; segpops[D] = ""; segstart[D] = at + 1; pd[D] = 0
        }
        # Whether a block with header `H` is the body of the block-like statement its segment
        # starts with (`if`, `match`, `while`, `for`, `loop`, `unsafe`, a label, a bare block, or
        # the last arm of an `if`/`else` chain): the head after its keyword holds no other
        # construct that takes a block and ends where an expression can end (a name or a literal,
        # `)`, `]` or `?`), not on an operator, a keyword or a macro `!`, before which the `{`
        # would open a block operand. A `{` after such a body opens a new block.
        function complete(H,   r, last) {
            sub("^" q " [A-Za-z_][A-Za-z0-9_]* : ?", "", H)
            if (H == "" || H == "loop" || H == "unsafe") return 1
            if (H ~ /^if / && H ~ / else$/) return 1
            if (H ~ /^if / && H ~ / else if /) { r = H; sub(/^.* else if /, "", r) }
            else if (H ~ /^(while|for|if|match) /) { r = H; sub(/^[a-z]+ /, "", r) }
            else return 0
            if (r ~ /(^| )(match|if|while|for|loop|unsafe|async|move|const|else|[|]|\{\})( |$)/) return 0
            last = r; sub(/.* /, "", last)
            if (last ~ /^(as|break|continue|in|let|mut|ref|return|static|dyn|yield|box)$/) return 0
            return last ~ /^([A-Za-z0-9_]+|[])?"])$/
        }
        function closeblock(at,   f, body, x, w, j, m, k) {
            # A `}` that closes a block opened before the region ends the scope of every `let` the
            # region made at its outermost level.
            if (D == 0) { onstack[fid[0]] = 0; fid[0] = -(++zc); onstack[fid[0]] = 1; reset(at + 1); return }
            f = fid[D]
            if (wid[f] != "") {
                x = wid[f]; body = substr(F, hopen[f] + 1, at - hopen[f] - 1)
                if (match(body, ("^ *" x " *-= *1 *;"))) inner[f] = outer[f] = mentions(x, hopen[f] + RLENGTH + 1, at - 1) == 0
            }
            # A candidate selection by this loop holds only if its body leaves the queue in place.
            if (plist[f] != "") {
                m = split(plist[f], pl, " ")
                for (k = 1; k <= m; k++) if (uses(pl[k], hopen[f] + 1, at - 1)) sel[pf[pl[k]], f] = 1
                plist[f] = ""
            }
            onstack[f] = 0; D--
            tok("{}")
            # After a closed block, outside brackets, a new statement or item starts at a `{`
            # when the block was a complete statement body, or at an identifier other than `as`,
            # `else` or `in` (which continue the expression or pattern), a label or an
            # attribute. Anything else keeps the segment open.
            if (pd[D]) return
            j = at + 1; while (substr(F, j, 1) == " ") j++
            if (substr(F, j, 1) == "{" && complete(hdr[f])) { reset(j); return }
            if (substr(F, j, 1) == q || substr(F, j, 1) == "#") { reset(j); return }
            w = ""; while (wc(substr(F, j + length(w), 1))) w = w substr(F, j + length(w), 1)
            if (w ~ /^[A-Za-z_]/ && w != "as" && w != "else" && w != "in") reset(j)
        }
        function check(  k, i, j, L, ch, w, line, s, m, f, l, ok) {
            st = 0; F = ""
            for (k = 1; k <= n; k++) {
                cl[k] = clean(k); off[k] = length(F) + 1; F = F cl[k] " "
                s = cl[k]; gsub(/(core|std)::array::from_fn/, " ", s)
                if (s ~ "(^|[^A-Za-z0-9_])(from_fn|repeat_with|successors)([^A-Za-z0-9_]|$)") hit(k)
            }
            off[n + 1] = length(F) + 1
            D = 0; nf = 0; np = 0; ns = 0; zc = 0; fid[0] = 0; onstack[0] = 1; pd[0] = 0; reset(1)
            for (line = 1; line <= n; line++) {
              s = cl[line]; L = length(s)
              for (j = 1; j <= L; j++) {
                ch = substr(s, j, 1); i = off[line] + j - 1
                if (ch == " ") continue
                if (wc(ch)) {
                    w = ch; while (wc(substr(s, j + 1, 1))) w = w substr(s, ++j, 1)
                    tok(w)
                    if (w == "try_pop") {
                        np++; pline[np] = line; bad[np] = segtop[D] ~ tw("from_fn"); stack[np] = ""
                        for (k = 1; k <= D; k++) stack[np] = fid[k] " " stack[np]
                        segpops[D] = segpops[D] " " np
                    }
                    continue
                }
                if (ch == "{") { openblock(i); continue }
                if (ch == "}") { closeblock(i); continue }
                if (ch == ";" && !pd[D]) {
                    ns++; st_top[ns] = trim(segtop[D]); st_start[ns] = segstart[D]; st_end[ns] = i; st_frame[ns] = fid[D]
                    reset(i + 1); continue
                }
                if (ch == "(" || ch == "[") pd[D]++
                if ((ch == ")" || ch == "]") && pd[D]) pd[D]--
                tok(ch)
              }
            }
            # A region that ends inside a literal or a block comment was not read as code.
            if (st) hit(n)
            # Judge each pop: its innermost enclosing loop must bound it, every other must be a
            # finite outer loop that selects the queue it drains. A block left open at the region
            # end is judged as it stands.
            for (i = 1; i <= np; i++) {
                ok = !bad[i]; l = 0; m = split(stack[i], tmp, " ")
                for (k = 1; k <= m && ok; k++) {
                    f = tmp[k]
                    if (!isl[f]) continue
                    if (l) ok = outer[f] && ((l, f) in sel); else { ok = inner[f]; l = f }
                }
                if (!ok) hit(pline[i])
            }
            delete sel; delete plist; delete pf; delete pws; delete pwr; pn = 0
            for (k = -zc; k <= nf; k++) { delete onstack[k] }
            for (i = 1; i <= np; i++) delete bad[i]
            delete seen_line
        }
        function hit(k) { if (!seen_line[k]++) print regfile ":" ln[k] ":" c[k] }
        function finish() { check(); inside = 0 }
        FNR == 1 && inside { finish() }
        /REALTIME_POLICY_BEGIN/ { if (!inside) { inside = 1; n = 0; regfile = FILENAME }; next }
        /REALTIME_POLICY_END/ { if (inside) finish(); next }
        inside { n++; c[n] = $0; ln[n] = FNR }
        END { if (inside) finish() }
    ' "${marked_list[@]}" 2>&1)"; then :; else rc=$?; printf '%s\n' "$drain_hits" >&2; fail "realtime drain-bound scan failed (awk status $rc)"; fi
    [[ -z "$drain_hits" ]] || {
        printf '%s\n' "$drain_hits" >&2
        fail 'marked realtime unbounded try_pop drain (bound it with available_at_entry)'
    }
fi

# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12): it scans the whole
# {crates,hosts,tools} tree, of which the realtime module is a part, rather than one of five copies
# of the same regex over five different root lists.

# Issue #1047 moved this scan here from capi's `ffi_never_forms_a_whole_plan_reference` test. No
# workflow runs Miri, so it is the only guard of the C ABI's control/render split: the render
# thread holds `&mut PlanState` while any thread may query the same handle, so the production
# code of `crates/capi/src/ffi.rs` never forms a reference to the whole `Plan`. It projects the
# fields it needs with `&raw const`/`&raw mut` (`plan_state`, `plan_error_slot`, `plan_queries`).
# `&*plan.cast::<HandleHeader>()` borrows only the shared header, `&*plan_state(..)` names another
# item, and `&(*plan).field` borrows one field, so a projection after the dereference is not a
# hit. Comment lines and the test module are not production code. The render entry point must be
# in the scanned region, so a renamed file or a moved test-module marker cannot pass as clean.
capi_ffi='crates/capi/src/ffi.rs'
[[ -f "$capi_ffi" ]] || fail "missing $capi_ffi"
if capi_production="$(awk '
    previous == "#[cfg(test)]" && $0 == "mod tests {" { exit }
    { previous = $0 }
    /^[[:space:]]*\/\// { next }
    { print FILENAME ":" FNR ":" $0 }
' "$capi_ffi" 2>&1)"; then :; else rc=$?; printf '%s\n' "$capi_production" >&2; fail "capi production-region extraction failed (awk status $rc)"; fi
[[ "$capi_production" == *miso_engine_v1_render_f32_planar* ]] ||
    fail "$capi_ffi has no production render entry point to scan"
whole_plan="$(gate_scan_text_collect 'whole-plan reference' '&(mut[[:space:]]+)?(\*plan|\(\*plan\))([^[:alnum:]_.]|$)' "$capi_production")" || exit $?
[[ -z "$whole_plan" ]] || {
    printf '%s\n' "$whole_plan" >&2
    fail "the C ABI forms a reference to a whole Plan; project the field with &raw const or &raw mut"
}

printf 'realtime policy: ok (%s marked regions in %s files)\n' "$marker_count" "$marked_file_count"
