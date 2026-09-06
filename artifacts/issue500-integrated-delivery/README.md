# Issue 500 integrated artifact qualification

Source review PASS is retained in the numbered spec; the original Luna package remains unchanged. Delivered compressor main was integrated without graph-compiler changes. The first ordinary builder ran during a documentation-only merge conflict: its HEAD label is insufficient source attribution and it is historical failure evidence only. Recovery and this limitation are explicitly recorded in the spec.

The separate clean ordinary build at be055b4c compiled successfully and exited 1 at the actual artifact comparison. Astra approved only the exact observed digest. Verified builder, static object/ABI checks, resource rejection controls, hermetic worklet tests, npm install, three-browser qualification with existing self-tests, and matrix check then all exited 0 on 02c74cc2. Generated candidate/hash records may change; numerical expectations remain frozen. Module identity is independently recorded here.

No timing or measured speedup is claimed. The unrelated post-main compressor allocation-test failure is tracked in successor #503 and remains a required delivery concern. This artifact evidence does not waive it. All raw historical failures are preserved verbatim.
