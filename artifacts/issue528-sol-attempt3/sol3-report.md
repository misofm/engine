# Issue #528 final Sol attempt 3 report

Final clean checkpoint: `e33a4cae977536870e9e83bafbde1269c4c2880b`.

- Endpoint source: SHA-256 `d08e5d958dad00a76cb8357c277a0a50e3f3da2ae57bfca59999c63be3a69f92`, Git blob `38325e0b6f7d1334ae83951b861b4b20f3f412e7`.
- Endpoint tests: SHA-256 `b33f0773da835856447f64e875a47c67a97b10cbf2f6e7e4e98d88d7e97c1354`, Git blob `304081e4502337fb20e42b65bf14885ada0ff24c`.
- Worktree was clean for every final proportional command.

The two Astra attempt-2 findings are corrected without API or scope expansion. The post-mark
`pending(ticket)` lookup now maps every error to the existing sticky Delivery/Pending fault with
the actual sample, processed-frame count and equal already-marked native/service prefixes. Gate 6
now allocates and frees a nonzero `black_box` probe inside `audit::in_render_scope`, asserts positive
render-audit allocation and deallocation counters, and resets before all zero-audit product paths.

All raw captures are `/tmp/issue528-sol3/<label>.{command.json,stdout,stderr,status}`:

- `focused-debug`: status 0; six endpoint tests passed plus isolated Gate 6 child passed.
- `focused-release`: status 0; six endpoint tests passed plus isolated Gate 6 child passed.
- `host-feature-tests`: status 0; 84 passed including the isolated child, 2 ignored.
- `clippy`: status 0; locked feature-enabled all-targets Clippy with `-D warnings`.
- `fmt` and `source-diff`: status 0.
- `workspace-policy`, `host-core-policy`, `realtime-policy`: status 0.
- `wasm-scalar`: status 0; locked `wasm32-unknown-unknown` host-core build with
  `control-provider`, `RUSTFLAGS=-Ctarget-feature=-simd128`, target directory
  `/tmp/issue528-sol3-target-wasm-scalar`.
- `wasm-simd`: status 0; the same feature-enabled target build with
  `RUSTFLAGS=-Ctarget-feature=+simd128`, target directory
  `/tmp/issue528-sol3-target-wasm-simd`.

Unchanged evidence is reused with its original identity rather than relabeled. Sol2
`/tmp/issue528-sol2/final-host-default-tests.*` records 72 passed and 2 ignored at `eb424753`; the
endpoint remains excluded without `control-provider`. Sol2 `final-protocol-delivery-unit.*` records
15 passed and `final-protocol-delivery-ownership.*` records 2 passed at `eb424753`; attempt 3 does
not change protocol or its ownership service. No broad workspace/browser/ABI suite or benchmark was
run before final source acceptance.
