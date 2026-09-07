# Issue #570 AudioWorklet qualification evidence

The frozen source is `3871e137b519815540c1b3abcd6cfcec7932efa7`; production/test source froze at
`6913563c661d9faaffd6c9b25d57fe04a8d37179`. Astra LOW authorized a detached qualification
worktree with only a provisional artifact-pin overlay. The ordinary builder produced the exact
six-file artifact set under `/tmp/issue570-qualified-artifact-3871e137`; its Wasm member is
SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`, matching the
single identity probe.

The retained builder, static/object/ABI checks, browser resource oracle with 26 red mutations,
SDK install/generated/deletion/type/headless/package gates, qualification install, three session
identities, and generated matrix check all passed. The first all-browser qualification invocation
then stopped in lineage preflight with numeric status 1 because `results.json` still named the old
artifact hash. No browser launched, the hermetic gate was not run, and no result is credited for
that invocation.

This is an authentic setup failure: the provisional pin alone cannot satisfy the runner's checked
lineage. The two generated lineage files remained unchanged. Root preserved the failure before
requesting a bounded ruling on provisional `results.json` and matrix overlays. Numeric resource
and PCM expectations remain frozen. No repository pin or generated consumer is authorized by this
checkpoint.

Raw command metadata and output are retained losslessly under `prepin/`. `sha256sums.txt` covers
every retained payload except itself.
