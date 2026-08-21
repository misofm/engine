# Source fixtures v1

The `miso-engine-source-fixture` tool is the single generator/checker for this corpus. Its first
checkpoint generates one RIFF PCM16 stereo case in memory, validates its independent PCM oracle
against production decoding, verifies this manifest, and confirms a header corruption rejects.

The manifest is sorted by fixture ID and uses `SHA-256<two spaces>fixture-id` lines.
