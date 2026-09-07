# Evidence format correction after Astra LOW attempt 1

Astra LOW found that the verbatim initial browser stdout ended with a blank line that makes a
committed-base `git diff --check` fail. Root preserved those exact bytes losslessly with deterministic
gzip as `08-qualification/stdout.raw.gz` and removed only the uncompressed copy. References in the
historical attempt-1 report to `08-qualification/stdout.raw` identify this same decompressed byte
stream. No captured byte was edited and no source, test, dependency, lock, pin, artifact, or product
file changed.

The manifest was regenerated after this format-only correction. The contemporaneous attempt-2
qualification record supersedes attempt 1 for acceptance; attempt 1 remains failure evidence.
