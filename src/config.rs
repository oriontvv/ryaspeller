use std::fmt::Display;
use std::str::FromStr;
use std::usize;

/// Describes all supported languages
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    /// Russian
    RU = 1 << 0,
    /// English
    EN = 1 << 1,
    /// Ukranian
    UA = 1 << 2,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::EN),
            "ru" => Ok(Language::RU),
            "uk" => Ok(Language::UA),
            _ => Err(format!("Unsupported language {}", s)),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match *self {
            Language::RU => "ru",
            Language::EN => "en",
            Language::UA => "uk",
        };
        write!(f, "{}", printable)
    }
}

/// Provides a way to specify a set of supported [Languages](Language)
///
/// Should be created with `Languages::default()`
#[derive(Debug, Clone, Copy)]
pub struct Languages {
    langs: usize,
}

#[allow(dead_code)]
impl Languages {
    fn new() -> Languages {
        Languages { langs: 0 }
    }

    /// Enables specified [Language]
    pub fn enable_language(&mut self, language: Language) {
        self.langs |= language as usize;
    }

    /// Disables specified [Language]
    pub fn disable_language(&mut self, language: Language) {
        self.langs |= !(language as usize);
    }
}

impl FromStr for Languages {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut languages = Languages::new();
        let mut errors = String::new();
        for split in s.split(',') {
            let lang = Language::from_str(split);
            match lang {
                Err(msg) => {
                    errors.push(',');
                    errors.push_str(&msg);
                }
                Ok(lang) => {
                    languages.enable_language(lang);
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(languages)
    }
}

impl Default for Languages {
    fn default() -> Self {
        Languages {
            langs: Language::EN as usize | Language::RU as usize,
        }
    }
}

impl Display for Languages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static ALL_LANGUAGES: [Language; 3] = [Language::EN, Language::RU, Language::UA];

        let mut first = true;
        for lang in ALL_LANGUAGES {
            if self.langs & (lang as usize) == 0 {
                continue;
            }
            if !first {
                write!(f, ",")?;
            }
            write!(f, "{}", lang)?;
            first = false;
        }

        Ok(())
    }
}

/// Provides customization for [crate::Speller]
///
/// Can be edited after Speller initialization and all changes will be
/// applyed to corresponded Speller
///
/// Example
/// ```rust
/// use ryaspeller::{Config, Speller};
/// let mut config = Config::default();
/// let speller = Speller::new(config);
/// speller.spell_text("can spell a text");
/// config.ignore_digits = true;
/// speller.spell_text("spelling with updated config");
/// ```

#[derive(Clone, Debug, Copy, Default)]
pub struct Config {
    _langs: Languages,

    /// Enables html format instead of plain text.
    pub is_html: bool,

    /// Ignores words with numbers, such as avp17h4534.
    pub ignore_digits: bool,

    /// Ignores Internet addresses, email addresses and filenames.
    pub ignore_urls: bool,

    /// Highlights repetitions of words, consecutive. For example, I flew to to to Cyprus.
    pub find_repeat_words: bool,

    /// Ignores the incorrect use of UPPERCASE / lowercase letters, for example, in the word moscow.
    pub ignore_capitalization: bool,
}

#[allow(dead_code)]
impl Config {
    /// Returns a Set of enabled [Languages]
    pub fn languages(&self) -> Languages {
        self._langs
    }
    /// Parses enabled [Languages] fron string representation
    ///
    /// Example
    /// ```rust
    /// use ryaspeller::Config;
    /// let mut config = Config::default();
    /// config.set_languages("en,ru,uk");
    /// ```
    pub fn set_languages(&mut self, langs: &str) -> Result<Languages, String> {
        let languages = Languages::from_str(langs)?;
        self._langs = languages;
        Ok(languages)
    }

    /// Enables specified [Language]
    pub fn enable_language(&mut self, language: Language) {
        self._langs.enable_language(language)
    }

    /// Disables specified [Language]
    pub fn disable_language(&mut self, language: Language) {
        self._langs.disable_language(language)
    }
}
