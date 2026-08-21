# Source fixtures v1

The `miso-engine-source-fixture` tool is the single generator/checker for this corpus. It generates
in-memory RIFF/RF64 cases for every supported scalar source encoding, including RF64 extensible
float with an odd-padded unknown chunk and a PCM16 stereo nonzero-start, one-frame region. Each
has independent expected `f32` bits, signed-zero/sanitation and EOF assertions. The tool's frozen
exact diagnostic matrix names root-size, truncation, RF64 `ds64`/placeholder, duplicate chunk,
format/GUID/valid-bits, alignment/divisibility, chunk-count and metadata-cap mutations with their
stable `SourceDiagnosticCode` values.

The manifest is sorted by fixture ID and uses `SHA-256<two spaces>fixture-id` lines.
