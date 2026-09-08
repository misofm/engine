# Issue 627 final post-pin attempt 3

This evidence is for the final attempt authorized at promotion head
`0bb5a820be33a49393386617358ec1db2fe7577d`. The detached worktree was clean.

The preflight created and proved the new external directory
`/tmp/issue627-postpin-attempt3-output-20260908` existed, was not a symlink,
and was empty. The unchanged no-bypass builder ran exactly once, returned zero,
produced exactly six files, and `02-output-identity.*` proves basename-normalized
byte identity with `/tmp/issue627-qualified-output`.

This attempt is stopped procedurally. My numbered `03-static` and
`04-resources` records each returned zero, but separate concurrent executor
records (`03-check-web.*`, `04-expected-resources.*`, and `05-test-web.*`) were
observed for the same output and gate family. The exact-once post-pin contract
therefore cannot be credited from this shared run. My hermetic invocation had
started when the duplicate was detected; it was terminated before completion,
and its partial streams are retained without a fabricated status. No SDK,
matrix, formatting, workspace, or effect-runtime stage was started by my
sequence.

No browser or package install, compiler capture, benchmark, timing operation,
source/spec/promotion edit, commit, push, PR, or GitHub operation was performed.
