//! Night-only scene parts (GAME-NIGHT §10, `art/night/README.md`): lamps and their light,
//! placeholder lamp props (post + glowing box) until the `kit_night` models exist, lit
//! windows on buildings, board lamps on every info board, wall lamps at doors, the glowing
//! moon sign, indoor lights of enterable buildings (the night house in blue and warm
//! red-orange, Q-116) and the firefly areas. Pure — the renderer only draws it, and hides it
//! all by day.

use glam::{Vec2, Vec3};

use crate::coords::level_to_world_at;
use crate::level::{ElementType, LevelData, Rect};
use crate::scene::{
    building_model, facing_yaw, info_board_pose, model_offset, moon_door_pose, BoxPlacement, Dir,
    Placement, BOARD_LAMP_LIGHT, BOARD_LAMP_MAP, LANTERN_LIGHT, MOON_DOOR_LIGHTS, STRING_SPAN_M,
    WALL_LAMP_LIGHT, WALL_LAMP_MOUNT_M,
};

/// Glow and light colours (sRGB) of the art plan's night colour table.
pub mod colors {
    /// Lamp glass `#FFD66B`.
    pub const LAMP_GLOW: [f32; 3] = [1.0, 0.839, 0.420];
    /// Lamp light on surfaces (`#FFC46E` pool as a warm light).
    pub const LAMP_LIGHT: [f32; 3] = [1.0, 0.90, 0.72];
    /// Lit window `#FFC857`.
    pub const WINDOW: [f32; 3] = [1.0, 0.784, 0.341];
    /// Moon sign `#FFF4C9`, rim `#8FB8FF`.
    pub const MOON: [f32; 3] = [1.0, 0.957, 0.788];
    pub const MOON_RIM: [f32; 3] = [0.561, 0.722, 1.0];
    /// Night house indoor light: soft blue `#5B7FE0`, warm red-orange `#E8735A` (Q-116),
    /// as lights on the albedo (lightened so nothing gets too dim, NIGHT-005).
    pub const HOUSE_BLUE: [f32; 3] = [0.62, 0.74, 1.0];
    pub const HOUSE_WARM: [f32; 3] = [1.0, 0.72, 0.62];
    /// Warm ceiling light of other buildings.
    pub const CEILING: [f32; 3] = [1.0, 0.92, 0.78];
    /// Lamp post wood and metal.
    pub const POST: [f32; 3] = [0.40, 0.28, 0.20];
    pub const METAL: [f32; 3] = [0.30, 0.30, 0.34];
}

/// Light radius (m) per lamp kind (art plan: post 3 m, board lamp 1.2 m, player 2.5 m; the
/// light sits at the model's `light` empty, README_night).
pub fn default_radius(kind: &str) -> f32 {
    match kind {
        "lantern_post" => 3.0,
        "string_lights" => 2.0,
        "wall_lamp" => 2.1,
        "board_lamp" => 1.2,
        "ceiling" => 3.0,
        _ => 2.5,
    }
}

/// One lamp at night: where its light is (world), how far it reaches, its colour.
#[derive(Debug, Clone, PartialEq)]
pub struct Lamp {
    pub id: String,
    pub kind: String,
    /// Light centre (world). Ground lamps sit low so their pool is centred under the lamp.
    pub light: Vec3,
    pub radius: f32,
    pub color: [f32; 3],
    /// Level part (render chunk).
    pub part: u8,
}

/// The night-only scene parts of a (joined) level.
#[derive(Debug, Clone, Default)]
pub struct NightScene {
    /// Lamp models (`kit_night`: lantern posts, string lights, wall and board lamps); their
    /// `*_glow` slots light up at night.
    pub placements: Vec<Placement>,
    /// Placeholder lamp props (posts, arms, wires): lit like everything else.
    pub boxes: Vec<BoxPlacement>,
    /// Emissive parts (lamp glass, lit windows, the moon sign).
    pub glows: Vec<BoxPlacement>,
    pub lamps: Vec<Lamp>,
    /// Firefly areas (level rect, part): where tiny blinking lights dance (Q-115).
    pub fireflies: Vec<(Rect, u8)>,
}

fn rect_center(r: Rect) -> Vec2 {
    Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0)
}

