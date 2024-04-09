#![allow(non_snake_case)]

use dioxus::prelude::*;
use lazy_static::lazy_static;
use simsearch::SimSearch;

use crate::components::emoji_card;
use crate::pages::*;

lazy_static! {
    pub static ref SEARCH: SimSearch<emoji_card::Props> = {
        let mut engine: SimSearch<emoji_card::Props> = SimSearch::new();
        for emoji in all_emojis().iter() {
            engine.insert(emoji.clone(), emoji.alias);
        }
        engine
    };
}

pub fn filter_emojis(input: &str) -> Vec<emoji_card::Props> {
    SEARCH.search(input)
}

pub fn all_emojis() -> Vec<emoji_card::Props> {
    let mut result = vec![];
    for (alias, emoji) in emojic::alias::GEMOJI_MAP.iter() {
        result.push(emoji_card::Props {
            alias: alias,
            value: emoji.grapheme,
        })
    }
    result
}

pub static EMOJIS: GlobalSignal<Vec<emoji_card::Props>> = Signal::global(|| all_emojis());
pub static FILTER: GlobalSignal<String> = Signal::global(|| String::new());

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
}

pub fn App() -> Element {
    rsx!{
        Router::<Route> {}
    }
}