# Archived research harnesses

These files are **not compiled**. Each was the evidence harness of a stopped issue; they are kept
as evidence of what was measured, not as code.

| path | issue | status |
| --- | --- | --- |
| `issue-031/portable_filter_quality.rs` | 031 portable higher-precision builtin filter quality mode | FINAL — NO ADOPTION |
| `issue-042/parametric_eq_candidates.rs` | 042 numerically conditioned launch parametric-EQ realization | STOPPED / RESCOPED |
| `issue-044/parametric_eq_time_domain_candidates.rs` | 044 conditioned time-domain launch parametric-EQ recurrence | FAIL / STOPPED |
| `issue-045/parametric_eq_recurrence_proof.rs` | 045 launch parametric-EQ recurrence derivation and runtime proof | FINAL FAIL / STOPPED |

They were last compiled as `#[cfg(test)]` modules of `miso-engine-dsp-reference` at commit
`3be899f`; #105 phase 1 moved them out of default compilation. They depend on `dsp-reference`
items that have since changed (`ReferenceTptStateSpace` is now a wrapper over the single
`ReferenceSvfStateSpace` model, and `ReferenceRetainedTptF32` is documented as a bit-identity twin
rather than an independent oracle), so they will not build unmodified.

The 042 `EXPECTED_SUMMARY_HASHES` mix platform-libm `f32` words and are not a cross-platform
invariant; do not treat them as a gate.

`scripts/check-dsp-research.sh` validates the citation-backed corpus in `dsp-research/*.md` and
does not scan this directory.
