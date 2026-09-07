# Issue #578 frozen-source AudioWorklet identity probe

Astra LOW accepted attempt-2 source at `2aa9cb18badbac6552ed9998864f9f1956b73616`. Root recorded that verdict without product changes at `05bdba6d` and then ran the one authorized identity probe with only `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1`.

The builder returned candidate SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357` with status 0. This exactly equals the delivered #570/#575 repository pin. Repin mode exits after printing the identity, so `/tmp/issue578-repin-probe-empty-05bdba6d` remained empty. The repository pin, source, tests, generated consumers and browser qualification records did not change.

This was the sole identity probe for #578. `command.txt` records the exact invocation; pre/post context records the pushed head, path status, source identities, pin and empty-output count. Raw stdout and stderr are compressed losslessly with deterministic gzip metadata. `sha256sums.txt` covers every retained payload except itself and verifies from the repository root.

This identity result does not by itself authorize a pin change, browser reuse, PR or delivery. Root still must build the ordinary six-file artifact, compare every byte and affected resource/PCM consumer to the delivered artifact, and obtain Astra LOW artifact review.
