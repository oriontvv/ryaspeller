extern crate serde;

use std::default;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Config {
    lang: String,

    ignore_digits: bool,
    ignore_urls: bool,
    find_repeat_words: bool,
    ignore_capitalization: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(_path: &Path) -> Self {
        // TODO
        Self::default()
    }

    pub fn lang(&mut self, lang: &str) -> &mut Config {
        self.lang = lang.into();
        self
    }
}

impl default::Default for Config {
    fn default() -> Self {
        Config {
            lang: String::from("ru,en"),
            ignore_digits: false,
            ignore_urls: false,
            find_repeat_words: false,
            ignore_capitalization: false,
        }
    }
}

#[derive(Debug, Serialize)]
struct RequestData<'a> {
    text: &'a str,
    options: u32,
    lang: &'a str,
    format: &'a str,
}

#[derive(Debug, Deserialize)]
struct SpellResult {
    code: u32,
    col: u32,
    len: u32,
    pos: u32,
    row: u32,
    s: Vec<String>,
    word: String,
}

type SpellResults = Vec<SpellResult>;

pub struct Speller {
    api_url: String,
    config: Config,
    client: reqwest::blocking::Client,
}

impl Speller {
    pub fn new(config: Config) -> Speller {
        Speller {
            api_url: String::from(
                "https://speller.yandex.net/services/spellservice.json/checkText",
            ),
            config,
            client: reqwest::blocking::Client::new(),
        }
    }
    pub fn spell_word(&self, text: &str) {
        self.call_api(text).unwrap();
    }

    pub fn spell_text(&self, text: &str) {
        // TODO
        self.spell_word(text);
    }

    fn api_options(&self) -> u32 {
        let mut options = 0u32;

        if self.config.ignore_digits {
            options |= 2
        }

        if self.config.ignore_urls {
            options |= 4
        }

        if self.config.find_repeat_words {
            options |= 8
        }

        if self.config.ignore_capitalization {
            options |= 512
        }

        options
    }

    fn call_api(&self, text: &str) -> Result<SpellResults, reqwest::Error> {
        assert!(text.len() < 10_000);

        let url = format!(
            "{}?text={}&options={}&lang={}&format={}",
            self.api_url,
            text,
            self.api_options(),
            self.config.lang,
            &"plain",
        );
        let response = self.client.get(url).send()?;

        let is_ok = response.status().is_success();
        assert!(is_ok);

        let results = response.json::<SpellResults>()?;
        Ok(results)
    }
}
