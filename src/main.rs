mod config;

use std::{path::Path, str::FromStr};

use clap::Parser;
use ryaspeller::{Config, Languages, Speller};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CliArgs {
    #[clap(required = true)]
    text_or_path: Vec<String>,

    #[clap(short, long)]
    lang: Option<String>,
    #[clap(long)]
    is_html: bool,
    #[clap(long)]
    ignore_digits: bool,
    #[clap(long)]
    ignore_urls: bool,
    #[clap(long)]
    find_repeat_words: bool,
    #[clap(long)]
    ignore_capitalization: bool,
}

fn create_config(args: &mut CliArgs) -> Result<Config, String> {
    let languages: Languages = if let Some(lang) = &args.lang {
        Languages::from_str(lang)?
    } else {
        Languages::default()
    };

    Ok(Config {
        languages,
        is_html: args.is_html,
        ignore_digits: args.ignore_digits,
        ignore_urls: args.ignore_urls,
        find_repeat_words: args.find_repeat_words,
        ignore_capitalization: args.ignore_capitalization,
    })
}

fn main() {
    let mut args = CliArgs::parse();

    let config = create_config(&mut args).expect("Can't create config");
    let speller = Speller::new(config);

    for text in args.text_or_path {
        if text.starts_with("http") {
            let spell_result = speller.spell_url(&text);
            match spell_result {
                Err(err) => println!("Spellcheck error for url: {err}"),
                Ok(spelled) => println!("{spelled}"),
            }
            continue;
        }

        let path = Path::new(&text);
        if path.exists() {
            if let Err(err) = speller.spell_path(path) {
                println!("Spellcheck error for path: {err}")
            }
            continue;
        }

        let spell_result = speller.spell_text(&text);
        match spell_result {
            Err(err) => println!("Spellcheck error for text: {err}"),
            Ok(spelled) => println!("{spelled}"),
        }
    }
}
