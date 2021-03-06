use ryaspeller::{Config, Language, SpellResults, Speller};

fn simplest() {
    let speller = Speller::default();
    let spelled: String = speller.spell_text("В суббботу утромъ.").unwrap();
    assert!(spelled == "В субботу утром.");
}

fn customize_config() {
    let mut config = Config::default();
    let speller: Speller = Speller::new(config);

    // config can be edited after creation of speller
    config.enable_language(Language::EN);
    config.ignore_digits = true;

    let spell_results: SpellResults = speller
        .check_text("Some engliish и русскиий тексты")
        .unwrap();
    assert!(spell_results.len() == 2);
    /*
    dbg!(spell_results);

    SpellResult {
        code: 1,
        col: 5,
        len: 8,
        pos: 5,
        row: 0,
        s: [
            "english",
        ],
        word: "engliish",
    },
    SpellResult {
        code: 1,
        col: 16,
        len: 8,
        pos: 16,
        row: 0,
        s: [
            "русский",
            "русские",
        ],
        word: "русскиий",
    },
     */
}

fn main() {
    simplest();
    customize_config()
}
