# Graph compiler: remove the per-compile `CompiledSession` clone (#99 F5 successor)

## Approved current scope — 2026-09-11

Remove the full CompiledSession clone in GraphCompiler::compile_with_builtin_tails by confining fallible preparation to a helper borrowing effects and returning owned build results. Ownership is consumed only after the final fallible checks; every failure returns the original caller-owned effects and builtins. Current canonical format is JSON and the function is roughly620lines, not the historical TOML/500line description.

Own crates/graph-compiler/src/compile.rs and directly relevant graph-compiler tests only. Preserve diagnostic ordering, all early/late failures, schedule, banking, PDC, arithmetic, graph identity and canonical bytes. No owned-string front-half redesign or other audit expansion. If a smaller lexical borrowing solution safely removes the clone without the larger helper extraction, prefer it and record why.

Gates: representative success and early/late failure ownership tests using distinguishable prepared state; exact diagnostic order and graph/audio parity; direct proof the full-session clone was removed (no replacing it with another equivalent copy). Focused compiler tests and strict Clippy. Root owns any changed shipped artifact qualification, with exact PR/main CI before closure. No speculative speedup or broad allocation campaign.

Astra XHIGH scoping approved; user authorized execution. Astra LOW implements, Astra XHIGH independently verifies. Five attempts maximum; each coherent pass gets one adversarial verdict. Root checkpoints exact paths and pushes promptly when focused checks pass, before more implementation. At most two active issues: #220 and #162; #221 queued behind delivered #220. Isolated worktrees; no overlapping edits. Root/lane B owns artifact qualification and pinning. Preserve histories and failed evidence; never weaken gates or commit compiler-IR captures.

Record actual argv/environment/source/exits/logs externally; pause green for root checkpoint. Stop on first unexpected failure and report for bounded correction. Required exact-head PR and main qualification plus upstream GitHub synchronization precede closure. Remove clean delivered worktrees after preserving evidence/history. Historical model names below are superseded by user routing.

## Historical issue body

`GraphCompiler::compile_with_builtin_tails` clones the whole `CompiledSession` — canonical TOML included — once per compile, purely to satisfy the borrow checker. #99 is closed, so the debt it was recorded against no longer tracks it. This issue carries it forward.

## Where

`crates/miso-engine-graph-compiler/src/compile.rs`, in `compile_with_builtin_tails`:

```rust
let session = effects.session.clone();
let model = session.normalized_model();
```

## Why it is there

`model` borrows the session, and the transactional failure path must hand `effects` back **by value** from inside the loops that read `model`. Every early `return Err(failure(effects, diagnostics))` therefore needs the borrow to be over, which the clone buys.

## Why it was not removed under #99

Removing it means restructuring a ~500-line function so every early `failure(effects, ..)` happens after the borrow ends, and the failure path is a frozen API contract. It was deliberately left as a bounded successor rather than rushed inside the #99 window. The comment at the site says so explicitly (`NOT YET REMOVED (#99 F5, deliberately)`).

## Successor shape (as specified in the #99 F5 comment)

Extract a

```text
build(&effects) -> Result<Built, Vec<GraphDiagnostic>>
```

that returns **owned** outputs, with `failure(effects, ..)` called only on its `Err`. That confines the borrow of `effects` to `build`, so the clone is no longer needed to end it.

## Scope note

The dominant F5 cost — the canonical dump, its SHA-256 and the Graphviz string on every compile — is already gone; `GraphCompiler::evidence` produces those on demand. What remains is this one `CompiledSession` clone per compile.

## Constraints on the fix

- Compile-path surgery: the transactional failure contract (every caller-owned input handed back by value on any failure) must be preserved exactly.
- Bit-identity: class-A digests and the wasm gate legs must be unchanged by the restructure.

## Provenance

Filed from the `allow-hygiene` window-cleanup pass (job 4, item 4), which was scoped to file-or-fix this debt and explicitly not to perform the restructure.

## Astra LOW attempt 1 focused checkpoint

The smallest lexical borrowing solution compiled: borrow effects.session directly
and pass that reference to the index helper. No large extraction or equivalent
copy is needed. The focused test preserves original session allocation, distinct
processor ownership and builtins allocations across early zero-cap and late
node-cap failures, then retries successfully with identical canonical graph
and nonzero four-block PCM. Formatting and this focused test passed. Actual
argv/env/source/exits/logs: `/tmp/engine-162-attempt1`. Incidental Cargo lock
ordering was preserved externally and restored; no dependency change.
Root checkpoints now; full compiler tests, strict Clippy and independent review
remain. No allocation-count or performance claim beyond removing this clone.

## Attempt 1 review and bounded attempt 2

Independent debug and release-unwind suites each passed86 tests. Strict Clippy
then rejected the new test closure's large Result error type; the frozen
ownership-returning API itself is unchanged. Receipt:
`/tmp/issue162-xhigh-qemy_v30/clippy.json`. Root authorized only replacing that
forwarding closure with direct compiler calls. Astra LOW did so; formatting,
locked strict all-target/all-feature Clippy and focused ownership test passed.
Evidence: `/tmp/engine-162-attempt2/manifest.json`. No production/API/dependency
change beyond original borrowing patch. Independent follow-up verdict pending;
prepared external final-plan-cap ownership control remains unrun.

## Independent Astra XHIGH attempt 2 source PASS

Reviewed d4906302381953efe8ce85dd3e0a9658c0b6f193: production borrowing patch
unchanged, direct test calls fix Clippy without lint suppression or API changes.
Retained86 debug and86 release-unwind test passes. Independent final-plan-cap
control (maximum_plan_bytes=1) reaches the final check after bank preparation,
returns original session/effect/builtin ownership with exact diagnostic, then
retries with equal canonical graph and four nonzero PCM blocks. Separate copy
and target used. Evidence: `/tmp/issue162-xhigh-qemy_v30/attempt2-review.md` and
final-cap-attempt2 receipts. Root defers artifact qualification for a coherent
joint delivery with #221 after #220's required delivery, while source remains
frozen. Required artifact/PR/main gates and synchronized closure remain pending.
