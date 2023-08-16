#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::prelude::*;
use lazy_static::lazy_static;
use simsearch::SimSearch;

use crate::components::emoji_card;
use crate::pages::home;

lazy_static! {
    pub static ref SEARCH: SimSearch<emoji_card::Props<'static>> = {
        let mut engine: SimSearch<emoji_card::Props> = SimSearch::new();
        for emoji in all_emojis().iter() {
            engine.insert(emoji.clone(), emoji.alias);
        }
        engine
    };
}

pub fn filter_emojis(input: &str) -> Vec<emoji_card::Props<'static>> {
    SEARCH.search(input)
}

pub fn all_emojis() -> Vec<emoji_card::Props<'static>> {
    let mut result = vec![];
    for (alias, emoji) in emojic::alias::GEMOJI_MAP.iter() {
        result.push(emoji_card::Props {
            alias: alias,
            value: emoji.grapheme,
        })
    }
    result
}

pub static EMOJIS: Atom<Vec<emoji_card::Props>> = Atom(|_| all_emojis());
pub static FILTER: Atom<String> = Atom(|_| String::new());

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        home::view {}
    }
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    render!{
        Router::<Route> {}
    }
}