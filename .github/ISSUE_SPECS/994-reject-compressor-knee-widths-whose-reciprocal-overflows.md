# Reject compressor knee widths whose reciprocal overflows

## Product outcome

A compressor knee of 2.8e-45 passes parameter validation (`crates/compressor/src/params.rs:128`), `1/(2W)` then overflows to +inf (`dynamics.rs:69`), and a sample exactly at threshold computes NaN (`dynamics.rs:120`). Today the gain-reduction clamp in the kernel (`kernel.rs:353-355`, `max(-100)`) turns that NaN into a -100 dB target, so the defect is masked rather than absent: the output is a hard -100 dB duck on that sample, which is wrong. Found by the compressor optimisation verification.

## Smallest closable slice

Reject (or snap to zero, if a zero knee is the documented hard-knee case) any knee width below the smallest value whose `1/(2W)` is finite and whose knee polynomial stays finite over the admissible level range, with a typed parameter diagnostic, at every entry that sets the parameter (session, control, automation). State the bound and derive it.

## Objective gates

- A test at the old failing knee shows the diagnostic (or the snap) and no NaN at threshold.
- A randomized sweep over admissible knees and levels produces no NaN or infinity in the gain computer.
- Every console digest unchanged.
