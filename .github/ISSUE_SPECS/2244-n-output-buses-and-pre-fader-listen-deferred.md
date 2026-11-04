# N output buses and pre-fader listen (deferred)

Filed by root on 2026-11-04 with the owner's ruling on #210 (close #210; keep one narrow deferred issue). Deferred: not scheduled.

## Problem
#210 delivered solo, track delay, live trim/polarity and live sends. Two items were never built: N output buses and pre-fader listen (PFL). The graph compiler still refuses `outputs.len() != 1` (`crates/graph-compiler/src/compile.rs:268-269`), and the rack holds only a PFL seam (`crates/rack/src/lib.rs:3435, 3997`).

## Scope (when scheduled)
- N output buses: more than one session output, each with its own routes and meter, on both hosts (the browser maps them to channels of one AudioWorklet output or to separate nodes, decided in the brief).
- PFL: a listen bus fed pre-fader from any strip, which does not change the main mix.
- Keep #210's design notes and its verifier's corrections (see #210 and its spec in git history).

## Gate
Defined in the brief when the console asks for it.
