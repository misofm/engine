# Issue 614 ordinary AudioWorklet artifact decision

Decision: **DRIFT — retained artifact qualification does not apply.**

Exactly one ordinary `bash scripts/build-web-audioworklet.sh` invocation ran at clean, pushed
source-accepted head `0c715de9fbdf0b3873c707e10c52095fad750287`, with main/merge-base
`9e113be98cf31c1eaf4297b0a031518244b71c33`. No repin or strip override was supplied.
The exact command, cwd, full streams and exit status are retained here.

Compilation succeeded. The builder exited 1 because observed simd128 Wasm SHA-256
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b` differs from delivered
pin `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`. The unchanged builder
checks the pin before copying output, so the scratch output directory contains zero files and its
temporary module was removed by the exit trap.

This candidate differs from #539's independently observed limiter candidate
`7e242eb8f283bcba2ef5778cd430950fee7df92ef66b6ddaaf2a0a68e5bc7409`. Neither candidate
qualifies the other. The delivered six-file manifest remains the comparison baseline, and the
builder, lock, toolchain, Cargo configuration, web sources and pin are unchanged from main.

No retry, overlay build, static/resource/hermetic/browser qualification, repin, consumer update,
benchmark, timing workload, source edit or workflow edit occurred in this probe. A bounded
artifact-promotion successor must reproduce the candidate from this frozen source with only a
scratch pin overlay, run the existing static/resource/hermetic/SDK/three-browser gates, and obtain
Astra LOW PASS before any repository pin or consumer metadata changes.
