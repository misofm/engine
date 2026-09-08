# Issue #608 AudioWorklet identity probe

After Astra LOW accepted source `0820c8a7ba785e5834694f1a5663dd7b2b835d46` and root pushed the
source verdict, root ran the ordinary no-bypass AudioWorklet builder exactly once at clean
documentation head `6366c304627f28ae8e27d3b9e5f7acce3860225a`.

The builder exited zero and emitted exactly six files under the fresh empty directory recorded in
`identity-probe.command.txt`. Their SHA-256 manifest is byte-identical to the canonical delivered
#587 six-file artifact in `artifacts/issue587-artifact-qualification/05-pin-and-build.sha256`:

- ABI layout: `40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919`
- host declaration: `445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf`
- host JavaScript: `21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a`
- AudioWorklet JavaScript: `225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb`
- simd128 Wasm: `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`
- parameter metadata: `6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d`

The Wasm digest equals the repository pin. The three copied JS/declaration files also compare
byte-for-byte with their repository sources. The probe used no repin or bypass environment,
changed no pin or generated consumer, and launched no browser. No retry or second artifact build
ran. Existing #587 browser and consumer attribution remains subject to Astra LOW artifact review.
