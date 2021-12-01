mod config;

use std::path::Path;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use ryaspeller::{Config, Speller};

fn create_config(args: &ArgMatches) -> Result<Config, String> {
    let mut config = Config::default();

    if let Some(lang) = args.value_of("lang") {
        config.set_languages(lang)?;
    }

    config.ignore_digits = args.is_present("ignore_digits");
    config.ignore_urls = args.is_present("ignore_urls");
    config.find_repeat_words = args.is_present("find_repeat_words");
    config.ignore_capitalization = args.is_present("ignore_capitalization");

    Ok(config)
}

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("text_or_path")
                .required(true)
                .multiple(true)
                .use_delimiter(false)
                .takes_value(true),
        )
        .arg(Arg::with_name("lang").long("lang").takes_value(true))
        .arg(
            Arg::with_name("ignore_digits")
                .long("ignore_digits")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("ignore_urls")
                .long("ignore_urls")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("find_repeat_words")
                .long("find_repeat_words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("ignore_capitalization")
                .long("ignore_capitalization")
                .takes_value(false),
        )
        .get_matches();

    let config = create_config(&args).expect("Can't create config");
    let speller = Speller::new(config);

    let text_or_paths = args.values_of("text_or_path").unwrap().collect::<Vec<_>>();

    for text in text_or_paths {
        if text.starts_with("http") {
            let spell_result = speller.spell_url(text);
            match spell_result {
                Err(err) => println!("Spellcheck error for url: {}", err),
                Ok(spelled) => println!("{}", spelled),
            }
            continue;
        }

        let path = Path::new(&text);
        if path.exists() {
            if let Err(err) = speller.spell_path(path) {
                println!("Spellcheck error for path: {}", err)
            }
            continue;
        }

        let spell_result = speller.spell_text(text);
        match spell_result {
            Err(err) => println!("Spellcheck error for text: {}", err),
            Ok(spelled) => println!("{}", spelled),
        }
    }
}
