# Sound brief (ART-SOUND)

Sound-artist log: where every cue comes from. The sound artist cannot listen; all cues are
`approved = false` until a human has listened. Manifest block: `python3 tools/sound/register.py`.
Build: `python3 tools/sound/{steps,doors,pickups,ui}.py` (deterministic, seeded, output to
`assets/audio/<group>/<cue>_<n>.{ogg,m4a}`); checks: `python3 tools/sound/check_audio.py`
and `cargo test -p zoo-assets --test audio` (ASND-001/002/003/008; ASND-004 `--ignored`).

Processing (`tools/sound/process.py`): trim at -50 dB, 5 ms fades, K-weighted (BS.1770,
ungated) loudness -16 LUFS, soft limiter, peaks <= -1.2 dBFS, mono 44.1 kHz, Vorbis q2 `.ogg`
+ AAC 48 kbit `.m4a`.

## Group steps (synthesised, `tools/sound/steps.py`, 4 variations each, 0.16-0.30 s)

| Cue | Recipe (intent: soft, cartoon-like) |
|---|---|
| `step_path` | band-passed noise tap + 130 Hz thump |
| `step_grass` | high-band swish + low body + 3 blade crackles |
| `step_sand` | soft hiss + low body + ~14 fine grains |
| `step_wood` | hollow "tok" from 3 damped sines (190/340/610 Hz) + click |
| `step_water` | band-passed splash + 2-3 rising bloops |

Variations differ by seed, pitch (+-10-20 %) and grain placement. The host adds +-5 % pitch.

## Group doors (`tools/sound/doors.py`, 0.55-1.2 s)

`door_wood_open/close` (latch knock + gentle creak), `gate_open/close` (metal clink + higher
creak, close = clonk + double clink), `glass_door` (slide swish + two-tone ding-dong),
`moon_door` (rising 5-note bell shimmer, magical, for the night door).

## Group pickups (`tools/sound/pickups.py`, 0.25-0.55 s)

`pickup_food` (rustle + rising pop), `drop_food` (falling plop + rustle), `pickup_item`
(2 rising marimba notes), `drop_item` (2 falling notes + thud), `harvest_plant` (root
squeak, pop, leaf rustle), `basket_add` (rustle + two-note bloop).

## Group ui (`tools/sound/ui.py`)

`ui_tap` (short marimba pip), `ui_refuse` (two falling soft marimba notes, no buzzer, not
scary), `ui_success` (rising C-E-G-C bell arpeggio).

## Group animals - Gemini feasibility test (2026-09-30)

Goal: can the Gemini API make short cute animal calls? Key from `GEMINI_API_KEY` (never stored).
REST `v1beta/models/<model>:generateContent`. Budget: at most 6 calls; 3 used. The analysis is
numeric only (length, RMS, spectrum, pitch track) - nobody has listened.

| # | Model | Prompt | Result |
|---|---|---|---|
| 1 | `lyria-3-clip-preview` (music), responseModalities AUDIO,TEXT | "A short cute cartoon zebra whinny sound effect, friendly and playful, one single neigh, no music, no melody, no speech, about 1.5 seconds." | 30.8 s mp3 (744 KB), text reply `<instrumental>`. Always a full music clip; loud rhythmic 0.5 s pulses, centroid 500 Hz, no single event. **Unusable** for a sound effect; discarded (not saved). |
| 2 | `gemini-3.1-flash-tts-preview` (speech), voice Puck | "Make a short, cute cartoon horse whinny sound, like a happy zebra: a rising, wobbling, playful 'nyeeeh-hehehe'. Only the animal sound, no words." | 1.96 s PCM 24 kHz; not silent (rms -19 dB, peak -2 dB); voiced 1.4 s, pitch track 650 -> 340 Hz (falling, wobbling); energy 93 % in 300-1000 Hz, almost nothing > 1 kHz: a human-voice imitation, no real horse timbre. Candidate `animal_zebra_call_1` (processed to 1.54 s). |
| 3 | `gemini-3.1-flash-tts-preview`, voice Puck | "Make a tiny, cute koala sound: a soft squeaky grunt, 'hnk-eek!', playful and friendly. Only the animal sound, no words." | 1.60 s; two voiced bits (450-490 Hz, then 520-545 Hz), quiet (rms -27 dB); 98 % in 300-1000 Hz. Candidate `animal_koala_call_1` (1.58 s). |

Not tried (cost / not REST-suitable): `gemini-2.5-flash-native-audio-*` (Live API over
websocket, speech oriented), `veo-3.1-*` (video with audio, paid per second of video).

Findings: see the sound artist report and Q-210..Q-213 in `specs/open-questions.md`.
Both candidates are `licence = "generated"`, `approved = false`.
