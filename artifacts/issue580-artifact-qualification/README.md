# Issue #580 AudioWorklet artifact qualification

Lane B qualified candidate commit `517bbf486f84fdfd6d59c42f7683348b55cea781` from frozen source
head `517bbf486f84fdfd6d59c42f7683348b55cea781`. The CI-observed candidate hash was independently
reproduced by the repin-mode probe and a direct pre-pin Wasm build:
`29abe2fa838ad4c24cbf19db9ac4185ac95c98d8e4f9e49ed8d668a35e577226`.

The pre-pin artifact passed `check-web-audioworklet.sh` and
`check-browser-expected-resources.py --artifacts`, including the native target-independent PCM
witness and all 26 expected-resource red mutations. After those gates, the shipped pin changed
from the inherited #578 value `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`
to the candidate. The ordinary locked builder then returned status 0 and reproduced the candidate
hash. All six shipped files matched the independent pre-pin build byte-for-byte:

- ABI layout: `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`
- AudioWorklet host declaration: `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`
- AudioWorklet host JavaScript: `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`
- AudioWorklet JavaScript: `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`
- simd128 Wasm: `29abe2fa838ad4c24cbf19db9ac4185ac95c98d8e4f9e49ed8d668a35e577226`
- parameter metadata: `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`

Final static/object/ABI and resource/PCM checks passed. SDK dependency installation and
`bash scripts/sdk-package.sh check` passed, including all 11 SDK tests and the publishable
package smoke gate. Playwright 1.62.1 qualification ran all three engines with the exact artifact
set and self-test mutations: Chromium `151.0.7922.34`, Firefox `153.0`, and WebKit `26.5` all
passed attestation, AudioWorklet boot, native-corpus digest, control path, observation, and
100 ms main-thread stall gates. `results.json` and the generated browser matrix were recorded
with the candidate commit and Wasm hash, and the generated-matrix check passed.

Each recorded command has a command file, pre-command context, exit status, stdout/stderr and
lossless gzip copies. The directory checksum manifest covers all retained evidence except itself.
`Cargo.lock` is restored; `target/` and temporary build directories are outside the worktree.
