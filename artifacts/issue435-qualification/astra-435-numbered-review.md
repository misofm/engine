# Astra #435 numbered scope approval

**PASS for planning checkpoint `af187c3ef322fff7378c6efdfd996c176ae19ceb`.** Remote #435 is OPEN with the matching title “Remove inactive lease muting from sequential render reads”. The numbered text retains the approved RT-14 scope; changes are the numbered title/queue and explicit retirement ruling.

The ruling authorizes the exact public Rust runtime API/behavior retirement openly, without implying compatibility or changing wire/C ABI identity. It expressly preserves builder wave arguments and ownership/order validation, write-access policy and release-mode read-ID bounds checks. The distinction between dead returned-lease wave/mute state and still-operative builder proofs is intact. Historical #420 mute-specific behavior is intentionally superseded only at the successor source; original evidence is not rewritten.

No scope expansion or material correction found. Freeze the post-#420 merged base and repeat complete production-call discovery before assignment; any live caller requires rebrief. Do not overlap #429 or another launch-critical feature. No implementation, benchmark or new architecture is authorized merely by this planning approval. Existing source/test/qualification and exact-head PR/required-CI gates remain.

Read-only local diff/spec and remote identity inspection. No tests, Cargo, timing or repository/GitHub mutation.
