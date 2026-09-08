# Issue 622 ordinary AudioWorklet artifact decision

Decision: **DRIFT — retained artifact qualification does not apply.**

Exactly one ordinary `bash scripts/build-web-audioworklet.sh
/tmp/issue622-artifact-output` invocation ran at clean, pushed source-accepted head
`ca5a8b49`, whose accepted production source is `fece7a2c`; live main and merge-base were
`cf9e079c`. No repin or strip override was supplied. The exact command, cwd, full streams, status,
source identities, frozen inputs, delivered web baseline, and empty output census are retained here.

Compilation succeeded. The unchanged builder exited 1 because the observed simd128 Wasm SHA-256
`ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3` differs from delivered pin
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`. The builder checks the pin
before copying output, so the scratch output directory contains zero files and its temporary module
was removed by the exit trap.

No retry, overlay build, qualification gate, browser run, SDK run, repin, consumer update,
benchmark, timing workload, source edit, workflow edit, or main mutation occurred. A separately
numbered lane-B successor must reproduce this candidate from the frozen accepted source with only a
scratch pin/lineage overlay, pass the existing static/resource/hermetic/SDK/three-browser gates,
receive Astra LOW review, and only then authorize the narrow pin/lineage promotion and post-pin
proof.
