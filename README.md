# ryaspeller
[![Actions Status](https://github.com/oriontvv/ryaspeller/workflows/CI/badge.svg)](https://github.com/oriontvv/ryaspeller/actions)
[![Coverage badge](https://raw.githubusercontent.com/oriontvv/ryaspeller/coverage/htmlcov/badges/flat.svg)](https://htmlpreview.github.io/?https://github.com/oriontvv/ryaspeller/coverage/htmlcov/index.html) 
[![dependency status](https://deps.rs/repo/github/oriontvv/ryaspeller/status.svg)](https://deps.rs/repo/github/oriontvv/ryaspeller)
[![Doc](https://docs.rs/ryaspeller/badge.svg)](https://docs.rs/ryaspeller)
[![Crates.io](https://img.shields.io/crates/v/ryaspeller.svg)](https://crates.io/crates/ryaspeller)


[ryaspeller](https://github.com/oriontvv/ryaspeller) (Rust Yandex Speller) is a tool and library for searching typos in text, files and websites.

Uses [Yandex.Speller API](https://tech.yandex.ru/speller/doc/dg/concepts/About-docpage/). ([restrictions](<https://yandex.ru/legal/speller_api/>))

## Installation

* binary
```bash
cargo install cargo-binstall && cargo binstall ryaspeller
```

* from sources
```bash
cargo install ryaspeller
```

## Usage

 * binary:

```bash
$ ryaspeller "text_or_path_or_url"
$ ryaspeller russt --lang en
rust
$ ryaspeller ./doc --lang en,ru
$ ryaspeller https://team-tricky.github.io > page.html
```

 * library:
 ```rust
use ryaspeller::Speller;
let speller = Speller::default();
let spelled = speller.spell_text("Triky Custle is a funny puzzle game.").unwrap();
assert!(spelled == "Tricky Castle is a funny puzzle game.");
 ```

There are also available [python](https://github.com/oriontvv/pyaspeller/) and [javascript](https://github.com/hcodes/yaspeller) versions of this speller.
