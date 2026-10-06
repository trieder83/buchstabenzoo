//! Math tasks (CONT-MATH): the math level (`mathe1`…`mathe5`) and the one seeded task of the
//! golf-cart note, whose result is the key-box combination (GAME-CART rules 14–18).
//!
//! The task is generated from the game seed on **its own RNG stream** (it never consumes the
//! game RNG), so it is reproducible and old saves keep their behaviour (MATH-009). Every task
//! has exactly one integer answer in `1..=999` (the lock has 3 digit wheels, Q-132/Q-373).

use crate::content::Language;
use crate::rng::Pcg32;

/// Math level (CONT-MATH): grade 1…5, independent of the reading level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum MathLevel {
    #[default]
    Mathe1,
    Mathe2,
    Mathe3,
    Mathe4,
    Mathe5,
}

impl MathLevel {
    pub const ALL: [MathLevel; 5] = [
        MathLevel::Mathe1,
        MathLevel::Mathe2,
        MathLevel::Mathe3,
        MathLevel::Mathe4,
        MathLevel::Mathe5,
    ];

    /// `mathe1` … `mathe5`.
    pub fn id(self) -> &'static str {
        match self {
            MathLevel::Mathe1 => "mathe1",
            MathLevel::Mathe2 => "mathe2",
            MathLevel::Mathe3 => "mathe3",
            MathLevel::Mathe4 => "mathe4",
            MathLevel::Mathe5 => "mathe5",
        }
    }

    /// The level of an id; `None` for anything unknown.
    pub fn from_id(id: &str) -> Option<MathLevel> {
        MathLevel::ALL.into_iter().find(|l| l.id() == id)
    }

    /// An id of a save or a setting; missing / unknown counts as `mathe1` (GAME-CART rule 17).
    pub fn from_id_or_default(id: &str) -> MathLevel {
        MathLevel::from_id(id).unwrap_or_default()
    }

    /// 0 for `mathe1` … 4 for `mathe5`.
    pub fn index(self) -> u64 {
        self as u64
    }

    /// The task kinds of this level (CONT-MATH "Cart note" table).
    pub fn kinds(self) -> &'static [MathKind] {
        match self {
            MathLevel::Mathe1 => &[MathKind::Add, MathKind::Sub],
            MathLevel::Mathe2 => &[MathKind::Add, MathKind::Sub, MathKind::Mul],
            MathLevel::Mathe3 => &[MathKind::Add, MathKind::Sub, MathKind::Mul, MathKind::Div],
            MathLevel::Mathe4 => &[MathKind::Add, MathKind::Sub, MathKind::Mul, MathKind::Unit],
            MathLevel::Mathe5 => &[MathKind::Frac, MathKind::Money, MathKind::Word],
        }
    }
}

/// Kind of a task (the word in `math-cart-<kind>-<reading level>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MathKind {
    Add,
    Sub,
    Mul,
    Div,
    Unit,
    Frac,
    Money,
    Word,
}

impl MathKind {
    pub const ALL: [MathKind; 8] = [
        MathKind::Add,
        MathKind::Sub,
        MathKind::Mul,
        MathKind::Div,
        MathKind::Unit,
        MathKind::Frac,
        MathKind::Money,
        MathKind::Word,
    ];

    pub fn id(self) -> &'static str {
        match self {
            MathKind::Add => "add",
            MathKind::Sub => "sub",
            MathKind::Mul => "mul",
            MathKind::Div => "div",
            MathKind::Unit => "unit",
            MathKind::Frac => "frac",
            MathKind::Money => "money",
            MathKind::Word => "word",
        }
    }
}

/// The visual aid of a task (shown on the note after [`LOCK_HELP_TRIES`] wrong codes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathAid {
    /// Two groups of dots (`minus`: the second group is taken away).
    Dots { a: u32, b: u32, minus: bool },
    /// A rows × columns block grid.
    Blocks { rows: u32, cols: u32 },
    /// `groups` equal groups of `per` dots.
    Groups { groups: u32, per: u32 },
    /// A bar of `parts` parts, `shaded` of them shaded.
    Bar { parts: u32, shaded: u32 },
}

impl MathAid {
    /// Id for the host (`dots`, `blocks`, `groups`, `bar`).
    pub fn id(self) -> &'static str {
        match self {
            MathAid::Dots { .. } => "dots",
            MathAid::Blocks { .. } => "blocks",
            MathAid::Groups { .. } => "groups",
            MathAid::Bar { .. } => "bar",
        }
    }
}

/// Wrong codes in a row (since the note was last read) after which the note pulses, the hint
/// leads to it and its visual aid is shown (GAME-CART rule 15, CONT-MATH rule 5).
pub const LOCK_HELP_TRIES: u8 = 3;

/// The task of the golf-cart note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathTask {
    pub level: MathLevel,
    pub kind: MathKind,
    /// Operands per kind: `add`/`sub`/`mul` `[a, b]`; `div` `[dividend, divisor]`; `unit`
    /// `[value, factor]` (factor 100 = m → cm, 10 = cm → mm); `frac` `[numerator, denominator,
    /// whole]`; `money` `[a_cents, b_cents]`; `word` `[boxes, per_box, extra]` (a × b + c).
    pub operands: Vec<u32>,
    /// The result, `1..=999`.
    pub answer: u32,
    pub aid: MathAid,
}

