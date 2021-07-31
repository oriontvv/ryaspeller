use std::fmt::Display;
use std::str::FromStr;
use std::usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    RU = 1 << 0,
    EN = 1 << 1,
    UA = 1 << 2,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::EN),
            "ru" => Ok(Language::RU),
            "uk" => Ok(Language::UA),
            _ => Err(format!("Unsupported langauge {}", s)),
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
#[derive(Debug, Clone, Copy)]
pub struct Languages {
    langs: usize,
}

#[allow(dead_code)]
impl Languages {
    pub fn new() -> Languages {
        Languages { langs: 0 }
    }
    pub fn enable_language(&mut self, language: Language) {
        self.langs |= language as usize;
    }

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

#[derive(Clone, Debug, Copy)]
pub struct Config {
    _langs: Languages,

    pub ignore_digits: bool,
    pub ignore_urls: bool,
    pub find_repeat_words: bool,
    pub ignore_capitalization: bool,
}

#[allow(dead_code)]
impl Config {
    pub fn languages(&self) -> Languages {
        self._langs
    }

    pub fn set_languages(&mut self, langs: Languages) -> Result<Languages, String> {
        self._langs = langs;
        Ok(langs)
    }

    pub fn enable_language(&mut self, language: Language) {
        self._langs.enable_language(language)
    }

    pub fn disable_language(&mut self, language: Language) {
        self._langs.disable_language(language)
    }

    pub fn set_ignore_digits(&mut self, value: bool) {
        self.ignore_digits = value
    }

    pub fn set_ignore_urls(&mut self, value: bool) {
        self.ignore_urls = value
    }

    pub fn set_find_repeat_words(&mut self, value: bool) {
        self.find_repeat_words = value
    }

    pub fn set_ignore_capitalization(&mut self, value: bool) {
        self.ignore_capitalization = value
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            _langs: Languages::default(),
            ignore_digits: false,
            ignore_urls: false,
            find_repeat_words: false,
            ignore_capitalization: false,
        }
    }
}
