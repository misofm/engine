# Session: move the consumed compiled canonical document without copying

## Smallest authorized slice

The full crate housekeeping review found two session-validator callers that immediately discard CompiledSession after cloning its complete canonical_json into an owned result. Its only existing accessor borrows the String. Add one documented consuming accessor in crates/session/src/compile.rs that moves this already-validated canonical String out of the consumed control-plane artifact. The existing borrowed accessor and all compilation, model, wire and preparation contracts remain unchanged. This producer-only issue is paired with the session-validator crate issue #1151, which owns adopting the accessor after its complete review.

## Source evidence and boundaries

CompiledSession privately owns canonical_json: String and has no Drop implementation. validate_session_document finishes all five stages, clones compiled.canonical_json and then discards compiled; fold_mono_document does the same after the final transactional compilation before checking output length. Neither caller subsequently needs any compiled field. These are complete document copies, not render-plane PCM or persistent DSP state. No public field, serialization model, numeric rule, ABI symbol, schema, algorithm, queue admission, resource estimate or realtime behavior changes. Consumption retires the remaining model fields on the existing control thread.

## Objective gates and test value

- Implement only the consuming canonical accessor and exercise it through the existing canonical compile test after that test has checked the compiled fields. Preserve the independent expected canonical document; do not add a duplicate test or a digest/resource-byte pin.
- The rewritten existing test must catch a wrong or empty owned snapshot from the new accessor that the borrowed getter alone cannot exercise. The session-validator existing canonical-output/fixed-point/no-op/transactional/native-render tests qualify its callers in #1151.
- Focused locked session tests, fmt/diff and strict package lint. Reuse the full original #1118 crate audit and unchanged algorithm/RT evidence candidly. Supported simd128/iOS/Android compile checks are compile-only. No benchmark, corpus/harness expansion or additional API.
- First coherent focused-green exact-path checkpoint pauses for root commit/push. One root adversarial verdict per coherent attempt, maximum two attempts; do not broaden this issue if a public ownership or destructor constraint appears.

## Sol approval and delivery

Root Sol approves this bounded producer ownership slice on 2026-10-01 under the user's unnecessary-copy cleanup request. Worker A (GPT-6.1 Sol xhigh) may implement after #1152 is remotely closed and the issue boundary is synchronized; worker B reviews #1151 independently. No third agent is requested. The local numbered spec and matching GitHub issue are confirmed before implementation. Root owns checkpoints, PASS, evidence synchronization/remote closure and required qualification/main delivery.

## Attempt evidence

Pending the final housekeeping pair. Source evidence only; no implementation, allocation count or timing win is claimed.
