# Issue-005 protocol comparison corpus

The benchmark’s frozen logical corpus has 54 frames per format: a capability
query/response; one 64-operation transaction; two 128-item descriptor pages;
two 128-item parameter-state pages; 40 automation batches totaling exactly
10,000 records; one 256-meter event; counter and diagnostic pages; and success,
revision-conflict, validation-failure, and backpressure responses.

The sequence is FNV-1a-64 over each stable frame label followed by its normalized
logical record: the fixed comparison header followed by every typed semantic leaf in
registered field/index/subfield order, `9eee4fcb61be3b9e`. Native unit tests reconstruct this corpus and
compare the independently computed value with the code constant. Scalar and
`simd128` Wasm execute that same assertion. The FlatBuffers root and its typed
unsigned, float-bit, and length-delimited UTF-8 key/value vectors are defined by `protocol_benchmark.fbs`, built and semantically verified by the
tool-only Apache-2.0 `flatbuffers` 25.12.19 runtime. It does not alter engine,
protocol, browser-host, or realtime dependencies.
