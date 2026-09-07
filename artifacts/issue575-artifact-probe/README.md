# Issue #575 frozen-source AudioWorklet identity probe

Astra LOW accepted source at `fa84469c43b88fa7664806c332aa363d9e6a014c`. Root then recorded that verdict without product changes at `ec992089d1fa1208ceb49ff6ca65e6e2c05bf2ed` and ran the single authorized ordinary identity-discovery invocation with only `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1`.

The builder compiled the frozen source and returned candidate SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357` with status 0. That is exactly the delivered #570 pin. Repin mode exits after printing the identity, so `/tmp/issue575-repin-probe-empty-ec992089` remained empty. The repository pin, source, tests, spec, generated consumers and browser qualification records were unchanged.

This was the only identity probe. Raw stdout/stderr are losslessly compressed. `sha256sums.txt` covers every retained payload except itself and is verified from the repository root.
