# 080 Effect state migration registry and bounded chains

## Outcome

Add explicit deterministic old-layout migration to the accepted current-layout state envelope without
changing effect runtime traits or permitting migration on render.

## Status and attempt budget

Stateless successor after Issue 079. Permit one Terra attempt and one bounded Sol correction; a
second failure stops. Workload/benchmark/timed counts remain zero.

## Scope and gates

Freeze a control-plane `StateMigrationRegistry` with one unique step per
`(descriptor_identity, from_layout_version)` and only `N -> N+1` edges. Registration rejects duplicate,
backward, skipped and zero versions. Resolution computes one unique bounded acyclic chain to the
current layout before execution; missing steps, overflow and excess declared scratch reject.

Each step receives immutable input plus caller-provided output/scratch, reports exact required sizes,
and writes all-or-none. The complete chain runs against unpublished temporary buffers and the Issue-
079 unpublished prepared destination; any step/restore failure preserves the live processor exactly.
Fixtures prove zero-step current restore, one/two-step success, every missing/duplicate/failing edge,
scratch size-minus-one, deterministic diagnostics and scalar/bank-member parity.

## Non-goals

Implicit migrations, arbitrary graph search, downgrade, runtime-trait changes, render work,
package/CID, broad qualification/fuzz, benchmark or timing.

## Dependencies by exact issue title

- Prepared effect state envelope and transactional current-layout restore
- Native effect runtime contract and conformance
