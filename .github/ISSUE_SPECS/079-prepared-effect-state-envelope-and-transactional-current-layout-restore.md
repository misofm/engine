# 079 Prepared effect state envelope and transactional current-layout restore

## Outcome

Define canonical persisted bytes for one accepted prepared effect's exact current state layout and
prove transactional restore into a separately prepared unpublished destination.

## Status and attempt budget

Stateless successor after accepted Issue 082. Permit one Terra attempt and one bounded Sol
correction; a second failure stops. Workload/benchmark/timed counts remain zero.

## Scope and gates

Freeze one exact state envelope containing descriptor identity, effect ID, contract/layout version,
sample rate, quantum, quality, bypass, link mode, prepared ports, latency, tail, state sizes, complete
ordered initial parameter configuration and exact common/left/right payload lengths/bytes with a
domain-separated digest. Use checked bounded caller storage and exact diagnostics. Do not require
left/right byte counts to differ from or equal anything beyond accepted metadata.

Snapshot validates exact current metadata and writes atomically. Restore verifies every binding and
payload digest, prepares an unpublished destination from the saved configuration, calls the accepted
current-layout restore hook, and publishes/returns it only on complete success. Wrong identity,
configuration, version, size or digest leaves the live processor and caller buffers unchanged.
Representative scalar and bank-member fixtures prove common/left/right preservation, active-state
continuation, malformed rows and size-minus-one canaries.

## Non-goals

Older-layout migration, package/CID, runtime-trait changes, live render mutation, repository work,
broad fuzz/targets, benchmark or timing.

## Dependencies by exact issue title

- Close canonical effect descriptor wire, identity, and C inspection ABI
- Native effect runtime contract and conformance
- Versioned TOML schema and transactional session compiler
