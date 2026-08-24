<!--
Provenance: copied from misofm/engine-v2-old docs/research/05-dynamics-and-oversampling.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Dynamics and oversampling

The compressor design separates detector, gain computer, smoothing, and gain application so every state update/order is owned and testable. Its behavior and terms are grounded in the survey by [Giannoulis, Massberg, and Reiss](https://eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf). Lookahead, if admitted, is an explicit bounded mount allocation and reported latency.

The master limiter targets true-peak measurement rather than sample-peak labeling. It must state its oversampling factor, filter/order, lookahead, delay, and ceiling in its descriptor, with ITU-R BS.1770 as the measurement reference ([BS.1770-5](https://www.itu.int/rec/R-REC-BS.1770-5-202311-I/en)). Oversampling design is reviewed against published tradeoffs ([JAES article](https://doi.org/10.17743/jaes.2019.0012)) and limiter behavior literature ([DAFx limiter paper](https://users.aalto.fi/~hamalap5/dafx2002/dafx_hamalainen.pdf)).

All time values, latency, tail, and automation conversions derive from the immutable session rate; rate validation occurs before allocation. No algorithm is accepted on sound alone: compressor and limiter require Sol-approved independent objective reference/oracle gates with derived tolerances before implementation. Gates require finite output, bounded state, declared latency/tail, no process allocation, deterministic event timing, and exact native/browser corpus agreement at certified rates. Quality modes that would change arithmetic/order receive a new determinism profile/version rather than silently joining V0.1.
