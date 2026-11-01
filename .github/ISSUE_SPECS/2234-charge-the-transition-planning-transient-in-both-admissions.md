# Charge the transition-planning transient in both admissions

Filed by root from the #2232 verify (2026-11-01). Follow-up; not launch-blocking.

## Problem
After #2232, a structural edit that takes the transition still allocates a transient in `plan_strip_transition` that no admission charges: 0.63 MB at 64 restarted strips, 2.53 MB at 256 and 10.1 MB at 1,024 (about 9.9 KB per strip; measured ceiling about 1.06 × `canonical_upper_bound_bytes` of the committed model). The replacement pre-check charges exact rows only (#2199 D3), so a session near its budget can pass the pre-check and then allocate past it while planning the transition.

## Scope
Charge the transition-planning transient as one row in both the replacement pre-check and the exact admission after preparation, with the same rule, so the pre-check never refuses what the exact admission admits (#2199 D3). Derive the row from counts known before preparation (restarted strips and the model's canonical bound), not from a measured constant.

## Gates
- A test with `bench_support`'s thread-scoped peak: the transition-planning peak on a bus add at 64 and 256 tracks stays under the new row (a ceiling). Red when the row is left out of the pre-check.
- The C ABI exact-boundary tests stay green unchanged.
- No rendered bit moves (one-time comparison, PR evidence only).
