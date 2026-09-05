# RT-1 direct bank scatter qualification

The source contract passed Astra review at eb3c9500f277ba02fc5be04ced1351ac8dc665cc. The immutable artifact source is e46bc0d1a7917de8c65204cdee931877aea671d8; the benchmark qualification candidate is d99a18f7867bf95c852bb721c141dac122e18886. Later commits only preserve evidence.

Full workspace: baseline1552/0/24, candidate1559/0/24. Direct/staged/scalar bit identity, release-safe borrowing, rejection sentinels and isolated live allocation/free-zero tests pass. The successful full unfolded bank path returns before staging writes/copies; staging storage and folded/partial fallbacks remain. No plan-memory reduction is claimed.

The reproducible shipped worklet SHA-256 is 60c23ee23e7f16c1f71c503baa07a462a8ce94c5287bec4580060e27a4651503. Static/resource parity, 26 resource mutations, hermetic worklet tests and all three browser recording/matrix checks pass. Browser results retain the artifact source above. The scalar non-LTO supplemental report records successful decoding and no atomic opcode in all three required archive objects. It does not rehabilitate the original fat-LTO inspection's four bad-magic errors; #404 owns that checker defect.

## Refused descriptive measurement

Preflight passed with zero workloads. Exactly one runner invocation was made; it refused on precondition_loadavg_above_ceiling before warmup. The disposition records zero workload launches, zero warmups, zero measured rounds and no raw/accepted records. Preserve both the runner's original stderr log and the outer invocation stderr, which records load1.06 against the ceiling. There are no timing results or speedup claims. #415 owns the outstanding descriptive measurement; this directory is permanently consumed and must not be overwritten or retried.

The preflight release binary used the repository's default release profile and SHA-256 2a18616204ace612b99f556b11d19ff074c5ab2f87883c91345c073c2cc44fbc. The runner explicitly used opt-level3, LTO=false and codegen-units16, producing SHA-256 748fe3460c6c52d457261f2cd41d4319fd6c888c0f525031cead8354fdb76eaf. These are deliberately distinct profiles/binaries, not a reproducibility mismatch. Workload, fixtures, floors and production validators were unchanged.
