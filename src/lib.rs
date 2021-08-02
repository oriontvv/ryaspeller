/*!
This crate is a client for [Yandex Speller Api] and a binary tool that
allows to check spelling in texts, paths and web-pages.

# Usage

This crate is on [crates.io] and can be used by adding `ryaspeller`
to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
ryaspeller = "0.1.0"
```

See [examples] of using.

[Yandex Speller Api]: https://yandex.ru/dev/speller
[examples]: https://github.com/oriontvv/ryaspeller/blob/master/examples
[crates.io]: https://crates.io/crates/ryaspeller

*/

#![warn(missing_docs)]
#![allow(
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else
)]

mod config;

extern crate serde;

use std::path::Path;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub use config::{Config, Language, Languages};

/// Describes a result of Speller backend-api response
#[derive(Debug, Deserialize)]
pub struct SpellResult {
    /// error code
    pub code: u32,
    /// column number
    col: u32,
    /// lenght of the word with error
    len: u32,
    /// position of the word with error
    pos: u32,
    /// row number
    row: u32,
    /// list of suggestions, can be empty
    s: Vec<String>,
    /// origin word
    word: String,
}

/// Describes a list of results op Speller backend api response
pub type SpellResults = Vec<SpellResult>;

/// Main instance, provides spelling functionality.
/// Must be created with [Config]
///
/// Example
/// ```rust
/// use ryaspeller::{Config, Speller, SpellResults};
/// let speller = Speller::new(Config::default());
/// let spelled: String = speller.spell_text("a text").unwrap();
/// let result: SpellResults = speller.check_text("a text").unwrap();
/// ```
pub struct Speller {
    /// Corresponding [Config]
    pub config: Config,

    api_url: String,
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
    /// Creates new [Speller] instance
    pub fn new(config: Config) -> Speller {
        Speller {
            api_url: String::from(
                "https://speller.yandex.net/services/spellservice.json/checkText",
            ),
            config,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Correct specified text of current [Speller]
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

    /// Correct texts inside specified path
    pub fn spell_path(&self, _path: &Path) -> Result<String> {
        todo!()
    }

    /// Runs spell cheking for specified text of current [Speller]
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
