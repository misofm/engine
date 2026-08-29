# FLAC delivery v1 fixtures

This corpus derives every sample from the frozen #241 rows in
`fixtures/stem-identity/v1/VECTORS.tsv`; it does not define a second sample-value oracle. The
generator repeats each complete frozen boundary pattern, then its leading frames, to make exactly
4096 canonical frames. It recomputes the expanded canonical-PCM identity and commits that exact
preimage under `pcm/`.

Each of the four `{16,24} x {mono,stereo}` expanded vectors is encoded twice with the pinned
publisher. A `-b32.flac` file contains 128 actual 32-sample FLAC frames; a `-b4096.flac` file
contains one actual 4096-sample FLAC frame. The native and browser gates derive frame lengths from
each decoded FLAC packet and require every length to agree with the file-name suffix. A name-only
32-to-4096 mutation proves that this self-check is red. `FLAC_VECTORS.tsv` pins transport bytes for
reproducible fixture provenance while both encodings of one expanded vector retain the same
canonical-PCM identity.

`mini-catalog/` models the old container-byte hashes, all five classes of embedded identity, and
the three exact one-way migration outputs consumed by #246. The app paths are inventory strings
only; this fixture never edits or executes the app repository.

Regenerate or check the complete derived tree with:

```sh
python3 fixtures/flac-delivery/v1/generate.py --write
python3 fixtures/flac-delivery/v1/generate.py --check
```
