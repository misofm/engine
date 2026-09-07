# Issue #570 frozen-source AudioWorklet identity probe

Astra LOW accepted source at `e99818a289019839cd34e9d1c2b2664286f8c4f6` and authorized exactly one
identity-discovery invocation. Root ran the ordinary repository builder with only
`MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1`; the builder compiled the frozen source successfully and
returned candidate SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`
with numeric status 0.

The repository pin remained
`63dd5f8b0febf193847b697fa8e4d92e791b7e4775f3b4b6b61252783f153e9f`. Repin mode exits after
printing the observed identity, so the dedicated output directory remained empty. No artifact,
pin, generated consumer, source, test, spec, or policy file changed. This record discovers an
actual identity difference; it does not qualify or authorize the candidate.

The raw stdout and stderr are losslessly compressed. `sha256sums.txt` covers every retained
payload except itself.
