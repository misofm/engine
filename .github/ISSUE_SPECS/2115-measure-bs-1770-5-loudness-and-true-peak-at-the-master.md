# Measure BS.1770-5 loudness and true peak at the master

Filed by root for the console sprint (2026-10-26). The owner chose loudness metering in the engine.

## Product outcome
An engine observation at the master (and any output) that gives ITU-R BS.1770-5 loudness: momentary (400 ms), short-term (3 s), integrated (gated) and loudness range (EBU Tech 3342), plus true-peak (BS.1770-5 Annex 2, 4x oversampling) in dBTP. It is readable by the console, by voice control and by future fan mixes on both hosts, through the core's observation frames.

## Smallest slice (the scope note decides)
K-weighting filters at the four launch rates, the 400 ms and 3 s windows with 75 % overlap, the absolute (-70 LUFS) and relative (-10 LU) gates, LRA by EBU Tech 3342, and true-peak via the existing Annex 2 detector code where possible. Reset and integration control (start or reset the integrated measure).

## Gates
EBU Tech 3341 and 3342 conformance signals (the minimum requirements), with each listed case within its tolerance; tests at all four launch rates; zero render-path allocation; the observation frames on both hosts.
