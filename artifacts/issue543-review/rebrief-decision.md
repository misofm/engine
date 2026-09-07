# Issue #543 rebrief — Sol HIGH decision

## Verdict

**APPROVED FOR NUMBERING.** Amend #543 to the independently closable Rust-package slice and create the separately numbered JS/TS successor from `rebrief-successor.md`. Do not start source implementation until the local numbered spec and matching GitHub issue exist and #546 is present in the successor baseline.

## Rationale

Sol XHIGH attempt 1 accepted #543's Rust implementation, dependency direction, unchanged pins/APIs, evidence integrity, and full Rust workspace gates. Its FAIL is valid because the issue promised workspace-wide semantic completion while its final census considered only Rust. The three JS/TS encoders have identical unprefixed lowercase whole-byte semantics and cannot be relabeled as exclusions.

The amended #543 scope preserves its complete, useful Rust outcome without claiming all of CP-20. The successor owns every discovered non-Rust residual. It uses one authority per actual shipped module closure: one emitted with the typed SDK package and one served by host-web for both qualification and stem-store. Direct Rust reuse would require a new Wasm/C ABI and runtime memory bridge; direct SDK/host source sharing would break one of the package closures. The recorded two-authority boundary is therefore the smallest correct implementation rather than a scope loss.

## Closure rule

#543 can pass and close only as the Rust slice after its amended claim is reviewed. Audit #349 CP-20 stays `PARTIAL` through that closure. CP-20 becomes delivered only after the numbered successor passes Sol XHIGH review, required CI, merge, issue synchronization, and tracker update.

No repository or source modification was authorized or performed by this rebrief.
