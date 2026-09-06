# #480 historical evidence preservation — APPROVED, narrowly

Read-only inspection confirms `/tmp/480-luna1-root-env.log` contains the seven rejected profile metadata environment names. The same search found no such occurrence in `/tmp/480-luna1-trace.log`; do not encode that file merely because it was suspected. Root must select other raw files by actual exact-token occurrence, not by filename or attempt number.

Approve lossless base64 archival representation ONLY for original raw evidence files containing those seven rejected names, using the established PR474 precedent. Preserve each original file's complete bytes, including failed output and final newline state. Publish its `.base64` representation rather than a second tracked plaintext copy; do not redact, rewrite, substitute names or modify a gate/vocabulary exemption.

Each manifest row must bind encoded path/size/SHA256 and original decoded filename/size/SHA256, with an explicit base64 encoding label. Verify decoding reproduces the original complete bytes and original digest before final packaging. Preserve raw originals outside the scanned tracked artifact tree. All other payloads retain their original bytes and ordinary representation. The archive is historical failed evidence, never current valid environment API or newly executed proof.

Run the existing packaged environment check after assembly and verify complete manifest/tracking/decoded identity at actual-PR review. No source, policy, vocabulary, runtime or benchmark authority changes are approved. This ruling does not itself edit artifacts or execute preparation/timing.
