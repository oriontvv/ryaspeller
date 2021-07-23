use std::path::Path;
use std::{collections::HashMap, default};

use serde::{Deserialize, Serialize};

pub struct Config {
    lang: String,

    ignore_uppercase: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: &Path) -> Self {
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
            lang: String::from("en"),
            ignore_uppercase: false,
        }
    }
}

// #[derive(Debug, Serialize)]
// struct RequestData<'a> {
//     text: &'a str,
//     options: u32,
//     lang: &'a str,
//     format: &'a str,
// }

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
            config: config,
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

        if self.config.ignore_uppercase {
            options |= 1
        }

        options
    }

    fn call_api(&self, text: &str) -> Result<String, reqwest::Error> {
        let options = String::from(self.api_options().to_string());
        let mut data = HashMap::new();
        data.insert("text", text);
        data.insert("options", options.as_str());
        data.insert("lang", "en");
        data.insert("format", "auto");

        // let data = RequestData {
        //     text: text,
        //     options: self.api_options(),
        //     lang: &"en",
        //     format: &"auto",
        // };
        //
        // dbg!(data);

        let response = self.client.post(self.api_url.as_str()).json(&data).send()?;

        if (!response.status().is_success()) {
            return Ok("".to_string());
        }

        dbg!(response.text());

        Ok(String::from(""))
    }
}