/// The combination text of an answer: always 3 digits with leading zeros (`5` → `005`).
pub fn pad3(answer: u32) -> String {
    format!("{answer:03}")
}

/// A typed code `000`…`999` back to its number (`None` unless exactly 3 digits).
pub fn parse_code(code: &str) -> Option<u32> {
    (code.len() == 3 && code.bytes().all(|b| b.is_ascii_digit()))
        .then(|| code.parse().ok())
        .flatten()
}

/// Evaluates an expression of `kind` exactly. `None` for a malformed expression or one whose
/// result is not an integer (no remainder, no fraction). The generator uses it for every task
/// it produces; the tests check it against their own evaluator.
pub fn evaluate(kind: MathKind, ops: &[u32]) -> Option<i64> {
    let o = |i: usize| ops.get(i).map(|&v| i64::from(v));
    match kind {
        MathKind::Add => Some(o(0)? + o(1)?),
        MathKind::Sub => Some(o(0)? - o(1)?),
        MathKind::Mul => Some(o(0)? * o(1)?),
        MathKind::Div => {
            let (a, b) = (o(0)?, o(1)?);
            (b > 0 && a % b == 0).then_some(a / b)
        }
        MathKind::Unit => Some(o(0)? * o(1)?),
        MathKind::Frac => {
            let (n, d, w) = (o(0)?, o(1)?, o(2)?);
            (d > 0 && w % d == 0).then_some(w / d * n)
        }
        MathKind::Money => {
            let s = o(0)? + o(1)?;
            (s % 100 == 0).then_some(s / 100)
        }
        MathKind::Word => Some(o(0)? * o(1)? + o(2)?),
    }
}

