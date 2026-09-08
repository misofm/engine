# Issue 627 applicability probe

This directory preserves the single ordinary no-bypass AudioWorklet build
authorized by Astra LOW after the scope PASS. The build ran from detached,
clean source `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251` and was not retried.

The unchanged builder completed Cargo's release build, then exited 1 at its
artifact pin comparison. It reported delivered pin
`ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3` and
candidate Wasm SHA-256
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`.
The external output directory remained empty, as required for the builder's
fail-before-publish path. The source worktree was clean before and after.

`command.txt` records the exact invocation and capture destinations.
`build.stdout`, `build.stderr`, and `status.txt` are the complete raw streams
and exit status. The two status files and two output files are intentionally
empty and prove clean source plus zero published output. `source-identity.txt`,
the tool versions, and `input-sha256.txt` bind the probe to its source,
toolchain, configuration, builder, delivered artifact records, and frozen
limiter implementation. `sha256sums.txt` covers every other retained file.

This record establishes a candidate mismatch only. It does not qualify the
candidate or authorize a scratch build, browser run, pin edit, or promotion.
Astra LOW must independently review it first.
