# Issue 619 ordinary post-pin build

Decision: **PASS — the repository pin reproduces the qualified preserved candidate.**

Root ran exactly one ordinary no-bypass `scripts/build-web-audioworklet.sh` invocation at clean,
pushed head `8577f1aa4e2f723929137a978703581ea25da483`. The command continued under one process after its
initial output yield, exited zero, and published exactly six files.

All six outputs match the preserved #617/#619 qualified candidate byte-for-byte. The Wasm is
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`; the five non-Wasm files
retain their delivered CP8 identities. Full build streams/status and pre-build source identities are
adjacent. Generated output files and build targets remain outside Git.

Astra LOW post-pin static/resource/hermetic/SDK and exact-head/current-main review remain required.
