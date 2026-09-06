# Candidate provenance completion

This record was added after product checkpoint `09c26a98` without rebuilding or rerunning a test.
The raw outputs named in `commands-and-statuses.txt` remain at their original `/tmp/463-sol2-*`
paths. `actual-statuses.tsv` records the numeric exit values returned by the original tool calls;
it does not infer success from prose or recreate a prior invocation.

## Manual worklet staging

The normal builder first compiled the candidate and returned status 1 after reporting the expected
content pin mismatch. To execute the existing direct oracle without changing that pin, the exact
manual build command actually run was:

```text
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/463-sol2-worklet-manual-target RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=/home/bl/.cargo=/cargo --remap-path-prefix=/home/bl/misofm/engine-lane2-plan=/repo" cargo build --locked --release --target wasm32-unknown-unknown -p host-web
```

That command returned numeric status 0 and its original output is
`/tmp/463-sol2-worklet-manual-build.log`. The exact staging sequence subsequently executed was:

```text
cp /tmp/463-sol2-worklet-manual-target/wasm32-unknown-unknown/release/host_web.wasm /tmp/463-sol2-worklet.4jaSqj/miso-engine-v1-audio-worklet.simd128.wasm
cp hosts/host-web/web/miso-engine-v1-audio-worklet.js /tmp/463-sol2-worklet.4jaSqj/
cp hosts/host-web/web/miso-engine-v1-audio-worklet-host.js /tmp/463-sol2-worklet.4jaSqj/
cp hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts /tmp/463-sol2-worklet.4jaSqj/
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/463-sol2-parameter-metadata-target cargo run --locked --release -q -p parameter-metadata -- --write /tmp/463-sol2-worklet.4jaSqj
sha256sum /tmp/463-sol2-worklet.4jaSqj/miso-engine-v1-audio-worklet.simd128.wasm
```

The unified tool call yielded before returning a captured numeric exit, so no numeric staging status
is claimed retroactively. All six expected staged files were observed afterward. The following
separate tool call then hashed the exact module path and immediately invoked the oracle:

```text
sha256sum /tmp/463-sol2-worklet.4jaSqj/miso-engine-v1-audio-worklet.simd128.wasm
PATH=/home/bl/.cargo/bin:$PATH node hosts/host-web/tests/browser-v1/direct-oracle.mjs /tmp/463-sol2-worklet.4jaSqj hosts/host-web/tests/browser-v1/expected.json 2>&1 | tee /tmp/463-sol2-direct-oracle.log
```

That call returned numeric status 0. Its tool output identified the oracle input as
`6dcf5e3a6f5a56feffce22133eb4c997d594db4bbc3624fa9cd0fe79111e10b7` and the oracle log says
`web AudioWorklet independent raw-Wasm oracle passed`. A fresh read-only hash of the still-retained
same path produced the same digest in `/tmp/463-sol2-candidate-oracle-input-hash.log`; this later
hash is corroboration, not attribution to the earlier invocation.

## Newly derived native body inventory

The six files under `native-bodies/` were extracted after qualification from the retained candidate
native disassembly. This is a new deterministic derivation, not an original compiler output claim.
The extraction command selected each named section from its symbol header through the next section
header and returned status 0. SHA-256 inventory:

```text
for n in issue463_sum2_simd8 issue463_sum_into_simd8 issue463_mix2x2_simd8 issue463_sum2_scalar issue463_sum_into_scalar issue463_mix2x2_scalar; do awk -v n="$n" '$0 ~ "<"n">:" {p=1} p && /^Disassembly of section/ && $0 !~ n {exit} p{print}' artifacts/issue463-lane-bounds-candidate/native-x86_64-v3.disassembly.txt > "artifacts/issue463-lane-bounds-candidate/native-bodies/$n.body"; done
sha256sum artifacts/issue463-lane-bounds-candidate/native-bodies/*.body
```

```text
c0bed25af824b473035aac2f24e41331aec866468f1b757d3bd781daaf113f44  issue463_mix2x2_scalar.body
32e5583cd34e3a8fe9ab1487e72d9a2cd6f3ceb394304d9eacd8faa7d759f9b5  issue463_mix2x2_simd8.body
90af05ccba03f59a09bf24cd7b0d5d6e61c729c6cf5c3bcab87ec548cb8dcb19  issue463_sum2_scalar.body
f62cd1a002786c12fd18353d213d3e4df763e8eb344f56824dc925acc16dae84  issue463_sum2_simd8.body
84975fe1fd7f0737b69696fe13ce7021818de0bd13a2497d1ddc5b1cd119558b  issue463_sum_into_scalar.body
4d92055cb8b67bb3de9504402e7fe34e84a25a0768032e23b022422a2b7ef351  issue463_sum_into_simd8.body
```
