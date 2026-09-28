# Rust test inventory by crate

Base `a9414c0c`.

How each column was counted:
- **tests:** `#[test]` functions parsed from source.
- **test LOC:** lines in `tests/` plus `#[cfg(test)]` code. **Product LOC** is the rest of the crate.
  Crates with no tests (`host-native`, `host-mobile`, the Wasm guest tools) are left out, so the
  product total here is lower than the audit's 196,732.
- **local s:** the sum of per-test `exec_time` in the debug CI shards, or in release for crates
  that only run in release. Measured once, under the shared timing lock, on a 32-core host.
- **A, D, R, P, S, B, Y+X, T:** the claim classes defined in `../TEST-VALUE-AUDIT.md` §2.
- **pin through out of scope:** counts of verified flags.
- **named in a MUTATIONS.md:** a heuristic. The test, or its test file, is named in some
  `MUTATIONS.md`.

| crate | tests | ignored | test LOC | product LOC | local s (debug) | CI job | A | D | R | P | S | B | Y+X | T | pin | scrape | tautology | no-assert | out of scope | family members | named in a MUTATIONS.md |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| host-core | 229 | 2 | 19070 | 12709 | 15.8 | debug-a | 37 | 6 | 20 | 27 | 139 | 0 | 0 | 0 | 3 | 0 | 4 | 0 | 0 | 86 | 55 |
| host-web | 211 | 2 | 18025 | 16042 | 9.6 | debug-a | 6 | 3 | 11 | 73 | 113 | 4 | 0 | 1 | 6 | 0 | 2 | 0 | 1 | 94 | 40 |
| protocol | 176 | 0 | 12200 | 17775 | 13.3 | debug-a | 1 | 0 | 21 | 105 | 48 | 1 | 0 | 0 | 3 | 0 | 2 | 1 | 0 | 94 | 4 |
| parametric-eq | 129 | 3 | 13432 | 5155 | 118.9 | debug-b | 46 | 20 | 3 | 31 | 17 | 8 | 0 | 4 | 9 | 0 | 1 | 3 | 1 | 29 | 94 |
| builtins | 120 | 1 | 10013 | 6878 | 59.4 | debug-b | 31 | 27 | 3 | 19 | 34 | 5 | 0 | 1 | 8 | 0 | 0 | 1 | 2 | 30 | 86 |
| graph | 117 | 0 | 17113 | 11885 | 39.4 | debug-a | 22 | 13 | 3 | 0 | 74 | 3 | 2 | 0 | 10 | 2 | 2 | 0 | 0 | 18 | 42 |
| graph-compiler | 113 | 0 | 17659 | 3652 | 266.6 | debug-a | 16 | 5 | 2 | 4 | 81 | 4 | 0 | 1 | 16 | 0 | 0 | 0 | 0 | 43 | 48 |
| compressor | 100 | 1 | 9313 | 3827 | 42.7 | debug-b | 31 | 19 | 2 | 10 | 26 | 8 | 0 | 4 | 10 | 0 | 3 | 1 | 3 | 35 | 93 |
| effect-runtime | 91 | 1 | 2906 | 2014 | 5.2 | debug-b | 17 | 36 | 0 | 17 | 18 | 2 | 0 | 1 | 2 | 0 | 1 | 1 | 0 | 20 | 78 |
| builtins-compiler | 79 | 0 | 2694 | 12361 | 49.3 | debug-a | 8 | 0 | 9 | 9 | 50 | 2 | 0 | 1 | 4 | 0 | 0 | 0 | 0 | 37 | 30 |
| effect-package | 73 | 1 | 8049 | 5354 | 0.2 | debug-a | 0 | 0 | 4 | 62 | 5 | 0 | 1 | 1 | 1 | 1 | 3 | 0 | 3 | 44 | 25 |
| source | 73 | 1 | 6061 | 4729 | 1.1 | debug-a | 8 | 2 | 1 | 1 | 51 | 2 | 7 | 1 | 1 | 9 | 5 | 1 | 0 | 28 | 2 |
| lane | 71 | 2 | 7300 | 4876 | 38.1 | debug-b/release | 30 | 23 | 1 | 0 | 13 | 1 | 1 | 2 | 0 | 1 | 5 | 1 | 2 | 5 | 54 |
| effect-compiler | 66 | 1 | 7738 | 3452 | 9.0 | debug-a | 5 | 0 | 2 | 4 | 49 | 3 | 3 | 0 | 5 | 2 | 1 | 0 | 0 | 20 | 9 |
| bench | 66 | 1 | 2194 | 11994 | 2.5 | audit-native (release) | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 61 | 10 | 2 | 3 | 0 | 11 | 24 | 0 |
| effect-contract | 64 | 0 | 2589 | 5027 | 0.3 | debug-a | 0 | 0 | 2 | 0 | 60 | 2 | 0 | 0 | 2 | 0 | 1 | 1 | 0 | 15 | 20 |
| console-workload | 64 | 2 | 6205 | 2809 | 16.5 | audit-native (release) | 33 | 0 | 0 | 0 | 19 | 1 | 0 | 11 | 12 | 0 | 0 | 0 | 0 | 20 | 41 |
| session | 63 | 3 | 3948 | 4590 | 39.1 | debug-a | 2 | 0 | 1 | 11 | 45 | 2 | 1 | 1 | 10 | 3 | 2 | 0 | 2 | 16 | 1 |
| rack | 56 | 0 | 5287 | 3212 | 0.0 | debug-a | 17 | 0 | 1 | 9 | 27 | 2 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 24 | 30 |
| true-peak-limiter | 54 | 1 | 7334 | 4681 | 39.3 | debug-b | 25 | 8 | 2 | 3 | 10 | 4 | 0 | 2 | 6 | 0 | 1 | 1 | 1 | 20 | 45 |
| math | 50 | 14 | 2827 | 5796 | 33.3 | debug-b/release | 5 | 40 | 0 | 0 | 0 | 1 | 3 | 1 | 7 | 3 | 1 | 0 | 0 | 12 | 0 |
| multiband-compressor | 50 | 2 | 4885 | 2088 | 22.8 | debug-b | 16 | 16 | 3 | 5 | 7 | 1 | 0 | 2 | 4 | 0 | 0 | 2 | 2 | 8 | 33 |
| audit | 49 | 0 | 2199 | 14120 | 52.0 | audit-native (release) | 2 | 0 | 2 | 0 | 2 | 1 | 3 | 39 | 5 | 4 | 3 | 0 | 0 | 24 | 2 |
| engine | 47 | 0 | 1824 | 3465 | 0.2 | debug-a | 2 | 0 | 16 | 0 | 26 | 3 | 0 | 0 | 3 | 0 | 2 | 0 | 4 | 20 | 9 |
| bench-support | 45 | 0 | 663 | 869 | 0.0 | debug-a | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 45 | 0 | 0 | 2 | 0 | 0 | 33 | 0 |
| gate-expander | 41 | 0 | 2726 | 1563 | 2.1 | debug-b | 5 | 10 | 0 | 10 | 15 | 1 | 0 | 0 | 2 | 0 | 0 | 1 | 0 | 14 | 38 |
| capi | 36 | 0 | 6542 | 3270 | 5.5 | debug-a | 2 | 0 | 2 | 24 | 7 | 0 | 1 | 0 | 3 | 1 | 0 | 0 | 0 | 14 | 3 |
| conformance | 28 | 0 | 1057 | 3159 | 2.3 | debug-b | 0 | 4 | 0 | 18 | 5 | 1 | 0 | 0 | 4 | 0 | 0 | 0 | 2 | 5 | 12 |
| dsp-reference | 28 | 0 | 955 | 3119 | 0.4 | debug-b | 0 | 22 | 0 | 4 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 6 |
| soft-clip | 26 | 1 | 2146 | 1534 | 12.8 | debug-b | 5 | 7 | 3 | 4 | 5 | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 26 |
| transient-shaper | 24 | 1 | 2128 | 1229 | 1.4 | debug-b | 5 | 7 | 1 | 3 | 3 | 1 | 0 | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 24 |
| parameter-metadata | 23 | 0 | 2119 | 4285 | 0.2 | debug-a | 0 | 0 | 0 | 21 | 0 | 0 | 1 | 1 | 0 | 1 | 2 | 0 | 0 | 12 | 5 |
| native-pcm-runner | 20 | 0 | 1304 | 1388 | 1.0 | debug-a | 0 | 0 | 0 | 0 | 19 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 14 | 1 |
| session-validator | 18 | 0 | 977 | 856 | 0.4 | debug-a | 2 | 0 | 0 | 0 | 15 | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 9 |
| delay | 16 | 1 | 1085 | 1885 | 1.7 | debug-b | 1 | 6 | 0 | 5 | 2 | 1 | 0 | 1 | 2 | 0 | 0 | 1 | 0 | 0 | 16 |
| rack-compiler | 13 | 0 | 748 | 451 | 0.1 | debug-a | 1 | 0 | 0 | 1 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 8 |
| stem-hasher | 12 | 0 | 383 | 500 | 0.0 | debug-a | 0 | 0 | 0 | 7 | 4 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | 9 |
| wasm-gates | 10 | 0 | 615 | 920 | 6.3 | release | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 10 |
| target-smoke | 1 | 0 | 58 | 30 | 0.0 | debug-a | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| **total** | 2552 | 41 | 214371 | 193549 | 908.8 |  | 387 | 277 | 116 | 488 | 1004 | 67 | 24 | 189 | 154 | 30 | 52 | 15 | 36 | 869 | 1008 |
