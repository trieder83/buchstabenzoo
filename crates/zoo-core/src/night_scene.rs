//! Night-only scene parts (GAME-NIGHT §10, `art/night/README.md`): lamps and their light,
//! placeholder lamp props (post + glowing box) until the `kit_night` models exist, lit
//! windows on buildings, board lamps on every info board, wall lamps at doors, the glowing
//! moon sign, indoor lights of enterable buildings (the night house in blue and warm
//! red-orange, Q-116) and the firefly areas. Pure — the renderer only draws it, and hides it
//! all by day.

use glam::{Vec2, Vec3};

use crate::coords::level_to_world_at;
use crate::level::{ElementType, LevelData, Rect};
use crate::scene::{facing_yaw, info_board_pose, BoxPlacement, Dir};

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

/// Light radius (m) per lamp kind (art plan: post 3 m, board lamp 1.2 m, player 2.5 m).
pub fn default_radius(kind: &str) -> f32 {
    match kind {
        "lantern_post" => 2.4,
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
                        s.board_lamp(&e.id, pos, dir, 1.62, part);
                    }
                }
                (ElementType::Barrier, "moon_door") => {
                    let (c, along_z) = crate::scene::moon_door_axis(e, data);
                    s.moon_door(c, along_z, &e.id, part)
                }
                (ElementType::Building, "entrance") => {}
                (ElementType::Building, _) => {
                    let h = e.height_m.unwrap_or(4.0) * 0.7;
                    let door = e.door_cell();
                    let door_lamp = !attached("wall_lamp", &e.id);
                    s.windows(e.rect, door, door_lamp, h, &e.id, part);
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
                                s.glow(
                                    &enc.id,
                                    level_to_world_at(ec, 3.0),
                                    Vec3::new(0.5, 0.1, 0.5),
                                    0.0,
                                    color,
                                    part,
                                );
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
                let pts = l.points();
                let h = l.height_m.unwrap_or(2.8);
                for w in pts.windows(2) {
                    let (a, b) = (w[0], w[1]);
                    let len = a.distance(b);
                    let n = (len / 0.8).ceil().max(1.0) as usize;
                    for k in 0..=n {
                        let p = a.lerp(b, k as f32 / n as f32);
                        let sag = 0.35 * (std::f32::consts::PI * k as f32 / n as f32).sin();
                        let bulb = [
                            colors::LAMP_GLOW,
                            [1.0, 0.62, 0.45],
                            [0.62, 0.86, 1.0],
                            [0.80, 1.0, 0.62],
                        ][k % 4];
                        self.glow(
                            &l.id,
                            level_to_world_at(p, h - sag - 0.12),
                            Vec3::splat(0.12),
                            0.0,
                            bulb,
                            part,
                        );
                    }
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
                for p in &pts {
                    self.prop(
                        &l.id,
                        level_to_world_at(*p, 0.0),
                        Vec3::new(0.12, h, 0.12),
                        0.0,
                        colors::POST,
                        part,
                    );
                }
            }
            "board_lamp" => {
                // on an info board: at the board's pose (its readable side)
                let board = l
                    .attach
                    .as_deref()
                    .and_then(|id| data.element(id))
                    .filter(|e| e.kind.as_deref() == Some("info_board"));
                if let Some(e) = board {
                    let (p, d) = info_board_pose(e, data);
                    self.board_lamp(&l.id, p, d, l.height_m.unwrap_or(1.62), part);
                } else if let Some(p) = l.pos() {
                    // map board: under its small roof
                    self.board_lamp(&l.id, p, dir, l.height_m.unwrap_or(2.1), part);
                }
            }
            "wall_lamp" => {
                if let Some(p) = l.pos() {
                    let h = l.height_m.unwrap_or(2.2);
                    let yaw = facing_yaw(dir);
                    self.prop(
                        &l.id,
                        level_to_world_at(p, h + 0.22),
                        Vec3::new(0.26, 0.06, 0.26),
                        yaw,
                        colors::METAL,
                        part,
                    );
                    self.glow(
                        &l.id,
                        level_to_world_at(p, h - 0.1),
                        Vec3::new(0.2, 0.3, 0.2),
                        yaw,
                        colors::LAMP_GLOW,
                        part,
                    );
                    self.lamp(
                        &l.id,
                        "wall_lamp",
                        level_to_world_at(p + fwd * 0.4, 1.2),
                        radius,
                        colors::LAMP_LIGHT,
                        part,
                    );
                }
            }
            "ceiling" | "indoor" => {
                if let Some(p) = l.pos() {
                    let c = l.color_rgb().unwrap_or(colors::CEILING);
                    // as a light the colour gives the hue; lightened so the room stays
                    // readable (NIGHT-005, "dim but readable")
                    let light = [
                        c[0] + (1.0 - c[0]) * 0.45,
                        c[1] + (1.0 - c[1]) * 0.45,
                        c[2] + (1.0 - c[2]) * 0.45,
                    ];
                    self.glow(
                        &l.id,
                        level_to_world_at(p, l.height_m.unwrap_or(2.4)),
                        Vec3::new(0.35, 0.12, 0.35),
                        0.0,
                        c,
                        part,
                    );
                    let kind = if l.color.is_some() {
                        "indoor_colored"
                    } else {
                        "ceiling"
                    };
                    self.lamp(&l.id, kind, level_to_world_at(p, 1.2), radius, light, part);
                }
            }
            _ => {
                // lantern post (default): post, arm towards `facing`, hanging lantern
                if let Some(p) = l.pos() {
                    let yaw = facing_yaw(dir);
                    let h = l.height_m.unwrap_or(2.4);
                    self.prop(
                        &l.id,
                        level_to_world_at(p, 0.0),
                        Vec3::new(0.14, h, 0.14),
                        yaw,
                        colors::POST,
                        part,
                    );
                    let arm = p + fwd * 0.3;
                    self.prop(
                        &l.id,
                        level_to_world_at(arm, h - 0.2),
                        Vec3::new(0.08, 0.08, 0.08)
                            + level_to_world_at(fwd.abs() * 0.55, 0.0).abs(),
                        0.0,
                        colors::POST,
                        part,
                    );
                    let lantern = p + fwd * 0.55;
                    self.prop(
                        &l.id,
                        level_to_world_at(lantern, h - 0.34),
                        Vec3::new(0.3, 0.08, 0.3),
                        yaw,
                        colors::METAL,
                        part,
                    );
                    self.glow(
                        &l.id,
                        level_to_world_at(lantern, h - 0.74),
                        Vec3::new(0.24, 0.4, 0.24),
                        yaw,
                        colors::LAMP_GLOW,
                        part,
                    );
                    self.lamp(
                        &l.id,
                        "lantern_post",
                        level_to_world_at(lantern, 1.2),
                        radius,
                        colors::LAMP_LIGHT,
                        part,
                    );
                }
            }
        }
    }

    /// Board lamp over an info board's panel (GAME-NIGHT rule 5): lamp head + emissive glass
    /// + a small light on the panel.
    fn board_lamp(&mut self, id: &str, pos: Vec2, dir: Dir, height: f32, part: u8) {
        let fwd = dir.offset().as_vec2();
        let yaw = facing_yaw(dir);
        let head = pos + fwd * 0.28;
        self.prop(
            id,
            level_to_world_at(pos + fwd * 0.14, height),
            Vec3::new(0.05, 0.05, 0.05) + level_to_world_at(fwd.abs() * 0.3, 0.0).abs(),
            0.0,
            colors::METAL,
            part,
        );
        self.prop(
            id,
            level_to_world_at(head, height - 0.02),
            Vec3::new(0.34, 0.06, 0.16),
            yaw,
            colors::METAL,
            part,
        );
        self.glow(
            id,
            level_to_world_at(head, height - 0.08),
            Vec3::new(0.28, 0.06, 0.1),
            yaw,
            colors::LAMP_GLOW,
            part,
        );
        self.lamp(
            id,
            "board_lamp",
            level_to_world_at(pos + fwd * 0.3, 1.1),
            1.2,
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
            let p = edge + side * 0.85 + o * 0.12;
            let yaw = facing_yaw(out);
            self.prop(
                id,
                level_to_world_at(p, 2.35),
                Vec3::new(0.24, 0.06, 0.24),
                yaw,
                colors::METAL,
                part,
            );
            self.glow(
                id,
                level_to_world_at(p, 2.05),
                Vec3::new(0.18, 0.28, 0.18),
                yaw,
                colors::LAMP_GLOW,
                part,
            );
            self.lamp(
                &format!("{id}:door"),
                "wall_lamp",
                level_to_world_at(p + o * 0.5, 1.2),
                2.1,
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

    /// Moon door (GAME-NIGHT rule 3): glowing moon sign + soft blue rim at night.
    fn moon_door(&mut self, c: Vec2, along_z: bool, id: &str, part: u8) {
        let size = if along_z {
            Vec3::new(0.16, 0.8, 0.8)
        } else {
            Vec3::new(0.8, 0.8, 0.16)
        };
        self.glow(
            id,
            level_to_world_at(c, 2.95),
            size,
            0.0,
            colors::MOON,
            part,
        );
        let rim = if along_z {
            Vec3::new(0.66, 0.1, 3.0)
        } else {
            Vec3::new(3.0, 0.1, 0.66)
        };
        self.glow(
            id,
            level_to_world_at(c, 2.6),
            rim,
            0.0,
            colors::MOON_RIM,
            part,
        );
        // lanterns on both pillars
        let side = if along_z { Vec2::Y } else { Vec2::X };
        for s in [-1.0f32, 1.0] {
            let p = c + side * s * 1.35;
            self.glow(
                id,
                level_to_world_at(p, 3.26),
                Vec3::new(0.3, 0.36, 0.3),
                0.0,
                colors::LAMP_GLOW,
                part,
            );
        }
        self.lamp(
            &format!("{id}:sign"),
            "moon_door",
            level_to_world_at(c, 1.2),
            2.8,
            [0.88, 0.92, 1.0],
            part,
        );
    }
}
