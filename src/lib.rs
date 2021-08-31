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

use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub use config::{Config, Language, Languages};
use walkdir::WalkDir;

/// Describes a result of Speller backend-api response
#[derive(Debug, Deserialize)]
pub struct SpellResult {
    /// error code, can be described with [Self::get_error_name]
    pub code: u32,
    /// column number
    pub col: u32,
    /// lenght of the word with error
    pub len: u32,
    /// position of the word with error
    pub pos: u32,
    /// row number
    pub row: u32,
    /// list of suggestions, can be empty
    pub s: Vec<String>,
    /// origin word
    pub word: String,
}

impl SpellResult {
    /// Decodes error code if it has detected
    pub fn get_error_name(&self) -> Option<&str> {
        match self.code {
            1 => Some("ERROR_UNKNOWN_WORD"),
            2 => Some("ERROR_REPEAT_WORD"),
            3 => Some("ERROR_CAPITALIZATION"),
            4 => Some("ERROR_UNKNOWN_WORD"),
            _ => None,
        }
    }
}

/// Describes a list of results of Speller backend api response
pub type SpellResults = Vec<SpellResult>;

/// Main instance, provides spelling functionality.
/// Must be initialized with [Config] instance.
///
/// Example
/// ```rust
/// use ryaspeller::{Config, Speller, SpellResults};
/// let speller = Speller::new(Config::default());
/// assert!(speller.spell_text("a texxt").unwrap() == "a text");
/// let result: SpellResults = speller.check_text("a text").unwrap();
/// ```
pub struct Speller {
    /// Corresponding [Config]
    pub config: Config,

    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
struct RequestData<'a> {
    text: &'a str,
    options: u32,
    lang: &'a str,
    format: &'a str,
}

const FORMAT_PLAIN: &str = "plain";
const FORMAT_HTML: &str = "html";

const API_URL: &str = "https://speller.yandex.net/services/spellservice.json/checkText";

impl Speller {
    /// Creates new [Speller] instance
    pub fn new(config: Config) -> Speller {
        Speller {
            config,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Corrects specified text
    ///
    /// Try to apply first available suggestion from [SpellResult]
    pub fn spell_text(&self, text: &str) -> Result<String> {
        self._spell_text(text, self.get_format())
    }

    /// Fetches specified url with blocking http request and check it as html
    pub fn spell_url(&self, url: &str) -> Result<String> {
        let content = self.fetch_page(url)?;
        self._spell_html_text(&content)
    }

    /// Corrects specified text
    ///
    /// Try to apply first available suggestion from [SpellResult]
    /// Unlike [`Self::spell_text`] Enforces html format and ignores [Config]::is_html
    fn _spell_html_text(&self, text: &str) -> Result<String> {
        self._spell_text(text, FORMAT_HTML)
    }

    fn _spell_text(&self, text: &str, format: &str) -> Result<String> {
        // TODO: split bit textes into chunks with no more 10000 chars
        // and make calls async

        let spell_results: SpellResults = self.call_api(text, format)?;

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

    /// Corrects all text files inside specified path
    pub fn spell_path(&self, path: &Path) -> Result<()> {
        // TODO: collect all texts and use batch request to minimize requsts count
        for entry in WalkDir::new(path) {
            if let Err(err) = self._spell_file(entry?.path()) {
                println!("Error: {}", err);
            }
        }
        Ok(())
    }

    fn _spell_file(&self, path: &Path) -> Result<()> {
        if !path.is_file() {
            return Ok(());
        }

        let mut content = String::new();
        let mut file = File::open(path)?;

        if file.read_to_string(&mut content).is_err() {
            return Ok(());
        }

        println!("check {}", path.to_string_lossy());

        let result = if let Some(ext) = path.extension() {
            if ext == "html" {
                self._spell_html_text(&content)
            } else {
                self.spell_text(&content)
            }
        } else {
            self.spell_text(&content)
        };

        if let Err(err) = result {
            // add information about path
            return Err(anyhow!("{}: {}", path.to_string_lossy(), err));
        }

        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write_all(result.unwrap().as_bytes())?;

        Ok(())
    }

    fn get_format(&self) -> &str {
        if self.config.is_html {
            FORMAT_HTML
        } else {
            FORMAT_PLAIN
        }
    }

    /// Runs spell cheking for specified text
    pub fn check_text(&self, text: &str) -> Result<SpellResults> {
        self.call_api(text, self.get_format())
    }

    fn fetch_page(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send()?;
        if !response.status().is_success() {
            return Err(anyhow!("Failed to call api {}", response.text()?));
        }
        Ok(response.text()?)
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

    fn call_api(&self, text: &str, format: &str) -> Result<SpellResults> {
        if text.len() >= 10_000 {
            return Err(anyhow!("Input text is too long: {} >= 10000", &text.len()));
        }

        let url = format!(
            "{}?text={}&options={}&lang={}&format={}",
            API_URL,
            text,
            self.api_options(),
            self.config.languages(),
            format,
        );
        let response = self.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to call api {}", response.text()?));
        }

        let results = response.json::<SpellResults>()?;
        Ok(results)
    }
}
