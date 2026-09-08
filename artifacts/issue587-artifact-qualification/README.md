# Issue #587 AudioWorklet artifact qualification

Lane B qualified the AudioWorklet artifact from frozen source head
`ab3766caef34bcb035d7394224b0ccff1ea0be2d`. The independently established candidate Wasm digest
was `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`; the inherited pin was
`29abe2fa838ad4c24cbf19db9ac4185ac95c98d8e4f9e49ed8d668a35e577226`. The existing `01-*` and
`02-*` command records were retained, and the numbered qualification rerun continues at `03-*`.

The independent pre-pin six-file build and the pinned rebuild both passed. The exact six-file
comparison passed byte-for-byte with these hashes:

- ABI layout: `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`
- AudioWorklet host declaration: `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`
- AudioWorklet host JavaScript: `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`
- AudioWorklet JavaScript: `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`
- simd128 Wasm: `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`
- parameter metadata: `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`

Pre-pin and post-pin static/object/ABI checks passed. The resource/native PCM witness passed with
all 26 expected-resource red mutations. SDK dependency installation and
`bash scripts/sdk-package.sh check` passed, including all 11 SDK tests and the publishable package
smoke gate. Chromium `151.0.7922.34`, Firefox `153.0`, and WebKit `26.5` passed attestation,
AudioWorklet boot, native-corpus digest, control path, observation, and 100 ms main-thread stall
gates with browser self-test mutations. The generated matrix check and artifact-evidence leak
checks passed.

Every recorded command has a command file, context, exit status, stdout/stderr, and lossless gzip
copies. `Cargo.lock` is unchanged from the frozen head. Build targets and temporary outputs are
outside the worktree. The directory checksum manifest covers all retained evidence except itself.
