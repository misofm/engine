# Issue 033 sealed preparation formats

The four JSON schemas freeze `Issue007ListeningPreparationV1`,
`Issue007ListeningResponseV1`, `Issue007ListeningRevealV1`, and
`Issue007ListeningQualificationV1`. Canonical records are UTF-8/LF JSON with recursively sorted
keys and no insignificant whitespace. Response records are canonical JSONL in strictly increasing
sequence order.

`provenance.template.json` is intentionally incomplete and must never pass preparation validation.
`response-form.jsonl` is intentionally empty and answer-free. Synthetic rows exist only inside the
validator self-test process and are never written as listening evidence.

Public stimulus names are 32 lowercase hexadecimal characters plus `.wav`. Only the private
mode-0600 assignment key maps these tokens to roles or trials. The public packet commits to the
key and schedule hashes without carrying the seed, role words, source path, or mapping. Its
preparation record also carries a closed SHA-256 map for every packet member except itself; packet
validation recomputes those hashes and rejects copied-input drift.

Reveal validation consumes the immutable response bytes and private assignment-key file, checks
the response/key hashes, exact trial order and committed token/trial mappings, and only then permits
a linked qualification record. Qualification validation requires exactly the preparation,
responses and reveal authorities and recomputes all three hashes, response-derived counts and
statistics.
