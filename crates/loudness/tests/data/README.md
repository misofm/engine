# Official ITU loudness file qualification

The explicit integration test qualifies all **19 mono/stereo-applicable files** from
[ITU-R BS.2217-2 (2016)](https://www.itu.int/dms_pub/itu-r/opb/rep/R-REP-BS.2217-2-2016-PDF-E.pdf)
at native 48 kHz, signed 16-bit PCM. Each integrated reading must meet the report's
file-based ±0.1 LKFS tolerance. The original PCM is preserved, including the two
WAVE_FORMAT_EXTENSIBLE programme files. Mono is fed to left with right zero:
the report assigns unit weight to mono and to each stereo channel. There is no
resampling, duplication, channel dropping, downmix, gain adjustment or EOF padding.

`official-loudness.tsv` is the sole corpus inventory: every resource on the
[official catalogue](https://www.itu.int/oth/R1102000001/en) is recorded, including
its aggregate archive. Nineteen selected components have observed archive/WAV
SHA-256 identities and PCM metadata; the other 28 have 6/8/10/12/24 channels and
are outside the shipped stereo-output meter contract. They are not passing
conformance rows. Excluded rows use the catalogue resource label; their member
name and hashes are deliberately unspecified because those bytes are not used.
All excluded rates/bit depths/layouts come from the report, not measured headers.
The two −24 LKFS programme WAVs (resources 0038/0040) have a published RIFF
container size eight bytes smaller than the physical file. Their individual chunk
lengths are correct. The manifest pins that exact delta to eight only for those
two byte-verified files (zero elsewhere); readers walk bounded complete chunks
to physical EOF and never rewrite the header or PCM. Initial strict container
size equality refused these files; this is an official header defect, not a DSP
failure. Truncated chunks, partial PCM frames and unpinned length drift fail.

The report's file typography sometimes differs from the actual archive member;
the selected rows pin the actual member spelling.

Download audio only from the official URLs into a private directory outside the
checkout. [ITU copyright terms](https://www.itu.int/home/copyright/index.html)
permit use within the receiving organization for product/service development and
implementation; external distribution requires permission. No audio, archives or
copied publication text are committed or distributed by this repository.

```sh
python3 crates/loudness/tests/data/fetch-official-loudness.py /private/path/itu-loudness
MISO_LOUDNESS_FIXTURES=/private/path/itu-loudness \
  cargo test -p loudness --test official_files -- --ignored --exact itu_bs2217_published_mono_stereo_files --nocapture --test-threads=1
```

The fetcher checks each archive SHA-256 before extracting its one exact member,
then checks WAV SHA-256 and PCM metadata. Existing WAVs are always verified;
mismatching WAVs/archives are refused without overwriting. The `--verify` option
requires and verifies every archive and WAV without network access. The test invokes
that offline verifier synchronously before measurement, so corrupt or absent
required files fail the gate. Python 3 is required only for this explicit gate.
The test is visibly ignored in ordinary offline `cargo test`; an ordinary suite
pass makes no claim that official file qualification ran. The explicit command
runs all selected cases with no missing-file/per-case skips. In coordinated
repository work, run acquisition checks, Cargo and format/lint commands through
the assigned serial wrapper.

Synthetic EBU Tech 3341/3342 recipe and true-peak cases keep their existing owners.
The original #2127 request also covered two authentic EBU programmes; external
access failed with HTTP 403 and [#2167](https://github.com/misofm/engine/issues/2167)
now owns their acquisition and I/LRA qualification. This ITU gate does not claim
EBU programme or multichannel compliance.
