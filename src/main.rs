use clap::{crate_authors, AppSettings, Clap};
use ryaspeller::{Config, Speller};

#[derive(Clap, Debug)]
#[clap(
    about = "Search tool typos in the text, files and websites.",
    name = "ryaspeller",
    version = "1.0",
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
    speller.spell_word("суббботу");
}
