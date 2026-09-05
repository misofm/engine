# Carry explicit between-render builtin control delivery into prepared bank ownership

This is the smallest policy-plumbing prerequisite for #430 under audit #349, based on delivered main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`. Planning only until Astra approves this numbered stateless scope.

## Product contract

Carry an immutable, unversioned builtin control-delivery policy from host preparation to concrete prepared bank owners. Existing general preparation APIs default to Concurrent. A dedicated preparation entry lets WebEngine declare BetweenRenderCalls because it privately retains the producer endpoints and exclusively owns both submit_commands and render_next. Only that proven production host opts in. This is an explicit caller contract, not runtime enforcement for arbitrary external hosts. Do not infer it from backend, queue emptiness or observed use.

## Scope and exclusions

Limit edits to policy definition and propagation in builtins-compiler, graph-compiler/host-core preparation and host-web private ReadyOwnership construction, their existing tests and this spec. Confirm exact propagation route in the final Astra brief before implementation. Do not fuse processors, add a queue/record field, move any drain, alter command acknowledgements, application samples, backpressure, callbacks or DSP arithmetic. No graph optimizer, rack renderer changes, new unsafe/dependency, timing or generic test framework.

## Acceptance

A typed preparation test must distinguish default/raw Concurrent from the dedicated BetweenRenderCalls path and show the policy reaches both concrete fader/matrix bank owners. Verify all current general call sites retain Concurrent; no raw producer-exporting API silently changes contract. Existing real browser tests must retain exact applied_at_sample, atomic all-or-nothing admission, saturation/backpressure and block application behavior. A deliberate wrong default or missing host opt-in must fail the same focused assertion. Preserve existing resource/realtime/lifecycle gates.

After focused source PASS, qualify proportional workspace and supported targets/current artifact if affected, then exact actual PR-head Astra review and required CI. No performance claim: no arithmetic changes.

## Workflow

Astra approves the numbered source-level brief; Luna attempt 1; Sol attempts 2/3 only after Astra FAIL; third FAIL requires explicit rebrief. Root owns checkpoint/push/issue synchronization and delivery. #430 implements pairing only after this prerequisite closes. Scalar and concurrent-native outcomes remain separate retained issues; #349 RT-4 remains open.

## Numbered accounting

This is #442. #442 owns the immutable delivery-policy prerequisite; #430 owns serialized live bank pairing; #443 retains scalar pairing; #444 retains concurrent-native admission and pairing. #431 owns separately briefed measurement. None alone closes audit RT-4/#349.
