# Issue 539 ordinary AudioWorklet artifact decision

Reviewer: Astra LOW acting for lane B.

Decision: **DRIFT — bounded artifact qualification decision required; no retained-artifact PASS.**

Exactly one ordinary `bash scripts/build-web-audioworklet.sh` invocation ran at clean, pushed
head `e8d1f2461f07806df307dcdc138f7e26edf07361`, with main/merge-base
`9e113be98cf31c1eaf4297b0a031518244b71c33`. No repin or strip override environment was present.
Exact argv, cwd and scratch output are in `command.json`; complete streams and status are retained.

Compilation succeeded. The builder exited 1 because the observed simd128 Wasm SHA-256 is
`7e242eb8f283bcba2ef5778cd430950fee7df92ef66b6ddaaf2a0a68e5bc7409`, differing from delivered
pin `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`.
The unchanged ordinary builder checks the pin before copying any output, so the scratch directory
contains zero files. Its EXIT trap removed the temporary module. No six-file candidate comparison
or candidate module preservation is claimed; the observed digest is the builder's retained output.
The canonical delivered six-file manifest from #587 is preserved for comparison; #608 previously
confirmed exact identity to that manifest. Builder, Cargo configuration/lock/toolchain, web source
copies and pin are unchanged from delivered main, as the input hashes and empty main diff show.

The accepted limiter source changes a builder dependency, so this is artifact drift requiring the
separate decision specified by #539. Existing #587 browser/consumer qualification remains attributed
to its old digest and does not qualify this observed digest. Lane B must record bounded scratch
candidate production and static/resource/hermetic/three-browser obligations before any pin or
consumer update. No candidate qualification, repin or delivery approval is supplied here.

No retry, second build, browser qualification, benchmark, timing workload, capture, source/spec/
workflow/consumer/pin edit, commit, push or GitHub mutation occurred. Only this bounded evidence
folder was added. The compiler's duration is ordinary build output, not performance evidence.
