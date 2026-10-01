The #246 integration did the row-by-row comparison of the app's deleted `installed-msf.ts` (1,190 lines of hard-won OPFS lessons) against the vendored stem store. Most lessons are carried, several are stronger. Five are not — all in engine-owned store code:

1. **Undeadlined awaits in the recovery path** (`opfs-store.js:815-820` `#demote`, `894-907` `#writeIndex`): `#demote` is the store's entire self-heal path and awaits `removeEntry` on the very handle that just failed, with no deadline; `#writeIndex` runs undeadlined inside the index lock, so one wedged write blocks every stem in the realm. The old code's rule: never await a delete on a handle that already hung. Everything else in the file is deadline-wrapped — these are the gap.
2. **`detectSharedReadOnlyMode` is called from nowhere**: an unwritable device downloads every stem in full (resolver reads the whole container before `createWritable`) and then fails with a raw TypeError. Detection must run before the network opens.
3. **Open-time maintenance is unbounded and fatal** (`opfs-store.js:110-117`): the three sweeps are awaited inside `open()`, iterate undeadlined, and any throw makes every mix unopenable. Old rule: bounded, best-effort, never fatal.
4. **Per-open full re-hash** (verify-on-open streams every stem on every `openSession`): correct but unmemoized — 8×100 MB = 800 MB hashed per open. A generation-keyed memo that still checks existence + size would keep the gate's intent.
5. **Minor**: no cheap `file.size` comparison before hashing, so truncation costs a full read.

App-side halves (persist(), Clear-Data lock registration, cross-tab progress, decode-stream cancel) are being fixed in the #246 branch. Full comparison table in that branch's verification record.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
