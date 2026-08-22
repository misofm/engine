# Issue 033 facilitator packet

This directory defines preparation formats only. It contains no listener identity, response,
observation, sign-off, live assignment, playback condition, or listening result.

The facilitator must obtain an exactly permissioned 48 kHz stereo `.mepcm` source and complete the
canonical provenance record before requesting the separately authorized machine preparation. The
private seed is an unsigned 64-bit decimal integer in a regular, one-link, mode-0600 file. Neither
the source, seed, private assignment key, nor role mapping is listener-visible.

After preparation, independently verify the public manifest and packet hashes. Keep the private
directory inaccessible to the listener. Use the historical preregistrations unchanged: 20 valid
filter ABX trials and 20 valid matrix randomized A/B trials, with at most two retained attempts per
logical trial. Invalid attempts record a reason and no answer. Training is separate and carries no
answer.

For filter ABX, privately label the candidate token `A` and comparator token `B`; present `X`
according to the frozen `filter_x_candidate` row, so the retained answer is exactly `A` or `B`.
For the matrix procedure, present the candidate first when the frozen
`matrix_candidate_first` row is true and second otherwise; the retained `A` or `B` answer means
preference for the first or second presentation respectively. Never expose those private labels or
positions in a listener-visible filename or record before reveal.

Before reveal, close and hash the canonical response bytes and obtain pseudonymous listener and
facilitator sign-off. Only then expose the assignment key to a distinct reveal verifier, create the
reveal record, recompute the frozen descriptive statistics, and create the qualification record.
All three pseudonyms must be distinct and must identify real people; agents and synthetic markers
are rejected. Record the real playback chain, calibration, transducer, environment, conflicts,
deviations, and adverse observations. Do not relabel preference or statistical significance as
better sound.

No command in this packet opens an audio device or invokes a player. Playback and the two human
sessions are manual, separately authorized activities. If permission, concealment, identity,
response immutability, or an adverse-observation gate fails, preserve the bytes and stop.
