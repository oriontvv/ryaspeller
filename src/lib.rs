mod config;

extern crate serde;

use std::path::Path;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub use config::{Config, Language, Languages};

#[derive(Debug, Deserialize)]
pub struct SpellResult {
    code: u32,
    col: u32,
    len: u32,
    pos: u32,
    row: u32,
    s: Vec<String>,
    word: String,
}

pub type SpellResults = Vec<SpellResult>;

pub struct Speller {
    api_url: String,
    config: Config,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
struct RequestData<'a> {
    text: &'a str,
    options: u32,
    lang: &'a str,
    format: &'a str,
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

    pub fn spell_text(&self, text: &str) -> Result<String> {
        let spell_results: SpellResults = self.call_api(text)?;

        let mut result_text = text.to_string();
        for spell_result in spell_results {
            if !spell_result.s.is_empty() {
                let word = spell_result.word;
                let suggestion = &spell_result.s[0];
                result_text = result_text.replace(&word, suggestion);
            }
        }

        Ok(result_text)
    }

    pub fn spell_path(&self, _path: &Path) -> Result<String> {
        todo!()
    }

    pub fn check_text(&self, text: &str) -> Result<SpellResults> {
        self.call_api(text)
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

    fn call_api(&self, text: &str) -> Result<SpellResults> {
        if text.len() >= 10_000 {
            return Err(anyhow!("Input text is too long"));
        }
        dbg!(self.api_options());

        let url = format!(
            "{}?text={}&options={}&lang={}&format={}",
            self.api_url,
            text,
            self.api_options(),
            self.config.languages(),
            &"plain",
        );
        let response = self.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to call api {}", response.text()?));
        }

        let results = response.json::<SpellResults>()?;
        Ok(results)
    }
}
