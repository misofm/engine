# Issue #578 AudioWorklet artifact identity qualification

Root qualified the ordinary six-file AudioWorklet artifact at frozen source/decision head `cf7629a0` after Astra LOW accepted attempt-2 source. The sole preceding repin-mode probe returned the existing pin and changed no repository file.

The ordinary build wrote exactly the expected six files to `/tmp/issue578-artifact-cf7629a0`. Every file is byte-identical to the retained #570 qualified artifact at `/tmp/issue570-qualified-artifact-3871e137`:

- ABI layout JSON: `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`
- AudioWorklet host declaration: `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`
- AudioWorklet host JavaScript: `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`
- AudioWorklet JavaScript: `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`
- simd128 Wasm: `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`
- parameter metadata JSON: `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`

All eight recorded commands exited zero: ordinary build, exact six-file comparison, static/object/ABI Web checks, expected resource/native-witness validation with its 26 red mutations, generated matrix check, clean SDK dependency installation, SDK generated-surface/package/self-test, and final source/pin/status verification. The SDK suite passed 11 tests and the publishable-tarball gate. Resource and PCM witnesses remain unchanged.

No browser was launched for #578. Exact byte identity reuses #570's original browser qualification under candidate `3871e137b519815540c1b3abcd6cfcec7932efa7`: Chromium `151.0.7922.34`, Firefox `153.0`, and WebKit `26.5`. This is inherited evidence with its original attribution, not a new browser run.

The repository pin remains `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`; no pin or lineage file changed. Each command has its exact invocation, pre-command context, deterministic-gzip stdout/stderr and exit record. `sha256sums.txt` covers every retained payload except itself and verifies from the repository root.

Artifact evidence still requires Astra LOW review before the final delivery decision, PR creation or merge.
