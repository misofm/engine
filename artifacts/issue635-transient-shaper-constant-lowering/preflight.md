# Issue 635 preflight record

The original preflight ran after `capture-plan.md` was created and before any
compiler command. No compilation or audio execution occurred during these
checks. Attempt 2 revalidated the retained source/configuration identities
against the corrected plan; it did not rerun compilation.

| Check | Command/result |
| --- | --- |
| exact HEAD | `git rev-parse HEAD` = `186e6b3080b040d4a6e7c25b1516762224297381` (status 0) |
| exact required base object | `git cat-file -e 62045f40048ec230298fe0fd3935da3333f90b83^{commit}` (status 0) |
| exact merge base | `git merge-base HEAD 62045f40048ec230298fe0fd3935da3333f90b83` = `62045f40048ec230298fe0fd3935da3333f90b83` (status 0) |
| named local `base/main` ref | absent in this worktree (the direct required commit object and merge-base check passed; no alternate base was used) |
| tracked worktree diff | `git diff --name-only` empty; `git diff --check` status 0 |
| source/configuration hashes | 11/11 retained values match the corrected `capture-plan.md`, including `.cargo/config.toml` = `03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e` (attempt-1’s malformed plan spelling is superseded; validation status 0) |
| locked metadata | `cargo metadata --locked --no-deps --format-version 1 >/dev/null` (status 0) |
| Rust toolchain | `rustc -Vv`: 1.97.1 / LLVM 22.1.6; `cargo -V`: 1.97.1 (status 0) |
| installed targets | `wasm32-unknown-unknown`, `x86_64-unknown-linux-gnu` (status 0) |
| inspection tools | `/usr/bin/wasm-objdump`, `/usr/bin/wasm2wat`, `/usr/bin/llvm-objdump` (status 0) |
| temporary targets | all three planned `/tmp/issue635-transient-*` directories absent (status 0) |

The evidence directory itself is the only untracked path introduced by this
tranche. Product, tests, manifests, lockfile, generated resources, and target
directories are unchanged. The direct required base hash is authoritative for
this worktree because no local symbolic `base/main` ref is installed.
