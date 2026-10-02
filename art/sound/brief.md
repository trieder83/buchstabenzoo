# Sound brief (ART-SOUND)

Sound-artist log: where every cue comes from. The sound artist cannot listen; all cues are
`approved = false` until a human has listened. Manifest block: `python3 tools/sound/register.py`.
Build: `python3 tools/sound/{steps_real,doors,pickups,ui,animals}.py` (deterministic, seeded, output to
`assets/audio/<group>/<cue>_<n>.{ogg,m4a}`); checks: `python3 tools/sound/check_audio.py`
and `cargo test -p zoo-assets --test audio` (ASND-001/002/003/008; ASND-004 `--ignored`).

Processing (`tools/sound/process.py`): trim at -50 dB, 5 ms fades, K-weighted (BS.1770,
ungated) loudness -16 LUFS, soft limiter, peaks <= -1.2 dBFS, mono 44.1 kHz, Vorbis q2 `.ogg`
+ AAC 48 kbit `.m4a`.

## Group steps (REDONE 2026-09-30, `tools/sound/steps_real.py`; the synth version in `steps.py` was "not realistic at all")

Real recordings cut at the onset, pitched up +1...+5 semitones by resampling (smaller, lighter, slightly
quicker step), high-pass 90-110 Hz, low-pass 2.2-5 kHz (soft, not crunchy), faded out within 0.25-0.35 s,
then `process.normalise`. Variant files `step_<surface>_<n>`; the host finds the count from the file index.

| Cue | Variants | Source | Licence |
|---|---|---|---|
| `step_path` | 6 | Fantozzi "Stone" L/R 1-3, https://opengameart.org/content/fantozzis-footsteps-grasssand-stone (Freesound pack 10338) | CC0 |
| `step_grass` | 6 | Fantozzi "Sand" L/R 1-3, same page (author: "sand sounds like grass too"), low-pass 3.6-4 kHz | CC0 |
| `step_sand` | 4 | TinyWorlds gravel (x2 pitches) + mud02, https://opengameart.org/content/different-steps-on-wood-stone-leaves-gravel-and-mud; 1x Fantozzi Sand R2 low-passed 2.2 kHz | CC0 |
| `step_wood` | 4 | TinyWorlds wood01/02/03 (+ wood03 at +5 st), same page; very bassy source (centroid ~180 Hz), high-passed 110 Hz | CC0 |
| `step_water` | 4 | Runway `eleven_text_to_sound_v2`: 2 takes x 2 pitches | generated (Q-250) |

Runway (docs.dev.runwayml.com: `POST /v1/sound_effect`, model `eleven_text_to_sound_v2`, 1 credit/s;
header `X-Runway-Version: 2024-11-06`; script `tools/sound/runway_gen.py`, key from `MF_RUNWAY_API_KEY`,
never stored). 8 calls, raw takes in `.run/sound-src/runway/` (not committed). Prompts:

| Take | Prompt | Duration | Analysis | Used |
|---|---|---|---|---|
| grass_a | Soft footstep of a small child in light shoes walking on grass, single step, dry, close microphone, gentle | 1 s | 2 peaks, -30 dB span 0.86 s, centroid 6.3 kHz | no (long, hissy) |
| path_a | ... walking on a paved stone path, single step, dry, close microphone, gentle | 1 s | one step at 0.44 s, weak (-29 dBFS), centroid 0.7 kHz | no |
| grass_b | One single soft footstep on lawn grass by a light child, no other sounds, dry, close microphone | 1 s | 5 peaks (several steps), centroid 5.0 kHz | no |
| path_b | One single soft footstep of a light child on cobblestone street, no other sounds, dry, close microphone | 1 s | 7 peaks, centroid 2.5 kHz | no |
| grass_c | Single soft footstep of a small child on grass, light shoe, muffled, dry, no echo, close microphone | 0.5 s | 2 peaks, span 0.38 s, centroid 4.3 kHz | no (brighter than the real ones, 2.6 kHz) |
| path_c | Single soft footstep of a small child on a paved stone path, light shoe, muffled, dry, no echo, close microphone | 0.5 s | 4 peaks, centroid 2.5 kHz | no |
| water_a | Single soft footstep of a child splashing in shallow water, small gentle splash, dry, close microphone | 0.6 s | 4 peaks, span 0.40 s | yes (step_water 1, 3) |
| water_b | Gentle small splash of a bare child foot stepping in a shallow puddle, single step, cartoon-soft | 0.6 s | 3 peaks, span 0.46 s | yes (step_water 2, 4) |

Generated footsteps came out multi-hit / hissy (the model does not reliably give one footfall), the
real recordings were cleaner, so only the water splashes (no CC0 source found) use Runway output.
Analysis of the delivered files (Welch centroid, -30 dB span): grass 0.23-0.25 s, centroid 2.4-2.8 kHz;
path 0.21-0.24 s, 2.2-2.5 kHz; sand 0.19-0.29 s, 1.1-2.0 kHz; wood 0.17-0.23 s, ~180 Hz; water
0.26-0.31 s, 2.1-3.0 kHz. All -16 LUFS, peaks -8...-5 dBFS, no clipping, tail < -24 dB at the end.
Nobody listened.

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

