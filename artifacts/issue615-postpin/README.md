# Issue 615 ordinary post-pin build

Decision: **PASS — repository pin reproduces the qualified scratch candidate.**

Root ran exactly one ordinary no-bypass `scripts/build-web-audioworklet.sh` invocation at clean,
pushed head `5bf40ad03b4ba1c422eb721529ceb88eef30b7c8`. The command exceeded its initial 30-second
output yield but continued as the same process; root polled that process and did not start another
builder invocation. It exited zero and published exactly six files.

All six hashes match the qualified frozen-source candidate. The Wasm is
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`; ABI layout, declaration,
host JavaScript, worklet JavaScript and metadata retain the delivered hashes recorded in
`six-file.sha256`. Full build streams/status and pre-build source identity/status are adjacent.

Generated output files and build targets remain outside Git; their exact hashes are retained.
Astra LOW post-pin static/resource/hermetic/SDK and exact-head review remain required.
