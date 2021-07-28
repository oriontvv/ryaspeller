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
}

fn main() {
    let _args = CliArgs::parse();

    let config = Config::default();
    let speller = Speller::new(config);
    let spelled = speller.spell_text("В суббботу утромъ").unwrap();
    dbg!(spelled);
}
