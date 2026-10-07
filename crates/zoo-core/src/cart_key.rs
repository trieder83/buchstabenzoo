//! The golf-cart key (GAME-CART rules 12–19): the note "Math Fighter" on the desk, the key box
//! with its 3-digit combination lock, the tries counter and the math level of the note.
//! The carts themselves are a later work package; here only the key can be won.

use glam::Vec2;

use crate::game::{Game, GameEvent, Interactable, Target, PANEL_KEEP_RANGE_M};
use crate::level::{facing_vec, ItemData};
use crate::math::{cart_note_task, MathLevel, MathTask, LOCK_HELP_TRIES};

/// `[[item]]` kind of the note on the desk.
pub const NOTE_KIND: &str = "note_math_fighter";
/// `[[item]]` kind of the key box.
pub const KEY_BOX_KIND: &str = "key_box";

/// Result of [`Game::enter_code`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeResult {
    /// The right code: the box opened, the key is in the pocket.
    Right,
    /// A wrong code; `tries` = wrong codes in a row (never a lockout).
    Wrong { tries: u8 },
    /// The player is too far from the key box (the panel is closed by the host then).
    OutOfReach,
    /// There is no closed key box (none in the level, or already open).
    NoBox,
}

impl Game {
    /// The math level of the note (setting, default `mathe1`).
    pub fn math_level(&self) -> MathLevel {
        self.settings.math_level
    }

    /// Sets the math level (GAME-CART rule 18): the note task and the combination are
    /// regenerated from the same seed with the new level, `key_box_tries` is 0 again, `note_read`
    /// and an opened box stay.
    pub fn set_math_level(&mut self, level: MathLevel) {
        if self.settings.math_level != level {
            self.settings.math_level = level;
            self.key_box_tries = 0;
            self.note_aid = false;
        }
    }

    /// The task of the golf-cart note for this playthrough's seed and the math level; its
    /// answer, padded to 3 digits, is the combination (own RNG stream, never the game RNG).
    pub fn cart_task(&self) -> MathTask {
        cart_note_task(self.seed, self.settings.math_level)
    }

    /// The combination of the key box (`005`).
    pub fn cart_code(&self) -> String {
        self.cart_task().code()
    }

    /// The `[[item]]`s of a kind in unlocked levels.
    fn cart_items(&self, kind: &'static str) -> impl Iterator<Item = &ItemData> {
        self.level
            .data
            .items
            .iter()
            .filter(move |it| it.kind == kind && self.part_unlocked(it.part))
    }

    pub(crate) fn cart_key_interactables(&self, out: &mut Vec<Interactable>) {
        let readable = |it: &ItemData| it.facing.as_deref().map(facing_vec);
        for it in self.cart_items(NOTE_KIND) {
            out.push(Interactable {
                target: Target::Note { id: it.id.clone() },
                point: it.pos(),
                readable: readable(it),
            });
        }
        if !self.key_box_open {
            for it in self.cart_items(KEY_BOX_KIND) {
                out.push(Interactable {
                    target: Target::KeyBox { id: it.id.clone() },
                    point: it.pos(),
                    readable: readable(it),
                });
            }
        }
    }

    /// Reading the note (GAME-CART rule 14): `note_read`, `key_box_tries = 0`.
    pub(crate) fn read_note(&mut self) {
        self.note_read = true;
        self.key_box_tries = 0;
        self.events.push(GameEvent::NoteRead);
    }

    /// Whether the note pulses and the hint leads to it (3 wrong codes in a row since the
    /// note was last read, GAME-CART rule 15).
    pub fn lock_help(&self) -> bool {
        self.key_box_tries >= LOCK_HELP_TRIES
    }

    /// Whether the note panel shows the visual aid of its task (CONT-MATH rule 5): from the 3rd
    /// wrong code on, until the task changes. No penalty (Q-014).
    pub fn note_aid(&self) -> bool {
        self.note_aid
    }

    /// The key box and the stand cell the player must be near (level `[[item]]`).
    pub fn key_box_point(&self) -> Option<Vec2> {
        self.cart_items(KEY_BOX_KIND).map(|it| it.pos()).next()
    }

    /// A code typed at the key box lock (GAME-CART rule 15; any number of tries, never a
    /// lockout). The right code opens the box and puts the key into the pocket.
    pub fn enter_code(&mut self, code: u32) -> CodeResult {
        let Some(at) = self.key_box_point().filter(|_| !self.key_box_open) else {
            return CodeResult::NoBox;
        };
        if at.distance(self.player.pos) > PANEL_KEEP_RANGE_M {
            return CodeResult::OutOfReach;
        }
        if code == self.cart_task().answer {
            self.key_box_open = true;
            self.has_cart_key = true;
            self.key_box_tries = 0;
            self.events.push(GameEvent::KeyBoxOpened);
            CodeResult::Right
        } else {
            self.key_box_tries = self.key_box_tries.saturating_add(1);
            if self.key_box_tries >= LOCK_HELP_TRIES {
                self.note_aid = true;
            }
            self.events.push(GameEvent::WrongCode {
                tries: self.key_box_tries,
            });
            CodeResult::Wrong {
                tries: self.key_box_tries,
            }
        }
    }
}
