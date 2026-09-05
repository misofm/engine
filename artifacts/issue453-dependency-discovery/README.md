# Benchmark and dependency scan qualification

#453 production scanner changes plus #462 completion and portability proof. Final accepted source review: Astra #462 attempt2 at8d4520bd. Immutable full-workspace candidate2c8e0c48646192ae1484e56356ff2a26279a403e; command, terminal exit0 and summary are retained. Rust/runtime/configuration inputs match delivered29a8c88b; full workspace including doctests retains275 result blocks,1576passed,0failed,24ignored.

`453-*` focused records belong to the stopped final parent attempt; its four absent rows are identified by the historical FAIL review. `462-*` initial focused records cover those four added rows. Initial #462 PASS was withdrawn for hardcoded Cargo delegates; the portability ruling and final Sol2/Astra review supersede it. `462-sol2-*` retains the ordinary and spaced-path suite proof, forwarding wrapper/trace and statuses. `462-verify-relocation.py` independently checks all five earlier-package delegation hits with exact flags against the retained trace; it is evidence, not a new production gate. Existing two actual production mutants return97 and the restored fixture0.

No benchmark, timing, runtime, crate boundary, parser grammar, owner exception, workflow or artifact implementation changes are claimed. #403/#306/#349 retain other obligations.
