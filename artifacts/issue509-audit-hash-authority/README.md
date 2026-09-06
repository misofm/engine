# Issue 509 audit SHA256 text aliases

Source5368843f replaces five local one-shot SHA256 encoders with imports of the existing bench-support authority, preserving all call arguments and required raw/incremental hashing. Each actual module alias has independent published empty/abc literal tests. Only the five authorized audit Rust modules change.

Final direct subprocess records execute the frozen test filters in both debug/release with populations5,6,3,1,1, each exit0. The two exact existing filters execute one test each, not an extra alias test; the report's imprecise generic wording does not change those raw counts. Strict affected Clippy, fmt/diff, bench policy and realtime-audit-leak policy each exit0. Metadata records dirty-source file blobs/SHA256 beside the base HEAD; root independently verified final file hashes.

The initial ten command failures are preserved: an explicitly empty CARGO_TARGET_DIR was passed by the first evidence wrapper. Corrected captures omit unset optional settings; no source failure is relabeled. All environment records are whitelisted. No full audit CLI, fixture/listening generation, repin, benchmark/preflight/capture or timing was invoked. Fixture/manifests, validators, source seals and runtime/worklet inputs remain unchanged. Consolidated Astra review and actual PR/required CI remain pending; CP20 stays partial outside this package.
