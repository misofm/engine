# Decision 15, stream B: verdicts

This directory holds byte-for-byte copies of every stream-B batch-1 verifier verdict. The work is
on branch `codex/d15-stream-b`, base `0a1176b3b`. Each issue's spec in `.github/ISSUE_SPECS/` holds
its Attempt record.

| Issue | Title | Commits on this branch | Attempts | Verdict |
| --- | --- | --- | --- | --- |
| #1309 | Extract the C ABI control plane into a portable crate both hosts call | `18f3ca30c`; follow-up `084754c4b` | 1 | PASS ([1309-attempt1.md](1309-attempt1.md)) |
| #1343 | Let the control thread withdraw an unadopted candidate plan | `fb961e801`; follow-up `7b212d48d` | 2 | FAIL ([1](1343-attempt1.md)), PASS ([2](1343-attempt2.md)) |
| #1314 | Publish an applied-revision watermark and complete edits asynchronously | `899db1be7`; follow-ups `0c15798c0`, `0e4dd03ac` (realtime-policy floors) | 1 | PASS ([1314-attempt1.md](1314-attempt1.md)) |
| #1311 | Adopt a successor plan no earlier than a scheduled sample | `efad0080a`; follow-ups `4199f5c05`, `cd5030923` (N4) | 1 | PASS ([1311-attempt1.md](1311-attempt1.md)) |
| #1348 | Add miso_engine_v1_service for bounded control work between edits | `244378b87`; follow-up `7fbccb7ca` (N-4) | 1 | PASS ([1348-attempt1.md](1348-attempt1.md)) |

## Open for root

- **#1314 spec text.** The #1314 Attempt record still names `MailboxPermit::write_revision`.
  `cd5030923` (#1311 N4) removed it: `MailboxPermit::commit(value, revision, adoption)` now stores
  the revision word. The #1314 spec was outside this follow-up's paths.
