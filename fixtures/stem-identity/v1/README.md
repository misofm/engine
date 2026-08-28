# Canonical-PCM stem identity vectors v1

`VECTORS.tsv` is the human-readable authority for four launch vectors: bit depths 16 and 24,
each as mono and stereo, with zero, positive full scale, negative full scale, `+1`, and `-1`
samples. `samples_by_frame` separates channels with commas and frames with pipes. `canonical_hex`
is therefore hand-derivable by reading each signed integer at its declared width in little-endian
frame-major order.

Each row names a committed headerless `.pcm` preimage. The stereo row at each depth also names a
committed `.wav` wrapper with the same samples. The WAVE files use 48 kHz only to make their
headers well-formed; sample rate and every other container byte are outside stem identity.

`generate.py` is an independent Python-standard-library generator. The Rust reference oracle is
tested against the frozen bytes and pinned identities, not used to generate its own expected
answers.

Run:

```sh
python3 fixtures/stem-identity/v1/generate.py --check
cargo test --locked -p miso-engine-stem-hasher
```