## Group animals - real recordings (route 1 of Q-210, 2026-09-30)

User decision: real recordings under a free licence, processed until cute; no speech. Built by
`python3 tools/sound/animals.py` (downloads the sources once into `.run/sound-src/`, cuts,
high-pass 120 Hz, pitch via ffmpeg `rubberband` (length kept), low-pass, gentle fades, then
`process.py`). Cue table (segment, semitones, filter) = `CUES` in that script and the `cut` field of
each manifest entry. Sources searched: Wikimedia Commons (API, category "Audio files of <taxon>",
licence from `extmetadata`), OpenGameArt (licence shown on the page). Freesound: no
`FREESOUND_API_KEY` in the environment, not used. Nobody has listened; analysis only (length,
LUFS, peak, spectral centroid, silence ratio, clipping: all ok, see report). Everything `approved = false`.

| Species | Cues | Source (licence) | Remark |
|---|---|---|---|
| elephant | call / happy / refuse | Commons `Elephant_voice_-_trumpeting.ogg` (CC0) | trumpet cut 1.3 / 0.5 / 0.3 s, +3 / +6 / -2 st, low-pass 6 kHz |
| zebra | call / happy / refuse | Commons `Wiehern.ogg` (public domain) | real horse neigh; `_baby` = foal whinny from OGA "Baby Animals - Sounds Pack" (CC0). The two Gemini TTS candidates are replaced. A real zebra whinny exists on OGA (CC-BY, `zebra-whinny`), not used because a CC0 / PD one was available |
| lion | call / happy / refuse | Commons `Lion_raring-sound1TamilNadu178.ogg` (PD) | first roar only, +4 st, low-pass 2.8 kHz to take the edge off; happy / refuse are short grumbles |
| panda | call / happy / refuse | Commons `Giant_panda_twittering.ogg` (PD) | friendly twitter by nature |
| monkey | call / happy / refuse | OGA `monkey-sounds` (CC0) | human imitation (Stendhal), not a recording of an animal |
| hippo | call / happy / refuse | OGA `camel-groan` (CC0) | STAND-IN, no free hippo recording found |
| snow_fox, fennec | call / happy / refuse each | Commons `Fennec_Singing.ogg` (PD) | snow fox = other pieces, lower pitch (stand-in) |
| bat | call / happy / refuse | OGA `bat-screeches` (CC0) | pitched down, low-pass 5 kHz |
| owl | call / happy / refuse | Commons `Steinkauz.OGG` (PD, little owl) | |
| hedgehog | call / happy / refuse | Commons `Hedgehog_O.ogg` (CC0, eared hedgehog) | short snippets, 3 segments guessed from the level envelope |
| koala | call / happy / refuse / baby | Commons PLoS ONE Movie S1 (CC-BY 2.5) | male bellow, pieces pitched +5 / +8 / +3 / +12 st, low-passed; credit in CREDITS.md. Replaces the Gemini candidate |
| kiwi | call / happy / refuse | Commons `Male-ni-brown-kiwi.wav` (CC-BY 4.0, DOC NZ) | credit in CREDITS.md |
| raccoon | call / happy / refuse | OGA `raccoon-chatter` (CC-BY 3.0+) | credit in CREDITS.md |
| goldfish | call / happy / refuse | synthesised (`animals.py`, own) | rising "blub" chirps, plop |

Not found under a free licence (Q-215): giraffe (hums exist but CC BY-SA), badger, porcupine,
slow loris, tarsier. Also excluded: OGA barn owl and toucan (CC BY-SA), dolphin (BY-SA), Commons
hedgehog "sleeping" (BY-SA). The Gemini feasibility test above is kept as history; its two
candidate files are gone (replaced by `animal_zebra_call_1`, `animal_koala_call_1` from recordings).

## Group ambient (`tools/sound/crickets.py`, 2026-10-01, user request: quiet crickets at night)

Cue `ambient_crickets` (1 variant, 21.8 s, mono, loop). Source search: Wikimedia Commons cricket
recordings are CC BY / CC BY-SA (not allowed or credit needed) or only Lingua Libre spoken words;
OpenGameArt "Crickets Ambient Noise - loopable" (Ted Kerr, **CC0**, 11.5 s stereo mp3,
https://opengameart.org/content/crickets-ambient-noise-loopable) is clean (energy 3-5 kHz, nothing else).
No Runway / Freesound call was needed. Processing: mono, high-pass 1 kHz, 2 passes of the source (second
rotated 37 % and resampled +3 %) joined with 0.8 s equal-power crossfades, wrap-around crossfade at the
loop point, -22 LUFS (ungated K-weighted), low-pass 9 kHz, Vorbis q0 / AAC 32 kbit.
Analysis: spectral peak 3.3 kHz, peak -9.2 dBFS, loop jump 0.22 x the largest normal sample step, level
at the seam +1.2 dB vs the file. ogg 119 KB, m4a 93 KB. Nobody listened (approved = false).
