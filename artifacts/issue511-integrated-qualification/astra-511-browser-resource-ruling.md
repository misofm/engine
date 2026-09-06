# Astra #511 browser resource integration ruling — approved, bounded

Reviewed clean frozen `e3cba8e853c86a8aa941cfa21470c6d9ae071a33` in `/home/bl/misofm/engine-slot-reservation`, the accepted reservation and earlier integration rulings, actual browser resource failure records, the separately captured successful direct oracle, the frozen browser session/expected document, and the relevant planner/layout/gate source. The ordinary consumer pipeline is terminal: builder 0, static checks 0, resource gate 1. This is an observed downstream numerical mirror discrepancy, not an inferred permission to repin other outputs or another production implementation attempt.

## Actual evidence and PCM ordering

`/tmp/511-qualified-resource.{command.json,log,status}` identifies this source and invokes `python3 scripts/check-browser-expected-resources.py --artifacts /tmp/engine-511-qualified`. The log names exactly three stale graph rows, each +204, and reports all 26 red mutation controls successful.

Independently hashed the published `/tmp/engine-511-qualified/miso-engine-v1-audio-worklet.simd128.wasm`: 2,693,746 bytes, SHA256 `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`, exactly the separately approved pin. No additional digest is authorized.

The gate collects the direct oracle through `MISO_ENGINE_WEB_ORACLE_PRINT=1` before `check_pins` compares resource rows. Inspection confirms that `direct-oracle.mjs` checks identity PCM against its unchanged native digest pin, command-timeline PCM against its unchanged native pin, and observed/unobserved PCM equality plus the unchanged native observation digest before it prints. Those checks precede the resource mismatch even though the later Python digest/full-document comparison follows the resource check and was not reached by the failing gate. This evidence establishes equality to the existing native pins; it does not pretend a fresh native PCM render ran inside that print invocation.

The additional `/tmp/511-integration-browser-direct-oracle.{command.json,stdout,stderr,status}` is an explicit clean-head print-mode invocation, status 0. Independently compared its complete recursive JSON to `expected.json.directOracle`: the only differences are the three resource fields listed below. All PCM digests, statuses, memory values, command results and observation values are identical. This resolves the full-document uncertainty left by the resource comparator's early failure without changing any oracle or pin.

## Independent component derivation

`hosts/host-web/tests/browser-v1/session.json` contains one track and no effects in any rack. The actual builtin planner groups each of post-input, post-fader and post-matrix into ceil(1/4)=1 padded bank on the shipped simd128 backend. Thus N=0 effect banks+3 builtin banks=3; later builtin pairing cannot reduce this pre-admission membership count.

For wasm32, the boxed stage's data/vtable pair occupies F=8 bytes. `BankSlot` has that owner plus a boxed bool slice, B=16 bytes. The selected four-lane mask has W=4 bytes. The frozen #511 calculation therefore gives C=3*(8+3*16+3*4)=204 and L=max(3*8,3*16,4)=48. This is the conservative named reservation, not 204 newly retained physical bytes. It adds only to graph metadata, incremental and session-plus-plan totals. L=48 is below the already pinned largest named allocation of 16,384, so that field stays unchanged.

## Exact authorization and gates

Approve only these three string-value replacements under `directOracle.simd128.resources` in `hosts/host-web/tests/browser-v1/expected.json`, plus the corresponding issue/commit/evidence derivation:

| Field | Existing | Approved |
| --- | ---: | ---: |
| graphSessionPlusPlanBytes | 29294 | 29498 |
| graphIncrementalPlanBytes | 29294 | 29498 |
| graphMetadataBytes | 3455 | 3659 |

Preserve every other expected value and byte authority: PCM/native digests, full document shape, source/builtin/effect/bridge rows, largest allocation, canonical/session fixtures, memory, protocol/status/timeline results and observation values. No production, dependencies, resource schema, gate scripts/classification, browser fixture, test filters or artifact pin change is authorized.

After this exact checkpoint, run the existing direct oracle without print mode against the same identified published module and corrected expected document; require its complete equality check to pass. Re-run the existing resource gate, including its 26 red mutation controls and native row-classification witness. Then finish the already approved hermetic worklet, npm installation, current three-browser qualification/self-tests and generated matrix checks. Reuse the accepted artifact when its source/pin/bytes are unchanged. Preserve the original failed resource record and successful pre-edit oracle capture unchanged; identify the corrected expected source and all terminal statuses honestly.

Further observed discrepancies require their own concrete ruling; this is no blanket numerical repin or new matrix. The existing immutable workspace/native/Wasm sequence, final integrated review, required CI and GitHub delivery synchronization remain root's obligations. Read-only review: no builds/tests, timing, source/spec/Git/GitHub mutations performed. Only this requested temporary ruling was written.
