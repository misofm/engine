# Third-party licensing

Except for the third-party material identified below, the original material in this repository is
licensed under the Apache License, Version 2.0. See `LICENSE`.

## Vendored libm material

`crates/math/src/vendored/` contains material derived from `rust-lang/libm` and its upstream
sources. That material remains under its applicable MIT, Apache-2.0, BSD, and other permissive
notices; it is not relicensed merely by inclusion in this Apache-2.0 project.

`crates/math/LICENSE-libm.txt` is the authoritative bundled license and attribution record for that
material. Distributions containing the vendored implementation must retain that file and the
copyright notices present in the individual source files.

## External dependencies

Dependencies resolved through Cargo or npm remain under their respective licenses. Lockfiles name
exact resolved packages but do not replace their license texts or notices. A binary distributor is
responsible for retaining the notices required by the dependencies included in that binary.

## References and research sources

Specifications, papers, books, product documentation, and other external works cited by the
research corpus are references rather than incorporated project material. Their citations do not
license those works under Apache-2.0.
