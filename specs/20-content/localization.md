---
id: CONT-L10N
title: Localization
aspect: content
module: localization
status: draft
depends_on: [CONT-READING]
test_prefix: L10N
updated: 2026-09-26
---

# Localization

## Behaviour

1. Languages: `de` (reference, required), `en` (required), `fr` (later).
2. All player-visible text is in Fluent files `assets/i18n/<lang>/*.ftl`; no text in code,
   textures or models.
3. Key naming: `<area>-<item>-<reading_level>`, e.g. `food-bamboo-kiga`, `sign-zebra`
   (when the reading level suffix is required is open — Q-038).
4. Language can be switched in settings at runtime without restart.
5. Default language = device language if supported, else `de`.
6. UI texts (settings labels, take/close, feedback bubbles) live in `ui.ftl` (`ui-<item>`),
   mission texts in `missions.ftl`. The PoC settings menu (icon button) switches language
   (`de`/`en`) and reading level at runtime and stores both in the browser (`localStorage`);
   without a stored choice the default of §5 applies.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| L10N-001 | Given every key in `de`, then the same key exists in `en`. | unit |
| L10N-002 | Given every key in `en`, then it exists in `de` (no orphans). | unit |
| L10N-003 | Given browser language `fr` while `fr` is disabled, then the game starts in `de`. | unit |
| L10N-004 | Given the language is switched at runtime, then all visible text updates within one frame. | e2e |
| L10N-005 | Given browser language `en`, then the game starts in `en`. | unit |

## Open questions

- Q-008 Voice audio per language. Q-038 Key naming vs. reading level variants.
- Q-064 Enclosure sign texts (`sign-<animal>`).
