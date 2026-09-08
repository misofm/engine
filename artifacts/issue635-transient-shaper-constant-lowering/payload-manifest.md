# Issue 635 capture payload manifest

The compiler payloads below remain in `/tmp` and were never copied into the
repository. These are the only payload identities retained for reproducible
static inspection. The command stdout, stderr, and status files are the raw
capture records in each target directory.

| Shape | Assembly payload | LLVM IR payload | Status |
| --- | --- | --- | --- |
| native scalar + AVX2 W8 | `/tmp/issue635-transient-native-target/release/deps/transient_shaper-f9656eb1caa13868.s`, 2,476,911 bytes, SHA-256 `ba9753757c06d8ca41d75c39fb27b4fea6425f1b20213bf7812d8cc1f9464786` | `/tmp/issue635-transient-native-target/release/deps/transient_shaper-f9656eb1caa13868.ll`, 5,126,399 bytes, SHA-256 `e65f936e8fc7d068029328915a104e5b64c2eefc1363b61147fdf0ebe1864b85` | `native/cargo.status` = `0` |
| Wasm scalar | `/tmp/issue635-transient-wasm-scalar-target/wasm32-unknown-unknown/release/deps/transient_shaper-68ed3788e42b95c7.s`, 6,648,320 bytes, SHA-256 `7bc83f6671fb03acf11d32b172c62508300e885df9c0d1a52d4da834a1d5dcf4` | `/tmp/issue635-transient-wasm-scalar-target/wasm32-unknown-unknown/release/deps/transient_shaper-68ed3788e42b95c7.ll`, 9,112,995 bytes, SHA-256 `dec78e0ac7bd31e5d4bbf93df6e2f194f247720d669ee70b1d49bb0965eef78e` | `wasm-scalar/cargo.status` = `0` |
| Wasm simd128 W4 | `/tmp/issue635-transient-wasm-simd128-target/wasm32-unknown-unknown/release/deps/transient_shaper-f46dd04c0682d0b7.s`, 2,965,834 bytes, SHA-256 `e050f25830bfc8a1e0dfd1595e22682332f08f29770a0d2186ba0d5b71215217` | `/tmp/issue635-transient-wasm-simd128-target/wasm32-unknown-unknown/release/deps/transient_shaper-f46dd04c0682d0b7.ll`, 5,550,847 bytes, SHA-256 `3335ace5c4b474ca7552a63ab648eb25e55ab52e8bceed2893347cf9f5c5de9b` | `wasm-simd128/cargo.status` = `0` |

The full payloads are not evidence artifacts. `capture-plan.md` freezes the
exact commands, flags, source hashes, toolchain, and expected status before
the first command; `preflight.md` records all prerequisite statuses.
