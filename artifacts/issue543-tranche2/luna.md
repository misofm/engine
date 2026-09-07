Implemented the remaining #543 raw-byte lowercase hex delegations. No commit or GitHub mutation was made. HEAD remains `acd57011eed98bf87975c36eb2c5070500897e0c`.

Manifest/lock changes:

- Added `engine` under dev-dependencies in `math`, `effect-runtime`, and `native-pcm-runner`.
- Added production `engine` dependency to `stem-hasher`.
- `Cargo.lock` changed only in those four local package dependency lists.

All requested gates passed:

- fmt check: 0
- Bench-support lock-update test: 5 passed
- Graph compiler: 84 passed
- Math: 9 passed
- Effect-runtime: 3 passed, 1 ignored
- Native PCM runner: 19 passed
- Wasm gates: 9 passed
- Effect package: 3 passed
- Effect compiler: 8 passed, 1 ignored
- Stem hasher: 11 passed
- Host-web native tests: 3 passed
- SDK oracle check: 0
- Parameter metadata: 10 passed
- Lattice oracle check: 0
- `git diff --check`: 0

Capture records are in `/tmp/issue543/tranche2/`. The first final-audit record retained a harmless `printf` label warning; the clean recapture passed.

Final census found only:

- `engine::hex_lower`’s authority nibble table.
- Excluded decorated `0x..` emitters.
- Excluded `rack_fixture.rs` encoder, reserved for #545/TOOL14 retirement.
- Parsers, word-oriented wasm-console formatting, and previously delivered adapters remain excluded.

`git diff --stat`:

```text
20 files changed, 25 insertions(+), 83 deletions(-)
```

Modified paths:

```text
Cargo.lock
crates/effect-compiler/tests/migration_terminal.rs
crates/effect-package/tests/package_v1_qualification.rs
crates/effect-runtime/Cargo.toml
crates/effect-runtime/tests/determinism.rs
crates/graph-compiler/src/bin/graph_fixture.rs
crates/graph-compiler/src/canonical.rs
crates/math/Cargo.toml
crates/math/tests/m2_lane_identity.rs
crates/math/tests/m3_determinism.rs
hosts/host-web/examples/sdk_render_oracle.rs
hosts/host-web/src/tests.rs
tools/bench-support/src/digest.rs
tools/native-pcm-runner/Cargo.toml
tools/native-pcm-runner/src/lib.rs
tools/parameter-metadata/src/bin/lattice_oracle.rs
tools/stem-hasher/Cargo.toml
tools/stem-hasher/src/lib.rs
tools/stem-hasher/tests/conformance.rs
tools/wasm-gates/src/lib.rs
```

SHA-256 values were captured for every modified file; worktree status contains exactly those 20 paths, all modified and uncommitted.