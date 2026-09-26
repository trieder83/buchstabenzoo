# art/ — concept art, mockups and references

Everything visual that exists **before** a 3D model: reference images, turnaround sheets,
environment mockups and their briefs. Review it by opening **`art/index.html`** in a
browser (double-click works, no server needed).

Rules: `specs/30-art/asset-pipeline.md` (ART-PIPELINE).

## Layout

```
art/
  index.html                 # review page — renders catalog.js
  catalog.js                 # list of all items, their status and images (edit this)
  reference/                 # external reference images (not our assets)
  props/<kit_id>/            # brief.md, sheet_v*.jpg (modular prop kits)
  characters/<asset_id>/     # brief.md, front.png, side.png, back.png, three_quarter.png, expressions.png
  animals/<asset_id>/        # brief.md, front.png, side.png, back.png, three_quarter.png
  environment/<asset_id>/    # brief.md, overview.png, player_view.png, layout.md
```

`<asset_id>` is the same id as in `assets/manifest.toml`.

## Adding or updating an item

1. Put the files in the item's folder (PNG or JPG, ≤ 2048 px on the long side).
2. Add/update the entry in `catalog.js` (id, title, status, description, spec, brief,
   images). Missing image files show up as "missing" placeholders on the page.
3. Set `status`:
   - `planned` — asset is needed, nothing made yet
   - `brief` — only the brief exists
   - `in-review` — images are ready for review
   - `changes-requested` — reviewer wants changes (write them in `notes`)
   - `approved` — **only a human sets this**, together with `concept_approved = true` in
     `assets/manifest.toml`. Modelling may start after that.