impl NightScene {
    /// Builds the night parts from the level data.
    pub fn build(data: &LevelData) -> Self {
        let mut s = NightScene::default();
        for l in &data.lights {
            s.light_entry(l, data);
        }
        // lamps the data already attaches to a board / building (no automatic second one)
        let attached = |kind: &str, id: &str| {
            data.lights
                .iter()
                .any(|l| l.kind == kind && l.attach.as_deref() == Some(id))
        };
        for e in &data.elements {
            let part = e.part as u8;
            let kind = e.kind.as_deref().unwrap_or("");
            match (e.ty, kind) {
                (ElementType::Decoration, "info_board") => {
                    // every info board gets a small board lamp (GAME-NIGHT rule 5)
                    if !attached("board_lamp", &e.id) {
                        let (pos, dir) = info_board_pose(e, data);
                        s.board_lamp(
                            &e.id,
                            pos,
                            facing_yaw(dir),
                            crate::scene::info_board_lamp_socket(e),
                            default_radius("board_lamp"),
                            part,
                        );
                    }
                }
                (ElementType::Barrier, "moon_door") => s.moon_door(e, data, part),
                (ElementType::Building, "entrance") => {}
                (ElementType::Building, _) => {
                    let h = e.height_m.unwrap_or(4.0) * 0.7;
                    let door = e.door_cell();
                    let door_lamp = !attached("wall_lamp", &e.id);
                    // a building model has its own glowing windows (`window_glow`) and
                    // ceiling lamps
                    let model = building_model(e);
                    if model.is_none() {
                        s.windows(e.rect, door, door_lamp, h, &e.id, part);
                    } else if door_lamp {
                        s.windows(e.rect, door, true, 0.0, &e.id, part);
                    }
                    if let Some(inner) = e.interior {
                        let night_house = data.is_night_part(e.part) || kind == "night_house";
                        if !attached("indoor", &e.id) {
                            s.indoor_lights(&e.id, inner, night_house, part);
                        }
                        if night_house {
                            // one light per indoor enclosure, soft blue and warm red-orange
                            // in turn (Q-116), inside the house model
                            let m = e.model_rect.unwrap_or(e.rect);
                            let mut k = 0;
                            for enc in data.elements_of(ElementType::Enclosure) {
                                let ec = rect_center(enc.rect);
                                if !m.contains(crate::level::cell_of(ec))
                                    || attached("indoor", &enc.id)
                                {
                                    continue; // (the data lists its own indoor light)
                                }
                                let color = if k % 2 == 0 {
                                    colors::HOUSE_BLUE
                                } else {
                                    colors::HOUSE_WARM
                                };
                                let r = (enc.rect.w.max(enc.rect.d) as f32 / 2.0 + 0.3).min(4.5);
                                s.lamp(
                                    &format!("{}:indoor", enc.id),
                                    "indoor_colored",
                                    level_to_world_at(ec, 1.4),
                                    r,
                                    color,
                                    part,
                                );
                                if model.is_none() {
                                    s.glow(
                                        &enc.id,
                                        level_to_world_at(ec, 3.0),
                                        Vec3::new(0.5, 0.1, 0.5),
                                        0.0,
                                        color,
                                        part,
                                    );
                                }
                                k += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        for sc in &data.scenery {
            if sc.kind == "fireflies" || sc.props.iter().any(|p| p == "firefly" || p == "fireflies")
            {
                let part = data
                    .part_at(glam::IVec2::new(sc.rect.x, sc.rect.z))
                    .unwrap_or(0) as u8;
                s.fireflies.push((sc.rect, part));
            }
        }
        s
    }

    fn glow(&mut self, source: &str, pos: Vec3, size: Vec3, yaw: f32, color: [f32; 3], part: u8) {
        self.glows.push(BoxPlacement {
            pos,
            size,
            yaw,
            color,
            fadeable: false,
            source: source.to_owned(),
            part,
        });
    }

    fn prop(&mut self, source: &str, pos: Vec3, size: Vec3, yaw: f32, color: [f32; 3], part: u8) {
        self.boxes.push(BoxPlacement {
            pos,
            size,
            yaw,
            color,
            fadeable: false,
            source: source.to_owned(),
            part,
        });
    }

    fn lamp(&mut self, id: &str, kind: &str, light: Vec3, radius: f32, color: [f32; 3], part: u8) {
        self.lamps.push(Lamp {
            id: id.to_owned(),
            kind: kind.to_owned(),
            light,
            radius,
            color,
            part,
        });
    }

    /// One `[[light]]` of the level data (placeholder props until the models exist).
    fn light_entry(&mut self, l: &crate::level::LightData, data: &LevelData) {
        let part = l
            .pos()
            .and_then(|p| data.part_at(crate::level::cell_of(p)))
            .unwrap_or(l.part) as u8;
        let radius = l.radius_m.unwrap_or_else(|| default_radius(&l.kind));
        let dir = l
            .facing
            .as_deref()
            .map_or(Dir::S, |f| Dir::from_vec(crate::level::facing_vec(f)));
        let fwd = dir.offset().as_vec2();
        match l.kind.as_str() {
            "string_lights" => {
                // `string_lights` spans (≤ 6 m, the cord stretched to the span, proposal
                // Q-147) from post to post, a `string_post` at the far end
                let pts = l.points();
                for w in pts.windows(2) {
                    let (a, b) = (w[0], w[1]);
                    let len = a.distance(b);
                    if len < 0.1 {
                        continue;
                    }
                    let n = (len / STRING_SPAN_M).ceil().max(1.0) as usize;
                    let d = (b - a) / len;
                    let yaw = d.y.atan2(d.x);
                    for k in 0..n {
                        let p = a.lerp(b, k as f32 / n as f32);
                        let mut pl =
                            Placement::new("string_lights", level_to_world_at(p, 0.0), yaw);
                        pl.stretch = len / n as f32 / STRING_SPAN_M;
                        pl.part = part;
                        self.placements.push(pl);
                    }
                    let mut end = Placement::new("string_post", level_to_world_at(b, 0.0), yaw);
                    end.part = part;
                    self.placements.push(end);
                    let mid = (a + b) / 2.0;
                    self.lamp(
                        &l.id,
                        "string_lights",
                        level_to_world_at(mid, 1.2),
                        radius,
                        colors::LAMP_LIGHT,
                        part,
                    );
                }
            }
            "board_lamp" => {
                // on an info board / the map board: on its socket (README_night)
                let board = l.attach.as_deref().and_then(|id| data.element(id));
                match board {
                    Some(e) if e.kind.as_deref() == Some("info_board") => {
                        let (p, d) = info_board_pose(e, data);
                        self.board_lamp(
                            &l.id,
                            p,
                            facing_yaw(d),
                            crate::scene::info_board_lamp_socket(e),
                            radius,
                            part,
                        );
                    }
                    Some(e) if e.kind.as_deref() == Some("map_board") => {
                        let spawn = data
                            .parts
                            .get(e.part)
                            .map_or(data.spawn.cell(), |p| p.spawn.cell());
                        let (p, d) = crate::scene::map_board_pose(e, spawn);
                        self.board_lamp(&l.id, p, facing_yaw(d), BOARD_LAMP_MAP, radius, part);
                    }
                    _ => {
                        if let Some(p) = l.pos() {
                            self.board_lamp(
                                &l.id,
                                p,
                                facing_yaw(dir),
                                BOARD_LAMP_MAP,
                                radius,
                                part,
                            );
                        }
                    }
                }
            }
            "wall_lamp" => {
                if let Some(p) = l.pos() {
                    let yaw = facing_yaw(dir);
                    // origin on the facade (the data point is 5 cm in front of it)
                    let wall = p - fwd * 0.05;
                    let h = l.height_m.unwrap_or(WALL_LAMP_MOUNT_M);
                    let mut pl = Placement::new("wall_lamp", level_to_world_at(wall, h), yaw);
                    pl.part = part;
                    self.placements.push(pl);
                    let q = wall + model_offset(WALL_LAMP_LIGHT, yaw);
                    self.lamp(
                        &l.id,
                        "wall_lamp",
                        level_to_world_at(q, h + WALL_LAMP_LIGHT.y),
                        radius,
                        colors::LAMP_LIGHT,
                        part,
                    );
                }
            }
            "ceiling" | "indoor" => {
                // the ceiling lamps / bedside lamp of a building model glow by themselves
                let modelled = l
                    .attach
                    .as_deref()
                    .and_then(|id| data.element(id))
                    .is_some_and(|a| {
                        building_model(a).is_some()
                            || data.elements_of(ElementType::Building).any(|b| {
                                building_model(b).is_some()
                                    && b.model_rect.is_some_and(|m| {
                                        m.contains(glam::IVec2::new(a.rect.x, a.rect.z))
                                    })
                            })
                    });
                if let Some(p) = l.pos() {
                    let c = l.color_rgb().unwrap_or(colors::CEILING);
                    // as a light the colour gives the hue; lightened so the room stays
                    // readable (NIGHT-005, "dim but readable")
                    let light = [
                        c[0] + (1.0 - c[0]) * 0.45,
                        c[1] + (1.0 - c[1]) * 0.45,
                        c[2] + (1.0 - c[2]) * 0.45,
                    ];
                    if !modelled {
                        self.glow(
                            &l.id,
                            level_to_world_at(p, l.height_m.unwrap_or(2.4)),
                            Vec3::new(0.35, 0.12, 0.35),
                            0.0,
                            c,
                            part,
                        );
                    }
                    let kind = if l.color.is_some() {
                        "indoor_colored"
                    } else {
                        "ceiling"
                    };
                    self.lamp(&l.id, kind, level_to_world_at(p, 1.2), radius, light, part);
                }
            }
            _ => {
                // lantern post (default): the arm and lantern towards `facing`
                if let Some(p) = l.pos() {
                    let yaw = facing_yaw(dir);
                    let mut pl = Placement::new("lantern_post", level_to_world_at(p, 0.0), yaw);
                    pl.part = part;
                    self.placements.push(pl);
                    let q = p + model_offset(LANTERN_LIGHT, yaw);
                    self.lamp(
                        &l.id,
                        "lantern_post",
                        level_to_world_at(q, LANTERN_LIGHT.y),
                        radius,
                        colors::LAMP_LIGHT,
                        part,
                    );
                }
            }
        }
    }

    /// `board_lamp` on a board's socket (same yaw as the board; README_night "Board-lamp
    /// sockets"), its light on the panel.
    fn board_lamp(&mut self, id: &str, pos: Vec2, yaw: f32, socket: Vec3, radius: f32, part: u8) {
        let q = pos + model_offset(socket, yaw);
        let mut pl = Placement::new("board_lamp", level_to_world_at(q, socket.y), yaw);
        pl.part = part;
        self.placements.push(pl);
        let light = q + model_offset(BOARD_LAMP_LIGHT, yaw);
        self.lamp(
            id,
            "board_lamp",
            level_to_world_at(light, (socket.y + BOARD_LAMP_LIGHT.y - 0.3).max(0.9)),
            radius,
            colors::LAMP_LIGHT,
            part,
        );
    }

    /// Lit windows (placeholder: warm rectangles on the building box) on the sides that
    /// face outwards, every ~2.5 m, not over the door; a wall lamp beside the door.
    fn windows(
        &mut self,
        r: Rect,
        door: Option<glam::IVec2>,
        door_lamp: bool,
        wall_h: f32,
        id: &str,
        part: u8,
    ) {
        let (x0, z0) = (r.x as f32, r.z as f32);
        let (x1, z1) = ((r.x + r.w) as f32, (r.z + r.d) as f32);
        let y = (wall_h * 0.45).clamp(1.0, 1.6);
        let (ww, wh) = (0.7, 0.6);
        let sides: [(Vec2, Vec2, Dir); 4] = [
            (Vec2::new(x0, z0), Vec2::new(x1, z0), Dir::S),
            (Vec2::new(x0, z1), Vec2::new(x1, z1), Dir::N),
            (Vec2::new(x0, z0), Vec2::new(x0, z1), Dir::W),
            (Vec2::new(x1, z0), Vec2::new(x1, z1), Dir::E),
        ];
        for (a, b, dir) in sides {
            let len = a.distance(b);
            let n = ((len - 0.6) / 2.5).floor() as i32;
            if n < 1 || wall_h < 1.6 {
                continue;
            }
            let out = dir.offset().as_vec2();
            for k in 0..n {
                let t = (k as f32 + 0.5) / n as f32;
                let p = a.lerp(b, t) + out * 0.04;
                if door.is_some_and(|d| crate::level::cell_center(d).distance(p) < 1.1) {
                    continue;
                }
                let size = if matches!(dir, Dir::S | Dir::N) {
                    Vec3::new(ww, wh, 0.06)
                } else {
                    Vec3::new(0.06, wh, ww)
                };
                self.glow(id, level_to_world_at(p, y), size, 0.0, colors::WINDOW, part);
                // wooden frame bar (keeps the window readable as a window)
                let bar = if matches!(dir, Dir::S | Dir::N) {
                    Vec3::new(0.06, wh, 0.08)
                } else {
                    Vec3::new(0.08, wh, 0.06)
                };
                self.prop(
                    id,
                    level_to_world_at(p + out * 0.01, y),
                    bar,
                    0.0,
                    colors::POST,
                    part,
                );
            }
        }
        if let Some(d) = door.filter(|_| door_lamp) {
            // wall lamp beside the door, outside
            let c = crate::level::cell_center(d);
            let rc = rect_center(r);
            let out = if (c.y - z0).abs() < 1.0 {
                Dir::S
            } else if (c.y - z1).abs() < 1.0 {
                Dir::N
            } else if c.x < rc.x {
                Dir::W
            } else {
                Dir::E
            };
            let o = out.offset().as_vec2();
            let side = Vec2::new(o.y, -o.x);
            let edge = match out {
                Dir::S => Vec2::new(c.x, z0),
                Dir::N => Vec2::new(c.x, z1),
                Dir::W => Vec2::new(x0, c.y),
                Dir::E => Vec2::new(x1, c.y),
            };
            // `wall_lamp` model on the facade beside the door (README_night mount 1.6 m)
            let p = edge + side * 0.85;
            let yaw = facing_yaw(out);
            let mut pl = Placement::new("wall_lamp", level_to_world_at(p, WALL_LAMP_MOUNT_M), yaw);
            pl.part = part;
            self.placements.push(pl);
            let q = p + model_offset(WALL_LAMP_LIGHT, yaw);
            self.lamp(
                &format!("{id}:door"),
                "wall_lamp",
                level_to_world_at(q, WALL_LAMP_MOUNT_M + WALL_LAMP_LIGHT.y),
                default_radius("wall_lamp"),
                colors::LAMP_LIGHT,
                part,
            );
        }
    }

    /// Indoor light of an enterable building (seen when the roof is hidden): a warm ceiling
    /// light, in the night house one soft blue and one warm red-orange light (Q-116).
    fn indoor_lights(&mut self, id: &str, inner: Rect, night_house: bool, part: u8) {
        let c = rect_center(inner);
        let r = (inner.w.max(inner.d) as f32 * 0.5 + 0.5).clamp(2.2, 3.5);
        if night_house {
            // the hall: a dim blue light along it (the enclosures get their own lights)
            let rr = (inner.w.min(inner.d) as f32 / 2.0 + 1.2).min(3.0);
            let n = (inner.w.max(inner.d) as f32 / 5.0).ceil().max(1.0) as usize;
            for k in 0..n {
                let t = (k as f32 + 0.5) / n as f32;
                let p = if inner.w >= inner.d {
                    Vec2::new(inner.x as f32 + t * inner.w as f32, c.y)
                } else {
                    Vec2::new(c.x, inner.z as f32 + t * inner.d as f32)
                };
                self.lamp(
                    &format!("{id}:hall{k}"),
                    "indoor_colored",
                    level_to_world_at(p, 1.4),
                    rr,
                    colors::HOUSE_BLUE,
                    part,
                );
            }
        } else {
            self.lamp(
                &format!("{id}:ceiling"),
                "ceiling",
                level_to_world_at(c, 1.2),
                r,
                colors::CEILING,
                part,
            );
        }
    }

    /// Moon door (GAME-NIGHT rule 3): the model's moon sign, rim and pillar lanterns glow at
    /// night (`*_glow` slots); its two lanterns light the doorway (`light_l` / `light_r`).
    fn moon_door(&mut self, e: &crate::level::Element, data: &LevelData, part: u8) {
        let (c, yaw) = moon_door_pose(e, data);
        for (k, p) in MOON_DOOR_LIGHTS.iter().enumerate() {
            let q = c + model_offset(*p, yaw);
            self.lamp(
                &format!("{}:lantern{k}", e.id),
                "moon_door",
                level_to_world_at(q, p.y),
                2.5,
                colors::LAMP_LIGHT,
                part,
            );
        }
        self.lamp(
            &format!("{}:sign", e.id),
            "moon_door",
            level_to_world_at(c, 1.2),
            2.8,
            [0.88, 0.92, 1.0],
            part,
        );
    }
}