/// Splitmix64 finaliser: the seed of the note stream (hash of the game seed, the tag `cart`
/// and the level index).
fn stream_seed(seed: u64, level: MathLevel) -> u64 {
    let mut z = seed
        ^ 0x6361_7274_0000_0000 // "cart"
        ^ (level.index() + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Uniform value in `lo..=hi`.
fn range(rng: &mut Pcg32, lo: u32, hi: u32) -> u32 {
    lo + rng.below(hi - lo + 1)
}

/// The one task of the golf-cart note for a game seed and a math level (CONT-MATH "Cart note").
/// Deterministic; own RNG stream (does not touch the game RNG); the answer is an integer in
/// `1..=999` that [`evaluate`] confirms.
pub fn cart_note_task(seed: u64, level: MathLevel) -> MathTask {
    let mut rng = Pcg32::new(stream_seed(seed, level));
    let kinds = level.kinds();
    // (the loop re-draws only for the rare operand combination that misses a rule)
    loop {
        let kind = kinds[rng.below(kinds.len() as u32) as usize];
        let Some(operands) = operands_for(&mut rng, level, kind) else {
            continue;
        };
        let Some(v) = evaluate(kind, &operands) else {
            continue;
        };
        if !(1..=999).contains(&v) {
            continue;
        }
        let answer = v as u32;
        let aid = aid_for(kind, &operands, answer);
        return MathTask {
            level,
            kind,
            operands,
            answer,
            aid,
        };
    }
}

fn operands_for(rng: &mut Pcg32, level: MathLevel, kind: MathKind) -> Option<Vec<u32>> {
    use MathKind::*;
    use MathLevel::*;
    Some(match (level, kind) {
        (Mathe1, Add) => {
            let a = range(rng, 1, 19);
            vec![a, range(rng, 1, 20 - a)]
        }
        (Mathe1, Sub) => {
            let a = range(rng, 2, 20);
            vec![a, range(rng, 1, a - 1)]
        }
        (Mathe2, Add) => {
            let a = range(rng, 11, 90);
            vec![a, range(rng, 10, 100 - a)]
        }
        (Mathe2, Sub) => {
            let a = range(rng, 20, 99);
            vec![a, range(rng, 10, a - 1)]
        }
        (Mathe2, Mul) => {
            let a = [2, 3, 4, 5, 10][rng.below(5) as usize];
            vec![a, range(rng, 1, 10)]
        }
        (Mathe3, Add) => {
            let a = range(rng, 100, 889);
            vec![a, range(rng, 10, 999 - a)]
        }
        (Mathe3, Sub) => {
            let a = range(rng, 100, 999);
            vec![a, range(rng, 10, a - 1)]
        }
        (Mathe3, Mul) => vec![range(rng, 2, 10), range(rng, 2, 10)],
        (Mathe3, Div) => {
            let (a, b) = (range(rng, 2, 10), range(rng, 1, 10));
            vec![a * b, a]
        }
        (Mathe4, Add) => {
            let a = range(rng, 100, 899);
            let b = range(rng, 100, 999 - a);
            // written addition with a carry
            ((a % 10 + b % 10 >= 10) || (a / 10 % 10 + b / 10 % 10 >= 10)).then(|| vec![a, b])?
        }
        (Mathe4, Sub) => {
            let a = range(rng, 200, 999);
            let b = range(rng, 100, a - 1);
            // written subtraction with a borrow
            ((a % 10 < b % 10) || (a / 10 % 10 < b / 10 % 10)).then(|| vec![a, b])?
        }
        (Mathe4, Mul) => {
            if rng.below(4) == 0 {
                vec![range(rng, 100, 499), 2]
            } else {
                let (a, b) = (range(rng, 11, 99), range(rng, 2, 9));
                (a * b <= 999).then(|| vec![a, b])?
            }
        }
        (Mathe4, Unit) => {
            if rng.below(2) == 0 {
                vec![range(rng, 1, 9), 100] // m → cm
            } else {
                vec![range(rng, 11, 99), 10] // cm → mm
            }
        }
        (Mathe5, Frac) => {
            let (n, d) = [(1, 2), (1, 4), (3, 4), (1, 5)][rng.below(4) as usize];
            let k = range(rng, 2, 999 / d);
            vec![n, d, k * d]
        }
        (Mathe5, Money) => {
            // two amounts that end in .50 add up to whole euros
            let (x, y) = (range(rng, 1, 400), range(rng, 1, 400));
            vec![x * 100 + 50, y * 100 + 50]
        }
        (Mathe5, Word) => vec![range(rng, 2, 9), range(rng, 5, 50), range(rng, 1, 50)],
        _ => return None,
    })
}

fn aid_for(kind: MathKind, ops: &[u32], answer: u32) -> MathAid {
    match kind {
        MathKind::Add => MathAid::Dots {
            a: ops[0],
            b: ops[1],
            minus: false,
        },
        MathKind::Sub => MathAid::Dots {
            a: ops[0],
            b: ops[1],
            minus: true,
        },
        MathKind::Mul | MathKind::Word => MathAid::Blocks {
            rows: ops[0],
            cols: ops[1],
        },
        MathKind::Div => MathAid::Groups {
            groups: answer,
            per: ops[1],
        },
        MathKind::Unit => MathAid::Groups {
            groups: ops[0],
            per: ops[1],
        },
        MathKind::Frac => MathAid::Bar {
            parts: ops[1],
            shaded: ops[0],
        },
        // two amounts ending in .50: rounding one up and one down keeps the sum
        MathKind::Money => MathAid::Dots {
            a: ops[0].div_ceil(100),
            b: ops[1] / 100,
            minus: false,
        },
    }
}

fn euros(cents: u32, lang: Language) -> String {
    let sep = if lang == Language::De { ',' } else { '.' };
    format!("{}{sep}{:02}", cents / 100, cents % 100)
}

impl MathTask {
    /// Fluent key of the task text: `math-cart-<kind>-<reading level>` (CONT-MATH rule 3).
    pub fn text_key(&self, reading_level: crate::content::ReadingLevel) -> String {
        format!("math-cart-{}-{}", self.kind.id(), reading_level.id())
    }

    /// The expression in numerals (every level, with `= ?`): `7 × 5 = ?`, `2 m = ? cm`.
    pub fn expression(&self, lang: Language) -> String {
        let o = &self.operands;
        match self.kind {
            MathKind::Add => format!("{} + {} = ?", o[0], o[1]),
            MathKind::Sub => format!("{} − {} = ?", o[0], o[1]),
            MathKind::Mul => format!("{} × {} = ?", o[0], o[1]),
            MathKind::Div => format!("{} ÷ {} = ?", o[0], o[1]),
            MathKind::Unit if o[1] == 100 => format!("{} m = ? cm", o[0]),
            MathKind::Unit => format!("{} cm = ? mm", o[0]),
            MathKind::Frac => {
                let f = match (o[0], o[1]) {
                    (1, 2) => "½",
                    (1, 4) => "¼",
                    (3, 4) => "¾",
                    _ => "⅕",
                };
                format!("{f} × {} = ?", o[2])
            }
            MathKind::Money => format!("{} € + {} € = ? €", euros(o[0], lang), euros(o[1], lang)),
            MathKind::Word => format!("{} × {} + {} = ?", o[0], o[1], o[2]),
        }
    }

    /// Fluent arguments of the task text (`$a`, `$b`, `$c`): the operands as numerals.
    pub fn text_args(&self, lang: Language) -> Vec<(&'static str, String)> {
        let o = &self.operands;
        let names = ["a", "b", "c"];
        let mut out: Vec<(&'static str, String)> = Vec::new();
        for (i, n) in names.iter().enumerate() {
            if let Some(&v) = o.get(i) {
                out.push((n, v.to_string()));
            }
        }
        if self.kind == MathKind::Money {
            out[0].1 = euros(o[0], lang);
            out[1].1 = euros(o[1], lang);
        }
        out
    }

    /// The lock code of the answer (`005`).
    pub fn code(&self) -> String {
        pad3(self.answer)
    }
}
