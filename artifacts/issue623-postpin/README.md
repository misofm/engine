# Issue 623 ordinary post-pin build

The single ordinary no-bypass builder invocation ran at clean, pushed promotion head `6fdbb377`.
It exited zero and emitted exactly six files into `/tmp/issue623-postpin-output`. Their basename-
normalized hashes match the qualified scratch output byte-for-byte, including Wasm
`ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`.

The first raw SHA-256 manifest diff returned status 1 solely because each manifest retained a
different absolute directory prefix. That raw diff and status remain preserved. Without rebuilding,
root normalized only each path to its basename and repeated the comparison over the already captured
hashes; `normalized-hash.status` is zero and `normalized-hash.diff` is empty. File-list comparison
was already zero. This is an evidence comparison correction, not a build retry.

No browser, static, resource, hermetic, SDK, benchmark, timing, source, pin, lineage, workflow, or
main mutation occurred in this step. The successful scratch browser qualification remains the only
browser run. Astra LOW must run the bounded post-pin gates and review this evidence before PR work.
