# #1055 listening packet: mute click, fader smoothness, responsiveness, polarity flip

A short blinded comparison the owner can run in about 23 minutes (blocks M, F and R of about 5
minutes each and block P of about 8, which may be run in separate sittings). It follows the
repository's listening conventions (`dsp-research/listening/TEMPLATE.md` and the issue-033 packet):
a preregistered record, anonymous stimulus tokens, SplitMix64-v1 randomisation, a private mode-0600
assignment key committed to by hash, a balanced schedule, positive controls, and a reveal that only
runs after every trial is answered. Nothing here is evidence until a human completes it;
`listening.py` never writes synthetic answers outside its self-test's temporary directory.

Files: `PREREGISTRATION.md` (the question, trial counts, statistics and decision rules, fixed
now; Amendment 1 of 2026-10-05, made before any trial, adds block P), `listening.py` (prepare,
run, validate, reveal, self-test; Python 3 standard library).

## 1. Render the stimuli (engine's own ramps)

From `../measure` (see its README for the build):

```text
<target>/release/control_smoothing_measure stimuli --out <scratch>/stimuli
```

This writes one 48 kHz WAV per condition, named by condition, and `manifest.tsv`. The names reveal
the answers: do not listen to these files. Optionally add `--mix-wav <48 kHz WAV> --mix-offset-s
<seconds>` to use a mix you have the right to use in place of the synthetic one (mute, chop
and polarity conditions); record its source in `PREREGISTRATION.md` first.

## 2. Prepare the blinded packet

```text
python3 listening.py prepare --stimuli <scratch>/stimuli --out <packet> --commit $(git rev-parse HEAD)
rm -r <scratch>/stimuli
```

`<packet>/public/` holds 104 anonymous trial files (each: interval 1, 0.6 s of silence, interval 2),
`trials.json` (order and question only), nine labelled training files and `preparation.json`
(hashes and the key commitment). `<packet>/private/assignment-key.json` holds the seed and the
mapping. Do not open `private/`. If someone else can act as facilitator, let them prepare the
packet and keep `private/`.

## 3. Listen

1. Use closed headphones or near-field monitors in a quiet room.
2. Play `public/training/02-mute-bass-50ms.wav` and set the volume to the loudest level you would
   normally mix at. Leave it there for the whole session. Note the level if you have a meter.
3. Play the nine training files in order to learn what each block is about: a hard-switch click, a
   stepped fader, a soft chop, and for block P a steady note with no flip, the same note with a
   click, and the same note with a dip. They carry no answers.
4. Run the blocks (`afplay` is used on macOS; pass `--player "<command>"` otherwise):

   ```text
   python3 listening.py run <packet> --block M
   python3 listening.py run <packet> --block F
   python3 listening.py run <packet> --block R
   python3 listening.py run <packet> --block P
   ```

   Each trial prints its question, plays the file, and asks `1` or `2` (the interval that fits the
   question), then an optional confidence and note. `r` replays, `x` marks a playback problem (one
   retry is allowed), `q` stops; running again resumes. Answers go to `<packet>/responses.jsonl`
   (mode 0600). Guess when unsure: every trial needs an answer. A trial marked `x` twice cannot be
   answered any more; keep that packet as it is and prepare a fresh one (new seed) instead.

## 4. Reveal

```text
python3 listening.py validate <packet>
python3 listening.py reveal <packet>
```

`reveal` refuses until all 104 trials have a valid answer, then prints each contrast's count and
exact one-sided binomial p-value and applies the preregistered rules, writing `<packet>/reveal.json`.
Its `decisions` hold the three values (blocks M, F and R) and, under `polarity`, block P's outcome:
whether the shipped flip was heard, and as a dip, a click or both; it changes no value.
Copy the counts, the playback chain and level, and any observations into a completed copy of
`PREREGISTRATION.md` (status `complete`), with the pseudonymous sign-offs.

## What the answer means

The decisions are bounded to this listener, this playback chain and this level. A missed positive
control makes every non-detection in its own part of the session (M, F and R, or P) inconclusive. A
detection says the difference was audible; it does not say which setting sounds "better". Block P's
outcome is a finding for root, never a value change (`PREREGISTRATION.md`, Amendment 1).

## Self-test

```text
TMPDIR=<scratch> python3 listening.py self-test
```
