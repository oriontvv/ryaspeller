mod config;

use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};
use ryaspeller::{Config, Speller};

#[derive(Clap, Debug)]
#[clap(
    about = crate_description!(),
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!()
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct CliArgs {
    #[clap(short, long)]
    path: Option<String>,

    #[clap(short, long)]
    text: Option<String>,

    #[clap(short, long)]
    lang: Option<String>,
    #[clap(long, takes_value = false)]
    ignore_digits: bool,
    #[clap(long, takes_value = false)]
    ignore_urls: bool,
    #[clap(long, takes_value = false)]
    find_repeat_words: bool,
    #[clap(long, takes_value = false)]
    ignore_capitalization: bool,
}

fn create_config(args: &mut CliArgs) -> Result<Config, String> {
    let mut config = Config::default();

    if let Some(lang) = &args.lang {
        config.set_languages(lang)?;
    }

    config.set_ignore_digits(args.ignore_digits);
    config.set_ignore_urls(args.ignore_urls);
    config.set_find_repeat_words(args.find_repeat_words);
    config.set_ignore_capitalization(args.ignore_capitalization);

    Ok(config)
}

fn main() {
    let mut args = CliArgs::parse();
    let config = create_config(&mut args).expect("Can't create config");
    let speller = Speller::new(config);

    if let Some(text) = args.text {
        let spell_result = speller.spell_text(&text);
        match spell_result {
            Err(err) => println!("Spellcheck error for text: {}", err),
            Ok(spelled) => println!("{}", spelled),
        }
    }

    // if let Some(path) = args.path {
    //     let path = Path::from_str(path);
    //     let spell_result = speller.spell_path(path);
    //     match spell_result {
    //         Err(err) => println!("Spellcheck error for path: {}", err),
    //         Ok(spelled) => println!("{}", spelled),
    //     }
    // }
}
