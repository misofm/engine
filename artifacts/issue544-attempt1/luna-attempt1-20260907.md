Attempt 1 stopped per protocol.

- Repository: `/home/bl/misofm/engine-audit-subject-disposition`
- Workflow changed: none; `git status` clean.
- Manifest: `/tmp/issue544/manifest-20260907T082112Z-544.txt`
- Pre/post workflow SHA-256: unchanged (`3434631972bf97a6c01a2c29090d89bd02969b28a0ae1d7956f8be2ec17eddcc`)
- Build: `cargo build --locked --release -p audit` — passed.
- Tests: `cargo test --locked --release -p audit` — 36 passed.
- `capi`: attempted exactly once; exit `127`, stdout empty. Failure: `./target/release/audit` absent because the mandated target directory placed the binary at `/tmp/issue544-target/release/audit`.
- `source-duration`: not run due stop-on-failure protocol.
- Captured evidence:
  - `/tmp/issue544/capi-stdout.json`
  - `/tmp/issue544/capi-stderr.txt`
  - `/tmp/issue544/capi-exit.txt`
- No JSON fields were observed.
- `cargo fmt` and YAML syntax validation were not run because execution stopped at the bounded launcher failure.
- Git diff scope: empty.