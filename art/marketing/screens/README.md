# Letter Zoo store screenshots and gameplay video

Produced 2026-10-06/07 from the **release build of commit `6c109b3`** (clean git worktree,
`npm run build`, `vite preview` on a private port), headless Chromium with SwiftShader (the same
flags as `web/playwright.config.ts`), German UI, reading level `kiga`, seed `?seed=17`.
No game code was changed; nothing is a mock-up, all frames are real renderer output.

## Files

- `letterzoo_screen_01..05_*` the five store screenshots (01 day, girl leads the zebra over the
  bridge; 02 riddle board with pictograms; 03 zebras home with confetti; 04 NIGHT plaza with
  lantern light and string lights; 05 NIGHT terrarium hall). `06_extra_food_box`, `07_extra_night_planets`
  (telescope, Jupiter selected) are extras.
- Landscape: rendered at 1920x1080 CSS px with device scale factor 2 -> `_3840x2160.png`; the
  `_1920x1080.png/.jpg` files are Lanczos downscales of it. Portrait: 1080x1920 at DSF 2 ->
  `_2160x3840.png`, `_1080x1920.png/.jpg`.
- `letterzoo_gameplay_1920x1080.mp4/.webm` and `letterzoo_gameplay_1080x1920.mp4/.webm`: 15.0 s, 30 fps
  (x264 crf 17 / VP9 crf 33, no audio).

## How

Playwright scripts (kept outside the repo) drive `window.__zoo.app` debug hooks: `debug_teleport`,
`debug_goto`, `debug_step`, `debug_stand_near`, `debug_face_animal`, `debug_send_home`,
`debug_set_daytime`-free night via `debug_send_home` x3 + `debug_step(18.5)`, `zoom`, `rotate`.
Real input (E, A/W, the take button) opens the board, takes the food and shows it to the zebra.
Overlays hidden by CSS only: the `E` hint pill and the ad reading panel (`ads.tick/interact` stubbed
so standing near an ad billboard does not open the ad panel and swallow the E key). HUD, speech
bubble and celebration banner are real game UI.

**Video is frame-stepped:** `requestAnimationFrame` is replaced by a manual queue; each captured frame
advances the game by exactly 1/30 s (the game loop calls `app.frame(dt)` with a fixed dt), `setTimeout`
timers and CSS animations are driven by the same virtual clock, so UI timing and confetti are
deterministic although software GL needs ~2 s per frame. 450 frames, JPEG capture, ffmpeg encode.
Story: 0-4.5 s walk to the zebra enclosure board, riddle panel; 4.5-6.5 s food box "Gras";
6.5-10.5 s find the zebra, "Juhu, Futter! Ich komme mit.", lead it (the middle of the walk
is skipped, hidden simulation); 10.5-13.2 s zebras home, confetti; 13.2-15 s night plaza.
Cuts are 6-frame cross-dissolves; the first 8 frames fade in from cream.

## Notes

- The camera is already at its maximum zoom-out (20 m) at start, so there is no wide "empty zoo"
  overview; the shot shows the empty zebra enclosure while walking.
- In-game ad billboards ("Math Fighter" etc.) are visible in some frames (06 food box, video C).
  Check before use as store material.
- No end card / title text; add later. The kiga riddle panel is picture-based (river icon + "Gras");
  use `reading_level = klasse1` for the full riddle sentence.
- Portrait terrarium shot shows only part of the hall (max zoom-out reached).
