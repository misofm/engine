Issue #544 attempt 1 continuation completed. No audit process remains running.

Changed only:

- [.github/workflows/qualification.yml](/home/bl/misofm/engine-audit-subject-disposition/.github/workflows/qualification.yml)

Actual audits, each run exactly once:

- `PATH=/home/bl/.cargo/bin:$PATH /tmp/issue544-target/release/audit capi` — status `0`
  - 100,000 calls; 48 kHz/128 frames; stable address; all counters and violations `0`
  - PCM digest: `ff6cdcb96cdcdad5`
- `PATH=/home/bl/.cargo/bin:$PATH /tmp/issue544-target/release/audit source-duration` — status `0`
  - frames `2880000/518400000`
  - file bytes `11520044/2073600044`
  - layout `17` entries / `6416` bytes
  - all equality flags true; timed benchmarks `0`
  - RSS `3801088/3936256`; Linux, x86_64, rustc 1.97.1

Evidence:

- `/tmp/issue544/capi-stdout-20260907T082741Z-continuation.json`
- `/tmp/issue544/capi-stderr-20260907T082741Z-continuation.txt`
- `/tmp/issue544/capi-exit-20260907T082741Z-continuation.txt`
- `/tmp/issue544/source-duration-stdout-20260907T082741Z-continuation.json`
- `/tmp/issue544/source-duration-stderr-20260907T082741Z-continuation.txt`
- `/tmp/issue544/source-duration-exit-20260907T082741Z-continuation.txt`
- `/tmp/issue544/manifest-20260907T082741Z-544-continuation.txt`

Both stderr captures are empty. Earlier failed-launch evidence remains untouched.

Validation:

- Captured JSON validated post-edit with workflow-equivalent strict Python logic: pass.
- `cargo fmt --all -- --check`: pass.
- YAML syntax: pass using installed PyYAML `yaml.parse`.
- Ruby/Psych was unavailable; no dependency was installed.
- `git diff --check`: pass.
- HEAD and audit source hashes remained unchanged; build/tests were not rerun.

Diff scope: exactly one repository file, 112 insertions, no other edits. No commit, push, or GitHub operations performed.