## Attempt 1 diagnostic checkpoint (Luna xhigh)

The frozen four debug comparisons each ran once and passed before any child argument change. Resource preparation: direct and wrapped each 27 allocations, 0 frees/reallocations, 102496 bytes; same-thread counts each 27/0 under inherited concurrency 1 and 2. Controller preparation: direct 31 allocations, 0 frees/reallocations, 120280 bytes; wrapped 33 allocations, 2 frees, 0 reallocations, 120289 bytes; corresponding same-thread counts agree under both settings. Original strict assertions remain.

The synchronized foreign-thread control recorded process-global 1 allocation, 2 frees and 4096 bytes while measured-thread counts remained 0/0. The extra foreign deallocation is retained honestly. This proves global-counter contamination sensitivity; passing comparisons do not reproduce or attribute the historical four extra allocations. No child scheduling change has been made. A preliminary invocation used a nonexistent worktree path and launched no test; all four actual comparisons are retained exactly once.

Source SHA-256: eafda5a261d900b89f463c91bbf12a050195c83f891755fd57aad0d4064add90. Full commands, inherited environments, dirty-source identity, numeric status and stdout/stderr are in artifacts/issue536-luna-attempt1. Implementation is paused for the required bounded Astra medium diagnostic decision.
