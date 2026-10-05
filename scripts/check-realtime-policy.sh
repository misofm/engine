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
# drains): twenty-five files and eighty-nine regions (they were twelve and forty-one after #664's
# complete LocalRing removal). Deleting a marker to silence the gate fails here -- the file leaves
# the discovered set or a region leaves the marked set -- instead of passing with less coverage.
# Raising a floor is part of the change that adds a marker.
[[ "$marked_file_count" -ge 25 ]] || fail "expected at least twenty-five marked realtime files"
[[ "$marker_count" -ge 89 ]] || fail "expected at least eighty-nine marked realtime regions"

gate_scan_forbidden 'marked realtime forbidden-body predicate' \
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp|\.expect\(|\.unwrap\(|panic!\(|unreachable!\(|todo!\(|unimplemented!\(' '' "$scratch_file" || exit $?

# #1253, #1302: a render-thread drain pops at most the records present at block entry. A loop
# that pops until the queue is empty keeps popping whatever a concurrent producer publishes while
# it runs (the C ABI prepares with `Concurrent` delivery) -- data-dependent, unbounded render
# work. The pass below reads each marked region as tokens, not lines, so layout plays no part:
# comments and string and character literals are blanked, a brace stack gives every block its
# header -- the tokens of its statement since the previous `;` outside brackets or the start of
# the enclosing block, across lines, each closed inner block one `{}` token -- and a block whose
# header holds `loop`, `while`, `for .. in` or `from_fn` is a loop. It refuses whatever it cannot
# prove bounded. What it proves, per marked region:
#   - a `try_pop` (the word: `Consumer::try_pop` counts, and the `fn try_pop` definition sits in
#     no loop) in the body of a loop has an innermost loop whose whole header, after an optional
#     label, is `for P in 0..<count>`, `for P in 0..<path>.available_at_entry()`, or
#     `while <count> != 0` (or `> 0`) whose body opens with `<count> -= 1;` and never names
#     `<count>` again. `<count>` is the nearest in-scope
#     `let [mut] <count>[: T] = <path>.available_at_entry();` (or
#     `= <path>.map_or(0, <Type>::available_at_entry);`), or one plain alias
#     `let [mut] <count> = <entry count>;`, and the code from that binding to the loop names
#     `<count>` nowhere else (no rebinding, assignment, borrow or other use);
#   - every other loop around that pop is finite: a bounded loop as above, `for P in 0..<literal>`,
#     `0..<fields>.len()`, `&<fields>`, `&mut <fields>` or `<fields>.iter()`/`.iter_mut()` (each
#     optionally `.enumerate()`d), or the closure of `core::array::from_fn`;
#   - a `try_pop` in the header of a loop (its condition, iterator or pattern, a block nested
#     there included) is refused: it runs once per iteration;
#   - `from_fn`, `repeat_with` and `successors` are refused outright, path-qualified or bare
#     (`core::array::from_fn` and `std::array::from_fn` excepted): they run a pop in a loop without
#     a loop keyword. A pop directly in an `array::from_fn` closure is refused (a constant count);
#   - a region still open where its file ends (markers out of order) is checked as it stands.
# A header ends early only where Rust must start a new statement or item. After a closed block
# that is: an identifier other than `as`, `else` or `in`, a label or an attribute; or a `{`, when
# the closed block was the body of a statement-leading `if`, `match`, `while`, `for`, `loop`,
# `unsafe`, label or bare block whose head ends where an expression can end (not on an operator,
# a keyword or a macro `!`, before which a `{` opens a block operand). Elsewhere it joins, which
# can only make a header look like a loop or miss a bounded form: both refuse. It is not a Rust
# parser, and it takes `available_at_entry`, `.len()`, `.iter()` and `.iter_mut()` at their word.
# These stay outside it: recursion; a pop in a helper function, through a function value, or in
# a closure that a loop calls or that is handed to a repeating adapter (`map_while` over
# `iter::repeat`, `for_each`, a renaming `use` of an iterator constructor); a loop around the
# drain outside the marked region (the rule is per region); and a macro that expands to a loop.
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
                    if (ch == "\"" && substr(s, i + 1, rh) == substr("################", 1, rh)) {
                        st = 0; out = out ch substr(s, i + 1, rh); i += rh; continue
                    }
                    out = out " "; continue
                }
                nx = substr(s, i + 1, 1)
                if (ch == "/" && nx == "/") { while (i <= len) { out = out " "; i++ }; break }
                if (ch == "/" && nx == "*") { st = 1; bd = 1; out = out "  "; i++; continue }
                if (ch == "\"") { st = 2; out = out ch; continue }
                if ((ch == "r" || ch == "b" && nx == "r") && (i == 1 || !wc(substr(s, i - 1, 1)))) {
                    j = i + (ch == "b") + 1; h = 0
                    while (substr(s, j, 1) == "#") { j++; h++ }
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
        # Whether `x` is an entry count at the `{` at `upto`: its nearest in-scope `let` before
        # `before` takes it from `available_at_entry` in a recognised form (or, at depth 0,
        # aliases one such count in one plain step), and from the end of that binding to `upto`
        # the code names `x` exactly once (the loop header or the alias): no rebinding,
        # assignment, borrow or other use of it in between.
        function counted(x, before, upto, depth,   k, t, b, chain) {
            for (k = ns; k >= 1; k--) {
                if (!onstack[st_frame[k]] || st_end[k] >= before) continue
                t = st_top[k]; sub(/^(# (! )?\[[^]]*\] )*/, "", t)
                if (t !~ ("^let( [^=]*)? " x "( |$)")) continue
                b = k; break
            }
            if (!b || mentions(x, st_end[b] + 1, upto) != 1) return 0
            chain = "[A-Za-z_][A-Za-z0-9_]*( \\. [A-Za-z_][A-Za-z0-9_]*( \\( \\))?)*"
            if (t ~ ("^let (mut )?" x "( : [A-Za-z_][A-Za-z0-9_]*)? = " chain " \\. (available_at_entry \\( \\)|map_or \\( 0 , [A-Za-z_][A-Za-z0-9_]* : : available_at_entry \\))$")) return 1
            if (depth == 0 && t ~ ("^let (mut )?" x "( : [A-Za-z_][A-Za-z0-9_]*)? = [A-Za-z_][A-Za-z0-9_]*$")) {
                sub(/^.* = /, "", t); return counted(t, st_start[b], st_end[b], 1)
            }
            return 0
        }
        function mentions(x, from, to,   s, m) {
            s = substr(F, from, to - from + 1); m = 0
            while (match(s, "(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)")) { m++; s = substr(s, RSTART + RLENGTH) }
            return m
        }
        # A block whose header holds `loop`, `while`, `for .. in` or `from_fn` is a loop. A loop
        # bounds a pop only if its whole header, after an optional label, is one of these:
        # `for P in 0..<count>`, `for P in 0..<path>.available_at_entry()`, or
        # `while <count> != 0` (or `> 0`) whose body opens with `<count> -= 1;` and never names
        # it again. Around such a drain, an enclosing loop may also be `for P in 0..<literal>`,
        # `0..<fields>.len()`, `&<fields>`, `&mut <fields>` or `<fields>.iter()`/`.iter_mut()`,
        # each optionally `.enumerate()`d, or the closure of `array::from_fn`, which runs a
        # constant number of times but never bounds a pop in its own body.
        function classify(f, H,   x, pat, ident, chain, fields) {
            isl[f] = H ~ tw("loop") || H ~ tw("while") || H ~ /(^| )for( | .* )in( |$)/ || H ~ tw("from_fn")
            inner[f] = 0; outer[f] = 0; wid[f] = ""
            if (!isl[f]) return
            ident = "[A-Za-z_][A-Za-z0-9_]*"
            pat = "( [A-Za-z0-9_(),&]+)+"
            fields = ident "( \\. " ident ")*"
            chain = ident "( \\. " ident "( \\( \\))?)*"
            sub("^" q " " ident " : ", "", H)
            x = H; if (gsub(/ in( |$)/, " ", x) > 1) return
            if (H ~ /(^| )array : : from_fn \( [|][^|]*[|]$/ && H !~ /(^| )(loop|while|for)( |$)/) {
                outer[f] = 1
            } else if (H ~ ("^for" pat " in 0 \\. \\. ([0-9][0-9_]*(u8|u16|u32|u64|usize)?|" fields " \\. len \\( \\))$")) {
                outer[f] = 1
            } else if (H ~ ("^for" pat " in 0 \\. \\. " ident "$")) {
                x = H; sub(/.* /, "", x); inner[f] = outer[f] = counted(x, hstart[f], hopen[f] - 1, 0)
            } else if (H ~ ("^for" pat " in 0 \\. \\. " chain " \\. available_at_entry \\( \\)$")) {
                inner[f] = 1; outer[f] = 1
            } else if (H ~ ("^for" pat " in (& (mut )?" fields "|" fields " \\. (iter|iter_mut) \\( \\))( \\. enumerate \\( \\))?$")) {
                outer[f] = 1
            } else if (H ~ ("^while " ident " (! =|>) 0$")) {
                x = H; sub(/^while /, "", x); sub(/ .*/, "", x)
                if (counted(x, hstart[f], hopen[f] - 1, 0)) wid[f] = x
            }
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
        function closeblock(at,   f, body, x, w, j) {
            if (D == 0) { reset(at + 1); return }
            f = fid[D]
            if (wid[f] != "") {
                x = wid[f]; body = substr(F, hopen[f] + 1, at - hopen[f] - 1)
                if (match(body, ("^ *" x " *-= *1 *;"))) inner[f] = outer[f] = mentions(x, hopen[f] + RLENGTH + 1, at - 1) == 0
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
            D = 0; nf = 0; np = 0; ns = 0; fid[0] = 0; onstack[0] = 1; pd[0] = 0; reset(1)
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
            # Judge each pop: its innermost enclosing loop must bound it, every other must be a
            # finite outer loop. A block left open at the region end is judged as it stands.
            for (i = 1; i <= np; i++) {
                ok = !bad[i]; l = 0; m = split(stack[i], tmp, " ")
                for (k = 1; k <= m && ok; k++) {
                    f = tmp[k]
                    if (!isl[f]) continue
                    ok = l ? outer[f] : inner[f]; l = 1
                }
                if (!ok) hit(pline[i])
            }
            for (k = 1; k <= nf; k++) { delete onstack[k] }
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
