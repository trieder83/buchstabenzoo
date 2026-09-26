//! Readable content: reading levels (CONT-READING), languages (CONT-L10N) and the Fluent
//! texts of missions and food labels (CONT-MISSIONS).

use std::collections::{BTreeMap, BTreeSet};

use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

/// Reading level (CONT-READING).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReadingLevel {
    Kiga,
    Klasse1,
    Klasse2,
    Klasse3,
}

impl ReadingLevel {
    pub const ALL: [ReadingLevel; 4] = [
        ReadingLevel::Kiga,
        ReadingLevel::Klasse1,
        ReadingLevel::Klasse2,
        ReadingLevel::Klasse3,
    ];

    pub fn from_id(id: &str) -> Option<ReadingLevel> {
        ReadingLevel::ALL.into_iter().find(|l| l.id() == id)
    }

    pub fn id(self) -> &'static str {
        match self {
            ReadingLevel::Kiga => "kiga",
            ReadingLevel::Klasse1 => "klasse1",
            ReadingLevel::Klasse2 => "klasse2",
            ReadingLevel::Klasse3 => "klasse3",
        }
    }
}

/// Supported languages (CONT-L10N §1). `fr` is planned and not enabled yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Language {
    De,
    En,
}

impl Language {
    pub const ALL: [Language; 2] = [Language::De, Language::En];

    pub fn id(self) -> &'static str {
        match self {
            Language::De => "de",
            Language::En => "en",
        }
    }

    pub fn from_id(id: &str) -> Option<Language> {
        Language::ALL.into_iter().find(|l| l.id() == id)
    }
}

/// Default language from a browser/device language tag such as `en-GB` (CONT-L10N §5):
/// the device language if supported, else `de`.
pub fn default_language(device_lang: &str) -> Language {
    let primary = device_lang
        .split(['-', '_'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    Language::from_id(&primary).unwrap_or(Language::De)
}

/// Fluent key of a location riddle (CONT-MISSIONS §1: `mission-<animal>-riddle-<level>`).
///
/// Every animal has one hiding place for now, so the key does not contain the hiding place;
/// more places per animal need a key scheme with the place id (spec gap).
pub fn riddle_key(animal: &str, _hiding_place: &str, level: ReadingLevel) -> String {
    format!("mission-{animal}-riddle-{}", level.id())
}

#[derive(Debug)]
pub enum ContentError {
    Parse { lang: Language, errors: String },
    Duplicate { lang: Language, key: String },
}

impl std::fmt::Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentError::Parse { lang, errors } => {
                write!(f, "ftl parse error ({}): {errors}", lang.id())
            }
            ContentError::Duplicate { lang, key } => {
                write!(f, "duplicate key {key} in {}", lang.id())
            }
        }
    }
}

impl std::error::Error for ContentError {}

/// All Fluent messages per language.
pub struct Content {
    bundles: BTreeMap<Language, FluentBundle<FluentResource>>,
    keys: BTreeMap<Language, BTreeSet<String>>,
}

impl Content {
    /// Loads Fluent sources (several files per language allowed).
    pub fn from_sources(sources: &[(Language, &str)]) -> Result<Self, ContentError> {
        let mut bundles = BTreeMap::new();
        let mut keys: BTreeMap<Language, BTreeSet<String>> = BTreeMap::new();
        for &(lang, src) in sources {
            let res = FluentResource::try_new(src.to_owned()).map_err(|(_, errs)| {
                ContentError::Parse {
                    lang,
                    errors: format!("{errs:?}"),
                }
            })?;
            let set = keys.entry(lang).or_default();
            for entry in res.entries() {
                if let fluent_syntax::ast::Entry::Message(m) = entry {
                    if !set.insert(m.id.name.to_owned()) {
                        return Err(ContentError::Duplicate {
                            lang,
                            key: m.id.name.to_owned(),
                        });
                    }
                }
            }
            let bundle = bundles.entry(lang).or_insert_with(|| {
                let langid: LanguageIdentifier = lang.id().parse().expect("valid language id");
                let mut b = FluentBundle::new(vec![langid]);
                b.set_use_isolating(false);
                b
            });
            bundle
                .add_resource(res)
                .map_err(|errs| ContentError::Parse {
                    lang,
                    errors: format!("{errs:?}"),
                })?;
        }
        Ok(Self { bundles, keys })
    }

    /// Message keys of a language.
    pub fn keys(&self, lang: Language) -> BTreeSet<String> {
        self.keys.get(&lang).cloned().unwrap_or_default()
    }

    /// Formatted text of a message without arguments.
    pub fn text(&self, lang: Language, key: &str) -> Option<String> {
        let bundle = self.bundles.get(&lang)?;
        let pattern = bundle.get_message(key)?.value()?;
        let mut errors = Vec::new();
        let s = bundle.format_pattern(pattern, None, &mut errors);
        errors.is_empty().then(|| s.into_owned())
    }
}

/// Splits text into sentences (at `.`, `!`, `?`) and counts words per sentence (READ-002).
pub fn sentence_word_counts(text: &str) -> Vec<usize> {
    text.split(['.', '!', '?'])
        .map(|s| {
            s.split_whitespace()
                .filter(|w| w.chars().any(char::is_alphanumeric))
                .count()
        })
        .filter(|&n| n > 0)
        .collect()
}

/// Whole-word, case-insensitive match (proposed rule of Q-039, used for RESC-011).
pub fn contains_word(text: &str, word: &str) -> bool {
    let word = word.to_lowercase();
    text.split(|c: char| !c.is_alphanumeric())
        .any(|w| !w.is_empty() && w.to_lowercase() == word)
}
