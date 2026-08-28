# FLAC delivery v1 fixtures

This corpus derives every sample from the frozen #241 rows in
`fixtures/stem-identity/v1/VECTORS.tsv`; it does not define a second PCM oracle. Each of the four
`{16,24} x {mono,stereo}` vectors is encoded twice with the pinned publisher at block settings 32
and 4096. `FLAC_VECTORS.tsv` pins transport bytes for reproducible fixture provenance while its
`identity` column remains the shared canonical-PCM identity.

`mini-catalog/` models the old container-byte hashes, all five classes of embedded identity, and
the three exact one-way migration outputs consumed by #246. The app paths are inventory strings
only; this fixture never edits or executes the app repository.

Regenerate or check the complete derived tree with:

```sh
python3 fixtures/flac-delivery/v1/generate.py --write
python3 fixtures/flac-delivery/v1/generate.py --check
```
