# Source fixtures v1

The `miso-engine-source-fixture` tool is the single generator/checker for this corpus. It generates
in-memory RIFF/RF64 cases for every supported scalar source encoding, including RF64 extensible
float with an odd-padded unknown chunk. Each has independent expected `f32` bits, and the tool also
checks representative malformed-format, duplicate-data, and metadata-cap mutations.

The manifest is sorted by fixture ID and uses `SHA-256<two spaces>fixture-id` lines.
